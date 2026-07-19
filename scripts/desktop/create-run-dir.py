#!/usr/bin/env python3
"""Create one QEMU evidence directory without following caller-controlled symlinks."""

from __future__ import annotations

import argparse
import os
from pathlib import Path
import re
import secrets
import sys


class OutputPathError(Exception):
    pass


class ExistingOutputError(OutputPathError):
    pass


def _open_directory(name: str | Path, *, dir_fd: int | None = None) -> int:
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
    return os.open(name, flags, dir_fd=dir_fd)


def _output_root(repo_root: str) -> tuple[Path, int]:
    try:
        repository = Path(repo_root).resolve(strict=True)
    except OSError as error:
        raise OutputPathError(f"cannot resolve repository root: {error}") from error
    root = repository / "test" / "output" / "desktop"

    opened: list[int] = []
    try:
        current_fd = _open_directory(repository)
        opened.append(current_fd)
        for component in ("test", "output"):
            current_fd = _open_directory(component, dir_fd=current_fd)
            opened.append(current_fd)

        try:
            root_fd = _open_directory("desktop", dir_fd=current_fd)
        except FileNotFoundError:
            try:
                os.mkdir("desktop", mode=0o755, dir_fd=current_fd)
            except FileExistsError:
                # A concurrent creator won the race. The O_NOFOLLOW open below
                # still verifies that the resulting entry is a real directory.
                pass
            root_fd = _open_directory("desktop", dir_fd=current_fd)
    except OSError as error:
        raise OutputPathError(f"desktop output root is not a real directory: {root}: {error}") from error
    finally:
        for descriptor in reversed(opened):
            os.close(descriptor)
    return root, root_fd


def _relative_parts(root: Path, candidate: str) -> tuple[str, ...]:
    normalized = Path(os.path.abspath(candidate))
    try:
        relative = normalized.relative_to(root)
    except ValueError as error:
        raise OutputPathError(f"output must be strictly below {root}: {candidate}") from error
    parts = relative.parts
    if not parts or any(part in {"", ".", ".."} for part in parts):
        raise OutputPathError(f"output must name a child directory below {root}: {candidate}")
    return parts


def _create_below(root_fd: int, parts: tuple[str, ...]) -> None:
    current_fd = os.dup(root_fd)
    try:
        for index, part in enumerate(parts):
            final = index == len(parts) - 1
            created = False
            try:
                next_fd = _open_directory(part, dir_fd=current_fd)
            except FileNotFoundError:
                try:
                    os.mkdir(part, mode=0o755, dir_fd=current_fd)
                    created = True
                    next_fd = _open_directory(part, dir_fd=current_fd)
                except OSError as error:
                    raise OutputPathError(f"cannot create output component {part!r}: {error}") from error
            except OSError as error:
                raise OutputPathError(
                    f"output component is not a real directory: {part!r}: {error}"
                ) from error

            os.close(current_fd)
            current_fd = next_fd
            if final and not created:
                raise ExistingOutputError("refusing to overwrite existing output directory")
    finally:
        os.close(current_fd)


def create_candidate(repo_root: str, candidate: str) -> Path:
    root, root_fd = _output_root(repo_root)
    try:
        parts = _relative_parts(root, candidate)
        _create_below(root_fd, parts)
        return root.joinpath(*parts)
    finally:
        os.close(root_fd)


def create_random(repo_root: str, prefix: str) -> Path:
    if re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]*", prefix) is None:
        raise OutputPathError(f"unsafe output prefix: {prefix!r}")
    root, root_fd = _output_root(repo_root)
    try:
        for _ in range(128):
            name = f"{prefix}{secrets.token_hex(3)}"
            try:
                _create_below(root_fd, (name,))
            except ExistingOutputError:
                continue
            return root / name
    finally:
        os.close(root_fd)
    raise OutputPathError("could not allocate a unique output directory")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", required=True)
    selection = parser.add_mutually_exclusive_group(required=True)
    selection.add_argument("--candidate")
    selection.add_argument("--prefix")
    args = parser.parse_args()
    try:
        if args.candidate is not None:
            output = create_candidate(args.repo_root, args.candidate)
        else:
            output = create_random(args.repo_root, args.prefix)
    except OutputPathError as error:
        print(f"invalid desktop output path: {error}", file=sys.stderr)
        return 2
    print(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
