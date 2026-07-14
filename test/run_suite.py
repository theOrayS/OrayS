#!/usr/bin/env python3
"""Run explicitly registered OrayS test profiles with strict result accounting."""

from __future__ import annotations

import argparse
import ast
import ctypes
import datetime as dt
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

from evaluation.validate_official_results import (
    CANONICAL_LTP_CASE_LIST,
    CANONICAL_OFFICIAL_CASE_COUNTS,
    CANONICAL_OFFICIAL_GROUPS,
    TRUSTED_BUILD_STDERR_RE,
    first_unsupported_output_character,
    normalize_output_text,
    trusted_ltp_stable_cases,
    validate_official_output,
)

SCHEMA_VERSION = 1
KNOWN_STATUSES = {"PASS", "FAIL", "TIMEOUT", "CRASH", "INFRA_ERROR", "NOT_RUN"}
SUCCESS_STATUS = "PASS"
PROFILE_NAME_RE = re.compile(r"^[a-z][a-z0-9_-]*$")
CASE_ID_RE = re.compile(r"^[a-z][a-z0-9]*(?:[._-][a-z0-9]+)*$")
HISTORICAL_ID_RE = re.compile(r"(?i)(?:^|[._-])g0\d{2}(?:$|[._-])")
ENV_NAME_RE = re.compile(r"^[A-Z_][A-Z0-9_]*$")
RESULT_TYPES = {"exit_code", "check", "unittest", "cargo_test", "case_result", "official"}
ARCH_POLICIES = {"none", "one", "one_or_all"}
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
CARGO_TEST_SIGNATURE_RE = re.compile(r"^(?:running\b.*\btests?\b|test result:)", re.I)
CARGO_TEST_CASE_SIGNATURE_RE = re.compile(r"^test\s+.+?\s+\.\.\.\s+\S.*$", re.I)
CARGO_TEST_ALLOWED_STDOUT_EPILOGUE_RE = re.compile(
    r"^make(?:\[\d+\])?: Leaving directory ['`].+['`]$"
)
TOKEN_RE = re.compile(r"\{[a-z_]+\}")
ALLOWED_TOKENS = {"{repo}", "{python}", "{output_dir}", "{case_output_dir}", "{arch}"}
TERMINATION_GRACE_SECONDS = 1.0
PR_SET_CHILD_SUBREAPER = 36
_SUBREAPER_ENABLED = False
CANONICAL_OFFICIAL_ENVIRONMENT = {
    "ORAYS_TEST_OUTPUT_DIR": "{case_output_dir}",
    "OSCOMP_TEST_GROUPS": "all",
    "OSCOMP_SKIP_TEST_GROUPS": "none",
    "LTP_CASES": CANONICAL_LTP_CASE_LIST,
}
UNTRUSTED_PYTHON_ENVIRONMENT = {
    "PYTHONHOME",
    "PYTHONINSPECT",
    "PYTHONOPTIMIZE",
    "PYTHONPATH",
    "PYTHONSTARTUP",
}
UNTRUSTED_EXECUTION_ENVIRONMENT = {
    "CARGO_BUILD_RUSTC_WRAPPER",
    "CARGO_ENCODED_RUSTFLAGS",
    "GNUMAKEFLAGS",
    "MAKEFLAGS",
    "MFLAGS",
    "RUSTC_WRAPPER",
    "RUSTC_WORKSPACE_WRAPPER",
    "RUSTDOCFLAGS",
    "RUSTFLAGS",
    *UNTRUSTED_PYTHON_ENVIRONMENT,
}
CANONICAL_CHECK_CASE_IDS = (
    "check.compliance_regressions",
    "check.evaluation_runner_and_parser_integrity",
    "check.kernel_state_backed_semantics",
    "check.libc_stateful_semantics",
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
    "unit.evaluation_failure_report",
    "unit.evaluation_runner_and_parser_integrity",
    "unit.kernel_state_backed_semantics",
    "unit.libc_stateful_semantics",
    "unit.ltp_result_summary",
    "unit.memory_policy_semantics",
    "unit.no_fake_success",
    "unit.official_result_validation",
    "unit.posix_state_integrity",
    "unit.rlimit_and_fd_semantics",
    "unit.runtime_binary_patch_prohibition",
    "unit.socket_message_and_buffer_semantics",
    "unit.stat_metadata_semantics",
    "unit.suite_runner",
    "unit.synthetic_capability_integrity",
    "unit.syscall_boundary_regressions",
    "unit.test_asset_integrity",
    "unit.timer_semantics",
    "unit.user_memory_copy_boundaries",
)
CANONICAL_UNIT_EXPECTED_TESTS = {
    "unit.compliance_regressions": 7,
    "unit.evaluation_failure_report": 6,
    "unit.evaluation_runner_and_parser_integrity": 18,
    "unit.kernel_state_backed_semantics": 36,
    "unit.libc_stateful_semantics": 9,
    "unit.ltp_result_summary": 18,
    "unit.memory_policy_semantics": 3,
    "unit.no_fake_success": 10,
    "unit.official_result_validation": 100,
    "unit.posix_state_integrity": 15,
    "unit.rlimit_and_fd_semantics": 13,
    "unit.runtime_binary_patch_prohibition": 9,
    "unit.socket_message_and_buffer_semantics": 10,
    "unit.stat_metadata_semantics": 7,
    "unit.suite_runner": 130,
    "unit.synthetic_capability_integrity": 5,
    "unit.syscall_boundary_regressions": 26,
    "unit.test_asset_integrity": 36,
    "unit.timer_semantics": 3,
    "unit.user_memory_copy_boundaries": 13,
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


class ManifestError(ValueError):
    """Raised when a manifest or profile cannot be trusted."""


class RunnerTermination(Exception):
    """Raised by the narrow SIGTERM handler while a child group is active."""

    def __init__(self, signum: int) -> None:
        super().__init__(signum)
        self.signum = signum


class OutputIntegrityError(ValueError):
    """Raised when captured child output cannot be parsed as trustworthy text."""


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
        if result_type == "unittest":
            allowed_contract_keys.add("expected_tests")
        if result_type == "official":
            allowed_contract_keys.update(
                {"expected_group_labels", "expected_group_case_counts"}
            )
        _reject_unknown_keys(contract, allowed_contract_keys, f"{location}.result_contract")
        if result_type == "unittest":
            expected = contract.get("expected_tests")
            if isinstance(expected, bool) or not isinstance(expected, int) or expected <= 0:
                raise ManifestError(
                    f"{location}.result_contract.expected_tests must be a positive integer"
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

        required_paths = _validate_string_list(case.get("required_paths", []), f"{location}.required_paths")
        for path_index, raw in enumerate(required_paths):
            _safe_repo_path(repo, raw, f"{location}.required_paths[{path_index}]")
        for command_index, raw in enumerate(command):
            if raw.startswith("{repo}/"):
                _safe_repo_path(repo, raw, f"{location}.command[{command_index}]")

        _validate_string_list(case.get("required_commands", []), f"{location}.required_commands")
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
    canonical_profile_names = {"checks", "unit", "quick", "baseline", "official", "full"}
    canonical_arch_cases = {
        "rv": ["official.riscv64"],
        "la": ["official.loongarch64"],
    }
    canonical_case_ids = {
        *CANONICAL_CHECK_CASE_IDS,
        *CANONICAL_UNIT_CASE_IDS,
        *CANONICAL_BASELINE_CASE_IDS,
        *(case_id for values in canonical_arch_cases.values() for case_id in values),
    }
    is_canonical_plan = canonical_manifest or bool(case_ids & canonical_case_ids)
    if is_canonical_plan:
        if baseline_ref != "origin/main":
            raise ManifestError("canonical manifest baseline_ref must be origin/main")
        if set(profiles) != canonical_profile_names:
            raise ManifestError(
                "canonical manifest profiles must be exactly: "
                f"{sorted(canonical_profile_names)}"
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
            "baseline": ("none", ["quick"], list(CANONICAL_BASELINE_CASE_IDS), {}),
            "official": ("one", [], [], canonical_arch_cases),
            "full": ("one_or_all", ["baseline"], [], canonical_arch_cases),
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
            case = cases_by_id[case_id]
            if (
                case.get("command") != ["{python}", "-B", implementation]
                or case.get("cwd", "{repo}") != "{repo}"
                or case.get("required_paths", []) != [implementation]
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
                observed_tests = sum(
                    isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
                    and node.name.startswith("test_")
                    for node in ast.walk(tree)
                )
                declared_tests = case["result_contract"].get("expected_tests")
                pinned_tests = CANONICAL_UNIT_EXPECTED_TESTS[case_id]
                if observed_tests != pinned_tests or declared_tests != pinned_tests:
                    raise ManifestError(
                        f"case {case_id} must preserve {pinned_tests} canonical unittest methods; "
                        f"observed={observed_tests}, declared={declared_tests}"
                    )
        for case_id, expected_command in CANONICAL_BASELINE_COMMANDS.items():
            case = cases_by_id[case_id]
            if (
                case.get("command") != expected_command
                or case.get("cwd", "{repo}") != "{repo}"
                or case.get("environment", {}) != {}
            ):
                raise ManifestError(
                    f"case {case_id} must invoke its exact canonical baseline command"
                )
        live_ltp_count = len(_live_ltp_stable_cases(repo))
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


def baseline_commit(repo: Path, ref: str) -> str:
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--verify", f"{ref}^{{commit}}"],
            cwd=repo,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=10,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        return "unknown"
    return result.stdout.strip() if result.returncode == 0 else "unknown"


def _write_report(path: Path, report: dict[str, Any]) -> None:
    temporary = path.with_suffix(".json.tmp")
    temporary.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temporary.replace(path)


def _preflight(
    case: dict[str, Any],
    *,
    repo: Path,
    environment: dict[str, str],
) -> str | None:
    for command in case.get("required_commands", []):
        if shutil.which(command, path=environment.get("PATH")) is None:
            return f"required command not found: {command}"
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
            return f"required file variable {env_name} has no value or fallback"
        path = Path(value).expanduser().resolve()
        if not path.is_file():
            return f"required file for {env_name} not found: {path}"
        if not os.access(path, os.R_OK):
            return f"required file for {env_name} is not readable: {path}"
        environment[env_name] = str(path)
    return None


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
        entries = Path("/proc").iterdir()
    except OSError:
        return snapshot
    for entry in entries:
        if not entry.name.isdigit():
            continue
        try:
            raw = (entry / "stat").read_text(encoding="utf-8")
            close = raw.rfind(")")
            fields = raw[close + 2 :].split()
            snapshot[int(entry.name)] = (int(fields[1]), int(fields[2]))
        except (OSError, ValueError, IndexError):
            continue
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
    snapshot = _proc_snapshot()
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


def _terminate_case_processes(
    process: subprocess.Popen[bytes],
    preexisting_runner_children: set[int],
) -> set[int]:
    related = _case_related_pids(process.pid, preexisting_runner_children)
    try:
        os.killpg(process.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    _signal_processes(related, signal.SIGTERM)
    deadline = time.monotonic() + TERMINATION_GRACE_SECONDS
    while time.monotonic() < deadline:
        related.update(
            _case_related_pids(process.pid, preexisting_runner_children, related)
        )
        if process.poll() is not None and not (_case_related_pids(
            process.pid, preexisting_runner_children, related
        )):
            break
        time.sleep(0.02)
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    related.update(_case_related_pids(process.pid, preexisting_runner_children, related))
    _signal_processes(related, signal.SIGKILL)
    if process.poll() is None:
        process.wait()
    kill_deadline = time.monotonic() + TERMINATION_GRACE_SECONDS
    while time.monotonic() < kill_deadline:
        alive = _case_related_pids(process.pid, preexisting_runner_children, related)
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
    return related


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


def parse_contract(
    case: dict[str, Any], stdout: str, stderr: str
) -> tuple[str, str, dict[str, Any]]:
    contract = case["result_contract"]
    result_type = contract["type"]
    combined = stdout + ("\n" if stdout and stderr else "") + stderr
    if result_type != "official" and UNKNOWN_STATE_OUTPUT_RE.search(combined):
        return "INFRA_ERROR", "output contains an unknown or unexecuted status", {}
    if result_type != "official" and ZERO_EXECUTION_OUTPUT_RE.search(combined):
        return "INFRA_ERROR", "output explicitly reports zero executed tests", {}
    if result_type == "exit_code":
        if UNKNOWN_STATUS_RECORD_RE.search(combined):
            return "INFRA_ERROR", "zero-exit command emitted an unsupported status record", {}
        if (
            NON_PASS_OUTPUT_RE.search(combined)
            or EXPLICIT_FAILURE_OUTPUT_RE.search(combined)
            or CRASH_OUTPUT_RE.search(combined)
        ):
            return "FAIL", "zero-exit command output contains explicit non-pass evidence", {}
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
        if UNKNOWN_STATUS_RECORD_RE.search(combined):
            return "INFRA_ERROR", "unittest output contains an unsupported status record", {}
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
        return "PASS", f"unittest completed exactly {observed} tests", {"observed_tests": observed}
    if result_type == "cargo_test":
        stdout_lines = stdout.splitlines()
        lines = stdout_lines
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
            ("\n" if stdout_non_protocol_text and stderr else "") + stderr
        )
        if UNKNOWN_STATUS_RECORD_RE.search(non_protocol_text):
            return "INFRA_ERROR", "cargo test output contains an unsupported status record", {}
        if (
            NON_PASS_OUTPUT_RE.search(non_protocol_text)
            or EXPLICIT_FAILURE_OUTPUT_RE.search(non_protocol_text)
            or CRASH_OUTPUT_RE.search(non_protocol_text)
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
            for line in stderr.splitlines()
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
        except ManifestError as error:
            return "INFRA_ERROR", str(error), {}
        validation = validate_official_output(
            stdout,
            stderr,
            expected_group_labels=contract.get("expected_group_labels"),
            expected_group_case_counts=contract.get("expected_group_case_counts"),
            expected_ltp_case_list=case.get("environment", {}).get("LTP_CASES"),
            expected_ltp_cases=expected_ltp_cases,
        )
        if validation["status"] == "PASS":
            return "PASS", "all official groups completed with explicit success", validation
        if validation["status"] == "FAIL":
            return "FAIL", "official output contains explicit non-pass results", validation
        if validation["status"] == "ERROR":
            return "INFRA_ERROR", "official output is incomplete, malformed, or lacks explicit success", validation
        return "INFRA_ERROR", f"unknown official parser status: {validation['status']!r}", validation
    return "INFRA_ERROR", f"unknown result contract: {result_type!r}", {}


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
    environment = os.environ.copy()
    environment["CARGO_NET_OFFLINE"] = "true"
    for name, value in case.get("environment", {}).items():
        environment[name] = expand_value(
            value, repo=repo, output_dir=output_dir, case_output_dir=case_output_dir, arch=arch
        )
    for name in UNTRUSTED_EXECUTION_ENVIRONMENT:
        environment.pop(name, None)
    environment["CARGO_NET_OFFLINE"] = "true"
    environment["PYTHONNOUSERSITE"] = "1"
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
        "details": {},
    }

    preflight_error = _preflight(case, repo=repo, environment=environment)
    if preflight_error is None:
        preflight_error = _command_preflight(argv, cwd, environment)
    if preflight_error is None:
        preflight_error = _enable_child_subreaper()
    if preflight_error is not None:
        record["result"] = preflight_error
        record["ended_at"] = utc_now()
        record["duration_seconds"] = round(time.monotonic() - started, 6)
        return record

    interrupted_by: int | None = None
    preexisting_runner_children = _direct_children(os.getpid(), _proc_snapshot())
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
                _terminate_case_processes(process, preexisting_runner_children)
                record["return_code"] = process.returncode
                record["status"] = "TIMEOUT"
                record["result"] = f"case exceeded {case['timeout_seconds']} seconds; process group terminated"
                return_code = process.returncode
            except KeyboardInterrupt:
                interrupted_by = signal.SIGINT
                _terminate_case_processes(process, preexisting_runner_children)
                return_code = process.returncode
            except RunnerTermination as interruption:
                interrupted_by = interruption.signum
                _terminate_case_processes(process, preexisting_runner_children)
                return_code = process.returncode
            finally:
                signal.signal(signal.SIGTERM, previous_sigterm)
    except (OSError, ValueError) as error:
        record["status"] = "INFRA_ERROR"
        record["result"] = f"could not launch case: {error}"
        return_code = None

    record["return_code"] = return_code
    if process is not None and interrupted_by is None and record["status"] != "TIMEOUT":
        surviving_descendants = _case_related_pids(
            process.pid,
            preexisting_runner_children,
        )
        if surviving_descendants:
            _terminate_case_processes(process, preexisting_runner_children)
    if interrupted_by is not None:
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
        elif return_code != 0:
            record["status"] = "FAIL"
            record["result"] = f"child exited with status {return_code}"
        else:
            try:
                stdout, stderr = _read_logs(stdout_path, stderr_path)
            except (OSError, OutputIntegrityError) as error:
                record["status"] = "INFRA_ERROR"
                record["result"] = f"captured output is malformed: {error}"
            else:
                status, result, details = parse_contract(case, stdout, stderr)
                record["status"] = status
                record["result"] = result
                record["details"] = details

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
    manifest_path = (args.manifest or repo / "test" / "suite_manifest.json").expanduser().resolve()
    try:
        manifest = load_manifest(manifest_path, repo)
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
        "invocation": [
            sys.executable,
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
