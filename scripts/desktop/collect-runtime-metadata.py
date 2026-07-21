#!/usr/bin/env python3
"""Record source, emulator, toolchain, invocation, and end-of-run provenance."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
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
    load_runtime_policy,
    qemu_version_is_canonical,
    require_canonical_qemu_version,
    validate_runtime_identity,
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
    identity_valid = True
    try:
        validate_runtime_identity(
            value,
            Path(value["run_dir"]) / "runtime-policy.json",
            value["architecture"],
            verify_files=True,
            require_complete=(
                isinstance(value.get("runtime_identity"), dict)
                and value["runtime_identity"].get("guest_artifact") is not None
            ),
        )
    except (KeyError, OSError, ValueError, json.JSONDecodeError) as error:
        errors.append(f"runtime identity verification failed: {error}")
        identity_valid = False
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
                and identity_valid
            ),
            "collection_errors": errors,
        }
    )
    output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0 if identity_valid else 1


def bind_runtime(
    repo: Path,
    output: Path,
    artifact_argument: Path,
    launch_argv_json: str,
) -> int:
    value = json.loads(output.read_text(encoding="utf-8"))
    if not isinstance(value, dict) or value.get("schema") != RUNTIME_METADATA_SCHEMA:
        raise ValueError("runtime metadata schema is invalid for identity binding")
    arch = value.get("architecture")
    if arch not in {"rv", "la"}:
        raise ValueError("runtime metadata architecture is invalid for identity binding")
    run_dir = Path(value["run_dir"])
    policy_path = run_dir / "runtime-policy.json"
    policy, _ = load_runtime_policy(policy_path)
    architecture_policy = policy["architectures"].get(arch)
    if not isinstance(architecture_policy, dict):
        raise ValueError("runtime policy has no selected architecture")
    artifact = artifact_argument.resolve(strict=True)
    expected_artifact = (repo / architecture_policy["artifact"]).resolve(strict=True)
    if artifact != expected_artifact:
        raise ValueError("artifact canonical path does not match the runtime policy")
    launch_argv = json.loads(launch_argv_json)
    if not isinstance(launch_argv, list):
        raise ValueError("QEMU launch argv JSON is not an array")
    identity = value.get("runtime_identity")
    if not isinstance(identity, dict):
        raise ValueError("runtime identity is missing before artifact binding")
    identity.update(
        {
            "guest_artifact": {
                "architecture": arch,
                "repository_path": architecture_policy["artifact"],
                "canonical_path": str(artifact),
                "sha256": hashlib.sha256(artifact.read_bytes()).hexdigest(),
                "source_commit": value.get("source_commit_before"),
            },
            "qemu_launch_argv": launch_argv,
        }
    )
    validate_runtime_identity(
        value, policy_path, arch, verify_files=True
    )
    output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--finalize", action="store_true")
    parser.add_argument("--bind-runtime", action="store_true")
    parser.add_argument("--arch", choices=("rv", "la"))
    parser.add_argument(
        "--scenario",
        choices=("boot", "launcher", "overlap", "applications", "resize"),
    )
    parser.add_argument("--qemu-binary")
    parser.add_argument("--qemu-path", type=Path)
    parser.add_argument("--required-qemu-version")
    parser.add_argument("--runtime-policy", type=Path)
    parser.add_argument("--run-dir", type=Path)
    parser.add_argument("--artifact", type=Path)
    parser.add_argument("--qemu-launch-argv-json")
    args = parser.parse_args()

    repo = args.repo_root.resolve(strict=True)
    output = args.output.resolve()
    if args.finalize:
        if args.bind_runtime:
            parser.error("--finalize and --bind-runtime are mutually exclusive")
        return finalize(repo, output)
    if args.bind_runtime:
        if args.artifact is None or args.qemu_launch_argv_json is None:
            parser.error("--bind-runtime requires --artifact and --qemu-launch-argv-json")
        return bind_runtime(repo, output, args.artifact, args.qemu_launch_argv_json)
    if args.required_qemu_version is not None:
        try:
            require_canonical_qemu_version(args.required_qemu_version)
        except ValueError as error:
            parser.error(str(error))
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
    run_dir = args.run_dir.resolve(strict=True)
    policy_source = (
        args.runtime_policy.resolve(strict=True)
        if args.runtime_policy is not None
        else (repo / "test/desktop/runtime-policy.json").resolve(strict=True)
    )
    expected_policy_source = (repo / "test/desktop/runtime-policy.json").resolve(strict=True)
    if policy_source != expected_policy_source:
        parser.error("runtime policy must be the tracked repository policy")
    policy, policy_sha256 = load_runtime_policy(policy_source)
    architecture_policy = policy["architectures"].get(args.arch)
    if not isinstance(architecture_policy, dict):
        parser.error("runtime policy has no selected architecture")
    if architecture_policy.get("qemu_binary") != args.qemu_binary:
        parser.error("QEMU binary name does not match the runtime policy")
    qemu_path = args.qemu_path.resolve(strict=True) if args.qemu_path is not None else None
    if qemu_path is not None and (
        qemu_path != args.qemu_path or qemu_path.name != args.qemu_binary
    ):
        parser.error("QEMU path must be absolute, canonical, and policy-named")
    policy_snapshot = run_dir / "runtime-policy.json"
    shutil.copyfile(policy_source, policy_snapshot)
    commit, status = git_state(repo)

    collection_errors: list[str] = []
    required_qemu_sha256 = architecture_policy.get("qemu_sha256")
    verified_exec = repo / "scripts/desktop/runtime-identity.py"
    if not verified_exec.is_file():
        verified_exec = SCRIPT_DIR / "runtime-identity.py"
    qemu_output = None
    if qemu_path is None:
        collection_errors.append("QEMU path unavailable before runtime start")
    else:
        qemu_output = collect_version(
            "QEMU",
            [
                sys.executable,
                "-B",
                str(verified_exec),
                "exec",
                "--canonical-path",
                str(qemu_path),
                "--required-sha256",
                str(required_qemu_sha256),
                "--",
                "--version",
            ],
            repo,
            collection_errors,
        )
    qemu_version = qemu_output.splitlines()[0] if qemu_output else None
    observed_qemu_sha256 = (
        hashlib.sha256(qemu_path.read_bytes()).hexdigest()
        if qemu_path is not None
        else None
    )
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
        "repository_root": str(repo),
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
        "runtime_identity": {
            "schema": 1,
            "policy_repository_path": "test/desktop/runtime-policy.json",
            "policy_sha256": policy_sha256,
            "qemu": {
                "canonical_path": str(qemu_path) if qemu_path is not None else None,
                "required_version": args.required_qemu_version,
                "observed_banner": qemu_version,
                "required_sha256": required_qemu_sha256,
                "observed_sha256": observed_qemu_sha256,
            },
            "guest_artifact": None,
            "build_invocation": architecture_policy.get("build_invocation"),
            "qemu_launch_argv": None,
        },
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
