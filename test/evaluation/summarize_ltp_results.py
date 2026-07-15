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
import hashlib
import importlib.util
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any

_PARSER_PATH = Path(__file__).resolve().with_name("parse_official_results.py")
_PARSER_SPEC = importlib.util.spec_from_file_location(
    "_orays_ltp_promotion_parser",
    _PARSER_PATH,
)
if _PARSER_SPEC is None or _PARSER_SPEC.loader is None:
    raise RuntimeError(f"cannot load canonical official parser: {_PARSER_PATH}")
_PARSER = importlib.util.module_from_spec(_PARSER_SPEC)
sys.modules[_PARSER_SPEC.name] = _PARSER
_PARSER_SPEC.loader.exec_module(_PARSER)
validate_ltp_output = _PARSER.validate_ltp_output
validate_capture_input_pairs = _PARSER.validate_capture_input_pairs
apply_process_exit_code = _PARSER.apply_process_exit_code

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
KNOWN_PROMOTION_ARCHES = frozenset({"rv", "la"})
KNOWN_PROMOTION_LIBCS = frozenset({"musl", "glibc"})
CANONICAL_BUILD_ARCH_RE = re.compile(
    r"App:\s*shell,\s*Arch:\s*(riscv64|loongarch64),\s*"
    r"Platform:\s*(riscv64-qemu-virt|loongarch64-qemu-virt),\s*App type:\s*rust"
)
CANONICAL_BUILD_ARCHES = {
    ("riscv64", "riscv64-qemu-virt"): "rv",
    ("loongarch64", "loongarch64-qemu-virt"): "la",
}
SUITE_SUMMARY_RE = re.compile(
    r"ltp cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed(?:,\s+(\d+)\s+timed out)?"
)


def strip_ansi(text: str) -> str:
    return ANSI_RE.sub("", text)


def infer_arch_tokens(path: Path) -> set[str]:
    return _PARSER.infer_capture_arch_tokens(path)


def infer_arch(path: Path) -> str:
    arches = infer_arch_tokens(path)
    return next(iter(arches)) if len(arches) == 1 else "unknown"


def capture_source_key(path: Path, stream: str) -> str:
    return _PARSER.capture_source_key(path, stream)


def validate_promotion_input_pairs(
    stdout_paths: list[Path],
    stderr_paths: list[Path],
    required_arches: set[str],
) -> list[dict[str, Any]]:
    if len(stdout_paths) != len(stderr_paths):
        raise ValueError(
            "--promotion-candidates requires exactly one --stderr-log for each stdout log"
        )
    return validate_capture_input_pairs(stdout_paths, stderr_paths, required_arches)


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


def strict_ltp_validation(text: str, stderr: str = "") -> dict[str, Any]:
    """Apply the authoritative official lifecycle parser to an LTP log.

    The historical summary view remains available for forensic reporting, but
    callers that need a pass/fail decision must opt into this complete-event
    contract.  Empty, partial, malformed, skipped, or marker-contaminated logs
    cannot pass it.
    """

    return validate_ltp_output(text, stderr)


def decode_log_bytes(raw: bytes) -> tuple[str, dict[str, Any] | None]:
    """Decode one captured log without ever discarding malformed input bytes."""

    try:
        return raw.decode("utf-8", errors="strict"), None
    except UnicodeDecodeError as error:
        finding = {
            "kind": "invalid-utf8",
            "message": (
                "captured log is not valid UTF-8 at byte offset "
                f"{error.start}: {error.reason}"
            ),
            "byte_offset": error.start,
            "byte_end": error.end,
            "reason": error.reason,
        }
        return raw.decode("utf-8", errors="replace"), finding


def decode_error_validation(findings: list[dict[str, Any]]) -> dict[str, Any]:
    """Represent a byte-decoding failure in the strict validator schema."""

    return {
        "status": "ERROR",
        "group_count": 0,
        "groups": [],
        "error_count": len(findings),
        "failure_count": 0,
        "errors": findings,
        "failures": [],
    }


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
            "zero_exit_record_count": len(group["pass_cases"]),
            "fail_count": len(group["fail_cases"]),
            "zero_exit_records": [entry["case"] for entry in group["pass_cases"]],
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
        row = {
            "case": detail["case"],
            "arch": arch,
            "libc": libc,
            "group": detail["group"],
            "status": (
                "ZERO_EXIT_RECORD"
                if detail["status"] == "PASS"
                else detail["status"] or "UNKNOWN"
            ),
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
        "zero_exit_without_detected_blocker": [],
        "zero_exit_with_tconf": [],
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
        if row["status"] == "ZERO_EXIT_RECORD" and not has_problem_marker:
            categories["zero_exit_without_detected_blocker"].append(label)
        has_only_tconf = (
            row["internal"].get("TCONF", 0)
            and not row["internal"].get("TFAIL", 0)
            and not row["internal"].get("TBROK", 0)
        )
        if row["status"] == "ZERO_EXIT_RECORD" and has_only_tconf:
            categories["zero_exit_with_tconf"].append(label)
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
        "validation_mode": "FORENSIC_UNVALIDATED",
        "zero_exit_record_count": len(summary["pass_cases"]),
        "fail_count": len(summary["fail_cases"]),
        "zero_exit_records": [
            f"{entry['group']}:{entry['case']}" for entry in summary["pass_cases"]
        ],
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
    }


def marker_value(row: dict[str, Any], marker: str) -> int:
    return int(row["internal"].get(marker, 0))


def parse_csv_set(value: str) -> set[str]:
    return {item.strip() for item in value.split(",") if item.strip()}


def validate_promotion_dimensions(
    required_arches: set[str], required_libcs: set[str]
) -> None:
    if not required_arches:
        raise ValueError("promotion arches must contain at least one known architecture")
    if not required_libcs:
        raise ValueError("promotion libcs must contain at least one known libc")
    if unknown_arches := sorted(required_arches - KNOWN_PROMOTION_ARCHES):
        raise ValueError(f"unknown promotion arches: {', '.join(unknown_arches)}")
    if unknown_libcs := sorted(required_libcs - KNOWN_PROMOTION_LIBCS):
        raise ValueError(f"unknown promotion libcs: {', '.join(unknown_libcs)}")
    if required_arches != KNOWN_PROMOTION_ARCHES:
        raise ValueError("promotion arches must be exactly rv,la")
    if required_libcs != KNOWN_PROMOTION_LIBCS:
        raise ValueError("promotion libcs must be exactly musl,glibc")


def row_problem_markers(row: dict[str, Any]) -> list[str]:
    problems = []
    if row.get("input_decode_error"):
        problems.append("input-decode-error")
    problems.extend(row.get("strict_validation_blockers", []))
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
    if row["status"] != "ZERO_EXIT_RECORD":
        problems.append(f"status={row['status']}")
    promotion_mode_blocker = row.get("promotion_mode_blocker")
    if promotion_mode_blocker:
        problems.append(promotion_mode_blocker)
    return problems


def promotion_mode_blocker(case_list: dict[str, Any] | None) -> str | None:
    if not case_list:
        return None
    mode = str(case_list.get("name") or "").strip()
    lowered = mode.lower()
    blocked_tokens = ("blacklist", "sweep:", "all-minus-blacklist")
    if lowered == "all" or any(token in lowered for token in blocked_tokens):
        return f"selection-mode={mode}"
    return None


def promotion_report(
    rows: list[dict[str, Any]],
    required_arches: set[str],
    required_libcs: set[str],
    input_validations: list[dict[str, Any]],
) -> dict[str, Any]:
    validate_promotion_dimensions(required_arches, required_libcs)
    if not input_validations:
        raise ValueError("promotion evidence requires input lifecycle validations")
    required_combos = {(arch, libc) for arch in required_arches for libc in required_libcs}
    input_blockers = [
        validation
        for validation in input_validations
        if validation.get("status") != "PASS"
    ]
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
        if missing or blockers or input_blockers:
            blocked.append(
                {
                    "case": case,
                    "missing": [
                        {"arch": arch, "libc": libc}
                        for arch, libc in missing
                    ],
                    "blockers": blockers,
                    "input_blockers": input_blockers,
                }
            )
            continue

        candidate_rows = [row for combo in sorted(required_combos) for row in combos[combo]]
        candidates.append(
            {
                "case": case,
                "combos": [
                    {
                        "arch": row["arch"],
                        "libc": row["libc"],
                        "group": row["group"],
                        "source_path": row["source_path"],
                        "stderr_path": row["stderr_path"],
                        "pair_id": row["pair_id"],
                        "stdout_sha256": row["stdout_sha256"],
                        "stderr_sha256": row["stderr_sha256"],
                        "strict_case_binding": row["strict_case_binding"],
                    }
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
        "input_blockers": input_blockers,
    }


def promotion_rows(
    raw_summary: dict[str, Any],
    data: dict[str, Any],
    arch: str,
    *,
    source_path: str,
    stderr_path: str,
    validation: dict[str, Any],
    pair_id: str | None = None,
    stdout_sha256: str | None = None,
    stderr_sha256: str | None = None,
) -> list[dict[str, Any]]:
    event_failures = Counter(
        (event["case"], arch, infer_libc(event["group"]))
        for event in raw_summary["case_events"]
        if event["status"] != "PASS"
    )
    strict_groups = {
        group["label"]: group for group in validation.get("groups", [])
    }
    validation_issue_kinds = [
        finding["kind"]
        for finding in [
            *validation.get("errors", []),
            *validation.get("failures", []),
        ]
    ]
    rows = []
    for row in data["case_matrix_rows"]:
        item = dict(row)
        item["source_path"] = source_path
        item["stderr_path"] = stderr_path
        item["pair_id"] = pair_id
        item["stdout_sha256"] = stdout_sha256
        item["stderr_sha256"] = stderr_sha256
        item["event_failures"] = event_failures[(row["case"], row["arch"], row["libc"])]
        item["promotion_mode_blocker"] = promotion_mode_blocker(row.get("case_list"))
        item["input_decode_error"] = bool(data.get("decode_error"))
        strict_records = [
            record
            for record in strict_groups.get(row["group"], {}).get("cases", [])
            if record.get("case") == row["case"]
        ]
        strict_blockers: list[str] = []
        expected_group = f"ltp-{row['libc']}"
        if row["group"] != expected_group:
            strict_blockers.append(f"noncanonical-ltp-group={row['group']}")
        if validation.get("status") != "PASS":
            strict_blockers.append(f"strict-validation={validation.get('status', 'ERROR')}")
            strict_blockers.extend(
                f"strict-{kind}" for kind in sorted(set(validation_issue_kinds))
            )
        if len(strict_records) != 1:
            strict_blockers.append(f"strict-case-binding-count={len(strict_records)}")
            strict_record = None
        else:
            strict_record = strict_records[0]
            if strict_record.get("code") != 0:
                strict_blockers.append(f"strict-case-code={strict_record.get('code')}")
            expected_events = ["START", "RUN", "RESULT", "PASS", "END"]
            if strict_record.get("events") != expected_events:
                strict_blockers.append(
                    "strict-case-events=" + ",".join(strict_record.get("events", []))
                )
        item["strict_case_binding"] = strict_record
        item["strict_validation_blockers"] = list(dict.fromkeys(strict_blockers))
        rows.append(item)
    return rows


def render_promotion_markdown(
    report: dict[str, Any], source_pairs: list[tuple[Path, Path]]
) -> str:
    lines = ["# LTP promotion-candidate report", ""]
    lines += [
        "- Inputs: "
        + ", ".join(
            f"stdout=`{stdout_path}`, stderr=`{stderr_path}`"
            for stdout_path, stderr_path in source_pairs
        ),
        "- Validation scope: LTP groups only; this is not a 24-group official-run verdict.",
        "- Candidate evidence requires exact START/RUN/result/Pass!/END lifecycle, "
        "manifest planned=executed, and an exact suite summary.",
        "- Required arches: " + ", ".join(report["required_arches"]),
        "- Required libcs: " + ", ".join(report["required_libcs"]),
        f"- Required arch/libc combos: {report['required_combo_count']}",
        f"- Promotion candidates: {report['candidate_count']}",
        f"- Blocked/incomplete cases: {report['blocked_count']}",
        "",
    ]
    if report.get("input_errors"):
        lines.append("## Input decoding errors")
        for finding in report["input_errors"]:
            lines.append(f"- `{finding['path']}`: {finding['message']}")
        lines.append("")

    if report.get("input_blockers"):
        lines.append("## Report-level input blockers")
        for blocker in report["input_blockers"]:
            lines.append(
                f"- stdout=`{blocker['path']}`, stderr=`{blocker['stderr_path']}` "
                f"({blocker['arch']}): {blocker['status']}"
            )
        lines.append("")

    if report.get("input_validations"):
        lines.append("## Input lifecycle validations")
        for validation in report["input_validations"]:
            lines.append(
                f"- stdout=`{validation['path']}`, stderr=`{validation['stderr_path']}` "
                f"({validation['arch']}): "
                f"{validation['status']}; groups={validation['group_count']}; "
                f"errors={validation['error_count']}; failures={validation['failure_count']}; "
                f"pair={validation['pair_id']}; "
                f"process_exit_code={validation['process_exit_code']}; "
                f"content_arch={validation.get('content_arch')}; "
                f"stdout_sha256={validation['stdout_sha256']}; "
                f"stderr_sha256={validation['stderr_sha256']}"
            )
            for finding_type in ("errors", "failures"):
                for finding in validation.get(finding_type, []):
                    context = []
                    if finding.get("group") is not None:
                        context.append(f"group={finding['group']}")
                    if finding.get("case") is not None:
                        context.append(f"case={finding['case']}")
                    suffix = f" ({', '.join(context)})" if context else ""
                    lines.append(
                        f"  - {finding_type[:-1]} `{finding.get('kind', 'unknown')}`"
                        f"{suffix}: {finding.get('message', 'no message supplied')}"
                    )
        lines.append("")

    lines.append("## Candidates")
    if report["candidates"]:
        lines.append(
            "| Case | Lifecycle-validated clean combos | Max runtime ms | Min free-frames delta after cleanup |"
        )
        lines.append("| --- | --- | ---: | ---: |")
        for item in report["candidates"]:
            combos = ", ".join(
                f"{combo['arch']}:{combo['libc']}:{combo['group']}"
                f"@stdout={combo['source_path']},stderr={combo['stderr_path']}"
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
            for input_blocker in item.get("input_blockers", []):
                reasons.append(
                    f"input {input_blocker['arch']}:{input_blocker['status']}"
                    f"@{input_blocker['path']}+{input_blocker['stderr_path']}"
                )
            lines.append(f"| {item['case']} | {'; '.join(reasons)} |")
    else:
        lines.append("- None")
    return "\n".join(lines).rstrip() + "\n"


def render_markdown(path: Path, data: dict[str, Any]) -> str:
    lines = [f"# LTP summary: `{path}`", ""]
    lines += [
        "- Verdict: UNVALIDATED FORENSIC VIEW unless the strict integrity section below is present",
        f"- Zero-exit wrapper records (not PASS evidence by themselves): {data['zero_exit_record_count']}",
        f"- Wrapper FAIL (nonzero/timeout): {data['fail_count']}",
        f"- Internal TFAIL/TBROK/TCONF: {sum(data['internal'].values())} ({dict(data['internal'])})",
        f"- timeout matches: {data['timeouts']}",
        f"- ENOSYS/not implemented matches: {data['enosys']}",
        f"- panic/trap matches: {data['panic_trap']}",
        "",
    ]
    if "decode_error" in data:
        lines += [
            "## Input decoding error",
            "- Status: ERROR",
            f"- Detail: {data['decode_error']['message']}",
            "- The forensic counts below use replacement characters; they are not PASS evidence.",
            "",
        ]
    if "strict_validation" in data:
        validation = data["strict_validation"]
        lines += [
            "## Strict integrity verdict",
            f"- Status: {validation['status']}",
            f"- Groups: {validation['group_count']}",
            f"- Integrity errors: {validation['error_count']}",
            f"- Explicit failures: {validation['failure_count']}",
        ]
        for finding_type in ("errors", "failures"):
            for finding in validation.get(finding_type, []):
                context = []
                if finding.get("group") is not None:
                    context.append(f"group={finding['group']}")
                if finding.get("case") is not None:
                    context.append(f"case={finding['case']}")
                suffix = f" ({', '.join(context)})" if context else ""
                lines.append(
                    f"- {finding_type[:-1]} `{finding.get('kind', 'unknown')}`"
                    f"{suffix}: {finding.get('message', 'no message supplied')}"
                )
        lines.append("")
    if "input_provenance" in data:
        evidence = data["input_provenance"]
        lines += [
            "## Input provenance",
            f"- stdout: `{evidence['stdout_path']}`",
            f"- stdout SHA-256: `{evidence['stdout_sha256']}`",
            f"- stderr: `{evidence['stderr_path']}`",
            f"- stderr SHA-256: `{evidence['stderr_sha256']}`",
            f"- evaluator process exit code: `{evidence['process_exit_code']}`",
            "",
        ]
    if data["case_list_manifests"]:
        lines.append("## Case-list manifests")
        for item in data["case_list_manifests"]:
            lines.append(
                "- {group}: `{name}` ({case_count} cases, timeout {timeout_secs}s)".format(
                    **item
                )
            )
        lines.append("")
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
        lines.append(
            f"- Zero-exit records (unvalidated alone): {group['zero_exit_record_count']}"
        )
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
        "--strict",
        action="store_true",
        help="validate complete group/case lifecycle and return nonzero for any non-pass result",
    )
    parser.add_argument(
        "--promotion-candidates",
        action="store_true",
        help=(
            "Emit a clean-pass promotion-candidate report across the required arch/libc matrix. "
            "LTP-scoped lifecycle validation is mandatory in this mode; this is not a "
            "24-group official verdict. The normal one-log summary output is unchanged "
            "when this flag is not used."
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
        "--stderr-log",
        action="append",
        type=Path,
        default=[],
        help=(
            "Captured stderr companion for the corresponding positional stdout log. "
            "Any supplied companion activates strict validation; promotion mode requires "
            "exactly one companion per input so stderr failures cannot be omitted."
        ),
    )
    parser.add_argument(
        "--process-exit-code",
        action="append",
        type=int,
        default=[],
        help=(
            "actual evaluator process return code for the corresponding capture pair; "
            "required once per input whenever strict or promotion validation is active"
        ),
    )
    args = parser.parse_args()

    if not args.promotion_candidates and len(args.log) != 1:
        parser.error("multiple logs require --promotion-candidates")
    if args.strict and not args.promotion_candidates and len(args.stderr_log) != 1:
        parser.error("--strict requires exactly one --stderr-log companion")
    if (
        not args.promotion_candidates
        and not args.strict
        and args.stderr_log
        and len(args.stderr_log) != 1
    ):
        parser.error("a single input accepts at most one --stderr-log")
    needs_process_evidence = bool(
        args.strict or args.promotion_candidates or args.stderr_log
    )
    if needs_process_evidence and len(args.process_exit_code) != len(args.log):
        parser.error(
            "strict capture validation requires exactly one --process-exit-code per input"
        )
    if not needs_process_evidence and args.process_exit_code:
        parser.error("--process-exit-code requires paired strict capture validation")

    required_arches = parse_csv_set(args.promotion_arches)
    required_libcs = parse_csv_set(args.promotion_libcs)
    capture_pairs: list[dict[str, Any]] = []
    if args.promotion_candidates:
        try:
            validate_promotion_dimensions(required_arches, required_libcs)
            capture_pairs = validate_promotion_input_pairs(
                args.log,
                args.stderr_log,
                required_arches,
            )
        except ValueError as error:
            parser.error(str(error))
    elif args.stderr_log:
        try:
            capture_pairs = validate_capture_input_pairs(args.log, args.stderr_log)
        except ValueError as error:
            parser.error(str(error))

    summaries = []
    strict_statuses = []
    decode_errors = []
    for index, path in enumerate(args.log):
        pair = capture_pairs[index] if capture_pairs else None
        arch = (pair["arch"] if pair is not None else None) or infer_arch(path)
        stderr_path = pair["stderr_path"] if pair is not None else (
            args.stderr_log[index] if args.stderr_log else None
        )
        try:
            raw = path.read_bytes()
        except OSError as error:
            print(f"input error: cannot read {path}: {error}", file=sys.stderr)
            return 2
        try:
            stderr_raw = stderr_path.read_bytes() if stderr_path is not None else None
        except OSError as error:
            print(f"input error: cannot read {stderr_path}: {error}", file=sys.stderr)
            return 2
        text, decode_error = decode_log_bytes(raw)
        stderr_text, stderr_decode_error = (
            decode_log_bytes(stderr_raw) if stderr_raw is not None else ("", None)
        )
        raw_summary = parse_log(text)
        data = compact(raw_summary, arch)
        input_decode_errors = []
        if decode_error is not None:
            finding = {"path": str(path), "stream": "stdout", **decode_error}
            input_decode_errors.append(finding)
            decode_errors.append(finding)
        if stderr_decode_error is not None:
            finding = {
                "path": str(stderr_path),
                "stream": "stderr",
                **stderr_decode_error,
            }
            input_decode_errors.append(finding)
            decode_errors.append(finding)
        if input_decode_errors:
            data["decode_error"] = input_decode_errors[0]
            data["decode_errors"] = input_decode_errors
        if args.strict or args.promotion_candidates or stderr_path is not None:
            validation = (
                decode_error_validation(input_decode_errors)
                if input_decode_errors
                else strict_ltp_validation(text, stderr_text)
            )
            process_exit_code = args.process_exit_code[index]
            apply_process_exit_code(validation, process_exit_code)
            data["strict_validation"] = validation
            data["validation_mode"] = "STRICT_LTP"
            strict_statuses.append(validation["status"])
        provenance = {
            "pair_id": pair["pair_id"] if pair is not None else None,
            "source_key": pair["source_key"] if pair is not None else None,
            "stdout_sha256": hashlib.sha256(raw).hexdigest(),
            "stderr_sha256": (
                hashlib.sha256(stderr_raw).hexdigest() if stderr_raw is not None else None
            ),
            "stdout_size_bytes": len(raw),
            "stderr_size_bytes": len(stderr_raw) if stderr_raw is not None else None,
            "stdout_path": str(path),
            "stderr_path": str(stderr_path) if stderr_path is not None else None,
            "process_exit_code": (
                args.process_exit_code[index] if needs_process_evidence else None
            ),
            "content_arches": [
                CANONICAL_BUILD_ARCHES.get((build_arch, platform))
                for build_arch, platform in CANONICAL_BUILD_ARCH_RE.findall(
                    strip_ansi(text)
                )
            ],
            "stdout_resolved_path": (
                str(pair["stdout_resolved_path"])
                if pair is not None
                else str(path.resolve())
            ),
            "stderr_resolved_path": (
                str(pair["stderr_resolved_path"]) if pair is not None else None
            ),
        }
        data["input_provenance"] = provenance
        summaries.append((path, stderr_path, raw_summary, data, arch, provenance))

    if args.promotion_candidates:
        provenance_errors: list[dict[str, Any]] = []
        for path, _stderr, _raw_summary, data, arch, provenance in summaries:
            content_arches = provenance["content_arches"]
            provenance["content_arch"] = (
                content_arches[0] if len(content_arches) == 1 else None
            )
            if len(content_arches) == 1 and content_arches[0] == arch:
                continue
            finding = {
                "kind": "capture-architecture-provenance",
                "message": (
                    "promotion stdout must contain exactly one canonical shell build "
                    f"marker for filename architecture {arch}; observed {content_arches}"
                ),
                "path": str(path),
                "arch": arch,
                "observed_content_arches": content_arches,
            }
            provenance_errors.append(finding)
            validation = data["strict_validation"]
            validation["errors"].append(finding)
            validation["error_count"] = len(validation["errors"])
            validation["status"] = "ERROR"
        digest_arches: dict[str, set[str]] = {}
        for _path, _stderr, _raw, _data, arch, provenance in summaries:
            digest_arches.setdefault(provenance["stdout_sha256"], set()).add(arch)
        duplicated_digests = {
            digest: arches for digest, arches in digest_arches.items() if len(arches) > 1
        }
        for path, _stderr, _raw, data, arch, provenance in summaries:
            arches = duplicated_digests.get(provenance["stdout_sha256"])
            if arches is None:
                continue
            finding = {
                "kind": "cross-arch-identical-stdout-digest",
                "message": (
                    "different architecture inputs have identical raw stdout SHA-256; "
                    "the capture provenance is ambiguous"
                ),
                "path": str(path),
                "arch": arch,
                "sha256": provenance["stdout_sha256"],
                "architectures": sorted(arches),
            }
            provenance_errors.append(finding)
            validation = data["strict_validation"]
            validation["errors"].append(finding)
            validation["error_count"] = len(validation["errors"])
            validation["status"] = "ERROR"
        strict_statuses = [
            data["strict_validation"]["status"]
            for _path, _stderr, _raw, data, _arch, _provenance in summaries
        ]
        rows = [
            row
            for path, stderr_path, raw_summary, data, arch, provenance in summaries
            for row in promotion_rows(
                raw_summary,
                data,
                arch,
                source_path=str(path),
                stderr_path=str(stderr_path),
                validation=data["strict_validation"],
                pair_id=provenance["pair_id"],
                stdout_sha256=provenance["stdout_sha256"],
                stderr_sha256=provenance["stderr_sha256"],
            )
        ]
        input_validations = [
            {
                "path": str(path),
                "stderr_path": str(stderr_path),
                "arch": arch,
                **provenance,
                "status": data["strict_validation"]["status"],
                "group_count": data["strict_validation"]["group_count"],
                "error_count": data["strict_validation"]["error_count"],
                "failure_count": data["strict_validation"]["failure_count"],
                "errors": list(data["strict_validation"].get("errors", [])),
                "failures": list(data["strict_validation"].get("failures", [])),
            }
            for path, stderr_path, _raw_summary, data, arch, provenance in summaries
        ]
        report = promotion_report(
            rows,
            required_arches,
            required_libcs,
            input_validations,
        )
        if decode_errors or provenance_errors:
            report["input_errors"] = [*decode_errors, *provenance_errors]
        report["validation_scope"] = "ltp"
        report["input_validations"] = input_validations
        report["input_pairs"] = [
            {
                "pair_id": validation["pair_id"],
                "source_key": validation["source_key"],
                "arch": validation["arch"],
                "stdout_path": validation["path"],
                "stderr_path": validation["stderr_path"],
                "stdout_sha256": validation["stdout_sha256"],
                "stderr_sha256": validation["stderr_sha256"],
                "stdout_size_bytes": validation["stdout_size_bytes"],
                "stderr_size_bytes": validation["stderr_size_bytes"],
                "stdout_resolved_path": validation["stdout_resolved_path"],
                "stderr_resolved_path": validation["stderr_resolved_path"],
                "process_exit_code": validation["process_exit_code"],
                "content_arch": validation.get("content_arch"),
            }
            for validation in input_validations
        ]
        if args.json:
            print(json.dumps(report, indent=2, sort_keys=True))
        else:
            print(
                render_promotion_markdown(
                    report,
                    [
                        (path, stderr_path)
                        for path, stderr_path, _raw, _data, _arch, _provenance in summaries
                    ],
                ),
                end="",
            )
        if decode_errors or "ERROR" in strict_statuses:
            return 2
        if "FAIL" in strict_statuses:
            return 1
        return 0

    path, _stderr_path, _raw_summary, data, _arch, _provenance = summaries[0]
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        print(render_markdown(path, data), end="")
    if decode_errors or "ERROR" in strict_statuses:
        return 2
    if "FAIL" in strict_statuses:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
