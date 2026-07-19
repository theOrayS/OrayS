#!/usr/bin/env python3
"""Capture a QEMU display to a PPM file through a Unix QMP socket."""

from __future__ import annotations

import argparse
import hashlib
import json
import socket
import time
from pathlib import Path
from typing import Any


class QmpError(RuntimeError):
    pass


def record_capture_precondition(
    serial_log: Path,
    action_marker: str,
    stable_marker: str,
    output: Path,
) -> None:
    serial_prefix = serial_log.read_bytes()
    lines = serial_prefix.decode("utf-8", errors="replace").replace("\x00", "").splitlines()
    action_lines = [index for index, line in enumerate(lines, 1) if line == action_marker]
    stable_lines = [index for index, line in enumerate(lines, 1) if line == stable_marker]
    if len(action_lines) != 1:
        raise QmpError(
            f"capture precondition requires exactly one action marker, got {len(action_lines)}"
        )
    if len(stable_lines) != 1:
        raise QmpError(
            f"capture precondition requires exactly one stable marker, got {len(stable_lines)}"
        )
    if stable_lines[0] <= action_lines[0]:
        raise QmpError("capture precondition stable marker does not follow action marker")

    value = {
        "schema": 1,
        "action_marker": action_marker,
        "action_line": action_lines[0],
        "stable_marker": stable_marker,
        "stable_line": stable_lines[0],
        "serial_prefix_bytes": len(serial_prefix),
        "serial_prefix_sha256": hashlib.sha256(serial_prefix).hexdigest(),
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def recv_message(stream, transcript: list[dict[str, Any]]) -> dict[str, Any]:
    while True:
        line = stream.readline()
        if not line:
            raise QmpError("QMP socket closed")
        msg = json.loads(line.decode("utf-8"))
        transcript.append({"direction": "receive", "message": msg})
        if "event" in msg:
            continue
        return msg


def send_message(
    stream, obj: dict[str, Any], transcript: list[dict[str, Any]]
) -> dict[str, Any]:
    transcript.append({"direction": "send", "message": obj})
    stream.write((json.dumps(obj) + "\r\n").encode("utf-8"))
    stream.flush()
    response = recv_message(stream, transcript)
    if "error" in response:
        raise QmpError(json.dumps(response["error"], ensure_ascii=False))
    return response


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--socket", required=True, help="QMP Unix socket path")
    parser.add_argument("--output", required=True, help="Output PPM path")
    parser.add_argument("--timeout", type=float, default=15.0)
    parser.add_argument("--settle", type=float, default=0.25)
    parser.add_argument("--transcript", type=Path)
    parser.add_argument("--quit-after", action="store_true")
    parser.add_argument("--serial-log", type=Path)
    parser.add_argument("--action-marker")
    parser.add_argument("--stable-marker")
    parser.add_argument("--precondition-output", type=Path)
    args = parser.parse_args()

    precondition_values = (
        args.serial_log,
        args.action_marker,
        args.stable_marker,
        args.precondition_output,
    )
    if any(value is not None for value in precondition_values) and not all(
        value is not None for value in precondition_values
    ):
        parser.error(
            "--serial-log, --action-marker, --stable-marker, and "
            "--precondition-output must be provided together"
        )

    socket_path = Path(args.socket)
    output = Path(args.output).resolve()
    output.parent.mkdir(parents=True, exist_ok=True)

    deadline = time.monotonic() + args.timeout
    while not socket_path.exists():
        if time.monotonic() >= deadline:
            raise QmpError(f"QMP socket did not appear: {socket_path}")
        time.sleep(0.1)

    transcript: list[dict[str, Any]] = []
    try:
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as sock:
            sock.settimeout(args.timeout)
            sock.connect(str(socket_path))
            stream = sock.makefile("rwb", buffering=0)

            greeting = recv_message(stream, transcript)
            if "QMP" not in greeting:
                raise QmpError(f"invalid QMP greeting: {greeting}")

            send_message(stream, {"execute": "qmp_capabilities"}, transcript)
            time.sleep(max(args.settle, 0.0))
            if args.serial_log is not None:
                record_capture_precondition(
                    args.serial_log,
                    args.action_marker,
                    args.stable_marker,
                    args.precondition_output,
                )
            send_message(
                stream,
                {
                    "execute": "screendump",
                    "arguments": {"filename": str(output)},
                },
                transcript,
            )
            if args.quit_after:
                send_message(stream, {"execute": "quit"}, transcript)
    finally:
        if args.transcript is not None:
            args.transcript.parent.mkdir(parents=True, exist_ok=True)
            args.transcript.write_text(
                "".join(json.dumps(row, ensure_ascii=False) + "\n" for row in transcript),
                encoding="utf-8",
            )

    deadline = time.monotonic() + args.timeout
    while time.monotonic() < deadline:
        if output.exists() and output.stat().st_size > 16:
            print(output)
            return 0
        time.sleep(0.1)

    raise QmpError(f"screendump did not create a non-empty file: {output}")


if __name__ == "__main__":
    raise SystemExit(main())
