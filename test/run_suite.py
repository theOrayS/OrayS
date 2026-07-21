#!/usr/bin/env python3
"""Run explicitly registered OrayS test profiles with strict result accounting."""

from __future__ import annotations

import sys as _bootstrap_sys

if __name__ == "__main__" and (
    not _bootstrap_sys.flags.isolated
    or not _bootstrap_sys.flags.no_site
    or not _bootstrap_sys.flags.dont_write_bytecode
    or _bootstrap_sys.pycache_prefix != "/dev/null"
):
    import os as _bootstrap_os

    _bootstrap_script = _bootstrap_os.path.abspath(_bootstrap_sys.argv[0])
    _bootstrap_os.execv(
        _bootstrap_sys.executable,
        [
            _bootstrap_sys.executable,
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
            _bootstrap_script,
            *_bootstrap_sys.argv[1:],
        ],
    )

import argparse
import ast
import ctypes
import datetime as dt
import importlib.util
import json
import math
import os
import re
import shutil
import signal
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True
sys.pycache_prefix = "/dev/null"
_TEST_ROOT = Path(__file__).resolve().parent


def _closed_git_environment(repo: Path) -> dict[str, str]:
    return {
        "PATH": os.defpath,
        "HOME": str(repo),
        "PWD": str(repo),
        "LC_ALL": "C",
        "LANG": "C",
        "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_CONFIG_GLOBAL": "/dev/null",
    }


def _canonical_manifest_requested(argv: list[str], canonical_manifest: Path) -> bool:
    manifest_value: str | None = None
    index = 0
    while index < len(argv):
        value = argv[index]
        if value == "--manifest":
            if index + 1 >= len(argv):
                return True
            manifest_value = argv[index + 1]
            index += 2
            continue
        if value.startswith("--manifest="):
            manifest_value = value.split("=", 1)[1]
        index += 1
    if manifest_value is None:
        return True
    return Path(manifest_value).expanduser().resolve() == canonical_manifest


def _bootstrap_clean_canonical_worktree() -> None:
    repo = _TEST_ROOT.parent
    canonical_manifest = (_TEST_ROOT / "suite_manifest.json").resolve()
    if "--list" in sys.argv[1:] or not _canonical_manifest_requested(
        sys.argv[1:], canonical_manifest
    ):
        return
    environment = _closed_git_environment(repo)
    git_command = shutil.which("git", path=environment["PATH"])
    if git_command is None:
        print(
            "infrastructure error: cannot inspect canonical runner provenance: git is unavailable",
            file=sys.stderr,
        )
        raise SystemExit(2)
    try:
        result = subprocess.run(
            [
                str(Path(git_command).resolve()),
                "status",
                "--porcelain=v1",
                "--untracked-files=all",
            ],
            cwd=repo,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=10,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        print(
            f"infrastructure error: cannot inspect canonical runner provenance: {error}",
            file=sys.stderr,
        )
        raise SystemExit(2) from error
    if result.returncode != 0:
        print(
            "infrastructure error: closed Git status failed before canonical code loading",
            file=sys.stderr,
        )
        raise SystemExit(2)
    status_lines = result.stdout.splitlines()
    if status_lines:
        print(
            "infrastructure error: canonical profile loading requires a clean runner worktree",
            file=sys.stderr,
        )
        for status_line in status_lines[:20]:
            print(f"  {status_line}", file=sys.stderr)
        if len(status_lines) > 20:
            print(f"  ... {len(status_lines) - 20} additional status entries", file=sys.stderr)
        raise SystemExit(2)


if __name__ == "__main__":
    _bootstrap_clean_canonical_worktree()

if str(_TEST_ROOT) not in sys.path:
    sys.path.insert(0, str(_TEST_ROOT))

_OFFICIAL_PARSER_PATH = _TEST_ROOT / "evaluation/parse_official_results.py"
_OFFICIAL_PARSER_SPEC = importlib.util.spec_from_file_location(
    "_orays_official_result_parser",
    _OFFICIAL_PARSER_PATH,
)
if _OFFICIAL_PARSER_SPEC is None or _OFFICIAL_PARSER_SPEC.loader is None:
    raise RuntimeError(f"cannot load canonical official parser: {_OFFICIAL_PARSER_PATH}")
_OFFICIAL_PARSER = importlib.util.module_from_spec(_OFFICIAL_PARSER_SPEC)
sys.modules[_OFFICIAL_PARSER_SPEC.name] = _OFFICIAL_PARSER
_OFFICIAL_PARSER_SPEC.loader.exec_module(_OFFICIAL_PARSER)

CANONICAL_LTP_CASE_LIST = _OFFICIAL_PARSER.CANONICAL_LTP_CASE_LIST
CANONICAL_OFFICIAL_CASE_COUNTS = _OFFICIAL_PARSER.CANONICAL_OFFICIAL_CASE_COUNTS
CANONICAL_OFFICIAL_GROUPS = _OFFICIAL_PARSER.CANONICAL_OFFICIAL_GROUPS
TRUSTED_BUILD_STDERR_RE = _OFFICIAL_PARSER.TRUSTED_BUILD_STDERR_RE
first_unsupported_output_character = _OFFICIAL_PARSER.first_unsupported_output_character
normalize_output_text = _OFFICIAL_PARSER.normalize_output_text
trusted_official_case_plan = _OFFICIAL_PARSER.trusted_official_case_plan
trusted_ltp_stable_cases = _OFFICIAL_PARSER.trusted_ltp_stable_cases
validate_official_output = _OFFICIAL_PARSER.validate_official_output

_FINAL_2026_PARSER_PATH = _TEST_ROOT / "evaluation/parse_final_2026_results.py"
_FINAL_2026_PARSER_SPEC = importlib.util.spec_from_file_location(
    "_orays_final_2026_result_parser",
    _FINAL_2026_PARSER_PATH,
)
if _FINAL_2026_PARSER_SPEC is None or _FINAL_2026_PARSER_SPEC.loader is None:
    raise RuntimeError(
        f"cannot load canonical final-2026 parser: {_FINAL_2026_PARSER_PATH}"
    )
_FINAL_2026_PARSER = importlib.util.module_from_spec(_FINAL_2026_PARSER_SPEC)
sys.modules[_FINAL_2026_PARSER_SPEC.name] = _FINAL_2026_PARSER
_FINAL_2026_PARSER_SPEC.loader.exec_module(_FINAL_2026_PARSER)

FINAL_2026_GROUPS = _FINAL_2026_PARSER.SUPPORTED_GROUPS
FINAL_2026_ARCHITECTURES = _FINAL_2026_PARSER.SUPPORTED_ARCHITECTURES
FINAL_2026_GROUP_LABELS = _FINAL_2026_PARSER.EXPECTED_GROUP_LABELS
validate_final_2026_output = _FINAL_2026_PARSER.validate_final_2026_output

SCHEMA_VERSION = 1
KNOWN_STATUSES = {"PASS", "FAIL", "TIMEOUT", "CRASH", "INFRA_ERROR", "NOT_RUN"}
SUCCESS_STATUS = "PASS"
PROFILE_NAME_RE = re.compile(r"^[a-z][a-z0-9_-]*$")
CASE_ID_RE = re.compile(r"^[a-z][a-z0-9]*(?:[._-][a-z0-9]+)*$")
HISTORICAL_ID_RE = re.compile(r"(?i)(?:^|[._-])g0\d{2}(?:$|[._-])")
ENV_NAME_RE = re.compile(r"^[A-Z_][A-Z0-9_]*$")
RESULT_TYPES = {
    "exit_code",
    "check",
    "unittest",
    "cargo_test",
    "case_result",
    "official",
    "final_2026",
}
STRUCTURED_RESULT_TYPES = frozenset({"official", "final_2026"})
ARCH_POLICIES = {"none", "one", "one_or_all"}
CANONICAL_PROFILE_NAMES = frozenset(
    {
        "checks",
        "unit",
        "quick",
        "evidence-host",
        "evidence-runtime",
        "evidence-aggregate",
        "evidence-required",
        "baseline",
        "official",
        "final-2026",
        "full",
    }
)
NON_PASS_OUTPUT_RE = re.compile(
    r"\b(?:SKIP(?:PED|PING)?|XFAIL|TCONF|TBROK|TFAIL|ENOSYS|TIMEOUT|TIMED[_ -]?OUT|"
    r"TIME[_ -]?LIMIT[_ -]?EXCEEDED|DEADLINE EXCEEDED|WATCHDOG EXPIRED|HANG|CRASH|PANIC)\b",
    re.I,
)
UNITTEST_COUNT_RE = re.compile(
    r"^Ran\s+(\d+)\s+tests?\s+in\s+(?:0|[1-9]\d*)(?:\.\d+)?s\s*$"
)
CASE_RESULT_RE = re.compile(r"^CASE_RESULT:\s*([A-Za-z_]+)\s*$", re.M)
CASE_RESULT_SIGNATURE_RE = re.compile(r"\bCASE_RESULT\b", re.I)
CHECK_DIRECT_PASS_RE = re.compile(r"^PASS$", re.I)
CHECK_NAMED_PASS_RE = re.compile(
    r"^(?P<label>[A-Za-z0-9][A-Za-z0-9 /_-]*\bcheck):\s*PASS(?:\s+\(0 findings\))?$",
    re.I,
)
CHECK_LABEL_NON_PASS_RE = re.compile(
    r"\b(?:NOT|ERRORS?|SKIP(?:PED|PING)?|FAIL(?:ED|URE|URES)?)\b",
    re.I,
)
POSITIVE_FINDINGS_RE = re.compile(r"\b(?:[1-9]\d*)\s+findings?\b", re.I)
EXPLICIT_FAILURE_OUTPUT_RE = re.compile(
    r"\b(?:FAIL(?:ED|URE|URES)?|ERRORS?|INCOMPLETE|PARTIAL(?:LY EXECUTED)?|"
    r"FATAL|ABORT(?:ED)?)\b|"
    r"\bNOT[ _-]?PASS\b|^Traceback \(most recent call last\):|"
    r"^[A-Za-z_][A-Za-z0-9_.]*(?:Error|Exception):|^\s*not\s+ok(?:\s+\d+)?(?:\b|\s*-)",
    re.I | re.M,
)
CRASH_OUTPUT_RE = re.compile(
    r"\b(?:segmentation fault|segfault|illegal instruction|bus error|core dumped|"
    r"process crash(?:ed)?|crashed|unknown trap|unhandled(?: user)? trap|fatal trap|"
    r"panic(?:ked)?|aborted|killed|terminated|signal\s+\d+|"
    r"SIG(?:ABRT|BUS|FPE|HUP|ILL|INT|KILL|QUIT|SEGV|SYS|TERM|TRAP))\b",
    re.I,
)
UNKNOWN_STATE_TOKEN_PATTERN = (
    r"NOT[-_ ]RUN|NOT[-_ ]EXECUTED|UNEXECUTED|DID[-_ ]NOT[-_ ]RUN|"
    r"NOT[-_ ]ATTEMPTED|INFRA_ERROR|UNKNOWN(?:[-_ ]STATUS)?|"
    r"(?:STATUS|RESULT)[-_ ]UNKNOWN|UNRESOLVED|UNSUPPORTED|INCONCLUSIVE|"
    r"PENDING|CANCELLED|CANCELED|"
    r"DISABLED|OMITTED"
)
ZERO_EXECUTION_OUTPUT_RE = re.compile(
    r"\b(?:NO\s+(?:TESTS?|CASES?)\s+(?:RAN|RUN|EXECUTED)|"
    r"(?:0|ZERO)\s+(?:TESTS?|CASES?)\s+(?:RAN|RUN|EXECUTED)|"
    r"RAN\s+(?:0|ZERO)\s+(?:TESTS?|CASES?)|NO\s+RUNNABLE\s+(?:TESTS?|CASES?)|"
    r"(?:TEST\s+SUITE|SUITE)\s+IS\s+EMPTY|EMPTY\s+(?:TEST\s+)?SUITE)\b",
    re.I,
)
EXIT_CODE_PROTOCOL_NON_PASS_OUTPUT_RE = re.compile(
    r"\b(?:TCONF|TBROK|TFAIL|ENOSYS|XFAIL|SKIP(?:PED|PING)?|"
    r"TIMEOUT|TIMED[_ -]?OUT|TIME[_ -]?LIMIT[_ -]?EXCEEDED|"
    r"INCOMPLETE|PARTIAL(?:LY EXECUTED)?|CRASH(?:ED)?|"
    r"FATAL|HANG(?:ED|ING)?|"
    r"SIG(?:ABRT|BUS|FPE|HUP|ILL|INT|KILL|QUIT|SEGV|SYS|TERM|TRAP)|"
    r"SIGNAL\s+\d+)\b|"
    r"\b(?:STATUS|RESULT|STATE)[_: -]+UNKNOWN\b|"
    r"\bUNKNOWN[_ -]+(?:STATUS|RESULT|STATE)\b|"
    r"\b(?:SUITE|CASE|VERDICT|OUTCOME)\s*[:=]\s*UNKNOWN\b|"
    r"\b(?:deadline exceeded|watchdog expired|segmentation fault|segfault|"
    r"illegal instruction|bus error|core dumped|unknown trap|"
    r"unhandled(?: user)? trap|fatal trap|failed but ignored|"
    r"failure ignored|ignoring failure)\b|"
    r"\b(?:exit(?:ed)?(?:\s+with)?\s+status|exit\s+code|return\s+code)"
    r"\s*[:=]?\s*-?[1-9]\d*\b|"
    r"\bnon[- ]zero\s+(?:exit|return)(?:\s+(?:status|code))?\b|"
    r"\b(?:exited|completed)\s+unsuccessfully\b|"
    r"\b(?:command|process|child|program|tool|build|test|case)\s+was\s+unsuccessful\b",
    re.I,
)
EXIT_CODE_NATURAL_NON_PASS_OUTPUT_RE = re.compile(
    r"\b(?:PANIC(?:KED)?|ABORT(?:ED)?|KILLED|TERMINATED)\b|"
    r"\b(?:build|kernel|guest|qemu|process|thread|worker|child)\b[^\n]*"
    r"\b(?:panic(?:ked)?|crash(?:ed)?|hang(?:ed|ing)?|killed|terminated)\b|"
    r"\bpanic(?:ked)?\s*[:=]",
    re.I,
)
EXIT_CODE_WARNING_TERMINAL_RE = re.compile(
    r"\b(?:kernel|guest|qemu|process|thread|worker|child|test|suite|case)\b[^\n]*"
    r"\b(?:panic(?:ked)?|abort(?:ed)?|crash(?:ed)?|hang(?:ed|ing)?|killed|"
    r"terminated)\b|"
    r"\bpanic(?:ked)?\s+(?:occurred|detected)\b|"
    r"\b(?:test|suite|case|build|operation|command|job|task|process|worker|child)\b"
    r"[^\n]*\b(?:failed(?:\s+unexpectedly)?|"
    r"failure\s+(?:occurred|detected|reported)|"
    r"error\s+(?:occurred|detected|reported))\b",
    re.I,
)
EXIT_CODE_GENERIC_NON_PASS_OUTPUT_RE = re.compile(
    r"\b(?:FAIL(?:ED|URE|URES)?|ERRORS?)\b",
    re.I,
)
EXIT_CODE_UNAMBIGUOUS_PROTOCOL_RE = re.compile(
    r"\b(?:TCONF|TBROK|TFAIL|ENOSYS|XFAIL)\b",
    re.I,
)
EXIT_CODE_SOURCE_DISPLAY_LINE_RE = re.compile(
    r"^\s*(?:\d+\s+(?:\||[+\-~])(?:\s|\|)|\|\s*(?:[|^]|$))"
)
EXIT_CODE_WARNING_LINE_RE = re.compile(r"^\s*warning\s*:", re.I)
EXIT_CODE_BUILD_PROGRESS_LINE_RE = re.compile(
    r"^\s*(?:(?:Compiling|Checking|Fresh|Downloaded|Adding|Removing)\s+"
    r"[A-Za-z0-9_][A-Za-z0-9_.+-]*\s+v\d\S*(?:\s+\([^\r\n]*\))?|"
    r"Running\s+(?:unittests?\s+\S+|tests?/\S+|doc-tests?\s+\S+)"
    r"(?:\s+\([^\r\n]*\))?)\s*$"
)
EXIT_CODE_ZERO_FAILURE_COUNT_RE = re.compile(
    r"\b(?:errors?|failures?|failed attempts?|failed tests?|timeouts?|hangs?|"
    r"crashes?|panics?|aborts?|skips?)\s*[:=]\s*(?:0|zero|none)\b|"
    r"\b(?:error|failure|failed|timeout|hang|crash|panic|abort|skip)\s+count\s*"
    r"[:=]\s*(?:0|zero)\b|"
    r"\b(?:0|zero|none)\s+(?:errors?|failures?|failed|failed attempts?|failed tests?|"
    r"timeouts?|hangs?|crashes?|panics?|aborts?|skips?)\b|"
    r"\b(?:0|zero|none)\s+(?:tests?|cases?|commands?|targets?|builds?)\s+failed\b",
    re.I,
)
EXIT_CODE_NEGATED_FAILURE_RE = re.compile(
    r"\b(?:no|zero)\s+(?:errors?|failures?|timeouts?|hangs?|crashes?|panics?|"
    r"aborts?|skips?)\b|"
    r"\b(?:did\s+not|does\s+not|do\s+not|not|never)\s+"
    r"(?:fail(?:ed)?|error|abort(?:ed)?|hang|crash|panic|timeout)\b|"
    r"\bwithout(?:\s+any)?\s+(?:errors?|failures?|timeouts?|hangs?|crashes?|"
    r"panics?|aborts?|skips?)\b|"
    r"\b(?:error|failure|timeout|hang|crash|panic|abort|skip)[- ]free\b|"
    r"\b(?:fail(?:ed)?|abort(?:ed)?|hang|crash|panic|timeout)\s*\?\s*no\b",
    re.I,
)


def exit_code_non_pass_evidence(text: str) -> str | None:
    """Return one explicit non-pass line while ignoring bounded diagnostic prose."""

    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        scrubbed = EXIT_CODE_ZERO_FAILURE_COUNT_RE.sub("", raw_line)
        scrubbed = EXIT_CODE_NEGATED_FAILURE_RE.sub("", scrubbed)
        if EXIT_CODE_SOURCE_DISPLAY_LINE_RE.match(raw_line):
            if EXIT_CODE_UNAMBIGUOUS_PROTOCOL_RE.search(scrubbed):
                return line
            continue
        if EXIT_CODE_PROTOCOL_NON_PASS_OUTPUT_RE.search(scrubbed):
            return line
        if EXIT_CODE_BUILD_PROGRESS_LINE_RE.match(raw_line):
            continue
        if EXIT_CODE_WARNING_LINE_RE.match(raw_line):
            if EXIT_CODE_WARNING_TERMINAL_RE.search(scrubbed):
                return line
            continue
        if EXIT_CODE_NATURAL_NON_PASS_OUTPUT_RE.search(scrubbed):
            return line
        if EXIT_CODE_GENERIC_NON_PASS_OUTPUT_RE.search(scrubbed):
            return line
    return None
UNKNOWN_STATE_OUTPUT_RE = re.compile(
    rf"^(?:(?:{UNKNOWN_STATE_TOKEN_PATTERN})\s*|"
    rf"\[(?:{UNKNOWN_STATE_TOKEN_PATTERN})\](?:\s+.*)?|"
    rf"(?:{UNKNOWN_STATE_TOKEN_PATTERN})\s*(?::|=)\s*.*|"
    rf"case\s+\S.*:\s*(?:{UNKNOWN_STATE_TOKEN_PATTERN})(?:\s+.*)?)$",
    re.I | re.M,
)
UNKNOWN_STATUS_RECORD_RE = re.compile(
    r"^(?:STATUS|RESULT|STATE|CASE_STATUS)\s*(?:(?::|=)\s*\S.*|\[[^\]]+\](?:\s.*)?)$",
    re.I | re.M,
)
UNITTEST_SUMMARY_SIGNATURE_RE = re.compile(r"^Ran\b.*\btests?\b", re.I)
UNITTEST_OK_SIGNATURE_RE = re.compile(r"^(?:NOT\s+)?OK\S*(?:\s.*)?$", re.I)
UNITTEST_UNEXECUTED_COROUTINE_RE = re.compile(
    r"(?:RuntimeWarning|coroutine\b.*\bnever awaited|was never awaited)",
    re.I | re.S,
)
UNITTEST_BINDING_RE = re.compile(
    r"^UNITTEST_BINDING: planned=(\d+) started=(\d+) executed=(\d+) stopped=(\d+)$"
)
UNITTEST_BINDING_SIGNATURE_RE = re.compile(r"^UNITTEST_BINDING\b")
CARGO_TEST_RUNNING_RE = re.compile(r"^running\s+(\d+)\s+tests?\s*$")
CARGO_TEST_RESULT_RE = re.compile(
    r"^test result:\s+(ok|FAILED)\.\s+"
    r"(\d+) passed;\s+(\d+) failed;\s+(\d+) ignored;\s+"
    r"(\d+) measured;\s+(\d+) filtered out;\s+finished in\s+"
    r"(?:0|[1-9]\d*)(?:\.\d+)?s$"
)
CARGO_TEST_CASE_RE = re.compile(
    r"^test\s+(?P<name>.+?)\s+\.\.\.\s+(?P<status>ok|ignored|FAILED)\s*$"
)
CARGO_TEST_EXPECTED_PANIC_CASE_RE = re.compile(
    r"^(?P<identity>.+?) - should panic(?: with message .+)?$"
)
CARGO_TEST_BENIGN_OPTION_RE = re.compile(
    r"(?<![A-Za-z0-9_-])--no-fail-fast(?![A-Za-z0-9_-])"
)
CARGO_TEST_PANIC_HEADER_RE = re.compile(
    r"^thread '(?P<identity>[^']+)' panicked at .+:$"
)
CARGO_TEST_PANIC_NOTE_RE = re.compile(
    r"^note: run with `RUST_BACKTRACE=(?:1|full)` environment variable to display a backtrace$"
)
CARGO_TEST_SIGNATURE_RE = re.compile(r"^(?:running\b.*\btests?\b|test result:)", re.I)
CARGO_TEST_CASE_SIGNATURE_RE = re.compile(r"^test\s+.+?\s+\.\.\.\s+\S.*$", re.I)
CARGO_TEST_ALLOWED_STDOUT_EPILOGUE_RE = re.compile(
    r"^make(?:\[\d+\])?: Leaving directory ['`].+['`]$"
)
CARGO_TEST_IDENTITY_UNITTEST_PROGRESS_RE = re.compile(r"^\.+$")
CARGO_TEST_UNITTEST_SEPARATOR = "-" * 70
CARGO_TEST_EXPECTED_PANIC_MAX_REPORT_LINES = 32
TOKEN_RE = re.compile(r"\{[a-z_]+\}")
ALLOWED_TOKENS = {"{repo}", "{python}", "{output_dir}", "{case_output_dir}", "{arch}"}
TERMINATION_GRACE_SECONDS = 1.0
PR_SET_CHILD_SUBREAPER = 36
_SUBREAPER_ENABLED = False


def cargo_test_stderr_after_identity_unittests(
    stderr: str,
) -> tuple[str, list[int]]:
    """Account for exact successful identity-bound unittest blocks on stderr."""

    stderr_lines = stderr.splitlines()
    retained: list[str] = []
    accounted: list[int] = []
    index = 0
    block_line_count = 7
    while index < len(stderr_lines):
        if index + block_line_count > len(stderr_lines):
            retained.append(stderr_lines[index])
            index += 1
            continue

        block = stderr_lines[index : index + block_line_count]
        progress = CARGO_TEST_IDENTITY_UNITTEST_PROGRESS_RE.fullmatch(block[0])
        binding = UNITTEST_BINDING_RE.fullmatch(block[1])
        summary = UNITTEST_COUNT_RE.fullmatch(block[4])
        if progress is None or binding is None or summary is None:
            retained.append(stderr_lines[index])
            index += 1
            continue

        binding_counts = tuple(int(value) for value in binding.groups())
        planned = binding_counts[0]
        if not (
            planned > 0
            and len(block[0]) == planned
            and all(value == planned for value in binding_counts)
            and block[2] == ""
            and block[3] == CARGO_TEST_UNITTEST_SEPARATOR
            and int(summary.group(1)) == planned
            and block[5] == ""
            and block[6] == "OK"
        ):
            retained.append(stderr_lines[index])
            index += 1
            continue

        accounted.append(planned)
        index += block_line_count

    return "\n".join(retained), accounted


def cargo_test_stderr_after_expected_panics(
    stdout_lines: list[str], stderr: str
) -> tuple[str, list[str]]:
    """Account for bounded libtest reports from successful ``should_panic`` cases."""

    expected_counts: dict[str, int] = {}
    for line in stdout_lines:
        case_match = CARGO_TEST_CASE_RE.fullmatch(line)
        if case_match is None or case_match.group("status") != "ok":
            continue
        expected_match = CARGO_TEST_EXPECTED_PANIC_CASE_RE.fullmatch(
            case_match.group("name")
        )
        if expected_match is None:
            continue
        identity = expected_match.group("identity")
        expected_counts[identity] = expected_counts.get(identity, 0) + 1

    stderr_lines = stderr.splitlines()
    retained: list[str] = []
    accounted: list[str] = []
    index = 0
    while index < len(stderr_lines):
        header = CARGO_TEST_PANIC_HEADER_RE.fullmatch(stderr_lines[index])
        identity = header.group("identity") if header is not None else None
        if identity is None or expected_counts.get(identity, 0) <= 0:
            retained.append(stderr_lines[index])
            index += 1
            continue

        note_index: int | None = None
        report_limit = min(
            len(stderr_lines),
            index + 1 + CARGO_TEST_EXPECTED_PANIC_MAX_REPORT_LINES,
        )
        for candidate_index in range(index + 1, report_limit):
            candidate = stderr_lines[candidate_index]
            if CARGO_TEST_PANIC_NOTE_RE.fullmatch(candidate):
                note_index = candidate_index
                break
            if (
                CARGO_TEST_PANIC_HEADER_RE.fullmatch(candidate) is not None
                or TRUSTED_BUILD_STDERR_RE.fullmatch(candidate) is not None
            ):
                break
        if note_index is None:
            retained.append(stderr_lines[index])
            index += 1
            continue

        report_body = "\n".join(stderr_lines[index + 1 : note_index])
        if (
            UNKNOWN_STATUS_RECORD_RE.search(report_body)
            or NON_PASS_OUTPUT_RE.search(report_body)
            or EXPLICIT_FAILURE_OUTPUT_RE.search(report_body)
            or CRASH_OUTPUT_RE.search(report_body)
        ):
            retained.append(stderr_lines[index])
            index += 1
            continue

        expected_counts[identity] -= 1
        accounted.append(identity)
        index = note_index + 1

    return "\n".join(retained), accounted


CANONICAL_OFFICIAL_ENVIRONMENT = {
    "ORAYS_TEST_OUTPUT_DIR": "{case_output_dir}",
    "OSCOMP_TEST_GROUPS": "all",
    "LTP_CASES": CANONICAL_LTP_CASE_LIST,
    "LTP_CASE_TIMEOUT_SECS": "180",
    "OSCOMP_GROUP_TIMEOUT_CEILING_SECS": "900",
}
OFFICIAL_BLACKLIST_FILE_ENVIRONMENT = (
    "LTP_BLACKLIST_FILE",
    "LTP_BLACKLIST_COMMON_FILE",
    "LTP_BLACKLIST_RV_FILE",
    "LTP_BLACKLIST_LA_FILE",
)
OFFICIAL_BLACKLIST_ENVIRONMENT = (
    "LTP_BLACKLIST",
    *OFFICIAL_BLACKLIST_FILE_ENVIRONMENT,
    "LTP_BLACKLIST_RV",
    "LTP_BLACKLIST_RISCV64",
    "LTP_BLACKLIST_LA",
    "LTP_BLACKLIST_LOONGARCH64",
)
OFFICIAL_CALLER_ENVIRONMENT = (
    "ORAYS_WORKSPACE_ROOT",
    "TESTSUITE_DIR",
    "RV_TESTSUITE_IMG",
    "LA_TESTSUITE_IMG",
    *OFFICIAL_BLACKLIST_ENVIRONMENT,
    "OSCOMP_SKIP_TEST_GROUPS",
)
FINAL_2026_CALLER_ENVIRONMENT = (
    "RV_CAGENT_FINAL_2026_IMG",
    "RV_CAGENT_FINAL_2026_IMG_SHA256",
    "RV_BUILDSTORM_FINAL_2026_IMG",
    "RV_BUILDSTORM_FINAL_2026_IMG_SHA256",
    "LA_CAGENT_FINAL_2026_IMG",
    "LA_CAGENT_FINAL_2026_IMG_SHA256",
    "LA_BUILDSTORM_FINAL_2026_IMG",
    "LA_BUILDSTORM_FINAL_2026_IMG_SHA256",
    "FINAL_2026_PROTOCOL_ROOT",
)
CANONICAL_FINAL_2026_ENVIRONMENT = {
    "ORAYS_TEST_OUTPUT_DIR": "{case_output_dir}",
}
CANONICAL_OFFICIAL_EXECUTION = {
    "rv": {
        "timeout_seconds": 21600,
        "required_paths": [
            "{repo}/test/evaluation/run_official_evaluation.sh",
            "{repo}/test/evaluation/official_case_plan.json",
        ],
        "required_commands": ["make", "cargo", "qemu-img", "qemu-system-riscv64"],
        "required_files": [
            {
                "environment": "RV_TESTSUITE_IMG",
                "directory_environment": "TESTSUITE_DIR",
                "basename": "sdcard-rv.img",
                "fallback": "{repo}/../sdcard-rv.img",
            }
        ],
        "infrastructure_exit_codes": [125],
    },
    "la": {
        "timeout_seconds": 21600,
        "required_paths": [
            "{repo}/test/evaluation/run_official_evaluation.sh",
            "{repo}/test/evaluation/official_case_plan.json",
        ],
        "required_commands": ["make", "cargo", "qemu-img", "qemu-system-loongarch64"],
        "required_files": [
            {
                "environment": "LA_TESTSUITE_IMG",
                "directory_environment": "TESTSUITE_DIR",
                "basename": "sdcard-la.img",
                "fallback": "{repo}/../sdcard-la.img",
            }
        ],
        "infrastructure_exit_codes": [125],
    },
}
CANONICAL_FINAL_2026_ARCH_CASES = {
    "rv": ["final.cagent.riscv64", "final.buildstorm.riscv64"],
    "la": ["final.cagent.loongarch64", "final.buildstorm.loongarch64"],
}
CANONICAL_FINAL_2026_EXECUTION = {
    "final.cagent.riscv64": {
        "architecture": "riscv64",
        "group": "cagent",
        "image_environment": "RV_CAGENT_FINAL_2026_IMG",
        "timeout_seconds": 1800,
        "qemu": "qemu-system-riscv64",
    },
    "final.buildstorm.riscv64": {
        "architecture": "riscv64",
        "group": "buildstorm",
        "image_environment": "RV_BUILDSTORM_FINAL_2026_IMG",
        "timeout_seconds": 21600,
        "qemu": "qemu-system-riscv64",
    },
    "final.cagent.loongarch64": {
        "architecture": "loongarch64",
        "group": "cagent",
        "image_environment": "LA_CAGENT_FINAL_2026_IMG",
        "timeout_seconds": 1800,
        "qemu": "qemu-system-loongarch64",
    },
    "final.buildstorm.loongarch64": {
        "architecture": "loongarch64",
        "group": "buildstorm",
        "image_environment": "LA_BUILDSTORM_FINAL_2026_IMG",
        "timeout_seconds": 21600,
        "qemu": "qemu-system-loongarch64",
    },
}
CANONICAL_CHECK_CASE_IDS = (
    "check.compliance_regressions",
    "check.competition_semantic_evidence",
    "check.evaluation_runner_and_parser_integrity",
    "check.file_object_event_core",
    "check.kernel_state_backed_semantics",
    "check.libc_stateful_semantics",
    "check.linux_boundary",
    "check.memory_policy_semantics",
    "check.no_fake_success",
    "check.posix_state_integrity",
    "check.rlimit_and_fd_semantics",
    "check.runtime_binary_patch_prohibition",
    "check.socket_message_and_buffer_semantics",
    "check.stat_metadata_semantics",
    "check.synthetic_capability_integrity",
    "check.syscall_boundary_regressions",
    "check.test_asset_integrity",
    "check.timer_semantics",
    "check.user_memory_copy_boundaries",
)
CANONICAL_UNIT_CASE_IDS = (
    "unit.compliance_regressions",
    "unit.competition_semantic_evidence",
    "unit.evaluation_failure_report",
    "unit.evaluation_runner_and_parser_integrity",
    "unit.evaluator_protocol",
    "unit.final_2026_adapter",
    "unit.final_2026_result_validation",
    "unit.file_object_event_core",
    "unit.kernel_state_backed_semantics",
    "unit.libc_stateful_semantics",
    "unit.linux_boundary",
    "unit.ltp_result_summary",
    "unit.memory_policy_semantics",
    "unit.no_fake_success",
    "unit.official_result_validation",
    "unit.posix_state_integrity",
    "unit.rlimit_and_fd_semantics",
    "unit.qemu_setup",
    "unit.runtime_binary_patch_prohibition",
    "unit.socket_message_and_buffer_semantics",
    "unit.stat_metadata_semantics",
    "unit.semantic_evidence",
    "unit.suite_runner",
    "unit.synthetic_capability_integrity",
    "unit.syscall_boundary_regressions",
    "unit.test_asset_integrity",
    "unit.timer_semantics",
    "unit.user_memory_copy_boundaries",
)
CANONICAL_UNIT_EXPECTED_TESTS = {
    "unit.compliance_regressions": 13,
    "unit.competition_semantic_evidence": 33,
    "unit.evaluation_failure_report": 9,
    "unit.evaluation_runner_and_parser_integrity": 29,
    "unit.evaluator_protocol": 27,
    "unit.final_2026_adapter": 8,
    "unit.final_2026_result_validation": 35,
    "unit.file_object_event_core": 33,
    "unit.kernel_state_backed_semantics": 41,
    "unit.libc_stateful_semantics": 9,
    "unit.linux_boundary": 17,
    "unit.ltp_result_summary": 20,
    "unit.memory_policy_semantics": 3,
    "unit.no_fake_success": 10,
    "unit.official_result_validation": 111,
    "unit.posix_state_integrity": 16,
    "unit.rlimit_and_fd_semantics": 13,
    "unit.qemu_setup": 9,
    "unit.runtime_binary_patch_prohibition": 9,
    "unit.socket_message_and_buffer_semantics": 10,
    "unit.stat_metadata_semantics": 7,
    "unit.semantic_evidence": 75,
    "unit.suite_runner": 135,
    "unit.synthetic_capability_integrity": 5,
    "unit.syscall_boundary_regressions": 36,
    "unit.test_asset_integrity": 36,
    "unit.timer_semantics": 3,
    "unit.user_memory_copy_boundaries": 13,
}
CANONICAL_PYTHON_EXTRA_REQUIRED_PATHS = {
    "unit.competition_semantic_evidence": [
        "{repo}/test/checks/check_competition_semantic_evidence.py",
    ],
    "unit.evaluation_failure_report": [
        "{repo}/test/evaluation/report_evaluation_failures.py",
        "{repo}/test/evaluation/parse_official_results.py",
    ],
    "unit.final_2026_result_validation": [
        "{repo}/test/evaluation/parse_final_2026_results.py",
    ],
    "unit.final_2026_adapter": [
        "{repo}/test/evaluation/run_final_2026_evaluation.py",
        "{repo}/test/images/manifest.final-2026.json",
    ],
    "unit.file_object_event_core": [
        "{repo}/test/checks/check_file_object_event_core.py",
    ],
    "unit.evaluator_protocol": [
        "{repo}/test/evidence/evaluator_protocol.py",
    ],
    "unit.linux_boundary": [
        "{repo}/test/checks/check_linux_boundary.py",
    ],
    "unit.qemu_setup": [
        "{repo}/test/evidence/setup_qemu.sh",
    ],
    "unit.semantic_evidence": [
        "{repo}/test/evidence/evaluator_protocol.py",
        "{repo}/test/evidence/render_semantic_evidence.py",
        "{repo}/test/evidence/semantic_evidence.py",
        "{repo}/test/evidence/semantic_evidence_manifest.json",
        "{repo}/test/evidence/semantic_evidence_schema.v1.json",
        "{repo}/test/fixtures/semantic_evidence/guard-ambiguous.txt",
        "{repo}/test/fixtures/semantic_evidence/guard-pass.txt",
        "{repo}/test/fixtures/semantic_evidence/smoke-rv64-duplicate.txt",
        "{repo}/test/fixtures/semantic_evidence/smoke-rv64-panic-after-pass.txt",
        "{repo}/test/fixtures/semantic_evidence/smoke-rv64-pass.txt",
        "{repo}/test/fixtures/semantic_evidence/smoke-rv64-truncated.txt",
    ],
}
CANONICAL_EVIDENCE_CASE_IDS = (
    "evidence.host",
    "evidence.riscv64",
    "evidence.loongarch64",
    "evidence.aggregate",
)
CANONICAL_EVIDENCE_COMMANDS = {
    "evidence.host": [
        "{python}", "-I", "-S", "-B", "-X", "pycache_prefix=/dev/null",
        "{repo}/test/evidence/semantic_evidence.py", "run",
        "--manifest", "{repo}/test/evidence/semantic_evidence_manifest.json",
        "--output", "build/pr3-evidence/host", "--arch", "host",
    ],
    "evidence.riscv64": [
        "{python}", "-I", "-S", "-B", "-X", "pycache_prefix=/dev/null",
        "{repo}/test/evidence/semantic_evidence.py", "run",
        "--manifest", "{repo}/test/evidence/semantic_evidence_manifest.json",
        "--output", "build/pr3-evidence/rv64", "--arch", "riscv64",
    ],
    "evidence.loongarch64": [
        "{python}", "-I", "-S", "-B", "-X", "pycache_prefix=/dev/null",
        "{repo}/test/evidence/semantic_evidence.py", "run",
        "--manifest", "{repo}/test/evidence/semantic_evidence_manifest.json",
        "--output", "build/pr3-evidence/la64", "--arch", "loongarch64",
    ],
    "evidence.aggregate": ["{repo}/test/evidence/aggregate_semantic_evidence.sh"],
}
CANONICAL_EVIDENCE_REQUIRED_PATHS = {
    "evidence.host": [
        "{repo}/test/evidence/evaluator_protocol.py",
        "{repo}/test/evidence/semantic_evidence.py",
        "{repo}/test/evidence/semantic_evidence_manifest.json",
    ],
    "evidence.riscv64": [
        "{repo}/test/evidence/evaluator_protocol.py",
        "{repo}/test/evidence/semantic_evidence.py",
        "{repo}/test/evidence/semantic_evidence_manifest.json",
        "{repo}/scripts/rust-lld.sh",
        "{repo}/scripts/rust-objcopy.sh",
    ],
    "evidence.loongarch64": [
        "{repo}/test/evidence/evaluator_protocol.py",
        "{repo}/test/evidence/semantic_evidence.py",
        "{repo}/test/evidence/semantic_evidence_manifest.json",
        "{repo}/scripts/rust-lld.sh",
        "{repo}/scripts/rust-objcopy.sh",
    ],
    "evidence.aggregate": [
        "{repo}/test/evidence/aggregate_semantic_evidence.sh",
        "{repo}/test/evidence/render_semantic_evidence.py",
        "{repo}/test/evidence/semantic_evidence.py",
        "{repo}/test/evidence/semantic_evidence_manifest.json",
    ],
}
CANONICAL_EVIDENCE_REQUIRED_COMMANDS = {
    "evidence.host": ["python3"],
    "evidence.riscv64": [
        "python3", "make", "cargo", "rustc", "qemu-system-riscv64",
    ],
    "evidence.loongarch64": [
        "python3", "make", "cargo", "rustc", "qemu-system-loongarch64",
    ],
    "evidence.aggregate": ["bash", "python3"],
}
CANONICAL_EVIDENCE_TIMEOUTS = {
    "evidence.host": 1800,
    "evidence.riscv64": 7200,
    "evidence.loongarch64": 7200,
    "evidence.aggregate": 600,
}
CANONICAL_BASELINE_CASE_IDS = (
    "baseline.cargo_format",
    "baseline.workspace_unit_tests",
    "baseline.clippy_default",
    "baseline.clippy_riscv64",
    "baseline.clippy_loongarch64",
    "baseline.kernel_riscv64",
    "baseline.kernel_loongarch64",
    "baseline.submission_build",
)
CANONICAL_BASELINE_COMMANDS = {
    "baseline.cargo_format": ["cargo", "fmt", "--all", "--", "--check"],
    "baseline.workspace_unit_tests": ["make", "-C", "{repo}", "unittest_no_fail_fast"],
    "baseline.clippy_default": ["make", "-C", "{repo}", "clippy"],
    "baseline.clippy_riscv64": ["make", "-C", "{repo}", "clippy", "ARCH=riscv64"],
    "baseline.clippy_loongarch64": ["make", "-C", "{repo}", "clippy", "ARCH=loongarch64"],
    "baseline.kernel_riscv64": ["make", "-C", "{repo}", "kernel-rv"],
    "baseline.kernel_loongarch64": ["make", "-C", "{repo}", "kernel-la"],
    "baseline.submission_build": ["make", "-C", "{repo}", "all"],
}
CAPABILITY_PROBE_TIMEOUT_SECONDS = 5
CAPABILITY_PROBES = {
    "clang.target.loongarch64-unknown-none": {
        "required_command": "clang",
        "arguments": [
            "--target=loongarch64-unknown-none",
            "-x",
            "c",
            "-fsyntax-only",
            "-",
        ],
    }
}
CANONICAL_BASELINE_REQUIRED_CAPABILITIES = {
    "baseline.clippy_loongarch64": ["clang.target.loongarch64-unknown-none"],
}
CANONICAL_BASELINE_REQUIRED_COMMANDS = {
    "baseline.cargo_format": ["cargo"],
    "baseline.workspace_unit_tests": ["make", "cargo"],
    "baseline.clippy_default": ["make", "cargo"],
    "baseline.clippy_riscv64": ["make", "cargo"],
    "baseline.clippy_loongarch64": ["make", "cargo", "clang"],
    "baseline.kernel_riscv64": ["make", "cargo"],
    "baseline.kernel_loongarch64": ["make", "cargo"],
    "baseline.submission_build": ["make", "cargo"],
}
CANONICAL_BASELINE_TIMEOUTS = {
    "baseline.cargo_format": 600,
    "baseline.workspace_unit_tests": 3600,
    "baseline.clippy_default": 1800,
    "baseline.clippy_riscv64": 1800,
    "baseline.clippy_loongarch64": 1800,
    "baseline.kernel_riscv64": 3600,
    "baseline.kernel_loongarch64": 3600,
    "baseline.submission_build": 5400,
}
CANONICAL_BASELINE_RESULT_CONTRACTS = {
    "baseline.cargo_format": {"type": "exit_code", "allow_empty_output": True},
    "baseline.workspace_unit_tests": {"type": "cargo_test"},
    "baseline.clippy_default": {"type": "exit_code", "allow_empty_output": False},
    "baseline.clippy_riscv64": {"type": "exit_code", "allow_empty_output": False},
    "baseline.clippy_loongarch64": {"type": "exit_code", "allow_empty_output": False},
    "baseline.kernel_riscv64": {"type": "exit_code", "allow_empty_output": False},
    "baseline.kernel_loongarch64": {"type": "exit_code", "allow_empty_output": False},
    "baseline.submission_build": {"type": "exit_code", "allow_empty_output": False},
}


class ManifestError(ValueError):
    """Raised when a manifest or profile cannot be trusted."""


class RunnerTermination(Exception):
    """Raised by the narrow SIGTERM handler while a child group is active."""

    def __init__(self, signum: int) -> None:
        super().__init__(signum)
        self.signum = signum


class OutputIntegrityError(ValueError):
    """Raised when captured child output cannot be parsed as trustworthy text."""


class ProcessSnapshotError(RuntimeError):
    """Raised when descendant containment cannot rely on a complete /proc view."""


def reject_duplicate_json_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ManifestError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def reject_non_finite_json(value: str) -> None:
    raise ManifestError(f"non-finite JSON number is not allowed: {value}")


@dataclass
class Selection:
    profile: str
    architecture: str | None
    cases: list[dict[str, Any]]
    case_architectures: list[str | None]


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat().replace("+00:00", "Z")


def repository_root() -> Path:
    return Path(__file__).resolve().parent.parent


def _require_type(value: Any, expected: type, location: str) -> None:
    if not isinstance(value, expected):
        raise ManifestError(f"{location} must be {expected.__name__}")


def _reject_unknown_keys(value: dict[str, Any], allowed: set[str], location: str) -> None:
    unknown = sorted(set(value) - allowed)
    if unknown:
        raise ManifestError(f"{location} contains unsupported fields: {unknown}")


def _validate_string_list(value: Any, location: str, *, allow_empty: bool = True) -> list[str]:
    _require_type(value, list, location)
    if not allow_empty and not value:
        raise ManifestError(f"{location} must not be empty")
    if any(not isinstance(item, str) or not item for item in value):
        raise ManifestError(f"{location} must contain only non-empty strings")
    for index, item in enumerate(value):
        _reject_embedded_nul(item, f"{location}[{index}]")
    return value


def _reject_embedded_nul(value: str, location: str) -> None:
    if "\x00" in value:
        raise ManifestError(f"{location} contains an embedded NUL byte")


def _check_tokens(value: str, location: str) -> None:
    _reject_embedded_nul(value, location)
    unknown = set(TOKEN_RE.findall(value)) - ALLOWED_TOKENS
    if unknown:
        raise ManifestError(f"{location} uses unsupported placeholders: {sorted(unknown)}")
    if "{" in TOKEN_RE.sub("", value) or "}" in TOKEN_RE.sub("", value):
        raise ManifestError(f"{location} contains malformed placeholder syntax")


def _safe_repo_path(repo: Path, raw: str, location: str, *, directory: bool = False) -> Path:
    _check_tokens(raw, location)
    expanded = raw.replace("{repo}", str(repo))
    if any(token in expanded for token in ALLOWED_TOKENS):
        raise ManifestError(f"{location} cannot use a runtime-only placeholder")
    path = Path(expanded)
    if not path.is_absolute():
        path = repo / path
    path = path.resolve()
    try:
        path.relative_to(repo.resolve())
    except ValueError as error:
        raise ManifestError(f"{location} escapes the repository: {raw}") from error
    if directory and not path.is_dir():
        raise ManifestError(f"{location} is not an existing directory: {raw}")
    if not directory and not path.is_file():
        raise ManifestError(f"{location} is not an existing file: {raw}")
    return path


def _live_ltp_stable_cases(repo: Path) -> list[str]:
    try:
        return trusted_ltp_stable_cases(repo)
    except (OSError, ValueError) as error:
        raise ManifestError(f"cannot read trusted LTP plan source: {error}") from error


def _is_plain_unittest_main_guard(node: ast.stmt) -> bool:
    if not isinstance(node, ast.If) or node.orelse or len(node.body) != 1:
        return False
    comparison = node.test
    if (
        not isinstance(comparison, ast.Compare)
        or not isinstance(comparison.left, ast.Name)
        or comparison.left.id != "__name__"
        or len(comparison.ops) != 1
        or not isinstance(comparison.ops[0], ast.Eq)
        or len(comparison.comparators) != 1
        or not isinstance(comparison.comparators[0], ast.Constant)
        or comparison.comparators[0].value != "__main__"
    ):
        return False
    statement = node.body[0]
    if not isinstance(statement, ast.Expr) or not isinstance(statement.value, ast.Call):
        return False
    call = statement.value
    return (
        isinstance(call.func, ast.Attribute)
        and isinstance(call.func.value, ast.Name)
        and call.func.value.id == "unittest"
        and call.func.attr == "main"
        and not call.args
        and not call.keywords
    )


def _contains_yield_in_function_scope(function: ast.FunctionDef) -> bool:
    class YieldVisitor(ast.NodeVisitor):
        found = False

        def visit_Yield(self, _node: ast.Yield) -> None:
            self.found = True

        def visit_YieldFrom(self, _node: ast.YieldFrom) -> None:
            self.found = True

        def visit_FunctionDef(self, _node: ast.FunctionDef) -> None:
            return

        def visit_AsyncFunctionDef(self, _node: ast.AsyncFunctionDef) -> None:
            return

        def visit_Lambda(self, _node: ast.Lambda) -> None:
            return

        def visit_ClassDef(self, _node: ast.ClassDef) -> None:
            return

    visitor = YieldVisitor()
    for statement in function.body:
        visitor.visit(statement)
    return visitor.found


def canonical_unittest_inventory(
    tree: ast.Module,
    implementation_path: Path,
) -> list[tuple[str, str, int]]:
    """Return exact directly discoverable synchronous canonical test identities."""

    for node in ast.walk(tree):
        binds_load_tests = False
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            binds_load_tests = node.name == "load_tests"
        elif isinstance(node, ast.Name):
            binds_load_tests = node.id == "load_tests" and isinstance(
                node.ctx, (ast.Store, ast.Del)
            )
        elif isinstance(node, ast.alias):
            bound_name = node.asname or node.name.split(".", 1)[0]
            binds_load_tests = bound_name == "load_tests"
        elif isinstance(node, ast.ExceptHandler):
            binds_load_tests = node.name == "load_tests"
        elif isinstance(node, (ast.MatchAs, ast.MatchStar)):
            binds_load_tests = node.name == "load_tests"
        if binds_load_tests:
            raise ManifestError(
                f"canonical unittest {implementation_path} must not bind load_tests at any scope"
            )

    main_guards = [node for node in tree.body if _is_plain_unittest_main_guard(node)]
    if len(main_guards) != 1 or tree.body[-1] is not main_guards[0]:
        raise ManifestError(
            f"canonical unittest {implementation_path} must end with exactly one plain unittest.main() guard"
        )
    allowed_main_name_nodes = {
        id(node)
        for node in ast.walk(main_guards[0].test)
        if isinstance(node, ast.Name) and node.id == "__name__"
    }
    allowed_runtime_name_nodes = set(allowed_main_name_nodes)
    for function in (
        node
        for node in ast.walk(tree)
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
    ):
        for statement in function.body:
            allowed_runtime_name_nodes.update(
                id(node)
                for node in ast.walk(statement)
                if isinstance(node, ast.Name) and node.id == "__name__"
            )
    if any(
        isinstance(node, ast.Name)
        and node.id == "__name__"
        and id(node) not in allowed_runtime_name_nodes
        for node in ast.walk(tree)
    ):
        raise ManifestError(
            f"canonical unittest {implementation_path} must not observe synthetic module identity outside its final guard"
        )
    forbidden_private_state = {
        "_outcome",
        "_cleanups",
        "_class_cleanups",
        "_module_cleanups",
    }
    if any(
        isinstance(node, ast.Attribute) and node.attr in forbidden_private_state
        for node in ast.walk(tree)
    ):
        raise ManifestError(
            f"canonical unittest {implementation_path} must not mutate unittest private outcome or cleanup state"
        )

    lifecycle_names = {
        "setUp",
        "tearDown",
        "setUpClass",
        "tearDownClass",
        "setUpModule",
        "tearDownModule",
    }
    for node in ast.walk(tree):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        if node.name not in lifecycle_names:
            continue
        if isinstance(node, ast.AsyncFunctionDef) or (
            isinstance(node, ast.FunctionDef) and _contains_yield_in_function_scope(node)
        ):
            raise ManifestError(
                f"canonical unittest {implementation_path} lifecycle hook {node.name} must execute synchronously"
            )

    direct_methods: list[tuple[str, ast.FunctionDef]] = []
    direct_method_ids: set[int] = set()
    for node in tree.body:
        if not isinstance(node, ast.ClassDef):
            continue
        declared_test_members = [
            member
            for member in node.body
            if isinstance(member, (ast.FunctionDef, ast.AsyncFunctionDef))
            and member.name.startswith("test")
        ]
        if not declared_test_members:
            continue
        is_direct_test_case = (
            len(node.bases) == 1
            and isinstance(node.bases[0], ast.Attribute)
            and isinstance(node.bases[0].value, ast.Name)
            and node.bases[0].value.id == "unittest"
            and node.bases[0].attr == "TestCase"
            and not node.decorator_list
            and not node.keywords
        )
        if not is_direct_test_case:
            raise ManifestError(
                f"canonical unittest {implementation_path} test class {node.name} must be an "
                "undecorated direct unittest.TestCase without mixins or metaclass keywords"
            )
        unsupported_discoverable_names = sorted(
            member.name
            for member in declared_test_members
            if not member.name.startswith("test_")
        )
        if unsupported_discoverable_names:
            raise ManifestError(
                f"canonical unittest {implementation_path} uses discoverable test names without the test_ prefix: "
                f"{unsupported_discoverable_names}"
            )
        forbidden_overrides = {
            "run",
            "runTest",
            "__call__",
            "_callTestMethod",
            "debug",
            "countTestCases",
        }
        overridden = sorted(
            member.name
            for member in node.body
            if isinstance(member, (ast.FunctionDef, ast.AsyncFunctionDef))
            and member.name in forbidden_overrides
        )
        if overridden:
            raise ManifestError(
                f"canonical unittest {implementation_path} test class {node.name} overrides "
                f"unittest execution hooks: {overridden}"
            )
        for member in node.body:
            if isinstance(member, ast.AsyncFunctionDef) and member.name.startswith("test"):
                raise ManifestError(
                    f"canonical unittest {implementation_path} contains async test method {member.name}"
                )
            if not isinstance(member, ast.FunctionDef) or not member.name.startswith("test_"):
                continue
            if member.decorator_list:
                raise ManifestError(
                    f"canonical unittest {implementation_path} contains decorated test method {member.name}"
                )
            if _contains_yield_in_function_scope(member):
                raise ManifestError(
                    f"canonical unittest {implementation_path} contains generator test method {member.name}"
                )
            direct_methods.append((node.name, member))
            direct_method_ids.add(id(member))

    all_named_tests = [
        node
        for node in ast.walk(tree)
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
        and node.name.startswith("test")
    ]
    if any(id(node) not in direct_method_ids for node in all_named_tests):
        raise ManifestError(
            f"canonical unittest {implementation_path} contains a test_ function outside a direct unittest.TestCase method"
        )
    if not direct_methods:
        raise ManifestError(f"canonical unittest {implementation_path} contains zero test methods")
    return [
        (class_name, method.name, method.lineno)
        for class_name, method in direct_methods
    ]


def canonical_unittest_method_count(tree: ast.Module, implementation_path: Path) -> int:
    """Count exact canonical test identities after applying discovery restrictions."""

    return len(canonical_unittest_inventory(tree, implementation_path))


def load_manifest(path: Path, repo: Path) -> dict[str, Any]:
    canonical_manifest = path.resolve() == (repo / "test/suite_manifest.json").resolve()
    if not path.is_file():
        raise ManifestError(f"manifest not found: {path}")
    try:
        manifest = json.loads(
            path.read_text(encoding="utf-8"),
            object_pairs_hook=reject_duplicate_json_keys,
            parse_constant=reject_non_finite_json,
        )
    except json.JSONDecodeError as error:
        raise ManifestError(
            f"malformed manifest JSON at line {error.lineno}, column {error.colno}: {error.msg}"
        ) from error
    except UnicodeDecodeError as error:
        raise ManifestError(
            f"manifest is not valid UTF-8 at byte offset {error.start}"
        ) from error
    except OSError as error:
        raise ManifestError(f"cannot read manifest: {error}") from error
    _require_type(manifest, dict, "manifest")
    _reject_unknown_keys(manifest, {"schema_version", "baseline_ref", "profiles", "cases"}, "manifest")
    if type(manifest.get("schema_version")) is not int or manifest["schema_version"] != SCHEMA_VERSION:
        raise ManifestError(
            f"unsupported manifest schema_version {manifest.get('schema_version')!r}; expected {SCHEMA_VERSION}"
        )
    baseline_ref = manifest.get("baseline_ref", "origin/main")
    if not isinstance(baseline_ref, str) or not baseline_ref or baseline_ref.startswith("-"):
        raise ManifestError("baseline_ref must be a non-empty revision name that does not start with '-'")
    _reject_embedded_nul(baseline_ref, "baseline_ref")
    profiles = manifest.get("profiles")
    cases = manifest.get("cases")
    _require_type(profiles, dict, "profiles")
    _require_type(cases, list, "cases")
    if not profiles:
        raise ManifestError("profiles must not be empty")
    if not cases:
        raise ManifestError("cases must not be empty")

    case_ids: set[str] = set()
    for index, case in enumerate(cases):
        location = f"cases[{index}]"
        _require_type(case, dict, location)
        _reject_unknown_keys(
            case,
            {
                "id",
                "description",
                "command",
                "cwd",
                "timeout_seconds",
                "result_contract",
                "required_paths",
                "required_commands",
                "required_capabilities",
                "required_files",
                "environment",
                "infrastructure_exit_codes",
            },
            location,
        )
        case_id = case.get("id")
        if not isinstance(case_id, str) or not CASE_ID_RE.fullmatch(case_id):
            raise ManifestError(f"{location}.id is not a stable semantic ID: {case_id!r}")
        if HISTORICAL_ID_RE.search(case_id):
            raise ManifestError(f"{location}.id contains a historical sequence ID: {case_id}")
        if case_id in case_ids:
            raise ManifestError(f"duplicate test ID: {case_id}")
        case_ids.add(case_id)
        description = case.get("description", "")
        if not isinstance(description, str):
            raise ManifestError(f"{location}.description must be a string")

        command = _validate_string_list(case.get("command"), f"{location}.command", allow_empty=False)
        for command_index, value in enumerate(command):
            _check_tokens(value, f"{location}.command[{command_index}]")
        cwd = case.get("cwd", "{repo}")
        if not isinstance(cwd, str) or not cwd:
            raise ManifestError(f"{location}.cwd must be a non-empty string")
        _safe_repo_path(repo, cwd, f"{location}.cwd", directory=True)

        timeout = case.get("timeout_seconds")
        timeout_is_finite = False
        if not isinstance(timeout, bool) and isinstance(timeout, (int, float)):
            try:
                timeout_is_finite = math.isfinite(timeout)
            except OverflowError:
                timeout_is_finite = False
        if not timeout_is_finite or timeout <= 0:
            raise ManifestError(f"{location}.timeout_seconds must be a finite positive number")
        contract = case.get("result_contract")
        _require_type(contract, dict, f"{location}.result_contract")
        result_type = contract.get("type")
        if result_type not in RESULT_TYPES:
            raise ManifestError(f"{location}.result_contract.type is unsupported: {result_type!r}")
        allowed_contract_keys = {"type"}
        if result_type == "exit_code":
            allowed_contract_keys.add("allow_empty_output")
        if result_type == "unittest":
            allowed_contract_keys.update({"expected_tests", "identity_binding"})
        if result_type == "official":
            allowed_contract_keys.update(
                {"expected_group_labels", "expected_group_case_counts"}
            )
        if result_type == "final_2026":
            allowed_contract_keys.update(
                {
                    "expected_group",
                    "expected_group_label",
                    "expected_arch",
                    "buildstorm_baseline_seconds",
                }
            )
        _reject_unknown_keys(contract, allowed_contract_keys, f"{location}.result_contract")
        if result_type == "exit_code" and not isinstance(
            contract.get("allow_empty_output", False), bool
        ):
            raise ManifestError(
                f"{location}.result_contract.allow_empty_output must be a boolean"
            )
        if result_type == "unittest":
            expected = contract.get("expected_tests")
            if isinstance(expected, bool) or not isinstance(expected, int) or expected <= 0:
                raise ManifestError(
                    f"{location}.result_contract.expected_tests must be a positive integer"
                )
            identity_binding = contract.get("identity_binding", False)
            if not isinstance(identity_binding, bool):
                raise ManifestError(
                    f"{location}.result_contract.identity_binding must be a boolean"
                )
        if result_type == "official":
            if "expected_group_labels" not in contract:
                raise ManifestError(
                    f"{location}.result_contract.expected_group_labels is required for official cases"
                )
            expected_groups = _validate_string_list(
                contract["expected_group_labels"],
                f"{location}.result_contract.expected_group_labels",
                allow_empty=False,
            )
            if len(expected_groups) != len(set(expected_groups)):
                raise ManifestError(
                    f"{location}.result_contract.expected_group_labels contains duplicates"
                )
            expected_case_counts = contract.get("expected_group_case_counts", {})
            _require_type(
                expected_case_counts,
                dict,
                f"{location}.result_contract.expected_group_case_counts",
            )
            for label, count in expected_case_counts.items():
                if not isinstance(label, str) or not label:
                    raise ManifestError(
                        f"{location}.result_contract.expected_group_case_counts has an invalid label"
                    )
                _reject_embedded_nul(
                    label,
                    f"{location}.result_contract.expected_group_case_counts label",
                )
                if isinstance(count, bool) or not isinstance(count, int) or count <= 0:
                    raise ManifestError(
                        f"{location}.result_contract.expected_group_case_counts.{label} "
                        "must be a positive integer"
                    )
            count_required_labels = {
                label
                for label in expected_groups
                if label.startswith(("ltp-", "busybox-", "libctest-"))
            }
            if set(expected_case_counts) != count_required_labels:
                raise ManifestError(
                    f"{location}.result_contract.expected_group_case_counts must exactly cover "
                    f"LTP/busybox/libctest groups: {sorted(count_required_labels)}"
                )
        if result_type == "final_2026":
            expected_group = contract.get("expected_group")
            if expected_group not in FINAL_2026_GROUPS:
                raise ManifestError(
                    f"{location}.result_contract.expected_group must be one of "
                    f"{sorted(FINAL_2026_GROUPS)}"
                )
            expected_group_label = contract.get("expected_group_label")
            if (
                expected_group in FINAL_2026_GROUPS
                and expected_group_label != FINAL_2026_GROUP_LABELS[expected_group]
            ):
                raise ManifestError(
                    f"{location}.result_contract.expected_group_label must be "
                    f"{FINAL_2026_GROUP_LABELS[expected_group]!r} for "
                    f"{expected_group!r}"
                )
            expected_arch = contract.get("expected_arch")
            if expected_arch not in FINAL_2026_ARCHITECTURES:
                raise ManifestError(
                    f"{location}.result_contract.expected_arch must be one of "
                    f"{sorted(FINAL_2026_ARCHITECTURES)}"
                )
            baseline_seconds = contract.get("buildstorm_baseline_seconds")
            baseline_is_finite = False
            if not isinstance(baseline_seconds, bool) and isinstance(
                baseline_seconds, (int, float)
            ):
                try:
                    baseline_is_finite = math.isfinite(baseline_seconds)
                except OverflowError:
                    baseline_is_finite = False
            if not baseline_is_finite or baseline_seconds <= 0:
                raise ManifestError(
                    f"{location}.result_contract.buildstorm baseline seconds "
                    "must be a finite positive number"
                )

        required_paths = _validate_string_list(case.get("required_paths", []), f"{location}.required_paths")
        for path_index, raw in enumerate(required_paths):
            _safe_repo_path(repo, raw, f"{location}.required_paths[{path_index}]")
        for command_index, raw in enumerate(command):
            if raw.startswith("{repo}/"):
                _safe_repo_path(repo, raw, f"{location}.command[{command_index}]")

        required_commands = _validate_string_list(
            case.get("required_commands", []),
            f"{location}.required_commands",
        )
        required_capabilities = _validate_string_list(
            case.get("required_capabilities", []),
            f"{location}.required_capabilities",
        )
        if len(required_capabilities) != len(set(required_capabilities)):
            raise ManifestError(f"{location}.required_capabilities contains duplicates")
        unknown_capabilities = sorted(set(required_capabilities) - set(CAPABILITY_PROBES))
        if unknown_capabilities:
            raise ManifestError(
                f"{location}.required_capabilities contains unsupported capability IDs: "
                f"{unknown_capabilities}"
            )
        for capability_id in required_capabilities:
            required_command = CAPABILITY_PROBES[capability_id]["required_command"]
            if required_command not in required_commands:
                raise ManifestError(
                    f"{location}.required_capabilities requires {required_command!r} "
                    "in required_commands"
                )
        infra_codes = case.get("infrastructure_exit_codes", [])
        _require_type(infra_codes, list, f"{location}.infrastructure_exit_codes")
        if any(isinstance(code, bool) or not isinstance(code, int) or code <= 0 for code in infra_codes):
            raise ManifestError(f"{location}.infrastructure_exit_codes must contain positive integers")

        environment = case.get("environment", {})
        _require_type(environment, dict, f"{location}.environment")
        for name, value in environment.items():
            if not isinstance(name, str) or not ENV_NAME_RE.fullmatch(name):
                raise ManifestError(f"{location}.environment has invalid variable name: {name!r}")
            if not isinstance(value, str):
                raise ManifestError(f"{location}.environment.{name} must be a string")
            _check_tokens(value, f"{location}.environment.{name}")

        required_files = case.get("required_files", [])
        _require_type(required_files, list, f"{location}.required_files")
        for file_index, requirement in enumerate(required_files):
            req_location = f"{location}.required_files[{file_index}]"
            _require_type(requirement, dict, req_location)
            _reject_unknown_keys(
                requirement,
                {"environment", "fallback", "directory_environment", "basename"},
                req_location,
            )
            env_name = requirement.get("environment")
            fallback = requirement.get("fallback")
            directory_env = requirement.get("directory_environment")
            basename = requirement.get("basename")
            if not isinstance(env_name, str) or not ENV_NAME_RE.fullmatch(env_name):
                raise ManifestError(f"{req_location}.environment is invalid")
            if fallback is not None and (not isinstance(fallback, str) or not fallback):
                raise ManifestError(f"{req_location}.fallback must be a non-empty string")
            if fallback is not None:
                _check_tokens(fallback, f"{req_location}.fallback")
            if directory_env is not None and (
                not isinstance(directory_env, str) or not ENV_NAME_RE.fullmatch(directory_env)
            ):
                raise ManifestError(f"{req_location}.directory_environment is invalid")
            if basename is not None and (not isinstance(basename, str) or not basename):
                raise ManifestError(f"{req_location}.basename must be a non-empty string")
            if basename is not None:
                _reject_embedded_nul(basename, f"{req_location}.basename")
            if directory_env is not None and basename is None:
                raise ManifestError(f"{req_location}.basename is required with directory_environment")

    for profile_name, profile in profiles.items():
        location = f"profiles.{profile_name}"
        if not isinstance(profile_name, str) or not PROFILE_NAME_RE.fullmatch(profile_name):
            raise ManifestError(f"invalid profile name: {profile_name!r}")
        _require_type(profile, dict, location)
        _reject_unknown_keys(
            profile,
            {"description", "arch_policy", "include", "cases", "arch_cases"},
            location,
        )
        description = profile.get("description", "")
        if not isinstance(description, str):
            raise ManifestError(f"{location}.description must be a string")
        policy = profile.get("arch_policy", "none")
        if policy not in ARCH_POLICIES:
            raise ManifestError(f"{location}.arch_policy is unsupported: {policy!r}")
        includes = _validate_string_list(profile.get("include", []), f"{location}.include")
        direct = _validate_string_list(profile.get("cases", []), f"{location}.cases")
        if len(includes) != len(set(includes)):
            raise ManifestError(f"{location}.include contains duplicates")
        if len(direct) != len(set(direct)):
            raise ManifestError(f"{location}.cases contains duplicates")
        arch_cases = profile.get("arch_cases", {})
        _require_type(arch_cases, dict, f"{location}.arch_cases")
        for include in includes:
            if include not in profiles:
                raise ManifestError(f"{location} includes unknown profile: {include}")
        for case_id in direct:
            if case_id not in case_ids:
                raise ManifestError(f"{location} references unknown case: {case_id}")
        for arch, arch_ids in arch_cases.items():
            if arch not in {"rv", "la"}:
                raise ManifestError(f"{location}.arch_cases has invalid architecture: {arch}")
            validated_arch_ids = _validate_string_list(arch_ids, f"{location}.arch_cases.{arch}")
            if len(validated_arch_ids) != len(set(validated_arch_ids)):
                raise ManifestError(f"{location}.arch_cases.{arch} contains duplicates")
            for case_id in validated_arch_ids:
                if case_id not in case_ids:
                    raise ManifestError(f"{location}.arch_cases.{arch} references unknown case: {case_id}")
        if policy == "none" and any(arch_cases.values()):
            raise ManifestError(f"{location} has arch cases but arch_policy is none")

    def visit(profile_name: str, stack: tuple[str, ...] = ()) -> None:
        if profile_name in stack:
            raise ManifestError("profile include cycle: " + " -> ".join((*stack, profile_name)))
        for included in profiles[profile_name].get("include", []):
            visit(included, (*stack, profile_name))

    for profile_name in profiles:
        visit(profile_name)
    referenced_case_ids = {
        case_id
        for profile in profiles.values()
        for case_id in [
            *profile.get("cases", []),
            *(case_id for values in profile.get("arch_cases", {}).values() for case_id in values),
        ]
    }
    orphaned = sorted(case_ids - referenced_case_ids)
    if orphaned:
        raise ManifestError(f"manifest cases are unreachable from every profile: {orphaned}")

    contract_by_id = {case["id"]: case["result_contract"]["type"] for case in cases}
    namespace_contracts = {
        "check.": "check",
        "unit.": "unittest",
        "official.": "official",
        "final.": "final_2026",
    }
    exact_case_contracts = {"baseline.workspace_unit_tests": "cargo_test"}
    for case_id, result_type in contract_by_id.items():
        for namespace, required_type in namespace_contracts.items():
            if case_id.startswith(namespace) and result_type != required_type:
                raise ManifestError(
                    f"case {case_id} must use the {required_type} result contract"
                )
    for case_id, required_type in exact_case_contracts.items():
        if case_id in contract_by_id and contract_by_id[case_id] != required_type:
            raise ManifestError(f"case {case_id} must use the {required_type} result contract")

    def profile_case_ids(profile_name: str) -> list[str]:
        profile = profiles[profile_name]
        selected = [
            *(
                case_id
                for included in profile.get("include", [])
                for case_id in profile_case_ids(included)
            ),
            *profile.get("cases", []),
            *(
                case_id
                for values in profile.get("arch_cases", {}).values()
                for case_id in values
            ),
        ]
        return selected

    profile_contracts = {
        "checks": "check",
        "unit": "unittest",
        "official": "official",
        "final-2026": "final_2026",
    }
    for profile_name, required_type in profile_contracts.items():
        if profile_name not in profiles:
            continue
        downgraded = sorted(
            case_id
            for case_id in profile_case_ids(profile_name)
            if contract_by_id[case_id] != required_type
        )
        if downgraded:
            raise ManifestError(
                f"profile {profile_name} requires the {required_type} result contract for: {downgraded}"
            )
    canonical_arch_cases = {
        "rv": ["official.riscv64"],
        "la": ["official.loongarch64"],
    }
    canonical_full_arch_cases = {
        architecture: [
            *canonical_arch_cases[architecture],
            *CANONICAL_FINAL_2026_ARCH_CASES[architecture],
        ]
        for architecture in ("rv", "la")
    }
    canonical_case_ids = {
        *CANONICAL_CHECK_CASE_IDS,
        *CANONICAL_UNIT_CASE_IDS,
        *CANONICAL_EVIDENCE_CASE_IDS,
        *CANONICAL_BASELINE_CASE_IDS,
        *(case_id for values in canonical_arch_cases.values() for case_id in values),
        *(
            case_id
            for values in CANONICAL_FINAL_2026_ARCH_CASES.values()
            for case_id in values
        ),
    }
    is_canonical_plan = canonical_manifest or bool(case_ids & canonical_case_ids)
    reserved_profile_collisions = sorted(CANONICAL_PROFILE_NAMES & set(profiles))
    if not is_canonical_plan and reserved_profile_collisions:
        raise ManifestError(
            "alternate manifests cannot define reserved canonical profile names: "
            f"{reserved_profile_collisions}"
        )
    if is_canonical_plan:
        if baseline_ref != "origin/main":
            raise ManifestError("canonical manifest baseline_ref must be origin/main")
        if set(profiles) != CANONICAL_PROFILE_NAMES:
            raise ManifestError(
                "canonical manifest profiles must be exactly: "
                f"{sorted(CANONICAL_PROFILE_NAMES)}"
            )
        if case_ids != canonical_case_ids:
            missing = sorted(canonical_case_ids - case_ids)
            extra = sorted(case_ids - canonical_case_ids)
            raise ManifestError(
                f"canonical manifest case inventory mismatch; missing={missing}, extra={extra}"
            )
        canonical_profile_shapes = {
            "checks": ("none", [], list(CANONICAL_CHECK_CASE_IDS), {}),
            "unit": ("none", [], list(CANONICAL_UNIT_CASE_IDS), {}),
            "quick": ("none", ["checks", "unit"], [], {}),
            "evidence-host": ("none", [], ["evidence.host"], {}),
            "evidence-runtime": (
                "one",
                [],
                [],
                {
                    "rv": ["evidence.riscv64"],
                    "la": ["evidence.loongarch64"],
                },
            ),
            "evidence-aggregate": ("none", [], ["evidence.aggregate"], {}),
            "evidence-required": (
                "none",
                ["evidence-host"],
                ["evidence.riscv64", "evidence.loongarch64", "evidence.aggregate"],
                {},
            ),
            "baseline": (
                "none",
                ["quick", "evidence-required"],
                list(CANONICAL_BASELINE_CASE_IDS),
                {},
            ),
            "official": ("one", [], [], canonical_arch_cases),
            "final-2026": ("one_or_all", [], [], CANONICAL_FINAL_2026_ARCH_CASES),
            "full": ("one_or_all", ["baseline"], [], canonical_full_arch_cases),
        }
        for profile_name, (policy, includes, direct_cases, arch_cases) in canonical_profile_shapes.items():
            profile = profiles[profile_name]
            actual_shape = (
                profile.get("arch_policy", "none"),
                profile.get("include", []),
                profile.get("cases", []),
                profile.get("arch_cases", {}),
            )
            expected_shape = (policy, includes, direct_cases, arch_cases)
            if actual_shape != expected_shape:
                raise ManifestError(
                    f"profile {profile_name} does not match the canonical case/include plan"
                )
        cases_by_id = {case["id"]: case for case in cases}
        for case_id in (*CANONICAL_CHECK_CASE_IDS, *CANONICAL_UNIT_CASE_IDS):
            namespace, semantic_name = case_id.split(".", 1)
            filename_prefix = "check" if namespace == "check" else "test"
            implementation_dir = "checks" if namespace == "check" else "unit"
            implementation = f"{{repo}}/test/{implementation_dir}/{filename_prefix}_{semantic_name}.py"
            unittest_harness = "{repo}/test/run_unittest_suite.py"
            case = cases_by_id[case_id]
            expected_command = [
                "{python}",
                "-I",
                "-S",
                "-B",
                "-X",
                "pycache_prefix=/dev/null",
                implementation,
            ]
            expected_paths = [implementation]
            if namespace == "unit":
                expected_command = [
                    "{python}",
                    "-I",
                    "-S",
                    "-B",
                    "-X",
                    "pycache_prefix=/dev/null",
                    unittest_harness,
                    implementation,
                ]
                expected_paths = [unittest_harness, implementation]
            expected_paths.extend(CANONICAL_PYTHON_EXTRA_REQUIRED_PATHS.get(case_id, []))
            if (
                case.get("command") != expected_command
                or case.get("cwd", "{repo}") != "{repo}"
                or case.get("required_paths", []) != expected_paths
                or case.get("environment", {}) != {}
            ):
                raise ManifestError(
                    f"case {case_id} must invoke its exact canonical Python implementation"
                )
            if namespace == "unit":
                implementation_path = repo / implementation.removeprefix("{repo}/")
                try:
                    tree = ast.parse(implementation_path.read_text(encoding="utf-8"))
                except (OSError, UnicodeDecodeError, SyntaxError) as error:
                    raise ManifestError(
                        f"cannot inventory canonical unittest implementation {implementation_path}: {error}"
                    ) from error
                observed_tests = canonical_unittest_method_count(tree, implementation_path)
                declared_tests = case["result_contract"].get("expected_tests")
                pinned_tests = CANONICAL_UNIT_EXPECTED_TESTS[case_id]
                identity_binding = case["result_contract"].get("identity_binding")
                if (
                    observed_tests != pinned_tests
                    or declared_tests != pinned_tests
                    or identity_binding is not True
                ):
                    raise ManifestError(
                        f"case {case_id} must preserve and identity-bind {pinned_tests} "
                        "canonical unittest methods; "
                        f"observed={observed_tests}, declared={declared_tests}, "
                        f"identity_binding={identity_binding!r}"
                    )
        for case_id, expected_command in CANONICAL_BASELINE_COMMANDS.items():
            case = cases_by_id[case_id]
            expected_capabilities = CANONICAL_BASELINE_REQUIRED_CAPABILITIES.get(
                case_id, []
            )
            if (
                case.get("command") != expected_command
                or case.get("cwd", "{repo}") != "{repo}"
                or case.get("environment", {}) != {}
                or case.get("required_capabilities", []) != expected_capabilities
                or case.get("result_contract")
                != CANONICAL_BASELINE_RESULT_CONTRACTS[case_id]
                or case.get("infrastructure_exit_codes", []) != []
                or case.get("required_commands", [])
                != CANONICAL_BASELINE_REQUIRED_COMMANDS[case_id]
                or case.get("required_paths", []) != []
                or case.get("required_files", []) != []
                or case.get("timeout_seconds") != CANONICAL_BASELINE_TIMEOUTS[case_id]
            ):
                raise ManifestError(
                    f"case {case_id} must preserve its exact canonical baseline command "
                    "and result/capability requirements"
                )
        for case_id, expected_command in CANONICAL_EVIDENCE_COMMANDS.items():
            case = cases_by_id[case_id]
            if (
                case.get("command") != expected_command
                or case.get("cwd", "{repo}") != "{repo}"
                or case.get("environment", {}) != {}
                or case.get("required_capabilities", []) != []
                or case.get("result_contract")
                != {"type": "exit_code", "allow_empty_output": False}
                or case.get("infrastructure_exit_codes", []) != [2]
                or case.get("required_commands", [])
                != CANONICAL_EVIDENCE_REQUIRED_COMMANDS[case_id]
                or case.get("required_paths", [])
                != CANONICAL_EVIDENCE_REQUIRED_PATHS[case_id]
                or case.get("required_files", []) != []
                or case.get("timeout_seconds") != CANONICAL_EVIDENCE_TIMEOUTS[case_id]
            ):
                raise ManifestError(
                    f"case {case_id} must preserve its exact canonical semantic-evidence "
                    "adapter and result/capability requirements"
                )
        live_ltp_count = len(_live_ltp_stable_cases(repo))
        try:
            trusted_official_case_plan(repo)
        except (OSError, ValueError) as error:
            raise ManifestError(f"cannot load trusted official identity plan: {error}") from error
        expected_ltp_counts = {
            CANONICAL_OFFICIAL_CASE_COUNTS["ltp-musl"],
            CANONICAL_OFFICIAL_CASE_COUNTS["ltp-glibc"],
        }
        if expected_ltp_counts != {live_ltp_count}:
            raise ManifestError(
                "canonical official LTP case counts do not match the trusted "
                f"LTP_STABLE_CASES source count {live_ltp_count}"
            )
        canonical_official_ids = set().union(*canonical_arch_cases.values())
        actual_official_ids = {
            case_id for case_id, result_type in contract_by_id.items() if result_type == "official"
        }
        if actual_official_ids != canonical_official_ids:
            raise ManifestError(
                "manifest official cases must be exactly the canonical RV/LA cases: "
                f"{sorted(canonical_official_ids)}"
            )
        for architecture, case_ids in canonical_arch_cases.items():
            case_id = case_ids[0]
            case = cases_by_id[case_id]
            contract = case["result_contract"]
            if contract.get("expected_group_labels") != list(CANONICAL_OFFICIAL_GROUPS):
                raise ManifestError(
                    f"case {case_id} must use the canonical ordered 24-group official plan"
                )
            if contract.get("expected_group_case_counts") != CANONICAL_OFFICIAL_CASE_COUNTS:
                raise ManifestError(
                    f"case {case_id} must use the canonical LTP/BusyBox/libctest case-count plan"
                )
            environment = case.get("environment", {})
            if environment != CANONICAL_OFFICIAL_ENVIRONMENT:
                raise ManifestError(
                    f"case {case_id} must use the exact canonical official environment"
                )
            expected_command = [
                "{repo}/test/evaluation/run_official_evaluation.sh",
                architecture,
            ]
            if case.get("command") != expected_command or case.get("cwd", "{repo}") != "{repo}":
                raise ManifestError(
                    f"case {case_id} must invoke the canonical official wrapper for {architecture}"
                )
            execution = CANONICAL_OFFICIAL_EXECUTION[architecture]
            for field, expected_value in execution.items():
                if case.get(field, []) != expected_value:
                    raise ManifestError(
                        f"case {case_id} must preserve canonical official {field}={expected_value!r}"
                    )
        actual_final_ids = {
            case_id
            for case_id, result_type in contract_by_id.items()
            if result_type == "final_2026"
        }
        if actual_final_ids != set(CANONICAL_FINAL_2026_EXECUTION):
            raise ManifestError(
                "manifest final-2026 cases must be exactly the canonical four cases"
            )
        for case_id, execution in CANONICAL_FINAL_2026_EXECUTION.items():
            case = cases_by_id[case_id]
            architecture = execution["architecture"]
            group = execution["group"]
            expected_contract = {
                "type": "final_2026",
                "expected_group": group,
                "expected_group_label": FINAL_2026_GROUP_LABELS[group],
                "expected_arch": architecture,
                "buildstorm_baseline_seconds": 400.0,
            }
            expected_command = [
                "{python}",
                "-I",
                "-S",
                "-B",
                "-X",
                "pycache_prefix=/dev/null",
                "{repo}/test/evaluation/run_final_2026_evaluation.py",
                architecture,
                group,
            ]
            expected_paths = [
                "{repo}/test/evaluation/run_final_2026_evaluation.py",
                "{repo}/test/evaluation/parse_final_2026_results.py",
                "{repo}/test/images/manifest.final-2026.json",
            ]
            expected_commands = [
                "python3",
                "git",
                "make",
                "cargo",
                "qemu-img",
                "debugfs",
                execution["qemu"],
            ]
            expected_files = [{"environment": execution["image_environment"]}]
            if (
                case.get("command") != expected_command
                or case.get("cwd", "{repo}") != "{repo}"
                or case.get("timeout_seconds") != execution["timeout_seconds"]
                or case.get("result_contract") != expected_contract
                or case.get("required_paths", []) != expected_paths
                or case.get("required_commands", []) != expected_commands
                or case.get("required_files", []) != expected_files
                or case.get("environment", {}) != CANONICAL_FINAL_2026_ENVIRONMENT
                or case.get("infrastructure_exit_codes", []) != [125]
            ):
                raise ManifestError(
                    f"case {case_id} must preserve canonical final-2026 execution fields"
                )
    return manifest


def select_cases(manifest: dict[str, Any], profile_name: str, arch: str | None) -> Selection:
    profiles = manifest["profiles"]
    if profile_name not in profiles:
        raise ManifestError(f"unknown profile: {profile_name}")
    profile = profiles[profile_name]
    policy = profile.get("arch_policy", "none")
    if policy == "none":
        if arch is not None:
            raise ManifestError(f"profile {profile_name} does not accept --arch")
        selected_arch = None
    elif policy == "one":
        if arch not in {"rv", "la"}:
            raise ManifestError(f"profile {profile_name} requires --arch rv or --arch la")
        selected_arch = arch
    else:
        if arch is None:
            arch = "all"
        if arch not in {"rv", "la", "all"}:
            raise ManifestError(f"profile {profile_name} accepts --arch rv, la, or all")
        selected_arch = arch

    ordered_ids: list[str] = []
    ordered_architectures: list[str | None] = []
    seen: set[str] = set()

    def add(case_id: str, case_architecture: str | None) -> None:
        if case_id in seen:
            raise ManifestError(
                f"profile {profile_name} selects case more than once through its include graph: {case_id}"
            )
        seen.add(case_id)
        ordered_ids.append(case_id)
        ordered_architectures.append(case_architecture)

    def resolve(name: str) -> None:
        item = profiles[name]
        for included in item.get("include", []):
            resolve(included)
        item_policy = item.get("arch_policy", "none")
        direct_architecture = (
            selected_arch
            if item_policy in {"one", "one_or_all"} and selected_arch in {"rv", "la"}
            else None
        )
        for case_id in item.get("cases", []):
            add(case_id, direct_architecture)
        requested_arches = [] if selected_arch is None else ["rv", "la"] if selected_arch == "all" else [selected_arch]
        for requested_arch in requested_arches:
            for case_id in item.get("arch_cases", {}).get(requested_arch, []):
                add(case_id, requested_arch)

    resolve(profile_name)
    by_id = {case["id"]: case for case in manifest["cases"]}
    selected = [by_id[case_id] for case_id in ordered_ids]
    if not selected:
        raise ManifestError(f"profile {profile_name} selected zero cases")
    for case, case_architecture in zip(selected, ordered_architectures):
        token_values = [
            *case["command"],
            case.get("cwd", "{repo}"),
            *case.get("environment", {}).values(),
        ]
        if any("{arch}" in value for value in token_values) and case_architecture is None:
            raise ManifestError(
                f"case {case['id']} uses {{arch}} but profile {profile_name} does not resolve one architecture"
            )
    return Selection(profile_name, selected_arch, selected, ordered_architectures)


def validate_all_profile_selections(manifest: dict[str, Any]) -> None:
    for profile_name, profile in manifest["profiles"].items():
        policy = profile.get("arch_policy", "none")
        architectures: tuple[str | None, ...]
        if policy == "none":
            architectures = (None,)
        elif policy == "one":
            architectures = ("rv", "la")
        else:
            architectures = ("rv", "la", "all")
        for architecture in architectures:
            select_cases(manifest, profile_name, architecture)


def expand_value(value: str, *, repo: Path, output_dir: Path, case_output_dir: Path, arch: str | None) -> str:
    replacements = {
        "{repo}": str(repo),
        "{python}": sys.executable,
        "{output_dir}": str(output_dir),
        "{case_output_dir}": str(case_output_dir),
        "{arch}": arch or "",
    }
    for token, replacement in replacements.items():
        value = value.replace(token, replacement)
    return value


def git_probe_environment(repo: Path) -> dict[str, str]:
    return _closed_git_environment(repo)


def trusted_git_probe(
    repo: Path,
    arguments: list[str],
) -> subprocess.CompletedProcess[str] | None:
    probe_environment = git_probe_environment(repo)
    if validate_child_path_environment(probe_environment) is not None:
        return None
    git_command = shutil.which("git", path=probe_environment["PATH"])
    if git_command is None:
        return None
    try:
        return subprocess.run(
            [str(Path(git_command).resolve()), *arguments],
            cwd=repo,
            env=probe_environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=10,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        return None


def baseline_commit(repo: Path, ref: str) -> str:
    result = trusted_git_probe(
        repo,
        ["rev-parse", "--verify", f"{ref}^{{commit}}"],
    )
    if result is None:
        return "unknown"
    return result.stdout.strip() if result.returncode == 0 else "unknown"


def git_worktree_status(repo: Path) -> str | None:
    result = trusted_git_probe(
        repo,
        ["status", "--porcelain=v1", "--untracked-files=all"],
    )
    if result is None or result.returncode != 0:
        return None
    return result.stdout


def _write_report(path: Path, report: dict[str, Any]) -> None:
    temporary = path.with_suffix(".json.tmp")
    temporary.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temporary.replace(path)


def _preflight(
    case: dict[str, Any],
    *,
    repo: Path,
    environment: dict[str, str],
) -> tuple[str | None, dict[str, str]]:
    resolved_commands: dict[str, str] = {}
    for command in case.get("required_commands", []):
        resolved = shutil.which(command, path=environment.get("PATH"))
        if resolved is None:
            return f"required command not found: {command}", resolved_commands
        resolved_commands[command] = str(Path(resolved).resolve())
    for requirement in case.get("required_files", []):
        env_name = requirement["environment"]
        value = environment.get(env_name)
        if not value and requirement.get("directory_environment"):
            directory = environment.get(requirement["directory_environment"])
            if directory:
                value = str(Path(directory) / requirement["basename"])
        if not value and requirement.get("fallback"):
            value = requirement["fallback"].replace("{repo}", str(repo))
            if not Path(value).is_absolute():
                value = str((repo / value).resolve())
        if not value:
            return f"required file variable {env_name} has no value or fallback", resolved_commands
        path = Path(value).expanduser().resolve()
        if not path.is_file():
            return f"required file for {env_name} not found: {path}", resolved_commands
        if not os.access(path, os.R_OK):
            return f"required file for {env_name} is not readable: {path}", resolved_commands
        environment[env_name] = str(path)
    return None, resolved_commands


def _command_preflight(argv: list[str], cwd: Path, environment: dict[str, str]) -> str | None:
    executable = argv[0]
    if os.path.sep in executable:
        path = Path(executable)
        if not path.is_absolute():
            path = cwd / path
        path = path.resolve()
        if not path.is_file():
            return f"command path does not exist: {path}"
        if not os.access(path, os.X_OK):
            return f"command path is not executable: {path}"
    elif shutil.which(executable, path=environment.get("PATH")) is None:
        return f"command not found: {executable}"
    return None


def _enable_child_subreaper() -> str | None:
    global _SUBREAPER_ENABLED
    if _SUBREAPER_ENABLED:
        return None
    if not sys.platform.startswith("linux"):
        return "reliable descendant containment requires Linux subreaper support"
    try:
        libc = ctypes.CDLL(None, use_errno=True)
        if libc.prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) != 0:
            error_number = ctypes.get_errno()
            return f"cannot enable child subreaper: {os.strerror(error_number)}"
    except (AttributeError, OSError) as error:
        return f"cannot enable child subreaper: {error}"
    _SUBREAPER_ENABLED = True
    return None


def _proc_snapshot() -> dict[int, tuple[int, int]]:
    snapshot: dict[int, tuple[int, int]] = {}
    try:
        entries = list(Path("/proc").iterdir())
    except OSError as error:
        raise ProcessSnapshotError(f"cannot enumerate /proc: {error}") from error
    for entry in entries:
        if not entry.name.isdigit():
            continue
        try:
            raw = (entry / "stat").read_text(encoding="utf-8")
            close = raw.rfind(")")
            if close < 0:
                raise ValueError("missing process-name terminator")
            fields = raw[close + 2 :].split()
            snapshot[int(entry.name)] = (int(fields[1]), int(fields[2]))
        except FileNotFoundError:
            # Processes may exit between enumerating /proc and reading stat.
            continue
        except (OSError, UnicodeDecodeError, ValueError, IndexError) as error:
            raise ProcessSnapshotError(
                f"cannot read a complete process snapshot from {entry / 'stat'}: {error}"
            ) from error
    return snapshot


def _validated_proc_snapshot() -> dict[int, tuple[int, int]]:
    snapshot = _proc_snapshot()
    own = snapshot.get(os.getpid())
    expected = (os.getppid(), os.getpgrp())
    if own is None:
        raise ProcessSnapshotError(
            "process snapshot does not contain the test runner itself"
        )
    if own != expected:
        raise ProcessSnapshotError(
            "process snapshot does not accurately describe the test runner: "
            f"observed={own}, expected={expected}"
        )
    return snapshot


def _direct_children(parent_pid: int, snapshot: dict[int, tuple[int, int]]) -> set[int]:
    return {pid for pid, (ppid, _pgrp) in snapshot.items() if ppid == parent_pid}


def _descendants(parent_pid: int, snapshot: dict[int, tuple[int, int]]) -> set[int]:
    descendants: set[int] = set()
    frontier = [parent_pid]
    while frontier:
        parent = frontier.pop()
        children = _direct_children(parent, snapshot) - descendants
        descendants.update(children)
        frontier.extend(children)
    return descendants


def _case_related_pids(
    leader_pid: int,
    preexisting_runner_children: set[int],
    tracked: set[int] | None = None,
) -> set[int]:
    snapshot = _validated_proc_snapshot()
    related = set(tracked or ()) & set(snapshot)
    related.update(_descendants(leader_pid, snapshot))
    related.update(
        pid
        for pid, (_ppid, pgrp) in snapshot.items()
        if pgrp == leader_pid and pid != leader_pid
    )
    related.update(
        _direct_children(os.getpid(), snapshot)
        - preexisting_runner_children
        - {leader_pid}
    )
    return related


def _signal_processes(pids: set[int], signum: int) -> None:
    for pid in sorted(pids):
        if pid == os.getpid():
            continue
        try:
            os.kill(pid, signum)
        except ProcessLookupError:
            pass


@dataclass
class TerminationOutcome:
    related_pids: set[int]
    tracking_error: str | None


def _terminate_case_processes(
    process: subprocess.Popen[bytes],
    preexisting_runner_children: set[int],
) -> TerminationOutcome:
    tracking_error: str | None = None

    def related_processes(tracked: set[int] | None = None) -> set[int]:
        nonlocal tracking_error
        if tracking_error is not None:
            return set(tracked or ())
        try:
            return _case_related_pids(
                process.pid,
                preexisting_runner_children,
                tracked,
            )
        except ProcessSnapshotError as error:
            tracking_error = str(error)
            return set(tracked or ())

    related = related_processes()
    try:
        os.killpg(process.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    _signal_processes(related, signal.SIGTERM)
    deadline = time.monotonic() + TERMINATION_GRACE_SECONDS
    while time.monotonic() < deadline:
        related.update(related_processes(related))
        if process.poll() is not None and not related_processes(related):
            break
        time.sleep(0.02)
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    related.update(related_processes(related))
    _signal_processes(related, signal.SIGKILL)
    if process.poll() is None:
        process.wait()
    kill_deadline = time.monotonic() + TERMINATION_GRACE_SECONDS
    while time.monotonic() < kill_deadline:
        alive = related_processes(related)
        if not alive:
            break
        _signal_processes(alive, signal.SIGKILL)
        related.update(alive)
        time.sleep(0.02)
    for pid in related:
        try:
            os.waitpid(pid, os.WNOHANG)
        except (ChildProcessError, ProcessLookupError):
            pass
    return TerminationOutcome(related_pids=related, tracking_error=tracking_error)


def _read_logs(stdout_path: Path, stderr_path: Path) -> tuple[str, str]:
    decoded: list[str] = []
    for stream_name, path in (("stdout", stdout_path), ("stderr", stderr_path)):
        raw = path.read_bytes()
        try:
            text = raw.decode("utf-8", errors="strict")
        except UnicodeDecodeError as error:
            raise OutputIntegrityError(
                f"{stream_name} is not valid UTF-8 at byte offset {error.start}"
            ) from error
        text = normalize_output_text(text)
        if "\r" in text:
            raise OutputIntegrityError(
                f"{stream_name} contains unsupported bare carriage return U+000D"
            )
        if invalid_character := first_unsupported_output_character(text):
            raise OutputIntegrityError(
                f"{stream_name} contains unsupported output character "
                f"U+{ord(invalid_character):04X}"
            )
        decoded.append(text)
    return decoded[0], decoded[1]


def _run_required_capability_probes(
    case: dict[str, Any],
    *,
    cwd: Path,
    environment: dict[str, str],
    resolved_commands: dict[str, str],
    case_output_dir: Path,
) -> tuple[str | None, list[dict[str, Any]]]:
    """Run fixed, read-only capability probes before the primary case command."""

    probe_records: list[dict[str, Any]] = []
    for capability_id in case.get("required_capabilities", []):
        definition = CAPABILITY_PROBES[capability_id]
        required_command = definition["required_command"]
        argv = [resolved_commands[required_command], *definition["arguments"]]
        safe_id = re.sub(r"[^a-zA-Z0-9_.-]", "_", capability_id)
        stdout_path = case_output_dir / f"capability-{safe_id}.stdout.log"
        stderr_path = case_output_dir / f"capability-{safe_id}.stderr.log"
        stdout_path.touch()
        stderr_path.touch()
        started = time.monotonic()
        probe_record: dict[str, Any] = {
            "id": capability_id,
            "command": argv,
            "timeout_seconds": CAPABILITY_PROBE_TIMEOUT_SECONDS,
            "executed": False,
            "status": "INFRA_ERROR",
            "result": "capability probe not launched",
            "return_code": None,
            "signal": None,
            "stdout_log": str(stdout_path),
            "stderr_log": str(stderr_path),
            "duration_seconds": 0.0,
        }
        probe_records.append(probe_record)
        try:
            preexisting_runner_children = _direct_children(
                os.getpid(), _validated_proc_snapshot()
            )
        except ProcessSnapshotError as error:
            probe_record["result"] = f"process tracking is unavailable: {error}"
            probe_record["duration_seconds"] = round(time.monotonic() - started, 6)
            return (
                f"required capability {capability_id} cannot be probed safely: {error}",
                probe_records,
            )
        process: subprocess.Popen[bytes] | None = None
        timed_out = False
        interrupted_by: int | None = None
        surviving_descendants: set[int] = set()
        process_tracking_error: str | None = None
        try:
            with stdout_path.open("wb") as stdout_file, stderr_path.open("wb") as stderr_file:
                process = subprocess.Popen(
                    argv,
                    cwd=cwd,
                    env=environment,
                    stdin=subprocess.DEVNULL,
                    stdout=stdout_file,
                    stderr=stderr_file,
                    start_new_session=True,
                )
                probe_record["executed"] = True
                previous_sigterm = signal.getsignal(signal.SIGTERM)

                def terminate_runner(signum: int, _frame: Any) -> None:
                    raise RunnerTermination(signum)

                signal.signal(signal.SIGTERM, terminate_runner)
                try:
                    process.wait(timeout=CAPABILITY_PROBE_TIMEOUT_SECONDS)
                except subprocess.TimeoutExpired:
                    timed_out = True
                    termination = _terminate_case_processes(
                        process, preexisting_runner_children
                    )
                    process_tracking_error = termination.tracking_error
                except KeyboardInterrupt:
                    interrupted_by = signal.SIGINT
                    termination = _terminate_case_processes(
                        process, preexisting_runner_children
                    )
                    process_tracking_error = termination.tracking_error
                except RunnerTermination as interruption:
                    interrupted_by = interruption.signum
                    termination = _terminate_case_processes(
                        process, preexisting_runner_children
                    )
                    process_tracking_error = termination.tracking_error
                finally:
                    signal.signal(signal.SIGTERM, previous_sigterm)
        except (OSError, ValueError) as error:
            probe_record["result"] = f"could not launch capability probe: {error}"
            probe_record["duration_seconds"] = round(time.monotonic() - started, 6)
            return (
                f"required capability {capability_id} could not be probed: {error}",
                probe_records,
            )

        if process is None:
            probe_record["result"] = "capability probe launch did not produce a process"
            probe_record["duration_seconds"] = round(time.monotonic() - started, 6)
            return (
                f"required capability {capability_id} probe did not launch",
                probe_records,
            )
        probe_record["return_code"] = process.returncode
        if process.returncode is not None and process.returncode < 0:
            probe_record["signal"] = -process.returncode
        if not timed_out and interrupted_by is None:
            try:
                surviving_descendants = _case_related_pids(
                    process.pid, preexisting_runner_children
                )
            except ProcessSnapshotError as error:
                process_tracking_error = str(error)
            else:
                if surviving_descendants:
                    termination = _terminate_case_processes(
                        process, preexisting_runner_children
                    )
                    process_tracking_error = termination.tracking_error
        probe_record["duration_seconds"] = round(time.monotonic() - started, 6)
        if process_tracking_error is not None:
            probe_record["result"] = (
                "capability probe process containment could not be verified: "
                f"{process_tracking_error}"
            )
            probe_record["process_tracking_error"] = process_tracking_error
            return (
                f"required capability {capability_id} probe process containment "
                f"could not be verified: {process_tracking_error}",
                probe_records,
            )
        if timed_out:
            probe_record["status"] = "TIMEOUT"
            probe_record["result"] = "capability probe timed out and was terminated"
            return (
                f"required capability {capability_id} probe timed out",
                probe_records,
            )
        if interrupted_by is not None:
            probe_record["status"] = "CRASH"
            probe_record["signal"] = interrupted_by
            probe_record["result"] = "capability probe was interrupted and terminated"
            return (
                f"required capability {capability_id} probe was interrupted by signal "
                f"{interrupted_by}",
                probe_records,
            )
        if surviving_descendants:
            probe_record["result"] = (
                "capability probe exited while descendant processes remained; "
                "all observed descendants were terminated"
            )
            probe_record["surviving_descendant_count"] = len(surviving_descendants)
            return (
                f"required capability {capability_id} probe left surviving descendants",
                probe_records,
            )
        try:
            _read_logs(stdout_path, stderr_path)
        except (OSError, OutputIntegrityError) as error:
            probe_record["result"] = f"capability probe output is malformed: {error}"
            return (
                f"required capability {capability_id} probe output is malformed: {error}",
                probe_records,
            )
        if process.returncode != 0:
            if process.returncode is not None and process.returncode < 0:
                probe_record["status"] = "CRASH"
                probe_record["result"] = (
                    f"capability probe terminated by signal {-process.returncode}"
                )
            else:
                probe_record["result"] = (
                    f"capability probe exited with status {process.returncode}"
                )
            return (
                f"required capability {capability_id} is unavailable: "
                f"probe exit status {process.returncode}",
                probe_records,
            )
        probe_record["status"] = "PASS"
        probe_record["result"] = "capability probe exited zero"
    return None, probe_records


def parse_contract(
    case: dict[str, Any], stdout: str, stderr: str
) -> tuple[str, str, dict[str, Any]]:
    contract = case["result_contract"]
    result_type = contract["type"]
    combined = stdout + ("\n" if stdout and stderr else "") + stderr
    if result_type not in STRUCTURED_RESULT_TYPES and UNKNOWN_STATE_OUTPUT_RE.search(
        combined
    ):
        return "INFRA_ERROR", "output contains an unknown or unexecuted status", {}
    if result_type not in STRUCTURED_RESULT_TYPES and ZERO_EXECUTION_OUTPUT_RE.search(
        combined
    ):
        return "INFRA_ERROR", "output explicitly reports zero executed tests", {}
    if result_type == "exit_code":
        if not combined.strip() and not contract.get("allow_empty_output", False):
            return "INFRA_ERROR", "zero-exit command emitted no completion evidence", {}
        if UNKNOWN_STATUS_RECORD_RE.search(combined):
            return "INFRA_ERROR", "zero-exit command emitted an unsupported status record", {}
        non_pass_evidence = exit_code_non_pass_evidence(combined)
        if non_pass_evidence is not None:
            return (
                "FAIL",
                "zero-exit command output contains explicit non-pass evidence",
                {"non_pass_evidence": non_pass_evidence},
            )
        return "PASS", "child exited zero", {}
    if result_type == "check":
        if UNKNOWN_STATUS_RECORD_RE.search(combined):
            return "INFRA_ERROR", "check output contains an unsupported status record", {}
        if (
            NON_PASS_OUTPUT_RE.search(combined)
            or EXPLICIT_FAILURE_OUTPUT_RE.search(combined)
            or CRASH_OUTPUT_RE.search(combined)
            or POSITIVE_FINDINGS_RE.search(combined)
        ):
            return "FAIL", "check output contains an explicit non-pass marker", {}
        lines = stdout.splitlines()
        pass_lines: list[tuple[int, str]] = []
        for index, line in enumerate(lines):
            direct = CHECK_DIRECT_PASS_RE.fullmatch(line)
            named = CHECK_NAMED_PASS_RE.fullmatch(line)
            if direct or (
                named is not None
                and CHECK_LABEL_NON_PASS_RE.search(named.group("label")) is None
            ):
                pass_lines.append((index, line))
        if len(pass_lines) != 1:
            return (
                "INFRA_ERROR",
                f"zero-exit check must emit exactly one explicit PASS status line; found {len(pass_lines)}",
                {"pass_record_count": len(pass_lines)},
            )
        terminal_index = max(
            (index for index, line in enumerate(lines) if line.strip()),
            default=-1,
        )
        if pass_lines[0][0] != terminal_index:
            return (
                "INFRA_ERROR",
                "the explicit check PASS record must be the terminal non-empty output line",
                {"pass_record_count": 1},
            )
        if stderr.strip():
            return (
                "INFRA_ERROR",
                "a passing check must keep stderr empty so completion order is unambiguous",
                {"pass_record_count": 1},
            )
        return "PASS", "one explicit PASS status line found", {"pass_record_count": 1}
    if result_type == "unittest":
        lines = stderr.splitlines()
        identity_binding_required = contract.get("identity_binding", False)
        if UNKNOWN_STATUS_RECORD_RE.search(combined):
            return "INFRA_ERROR", "unittest output contains an unsupported status record", {}
        if UNITTEST_UNEXECUTED_COROUTINE_RE.search(combined):
            return (
                "INFRA_ERROR",
                "unittest output shows a coroutine test that was not executed",
                {},
            )
        if NON_PASS_OUTPUT_RE.search(combined) or CRASH_OUTPUT_RE.search(combined):
            return "FAIL", "unittest output contains failure, skip, or unsupported status", {}
        malformed_protocol_lines = [
            line
            for line in lines
            if (
                UNITTEST_SUMMARY_SIGNATURE_RE.search(line)
                and UNITTEST_COUNT_RE.fullmatch(line) is None
            )
            or (
                UNITTEST_OK_SIGNATURE_RE.fullmatch(line) is not None
                and line != "OK"
            )
            or (
                UNITTEST_BINDING_SIGNATURE_RE.search(line)
                and UNITTEST_BINDING_RE.fullmatch(line) is None
            )
        ]
        if malformed_protocol_lines:
            return (
                "INFRA_ERROR",
                "unittest output contains malformed summary or status records",
                {"malformed_records": malformed_protocol_lines},
            )
        matches = [
            (index, match)
            for index, line in enumerate(lines)
            if (match := UNITTEST_COUNT_RE.fullmatch(line))
        ]
        if len(matches) != 1:
            return "INFRA_ERROR", f"expected exactly one unittest count, found {len(matches)}", {}
        summary_index, summary_match = matches[0]
        observed = int(summary_match.group(1))
        expected = contract["expected_tests"]
        if observed <= 0:
            return "INFRA_ERROR", "unittest executed zero tests", {"observed_tests": observed}
        if observed != expected:
            return (
                "INFRA_ERROR",
                f"unittest count mismatch: expected {expected}, observed {observed}",
                {"expected_tests": expected, "observed_tests": observed},
            )
        binding_matches = [
            (index, match)
            for index, line in enumerate(lines)
            if (match := UNITTEST_BINDING_RE.fullmatch(line))
        ]
        binding_details: dict[str, int] = {}
        if identity_binding_required:
            if len(binding_matches) != 1:
                return (
                    "INFRA_ERROR",
                    "identity-bound unittest must emit exactly one binding record; "
                    f"found {len(binding_matches)}",
                    {"observed_tests": observed},
                )
            binding_index, binding_match = binding_matches[0]
            binding_values = tuple(int(value) for value in binding_match.groups())
            if binding_index >= summary_index:
                return (
                    "INFRA_ERROR",
                    "unittest identity binding must precede the terminal result summary",
                    {"observed_tests": observed},
                )
            if any(value != expected for value in binding_values):
                return (
                    "INFRA_ERROR",
                    "unittest identity binding count mismatch: "
                    f"expected {expected}, observed {binding_values}",
                    {"expected_tests": expected, "observed_binding": binding_values},
                )
            binding_details = {
                "planned_tests": binding_values[0],
                "started_tests": binding_values[1],
                "executed_tests": binding_values[2],
                "stopped_tests": binding_values[3],
            }
        elif binding_matches:
            return (
                "INFRA_ERROR",
                "unittest emitted an identity binding record without requiring that contract",
                {"observed_tests": observed},
            )
        if (
            EXPLICIT_FAILURE_OUTPUT_RE.search(combined)
            or "FAILED (" in combined
            or re.search(r"^FAILED\b", combined, re.M)
        ):
            return "FAIL", "unittest output contains failure, skip, or unsupported status", {"observed_tests": observed}
        ok_indices = [index for index, line in enumerate(lines) if line == "OK"]
        if len(ok_indices) != 1:
            return "INFRA_ERROR", f"expected one terminal unittest OK, found {len(ok_indices)}", {"observed_tests": observed}
        terminal_index = max(
            (index for index, line in enumerate(lines) if line.strip()),
            default=-1,
        )
        if summary_index >= ok_indices[0] or ok_indices[0] != terminal_index:
            return (
                "INFRA_ERROR",
                "unittest summary must precede one terminal plain OK line",
                {"observed_tests": observed},
            )
        if stdout.strip():
            return (
                "INFRA_ERROR",
                "a passing unittest suite must keep stdout empty so completion order is unambiguous",
                {"observed_tests": observed},
            )
        return (
            "PASS",
            f"unittest completed exactly {observed} tests with exact identity binding"
            if identity_binding_required
            else f"unittest completed exactly {observed} tests",
            {"observed_tests": observed, **binding_details},
        )
    if result_type == "cargo_test":
        stdout_lines = stdout.splitlines()
        lines = stdout_lines
        classified_stderr, accounted_identity_unittests = (
            cargo_test_stderr_after_identity_unittests(stderr)
        )
        classified_stderr, accounted_expected_panics = (
            cargo_test_stderr_after_expected_panics(stdout_lines, classified_stderr)
        )
        stderr_protocol_records = [
            line
            for line in stderr.splitlines()
            if CARGO_TEST_SIGNATURE_RE.search(line)
            or CARGO_TEST_CASE_SIGNATURE_RE.search(line)
        ]
        if stderr_protocol_records:
            return (
                "INFRA_ERROR",
                "cargo test lifecycle records must be emitted on stdout",
                {"stderr_protocol_records": stderr_protocol_records},
            )
        malformed_records = [
            line
            for line in lines
            if (
                CARGO_TEST_SIGNATURE_RE.search(line)
                and CARGO_TEST_RUNNING_RE.fullmatch(line) is None
                and CARGO_TEST_RESULT_RE.fullmatch(line) is None
            )
            or (
                CARGO_TEST_CASE_SIGNATURE_RE.search(line)
                and CARGO_TEST_CASE_RE.fullmatch(line) is None
            )
        ]
        if malformed_records:
            return (
                "INFRA_ERROR",
                "cargo test output contains malformed lifecycle records",
                {"malformed_records": malformed_records},
            )
        stdout_non_protocol_text = "\n".join(
            line
            for line in lines
            if CARGO_TEST_RUNNING_RE.fullmatch(line) is None
            and CARGO_TEST_RESULT_RE.fullmatch(line) is None
            and CARGO_TEST_CASE_RE.fullmatch(line) is None
        )
        non_protocol_text = stdout_non_protocol_text + (
            ("\n" if stdout_non_protocol_text and classified_stderr else "")
            + classified_stderr
        )
        if UNKNOWN_STATUS_RECORD_RE.search(non_protocol_text):
            return "INFRA_ERROR", "cargo test output contains an unsupported status record", {}
        failure_scan_text = CARGO_TEST_BENIGN_OPTION_RE.sub("", non_protocol_text)
        if (
            NON_PASS_OUTPUT_RE.search(failure_scan_text)
            or EXPLICIT_FAILURE_OUTPUT_RE.search(failure_scan_text)
            or CRASH_OUTPUT_RE.search(failure_scan_text)
        ):
            return "FAIL", "cargo test output contains explicit non-pass evidence", {}
        aggregate_planned = 0
        aggregate_executed = 0
        block_count = 0
        active_planned: int | None = None
        active_statuses: list[str] = []
        active_names: set[str] = set()
        for line in lines:
            if match := CARGO_TEST_RUNNING_RE.fullmatch(line):
                if active_planned is not None:
                    return "INFRA_ERROR", "cargo test lifecycle records are out of order", {}
                active_planned = int(match.group(1))
                active_statuses = []
                active_names = set()
                continue
            if match := CARGO_TEST_CASE_RE.fullmatch(line):
                if active_planned is None:
                    return "INFRA_ERROR", "cargo test case record appears outside a test block", {}
                case_name = match.group("name")
                if case_name in active_names:
                    return (
                        "INFRA_ERROR",
                        f"cargo test block repeats case identity: {case_name}",
                        {"block": block_count + 1, "duplicate_case": case_name},
                    )
                active_names.add(case_name)
                active_statuses.append(match.group("status"))
                continue
            match = CARGO_TEST_RESULT_RE.fullmatch(line)
            if match is None:
                continue
            if active_planned is None:
                return "INFRA_ERROR", "cargo test result appears without a running record", {}
            status, passed_raw, failed_raw, ignored_raw, measured_raw, filtered_raw = match.groups()
            passed, failed, ignored, measured, filtered = map(
                int,
                (passed_raw, failed_raw, ignored_raw, measured_raw, filtered_raw),
            )
            executed = passed + failed + ignored + measured
            if executed != active_planned:
                return (
                    "INFRA_ERROR",
                    f"cargo test block planned {active_planned} but accounted for {executed}",
                    {"block": block_count + 1},
                )
            if len(active_statuses) != active_planned:
                return (
                    "INFRA_ERROR",
                    f"cargo test block planned {active_planned} individual tests but emitted "
                    f"{len(active_statuses)} terminal case records",
                    {"block": block_count + 1},
                )
            if any(case_status != "ok" for case_status in active_statuses):
                return "FAIL", "cargo test output reports a failed or ignored individual test", {
                    "block": block_count + 1,
                    "individual_statuses": active_statuses,
                }
            aggregate_planned += active_planned
            aggregate_executed += executed
            block_count += 1
            if status != "ok" or failed:
                return "FAIL", "cargo test output reports failed tests", {"failed_tests": failed}
            if ignored or filtered or measured:
                return (
                    "FAIL",
                    "cargo test output reports ignored, filtered, or measured tests",
                    {
                        "ignored_tests": ignored,
                        "filtered_tests": filtered,
                        "measured_tests": measured,
                    },
                )
            active_planned = None
            active_statuses = []
            active_names = set()
        if active_planned is not None:
            return "INFRA_ERROR", "cargo test output has an incomplete final test block", {}
        if block_count == 0:
            return "INFRA_ERROR", "cargo test output contains no complete test blocks", {}
        if aggregate_planned <= 0 or aggregate_executed <= 0:
            return "INFRA_ERROR", "cargo test executed zero tests across all blocks", {
                "block_count": block_count,
            }
        stdout_result_indices = [
            index
            for index, line in enumerate(stdout_lines)
            if CARGO_TEST_RESULT_RE.fullmatch(line)
        ]
        if not stdout_result_indices:
            return "INFRA_ERROR", "cargo test emitted no lifecycle result on stdout", {}
        trailing_stdout = [
            line
            for line in stdout_lines[stdout_result_indices[-1] + 1 :]
            if line.strip()
        ]
        unexpected_epilogue = [
            line
            for line in trailing_stdout
            if CARGO_TEST_ALLOWED_STDOUT_EPILOGUE_RE.fullmatch(line) is None
        ]
        if unexpected_epilogue:
            return (
                "INFRA_ERROR",
                "cargo test emitted unaccounted output after its final lifecycle result",
                {"unexpected_epilogue": unexpected_epilogue},
            )
        unexpected_stderr = [
            line
            for line in classified_stderr.splitlines()
            if line.strip() and TRUSTED_BUILD_STDERR_RE.fullmatch(line) is None
        ]
        if unexpected_stderr:
            return (
                "INFRA_ERROR",
                "cargo test emitted unaccounted stderr outside the trusted build-diagnostic grammar",
                {"unexpected_stderr": unexpected_stderr},
            )
        return "PASS", f"cargo test completed {aggregate_executed} tests in {block_count} blocks", {
            "block_count": block_count,
            "planned_tests": aggregate_planned,
            "executed_tests": aggregate_executed,
            "accounted_identity_unittests": accounted_identity_unittests,
            "accounted_expected_panics": accounted_expected_panics,
        }
    if result_type == "case_result":
        if UNKNOWN_STATUS_RECORD_RE.search(combined):
            return "INFRA_ERROR", "case output contains an unsupported status record", {}
        stderr_case_records = [
            line for line in stderr.splitlines() if CASE_RESULT_SIGNATURE_RE.search(line)
        ]
        if stderr_case_records:
            return (
                "INFRA_ERROR",
                "CASE_RESULT records must be emitted on stdout",
                {"stderr_case_records": stderr_case_records},
            )
        malformed_records = [
            line
            for line in stdout.splitlines()
            if CASE_RESULT_SIGNATURE_RE.search(line) and CASE_RESULT_RE.fullmatch(line) is None
        ]
        if malformed_records:
            return (
                "INFRA_ERROR",
                "malformed CASE_RESULT record",
                {"malformed_records": malformed_records},
            )
        result_lines = CASE_RESULT_RE.findall(stdout)
        if len(result_lines) != 1:
            return "INFRA_ERROR", f"expected exactly one CASE_RESULT record, found {len(result_lines)}", {}
        status = result_lines[0]
        if status not in {"PASS", "FAIL"}:
            return "INFRA_ERROR", f"unknown CASE_RESULT status: {status}", {"case_status": status}
        if status == "PASS" and (
            NON_PASS_OUTPUT_RE.search(combined)
            or EXPLICIT_FAILURE_OUTPUT_RE.search(combined)
            or CRASH_OUTPUT_RE.search(combined)
        ):
            return "FAIL", "CASE_RESULT PASS conflicts with explicit non-pass evidence", {"case_status": status}
        lines = stdout.splitlines()
        terminal_index = max(
            (index for index, line in enumerate(lines) if line.strip()),
            default=-1,
        )
        if terminal_index < 0 or CASE_RESULT_RE.fullmatch(lines[terminal_index]) is None:
            return (
                "INFRA_ERROR",
                "the CASE_RESULT record must be the terminal non-empty output line",
                {"case_status": status},
            )
        if status == "PASS" and stderr.strip():
            return (
                "INFRA_ERROR",
                "a passing CASE_RESULT case must keep stderr empty so completion order is unambiguous",
                {"case_status": status},
            )
        return status, f"explicit CASE_RESULT: {status}", {"case_status": status}
    if result_type == "official":
        try:
            expected_ltp_cases = _live_ltp_stable_cases(repository_root())
            expected_busybox_cases, expected_libctest_cases = trusted_official_case_plan(
                repository_root()
            )
        except (ManifestError, OSError, ValueError) as error:
            return "INFRA_ERROR", str(error), {}
        validation = validate_official_output(
            stdout,
            stderr,
            expected_group_labels=contract.get("expected_group_labels"),
            expected_group_case_counts=contract.get("expected_group_case_counts"),
            expected_ltp_case_list=case.get("environment", {}).get("LTP_CASES"),
            expected_ltp_cases=expected_ltp_cases,
            expected_busybox_cases=expected_busybox_cases,
            expected_libctest_cases=expected_libctest_cases,
        )
        if validation["status"] == "PASS":
            return "PASS", "all official groups completed with explicit success", validation
        if validation["status"] == "FAIL":
            return "FAIL", "official output contains explicit non-pass results", validation
        if validation["status"] == "ERROR":
            return "INFRA_ERROR", "official output is incomplete, malformed, or lacks explicit success", validation
        return "INFRA_ERROR", f"unknown official parser status: {validation['status']!r}", validation
    if result_type == "final_2026":
        validation = validate_final_2026_output(
            stdout,
            stderr,
            expected_group=contract["expected_group"],
            expected_group_label=contract["expected_group_label"],
            expected_arch=contract["expected_arch"],
            buildstorm_baseline_seconds=contract["buildstorm_baseline_seconds"],
        )
        if validation["status"] == "PASS":
            return (
                "PASS",
                "final-2026 group completed with explicit eligible success",
                validation,
            )
        if validation["status"] == "FAIL":
            return "FAIL", "final-2026 output contains real failed score items", validation
        if validation["status"] == "ERROR":
            return (
                "INFRA_ERROR",
                "final-2026 output is incomplete, malformed, or score-ineligible",
                validation,
            )
        return (
            "INFRA_ERROR",
            f"unknown final-2026 parser status: {validation['status']!r}",
            validation,
        )
    return "INFRA_ERROR", f"unknown result contract: {result_type!r}", {}


def prepare_official_scouting_environment(
    case: dict[str, Any],
    environment: dict[str, str],
    *,
    invocation_cwd: Path,
) -> list[str]:
    """Preserve legacy scouting inputs while making their non-PASS status explicit."""
    if case["result_contract"]["type"] != "official":
        return []

    for name in OFFICIAL_BLACKLIST_FILE_ENVIRONMENT:
        tokens = environment.get(name, "").split()
        if not tokens:
            continue
        absolute_tokens: list[str] = []
        for token in tokens:
            path = Path(token)
            if not path.is_absolute():
                path = invocation_cwd / path
            absolute_tokens.append(str(path.resolve()))
        environment[name] = " ".join(absolute_tokens)

    configured = [
        name
        for name in OFFICIAL_BLACKLIST_ENVIRONMENT
        if environment.get(name, "").strip()
    ]
    skipped_groups = environment.get("OSCOMP_SKIP_TEST_GROUPS", "").strip()
    if skipped_groups and skipped_groups.lower() != "none":
        configured.append("OSCOMP_SKIP_TEST_GROUPS")
    return configured


def child_environment(
    case: dict[str, Any], *, repo: Path, cwd: Path
) -> tuple[dict[str, str], str | None]:
    """Build a closed child environment instead of inheriting ambient controls."""
    environment = {
        "PATH": os.environ.get("PATH", os.defpath),
        "HOME": os.environ.get("HOME", str(Path.home())),
        "PWD": str(cwd),
        "LC_ALL": "C",
        "LANG": "C",
        "CARGO_NET_OFFLINE": "true",
        "CARGO_HOME": str(repo / "cargo-home"),
        "PYTHONNOUSERSITE": "1",
        "PYTHONDONTWRITEBYTECODE": "1",
        "PYTHONPYCACHEPREFIX": "/dev/null",
    }
    if case["result_contract"]["type"] == "official":
        for name in OFFICIAL_CALLER_ENVIRONMENT:
            if name in os.environ:
                environment[name] = os.environ[name]
        if "TESTSUITE_DIR" not in environment and environment.get("ORAYS_WORKSPACE_ROOT"):
            environment["TESTSUITE_DIR"] = environment["ORAYS_WORKSPACE_ROOT"]
    if case["result_contract"]["type"] == "final_2026":
        for name in FINAL_2026_CALLER_ENVIRONMENT:
            if name in os.environ:
                environment[name] = os.environ[name]
    return environment, None


def validate_child_path_environment(environment: dict[str, str]) -> str | None:
    for name in ("PATH", "HOME"):
        value = environment.get(name, "")
        if any(character in value for character in ("\0", "\n", "\r", "$")):
            return (
                f"child environment {name} contains control or Make-expansion syntax; "
                "caller PATH/HOME must be literal paths"
            )
    path_entries = environment.get("PATH", "").split(":")
    if not path_entries or any(
        not entry or not Path(entry).is_absolute() for entry in path_entries
    ):
        return "child environment PATH must contain only non-empty absolute entries"
    if not Path(environment.get("HOME", "")).is_absolute():
        return "child environment HOME must be an absolute path"
    return None


def run_case(
    case: dict[str, Any],
    *,
    repo: Path,
    output_dir: Path,
    arch: str | None,
) -> dict[str, Any]:
    safe_id = re.sub(r"[^a-zA-Z0-9_.-]", "_", case["id"])
    case_output_dir = output_dir / "artifacts" / safe_id
    case_output_dir.mkdir(parents=True, exist_ok=True)
    logs_dir = output_dir / "logs"
    logs_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = logs_dir / f"{safe_id}.stdout.log"
    stderr_path = logs_dir / f"{safe_id}.stderr.log"
    stdout_path.touch()
    stderr_path.touch()
    argv = [
        expand_value(value, repo=repo, output_dir=output_dir, case_output_dir=case_output_dir, arch=arch)
        for value in case["command"]
    ]
    cwd = Path(
        expand_value(case.get("cwd", "{repo}"), repo=repo, output_dir=output_dir, case_output_dir=case_output_dir, arch=arch)
    )
    environment, environment_error = child_environment(case, repo=repo, cwd=cwd)
    for name, value in case.get("environment", {}).items():
        environment[name] = expand_value(
            value, repo=repo, output_dir=output_dir, case_output_dir=case_output_dir, arch=arch
        )
    noncanonical_official_environment = prepare_official_scouting_environment(
        case,
        environment,
        invocation_cwd=Path.cwd(),
    )
    environment["CARGO_NET_OFFLINE"] = "true"
    environment["PYTHONNOUSERSITE"] = "1"
    environment["PYTHONDONTWRITEBYTECODE"] = "1"
    environment["PYTHONPYCACHEPREFIX"] = "/dev/null"
    environment_error = environment_error or validate_child_path_environment(environment)
    started = time.monotonic()
    record: dict[str, Any] = {
        "id": case["id"],
        "description": case.get("description", ""),
        "architecture": arch,
        "command": argv,
        "cwd": str(cwd),
        "timeout_seconds": case["timeout_seconds"],
        "result_contract": case["result_contract"],
        "status": "INFRA_ERROR",
        "result": "preflight not completed",
        "started_at": utc_now(),
        "ended_at": None,
        "duration_seconds": 0.0,
        "executed": False,
        "return_code": None,
        "signal": None,
        "stdout_log": str(stdout_path),
        "stderr_log": str(stderr_path),
        "details": (
            {
                "noncanonical_official_environment": noncanonical_official_environment,
            }
            if noncanonical_official_environment
            else {}
        ),
        "required_command_paths": {},
    }

    preflight_error = environment_error
    resolved_commands: dict[str, str] = {}
    capability_records: list[dict[str, Any]] = []
    if preflight_error is None:
        preflight_error, resolved_commands = _preflight(
            case, repo=repo, environment=environment
        )
    record["required_command_paths"] = resolved_commands
    if preflight_error is None:
        preflight_error = _command_preflight(argv, cwd, environment)
    if preflight_error is None:
        preflight_error = _enable_child_subreaper()
    if preflight_error is None:
        try:
            _validated_proc_snapshot()
        except ProcessSnapshotError as error:
            preflight_error = f"reliable process tracking is unavailable: {error}"
    if preflight_error is None:
        preflight_error, capability_records = _run_required_capability_probes(
            case,
            cwd=cwd,
            environment=environment,
            resolved_commands=resolved_commands,
            case_output_dir=case_output_dir,
        )
    if capability_records:
        record["details"] = {
            **record["details"],
            "capability_executed": any(
                probe.get("executed") for probe in capability_records
            ),
            "capability_probes": capability_records,
        }
    if preflight_error is not None:
        record["result"] = preflight_error
        record["ended_at"] = utc_now()
        record["duration_seconds"] = round(time.monotonic() - started, 6)
        return record

    interrupted_by: int | None = None
    process_tracking_error: str | None = None
    try:
        preexisting_runner_children = _direct_children(
            os.getpid(), _validated_proc_snapshot()
        )
    except ProcessSnapshotError as error:
        record["result"] = f"reliable process tracking became unavailable: {error}"
        record["ended_at"] = utc_now()
        record["duration_seconds"] = round(time.monotonic() - started, 6)
        return record
    surviving_descendants: set[int] = set()
    process: subprocess.Popen[bytes] | None = None
    try:
        with stdout_path.open("wb") as stdout_file, stderr_path.open("wb") as stderr_file:
            process = subprocess.Popen(
                argv,
                cwd=cwd,
                env=environment,
                stdin=subprocess.DEVNULL,
                stdout=stdout_file,
                stderr=stderr_file,
                start_new_session=True,
            )
            record["executed"] = True
            previous_sigterm = signal.getsignal(signal.SIGTERM)

            def terminate_runner(signum: int, _frame: Any) -> None:
                raise RunnerTermination(signum)

            signal.signal(signal.SIGTERM, terminate_runner)
            try:
                return_code = process.wait(timeout=float(case["timeout_seconds"]))
            except subprocess.TimeoutExpired:
                termination = _terminate_case_processes(
                    process, preexisting_runner_children
                )
                process_tracking_error = termination.tracking_error
                record["return_code"] = process.returncode
                record["status"] = "TIMEOUT"
                record["result"] = f"case exceeded {case['timeout_seconds']} seconds; process group terminated"
                return_code = process.returncode
            except KeyboardInterrupt:
                interrupted_by = signal.SIGINT
                termination = _terminate_case_processes(
                    process, preexisting_runner_children
                )
                process_tracking_error = termination.tracking_error
                return_code = process.returncode
            except RunnerTermination as interruption:
                interrupted_by = interruption.signum
                termination = _terminate_case_processes(
                    process, preexisting_runner_children
                )
                process_tracking_error = termination.tracking_error
                return_code = process.returncode
            finally:
                signal.signal(signal.SIGTERM, previous_sigterm)
    except (OSError, ValueError) as error:
        record["status"] = "INFRA_ERROR"
        record["result"] = f"could not launch case: {error}"
        return_code = None

    record["return_code"] = return_code
    if process is not None and interrupted_by is None and record["status"] != "TIMEOUT":
        try:
            surviving_descendants = _case_related_pids(
                process.pid,
                preexisting_runner_children,
            )
        except ProcessSnapshotError as error:
            process_tracking_error = str(error)
        else:
            if surviving_descendants:
                termination = _terminate_case_processes(
                    process, preexisting_runner_children
                )
                process_tracking_error = termination.tracking_error
    if process_tracking_error is not None:
        case_timed_out = record["status"] == "TIMEOUT"
        record["status"] = "INFRA_ERROR"
        record["result"] = (
            "child process containment could not be verified: "
            f"{process_tracking_error}"
        )
        record["details"] = {
            "process_tracking_error": process_tracking_error,
            "runner_interrupted": interrupted_by is not None,
            "timed_out": case_timed_out,
        }
        if interrupted_by is not None:
            record["signal"] = interrupted_by
    elif interrupted_by is not None:
        record["status"] = "CRASH"
        record["signal"] = interrupted_by
        try:
            signal_name = signal.Signals(interrupted_by).name
        except ValueError:
            signal_name = f"signal {interrupted_by}"
        record["result"] = f"runner interrupted by {signal_name}; child process group terminated"
        record["details"] = {"runner_interrupted": True}
    elif surviving_descendants:
        record["status"] = "INFRA_ERROR"
        record["result"] = (
            "case leader exited while descendant processes were still running; "
            "all observed descendants were terminated"
        )
        record["details"] = {"surviving_descendant_count": len(surviving_descendants)}
    elif record["status"] != "TIMEOUT" and return_code is not None:
        if return_code < 0:
            record["status"] = "CRASH"
            record["signal"] = -return_code
            try:
                signal_name = signal.Signals(-return_code).name
            except ValueError:
                signal_name = f"signal {-return_code}"
            record["result"] = f"child terminated by {signal_name}"
        elif return_code in case.get("infrastructure_exit_codes", []):
            record["status"] = "INFRA_ERROR"
            record["result"] = f"child reported infrastructure exit code {return_code}"
        elif (
            return_code != 0
            and case["result_contract"]["type"] not in STRUCTURED_RESULT_TYPES
        ):
            record["status"] = "FAIL"
            record["result"] = f"child exited with status {return_code}"
        else:
            try:
                stdout, stderr = _read_logs(stdout_path, stderr_path)
            except (OSError, OutputIntegrityError) as error:
                record["status"] = "INFRA_ERROR"
                record["result"] = f"captured output is malformed: {error}"
                if (
                    case["result_contract"]["type"] in STRUCTURED_RESULT_TYPES
                    and return_code != 0
                ):
                    record["details"] = {
                        "output_integrity_error": str(error),
                        "process_exit_code": return_code,
                    }
            else:
                status, result, details = parse_contract(case, stdout, stderr)
                if (
                    case["result_contract"]["type"] in STRUCTURED_RESULT_TYPES
                    and return_code != 0
                ):
                    details = {**details, "process_exit_code": return_code}
                    if status == "PASS":
                        status = "INFRA_ERROR"
                        result = (
                            "structured output reports complete success but the child exited "
                            f"with status {return_code}"
                        )
                    elif status == "FAIL":
                        result = f"{result}; child exited with status {return_code}"
                record["status"] = status
                record["result"] = result
                record["details"] = details

    if capability_records:
        record["details"] = {
            **record["details"],
            "capability_executed": any(
                probe.get("executed") for probe in capability_records
            ),
            "capability_probes": capability_records,
        }
    if noncanonical_official_environment:
        record["details"] = {
            **record["details"],
            "noncanonical_official_environment": noncanonical_official_environment,
        }
        if record["status"] == "PASS":
            record["status"] = "INFRA_ERROR"
            record["result"] = (
                "official run used noncanonical scouting configuration and "
                "cannot count as official PASS"
            )

    record["ended_at"] = utc_now()
    record["duration_seconds"] = round(time.monotonic() - started, 6)
    return record


def report_totals(planned_count: int, results: list[dict[str, Any]]) -> tuple[dict[str, int], int]:
    totals = {status: 0 for status in sorted(KNOWN_STATUSES)}
    unknown = 0
    for result in results:
        status = result.get("status")
        if status not in KNOWN_STATUSES:
            unknown += 1
        else:
            totals[status] += 1
    executed = sum(bool(result.get("executed")) for result in results)
    completed = sum(result.get("status") != "NOT_RUN" for result in results)
    totals.update(
        {
            "planned": planned_count,
            "executed": executed,
            "completed": completed,
            "unknown": unknown,
        }
    )
    if unknown or totals["INFRA_ERROR"]:
        return totals, 2
    if (
        totals["FAIL"]
        or totals["TIMEOUT"]
        or totals["CRASH"]
        or totals["NOT_RUN"]
    ):
        return totals, 1
    if planned_count <= 0 or executed != planned_count or completed != planned_count:
        return totals, 2
    if totals["PASS"] != planned_count:
        return totals, 2
    return totals, 0


def default_output_dir(repo: Path, profile: str, arch: str | None) -> Path:
    stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    suffix = arch or "none"
    return repo / "test" / "output" / f"{stamp}-{profile}-{suffix}-{os.getpid()}"


def not_run_record(
    case: dict[str, Any],
    *,
    repo: Path,
    output_dir: Path,
    arch: str | None,
    reason: str,
) -> dict[str, Any]:
    safe_id = re.sub(r"[^a-zA-Z0-9_.-]", "_", case["id"])
    case_output_dir = output_dir / "artifacts" / safe_id
    logs_dir = output_dir / "logs"
    logs_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = logs_dir / f"{safe_id}.stdout.log"
    stderr_path = logs_dir / f"{safe_id}.stderr.log"
    stdout_path.write_bytes(b"")
    stderr_path.write_bytes(b"")
    command = [
        expand_value(
            value,
            repo=repo,
            output_dir=output_dir,
            case_output_dir=case_output_dir,
            arch=arch,
        )
        for value in case["command"]
    ]
    cwd = expand_value(
        case.get("cwd", "{repo}"),
        repo=repo,
        output_dir=output_dir,
        case_output_dir=case_output_dir,
        arch=arch,
    )
    return {
        "id": case["id"],
        "description": case.get("description", ""),
        "architecture": arch,
        "command": command,
        "cwd": cwd,
        "timeout_seconds": case["timeout_seconds"],
        "result_contract": case["result_contract"],
        "status": "NOT_RUN",
        "result": reason,
        "started_at": None,
        "ended_at": None,
        "duration_seconds": 0.0,
        "executed": False,
        "return_code": None,
        "signal": None,
        "stdout_log": str(stdout_path),
        "stderr_log": str(stderr_path),
        "details": {},
    }


def print_plan(selection: Selection) -> None:
    print(
        f"Execution plan: profile={selection.profile} "
        f"arch={selection.architecture or 'n/a'} planned={len(selection.cases)}"
    )
    for index, case in enumerate(selection.cases, start=1):
        print(f"  {index:02d}. {case['id']} (timeout={case['timeout_seconds']}s)")


def list_manifest(manifest: dict[str, Any]) -> None:
    case_by_id = {case["id"]: case for case in manifest["cases"]}
    print(f"Manifest schema: {manifest['schema_version']}")
    print(f"Registered cases: {len(case_by_id)}")
    for profile_name in manifest["profiles"]:
        profile = manifest["profiles"][profile_name]
        policy = profile.get("arch_policy", "none")
        print(f"\n{profile_name} [{policy}]: {profile.get('description', '')}")
        references = [*profile.get("cases", [])]
        for arch, case_ids in profile.get("arch_cases", {}).items():
            references.extend(f"{case_id} ({arch})" for case_id in case_ids)
        if profile.get("include"):
            print("  includes: " + ", ".join(profile["include"]))
        for reference in references:
            case_id = reference.split(" ", 1)[0]
            print(f"  - {reference}: {case_by_id[case_id].get('description', '')}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--list", action="store_true", help="validate and list the manifest without running")
    parser.add_argument("--profile", default="quick", help="profile name (default: quick)")
    parser.add_argument("--arch", help="architecture requested by official/full profiles")
    parser.add_argument("--manifest", type=Path, help="alternate manifest for integrity testing")
    parser.add_argument("--output-dir", type=Path, help="exact directory for logs and summary")
    parser.add_argument("--fail-fast", action="store_true", help="stop launching cases after the first non-pass")
    args = parser.parse_args(argv)

    repo = repository_root()
    canonical_manifest_path = (repo / "test" / "suite_manifest.json").resolve()
    manifest_path = (args.manifest or canonical_manifest_path).expanduser().resolve()
    try:
        manifest = load_manifest(manifest_path, repo)
        if (
            not args.list
            and args.profile in CANONICAL_PROFILE_NAMES
            and manifest_path != canonical_manifest_path
        ):
            raise ManifestError(
                f"profile {args.profile} requires the canonical manifest: {canonical_manifest_path}"
            )
        if args.list:
            validate_all_profile_selections(manifest)
            list_manifest(manifest)
            return 0
        selection = select_cases(manifest, args.profile, args.arch)
    except ManifestError as error:
        print(f"infrastructure error: {error}", file=sys.stderr)
        return 2

    baseline_ref = manifest.get("baseline_ref", "origin/main")
    baseline_sha = baseline_commit(repo, baseline_ref)
    runner_sha = baseline_commit(repo, "HEAD")
    if "unknown" in {baseline_sha, runner_sha}:
        print(
            f"infrastructure error: cannot resolve baseline/head commits ({baseline_ref}={baseline_sha}, HEAD={runner_sha})",
            file=sys.stderr,
        )
        return 2

    runner_status = git_worktree_status(repo)
    if runner_status is None:
        print(
            "infrastructure error: cannot inspect the runner worktree status with the closed Git environment",
            file=sys.stderr,
        )
        return 2
    runner_status_lines = runner_status.splitlines()
    runner_dirty = bool(runner_status_lines)
    canonical_evidence_profile = (
        manifest_path == canonical_manifest_path
        and selection.profile in CANONICAL_PROFILE_NAMES
    )
    if canonical_evidence_profile and runner_dirty:
        print(
            "infrastructure error: canonical profile evidence requires a clean runner worktree; "
            "commit or remove every tracked and untracked change first",
            file=sys.stderr,
        )
        for status_line in runner_status_lines[:20]:
            print(f"  {status_line}", file=sys.stderr)
        if len(runner_status_lines) > 20:
            print(
                f"  ... {len(runner_status_lines) - 20} additional status entries",
                file=sys.stderr,
            )
        return 2

    output_dir = (args.output_dir or default_output_dir(repo, selection.profile, selection.architecture)).expanduser().resolve()
    try:
        output_dir.mkdir(parents=True, exist_ok=False)
    except FileExistsError:
        print(
            f"infrastructure error: output directory already exists; refusing to mix or overwrite evidence: {output_dir}",
            file=sys.stderr,
        )
        return 2
    except OSError as error:
        print(f"infrastructure error: cannot create output directory: {error}", file=sys.stderr)
        return 2

    print_plan(selection)
    suite_started = time.monotonic()
    report: dict[str, Any] = {
        "schema_version": 1,
        "manifest": str(manifest_path),
        "profile": selection.profile,
        "architecture": selection.architecture,
        "baseline_ref": baseline_ref,
        "baseline_commit": baseline_sha,
        "runner_commit": runner_sha,
        "runner_dirty": runner_dirty,
        "runner_status": runner_status_lines,
        "runner_commit_final": None,
        "runner_dirty_final": None,
        "runner_status_final": None,
        "runner_provenance_stable": None,
        "python_runtime": {
            "isolated": bool(sys.flags.isolated),
            "no_site": bool(sys.flags.no_site),
            "dont_write_bytecode": bool(sys.flags.dont_write_bytecode),
            "pycache_prefix": sys.pycache_prefix,
        },
        "invocation": [
            sys.executable,
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
            str(Path(__file__).resolve()),
            *(argv if argv is not None else sys.argv[1:]),
        ],
        "started_at": utc_now(),
        "ended_at": None,
        "duration_seconds": 0.0,
        "planned_count": len(selection.cases),
        "executed_count": 0,
        "completed_count": 0,
        "totals": {},
        "cases": [],
        "result": "RUNNING",
        "exit_code": None,
    }
    summary_path = output_dir / "summary.json"
    _write_report(summary_path, report)

    stopped = False
    stop_reason = "not launched because --fail-fast stopped the suite"
    for index, (case, case_architecture) in enumerate(
        zip(selection.cases, selection.case_architectures), start=1
    ):
        if stopped:
            report["cases"].append(
                not_run_record(
                    case,
                    repo=repo,
                    output_dir=output_dir,
                    arch=case_architecture,
                    reason=stop_reason,
                )
            )
            continue
        print(f"[{index}/{len(selection.cases)}] RUN {case['id']}", flush=True)
        result = run_case(
            case,
            repo=repo,
            output_dir=output_dir,
            arch=case_architecture,
        )
        report["cases"].append(result)
        print(f"[{index}/{len(selection.cases)}] {result['status']} {case['id']}: {result['result']}", flush=True)
        totals, provisional_exit = report_totals(len(selection.cases), report["cases"])
        report["totals"] = totals
        report["executed_count"] = totals["executed"]
        report["completed_count"] = totals["completed"]
        report["exit_code"] = provisional_exit
        _write_report(summary_path, report)
        if result.get("details", {}).get("runner_interrupted"):
            stopped = True
            stop_reason = "not launched because the runner was interrupted"
        elif args.fail_fast and result["status"] != "PASS":
            stopped = True

    totals, exit_code = report_totals(len(selection.cases), report["cases"])
    final_runner_sha = baseline_commit(repo, "HEAD")
    final_runner_status = git_worktree_status(repo)
    final_runner_status_lines = (
        final_runner_status.splitlines()
        if final_runner_status is not None
        else None
    )
    runner_provenance_stable = (
        final_runner_sha != "unknown"
        and final_runner_status_lines is not None
        and final_runner_sha == runner_sha
        and final_runner_status_lines == runner_status_lines
    )
    report["runner_commit_final"] = final_runner_sha
    report["runner_dirty_final"] = (
        bool(final_runner_status_lines)
        if final_runner_status_lines is not None
        else None
    )
    report["runner_status_final"] = final_runner_status_lines
    report["runner_provenance_stable"] = runner_provenance_stable
    if not runner_provenance_stable:
        exit_code = 2
        report["provenance_error"] = (
            "runner HEAD or worktree status changed while the suite was executing; "
            "case results are retained for diagnosis but cannot be commit-attributed evidence"
        )
    report["totals"] = totals
    report["executed_count"] = totals["executed"]
    report["completed_count"] = totals["completed"]
    report["ended_at"] = utc_now()
    report["duration_seconds"] = round(time.monotonic() - suite_started, 6)
    report["exit_code"] = exit_code
    report["result"] = "PASS" if exit_code == 0 else "FAIL" if exit_code == 1 else "INFRA_ERROR"
    _write_report(summary_path, report)
    print(
        f"Suite result: {report['result']} planned={totals['planned']} executed={totals['executed']} "
        f"completed={totals['completed']} pass={totals['PASS']} fail={totals['FAIL']} "
        f"timeout={totals['TIMEOUT']} crash={totals['CRASH']} infra={totals['INFRA_ERROR']}"
    )
    print(f"JSON summary: {summary_path}")
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
