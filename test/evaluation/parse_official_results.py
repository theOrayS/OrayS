#!/usr/bin/env python3
"""Strictly validate completion and result integrity in a local official run.

The guest evaluator currently emits several kinds of result records.  This
validator accepts only complete, paired groups with a group-specific success
contract.  It never converts a missing result, an empty group, a configured
skip, or the absence of a failure substring into PASS.
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
import json
import re
import stat
import unicodedata
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

ANSI_SGR_RE = re.compile(r"\x1b\[(?:[0-9]{1,3}(?:;[0-9]{1,3})*)?[mK]")
ANSI_CLEAR_HOME_RE = re.compile(r"\x1b\[H\x1b\[J")
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
ARCH_TOKEN_RE = re.compile(
    r"(?<![a-z0-9])(rv|riscv|riscv64|la|loongarch|loongarch64)(?![a-z0-9])",
    re.IGNORECASE,
)
ARCH_ALIASES = {
    "rv": "rv",
    "riscv": "rv",
    "riscv64": "rv",
    "la": "la",
    "loongarch": "la",
    "loongarch64": "la",
}
BUSYBOX_EXPLICIT_ID_RE = re.compile(r"[A-Za-z0-9][A-Za-z0-9._:-]*")


def infer_capture_arch_tokens(path: Path) -> set[str]:
    """Return exact, separator-delimited architecture tokens in a capture name."""

    return {ARCH_ALIASES[token.lower()] for token in ARCH_TOKEN_RE.findall(path.name)}


def capture_source_key(path: Path, stream: str) -> str:
    """Return the case-sensitive source key shared by paired capture streams."""

    raw_name = path.name
    lowered_name = raw_name.lower()
    if stream == "stdout":
        if lowered_name.endswith(".stderr.log"):
            raise ValueError(f"stdout capture path has stderr suffix: {path}")
        suffix = ".stdout.log" if lowered_name.endswith(".stdout.log") else ".log"
    elif stream == "stderr":
        suffix = ".stderr.log"
    else:
        raise ValueError(f"unknown capture stream: {stream}")
    if not lowered_name.endswith(suffix):
        raise ValueError(f"{stream} capture filename must end with {suffix}: {path}")
    key = raw_name[: -len(suffix)]
    if not key:
        raise ValueError(f"{stream} capture filename has an empty source key: {path}")
    return key


def validate_capture_input_pairs(
    stdout_paths: list[Path],
    stderr_paths: list[Path],
    required_arches: set[str] | None = None,
) -> list[dict[str, Any]]:
    """Bind stdout/stderr captures by path, source key, identity, and architecture.

    With ``required_arches=None``, architecture-free names are accepted, but if
    either stream names an architecture then both must name the same single
    architecture.  Promotion callers pass the exact required architecture set.
    """

    if len(stdout_paths) != len(stderr_paths):
        raise ValueError("requires exactly one stderr capture for each stdout capture")
    if not stdout_paths:
        raise ValueError("capture input pair list must not be empty")

    resolved_paths: set[Path] = set()
    file_identities: set[tuple[int, int]] = set()
    observed_arches: set[str] = set()
    pairs: list[dict[str, Any]] = []
    for stdout_path, stderr_path in zip(stdout_paths, stderr_paths):
        try:
            stdout_resolved = stdout_path.resolve()
            stderr_resolved = stderr_path.resolve()
        except OSError as error:
            raise ValueError(f"cannot resolve capture input path: {error}") from error

        for resolved in (stdout_resolved, stderr_resolved):
            if resolved in resolved_paths:
                raise ValueError(f"capture inputs must reference unique files: {resolved}")
            resolved_paths.add(resolved)
            try:
                file_status = resolved.stat()
            except OSError as error:
                raise ValueError(f"cannot stat capture input {resolved}: {error}") from error
            if not stat.S_ISREG(file_status.st_mode):
                raise ValueError(f"capture input is not a regular file: {resolved}")
            identity = (file_status.st_dev, file_status.st_ino)
            if identity in file_identities:
                raise ValueError(
                    f"capture inputs must not alias the same physical file: {resolved}"
                )
            file_identities.add(identity)

        if stdout_resolved.parent != stderr_resolved.parent:
            raise ValueError(
                "stdout/stderr companions must be in the same resolved directory: "
                f"{stdout_path} + {stderr_path}"
            )
        stdout_key = capture_source_key(stdout_path, "stdout")
        stderr_key = capture_source_key(stderr_path, "stderr")
        if stdout_key != stderr_key:
            raise ValueError(
                "stdout/stderr companions must have the same case-sensitive source key: "
                f"{stdout_key!r} != {stderr_key!r}"
            )

        stdout_arches = infer_capture_arch_tokens(stdout_path)
        stderr_arches = infer_capture_arch_tokens(stderr_path)
        if len(stdout_arches) > 1 or len(stderr_arches) > 1:
            raise ValueError(
                "capture filenames must not identify multiple architectures: "
                f"{stdout_path} + {stderr_path}"
            )
        arch: str | None
        if required_arches is not None:
            if len(stdout_arches) != 1 or len(stderr_arches) != 1:
                raise ValueError(
                    "promotion capture filenames must each identify exactly one rv or la "
                    f"architecture: {stdout_path} + {stderr_path}"
                )
            arch = next(iter(stdout_arches))
            if stderr_arches != {arch}:
                raise ValueError(
                    "stdout/stderr companions identify different architectures: "
                    f"{stdout_path} + {stderr_path}"
                )
            if arch not in required_arches:
                raise ValueError(f"capture architecture {arch} is not in the required matrix")
            if arch in observed_arches:
                raise ValueError(f"capture inputs contain more than one pair for architecture {arch}")
            observed_arches.add(arch)
        elif stdout_arches or stderr_arches:
            if len(stdout_arches) != 1 or stdout_arches != stderr_arches:
                raise ValueError(
                    "stdout/stderr companions must identify the same single architecture: "
                    f"{stdout_path} + {stderr_path}"
                )
            arch = next(iter(stdout_arches))
        else:
            arch = None

        pairs.append(
            {
                "pair_id": f"{arch or 'capture'}:{stdout_key}",
                "source_key": stdout_key,
                "arch": arch,
                "stdout_path": stdout_path,
                "stderr_path": stderr_path,
                "stdout_resolved_path": stdout_resolved,
                "stderr_resolved_path": stderr_resolved,
            }
        )

    if required_arches is not None and observed_arches != required_arches:
        raise ValueError(
            "capture inputs must provide exactly one stdout/stderr pair for each required architecture"
        )
    return pairs


def normalize_output_text(raw_text: str) -> str:
    """Remove trusted styling/clear pairs and normalize CRLF for parsing."""

    without_clear = ANSI_CLEAR_HOME_RE.sub("", raw_text)
    return ANSI_SGR_RE.sub("", without_clear).replace("\r\n", "\n")


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


@dataclass(frozen=True)
class BusyBoxCase:
    """Stable identity and evidence payload for one ordered BusyBox command."""

    ordinal: int
    command: str
    explicit_id: str | None = None


def trusted_official_case_plan(
    repo: Path,
) -> tuple[list[BusyBoxCase], list[tuple[str, str]]]:
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
        "busybox_cases",
        "libctest_cases",
    }:
        raise ValueError("official case plan must use the exact schema-v2 top-level fields")
    if type(payload["schema_version"]) is not int or payload["schema_version"] != 2:
        raise ValueError("official case plan schema_version must be 2")

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

    raw_busybox_cases = payload["busybox_cases"]
    if not isinstance(raw_busybox_cases, list):
        raise ValueError("official case plan busybox_cases must be a list")
    busybox_cases: list[BusyBoxCase] = []
    explicit_ids: set[str] = set()
    for index, entry in enumerate(raw_busybox_cases):
        location = f"official case plan busybox_cases[{index}]"
        if (
            not isinstance(entry, dict)
            or not {"ordinal", "command"} <= set(entry)
            or not set(entry) <= {"ordinal", "command", "id"}
        ):
            raise ValueError(f"{location} has missing or unknown fields")
        ordinal = entry["ordinal"]
        if type(ordinal) is not int or ordinal != index + 1:
            raise ValueError(
                f"{location} ordinal must be the contiguous one-based value {index + 1}"
            )
        command = entry["command"]
        if (
            not isinstance(command, str)
            or not command
            or command != command.strip()
            or "\n" in command
            or "\r" in command
            or first_unsupported_output_character(command) is not None
        ):
            raise ValueError(f"{location} command must be one non-empty trimmed safe line")
        explicit_id = entry.get("id")
        if explicit_id is not None:
            if (
                not isinstance(explicit_id, str)
                or BUSYBOX_EXPLICIT_ID_RE.fullmatch(explicit_id) is None
            ):
                raise ValueError(f"{location} id is not a stable explicit ID")
            if explicit_id in explicit_ids:
                raise ValueError(
                    f"official case plan has duplicate BusyBox explicit ID: {explicit_id}"
                )
            explicit_ids.add(explicit_id)
        busybox_cases.append(BusyBoxCase(ordinal, command, explicit_id))

    if len(busybox_cases) != CANONICAL_OFFICIAL_CASE_COUNTS["busybox-musl"]:
        raise ValueError("official case plan has the wrong BusyBox row count")
    if snapshot["busybox_row_count"] != len(busybox_cases):
        raise ValueError("official case plan BusyBox row metadata does not match its rows")
    busybox_commands = [case.command for case in busybox_cases]
    if snapshot["busybox_unique_count"] != len(set(busybox_commands)):
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
    return busybox_cases, libctest_cases
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
LTP_DIAGNOSTIC_STATUS_RE = re.compile(r"\b(TPASS|TINFO|TWARN|TFAIL|TBROK|TCONF)\b", re.I)
LTP_INTERNAL_SUMMARY_RE = re.compile(
    r"^\s*(passed|failed|broken|skipped|warnings)\s+(\d+)\s*$",
    re.I,
)
LTP_FATAL_DIAGNOSTIC_RE = re.compile(
    r"\b(?:kernel panic|unknown trap|unhandled(?: user)? trap|fatal trap|"
    r"InstructionNotExist|IllegalInstruction|illegal instruction|SegmentationFault|"
    r"segmentation fault|segfault|bus error|core dump(?:ed)?|process crash(?:ed)?|"
    r"crashed)\b",
    re.I,
)
LTP_TIMEOUT_EVENT_RE = re.compile(
    r"\b(?:timed out after|timeout reached|timeout expired|killed after timeout|"
    r"command timed out|deadline exceeded|watchdog expired|"
    r"time[_ -]?limit[_ -]?exceeded|timeouted|deadline_exceeded|"
    r"watchdog_expired|command_timed_out|timeout_error)\b",
    re.I,
)
BUSYBOX_CASE_START_RE = re.compile(
    r"^#### OS COMP BUSYBOX CASE START ordinal=([1-9]\d*) ####$"
)
BUSYBOX_CASE_RESULT_RE = re.compile(
    r"^BUSYBOX CASE RESULT ordinal=([1-9]\d*) "
    r"status=(success|fail) command=(.+)$"
)
BUSYBOX_CASE_END_RE = re.compile(
    r"^#### OS COMP BUSYBOX CASE END ordinal=([1-9]\d*) ####$"
)
BUSYBOX_LEGACY_RESULT_RE = re.compile(
    r"^testcase busybox\s+(.+?)\s+(success|fail)\s*$"
)
LIBCTEST_START_RE = re.compile(r"^=+ START\s+(\S+)\s+(\S+)\s+=+$")
LIBCTEST_END_RE = re.compile(r"^=+ END\s+(\S+)\s+(\S+)\s+=+$")
LIBCTEST_FAIL_RE = re.compile(r"^FAIL libctest\s+(\S+)\s+(\S+)\s*:\s*(.+)\s*$")
LIBCTEST_SUMMARY_RE = re.compile(
    r"^libctest cases:\s+(\d+)\s+passed,\s+(\d+)\s+failed,\s+(\d+)\s+timed out\s*$",
    re.I,
)
FORBIDDEN_STATUS_RE = re.compile(
    r"\b(TCONF|TBROK|TFAIL|ENOSYS|XFAIL|SKIP(?:PED)?|TIMEOUT|TIMED[_ -]?OUT|"
    r"TIME[_ -]?LIMIT[_ -]?EXCEEDED|HANG|CRASH|PANIC|ERROR)\b",
    re.I,
)
TIMEOUT_RE = re.compile(
    r"\b(?:TIMEOUT (?:LTP CASE|OFFICIAL TEST GROUP)|timed out after|timeout reached|"
    r"timeout expired|killed after timeout|command timed out|deadline exceeded|"
    r"watchdog expired|time[_ -]?limit[_ -]?exceeded|timed[_ -]?out|"
    r"timeouted|deadline_exceeded|watchdog_expired|command_timed_out|timeout_error|ETIMEDOUT)\b",
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
GENERIC_SUBTEST_FAILURE_RE = re.compile(
    r"^\s*=+\s*(?:"
    r"(?:iperf|netperf|cyclictest)\b.*\bend\s*:\s*fail|"
    r"kill\s+\S+\s*:\s*fail(?:\s*,.*)?"
    r")\s*=+\s*$",
    re.I,
)
EXPLICIT_NONZERO_RE = re.compile(
    r"^\s*(?:not successful|exit status\s+[1-9]\d*|return\s*:\s*[1-9]\d*|"
    r"autorun:\s+.+\s+exited with status\s+[1-9]\d*|"
    r"warning:\s+(?:(?:subprocess|command)(?:\s+.*?)?\s+"
    r"(?:(?:exited|failed)\s+with\s+(?:exit\s+)?(?:status|code)|"
    r"returned(?:\s+(?:status|code))?)\s*[:=]?\s*[1-9]\d*|"
    r"qemu(?:\s+.*?)?\s+exit (?:status|code)\s*[:=]?\s*[1-9]\d*))\s*$",
    re.I,
)
OFFICIAL_TIMEOUT_BUDGET_RE = re.compile(
    r"^autorun:\s+\S+\s+timeout bounded to \d+s \(nominal \d+s\)$",
    re.I,
)
CONTROLLED_PROCESS_CLEANUP_RE = re.compile(
    r"^(?:Signal (?:2|15) caught, longjmp'ing out!|"
    r"sending SIGTERM to all child processes|"
    r"signaling \d+ worker threads to terminate)$",
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
    r"\bBUSYBOX CASE RESULT\b|\btestcase busybox\b.*\b(?:success|fail)\b|"
    r"\bFAIL libctest\b|\bPass!\s*$",
    re.I,
)
PROTOCOL_SIGNATURE_RE = re.compile(
    r"OS COMP TEST GROUP\s+(?:START|END)|"
    r"(?:PASS|FAIL) OFFICIAL TEST GROUP|"
    r"\bltp case list:|\bltp cases:|"
    r"\bRUN LTP CASE\b|\b(?:PASS|FAIL) LTP CASE\b|"
    r"OS COMP BUSYBOX CASE\s+(?:START|END)|\bBUSYBOX CASE RESULT\b|"
    r"\btestcase busybox\b|"
    r"=+\s+(?:START|END)\s+\S+\s+\S+\s+=+|"
    r"\bFAIL libctest\b|\blibctest cases:|\bPass!",
    re.I,
)
LTP_PROTOCOL_SIGNATURE_RE = re.compile(
    r"\bltp case list:|\bltp cases:|\bRUN LTP CASE\b|"
    r"\b(?:PASS|FAIL) LTP CASE\b|=+\s+(?:START|END)\s+ltp\b",
    re.I,
)
GROUP_PROTOCOL_SIGNATURE_RE = re.compile(
    r"OS COMP TEST GROUP\s+(?:START|END)",
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
    ("busybox-start", BUSYBOX_CASE_START_RE),
    ("busybox-result", BUSYBOX_CASE_RESULT_RE),
    ("busybox-end", BUSYBOX_CASE_END_RE),
    ("busybox-legacy-result", BUSYBOX_LEGACY_RESULT_RE),
    ("libctest-start", LIBCTEST_START_RE),
    ("libctest-end", LIBCTEST_END_RE),
    ("libctest-fail", LIBCTEST_FAIL_RE),
    ("libctest-summary", LIBCTEST_SUMMARY_RE),
)


@dataclass
class Group:
    label: str
    lines: list[str] = field(default_factory=list)
    completed: bool = False


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


def _group_family(label: str) -> str:
    if label.startswith("ltp-"):
        return "ltp"
    if label.startswith("busybox-"):
        return "busybox"
    if label.startswith("libctest-"):
        return "libctest"
    return "generic"


def _scan_unstructured_line(
    source: str,
    line: str,
    family: str = "generic",
    group: str | None = None,
) -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    """Classify human output without reinterpreting trusted case grammar.

    Specialized validators bind their machine records to exact lifecycles.
    Their diagnostics still need fail-closed checks, but wording such as
    ``failed 0``, ``Timeout per run``, or a BusyBox applet named ``timeout`` is
    not itself a failure. Keep those family grammars narrow while retaining
    explicit TFAIL/TBROK/TCONF/ENOSYS, non-zero summaries, timeouts, and crashes.
    """

    errors: list[dict[str, str]] = []
    failures: list[dict[str, str]] = []
    context = f"{source}: {line.strip()}"

    if _protocol_record_kinds(line):
        return errors, failures
    if OFFICIAL_TIMEOUT_BUDGET_RE.fullmatch(line):
        return errors, failures
    if family == "generic" and CONTROLLED_PROCESS_CLEANUP_RE.fullmatch(line):
        return errors, failures

    if UNKNOWN_STATE_RE.search(line) and not UNCONSUMED_FAILURE_RE.search(line):
        errors.append(issue("unknown-status", context, group))
    if ZERO_EXECUTION_RE.search(line):
        errors.append(issue("zero-execution", context, group))

    if family == "ltp":
        if summary := LTP_INTERNAL_SUMMARY_RE.fullmatch(line):
            summary_kind = summary.group(1).lower()
            summary_count = int(summary.group(2))
            if summary_kind != "passed" and summary_count != 0:
                failures.append(
                    issue(
                        "ltp-internal-summary-failure",
                        f"{source}: {summary_kind}={summary_count}",
                        group,
                    )
                )
            return errors, failures

        diagnostic_statuses = {
            marker.upper() for marker in LTP_DIAGNOSTIC_STATUS_RE.findall(line)
        }
        failing_statuses = sorted(
            diagnostic_statuses & {"TFAIL", "TBROK", "TCONF"}
        )
        if re.search(r"\bENOSYS\b", line, re.I):
            failing_statuses.append("ENOSYS")
        for marker in failing_statuses:
            failures.append(
                issue("forbidden-status", f"{source}: {marker}", group)
            )
        if diagnostic_statuses:
            # LTP result tags own expected errno/signal wording, but they do
            # not make an actual panic/trap or timeout event benign.  Keep the
            # latter checks narrower than the generic scanners so legitimate
            # diagnostics such as "Timeout per run", "failed as expected",
            # and ETIMEDOUT under TPASS remain valid.
            if LTP_FATAL_DIAGNOSTIC_RE.search(line):
                failures.append(issue("panic-or-trap", context, group))
            if LTP_TIMEOUT_EVENT_RE.search(line):
                failures.append(issue("timeout", context, group))
            return errors, failures

        # Untagged LTP help text legitimately uses bare words such as "error"
        # and "hang". Exact numeric results and tagged LTP statuses own those
        # semantics; retain only high-confidence untagged status markers here.
        retained_markers = {
            "TCONF",
            "TBROK",
            "TFAIL",
            "ENOSYS",
            "XFAIL",
            "SKIP",
            "SKIPPED",
            "CRASH",
            "PANIC",
        }
        for marker in FORBIDDEN_STATUS_RE.findall(line):
            if marker.upper() in retained_markers:
                failures.append(
                    issue("forbidden-status", f"{source}: {marker}", group)
                )
    elif family in {"busybox", "libctest"}:
        # Exact per-case records own timeout/success semantics for these
        # families. Do not confuse command output with a terminal status.
        retained_markers = {
            "TCONF",
            "TBROK",
            "TFAIL",
            "ENOSYS",
            "XFAIL",
            "SKIP",
            "SKIPPED",
            "CRASH",
            "PANIC",
        }
        for marker in FORBIDDEN_STATUS_RE.findall(line):
            if marker.upper() in retained_markers:
                failures.append(
                    issue("forbidden-status", f"{source}: {marker}", group)
                )
    else:
        for marker in FORBIDDEN_STATUS_RE.findall(line):
            failures.append(
                issue("forbidden-status", f"{source}: {marker}", group)
            )

    if TIMEOUT_RE.search(line):
        failures.append(issue("timeout", context, group))
    if SKIP_RE.search(line):
        failures.append(issue("skipped-group", context, group))
    if PANIC_RE.search(line):
        failures.append(issue("panic-or-trap", context, group))
    if UNCONSUMED_FAILURE_RE.search(line):
        failures.append(issue("explicit-failure", context, group))
    if GENERIC_SUBTEST_FAILURE_RE.fullmatch(line):
        failures.append(issue("generic-subtest-failure", context, group))
    if EXPLICIT_NONZERO_RE.search(line):
        failures.append(issue("explicit-nonzero", context, group))
    return errors, failures


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
        "started_cases": len(starts),
        "executed_cases": len(all_cases),
        "result_cases": len(results),
        "completed_cases": len(ends),
        "passed_cases": sum(code == 0 for _name, code in results),
        "failed_cases": sum(code != 0 for _name, code in results),
        "summary": (
            {
                "passed": summaries[0][0],
                "failed": summaries[0][1],
                "timed_out": summaries[0][2],
            }
            if len(summaries) == 1
            else None
        ),
        "cases": [
            {
                "case": name,
                "code": code,
                "events": event_sequences.get(name, []),
            }
            for name, code in results
        ],
    }


def _busybox_plan_error(expected_cases: Any) -> str | None:
    if not isinstance(expected_cases, list) or not expected_cases:
        return "expected BusyBox identities must be a non-empty list"
    explicit_ids: set[str] = set()
    for index, case in enumerate(expected_cases):
        if not isinstance(case, BusyBoxCase):
            return f"expected BusyBox identity at index {index} is not a BusyBoxCase"
        if case.ordinal != index + 1:
            return "expected BusyBox ordinals must be contiguous and one-based"
        if (
            not case.command
            or case.command != case.command.strip()
            or "\n" in case.command
            or "\r" in case.command
            or first_unsupported_output_character(case.command) is not None
        ):
            return f"expected BusyBox command at ordinal {case.ordinal} is malformed"
        if case.explicit_id is not None:
            if BUSYBOX_EXPLICIT_ID_RE.fullmatch(case.explicit_id) is None:
                return f"expected BusyBox explicit ID at ordinal {case.ordinal} is malformed"
            if case.explicit_id in explicit_ids:
                return f"expected BusyBox explicit ID is duplicated: {case.explicit_id}"
            explicit_ids.add(case.explicit_id)
    return None


def _validate_busybox(
    group: Group,
    expected_cases: list[BusyBoxCase] | None = None,
) -> tuple[list[dict[str, str]], dict[str, Any]]:
    issues: list[dict[str, str]] = []
    legacy_results = [
        (match.group(1), match.group(2).lower())
        for line in group.lines
        if (match := BUSYBOX_LEGACY_RESULT_RE.fullmatch(line))
    ]
    has_structured_records = any(
        BUSYBOX_CASE_START_RE.fullmatch(line)
        or BUSYBOX_CASE_RESULT_RE.fullmatch(line)
        or BUSYBOX_CASE_END_RE.fullmatch(line)
        for line in group.lines
    )

    accepted_results: list[tuple[int, str, str]] = []
    completed_results: list[tuple[int, str, str]] = []
    starts = 0
    result_records = 0
    if legacy_results:
        issues.append(
            issue(
                "busybox-legacy-identity",
                "legacy text-only BusyBox results do not provide replay-safe case identity",
                group.label,
            )
        )
        if has_structured_records:
            issues.append(
                issue(
                    "busybox-mixed-protocol",
                    "legacy and structured BusyBox case records must not be mixed",
                    group.label,
                )
            )
        accepted_results = [
            (ordinal, command, status)
            for ordinal, (command, status) in enumerate(legacy_results, start=1)
        ]
        completed_results = list(accepted_results)
        starts = len(legacy_results)
        result_records = len(legacy_results)
    elif has_structured_records:
        current_ordinal: int | None = None
        current_result: tuple[int, str, str] | None = None
        seen_ordinals: set[int] = set()
        for line in group.lines:
            if match := BUSYBOX_CASE_START_RE.fullmatch(line):
                ordinal = int(match.group(1))
                starts += 1
                if current_ordinal is not None:
                    issues.append(
                        issue(
                            "busybox-nested-frame",
                            f"ordinal {ordinal} started before ordinal {current_ordinal} ended",
                            group.label,
                        )
                    )
                    continue
                if ordinal in seen_ordinals:
                    issues.append(
                        issue(
                            "busybox-duplicate-identity",
                            f"BusyBox ordinal {ordinal} was replayed",
                            group.label,
                        )
                    )
                seen_ordinals.add(ordinal)
                current_ordinal = ordinal
                current_result = None
                continue
            if match := BUSYBOX_CASE_RESULT_RE.fullmatch(line):
                ordinal = int(match.group(1))
                status = match.group(2).lower()
                command = match.group(3)
                result_records += 1
                if current_ordinal is None:
                    issues.append(
                        issue(
                            "busybox-orphan-result",
                            f"result for ordinal {ordinal} appeared outside a case frame",
                            group.label,
                        )
                    )
                elif ordinal != current_ordinal:
                    issues.append(
                        issue(
                            "busybox-identity-mismatch",
                            f"active ordinal {current_ordinal} received result for ordinal {ordinal}",
                            group.label,
                        )
                    )
                elif current_result is not None:
                    issues.append(
                        issue(
                            "busybox-duplicate-result",
                            f"ordinal {ordinal} emitted more than one terminal result",
                            group.label,
                        )
                    )
                else:
                    current_result = (ordinal, command, status)
                    accepted_results.append(current_result)
                continue
            if match := BUSYBOX_CASE_END_RE.fullmatch(line):
                ordinal = int(match.group(1))
                if current_ordinal is None:
                    issues.append(
                        issue(
                            "busybox-orphan-end",
                            f"end for ordinal {ordinal} appeared without an active frame",
                            group.label,
                        )
                    )
                elif ordinal != current_ordinal:
                    issues.append(
                        issue(
                            "busybox-identity-mismatch",
                            f"active ordinal {current_ordinal} ended as ordinal {ordinal}",
                            group.label,
                        )
                    )
                    current_ordinal = None
                    current_result = None
                else:
                    if current_result is None:
                        issues.append(
                            issue(
                                "busybox-missing-result",
                                f"ordinal {ordinal} ended without a terminal result",
                                group.label,
                            )
                        )
                    else:
                        completed_results.append(current_result)
                    current_ordinal = None
                    current_result = None
        if current_ordinal is not None:
            issues.append(
                issue(
                    "busybox-missing-end",
                    f"ordinal {current_ordinal} did not emit its end marker",
                    group.label,
                )
            )

    observed_cases = [(ordinal, command) for ordinal, command, _status in accepted_results]
    if expected_cases is not None:
        expected_identities = [(case.ordinal, case.command) for case in expected_cases]
        if observed_cases != expected_identities:
            mismatch_index = next(
                (
                    index
                    for index, (observed, expected) in enumerate(
                        zip(observed_cases, expected_identities)
                    )
                    if observed != expected
                ),
                min(len(observed_cases), len(expected_identities)),
            )
            observed = (
                observed_cases[mismatch_index]
                if mismatch_index < len(observed_cases)
                else "<missing>"
            )
            expected = (
                expected_identities[mismatch_index]
                if mismatch_index < len(expected_identities)
                else "<none>"
            )
            issues.append(
                issue(
                    "busybox-case-plan-mismatch",
                    f"case sequence diverges at index {mismatch_index}: expected {expected!r}, "
                    f"observed {observed!r}; expected {len(expected_identities)} identities, "
                    f"observed {len(observed_cases)}",
                    group.label,
                )
            )

    passed = sum(status == "success" for _ordinal, _command, status in accepted_results)
    failed = sum(status == "fail" for _command, status in legacy_results) + sum(
        match.group(2).lower() == "fail"
        for line in group.lines
        if (match := BUSYBOX_CASE_RESULT_RE.fullmatch(line))
    )
    if not accepted_results:
        issues.append(issue("busybox-empty", "busybox group contains no case results", group.label))
    if failed:
        issues.append(issue("busybox-failure", f"{failed} busybox cases failed", group.label))
    return issues, {
        "started_cases": starts,
        "executed_cases": starts,
        "result_cases": result_records,
        "completed_cases": len(completed_results),
        "passed_cases": passed,
        "failed_cases": failed,
        "cases": [
            {
                "ordinal": ordinal,
                "command": command,
                "status": status,
                "explicit_id": (
                    expected_cases[ordinal - 1].explicit_id
                    if expected_cases is not None
                    and ordinal <= len(expected_cases)
                    and expected_cases[ordinal - 1].ordinal == ordinal
                    else None
                ),
            }
            for ordinal, command, status in completed_results
        ],
    }


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
    completed_frames = 0
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
            else:
                completed_frames += 1
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
        return issues, {
            "started_cases": starts,
            "executed_cases": starts,
            "completed_cases": completed_frames,
            "passed_cases": 0,
            "failed_cases": 0,
        }
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
    return issues, {
        "started_cases": starts,
        "executed_cases": starts,
        "completed_cases": completed_frames,
        "passed_cases": passed,
        "failed_cases": failed,
    }


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
            or GENERIC_SUBTEST_FAILURE_RE.fullmatch(line)
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
    expected_busybox_cases: list[BusyBoxCase] | None = None,
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
    case_count_plan = (
        expected_group_case_counts
        if expected_group_case_counts is not None
        else {}
    )

    for source, cleaned_text in (("stdout", text), ("stderr", stderr_text)):
        if invalid_character := first_unsupported_output_character(cleaned_text):
            structural_errors.append(
                issue(
                    "invalid-output-control",
                    f"{source} contains unsupported output character "
                    f"U+{ord(invalid_character):04X}",
                )
            )

    if expected_group_labels is not None and (
        not isinstance(expected_group_labels, list)
        or not expected_group_labels
        or any(not isinstance(label, str) or not label for label in expected_group_labels)
        or len(expected_group_labels) != len(set(expected_group_labels))
    ):
        structural_errors.append(
            issue(
                "official-group-plan",
                "expected official group labels must be non-empty and unique strings",
            )
        )
        expected_group_labels = None
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
    busybox_plan_error = (
        _busybox_plan_error(expected_busybox_cases)
        if expected_busybox_cases is not None
        else None
    )
    if busybox_plan_error is not None:
        structural_errors.append(
            issue("official-busybox-case-plan", busybox_plan_error)
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
                current.completed = True
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
        "busybox": {
            "official-pass", "official-fail", "busybox-start", "busybox-result",
            "busybox-end", "busybox-legacy-result",
        },
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
            family = _group_family(source_group.label)
            if not kinds & allowed_protocols[family]:
                structural_errors.append(
                    issue("unexpected-protocol-record", line.strip(), source_group.label)
                )

    stderr_lines = stderr_text.splitlines()
    for line in stderr_lines:
        if PROTOCOL_SIGNATURE_RE.search(line):
            structural_errors.append(issue("protocol-record-on-stderr", line.strip()))

    scan_inputs = [
        *(("stdout", line, "generic", None) for line in outside_lines),
        *(
            ("stdout", line, _group_family(group.label), group.label)
            for group in groups
            for line in group.lines
        ),
        *(("stderr", line, "generic", None) for line in stderr_lines),
    ]
    for source, line, family, group_label in scan_inputs:
        line_errors, line_failures = _scan_unstructured_line(
            source,
            line,
            family,
            group_label,
        )
        structural_errors.extend(line_errors)
        failures.extend(line_failures)

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
                completed_cases = counts.get("completed_cases", counts["executed_cases"])
                if completed_cases != expected_cases:
                    structural_errors.append(
                        issue(
                            "official-planned-completed-mismatch",
                            f"expected {expected_cases} completed cases but observed {completed_cases}",
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

    planned_group_count = (
        len(expected_group_labels) if expected_group_labels is not None else None
    )
    planned_case_count = (
        sum(
            count
            for count in case_count_plan.values()
            if type(count) is int and count > 0
        )
        if isinstance(case_count_plan, dict)
        else None
    )
    counted_group_rows = [
        row for row in group_rows if row["label"] in case_count_plan
    ]
    status = "ERROR" if structural_errors else "FAIL" if failures else "PASS"
    return {
        "status": status,
        "group_count": len(groups),
        "planned_group_count": planned_group_count,
        "executed_group_count": len(groups),
        "completed_group_count": sum(group.completed for group in groups),
        "planned_case_count": planned_case_count,
        "executed_case_count": sum(
            row.get("executed_cases", 0) for row in counted_group_rows
        ),
        "completed_case_count": sum(
            row.get("completed_cases", row.get("executed_cases", 0))
            for row in counted_group_rows
        ),
        "groups": group_rows,
        "error_count": len(structural_errors),
        "failure_count": len(failures),
        "errors": structural_errors,
        "failures": failures,
    }


def validate_ltp_output(stdout: str, stderr: str = "") -> dict[str, Any]:
    """Strictly validate only LTP groups while preserving global input framing.

    This scoped API is intended for promotion evidence.  It does not load or
    require the canonical 24-group official plan, but malformed global group
    framing, unsupported controls, and LTP protocol records outside an LTP
    group remain integrity errors.
    """

    text = normalize_output_text(stdout)
    stderr_text = normalize_output_text(stderr)
    scope_errors: list[dict[str, str]] = []
    scope_failures: list[dict[str, str]] = []
    for source, cleaned_text in (("stdout", text), ("stderr", stderr_text)):
        if invalid_character := first_unsupported_output_character(cleaned_text):
            scope_errors.append(
                issue(
                    "invalid-output-control",
                    f"{source} contains unsupported output character "
                    f"U+{ord(invalid_character):04X}",
                )
            )

    groups: list[Group] = []
    current: Group | None = None
    outside_lines: list[str] = []
    group_labels: set[str] = set()
    for line in text.splitlines():
        if match := GROUP_START_RE.fullmatch(line):
            label = match.group(1)
            if current is not None:
                scope_errors.append(
                    issue(
                        "nested-group",
                        f"group {label} started before {current.label} ended",
                        current.label,
                    )
                )
                continue
            current = Group(label)
            groups.append(current)
            if label in group_labels:
                scope_errors.append(
                    issue("duplicate-group", f"group {label} ran more than once", label)
                )
            group_labels.add(label)
            continue
        if match := GROUP_END_RE.fullmatch(line):
            label = match.group(1)
            if current is None:
                scope_errors.append(
                    issue("unmatched-group-end", f"group end without start: {label}", label)
                )
            elif current.label != label:
                scope_errors.append(
                    issue(
                        "mismatched-group-end",
                        f"started {current.label} but ended {label}",
                        current.label,
                    )
                )
                current = None
            else:
                current.completed = True
                current = None
            continue
        if current is None:
            outside_lines.append(line)
        else:
            current.lines.append(line)

    if current is not None:
        scope_errors.append(
            issue("missing-group-end", "group did not emit its end marker", current.label)
        )

    ltp_groups = [group for group in groups if group.label.startswith("ltp-")]
    if not ltp_groups:
        scope_errors.append(issue("zero-ltp-groups", "no LTP test groups executed"))

    ltp_protocol_kinds = {
        "ltp-list",
        "ltp-start",
        "ltp-run",
        "ltp-result",
        "ltp-end",
        "ltp-summary",
    }
    for line in outside_lines:
        kinds = _protocol_record_kinds(line)
        if PROTOCOL_SIGNATURE_RE.search(line) and not kinds:
            scope_errors.append(issue("malformed-protocol-record", line.strip()))
        elif kinds & ltp_protocol_kinds:
            scope_errors.append(issue("ltp-record-outside-group", line.strip()))
        elif kinds:
            scope_errors.append(
                issue("protocol-record-outside-ltp-group", line.strip())
            )
        if kinds:
            continue
        if UNKNOWN_STATE_RE.search(line) and not UNCONSUMED_FAILURE_RE.search(line):
            scope_errors.append(issue("unknown-status-outside-ltp-group", line.strip()))
        if ZERO_EXECUTION_RE.search(line):
            scope_errors.append(issue("zero-execution-outside-ltp-group", line.strip()))
        markers = FORBIDDEN_STATUS_RE.findall(line)
        for marker in markers:
            scope_failures.append(
                issue("forbidden-status-outside-ltp-group", marker.upper())
            )
        if TIMEOUT_RE.search(line) and not any(marker.upper() == "TIMEOUT" for marker in markers):
            scope_failures.append(issue("timeout-outside-ltp-group", line.strip()))
        if SKIP_RE.search(line) and not any(marker.upper().startswith("SKIP") for marker in markers):
            scope_failures.append(issue("skip-outside-ltp-group", line.strip()))
        if PANIC_RE.search(line) and not any(marker.upper() == "PANIC" for marker in markers):
            scope_failures.append(issue("panic-or-trap-outside-ltp-group", line.strip()))
        if UNCONSUMED_FAILURE_RE.search(line):
            scope_failures.append(issue("explicit-failure-outside-ltp-group", line.strip()))
        if EXPLICIT_NONZERO_RE.search(line):
            scope_failures.append(issue("explicit-nonzero-outside-ltp-group", line.strip()))
        if INFRA_TEXT_RE.search(line):
            scope_errors.append(issue("runtime-infrastructure-outside-ltp-group", line.strip()))
    for group in groups:
        if group.label.startswith("ltp-"):
            continue
        family = (
            "busybox" if group.label.startswith("busybox-")
            else "libctest" if group.label.startswith("libctest-")
            else "generic"
        )
        allowed_protocols = {
            "busybox": {
                "official-pass", "official-fail", "busybox-start", "busybox-result",
                "busybox-end", "busybox-legacy-result",
            },
            "libctest": {
                "official-pass",
                "official-fail",
                "libctest-start",
                "libctest-end",
                "libctest-fail",
                "libctest-summary",
            },
            "generic": {"official-pass", "official-fail"},
        }[family]
        for line in group.lines:
            kinds = _protocol_record_kinds(line)
            if PROTOCOL_SIGNATURE_RE.search(line) and not kinds:
                scope_errors.append(
                    issue("malformed-protocol-record", line.strip(), group.label)
                )
            elif kinds and not kinds & allowed_protocols:
                scope_errors.append(
                    issue(
                        "unexpected-protocol-record-in-non-ltp-group",
                        line.strip(),
                        group.label,
                    )
                )
            for pattern in (OFFICIAL_PASS_RE, OFFICIAL_FAIL_RE):
                match = pattern.fullmatch(line)
                if match is not None and match.group(1) != group.label:
                    scope_errors.append(
                        issue(
                            "protocol-group-label-mismatch",
                            f"record names {match.group(1)} inside {group.label}",
                            group.label,
                        )
                    )

    scoped_lines: list[str] = []
    for group in ltp_groups:
        scoped_lines.append(f"#### OS COMP TEST GROUP START {group.label} ####")
        scoped_lines.extend(group.lines)
        scoped_lines.append(f"#### OS COMP TEST GROUP END {group.label} ####")
    validation = validate_official_output("\n".join(scoped_lines), stderr_text)
    for finding in scope_errors:
        if finding not in validation["errors"]:
            validation["errors"].append(finding)
    for finding in scope_failures:
        if finding not in validation["failures"]:
            validation["failures"].append(finding)
    validation["error_count"] = len(validation["errors"])
    validation["failure_count"] = len(validation["failures"])
    if validation["errors"]:
        validation["status"] = "ERROR"
    elif validation["failures"]:
        validation["status"] = "FAIL"
    validation["validation_scope"] = "ltp"
    return validation


def apply_process_exit_code(validation: dict[str, Any], process_exit_code: int) -> None:
    """Bind the captured evaluator return code into an existing validation."""

    validation["process_exit_code"] = process_exit_code
    if process_exit_code == 0:
        return
    finding = issue(
        "evaluator-process-nonzero",
        f"captured evaluator process exited with status {process_exit_code}",
    )
    if finding not in validation["failures"]:
        validation["failures"].append(finding)
    validation["failure_count"] = len(validation["failures"])
    if validation["status"] == "PASS":
        validation["status"] = "FAIL"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stdout", required=True, type=Path, help="captured evaluator stdout")
    parser.add_argument(
        "--stderr",
        required=True,
        type=Path,
        help="captured evaluator stderr companion",
    )
    parser.add_argument(
        "--process-exit-code",
        required=True,
        type=int,
        help="actual evaluator process return code associated with the capture pair",
    )
    parser.add_argument("--json", action="store_true", help="emit machine-readable validation")
    args = parser.parse_args(argv)

    try:
        pair = validate_capture_input_pairs([args.stdout], [args.stderr])[0]
        stdout_raw = args.stdout.read_bytes()
        stderr_raw = args.stderr.read_bytes()
        stdout = stdout_raw.decode("utf-8", errors="strict")
        stderr = stderr_raw.decode("utf-8", errors="strict")
    except (OSError, UnicodeDecodeError, ValueError) as error:
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
    result["input_evidence"] = {
        "pair_id": pair["pair_id"],
        "source_key": pair["source_key"],
        "arch": pair["arch"],
        "stdout_path": str(args.stdout),
        "stderr_path": str(args.stderr),
        "stdout_resolved_path": str(pair["stdout_resolved_path"]),
        "stderr_resolved_path": str(pair["stderr_resolved_path"]),
        "stdout_size_bytes": len(stdout_raw),
        "stderr_size_bytes": len(stderr_raw),
        "stdout_sha256": hashlib.sha256(stdout_raw).hexdigest(),
        "stderr_sha256": hashlib.sha256(stderr_raw).hexdigest(),
        "process_exit_code": args.process_exit_code,
    }
    apply_process_exit_code(result, args.process_exit_code)
    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(
            f"official result validation: {result['status']} "
            f"({result['group_count']} groups, {result['failure_count']} failures, "
            f"{result['error_count']} integrity errors)"
        )
        evidence = result["input_evidence"]
        print(
            "input evidence: "
            f"pair={evidence['pair_id']} process_exit_code={evidence['process_exit_code']} "
            f"stdout_sha256={evidence['stdout_sha256']} "
            f"stderr_sha256={evidence['stderr_sha256']}"
        )
        for item in result["errors"] + result["failures"]:
            location = f" [{item['group']}]" if "group" in item else ""
            print(f"- {item['kind']}{location}: {item['message']}")
    return {"PASS": 0, "FAIL": 1, "ERROR": 2}[result["status"]]


if __name__ == "__main__":
    raise SystemExit(main())
