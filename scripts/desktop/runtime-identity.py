#!/usr/bin/env python3
"""Resolve, verify, and execute one approved runtime binary without pathname TOCTOU."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import sys


SHA256 = re.compile(r"[0-9a-f]{64}")


def require_sha256(value: object) -> str:
    if not isinstance(value, str) or SHA256.fullmatch(value) is None:
        raise ValueError("required SHA-256 is not a lowercase 64-digit digest")
    return value


def resolve_executable(candidate: str) -> Path:
    located = shutil.which(candidate) if "/" not in candidate else candidate
    if located is None:
        raise ValueError(f"executable is unavailable: {candidate}")
    path = Path(located).resolve(strict=True)
    if not path.is_absolute() or not path.is_file():
        raise ValueError(f"executable does not resolve to a regular file: {candidate}")
    return path


def digest_descriptor(descriptor: int) -> str:
    value = hashlib.sha256()
    offset = 0
    while True:
        chunk = os.pread(descriptor, 1024 * 1024, offset)
        if not chunk:
            break
        value.update(chunk)
        offset += len(chunk)
    return value.hexdigest()


def open_verified_executable(path: Path, required_sha256: str) -> tuple[int, str]:
    required_sha256 = require_sha256(required_sha256)
    if not path.is_absolute() or path.resolve(strict=True) != path:
        raise ValueError("executable path must already be absolute and canonical")
    flags = os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0)
    descriptor = os.open(path, flags)
    try:
        information = os.fstat(descriptor)
        if not stat.S_ISREG(information.st_mode):
            raise ValueError("verified executable object is not a regular file")
        if information.st_mode & 0o111 == 0:
            raise ValueError("verified executable object has no execute permission")
        observed = digest_descriptor(descriptor)
        if observed != required_sha256:
            raise ValueError(
                f"executable digest mismatch: required {required_sha256}, observed {observed}"
            )
        return descriptor, observed
    except BaseException:
        os.close(descriptor)
        raise


def main() -> int:
    parser = argparse.ArgumentParser()
    subcommands = parser.add_subparsers(dest="command", required=True)

    resolve_parser = subcommands.add_parser("resolve")
    resolve_parser.add_argument("--candidate", required=True)

    verify_parser = subcommands.add_parser("verify")
    verify_parser.add_argument("--canonical-path", type=Path, required=True)
    verify_parser.add_argument("--required-sha256", required=True)

    exec_parser = subcommands.add_parser("exec")
    exec_parser.add_argument("--canonical-path", type=Path, required=True)
    exec_parser.add_argument("--required-sha256", required=True)
    exec_parser.add_argument("--new-session", action="store_true")
    exec_parser.add_argument("arguments", nargs=argparse.REMAINDER)

    policy_parser = subcommands.add_parser("policy-qemu-sha256")
    policy_parser.add_argument("--policy", type=Path, required=True)
    policy_parser.add_argument("--arch", choices=("rv", "la"), required=True)

    argv_parser = subcommands.add_parser("argv-json")
    argv_parser.add_argument("arguments", nargs=argparse.REMAINDER)
    args = parser.parse_args()

    try:
        if args.command == "resolve":
            print(resolve_executable(args.candidate))
            return 0
        if args.command == "policy-qemu-sha256":
            policy = json.loads(args.policy.read_text(encoding="utf-8"))
            digest = policy.get("architectures", {}).get(args.arch, {}).get(
                "qemu_sha256"
            )
            print(require_sha256(digest))
            return 0
        if args.command == "argv-json":
            arguments = args.arguments
            if arguments and arguments[0] == "--":
                arguments = arguments[1:]
            if not arguments:
                raise ValueError("argv-json requires at least one argument")
            print(json.dumps(arguments, separators=(",", ":")))
            return 0

        descriptor, observed = open_verified_executable(
            args.canonical_path, args.required_sha256
        )
        if args.command == "verify":
            os.close(descriptor)
            print(observed)
            return 0

        arguments = args.arguments
        if arguments and arguments[0] == "--":
            arguments = arguments[1:]
        if not arguments:
            os.close(descriptor)
            raise ValueError("verified exec requires at least one executable argument")
        os.set_inheritable(descriptor, True)
        if args.new_session:
            os.setsid()
        os.execve(
            f"/proc/self/fd/{descriptor}",
            [str(args.canonical_path), *arguments],
            os.environ.copy(),
        )
    except (OSError, ValueError) as error:
        print(f"runtime identity error: {error}", file=sys.stderr)
        return 2
    return 70


if __name__ == "__main__":
    raise SystemExit(main())
