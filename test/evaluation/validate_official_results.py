#!/usr/bin/env python3
"""Strictly validate completion and result integrity in a local official run.

The guest evaluator currently emits several kinds of result records.  This
validator accepts only complete, paired groups with a group-specific success
contract.  It never converts a missing result, an empty group, a configured
skip, or the absence of a failure substring into PASS.
"""

from __future__ import annotations

import argparse
import json
import re
import unicodedata
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

ANSI_SGR_RE = re.compile(r"\x1b\[(?:[0-9]{1,3}(?:;[0-9]{1,3})*)?[mK]")
CANONICAL_OFFICIAL_GROUPS = (
    "ltp-musl", "ltp-glibc", "libctest-musl", "basic-musl", "basic-glibc",
    "busybox-musl", "busybox-glibc", "libctest-glibc", "lua-musl", "lua-glibc",
    "iperf-musl", "iperf-glibc", "netperf-musl", "netperf-glibc",
    "unixbench-musl", "unixbench-glibc", "libcbench-musl", "libcbench-glibc",
    "lmbench-musl", "lmbench-glibc", "cyclictest-musl", "cyclictest-glibc",
    "iozone-musl", "iozone-glibc",
)
CANONICAL_OFFICIAL_CASE_COUNTS = {
    "ltp-musl": 1000,
    "ltp-glibc": 1000,
    "busybox-musl": 55,
    "busybox-glibc": 55,
    "libctest-musl": 217,
    "libctest-glibc": 217,
}
CANONICAL_LTP_CASE_LIST = "stable-full"
OFFICIAL_CASE_PLAN_RELATIVE_PATH = Path("test/evaluation/official_case_plan.json")


def normalize_output_text(raw_text: str) -> str:
    """Remove only trusted terminal styling and normalize CRLF for parsing."""

    return ANSI_SGR_RE.sub("", raw_text).replace("\r\n", "\n")


def first_unsupported_output_character(text: str) -> str | None:
    """Reject characters that can hide, reorder, or split machine records."""

    for character in text:
        category = unicodedata.category(character)
        if (
            (category == "Cc" and character not in {"\t", "\n"})
            or category == "Cf"
            or category in {"Zl", "Zp"}
        ):
            return character
    return None


def trusted_ltp_stable_cases(repo: Path) -> list[str]:
    """Read the exact canonical stable-full identities from tracked source."""

    source_path = repo / "user/shell/src/cmd.rs"
    source = source_path.read_text(encoding="utf-8")
    match = re.search(
        r"const\s+LTP_STABLE_CASES:\s*&\[&str\]\s*=\s*&\[(.*?)\n\];",
        source,
        re.S,
    )
    if match is None:
        raise ValueError("cannot locate LTP_STABLE_CASES")
    without_comments = "\n".join(
        line.split("//", 1)[0] for line in match.group(1).splitlines()
    )
    cases = re.findall(r'"([^"\\]+)"', without_comments)
    if not cases or len(cases) != len(set(cases)):
        raise ValueError("LTP_STABLE_CASES must be non-empty and unique")
    return cases


def _reject_duplicate_json_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def trusted_official_case_plan(
    repo: Path,
) -> tuple[list[str], list[tuple[str, str]]]:
    """Read exact BusyBox and libctest identities from the tracked snapshot."""

    path = repo / OFFICIAL_CASE_PLAN_RELATIVE_PATH
    try:
        payload = json.loads(
            path.read_text(encoding="utf-8", errors="strict"),
            object_pairs_hook=_reject_duplicate_json_keys,
        )
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"cannot read {OFFICIAL_CASE_PLAN_RELATIVE_PATH}: {error}") from error

    if not isinstance(payload, dict) or set(payload) != {
        "schema_version",
        "source_snapshot",
        "busybox_rows",
        "libctest_cases",
    }:
        raise ValueError("official case plan must use the exact schema-v1 top-level fields")
    if payload["schema_version"] != 1:
        raise ValueError("official case plan schema_version must be 1")

    snapshot = payload["source_snapshot"]
    snapshot_fields = {
        "captured_date",
        "architectures",
        "libc_variants",
        "busybox_source",
        "busybox_source_sha256",
        "busybox_row_count",
        "busybox_unique_count",
        "libctest_static_source",
        "libctest_static_source_sha256",
        "libctest_dynamic_source",
        "libctest_dynamic_source_sha256",
        "libctest_case_count",
    }
    if not isinstance(snapshot, dict) or set(snapshot) != snapshot_fields:
        raise ValueError("official case plan source_snapshot has missing or unknown fields")
    if snapshot["architectures"] != ["riscv64", "loongarch64"]:
        raise ValueError("official case plan must cover the RV and LA snapshots")
    if snapshot["libc_variants"] != ["musl", "glibc"]:
        raise ValueError("official case plan must cover the musl and glibc snapshots")
    expected_sources = {
        "busybox_source": "/musl/busybox_cmd.txt",
        "libctest_static_source": "/musl/run-static.sh",
        "libctest_dynamic_source": "/musl/run-dynamic.sh",
    }
    for field_name, expected_value in expected_sources.items():
        if snapshot[field_name] != expected_value:
            raise ValueError(f"official case plan {field_name} must be {expected_value!r}")
    if not isinstance(snapshot["captured_date"], str) or not re.fullmatch(
        r"\d{4}-\d{2}-\d{2}", snapshot["captured_date"]
    ):
        raise ValueError("official case plan captured_date must be YYYY-MM-DD")
    for field_name in (
        "busybox_source_sha256",
        "libctest_static_source_sha256",
        "libctest_dynamic_source_sha256",
    ):
        if not isinstance(snapshot[field_name], str) or not re.fullmatch(
            r"[0-9a-f]{64}", snapshot[field_name]
        ):
            raise ValueError(f"official case plan {field_name} must be a SHA-256 digest")

    busybox_rows = payload["busybox_rows"]
    if (
        not isinstance(busybox_rows, list)
        or any(not isinstance(row, str) or not row or row != row.strip() for row in busybox_rows)
    ):
        raise ValueError("official case plan busybox_rows must be non-empty trimmed strings")
    if len(busybox_rows) != CANONICAL_OFFICIAL_CASE_COUNTS["busybox-musl"]:
        raise ValueError("official case plan has the wrong BusyBox row count")
    if snapshot["busybox_row_count"] != len(busybox_rows):
        raise ValueError("official case plan BusyBox row metadata does not match its rows")
    if snapshot["busybox_unique_count"] != len(set(busybox_rows)):
        raise ValueError("official case plan BusyBox unique-count metadata is inconsistent")

    raw_libctest_cases = payload["libctest_cases"]
    libctest_cases: list[tuple[str, str]] = []
    if not isinstance(raw_libctest_cases, list):
        raise ValueError("official case plan libctest_cases must be a list")
    for index, entry in enumerate(raw_libctest_cases):
        if (
            not isinstance(entry, dict)
            or set(entry) != {"binary", "case"}
            or any(not isinstance(entry[field], str) or not entry[field] for field in ("binary", "case"))
        ):
            raise ValueError(f"official case plan libctest_cases[{index}] is malformed")
        libctest_cases.append((entry["binary"], entry["case"]))
    if len(libctest_cases) != CANONICAL_OFFICIAL_CASE_COUNTS["libctest-musl"]:
        raise ValueError("official case plan has the wrong libctest case count")
    if len(libctest_cases) != len(set(libctest_cases)):
        raise ValueError("official case plan libctest identities must be unique")
    if snapshot["libctest_case_count"] != len(libctest_cases):
        raise ValueError("official case plan libctest count metadata is inconsistent")
    return busybox_rows, libctest_cases
GROUP_START_RE = re.compile(r"^#### OS COMP TEST GROUP START (.+?) ####$")
GROUP_END_RE = re.compile(r"^#### OS COMP TEST GROUP END (.+?) ####$")
OFFICIAL_PASS_RE = re.compile(r"^PASS OFFICIAL TEST GROUP\s+(.+?)\s*:\s*(-?\d+)\s*$")
OFFICIAL_FAIL_RE = re.compile(r"^FAIL OFFICIAL TEST GROUP(?: FILTER)?\s+(.+?)\s*:\s*(-?\d+)\s*$")
LTP_LIST_RE = re.compile(
    r"^ltp case list:\s+(.+?)\s+\((\d+)\s+cases,\s+timeout\s+(\d+)s\)\s*$", re.I
)
LTP_START_RE = re.compile(r"^=+ START ltp\s+(\S+)\s+=+$")
LTP_RUN_RE = re.compile(r"^RUN LTP CASE\s+(\S+)\s*$")
LTP_RESULT_RE = re.compile(r"^(?:PASS|FAIL) LTP CASE\s+(\S+)\s*:\s*(-?\d+)\s*$")
LTP_END_RE = re.compile(r"^=+ END ltp\s+(\S+)\s+=+$")
LTP_SUMMARY_RE = re.compile(
    r"^ltp cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed(?:,\s+(\d+)\s+timed out)?\s*$",
    re.I,
)
BUSYBOX_RESULT_RE = re.compile(r"^testcase busybox\s+(.+?)\s+(success|fail)\s*$")
LIBCTEST_START_RE = re.compile(r"^=+ START\s+(\S+)\s+(\S+)\s+=+$")
LIBCTEST_END_RE = re.compile(r"^=+ END\s+(\S+)\s+(\S+)\s+=+$")
LIBCTEST_FAIL_RE = re.compile(r"^FAIL libctest\s+(\S+)\s+(\S+)\s*:\s*(.+)\s*$")
LIBCTEST_SUMMARY_RE = re.compile(
    r"^libctest cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed,\s+(\d+)\s+timed out\s*$",
    re.I,
)
FORBIDDEN_STATUS_RE = re.compile(
    r"\b(TCONF|TBROK|TFAIL|ENOSYS|XFAIL|SKIP(?:PED)?|TIMEOUT|TIMED[_ -]?OUT|"
    r"TIME[_ -]?LIMIT[_ -]?EXCEEDED|HANG|CRASH|PANIC|ERROR|FAIL(?:ED|URE|URES)?)\b",
    re.I,
)
TIMEOUT_RE = re.compile(
    r"\b(?:TIMEOUT (?:LTP CASE|OFFICIAL TEST GROUP)|timed out after|timeout reached|"
    r"timeout expired|killed after timeout|command timed out|deadline exceeded|"
    r"watchdog expired|time[_ -]?limit[_ -]?exceeded|timed[_ -]?out|"
    r"deadline_exceeded|watchdog_expired|command_timed_out|timeout_error|ETIMEDOUT)\b",
    re.I,
)
PANIC_RE = re.compile(
    r"\b(?:kernel panic|panic(?:ked)?|unknown trap|unhandled(?: user)? trap|fatal trap|"
    r"InstructionNotExist|IllegalInstruction|illegal instruction|SegmentationFault|"
    r"segmentation fault|segfault|bus error|core dump(?:ed)?|process crash(?:ed)?|"
    r"crashed|aborted|killed|terminated|signal\s*(?::|=|\s)\s*\d+|trap\s*[:=]?\s*\d+|"
    r"SIG(?:ABRT|BUS|FPE|HUP|ILL|INT|KILL|QUIT|SEGV|SYS|TERM|TRAP))\b",
    re.I,
)
SKIP_RE = re.compile(
    r"\[CONTEST\]\[OFFICIAL\]\[SKIP\]|\bofficial test group\b.*\bskip(?:ped)?\b|"
    r"^\s*(?:\d+\s+ignored|test\s+\S.*\s+ignored|NOT[-_ ]APPLICABLE(?:\s+.*)?|"
    r".*\bnot selected\b.*)\s*$",
    re.I,
)
INFRA_TEXT_RE = re.compile(
    r"\b(?:busybox shell not found|prepare .+ failed|missing (?:ltp testcase|libctest entry)|"
    r"qemu: terminating on signal)\b",
    re.I,
)
UNCONSUMED_FAILURE_RE = re.compile(
    r"(?:^\s*FAIL(?:ED|URE|URES)?(?:\b|:)|"
    r"^\s*not\s+ok(?:\s+\d+)?(?:\b|\s*-)|"
    r":\s*FAIL(?:ED|URE|URES)?\s*$|"
    r"\[\s*FAIL(?:ED|URE|URES)?\s*\]|"
    r"\bresult\s*=\s*FAIL(?:ED|URE|URES)?\b)",
    re.I,
)
EXPLICIT_NONZERO_RE = re.compile(
    r"^\s*(?:not successful|exit status\s+[1-9]\d*|return\s*:\s*[1-9]\d*)\s*$",
    re.I,
)
UNKNOWN_STATE_RE = re.compile(
    r"^(?:STATUS|RESULT|STATE|CASE_STATUS)\s*(?:(?::|=)\s*\S.*|\[[^\]]+\](?:\s.*)?)$|"
    r"^(?:NOT[-_ ]RUN|NOT[-_ ]EXECUTED|UNEXECUTED|DID[-_ ]NOT[-_ ]RUN|"
    r"NOT[-_ ]ATTEMPTED|UNKNOWN(?:[-_ ]STATUS)?|(?:STATUS|RESULT)[-_ ]UNKNOWN|"
    r"UNRESOLVED|UNSUPPORTED|INCONCLUSIVE|INCOMPLETE|PARTIAL(?:LY[-_ ]EXECUTED)?|"
    r"INFRA_ERROR|PENDING|CANCELLED|CANCELED|DISABLED|OMITTED)\s*(?:(?::|=)\s*.*)?$|"
    r"^\[(?:NOT[-_ ]RUN|NOT[-_ ]EXECUTED|UNEXECUTED|DID[-_ ]NOT[-_ ]RUN|"
    r"NOT[-_ ]ATTEMPTED|UNKNOWN(?:[-_ ]STATUS)?|(?:STATUS|RESULT)[-_ ]UNKNOWN|"
    r"UNRESOLVED|UNSUPPORTED|INCONCLUSIVE|INCOMPLETE|PARTIAL(?:LY[-_ ]EXECUTED)?|"
    r"INFRA_ERROR|PENDING|CANCELLED|CANCELED|DISABLED|OMITTED)\](?:\s+.*)?$|"
    r"^(?:case|test)\s+\S.*:\s*(?:NOT[-_ ]RUN|NOT[-_ ]EXECUTED|UNEXECUTED|"
    r"DID[-_ ]NOT[-_ ]RUN|NOT[-_ ]ATTEMPTED|UNKNOWN|UNRESOLVED|UNSUPPORTED|"
    r"INCONCLUSIVE|INCOMPLETE|PARTIAL|INFRA_ERROR|PENDING|CANCELLED|CANCELED|"
    r"DISABLED|OMITTED|NOT[-_ ]APPLICABLE)(?:\s+.*)?$|"
    r"^(?:STATUS|RESULT|STATE|CASE_STATUS)\s+(?:NOT[-_ ]RUN|NOT[-_ ]EXECUTED|"
    r"UNEXECUTED|DID[-_ ]NOT[-_ ]RUN|NOT[-_ ]ATTEMPTED|UNKNOWN|UNRESOLVED|"
    r"UNSUPPORTED|INCONCLUSIVE|INCOMPLETE|PARTIAL|INFRA_ERROR|PENDING|"
    r"CANCELLED|CANCELED|DISABLED|OMITTED|NOT[-_ ]APPLICABLE)\s*$",
    re.I,
)
ZERO_EXECUTION_RE = re.compile(
    r"\b(?:NO\s+(?:TESTS?|CASES?)\s+(?:RAN|RUN|EXECUTED)|"
    r"(?:0|ZERO)\s+(?:TESTS?|CASES?)\s+(?:RAN|RUN|EXECUTED)|"
    r"RAN\s+(?:0|ZERO)\s+(?:TESTS?|CASES?)|NO\s+RUNNABLE\s+(?:TESTS?|CASES?)|"
    r"(?:TEST\s+SUITE|SUITE)\s+IS\s+EMPTY|EMPTY\s+(?:TEST\s+)?SUITE|"
    r"(?:RUNNING|EXECUTED|COLLECTED)\s+(?:0|ZERO)\s+(?:TESTS?|CASES?|ITEMS?)|"
    r"(?:TESTS?|CASES?)\s+RUN\s*:\s*(?:0|ZERO)|"
    r"(?:0|ZERO)\s+(?:TESTS?|CASES?|ITEMS?)\s+(?:WERE\s+)?(?:RUN|EXECUTED|COLLECTED)|"
    r"NO\s+(?:TESTS?|CASES?)\s+WERE\s+(?:RUN|EXECUTED)|NOTHING\s+TO\s+RUN)\b",
    re.I,
)
TRUSTED_BUILD_STDERR_RE = re.compile(
    r"^\s*(?:"
    r"(?:warning|note|help):.*|"
    r"(?:Compiling|Checking|Finished|Running|Doc-tests|Fresh|Blocking|Waiting)\b.*|"
    r"-->\s+.*|\|.*|\d+\s+\|.*|\^+.*|\.\.\.|"
    r"=\s*(?:note|help):.*"
    r")$"
)
RESULT_RECORD_RE = re.compile(
    r"\b(?:PASS|FAIL) OFFICIAL TEST GROUP|\b(?:PASS|FAIL) LTP CASE|"
    r"\btestcase busybox\b.*\b(?:success|fail)\b|\bFAIL libctest\b|\bPass!\s*$",
    re.I,
)
PROTOCOL_SIGNATURE_RE = re.compile(
    r"OS COMP TEST GROUP\s+(?:START|END)|"
    r"(?:PASS|FAIL) OFFICIAL TEST GROUP|"
    r"\bltp case list:|\bltp cases:|"
    r"\bRUN LTP CASE\b|\b(?:PASS|FAIL) LTP CASE\b|"
    r"\btestcase busybox\b|"
    r"=+\s+(?:START|END)\s+\S+\s+\S+\s+=+|"
    r"\bFAIL libctest\b|\blibctest cases:|\bPass!\s*$",
    re.I,
)
STRICT_PROTOCOL_PATTERNS = (
    ("group-start", GROUP_START_RE),
    ("group-end", GROUP_END_RE),
    ("official-pass", OFFICIAL_PASS_RE),
    ("official-fail", OFFICIAL_FAIL_RE),
    ("ltp-list", LTP_LIST_RE),
    ("ltp-start", LTP_START_RE),
    ("ltp-run", LTP_RUN_RE),
    ("ltp-result", LTP_RESULT_RE),
    ("ltp-end", LTP_END_RE),
    ("ltp-summary", LTP_SUMMARY_RE),
    ("busybox-result", BUSYBOX_RESULT_RE),
    ("libctest-start", LIBCTEST_START_RE),
    ("libctest-end", LIBCTEST_END_RE),
    ("libctest-fail", LIBCTEST_FAIL_RE),
    ("libctest-summary", LIBCTEST_SUMMARY_RE),
)


@dataclass
class Group:
    label: str
    lines: list[str] = field(default_factory=list)


def issue(kind: str, message: str, group: str | None = None) -> dict[str, str]:
    item = {"kind": kind, "message": message}
    if group is not None:
        item["group"] = group
    return item


def _duplicates(values: list[str]) -> list[str]:
    seen: set[str] = set()
    duplicates: set[str] = set()
    for value in values:
        if value in seen:
            duplicates.add(value)
        seen.add(value)
    return sorted(duplicates)


def _protocol_record_kinds(line: str) -> set[str]:
    kinds = {
        kind
        for kind, pattern in STRICT_PROTOCOL_PATTERNS
        if pattern.fullmatch(line)
    }
    if line == "Pass!":
        kinds.add("case-pass")
    return kinds


def _validate_ltp(
    group: Group,
    expected_cases: list[str] | None = None,
) -> tuple[list[dict[str, str]], dict[str, Any]]:
    errors: list[dict[str, str]] = []
    failures: list[dict[str, str]] = []
    list_records = [
        (index, match.groups())
        for index, line in enumerate(group.lines)
        if (match := LTP_LIST_RE.fullmatch(line))
    ]
    lists = [record for _index, record in list_records]
    starts = [match.group(1) for line in group.lines if (match := LTP_START_RE.fullmatch(line))]
    runs = [match.group(1) for line in group.lines if (match := LTP_RUN_RE.fullmatch(line))]
    results = [
        (match.group(1), int(match.group(2)))
        for line in group.lines
        if (match := LTP_RESULT_RE.fullmatch(line))
    ]
    ends = [match.group(1) for line in group.lines if (match := LTP_END_RE.fullmatch(line))]
    summary_records = [
        (index, tuple(int(value or 0) for value in match.groups()))
        for index, line in enumerate(group.lines)
        if (match := LTP_SUMMARY_RE.fullmatch(line))
    ]
    summaries = [record for _index, record in summary_records]
    timeout_cases = [
        match.group(1)
        for line in group.lines
        if (match := re.search(r"\bTIMEOUT LTP CASE\s+(\S+)", line, re.I))
    ]
    event_sequences: dict[str, list[str]] = {}
    active_case: str | None = None
    for line in group.lines:
        if match := LTP_START_RE.fullmatch(line):
            if active_case is not None:
                errors.append(
                    issue(
                        "ltp-nested-case",
                        f"{match.group(1)} started before {active_case} ended",
                        group.label,
                    )
                )
            active_case = match.group(1)
            event_sequences.setdefault(active_case, []).append("START")
        elif match := LTP_RUN_RE.fullmatch(line):
            event_sequences.setdefault(match.group(1), []).append("RUN")
        elif match := LTP_RESULT_RE.fullmatch(line):
            event_sequences.setdefault(match.group(1), []).append("RESULT")
        elif line == "Pass!":
            if active_case is None:
                errors.append(issue("ltp-orphan-pass", "Pass! appeared outside an active LTP case", group.label))
            else:
                event_sequences.setdefault(active_case, []).append("PASS")
        elif match := LTP_END_RE.fullmatch(line):
            event_sequences.setdefault(match.group(1), []).append("END")
            if active_case == match.group(1):
                active_case = None
            else:
                errors.append(
                    issue(
                        "ltp-mismatched-case-end",
                        f"ended {match.group(1)} while active case was {active_case!r}",
                        group.label,
                    )
                )

    if len(lists) != 1:
        errors.append(issue("ltp-manifest-count", f"expected one LTP case-list manifest, found {len(lists)}", group.label))
        planned = None
    else:
        planned = int(lists[0][1])
        if planned <= 0:
            errors.append(issue("ltp-empty", "LTP case-list manifest selected zero cases", group.label))
        first_start_index = next(
            (
                index
                for index, line in enumerate(group.lines)
                if LTP_START_RE.fullmatch(line)
            ),
            None,
        )
        if first_start_index is not None and list_records[0][0] >= first_start_index:
            errors.append(
                issue(
                    "ltp-manifest-order",
                    "LTP case-list manifest must precede the first case start",
                    group.label,
                )
            )

    for label, values in (("start", starts), ("RUN", runs), ("result", [name for name, _ in results]), ("end", ends)):
        duplicates = _duplicates(values)
        if duplicates:
            errors.append(issue("ltp-duplicate-case", f"duplicate {label} records: {', '.join(duplicates)}", group.label))

    if expected_cases is not None and starts != expected_cases:
        mismatch_index = next(
            (
                index
                for index, (observed, expected) in enumerate(zip(starts, expected_cases))
                if observed != expected
            ),
            min(len(starts), len(expected_cases)),
        )
        observed = starts[mismatch_index] if mismatch_index < len(starts) else "<missing>"
        expected = expected_cases[mismatch_index] if mismatch_index < len(expected_cases) else "<none>"
        errors.append(
            issue(
                "ltp-case-plan-mismatch",
                f"case sequence diverges at index {mismatch_index}: expected {expected!r}, observed {observed!r}; "
                f"expected {len(expected_cases)} identities, observed {len(starts)}",
                group.label,
            )
        )

    result_names = [name for name, _code in results]
    case_sets = {"start": set(starts), "RUN": set(runs), "result": set(result_names), "end": set(ends)}
    all_cases = set().union(*case_sets.values())
    if not all_cases:
        errors.append(issue("ltp-no-execution", "no LTP cases executed", group.label))
    for label, values in case_sets.items():
        missing = sorted(all_cases - values)
        if missing:
            errors.append(issue("ltp-incomplete-case", f"missing {label} record for: {', '.join(missing)}", group.label))
    result_codes = {name: code for name, code in results}
    for case_name, sequence in sorted(event_sequences.items()):
        expected_sequence = ["START", "RUN", "RESULT"]
        if result_codes.get(case_name) == 0:
            expected_sequence.append("PASS")
        expected_sequence.append("END")
        if sequence != expected_sequence:
            errors.append(
                issue(
                    "ltp-event-order",
                    f"{case_name} events are {sequence}, expected {expected_sequence}",
                    group.label,
                )
            )
    if planned is not None and len(all_cases) != planned:
        errors.append(
            issue(
                "ltp-planned-executed-mismatch",
                f"manifest planned {planned} cases but observed {len(all_cases)} unique cases",
                group.label,
            )
        )

    if len(summaries) != 1:
        errors.append(issue("ltp-summary-count", f"expected one LTP suite summary, found {len(summaries)}", group.label))
    else:
        last_end_index = max(
            (
                index
                for index, line in enumerate(group.lines)
                if LTP_END_RE.fullmatch(line)
            ),
            default=-1,
        )
        summary_index = summary_records[0][0]
        trailing = [line for line in group.lines[summary_index + 1 :] if line.strip()]
        valid_terminal_status = len(trailing) == 1 and (
            (match := OFFICIAL_PASS_RE.fullmatch(trailing[0])) is not None
            and match.group(1) == group.label
            or (match := OFFICIAL_FAIL_RE.fullmatch(trailing[0])) is not None
            and match.group(1) == group.label
        )
        if summary_index <= last_end_index or (trailing and not valid_terminal_status):
            errors.append(
                issue(
                    "ltp-summary-order",
                    "LTP summary must follow every case end and precede at most one terminal group status",
                    group.label,
                )
            )
        passed, failed, timed_out = summaries[0]
        observed_passed = sum(code == 0 for _name, code in results)
        observed_failed = sum(code != 0 for _name, code in results)
        observed_timeouts = len(set(timeout_cases))
        if planned is not None and passed + failed != planned:
            errors.append(
                issue(
                    "ltp-summary-mismatch",
                    f"summary completed {passed + failed} cases but manifest planned {planned}",
                    group.label,
                )
            )
        if timed_out > failed:
            errors.append(issue("ltp-summary-malformed", "timed-out count exceeds failed count", group.label))
        if (passed, failed, timed_out) != (observed_passed, observed_failed, observed_timeouts):
            errors.append(
                issue(
                    "ltp-summary-result-mismatch",
                    "summary counts do not exactly match observed numeric results and timeout records: "
                    f"summary={(passed, failed, timed_out)}, "
                    f"observed={(observed_passed, observed_failed, observed_timeouts)}",
                    group.label,
                )
            )
        if failed or timed_out:
            failures.append(
                issue(
                    "ltp-summary-failure",
                    f"summary reports {failed} failed and {timed_out} timed out",
                    group.label,
                )
            )

    nonzero = [(name, code) for name, code in results if code != 0]
    if nonzero:
        failures.append(
            issue(
                "ltp-nonzero-result",
                ", ".join(f"{name}={code}" for name, code in nonzero),
                group.label,
            )
        )
    return errors + failures, {
        "case_list_name": lists[0][0] if len(lists) == 1 else None,
        "planned_cases": planned,
        "executed_cases": len(all_cases),
        "result_cases": len(results),
        "passed_cases": sum(code == 0 for _name, code in results),
        "failed_cases": sum(code != 0 for _name, code in results),
    }


def _validate_busybox(
    group: Group,
    expected_cases: list[str] | None = None,
) -> tuple[list[dict[str, str]], dict[str, int]]:
    results = [
        (match.group(1), match.group(2).lower())
        for line in group.lines
        if (match := BUSYBOX_RESULT_RE.fullmatch(line))
    ]
    passed = sum(status == "success" for _case, status in results)
    failed = sum(status == "fail" for _case, status in results)
    issues: list[dict[str, str]] = []
    duplicate_cases = _duplicates([case for case, _status in results])
    if duplicate_cases:
        issues.append(
            issue(
                "busybox-duplicate-case",
                f"duplicate busybox case results: {', '.join(duplicate_cases)}",
                group.label,
            )
        )
    observed_cases = [case for case, _status in results]
    if expected_cases is not None and observed_cases != expected_cases:
        mismatch_index = next(
            (
                index
                for index, (observed, expected) in enumerate(zip(observed_cases, expected_cases))
                if observed != expected
            ),
            min(len(observed_cases), len(expected_cases)),
        )
        observed = observed_cases[mismatch_index] if mismatch_index < len(observed_cases) else "<missing>"
        expected = expected_cases[mismatch_index] if mismatch_index < len(expected_cases) else "<none>"
        issues.append(
            issue(
                "busybox-case-plan-mismatch",
                f"case sequence diverges at index {mismatch_index}: expected {expected!r}, "
                f"observed {observed!r}; expected {len(expected_cases)} identities, "
                f"observed {len(observed_cases)}",
                group.label,
            )
        )
    if passed + failed == 0:
        issues.append(issue("busybox-empty", "busybox group contains no case results", group.label))
    if failed:
        issues.append(issue("busybox-failure", f"{failed} busybox cases failed", group.label))
    return issues, {"executed_cases": passed + failed, "passed_cases": passed, "failed_cases": failed}


def _validate_libctest(
    group: Group,
    expected_cases: list[tuple[str, str]] | None = None,
) -> tuple[list[dict[str, str]], dict[str, int]]:
    summary_records = [
        (index, tuple(int(value) for value in match.groups()))
        for index, line in enumerate(group.lines)
        if (match := LIBCTEST_SUMMARY_RE.fullmatch(line))
    ]
    summaries = [record for _index, record in summary_records]
    issues: list[dict[str, str]] = []
    current: tuple[str, str] | None = None
    current_terminal: str | None = None
    observed_passed = 0
    observed_failed = 0
    starts = 0
    seen_cases: set[tuple[str, str]] = set()
    started_cases: list[tuple[str, str]] = []
    for line in group.lines:
        if match := LIBCTEST_START_RE.fullmatch(line):
            started_case = (match.group(1), match.group(2))
            started_cases.append(started_case)
            if current is not None:
                issues.append(
                    issue(
                        "libctest-nested-case",
                        f"{match.group(1)} {match.group(2)} started before {current[0]} {current[1]} ended",
                        group.label,
                    )
                )
            else:
                if started_case in seen_cases:
                    issues.append(
                        issue("libctest-duplicate-case", f"duplicate libctest case: {started_case}", group.label)
                    )
                seen_cases.add(started_case)
                current = started_case
                current_terminal = None
                starts += 1
            continue
        if line.strip() == "Pass!":
            if current is None:
                issues.append(issue("libctest-orphan-result", "Pass! appeared outside a case", group.label))
            elif current_terminal is not None:
                issues.append(issue("libctest-duplicate-result", f"duplicate result for {current}", group.label))
            else:
                current_terminal = "PASS"
                observed_passed += 1
            continue
        if match := LIBCTEST_FAIL_RE.fullmatch(line):
            failed_case = (match.group(1), match.group(2))
            if current is None or failed_case != current:
                issues.append(issue("libctest-orphan-result", f"failure record does not match active case: {failed_case}", group.label))
            elif current_terminal is not None:
                issues.append(issue("libctest-duplicate-result", f"duplicate result for {current}", group.label))
            else:
                current_terminal = "FAIL"
                observed_failed += 1
            continue
        if match := LIBCTEST_END_RE.fullmatch(line):
            ended = (match.group(1), match.group(2))
            if current is None or ended != current:
                issues.append(issue("libctest-mismatched-end", f"case end does not match active case: {ended}", group.label))
            elif current_terminal is None:
                issues.append(issue("libctest-missing-result", f"case {current} ended without Pass!/FAIL", group.label))
            current = None
            current_terminal = None
    if expected_cases is not None and started_cases != expected_cases:
        mismatch_index = next(
            (
                index
                for index, (observed, expected) in enumerate(zip(started_cases, expected_cases))
                if observed != expected
            ),
            min(len(started_cases), len(expected_cases)),
        )
        observed = started_cases[mismatch_index] if mismatch_index < len(started_cases) else "<missing>"
        expected = expected_cases[mismatch_index] if mismatch_index < len(expected_cases) else "<none>"
        issues.append(
            issue(
                "libctest-case-plan-mismatch",
                f"case sequence diverges at index {mismatch_index}: expected {expected!r}, "
                f"observed {observed!r}; expected {len(expected_cases)} identities, "
                f"observed {len(started_cases)}",
                group.label,
            )
        )
    if current is not None:
        issues.append(issue("libctest-missing-end", f"case {current} did not emit its end marker", group.label))
    if len(summaries) != 1:
        issues.append(issue("libctest-summary-count", f"expected one libctest summary, found {len(summaries)}", group.label))
        return issues, {"executed_cases": starts, "passed_cases": 0, "failed_cases": 0}
    last_end_index = max(
        (
            index
            for index, line in enumerate(group.lines)
            if LIBCTEST_END_RE.fullmatch(line)
        ),
        default=-1,
    )
    summary_index = summary_records[0][0]
    trailing = [line for line in group.lines[summary_index + 1 :] if line.strip()]
    valid_terminal_status = len(trailing) == 1 and (
        (match := OFFICIAL_PASS_RE.fullmatch(trailing[0])) is not None
        and match.group(1) == group.label
        or (match := OFFICIAL_FAIL_RE.fullmatch(trailing[0])) is not None
        and match.group(1) == group.label
    )
    if summary_index <= last_end_index or (trailing and not valid_terminal_status):
        issues.append(
            issue(
                "libctest-summary-order",
                "libctest summary must follow every case end and precede at most one terminal group status",
                group.label,
            )
        )
    passed, failed, timed_out = summaries[0]
    completed = passed + failed
    if completed <= 0:
        issues.append(issue("libctest-empty", "libctest summary completed zero cases", group.label))
    if starts != completed:
        issues.append(
            issue(
                "libctest-count-mismatch",
                f"observed {starts} case starts but summary completed {completed}",
                group.label,
            )
        )
    if (passed, failed) != (observed_passed, observed_failed):
        issues.append(
            issue(
                "libctest-summary-result-mismatch",
                f"summary={(passed, failed)} does not match observed={(observed_passed, observed_failed)}",
                group.label,
            )
        )
    if failed or timed_out:
        issues.append(
            issue(
                "libctest-failure",
                f"summary reports {failed} failed and {timed_out} timed out",
                group.label,
            )
        )
    return issues, {"executed_cases": completed, "passed_cases": passed, "failed_cases": failed}


def _validate_generic(group: Group) -> tuple[list[dict[str, str]], dict[str, int]]:
    passes = [
        (match.group(1), int(match.group(2)))
        for line in group.lines
        if (match := OFFICIAL_PASS_RE.fullmatch(line))
    ]
    failures = [
        (match.group(1), int(match.group(2)))
        for line in group.lines
        if (match := OFFICIAL_FAIL_RE.fullmatch(line))
    ]
    issues: list[dict[str, str]] = []
    expected_passes = [(group.label, 0)]
    matching_passes = [(label, code) for label, code in passes if label == group.label and code == 0]
    if not failures and passes != expected_passes:
        issues.append(
            issue(
                "group-success-record",
                f"expected exactly {expected_passes}, observed {passes}",
                group.label,
            )
        )
    if passes == expected_passes:
        terminal_index = max(
            (index for index, line in enumerate(group.lines) if line.strip()),
            default=-1,
        )
        pass_index = next(
            index
            for index, line in enumerate(group.lines)
            if OFFICIAL_PASS_RE.fullmatch(line)
        )
        trailing_lines = [line for line in group.lines[pass_index + 1 :] if line.strip()]
        trailing_has_failure = any(
            FORBIDDEN_STATUS_RE.search(line)
            or TIMEOUT_RE.search(line)
            or SKIP_RE.search(line)
            or PANIC_RE.search(line)
            or UNCONSUMED_FAILURE_RE.search(line)
            or EXPLICIT_NONZERO_RE.search(line)
            for line in trailing_lines
        )
        if pass_index != terminal_index and not trailing_has_failure:
            issues.append(
                issue(
                    "group-success-order",
                    "explicit group success must be the terminal group record",
                    group.label,
                )
            )
    if failures:
        issues.append(issue("official-group-failure", f"explicit failure records: {failures}", group.label))
    return issues, {
        "executed_cases": 1 if passes or failures else 0,
        "passed_cases": len(matching_passes),
        "failed_cases": len(failures),
    }


def validate_official_output(
    stdout: str,
    stderr: str = "",
    expected_group_labels: list[str] | None = None,
    expected_group_case_counts: dict[str, int] | None = None,
    expected_ltp_case_list: str | None = None,
    expected_ltp_cases: list[str] | None = None,
    expected_busybox_cases: list[str] | None = None,
    expected_libctest_cases: list[tuple[str, str]] | None = None,
) -> dict[str, Any]:
    """Return a strict PASS/FAIL/ERROR result for captured official output."""

    text = normalize_output_text(stdout)
    stderr_text = normalize_output_text(stderr)
    lines = text.splitlines()
    structural_errors: list[dict[str, str]] = []
    failures: list[dict[str, str]] = []
    groups: list[Group] = []
    labels: set[str] = set()
    current: Group | None = None
    outside_lines: list[str] = []
    case_count_plan = expected_group_case_counts or {}

    for source, cleaned_text in (("stdout", text), ("stderr", stderr_text)):
        if invalid_character := first_unsupported_output_character(cleaned_text):
            structural_errors.append(
                issue(
                    "invalid-output-control",
                    f"{source} contains unsupported output character "
                    f"U+{ord(invalid_character):04X}",
                )
            )

    if expected_ltp_case_list is not None and (
        not isinstance(expected_ltp_case_list, str) or not expected_ltp_case_list
    ):
        structural_errors.append(
            issue("official-ltp-case-list-plan", "expected LTP case-list name must be non-empty")
        )
    if expected_ltp_cases is not None and (
        not isinstance(expected_ltp_cases, list)
        or not expected_ltp_cases
        or any(not isinstance(case, str) or not case for case in expected_ltp_cases)
        or len(expected_ltp_cases) != len(set(expected_ltp_cases))
    ):
        structural_errors.append(
            issue("official-ltp-case-plan", "expected LTP case identities must be non-empty and unique")
        )
        expected_ltp_cases = None
    if expected_busybox_cases is not None and (
        not isinstance(expected_busybox_cases, list)
        or not expected_busybox_cases
        or any(not isinstance(case, str) or not case for case in expected_busybox_cases)
    ):
        structural_errors.append(
            issue("official-busybox-case-plan", "expected BusyBox identities must be non-empty strings")
        )
        expected_busybox_cases = None
    if expected_libctest_cases is not None and (
        not isinstance(expected_libctest_cases, list)
        or not expected_libctest_cases
        or any(
            not isinstance(case, (list, tuple))
            or len(case) != 2
            or any(not isinstance(value, str) or not value for value in case)
            for case in expected_libctest_cases
        )
        or len(expected_libctest_cases) != len(set(map(tuple, expected_libctest_cases)))
    ):
        structural_errors.append(
            issue("official-libctest-case-plan", "expected libctest identities must be non-empty and unique pairs")
        )
        expected_libctest_cases = None
    elif expected_libctest_cases is not None:
        expected_libctest_cases = [tuple(case) for case in expected_libctest_cases]

    if not isinstance(case_count_plan, dict):
        structural_errors.append(
            issue("official-case-count-plan", "expected group case counts must be a mapping")
        )
        case_count_plan = {}
    else:
        for label, count in case_count_plan.items():
            if (
                not isinstance(label, str)
                or not label
                or isinstance(count, bool)
                or not isinstance(count, int)
                or count <= 0
            ):
                structural_errors.append(
                    issue(
                        "official-case-count-plan",
                        f"invalid expected case count: {label!r}={count!r}",
                    )
                )

    if not text.strip():
        structural_errors.append(issue("empty-output", "official evaluation produced no output"))

    for line in lines:
        if match := GROUP_START_RE.fullmatch(line):
            label = match.group(1)
            if current is not None:
                structural_errors.append(
                    issue("nested-group", f"group {label} started before {current.label} ended", current.label)
                )
                continue
            if label in labels:
                structural_errors.append(issue("duplicate-group", f"group {label} ran more than once", label))
            labels.add(label)
            current = Group(label)
            groups.append(current)
            continue
        if match := GROUP_END_RE.fullmatch(line):
            label = match.group(1)
            if current is None:
                structural_errors.append(issue("unmatched-group-end", f"group end without start: {label}", label))
            elif current.label != label:
                structural_errors.append(
                    issue("mismatched-group-end", f"started {current.label} but ended {label}", current.label)
                )
                current = None
            else:
                current = None
            continue
        if current is not None:
            current.lines.append(line)
        else:
            outside_lines.append(line)

    if current is not None:
        structural_errors.append(issue("missing-group-end", "group did not emit its end marker", current.label))
    if not groups:
        structural_errors.append(issue("zero-groups", "no official test groups executed"))
    observed_labels = [group.label for group in groups]
    if expected_group_labels is not None and observed_labels != expected_group_labels:
        structural_errors.append(
            issue(
                "official-group-plan-mismatch",
                f"expected ordered groups {expected_group_labels}, observed {observed_labels}",
            )
        )

    for line in outside_lines:
        if RESULT_RECORD_RE.search(line):
            structural_errors.append(issue("result-outside-group", line.strip()))

    allowed_protocols = {
        "ltp": {
            "official-pass", "official-fail", "ltp-list", "ltp-start",
            "ltp-run", "ltp-result", "ltp-end", "ltp-summary", "case-pass",
        },
        "busybox": {"official-pass", "official-fail", "busybox-result"},
        "libctest": {
            "official-pass", "official-fail", "libctest-start", "libctest-end",
            "libctest-fail", "libctest-summary", "case-pass",
        },
        "generic": {"official-pass", "official-fail"},
    }
    for source_group, protocol_lines in [
        *( (None, [line]) for line in outside_lines ),
        *( (group, group.lines) for group in groups ),
    ]:
        for line in protocol_lines:
            if not PROTOCOL_SIGNATURE_RE.search(line):
                continue
            kinds = _protocol_record_kinds(line)
            if not kinds:
                structural_errors.append(
                    issue(
                        "malformed-protocol-record",
                        line.strip(),
                        source_group.label if source_group is not None else None,
                    )
                )
                continue
            if source_group is None:
                if not kinds <= {"group-start", "group-end"}:
                    structural_errors.append(issue("protocol-record-outside-group", line.strip()))
                continue
            family = (
                "ltp" if source_group.label.startswith("ltp-")
                else "busybox" if source_group.label.startswith("busybox-")
                else "libctest" if source_group.label.startswith("libctest-")
                else "generic"
            )
            if not kinds & allowed_protocols[family]:
                structural_errors.append(
                    issue("unexpected-protocol-record", line.strip(), source_group.label)
                )

    stderr_lines = stderr_text.splitlines()
    for line in stderr_lines:
        if PROTOCOL_SIGNATURE_RE.search(line):
            structural_errors.append(issue("protocol-record-on-stderr", line.strip()))

    for source, line in [
        *(("stdout", line) for line in lines),
        *(("stderr", line) for line in stderr_lines),
    ]:
        if _protocol_record_kinds(line):
            # Strict machine records are interpreted by their group-specific
            # validator.  Their vocabulary legitimately includes strings such
            # as "FAIL ... : 0", "0 failed", and "timeout 180s".
            continue
        if UNKNOWN_STATE_RE.search(line) and not UNCONSUMED_FAILURE_RE.search(line):
            structural_errors.append(issue("unknown-status", f"{source}: {line.strip()}"))
        if ZERO_EXECUTION_RE.search(line):
            structural_errors.append(issue("zero-execution", f"{source}: {line.strip()}"))
        markers = FORBIDDEN_STATUS_RE.findall(line)
        for marker in markers:
            failures.append(issue("forbidden-status", f"{source}: {marker}"))
        if TIMEOUT_RE.search(line) and "TIMEOUT" not in markers:
            failures.append(issue("timeout", f"{source}: {line.strip()}"))
        if SKIP_RE.search(line) and not any(marker.startswith("SKIP") for marker in markers):
            failures.append(issue("skipped-group", f"{source}: {line.strip()}"))
        if PANIC_RE.search(line) and "PANIC" not in markers:
            failures.append(issue("panic-or-trap", f"{source}: {line.strip()}"))
        if UNCONSUMED_FAILURE_RE.search(line) and not _protocol_record_kinds(line):
            failures.append(issue("explicit-failure", f"{source}: {line.strip()}"))
        if EXPLICIT_NONZERO_RE.search(line):
            failures.append(issue("explicit-nonzero", f"{source}: {line.strip()}"))

    for line in [*outside_lines, *stderr_lines]:
        if OFFICIAL_FAIL_RE.search(line):
            failures.append(issue("official-group-failure", line.strip()))
        if INFRA_TEXT_RE.search(line):
            structural_errors.append(issue("runtime-infrastructure", line.strip()))

    for line in lines:
        if INFRA_TEXT_RE.search(line):
            structural_errors.append(issue("runtime-infrastructure", line.strip()))

    group_rows: list[dict[str, Any]] = []
    for group in groups:
        group_text = "\n".join(group.lines)
        group_issues: list[dict[str, str]]
        if group.label.startswith("ltp-"):
            group_issues, counts = _validate_ltp(group, expected_ltp_cases)
        elif group.label.startswith("busybox-"):
            group_issues, counts = _validate_busybox(group, expected_busybox_cases)
        elif group.label.startswith("libctest-"):
            group_issues, counts = _validate_libctest(group, expected_libctest_cases)
        else:
            group_issues, counts = _validate_generic(group)

        if group.label.startswith("ltp-") and expected_ltp_case_list is not None:
            observed_case_list = counts.get("case_list_name")
            if observed_case_list != expected_ltp_case_list:
                structural_errors.append(
                    issue(
                        "official-ltp-case-list-mismatch",
                        f"expected {expected_ltp_case_list!r}, observed {observed_case_list!r}",
                        group.label,
                    )
                )

        if group.label in case_count_plan or group.label.startswith(("busybox-", "libctest-")):
            expected_cases = case_count_plan.get(group.label)
            counts["expected_cases"] = expected_cases
            if expected_cases is None:
                structural_errors.append(
                    issue(
                        "official-case-count-missing",
                        "no trusted expected case count was supplied",
                        group.label,
                    )
                )
            else:
                if group.label.startswith("ltp-") and counts.get("planned_cases") != expected_cases:
                    structural_errors.append(
                        issue(
                            "official-reported-plan-mismatch",
                            f"trusted plan expects {expected_cases} cases but guest reported {counts.get('planned_cases')}",
                            group.label,
                        )
                    )
                if counts["executed_cases"] != expected_cases:
                    structural_errors.append(
                        issue(
                            "official-planned-executed-mismatch",
                            f"expected {expected_cases} cases but observed {counts['executed_cases']}",
                            group.label,
                        )
                    )

        if group.label.startswith(("ltp-", "busybox-", "libctest-")):
            explicit_passes = [
                (match.group(1), int(match.group(2)))
                for line in group.lines
                if (match := OFFICIAL_PASS_RE.fullmatch(line))
            ]
            explicit_failures = [
                (match.group(1), int(match.group(2)))
                for line in group.lines
                if (match := OFFICIAL_FAIL_RE.fullmatch(line))
            ]
            if explicit_passes not in ([], [(group.label, 0)]):
                structural_errors.append(
                    issue(
                        "specialized-group-success-record",
                        f"unexpected explicit success records: {explicit_passes}",
                        group.label,
                    )
                )
            if explicit_failures:
                failures.append(
                    issue(
                        "official-group-failure",
                        f"explicit failure records: {explicit_failures}",
                        group.label,
                    )
                )

        for item in group_issues:
            if item["kind"] in {
                "ltp-summary-failure",
                "ltp-nonzero-result",
                "busybox-failure",
                "libctest-failure",
                "official-group-failure",
            }:
                failures.append(item)
            else:
                structural_errors.append(item)
        group_rows.append({"label": group.label, **counts})

    count_labels_without_groups = sorted(set(case_count_plan) - set(observed_labels))
    if count_labels_without_groups:
        structural_errors.append(
            issue(
                "official-case-count-plan-mismatch",
                f"expected counts supplied for unobserved groups: {count_labels_without_groups}",
            )
        )

    unexpected_stderr = [
        line
        for line in stderr_lines
        if line.strip() and TRUSTED_BUILD_STDERR_RE.fullmatch(line) is None
    ]
    if unexpected_stderr and not failures:
        structural_errors.append(
            issue(
                "unaccounted-stderr-output",
                "stderr contains output outside the trusted build-diagnostic grammar: "
                + repr(unexpected_stderr[:5]),
            )
        )

    status = "ERROR" if structural_errors else "FAIL" if failures else "PASS"
    return {
        "status": status,
        "group_count": len(groups),
        "groups": group_rows,
        "error_count": len(structural_errors),
        "failure_count": len(failures),
        "errors": structural_errors,
        "failures": failures,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stdout", required=True, type=Path, help="captured evaluator stdout")
    parser.add_argument("--stderr", type=Path, help="captured evaluator stderr")
    parser.add_argument("--json", action="store_true", help="emit machine-readable validation")
    args = parser.parse_args(argv)

    try:
        stdout = args.stdout.read_text(encoding="utf-8", errors="strict")
        stderr = args.stderr.read_text(encoding="utf-8", errors="strict") if args.stderr else ""
    except (OSError, UnicodeDecodeError) as error:
        parser.error(str(error))
    try:
        expected_ltp_cases = trusted_ltp_stable_cases(Path(__file__).resolve().parents[2])
        expected_busybox_cases, expected_libctest_cases = trusted_official_case_plan(
            Path(__file__).resolve().parents[2]
        )
    except (OSError, ValueError) as error:
        parser.error(f"cannot load trusted official case plan: {error}")
    result = validate_official_output(
        stdout,
        stderr,
        expected_group_labels=list(CANONICAL_OFFICIAL_GROUPS),
        expected_group_case_counts=CANONICAL_OFFICIAL_CASE_COUNTS,
        expected_ltp_case_list=CANONICAL_LTP_CASE_LIST,
        expected_ltp_cases=expected_ltp_cases,
        expected_busybox_cases=expected_busybox_cases,
        expected_libctest_cases=expected_libctest_cases,
    )
    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(
            f"official result validation: {result['status']} "
            f"({result['group_count']} groups, {result['failure_count']} failures, "
            f"{result['error_count']} integrity errors)"
        )
        for item in result["errors"] + result["failures"]:
            location = f" [{item['group']}]" if "group" in item else ""
            print(f"- {item['kind']}{location}: {item['message']}")
    return {"PASS": 0, "FAIL": 1, "ERROR": 2}[result["status"]]


if __name__ == "__main__":
    raise SystemExit(main())
