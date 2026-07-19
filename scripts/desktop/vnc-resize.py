#!/usr/bin/env python3
"""Request a live guest display size through the RFB SetDesktopSize extension."""

from __future__ import annotations

import argparse
import json
import socket
import struct
import time
from pathlib import Path


class VncResizeError(RuntimeError):
    pass


def receive_exact(sock: socket.socket, length: int) -> bytes:
    value = bytearray()
    while len(value) < length:
        chunk = sock.recv(length - len(value))
        if not chunk:
            raise VncResizeError("VNC connection closed during handshake")
        value.extend(chunk)
    return bytes(value)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--display", type=int, required=True)
    parser.add_argument("--width", type=int, required=True)
    parser.add_argument("--height", type=int, required=True)
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--timeout", type=float, default=15.0)
    args = parser.parse_args()
    if args.host not in {"127.0.0.1", "localhost", "::1"}:
        parser.error("resize client is restricted to the local host")
    if args.display < 0 or args.width <= 1 or args.height <= 1:
        parser.error("display must be non-negative and geometry must exceed one pixel")

    port = 5900 + args.display
    deadline = time.monotonic() + args.timeout
    last_error: OSError | None = None
    while True:
        try:
            sock = socket.create_connection((args.host, port), timeout=args.timeout)
            break
        except OSError as error:
            last_error = error
            if time.monotonic() >= deadline:
                raise VncResizeError(f"VNC connection failed: {last_error}") from error
            time.sleep(0.05)

    with sock:
        server_version = receive_exact(sock, 12)
        if not server_version.startswith(b"RFB 003."):
            raise VncResizeError(f"unexpected RFB version: {server_version!r}")
        sock.sendall(b"RFB 003.008\n")
        security_count = receive_exact(sock, 1)[0]
        if security_count == 0:
            reason_length = struct.unpack(">I", receive_exact(sock, 4))[0]
            reason = receive_exact(sock, reason_length).decode(errors="replace")
            raise VncResizeError(f"VNC server rejected security negotiation: {reason}")
        security_types = receive_exact(sock, security_count)
        if 1 not in security_types:
            raise VncResizeError("VNC server does not offer localhost no-auth security")
        sock.sendall(b"\x01")
        security_result = struct.unpack(">I", receive_exact(sock, 4))[0]
        if security_result != 0:
            raise VncResizeError(f"VNC no-auth negotiation failed: {security_result}")

        sock.sendall(b"\x01")
        initial_width, initial_height = struct.unpack(">HH", receive_exact(sock, 4))
        receive_exact(sock, 16)
        name_length = struct.unpack(">I", receive_exact(sock, 4))[0]
        server_name = receive_exact(sock, name_length).decode(errors="replace")

        # Advertise ExtendedDesktopSize (-308), then use the standard
        # SetDesktopSize client message (251) with one screen descriptor.
        sock.sendall(struct.pack(">BBHi", 2, 0, 1, -308))
        request = struct.pack(
            ">BBHHBBIHHHHI",
            251,
            0,
            args.width,
            args.height,
            1,
            0,
            0,
            0,
            0,
            args.width,
            args.height,
            0,
        )
        sock.sendall(request)

    evidence = {
        "schema": 1,
        "transport": "RFB SetDesktopSize",
        "endpoint": f"{args.host}:{port}",
        "server_version": server_version.decode(errors="replace").strip(),
        "server_name": server_name,
        "security_type": "None",
        "initial_geometry": [initial_width, initial_height],
        "requested_geometry": [args.width, args.height],
        "screen_count": 1,
        "extended_desktop_size_encoding": -308,
    }
    args.evidence.parent.mkdir(parents=True, exist_ok=True)
    args.evidence.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
    print(
        f"VNC_RESIZE=REQUESTED initial={initial_width}x{initial_height} "
        f"requested={args.width}x{args.height}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
