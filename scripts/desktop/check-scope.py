#!/usr/bin/env python3
"""Fail closed when the desktop PR escapes its authorized path and bridge budget."""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path

MAX_EXISTING_BRIDGE_FILES = 8
MAX_EXISTING_BRIDGE_CHURN = 250

FORBIDDEN = (
    "Cargo.toml",
    "Cargo.lock",
    "Makefile",
    "rust-toolchain.toml",
    "run-eval.sh",
    "user/shell/**",
    "api/arceos_posix_api/**",
    "api/orays_linux/**",
    "api/orays_linux_abi/**",
    "test/evaluation/**",
    "test/evidence/**",
    "vendor/**",
    "cargo-home/**",
    ".github/workflows/build.yml",
    ".github/workflows/test.yml",
    ".github/workflows/docs.yml",
)

def run(*args: str, check: bool = True, cwd: Path | None = None) -> str:
    p = subprocess.run(
        args,
        cwd=cwd,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if check and p.returncode != 0:
        raise RuntimeError(
            f"command failed ({p.returncode}): {' '.join(args)}\n{p.stderr}"
        )
    return p.stdout


def run_bytes(*args: str, check: bool = True, cwd: Path | None = None) -> bytes:
    p = subprocess.run(
        args,
        cwd=cwd,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if check and p.returncode != 0:
        raise RuntimeError(
            f"command failed ({p.returncode}): {args!r}\n{os.fsdecode(p.stderr)}"
        )
    return p.stdout


def repo_root() -> Path:
    script_root = Path(__file__).resolve().parents[2]
    return Path(
        run("git", "rev-parse", "--show-toplevel", cwd=script_root).strip()
    ).resolve()


def load_patterns(path: Path) -> list[str]:
    patterns: list[str] = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if "*" in line and not line.endswith("/**"):
            raise ValueError(f"unsupported pattern (only exact or trailing /**): {line}")
        if line[:-3].find("*") >= 0 if line.endswith("/**") else False:
            raise ValueError(f"unsupported wildcard before trailing /**: {line}")
        patterns.append(line)
    return patterns


def matches(path: str, pattern: str) -> bool:
    if pattern.endswith("/**"):
        prefix = pattern[:-3].rstrip("/")
        return path == prefix or path.startswith(prefix + "/")
    return path == pattern


def any_match(path: str, patterns: tuple[str, ...] | list[str]) -> bool:
    return any(matches(path, p) for p in patterns)


def default_base(root: Path) -> str:
    candidate = "origin/develop/post-integration-next^{commit}"
    p = subprocess.run(
        ["git", "rev-parse", "--verify", candidate],
        cwd=root,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    )
    if p.returncode == 0:
        return p.stdout.rstrip("\n")
    raise RuntimeError("unable to resolve desktop base")


def changed_paths(root: Path, base: str) -> set[str]:
    paths: set[str] = set()
    commands = (
        ("git", "diff", "--name-only", "-z", f"{base}...HEAD"),
        ("git", "diff", "--name-only", "-z"),
        ("git", "diff", "--name-only", "-z", "--cached"),
        (
            "git",
            "ls-files",
            "-z",
            "--others",
            "--exclude-standard",
        ),
    )
    for cmd in commands:
        output = run_bytes(*cmd, cwd=root)
        if output and not output.endswith(b"\0"):
            raise RuntimeError(f"Git path output is not NUL terminated: {cmd!r}")
        for raw_path in output.split(b"\0"):
            if raw_path:
                paths.add(os.fsdecode(raw_path))
    return paths


def existed_at_base(root: Path, base: str, path: str) -> bool:
    p = subprocess.run(
        ["git", "cat-file", "-e", f"{base}:{path}"],
        cwd=root,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return p.returncode == 0


def bridge_churn(root: Path, base: str, paths: list[str]) -> int:
    if not paths:
        return 0
    out = run_bytes("git", "diff", "--numstat", "-z", base, "--", *paths, cwd=root)
    if out and not out.endswith(b"\0"):
        raise RuntimeError("Git numstat output is not NUL terminated")
    total = 0
    records = out.split(b"\0")
    index = 0
    while index < len(records) - 1:
        record = records[index]
        index += 1
        parts = record.split(b"\t", 2)
        if len(parts) < 3:
            raise RuntimeError("Git numstat output is malformed")
        a, d = parts[0], parts[1]
        if a.isdigit():
            total += int(a)
        if d.isdigit():
            total += int(d)
        if parts[2] == b"":
            if index + 1 >= len(records):
                raise RuntimeError("Git rename numstat output is truncated")
            index += 2
    return total


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base")
    args = parser.parse_args()

    root = repo_root()
    allowed = load_patterns(root / "test/desktop/allowed_paths.txt")
    bridges = load_patterns(root / "test/desktop/bridge_paths.txt")
    base = args.base or default_base(root)

    subprocess.run(
        ["git", "rev-parse", "--verify", base],
        cwd=root,
        check=True,
        stdout=subprocess.DEVNULL,
    )

    changed = sorted(changed_paths(root, base))
    failures: list[str] = []
    bridge_changed: list[str] = []

    for path in changed:
        if any_match(path, FORBIDDEN):
            failures.append(f"forbidden path: {path!r}")
            continue
        if any_match(path, allowed):
            continue
        if any_match(path, bridges):
            bridge_changed.append(path)
            continue
        failures.append(f"outside desktop allowlist: {path!r}")

    existing_bridge = [
        p for p in bridge_changed if existed_at_base(root, base, p)
    ]
    churn = bridge_churn(root, base, existing_bridge)

    if len(existing_bridge) > MAX_EXISTING_BRIDGE_FILES:
        failures.append(
            f"bridge existing-file count {len(existing_bridge)} exceeds "
            f"{MAX_EXISTING_BRIDGE_FILES}: {existing_bridge}"
        )
    if churn > MAX_EXISTING_BRIDGE_CHURN:
        failures.append(
            f"bridge churn {churn} exceeds {MAX_EXISTING_BRIDGE_CHURN}"
        )

    print(f"base={base}")
    print(f"changed_paths={len(changed)}")
    print(f"bridge_paths={len(bridge_changed)}")
    print(f"existing_bridge_files={len(existing_bridge)}")
    print(f"existing_bridge_churn={churn}")

    if failures:
        for item in failures:
            print(f"FAIL {item}", file=sys.stderr)
        print("DESKTOP_SCOPE=FAIL")
        return 1

    print("DESKTOP_SCOPE=PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
