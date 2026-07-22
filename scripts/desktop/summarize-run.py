#!/usr/bin/env python3
"""Validate one desktop QEMU run and write fail-closed evidence metadata."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
import sys
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from runtime_evidence_contract import (  # noqa: E402
    RUNTIME_METADATA_SCHEMA,
    RUN_SUMMARY_SCHEMA,
    SHA256_TOKEN,
    default_failure_reason,
    qemu_version_is_canonical,
    validate_qemu_digest_policy,
    validate_runtime_status,
)


EXPECTED_MARKERS = {
    "boot": [],
    "launcher": ["ORAYS_DESKTOP_ACTION LAUNCHER OPEN"],
    "overlap": ["ORAYS_DESKTOP_ACTION ALT_TAB reverse=false"],
    "applications": [
        "ORAYS_DESKTOP_ACTION LAUNCH Settings",
        "ORAYS_DESKTOP_ACTION THEME Light",
    ],
    "resize": [
        "ORAYS_DESKTOP_DISPLAY_CHANGED width=900 height=650",
        (
            "ORAYS_DESKTOP_INPUT PointerMoved { position: Point { x: 450, y: 325 }, "
            "delta_x: 450, delta_y: 325 }"
        ),
    ],
}

POST_ACTION_STABLE_MARKERS = {
    "launcher": (
        "ORAYS_DESKTOP_ACTION LAUNCHER OPEN",
        "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE",
    ),
}

CAPTURE_READY_MARKERS = {
    "boot": [],
    "overlap": ["ORAYS_DESKTOP_ACTION ALT_TAB reverse=false"],
    "applications": ["ORAYS_DESKTOP_ACTION THEME Light"],
    "resize": ["ORAYS_DESKTOP_DISPLAY_CHANGED width=900 height=650"],
}


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def failure_detail(error: Exception) -> str:
    """Return a location-independent failure description.

    OSError messages embed the absolute path of the missing/unreadable file,
    which would make an otherwise identical failure look different between
    the original run directory and a relocated review package. The recorded
    failure identity must not depend on where the evidence lives.
    """
    if isinstance(error, OSError):
        return error.strerror or str(error)
    return str(error)


def validate_runtime_metadata(path: Path, arch: str, scenario: str) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict) or value.get("schema") != RUNTIME_METADATA_SCHEMA:
        raise ValueError("runtime metadata schema is invalid")
    if value.get("architecture") != arch or value.get("scenario") != scenario:
        raise ValueError("runtime metadata architecture or scenario is invalid")
    before_commit = value.get("source_commit_before")
    after_commit = value.get("source_commit_after")
    before_status = value.get("source_status_before")
    after_status = value.get("source_status_after")
    if any(
        re.fullmatch(r"[0-9a-f]{40}", str(commit)) is None
        for commit in (before_commit, after_commit)
    ):
        raise ValueError("runtime metadata source commits are invalid")
    if not all(
        isinstance(status, list) and all(isinstance(line, str) for line in status)
        for status in (before_status, after_status)
    ):
        raise ValueError("runtime metadata source statuses are invalid")
    if any(
        not isinstance(value.get(name), bool)
        for name in ("source_dirty_before", "source_dirty_after", "provenance_stable")
    ):
        raise ValueError("runtime metadata provenance flags are invalid")
    if (
        value.get("source_dirty_before") != bool(before_status)
        or value.get("source_dirty_after") != bool(after_status)
    ):
        raise ValueError("runtime metadata dirty flags disagree with source statuses")
    if (
        value.get("source_commit") != before_commit
        or value.get("source_status") != before_status
        or value.get("source_dirty") != value.get("source_dirty_before")
    ):
        raise ValueError("runtime metadata compatibility source fields disagree")
    expected_stable = (
        before_commit == after_commit
        and before_status == after_status
        and not before_status
        and not after_status
    )
    if value.get("provenance_stable") is not expected_stable:
        raise ValueError("runtime metadata provenance_stable is inconsistent")
    errors = value.get("collection_errors")
    if not isinstance(errors, list) or not all(isinstance(error, str) for error in errors):
        raise ValueError("runtime metadata collection errors are invalid")
    if errors:
        raise ValueError("runtime metadata collection was incomplete: " + "; ".join(errors))
    if not expected_stable:
        raise ValueError("source commit or status changed during the run")
    run_dir_value = value.get("run_dir")
    if not isinstance(run_dir_value, str) or not Path(run_dir_value).is_absolute():
        raise ValueError("runtime metadata run directory is invalid")
    if not isinstance(value.get("qemu_binary"), str) or not value.get("qemu_version"):
        raise ValueError("runtime metadata QEMU identity is invalid")
    required_qemu_version = value.get("required_qemu_version")
    observed_qemu_version = value.get("observed_qemu_version")
    expected_version_match = qemu_version_is_canonical(
        required_qemu_version, observed_qemu_version
    )
    if value.get("qemu_version") != observed_qemu_version:
        raise ValueError("runtime metadata observed QEMU version fields disagree")
    if value.get("qemu_version_matches_required") is not expected_version_match:
        raise ValueError("runtime metadata QEMU version match flag is inconsistent")
    if not expected_version_match:
        raise ValueError(
            "QEMU version mismatch: "
            f"required {required_qemu_version}, observed {observed_qemu_version}"
        )
    validate_qemu_digest_policy(
        policy=value.get("qemu_digest_policy"),
        authorized_sha256=value.get("qemu_authorized_sha256"),
        matches_authorized=value.get("qemu_digest_matches_authorized"),
        qemu_sha256=value.get("qemu_sha256"),
    )
    qemu_binary_path = Path(value["qemu_binary"])
    if not qemu_binary_path.is_absolute():
        raise ValueError("runtime metadata QEMU binary path is not absolute")
    if qemu_binary_path.is_file() and digest(qemu_binary_path) != value["qemu_sha256"]:
        raise ValueError("runtime metadata QEMU digest does not match the binary")
    argv = value.get("qemu_argv")
    if (
        not isinstance(argv, list)
        or not argv
        or not all(isinstance(item, str) and item for item in argv)
    ):
        raise ValueError("runtime metadata QEMU argv is invalid")
    if argv[0] != value["qemu_binary"]:
        raise ValueError("runtime metadata QEMU argv[0] does not match the QEMU binary")
    artifact = value.get("guest_artifact")
    if not isinstance(artifact, dict) or set(artifact) != {
        "path",
        "type",
        "size",
        "sha256",
        "architecture",
    }:
        raise ValueError("runtime metadata guest artifact identity is invalid")
    if artifact["architecture"] != arch:
        raise ValueError("runtime metadata guest artifact architecture is invalid")
    if artifact["type"] not in {"elf", "raw-binary"}:
        raise ValueError("runtime metadata guest artifact type is invalid")
    if (
        not isinstance(artifact["size"], int)
        or isinstance(artifact["size"], bool)
        or artifact["size"] <= 0
    ):
        raise ValueError("runtime metadata guest artifact size is invalid")
    if (
        not isinstance(artifact["path"], str)
        or not Path(artifact["path"]).is_absolute()
        or not isinstance(artifact["sha256"], str)
        or SHA256_TOKEN.fullmatch(artifact["sha256"]) is None
    ):
        raise ValueError("runtime metadata guest artifact path or digest is invalid")
    kernel_index = [index for index, item in enumerate(argv) if item == "-kernel"]
    if (
        len(kernel_index) != 1
        or kernel_index[0] + 1 >= len(argv)
        or argv[kernel_index[0] + 1] != artifact["path"]
    ):
        raise ValueError("runtime metadata QEMU argv is not bound to the guest artifact")
    artifact_path = Path(artifact["path"])
    if artifact_path.is_file():
        if (
            artifact_path.stat().st_size != artifact["size"]
            or digest(artifact_path) != artifact["sha256"]
        ):
            raise ValueError(
                "runtime metadata guest artifact identity does not match the file"
            )
        with artifact_path.open("rb") as stream:
            magic = stream.read(4)
        if (magic == b"\x7fELF") != (artifact["type"] == "elf"):
            raise ValueError("runtime metadata guest artifact type does not match the file")
    runner_inputs = value.get("runner_inputs")
    if not isinstance(runner_inputs, dict) or set(runner_inputs) != {
        "vnc_display",
        "qemu_timeout_seconds",
    }:
        raise ValueError("runtime metadata runner inputs are invalid")
    vnc_display = runner_inputs["vnc_display"]
    qemu_timeout = runner_inputs["qemu_timeout_seconds"]
    if (
        not isinstance(vnc_display, int)
        or isinstance(vnc_display, bool)
        or vnc_display < 0
        or not isinstance(qemu_timeout, int)
        or isinstance(qemu_timeout, bool)
        or qemu_timeout <= 0
    ):
        raise ValueError("runtime metadata runner input values are invalid")
    toolchains = value.get("toolchain_versions")
    if not isinstance(toolchains, dict) or any(
        not isinstance(toolchains.get(name), str) or not toolchains[name]
        for name in ("rustc", "cargo", "python")
    ):
        raise ValueError("runtime metadata toolchain versions are invalid")
    command = value.get("generation_command")
    if not isinstance(command, list) or command[:5] != [
        "scripts/desktop/run-headless-qemu.sh",
        "--arch",
        arch,
        "--scenario",
        scenario,
    ]:
        raise ValueError("runtime metadata generation command is invalid")
    return value


def validate_capture_precondition(
    path: Path,
    serial_path: Path,
    action_marker: str,
    stable_marker: str,
) -> None:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict) or value.get("schema") != 1:
        raise ValueError("capture precondition schema is invalid")
    if value.get("action_marker") != action_marker:
        raise ValueError("capture precondition action marker is invalid")
    if value.get("stable_marker") != stable_marker:
        raise ValueError("capture precondition stable marker is invalid")
    prefix_size = value.get("serial_prefix_bytes")
    if not isinstance(prefix_size, int) or prefix_size <= 0:
        raise ValueError("capture precondition serial prefix size is invalid")
    serial = serial_path.read_bytes()
    if prefix_size > len(serial):
        raise ValueError("capture precondition serial prefix exceeds the final log")
    prefix = serial[:prefix_size]
    if hashlib.sha256(prefix).hexdigest() != value.get("serial_prefix_sha256"):
        raise ValueError("capture precondition serial prefix hash does not match")

    lines = prefix.decode("utf-8", errors="replace").replace("\x00", "").splitlines()
    action_lines = [index for index, line in enumerate(lines, 1) if line == action_marker]
    stable_lines = [index for index, line in enumerate(lines, 1) if line == stable_marker]
    if len(action_lines) != 1 or value.get("action_line") != action_lines[0]:
        raise ValueError("capture precondition action line is missing or ambiguous")
    if len(stable_lines) != 1 or value.get("stable_line") != stable_lines[0]:
        raise ValueError("capture precondition stable line is missing or ambiguous")
    if stable_lines[0] <= action_lines[0]:
        raise ValueError("capture precondition stable marker is not after the action")


def validate_required_markers_precondition(
    path: Path,
    serial_path: Path,
    required_markers: list[str],
) -> None:
    value = json.loads(path.read_text(encoding="utf-8"))
    if (
        not isinstance(value, dict)
        or value.get("schema") != 1
        or value.get("kind") != "required-markers"
    ):
        raise ValueError("capture marker precondition schema is invalid")
    prefix_size = value.get("serial_prefix_bytes")
    if not isinstance(prefix_size, int) or prefix_size <= 0:
        raise ValueError("capture marker precondition serial prefix size is invalid")
    serial = serial_path.read_bytes()
    if prefix_size > len(serial):
        raise ValueError("capture marker precondition prefix exceeds the final log")
    prefix = serial[:prefix_size]
    if hashlib.sha256(prefix).hexdigest() != value.get("serial_prefix_sha256"):
        raise ValueError("capture marker precondition prefix hash does not match")
    lines = prefix.decode("utf-8", errors="replace").replace("\x00", "").splitlines()
    records = value.get("markers")
    if not isinstance(records, list) or len(records) != len(required_markers):
        raise ValueError("capture marker precondition list is invalid")
    previous_line = 0
    for marker, record in zip(required_markers, records, strict=True):
        matches = [index for index, line in enumerate(lines, 1) if line == marker]
        if (
            len(matches) != 1
            or not isinstance(record, dict)
            or record != {"marker": marker, "line": matches[0]}
            or matches[0] <= previous_line
        ):
            raise ValueError("capture marker precondition is missing, ambiguous, or unordered")
        previous_line = matches[0]


def validate_presented_input_order(
    serial_lines: list[str],
    prerequisite_marker: str,
    input_marker: str,
) -> None:
    prerequisite_lines = [
        index for index, line in enumerate(serial_lines) if line == prerequisite_marker
    ]
    input_lines = [index for index, line in enumerate(serial_lines) if line == input_marker]
    if len(prerequisite_lines) != 1 or len(input_lines) != 1:
        raise ValueError("input barrier markers are missing or ambiguous")
    if not any(
        prerequisite_lines[0] < index < input_lines[0]
        for index, line in enumerate(serial_lines)
        if line.startswith("ORAYS_DESKTOP_FRAME input ")
    ):
        raise ValueError("no presented input frame exists before the processed-input marker")


def ppm_geometry(path: Path) -> tuple[int, int]:
    with path.open("rb") as stream:
        magic = stream.readline().strip()
        dimensions = stream.readline().strip().split()
        maximum = stream.readline().strip()
    if magic != b"P6" or len(dimensions) != 2 or maximum != b"255":
        raise ValueError("screenshot is not a binary 8-bit PPM")
    width, height = (int(value) for value in dimensions)
    if width <= 0 or height <= 0:
        raise ValueError("screenshot has invalid dimensions")
    return width, height


def read_json_lines(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for line_number, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.strip():
            continue
        value = json.loads(raw)
        if (
            not isinstance(value, dict)
            or value.get("direction") not in {"send", "receive"}
            or not isinstance(value.get("message"), dict)
        ):
            raise ValueError(f"invalid QMP transcript row {line_number}")
        rows.append(value)
    if not rows:
        raise ValueError("QMP transcript is empty")
    return rows


def successful_qmp_commands(path: Path) -> list[dict[str, Any]]:
    rows = read_json_lines(path)
    greeting = rows[0]
    if greeting["direction"] != "receive" or "QMP" not in greeting["message"]:
        raise ValueError("QMP greeting is missing")

    commands: list[dict[str, Any]] = []
    pending = False
    for row in rows[1:]:
        direction = row["direction"]
        message = row["message"]
        if direction == "send":
            if pending or not isinstance(message.get("execute"), str):
                raise ValueError("QMP command ordering is invalid")
            commands.append(message)
            pending = True
        elif "event" in message:
            continue
        elif not pending:
            raise ValueError("QMP response has no pending command")
        elif "error" in message or "return" not in message:
            raise ValueError("QMP command did not return success")
        else:
            pending = False
    if pending:
        raise ValueError("QMP command has no response")
    return commands


def load_sequence(path: Path) -> list[dict[str, Any]]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, list) or not value:
        raise ValueError("input sequence must be a non-empty array")
    for index, step in enumerate(value):
        if not isinstance(step, dict):
            raise ValueError(f"input sequence step {index} is not an object")
        events = step.get("events")
        wait_ms = step.get("wait_ms", 0)
        if not isinstance(wait_ms, int) or not 0 <= wait_ms <= 10_000:
            raise ValueError(f"input sequence step {index} has invalid wait")
        if events is None:
            if wait_ms == 0:
                raise ValueError(f"input sequence step {index} has no work")
        elif not isinstance(events, list) or not events or not all(
            isinstance(event, dict) and isinstance(event.get("data"), dict)
            for event in events
        ):
            raise ValueError(f"input sequence step {index} has invalid events")
    return value


def resolve_event(event: dict[str, Any], geometry: tuple[int, int]) -> dict[str, Any]:
    if event.get("type") != "abs-pixel":
        return event
    data = event["data"]
    axis = data.get("axis")
    coordinate = data.get("coordinate")
    if axis not in {"x", "y"} or not isinstance(coordinate, dict):
        raise ValueError("invalid abs-pixel event")
    extent = geometry[0 if axis == "x" else 1]
    anchor = coordinate.get("anchor")
    offset = coordinate.get("offset")
    if anchor not in {"start", "center", "end"} or not isinstance(offset, int):
        raise ValueError("invalid abs-pixel coordinate")
    base = {"start": 0, "center": extent // 2, "end": extent - 1}[anchor]
    pixel = base + offset
    if extent <= 1 or not 0 <= pixel < extent:
        raise ValueError("abs-pixel coordinate is outside the guest display")
    value = (pixel * 32767 + extent - 2) // (extent - 1)
    return {"type": "abs", "data": {"axis": axis, "value": value}}


def validate_input_evidence(
    sequence_path: Path,
    transcript_path: Path,
    geometry: tuple[int, int],
) -> None:
    steps = load_sequence(sequence_path)
    expected_events = [
        [resolve_event(event, geometry) for event in step["events"]]
        for step in steps
        if "events" in step
    ]
    commands = successful_qmp_commands(transcript_path)
    if not commands or commands[0] != {"execute": "qmp_capabilities"}:
        raise ValueError("input QMP capabilities negotiation is invalid")
    input_commands = commands[1:]
    if len(input_commands) != len(expected_events):
        raise ValueError("input QMP command count does not match the sequence")
    for command, events in zip(input_commands, expected_events, strict=True):
        if command != {
            "execute": "input-send-event",
            "arguments": {"events": events},
        }:
            raise ValueError("input QMP command does not match the sequence")


def validate_capture_evidence(
    transcript_path: Path,
    screenshot: Path,
    original_screenshot: Path | None = None,
) -> None:
    commands = successful_qmp_commands(transcript_path)
    if [command.get("execute") for command in commands] != [
        "qmp_capabilities",
        "screendump",
        "quit",
    ]:
        raise ValueError("capture QMP command sequence is invalid")
    filename = commands[1].get("arguments", {}).get("filename")
    expected_target = original_screenshot or screenshot
    if not isinstance(filename, str) or Path(filename).resolve() != expected_target.resolve():
        raise ValueError("screendump target does not match the captured frame")


def validate_resize_evidence(
    path: Path,
    initial_geometry: tuple[int, int],
    geometry: tuple[int, int],
) -> None:
    value = json.loads(path.read_text(encoding="utf-8"))
    expected_fields = {
        "schema": 1,
        "transport": "RFB SetDesktopSize",
        "security_type": "None",
        "requested_geometry": list(geometry),
        "screen_count": 1,
        "extended_desktop_size_encoding": -308,
    }
    if not isinstance(value, dict) or any(
        value.get(key) != expected for key, expected in expected_fields.items()
    ):
        raise ValueError("resize VNC evidence fields or geometry are invalid")
    if value.get("initial_geometry") != list(initial_geometry):
        raise ValueError("resize VNC initial geometry is invalid")
    if not isinstance(value.get("endpoint"), str) or re.fullmatch(
        r"127\.0\.0\.1:[0-9]+", value["endpoint"]
    ) is None:
        raise ValueError("resize VNC endpoint is not localhost")
    if not isinstance(value.get("server_version"), str) or not value[
        "server_version"
    ].startswith("RFB 003."):
        raise ValueError("resize VNC server version is invalid")


def screenshot_is_uniform(path: Path) -> bool:
    with path.open("rb") as stream:
        stream.readline()
        stream.readline()
        stream.readline()
        first = stream.read(3)
        if len(first) != 3:
            return True
        for pixel in iter(lambda: stream.read(3), b""):
            if len(pixel) != 3 or pixel != first:
                return False
    return True


def validate_run(
    run_dir: Path,
    arch: str,
    scenario: str,
    qemu_exit: int | None,
    *,
    qemu_started: bool = True,
    original_screenshot: Path | None = None,
    additional_failures: list[str] | None = None,
) -> tuple[list[str], tuple[int, int] | None, dict[str, str]]:
    serial = run_dir / "serial.log"
    screenshot = run_dir / "frame.ppm"
    capture_transcript = run_dir / "qmp-capture.jsonl"
    input_transcript = run_dir / "qmp-input.jsonl"
    sequence = run_dir / "input-sequence.json"
    metadata_path = run_dir / "runtime-metadata.json"

    failures: list[str] = list(additional_failures or [])
    try:
        validate_runtime_metadata(metadata_path, arch, scenario)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        failures.append(f"invalid runtime metadata: {failure_detail(error)}")
    serial_text = serial.read_text(encoding="utf-8", errors="replace") if serial.exists() else ""
    serial_lines = serial_text.replace("\x00", "").splitlines()
    if qemu_started:
        if not isinstance(qemu_exit, int):
            failures.append("QEMU started but exit status is unavailable")
        elif qemu_exit != 0:
            failures.append(f"qemu exit was {qemu_exit}, expected 0")
    elif qemu_exit is not None:
        failures.append("QEMU was not started but an exit status was recorded")
    else:
        failures.append("QEMU was not started")
    if "ORAYS_DESKTOP_FRAME boot " not in serial_text:
        failures.append("guest boot frame marker missing")
    if "ORAYS_DESKTOP_INPUT_READY devices=2" not in serial_lines:
        failures.append("guest input readiness marker missing or device count is not 2")
    for marker in EXPECTED_MARKERS[scenario]:
        if marker not in serial_lines:
            failures.append(f"guest action marker missing: {marker}")
    if scenario == "resize":
        try:
            validate_presented_input_order(
                serial_lines,
                EXPECTED_MARKERS["resize"][0],
                EXPECTED_MARKERS["resize"][1],
            )
        except ValueError as error:
            failures.append(f"invalid resize input presentation order: {failure_detail(error)}")
    stable_pair = POST_ACTION_STABLE_MARKERS.get(scenario)
    if stable_pair is not None:
        action_marker, stable_marker = stable_pair
        action_lines = [index for index, line in enumerate(serial_lines) if line == action_marker]
        stable_lines = [index for index, line in enumerate(serial_lines) if line == stable_marker]
        if len(stable_lines) != 1:
            failures.append(
                f"guest stable marker missing or ambiguous: {stable_marker}"
            )
        elif len(action_lines) != 1 or stable_lines[0] <= action_lines[0]:
            failures.append("guest stable marker does not uniquely follow the launcher action")
        try:
            validate_capture_precondition(
                run_dir / "capture-precondition.json",
                serial,
                action_marker,
                stable_marker,
            )
        except (OSError, ValueError, json.JSONDecodeError) as error:
            failures.append(f"invalid capture precondition evidence: {failure_detail(error)}")

    initial_display_matches = re.findall(
        r"^ORAYS_DESKTOP_DISPLAY width=([0-9]+) height=([0-9]+)$",
        serial_text,
        re.MULTILINE,
    )
    initial_geometry = None
    if len(initial_display_matches) != 1:
        failures.append("guest display geometry marker missing or ambiguous")
    else:
        initial_geometry = tuple(int(value) for value in initial_display_matches[0])

    changed_display_matches = re.findall(
        r"^ORAYS_DESKTOP_DISPLAY_CHANGED width=([0-9]+) height=([0-9]+)$",
        serial_text,
        re.MULTILINE,
    )
    changed_geometries = [
        tuple(int(value) for value in match) for match in changed_display_matches
    ]
    display_geometry = initial_geometry
    if scenario == "resize":
        if not changed_geometries or changed_geometries[-1] != (900, 650):
            failures.append("guest final runtime resize marker missing or invalid")
        else:
            display_geometry = changed_geometries[-1]
        try:
            if initial_geometry is None:
                raise ValueError("initial display geometry is unavailable")
            validate_resize_evidence(
                run_dir / "vnc-resize.json", initial_geometry, (900, 650)
            )
        except (OSError, ValueError, json.JSONDecodeError) as error:
            failures.append(f"invalid resize evidence: {failure_detail(error)}")
    elif changed_geometries:
        failures.append("unexpected guest runtime resize marker")

    if stable_pair is None and initial_geometry is not None:
        required_markers = [
            f"ORAYS_DESKTOP_DISPLAY width={initial_geometry[0]} height={initial_geometry[1]}",
            *CAPTURE_READY_MARKERS[scenario],
        ]
        try:
            validate_required_markers_precondition(
                run_dir / "capture-precondition.json", serial, required_markers
            )
        except (OSError, ValueError, json.JSONDecodeError) as error:
            failures.append(f"invalid capture precondition evidence: {failure_detail(error)}")

    try:
        geometry_text = (run_dir / "display-geometry.txt").read_text(encoding="utf-8")
        expected_geometry_text = (
            f"DISPLAY_GEOMETRY={display_geometry[0]}x{display_geometry[1]}\n"
            if display_geometry is not None
            else None
        )
        if expected_geometry_text is None or geometry_text != expected_geometry_text:
            raise ValueError("display geometry sidecar does not match guest markers")
    except (OSError, ValueError) as error:
        failures.append(f"invalid display geometry evidence: {failure_detail(error)}")

    geometry: tuple[int, int] | None = None
    try:
        geometry = ppm_geometry(screenshot)
        expected_size = len(f"P6\n{geometry[0]} {geometry[1]}\n255\n".encode()) + geometry[0] * geometry[1] * 3
        if screenshot.stat().st_size != expected_size:
            failures.append(
                f"screenshot byte size {screenshot.stat().st_size} != expected {expected_size}"
            )
        if display_geometry is not None and geometry != display_geometry:
            failures.append("screenshot geometry does not match the guest display marker")
        if screenshot_is_uniform(screenshot):
            failures.append("screenshot is a uniform frame")
    except (OSError, ValueError) as error:
        failures.append(f"invalid screenshot: {failure_detail(error)}")

    input_geometry = display_geometry if scenario == "resize" else initial_geometry
    if input_geometry is not None:
        try:
            validate_input_evidence(sequence, input_transcript, input_geometry)
        except (OSError, ValueError, json.JSONDecodeError) as error:
            failures.append(f"invalid input evidence: {failure_detail(error)}")
    try:
        validate_capture_evidence(capture_transcript, screenshot, original_screenshot)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        failures.append(f"invalid capture evidence: {failure_detail(error)}")

    evidence_names = [
        "serial.log",
        "qmp-capture.jsonl",
        "qmp-input.jsonl",
        "input-sequence.json",
        "frame.ppm",
        "runtime-metadata.json",
        "display-geometry.txt",
        "capture-precondition.json",
    ]
    if scenario == "resize":
        evidence_names.append("vnc-resize.json")
    hashes = {
        name: digest(run_dir / name)
        for name in evidence_names
        if (run_dir / name).is_file()
    }
    return failures, geometry, hashes


def parse_bool(value: str) -> bool:
    if value == "true":
        return True
    if value == "false":
        return False
    raise argparse.ArgumentTypeError("expected true or false")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--arch", choices=("rv", "la"), required=True)
    parser.add_argument("--scenario", choices=tuple(EXPECTED_MARKERS), required=True)
    parser.add_argument("--qemu-started", type=parse_bool, default=True)
    parser.add_argument("--qemu-exit", type=int)
    parser.add_argument("--runner-exit", type=int, default=0)
    parser.add_argument("--failure-stage")
    parser.add_argument("--failure-reason")
    args = parser.parse_args()
    if args.qemu_started and args.qemu_exit is None:
        parser.error("--qemu-exit is required when --qemu-started=true")
    if not args.qemu_started and args.qemu_exit is not None:
        parser.error("--qemu-exit must be omitted when --qemu-started=false")

    failure_stage = args.failure_stage
    if failure_stage is None and args.runner_exit == 0:
        failure_stage = "complete"
    failure_reason = args.failure_reason
    if failure_reason is None and failure_stage is not None:
        try:
            failure_reason = default_failure_reason(failure_stage, args.runner_exit)
        except ValueError as error:
            parser.error(str(error))
    try:
        validate_runtime_status(
            qemu_started=args.qemu_started,
            qemu_exit=args.qemu_exit,
            runner_exit=args.runner_exit,
            failure_stage=failure_stage,
            failure_reason=failure_reason,
        )
    except ValueError as error:
        parser.error(str(error))

    run_dir = args.run_dir.resolve()
    additional_failures = []
    if args.runner_exit != 0:
        additional_failures.append(
            f"runtime runner exited {args.runner_exit} during stage {failure_stage}"
        )
    failures, geometry, hashes = validate_run(
        run_dir,
        args.arch,
        args.scenario,
        args.qemu_exit,
        qemu_started=args.qemu_started,
        additional_failures=additional_failures,
    )
    serial_path = run_dir / "serial.log"
    serial_text = (
        serial_path.read_text(encoding="utf-8", errors="replace")
        if serial_path.is_file()
        else ""
    )
    (run_dir / "hashes.sha256").write_text(
        "".join(f"{value}  {name}\n" for name, value in sorted(hashes.items())),
        encoding="utf-8",
    )

    result = "PASS" if not failures else "FAIL"
    summary = {
        "schema": RUN_SUMMARY_SCHEMA,
        "result": result,
        "architecture": args.arch,
        "scenario": args.scenario,
        "qemu_started": args.qemu_started,
        "qemu_exit": args.qemu_exit,
        "runner_exit": args.runner_exit,
        "failure_stage": failure_stage,
        "failure_reason": failure_reason,
        "frame_marker_count": serial_text.count("ORAYS_DESKTOP_FRAME "),
        "input_marker_count": serial_text.count("ORAYS_DESKTOP_INPUT "),
        "post_action_state_marker_count": serial_text.count(
            "ORAYS_DESKTOP_STATE "
        ),
        "screenshot": {
            "width": geometry[0] if geometry else None,
            "height": geometry[1] if geometry else None,
            "sha256": hashes.get("frame.ppm"),
        },
        "hashes": hashes,
        "failures": failures,
    }
    (run_dir / "summary.json").write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(f"DESKTOP_QEMU_{args.scenario.upper()}={result} evidence={run_dir}")
    for failure in failures:
        print(f"FAIL {failure}")
    return 0 if result == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
