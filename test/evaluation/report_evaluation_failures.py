#!/usr/bin/env python3
"""Build a markdown failure report from OSKernel official evaluator logs.

This is a read-only post-processor.  It does not affect scoring and it treats
all visible failure signals as evidence: non-zero wrapper exits, explicit
TIMEOUT markers, libctest/busybox failures, official-group failures, and LTP
internal TFAIL/TBROK/TCONF lines.  OSKernel's LTP wrapper historically prints
`FAIL LTP CASE <case> : 0` for zero-exit cases; numeric status 0 is therefore
handled as wrapper pass while internal LTP signals remain visible.
"""

from __future__ import annotations

import argparse
import datetime as _dt
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable

ANSI_RE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
GROUP_START_RE = re.compile(r"#### OS COMP TEST GROUP START (.+?) ####")
GROUP_END_RE = re.compile(r"#### OS COMP TEST GROUP END (.+?) ####")
RUN_LTP_RE = re.compile(r"\bRUN LTP CASE\s+(\S+)")
LTP_RESULT_RE = re.compile(r"\b(?:PASS|FAIL) LTP CASE\s+(\S+)\s*:\s*(-?\d+)")
LTP_TIMEOUT_RE = re.compile(r"\bTIMEOUT LTP CASE\s+(\S+)(?:\s+after\s+(\d+)s)?", re.I)
LTP_RUNTIME_RE = re.compile(r"\bLTP CASE RUNTIME\s+(\S+):\s+(\d+)\s+ms\b")
LTP_LIST_RE = re.compile(r"\bltp case list:\s+(.+?)\s+\((\d+)\s+cases,\s+timeout\s+(\d+)s\)", re.I)
LTP_SUITE_SUMMARY_RE = re.compile(r"\bltp cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed(?:,\s+(\d+)\s+timed out)?", re.I)
LIBCTEST_FAIL_RE = re.compile(r"\bFAIL libctest\s+(\S+)\s+([^:]+):\s*(.*)")
LIBCTEST_SUMMARY_RE = re.compile(r"\blibctest cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed,\s+(\d+)\s+timed out", re.I)
BUSYBOX_FAIL_RE = re.compile(r"\btestcase busybox\s+(.+?)\s+fail\b")
OFFICIAL_GROUP_FAIL_RE = re.compile(r"\bFAIL OFFICIAL TEST GROUP\s+(.+?)\s*:\s*(-?\d+)")
OFFICIAL_GROUP_TIMEOUT_RE = re.compile(r"\bTIMEOUT OFFICIAL TEST GROUP\s+(.+?)\s+after\s+(\d+)s", re.I)
AUTORUN_TIMEOUT_RE = re.compile(r"\bautorun:\s+(.+?)\s+timed out after\s+(\d+)s", re.I)
AUTORUN_EXIT_RE = re.compile(r"\bautorun:\s+(.+?)\s+exited with status\s+(-?\d+)", re.I)
INTERNAL_RE = re.compile(r"\b(TFAIL|TBROK|TCONF)\b")
ENOSYS_RE = re.compile(r"\bENOSYS\b|errno=ENOSYS|not implemented", re.I)
PANIC_RE = re.compile(r"\b(panic|panicked|trap|Unhandled trap|InstructionNotExist|fatal trap|kernel trap)\b", re.I)
LIBCTEST_START_STATIC_RE = re.compile(r"\bSTART entry-static\.exe\s+(\S+)")
LIBCTEST_START_DYNAMIC_RE = re.compile(r"\bSTART entry-dynamic\.exe\s+(\S+)")


@dataclass
class OfficialLibctestJudge:
    group: str
    passed: int
    total: int
    failed: list[str] = field(default_factory=list)

    @property
    def failed_count(self) -> int:
        return len(self.failed)


@dataclass
class LtpCase:
    group: str
    name: str
    code: int | None = None
    timed_out: bool = False
    timeout_secs: int | None = None
    runtime_ms: int | None = None
    internal: set[str] = field(default_factory=set)
    enosys: bool = False
    panic: bool = False
    evidence: list[str] = field(default_factory=list)

    @property
    def failed(self) -> bool:
        return (
            self.code is None
            or self.timed_out
            or self.panic
            or self.enosys
            or bool(self.internal & {"TFAIL", "TBROK"})
            or self.code != 0
        )

    @property
    def tconf_only(self) -> bool:
        return self.code == 0 and not self.failed and "TCONF" in self.internal


@dataclass
class LogReport:
    path: Path
    ltp_cases: dict[tuple[str, str], LtpCase] = field(default_factory=dict)
    ltp_manifests: list[str] = field(default_factory=list)
    ltp_summaries: list[str] = field(default_factory=list)
    libctest_failures: list[str] = field(default_factory=list)
    libctest_summaries: list[str] = field(default_factory=list)
    busybox_failures: list[str] = field(default_factory=list)
    group_failures: list[str] = field(default_factory=list)
    group_timeouts: list[str] = field(default_factory=list)
    autorun_failures: list[str] = field(default_factory=list)
    panic_lines: list[str] = field(default_factory=list)
    enosys_lines: list[str] = field(default_factory=list)
    official_libctest: list[OfficialLibctestJudge] = field(default_factory=list)


def clean(line: str) -> str:
    return ANSI_RE.sub("", line).rstrip("\n")


def bullet_escape(text: str, limit: int = 220) -> str:
    text = text.replace("|", "\\|").strip()
    if len(text) > limit:
        return text[: limit - 3] + "..."
    return text


def parse_log(path: Path) -> LogReport:
    report = LogReport(path=path)
    current_group = "ungrouped"
    current_ltp: LtpCase | None = None

    for raw in path.read_text(errors="replace").splitlines():
        line = clean(raw)
        if match := GROUP_START_RE.search(line):
            current_group = match.group(1)
            current_ltp = None
            continue
        if GROUP_END_RE.search(line):
            current_group = "ungrouped"
            current_ltp = None
            continue
        if match := LTP_LIST_RE.search(line):
            report.ltp_manifests.append(line.strip())
            continue
        if match := RUN_LTP_RE.search(line):
            key = (current_group, match.group(1))
            current_ltp = report.ltp_cases.setdefault(key, LtpCase(current_group, match.group(1)))
            continue
        if match := LTP_RESULT_RE.search(line):
            key = (current_group, match.group(1))
            current_ltp = report.ltp_cases.setdefault(key, LtpCase(current_group, match.group(1)))
            current_ltp.code = int(match.group(2))
            if current_ltp.code != 0:
                current_ltp.evidence.append(line.strip())
            continue
        if match := LTP_TIMEOUT_RE.search(line):
            key = (current_group, match.group(1))
            current_ltp = report.ltp_cases.setdefault(key, LtpCase(current_group, match.group(1)))
            current_ltp.timed_out = True
            if match.group(2):
                current_ltp.timeout_secs = int(match.group(2))
            current_ltp.evidence.append(line.strip())
            continue
        if match := LTP_RUNTIME_RE.search(line):
            key = (current_group, match.group(1))
            case = report.ltp_cases.setdefault(key, LtpCase(current_group, match.group(1)))
            case.runtime_ms = int(match.group(2))
            continue
        if LTP_SUITE_SUMMARY_RE.search(line):
            report.ltp_summaries.append(line.strip())
            continue
        if match := LIBCTEST_FAIL_RE.search(line):
            report.libctest_failures.append(f"{current_group}: {match.group(1)} {match.group(2)} => {match.group(3)}")
            continue
        if LIBCTEST_SUMMARY_RE.search(line):
            report.libctest_summaries.append(f"{current_group}: {line.strip()}")
            continue
        if match := BUSYBOX_FAIL_RE.search(line):
            report.busybox_failures.append(f"{current_group}: {match.group(1)}")
            continue
        if match := OFFICIAL_GROUP_FAIL_RE.search(line):
            report.group_failures.append(f"{match.group(1)} => status {match.group(2)}")
            continue
        if match := OFFICIAL_GROUP_TIMEOUT_RE.search(line):
            report.group_timeouts.append(f"{match.group(1)} => timeout after {match.group(2)}s")
            continue
        if match := AUTORUN_TIMEOUT_RE.search(line):
            report.autorun_failures.append(f"{current_group}: {match.group(1)} timed out after {match.group(2)}s")
            continue
        if match := AUTORUN_EXIT_RE.search(line):
            report.autorun_failures.append(f"{current_group}: {match.group(1)} exited with status {match.group(2)}")
            continue

        internal = INTERNAL_RE.findall(line)
        if internal and current_ltp is not None:
            current_ltp.internal.update(internal)
            current_ltp.evidence.append(line.strip())
        if ENOSYS_RE.search(line):
            if current_ltp is not None:
                current_ltp.enosys = True
                current_ltp.evidence.append(line.strip())
            else:
                report.enosys_lines.append(f"{current_group}: {line.strip()}")
        if PANIC_RE.search(line):
            if current_ltp is not None:
                current_ltp.panic = True
                current_ltp.evidence.append(line.strip())
            else:
                report.panic_lines.append(f"{current_group}: {line.strip()}")

    return report


def parse_libctest_passes(text: str) -> dict[str, int]:
    """Parse the official libctest judge key space from a log section.

    This mirrors `/root/autotest-for-oskernel/kernel/judge/judge_libctest-*.py`:
    a case is counted only when a matching START line is followed by a literal
    `Pass!`.  It intentionally does not infer success from wrapper summaries.
    """

    passed: dict[str, int] = {}
    current_key = ""
    for line in text.splitlines():
        if match := LIBCTEST_START_STATIC_RE.search(line):
            current_key = "libctest static " + match.group(1)
        elif match := LIBCTEST_START_DYNAMIC_RE.search(line):
            current_key = "libctest dynamic " + match.group(1)
        if clean(line) == "Pass!" and current_key:
            passed[current_key] = 1
    return passed


def parse_libctest_baseline(judge_file: Path) -> dict[str, int]:
    """Extract the official libctest baseline embedded in a judge script."""

    text = judge_file.read_text(errors="replace")
    match = re.search(r'libctest_baseline\s*=\s*"""(.*?)"""', text, re.S)
    if not match:
        return {}
    return parse_libctest_passes(match.group(1))


def extract_group_sections(path: Path) -> dict[str, str]:
    """Return exact text for every official-test group in a log."""

    sections: dict[str, list[str]] = {}
    current_group: str | None = None
    for raw in path.read_text(errors="replace").splitlines():
        line = clean(raw)
        if match := GROUP_START_RE.search(line):
            current_group = match.group(1)
            sections.setdefault(current_group, []).append(raw)
            continue
        if current_group is not None:
            sections.setdefault(current_group, []).append(raw)
        if GROUP_END_RE.search(line):
            current_group = None
    return {group: "\n".join(lines) for group, lines in sections.items()}


def attach_official_libctest_judge(report: LogReport, judge_dir: Path) -> None:
    """Attach section-scoped official libctest judge results when judges exist."""

    sections = extract_group_sections(report.path)
    for group, judge_name in (
        ("libctest-musl", "judge_libctest-musl.py"),
        ("libctest-glibc", "judge_libctest-glibc.py"),
    ):
        section = sections.get(group)
        judge_file = judge_dir / judge_name
        if section is None or not judge_file.is_file():
            continue
        baseline = parse_libctest_baseline(judge_file)
        if not baseline:
            continue
        observed = parse_libctest_passes(section)
        failed = sorted(key for key in baseline if key not in observed)
        report.official_libctest.append(
            OfficialLibctestJudge(
                group=group,
                passed=len(baseline) - len(failed),
                total=len(baseline),
                failed=failed,
            )
        )


def ltp_failed_cases(report: LogReport) -> list[LtpCase]:
    return sorted((case for case in report.ltp_cases.values() if case.failed), key=lambda c: (c.group, c.name))


def ltp_tconf_cases(report: LogReport) -> list[LtpCase]:
    return sorted((case for case in report.ltp_cases.values() if case.tconf_only), key=lambda c: (c.group, c.name))


def write_section_list(lines: list[str], title: str, rows: Iterable[str], empty: str = "无") -> None:
    rows = list(rows)
    lines.append(f"### {title}")
    if not rows:
        lines.append(f"- {empty}")
    else:
        for row in rows:
            lines.append(f"- {bullet_escape(row)}")
    lines.append("")


def render_markdown(reports: list[LogReport]) -> str:
    now = _dt.datetime.now(_dt.timezone.utc).isoformat(timespec="seconds")
    lines: list[str] = [
        "# OSKernel 官方评测未通过用例记录",
        "",
        f"生成时间：{now}",
        "",
        "说明：本报告由 evaluator 原始日志只读生成；非零退出、TIMEOUT、libctest/busybox fail、官方组 FAIL、LTP TFAIL/TBROK 均列为未通过；LTP TCONF 单独列为配置性未通过/跳过。",
        "若传入官方 judge 目录，libctest 会额外按官方 `judge_libctest-*.py` 的 section-scoped `START ...` + literal `Pass!` 规则列出缺失/未 Pass 项；这会暴露 wrapper 未运行但官方 baseline 计分的 case。",
        "注：官方组通用超时上限只用于把仍未结束的长跑组闭合并记录为失败/超时；不会跳过用例，也不会把失败或超时改写为 PASS。",
        "",
        "## 汇总",
        "",
        "| 日志 | LTP失败 | LTP TCONF-only | libctest失败(日志) | libctest失败(官方judge) | busybox失败 | 官方组非零退出 | autorun故障 | 官方组超时 | panic/trap | ENOSYS/not implemented |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for report in reports:
        lines.append(
            "| "
            + " | ".join(
                [
                    bullet_escape(str(report.path)),
                    str(len(ltp_failed_cases(report))),
                    str(len(ltp_tconf_cases(report))),
                    str(len(report.libctest_failures)),
                    str(sum(item.failed_count for item in report.official_libctest)),
                    str(len(report.busybox_failures)),
                    str(len(report.group_failures)),
                    str(len(report.autorun_failures)),
                    str(len(report.group_timeouts)),
                    str(len(report.panic_lines) + sum(1 for c in report.ltp_cases.values() if c.panic)),
                    str(len(report.enosys_lines) + sum(1 for c in report.ltp_cases.values() if c.enosys)),
                ]
            )
            + " |"
        )
    lines.append("")

    for report in reports:
        lines.append(f"## {report.path}")
        lines.append("")
        if report.ltp_manifests:
            write_section_list(lines, "LTP case list", report.ltp_manifests)
        if report.ltp_summaries:
            write_section_list(lines, "LTP suite summaries", report.ltp_summaries)
        if report.libctest_summaries:
            write_section_list(lines, "libctest summaries", report.libctest_summaries)
        if report.official_libctest:
            lines.append("### 官方 libctest judge 汇总")
            lines.append("| group | passed | total | failed |")
            lines.append("| --- | ---: | ---: | ---: |")
            for item in report.official_libctest:
                lines.append(f"| {bullet_escape(item.group)} | {item.passed} | {item.total} | {item.failed_count} |")
            lines.append("")

        failed = ltp_failed_cases(report)
        lines.append("### LTP 失败/不完整/TBROK/TFAIL/超时")
        if not failed:
            lines.append("- 无")
        else:
            lines.append("| group | case | code | timeout | internal | runtime_ms | evidence |")
            lines.append("| --- | --- | ---: | --- | --- | ---: | --- |")
            for case in failed:
                evidence = "; ".join(case.evidence[:3])
                lines.append(
                    f"| {bullet_escape(case.group)} | {bullet_escape(case.name)} | "
                    f"{'' if case.code is None else case.code} | "
                    f"{case.timeout_secs if case.timed_out and case.timeout_secs else ('yes' if case.timed_out else '')} | "
                    f"{','.join(sorted(case.internal))} | "
                    f"{'' if case.runtime_ms is None else case.runtime_ms} | "
                    f"{bullet_escape(evidence)} |"
                )
        lines.append("")

        tconf = ltp_tconf_cases(report)
        lines.append("### LTP TCONF-only（配置性未通过/跳过，未计为 PASS）")
        if not tconf:
            lines.append("- 无")
        else:
            for case in tconf:
                evidence = "; ".join(case.evidence[:2])
                lines.append(f"- `{case.group}` `{case.name}`: {bullet_escape(evidence)}")
        lines.append("")

        write_section_list(lines, "libctest 失败", report.libctest_failures)
        for item in report.official_libctest:
            write_section_list(
                lines,
                f"官方 libctest judge 未通过（{item.group}，含缺失/未运行/无 Pass!）",
                item.failed,
            )
        write_section_list(lines, "busybox 失败", report.busybox_failures)
        write_section_list(lines, "官方组非零退出", report.group_failures)
        write_section_list(lines, "autorun 故障细节", report.autorun_failures)
        write_section_list(lines, "官方组超时", report.group_timeouts)
        write_section_list(lines, "panic/trap", report.panic_lines)
        write_section_list(lines, "ENOSYS/not implemented", report.enosys_lines)

    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("logs", nargs="+", type=Path, help="evaluator log files")
    parser.add_argument("-o", "--output", type=Path, required=True, help="markdown report path")
    parser.add_argument(
        "--judge-dir",
        type=Path,
        default=None,
        help="optional official /root/autotest-for-oskernel/kernel/judge directory for section-scoped libctest baseline parsing",
    )
    args = parser.parse_args()

    reports = [parse_log(path) for path in args.logs]
    if args.judge_dir is not None:
        for report in reports:
            attach_official_libctest_judge(report, args.judge_dir)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(render_markdown(reports))
    print(f"wrote {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
