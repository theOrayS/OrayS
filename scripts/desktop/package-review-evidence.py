#!/usr/bin/env python3
"""Validate and assemble one self-contained desktop human-review evidence package."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import sys


SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from runtime_evidence_contract import (  # noqa: E402
    REVIEW_PACKAGE_SCHEMA,
    RUNTIME_METADATA_SCHEMA,
    RUN_SUMMARY_SCHEMA,
    validate_runtime_status,
)


RAW_EVIDENCE_ALLOWLIST = {
    "serial.log",
    "qmp-input.jsonl",
    "qmp-capture.jsonl",
    "input-sequence.json",
    "frame.ppm",
    "runtime-metadata.json",
    "display-geometry.txt",
    "capture-precondition.json",
    "vnc-resize.json",
}

FAILURE_CORE_FILES = {
    "serial.log",
    "qmp-input.jsonl",
    "qmp-capture.jsonl",
    "input-sequence.json",
    "runtime-metadata.json",
}


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def parse_hash_manifest(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        match = re.fullmatch(r"([0-9a-f]{64})  ([A-Za-z0-9][A-Za-z0-9._-]*)", line)
        if match is None or match.group(2) in values:
            raise ValueError(f"invalid hashes.sha256 row {line_number}")
        values[match.group(2)] = match.group(1)
    if not values:
        raise ValueError("hashes.sha256 is empty")
    return values


def required_files(scenario: str) -> set[str]:
    files = set(RAW_EVIDENCE_ALLOWLIST - {"vnc-resize.json"})
    if scenario == "resize":
        files.add("vnc-resize.json")
    return files


def capture_binding(
    run_dir: Path, files: set[str], *, required: bool
) -> dict[str, str] | None:
    if "frame.ppm" not in files:
        return None
    transcript = run_dir / "qmp-capture.jsonl"
    targets: list[str] = []
    for line_number, line in enumerate(
        transcript.read_text(encoding="utf-8").splitlines(), 1
    ):
        if not line.strip():
            continue
        try:
            value = json.loads(line)
        except json.JSONDecodeError as error:
            if not required:
                return None
            raise ValueError(
                f"invalid capture transcript row {line_number}: {error}"
            ) from error
        if not isinstance(value, dict):
            if not required:
                return None
            raise ValueError(f"invalid capture transcript row {line_number}")
        message = value.get("message")
        if (
            value.get("direction") == "send"
            and isinstance(message, dict)
            and message.get("execute") == "screendump"
        ):
            arguments = message.get("arguments")
            filename = arguments.get("filename") if isinstance(arguments, dict) else None
            if not isinstance(filename, str):
                if not required:
                    return None
                raise ValueError("capture transcript screendump filename is invalid")
            targets.append(filename)
    if len(targets) != 1:
        if not required:
            return None
        raise ValueError("capture transcript must contain exactly one screendump target")
    target = Path(targets[0])
    if not target.is_absolute() or target.resolve() != (run_dir / "frame.ppm").resolve():
        if not required:
            return None
        raise ValueError("capture transcript is not bound to the source frame")
    return {
        "original_run_dir": str(run_dir),
        "original_filename": str(target),
        "evidence_relative_filename": "frame.ppm",
    }


def validate(run_dir: Path) -> tuple[dict, dict, set[str]]:
    summary = json.loads((run_dir / "summary.json").read_text(encoding="utf-8"))
    metadata = json.loads((run_dir / "runtime-metadata.json").read_text(encoding="utf-8"))
    scenario = summary.get("scenario")
    architecture = summary.get("architecture")
    if scenario not in {"boot", "launcher", "overlap", "applications", "resize"}:
        raise ValueError("summary scenario is invalid")
    if architecture not in {"rv", "la"} or summary.get("result") not in {"PASS", "FAIL"}:
        raise ValueError("summary architecture or result is invalid")
    if summary.get("schema") != RUN_SUMMARY_SCHEMA:
        raise ValueError("summary schema is invalid")
    if metadata.get("schema") != RUNTIME_METADATA_SCHEMA:
        raise ValueError("runtime metadata schema is invalid")
    if metadata.get("scenario") != scenario or metadata.get("architecture") != architecture:
        raise ValueError("runtime metadata does not match summary")

    result = summary["result"]
    failures = summary.get("failures")
    if not isinstance(failures, list) or not all(isinstance(item, str) for item in failures):
        raise ValueError("summary failures are invalid")
    if result == "PASS" and failures:
        raise ValueError("PASS summary contains failures")
    if result == "FAIL" and not failures:
        raise ValueError("FAIL summary does not explain the failure")
    validate_runtime_status(
        qemu_started=summary.get("qemu_started"),
        qemu_exit=summary.get("qemu_exit"),
        runner_exit=summary.get("runner_exit"),
        failure_stage=summary.get("failure_stage"),
        failure_reason=summary.get("failure_reason"),
    )

    manifest = parse_hash_manifest(run_dir / "hashes.sha256")
    if not set(manifest) <= RAW_EVIDENCE_ALLOWLIST:
        names = sorted(set(manifest) - RAW_EVIDENCE_ALLOWLIST)
        raise ValueError(f"hash manifest contains non-allowlisted evidence: {names}")
    if result == "PASS":
        raw_files = required_files(scenario)
        if set(manifest) != raw_files:
            raise ValueError("PASS hash manifest does not exactly cover required raw evidence")
    else:
        if not FAILURE_CORE_FILES <= set(manifest):
            missing = sorted(FAILURE_CORE_FILES - set(manifest))
            raise ValueError(f"FAIL hash manifest is missing core evidence: {missing}")

    files = set(manifest) | {"summary.json", "hashes.sha256"}
    for name in sorted(files):
        path = run_dir / name
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"required regular evidence file is missing: {name}")

    summary_hashes = summary.get("hashes")
    if not isinstance(summary_hashes, dict) or summary_hashes != manifest:
        raise ValueError("summary hashes do not exactly match hashes.sha256")
    for name, expected in manifest.items():
        if digest(run_dir / name) != expected:
            raise ValueError(f"evidence digest mismatch: {name}")
    return summary, metadata, files


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    run_dir = args.run_dir.resolve(strict=True)
    summary, metadata, files = validate(run_dir)
    output = (args.output or (run_dir / "review-package")).resolve()
    if output.exists():
        raise ValueError(f"refusing to overwrite existing review package: {output}")
    output.mkdir(parents=False)
    for name in sorted(files):
        shutil.copyfile(run_dir / name, output / name)

    package_hashes = {name: digest(output / name) for name in sorted(files)}
    (output / "package-files.sha256").write_text(
        "".join(f"{value}  {name}\n" for name, value in package_hashes.items()),
        encoding="utf-8",
    )
    package = {
        "schema": REVIEW_PACKAGE_SCHEMA,
        "result": summary["result"],
        "architecture": summary["architecture"],
        "scenario": summary["scenario"],
        "source_commit": metadata["source_commit"],
        "source_dirty": metadata["source_dirty"],
        "source_commit_before": metadata.get("source_commit_before"),
        "source_commit_after": metadata.get("source_commit_after"),
        "source_status_before": metadata.get("source_status_before"),
        "source_status_after": metadata.get("source_status_after"),
        "provenance_stable": metadata.get("provenance_stable"),
        "required_qemu_version": metadata.get("required_qemu_version"),
        "observed_qemu_version": metadata.get("observed_qemu_version"),
        "qemu_version_matches_required": metadata.get("qemu_version_matches_required"),
        "qemu_sha256": metadata.get("qemu_sha256"),
        "qemu_digest_policy": metadata.get("qemu_digest_policy"),
        "qemu_authorized_sha256": metadata.get("qemu_authorized_sha256"),
        "qemu_digest_matches_authorized": metadata.get(
            "qemu_digest_matches_authorized"
        ),
        "qemu_argv": metadata.get("qemu_argv"),
        "guest_artifact": metadata.get("guest_artifact"),
        "runner_inputs": metadata.get("runner_inputs"),
        "qemu_started": summary.get("qemu_started"),
        "qemu_exit": summary.get("qemu_exit"),
        "runner_exit": summary.get("runner_exit"),
        "failure_stage": summary.get("failure_stage"),
        "failure_reason": summary.get("failure_reason"),
        "capture_binding": capture_binding(
            run_dir, files, required=summary["result"] == "PASS"
        ),
        "files": package_hashes,
    }
    (output / "review-package.json").write_text(
        json.dumps(package, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(
        f"DESKTOP_REVIEW_PACKAGE=VALID result={summary['result']} evidence={output}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
