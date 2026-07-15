#!/usr/bin/env python3
"""Summarize OSKernel evaluator LTP core output.

Counts wrapper-level LTP case result lines and internal LTP quality signals
(TFAIL/TBROK/TCONF, timeouts, ENOSYS) so RUN_EVAL_DEFAULT_STATUS=0 is not
mistaken for a clean LTP result.  Official OSKernel judge scripts use the
legacy wrapper record `FAIL LTP CASE <case> : <code>` for every completed case,
including zero-exit successes.  The parser also accepts intermediate
`PASS LTP CASE <case> : 0` logs, but keeps the numeric status as the source of
truth for wrapper pass/fail classification.
"""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any

from evaluator_protocol import parse_evaluator_bytes


MAX_EVALUATOR_LOG_BYTES = 64 * 1024 * 1024

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
CASE_LIST_RE = re.compile(
    r"\bltp case list:\s+(.+?)\s+\((\d+)\s+cases,\s+timeout\s+(\d+)s\)",
    re.IGNORECASE,
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


def markdown_escape(value: object) -> str:
    """Render untrusted evaluator fields as a single Markdown text fragment."""

    return (
        str(value)
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\\", "\\\\")
        .replace("|", "\\|")
        .replace("`", "\\`")
        .replace("\r", " ")
        .replace("\n", " ")
    )


def stable_path_labels(paths: list[Path]) -> list[str]:
    """Return deterministic report labels without exposing parent directories."""

    names = [path.name or "input" for path in paths]
    if len(set(names)) == len(names):
        return names
    # Prefix every label when any basename repeats.  Indexing only the repeated
    # basename could collide with a real basename such as ``evaluator.log#1``.
    return [f"input-{index}:{name}" for index, name in enumerate(names, 1)]


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
        "case_list": None,
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

    Current official-compatible OSKernel logs use `FAIL LTP CASE <case> : 0`
    for zero-exit cases because the official judge finalizes cases on that
    wrapper record.  Intermediate logs may contain `PASS LTP CASE <case> : 0`;
    keep accepting them so historical evidence stays parseable.  The numeric
    exit status is the source of truth: only status 0 is PASS, and every
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
        "case_list_manifests": [],
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
        if match := CASE_LIST_RE.search(line):
            name, count, timeout_secs = match.groups()
            manifest = {
                "group": current_group,
                "name": name,
                "case_count": int(count),
                "timeout_secs": int(timeout_secs),
            }
            bucket(summary, current_group)["case_list"] = manifest
            summary["case_list_manifests"].append(manifest)
            current_case = None
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


def parse_log_bytes(data: bytes) -> dict[str, Any]:
    """Parse raw evaluator bytes and attach the strict shared protocol result.

    The legacy counters remain available for compatibility, but all gate and
    promotion decisions use ``protocol``.  latin-1 is a lossless one-byte
    adapter for the legacy display parser; unlike ``errors=ignore`` it cannot
    delete a byte and turn a malformed status into zero.
    """

    protocol = parse_evaluator_bytes(data)
    try:
        summary = parse_log(data.decode("latin-1"))
    except (ValueError, OverflowError, MemoryError) as exc:
        summary = parse_log("")
        protocol["diagnostics"].append(
            {
                "code": "legacy_numeric_field_error",
                "line": 0,
                "detail": f"legacy compatibility counters rejected numeric field: {exc}",
            }
        )
        protocol["state"] = "error"
    summary["protocol"] = protocol
    return summary


def read_log_bytes(path: Path) -> bytes:
    if not path.is_file():
        raise ValueError(f"not a regular evaluator log: {path}")
    size = path.stat().st_size
    if size > MAX_EVALUATOR_LOG_BYTES:
        raise ValueError(
            f"evaluator log exceeds limit ({size} > {MAX_EVALUATOR_LOG_BYTES} bytes): {path}"
        )
    try:
        return path.read_bytes()
    except MemoryError as exc:
        raise ValueError(f"cannot allocate memory for evaluator log: {path}") from exc


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
        if not detail["timeouts"]:
            continue
        group = detail["group"]
        case = detail["case"]
        detail["status"] = "TIMEOUT"
        summary["pass_cases"] = remove_case_records(summary["pass_cases"], group, case)
        if group in summary["groups"]:
            summary["groups"][group]["pass_cases"] = remove_case_records(
                summary["groups"][group]["pass_cases"], group, case
            )


def compact(summary: dict[str, Any], arch: str = "unknown") -> dict[str, Any]:
    protocol = summary.get("protocol")
    protocol_cases = (
        {(item["group"], item["case"]): item for item in protocol["cases"]}
        if protocol is not None
        else {}
    )

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
            "case_list": group.get("case_list"),
        }

    rows = []
    matrix: dict[str, dict[str, dict[str, dict[str, Any]]]] = {}
    for detail in sorted(
        summary["case_details"].values(), key=lambda item: (item["case"], item["group"])
    ):
        libc = infer_libc(detail["group"])
        protocol_case = protocol_cases.get((detail["group"], detail["case"]))
        protocol_status = protocol_case["state"] if protocol_case is not None else None
        semantic_status = (
            {
                "pass": "PASS",
                "fail": "FAIL",
                "error": "ERROR",
                "timeout": "TIMEOUT",
                "skipped": "SKIPPED",
                "blocked": "BLOCKED",
            }[protocol_status]
            if protocol_status is not None
            else detail["status"] or "UNKNOWN"
        )
        row = {
            "case": detail["case"],
            "arch": arch,
            "libc": libc,
            "group": detail["group"],
            "status": semantic_status,
            "protocol_state": protocol_status,
            "protocol_signals": protocol_case["signals"] if protocol_case is not None else None,
            "code": detail["code"],
            "internal": dict(detail["internal"]),
            "timeouts": detail["timeouts"],
            "enosys": detail["enosys"],
            "panic_trap": detail["panic_trap"],
            "runtime_ms": detail["runtime_ms"],
            "memory": detail["memory"],
            "case_list": summary["groups"].get(detail["group"], {}).get("case_list"),
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
        "case_list_manifests": summary["case_list_manifests"],
        "groups": {name: compact_group(group) for name, group in summary["groups"].items()},
        "case_matrix_rows": rows,
        "case_matrix": matrix,
        "categories": categories,
        "protocol": (
            None
            if protocol is None
            else {
                "schema_version": protocol["schema_version"],
                "raw_sha256": protocol["raw_sha256"],
                "size_bytes": protocol["size_bytes"],
                "state": protocol["state"],
                "diagnostics": protocol["diagnostics"],
                "global_signals": protocol["global_signals"],
            }
        ),
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
    protocol_overall_state = row.get("protocol_overall_state")
    if protocol_overall_state not in (None, "pass"):
        problems.append(f"protocol-overall={protocol_overall_state}")
    protocol_diagnostics = int(row.get("protocol_diagnostics", 0))
    if protocol_diagnostics:
        problems.append(f"protocol-diagnostics={protocol_diagnostics}")
    promotion_mode_blocker = row.get("promotion_mode_blocker")
    if promotion_mode_blocker:
        problems.append(promotion_mode_blocker)
    return problems


def promotion_mode_blocker(case_list: dict[str, Any] | None) -> str | None:
    if not case_list:
        return "missing-case-list"
    mode = str(case_list.get("name") or "").strip()
    lowered = mode.lower()
    blocked_tokens = ("blacklist", "sweep:", "all-minus-blacklist")
    if lowered == "all" or any(token in lowered for token in blocked_tokens):
        return f"selection-mode={mode}"
    return None


def promotion_report(
    rows: list[dict[str, Any]], required_arches: set[str], required_libcs: set[str]
) -> dict[str, Any]:
    if not required_arches or not required_libcs:
        raise ValueError("promotion arches and libcs must both be non-empty")
    unknown_arches = required_arches - {"rv", "la"}
    unknown_libcs = required_libcs - {"musl", "glibc"}
    if unknown_arches or unknown_libcs:
        raise ValueError(
            "unsupported promotion axes: "
            + ", ".join(sorted(unknown_arches | unknown_libcs))
        )
    required_combos = {(arch, libc) for arch in required_arches for libc in required_libcs}
    by_case: dict[str, dict[tuple[str, str], list[dict[str, Any]]]] = {}
    for row in rows:
        by_case.setdefault(row["case"], {}).setdefault((row["arch"], row["libc"]), []).append(row)

    candidates = []
    blocked = []
    if not by_case:
        blocked.append(
            {
                "case": "<no-cases-observed>",
                "missing": [
                    {"arch": arch, "libc": libc}
                    for arch, libc in sorted(required_combos)
                ],
                "blockers": [],
            }
        )
    for case, combos in sorted(by_case.items()):
        missing = sorted(required_combos - set(combos))
        blockers = []
        for arch, libc in sorted(required_combos & set(combos)):
            if len(combos[(arch, libc)]) != 1:
                blockers.append(
                    {
                        "arch": arch,
                        "libc": libc,
                        "group": ",".join(sorted(row["group"] for row in combos[(arch, libc)])),
                        "reasons": [f"duplicate-combo={len(combos[(arch, libc)])}"],
                    }
                )
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
    protocol = raw_summary.get("protocol")
    for row in data["case_matrix_rows"]:
        item = dict(row)
        item["event_failures"] = event_failures[(row["case"], row["arch"], row["libc"])]
        item["promotion_mode_blocker"] = promotion_mode_blocker(row.get("case_list"))
        item["protocol_overall_state"] = protocol["state"] if protocol is not None else "error"
        item["protocol_diagnostics"] = (
            len(protocol["diagnostics"]) if protocol is not None else 1
        )
        rows.append(item)
    return rows


def render_promotion_markdown(report: dict[str, Any], input_labels: list[str]) -> str:
    lines = ["# LTP promotion-candidate report", ""]
    lines += [
        "- Inputs: " + ", ".join(markdown_escape(label) for label in input_labels),
        "- Required arches: "
        + ", ".join(markdown_escape(item) for item in report["required_arches"]),
        "- Required libcs: "
        + ", ".join(markdown_escape(item) for item in report["required_libcs"]),
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
                markdown_escape(
                    f"{combo['arch']}:{combo['libc']}:{combo['group']}"
                )
                for combo in item["combos"]
            )
            max_runtime = "" if item["max_runtime_ms"] is None else str(item["max_runtime_ms"])
            min_delta = (
                ""
                if item["min_free_frames_delta_after_cleanup"] is None
                else str(item["min_free_frames_delta_after_cleanup"])
            )
            lines.append(
                f"| {markdown_escape(item['case'])} | {combos} | "
                f"{max_runtime} | {min_delta} |"
            )
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
            lines.append(
                f"| {markdown_escape(item['case'])} | "
                f"{markdown_escape('; '.join(reasons))} |"
            )
    else:
        lines.append("- None")
    return "\n".join(lines).rstrip() + "\n"


def render_markdown(input_label: str, data: dict[str, Any]) -> str:
    lines = [f"# LTP summary: {markdown_escape(input_label)}", ""]
    if data.get("protocol") is not None:
        protocol = data["protocol"]
        lines += [
            f"- Overall protocol state: **{protocol['state'].upper()}**",
            f"- Protocol diagnostics: {len(protocol['diagnostics'])}",
            f"- Raw SHA-256: `{protocol['raw_sha256']}`",
        ]
    lines += [
        f"- Wrapper PASS (code 0): {data['pass_count']}",
        f"- Wrapper FAIL (nonzero/timeout): {data['fail_count']}",
        f"- Internal TFAIL/TBROK/TCONF: {sum(data['internal'].values())} ({dict(data['internal'])})",
        f"- timeout matches: {data['timeouts']}",
        f"- ENOSYS/not implemented matches: {data['enosys']}",
        f"- panic/trap matches: {data['panic_trap']}",
        "",
    ]
    if data["case_list_manifests"]:
        lines.append("## Case-list manifests")
        for item in data["case_list_manifests"]:
            lines.append(
                "- {group}: {name} ({case_count} cases, timeout {timeout_secs}s)".format(
                    group=markdown_escape(item["group"]),
                    name=markdown_escape(item["name"]),
                    case_count=item["case_count"],
                    timeout_secs=item["timeout_secs"],
                )
            )
        lines.append("")
    if data["suite_summaries"]:
        lines.append("## Suite summaries")
        for item in data["suite_summaries"]:
            lines.append(
                f"- {markdown_escape(item['group'])}: {item['passed']} passed, "
                f"{item['failed']} failed"
            )
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
                    case=markdown_escape(row["case"]),
                    arch=markdown_escape(row["arch"]),
                    libc=markdown_escape(row["libc"]),
                    group=markdown_escape(row["group"]),
                    status=markdown_escape(row["status"]),
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
            escaped_cases = ", ".join(markdown_escape(case) for case in cases)
            lines.append(
                f"- {markdown_escape(name)}: {len(cases)}"
                + (f" ({escaped_cases})" if cases else "")
            )
        lines.append("")
    if data["fail_cases"]:
        lines.append("## FAIL LTP CASE")
        for case in data["fail_cases"]:
            lines.append(f"- {markdown_escape(case)}")
        lines.append("")
    lines.append("## Groups")
    for name, group in data["groups"].items():
        lines.append(f"### {markdown_escape(name)}")
        lines.append(f"- PASS: {group['pass_count']}")
        lines.append(f"- FAIL: {group['fail_count']}")
        lines.append(f"- Internal: {group['internal']}")
        lines.append(f"- timeout: {group['timeouts']}")
        lines.append(f"- ENOSYS/not implemented: {group['enosys']}")
        lines.append(f"- panic/trap: {group['panic_trap']}")
        if group["fail_cases"]:
            lines.append(
                "- Fail cases: "
                + ", ".join(markdown_escape(case) for case in group["fail_cases"])
            )
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
    parser.add_argument(
        "--require-clean",
        action="store_true",
        help="Return nonzero unless every input is structurally valid and semantically pass",
    )
    args = parser.parse_args()

    if not args.promotion_candidates and len(args.log) != 1:
        parser.error("multiple logs require --promotion-candidates")

    summaries = []
    for path in args.log:
        arch = infer_arch(path)
        try:
            raw_summary = parse_log_bytes(read_log_bytes(path))
        except (OSError, ValueError, MemoryError) as exc:
            parser.error(f"cannot read {path}: {exc}")
        summaries.append((path, raw_summary, compact(raw_summary, arch), arch))
    input_labels = stable_path_labels([path for path, _raw, _data, _arch in summaries])

    protocol_results = [raw_summary["protocol"] for _path, raw_summary, _data, _arch in summaries]
    has_integrity_error = any(result["diagnostics"] for result in protocol_results)
    has_nonpass = any(result["state"] != "pass" for result in protocol_results)

    if args.promotion_candidates:
        provenance_errors = []
        raw_arches: dict[str, list[tuple[str, str]]] = {}
        for index, (_path, raw_summary, _data, arch) in enumerate(summaries):
            raw_sha256 = raw_summary["protocol"]["raw_sha256"]
            raw_arches.setdefault(raw_sha256, []).append((arch, input_labels[index]))
        for raw_sha256, bindings in sorted(raw_arches.items()):
            arches = {arch for arch, _label in bindings}
            if len(arches) > 1:
                provenance_errors.append(
                    "identical raw evaluator bytes were assigned to different architectures "
                    f"({raw_sha256}: "
                    + ", ".join(f"{arch}={label}" for arch, label in bindings)
                    + ")"
                )
        rows = [
            row
            for _path, raw_summary, data, arch in summaries
            for row in promotion_rows(raw_summary, data, arch)
        ]
        try:
            report = promotion_report(
                rows,
                parse_csv_set(args.promotion_arches),
                parse_csv_set(args.promotion_libcs),
            )
        except ValueError as exc:
            parser.error(str(exc))
        if provenance_errors:
            report["candidates"] = []
            report["candidate_count"] = 0
            report["blocked"].append(
                {
                    "case": "<architecture-provenance>",
                    "missing": [],
                    "blockers": [
                        {
                            "arch": "cross-arch",
                            "libc": "all",
                            "group": "raw-log-binding",
                            "reasons": provenance_errors,
                        }
                    ],
                }
            )
            report["blocked_count"] = len(report["blocked"])
            has_integrity_error = True
        if args.json:
            print(json.dumps(report, indent=2, sort_keys=True))
        else:
            print(render_promotion_markdown(report, input_labels), end="")
        if has_integrity_error:
            return 2
        promotion_nonpass = has_nonpass or bool(report["blocked_count"])
        return 1 if args.require_clean and promotion_nonpass else 0

    path, _raw_summary, data, _arch = summaries[0]
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        print(render_markdown(input_labels[0], data), end="")
    if has_integrity_error:
        return 2
    return 1 if args.require_clean and has_nonpass else 0


if __name__ == "__main__":
    raise SystemExit(main())
