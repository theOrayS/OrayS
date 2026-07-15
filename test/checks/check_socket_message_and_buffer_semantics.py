#!/usr/bin/env python3
"""Guard socket message and buffer options against state-free success."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from source_scan import function_block, read


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
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    root = args.root.resolve()
    findings = scan_socket(root)
    if findings:
        print("socket message and buffer semantics check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("socket message and buffer semantics check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
