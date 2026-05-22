#!/usr/bin/env python3
"""Summarize OSKernel evaluator LTP core output.

Counts wrapper-level LTP case result lines and internal LTP quality signals
(TFAIL/TBROK/TCONF, timeouts, ENOSYS) so RUN_EVAL_DEFAULT_STATUS=0 is not
mistaken for a clean LTP result.  The evaluator's stable wire format may print
`FAIL LTP CASE <case> : 0` for a successful case, so the numeric status is the
source of truth for wrapper pass/fail classification.
"""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any

ANSI_RE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
GROUP_START_RE = re.compile(r"#### OS COMP TEST GROUP START (.+?) ####")
GROUP_END_RE = re.compile(r"#### OS COMP TEST GROUP END (.+?) ####")
CASE_START_RE = re.compile(r"RUN LTP CASE\s+(\S+)")
CASE_RESULT_RE = re.compile(r"\b(PASS|FAIL) LTP CASE\s+(\S+)\s*:\s*(-?\d+)")
TIMEOUT_CASE_RE = re.compile(r"\bTIMEOUT LTP CASE\s+([^\s:]+)(?:\s*:\s*(-?\d+))?", re.IGNORECASE)
CASE_RUNTIME_RE = re.compile(r"\bLTP CASE RUNTIME\s+(\S+):\s+(\d+)\s+ms\b")
CASE_MEMORY_RE = re.compile(
    r"\bLTP MEMORY\s+(\S+)\s+(\S+):\s+free_frames=(\d+)\s+allocated_frames=(\d+)\b"
)
INTERNAL_RE = re.compile(r"\b(TFAIL|TBROK|TCONF)\b")
TIMEOUT_RE = re.compile(
    r"\b(TIMEOUT LTP CASE|timed out|timeout reached|timeout expired|killed after timeout)\b",
    re.IGNORECASE,
)
ENOSYS_RE = re.compile(r"\bENOSYS\b|errno=ENOSYS|not implemented", re.IGNORECASE)
PANIC_TRAP_RE = re.compile(
    r"\b(panic|panicked|trap|Unhandled trap|InstructionNotExist|fatal trap|kernel trap)\b",
    re.IGNORECASE,
)
SUITE_SUMMARY_RE = re.compile(
    r"ltp cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed(?:,\s+(\d+)\s+timed out)?"
)


def strip_ansi(text: str) -> str:
    return ANSI_RE.sub("", text)


def infer_arch(path: Path) -> str:
    name = path.name.lower()
    stem = path.stem.lower()
    if re.search(r"(^|[_-])(la|loongarch64?)([_-]|$)", stem) or "loongarch" in name:
        return "la"
    if re.search(r"(^|[_-])(rv|riscv64?)([_-]|$)", stem) or "riscv" in name:
        return "rv"
    return "unknown"


def infer_libc(group: str) -> str:
    lowered = group.lower()
    if "musl" in lowered:
        return "musl"
    if "glibc" in lowered:
        return "glibc"
    return "unknown"


def new_group() -> dict[str, Any]:
    return {
        "pass_cases": [],
        "fail_cases": [],
        "internal": Counter(),
        "timeouts": 0,
        "enosys": 0,
        "panic_trap": 0,
        "suite_summaries": [],
    }


def new_case_detail(group: str, case: str) -> dict[str, Any]:
    return {
        "group": group,
        "case": case,
        "status": None,
        "code": None,
        "internal": Counter(),
        "timeouts": 0,
        "enosys": 0,
        "panic_trap": 0,
        "runtime_ms": None,
        "memory": {},
    }


def bucket(summary: dict[str, Any], group: str) -> dict[str, Any]:
    groups = summary.setdefault("groups", {})
    return groups.setdefault(group, new_group())


def case_bucket(summary: dict[str, Any], group: str, case: str) -> dict[str, Any]:
    cases = summary.setdefault("case_details", {})
    key = f"{group}\0{case}"
    if key not in cases:
        cases[key] = new_case_detail(group, case)
    return cases[key]


def normalize_wrapper_status(raw_status: str, code: int) -> str:
    """Return the semantic wrapper status for an LTP result line.

    The official score parser-compatible harness line is historically
    `FAIL LTP CASE <case> : 0` even when the test program exited cleanly.  The
    numeric exit status is the source of truth: only status 0 is PASS, and every
    non-zero status remains FAIL even if an input log ever contains a misleading
    PASS token.
    """

    return "PASS" if code == 0 else "FAIL"


def parse_log(text: str) -> dict[str, Any]:
    summary: dict[str, Any] = {
        "pass_cases": [],
        "fail_cases": [],
        "internal": Counter(),
        "timeouts": 0,
        "enosys": 0,
        "panic_trap": 0,
        "suite_summaries": [],
        "case_events": [],
        "case_details": {},
        "groups": {},
    }
    current_group = "ungrouped"
    current_case = None

    for line in strip_ansi(text).splitlines():
        if match := GROUP_START_RE.search(line):
            current_group = match.group(1)
            bucket(summary, current_group)
            current_case = None
            continue
        if GROUP_END_RE.search(line):
            current_group = "ungrouped"
            current_case = None
            continue
        if match := CASE_START_RE.search(line):
            current_case = match.group(1)
            case_bucket(summary, current_group, current_case)
            continue
        if match := CASE_RESULT_RE.search(line):
            raw_status, case, code_text = match.groups()
            code = int(code_text)
            status = normalize_wrapper_status(raw_status, code)
            record = {"group": current_group, "case": case, "code": code}
            target = "pass_cases" if status == "PASS" else "fail_cases"
            summary[target].append(record)
            bucket(summary, current_group)[target].append(record)
            detail = case_bucket(summary, current_group, case)
            detail["status"] = status
            detail["code"] = code
            summary["case_events"].append({"status": status, "raw_status": raw_status, **record})
            current_case = case
            continue
        if match := TIMEOUT_CASE_RE.search(line):
            case, code = match.groups()
            detail = case_bucket(summary, current_group, case)
            detail["status"] = detail["status"] or "TIMEOUT"
            if code is not None:
                detail["code"] = int(code)
            current_case = case
        if match := CASE_RUNTIME_RE.search(line):
            case, runtime_ms = match.groups()
            detail = case_bucket(summary, current_group, case)
            detail["runtime_ms"] = int(runtime_ms)
            current_case = case
            continue
        if match := CASE_MEMORY_RE.search(line):
            case, phase, free_frames, allocated_frames = match.groups()
            detail = case_bucket(summary, current_group, case)
            detail["memory"][phase] = {
                "free_frames": int(free_frames),
                "allocated_frames": int(allocated_frames),
            }
            current_case = case
            continue
        if match := SUITE_SUMMARY_RE.search(line):
            record = {
                "group": current_group,
                "passed": int(match.group(1)),
                "failed": int(match.group(2)),
                "timed_out": int(match.group(3) or 0),
            }
            summary["suite_summaries"].append(record)
            bucket(summary, current_group)["suite_summaries"].append(record)
            current_case = None
            continue
        for marker in INTERNAL_RE.findall(line):
            marker = marker.upper()
            summary["internal"][marker] += 1
            bucket(summary, current_group)["internal"][marker] += 1
            if current_case:
                case_bucket(summary, current_group, current_case)["internal"][marker] += 1
        if "Timeout per run" not in line and TIMEOUT_RE.search(line):
            summary["timeouts"] += 1
            bucket(summary, current_group)["timeouts"] += 1
            if current_case:
                case_bucket(summary, current_group, current_case)["timeouts"] += 1
        if ENOSYS_RE.search(line):
            summary["enosys"] += 1
            bucket(summary, current_group)["enosys"] += 1
            if current_case:
                case_bucket(summary, current_group, current_case)["enosys"] += 1
        if PANIC_TRAP_RE.search(line):
            summary["panic_trap"] += 1
            bucket(summary, current_group)["panic_trap"] += 1
            if current_case:
                case_bucket(summary, current_group, current_case)["panic_trap"] += 1

    reconcile_timeout_statuses(summary)
    return summary


def remove_case_records(records: list[dict[str, Any]], group: str, case: str) -> list[dict[str, Any]]:
    return [
        record
        for record in records
        if not (record["group"] == group and record["case"] == case)
    ]


def reconcile_timeout_statuses(summary: dict[str, Any]) -> None:
    """Keep timeout evidence separate and never count timed-out cases as PASS.

    Normal runner output emits a non-zero `FAIL LTP CASE` line before
    `TIMEOUT LTP CASE`, but this parser is also used on hand-captured and
    remote logs.  If a timeout marker appears after a parser-compatible
    zero-status result line, the timeout evidence wins: remove that case from
    wrapper pass lists and expose its per-case status as `TIMEOUT`.
    """

    for detail in summary["case_details"].values():
        if not detail["timeouts"] or detail["status"] != "PASS":
            continue
        group = detail["group"]
        case = detail["case"]
        detail["status"] = "TIMEOUT"
        summary["pass_cases"] = remove_case_records(summary["pass_cases"], group, case)
        summary["case_events"] = remove_case_records(summary["case_events"], group, case)
        if group in summary["groups"]:
            summary["groups"][group]["pass_cases"] = remove_case_records(
                summary["groups"][group]["pass_cases"], group, case
            )


def compact(summary: dict[str, Any], arch: str = "unknown") -> dict[str, Any]:
    def compact_group(group: dict[str, Any]) -> dict[str, Any]:
        return {
            "pass_count": len(group["pass_cases"]),
            "fail_count": len(group["fail_cases"]),
            "pass_cases": [entry["case"] for entry in group["pass_cases"]],
            "fail_cases": [entry["case"] for entry in group["fail_cases"]],
            "internal": dict(group["internal"]),
            "timeouts": group["timeouts"],
            "enosys": group["enosys"],
            "panic_trap": group["panic_trap"],
            "suite_summaries": group["suite_summaries"],
        }

    rows = []
    matrix: dict[str, dict[str, dict[str, dict[str, Any]]]] = {}
    for detail in sorted(
        summary["case_details"].values(), key=lambda item: (item["case"], item["group"])
    ):
        libc = infer_libc(detail["group"])
        row = {
            "case": detail["case"],
            "arch": arch,
            "libc": libc,
            "group": detail["group"],
            "status": detail["status"] or "UNKNOWN",
            "code": detail["code"],
            "internal": dict(detail["internal"]),
            "timeouts": detail["timeouts"],
            "enosys": detail["enosys"],
            "panic_trap": detail["panic_trap"],
            "runtime_ms": detail["runtime_ms"],
            "memory": detail["memory"],
        }
        before = row["memory"].get("before")
        after_cleanup = row["memory"].get("after_cleanup")
        row["free_frames_delta_after_cleanup"] = (
            None
            if before is None or after_cleanup is None
            else after_cleanup["free_frames"] - before["free_frames"]
        )
        rows.append(row)
        matrix.setdefault(detail["case"], {}).setdefault(arch, {})[libc] = row

    categories: dict[str, list[str]] = {
        "pass_clean": [],
        "pass_with_tconf": [],
        "fail_wrapper": [],
        "internal_tfail": [],
        "internal_tbrok": [],
        "timeout": [],
        "enosys": [],
        "panic_trap": [],
        "unknown": [],
    }
    for row in rows:
        label = f"{row['arch']}:{row['libc']}:{row['case']}"
        has_problem_marker = row["internal"] or row["timeouts"] or row["enosys"] or row["panic_trap"]
        if row["status"] == "PASS" and not has_problem_marker:
            categories["pass_clean"].append(label)
        has_only_tconf = (
            row["internal"].get("TCONF", 0)
            and not row["internal"].get("TFAIL", 0)
            and not row["internal"].get("TBROK", 0)
        )
        if row["status"] == "PASS" and has_only_tconf:
            categories["pass_with_tconf"].append(label)
        if row["status"] == "FAIL":
            categories["fail_wrapper"].append(label)
        if row["internal"].get("TFAIL", 0):
            categories["internal_tfail"].append(label)
        if row["internal"].get("TBROK", 0):
            categories["internal_tbrok"].append(label)
        if row["timeouts"]:
            categories["timeout"].append(label)
        if row["enosys"]:
            categories["enosys"].append(label)
        if row["panic_trap"]:
            categories["panic_trap"].append(label)
        if row["status"] == "UNKNOWN":
            categories["unknown"].append(label)

    return {
        "pass_count": len(summary["pass_cases"]),
        "fail_count": len(summary["fail_cases"]),
        "pass_cases": [f"{entry['group']}:{entry['case']}" for entry in summary["pass_cases"]],
        "fail_cases": [f"{entry['group']}:{entry['case']}" for entry in summary["fail_cases"]],
        "internal": dict(summary["internal"]),
        "timeouts": summary["timeouts"],
        "enosys": summary["enosys"],
        "panic_trap": summary["panic_trap"],
        "suite_summaries": summary["suite_summaries"],
        "groups": {name: compact_group(group) for name, group in summary["groups"].items()},
        "case_matrix_rows": rows,
        "case_matrix": matrix,
        "categories": categories,
    }


def marker_value(row: dict[str, Any], marker: str) -> int:
    return int(row["internal"].get(marker, 0))


def parse_csv_set(value: str) -> set[str]:
    return {item.strip() for item in value.split(",") if item.strip()}


def row_problem_markers(row: dict[str, Any]) -> list[str]:
    problems = []
    for marker in ("TFAIL", "TBROK", "TCONF"):
        count = marker_value(row, marker)
        if count:
            problems.append(f"{marker}={count}")
    for key, label in (
        ("timeouts", "timeout"),
        ("enosys", "ENOSYS"),
        ("panic_trap", "panic/trap"),
        ("event_failures", "event-failures"),
    ):
        count = int(row.get(key, 0))
        if count:
            problems.append(f"{label}={count}")
    if row["status"] != "PASS":
        problems.append(f"status={row['status']}")
    return problems


def promotion_report(
    rows: list[dict[str, Any]], required_arches: set[str], required_libcs: set[str]
) -> dict[str, Any]:
    required_combos = {(arch, libc) for arch in required_arches for libc in required_libcs}
    by_case: dict[str, dict[tuple[str, str], list[dict[str, Any]]]] = {}
    for row in rows:
        by_case.setdefault(row["case"], {}).setdefault((row["arch"], row["libc"]), []).append(row)

    candidates = []
    blocked = []
    for case, combos in sorted(by_case.items()):
        missing = sorted(required_combos - set(combos))
        blockers = []
        for arch, libc in sorted(required_combos & set(combos)):
            for row in combos[(arch, libc)]:
                problems = row_problem_markers(row)
                if problems:
                    blockers.append(
                        {
                            "arch": arch,
                            "libc": libc,
                            "group": row["group"],
                            "reasons": problems,
                        }
                    )
        if missing or blockers:
            blocked.append(
                {
                    "case": case,
                    "missing": [
                        {"arch": arch, "libc": libc}
                        for arch, libc in missing
                    ],
                    "blockers": blockers,
                }
            )
            continue

        candidate_rows = [row for combo in sorted(required_combos) for row in combos[combo]]
        candidates.append(
            {
                "case": case,
                "combos": [
                    {"arch": row["arch"], "libc": row["libc"], "group": row["group"]}
                    for row in candidate_rows
                ],
                "max_runtime_ms": max(
                    (row["runtime_ms"] for row in candidate_rows if row["runtime_ms"] is not None),
                    default=None,
                ),
                "min_free_frames_delta_after_cleanup": min(
                    (
                        row["free_frames_delta_after_cleanup"]
                        for row in candidate_rows
                        if row["free_frames_delta_after_cleanup"] is not None
                    ),
                    default=None,
                ),
            }
        )

    return {
        "required_arches": sorted(required_arches),
        "required_libcs": sorted(required_libcs),
        "required_combo_count": len(required_combos),
        "candidate_count": len(candidates),
        "blocked_count": len(blocked),
        "candidates": candidates,
        "blocked": blocked,
    }


def promotion_rows(raw_summary: dict[str, Any], data: dict[str, Any], arch: str) -> list[dict[str, Any]]:
    event_failures = Counter(
        (event["case"], arch, infer_libc(event["group"]))
        for event in raw_summary["case_events"]
        if event["status"] != "PASS"
    )
    rows = []
    for row in data["case_matrix_rows"]:
        item = dict(row)
        item["event_failures"] = event_failures[(row["case"], row["arch"], row["libc"])]
        rows.append(item)
    return rows


def render_promotion_markdown(report: dict[str, Any], paths: list[Path]) -> str:
    lines = ["# LTP promotion-candidate report", ""]
    lines += [
        "- Inputs: " + ", ".join(f"`{path}`" for path in paths),
        "- Required arches: " + ", ".join(report["required_arches"]),
        "- Required libcs: " + ", ".join(report["required_libcs"]),
        f"- Required arch/libc combos: {report['required_combo_count']}",
        f"- Promotion candidates: {report['candidate_count']}",
        f"- Blocked/incomplete cases: {report['blocked_count']}",
        "",
    ]

    lines.append("## Candidates")
    if report["candidates"]:
        lines.append(
            "| Case | Clean combos | Max runtime ms | Min free-frames delta after cleanup |"
        )
        lines.append("| --- | --- | ---: | ---: |")
        for item in report["candidates"]:
            combos = ", ".join(
                f"{combo['arch']}:{combo['libc']}:{combo['group']}"
                for combo in item["combos"]
            )
            max_runtime = "" if item["max_runtime_ms"] is None else str(item["max_runtime_ms"])
            min_delta = (
                ""
                if item["min_free_frames_delta_after_cleanup"] is None
                else str(item["min_free_frames_delta_after_cleanup"])
            )
            lines.append(f"| {item['case']} | {combos} | {max_runtime} | {min_delta} |")
    else:
        lines.append("- None")
    lines.append("")

    lines.append("## Blocked or incomplete")
    if report["blocked"]:
        lines.append("| Case | Reason |")
        lines.append("| --- | --- |")
        for item in report["blocked"]:
            reasons = []
            if item["missing"]:
                reasons.append(
                    "missing "
                    + ", ".join(f"{miss['arch']}:{miss['libc']}" for miss in item["missing"])
                )
            for blocker in item["blockers"]:
                reasons.append(
                    f"{blocker['arch']}:{blocker['libc']}:{blocker['group']} "
                    + "/".join(blocker["reasons"])
                )
            lines.append(f"| {item['case']} | {'; '.join(reasons)} |")
    else:
        lines.append("- None")
    return "\n".join(lines).rstrip() + "\n"


def render_markdown(path: Path, data: dict[str, Any]) -> str:
    lines = [f"# LTP summary: `{path}`", ""]
    lines += [
        f"- PASS LTP CASE: {data['pass_count']}",
        f"- FAIL LTP CASE: {data['fail_count']}",
        f"- Internal TFAIL/TBROK/TCONF: {sum(data['internal'].values())} ({dict(data['internal'])})",
        f"- timeout matches: {data['timeouts']}",
        f"- ENOSYS/not implemented matches: {data['enosys']}",
        f"- panic/trap matches: {data['panic_trap']}",
        "",
    ]
    if data["suite_summaries"]:
        lines.append("## Suite summaries")
        for item in data["suite_summaries"]:
            lines.append(f"- {item['group']}: {item['passed']} passed, {item['failed']} failed")
        lines.append("")
    if data["case_matrix_rows"]:
        lines.append("## Case matrix")
        lines.append(
            "| Case | Arch | Libc | Group | Status | Code | Runtime ms | Free frames before | Free frames after cleanup | Free frames delta | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap |"
        )
        lines.append(
            "| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
        )
        for row in data["case_matrix_rows"]:
            code = "" if row["code"] is None else str(row["code"])
            runtime_ms = "" if row["runtime_ms"] is None else str(row["runtime_ms"])
            before = row["memory"].get("before", {})
            after_cleanup = row["memory"].get("after_cleanup", {})
            before_free = before.get("free_frames", "")
            after_cleanup_free = after_cleanup.get("free_frames", "")
            free_delta = (
                ""
                if row["free_frames_delta_after_cleanup"] is None
                else str(row["free_frames_delta_after_cleanup"])
            )
            lines.append(
                "| {case} | {arch} | {libc} | {group} | {status} | {code} | {runtime_ms} | {before_free} | {after_cleanup_free} | {free_delta} | {tfail} | {tbrok} | {tconf} | {timeout} | {enosys} | {panic} |".format(
                    case=row["case"],
                    arch=row["arch"],
                    libc=row["libc"],
                    group=row["group"],
                    status=row["status"],
                    code=code,
                    runtime_ms=runtime_ms,
                    before_free=before_free,
                    after_cleanup_free=after_cleanup_free,
                    free_delta=free_delta,
                    tfail=marker_value(row, "TFAIL"),
                    tbrok=marker_value(row, "TBROK"),
                    tconf=marker_value(row, "TCONF"),
                    timeout=row["timeouts"],
                    enosys=row["enosys"],
                    panic=row["panic_trap"],
                )
            )
        lines.append("")
    if data.get("categories"):
        lines.append("## Categories")
        for name, cases in data["categories"].items():
            lines.append(f"- {name}: {len(cases)}" + (f" ({', '.join(cases)})" if cases else ""))
        lines.append("")
    if data["fail_cases"]:
        lines.append("## FAIL LTP CASE")
        for case in data["fail_cases"]:
            lines.append(f"- {case}")
        lines.append("")
    lines.append("## Groups")
    for name, group in data["groups"].items():
        lines.append(f"### {name}")
        lines.append(f"- PASS: {group['pass_count']}")
        lines.append(f"- FAIL: {group['fail_count']}")
        lines.append(f"- Internal: {group['internal']}")
        lines.append(f"- timeout: {group['timeouts']}")
        lines.append(f"- ENOSYS/not implemented: {group['enosys']}")
        lines.append(f"- panic/trap: {group['panic_trap']}")
        if group["fail_cases"]:
            lines.append(f"- Fail cases: {', '.join(group['fail_cases'])}")
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log", type=Path, nargs="+", help="Evaluator output log/Markdown file")
    parser.add_argument("--json", action="store_true", help="Emit compact JSON instead of Markdown")
    parser.add_argument(
        "--promotion-candidates",
        action="store_true",
        help=(
            "Emit a clean-pass promotion-candidate report across the required arch/libc matrix. "
            "The normal one-log summary output is unchanged when this flag is not used."
        ),
    )
    parser.add_argument(
        "--promotion-arches",
        default="rv,la",
        help="Comma-separated required arches for --promotion-candidates (default: rv,la)",
    )
    parser.add_argument(
        "--promotion-libcs",
        default="musl,glibc",
        help="Comma-separated required libc variants for --promotion-candidates (default: musl,glibc)",
    )
    args = parser.parse_args()

    if not args.promotion_candidates and len(args.log) != 1:
        parser.error("multiple logs require --promotion-candidates")

    summaries = []
    for path in args.log:
        arch = infer_arch(path)
        raw_summary = parse_log(path.read_text(errors="ignore"))
        summaries.append((path, raw_summary, compact(raw_summary, arch), arch))

    if args.promotion_candidates:
        rows = [
            row
            for _path, raw_summary, data, arch in summaries
            for row in promotion_rows(raw_summary, data, arch)
        ]
        report = promotion_report(
            rows,
            parse_csv_set(args.promotion_arches),
            parse_csv_set(args.promotion_libcs),
        )
        if args.json:
            print(json.dumps(report, indent=2, sort_keys=True))
        else:
            print(render_promotion_markdown(report, [path for path, _raw, _data, _arch in summaries]), end="")
        return 0

    path, _raw_summary, data, _arch = summaries[0]
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        print(render_markdown(path, data), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
