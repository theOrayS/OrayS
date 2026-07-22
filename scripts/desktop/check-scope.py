#!/usr/bin/env python3
"""Fail closed when the desktop PR escapes its authorized path and bridge budget."""

from __future__ import annotations

import argparse
import os
import re
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
    "api/arceos_posix_api/**",
    "api/orays_linux/**",
    "api/orays_linux_abi/**",
    "test/evaluation/**",
    "test/evidence/**",
    "vendor/**",
    "cargo-home/**",
)

BASE_CANDIDATE_TOKEN = re.compile(r"[0-9A-Za-z][0-9A-Za-z._/-]*")
COMMIT_SHA = re.compile(r"[0-9a-f]{40}")


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


def run_bytes(*args: str, cwd: Path | None = None) -> bytes:
    p = subprocess.run(
        args,
        cwd=cwd,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if p.returncode != 0:
        raise RuntimeError(
            f"command failed ({p.returncode}): {' '.join(args)}\n"
            + p.stderr.decode("utf-8", errors="replace")
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


def classify(
    changed: list[str], allowed: list[str], bridges: list[str]
) -> tuple[list[str], list[str]]:
    """Split changed paths into failures and bridge-budget consumers.

    Paths are compared byte-for-byte as Git reports them. No stripping,
    unquoting, or other mutation ever happens here, so a name with leading
    or trailing whitespace, control characters, or quoting-sensitive bytes
    can only match a rule when the rule literally contains those bytes.
    """
    failures: list[str] = []
    bridge_changed: list[str] = []
    for path in changed:
        if any_match(path, FORBIDDEN):
            failures.append(f"forbidden path: {path}")
            continue
        if any_match(path, allowed):
            continue
        if any_match(path, bridges):
            bridge_changed.append(path)
            continue
        failures.append(f"outside desktop allowlist: {path}")
    return failures, bridge_changed


def resolve_base(root: Path, base_arg: str | None) -> tuple[str, str]:
    """Resolve the authorized baseline to a full commit id, or fail hard.

    There is deliberately no fallback: without an explicit --base or a
    non-empty .codex/state/desktop-base-sha the check cannot run, and an
    unresolvable candidate is an error, never a silent pass over an empty
    diff.
    """
    candidate = base_arg
    source = "cli"
    if candidate is None:
        state = root / ".codex/state/desktop-base-sha"
        if not state.is_file():
            raise RuntimeError(
                "no authorized desktop base: pass --base <commit> or create "
                ".codex/state/desktop-base-sha"
            )
        candidate = state.read_text(encoding="utf-8").strip()
        source = "state-file"
        if not candidate:
            raise RuntimeError(f"authorized desktop base state file is empty: {state}")
    if BASE_CANDIDATE_TOKEN.fullmatch(candidate) is None:
        raise RuntimeError(f"authorized desktop base is not a plain ref: {candidate!r}")
    p = subprocess.run(
        ["git", "rev-parse", "--verify", f"{candidate}^{{commit}}"],
        cwd=root,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    resolved = p.stdout.strip()
    if p.returncode != 0 or COMMIT_SHA.fullmatch(resolved) is None:
        raise RuntimeError(
            f"authorized desktop base does not resolve to a commit: {candidate!r}"
        )
    return resolved, source


def changed_paths(root: Path, base: str) -> set[str]:
    """Collect changed paths from NUL-delimited Git output, byte-exact.

    -z disables all C-style quoting, and records are split only on NUL, so
    names containing newlines, tabs, quotes, or surrounding whitespace keep
    their exact bytes (decoded with surrogateescape for non-UTF-8 names).
    """
    paths: set[str] = set()
    commands = (
        ("git", "diff", "--name-only", "-z", "--no-renames", f"{base}...HEAD"),
        ("git", "diff", "--name-only", "-z", "--no-renames"),
        ("git", "diff", "--name-only", "-z", "--no-renames", "--cached"),
        ("git", "ls-files", "--others", "--exclude-standard", "-z"),
    )
    for cmd in commands:
        out = run_bytes(*cmd, cwd=root)
        for raw in out.split(b"\0"):
            if raw:
                paths.add(os.fsdecode(raw))
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
    out = run_bytes(
        "git", "diff", "--numstat", "-z", "--no-renames", base, "--", *paths, cwd=root
    )
    total = 0
    for record in out.split(b"\0"):
        if not record:
            continue
        parts = record.split(b"\t", 2)
        if len(parts) != 3:
            raise RuntimeError(
                f"unparseable git numstat record: {record!r}"
            )
        added, deleted = parts[0], parts[1]
        if added.isdigit():
            total += int(added)
        if deleted.isdigit():
            total += int(deleted)
    return total


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base")
    args = parser.parse_args()

    root = repo_root()
    allowed = load_patterns(root / "test/desktop/allowed_paths.txt")
    bridges = load_patterns(root / "test/desktop/bridge_paths.txt")
    try:
        base, base_source = resolve_base(root, args.base)
    except RuntimeError as error:
        print(f"FAIL {error}", file=sys.stderr)
        print("DESKTOP_SCOPE=FAIL")
        return 1

    changed = sorted(changed_paths(root, base))
    failures, bridge_changed = classify(changed, allowed, bridges)

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
    print(f"base_source={base_source}")
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
