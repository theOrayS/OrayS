#!/usr/bin/env python3
"""Summarize OSKernel evaluator LTP core output.

Counts wrapper-level LTP case PASS/FAIL lines and internal LTP quality signals
(TFAIL/TBROK/TCONF, timeouts, ENOSYS) so RUN_EVAL_DEFAULT_STATUS=0 is not
mistaken for a clean LTP result.
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
TIMEOUT_CASE_RE = re.compile(r"\bTIMEOUT LTP CASE\s+(\S+)(?:\s*:\s*(-?\d+))?", re.IGNORECASE)
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
SUITE_SUMMARY_RE = re.compile(r"ltp cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed")


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
            status, case, code = match.groups()
            record = {"group": current_group, "case": case, "code": int(code)}
            target = "pass_cases" if status == "PASS" else "fail_cases"
            summary[target].append(record)
            bucket(summary, current_group)[target].append(record)
            detail = case_bucket(summary, current_group, case)
            detail["status"] = status
            detail["code"] = int(code)
            summary["case_events"].append({"status": status, **record})
            current_case = case
            continue
        if match := TIMEOUT_CASE_RE.search(line):
            case, code = match.groups()
            detail = case_bucket(summary, current_group, case)
            detail["status"] = detail["status"] or "TIMEOUT"
            if code is not None:
                detail["code"] = int(code)
            current_case = case
        if match := SUITE_SUMMARY_RE.search(line):
            record = {
                "group": current_group,
                "passed": int(match.group(1)),
                "failed": int(match.group(2)),
            }
            summary["suite_summaries"].append(record)
            bucket(summary, current_group)["suite_summaries"].append(record)
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

    return summary


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
        }
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
            "| Case | Arch | Libc | Group | Status | Code | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap |"
        )
        lines.append("| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |")
        for row in data["case_matrix_rows"]:
            code = "" if row["code"] is None else str(row["code"])
            lines.append(
                "| {case} | {arch} | {libc} | {group} | {status} | {code} | {tfail} | {tbrok} | {tconf} | {timeout} | {enosys} | {panic} |".format(
                    case=row["case"],
                    arch=row["arch"],
                    libc=row["libc"],
                    group=row["group"],
                    status=row["status"],
                    code=code,
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
    parser.add_argument("log", type=Path, help="Evaluator output log/Markdown file")
    parser.add_argument("--json", action="store_true", help="Emit compact JSON instead of Markdown")
    args = parser.parse_args()

    text = args.log.read_text(errors="ignore")
    data = compact(parse_log(text), infer_arch(args.log))
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        print(render_markdown(args.log, data), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
