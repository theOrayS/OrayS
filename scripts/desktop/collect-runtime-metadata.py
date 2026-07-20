#!/usr/bin/env python3
"""Record source, emulator, toolchain, invocation, and end-of-run provenance."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shutil
import subprocess
import sys


SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from runtime_evidence_contract import (  # noqa: E402
    RUNTIME_METADATA_SCHEMA,
    qemu_version_is_canonical,
    require_canonical_qemu_version,
)


def command_output(argv: list[str], cwd: Path) -> str:
    result = subprocess.run(
        argv,
        cwd=cwd,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError(f"command failed ({result.returncode}): {' '.join(argv)}")
    output = result.stdout.strip()
    if not output:
        raise RuntimeError(f"command produced no version output: {' '.join(argv)}")
    return output


def collect_version(
    label: str, argv: list[str], cwd: Path, errors: list[str]
) -> str | None:
    try:
        return command_output(argv, cwd)
    except (OSError, RuntimeError) as error:
        errors.append(f"{label} version collection failed: {error}")
        return None


def git_state(repo: Path) -> tuple[str, list[str]]:
    status_result = subprocess.run(
        ["git", "-c", "core.quotepath=false", "status", "--short", "--untracked-files=all"],
        cwd=repo,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if status_result.returncode != 0:
        raise RuntimeError(f"git status failed ({status_result.returncode})")
    commit = command_output(["git", "rev-parse", "HEAD"], repo).splitlines()[0]
    return commit, status_result.stdout.splitlines()


def finalize(repo: Path, output: Path) -> int:
    value = json.loads(output.read_text(encoding="utf-8"))
    if not isinstance(value, dict) or value.get("schema") != RUNTIME_METADATA_SCHEMA:
        raise ValueError("runtime metadata schema is invalid for finalization")
    errors = value.get("collection_errors")
    if not isinstance(errors, list) or not all(isinstance(error, str) for error in errors):
        raise ValueError("runtime metadata collection_errors is invalid")
    try:
        commit, status = git_state(repo)
    except RuntimeError as error:
        errors.append(f"final provenance collection failed: {error}")
        commit = None
        status = None
    value.update(
        {
            "finalized_at_utc": datetime.now(timezone.utc).isoformat(),
            "source_commit_after": commit,
            "source_dirty_after": bool(status) if status is not None else None,
            "source_status_after": status,
            "provenance_stable": (
                commit is not None
                and status is not None
                and commit == value.get("source_commit_before")
                and status == value.get("source_status_before")
                and not status
                and value.get("source_dirty_before") is False
            ),
            "collection_errors": errors,
        }
    )
    output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--finalize", action="store_true")
    parser.add_argument("--arch", choices=("rv", "la"))
    parser.add_argument(
        "--scenario",
        choices=("boot", "launcher", "overlap", "applications", "resize"),
    )
    parser.add_argument("--qemu-binary")
    parser.add_argument("--required-qemu-version")
    parser.add_argument("--run-dir", type=Path)
    args = parser.parse_args()

    repo = args.repo_root.resolve(strict=True)
    output = args.output.resolve()
    if args.finalize:
        return finalize(repo, output)
    if (
        args.arch is None
        or args.scenario is None
        or args.qemu_binary is None
        or args.required_qemu_version is None
        or args.run_dir is None
    ):
        parser.error(
            "initial collection requires --arch, --scenario, --qemu-binary, "
            "--required-qemu-version, and --run-dir"
        )
    try:
        require_canonical_qemu_version(args.required_qemu_version)
    except ValueError as error:
        parser.error(str(error))
    run_dir = args.run_dir.resolve(strict=True)
    commit, status = git_state(repo)

    collection_errors: list[str] = []
    qemu_path_value = shutil.which(args.qemu_binary)
    if qemu_path_value is None:
        qemu_path = None
        qemu_version = None
        collection_errors.append(f"QEMU binary is unavailable: {args.qemu_binary}")
    else:
        try:
            qemu_path = Path(qemu_path_value).resolve(strict=True)
        except OSError as error:
            qemu_path = None
            collection_errors.append(f"QEMU path resolution failed: {error}")
        if qemu_path is None:
            qemu_version = None
        else:
            qemu_output = collect_version(
                "QEMU", [str(qemu_path), "--version"], repo, collection_errors
            )
            qemu_version = qemu_output.splitlines()[0] if qemu_output else None
    rustc_version = collect_version(
        "rustc", ["rustc", "-Vv"], repo, collection_errors
    )
    cargo_output = collect_version(
        "cargo", ["cargo", "-V"], repo, collection_errors
    )
    cargo_version = cargo_output.splitlines()[0] if cargo_output else None
    python_output = collect_version(
        "python", ["python3", "--version"], repo, collection_errors
    )
    python_version = python_output.splitlines()[0] if python_output else None

    try:
        relative_run_dir = run_dir.relative_to(repo).as_posix()
    except ValueError:
        relative_run_dir = str(run_dir)
    value = {
        "schema": RUNTIME_METADATA_SCHEMA,
        "created_at_utc": datetime.now(timezone.utc).isoformat(),
        "finalized_at_utc": None,
        "source_commit": commit,
        "source_dirty": bool(status),
        "source_status": status,
        "source_commit_before": commit,
        "source_dirty_before": bool(status),
        "source_status_before": status,
        "source_commit_after": None,
        "source_dirty_after": None,
        "source_status_after": None,
        "provenance_stable": False,
        "collection_errors": collection_errors,
        "architecture": args.arch,
        "scenario": args.scenario,
        "run_dir": str(run_dir),
        "qemu_binary": str(qemu_path) if qemu_path is not None else args.qemu_binary,
        "qemu_version": qemu_version,
        "required_qemu_version": args.required_qemu_version,
        "observed_qemu_version": qemu_version,
        "qemu_version_matches_required": qemu_version_is_canonical(
            args.required_qemu_version, qemu_version
        ),
        "toolchain_versions": {
            "rustc": rustc_version,
            "cargo": cargo_version,
            "python": python_version,
        },
        "generation_command": [
            "scripts/desktop/run-headless-qemu.sh",
            "--arch",
            args.arch,
            "--scenario",
            args.scenario,
            "--output",
            relative_run_dir,
        ],
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
