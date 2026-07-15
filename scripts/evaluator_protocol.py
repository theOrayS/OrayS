#!/usr/bin/env python3
"""Fail-closed parser for the repository's evaluator/LTP wire protocol.

The parser consumes bytes.  It never drops or replacement-decodes bytes in a
control record, and it separates protocol integrity from semantic outcomes.
"""

from __future__ import annotations

import hashlib
import re
from typing import Any


PROTOCOL_SCHEMA_VERSION = 1
STATES = ("pass", "fail", "error", "timeout", "blocked", "skipped")

SAFE_TERMINAL_CONTROL_RE = re.compile(
    rb"\x1b\[[0-9;:]*m"  # SGR has no text payload; every other escape is ambiguous.
)
GROUP_START_RE = re.compile(rb"^\s*#### OS COMP TEST GROUP START ([A-Za-z0-9._-]+) ####\s*$")
GROUP_END_RE = re.compile(rb"^\s*#### OS COMP TEST GROUP END ([A-Za-z0-9._-]+) ####\s*$")
CASE_LIST_RE = re.compile(
    rb"^\s*ltp case list:\s+([ -~]+?)\s+\((\d{1,10})\s+cases,\s+timeout\s+(\d{1,10})s\)\s*$",
    re.I,
)
CASE_START_RE = re.compile(rb"^\s*RUN LTP CASE\s+([!-9;-~]+)\s*$")
CASE_RESULT_RE = re.compile(
    rb"^\s*(PASS|FAIL) LTP CASE\s+([!-9;-~]+)\s*:\s*(-?\d{1,10})\s*$"
)
CASE_TIMEOUT_RE = re.compile(
    rb"^\s*TIMEOUT LTP CASE\s+([!-9;-~]+)(?:\s+(?:after\s+)?(\d{1,10})s|\s*:\s*(-?\d{1,10}))?\s*$",
    re.I,
)
SUITE_SUMMARY_RE = re.compile(
    rb"^\s*ltp cases:\s+(\d{1,10})\s+passed,\s+(\d{1,10})\s+failed(?:,\s+(\d{1,10})\s+timed out)?\s*$",
    re.I,
)
SETUP_FAILURE_RE = re.compile(
    rb"^\s*FAIL LTP SETUP\s+([!-9;-~]+)\s*:\s*(-?\d{1,10})\s*$"
)
OFFICIAL_GROUP_FAIL_RE = re.compile(
    rb"^\s*FAIL OFFICIAL TEST GROUP\s+([ -~]+?)\s*:\s*(-?\d{1,10})\s*$"
)
OFFICIAL_GROUP_TIMEOUT_RE = re.compile(
    rb"^\s*TIMEOUT OFFICIAL TEST GROUP\s+([ -~]+?)\s+after\s+(\d{1,10})s\s*$",
    re.I,
)
AUTORUN_EXIT_RE = re.compile(
    rb"^\s*autorun:\s+([ -~]+?)\s+exited with status\s+(-?\d{1,10})\s*$", re.I
)
AUTORUN_TIMEOUT_RE = re.compile(
    rb"^\s*autorun:\s+([ -~]+?)\s+timed out after\s+(\d{1,10})s\s*$", re.I
)
AUTORUN_ERROR_RE = re.compile(
    rb"^\s*autorun:\s+([ -~]*?(?:failed(?::.*)?|not found))\s*$", re.I
)
LIBCTEST_FAIL_RE = re.compile(
    rb"^\s*FAIL libctest\s+([!-~]+)\s+([!-~]+):\s+([ -~]+?)\s*$", re.I
)
LIBCTEST_CASE_START_RE = re.compile(
    rb"^\s*=+ START ([!-~]+) ([!-~]+) =+\s*$", re.I
)
LIBCTEST_CASE_END_RE = re.compile(
    rb"^\s*=+ END ([!-~]+) ([!-~]+) =+\s*$", re.I
)
LIBCTEST_PASS_RE = re.compile(rb"^\s*Pass!\s*$")
LIBCTEST_SUMMARY_RE = re.compile(
    rb"^\s*libctest cases:\s+(\d{1,10})\s+passed,\s+(\d{1,10})\s+failed,\s+(\d{1,10})\s+timed out\s*$",
    re.I,
)
BUSYBOX_RESULT_RE = re.compile(
    rb"^\s*testcase busybox\s+([ -~]+?)\s+(success|fail)\s*$", re.I
)
LIBCTEST_ERROR_RE = re.compile(
    rb"^\s*libctest:\s+([ -~]*?(?:failed(?::.*)?|no runnable commands found))\s*$",
    re.I,
)
INTERNAL_SUMMARY_START_RE = re.compile(rb"^\s*Summary:\s*$", re.I)
INTERNAL_SUMMARY_FIELD_RE = re.compile(
    rb"^\s*(passed|failed|broken|skipped|warnings)\s*[: ]\s*(\d{1,10})\s*$", re.I
)
OFFICIAL_SKIP_RE = re.compile(
    rb"^\s*\[CONTEST\]\[OFFICIAL\]\[SKIP\]\s+([A-Za-z0-9._-]+):\s+configured skip\s*$",
    re.I,
)
INTERNAL_RE = re.compile(rb"(?<![A-Za-z0-9_])(TFAIL|TBROK|TCONF)(?![A-Za-z0-9_])")
ENOSYS_RE = re.compile(rb"\bENOSYS\b|errno=ENOSYS|not implemented", re.I)
PANIC_RE = re.compile(
    rb"(?<![A-Za-z0-9_])(?:panic|panicked|trap|Unhandled trap|InstructionNotExist|fatal trap|kernel trap)(?![A-Za-z0-9_])",
    re.I,
)
GENERIC_TIMEOUT_RE = re.compile(
    rb"\b(?:TIMEOUT|timed out|timeout reached|timeout expired|killed after timeout)\b",
    re.I,
)
CONTROL_HINT_RE = re.compile(
    rb"(?:OS COMP TEST GROUP|RUN LTP CASE|(?:PASS|FAIL|TIMEOUT) LTP (?:CASE|SETUP)|"
    rb"(?:FAIL|TIMEOUT) OFFICIAL TEST GROUP|ltp case list:|ltp cases:|autorun:|"
    rb"FAIL libctest|libctest cases:|libctest:|testcase busybox|\[CONTEST\]\[OFFICIAL\])",
    re.I,
)


def normalize_terminal_bytes(data: bytes) -> tuple[bytes, tuple[int, str] | None]:
    """Normalize display-ignorable bytes and reject stateful/ambiguous controls."""

    normalized = data.replace(b"\r\n", b"\n")
    normalized = SAFE_TERMINAL_CONTROL_RE.sub(b"", normalized)
    for index, value in enumerate(normalized):
        if value == 0x1B:
            reason = "unsupported or incomplete terminal escape sequence"
        elif value == 0x0D:
            reason = "bare carriage return can overwrite visible text"
        elif 0x80 <= value <= 0x9F:
            reason = "C1 terminal control byte is forbidden"
        elif value == 0x7F or value in {*range(0x01, 0x07), 0x08, 0x0B, 0x0C, *range(0x0E, 0x1B), *range(0x1C, 0x20)}:
            reason = "stateful C0/DEL terminal control byte is forbidden"
        else:
            continue
        return normalized, (normalized[:index].count(b"\n") + 1, reason)
    return normalized, None


def _display(data: bytes) -> str:
    return data.decode("ascii", errors="backslashreplace")


def _signals() -> dict[str, int]:
    return {
        "TFAIL": 0,
        "TBROK": 0,
        "TCONF": 0,
        "enosys": 0,
        "panic_trap": 0,
        "timeout": 0,
        "ltp_failed": 0,
        "ltp_broken": 0,
        "ltp_skipped": 0,
        "ltp_warnings": 0,
        "official_fail": 0,
        "official_timeout": 0,
        "autorun_fail": 0,
        "autorun_error": 0,
        "official_skip": 0,
    }


def _new_group(name: str, line: int) -> dict[str, Any]:
    return {
        "name": name,
        "start_line": line,
        "end_line": None,
        "case_list": None,
        "suite_summary": None,
        "setup_failure": None,
        "libctest_summary": None,
        "libctest_case_pass_count": 0,
        "libctest_case_fail_count": 0,
        "libctest_error_count": 0,
        "libctest_timeout_count": 0,
        "busybox_result_count": 0,
        "signals": _signals(),
        "cases": [],
        "state": "error",
    }


def _new_case(group: str, name: str, line: int) -> dict[str, Any]:
    return {
        "group": group,
        "case": name,
        "run_line": line,
        "result_line": None,
        "raw_status": None,
        "code": None,
        "timed_out": False,
        "timeout_line": None,
        "timeout_seconds": None,
        "ltp_summary": None,
        "signals": _signals(),
        "state": "error",
    }


def _classify(signals: dict[str, int], code: int | None, timed_out: bool) -> str:
    if (
        signals["panic_trap"]
        or signals["TBROK"]
        or signals["ltp_broken"]
        or signals["autorun_error"]
    ):
        return "error"
    if timed_out or signals["timeout"] or signals["official_timeout"]:
        return "timeout"
    if code is not None and code != 0:
        return "fail"
    if (
        signals["TFAIL"]
        or signals["enosys"]
        or signals["ltp_failed"]
        or signals["ltp_warnings"]
        or signals["official_fail"]
        or signals["autorun_fail"]
    ):
        return "fail"
    if signals["TCONF"] or signals["ltp_skipped"] or signals["official_skip"]:
        return "skipped"
    if code == 0:
        return "pass"
    return "error"


def parse_evaluator_bytes(data: bytes) -> dict[str, Any]:
    diagnostics: list[dict[str, Any]] = []
    groups: list[dict[str, Any]] = []
    cases: list[dict[str, Any]] = []
    official_events: list[dict[str, Any]] = []
    global_signals = _signals()
    current_group: dict[str, Any] | None = None
    current_case: dict[str, Any] | None = None
    current_libctest: dict[str, Any] | None = None
    internal_summary_case: dict[str, Any] | None = None
    seen_group_names: set[str] = set()

    def diagnostic(code: str, line: int, detail: str) -> None:
        diagnostics.append({"code": code, "line": line, "detail": detail})

    if not data:
        diagnostic("empty_log", 0, "evaluator log is empty")

    normalized_data, terminal_error = normalize_terminal_bytes(data)
    if terminal_error is not None:
        diagnostic("malformed_terminal_control", terminal_error[0], terminal_error[1])

    for line_number, line in enumerate(normalized_data.split(b"\n"), 1):
        recognized = False
        if b"\0" in line or b"\a" in line:
            cleaned = line.replace(b"\0", b"").replace(b"\a", b"")
            if CONTROL_HINT_RE.search(cleaned):
                diagnostic(
                    "malformed_control_record",
                    line_number,
                    "control record contains display-ignorable bytes",
                )
                line = b""
            else:
                line = cleaned
        if any(value >= 0x80 for value in line):
            ascii_projection = bytes(value for value in line if value < 0x80)
            if (
                CONTROL_HINT_RE.search(ascii_projection)
                or INTERNAL_RE.search(ascii_projection)
                or ENOSYS_RE.search(ascii_projection)
                or PANIC_RE.search(ascii_projection)
                or GENERIC_TIMEOUT_RE.search(ascii_projection)
            ):
                diagnostic(
                    "malformed_control_record",
                    line_number,
                    "non-ASCII bytes split a control or failure signal",
                )
                line = b""

        if match := GROUP_START_RE.search(line):
            recognized = True
            name = _display(match.group(1))
            if current_group is not None:
                diagnostic(
                    "nested_group",
                    line_number,
                    f"group {name} started before {current_group['name']} ended",
                )
            if name in seen_group_names:
                diagnostic("duplicate_group", line_number, f"duplicate group {name}")
            seen_group_names.add(name)
            current_group = _new_group(name, line_number)
            groups.append(current_group)
            current_case = None
            current_libctest = None
            internal_summary_case = None
        elif match := GROUP_END_RE.search(line):
            recognized = True
            name = _display(match.group(1))
            if current_group is None:
                diagnostic("orphan_group_end", line_number, f"group end without start: {name}")
            else:
                if current_group["name"] != name:
                    diagnostic(
                        "group_label_mismatch",
                        line_number,
                        f"started {current_group['name']} but ended {name}",
                    )
                if current_case is not None and current_case["result_line"] is None:
                    diagnostic(
                        "truncated_case",
                        line_number,
                        f"case {current_case['case']} has no result",
                    )
                current_group["end_line"] = line_number
            if current_libctest is not None:
                diagnostic(
                    "truncated_libctest_case",
                    line_number,
                    f"{current_libctest['entry']} {current_libctest['case']}",
                )
            current_group = None
            current_case = None
            current_libctest = None
            internal_summary_case = None
        elif match := CASE_LIST_RE.search(line):
            recognized = True
            if current_group is None:
                diagnostic("case_list_outside_group", line_number, "case list is outside a group")
            else:
                record = {
                    "name": _display(match.group(1)),
                    "case_count": int(match.group(2)),
                    "timeout_seconds": int(match.group(3)),
                    "line": line_number,
                }
                if current_group["case_list"] is not None:
                    diagnostic("duplicate_case_list", line_number, current_group["name"])
                else:
                    current_group["case_list"] = record
        elif match := CASE_START_RE.search(line):
            recognized = True
            name = _display(match.group(1))
            if current_group is None:
                diagnostic("case_outside_group", line_number, f"RUN for {name} is outside a group")
            else:
                if current_case is not None and current_case["result_line"] is None:
                    diagnostic(
                        "truncated_case",
                        line_number,
                        f"case {current_case['case']} has no result before {name}",
                    )
                if any(item["case"] == name for item in current_group["cases"]):
                    diagnostic("duplicate_case", line_number, f"duplicate case {name}")
                current_case = _new_case(current_group["name"], name, line_number)
                current_group["cases"].append(current_case)
                cases.append(current_case)
                internal_summary_case = None
        elif match := CASE_RESULT_RE.search(line):
            recognized = True
            raw_status = _display(match.group(1)).upper()
            name = _display(match.group(2))
            code = int(match.group(3))
            if current_case is None:
                diagnostic("orphan_result", line_number, f"result without RUN for {name}")
            elif current_case["case"] != name:
                diagnostic(
                    "case_result_mismatch",
                    line_number,
                    f"RUN {current_case['case']} but result {name}",
                )
            elif current_case["result_line"] is not None:
                diagnostic("duplicate_result", line_number, f"duplicate result for {name}")
            else:
                current_case["result_line"] = line_number
                current_case["raw_status"] = raw_status
                current_case["code"] = code
        elif match := CASE_TIMEOUT_RE.search(line):
            recognized = True
            name = _display(match.group(1))
            timeout_seconds = match.group(2)
            if current_case is None or current_case["case"] != name:
                diagnostic("orphan_timeout", line_number, f"timeout without matching RUN for {name}")
            elif current_case["timeout_line"] is not None:
                diagnostic("duplicate_timeout", line_number, f"duplicate timeout for {name}")
            else:
                current_case["timed_out"] = True
                current_case["timeout_line"] = line_number
                current_case["timeout_seconds"] = (
                    int(timeout_seconds) if timeout_seconds is not None else None
                )
        elif match := SUITE_SUMMARY_RE.search(line):
            recognized = True
            if current_group is None:
                diagnostic("summary_outside_group", line_number, "LTP summary is outside a group")
            else:
                record = {
                    "passed": int(match.group(1)),
                    "failed": int(match.group(2)),
                    "timed_out": int(match.group(3) or 0),
                    "line": line_number,
                }
                if current_group["suite_summary"] is not None:
                    diagnostic("duplicate_suite_summary", line_number, current_group["name"])
                else:
                    current_group["suite_summary"] = record
                if current_case is not None and current_case["result_line"] is None:
                    diagnostic(
                        "truncated_case",
                        line_number,
                        f"case {current_case['case']} has no result before suite summary",
                    )
                current_case = None
                internal_summary_case = None
        elif match := SETUP_FAILURE_RE.search(line):
            recognized = True
            if current_group is None:
                diagnostic("setup_failure_outside_group", line_number, "LTP setup failure outside group")
            elif current_group["setup_failure"] is not None:
                diagnostic("duplicate_setup_failure", line_number, current_group["name"])
            else:
                current_group["setup_failure"] = {
                    "group": _display(match.group(1)),
                    "code": int(match.group(2)),
                    "line": line_number,
                }
        elif match := OFFICIAL_GROUP_FAIL_RE.search(line):
            recognized = True
            label = _display(match.group(1))
            code = int(match.group(2))
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["official_fail"] += 1
            official_events.append(
                {"kind": "group_fail", "label": label, "code": code, "line": line_number}
            )
        elif match := OFFICIAL_GROUP_TIMEOUT_RE.search(line):
            recognized = True
            label = _display(match.group(1))
            timeout_seconds = int(match.group(2))
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["official_timeout"] += 1
            official_events.append(
                {
                    "kind": "group_timeout",
                    "label": label,
                    "timeout_seconds": timeout_seconds,
                    "line": line_number,
                }
            )
        elif match := OFFICIAL_SKIP_RE.search(line):
            recognized = True
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["official_skip"] += 1
            official_events.append(
                {"kind": "official_skip", "label": _display(match.group(1)), "line": line_number}
            )
        elif match := AUTORUN_TIMEOUT_RE.search(line):
            recognized = True
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["official_timeout"] += 1
            official_events.append(
                {
                    "kind": "autorun_timeout",
                    "label": _display(match.group(1)),
                    "timeout_seconds": int(match.group(2)),
                    "line": line_number,
                }
            )
        elif match := AUTORUN_EXIT_RE.search(line):
            recognized = True
            code = int(match.group(2))
            if code != 0:
                signal_target = current_group["signals"] if current_group is not None else global_signals
                signal_target["autorun_fail"] += 1
                official_events.append(
                    {
                        "kind": "autorun_exit",
                        "label": _display(match.group(1)),
                        "code": code,
                        "line": line_number,
                    }
                )
        elif match := AUTORUN_ERROR_RE.search(line):
            recognized = True
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["autorun_error"] += 1
            official_events.append(
                {
                    "kind": "autorun_error",
                    "label": _display(match.group(1)),
                    "line": line_number,
                }
            )
        elif match := LIBCTEST_CASE_START_RE.search(line):
            recognized = True
            if current_group is None or not current_group["name"].lower().startswith(
                "libctest-"
            ):
                diagnostic("libctest_case_outside_group", line_number, _display(line[:180]))
            elif current_libctest is not None:
                diagnostic(
                    "nested_libctest_case",
                    line_number,
                    f"{current_libctest['entry']} {current_libctest['case']}",
                )
            else:
                current_libctest = {
                    "entry": _display(match.group(1)),
                    "case": _display(match.group(2)),
                    "result": None,
                    "line": line_number,
                }
        elif LIBCTEST_PASS_RE.fullmatch(line):
            recognized = True
            if current_libctest is None:
                diagnostic("orphan_libctest_pass", line_number, "Pass! without START")
            elif current_libctest["result"] is not None:
                diagnostic("duplicate_libctest_result", line_number, current_libctest["case"])
            else:
                current_libctest["result"] = "pass"
        elif match := LIBCTEST_SUMMARY_RE.search(line):
            recognized = True
            failed = int(match.group(2))
            timed_out = int(match.group(3))
            if current_group is None or not current_group["name"].lower().startswith(
                "libctest-"
            ):
                diagnostic(
                    "libctest_summary_outside_group", line_number, _display(line[:180])
                )
            else:
                if current_group["libctest_summary"] is not None:
                    diagnostic("duplicate_libctest_summary", line_number, current_group["name"])
                else:
                    current_group["libctest_summary"] = {
                        "passed": int(match.group(1)),
                        "failed": failed,
                        "timed_out": timed_out,
                        "line": line_number,
                    }
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["official_fail"] += failed
            signal_target["official_timeout"] += timed_out
            if failed or timed_out:
                official_events.append(
                    {
                        "kind": "libctest_summary",
                        "failed": failed,
                        "timed_out": timed_out,
                        "line": line_number,
                    }
                )
        elif match := LIBCTEST_FAIL_RE.search(line):
            recognized = True
            entry = _display(match.group(1))
            case_name = _display(match.group(2))
            reason = _display(match.group(3))
            if current_group is None or not current_group["name"].lower().startswith(
                "libctest-"
            ):
                diagnostic("libctest_fail_outside_group", line_number, f"{entry} {case_name}")
            elif current_libctest is None:
                diagnostic("orphan_libctest_fail", line_number, f"{entry} {case_name}")
            elif (current_libctest["entry"], current_libctest["case"]) != (
                entry,
                case_name,
            ):
                diagnostic("libctest_result_mismatch", line_number, f"{entry} {case_name}")
            elif current_libctest["result"] is not None:
                diagnostic("duplicate_libctest_result", line_number, case_name)
            else:
                current_libctest["result"] = "fail"
                if "timeout" in reason.lower() and current_group is not None:
                    current_group["libctest_timeout_count"] += 1
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["official_fail"] += 1
            official_events.append(
                {
                    "kind": "libctest_fail",
                    "label": f"{entry} {case_name}: {reason}",
                    "line": line_number,
                }
            )
        elif match := LIBCTEST_CASE_END_RE.search(line):
            recognized = True
            entry = _display(match.group(1))
            case_name = _display(match.group(2))
            if current_libctest is None:
                diagnostic("orphan_libctest_end", line_number, f"{entry} {case_name}")
            else:
                if (current_libctest["entry"], current_libctest["case"]) != (
                    entry,
                    case_name,
                ):
                    diagnostic("libctest_end_mismatch", line_number, f"{entry} {case_name}")
                if current_libctest["result"] is None:
                    diagnostic("missing_libctest_result", line_number, f"{entry} {case_name}")
                elif current_group is not None:
                    current_group[
                        "libctest_case_pass_count"
                        if current_libctest["result"] == "pass"
                        else "libctest_case_fail_count"
                    ] += 1
                current_libctest = None
        elif match := LIBCTEST_ERROR_RE.search(line):
            recognized = True
            if current_group is None or not current_group["name"].lower().startswith(
                "libctest-"
            ):
                diagnostic("libctest_error_outside_group", line_number, _display(line[:180]))
            else:
                current_group["libctest_error_count"] += 1
                if "timeout" in _display(match.group(1)).lower():
                    current_group["libctest_timeout_count"] += 1
            signal_target = current_group["signals"] if current_group is not None else global_signals
            signal_target["official_fail"] += 1
            official_events.append(
                {"kind": "libctest_error", "label": _display(match.group(1)), "line": line_number}
            )
        elif match := BUSYBOX_RESULT_RE.search(line):
            recognized = True
            if current_group is None or not current_group["name"].lower().startswith(
                "busybox-"
            ):
                diagnostic("busybox_result_outside_group", line_number, _display(line[:180]))
            else:
                current_group["busybox_result_count"] += 1
            if _display(match.group(2)).lower() == "fail":
                signal_target = current_group["signals"] if current_group is not None else global_signals
                signal_target["official_fail"] += 1
                official_events.append(
                    {"kind": "busybox_fail", "label": _display(match.group(1)), "line": line_number}
                )
        elif INTERNAL_SUMMARY_START_RE.fullmatch(line):
            recognized = True
            if current_case is None:
                diagnostic(
                    "internal_summary_outside_case",
                    line_number,
                    "internal LTP Summary is not attached to a running case",
                )
            else:
                if current_case["ltp_summary"] is not None:
                    diagnostic(
                        "duplicate_internal_summary", line_number, current_case["case"]
                    )
                else:
                    current_case["ltp_summary"] = {
                        "line": line_number,
                        "passed": None,
                        "failed": None,
                        "broken": None,
                        "skipped": None,
                        "warnings": None,
                    }
                    internal_summary_case = current_case
        elif match := INTERNAL_SUMMARY_FIELD_RE.fullmatch(line):
            recognized = True
            if internal_summary_case is None:
                diagnostic(
                    "orphan_internal_summary_field",
                    line_number,
                    _display(match.group(1)).lower(),
                )
            else:
                field = _display(match.group(1)).lower()
                value = int(match.group(2))
                summary = internal_summary_case["ltp_summary"]
                if summary[field] is not None:
                    diagnostic(
                        "duplicate_internal_summary_field",
                        line_number,
                        f"{internal_summary_case['case']}:{field}",
                    )
                else:
                    summary[field] = value
                    signal_key = {
                        "failed": "ltp_failed",
                        "broken": "ltp_broken",
                        "skipped": "ltp_skipped",
                        "warnings": "ltp_warnings",
                    }.get(field)
                    if signal_key is not None:
                        internal_summary_case["signals"][signal_key] += value

        target_signals = (
            current_case["signals"]
            if current_case is not None
            else current_group["signals"]
            if current_group is not None
            else global_signals
        )
        for marker in INTERNAL_RE.findall(line):
            target_signals[_display(marker).upper()] += 1
        if ENOSYS_RE.search(line):
            target_signals["enosys"] += 1
        if PANIC_RE.search(line):
            target_signals["panic_trap"] += 1
        if (
            GENERIC_TIMEOUT_RE.search(line)
            and not CASE_TIMEOUT_RE.search(line)
            and not CASE_LIST_RE.search(line)
            and not SUITE_SUMMARY_RE.search(line)
            and not LIBCTEST_SUMMARY_RE.search(line)
        ):
            target_signals["timeout"] += 1

        if CONTROL_HINT_RE.search(line) and not recognized:
            diagnostic(
                "malformed_control_record",
                line_number,
                "control-like line did not match the ASCII protocol: " + _display(line[:180]),
            )

    if current_group is not None:
        diagnostic("unclosed_group", len(data.splitlines()), current_group["name"])
    if current_case is not None and current_case["result_line"] is None:
        diagnostic("truncated_case", len(data.splitlines()), current_case["case"])

    ltp_groups = [
        group
        for group in groups
        if group["name"].lower().startswith("ltp-")
        or group["cases"]
        or group["case_list"] is not None
        or group["suite_summary"] is not None
        or group["setup_failure"] is not None
    ]
    if not ltp_groups:
        diagnostic("no_ltp_group", 0, "no LTP evaluator group was found")

    for case in cases:
        if case["ltp_summary"] is not None:
            missing_fields = [
                field
                for field in ("passed", "failed", "broken", "skipped", "warnings")
                if case["ltp_summary"][field] is None
            ]
            if missing_fields:
                diagnostic(
                    "incomplete_internal_summary",
                    case["ltp_summary"]["line"],
                    f"{case['case']} missing {','.join(missing_fields)}",
                )
        case["state"] = _classify(case["signals"], case["code"], case["timed_out"])

    for group in ltp_groups:
        if group["setup_failure"] is None:
            if group["case_list"] is None:
                diagnostic("missing_case_list", group["start_line"], group["name"])
            if group["suite_summary"] is None:
                diagnostic("missing_suite_summary", group["start_line"], group["name"])
        if group["end_line"] is None:
            diagnostic("missing_group_end", group["start_line"], group["name"])
        if group["case_list"] is not None and group["case_list"]["case_count"] != len(
            group["cases"]
        ):
            diagnostic(
                "case_count_mismatch",
                group["case_list"]["line"],
                f"{group['name']} declared {group['case_list']['case_count']} but ran {len(group['cases'])}",
            )
        if group["case_list"] is not None and group["case_list"]["case_count"] == 0:
            diagnostic("empty_case_list", group["case_list"]["line"], group["name"])
        if group["suite_summary"] is not None:
            if group["setup_failure"] is not None:
                expected = (0, 1, 0)
            else:
                passed = sum(
                    case["code"] == 0 and not case["timed_out"] for case in group["cases"]
                )
                failed = sum(
                    case["code"] is not None and case["code"] != 0
                    for case in group["cases"]
                )
                timed_out = sum(case["timed_out"] for case in group["cases"])
                expected = (passed, failed, timed_out)
            observed = (
                group["suite_summary"]["passed"],
                group["suite_summary"]["failed"],
                group["suite_summary"]["timed_out"],
            )
            if observed != expected:
                diagnostic(
                    "suite_summary_mismatch",
                    group["suite_summary"]["line"],
                    f"{group['name']} observed {observed}, parsed {expected}",
                )
    for group in groups:
        lowered = group["name"].lower()
        if lowered.startswith("libctest-"):
            summary = group["libctest_summary"]
            if summary is None:
                diagnostic("missing_libctest_summary", group["start_line"], group["name"])
            elif summary["passed"] + summary["failed"] == 0:
                diagnostic("empty_libctest_summary", summary["line"], group["name"])
            elif (
                summary["passed"],
                summary["failed"],
                summary["timed_out"],
            ) != (
                group["libctest_case_pass_count"],
                group["libctest_case_fail_count"] + group["libctest_error_count"],
                group["libctest_timeout_count"],
            ):
                diagnostic(
                    "libctest_summary_mismatch",
                    summary["line"],
                    f"{group['name']} summary does not match observed case/error records",
                )
        if lowered.startswith("busybox-") and group["busybox_result_count"] == 0:
            diagnostic("empty_busybox_group", group["start_line"], group["name"])
    for group in groups:
        group_case_states = [case["state"] for case in group["cases"]]
        group_signal_state = _classify(group["signals"], 0, False)
        if group["setup_failure"] is not None:
            group["state"] = "fail"
        elif "error" in group_case_states or group_signal_state == "error":
            group["state"] = "error"
        elif "timeout" in group_case_states or group_signal_state == "timeout":
            group["state"] = "timeout"
        elif "fail" in group_case_states or group_signal_state == "fail":
            group["state"] = "fail"
        elif "skipped" in group_case_states or group_signal_state == "skipped":
            group["state"] = "skipped"
        else:
            group["state"] = "pass"

    if (
        diagnostics
        or global_signals["panic_trap"]
        or global_signals["TBROK"]
        or global_signals["autorun_error"]
        or any(group["state"] == "error" for group in groups)
    ):
        state = "error"
    elif (
        global_signals["timeout"]
        or global_signals["official_timeout"]
        or any(group["state"] == "timeout" for group in groups)
    ):
        state = "timeout"
    elif (
        global_signals["TFAIL"]
        or global_signals["enosys"]
        or global_signals["official_fail"]
        or global_signals["autorun_fail"]
        or any(group["state"] == "fail" for group in groups)
    ):
        state = "fail"
    elif (
        global_signals["TCONF"]
        or global_signals["official_skip"]
        or any(group["state"] == "skipped" for group in groups)
    ):
        state = "skipped"
    else:
        state = "pass"

    return {
        "schema_version": PROTOCOL_SCHEMA_VERSION,
        "raw_sha256": hashlib.sha256(data).hexdigest(),
        "size_bytes": len(data),
        "state": state,
        "diagnostics": diagnostics,
        "global_signals": global_signals,
        "groups": groups,
        "official_events": official_events,
        "cases": cases,
    }


def is_clean(result: dict[str, Any]) -> bool:
    return result["state"] == "pass" and not result["diagnostics"]
