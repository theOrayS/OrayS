#!/usr/bin/env python3
"""Create a short QEMU-safe QMP runtime directory under canonical /tmp."""

from __future__ import annotations

import os
from pathlib import Path
import tempfile


QEMU_UNIX_SOCKET_PATH_MAX = 107


def main() -> int:
    runtime_dir = Path(tempfile.mkdtemp(prefix="orays-qmp.", dir="/tmp"))
    socket_path = runtime_dir / "qmp.sock"
    encoded = os.fsencode(socket_path)
    if (
        len(encoded) > QEMU_UNIX_SOCKET_PATH_MAX
        or b"," in encoded
        or b":" in encoded
        or b"\n" in encoded
        or b"\r" in encoded
    ):
        runtime_dir.rmdir()
        raise ValueError("generated QMP socket path is not AF_UNIX/QEMU argument safe")
    print(runtime_dir)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
