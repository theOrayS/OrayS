#!/usr/bin/env python3
"""Compare generated PPM scenes with the versioned SHA-256 manifest."""

from __future__ import annotations

import argparse
import hashlib
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_hashes(path: Path) -> dict[str, str]:
    expected: dict[str, str] = {}
    for number, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) != 2 or len(parts[0]) != 64:
            raise ValueError(f"invalid manifest line {number}")
        digest, name = parts
        if any(character not in "0123456789abcdef" for character in digest):
            raise ValueError(f"invalid digest on manifest line {number}")
        if name in expected:
            raise ValueError(f"duplicate manifest entry: {name}")
        path_name = Path(name)
        if path_name.is_absolute() or ".." in path_name.parts or len(path_name.parts) != 1:
            raise ValueError(f"unsafe manifest filename: {name}")
        expected[name] = digest
    if not expected:
        raise ValueError("golden manifest is empty")
    return expected


def valid_ppm(path: Path) -> bool:
    with path.open("rb") as stream:
        magic = stream.readline().strip()
        dimensions = stream.readline().strip().split()
        maximum = stream.readline().strip()
    if magic != b"P6" or len(dimensions) != 2 or maximum != b"255":
        return False
    try:
        width, height = (int(value) for value in dimensions)
    except ValueError:
        return False
    if width <= 0 or height <= 0:
        return False
    header = f"P6\n{width} {height}\n255\n".encode()
    return path.stat().st_size == len(header) + width * height * 3


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--actual-dir", type=Path, required=True)
    args = parser.parse_args()

    failures: list[str] = []
    try:
        expected = load_hashes(args.manifest)
    except (OSError, ValueError) as error:
        print(f"FAIL {error}")
        print("GOLDEN_COMPARE=FAIL")
        return 1
    for name, digest in sorted(expected.items()):
        path = args.actual_dir / name
        if not path.is_file():
            failures.append(f"missing golden output: {name}")
            continue
        if not valid_ppm(path):
            failures.append(f"invalid P6 PPM: {name}")
            continue
        actual = sha256(path)
        if actual != digest:
            failures.append(f"SHA-256 mismatch: {name} expected={digest} actual={actual}")
        else:
            print(f"MATCH {name} {actual}")
    if failures:
        for failure in failures:
            print(f"FAIL {failure}")
        print("GOLDEN_COMPARE=FAIL")
        return 1
    print(f"GOLDEN_COMPARE=PASS files={len(expected)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
