#!/usr/bin/env python3
"""Static guard for G009 post-review real LTP semantics blockers."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def function_block(text: str, name: str) -> str:
    marker = f"fn {name}"
    start = text.find(marker)
    if start < 0:
        marker = f"pub unsafe fn {name}"
        start = text.find(marker)
    if start < 0:
        marker = f"pub fn {name}"
        start = text.find(marker)
    if start < 0:
        return ""
    candidates = [
        pos
        for pos in (
            text.find("\nfn ", start + len(marker)),
            text.find("\npub fn ", start + len(marker)),
            text.find("\npub unsafe fn ", start + len(marker)),
            text.find("\npub(super) fn ", start + len(marker)),
            text.find("\n#[", start + len(marker)),
        )
        if pos >= 0
    ]
    end = min(candidates) if candidates else len(text)
    return text[start:end]


def scan_stdio_close(root: Path) -> list[str]:
    api_text = read(root / "api/arceos_posix_api/src/imp/fd_ops.rs")
    fd_text = read(root / "examples/shell/src/uspace/fd_table.rs")
    block = function_block(api_text, "sys_close")
    is_stdio = function_block(fd_text, "is_stdio")
    findings: list[str] = []
    if not block:
        findings.append("api fd_ops.rs: missing sys_close")
    if "close_file_like(fd)" not in block:
        findings.append("sys_close must remove every fd through close_file_like")
    if re.search(r"\(0\.\.=2\).*return\s+0", block, re.S):
        findings.append("sys_close still fake-succeeds for stdin/stdout/stderr")
    stdio_pattern = (
        r"FdEntry::Stdin(?:\(_\))?\s*\|\s*"
        r"FdEntry::Stdout(?:\(_\))?\s*\|\s*"
        r"FdEntry::Stderr(?:\(_\))?"
    )
    if not re.search(stdio_pattern, is_stdio):
        findings.append("shell is_stdio must consult live FdEntry, not numeric fd 0..=2")
    if "matches!(fd, 0..=2)" in is_stdio:
        findings.append("shell is_stdio still treats closed/reused numeric stdio fds as terminals")
    return findings


def scan_block_device(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/uspace/fd_table.rs")
    metadata = read(root / "examples/shell/src/uspace/metadata.rs")
    linux_abi = read(root / "examples/shell/src/uspace/linux_abi.rs")
    findings: list[str] = []
    for token in ("status_flags", "offset: Arc<Mutex<u64>>", "storage: Arc<Mutex<Vec<u8>>>"):
        if token not in text:
            findings.append(f"BlockDeviceEntry must keep real access/offset/storage state; missing {token}")
    if re.search(r"FdEntry::BlockDevice\([^)]*\)\s*=>\s*Ok\(src\.len\(\)\)", text):
        findings.append("block-device write still bit-buckets data as full success")
    if re.search(r"FdEntry::BlockDevice[\s\S]{0,120}dst\.fill\(0\);\s*Ok\(dst\.len\(\)\)", text):
        findings.append("block-device read still returns zero-filled fake data without storage")
    if re.search(r"FdEntry::BlockDevice\([^)]*\)\s*=>\s*Ok\(0\)", text):
        findings.append("block-device lseek still returns constant 0")
    for token in ("SYNTHETIC_BLOCK_DEVICE_SIZE", "ENOSPC", "file_is_readable", "file_is_writable"):
        if token not in text:
            findings.append(f"block-device path must enforce capacity and access mode; missing {token}")
    if "512 * 1024 * 1024" in text:
        findings.append("BLKGETSIZE64 must not report a different hard-coded block size")
    if "pub(super) const SYNTHETIC_BLOCK_DEVICE_SIZE" not in linux_abi:
        findings.append("synthetic block capacity must live in the shared linux_abi constant")
    if "SYNTHETIC_BLOCK_DEVICE_SIZE" not in metadata:
        findings.append("synthetic block stat must use the shared block-device capacity")
    return findings


def scan_getaddrinfo(root: Path) -> list[str]:
    api = read(root / "api/arceos_posix_api/src/imp/net.rs")
    libc = read(root / "ulib/axlibc/src/net.rs")
    block = function_block(api, "sys_getaddrinfo")
    findings: list[str] = []
    if "_hints" in block:
        findings.append("sys_getaddrinfo still names hints as ignored")
    for token in ("ResolvedAddrInfoHints::from_ptr", "EAI_FAMILY", "EAI_SOCKTYPE", "EAI_SERVICE"):
        if token not in api:
            findings.append(f"getaddrinfo must validate hints and return EAI errors; missing {token}")
    if "panic!(\"IPv6 is not supported\")" in api:
        findings.append("getaddrinfo must not panic for IPv6")
    if "unwrap_or(0)" in block:
        findings.append("getaddrinfo must not silently map bad service names to port 0")
    if "Vec::with_capacity(max_results)" not in block:
        findings.append("getaddrinfo must reserve addrinfo storage before writing internal pointers")
    if "copy_canonname_to_aibuf" not in block or "ai.ai_canonname" not in api:
        findings.append("AI_CANONNAME must either be rejected or populate stable ai_canonname storage")
    if "canonname:" not in api:
        findings.append("addrinfo allocation must include stable canonname storage")
    if "return Err(ctypes::EAI_FAMILY);" in block:
        findings.append("AF_UNSPEC getaddrinfo must filter unsupported IPv6 instead of failing early")
    if "e(sys_getaddrinfo" in libc:
        findings.append("axlibc getaddrinfo must not collapse all EAI errors through errno wrapper")
    return findings


def scan_socket_stat(root: Path) -> list[str]:
    text = read(root / "api/arceos_posix_api/src/imp/net.rs")
    block = function_block(text, "stat")
    findings: list[str] = []
    if "not really implemented" in block:
        findings.append("socket fstat must not advertise success with not-implemented metadata")
    if re.search(r"st_ino:\s*1\s*,", block):
        findings.append("socket fstat must not report every socket inode as 1")
    for token in ("SOCKET_STAT_DEV", "SOCKET_STAT_BLKSIZE", "self as *const Self"):
        if token not in block and token not in text:
            findings.append(f"socket fstat must expose object-derived metadata; missing {token}")
    return findings


def scan_adjtimex(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/uspace/time_abi.rs")
    block = function_block(text, "sys_adjtimex")
    clock_block = function_block(text, "clock_now_duration")
    findings: list[str] = []
    for token in ("TIME_DISCIPLINE", "apply_adjtimex_update", "add_realtime_offset_ns"):
        if token not in text:
            findings.append(f"adjtimex must have real state/effect path; missing {token}")
    if "process.uid() != 0" in block:
        findings.append("adjtimex must use CAP_SYS_TIME capability check, not raw uid-only gate")
    if "can_set_system_time(process)" not in block:
        findings.append("adjtimex must share clock_settime capability semantics")
    if "adjtimex_unsupported_update" not in block or "LinuxError::EOPNOTSUPP" not in block:
        findings.append("adjtimex must reject unsupported effect modes instead of success-without-effect")
    if "general::CLOCK_TAI" not in clock_block or "TIME_DISCIPLINE.tai.load" not in clock_block:
        findings.append("ADJ_TAI must affect CLOCK_TAI or be rejected")
    return findings




def scan_file_lease(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/uspace/fd_table.rs")
    findings: list[str] = []
    if re.search(r"^\s+lease_type:\s*Arc<", text, re.M):
        findings.append("F_SETLEASE must not be a per-FD private field with fake readback")
    for token in (
        "FileLeaseState",
        "file_lease_table",
        "file_lease_type(file)",
        "apply_file_lease(file, lease_type)",
        "release_file_lease_on_last_close(file)",
    ):
        if token not in text:
            findings.append(f"F_SETLEASE/F_GETLEASE must use shared file lease state; missing {token}")
    block = function_block(text, "fcntl_setlease")
    if "file_lease_access_allowed" not in block or "LinuxError::EAGAIN" not in block:
        findings.append("F_SETLEASE must validate access mode and return EAGAIN for incompatible leases")
    return findings


def scan_synthetic_pid1_state(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/uspace/resource_sched.rs")
    findings: list[str] = []
    impl_start = text.find("impl UserProcessRef")
    impl_end = text.find("pub(super) fn sys_getpriority", impl_start)
    user_ref_impl = text[impl_start:impl_end if impl_end >= 0 else len(text)] if impl_start >= 0 else ""
    for token in (
        "SYNTHETIC_INIT_NICE",
        "SYNTHETIC_INIT_IOPRIO",
        "SYNTHETIC_INIT_NICE.store",
        "SYNTHETIC_INIT_IOPRIO.store",
    ):
        if token not in text:
            findings.append(f"synthetic PID1 priority/ioprio must be stateful, not no-op; missing {token}")
    if re.search(r"UserProcessRef::InitProcess\s*=>\s*\{\s*\}", user_ref_impl):
        findings.append("synthetic PID1 setters still no-op while returning success")
    for setter, store_token in (
        ("set_nice", "SYNTHETIC_INIT_NICE.store"),
        ("set_ioprio", "SYNTHETIC_INIT_IOPRIO.store"),
    ):
        setter_pos = user_ref_impl.find(f"fn {setter}")
        if setter_pos < 0:
            findings.append(f"synthetic PID1 {setter} state update path is missing")
            continue
        next_fn = user_ref_impl.find("\n    fn ", setter_pos + 1)
        setter_block = user_ref_impl[setter_pos:next_fn if next_fn >= 0 else len(user_ref_impl)]
        if store_token not in setter_block:
            findings.append(f"synthetic PID1 {setter} must store state instead of fake success")
    return findings

def scan_api_stat(root: Path) -> list[str]:
    text = read(root / "api/arceos_posix_api/src/imp/fs.rs")
    findings: list[str] = []
    if re.search(r"st_ino:\s*1\s*,", text):
        findings.append("api fs stat still reports every inode as 1")
    for token in ("path_inode", "stat_from_parts", "path: String"):
        if token not in text:
            findings.append(f"api fs stat/fstat must keep path-derived metadata; missing {token}")
    if "file_attr_to_stat(metadata, Some(self.path.as_str()))" not in text:
        findings.append("api fstat must pass the opened path into file_attr_to_stat")
    if text.count("st_uid:") > 1 or text.count("st_gid:") > 1:
        findings.append("api fs uid/gid defaults should stay centralized, not scattered per object")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    root = args.root.resolve()
    findings: list[str] = []
    findings.extend(scan_stdio_close(root))
    findings.extend(scan_block_device(root))
    findings.extend(scan_getaddrinfo(root))
    findings.extend(scan_socket_stat(root))
    findings.extend(scan_adjtimex(root))
    findings.extend(scan_api_stat(root))
    findings.extend(scan_file_lease(root))
    findings.extend(scan_synthetic_pid1_state(root))
    if findings:
        print("G009 post-review real-semantics static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("G009 post-review real-semantics static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
