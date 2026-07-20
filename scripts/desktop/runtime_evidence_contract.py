#!/usr/bin/env python3
"""Shared, fail-closed contract for desktop runtime evidence."""

from __future__ import annotations

import re


CANONICAL_QEMU_VERSION = "9.2.4"
CANONICAL_QEMU_BANNER = f"QEMU emulator version {CANONICAL_QEMU_VERSION}"
RUNTIME_METADATA_SCHEMA = 3
RUN_SUMMARY_SCHEMA = 2
REVIEW_PACKAGE_SCHEMA = 3

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
