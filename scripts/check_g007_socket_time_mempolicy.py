#!/usr/bin/env python3
"""Static guard for G007 socket/time/mempolicy no-fake-success boundaries."""

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
        return ""
    candidates = [pos for pos in (text.find("\nfn ", start + len(marker)), text.find("\npub(super) fn ", start + len(marker)), text.find("\n#[", start + len(marker))) if pos >= 0]
    end = min(candidates) if candidates else len(text)
    return text[start:end]


def scan_time(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/uspace/time_abi.rs")
    findings: list[str] = []
    parse_block = function_block(text, "parse_posix_timer_notify")
    setitimer_block = function_block(text, "sys_setitimer")
    if not parse_block:
        findings.append("examples/shell/src/uspace/time_abi.rs: missing parse_posix_timer_notify")
    else:
        if "SIGEV_THREAD" not in parse_block or "Err(LinuxError::EINVAL)" not in parse_block:
            findings.append("timer_create raw SIGEV_THREAD must fail with EINVAL instead of creating a silent timer")
        if "Accept it as a non-delivering" in parse_block or re.search(r"SIGEV_THREAD[\s\S]{0,240}Ok\(PosixTimerNotify::None\)", parse_block):
            findings.append("timer_create still accepts SIGEV_THREAD as non-delivering fake success")
    if not setitimer_block:
        findings.append("examples/shell/src/uspace/time_abi.rs: missing sys_setitimer")
    else:
        required = ["which != general::ITIMER_REAL", "first_us != 0 || interval_us != 0", "LinuxError::EOPNOTSUPP"]
        for token in required:
            if token not in setitimer_block:
                findings.append(f"setitimer must reject armed ITIMER_VIRTUAL/PROF with unsupported errno; missing {token}")
        if "virtual/prof" in setitimer_block and "report real state" in setitimer_block:
            findings.append("setitimer still documents fake tracked virtual/prof timer state")
    return findings


def scan_mempolicy(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/uspace/memory_policy.rs")
    dispatch = read(root / "examples/shell/src/uspace/syscall_dispatch.rs")
    findings: list[str] = []
    if "fn nodemask_is_empty" not in text or "fn default_policy_only" not in text:
        findings.append("memory policy must validate default-only state with nodemask contents")
    if "LinuxError::EOPNOTSUPP" not in text:
        findings.append("memory policy must return EOPNOTSUPP for unsupported NUMA policy changes")
    mbind_block = function_block(text, "sys_mbind")
    set_block = function_block(text, "sys_set_mempolicy")
    get_block = function_block(text, "sys_get_mempolicy")
    if "default_policy_only" not in mbind_block:
        findings.append("mbind must reject non-default mempolicy modes instead of validating only nodemask")
    if "flags: usize" not in mbind_block or "flags != MEMBIND_SUPPORTED_FLAGS" not in mbind_block:
        findings.append("mbind must consume and reject unsupported flags instead of ignoring arg5")
    if "tf.arg5()" not in dispatch[dispatch.find("general::__NR_mbind") : dispatch.find("general::__NR_get_mempolicy")]:
        findings.append("syscall_dispatch must pass mbind arg5 flags to sys_mbind")
    if re.search(r"let\s+_\s*=\s*\(start,\s*len,\s*mode\);\s*validate_mempolicy_request", mbind_block):
        findings.append("mbind still ignores range/mode and validates only the nodemask")
    if "default_policy_only" not in set_block or re.search(r"let\s+_\s*=\s*mode;\s*validate_mempolicy_request", set_block):
        findings.append("set_mempolicy must not accept every validated mode as success")
    if "flags != 0" not in get_block or "LinuxError::EOPNOTSUPP" not in get_block:
        findings.append("get_mempolicy must reject unsupported flags instead of clearing outputs")
    return findings


def scan_socket(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/uspace/fd_socket.rs")
    findings: list[str] = []
    recvmsg_block = function_block(text, "sys_recvmsg_bridge")
    sockopt_block = function_block(text, "socket_option_supported")
    setsockopt_block = function_block(text, "sys_setsockopt_bridge")
    getsockopt_block = function_block(text, "sys_getsockopt_bridge")
    if not recvmsg_block:
        findings.append("examples/shell/src/uspace/fd_socket.rs: missing sys_recvmsg_bridge")
    else:
        for token in ("capped_iovec_write_len", "validate_iovec_write", "scatter_iovec_bytes_to_user", "msg_value.msg_controllen = 0"):
            if token not in recvmsg_block and token not in text:
                findings.append(f"recvmsg must scatter across iovecs and reset ancillary length; missing {token}")
        if "let Some(first_iov)" in recvmsg_block or "basic LTP cases only assert" in recvmsg_block:
            findings.append("recvmsg still contains first-iov-only or LTP-return-value-only fake semantics")
    fake_sockopt_tokens = [
        "SO_REUSEADDR_OPT",
        "SO_REUSEPORT_OPT",
        "SO_DONTROUTE_OPT",
        "SO_BROADCAST_OPT",
        "SO_KEEPALIVE_OPT",
        "SO_SNDBUF_OPT",
        "SO_RCVBUF_OPT",
        "IP_RECVERR_OPT",
        "MCAST_JOIN_GROUP_OPT",
        "MCAST_LEAVE_GROUP_OPT",
        "TCP_NODELAY_OPT",
        "TCP_MAXSEG_OPT",
    ]
    for token in fake_sockopt_tokens:
        if token in sockopt_block or token in setsockopt_block or token in getsockopt_block:
            findings.append(f"socket option path still advertises or accepts unbacked option {token}")
    forbidden_patterns = [
        "ip_mcast_joined",
        "SO_SNDBUFFORCE_OPT | SO_RCVBUFFORCE_OPT",
        "TCP_INFO_OPT",
        "clear_user_bytes(process, optval",
    ]
    for token in forbidden_patterns:
        if token in setsockopt_block or token in getsockopt_block or token in text.split("pub(super) struct SocketOptions", 1)[-1].split("#[derive(Clone)]", 1)[0]:
            findings.append(f"socket option path still contains unbacked success token {token}")
    if "SO_RCVTIMEO_OPT | SO_SNDTIMEO_OPT" not in setsockopt_block:
        findings.append("setsockopt must keep the backed send/recv timeout path")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    root = args.root.resolve()
    findings: list[str] = []
    findings.extend(scan_time(root))
    findings.extend(scan_mempolicy(root))
    findings.extend(scan_socket(root))
    if findings:
        print("G007 socket/time/mempolicy static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("G007 socket/time/mempolicy static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
