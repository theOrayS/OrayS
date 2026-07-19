#!/usr/bin/env python3
"""Validate and inject deterministic input-send-event steps through QMP."""

from __future__ import annotations

import argparse
import json
import socket
import time
from pathlib import Path
from typing import Any, BinaryIO


class InputInjectionError(RuntimeError):
    pass


def validate_event(event: Any) -> None:
    if not isinstance(event, dict) or event.get("type") not in {
        "key",
        "btn",
        "rel",
        "abs",
        "abs-pixel",
    }:
        raise InputInjectionError(f"unsupported QMP input event: {event!r}")
    data = event.get("data")
    if not isinstance(data, dict):
        raise InputInjectionError(f"input event has no data object: {event!r}")
    event_type = event["type"]
    if event_type == "key":
        key = data.get("key")
        if not isinstance(data.get("down"), bool) or not isinstance(key, dict):
            raise InputInjectionError(f"invalid key event: {event!r}")
        if key.get("type") not in {"qcode", "number"} or "data" not in key:
            raise InputInjectionError(f"invalid key descriptor: {key!r}")
    elif event_type == "btn":
        if not isinstance(data.get("down"), bool) or not isinstance(data.get("button"), str):
            raise InputInjectionError(f"invalid button event: {event!r}")
    elif event_type in {"rel", "abs"}:
        if data.get("axis") not in {"x", "y"} or not isinstance(data.get("value"), int):
            raise InputInjectionError(f"invalid axis event: {event!r}")
    else:
        coordinate = data.get("coordinate")
        if (
            data.get("axis") not in {"x", "y"}
            or not isinstance(coordinate, dict)
            or coordinate.get("anchor") not in {"start", "center", "end"}
            or not isinstance(coordinate.get("offset"), int)
        ):
            raise InputInjectionError(f"invalid pixel axis event: {event!r}")


def resolve_event(event: dict[str, Any], display_size: tuple[int, int] | None) -> dict[str, Any]:
    if event["type"] != "abs-pixel":
        return event
    if display_size is None:
        raise InputInjectionError("abs-pixel event requires display geometry")
    axis = event["data"]["axis"]
    extent = display_size[0 if axis == "x" else 1]
    if extent <= 1:
        raise InputInjectionError(f"invalid display extent for {axis}: {extent}")
    coordinate = event["data"]["coordinate"]
    anchor = coordinate["anchor"]
    base = {"start": 0, "center": extent // 2, "end": extent - 1}[anchor]
    pixel = base + coordinate["offset"]
    if not 0 <= pixel < extent:
        raise InputInjectionError(
            f"resolved {axis} pixel {pixel} is outside display extent {extent}"
        )
    # VirtIO tablets report the QMP absolute range as 0..32767. Round up so
    # the guest's integer normalization resolves to the requested pixel.
    value = (pixel * 32767 + extent - 2) // (extent - 1)
    return {"type": "abs", "data": {"axis": axis, "value": value}}


def load_sequence(path: Path) -> list[dict[str, Any]]:
    raw = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(raw, list) or not raw:
        raise InputInjectionError("input sequence must be a non-empty JSON array")
    steps: list[dict[str, Any]] = []
    for index, step in enumerate(raw):
        if not isinstance(step, dict):
            raise InputInjectionError(f"step {index} is not an object")
        wait_ms = step.get("wait_ms", 0)
        if not isinstance(wait_ms, int) or not 0 <= wait_ms <= 10_000:
            raise InputInjectionError(f"step {index} has invalid wait_ms")
        events = step.get("events")
        if events is not None:
            if not isinstance(events, list) or not events:
                raise InputInjectionError(f"step {index} events must be a non-empty array")
            for event in events:
                validate_event(event)
        elif wait_ms == 0:
            raise InputInjectionError(f"step {index} has neither events nor a wait")
        steps.append(step)
    return steps


def read_message(stream: BinaryIO, transcript: list[dict[str, Any]]) -> dict[str, Any]:
    while True:
        line = stream.readline()
        if not line:
            raise InputInjectionError("QMP socket closed")
        message = json.loads(line.decode("utf-8"))
        transcript.append({"direction": "receive", "message": message})
        if "event" not in message:
            return message


def send_message(
    stream: BinaryIO,
    message: dict[str, Any],
    transcript: list[dict[str, Any]],
) -> dict[str, Any]:
    transcript.append({"direction": "send", "message": message})
    stream.write((json.dumps(message, separators=(",", ":")) + "\r\n").encode("utf-8"))
    stream.flush()
    response = read_message(stream, transcript)
    if "error" in response:
        raise InputInjectionError(json.dumps(response["error"], ensure_ascii=False))
    if "return" not in response:
        raise InputInjectionError(f"invalid QMP response: {response!r}")
    return response


def connect(socket_path: Path, timeout: float) -> socket.socket:
    deadline = time.monotonic() + timeout
    while not socket_path.exists():
        if time.monotonic() >= deadline:
            raise InputInjectionError(f"QMP socket did not appear: {socket_path}")
        time.sleep(0.05)
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.settimeout(timeout)
    sock.connect(str(socket_path))
    return sock


def inject(
    socket_path: Path,
    steps: list[dict[str, Any]],
    timeout: float,
    transcript: list[dict[str, Any]] | None = None,
    display_size: tuple[int, int] | None = None,
) -> list[dict[str, Any]]:
    if transcript is None:
        transcript = []
    with connect(socket_path, timeout) as sock:
        stream = sock.makefile("rwb", buffering=0)
        greeting = read_message(stream, transcript)
        if "QMP" not in greeting:
            raise InputInjectionError(f"invalid QMP greeting: {greeting!r}")
        send_message(stream, {"execute": "qmp_capabilities"}, transcript)
        for step in steps:
            events = step.get("events")
            if events:
                resolved_events = [resolve_event(event, display_size) for event in events]
                send_message(
                    stream,
                    {
                        "execute": "input-send-event",
                        "arguments": {"events": resolved_events},
                    },
                    transcript,
                )
            wait_ms = step.get("wait_ms", 0)
            if wait_ms:
                time.sleep(wait_ms / 1000)
    return transcript


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sequence", type=Path, required=True)
    parser.add_argument("--socket", type=Path)
    parser.add_argument("--transcript", type=Path)
    parser.add_argument("--timeout", type=float, default=15.0)
    parser.add_argument("--display-width", type=int)
    parser.add_argument("--display-height", type=int)
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args()

    steps = load_sequence(args.sequence)
    event_steps = sum("events" in step for step in steps)
    if args.validate_only:
        print(f"INPUT_SEQUENCE=PASS steps={len(steps)} event_steps={event_steps}")
        return 0
    if args.socket is None or args.transcript is None:
        parser.error("--socket and --transcript are required unless --validate-only is used")
    if (args.display_width is None) != (args.display_height is None):
        parser.error("--display-width and --display-height must be provided together")
    display_size = None
    if args.display_width is not None:
        if args.display_width <= 1 or args.display_height <= 1:
            parser.error("display geometry must be greater than one pixel on both axes")
        display_size = (args.display_width, args.display_height)

    args.transcript.parent.mkdir(parents=True, exist_ok=True)
    transcript: list[dict[str, Any]] = []
    try:
        inject(args.socket, steps, args.timeout, transcript, display_size)
    finally:
        args.transcript.write_text(
            "".join(json.dumps(row, ensure_ascii=False) + "\n" for row in transcript),
            encoding="utf-8",
        )
    print(f"INPUT_INJECTION=PASS steps={len(steps)} transcript={args.transcript}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
