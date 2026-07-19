#!/usr/bin/env python3
"""Validate one desktop QEMU run and write fail-closed evidence metadata."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any


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


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


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


def validate_capture_evidence(transcript_path: Path, screenshot: Path) -> None:
    commands = successful_qmp_commands(transcript_path)
    if [command.get("execute") for command in commands] != [
        "qmp_capabilities",
        "screendump",
        "quit",
    ]:
        raise ValueError("capture QMP command sequence is invalid")
    filename = commands[1].get("arguments", {}).get("filename")
    if not isinstance(filename, str) or Path(filename).resolve() != screenshot.resolve():
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
    qemu_exit: int,
) -> tuple[list[str], tuple[int, int] | None, dict[str, str]]:
    serial = run_dir / "serial.log"
    screenshot = run_dir / "frame.ppm"
    capture_transcript = run_dir / "qmp-capture.jsonl"
    input_transcript = run_dir / "qmp-input.jsonl"
    sequence = run_dir / "input-sequence.json"

    failures: list[str] = []
    serial_text = serial.read_text(encoding="utf-8", errors="replace") if serial.exists() else ""
    serial_lines = serial_text.replace("\x00", "").splitlines()
    if qemu_exit != 0:
        failures.append(f"qemu exit was {qemu_exit}, expected 0")
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
            failures.append(f"invalid resize input presentation order: {error}")
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
            failures.append(f"invalid capture precondition evidence: {error}")

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
            failures.append(f"invalid resize evidence: {error}")
    elif changed_geometries:
        failures.append("unexpected guest runtime resize marker")

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
        failures.append(f"invalid screenshot: {error}")

    input_geometry = display_geometry if scenario == "resize" else initial_geometry
    if input_geometry is not None:
        try:
            validate_input_evidence(sequence, input_transcript, input_geometry)
        except (OSError, ValueError, json.JSONDecodeError) as error:
            failures.append(f"invalid input evidence: {error}")
    try:
        validate_capture_evidence(capture_transcript, screenshot)
    except (OSError, ValueError, json.JSONDecodeError) as error:
        failures.append(f"invalid capture evidence: {error}")

    evidence_names = [
        "serial.log",
        "qmp-capture.jsonl",
        "qmp-input.jsonl",
        "input-sequence.json",
        "frame.ppm",
    ]
    if scenario in POST_ACTION_STABLE_MARKERS:
        evidence_names.append("capture-precondition.json")
    if scenario == "resize":
        evidence_names.append("vnc-resize.json")
    hashes = {
        name: digest(run_dir / name)
        for name in evidence_names
        if (run_dir / name).is_file()
    }
    return failures, geometry, hashes


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--arch", choices=("rv", "la"), required=True)
    parser.add_argument("--scenario", choices=tuple(EXPECTED_MARKERS), required=True)
    parser.add_argument("--qemu-exit", type=int, required=True)
    args = parser.parse_args()

    run_dir = args.run_dir.resolve()
    failures, geometry, hashes = validate_run(
        run_dir,
        args.arch,
        args.scenario,
        args.qemu_exit,
    )
    serial_text = (run_dir / "serial.log").read_text(encoding="utf-8", errors="replace")
    (run_dir / "hashes.sha256").write_text(
        "".join(f"{value}  {name}\n" for name, value in sorted(hashes.items())),
        encoding="utf-8",
    )

    result = "PASS" if not failures else "FAIL"
    summary = {
        "schema": 1,
        "result": result,
        "architecture": args.arch,
        "scenario": args.scenario,
        "qemu_exit": args.qemu_exit,
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
