#!/usr/bin/env python3
"""Record host software-render frame timings without imposing a flaky threshold."""

from __future__ import annotations

import argparse
import json
import os
import re
import statistics
import subprocess
from pathlib import Path


TIMING = re.compile(r"render_elapsed_us=(\d+)")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--iterations", type=int, default=7)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if not 1 <= args.iterations <= 100:
        parser.error("--iterations must be between 1 and 100")

    repo_root = Path(__file__).resolve().parents[2]
    output = args.output or repo_root / "test/output/desktop/performance/host-render.json"
    frame = output.parent / "measured-boot.ppm"
    environment = os.environ.copy()
    for name in ("DISPLAY", "WAYLAND_DISPLAY", "DBUS_SESSION_BUS_ADDRESS"):
        environment.pop(name, None)

    build = subprocess.run(
        [str(repo_root / "scripts/desktop/build.sh"), "scene", "boot"],
        cwd=repo_root,
        env=environment,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    if build.returncode != 0:
        print(build.stdout, end="")
        return build.returncode
    target_dir = repo_root / "build/desktop/target/host-performance"
    release_build = subprocess.run(
        [
            "cargo",
            "build",
            "--offline",
            "--locked",
            "--manifest-path",
            str(repo_root / "user/desktop/Cargo.toml"),
            "--release",
            "--features",
            "host-tools",
            "--bin",
            "render-scene",
            "--target-dir",
            str(target_dir),
        ],
        cwd=repo_root,
        env={
            **environment,
            "CARGO_HOME": str(repo_root / "build/desktop/cargo-home"),
            "CARGO_NET_OFFLINE": "true",
        },
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    if release_build.returncode != 0:
        print(release_build.stdout, end="")
        return release_build.returncode
    binary = target_dir / "release/render-scene"
    samples: list[int] = []
    output.parent.mkdir(parents=True, exist_ok=True)
    for _ in range(args.iterations):
        result = subprocess.run(
            [str(binary), str(frame), "boot", "1024", "768"],
            cwd=repo_root,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            check=False,
        )
        if result.returncode != 0:
            print(result.stdout, end="")
            return result.returncode
        match = TIMING.search(result.stdout)
        if match is None:
            print("missing render_elapsed_us marker")
            return 1
        samples.append(int(match.group(1)))
    record = {
        "schema": 1,
        "renderer": "host-tools MemoryDisplay software compositor",
        "profile": "release",
        "scene": "boot",
        "resolution": {"width": 1024, "height": 768},
        "scope": "desktop construction through final damage composition; excludes PPM write",
        "unit": "microseconds",
        "samples": samples,
        "min": min(samples),
        "median": statistics.median(samples),
        "max": max(samples),
        "threshold": None,
    }
    output.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"PERFORMANCE_RECORD=CREATED samples={len(samples)} output={output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
