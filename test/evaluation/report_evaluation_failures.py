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

import sys as _bootstrap_sys

if __name__ == "__main__" and (
    not _bootstrap_sys.flags.isolated
    or not _bootstrap_sys.flags.no_site
    or not _bootstrap_sys.flags.dont_write_bytecode
    or _bootstrap_sys.pycache_prefix != "/dev/null"
):
    import os as _bootstrap_os

    _bootstrap_os.execv(
        _bootstrap_sys.executable,
        [
            _bootstrap_sys.executable,
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
            _bootstrap_os.path.abspath(_bootstrap_sys.argv[0]),
            *_bootstrap_sys.argv[1:],
        ],
    )

import argparse
import datetime as _dt
import hashlib
import importlib.util
import re
import stat
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Iterable

_PARSER_PATH = Path(__file__).resolve().with_name("parse_official_results.py")
_PARSER_SPEC = importlib.util.spec_from_file_location(
    "_orays_failure_report_parser",
    _PARSER_PATH,
)
if _PARSER_SPEC is None or _PARSER_SPEC.loader is None:
    raise RuntimeError(f"cannot load canonical official parser: {_PARSER_PATH}")
_PARSER = importlib.util.module_from_spec(_PARSER_SPEC)
sys.modules[_PARSER_SPEC.name] = _PARSER
_PARSER_SPEC.loader.exec_module(_PARSER)
validate_ltp_output = _PARSER.validate_ltp_output
validate_official_output = _PARSER.validate_official_output
validate_capture_input_pairs = _PARSER.validate_capture_input_pairs
apply_process_exit_code = _PARSER.apply_process_exit_code

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
    strict_events: tuple[str, ...] = ()
    strict_binding_count: int = 0
    lifecycle_complete: bool = False

    @property
    def failed(self) -> bool:
        return (
            not self.lifecycle_complete
            or self.code is None
            or self.timed_out
            or self.panic
            or self.enosys
            or bool(self.internal & {"TFAIL", "TBROK"})
            or self.code != 0
        )

    @property
    def tconf_only(self) -> bool:
        return (
            self.lifecycle_complete
            and self.code == 0
            and not self.failed
            and "TCONF" in self.internal
        )


@dataclass
class LogReport:
    path: Path
    stderr_path: Path
    ltp_validation: dict[str, Any]
    overall_validation: dict[str, Any]
    stdout_text: str = field(repr=False)
    pair_id: str = ""
    source_key: str = ""
    arch: str | None = None
    stdout_resolved_path: str = ""
    stderr_resolved_path: str = ""
    stdout_sha256: str = ""
    stderr_sha256: str = ""
    stdout_size_bytes: int = 0
    stderr_size_bytes: int = 0
    process_exit_code: int = 0
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


class InputReadError(Exception):
    """One evidence input could not be read without changing its bytes."""


def read_utf8_bytes(path: Path) -> tuple[bytes, str]:
    try:
        raw = path.read_bytes()
        return raw, raw.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise InputReadError(
            f"{path}: invalid UTF-8 at byte offset {error.start}: {error.reason}"
        ) from error
    except OSError as error:
        raise InputReadError(f"{path}: {error}") from error


def read_utf8(path: Path) -> str:
    return read_utf8_bytes(path)[1]


def validate_output_separation(output: Path, protected_paths: list[Path]) -> None:
    """Reject output paths that would overwrite or alias any evidence input."""

    try:
        output_resolved = output.resolve()
    except OSError as error:
        raise InputReadError(f"cannot resolve output path {output}: {error}") from error
    protected_identities: set[tuple[int, int]] = set()
    for protected in protected_paths:
        try:
            protected_resolved = protected.resolve()
        except OSError as error:
            raise InputReadError(f"cannot resolve protected input {protected}: {error}") from error
        if output_resolved == protected_resolved:
            raise InputReadError(
                f"output path {output} must not overwrite protected input {protected}"
            )
        try:
            protected_status = protected_resolved.stat()
        except FileNotFoundError:
            continue
        except OSError as error:
            raise InputReadError(f"cannot stat protected input {protected}: {error}") from error
        if stat.S_ISREG(protected_status.st_mode):
            protected_identities.add((protected_status.st_dev, protected_status.st_ino))
    try:
        output_status = output_resolved.stat()
    except FileNotFoundError:
        return
    except OSError as error:
        raise InputReadError(f"cannot stat output path {output}: {error}") from error
    if (
        stat.S_ISREG(output_status.st_mode)
        and (output_status.st_dev, output_status.st_ino) in protected_identities
    ):
        raise InputReadError(
            f"output path {output} must not alias the same physical file as an evidence input"
        )


def clean(line: str) -> str:
    return ANSI_RE.sub("", line).rstrip("\n")


def bullet_escape(text: str, limit: int = 220) -> str:
    text = text.replace("|", "\\|").strip()
    if len(text) > limit:
        return text[: limit - 3] + "..."
    return text


def bind_strict_ltp_cases(report: LogReport) -> None:
    strict_records: dict[tuple[str, str], list[dict[str, Any]]] = {}
    for group in report.ltp_validation.get("groups", []):
        label = str(group.get("label", ""))
        for record in group.get("cases", []):
            name = str(record.get("case", ""))
            strict_records.setdefault((label, name), []).append(record)

    for key, case in report.ltp_cases.items():
        records = strict_records.get(key, [])
        case.strict_binding_count = len(records)
        if len(records) != 1:
            case.evidence.append(
                f"strict lifecycle binding count is {len(records)}, expected exactly 1"
            )
            continue
        record = records[0]
        events = tuple(str(event) for event in record.get("events", []))
        case.strict_events = events
        strict_code = record.get("code")
        expected_events = ("START", "RUN", "RESULT")
        if strict_code == 0:
            expected_events += ("PASS",)
        expected_events += ("END",)
        if strict_code != case.code:
            case.evidence.append(
                f"strict lifecycle code {strict_code!r} does not match parsed code {case.code!r}"
            )
            continue
        if events != expected_events:
            case.evidence.append(
                "strict lifecycle events are "
                + ",".join(events)
                + "; expected "
                + ",".join(expected_events)
            )
            continue
        case.lifecycle_complete = True


def parse_log(
    path: Path,
    stderr_path: Path,
    process_exit_code: int,
    pair: dict[str, Any] | None = None,
) -> LogReport:
    if pair is None:
        try:
            pair = validate_capture_input_pairs([path], [stderr_path])[0]
        except ValueError as error:
            raise InputReadError(str(error)) from error
    stdout_raw, stdout_text = read_utf8_bytes(path)
    stderr_raw, stderr_text = read_utf8_bytes(stderr_path)
    ltp_validation = validate_ltp_output(stdout_text, stderr_text)
    overall_validation = validate_official_output(stdout_text, stderr_text)
    apply_process_exit_code(overall_validation, process_exit_code)
    if overall_validation["status"] == "PASS":
        overall_validation["status"] = "NO_VISIBLE_FAILURE_UNVALIDATED"
    overall_validation["validation_scope"] = "observed-stream-visible-failures"
    apply_process_exit_code(ltp_validation, process_exit_code)
    report = LogReport(
        path=path,
        stderr_path=stderr_path,
        ltp_validation=ltp_validation,
        overall_validation=overall_validation,
        stdout_text=stdout_text,
        pair_id=pair["pair_id"],
        source_key=pair["source_key"],
        arch=pair["arch"],
        stdout_resolved_path=str(pair["stdout_resolved_path"]),
        stderr_resolved_path=str(pair["stderr_resolved_path"]),
        stdout_sha256=hashlib.sha256(stdout_raw).hexdigest(),
        stderr_sha256=hashlib.sha256(stderr_raw).hexdigest(),
        stdout_size_bytes=len(stdout_raw),
        stderr_size_bytes=len(stderr_raw),
        process_exit_code=process_exit_code,
    )
    current_group = "ungrouped"
    current_ltp: LtpCase | None = None

    for raw in stdout_text.splitlines():
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

    bind_strict_ltp_cases(report)
    return report


def parse_libctest_passes(text: str) -> dict[str, int]:
    """Parse the official libctest judge key space from a log section.

    This mirrors the official autotest `kernel/judge/judge_libctest-*.py` protocol:
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

    text = read_utf8(judge_file)
    match = re.search(r'libctest_baseline\s*=\s*"""(.*?)"""', text, re.S)
    if not match:
        return {}
    return parse_libctest_passes(match.group(1))


def extract_group_sections(text: str) -> dict[str, str]:
    """Return exact text for every official-test group in a log."""

    sections: dict[str, list[str]] = {}
    current_group: str | None = None
    for raw in text.splitlines():
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

    sections = extract_group_sections(report.stdout_text)
    for group, judge_name in (
        ("libctest-musl", "judge_libctest-musl.py"),
        ("libctest-glibc", "judge_libctest-glibc.py"),
    ):
        section = sections.get(group)
        judge_file = judge_dir / judge_name
        if section is None:
            continue
        if not judge_file.is_file():
            raise InputReadError(
                f"{judge_file}: required official judge file is missing for {group}"
            )
        baseline = parse_libctest_baseline(judge_file)
        if not baseline:
            raise InputReadError(
                f"{judge_file}: official libctest baseline is missing or empty for {group}"
            )
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


def validation_finding_text(finding: dict[str, Any]) -> str:
    group = finding.get("group")
    prefix = f"{finding.get('kind', 'unknown')}"
    if group:
        prefix += f" [{group}]"
    return f"{prefix}: {finding.get('message', '')}"


def rendered_ltp_status(report: LogReport) -> str:
    status = str(report.ltp_validation.get("status", "ERROR"))
    return "LTP_SCOPE_COMPLETE_NOT_OFFICIAL_VERDICT" if status == "PASS" else status


def render_markdown(reports: list[LogReport]) -> str:
    now = _dt.datetime.now(_dt.timezone.utc).isoformat(timespec="seconds")
    lines: list[str] = [
        "# OSKernel 官方评测未通过用例记录",
        "",
        f"生成时间：{now}",
        "",
        "说明：本报告由配对的 evaluator stdout/stderr 原始日志只读生成；非零退出、TIMEOUT、libctest/busybox fail、官方组 FAIL、LTP TFAIL/TBROK 以及不完整生命周期均列为未通过；LTP TCONF 单独列为配置性未通过/跳过。",
        "报告生成命令退出 0 只表示 Markdown 已成功写出，不表示官方评测或任何 LTP case 通过。",
        "若传入官方 judge 目录，libctest 会额外按官方 `judge_libctest-*.py` 的 section-scoped `START ...` + literal `Pass!` 规则列出缺失/未 Pass 项；这会暴露 wrapper 未运行但官方 baseline 计分的 case。",
        "注：官方组通用超时上限只用于把仍未结束的长跑组闭合并记录为失败/超时；不会跳过用例，也不会把失败或超时改写为 PASS。",
        "",
        "## 汇总",
        "",
        "| stdout | stderr | 全流严格状态 | LTP严格状态 | 完整性错误 | 失败信号 | LTP失败 | LTP TCONF-only | libctest失败(日志) | libctest失败(官方judge) | busybox失败 | 官方组非零退出 | autorun故障 | 官方组超时 | panic/trap | ENOSYS/not implemented |",
        "| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for report in reports:
        lines.append(
            "| "
            + " | ".join(
                [
                    bullet_escape(str(report.path)),
                    bullet_escape(str(report.stderr_path)),
                    str(report.overall_validation.get("status", "ERROR")),
                    rendered_ltp_status(report),
                    str(report.ltp_validation.get("error_count", 0)),
                    str(report.ltp_validation.get("failure_count", 0)),
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
        lines.append(f"- stderr companion: `{report.stderr_path}`")
        lines.append(f"- pair id: `{report.pair_id}`")
        lines.append(f"- source key: `{report.source_key}`")
        lines.append(f"- evaluator process exit code: {report.process_exit_code}")
        lines.append(f"- stdout resolved path: `{report.stdout_resolved_path}`")
        lines.append(f"- stderr resolved path: `{report.stderr_resolved_path}`")
        lines.append(
            f"- stdout bytes / SHA-256: {report.stdout_size_bytes} / `{report.stdout_sha256}`"
        )
        lines.append(
            f"- stderr bytes / SHA-256: {report.stderr_size_bytes} / `{report.stderr_sha256}`"
        )
        lines.append("")
        lines.append("### 全流可见故障扫描（非完整官方 verdict）")
        lines.append(f"- status: **{report.overall_validation.get('status', 'ERROR')}**")
        lines.append(f"- integrity errors: {report.overall_validation.get('error_count', 0)}")
        lines.append(f"- failure signals: {report.overall_validation.get('failure_count', 0)}")
        overall_findings = [
            *report.overall_validation.get("errors", []),
            *report.overall_validation.get("failures", []),
        ]
        if overall_findings:
            for finding in overall_findings:
                lines.append(f"- {bullet_escape(validation_finding_text(finding))}")
        else:
            lines.append("- findings: 无")
        lines.append("")
        lines.append("### LTP 严格生命周期完整性（仅已观察 LTP 组）")
        lines.append(f"- status: **{rendered_ltp_status(report)}**")
        lines.append(f"- groups: {report.ltp_validation.get('group_count', 0)}")
        lines.append(f"- integrity errors: {report.ltp_validation.get('error_count', 0)}")
        lines.append(f"- failure signals: {report.ltp_validation.get('failure_count', 0)}")
        findings = [
            *report.ltp_validation.get("errors", []),
            *report.ltp_validation.get("failures", []),
        ]
        if findings:
            for finding in findings:
                lines.append(f"- {bullet_escape(validation_finding_text(finding))}")
        else:
            lines.append("- findings: 无")
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
            if report.ltp_validation.get("status") == "PASS":
                lines.append("- 无")
            else:
                lines.append(
                    "- 无可列出的 case 级失败；严格生命周期状态为 "
                    f"{report.ltp_validation.get('status', 'ERROR')}，不得据此判定 PASS。"
                )
        else:
            lines.append("| group | case | code | lifecycle | strict events | timeout | internal | runtime_ms | evidence |")
            lines.append("| --- | --- | ---: | --- | --- | --- | --- | ---: | --- |")
            for case in failed:
                evidence = "; ".join(case.evidence[:3])
                lines.append(
                    f"| {bullet_escape(case.group)} | {bullet_escape(case.name)} | "
                    f"{'' if case.code is None else case.code} | "
                    f"{'complete' if case.lifecycle_complete else 'INCOMPLETE'} | "
                    f"{bullet_escape(','.join(case.strict_events))} | "
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
    parser.add_argument(
        "--stderr-log",
        action="append",
        required=True,
        type=Path,
        help=(
            "captured stderr companion for the corresponding positional stdout log; "
            "repeat once per stdout log in the same order"
        ),
    )
    parser.add_argument(
        "--process-exit-code",
        action="append",
        required=True,
        type=int,
        help=(
            "actual evaluator process return code for the corresponding positional stdout "
            "log; repeat once per input pair"
        ),
    )
    parser.add_argument("-o", "--output", type=Path, required=True, help="markdown report path")
    parser.add_argument(
        "--judge-dir",
        type=Path,
        default=None,
        help="optional official kernel/judge directory for section-scoped libctest baseline parsing",
    )
    args = parser.parse_args()

    if len(args.logs) != len(args.stderr_log):
        parser.error("requires exactly one --stderr-log for each positional stdout log")
    if len(args.logs) != len(args.process_exit_code):
        parser.error("requires exactly one --process-exit-code for each capture pair")

    try:
        pairs = validate_capture_input_pairs(args.logs, args.stderr_log)
        protected_paths = [*args.logs, *args.stderr_log]
        if args.judge_dir is not None:
            protected_paths.extend(
                args.judge_dir / name
                for name in ("judge_libctest-musl.py", "judge_libctest-glibc.py")
            )
        validate_output_separation(args.output, protected_paths)
        reports = [
            parse_log(path, stderr_path, process_exit_code, pair)
            for path, stderr_path, process_exit_code, pair in zip(
                args.logs, args.stderr_log, args.process_exit_code, pairs
            )
        ]
        if args.judge_dir is not None:
            for report in reports:
                attach_official_libctest_judge(report, args.judge_dir)
    except (InputReadError, ValueError) as error:
        print(f"input error: {error}; no report was written", file=sys.stderr)
        return 2
    try:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(render_markdown(reports), encoding="utf-8")
    except OSError as error:
        print(f"output error: cannot write {args.output}: {error}", file=sys.stderr)
        return 2
    print(f"wrote {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
