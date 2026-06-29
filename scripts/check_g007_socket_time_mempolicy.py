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
    candidates = [
        pos
        for pos in (
            text.find("\nfn ", start + len(marker)),
            text.find("\npub fn ", start + len(marker)),
            text.find("\npub(super) fn ", start + len(marker)),
            text.find("\n    fn ", start + len(marker)),
            text.find("\n    pub fn ", start + len(marker)),
            text.find("\n    pub(super) fn ", start + len(marker)),
            text.find("\n#[", start + len(marker)),
        )
        if pos >= 0
    ]
    end = min(candidates) if candidates else len(text)
    return text[start:end]


def scan_time(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/time_abi.rs")
    findings: list[str] = []
    parse_block = function_block(text, "parse_posix_timer_notify")
    setitimer_block = function_block(text, "sys_setitimer")
    if not parse_block:
        findings.append("user/shell/src/uspace/time_abi.rs: missing parse_posix_timer_notify")
    else:
        if "SIGEV_THREAD" not in parse_block or "Err(LinuxError::EINVAL)" not in parse_block:
            findings.append("timer_create raw SIGEV_THREAD must fail with EINVAL instead of creating a silent timer")
        if "Accept it as a non-delivering" in parse_block or re.search(r"SIGEV_THREAD[\s\S]{0,240}Ok\(PosixTimerNotify::None\)", parse_block):
            findings.append("timer_create still accepts SIGEV_THREAD as non-delivering fake success")
    if not setitimer_block:
        findings.append("user/shell/src/uspace/time_abi.rs: missing sys_setitimer")
    else:
        required = [
            "itimer_clock_micros(process, which)",
            "deadline_cell.store(now_us.saturating_add(first_us)",
            "process.real_timer_generation",
        ]
        for token in required:
            if token not in setitimer_block:
                findings.append(f"setitimer must arm timers against their real backing clock; missing {token}")
        if "which != general::ITIMER_REAL" in setitimer_block and "LinuxError::EOPNOTSUPP" in setitimer_block:
            findings.append("setitimer must not preserve LTP capability by rejecting armed ITIMER_VIRTUAL/PROF")
        if "virtual/prof" in setitimer_block and "report real state" in setitimer_block:
            findings.append("setitimer still documents fake tracked virtual/prof timer state")
    if "fn itimer_clock_micros" not in text or "general::ITIMER_VIRTUAL" not in text or "general::ITIMER_PROF" not in text:
        findings.append("time_abi must expose real backing clocks for ITIMER_VIRTUAL and ITIMER_PROF")
    if "consume_expired_cpu_timers" not in text:
        findings.append("time_abi must check CPU-backed interval timers on user return")
    return findings


def scan_mempolicy(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/memory_policy.rs")
    dispatch = read(root / "user/shell/src/uspace/syscall_dispatch.rs")
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
    text = read(root / "user/shell/src/uspace/fd_socket.rs")
    findings: list[str] = []
    recvmsg_block = function_block(text, "sys_recvmsg_bridge")
    sockopt_block = function_block(text, "socket_option_supported")
    setsockopt_block = function_block(text, "sys_setsockopt_bridge")
    getsockopt_block = function_block(text, "sys_getsockopt_bridge")
    if not recvmsg_block:
        findings.append("user/shell/src/uspace/fd_socket.rs: missing sys_recvmsg_bridge")
    else:
        for token in ("capped_iovec_write_len", "validate_iovec_write", "scatter_iovec_bytes_to_user", "msg_value.msg_controllen = 0"):
            if token not in recvmsg_block and token not in text:
                findings.append(f"recvmsg must scatter across iovecs and reset ancillary length; missing {token}")
        if "let Some(first_iov)" in recvmsg_block or "basic LTP cases only assert" in recvmsg_block:
            findings.append("recvmsg still contains first-iov-only or LTP-return-value-only fake semantics")
    fake_sockopt_tokens = [
        "SO_REUSEPORT_OPT",
        "SO_DONTROUTE_OPT",
        "SO_BROADCAST_OPT",
        "SO_KEEPALIVE_OPT",
        "IP_RECVERR_OPT",
    ]
    for token in fake_sockopt_tokens:
        if token in sockopt_block or token in setsockopt_block or token in getsockopt_block:
            findings.append(f"socket option path still advertises or accepts unbacked option {token}")
    buffer_tokens = ["SO_SNDBUF_OPT", "SO_RCVBUF_OPT"]
    buffer_backend_tokens = [
        "set_socket_send_buffer_size",
        "set_socket_recv_buffer_size",
        "socket_send_buffer_size",
        "socket_recv_buffer_size",
    ]
    buffer_backed = all(token in text for token in buffer_backend_tokens)
    for token in buffer_tokens:
        if token in sockopt_block or token in setsockopt_block or token in getsockopt_block:
            if not buffer_backed:
                findings.append(
                    f"socket option path still advertises or accepts unbacked option {token}"
                )
    api_net = read(root / "api/arceos_posix_api/src/imp/net.rs")
    axnet_tcp = read(root / "kernel/net/axnet/src/smoltcp_impl/tcp.rs")
    axnet_udp = read(root / "kernel/net/axnet/src/smoltcp_impl/udp.rs")
    axnet_mod = read(root / "kernel/net/axnet/src/smoltcp_impl/mod.rs")
    axnet_loopback = read(root / "kernel/net/axnet/src/smoltcp_impl/loopback.rs")
    axnet_udp_loopback = read(root / "kernel/net/axnet/src/smoltcp_impl/udp_loopback.rs")
    listen_table = read(root / "kernel/net/axnet/src/smoltcp_impl/listen_table.rs")
    backed_sockopts = {
        "SO_REUSEADDR_OPT": [
            (text, "arceos_posix_api::set_socket_reuse_addr"),
            (text, "arceos_posix_api::socket_reuse_addr"),
            (api_net, "fn set_reuse_addr(&self, enabled: bool)"),
            (api_net, "fn reuse_addr(&self) -> bool"),
            (api_net, "pub fn set_socket_reuse_addr"),
            (api_net, "pub fn socket_reuse_addr"),
            (axnet_tcp, "reuse_addr: AtomicBool"),
            (axnet_tcp, "pub fn set_reuse_addr(&self, enabled: bool)"),
            (axnet_tcp, "pub fn reuse_addr(&self) -> bool"),
            (axnet_udp, "reuse_addr: AtomicBool"),
            (axnet_udp, "pub fn set_reuse_addr(&self, enabled: bool)"),
            (axnet_udp, "pub fn reuse_addr(&self) -> bool"),
        ],
        "TCP_NODELAY_OPT": [
            (text, "arceos_posix_api::set_socket_tcp_nodelay"),
            (text, "arceos_posix_api::socket_tcp_nodelay"),
            (api_net, "fn set_tcp_nodelay(&self, enabled: bool)"),
            (api_net, "fn tcp_nodelay(&self) -> LinuxResult<bool>"),
            (api_net, "pub fn set_socket_tcp_nodelay"),
            (api_net, "pub fn socket_tcp_nodelay"),
            (axnet_tcp, "nodelay: AtomicBool"),
            (axnet_tcp, "pub fn set_nodelay(&self, enabled: bool)"),
            (axnet_tcp, "pub fn nodelay(&self) -> bool"),
            (axnet_tcp, "socket.set_nagle_enabled(!self.nodelay())"),
            (axnet_tcp, "socket.set_nagle_enabled(!enabled)"),
        ],
        "TCP_MAXSEG_OPT": [
            (text, "arceos_posix_api::socket_tcp_max_segment_size"),
            (api_net, "fn tcp_max_segment_size(&self) -> LinuxResult<usize>"),
            (api_net, "pub fn socket_tcp_max_segment_size"),
            (axnet_tcp, "pub fn max_segment_size(&self) -> usize"),
            (axnet_tcp, "super::tcp_ipv4_max_segment_size()"),
            (axnet_mod, "fn tcp_ipv4_max_segment_size() -> usize"),
            (axnet_mod, "STANDARD_MTU - IPV4_HEADER_LEN - TCP_HEADER_LEN"),
        ],
        "IP_MCAST_JOIN_GROUP_OPT": [
            (text, "multicast_groups: Arc<Mutex<Vec<Vec<u8>>>>"),
            (text, "fn join_multicast_group(&self, group: Vec<u8>)"),
            (text, "socket.join_multicast_group(group)"),
        ],
        "IP_MCAST_LEAVE_GROUP_OPT": [
            (text, "fn leave_multicast_group(&self, group: &[u8]) -> Result<(), LinuxError>"),
            (text, "LinuxError::EADDRNOTAVAIL"),
            (text, "socket.leave_multicast_group(group.as_slice())"),
        ],
    }
    option_blocks = sockopt_block + setsockopt_block + getsockopt_block
    for token, required_tokens in backed_sockopts.items():
        if token not in option_blocks:
            continue
        for haystack, required in required_tokens:
            if required not in haystack:
                findings.append(f"{token} backend is incomplete; missing {required}")
    if buffer_backed:
        tcp_recv_set = function_block(axnet_tcp, "set_recv_buffer_size")
        tcp_send_set = function_block(axnet_tcp, "set_send_buffer_size")
        backend_required = [
            (api_net, "set_socket_send_buffer_size"),
            (api_net, "socket_recv_buffer_size"),
            (api_net, "socket_buffer_option_error"),
            (api_net, "LinuxError::ENOPROTOOPT"),
            (axnet_tcp, "new_tcp_socket_with_buffer_lengths"),
            (axnet_tcp, "recv_buffer_size"),
            (axnet_tcp, "clear_backing_allocation"),
            (axnet_tcp, "loopback_listener_buffer_sizes"),
            (axnet_tcp, "OperationNotSupported"),
            (axnet_mod, "fn udp_packet_metadata_len"),
            (axnet_mod, "udp_packet_metadata_len(recv_len)"),
            (axnet_mod, "udp_packet_metadata_len(send_len)"),
            (axnet_loopback, "client_to_server_limit"),
            (axnet_loopback, "server_to_client_limit"),
            (axnet_loopback, "loopback_buffer_limit"),
            (axnet_udp, "new_udp_socket_with_buffer_lengths"),
            (axnet_udp, "recreate_unbound_socket"),
            (axnet_udp, "self.loopback_queue.set_recv_buffer_size(size)"),
            (axnet_udp, "buf.len() > self.send_buffer_size"),
            (axnet_udp, "OperationNotSupported"),
            (axnet_udp_loopback, "byte_limit"),
            (axnet_udp_loopback, "packet_limit"),
            (axnet_udp_loopback, "udp_packet_metadata_len(recv_buffer_size)"),
            (listen_table, "recv_buffer_size"),
            (listen_table, "new_tcp_socket_with_buffer_lengths"),
        ]
        for haystack, token in backend_required:
            if token not in haystack:
                findings.append(f"SO_SNDBUF/SO_RCVBUF backend is incomplete; missing {token}")
        if "recv_capacity_limit" in tcp_recv_set or "send_capacity_limit" in tcp_send_set:
            findings.append("SO_SNDBUF/SO_RCVBUF active TCP resize must compare against global requested size before capacity clamping")
        if "size < SOCKET_BUFFER_SIZE_MIN" in api_net or re.search(r"if\s+size\s+<\s+1\b", api_net):
            findings.append("SO_SNDBUF/SO_RCVBUF must clamp zero/small requests instead of rejecting them before backend normalization")
        if "Unsupported,\n                \"socket" in axnet_tcp or "Unsupported,\n                \"socket" in axnet_udp:
            findings.append("SO_SNDBUF/SO_RCVBUF active resize still maps through AxError::Unsupported/ENOSYS")
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
