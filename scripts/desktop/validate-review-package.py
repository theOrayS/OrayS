#!/usr/bin/env python3
"""Validate a relocated desktop review package, including semantic evidence."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
from types import ModuleType
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from runtime_evidence_contract import (  # noqa: E402
    REVIEW_PACKAGE_SCHEMA,
    RUNTIME_METADATA_SCHEMA,
    RUN_SUMMARY_SCHEMA,
    validate_runtime_status,
)


def load_summarizer() -> ModuleType:
    path = Path(__file__).with_name("summarize-run.py")
    spec = importlib.util.spec_from_file_location("desktop_summarize_run", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("could not load summarize-run.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def parse_manifest(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        match = re.fullmatch(r"([0-9a-f]{64})  ([A-Za-z0-9][A-Za-z0-9._-]*)", line)
        if match is None or match.group(2) in values:
            raise ValueError(f"invalid package-files.sha256 row {line_number}")
        values[match.group(2)] = match.group(1)
    if not values:
        raise ValueError("package-files.sha256 is empty")
    return values


def validate_capture_binding(
    package_dir: Path, package: dict[str, Any], metadata: dict[str, Any]
) -> Path | None:
    binding = package.get("capture_binding")
    if binding is None:
        if package.get("result") == "PASS" or "frame.ppm" not in package["files"]:
            if package.get("result") == "PASS":
                raise ValueError("PASS package has no capture binding")
            return None
        return None
    if not isinstance(binding, dict) or set(binding) != {
        "original_run_dir",
        "original_filename",
        "evidence_relative_filename",
    }:
        raise ValueError("capture binding fields are invalid")
    relative = binding["evidence_relative_filename"]
    if relative != "frame.ppm":
        raise ValueError("capture binding relative filename is invalid")
    evidence_path = package_dir / relative
    if evidence_path.resolve().parent != package_dir or not evidence_path.is_file():
        raise ValueError("capture binding escapes or misses the package")
    original_run_dir = Path(str(binding["original_run_dir"]))
    original_filename = Path(str(binding["original_filename"]))
    metadata_run_dir = Path(str(metadata.get("run_dir")))
    if (
        not original_run_dir.is_absolute()
        or not original_filename.is_absolute()
        or not metadata_run_dir.is_absolute()
        or original_run_dir != metadata_run_dir
        or original_filename != original_run_dir / relative
    ):
        raise ValueError("capture binding original path is invalid")
    return original_filename


def validate(package_dir: Path) -> tuple[str, list[str]]:
    for required in ("review-package.json", "package-files.sha256"):
        path = package_dir / required
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"required package file is missing: {required}")
    package = json.loads((package_dir / "review-package.json").read_text(encoding="utf-8"))
    if not isinstance(package, dict) or package.get("schema") != REVIEW_PACKAGE_SCHEMA:
        raise ValueError("review package schema is invalid")
    result = package.get("result")
    arch = package.get("architecture")
    scenario = package.get("scenario")
    if result not in {"PASS", "FAIL"} or arch not in {"rv", "la"}:
        raise ValueError("review package result or architecture is invalid")
    if scenario not in {"boot", "launcher", "overlap", "applications", "resize"}:
        raise ValueError("review package scenario is invalid")

    manifest = parse_manifest(package_dir / "package-files.sha256")
    if package.get("files") != manifest:
        raise ValueError("review-package.json files do not match package-files.sha256")
    actual_names = {path.name for path in package_dir.iterdir()}
    expected_names = set(manifest) | {"review-package.json", "package-files.sha256"}
    if actual_names != expected_names:
        raise ValueError("package directory contains missing or unexpected files")
    for name, expected in manifest.items():
        path = package_dir / name
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"package evidence is not a regular file: {name}")
        if digest(path) != expected:
            raise ValueError(f"package evidence digest mismatch: {name}")

    summary = json.loads((package_dir / "summary.json").read_text(encoding="utf-8"))
    metadata = json.loads(
        (package_dir / "runtime-metadata.json").read_text(encoding="utf-8")
    )
    if summary.get("schema") != RUN_SUMMARY_SCHEMA:
        raise ValueError("summary schema is invalid")
    if metadata.get("schema") != RUNTIME_METADATA_SCHEMA:
        raise ValueError("runtime metadata schema is invalid")
    if any(
        package.get(name) != summary.get(name)
        for name in ("result", "architecture", "scenario")
    ):
        raise ValueError("package identity does not match summary")
    for package_name, metadata_name in (
        ("source_commit", "source_commit"),
        ("source_dirty", "source_dirty"),
        ("source_commit_before", "source_commit_before"),
        ("source_commit_after", "source_commit_after"),
        ("source_status_before", "source_status_before"),
        ("source_status_after", "source_status_after"),
        ("provenance_stable", "provenance_stable"),
        ("required_qemu_version", "required_qemu_version"),
        ("observed_qemu_version", "observed_qemu_version"),
        ("qemu_version_matches_required", "qemu_version_matches_required"),
        ("qemu_sha256", "qemu_sha256"),
        ("qemu_digest_policy", "qemu_digest_policy"),
        ("qemu_authorized_sha256", "qemu_authorized_sha256"),
        ("qemu_digest_matches_authorized", "qemu_digest_matches_authorized"),
        ("qemu_argv", "qemu_argv"),
        ("guest_artifact", "guest_artifact"),
        ("runner_inputs", "runner_inputs"),
    ):
        if package.get(package_name) != metadata.get(metadata_name):
            raise ValueError(f"package {package_name} does not match runtime metadata")
    for name in (
        "qemu_started",
        "qemu_exit",
        "runner_exit",
        "failure_stage",
        "failure_reason",
    ):
        if package.get(name) != summary.get(name):
            raise ValueError(f"package {name} does not match summary")

    original_screenshot = validate_capture_binding(package_dir, package, metadata)
    qemu_started = summary.get("qemu_started")
    qemu_exit = summary.get("qemu_exit")
    runner_exit = summary.get("runner_exit", 0)
    failure_stage = summary.get("failure_stage")
    failure_reason = summary.get("failure_reason")
    validate_runtime_status(
        qemu_started=qemu_started,
        qemu_exit=qemu_exit,
        runner_exit=runner_exit,
        failure_stage=failure_stage,
        failure_reason=failure_reason,
    )
    summarizer = load_summarizer()
    failures, geometry, hashes = summarizer.validate_run(
        package_dir,
        arch,
        scenario,
        qemu_exit,
        qemu_started=qemu_started,
        original_screenshot=original_screenshot,
    )
    if hashes != summary.get("hashes"):
        raise ValueError("semantic evidence hashes do not match summary")
    recorded_failures = summary.get("failures")
    if not isinstance(recorded_failures, list) or not all(
        isinstance(failure, str) for failure in recorded_failures
    ):
        raise ValueError("summary failures are invalid")
    expected_failures = list(failures)
    if runner_exit != 0:
        expected_failures.insert(
            0, f"runtime runner exited {runner_exit} during stage {failure_stage}"
        )
    if recorded_failures != expected_failures:
        raise ValueError(
            "summary failures do not exactly match the reproducible semantic failures"
        )
    if result == "PASS":
        if (
            failures
            or recorded_failures
            or not qemu_started
            or metadata.get("provenance_stable") is not True
            or metadata.get("source_dirty_before") is not False
            or metadata.get("source_dirty_after") is not False
            or metadata.get("qemu_version_matches_required") is not True
        ):
            raise ValueError(
                "PASS package does not reproduce a clean, version-pinned semantic PASS"
            )
        screenshot = summary.get("screenshot")
        if (
            not isinstance(screenshot, dict)
            or geometry != (screenshot.get("width"), screenshot.get("height"))
            or screenshot.get("sha256") != hashes.get("frame.ppm")
        ):
            raise ValueError("PASS package screenshot geometry is inconsistent")
    else:
        if not recorded_failures or (not failures and runner_exit == 0):
            raise ValueError("FAIL package does not reproduce or record a runner failure")
        if runner_exit != 0:
            runner_failure = (
                f"runtime runner exited {runner_exit} during stage {failure_stage}"
            )
            if runner_failure not in recorded_failures:
                raise ValueError("FAIL package runner status is not bound to its summary")
    return result, recorded_failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--package", type=Path, required=True)
    args = parser.parse_args()
    package_dir = args.package.resolve(strict=True)
    result, failures = validate(package_dir)
    print(
        f"DESKTOP_REVIEW_PACKAGE=VALID_{result} failures={len(failures)} "
        f"evidence={package_dir}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
