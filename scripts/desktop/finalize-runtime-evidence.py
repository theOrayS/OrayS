#!/usr/bin/env python3
"""Finalize provenance, summary, and a filtered package for every runtime outcome."""

from __future__ import annotations

import argparse
from pathlib import Path
import subprocess
import sys


SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from runtime_evidence_contract import (  # noqa: E402
    CANONICAL_QEMU_VERSION,
    default_failure_reason,
    require_canonical_qemu_version,
    validate_runtime_status,
)


CORE_FILES = (
    "serial.log",
    "qmp-input.jsonl",
    "qmp-capture.jsonl",
    "input-sequence.json",
)


def run(argv: list[str]) -> int:
    return subprocess.run(argv, check=False).returncode


def parse_bool(value: str) -> bool:
    if value == "true":
        return True
    if value == "false":
        return False
    raise argparse.ArgumentTypeError("expected true or false")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--arch", choices=("rv", "la"), required=True)
    parser.add_argument(
        "--scenario",
        choices=("boot", "launcher", "overlap", "applications", "resize"),
        required=True,
    )
    parser.add_argument("--qemu-binary", required=True)
    parser.add_argument("--required-qemu-version", default=CANONICAL_QEMU_VERSION)
    parser.add_argument("--qemu-started", type=parse_bool, default=True)
    parser.add_argument("--qemu-exit", type=int)
    parser.add_argument("--runner-exit", type=int, required=True)
    parser.add_argument("--failure-stage", required=True)
    parser.add_argument("--failure-reason")
    args = parser.parse_args()
    if args.qemu_started and args.qemu_exit is None:
        parser.error("--qemu-exit is required when --qemu-started=true")
    if not args.qemu_started and args.qemu_exit is not None:
        parser.error("--qemu-exit must be omitted when --qemu-started=false")
    try:
        require_canonical_qemu_version(args.required_qemu_version)
    except ValueError as error:
        parser.error(str(error))

    repo = args.repo_root.resolve(strict=True)
    run_dir = args.run_dir.resolve(strict=True)
    scripts = repo / "scripts/desktop"
    for name in CORE_FILES:
        path = run_dir / name
        if not path.exists():
            if name == "input-sequence.json":
                fixture = repo / f"test/desktop/fixtures/input/{args.scenario}.json"
                if fixture.is_file():
                    path.write_bytes(fixture.read_bytes())
                    continue
            path.touch()

    metadata = run_dir / "runtime-metadata.json"
    metadata_error = 0
    if not metadata.is_file():
        metadata_error = run(
            [
                sys.executable,
                "-B",
                str(scripts / "collect-runtime-metadata.py"),
                "--repo-root",
                str(repo),
                "--output",
                str(metadata),
                "--arch",
                args.arch,
                "--scenario",
                args.scenario,
                "--qemu-binary",
                args.qemu_binary,
                "--required-qemu-version",
                args.required_qemu_version,
                "--run-dir",
                str(run_dir),
            ]
        )
    if metadata_error == 0:
        metadata_error = run(
            [
                sys.executable,
                "-B",
                str(scripts / "collect-runtime-metadata.py"),
                "--repo-root",
                str(repo),
                "--output",
                str(metadata),
                "--finalize",
            ]
        )

    effective_runner_exit = args.runner_exit
    failure_stage = args.failure_stage
    if metadata_error != 0 and effective_runner_exit == 0:
        effective_runner_exit = 70
        failure_stage = "runtime-metadata-finalize"
    failure_reason = (
        args.failure_reason if failure_stage == args.failure_stage else None
    )
    if failure_reason is None:
        try:
            failure_reason = default_failure_reason(failure_stage, effective_runner_exit)
        except ValueError as error:
            parser.error(str(error))
    try:
        validate_runtime_status(
            qemu_started=args.qemu_started,
            qemu_exit=args.qemu_exit,
            runner_exit=effective_runner_exit,
            failure_stage=failure_stage,
            failure_reason=failure_reason,
        )
    except ValueError as error:
        parser.error(str(error))
    summary_argv = [
        sys.executable,
        "-B",
        str(scripts / "summarize-run.py"),
        "--run-dir",
        str(run_dir),
        "--arch",
        args.arch,
        "--scenario",
        args.scenario,
        "--qemu-started",
        str(args.qemu_started).lower(),
        "--runner-exit",
        str(effective_runner_exit),
        "--failure-stage",
        failure_stage,
    ]
    if args.qemu_exit is not None:
        summary_argv.extend(("--qemu-exit", str(args.qemu_exit)))
    summary_argv.extend(("--failure-reason", failure_reason))
    summary_status = run(summary_argv)
    if not (run_dir / "summary.json").is_file() or not (run_dir / "hashes.sha256").is_file():
        print("runtime summarizer did not produce the required failure evidence", file=sys.stderr)
        return 70

    package_status = run(
        [
            sys.executable,
            "-B",
            str(scripts / "package-review-evidence.py"),
            "--run-dir",
            str(run_dir),
        ]
    )
    if package_status != 0:
        return 70
    validation_status = run(
        [
            sys.executable,
            "-B",
            str(scripts / "validate-review-package.py"),
            "--package",
            str(run_dir / "review-package"),
        ]
    )
    if validation_status != 0:
        return 70
    return summary_status


if __name__ == "__main__":
    raise SystemExit(main())
