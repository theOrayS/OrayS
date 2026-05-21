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
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

ANSI_RE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
GROUP_START_RE = re.compile(r"#### OS COMP TEST GROUP START (.+?) ####")
GROUP_END_RE = re.compile(r"#### OS COMP TEST GROUP END (.+?) ####")
CASE_START_RE = re.compile(r"RUN LTP CASE\s+(\S+)")
CASE_RESULT_RE = re.compile(r"\b(PASS|FAIL) LTP CASE\s+(\S+)\s*:\s*(-?\d+)")
INTERNAL_RE = re.compile(r"\b(TFAIL|TBROK|TCONF)\b")
TIMEOUT_RE = re.compile(r"\b(timed out|timeout reached|timeout expired|killed after timeout)\b", re.IGNORECASE)
ENOSYS_RE = re.compile(r"\bENOSYS\b|errno=ENOSYS|not implemented", re.IGNORECASE)
SUITE_SUMMARY_RE = re.compile(r"ltp cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed")


def strip_ansi(text: str) -> str:
    return ANSI_RE.sub("", text)


def bucket(summary: dict[str, Any], group: str) -> dict[str, Any]:
    groups = summary.setdefault("groups", {})
    return groups.setdefault(
        group,
        {
            "pass_cases": [],
            "fail_cases": [],
            "internal": Counter(),
            "timeouts": 0,
            "enosys": 0,
            "suite_summaries": [],
        },
    )


def parse_log(text: str) -> dict[str, Any]:
    summary: dict[str, Any] = {
        "pass_cases": [],
        "fail_cases": [],
        "internal": Counter(),
        "timeouts": 0,
        "enosys": 0,
        "suite_summaries": [],
        "case_events": [],
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
            continue
        if match := CASE_RESULT_RE.search(line):
            status, case, code = match.groups()
            record = {"group": current_group, "case": case, "code": int(code)}
            target = "pass_cases" if status == "PASS" else "fail_cases"
            summary[target].append(record)
            bucket(summary, current_group)[target].append(record)
            summary["case_events"].append({"status": status, **record})
            continue
        if match := SUITE_SUMMARY_RE.search(line):
            record = {
                "group": current_group,
                "passed": int(match.group(1)),
                "failed": int(match.group(2)),
            }
            summary["suite_summaries"].append(record)
            bucket(summary, current_group)["suite_summaries"].append(record)
        for marker in INTERNAL_RE.findall(line):
            key = f"{current_group}:{current_case or '-'}:{marker}"
            summary["internal"][marker] += 1
            bucket(summary, current_group)["internal"][marker] += 1
        if "Timeout per run" not in line and TIMEOUT_RE.search(line):
            summary["timeouts"] += 1
            bucket(summary, current_group)["timeouts"] += 1
        if ENOSYS_RE.search(line):
            summary["enosys"] += 1
            bucket(summary, current_group)["enosys"] += 1

    return summary


def compact(summary: dict[str, Any]) -> dict[str, Any]:
    def compact_group(group: dict[str, Any]) -> dict[str, Any]:
        return {
            "pass_count": len(group["pass_cases"]),
            "fail_count": len(group["fail_cases"]),
            "pass_cases": [entry["case"] for entry in group["pass_cases"]],
            "fail_cases": [entry["case"] for entry in group["fail_cases"]],
            "internal": dict(group["internal"]),
            "timeouts": group["timeouts"],
            "enosys": group["enosys"],
            "suite_summaries": group["suite_summaries"],
        }

    return {
        "pass_count": len(summary["pass_cases"]),
        "fail_count": len(summary["fail_cases"]),
        "pass_cases": [f"{entry['group']}:{entry['case']}" for entry in summary["pass_cases"]],
        "fail_cases": [f"{entry['group']}:{entry['case']}" for entry in summary["fail_cases"]],
        "internal": dict(summary["internal"]),
        "timeouts": summary["timeouts"],
        "enosys": summary["enosys"],
        "suite_summaries": summary["suite_summaries"],
        "groups": {name: compact_group(group) for name, group in summary["groups"].items()},
    }


def render_markdown(path: Path, data: dict[str, Any]) -> str:
    lines = [f"# LTP summary: `{path}`", ""]
    lines += [
        f"- PASS LTP CASE: {data['pass_count']}",
        f"- FAIL LTP CASE: {data['fail_count']}",
        f"- Internal TFAIL/TBROK/TCONF: {sum(data['internal'].values())} ({dict(data['internal'])})",
        f"- timeout matches: {data['timeouts']}",
        f"- ENOSYS/not implemented matches: {data['enosys']}",
        "",
    ]
    if data["suite_summaries"]:
        lines.append("## Suite summaries")
        for item in data["suite_summaries"]:
            lines.append(f"- {item['group']}: {item['passed']} passed, {item['failed']} failed")
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
    data = compact(parse_log(text))
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        print(render_markdown(args.log, data), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
