#!/usr/bin/env python3
"""Static regression guard for the synthetic Linux /dev/full implementation."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def section(text: str, start_marker: str, end_marker: str) -> str:
    start = text.find(start_marker)
    if start < 0:
        return ""
    end = text.find(end_marker, start + len(start_marker))
    return text[start:] if end < 0 else text[start:end]


def require(findings: list[str], condition: bool, detail: str) -> None:
    if not condition:
        findings.append(detail)


def scan(root: Path) -> list[str]:
    fd_table = read(root / "user/shell/src/uspace/fd_table.rs")
    metadata = read(root / "user/shell/src/uspace/metadata.rs")
    findings: list[str] = []

    require(
        findings,
        re.search(r"DEV_FULL_RDEV\s*:\s*u64\s*=\s*263\s*;", metadata) is not None,
        "metadata.rs: /dev/full must use Linux makedev(1, 7)",
    )
    require(
        findings,
        '"/dev/full" => DEV_FULL_RDEV' in metadata,
        "metadata.rs: synthetic character stat must map /dev/full to DEV_FULL_RDEV",
    )
    require(
        findings,
        'synthetic_char_stat_for_path("/dev/full", ST_MODE_CHR | 0o666)' in metadata,
        "metadata.rs: /dev/full stat mode must be S_IFCHR | 0666",
    )

    require(
        findings,
        "DevFull(u32)" in fd_table,
        "fd_table.rs: missing status-flag-carrying DevFull entry",
    )

    read_impl = section(fd_table, "pub(super) fn read(", "pub(super) fn write(")
    full_read = section(
        read_impl,
        "FdEntry::DevFull(status_flags) => {",
        "FdEntry::DevRandom(status_flags)",
    )
    require(
        findings,
        all(
            token in full_read
            for token in (
                "*status_flags & general::O_ACCMODE",
                "general::O_RDONLY | general::O_RDWR",
                "Err(LinuxError::EBADF)",
                "dst.fill(0)",
                "Ok(dst.len())",
            )
        ),
        "fd_table.rs: /dev/full read must enforce access mode and fill the request with zeros",
    )

    write_impl = section(fd_table, "pub(super) fn write(", "pub(super) fn close(")
    full_write = section(
        write_impl,
        "FdEntry::DevFull(status_flags) => {",
        "FdEntry::DevRandom(status_flags)",
    )
    require(
        findings,
        all(
            token in full_write
            for token in (
                "!file_is_writable(*status_flags)",
                "Err(LinuxError::EBADF)",
                "Err(LinuxError::ENOSPC)",
            )
        )
        and "src.is_empty" not in full_write
        and "Ok(src.len())" not in full_write,
        "fd_table.rs: every writable /dev/full write, including length zero, must fail with ENOSPC",
    )
    sys_write_impl = section(fd_table, "pub(super) fn sys_write(", "pub(super) fn sys_pwrite64(")
    require(
        findings,
        "matches!(table.entry(fd as i32), Ok(FdEntry::DevFull(_)))" in sys_write_impl
        and "table.write(process, fd as i32, &[], Some(file_size_limit))" in sys_write_impl,
        "fd_table.rs: /dev/full write must not access a user buffer before returning ENOSPC",
    )

    lseek_impl = section(fd_table, "pub(super) fn lseek(", "pub(super) fn dup(")
    require(
        findings,
        "matches!(self.entry(fd), Ok(FdEntry::DevFull(_)))" in lseek_impl
        and all(
            token in lseek_impl
            for token in (
                "general::SEEK_SET",
                "general::SEEK_CUR",
                "general::SEEK_END",
                "SEEK_DATA_WHENCE",
                "SEEK_HOLE_WHENCE",
                "=> Ok(0)",
                "_ => Err(LinuxError::EINVAL)",
            )
        ),
        "fd_table.rs: /dev/full lseek must return zero for all five Linux whence values",
    )

    poll_impl = section(fd_table, "pub(super) fn poll_entry(", "pub(super) fn epoll_ctl(")
    fast_poll_impl = section(fd_table, "fn poll_entry_fast(", "pub(super) fn read(")
    require(
        findings,
        poll_impl.count("FdEntry::DevFull(_)") >= 2
        and fast_poll_impl.count("FdEntry::DevFull(_)") >= 2,
        "fd_table.rs: /dev/full must be immediately readable and writable in poll and epoll",
    )

    stat_impl = section(fd_table, "pub(super) fn stat(", "pub(super) fn stat_with_recorded_path(")
    require(
        findings,
        "FdEntry::DevFull(_) => Ok(dev_full_stat())" in stat_impl,
        "fd_table.rs: fstat on /dev/full must use dev_full_stat",
    )

    require(
        findings,
        re.search(
            r"SYNTHETIC_CHAR_DEVICE_NAMES[^;]*\[\s*\"cpu_dma_latency\"\s*,\s*\"full\"\s*\]",
            fd_table,
            re.DOTALL,
        )
        is not None,
        "fd_table.rs: /dev getdents64 capability list must include full",
    )

    full_open = section(fd_table, 'if path == "/dev/full" {', 'if path == "/dev/urandom"')
    require(
        findings,
        all(
            token in full_open
            for token in (
                "if path_only",
                'PathEntry::synthetic_char_with_mode("/dev/full", 0o666)',
                "FdEntry::DevFull(fcntl_status_flags(flags))",
                "LinuxError::EEXIST",
                "LinuxError::ENOTDIR",
            )
        ),
        "fd_table.rs: /dev/full open must cover O_PATH and ordinary existing-node errors",
    )
    require(
        findings,
        "(ST_MODE_CHR, Some(DEV_FULL_RDEV))" in fd_table
        and "return Ok(FdEntry::DevFull(fcntl_status_flags(flags)))" in fd_table,
        "fd_table.rs: mknod character device 1:7 must map to DevFull",
    )

    duplicate_impl = section(fd_table, "pub(super) fn duplicate_for_fork(", "fn open_fd_entry(")
    fcntl_impl = section(fd_table, "pub(super) fn fcntl(", "pub(super) fn flock(")
    require(
        findings,
        "Self::DevFull(status_flags) => Ok(Self::DevFull(*status_flags))" in duplicate_impl,
        "fd_table.rs: dup/fork must preserve the synthetic /dev/full entry",
    )
    require(
        findings,
        fcntl_impl.count("FdEntry::DevFull(status_flags)") >= 2,
        "fd_table.rs: F_GETFL and F_SETFL must handle /dev/full status flags",
    )

    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    findings = scan(args.root.resolve())
    if findings:
        print("/dev/full static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("/dev/full static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
