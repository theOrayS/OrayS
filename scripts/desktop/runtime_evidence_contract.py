#!/usr/bin/env python3
"""Shared, fail-closed contract for desktop runtime evidence."""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re


CANONICAL_QEMU_VERSION = "9.2.4"
CANONICAL_QEMU_BANNER = f"QEMU emulator version {CANONICAL_QEMU_VERSION}"
RUNTIME_METADATA_SCHEMA = 4
RUN_SUMMARY_SCHEMA = 3
REVIEW_PACKAGE_SCHEMA = 4

FAILURE_REASONS_BY_STAGE = {
    "runtime-prerequisites": {
        "runtime_prerequisites_failure",
        "missing_runtime_prerequisite",
        "missing_qemu_binary",
        "qemu_version_mismatch",
        "qemu_version_override_rejected",
    },
    "runtime-metadata-initial": {"runtime_metadata_initial_failure"},
    "desktop-build": {"desktop_build_failure"},
    "qmp-runtime-setup": {"qmp_runtime_setup_failure"},
    "disk-setup": {"disk_setup_failure"},
    "qemu-boot": {"qemu_boot_failure"},
    "input-injection": {"input_injection_failure"},
    "runtime-resize": {"runtime_resize_failure"},
    "guest-action": {"guest_action_failure"},
    "frame-capture": {"frame_capture_failure"},
    "qemu-shutdown": {"qemu_shutdown_failure"},
    "runtime-metadata-finalize": {"runtime_metadata_finalize_failure"},
    "runner-signal": {
        "runner_signal_failure",
        "signal_interrupted",
        "signal_terminated",
    },
    "complete": {"none"},
}

PRE_START_STAGES = {
    "runtime-prerequisites",
    "runtime-metadata-initial",
    "desktop-build",
    "qmp-runtime-setup",
    "disk-setup",
    "runner-signal",
}

STAGE_TOKEN = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)*")
REASON_TOKEN = re.compile(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*")
SHA256_TOKEN = re.compile(r"[0-9a-f]{64}")


def digest_file(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def load_runtime_policy(path: Path) -> tuple[dict, str]:
    encoded = path.read_bytes()
    value = json.loads(encoded)
    if not isinstance(value, dict) or value.get("schema") != 1:
        raise ValueError("runtime policy schema is invalid")
    if value.get("qemu_version") != CANONICAL_QEMU_VERSION:
        raise ValueError("runtime policy QEMU version is not canonical")
    architectures = value.get("architectures")
    if not isinstance(architectures, dict):
        raise ValueError("runtime policy architectures are invalid")
    return value, hashlib.sha256(encoded).hexdigest()


def _require_sha256(value: object, label: str) -> str:
    if not isinstance(value, str) or SHA256_TOKEN.fullmatch(value) is None:
        raise ValueError(f"{label} is not a lowercase SHA-256 digest")
    return value


def _lexical_absolute_path(value: object, label: str) -> Path:
    if not isinstance(value, str) or not value:
        raise ValueError(f"{label} is missing")
    path = Path(value)
    if not path.is_absolute() or Path(os.path.abspath(value)) != path:
        raise ValueError(f"{label} is not an absolute canonical path")
    return path


def validate_runtime_identity(
    metadata: dict,
    policy_path: Path,
    arch: str,
    *,
    verify_files: bool,
    require_complete: bool = True,
) -> dict:
    if arch not in {"rv", "la"}:
        raise ValueError("runtime identity architecture is invalid")
    policy, policy_sha256 = load_runtime_policy(policy_path)
    architecture_policy = policy["architectures"].get(arch)
    if not isinstance(architecture_policy, dict):
        raise ValueError("runtime policy has no selected architecture")
    identity = metadata.get("runtime_identity")
    if not isinstance(identity, dict) or identity.get("schema") != 1:
        raise ValueError("runtime identity schema is invalid")
    if identity.get("policy_repository_path") != "test/desktop/runtime-policy.json":
        raise ValueError("runtime identity policy path is invalid")
    if identity.get("policy_sha256") != policy_sha256:
        raise ValueError("runtime identity policy digest mismatch")

    qemu = identity.get("qemu")
    if not isinstance(qemu, dict):
        raise ValueError("runtime identity QEMU binding is missing")
    required_qemu_digest = _require_sha256(
        architecture_policy.get("qemu_sha256"), "policy QEMU digest"
    )
    if (
        qemu.get("required_sha256") != required_qemu_digest
        or qemu.get("required_version") != CANONICAL_QEMU_VERSION
    ):
        raise ValueError("QEMU requirements do not match the approved policy")
    partial_qemu = qemu.get("canonical_path") is None
    qemu_path: Path | None = None
    if partial_qemu:
        if require_complete:
            raise ValueError("complete runtime identity has no verified QEMU object")
        if metadata.get("qemu_binary") != architecture_policy.get("qemu_binary"):
            raise ValueError("partial QEMU identity does not bind the policy binary name")
        if qemu.get("observed_sha256") is not None or qemu.get("observed_banner") is not None:
            raise ValueError("partial QEMU identity makes an unverified observation claim")
    else:
        qemu_path = _lexical_absolute_path(
            qemu.get("canonical_path"), "QEMU canonical path"
        )
        observed_qemu_digest = _require_sha256(
            qemu.get("observed_sha256"), "observed QEMU digest"
        )
        if observed_qemu_digest != required_qemu_digest:
            raise ValueError("QEMU digest does not match the approved policy")
        if qemu_path.name != architecture_policy.get("qemu_binary"):
            raise ValueError("QEMU canonical path does not match the approved binary name")
        if qemu.get("observed_banner") != CANONICAL_QEMU_BANNER:
            raise ValueError("QEMU version binding is not the exact canonical banner")
        if verify_files:
            try:
                if qemu_path.resolve(strict=True) != qemu_path:
                    raise ValueError("QEMU canonical path resolves through a different path")
                if digest_file(qemu_path) != observed_qemu_digest:
                    raise ValueError("QEMU digest no longer matches the verified object")
            except OSError as error:
                raise ValueError(f"QEMU identity file verification failed: {error}") from error

    build_invocation = architecture_policy.get("build_invocation")
    if (
        not isinstance(build_invocation, list)
        or not all(isinstance(item, str) and item for item in build_invocation)
        or identity.get("build_invocation") != build_invocation
    ):
        raise ValueError("build invocation does not match the runtime policy")

    artifact = identity.get("guest_artifact")
    launch_argv = identity.get("qemu_launch_argv")
    if partial_qemu:
        if artifact is not None or launch_argv is not None:
            raise ValueError("partial QEMU identity cannot bind a launch or guest artifact")
        return identity
    if artifact is None or launch_argv is None:
        if require_complete or artifact is not None or launch_argv is not None:
            raise ValueError("runtime identity has an incomplete artifact/launch binding")
        return identity
    if not isinstance(artifact, dict) or artifact.get("architecture") != arch:
        raise ValueError("guest artifact architecture binding is invalid")
    repository_path = architecture_policy.get("artifact")
    if (
        not isinstance(repository_path, str)
        or Path(repository_path).is_absolute()
        or ".." in Path(repository_path).parts
        or artifact.get("repository_path") != repository_path
    ):
        raise ValueError("guest artifact repository path is invalid")
    repository_root = _lexical_absolute_path(
        metadata.get("repository_root"), "repository root"
    )
    artifact_path = _lexical_absolute_path(
        artifact.get("canonical_path"), "artifact canonical path"
    )
    expected_artifact = Path(os.path.abspath(repository_root / repository_path))
    if artifact_path != expected_artifact:
        raise ValueError("artifact canonical path does not match the runtime policy")
    artifact_digest = _require_sha256(artifact.get("sha256"), "artifact digest")
    if artifact.get("source_commit") != metadata.get("source_commit_before"):
        raise ValueError("guest artifact is not bound to the source commit")
    if verify_files:
        try:
            if artifact_path.resolve(strict=True) != artifact_path:
                raise ValueError("artifact canonical path resolves through a different path")
            if digest_file(artifact_path) != artifact_digest:
                raise ValueError("artifact digest does not match the built guest artifact")
        except OSError as error:
            raise ValueError(f"artifact identity file verification failed: {error}") from error

    if (
        not isinstance(launch_argv, list)
        or not launch_argv
        or not all(isinstance(item, str) and item for item in launch_argv)
        or launch_argv[0] != str(qemu_path)
    ):
        raise ValueError("QEMU launch argv is invalid or uses another executable")
    if launch_argv.count("-kernel") != 1:
        raise ValueError("QEMU launch argv must contain exactly one guest artifact")
    kernel_index = launch_argv.index("-kernel")
    if kernel_index + 1 >= len(launch_argv) or launch_argv[kernel_index + 1] != str(
        artifact_path
    ):
        raise ValueError("QEMU launch artifact does not match the bound guest artifact")
    return identity


def qemu_version_is_canonical(required: object, observed: object) -> bool:
    return required == CANONICAL_QEMU_VERSION and observed == CANONICAL_QEMU_BANNER


def require_canonical_qemu_version(required: object) -> None:
    if required != CANONICAL_QEMU_VERSION:
        raise ValueError(
            f"required QEMU version must be exactly {CANONICAL_QEMU_VERSION}"
        )


def default_failure_reason(stage: str, runner_exit: int) -> str:
    if runner_exit == 0:
        return "none"
    values = FAILURE_REASONS_BY_STAGE.get(stage)
    if values is None:
        raise ValueError(f"unknown failure stage: {stage}")
    generic = stage.replace("-", "_") + "_failure"
    if generic not in values:
        raise ValueError(f"failure stage has no generic failure reason: {stage}")
    return generic


def validate_runtime_status(
    *,
    qemu_started: object,
    qemu_exit: object,
    runner_exit: object,
    failure_stage: object,
    failure_reason: object,
) -> None:
    if not isinstance(qemu_started, bool):
        raise ValueError("qemu_started must be a boolean")
    if not isinstance(runner_exit, int) or isinstance(runner_exit, bool):
        raise ValueError("runner_exit must be an integer")
    if (
        not isinstance(failure_stage, str)
        or STAGE_TOKEN.fullmatch(failure_stage) is None
        or failure_stage not in FAILURE_REASONS_BY_STAGE
    ):
        raise ValueError("failure_stage is not a known nonempty token")
    if (
        not isinstance(failure_reason, str)
        or REASON_TOKEN.fullmatch(failure_reason) is None
        or failure_reason not in FAILURE_REASONS_BY_STAGE[failure_stage]
    ):
        raise ValueError("failure_reason is invalid or inconsistent with failure_stage")

    if qemu_started:
        if not isinstance(qemu_exit, int) or isinstance(qemu_exit, bool):
            raise ValueError("started QEMU must have an integer exit status")
    elif qemu_exit is not None:
        raise ValueError("QEMU not started must have a null exit status")

    if runner_exit == 0:
        if not qemu_started:
            raise ValueError("QEMU not started requires a nonzero runner_exit")
        if failure_stage != "complete" or failure_reason != "none":
            raise ValueError("zero runner_exit requires complete/none status tokens")
    else:
        if failure_stage == "complete" or failure_reason == "none":
            raise ValueError("nonzero runner_exit requires failure status tokens")
        if not qemu_started and failure_stage not in PRE_START_STAGES:
            raise ValueError("QEMU not started has a post-start failure stage")
