#!/usr/bin/env python3
"""Strictly validate final-2026 CAgent and BuildStorm console records.

The accepted protocol is derived from the read-only final-2026 reference tree at
commit 15e0355bbee0373de4048002448cee37dbb7ca1b.  Unlike the contestant-facing
reference judges, this validator rejects duplicate, missing, unknown, misplaced,
or contradictory records instead of silently overwriting or ignoring them.
"""

from __future__ import annotations

import math
import re
from typing import Any


PROTOCOL_REFERENCE_COMMIT = "15e0355bbee0373de4048002448cee37dbb7ca1b"
SUPPORTED_GROUPS = frozenset({"cagent", "buildstorm"})
SUPPORTED_ARCHITECTURES = frozenset({"riscv64", "loongarch64"})
EXPECTED_GROUP_LABELS = {
    "cagent": "cagent-glibc",
    "buildstorm": "buildstorm",
}
EXPECTED_BUILDSTORM_CORES = 8
MINIMUM_BUILDSTORM_ARTIFACT_BYTES = 500_000
BUILDSTORM_MAX_SCRIPTED_SCORE = 180.0

CAGENT_TESTS: dict[str, tuple[str, float, int]] = {
    "factorial": ("easy", 13.5, 20_000),
    "date": ("easy", 13.5, 20_000),
    "network": ("medium", 20.0, 25_000),
    "cpu": ("easy", 13.5, 20_000),
    "kernel": ("easy", 13.5, 20_000),
    "fs-create": ("medium", 20.0, 25_000),
    "fs-readwrite": ("medium", 20.0, 30_000),
    "fs-directory": ("medium", 20.0, 30_000),
    "fs-search": ("hard", 27.0, 35_000),
    "fs-usage": ("medium", 20.0, 25_000),
}
CAGENT_MAX_SCRIPTED_SCORE = round(
    sum(weight * 1.1 for _difficulty, weight, _timeout in CAGENT_TESTS.values()),
    2,
)

GROUP_RECORD_RE = re.compile(
    r"^#### OS COMP TEST GROUP (?P<event>START|END) "
    r"(?P<group>[A-Za-z0-9._-]+) ####$"
)
GROUP_SIGNATURE_RE = re.compile(r"OS COMP TEST GROUP")
CAGENT_RECORD_RE = re.compile(
    r"^testcase cagent (?P<name>\S+) (?P<status>pass|reject) "
    r"(?P<elapsed_ms>[0-9]+)$"
)
CAGENT_SIGNATURE_RE = re.compile(r"\btestcase\s+cagent\b")
BUILDSTORM_ENV_RE = re.compile(
    r"^BUILDSTORM_(?P<tag>TOOLCHAIN|MINIBUILD) (?P<status>ok|fail)$"
)
BUILDSTORM_BEGIN_RE = re.compile(r"^BUILDSTORM_BEGIN mode=multi$")
BUILDSTORM_COMPILE_RE = re.compile(r"^BUILDSTORM_COMPILE\s+(?P<fields>.+)$")
BUILDSTORM_SIGNATURE_RE = re.compile(r"\bBUILDSTORM_[A-Z_]+\b")
FATAL_RUNTIME_RE = re.compile(
    r"\b(?:kernel panic|unknown trap|fatal trap|watchdog expired|"
    r"segmentation fault|illegal instruction|core dumped)\b|^TIMEOUT\b",
    re.IGNORECASE | re.MULTILINE,
)


def _base_result(group: str) -> dict[str, Any]:
    return {
        "status": "ERROR",
        "group": group,
        "protocol_reference_commit": PROTOCOL_REFERENCE_COMMIT,
        "score": 0.0,
        "max_scripted_score": (
            CAGENT_MAX_SCRIPTED_SCORE
            if group == "cagent"
            else BUILDSTORM_MAX_SCRIPTED_SCORE
        ),
        "score_eligible": False,
        "failed_items": [],
        "errors": [],
        "records": [],
    }


def _validate_inputs(
    expected_group: str,
    expected_group_label: str,
    expected_arch: str,
    baseline_seconds: float,
) -> list[str]:
    errors: list[str] = []
    if expected_group not in SUPPORTED_GROUPS:
        errors.append(f"unsupported expected group: {expected_group!r}")
    elif expected_group_label != EXPECTED_GROUP_LABELS[expected_group]:
        errors.append(
            "expected_group_label does not match the fixed final-2026 protocol: "
            f"group={expected_group!r}, label={expected_group_label!r}, "
            f"required={EXPECTED_GROUP_LABELS[expected_group]!r}"
        )
    if expected_arch not in SUPPORTED_ARCHITECTURES:
        errors.append(f"unsupported expected architecture: {expected_arch!r}")
    if (
        isinstance(baseline_seconds, bool)
        or not isinstance(baseline_seconds, (int, float))
        or not math.isfinite(float(baseline_seconds))
        or baseline_seconds <= 0
    ):
        errors.append("buildstorm baseline seconds must be finite and positive")
    return errors


def _validate_lifecycle(
    stdout_lines: list[str], expected_group_label: str
) -> tuple[int | None, int | None, list[str]]:
    errors: list[str] = []
    markers: list[tuple[int, str, str]] = []
    for index, line in enumerate(stdout_lines):
        match = GROUP_RECORD_RE.fullmatch(line)
        if match is not None:
            markers.append((index, match.group("event"), match.group("group")))
        elif GROUP_SIGNATURE_RE.search(line):
            errors.append(f"malformed group lifecycle record at stdout line {index + 1}")

    if len(markers) != 2:
        errors.append(
            "group lifecycle must contain exactly one START and one END record; "
            f"observed {len(markers)}"
        )
        return None, None, errors

    start_index, start_event, start_group = markers[0]
    end_index, end_event, end_group = markers[1]
    if start_event != "START" or end_event != "END" or start_index >= end_index:
        errors.append("group lifecycle records are out of order")
    if start_group != end_group:
        errors.append(
            f"group lifecycle labels disagree: START={start_group}, END={end_group}"
        )
    if start_group != expected_group_label or end_group != expected_group_label:
        errors.append(
            f"observed group {start_group}/{end_group} does not match expected group label "
            f"{expected_group_label}"
        )
    return start_index, end_index, errors


def _protocol_on_stderr(stderr_lines: list[str]) -> list[str]:
    return [
        f"protocol record must not appear on stderr line {index + 1}: {line}"
        for index, line in enumerate(stderr_lines)
        if (
            GROUP_SIGNATURE_RE.search(line)
            or CAGENT_SIGNATURE_RE.search(line)
            or BUILDSTORM_SIGNATURE_RE.search(line)
        )
    ]


def _record_is_inside(
    line_index: int, start_index: int | None, end_index: int | None
) -> bool:
    return (
        start_index is not None
        and end_index is not None
        and start_index < line_index < end_index
    )


def _validate_cagent(
    stdout_lines: list[str], start_index: int | None, end_index: int | None
) -> dict[str, Any]:
    result = _base_result("cagent")
    errors: list[str] = []
    observed: dict[str, dict[str, Any]] = {}

    for index, line in enumerate(stdout_lines):
        match = CAGENT_RECORD_RE.fullmatch(line)
        if match is None:
            if CAGENT_SIGNATURE_RE.search(line):
                errors.append(f"malformed cagent record at stdout line {index + 1}: {line}")
            continue
        if not _record_is_inside(index, start_index, end_index):
            errors.append(f"cagent record outside group lifecycle at stdout line {index + 1}")
        name = match.group("name")
        if name not in CAGENT_TESTS:
            errors.append(f"unknown cagent identity: {name}")
            continue
        if name in observed:
            errors.append(f"duplicate cagent identity: {name}")
            continue
        observed[name] = {
            "name": name,
            "status": match.group("status"),
            "elapsed_ms": int(match.group("elapsed_ms")),
        }

    missing = [name for name in CAGENT_TESTS if name not in observed]
    if missing:
        errors.append("missing cagent identities: " + ", ".join(missing))

    score = 0.0
    records: list[dict[str, Any]] = []
    failed_items: list[str] = []
    for name, (difficulty, weight, timeout_ms) in CAGENT_TESTS.items():
        record = observed.get(name)
        if record is None:
            continue
        passed = record["status"] == "pass"
        bonus = (
            weight * 0.1
            if passed and 0 < record["elapsed_ms"] < timeout_ms / 2
            else 0.0
        )
        item_score = round(weight + bonus, 2) if passed else 0.0
        if not passed:
            failed_items.append(name)
        score += item_score
        records.append(
            {
                **record,
                "difficulty": difficulty,
                "weight": weight,
                "timeout_ms": timeout_ms,
                "bonus": round(bonus, 2),
                "score": item_score,
            }
        )

    result.update(
        {
            "score": round(score, 2),
            "score_eligible": not errors,
            "failed_items": failed_items,
            "errors": errors,
            "records": records,
            "observed_case_count": len(observed),
        }
    )
    result["status"] = "ERROR" if errors else ("FAIL" if failed_items else "PASS")
    return result


def _parse_key_values(fields: str) -> tuple[dict[str, str], list[str]]:
    values: dict[str, str] = {}
    errors: list[str] = []
    for token in fields.split():
        if token.count("=") != 1:
            errors.append(f"malformed buildstorm key/value token: {token}")
            continue
        name, value = token.split("=", 1)
        if not name or not value:
            errors.append(f"malformed buildstorm key/value token: {token}")
            continue
        if name in values:
            errors.append(f"duplicate buildstorm key: {name}")
            continue
        values[name] = value
    return values, errors


def _strict_float(value: str | None, field: str, errors: list[str]) -> float | None:
    if value is None:
        errors.append(f"missing buildstorm field: {field}")
        return None
    try:
        parsed = float(value)
    except ValueError:
        errors.append(f"buildstorm {field} is not numeric: {value!r}")
        return None
    if not math.isfinite(parsed) or parsed < 0:
        errors.append(f"buildstorm {field} must be finite and non-negative: {value!r}")
        return None
    return parsed


def _strict_int(value: str | None, field: str, errors: list[str]) -> int | None:
    if value is None:
        errors.append(f"missing buildstorm field: {field}")
        return None
    if re.fullmatch(r"(?:0|[1-9][0-9]*)", value) is None:
        errors.append(f"buildstorm {field} is not a non-negative integer: {value!r}")
        return None
    return int(value)


def _validate_buildstorm(
    stdout_lines: list[str],
    start_index: int | None,
    end_index: int | None,
    *,
    expected_arch: str,
    baseline_seconds: float,
) -> dict[str, Any]:
    result = _base_result("buildstorm")
    errors: list[str] = []
    environment_records: dict[str, tuple[int, str]] = {}
    begin_indices: list[int] = []
    compile_records: list[tuple[int, str]] = []

    for index, line in enumerate(stdout_lines):
        env_match = BUILDSTORM_ENV_RE.fullmatch(line)
        compile_match = BUILDSTORM_COMPILE_RE.fullmatch(line)
        if env_match is not None:
            if not _record_is_inside(index, start_index, end_index):
                errors.append(
                    f"buildstorm record outside group lifecycle at stdout line {index + 1}"
                )
            tag = env_match.group("tag")
            if tag in environment_records:
                errors.append(f"duplicate buildstorm {tag.lower()} record")
            else:
                environment_records[tag] = (index, env_match.group("status"))
        elif BUILDSTORM_BEGIN_RE.fullmatch(line) is not None:
            if not _record_is_inside(index, start_index, end_index):
                errors.append(
                    f"buildstorm begin outside group lifecycle at stdout line {index + 1}"
                )
            begin_indices.append(index)
        elif compile_match is not None:
            if not _record_is_inside(index, start_index, end_index):
                errors.append(
                    f"buildstorm compile outside group lifecycle at stdout line {index + 1}"
                )
            compile_records.append((index, compile_match.group("fields")))
        elif BUILDSTORM_SIGNATURE_RE.search(line):
            errors.append(f"malformed buildstorm record at stdout line {index + 1}: {line}")

    for tag in ("TOOLCHAIN", "MINIBUILD"):
        if tag not in environment_records:
            errors.append(f"missing buildstorm {tag.lower()} record")
    if len(begin_indices) > 1:
        errors.append(f"duplicate buildstorm begin record: observed {len(begin_indices)}")
    if len(compile_records) != 1:
        qualifier = "missing" if not compile_records else "duplicate"
        errors.append(
            f"{qualifier} buildstorm compile record: observed {len(compile_records)}"
        )

    compile_index: int | None = None
    fields: dict[str, str] = {}
    if compile_records:
        compile_index, raw_fields = compile_records[0]
        fields, field_errors = _parse_key_values(raw_fields)
        errors.extend(field_errors)

    expected_fields = {"mode", "ok", "elapsed_s", "cores", "bytes", "arch"}
    allowed_fields = expected_fields | {"rc"}
    missing_fields = sorted(expected_fields - set(fields))
    unknown_fields = sorted(set(fields) - allowed_fields)
    if missing_fields:
        errors.append("missing buildstorm fields: " + ", ".join(missing_fields))
    if unknown_fields:
        errors.append("unknown buildstorm fields: " + ", ".join(unknown_fields))
    if fields.get("mode") not in {None, "multi"}:
        errors.append(f"buildstorm mode must be multi: {fields.get('mode')!r}")
    if fields.get("ok") not in {None, "true", "false"}:
        errors.append(f"buildstorm ok field is invalid: {fields.get('ok')!r}")
    if fields.get("ok") == "true" and "rc" in fields:
        errors.append("successful buildstorm compile must not carry an rc field")
    if "rc" in fields:
        _strict_int(fields.get("rc"), "rc", errors)

    elapsed = _strict_float(fields.get("elapsed_s"), "elapsed_s", errors)
    cores = _strict_int(fields.get("cores"), "cores", errors)
    artifact_bytes = _strict_int(fields.get("bytes"), "bytes", errors)
    observed_arch = fields.get("arch")
    if observed_arch is not None and observed_arch not in SUPPORTED_ARCHITECTURES:
        errors.append(f"unsupported buildstorm arch: {observed_arch!r}")
    elif observed_arch is not None and observed_arch != expected_arch:
        errors.append(
            f"buildstorm arch mismatch: expected {expected_arch}, observed {observed_arch}"
        )
    if cores is not None and cores != EXPECTED_BUILDSTORM_CORES:
        errors.append(
            f"buildstorm cores mismatch: expected {EXPECTED_BUILDSTORM_CORES}, "
            f"observed {cores}; score is ineligible"
        )
    compile_ok = fields.get("ok") == "true"
    if (
        compile_ok
        and artifact_bytes is not None
        and artifact_bytes < MINIMUM_BUILDSTORM_ARTIFACT_BYTES
    ):
        errors.append(
            "buildstorm ok=true conflicts with bytes below "
            f"{MINIMUM_BUILDSTORM_ARTIFACT_BYTES}: {artifact_bytes}"
        )
    if compile_ok and len(begin_indices) != 1:
        errors.append("successful buildstorm compile requires exactly one begin record")

    if compile_index is not None:
        toolchain_index = environment_records.get("TOOLCHAIN", (None, ""))[0]
        minibuild_index = environment_records.get("MINIBUILD", (None, ""))[0]
        ordered_indices = [toolchain_index, minibuild_index]
        if all(index is not None for index in ordered_indices):
            if not (toolchain_index < minibuild_index < compile_index):
                errors.append("buildstorm environment and compile records are out of order")
        if begin_indices and not (begin_indices[0] < compile_index):
            errors.append("buildstorm begin must precede compile result")

    failed_items: list[str] = []
    score = 0.0
    records: list[dict[str, Any]] = []
    for tag, points, item_name in (
        ("TOOLCHAIN", 8.0, "toolchain"),
        ("MINIBUILD", 12.0, "minibuild"),
    ):
        status = environment_records.get(tag, (0, "missing"))[1]
        passed = status == "ok"
        if passed:
            score += points
        else:
            failed_items.append(item_name)
        records.append(
            {"name": item_name, "status": status, "score": points if passed else 0.0}
        )

    time_score = 0.0
    if compile_ok:
        score += 40.0
        if elapsed is not None:
            time_score = round(
                120.0
                * max(
                    0.0,
                    min(
                        1.0,
                        (2.0 * float(baseline_seconds) - elapsed)
                        / float(baseline_seconds),
                    ),
                ),
                1,
            )
            score += time_score
            if time_score <= 0:
                failed_items.append("compile-time")
    else:
        failed_items.append("compile")
        failed_items.append("compile-time")
    records.append(
        {
            "name": "compile",
            "status": "pass" if compile_ok else "fail",
            "score": 40.0 if compile_ok else 0.0,
            "elapsed_s": elapsed,
            "time_score": time_score,
            "cores": cores,
            "bytes": artifact_bytes,
            "arch": observed_arch,
        }
    )

    result.update(
        {
            "score": round(score, 1),
            "score_eligible": not errors,
            "failed_items": failed_items,
            "errors": errors,
            "records": records,
            "observed_record_count": (
                len(environment_records) + len(begin_indices) + len(compile_records)
            ),
            "buildstorm_baseline_seconds": float(baseline_seconds),
        }
    )
    result["status"] = "ERROR" if errors else ("FAIL" if failed_items else "PASS")
    return result


def validate_final_2026_output(
    stdout: str,
    stderr: str,
    *,
    expected_group: str,
    expected_group_label: str,
    expected_arch: str,
    buildstorm_baseline_seconds: float = 400.0,
) -> dict[str, Any]:
    """Validate one exact final-2026 group and reproduce its reference score."""

    result = _base_result(expected_group)
    input_errors = _validate_inputs(
        expected_group,
        expected_group_label,
        expected_arch,
        buildstorm_baseline_seconds,
    )
    stdout_lines = stdout.splitlines()
    stderr_lines = stderr.splitlines()
    start_index, end_index, lifecycle_errors = _validate_lifecycle(
        stdout_lines, expected_group_label
    )
    shared_errors = [
        *input_errors,
        *lifecycle_errors,
        *_protocol_on_stderr(stderr_lines),
    ]
    unexpected_protocol = (
        BUILDSTORM_SIGNATURE_RE if expected_group == "cagent" else CAGENT_SIGNATURE_RE
    )
    if any(unexpected_protocol.search(line) for line in stdout_lines):
        shared_errors.append(
            f"stdout contains records from a group other than {expected_group}"
        )
    combined = stdout + ("\n" if stdout and stderr else "") + stderr
    if FATAL_RUNTIME_RE.search(combined):
        shared_errors.append("fatal runtime evidence is present in captured output")

    if input_errors:
        result["errors"] = shared_errors
        return result
    if expected_group == "cagent":
        result = _validate_cagent(stdout_lines, start_index, end_index)
    elif expected_group == "buildstorm":
        result = _validate_buildstorm(
            stdout_lines,
            start_index,
            end_index,
            expected_arch=expected_arch,
            baseline_seconds=buildstorm_baseline_seconds,
        )
    else:
        result["errors"] = []

    result["errors"] = [*shared_errors, *result["errors"]]
    if result["errors"]:
        result["status"] = "ERROR"
        result["score_eligible"] = False
    return result


__all__ = [
    "BUILDSTORM_MAX_SCRIPTED_SCORE",
    "CAGENT_MAX_SCRIPTED_SCORE",
    "CAGENT_TESTS",
    "EXPECTED_GROUP_LABELS",
    "PROTOCOL_REFERENCE_COMMIT",
    "validate_final_2026_output",
]
