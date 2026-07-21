#!/usr/bin/env python3
"""Bound one runtime process group to a wall-clock deadline."""

from __future__ import annotations

import argparse
import os
from pathlib import Path
import signal
import time


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--process-group", type=int, required=True)
    parser.add_argument("--timeout", type=int, required=True)
    parser.add_argument("--marker", type=Path, required=True)
    args = parser.parse_args()
    if args.process_group <= 0 or args.timeout <= 0:
        parser.error("process group and timeout must be positive")
    time.sleep(args.timeout)
    try:
        os.killpg(args.process_group, 0)
    except ProcessLookupError:
        return 0
    args.marker.touch(exist_ok=False)
    os.killpg(args.process_group, signal.SIGTERM)
    time.sleep(5)
    try:
        os.killpg(args.process_group, signal.SIGKILL)
    except ProcessLookupError:
        pass
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
