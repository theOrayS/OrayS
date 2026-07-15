#!/usr/bin/env python3
"""Versioned, fail-closed semantic evidence collection for OrayS PR3.

The manifest is the only case inventory.  This module validates that manifest,
executes commands under a bounded process-group supervisor, classifies the raw
evidence, and writes a canonical JSON result.  Renderers consume that result;
they do not re-classify raw logs.
"""

from __future__ import annotations

import argparse
import copy
import ctypes
import datetime as dt
import hashlib
import json
import math
import os
import re
import shutil
import signal
import stat
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any, Iterable, Sequence

from evaluator_protocol import normalize_terminal_bytes


MANIFEST_SCHEMA_VERSION = 1
RESULT_SCHEMA_VERSION = 1
EVIDENCE_LEVELS = (
    "declared",
    "static_checked",
    "built",
    "booted",
    "runtime_semantic",
)
RESULT_STATES = ("pass", "fail", "error", "timeout", "blocked", "skipped")
POLICIES = ("required", "observational")
ARCHITECTURES = ("host", "riscv64", "loongarch64")
CAPABILITY_KINDS = ("tool", "executable", "path", "env")
CLASSIFIER_CEILINGS = {
    "exit_code": "built",
    "guard_protocol": "static_checked",
    "marker_protocol": "runtime_semantic",
}
DEFAULT_GRACE_SECONDS = 2.0
MIN_TIMEOUT_SECONDS = 1
MAX_TIMEOUT_SECONDS = 21600
MAX_CLASSIFIABLE_LOG_BYTES = 64 * 1024 * 1024
MAX_JSON_INPUT_BYTES = 16 * 1024 * 1024
GLOBAL_RUNTIME_FATAL_PATTERNS = (
    r"\bTFAIL\b",
    r"\bTBROK\b",
    r"\bTCONF\b",
    r"\bENOSYS\b",
    r"\b(?:panic|panicked)\b",
    r"\b(?:trap|InstructionNotExist)\b",
    r"\bnot implemented\b",
    r"\b(?:TIMEOUT|timed out)\b",
)


class EvidenceError(Exception):
    """Base class for user-facing evidence errors."""


class JsonInputError(EvidenceError):
    """The JSON input is malformed or ambiguous."""


class ManifestError(EvidenceError):
    """The manifest violates the v1 contract."""


class ResultError(EvidenceError):
    """A canonical result violates the v1 contract."""


class TerminationRequested(EvidenceError):
    """The evidence process received a host termination signal."""

    def __init__(self, signum: int) -> None:
        super().__init__(f"received signal {signum}")
        self.signum = signum


def _reject_constant(value: str) -> None:
    raise JsonInputError(f"non-finite JSON number is forbidden: {value}")


def _unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise JsonInputError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def strict_json_load(path: Path) -> Any:
    fd: int | None = None
    try:
        flags = os.O_RDONLY | getattr(os, "O_CLOEXEC", 0) | getattr(os, "O_NONBLOCK", 0)
        flags |= getattr(os, "O_NOFOLLOW", 0)
        fd = os.open(path, flags)
        metadata = os.fstat(fd)
        if not stat.S_ISREG(metadata.st_mode):
            raise JsonInputError(f"JSON input is not a regular file: {path}")
        if metadata.st_size > MAX_JSON_INPUT_BYTES:
            raise JsonInputError(
                f"JSON input exceeds limit ({metadata.st_size} > "
                f"{MAX_JSON_INPUT_BYTES} bytes): {path}"
            )
        with os.fdopen(fd, "rb") as stream:
            fd = None
            raw = stream.read(MAX_JSON_INPUT_BYTES + 1)
        if len(raw) > MAX_JSON_INPUT_BYTES:
            raise JsonInputError(
                f"JSON input exceeds limit ({len(raw)} > "
                f"{MAX_JSON_INPUT_BYTES} bytes): {path}"
            )
    except JsonInputError:
        raise
    except OSError as exc:
        raise JsonInputError(f"cannot read {path}: {exc}") from exc
    except MemoryError as exc:
        raise JsonInputError(f"cannot allocate memory for JSON input: {path}") from exc
    finally:
        if fd is not None:
            os.close(fd)
    try:
        text = raw.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        raise JsonInputError(f"{path} is not valid UTF-8: {exc}") from exc
    try:
        value = json.loads(
            text,
            object_pairs_hook=_unique_object,
            parse_constant=_reject_constant,
        )
    except JsonInputError:
        raise
    except (json.JSONDecodeError, ValueError, RecursionError, MemoryError) as exc:
        raise JsonInputError(f"malformed JSON in {path}: {exc}") from exc
    pending = [value]
    while pending:
        item = pending.pop()
        if isinstance(item, str):
            try:
                item.encode("utf-8", errors="strict")
            except UnicodeEncodeError as exc:
                raise JsonInputError(
                    f"JSON string contains an unpaired surrogate in {path}"
                ) from exc
        elif isinstance(item, dict):
            pending.extend(item.keys())
            pending.extend(item.values())
        elif isinstance(item, list):
            pending.extend(item)
    return value


def canonical_json_bytes(value: Any) -> bytes:
    return (
        json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2, allow_nan=False)
        + "\n"
    ).encode("utf-8")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_size_sha256(path: Path) -> tuple[int, str]:
    if not path.is_file():
        raise OSError(f"not a regular file: {path}")
    size = path.stat().st_size
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(1024 * 1024):
            digest.update(chunk)
    return size, digest.hexdigest()


def _atomic_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=f".{path.name}.", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    except BaseException:
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass
        raise


def _expect_object(value: Any, where: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ManifestError(f"{where} must be an object")
    return value


def _expect_list(value: Any, where: str) -> list[Any]:
    if not isinstance(value, list):
        raise ManifestError(f"{where} must be an array")
    return value


def _expect_string(value: Any, where: str) -> str:
    if not isinstance(value, str) or not value:
        raise ManifestError(f"{where} must be a non-empty string")
    return value


def _check_keys(
    obj: dict[str, Any],
    *,
    required: Iterable[str],
    optional: Iterable[str] = (),
    where: str,
) -> None:
    required_set = set(required)
    allowed = required_set | set(optional)
    missing = sorted(required_set - obj.keys())
    unknown = sorted(obj.keys() - allowed)
    if missing:
        raise ManifestError(f"{where} missing fields: {', '.join(missing)}")
    if unknown:
        raise ManifestError(f"{where} has unknown fields: {', '.join(unknown)}")


def _check_id(value: Any, where: str) -> str:
    identifier = _expect_string(value, where)
    if not re.fullmatch(r"[a-z0-9][a-z0-9._-]*", identifier):
        raise ManifestError(f"{where} is not a stable lowercase identifier: {identifier}")
    return identifier


def _check_enum(value: Any, choices: Sequence[str], where: str) -> str:
    text = _expect_string(value, where)
    if text not in choices:
        raise ManifestError(f"{where} must be one of {', '.join(choices)}; got {text}")
    return text


def _check_timeout(value: Any, where: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int):
        raise ManifestError(f"{where} must be an integer")
    if not MIN_TIMEOUT_SECONDS <= value <= MAX_TIMEOUT_SECONDS:
        raise ManifestError(
            f"{where} must be in {MIN_TIMEOUT_SECONDS}..{MAX_TIMEOUT_SECONDS}"
        )
    return value


def _check_repo_path(value: Any, where: str) -> str:
    text = _expect_string(value, where)
    candidate = Path(text)
    if candidate.is_absolute() or ".." in candidate.parts:
        raise ManifestError(f"{where} must be a repository-relative path: {text}")
    return candidate.as_posix()


def _resolve_repo_path(
    repo_root: Path, value: Any, where: str, *, must_exist: bool
) -> str:
    relative = _check_repo_path(value, where)
    root = repo_root.resolve()
    candidate = (root / relative).resolve()
    try:
        candidate.relative_to(root)
    except ValueError as exc:
        raise ManifestError(f"{where} escapes the repository through a symlink: {relative}") from exc
    if must_exist and not candidate.is_file():
        raise ManifestError(f"{where} does not name an existing repository file: {relative}")
    return relative


def _check_string_list(
    value: Any,
    where: str,
    *,
    unique: bool = True,
    sorted_values: bool = False,
) -> list[str]:
    items = _expect_list(value, where)
    result = [_expect_string(item, f"{where}[{index}]") for index, item in enumerate(items)]
    if unique and len(set(result)) != len(result):
        raise ManifestError(f"{where} contains duplicates")
    if sorted_values and result != sorted(result):
        raise ManifestError(f"{where} must be sorted")
    return result


def _check_command(value: Any, where: str) -> list[str]:
    command = _check_string_list(value, where, unique=False)
    if not command:
        raise ManifestError(f"{where} must not be empty")
    for token in command:
        if "\x00" in token:
            raise ManifestError(f"{where} contains a NUL byte")
    return command


def _check_provenance(value: Any, where: str, repo_root: Path) -> list[str]:
    raw = _expect_list(value, where)
    result = [
        _resolve_repo_path(repo_root, item, f"{where}[{index}]", must_exist=True)
        for index, item in enumerate(raw)
    ]
    if not result:
        raise ManifestError(f"{where} must not be empty")
    if len(set(result)) != len(result):
        raise ManifestError(f"{where} contains duplicates")
    if result != sorted(result):
        raise ManifestError(f"{where} must be sorted")
    return result


def _evidence_rank(level: str) -> int:
    return EVIDENCE_LEVELS.index(level)


def _substitute_arch(value: str, arch: str) -> str:
    return value.replace("{arch}", arch)


def _runner_map(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {runner["id"]: runner for runner in manifest["runners"]}


def _capability_map(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {capability["id"]: capability for capability in manifest["capabilities"]}


def _validate_classifier(
    classifier: Any,
    *,
    target_evidence: str,
    runner_ceiling: str,
    where: str,
) -> dict[str, Any]:
    obj = _expect_object(classifier, where)
    kind = _check_enum(obj.get("kind"), tuple(CLASSIFIER_CEILINGS), f"{where}.kind")
    if kind == "exit_code":
        _check_keys(obj, required=("kind",), where=where)
    elif kind == "guard_protocol":
        _check_keys(
            obj,
            required=("kind", "pass_pattern", "fail_pattern"),
            optional=("min_tests",),
            where=where,
        )
        for name in ("pass_pattern", "fail_pattern"):
            pattern = _expect_string(obj[name], f"{where}.{name}")
            try:
                re.compile(pattern)
            except re.error as exc:
                raise ManifestError(f"{where}.{name} is invalid regex: {exc}") from exc
        if "min_tests" in obj:
            minimum = obj["min_tests"]
            if isinstance(minimum, bool) or not isinstance(minimum, int) or minimum < 1:
                raise ManifestError(f"{where}.min_tests must be a positive integer")
    else:
        _check_keys(
            obj,
            required=("kind", "ordered_markers", "fatal_patterns", "boot_marker"),
            where=where,
        )
        markers = _check_string_list(obj["ordered_markers"], f"{where}.ordered_markers")
        if not markers:
            raise ManifestError(f"{where}.ordered_markers must not be empty")
        fatal_patterns = _check_string_list(
            obj["fatal_patterns"], f"{where}.fatal_patterns"
        )
        if not fatal_patterns:
            raise ManifestError(f"{where}.fatal_patterns must not be empty")
        for index, pattern in enumerate(fatal_patterns):
            try:
                re.compile(pattern)
            except re.error as exc:
                raise ManifestError(
                    f"{where}.fatal_patterns[{index}] is invalid regex: {exc}"
                ) from exc
        boot_marker = _expect_string(obj["boot_marker"], f"{where}.boot_marker")
        if boot_marker != markers[0]:
            raise ManifestError(f"{where}.boot_marker must be the first ordered marker")

    kind_ceiling = CLASSIFIER_CEILINGS[kind]
    allowed_rank = min(_evidence_rank(kind_ceiling), _evidence_rank(runner_ceiling))
    if _evidence_rank(target_evidence) > allowed_rank:
        raise ManifestError(
            f"{where} cannot establish {target_evidence}; ceiling is "
            f"{EVIDENCE_LEVELS[allowed_rank]}"
        )
    return obj


def _validate_case(
    raw_case: Any,
    *,
    runner_ids: dict[str, dict[str, Any]],
    capability_ids: dict[str, dict[str, Any]],
    repo_root: Path,
    where: str,
) -> dict[str, Any]:
    case = _expect_object(raw_case, where)
    _check_keys(
        case,
        required=(
            "id",
            "title",
            "category",
            "architectures",
            "runner_id",
            "evidence_level",
            "policy",
            "timeout_seconds",
            "requires",
            "depends_on",
            "command",
            "classifier",
            "provenance",
        ),
        optional=("artifacts", "environment"),
        where=where,
    )
    _check_id(case["id"], f"{where}.id")
    _expect_string(case["title"], f"{where}.title")
    _check_id(case["category"], f"{where}.category")
    architectures = _check_string_list(case["architectures"], f"{where}.architectures")
    if not architectures:
        raise ManifestError(f"{where}.architectures must not be empty")
    for index, arch in enumerate(architectures):
        _check_enum(arch, ARCHITECTURES, f"{where}.architectures[{index}]")
    expected_arch_order = [arch for arch in ARCHITECTURES if arch in architectures]
    if architectures != expected_arch_order:
        raise ManifestError(f"{where}.architectures must use canonical architecture order")
    runner_id = _check_id(case["runner_id"], f"{where}.runner_id")
    if runner_id not in runner_ids:
        raise ManifestError(f"{where}.runner_id references unknown runner {runner_id}")
    evidence = _check_enum(case["evidence_level"], EVIDENCE_LEVELS, f"{where}.evidence_level")
    _check_enum(case["policy"], POLICIES, f"{where}.policy")
    _check_timeout(case["timeout_seconds"], f"{where}.timeout_seconds")
    requires = _check_string_list(case["requires"], f"{where}.requires", sorted_values=True)
    for capability in requires:
        if capability not in capability_ids:
            raise ManifestError(f"{where}.requires references unknown capability {capability}")
        if case["policy"] == "required" and capability_ids[capability].get("external", False):
            raise ManifestError(
                f"{where} is required but depends on external capability {capability}"
            )
    _check_string_list(case["depends_on"], f"{where}.depends_on", sorted_values=True)
    _check_command(case["command"], f"{where}.command")
    _check_provenance(case["provenance"], f"{where}.provenance", repo_root)
    artifacts = _check_string_list(
        case.get("artifacts", []), f"{where}.artifacts", sorted_values=True
    )
    for index, artifact in enumerate(artifacts):
        relative = _resolve_repo_path(
            repo_root, artifact, f"{where}.artifacts[{index}]", must_exist=False
        )
        parts = Path(relative).parts
        if relative not in {"kernel-la", "kernel-rv"} and (
            not parts or parts[0] != "build"
        ):
            raise ManifestError(
                f"{where}.artifacts[{index}] must be an owned kernel-* or build/ output"
            )
    if "environment" in case:
        environment = _expect_object(case["environment"], f"{where}.environment")
        for key, value in environment.items():
            if not re.fullmatch(r"[A-Z_][A-Z0-9_]*", key):
                raise ManifestError(f"{where}.environment has invalid key {key}")
            _expect_string(value, f"{where}.environment.{key}")
    runner = runner_ids[runner_id]
    classifier = _validate_classifier(
        case["classifier"],
        target_evidence=evidence,
        runner_ceiling=runner["max_evidence"],
        where=f"{where}.classifier",
    )
    if classifier["kind"] != runner["classifier_kind"]:
        raise ManifestError(
            f"{where}.classifier.kind must match runner {runner_id}: "
            f"{runner['classifier_kind']}"
        )
    if (
        case["policy"] == "required"
        and evidence == "built"
        and classifier["kind"] == "exit_code"
        and not artifacts
    ):
        raise ManifestError(f"{where} required build evidence must declare artifacts")
    return case


def _expand_guard_inventory(
    inventory: dict[str, Any], repo_root: Path
) -> list[dict[str, Any]]:
    check_glob = inventory["check_glob"]
    test_glob = inventory["test_glob"]
    checks = sorted(repo_root.glob(check_glob))
    tests = sorted(repo_root.glob(test_glob))
    if not checks:
        raise ManifestError(f"inventory {inventory['id']} matched no checks: {check_glob}")
    test_by_suffix: dict[str, Path] = {}
    for test in tests:
        if not test.name.startswith("test_"):
            raise ManifestError(f"inventory {inventory['id']} has invalid test name {test}")
        suffix = test.name[len("test_") :]
        if suffix in test_by_suffix:
            raise ManifestError(f"inventory {inventory['id']} has duplicate test suffix {suffix}")
        test_by_suffix[suffix] = test

    cases: list[dict[str, Any]] = []
    paired: set[str] = set()
    found_ids: dict[str, str] = {}
    for check in checks:
        if not check.name.startswith("check_"):
            raise ManifestError(f"inventory {inventory['id']} has invalid check name {check}")
        suffix = check.name[len("check_") :]
        guard_id = suffix.removesuffix(".py").split("_", 1)[0]
        if guard_id in found_ids:
            raise ManifestError(
                f"inventory {inventory['id']} has multiple checks for {guard_id}: "
                f"{found_ids[guard_id]}, {check}"
            )
        found_ids[guard_id] = check.relative_to(repo_root).as_posix()
        test = test_by_suffix.get(suffix)
        if test is None:
            raise ManifestError(f"inventory {inventory['id']} missing test for {check}")
        paired.add(suffix)
        stable = check.stem[len("check_") :].replace("_", "-")
        check_rel = check.relative_to(repo_root).as_posix()
        test_rel = test.relative_to(repo_root).as_posix()
        common = {
            "category": inventory["category"],
            "architectures": ["host"],
            "runner_id": inventory["runner_id"],
            "evidence_level": "static_checked",
            "policy": inventory["policy"],
            "timeout_seconds": inventory["timeout_seconds"],
            "requires": list(inventory["requires"]),
            "depends_on": [],
        }
        if guard_id.startswith("g") and guard_id[1:].isdigit():
            check_pass_pattern = (
                rf"^(?:G{guard_id[1:]} .*: PASS(?: .*)?"
                rf"|PASS:\s+G{guard_id[1:]}\b.*)$"
            )
        else:
            check_pass_pattern = r"^PASS self-check"
        cases.append(
            {
                **common,
                "id": f"guard.{stable}.check",
                "title": f"Compliance guard: {check.stem}",
                "command": ["python3", check_rel],
                "classifier": {
                    "kind": "guard_protocol",
                    "pass_pattern": check_pass_pattern,
                    "fail_pattern": r"(?:^FAIL\b|: FAIL\b|Traceback \(most recent call last\):)",
                },
                "provenance": [check_rel, test_rel],
            }
        )
        cases.append(
            {
                **common,
                "id": f"guard.{stable}.mutations",
                "title": f"Compliance guard mutation tests: {check.stem}",
                "command": ["python3", test_rel],
                "classifier": {
                    "kind": "guard_protocol",
                    "pass_pattern": r"^OK$",
                    "fail_pattern": r"(?:^FAILED \(|Traceback \(most recent call last\):)",
                    "min_tests": 1,
                },
                "provenance": [check_rel, test_rel],
            }
        )
    unpaired = sorted(set(test_by_suffix) - paired)
    if unpaired:
        raise ManifestError(
            f"inventory {inventory['id']} has tests without checks: {', '.join(unpaired)}"
        )
    expected_ids = set(inventory["expected_ids"])
    actual_ids = set(found_ids)
    if actual_ids != expected_ids:
        missing = sorted(expected_ids - actual_ids)
        unexpected = sorted(actual_ids - expected_ids)
        details = []
        if missing:
            details.append(f"missing expected guard ids: {', '.join(missing)}")
        if unexpected:
            details.append(f"unexpected guard ids: {', '.join(unexpected)}")
        raise ManifestError(f"inventory {inventory['id']} coverage mismatch: {'; '.join(details)}")
    return cases


def validate_manifest(document: Any, repo_root: Path) -> dict[str, Any]:
    manifest = _expect_object(document, "manifest")
    _check_keys(
        manifest,
        required=("schema_version", "suite_id", "capabilities", "runners", "inventories", "cases"),
        where="manifest",
    )
    if (
        isinstance(manifest["schema_version"], bool)
        or not isinstance(manifest["schema_version"], int)
        or manifest["schema_version"] != MANIFEST_SCHEMA_VERSION
    ):
        raise ManifestError(
            f"unsupported manifest schema_version {manifest['schema_version']}; "
            f"expected {MANIFEST_SCHEMA_VERSION}"
        )
    _check_id(manifest["suite_id"], "manifest.suite_id")

    capabilities = _expect_list(manifest["capabilities"], "manifest.capabilities")
    capability_ids: dict[str, dict[str, Any]] = {}
    for index, raw in enumerate(capabilities):
        where = f"manifest.capabilities[{index}]"
        capability = _expect_object(raw, where)
        _check_keys(
            capability,
            required=("id", "kind", "value", "external"),
            optional=("required_version",),
            where=where,
        )
        identifier = _check_id(capability["id"], f"{where}.id")
        kind = _check_enum(capability["kind"], CAPABILITY_KINDS, f"{where}.kind")
        _expect_string(capability["value"], f"{where}.value")
        if "required_version" in capability:
            _expect_string(capability["required_version"], f"{where}.required_version")
            if kind != "tool":
                raise ManifestError(
                    f"{where}.required_version is supported only for tool capabilities"
                )
        if kind in {"executable", "path"}:
            _resolve_repo_path(
                repo_root, capability["value"], f"{where}.value", must_exist=False
            )
        if kind == "env" and not re.fullmatch(
            r"[A-Z_][A-Z0-9_]*", capability["value"]
        ):
            raise ManifestError(f"{where}.value must be an environment variable name")
        if not isinstance(capability["external"], bool):
            raise ManifestError(f"{where}.external must be boolean")
        if identifier in capability_ids:
            raise ManifestError(f"duplicate capability id: {identifier}")
        capability_ids[identifier] = capability
    if [item["id"] for item in capabilities] != sorted(capability_ids):
        raise ManifestError("manifest.capabilities must be sorted by id")

    runners = _expect_list(manifest["runners"], "manifest.runners")
    runner_ids: dict[str, dict[str, Any]] = {}
    for index, raw in enumerate(runners):
        where = f"manifest.runners[{index}]"
        runner = _expect_object(raw, where)
        _check_keys(
            runner,
            required=(
                "id",
                "kind",
                "classifier_kind",
                "max_evidence",
                "combine_output",
                "grace_seconds",
            ),
            where=where,
        )
        identifier = _check_id(runner["id"], f"{where}.id")
        if runner["kind"] != "process":
            raise ManifestError(f"{where}.kind must be process")
        classifier_kind = _check_enum(
            runner["classifier_kind"], tuple(CLASSIFIER_CEILINGS), f"{where}.classifier_kind"
        )
        max_evidence = _check_enum(
            runner["max_evidence"], EVIDENCE_LEVELS, f"{where}.max_evidence"
        )
        if _evidence_rank(max_evidence) > _evidence_rank(CLASSIFIER_CEILINGS[classifier_kind]):
            raise ManifestError(
                f"{where}.max_evidence exceeds classifier ceiling "
                f"{CLASSIFIER_CEILINGS[classifier_kind]}"
            )
        if not isinstance(runner["combine_output"], bool):
            raise ManifestError(f"{where}.combine_output must be boolean")
        grace = runner["grace_seconds"]
        if isinstance(grace, bool) or not isinstance(grace, (int, float)):
            raise ManifestError(f"{where}.grace_seconds must be numeric")
        if not math.isfinite(float(grace)) or not 0.1 <= float(grace) <= 30:
            raise ManifestError(f"{where}.grace_seconds must be in 0.1..30")
        if identifier in runner_ids:
            raise ManifestError(f"duplicate runner id: {identifier}")
        runner_ids[identifier] = runner
    if [item["id"] for item in runners] != sorted(runner_ids):
        raise ManifestError("manifest.runners must be sorted by id")

    inventories = _expect_list(manifest["inventories"], "manifest.inventories")
    inventory_ids: set[str] = set()
    generated_cases: list[dict[str, Any]] = []
    for index, raw in enumerate(inventories):
        where = f"manifest.inventories[{index}]"
        inventory = _expect_object(raw, where)
        _check_keys(
            inventory,
            required=(
                "id",
                "kind",
                "check_glob",
                "test_glob",
                "runner_id",
                "category",
                "policy",
                "timeout_seconds",
                "requires",
                "expected_ids",
            ),
            where=where,
        )
        identifier = _check_id(inventory["id"], f"{where}.id")
        if identifier in inventory_ids:
            raise ManifestError(f"duplicate inventory id: {identifier}")
        inventory_ids.add(identifier)
        if inventory["kind"] != "python_guard_pairs":
            raise ManifestError(f"{where}.kind must be python_guard_pairs")
        _check_repo_path(inventory["check_glob"], f"{where}.check_glob")
        _check_repo_path(inventory["test_glob"], f"{where}.test_glob")
        runner_id = _check_id(inventory["runner_id"], f"{where}.runner_id")
        if runner_id not in runner_ids:
            raise ManifestError(f"{where}.runner_id references unknown runner {runner_id}")
        if runner_ids[runner_id]["classifier_kind"] != "guard_protocol":
            raise ManifestError(f"{where}.runner_id must use guard_protocol classification")
        if _evidence_rank(runner_ids[runner_id]["max_evidence"]) < _evidence_rank("static_checked"):
            raise ManifestError(f"{where}.runner_id cannot establish static_checked evidence")
        _check_id(inventory["category"], f"{where}.category")
        _check_enum(inventory["policy"], POLICIES, f"{where}.policy")
        _check_timeout(inventory["timeout_seconds"], f"{where}.timeout_seconds")
        requires = _check_string_list(
            inventory["requires"], f"{where}.requires", sorted_values=True
        )
        for capability in requires:
            if capability not in capability_ids:
                raise ManifestError(f"{where}.requires unknown capability {capability}")
        expected_ids = _check_string_list(
            inventory["expected_ids"], f"{where}.expected_ids", sorted_values=True
        )
        if not expected_ids:
            raise ManifestError(f"{where}.expected_ids must not be empty")
        for expected_id in expected_ids:
            _check_id(expected_id, f"{where}.expected_ids")
        generated_cases.extend(_expand_guard_inventory(inventory, repo_root))
    if [item["id"] for item in inventories] != sorted(inventory_ids):
        raise ManifestError("manifest.inventories must be sorted by id")

    explicit_cases = _expect_list(manifest["cases"], "manifest.cases")
    explicit_ids: list[str] = []
    for index, raw_case in enumerate(explicit_cases):
        case = _validate_case(
            raw_case,
            runner_ids=runner_ids,
            capability_ids=capability_ids,
            repo_root=repo_root,
            where=f"manifest.cases[{index}]",
        )
        explicit_ids.append(case["id"])
    if explicit_ids != sorted(explicit_ids):
        raise ManifestError("manifest.cases must be sorted by id")

    all_cases = [dict(item) for item in explicit_cases] + generated_cases
    all_cases.sort(key=lambda item: item["id"])
    seen: set[str] = set()
    for index, case in enumerate(all_cases):
        _validate_case(
            case,
            runner_ids=runner_ids,
            capability_ids=capability_ids,
            repo_root=repo_root,
            where=f"expanded_cases[{index}]",
        )
        if case["id"] in seen:
            raise ManifestError(f"duplicate expanded case id: {case['id']}")
        seen.add(case["id"])

    case_by_id = {case["id"]: case for case in all_cases}
    for case in all_cases:
        for dependency in case["depends_on"]:
            if dependency not in case_by_id:
                raise ManifestError(
                    f"case {case['id']} depends_on unknown case {dependency}"
                )
            if dependency == case["id"]:
                raise ManifestError(f"case {case['id']} depends on itself")
            dependency_arches = case_by_id[dependency]["architectures"]
            for arch in case["architectures"]:
                if arch not in dependency_arches and dependency_arches != ["host"]:
                    raise ManifestError(
                        f"case {case['id']} architecture {arch} cannot resolve dependency "
                        f"{dependency} architectures {dependency_arches}"
                    )

    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(case_id: str, trail: list[str]) -> None:
        if case_id in visiting:
            cycle = " -> ".join([*trail, case_id])
            raise ManifestError(f"case dependency cycle: {cycle}")
        if case_id in visited:
            return
        visiting.add(case_id)
        for dependency in case_by_id[case_id]["depends_on"]:
            visit(dependency, [*trail, case_id])
        visiting.remove(case_id)
        visited.add(case_id)

    for case_id in sorted(case_by_id):
        visit(case_id, [])

    validated = dict(manifest)
    validated["expanded_cases"] = all_cases
    return validated


def load_and_validate_manifest(path: Path, repo_root: Path) -> tuple[dict[str, Any], str]:
    document = strict_json_load(path)
    manifest = validate_manifest(document, repo_root)
    effective = dict(document)
    effective["expanded_cases"] = manifest["expanded_cases"]
    return manifest, sha256_bytes(canonical_json_bytes(effective))


def schema_document() -> dict[str, Any]:
    """Return the generated public schema bundle for manifest and result v1."""

    identifier = {"type": "string", "pattern": "^[a-z0-9][a-z0-9._-]*$"}
    nonempty = {"type": "string", "minLength": 1}
    repo_path = {
        "type": "string",
        "minLength": 1,
        "pattern": "^(?!/)(?!.*(?:^|/)\\.\\.(?:/|$)).+$",
    }
    string_array = {
        "type": "array",
        "items": nonempty,
        "uniqueItems": True,
    }
    timestamp = {
        "type": "string",
        "pattern": "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\\.[0-9]{3}Z$",
    }
    positive_integer = {"type": "integer", "minimum": 1}
    sha256 = {"type": "string", "pattern": "^[0-9a-f]{64}$"}
    nullable_string = {"type": ["string", "null"]}
    bundle_reference = {
        "type": "object",
        "additionalProperties": False,
        "required": ["path", "size_bytes", "sha256"],
        "properties": {
            "path": repo_path,
            "size_bytes": {"type": "integer", "minimum": 0},
            "sha256": sha256,
        },
    }
    classifier = {
        "oneOf": [
            {
                "type": "object",
                "additionalProperties": False,
                "required": ["kind"],
                "properties": {"kind": {"const": "exit_code"}},
            },
            {
                "type": "object",
                "additionalProperties": False,
                "required": ["kind", "pass_pattern", "fail_pattern"],
                "properties": {
                    "kind": {"const": "guard_protocol"},
                    "pass_pattern": nonempty,
                    "fail_pattern": nonempty,
                    "min_tests": positive_integer,
                },
            },
            {
                "type": "object",
                "additionalProperties": False,
                "required": ["kind", "ordered_markers", "fatal_patterns", "boot_marker"],
                "properties": {
                    "kind": {"const": "marker_protocol"},
                    "ordered_markers": {**string_array, "minItems": 1},
                    "fatal_patterns": {**string_array, "minItems": 1},
                    "boot_marker": nonempty,
                },
            },
        ]
    }
    case_properties = {
        "id": identifier,
        "title": nonempty,
        "category": identifier,
        "architectures": {
            "type": "array",
            "items": {"enum": list(ARCHITECTURES)},
            "minItems": 1,
            "uniqueItems": True,
        },
        "runner_id": identifier,
        "evidence_level": {"enum": list(EVIDENCE_LEVELS)},
        "policy": {"enum": list(POLICIES)},
        "timeout_seconds": {
            "type": "integer",
            "minimum": MIN_TIMEOUT_SECONDS,
            "maximum": MAX_TIMEOUT_SECONDS,
        },
        "requires": string_array,
        "depends_on": string_array,
        "command": {
            "type": "array",
            "items": nonempty,
            "minItems": 1,
        },
        "classifier": classifier,
        "provenance": {**string_array, "items": repo_path, "minItems": 1},
        "artifacts": {**string_array, "items": repo_path},
        "environment": {
            "type": "object",
            "propertyNames": {"pattern": "^[A-Z_][A-Z0-9_]*$"},
            "additionalProperties": nonempty,
        },
    }
    case_required = [
        "id",
        "title",
        "category",
        "architectures",
        "runner_id",
        "evidence_level",
        "policy",
        "timeout_seconds",
        "requires",
        "depends_on",
        "command",
        "classifier",
        "provenance",
    ]
    process_fields = [
        "spawned",
        "exit_code",
        "signal",
        "timed_out",
        "residual_processes_killed",
        "cleanup_complete",
        "cleanup_diagnostics",
        "term_sent",
        "kill_sent",
        "reaped",
        "spawn_error",
    ]
    result_case_fields = [
        "case_id",
        "title",
        "category",
        "architecture",
        "policy",
        "target_evidence",
        "observed_evidence",
        "state",
        "reason_code",
        "reason",
        "command",
        "cwd",
        "started_at",
        "ended_at",
        "duration_seconds",
        "process",
        "artifacts",
        "logs",
        "provenance",
    ]
    definitions: dict[str, Any] = {
        "manifest": {
            "type": "object",
            "additionalProperties": False,
            "required": [
                "schema_version",
                "suite_id",
                "capabilities",
                "runners",
                "inventories",
                "cases",
            ],
            "properties": {
                "schema_version": {"const": MANIFEST_SCHEMA_VERSION},
                "suite_id": identifier,
                "capabilities": {
                    "type": "array",
                    "items": {"$ref": "#/$defs/capability"},
                },
                "runners": {"type": "array", "items": {"$ref": "#/$defs/runner"}},
                "inventories": {
                    "type": "array",
                    "items": {"$ref": "#/$defs/inventory"},
                },
                "cases": {"type": "array", "items": {"$ref": "#/$defs/case"}},
            },
        },
        "capability": {
            "type": "object",
            "additionalProperties": False,
            "required": ["id", "kind", "value", "external"],
            "dependentSchemas": {
                "required_version": {
                    "properties": {"kind": {"const": "tool"}},
                },
            },
            "properties": {
                "id": identifier,
                "kind": {"enum": list(CAPABILITY_KINDS)},
                "value": nonempty,
                "external": {"type": "boolean"},
                "required_version": nonempty,
            },
        },
        "runner": {
            "type": "object",
            "additionalProperties": False,
            "required": [
                "id",
                "kind",
                "classifier_kind",
                "max_evidence",
                "combine_output",
                "grace_seconds",
            ],
            "properties": {
                "id": identifier,
                "kind": {"const": "process"},
                "classifier_kind": {"enum": list(CLASSIFIER_CEILINGS)},
                "max_evidence": {"enum": list(EVIDENCE_LEVELS)},
                "combine_output": {"type": "boolean"},
                "grace_seconds": {
                    "type": "number",
                    "minimum": 0.1,
                    "maximum": 30,
                },
            },
        },
        "inventory": {
            "type": "object",
            "additionalProperties": False,
            "required": [
                "id",
                "kind",
                "check_glob",
                "test_glob",
                "runner_id",
                "category",
                "policy",
                "timeout_seconds",
                "requires",
                "expected_ids",
            ],
            "properties": {
                "id": identifier,
                "kind": {"const": "python_guard_pairs"},
                "check_glob": repo_path,
                "test_glob": repo_path,
                "runner_id": identifier,
                "category": identifier,
                "policy": {"enum": list(POLICIES)},
                "timeout_seconds": {
                    "type": "integer",
                    "minimum": MIN_TIMEOUT_SECONDS,
                    "maximum": MAX_TIMEOUT_SECONDS,
                },
                "requires": string_array,
                "expected_ids": {**string_array, "minItems": 1},
            },
        },
        "case": {
            "type": "object",
            "additionalProperties": False,
            "required": case_required,
            "properties": case_properties,
        },
        "process": {
            "type": "object",
            "additionalProperties": False,
            "required": process_fields,
            "properties": {
                **{
                    name: {"type": "boolean"}
                    for name in (
                        "spawned",
                        "timed_out",
                        "residual_processes_killed",
                        "cleanup_complete",
                        "term_sent",
                        "kill_sent",
                        "reaped",
                    )
                },
                "exit_code": {"type": ["integer", "null"]},
                "signal": {"type": ["integer", "null"], "minimum": 1},
                "cleanup_diagnostics": {"type": "array", "items": nonempty},
                "spawn_error": nullable_string,
            },
        },
        "artifact_reference": {
            "type": "object",
            "additionalProperties": False,
            "required": ["source", "path", "size_bytes", "sha256"],
            "properties": {
                "source": repo_path,
                "path": repo_path,
                "size_bytes": {"type": "integer", "minimum": 1},
                "sha256": sha256,
            },
        },
        "result_case": {
            "type": "object",
            "additionalProperties": False,
            "required": result_case_fields,
            "properties": {
                "case_id": identifier,
                "title": nonempty,
                "category": nonempty,
                "architecture": {"enum": list(ARCHITECTURES)},
                "policy": {"enum": list(POLICIES)},
                "target_evidence": {"enum": list(EVIDENCE_LEVELS)},
                "observed_evidence": {
                    "oneOf": [{"enum": list(EVIDENCE_LEVELS)}, {"type": "null"}]
                },
                "state": {"enum": list(RESULT_STATES)},
                "reason_code": identifier,
                "reason": nonempty,
                "command": {"type": "array", "items": nonempty, "minItems": 1},
                "cwd": nonempty,
                "started_at": timestamp,
                "ended_at": timestamp,
                "duration_seconds": {"type": "number", "minimum": 0},
                "process": {"$ref": "#/$defs/process"},
                "artifacts": {
                    "type": "array",
                    "items": {"$ref": "#/$defs/artifact_reference"},
                },
                "logs": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["stdout", "stderr", "raw"],
                    "properties": {
                        "stdout": {
                            "oneOf": [bundle_reference, {"type": "null"}]
                        },
                        "stderr": {
                            "oneOf": [bundle_reference, {"type": "null"}]
                        },
                        "raw": bundle_reference,
                    },
                },
                "provenance": {**string_array, "items": repo_path, "minItems": 1},
            },
        },
        "result": {
            "type": "object",
            "additionalProperties": False,
            "required": [
                "schema_version",
                "manifest",
                "repository",
                "tools",
                "run",
                "cases",
                "summary",
            ],
            "properties": {
                "schema_version": {"const": RESULT_SCHEMA_VERSION},
                "manifest": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["schema_version", "suite_id", "sha256", "path"],
                    "properties": {
                        "schema_version": {"const": MANIFEST_SCHEMA_VERSION},
                        "suite_id": identifier,
                        "sha256": sha256,
                        "path": repo_path,
                    },
                },
                "repository": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["revision", "dirty", "content_sha256"],
                    "properties": {
                        "revision": {
                            "type": "string",
                            "pattern": "^(?:unknown|[0-9a-f]{40})$",
                        },
                        "dirty": {"type": "boolean"},
                        "content_sha256": {
                            "type": "string",
                            "pattern": "^(?:unknown|[0-9a-f]{64})$",
                        },
                    },
                },
                "tools": {
                    "type": "object",
                    "additionalProperties": {
                        "type": "object",
                        "additionalProperties": False,
                        "required": ["path", "version", "sha256"],
                        "properties": {
                            "path": {"anyOf": [repo_path, {"type": "null"}]},
                            "version": nullable_string,
                            "sha256": {"anyOf": [sha256, {"type": "null"}]},
                        },
                    },
                },
                "run": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": [
                        "started_at",
                        "ended_at",
                        "duration_seconds",
                        "repository_before",
                        "selected_case_count",
                        "expected_instances",
                        "selection",
                    ],
                    "properties": {
                        "started_at": timestamp,
                        "ended_at": timestamp,
                        "duration_seconds": {"type": "number", "minimum": 0},
                        "repository_before": {
                            "type": "object",
                            "additionalProperties": False,
                            "required": ["revision", "dirty", "content_sha256"],
                            "properties": {
                                "revision": {
                                    "type": "string",
                                    "pattern": "^(?:unknown|[0-9a-f]{40})$",
                                },
                                "dirty": {"type": "boolean"},
                                "content_sha256": {
                                    "type": "string",
                                    "pattern": "^(?:unknown|[0-9a-f]{64})$",
                                },
                            },
                        },
                        "selected_case_count": {"type": "integer", "minimum": 1},
                        "expected_instances": {**string_array, "minItems": 1},
                        "selection": {
                            "type": "object",
                            "additionalProperties": False,
                            "required": [
                                "requested_instances",
                                "included_dependency_instances",
                                "full_instance_count",
                                "full_required_instance_count",
                                "complete_suite",
                                "complete_required",
                            ],
                            "properties": {
                                "requested_instances": string_array,
                                "included_dependency_instances": string_array,
                                "full_instance_count": {"type": "integer", "minimum": 0},
                                "full_required_instance_count": {
                                    "type": "integer",
                                    "minimum": 0,
                                },
                                "complete_suite": {"type": "boolean"},
                                "complete_required": {"type": "boolean"},
                            },
                        },
                    },
                },
                "cases": {
                    "type": "array",
                    "items": {"$ref": "#/$defs/result_case"},
                    "minItems": 1,
                },
                "summary": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["total", "states", "policies", "required_nonpass"],
                    "properties": {
                        "total": {"type": "integer", "minimum": 1},
                        "states": {
                            "type": "object",
                            "additionalProperties": False,
                            "required": list(RESULT_STATES),
                            "properties": {
                                state: {"type": "integer", "minimum": 0}
                                for state in RESULT_STATES
                            },
                        },
                        "policies": {
                            "type": "object",
                            "additionalProperties": False,
                            "required": list(POLICIES),
                            "properties": {
                                policy: {"type": "integer", "minimum": 0}
                                for policy in POLICIES
                            },
                        },
                        "required_nonpass": {"type": "integer", "minimum": 0},
                    },
                },
            },
        },
    }
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://orays.invalid/schemas/semantic-evidence-v1.json",
        "title": "OrayS semantic evidence manifest or canonical result v1",
        "oneOf": [
            {"$ref": "#/$defs/manifest"},
            {"$ref": "#/$defs/result"},
        ],
        "$defs": definitions,
        "x-orays-enums": {
            "architectures": list(ARCHITECTURES),
            "evidence_levels": list(EVIDENCE_LEVELS),
            "policies": list(POLICIES),
            "result_states": list(RESULT_STATES),
        },
    }


def _utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")


def _safe_slug(case_id: str, arch: str) -> str:
    return re.sub(r"[^a-zA-Z0-9_.-]+", "-", f"{case_id}--{arch}")


def _write_diagnostic_logs(log_dir: Path, slug: str, message: str) -> dict[str, str | None]:
    path = log_dir / f"{slug}.log"
    _atomic_write(path, (message.rstrip() + "\n").encode("utf-8"))
    relative = path.as_posix()
    return {"stdout": None, "stderr": None, "raw": relative}


def _kill_process_group(
    proc: subprocess.Popen[Any], grace_seconds: float
) -> tuple[bool, list[str], bool, bool, bool]:
    diagnostics: list[str] = []
    complete = True
    term_sent = False
    kill_sent = False
    reaped = proc.returncode is not None
    try:
        os.killpg(proc.pid, signal.SIGTERM)
        term_sent = True
        diagnostics.append("sent SIGTERM to process group")
    except ProcessLookupError:
        try:
            proc.wait(timeout=grace_seconds)
            reaped = True
        except subprocess.TimeoutExpired:
            complete = False
        return complete, diagnostics, term_sent, kill_sent, reaped
    except OSError as exc:
        diagnostics.append(f"SIGTERM process-group error: {exc}")
        complete = False

    deadline = time.monotonic() + grace_seconds
    while _process_group_exists(proc.pid) and time.monotonic() < deadline:
        if proc.poll() is not None:
            reaped = True
        _reap_process_group_children(proc.pid)
        time.sleep(0.02)
    if _process_group_exists(proc.pid):
        try:
            os.killpg(proc.pid, signal.SIGKILL)
            kill_sent = True
            diagnostics.append("sent SIGKILL to process group after grace period")
        except ProcessLookupError:
            pass
        except OSError as exc:
            diagnostics.append(f"SIGKILL process-group error: {exc}")
            complete = False
        deadline = time.monotonic() + grace_seconds
        while _process_group_exists(proc.pid) and time.monotonic() < deadline:
            if proc.poll() is not None:
                reaped = True
            _reap_process_group_children(proc.pid)
            time.sleep(0.02)
        _reap_process_group_children(proc.pid)
        if _process_group_exists(proc.pid):
            diagnostics.append("process group still exists after SIGKILL")
            complete = False
    try:
        proc.wait(timeout=grace_seconds)
        reaped = True
    except subprocess.TimeoutExpired:
        diagnostics.append("process leader could not be reaped")
        complete = False
    return complete, diagnostics, term_sent, kill_sent, reaped


def _process_group_exists(pgid: int) -> bool:
    try:
        os.killpg(pgid, 0)
    except ProcessLookupError:
        return False
    except PermissionError:
        return True
    return True


def _reap_process_group_children(pgid: int) -> None:
    """Reap subreaper-owned zombies without touching unrelated child groups."""

    while True:
        try:
            pid, _ = os.waitpid(-pgid, os.WNOHANG)
        except (ChildProcessError, ProcessLookupError):
            return
        if pid == 0:
            return


def _enable_child_subreaper() -> tuple[bool, str | None]:
    """Make escaped, orphaned grandchildren observable by this process.

    Linux process groups alone are insufficient because a command can call
    ``setsid()``.  PR_SET_CHILD_SUBREAPER makes such descendants reparent here
    when their intermediate parent exits, so they can still be terminated and
    reaped.  Required evidence fails closed when this facility is unavailable.
    """

    if not sys.platform.startswith("linux"):
        return False, "detached-descendant tracking requires Linux PR_SET_CHILD_SUBREAPER"
    try:
        libc = ctypes.CDLL(None, use_errno=True)
        result = libc.prctl(36, 1, 0, 0, 0)  # PR_SET_CHILD_SUBREAPER
    except (AttributeError, OSError) as exc:
        return False, f"cannot enable PR_SET_CHILD_SUBREAPER: {exc}"
    if result != 0:
        error_number = ctypes.get_errno()
        return False, f"cannot enable PR_SET_CHILD_SUBREAPER: errno {error_number}"
    children = Path(f"/proc/{os.getpid()}/task/{os.getpid()}/children")
    if not children.is_file():
        return False, "Linux /proc child tracking is unavailable"
    return True, None


def _direct_child_pids(pid: int) -> set[int] | None:
    try:
        text = Path(f"/proc/{pid}/task/{pid}/children").read_text(encoding="ascii")
    except (OSError, UnicodeError):
        return None
    try:
        return {int(value) for value in text.split()}
    except ValueError:
        return None


def _new_descendant_pids(baseline_children: set[int]) -> set[int] | None:
    direct = _direct_child_pids(os.getpid())
    if direct is None:
        return None
    pending = list(direct - baseline_children)
    descendants: set[int] = set()
    while pending:
        pid = pending.pop()
        if pid in descendants:
            continue
        descendants.add(pid)
        children = _direct_child_pids(pid)
        if children is not None:
            pending.extend(children - descendants)
    return descendants


def _reap_known_children(pids: set[int]) -> None:
    for pid in sorted(pids):
        try:
            os.waitpid(pid, os.WNOHANG)
        except (ChildProcessError, ProcessLookupError):
            pass


def _cleanup_detached_descendants(
    baseline_children: set[int], grace_seconds: float
) -> tuple[bool, bool, list[str], bool, bool]:
    """Terminate descendants that escaped the command's original session."""

    diagnostics: list[str] = []
    observed = _new_descendant_pids(baseline_children)
    if observed is None:
        return False, False, ["cannot enumerate detached descendants"], False, False
    if not observed:
        return False, True, diagnostics, False, False

    diagnostics.append(
        "detected detached descendant process(es): "
        + ", ".join(str(pid) for pid in sorted(observed))
    )
    term_sent = False
    kill_sent = False
    known = set(observed)
    for pid in sorted(known, reverse=True):
        try:
            os.kill(pid, signal.SIGTERM)
            term_sent = True
        except ProcessLookupError:
            pass
        except OSError as exc:
            diagnostics.append(f"SIGTERM detached descendant {pid} error: {exc}")

    deadline = time.monotonic() + grace_seconds
    remaining: set[int] | None = known
    while time.monotonic() < deadline:
        _reap_known_children(known)
        remaining = _new_descendant_pids(baseline_children)
        if remaining is None or not remaining:
            break
        known.update(remaining)
        time.sleep(0.02)

    if remaining is None:
        return True, False, diagnostics + ["lost detached-descendant visibility"], term_sent, kill_sent
    if remaining:
        for pid in sorted(remaining, reverse=True):
            try:
                os.kill(pid, signal.SIGKILL)
                kill_sent = True
            except ProcessLookupError:
                pass
            except OSError as exc:
                diagnostics.append(f"SIGKILL detached descendant {pid} error: {exc}")
        deadline = time.monotonic() + max(grace_seconds, 0.25)
        while time.monotonic() < deadline:
            _reap_known_children(known | remaining)
            remaining = _new_descendant_pids(baseline_children)
            if remaining is None or not remaining:
                break
            known.update(remaining)
            time.sleep(0.02)

    _reap_known_children(known)
    remaining = _new_descendant_pids(baseline_children)
    complete = remaining == set()
    if remaining is None:
        diagnostics.append("cannot verify detached-descendant cleanup")
        complete = False
    elif remaining:
        diagnostics.append(
            "detached descendants remain after SIGKILL: "
            + ", ".join(str(pid) for pid in sorted(remaining))
        )
    return True, complete, diagnostics, term_sent, kill_sent


def run_process(
    *,
    command: list[str],
    cwd: Path,
    environment: dict[str, str],
    timeout_seconds: int,
    grace_seconds: float,
    combine_output: bool,
    log_dir: Path,
    slug: str,
) -> dict[str, Any]:
    log_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = log_dir / f"{slug}.stdout.log"
    stderr_path = log_dir / f"{slug}.stderr.log"
    raw_path = log_dir / f"{slug}.log"
    started_at = _utc_now()
    started = time.monotonic()
    spawned = False
    timed_out = False
    residual_processes_killed = False
    cleanup_complete = True
    cleanup_diagnostics: list[str] = []
    term_sent = False
    kill_sent = False
    reaped = False
    exit_code: int | None = None
    signal_number: int | None = None
    spawn_error: str | None = None
    proc: subprocess.Popen[Any] | None = None
    pending_exception: BaseException | None = None
    subreaper_available, subreaper_error = _enable_child_subreaper()
    baseline_children = _direct_child_pids(os.getpid()) if subreaper_available else None
    if baseline_children is None:
        cleanup_complete = False
        cleanup_diagnostics.append(
            subreaper_error or "cannot establish detached-descendant tracking baseline"
        )

    with stdout_path.open("wb") as stdout_stream:
        stderr_stream = None if combine_output else stderr_path.open("wb")
        try:
            try:
                proc = subprocess.Popen(
                    command,
                    cwd=cwd,
                    env=environment,
                    stdin=subprocess.DEVNULL,
                    stdout=stdout_stream,
                    stderr=subprocess.STDOUT if combine_output else stderr_stream,
                    start_new_session=True,
                )
            except (OSError, ValueError) as exc:
                spawn_error = f"{type(exc).__name__}: {exc}"
            else:
                spawned = True
                try:
                    try:
                        exit_code = proc.wait(timeout=timeout_seconds)
                        reaped = True
                    except subprocess.TimeoutExpired:
                        timed_out = True
                        complete, diagnostics, sent_term, sent_kill, group_reaped = (
                            _kill_process_group(proc, grace_seconds)
                        )
                        cleanup_complete = cleanup_complete and complete
                        cleanup_diagnostics.extend(diagnostics)
                        term_sent = term_sent or sent_term
                        kill_sent = kill_sent or sent_kill
                        reaped = reaped or group_reaped
                        exit_code = proc.returncode
                    if not timed_out and _process_group_exists(proc.pid):
                        residual_processes_killed = True
                        complete, diagnostics, sent_term, sent_kill, group_reaped = (
                            _kill_process_group(proc, grace_seconds)
                        )
                        cleanup_complete = cleanup_complete and complete
                        cleanup_diagnostics.extend(diagnostics)
                        term_sent = term_sent or sent_term
                        kill_sent = kill_sent or sent_kill
                        reaped = reaped or group_reaped
                        exit_code = proc.returncode
                    if baseline_children is not None:
                        detached, complete, diagnostics, sent_term, sent_kill = (
                            _cleanup_detached_descendants(baseline_children, grace_seconds)
                        )
                        residual_processes_killed = residual_processes_killed or detached
                        cleanup_complete = cleanup_complete and complete
                        cleanup_diagnostics.extend(diagnostics)
                        term_sent = term_sent or sent_term
                        kill_sent = kill_sent or sent_kill
                except BaseException as exc:
                    complete, diagnostics, sent_term, sent_kill, group_reaped = (
                        _kill_process_group(proc, grace_seconds)
                    )
                    cleanup_complete = cleanup_complete and complete
                    cleanup_diagnostics.extend(diagnostics)
                    term_sent = term_sent or sent_term
                    kill_sent = kill_sent or sent_kill
                    reaped = reaped or group_reaped
                    if baseline_children is not None:
                        detached, complete, diagnostics, sent_term, sent_kill = (
                            _cleanup_detached_descendants(baseline_children, grace_seconds)
                        )
                        residual_processes_killed = residual_processes_killed or detached
                        cleanup_complete = cleanup_complete and complete
                        cleanup_diagnostics.extend(diagnostics)
                        term_sent = term_sent or sent_term
                        kill_sent = kill_sent or sent_kill
                    pending_exception = exc
        finally:
            if stderr_stream is not None:
                stderr_stream.close()

    if exit_code is not None and exit_code < 0:
        signal_number = -exit_code

    if combine_output:
        shutil.copyfile(stdout_path, raw_path)
        stderr_ref: str | None = None
    else:
        with stdout_path.open("rb") as stdout_source, stderr_path.open(
            "rb"
        ) as stderr_source, raw_path.open("wb") as raw:
            shutil.copyfileobj(stdout_source, raw)
            if stdout_path.stat().st_size and stderr_path.stat().st_size:
                raw.write(b"\n--- stderr ---\n")
            shutil.copyfileobj(stderr_source, raw)
        stderr_ref = stderr_path.as_posix()

    if pending_exception is not None:
        raise pending_exception

    if spawn_error is not None and raw_path.stat().st_size == 0:
        raw_path.write_text(f"spawn error: {spawn_error}\n", encoding="utf-8")

    ended_at = _utc_now()
    duration = time.monotonic() - started
    return {
        "started_at": started_at,
        "ended_at": ended_at,
        "duration_seconds": round(duration, 6),
        "spawned": spawned,
        "exit_code": exit_code,
        "signal": signal_number,
        "timed_out": timed_out,
        "residual_processes_killed": residual_processes_killed,
        "cleanup_complete": cleanup_complete,
        "cleanup_diagnostics": cleanup_diagnostics,
        "term_sent": term_sent,
        "kill_sent": kill_sent,
        "reaped": reaped,
        "spawn_error": spawn_error,
        "logs": {
            "stdout": stdout_path.as_posix(),
            "stderr": stderr_ref,
            "raw": raw_path.as_posix(),
        },
    }


def _decode_log(path: Path) -> tuple[str | None, str | None]:
    try:
        size = path.stat().st_size
        if size > MAX_CLASSIFIABLE_LOG_BYTES:
            return (
                None,
                f"raw log exceeds classification limit "
                f"({size} > {MAX_CLASSIFIABLE_LOG_BYTES} bytes)",
            )
        normalized, terminal_error = normalize_terminal_bytes(path.read_bytes())
        if terminal_error is not None:
            return None, (
                "raw log contains an ambiguous terminal control at line "
                f"{terminal_error[0]}: {terminal_error[1]}"
            )
        return normalized.decode("utf-8", errors="strict"), None
    except UnicodeDecodeError as exc:
        return None, f"raw log is not valid UTF-8: {exc}"
    except OSError as exc:
        return None, f"raw log cannot be read: {exc}"


def classify_process(
    *,
    case: dict[str, Any],
    arch: str,
    process: dict[str, Any],
    raw_log: Path,
    artifacts: list[dict[str, Any]] | None = None,
) -> dict[str, Any]:
    target = case["evidence_level"]
    if not process["spawned"]:
        return {
            "state": "error",
            "reason_code": "spawn_error",
            "reason": process["spawn_error"] or "process was not spawned",
            "observed_evidence": None,
        }
    if process["timed_out"] and not process["cleanup_complete"]:
        return {
            "state": "error",
            "reason_code": "timeout_cleanup_incomplete",
            "reason": "command timed out and process cleanup could not be verified",
            "observed_evidence": None,
        }
    if process["timed_out"]:
        return {
            "state": "timeout",
            "reason_code": "process_timeout",
            "reason": f"command exceeded {case['timeout_seconds']} seconds",
            "observed_evidence": None,
        }
    if not process["cleanup_complete"]:
        return {
            "state": "error",
            "reason_code": "cleanup_incomplete",
            "reason": "process-group cleanup could not be verified",
            "observed_evidence": None,
        }
    if process["residual_processes_killed"]:
        return {
            "state": "error",
            "reason_code": "residual_process",
            "reason": "the command exited while child processes remained",
            "observed_evidence": None,
        }

    text, decode_error = _decode_log(raw_log)
    if decode_error is not None:
        return {
            "state": "error",
            "reason_code": "malformed_log",
            "reason": decode_error,
            "observed_evidence": None,
        }
    assert text is not None
    if not text.strip():
        return {
            "state": "error",
            "reason_code": "empty_log",
            "reason": "command produced no output",
            "observed_evidence": None,
        }

    classifier = case["classifier"]
    if classifier["kind"] in {"guard_protocol", "marker_protocol"} and not text.isascii():
        return {
            "state": "error",
            "reason_code": "malformed_log",
            "reason": "text protocol log contains non-ASCII characters",
            "observed_evidence": None,
        }

    protocol_text = text
    fatal_scan_text = text.replace("\0", "").replace("\a", "")

    if classifier["kind"] == "exit_code":
        expected_artifacts = case.get("artifacts", [])
        captured_artifacts = artifacts or []
        captured_sources = [item.get("source") for item in captured_artifacts]
        if expected_artifacts and captured_sources != expected_artifacts:
            missing = sorted(set(expected_artifacts) - set(captured_sources))
            return {
                "state": "error",
                "reason_code": "artifact_missing",
                "reason": "build did not produce required fresh artifact(s): "
                + ", ".join(missing or expected_artifacts),
                "observed_evidence": None,
            }
        if process["exit_code"] == 0:
            return {
                "state": "pass",
                "reason_code": "exit_zero",
                "reason": "command completed successfully",
                "observed_evidence": target,
            }
        return {
            "state": "fail",
            "reason_code": "nonzero_exit",
            "reason": f"command exited with status {process['exit_code']}",
            "observed_evidence": None,
        }

    if classifier["kind"] == "guard_protocol":
        pass_matches = re.findall(
            classifier["pass_pattern"], protocol_text, flags=re.IGNORECASE | re.MULTILINE
        )
        fail_match = re.search(
            classifier["fail_pattern"],
            fatal_scan_text,
            flags=re.IGNORECASE | re.MULTILINE,
        )
        minimum_tests = classifier.get("min_tests")
        test_counts = re.findall(
            r"^Ran ([0-9]+) tests? in [0-9.]+s$", protocol_text, flags=re.MULTILINE
        )
        nonempty_lines = [
            line.strip() for line in protocol_text.splitlines() if line.strip()
        ]
        unittest_protocol_ok = minimum_tests is None or (
            len(test_counts) == 1
            and int(test_counts[0]) >= minimum_tests
            and bool(nonempty_lines)
            and nonempty_lines[-1] == "OK"
        )
        if (
            process["exit_code"] == 0
            and len(pass_matches) == 1
            and fail_match is None
            and unittest_protocol_ok
        ):
            return {
                "state": "pass",
                "reason_code": "guard_protocol_pass",
                "reason": "guard exited zero with one valid PASS record",
                "observed_evidence": target,
            }
        if process["exit_code"] != 0 and fail_match is not None and not pass_matches:
            return {
                "state": "fail",
                "reason_code": "guard_protocol_fail",
                "reason": f"guard reported failure and exited {process['exit_code']}",
                "observed_evidence": None,
            }
        return {
            "state": "error",
            "reason_code": "ambiguous_guard_protocol",
            "reason": (
                "guard exit status and PASS/FAIL protocol disagree "
                f"(exit={process['exit_code']}, pass_records={len(pass_matches)}, "
                f"fail_record={fail_match is not None}, unittest_counts={test_counts})"
            ),
            "observed_evidence": None,
        }

    markers = [_substitute_arch(marker, arch) for marker in classifier["ordered_markers"]]
    boot_marker = _substitute_arch(classifier["boot_marker"], arch)
    lines = protocol_text.splitlines()
    observed: str | None = "booted" if boot_marker in lines else "built"
    for pattern in (*GLOBAL_RUNTIME_FATAL_PATTERNS, *classifier["fatal_patterns"]):
        if re.search(pattern, fatal_scan_text, flags=re.IGNORECASE | re.MULTILINE):
            return {
                "state": "error",
                "reason_code": "fatal_runtime_signal",
                "reason": f"runtime log matched forbidden pattern: {pattern}",
                "observed_evidence": observed,
            }

    positions: list[int] = []
    for marker in markers:
        count = lines.count(marker)
        if count != 1:
            return {
                "state": "error",
                "reason_code": "marker_count",
                "reason": f"expected marker exactly once, found {count}: {marker}",
                "observed_evidence": observed,
            }
        positions.append(lines.index(marker))
    if positions != sorted(positions) or len(set(positions)) != len(positions):
        return {
            "state": "error",
            "reason_code": "marker_order",
            "reason": "runtime markers are not in the required order",
            "observed_evidence": observed,
        }
    if process["exit_code"] != 0:
        return {
            "state": "fail",
            "reason_code": "nonzero_exit_after_markers",
            "reason": f"runtime protocol completed but process exited {process['exit_code']}",
            "observed_evidence": "booted",
        }
    return {
        "state": "pass",
        "reason_code": "runtime_protocol_complete",
        "reason": "all runtime semantic markers were observed exactly once and in order",
        "observed_evidence": target,
    }


def _capability_available(
    capability: dict[str, Any], repo_root: Path, environment: dict[str, str]
) -> tuple[bool, str]:
    kind = capability["kind"]
    value = capability["value"]
    if kind == "tool":
        resolved = shutil.which(value, path=environment.get("PATH"))
        if resolved is None:
            return False, f"tool not found: {value}"
        required_version = capability.get("required_version")
        if required_version is not None:
            observed_version = _tool_identity(value, environment)["version"]
            if observed_version != required_version:
                return False, (
                    f"tool version mismatch for {value}: expected {required_version!r}, "
                    f"observed {observed_version!r}"
                )
        return True, resolved
    if kind in {"executable", "path"}:
        path = repo_root / value
        if path.is_symlink() or not path.is_file():
            return False, f"repository file not found or not regular: {value}"
        if kind == "executable" and not os.access(path, os.X_OK):
            return False, f"repository executable is not executable: {value}"
        return True, path.resolve().as_posix()
    present = bool(environment.get(value))
    return (present, f"environment variable {'is set' if present else 'is missing'}: {value}")


def _repository_identity(repo_root: Path) -> dict[str, Any]:
    def git_bytes(*args: str) -> bytes | None:
        result = subprocess.run(
            ["git", *args], cwd=repo_root, capture_output=True, check=False
        )
        return result.stdout if result.returncode == 0 else None

    revision_raw = git_bytes("rev-parse", "HEAD")
    diff = git_bytes("diff", "--binary", "--no-ext-diff", "--no-renames", "HEAD", "--")
    untracked_raw = git_bytes("ls-files", "-z", "--others", "--exclude-standard")
    if revision_raw is None or diff is None or untracked_raw is None:
        return {"revision": "unknown", "dirty": True, "content_sha256": "unknown"}
    try:
        revision = revision_raw.decode("ascii").strip()
    except UnicodeDecodeError:
        return {"revision": "unknown", "dirty": True, "content_sha256": "unknown"}

    untracked = sorted(path for path in untracked_raw.split(b"\0") if path)
    digest = hashlib.sha256()

    def feed(label: bytes, payload: bytes) -> None:
        digest.update(label)
        digest.update(len(payload).to_bytes(8, "big"))
        digest.update(payload)

    feed(b"format\0", b"orays-repository-content-v1")
    feed(b"revision\0", revision.encode("ascii"))
    feed(b"tracked-diff\0", diff)
    try:
        for raw_path in untracked:
            candidate = repo_root / os.fsdecode(raw_path)
            metadata = candidate.lstat()
            feed(b"untracked-path\0", raw_path)
            if candidate.is_symlink():
                feed(b"symlink\0", os.fsencode(os.readlink(candidate)))
            elif candidate.is_file():
                feed(b"mode\0", b"executable" if metadata.st_mode & 0o111 else b"regular")
                file_digest = hashlib.sha256()
                size = 0
                with candidate.open("rb") as stream:
                    while chunk := stream.read(1024 * 1024):
                        size += len(chunk)
                        file_digest.update(chunk)
                feed(b"file-size\0", str(size).encode("ascii"))
                feed(b"file-sha256\0", file_digest.digest())
            else:
                feed(b"unsupported-type\0", str(metadata.st_mode).encode("ascii"))
    except OSError:
        return {"revision": revision, "dirty": True, "content_sha256": "unknown"}
    return {
        "revision": revision,
        "dirty": bool(diff or untracked),
        "content_sha256": digest.hexdigest(),
    }


def _resolved_executable_identity(
    resolved: Path,
    display_path: str,
    environment: dict[str, str],
    *,
    effective_override: Path | None = None,
    version_commands: list[list[str]] | None = None,
) -> dict[str, Any]:
    invocation_path = resolved.absolute()
    try:
        resolved_invocation = invocation_path.resolve(strict=True)
    except OSError:
        return {"path": None, "version": None, "sha256": None}
    if not resolved_invocation.is_file() or not os.access(resolved_invocation, os.X_OK):
        return {"path": None, "version": None, "sha256": None}
    if Path(display_path).is_absolute() or ".." in Path(display_path).parts:
        return {"path": None, "version": None, "sha256": None}
    effective_path: Path | None = resolved_invocation
    if effective_override is not None:
        try:
            candidate = effective_override.resolve(strict=True)
        except OSError:
            return {"path": None, "version": None, "sha256": None}
        if not candidate.is_file() or not os.access(candidate, os.X_OK):
            return {"path": None, "version": None, "sha256": None}
        effective_path = candidate

    version: str | None = None
    # Execute the original path, not its symlink target. Rustup selects rustc,
    # cargo, and related tools from argv[0]; resolving the proxy first would
    # falsely report `rustup --version`.
    if version_commands is None:
        version_commands = [
            [invocation_path.as_posix(), "--version"],
            [invocation_path.as_posix(), "-V"],
        ]
    for args in version_commands:
        try:
            result = subprocess.run(
                args,
                text=True,
                capture_output=True,
                timeout=5,
                check=False,
                env=environment,
            )
        except (OSError, subprocess.TimeoutExpired):
            continue
        if result.returncode != 0:
            continue
        output = (result.stdout or result.stderr).strip().splitlines()
        if output:
            version = output[0][:300]
            break

    # Hash the binary rustup will actually dispatch, not the proxy. Failure to
    # resolve it deliberately leaves the digest absent so passing required
    # evidence fails closed during manifest-aware validation.
    if (
        effective_override is None
        and effective_path.name == "rustup"
        and invocation_path.name != "rustup"
    ):
        try:
            result = subprocess.run(
                [effective_path.as_posix(), "which", invocation_path.name],
                text=True,
                capture_output=True,
                timeout=5,
                check=False,
                env=environment,
            )
        except (OSError, subprocess.TimeoutExpired):
            effective_path = None
        else:
            lines = result.stdout.strip().splitlines() if result.returncode == 0 else []
            if len(lines) != 1:
                effective_path = None
            else:
                candidate = Path(lines[0])
                try:
                    candidate = candidate.resolve(strict=True)
                except OSError:
                    effective_path = None
                else:
                    effective_path = (
                        candidate
                        if candidate.is_file() and os.access(candidate, os.X_OK)
                        else None
                    )
    digest: str | None = None
    if effective_path is not None:
        try:
            hasher = hashlib.sha256()
            with effective_path.open("rb") as stream:
                while chunk := stream.read(1024 * 1024):
                    hasher.update(chunk)
            digest = hasher.hexdigest()
        except OSError:
            pass
    # `path` is intentionally a stable logical name.  The SHA-256 binds the
    # actual executable selected above without leaking developer-home paths or
    # making otherwise compatible shards conflict only because runner homes
    # differ.
    return {"path": display_path, "version": version, "sha256": digest}


def _tool_identity(command: str, environment: dict[str, str]) -> dict[str, Any]:
    resolved = shutil.which(command, path=environment.get("PATH"))
    if resolved is None:
        return {"path": None, "version": None, "sha256": None}
    return _resolved_executable_identity(
        Path(resolved),
        Path(command).name,
        environment,
    )


def _repository_executable_identity(
    relative: str, repo_root: Path, environment: dict[str, str]
) -> dict[str, Any]:
    wrapper = repo_root / relative
    try:
        resolution = subprocess.run(
            [wrapper.as_posix(), "--pr3-print-effective-tool"],
            text=True,
            capture_output=True,
            timeout=5,
            check=False,
            env=environment,
            cwd=repo_root,
        )
    except (OSError, subprocess.TimeoutExpired):
        return {"path": None, "version": None, "sha256": None}
    lines = resolution.stdout.strip().splitlines() if resolution.returncode == 0 else []
    if len(lines) != 1 or resolution.stderr.strip():
        return {"path": None, "version": None, "sha256": None}
    effective = Path(lines[0])
    if not effective.is_absolute():
        return {"path": None, "version": None, "sha256": None}
    logical_name = effective.name
    if not logical_name:
        return {"path": None, "version": None, "sha256": None}
    if logical_name == "rust-lld":
        version_commands = [
            [effective.as_posix(), "-flavor", "gnu", "--version"],
        ]
    else:
        version_commands = [
            [effective.as_posix(), "--version"],
            [effective.as_posix(), "-V"],
        ]
    return _resolved_executable_identity(
        wrapper,
        logical_name,
        environment,
        effective_override=effective,
        version_commands=version_commands,
    )


def _baseline_environment(repo_root: Path) -> dict[str, str]:
    # Required evidence must not inherit Make overrides, compiler wrappers,
    # arbitrary Cargo configuration, CI secrets, or host-only feature flags.
    # Keep only process-discovery and host plumbing; a case may add an explicit,
    # manifest-versioned environment entry after this baseline is constructed.
    passthrough = (
        "ALL_PROXY",
        "HOME",
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "LOGNAME",
        "NO_PROXY",
        "PATH",
        "RUSTUP_HOME",
        "SHELL",
        "SSL_CERT_DIR",
        "SSL_CERT_FILE",
        "TEMP",
        "TERM",
        "TMP",
        "TMPDIR",
        "USER",
        "all_proxy",
        "http_proxy",
        "https_proxy",
        "no_proxy",
    )
    environment = {name: os.environ[name] for name in passthrough if name in os.environ}
    if not environment.get("PATH"):
        raise EvidenceError("required evidence environment has no PATH")
    try:
        toolchain_text = (repo_root / "rust-toolchain.toml").read_text(encoding="utf-8")
    except OSError as exc:
        raise EvidenceError(f"cannot load fixed Rust baseline: {exc}") from exc
    channels = re.findall(
        r'^\s*channel\s*=\s*"(nightly-\d{4}-\d{2}-\d{2})"\s*(?:#.*)?$',
        toolchain_text,
        flags=re.MULTILINE,
    )
    if len(channels) != 1:
        raise EvidenceError("rust-toolchain.toml does not contain a fixed nightly channel")
    environment["RUSTUP_TOOLCHAIN"] = channels[0]
    environment["CARGO_TERM_COLOR"] = "never"
    environment["LANG"] = "C.UTF-8"
    environment["LC_ALL"] = "C.UTF-8"
    environment["NO_COLOR"] = "1"
    environment["TZ"] = "UTC"
    return environment


def _normalize_command(command: list[str], repo_root: Path) -> list[str]:
    root = repo_root.as_posix().rstrip("/") + "/"
    return [f"<repo>/{token[len(root):]}" if token.startswith(root) else token for token in command]


def _log_reference(path: str | None, output_dir: Path) -> dict[str, Any] | None:
    if path is None:
        return None
    resolved_output = output_dir.resolve()
    resolved = Path(path).resolve()
    try:
        relative = resolved.relative_to(resolved_output).as_posix()
    except ValueError as exc:
        raise EvidenceError(f"log path escapes output directory: {path}") from exc
    size, digest = file_size_sha256(resolved)
    return {
        "path": relative,
        "size_bytes": size,
        "sha256": digest,
    }


def _prepare_expected_artifacts(case: dict[str, Any], repo_root: Path) -> str | None:
    for relative in case.get("artifacts", []):
        path = repo_root / relative
        try:
            if path.is_symlink():
                return f"refusing symlink artifact output: {relative}"
            if path.exists():
                if not path.is_file():
                    return f"artifact output is not a regular file: {relative}"
                path.unlink()
        except OSError as exc:
            return f"cannot clear stale artifact {relative}: {exc}"
    return None


def _capture_expected_artifacts(
    case: dict[str, Any], arch: str, repo_root: Path, output_dir: Path
) -> list[dict[str, Any]]:
    captured: list[dict[str, Any]] = []
    artifact_dir = output_dir / "artifacts"
    for relative in case.get("artifacts", []):
        source = repo_root / relative
        try:
            if source.is_symlink() or not source.is_file() or source.stat().st_size <= 0:
                continue
            destination_name = (
                f"{_safe_slug(case['id'], arch)}--{Path(relative).name}"
            )
            destination = artifact_dir / destination_name
            artifact_dir.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
            size, digest = file_size_sha256(destination)
        except OSError:
            continue
        captured.append(
            {
                "source": relative,
                "path": destination.relative_to(output_dir).as_posix(),
                "size_bytes": size,
                "sha256": digest,
            }
        )
    return captured


def _summarize_cases(cases: list[dict[str, Any]]) -> dict[str, Any]:
    states = {state: 0 for state in RESULT_STATES}
    policies = {policy: 0 for policy in POLICIES}
    required_nonpass = 0
    for case in cases:
        states[case["state"]] += 1
        policies[case["policy"]] += 1
        if case["policy"] == "required" and case["state"] != "pass":
            required_nonpass += 1
    return {
        "total": len(cases),
        "states": states,
        "policies": policies,
        "required_nonpass": required_nonpass,
    }


def _instance_records(manifest: dict[str, Any]) -> list[tuple[dict[str, Any], str]]:
    return [
        (case, arch)
        for case in manifest["expanded_cases"]
        for arch in case["architectures"]
    ]


def _dependency_arch(dependency: dict[str, Any], arch: str) -> str:
    if arch in dependency["architectures"]:
        return arch
    if dependency["architectures"] == ["host"]:
        return "host"
    raise AssertionError(f"unresolved dependency architecture: {dependency['id']} for {arch}")


def _select_instances(
    manifest: dict[str, Any],
    *,
    case_filters: set[str],
    policy_filter: str | None,
    category_filter: str | None,
    arch_filter: str | None,
) -> tuple[list[tuple[dict[str, Any], str]], list[str]]:
    case_by_id = {case["id"]: case for case in manifest["expanded_cases"]}
    unknown_filters = sorted(case_filters - case_by_id.keys())
    if unknown_filters:
        raise EvidenceError(f"unknown manifest case filter(s): {', '.join(unknown_filters)}")

    requested: set[tuple[str, str]] = set()
    for case, arch in _instance_records(manifest):
        if case_filters and case["id"] not in case_filters:
            continue
        if policy_filter and case["policy"] != policy_filter:
            continue
        if category_filter and case["category"] != category_filter:
            continue
        if arch_filter and arch != arch_filter:
            continue
        requested.add((case["id"], arch))
    if not requested:
        raise EvidenceError("no manifest cases matched the requested filters")

    ordered: list[tuple[dict[str, Any], str]] = []
    visited: set[tuple[str, str]] = set()

    def visit(case_id: str, arch: str) -> None:
        key = (case_id, arch)
        if key in visited:
            return
        case = case_by_id[case_id]
        for dependency_id in case["depends_on"]:
            dependency = case_by_id[dependency_id]
            visit(dependency_id, _dependency_arch(dependency, arch))
        visited.add(key)
        ordered.append((case, arch))

    for case_id, arch in sorted(requested):
        visit(case_id, arch)
    requested_instances = sorted(f"{case_id}@{arch}" for case_id, arch in requested)
    return ordered, requested_instances


def ensure_safe_output_directory(output_dir: Path, repo_root: Path) -> None:
    """Reject repository-local evidence output that would mutate source identity."""

    resolved_root = repo_root.resolve()
    lexical_output = Path(os.path.abspath(output_dir))
    resolved_output = output_dir.resolve()
    if resolved_output == Path(resolved_output.anchor):
        raise EvidenceError("evidence output must not be a filesystem root")
    if lexical_output != resolved_output:
        raise EvidenceError("evidence output path must not traverse symbolic links")
    try:
        relative = resolved_output.relative_to(resolved_root)
    except ValueError:
        return
    if relative == Path(".") or relative.parts[0] == ".git":
        raise EvidenceError("evidence output must not be the repository root or .git")
    check = subprocess.run(
        ["git", "check-ignore", "-q", "--", relative.as_posix()],
        cwd=resolved_root,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        text=True,
        check=False,
    )
    if check.returncode == 0:
        return
    if check.returncode == 1:
        raise EvidenceError(
            "repository-local evidence output must be ignored so collection cannot "
            f"change its own source identity: {relative.as_posix()}"
        )
    detail = check.stderr.strip() or f"git check-ignore exited {check.returncode}"
    raise EvidenceError(f"cannot verify evidence output isolation: {detail}")


def run_manifest(
    *,
    manifest_path: Path,
    output_dir: Path,
    repo_root: Path,
    case_filters: set[str],
    policy_filter: str | None,
    category_filter: str | None,
    arch_filter: str | None,
) -> tuple[dict[str, Any], int]:
    ensure_safe_output_directory(output_dir, repo_root)
    output_dir.mkdir(parents=True, exist_ok=True)
    (output_dir / "semantic-evidence-v1.json").unlink(missing_ok=True)
    for owned_directory in (output_dir / "logs", output_dir / "artifacts"):
        if owned_directory.exists():
            shutil.rmtree(owned_directory)
    manifest, manifest_hash = load_and_validate_manifest(manifest_path, repo_root)
    repository_before = _repository_identity(repo_root)
    runners = _runner_map(manifest)
    capabilities = _capability_map(manifest)
    log_dir = output_dir / "logs"
    run_started_at = _utc_now()
    run_started = time.monotonic()
    base_environment = _baseline_environment(repo_root)
    records: list[dict[str, Any]] = []

    selected, requested_instances = _select_instances(
        manifest,
        case_filters=case_filters,
        policy_filter=policy_filter,
        category_filter=category_filter,
        arch_filter=arch_filter,
    )
    completed: dict[tuple[str, str], dict[str, Any]] = {}

    for case, arch in selected:
        runner = runners[case["runner_id"]]
        slug = _safe_slug(case["id"], arch)
        command = [_substitute_arch(token, arch) for token in case["command"]]
        case_environment = dict(base_environment)
        for key, value in case.get("environment", {}).items():
            case_environment[key] = _substitute_arch(value, arch)
        dependency_failures: list[str] = []
        case_by_id = {item["id"]: item for item in manifest["expanded_cases"]}
        for dependency_id in case["depends_on"]:
            dependency = case_by_id[dependency_id]
            dependency_arch = _dependency_arch(dependency, arch)
            dependency_record = completed[(dependency_id, dependency_arch)]
            if dependency_record["state"] != "pass":
                dependency_failures.append(
                    f"{dependency_id}@{dependency_arch}={dependency_record['state']}"
                )
        missing: list[str] = []
        for capability_id in case["requires"]:
            capability = capabilities[capability_id]
            available, detail = _capability_available(capability, repo_root, case_environment)
            if not available:
                missing.append(f"{capability_id}: {detail}")
            elif capability["kind"] == "tool":
                qemu_environment = {
                    "qemu-system-riscv64": "PR3_QEMU_RV_BIN",
                    "qemu-system-loongarch64": "PR3_QEMU_LA_BIN",
                }.get(capability["value"])
                if qemu_environment is not None:
                    resolved_tool = Path(detail).resolve()
                    if not resolved_tool.is_absolute() or not os.access(resolved_tool, os.X_OK):
                        missing.append(f"{capability_id}: resolved tool is not executable")
                    else:
                        case_environment[qemu_environment] = resolved_tool.as_posix()
        artifact_prepare_error = (
            None
            if dependency_failures or missing
            else _prepare_expected_artifacts(case, repo_root)
        )
        captured_artifacts: list[dict[str, Any]] = []
        if dependency_failures or missing:
            started_at = _utc_now()
            if dependency_failures:
                reason_code = "dependency_nonpass"
                reason = "required dependency did not pass: " + ", ".join(dependency_failures)
            else:
                reason_code = "missing_prerequisite"
                reason = "; ".join(missing)
            logs = _write_diagnostic_logs(
                log_dir,
                slug,
                "blocked before execution:\n" + reason,
            )
            process = {
                "spawned": False,
                "exit_code": None,
                "signal": None,
                "timed_out": False,
                "residual_processes_killed": False,
                "cleanup_complete": True,
                "cleanup_diagnostics": [],
                "term_sent": False,
                "kill_sent": False,
                "reaped": False,
                "spawn_error": None,
            }
            classification = {
                "state": "blocked",
                "reason_code": reason_code,
                "reason": reason,
                "observed_evidence": None,
            }
            ended_at = _utc_now()
            duration = 0.0
        elif artifact_prepare_error is not None:
            started_at = _utc_now()
            logs = _write_diagnostic_logs(log_dir, slug, artifact_prepare_error)
            process = {
                "spawned": False,
                "exit_code": None,
                "signal": None,
                "timed_out": False,
                "residual_processes_killed": False,
                "cleanup_complete": True,
                "cleanup_diagnostics": [],
                "term_sent": False,
                "kill_sent": False,
                "reaped": False,
                "spawn_error": artifact_prepare_error,
            }
            classification = classify_process(
                case=case,
                arch=arch,
                process=process,
                raw_log=Path(logs["raw"]),
                artifacts=captured_artifacts,
            )
            ended_at = _utc_now()
            duration = 0.0
        else:
            process_result = run_process(
                command=command,
                cwd=repo_root,
                environment=case_environment,
                timeout_seconds=case["timeout_seconds"],
                grace_seconds=float(runner["grace_seconds"]),
                combine_output=runner["combine_output"],
                log_dir=log_dir,
                slug=slug,
            )
            started_at = process_result.pop("started_at")
            ended_at = process_result.pop("ended_at")
            duration = process_result.pop("duration_seconds")
            raw_logs = process_result.pop("logs")
            logs = raw_logs
            process = process_result
            captured_artifacts = _capture_expected_artifacts(
                case, arch, repo_root, output_dir
            )
            classification = classify_process(
                case=case,
                arch=arch,
                process=process,
                raw_log=Path(logs["raw"]),
                artifacts=captured_artifacts,
            )
        record = {
                "case_id": case["id"],
                "title": case["title"],
                "category": case["category"],
                "architecture": arch,
                "policy": case["policy"],
                "target_evidence": case["evidence_level"],
                "observed_evidence": classification["observed_evidence"],
                "state": classification["state"],
                "reason_code": classification["reason_code"],
                "reason": classification["reason"],
                "command": _normalize_command(command, repo_root),
                "cwd": ".",
                "started_at": started_at,
                "ended_at": ended_at,
                "duration_seconds": duration,
                "process": process,
                "artifacts": captured_artifacts,
                "logs": {key: _log_reference(value, output_dir) for key, value in logs.items()},
                "provenance": list(case["provenance"]),
            }
        records.append(record)
        completed[(case["id"], arch)] = record

    repository_after = _repository_identity(repo_root)
    if repository_after != repository_before:
        reason = (
            "repository content changed while evidence commands were running "
            f"(before={repository_before['content_sha256']}, "
            f"after={repository_after['content_sha256']})"
        )
        for record in records:
            if not record["process"]["spawned"]:
                continue
            record["state"] = "error"
            record["reason_code"] = "repository_changed_during_run"
            record["reason"] = reason
            record["observed_evidence"] = None
    records.sort(key=lambda item: (item["case_id"], ARCHITECTURES.index(item["architecture"])))
    expected_instances = [f"{case['id']}@{arch}" for case, arch in selected]
    expected_instances.sort()
    full_instances = [f"{case['id']}@{arch}" for case, arch in _instance_records(manifest)]
    full_instances.sort()
    full_required_instances = [
        f"{case['id']}@{arch}"
        for case, arch in _instance_records(manifest)
        if case["policy"] == "required"
    ]
    full_required_instances.sort()
    result = {
        "schema_version": RESULT_SCHEMA_VERSION,
        "manifest": {
            "schema_version": manifest["schema_version"],
            "suite_id": manifest["suite_id"],
            "sha256": manifest_hash,
            "path": manifest_path.relative_to(repo_root).as_posix(),
        },
        "repository": repository_after,
        "tools": {
            capability["value"]: (
                _tool_identity(capability["value"], base_environment)
                if capability["kind"] == "tool"
                else _repository_executable_identity(
                    capability["value"], repo_root, base_environment
                )
            )
            for capability in sorted(
                {
                    capabilities[capability_id]["value"]: capabilities[capability_id]
                    for case, _arch in selected
                    for capability_id in case["requires"]
                    if capabilities[capability_id]["kind"] in {"tool", "executable"}
                }.values(),
                key=lambda item: item["value"],
            )
        },
        "run": {
            "started_at": run_started_at,
            "ended_at": _utc_now(),
            "duration_seconds": round(time.monotonic() - run_started, 6),
            "repository_before": repository_before,
            "selected_case_count": len(records),
            "expected_instances": expected_instances,
            "selection": {
                "requested_instances": requested_instances,
                "included_dependency_instances": sorted(
                    set(expected_instances) - set(requested_instances)
                ),
                "full_instance_count": len(full_instances),
                "full_required_instance_count": len(full_required_instances),
                "complete_suite": expected_instances == full_instances,
                "complete_required": sorted(
                    f"{record['case_id']}@{record['architecture']}"
                    for record in records
                    if record["policy"] == "required"
                ) == full_required_instances,
            },
        },
        "cases": records,
        "summary": _summarize_cases(records),
    }
    validate_result_document(result, bundle_root=output_dir)
    validate_result_against_manifest(
        result,
        manifest_path=manifest_path,
        repo_root=repo_root,
        bundle_root=output_dir,
        require_full_required=False,
    )
    result_path = output_dir / "semantic-evidence-v1.json"
    _atomic_write(result_path, canonical_json_bytes(result))
    required_nonpass = any(
        case["policy"] == "required" and case["state"] != "pass" for case in records
    )
    return result, 1 if required_nonpass else 0


def _merge_tool_identities(documents: list[dict[str, Any]]) -> dict[str, Any]:
    merged: dict[str, Any] = {}
    names = sorted({name for document in documents for name in document["tools"]})
    for name in names:
        identities = [
            document["tools"].get(name, {"path": None, "version": None, "sha256": None})
            for document in documents
        ]
        paths = {identity["path"] for identity in identities if identity["path"] is not None}
        versions = {
            identity["version"]
            for identity in identities
            if identity["version"] is not None
        }
        hashes = {
            identity["sha256"]
            for identity in identities
            if identity["sha256"] is not None
        }
        if len(paths) > 1 or len(versions) > 1 or len(hashes) > 1:
            raise ResultError(f"shard tool identity conflict for {name}")
        merged[name] = {
            "path": next(iter(paths), None),
            "version": next(iter(versions), None),
            "sha256": next(iter(hashes), None),
        }
    return merged


def _copy_bundle_reference(
    reference: dict[str, Any], source_root: Path, output_dir: Path
) -> None:
    relative = reference["path"]
    source = source_root / relative
    destination = output_dir / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination.exists():
        if file_size_sha256(destination) != file_size_sha256(source):
            raise ResultError(f"shard bundle path collision: {relative}")
        return
    shutil.copyfile(source, destination)


def merge_result_shards(
    *,
    shard_paths: list[Path],
    manifest_path: Path,
    output_dir: Path,
    repo_root: Path,
) -> tuple[dict[str, Any], int]:
    ensure_safe_output_directory(output_dir, repo_root)
    resolved_shards = [path.resolve() for path in shard_paths]
    output_root = output_dir.resolve()
    for shard in resolved_shards:
        try:
            shard.relative_to(output_root)
        except ValueError:
            continue
        raise ResultError("merge output directory must not contain an input shard")

    output_dir.mkdir(parents=True, exist_ok=True)
    (output_dir / "semantic-evidence-v1.json").unlink(missing_ok=True)
    for owned_directory in (
        output_dir / "logs",
        output_dir / "artifacts",
        output_dir / "reports",
    ):
        if owned_directory.exists():
            shutil.rmtree(owned_directory)
    if len(shard_paths) < 2:
        raise ResultError("merge requires at least two shard results")
    if len(set(resolved_shards)) != len(resolved_shards):
        raise ResultError("merge shard paths must be unique")

    loaded = [
        (
            path,
            load_validate_result_with_manifest(
                path,
                manifest_path=manifest_path,
                repo_root=repo_root,
                require_full_required=False,
            ),
        )
        for path in resolved_shards
    ]
    loaded.sort(key=lambda item: tuple(item[1]["run"]["expected_instances"]))
    documents = [document for _, document in loaded]
    manifest_identity = documents[0]["manifest"]
    repository_identity = documents[0]["repository"]
    for document in documents[1:]:
        if document["manifest"] != manifest_identity:
            raise ResultError("shard manifest identities do not match")
        if document["repository"] != repository_identity:
            raise ResultError("shard repository identities do not match")
        if document["run"]["repository_before"] != repository_identity:
            raise ResultError("cannot merge a shard whose repository changed during execution")
    if documents[0]["run"]["repository_before"] != repository_identity:
        raise ResultError("cannot merge a shard whose repository changed during execution")
    if repository_identity["revision"] == "unknown":
        raise ResultError("cannot merge shards with unknown repository revision")

    records: list[dict[str, Any]] = []
    seen: set[tuple[str, str]] = set()
    for shard_path, document in loaded:
        for original in document["cases"]:
            key = (original["case_id"], original["architecture"])
            if key in seen:
                raise ResultError(f"duplicate shard instance: {key[0]}@{key[1]}")
            seen.add(key)
            records.append(copy.deepcopy(original))

    manifest, _ = load_and_validate_manifest(manifest_path, repo_root)
    full_instances = {
        (case["id"], arch) for case, arch in _instance_records(manifest)
    }
    full_required = {
        (case["id"], arch)
        for case, arch in _instance_records(manifest)
        if case["policy"] == "required"
    }
    missing_required = sorted(
        f"{case_id}@{arch}" for case_id, arch in full_required - seen
    )
    unexpected = sorted(f"{case_id}@{arch}" for case_id, arch in seen - full_instances)
    if missing_required:
        raise ResultError("required shard instances are missing: " + ", ".join(missing_required))
    if unexpected:
        raise ResultError("shards contain unexpected instances: " + ", ".join(unexpected))

    for shard_path, document in loaded:
        for record in document["cases"]:
            for reference in record["logs"].values():
                if reference is not None:
                    _copy_bundle_reference(reference, shard_path.parent, output_dir)
            for reference in record["artifacts"]:
                _copy_bundle_reference(reference, shard_path.parent, output_dir)

    records.sort(key=lambda item: (item["case_id"], ARCHITECTURES.index(item["architecture"])))
    expected_instances = sorted(f"{case_id}@{arch}" for case_id, arch in seen)
    started_at = min(document["run"]["started_at"] for document in documents)
    ended_at = max(document["run"]["ended_at"] for document in documents)
    result = {
        "schema_version": RESULT_SCHEMA_VERSION,
        "manifest": manifest_identity,
        "repository": repository_identity,
        "tools": _merge_tool_identities(documents),
        "run": {
            "started_at": started_at,
            "ended_at": ended_at,
            "duration_seconds": round(
                sum(float(document["run"]["duration_seconds"]) for document in documents),
                6,
            ),
            "repository_before": repository_identity,
            "selected_case_count": len(records),
            "expected_instances": expected_instances,
            "selection": {
                "requested_instances": expected_instances,
                "included_dependency_instances": [],
                "full_instance_count": len(full_instances),
                "full_required_instance_count": len(full_required),
                "complete_suite": seen == full_instances,
                "complete_required": (seen & full_required) == full_required,
            },
        },
        "cases": records,
        "summary": _summarize_cases(records),
    }
    validate_result_document(result, bundle_root=output_dir)
    validate_result_against_manifest(
        result,
        manifest_path=manifest_path,
        repo_root=repo_root,
        bundle_root=output_dir,
        require_full_required=True,
    )
    _atomic_write(output_dir / "semantic-evidence-v1.json", canonical_json_bytes(result))
    return result, 1 if result["summary"]["required_nonpass"] else 0


def _result_check_keys(
    obj: dict[str, Any], required: Iterable[str], optional: Iterable[str], where: str
) -> None:
    required_set = set(required)
    allowed = required_set | set(optional)
    missing = sorted(required_set - obj.keys())
    unknown = sorted(obj.keys() - allowed)
    if missing:
        raise ResultError(f"{where} missing fields: {', '.join(missing)}")
    if unknown:
        raise ResultError(f"{where} has unknown fields: {', '.join(unknown)}")


def _result_string(value: Any, where: str) -> str:
    if not isinstance(value, str) or not value:
        raise ResultError(f"{where} must be a non-empty string")
    return value


def _result_number(value: Any, where: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ResultError(f"{where} must be numeric")
    number = float(value)
    if not math.isfinite(number) or number < 0:
        raise ResultError(f"{where} must be finite and non-negative")
    return number


def _result_timestamp(value: Any, where: str) -> dt.datetime:
    text = _result_string(value, where)
    if not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z", text):
        raise ResultError(f"{where} must be a millisecond UTC timestamp")
    try:
        return dt.datetime.strptime(text, "%Y-%m-%dT%H:%M:%S.%fZ")
    except ValueError as exc:
        raise ResultError(f"{where} must be a valid millisecond UTC timestamp") from exc


def _validate_repository_identity(value: Any, where: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ResultError(f"{where} must be an object")
    _result_check_keys(
        value,
        ("revision", "dirty", "content_sha256"),
        (),
        where,
    )
    revision = _result_string(value["revision"], f"{where}.revision")
    if revision != "unknown" and not re.fullmatch(r"[0-9a-f]{40}", revision):
        raise ResultError(f"{where}.revision must be a Git SHA-1 or unknown")
    content_sha256 = _result_string(
        value["content_sha256"], f"{where}.content_sha256"
    )
    if content_sha256 != "unknown" and not re.fullmatch(
        r"[0-9a-f]{64}", content_sha256
    ):
        raise ResultError(f"{where}.content_sha256 must be SHA-256 or unknown")
    if not isinstance(value["dirty"], bool):
        raise ResultError(f"{where}.dirty must be boolean")
    return value


def _validate_process(process: Any, state: str, where: str) -> dict[str, Any]:
    if not isinstance(process, dict):
        raise ResultError(f"{where} must be an object")
    fields = (
        "spawned",
        "exit_code",
        "signal",
        "timed_out",
        "residual_processes_killed",
        "cleanup_complete",
        "cleanup_diagnostics",
        "term_sent",
        "kill_sent",
        "reaped",
        "spawn_error",
    )
    _result_check_keys(process, fields, (), where)
    for name in (
        "spawned",
        "timed_out",
        "residual_processes_killed",
        "cleanup_complete",
        "term_sent",
        "kill_sent",
        "reaped",
    ):
        if not isinstance(process[name], bool):
            raise ResultError(f"{where}.{name} must be boolean")
    for name in ("exit_code", "signal"):
        value = process[name]
        if value is not None and (isinstance(value, bool) or not isinstance(value, int)):
            raise ResultError(f"{where}.{name} must be an integer or null")
    if process["signal"] is not None and process["signal"] <= 0:
        raise ResultError(f"{where}.signal must be positive")
    if process["exit_code"] is not None and process["exit_code"] < 0:
        if process["signal"] != -process["exit_code"]:
            raise ResultError(f"{where}.signal must match the negative exit status")
    elif process["signal"] is not None:
        raise ResultError(f"{where}.signal requires a negative exit status")
    if not isinstance(process["cleanup_diagnostics"], list) or not all(
        isinstance(item, str) and item for item in process["cleanup_diagnostics"]
    ):
        raise ResultError(f"{where}.cleanup_diagnostics must be an array of strings")
    if process["spawn_error"] is not None and not isinstance(process["spawn_error"], str):
        raise ResultError(f"{where}.spawn_error must be a string or null")

    if not process["spawned"]:
        forbidden = (
            process["exit_code"] is not None
            or process["signal"] is not None
            or process["timed_out"]
            or process["residual_processes_killed"]
            or process["term_sent"]
            or process["kill_sent"]
            or process["reaped"]
        )
        if forbidden:
            raise ResultError(f"{where} has impossible fields for an unspawned process")
        if state not in {"blocked", "skipped", "error"}:
            raise ResultError(f"{where} unspawned process cannot produce state {state}")
        if state == "error" and not process["spawn_error"]:
            raise ResultError(f"{where} unspawned error requires spawn_error")
        if state in {"blocked", "skipped"} and process["spawn_error"] is not None:
            raise ResultError(f"{where} blocked/skipped process cannot have spawn_error")
    else:
        if process["spawn_error"] is not None:
            raise ResultError(f"{where} spawned process cannot have spawn_error")
        if process["exit_code"] is None and state not in {"error"}:
            raise ResultError(f"{where} spawned process requires an exit status")
        if not process["reaped"] and state != "error":
            raise ResultError(f"{where} unreaped process must be an error")

    if process["timed_out"] and state not in {"timeout", "error"}:
        raise ResultError(f"{where} timed-out process cannot produce state {state}")
    if process["residual_processes_killed"] and state != "error":
        raise ResultError(f"{where} residual processes must produce error")
    if not process["cleanup_complete"] and state != "error":
        raise ResultError(f"{where} incomplete cleanup must produce error")
    if state == "pass" and (
        not process["spawned"]
        or process["exit_code"] != 0
        or process["timed_out"]
        or process["residual_processes_killed"]
        or not process["cleanup_complete"]
        or not process["reaped"]
    ):
        raise ResultError(f"{where} process state cannot support pass")
    return process


def validate_result_document(
    document: Any, *, bundle_root: Path | None = None
) -> dict[str, Any]:
    if not isinstance(document, dict):
        raise ResultError("result must be an object")
    _result_check_keys(
        document,
        ("schema_version", "manifest", "repository", "tools", "run", "cases", "summary"),
        (),
        "result",
    )
    if (
        isinstance(document["schema_version"], bool)
        or not isinstance(document["schema_version"], int)
        or document["schema_version"] != RESULT_SCHEMA_VERSION
    ):
        raise ResultError(f"unsupported result schema_version {document['schema_version']}")
    manifest = document["manifest"]
    if not isinstance(manifest, dict):
        raise ResultError("result.manifest must be an object")
    _result_check_keys(
        manifest, ("schema_version", "suite_id", "sha256", "path"), (), "result.manifest"
    )
    if (
        isinstance(manifest["schema_version"], bool)
        or not isinstance(manifest["schema_version"], int)
        or manifest["schema_version"] != MANIFEST_SCHEMA_VERSION
    ):
        raise ResultError("result manifest schema version mismatch")
    _result_string(manifest["suite_id"], "result.manifest.suite_id")
    if not re.fullmatch(r"[0-9a-f]{64}", str(manifest["sha256"])):
        raise ResultError("result.manifest.sha256 must be lowercase SHA-256")
    manifest_path = _result_string(manifest["path"], "result.manifest.path")
    if Path(manifest_path).is_absolute() or ".." in Path(manifest_path).parts:
        raise ResultError("result.manifest.path must be repository-relative")
    repository = _validate_repository_identity(document["repository"], "result.repository")
    if not isinstance(document["tools"], dict):
        raise ResultError("result.tools must be an object")
    for name, identity in document["tools"].items():
        if not isinstance(name, str) or not name:
            raise ResultError("result.tools keys must be non-empty strings")
        if not isinstance(identity, dict):
            raise ResultError(f"result.tools.{name} must be an object")
        _result_check_keys(identity, ("path", "version", "sha256"), (), f"result.tools.{name}")
        for field in ("path", "version"):
            if identity[field] is not None and not isinstance(identity[field], str):
                raise ResultError(f"result.tools.{name}.{field} must be a string or null")
        tool_path = identity["path"]
        if tool_path is not None and (
            not tool_path
            or Path(tool_path).is_absolute()
            or ".." in Path(tool_path).parts
        ):
            raise ResultError(
                f"result.tools.{name}.path must be a normalized relative logical name or null"
            )
        if identity["sha256"] is not None and not re.fullmatch(
            r"[0-9a-f]{64}", str(identity["sha256"])
        ):
            raise ResultError(f"result.tools.{name}.sha256 must be SHA-256 or null")
    if not isinstance(document["run"], dict):
        raise ResultError("result.run must be an object")
    run = document["run"]
    _result_check_keys(
        run,
        (
            "started_at",
            "ended_at",
            "duration_seconds",
            "repository_before",
            "selected_case_count",
            "expected_instances",
            "selection",
        ),
        (),
        "result.run",
    )
    run_started = _result_timestamp(run["started_at"], "result.run.started_at")
    run_ended = _result_timestamp(run["ended_at"], "result.run.ended_at")
    if run_ended < run_started:
        raise ResultError("result.run.ended_at must not precede started_at")
    _result_number(run["duration_seconds"], "result.run.duration_seconds")
    _validate_repository_identity(
        run["repository_before"], "result.run.repository_before"
    )
    if isinstance(run["selected_case_count"], bool) or not isinstance(
        run["selected_case_count"], int
    ):
        raise ResultError("result.run.selected_case_count must be an integer")
    if not isinstance(run["expected_instances"], list) or not all(
        isinstance(item, str) and item for item in run["expected_instances"]
    ):
        raise ResultError("result.run.expected_instances must be an array of strings")
    if run["expected_instances"] != sorted(set(run["expected_instances"])):
        raise ResultError("result.run.expected_instances must be unique and sorted")
    selection = run["selection"]
    if not isinstance(selection, dict):
        raise ResultError("result.run.selection must be an object")
    _result_check_keys(
        selection,
        (
            "requested_instances",
            "included_dependency_instances",
            "full_instance_count",
            "full_required_instance_count",
            "complete_suite",
            "complete_required",
        ),
        (),
        "result.run.selection",
    )
    for name in ("requested_instances", "included_dependency_instances"):
        value = selection[name]
        if not isinstance(value, list) or not all(isinstance(item, str) and item for item in value):
            raise ResultError(f"result.run.selection.{name} must be an array of strings")
        if value != sorted(set(value)):
            raise ResultError(f"result.run.selection.{name} must be unique and sorted")
    if set(selection["requested_instances"]) & set(selection["included_dependency_instances"]):
        raise ResultError("result.run.selection requested and dependency sets overlap")
    if sorted(
        [*selection["requested_instances"], *selection["included_dependency_instances"]]
    ) != run["expected_instances"]:
        raise ResultError("result.run.selection does not partition expected_instances")
    for name in ("full_instance_count", "full_required_instance_count"):
        value = selection[name]
        if isinstance(value, bool) or not isinstance(value, int) or value < 0:
            raise ResultError(f"result.run.selection.{name} must be a non-negative integer")
    for name in ("complete_suite", "complete_required"):
        if not isinstance(selection[name], bool):
            raise ResultError(f"result.run.selection.{name} must be boolean")
    cases = document["cases"]
    if not isinstance(cases, list) or not cases:
        raise ResultError("result.cases must be a non-empty array")
    required_case_keys = (
        "case_id",
        "title",
        "category",
        "architecture",
        "policy",
        "target_evidence",
        "observed_evidence",
        "state",
        "reason_code",
        "reason",
        "command",
        "cwd",
        "started_at",
        "ended_at",
        "duration_seconds",
        "process",
        "artifacts",
        "logs",
        "provenance",
    )
    seen: set[tuple[str, str]] = set()
    order: list[tuple[str, int]] = []
    for index, raw in enumerate(cases):
        where = f"result.cases[{index}]"
        if not isinstance(raw, dict):
            raise ResultError(f"{where} must be an object")
        _result_check_keys(raw, required_case_keys, (), where)
        case_id = raw["case_id"]
        arch = raw["architecture"]
        if not isinstance(case_id, str) or not re.fullmatch(r"[a-z0-9][a-z0-9._-]*", case_id):
            raise ResultError(f"{where}.case_id is invalid")
        if arch not in ARCHITECTURES:
            raise ResultError(f"{where}.architecture is invalid: {arch}")
        key = (case_id, arch)
        if key in seen:
            raise ResultError(f"duplicate result case: {case_id}/{arch}")
        seen.add(key)
        order.append((case_id, ARCHITECTURES.index(arch)))
        if raw["policy"] not in POLICIES:
            raise ResultError(f"{where}.policy is invalid")
        if raw["target_evidence"] not in EVIDENCE_LEVELS:
            raise ResultError(f"{where}.target_evidence is invalid")
        observed = raw["observed_evidence"]
        if observed is not None and observed not in EVIDENCE_LEVELS:
            raise ResultError(f"{where}.observed_evidence is invalid")
        if observed is not None and _evidence_rank(observed) > _evidence_rank(raw["target_evidence"]):
            raise ResultError(f"{where} overclaims observed evidence")
        if raw["state"] not in RESULT_STATES:
            raise ResultError(f"{where}.state is invalid")
        if raw["state"] == "pass" and observed != raw["target_evidence"]:
            raise ResultError(f"{where} pass must establish target evidence")
        if raw["state"] in {"blocked", "skipped"} and observed is not None:
            raise ResultError(f"{where} blocked/skipped state cannot claim observed evidence")
        for name in ("title", "category", "reason_code", "reason", "cwd"):
            _result_string(raw[name], f"{where}.{name}")
        if not re.fullmatch(r"[a-z0-9][a-z0-9._-]*", raw["reason_code"]):
            raise ResultError(f"{where}.reason_code is invalid")
        case_started = _result_timestamp(raw["started_at"], f"{where}.started_at")
        case_ended = _result_timestamp(raw["ended_at"], f"{where}.ended_at")
        if case_ended < case_started:
            raise ResultError(f"{where}.ended_at must not precede started_at")
        if case_started < run_started or case_ended > run_ended:
            raise ResultError(f"{where} timestamps must be contained by result.run")
        if not isinstance(raw["command"], list) or not all(
            isinstance(item, str) and item for item in raw["command"]
        ):
            raise ResultError(f"{where}.command must be a non-empty argv array")
        _result_number(raw["duration_seconds"], f"{where}.duration_seconds")
        provenance = raw["provenance"]
        if not isinstance(provenance, list) or not provenance:
            raise ResultError(f"{where}.provenance must be a non-empty array")
        if provenance != sorted(set(provenance)):
            raise ResultError(f"{where}.provenance must be unique and sorted")
        for item in provenance:
            if not isinstance(item, str) or not item or Path(item).is_absolute() or ".." in Path(item).parts:
                raise ResultError(f"{where}.provenance contains an invalid repository path")
        artifacts = raw["artifacts"]
        if not isinstance(artifacts, list):
            raise ResultError(f"{where}.artifacts must be an array")
        artifact_sources: list[str] = []
        for artifact_index, artifact in enumerate(artifacts):
            artifact_where = f"{where}.artifacts[{artifact_index}]"
            if not isinstance(artifact, dict):
                raise ResultError(f"{artifact_where} must be an object")
            _result_check_keys(
                artifact,
                ("source", "path", "size_bytes", "sha256"),
                (),
                artifact_where,
            )
            source = _result_string(artifact["source"], f"{artifact_where}.source")
            if Path(source).is_absolute() or ".." in Path(source).parts:
                raise ResultError(f"{artifact_where}.source must be repository-relative")
            artifact_sources.append(source)
            artifact_path = _result_string(artifact["path"], f"{artifact_where}.path")
            if (
                Path(artifact_path).is_absolute()
                or ".." in Path(artifact_path).parts
                or not artifact_path.startswith("artifacts/")
                or not re.fullmatch(r"[A-Za-z0-9._/-]+", artifact_path)
            ):
                raise ResultError(f"{artifact_where}.path must be a bundle artifact path")
            if (
                isinstance(artifact["size_bytes"], bool)
                or not isinstance(artifact["size_bytes"], int)
                or artifact["size_bytes"] <= 0
            ):
                raise ResultError(f"{artifact_where}.size_bytes must be positive")
            if not re.fullmatch(r"[0-9a-f]{64}", str(artifact["sha256"])):
                raise ResultError(f"{artifact_where}.sha256 is invalid")
            if bundle_root is not None:
                root = bundle_root.resolve()
                candidate = (root / artifact_path).resolve()
                try:
                    candidate.relative_to(root)
                except ValueError as exc:
                    raise ResultError(f"{artifact_where}.path escapes bundle") from exc
                if not candidate.is_file():
                    raise ResultError(f"{artifact_where}.path does not exist")
                actual_size, actual_sha256 = file_size_sha256(candidate)
                if actual_size != artifact["size_bytes"]:
                    raise ResultError(f"{artifact_where}.size_bytes mismatch")
                if actual_sha256 != artifact["sha256"]:
                    raise ResultError(f"{artifact_where}.sha256 mismatch")
        if artifact_sources != sorted(set(artifact_sources)):
            raise ResultError(f"{where}.artifacts must be unique and source-sorted")
        logs = raw["logs"]
        if not isinstance(logs, dict) or set(logs) != {"stdout", "stderr", "raw"}:
            raise ResultError(f"{where}.logs must contain stdout, stderr, raw")
        for name in ("stdout", "stderr", "raw"):
            value = logs[name]
            if value is None:
                if name == "raw":
                    raise ResultError(f"{where}.logs.raw must not be null")
                continue
            if not isinstance(value, dict) or set(value) != {"path", "size_bytes", "sha256"}:
                raise ResultError(
                    f"{where}.logs.{name} must contain path, size_bytes, sha256"
                )
            log_path = value["path"]
            if (
                not isinstance(log_path, str)
                or not log_path
                or Path(log_path).is_absolute()
                or ".." in Path(log_path).parts
                or not log_path.startswith("logs/")
                or not re.fullmatch(r"[A-Za-z0-9._/-]+", log_path)
            ):
                raise ResultError(f"{where}.logs.{name}.path must be a relative path")
            if (
                isinstance(value["size_bytes"], bool)
                or not isinstance(value["size_bytes"], int)
                or value["size_bytes"] < 0
            ):
                raise ResultError(f"{where}.logs.{name}.size_bytes is invalid")
            if not re.fullmatch(r"[0-9a-f]{64}", str(value["sha256"])):
                raise ResultError(f"{where}.logs.{name}.sha256 is invalid")
            if bundle_root is not None:
                root = bundle_root.resolve()
                candidate = (root / log_path).resolve()
                try:
                    candidate.relative_to(root)
                except ValueError as exc:
                    raise ResultError(f"{where}.logs.{name}.path escapes bundle") from exc
                if not candidate.is_file():
                    raise ResultError(f"{where}.logs.{name}.path does not exist: {log_path}")
                actual_size, actual_sha256 = file_size_sha256(candidate)
                if actual_size != value["size_bytes"]:
                    raise ResultError(f"{where}.logs.{name}.size_bytes mismatch")
                if actual_sha256 != value["sha256"]:
                    raise ResultError(f"{where}.logs.{name}.sha256 mismatch")
        _validate_process(raw["process"], raw["state"], f"{where}.process")
    if order != sorted(order):
        raise ResultError("result.cases must use deterministic case/architecture ordering")
    actual_instances = sorted(f"{case['case_id']}@{case['architecture']}" for case in cases)
    if actual_instances != run["expected_instances"]:
        raise ResultError("result cases do not exactly cover run.expected_instances")
    if run["selected_case_count"] != len(cases):
        raise ResultError("result.run.selected_case_count mismatch")
    summary = document["summary"]
    if not isinstance(summary, dict):
        raise ResultError("result.summary must be an object")
    _result_check_keys(
        summary,
        ("total", "states", "policies", "required_nonpass"),
        (),
        "result.summary",
    )
    for name in ("total", "required_nonpass"):
        value = summary[name]
        if isinstance(value, bool) or not isinstance(value, int) or value < 0:
            raise ResultError(f"result.summary.{name} must be a non-negative integer")
    for name, expected_keys in (("states", RESULT_STATES), ("policies", POLICIES)):
        counts = summary[name]
        if not isinstance(counts, dict) or set(counts) != set(expected_keys):
            raise ResultError(
                f"result.summary.{name} must contain exactly the canonical keys"
            )
        for key in expected_keys:
            value = counts[key]
            if isinstance(value, bool) or not isinstance(value, int) or value < 0:
                raise ResultError(
                    f"result.summary.{name}.{key} must be a non-negative integer"
                )
    expected_summary = _summarize_cases(cases)
    if summary != expected_summary:
        raise ResultError("result.summary does not match case records")
    return document


def load_and_validate_result(path: Path) -> dict[str, Any]:
    try:
        document = strict_json_load(path)
    except JsonInputError as exc:
        raise ResultError(str(exc)) from exc
    return validate_result_document(document, bundle_root=path.parent)


def validate_result_against_manifest(
    document: dict[str, Any],
    *,
    manifest_path: Path,
    repo_root: Path,
    bundle_root: Path,
    require_full_required: bool,
) -> dict[str, Any]:
    manifest, manifest_hash = load_and_validate_manifest(manifest_path, repo_root)
    manifest_ref = document["manifest"]
    try:
        expected_manifest_path = manifest_path.resolve().relative_to(repo_root.resolve()).as_posix()
    except ValueError as exc:
        raise ResultError("manifest path is outside the repository") from exc
    expected_manifest_identity = {
        "schema_version": manifest["schema_version"],
        "suite_id": manifest["suite_id"],
        "sha256": manifest_hash,
        "path": expected_manifest_path,
    }
    if manifest_ref != expected_manifest_identity:
        raise ResultError("result manifest identity does not match the effective manifest")

    current_repository = _repository_identity(repo_root)
    repository = document["repository"]
    if repository["revision"] == "unknown":
        raise ResultError("manifest-aware evidence requires a known repository revision")
    if current_repository["revision"] == "unknown":
        raise ResultError("cannot determine the current repository revision")
    if repository["revision"] != current_repository["revision"]:
        raise ResultError(
            "result repository revision does not match the current checkout"
        )
    if repository["content_sha256"] == "unknown" or current_repository["content_sha256"] == "unknown":
        raise ResultError("cannot establish repository content identity")
    if repository["content_sha256"] != current_repository["content_sha256"]:
        raise ResultError("result repository content does not match the current checkout")
    repository_before = document["run"]["repository_before"]
    if repository_before["revision"] == "unknown":
        raise ResultError(
            "manifest-aware evidence requires a known pre-execution repository revision"
        )
    if repository_before["content_sha256"] == "unknown":
        raise ResultError("cannot establish pre-execution repository content identity")
    repository_changed = repository_before != repository
    repository_changed_reason = (
        "repository content changed while evidence commands were running "
        f"(before={repository_before['content_sha256']}, "
        f"after={repository['content_sha256']})"
    )
    if require_full_required and os.environ.get("CI", "").lower() == "true":
        if repository["dirty"] or current_repository["dirty"]:
            raise ResultError("CI required evidence must come from a clean checkout")

    case_by_id = {case["id"]: case for case in manifest["expanded_cases"]}
    result_by_instance = {
        (case["case_id"], case["architecture"]): case for case in document["cases"]
    }
    full_instances = {
        (case["id"], arch)
        for case, arch in _instance_records(manifest)
    }
    full_required = {
        (case["id"], arch)
        for case, arch in _instance_records(manifest)
        if case["policy"] == "required"
    }

    def parse_instance(value: str, where: str) -> tuple[str, str]:
        if "@" not in value:
            raise ResultError(f"{where} has invalid instance id: {value}")
        case_id, arch = value.rsplit("@", 1)
        key = (case_id, arch)
        if key not in full_instances:
            raise ResultError(f"{where} references unknown manifest instance: {value}")
        return key

    selection = document["run"]["selection"]
    requested = {
        parse_instance(item, "result.run.selection.requested_instances")
        for item in selection["requested_instances"]
    }
    included_dependencies = {
        parse_instance(item, "result.run.selection.included_dependency_instances")
        for item in selection["included_dependency_instances"]
    }
    selected = set(result_by_instance)
    if selected != requested | included_dependencies:
        raise ResultError("result selection does not match selected manifest instances")

    closure: set[tuple[str, str]] = set()

    def include_dependencies(case_id: str, arch: str) -> None:
        key = (case_id, arch)
        if key in closure:
            return
        case = case_by_id[case_id]
        for dependency_id in case["depends_on"]:
            dependency = case_by_id[dependency_id]
            include_dependencies(dependency_id, _dependency_arch(dependency, arch))
        closure.add(key)

    for case_id, arch in requested:
        include_dependencies(case_id, arch)
    if closure != selected:
        raise ResultError("result selection is not the exact dependency closure of requested cases")
    if selection["full_instance_count"] != len(full_instances):
        raise ResultError("result.run.selection.full_instance_count mismatch")
    if selection["full_required_instance_count"] != len(full_required):
        raise ResultError("result.run.selection.full_required_instance_count mismatch")
    if selection["complete_suite"] != (selected == full_instances):
        raise ResultError("result.run.selection.complete_suite mismatch")
    selected_required = selected & full_required
    if selection["complete_required"] != (selected_required == full_required):
        raise ResultError("result.run.selection.complete_required mismatch")
    if require_full_required and selected_required != full_required:
        missing = sorted(f"{case_id}@{arch}" for case_id, arch in full_required - selected)
        raise ResultError(
            "result does not cover every required manifest instance: " + ", ".join(missing)
        )

    for key, record in result_by_instance.items():
        case_id, arch = key
        manifest_case = case_by_id.get(case_id)
        if manifest_case is None or arch not in manifest_case["architectures"]:
            raise ResultError(f"result contains unknown manifest instance: {case_id}@{arch}")
        expected_static = {
            "title": manifest_case["title"],
            "category": manifest_case["category"],
            "policy": manifest_case["policy"],
            "target_evidence": manifest_case["evidence_level"],
            "command": _normalize_command(
                [_substitute_arch(token, arch) for token in manifest_case["command"]],
                repo_root,
            ),
            "cwd": ".",
            "provenance": manifest_case["provenance"],
        }
        for field, expected in expected_static.items():
            if record[field] != expected:
                raise ResultError(
                    f"result case {case_id}@{arch} field {field} does not match manifest"
                )

        if record["state"] == "pass":
            for capability_id in manifest_case["requires"]:
                capability = _capability_map(manifest)[capability_id]
                if capability["kind"] not in {"tool", "executable"}:
                    continue
                identity = document["tools"].get(capability["value"])
                if (
                    not isinstance(identity, dict)
                    or not identity.get("path")
                    or not identity.get("version")
                    or not identity.get("sha256")
                ):
                    raise ResultError(
                        f"passing case {case_id}@{arch} lacks tool identity for "
                        f"{capability['value']}"
                    )
                required_version = capability.get("required_version")
                if (
                    required_version is not None
                    and identity["version"] != required_version
                ):
                    raise ResultError(
                        f"passing case {case_id}@{arch} tool version mismatch for "
                        f"{capability['value']}: expected {required_version!r}, "
                        f"observed {identity['version']!r}"
                    )

        declared_artifacts = manifest_case.get("artifacts", [])
        captured_sources = [artifact["source"] for artifact in record["artifacts"]]
        if record["state"] == "pass":
            artifacts_match = captured_sources == declared_artifacts
        else:
            captured_set = set(captured_sources)
            artifacts_match = (
                len(captured_set) == len(captured_sources)
                and captured_set.issubset(declared_artifacts)
                and captured_sources
                == [source for source in declared_artifacts if source in captured_set]
            )
        if not artifacts_match:
            raise ResultError(
                f"result case {case_id}@{arch} artifacts do not match manifest"
            )

        repository_change_override = (
            record["reason_code"] == "repository_changed_during_run"
        )
        if repository_change_override:
            if not repository_changed:
                raise ResultError(
                    f"result case {case_id}@{arch} claims a repository change "
                    "without different before/after identities"
                )
            if not record["process"]["spawned"]:
                raise ResultError(
                    f"result case {case_id}@{arch} applies the repository change "
                    "override to an unspawned process"
                )
            expected_override = {
                "state": "error",
                "reason": repository_changed_reason,
                "observed_evidence": None,
            }
            for field, expected in expected_override.items():
                if record[field] != expected:
                    raise ResultError(
                        f"result case {case_id}@{arch} has an invalid repository "
                        f"change override field: {field}"
                    )
            continue
        if repository_changed and record["process"]["spawned"]:
            raise ResultError(
                f"result case {case_id}@{arch} does not fail closed after the "
                "repository changed during execution"
            )

        if record["state"] == "blocked":
            if record["reason_code"] not in {"missing_prerequisite", "dependency_nonpass"}:
                raise ResultError(
                    f"result case {case_id}@{arch} has unsupported blocked reason"
                )
            if record["reason_code"] == "dependency_nonpass":
                nonpass_dependencies = []
                for dependency_id in manifest_case["depends_on"]:
                    dependency = case_by_id[dependency_id]
                    dependency_arch = _dependency_arch(dependency, arch)
                    dependency_record = result_by_instance.get((dependency_id, dependency_arch))
                    if dependency_record is None or dependency_record["state"] != "pass":
                        nonpass_dependencies.append(dependency_id)
                if not nonpass_dependencies:
                    raise ResultError(
                        f"result case {case_id}@{arch} claims dependency_nonpass without one"
                    )
            continue
        if record["state"] == "skipped":
            if record["policy"] == "required":
                raise ResultError(f"required result case {case_id}@{arch} cannot be skipped")
            continue

        raw_path = bundle_root / record["logs"]["raw"]["path"]
        recalculated = classify_process(
            case=manifest_case,
            arch=arch,
            process=record["process"],
            raw_log=raw_path,
            artifacts=record["artifacts"],
        )
        for field in ("state", "reason_code", "reason", "observed_evidence"):
            if record[field] != recalculated[field]:
                raise ResultError(
                    f"result case {case_id}@{arch} {field} disagrees with raw evidence"
                )
    return document


def load_validate_result_with_manifest(
    path: Path,
    *,
    manifest_path: Path,
    repo_root: Path,
    require_full_required: bool,
) -> dict[str, Any]:
    document = load_and_validate_result(path)
    return validate_result_against_manifest(
        document,
        manifest_path=manifest_path,
        repo_root=repo_root,
        bundle_root=path.parent,
        require_full_required=require_full_required,
    )


def supervise_command(command: list[str], timeout: int, log: Path, cwd: Path) -> int:
    result = run_process(
        command=command,
        cwd=cwd,
        environment=dict(os.environ),
        timeout_seconds=timeout,
        grace_seconds=DEFAULT_GRACE_SECONDS,
        combine_output=True,
        log_dir=log.parent,
        slug=log.stem,
    )
    produced = Path(result["logs"]["raw"])
    if produced != log:
        log.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(produced, log)
    try:
        with log.open("rb") as stream:
            shutil.copyfileobj(stream, sys.stdout.buffer)
    except OSError:
        pass
    if not result["cleanup_complete"]:
        return 125
    if result["timed_out"]:
        return 124
    if result["residual_processes_killed"]:
        return 125
    if not result["spawned"]:
        return 126
    return int(result["exit_code"] or 0)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command_name", required=True)

    validate = sub.add_parser("validate", help="validate and expand the manifest")
    validate.add_argument("--manifest", type=Path, required=True)

    schema = sub.add_parser("schema", help="write or check the generated v1 schema")
    group = schema.add_mutually_exclusive_group(required=True)
    group.add_argument("--write", type=Path)
    group.add_argument("--check", type=Path)

    run = sub.add_parser("run", help="execute selected manifest cases")
    run.add_argument("--manifest", type=Path, required=True)
    run.add_argument("--output", type=Path, required=True)
    run.add_argument("--case", action="append", default=[])
    run.add_argument("--policy", choices=POLICIES)
    run.add_argument("--category")
    run.add_argument("--arch", choices=ARCHITECTURES)

    merge = sub.add_parser("merge", help="merge manifest-aware result shards")
    merge.add_argument("--manifest", type=Path, required=True)
    merge.add_argument("--output", type=Path, required=True)
    merge.add_argument("--shard", type=Path, action="append", required=True)

    result = sub.add_parser("validate-result", help="validate canonical result JSON")
    result.add_argument("path", type=Path)
    result.add_argument("--manifest", type=Path, required=True)
    result.add_argument("--require-full-required", action="store_true")

    supervise = sub.add_parser("supervise", help="run a command with timeout and process cleanup")
    supervise.add_argument("--timeout", type=int, required=True)
    supervise.add_argument("--log", type=Path, required=True)
    supervise.add_argument("argv", nargs=argparse.REMAINDER)
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    repo_root = Path(__file__).resolve().parent.parent
    for termination_signal in (signal.SIGHUP, signal.SIGINT, signal.SIGTERM):
        signal.signal(
            termination_signal,
            lambda signum, _frame: (_ for _ in ()).throw(TerminationRequested(signum)),
        )
    try:
        if args.command_name == "validate":
            manifest, digest = load_and_validate_manifest(args.manifest.resolve(), repo_root)
            print(
                f"manifest ok: schema={manifest['schema_version']} "
                f"cases={len(manifest['expanded_cases'])} sha256={digest}"
            )
            return 0
        if args.command_name == "schema":
            data = canonical_json_bytes(schema_document())
            if args.write is not None:
                _atomic_write(args.write, data)
                print(f"wrote {args.write}")
                return 0
            assert args.check is not None
            try:
                current = args.check.read_bytes()
            except OSError as exc:
                raise EvidenceError(f"cannot read schema {args.check}: {exc}") from exc
            if current != data:
                raise EvidenceError(
                    f"generated schema differs from {args.check}; run schema --write"
                )
            print(f"schema ok: {args.check}")
            return 0
        if args.command_name == "run":
            _, status = run_manifest(
                manifest_path=args.manifest.resolve(),
                output_dir=args.output,
                repo_root=repo_root,
                case_filters=set(args.case),
                policy_filter=args.policy,
                category_filter=args.category,
                arch_filter=args.arch,
            )
            print(args.output / "semantic-evidence-v1.json")
            return status
        if args.command_name == "merge":
            _, status = merge_result_shards(
                shard_paths=args.shard,
                manifest_path=args.manifest.resolve(),
                output_dir=args.output,
                repo_root=repo_root,
            )
            print(args.output / "semantic-evidence-v1.json")
            return status
        if args.command_name == "validate-result":
            document = load_validate_result_with_manifest(
                args.path.resolve(),
                manifest_path=args.manifest.resolve(),
                repo_root=repo_root,
                require_full_required=args.require_full_required,
            )
            print(f"result ok: cases={len(document['cases'])} schema={document['schema_version']}")
            return 0
        if args.command_name == "supervise":
            command = list(args.argv)
            if command and command[0] == "--":
                command.pop(0)
            if not command:
                raise EvidenceError("supervise requires a command after --")
            _check_timeout(args.timeout, "--timeout")
            return supervise_command(command, args.timeout, args.log.resolve(), repo_root)
        raise AssertionError(args.command_name)
    except TerminationRequested as exc:
        print(f"semantic-evidence: {exc}", file=sys.stderr)
        return 128 + exc.signum
    except KeyboardInterrupt:
        print("semantic-evidence: interrupted", file=sys.stderr)
        return 130
    except EvidenceError as exc:
        print(f"semantic-evidence: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
