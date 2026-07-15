#!/usr/bin/env python3
"""Focused regression checks for socket identity and listener introspection."""

from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FD_SOCKET = ROOT / "user/shell/src/uspace/fd_socket.rs"
POSIX_NET = ROOT / "api/arceos_posix_api/src/imp/net.rs"
POSIX_LIB = ROOT / "api/arceos_posix_api/src/lib.rs"
POSIX_FD = ROOT / "api/arceos_posix_api/src/imp/fd_ops.rs"
TCP = ROOT / "kernel/net/axnet/src/smoltcp_impl/tcp.rs"
FD_TABLE = ROOT / "user/shell/src/uspace/fd_table.rs"


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def braced_block(text: str, marker: str) -> str:
    start = text.index(marker)
    brace = text.index("{", start + len(marker))
    depth = 0
    for offset in range(brace, len(text)):
        if text[offset] == "{":
            depth += 1
        elif text[offset] == "}":
            depth -= 1
            if depth == 0:
                return text[start : offset + 1]
    raise AssertionError(f"unclosed Rust block after {marker!r}")


class SocketIntrospectionRegressionTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.fd_socket = read(FD_SOCKET)
        cls.posix_net = read(POSIX_NET)
        cls.posix_lib = read(POSIX_LIB)
        cls.posix_fd = read(POSIX_FD)
        cls.tcp = read(TCP)
        cls.fd_table = read(FD_TABLE)

    def test_inet_identity_comes_from_backend_socket_variant(self) -> None:
        identity = braced_block(self.posix_net, "fn identity(&self)")
        for token in (
            "Socket::Udp(_)",
            "ctypes::AF_INET as c_int",
            "ctypes::IPPROTO_UDP as c_int",
            "Socket::Tcp(tcpsocket)",
            "ctypes::IPPROTO_TCP as c_int",
            "tcpsocket.lock().is_listening()",
        ):
            self.assertIn(token, identity)
        self.assertIn("socket_identity", self.posix_lib)

        readonly = braced_block(self.fd_socket, "fn socket_readonly_scalar(")
        self.assertIn("arceos_posix_api::socket_identity(socket.posix_fd)?", readonly)
        self.assertIn("SO_DOMAIN_OPT => domain", readonly)
        self.assertIn("SO_PROTOCOL_OPT => protocol", readonly)
        self.assertIn("SO_ACCEPTCONN_OPT => i32::from(is_listening)", readonly)
        self.assertNotIn("SO_DOMAIN_OPT => Some(posix_ctypes::AF_INET", readonly)

    def test_inet_acceptconn_tracks_listen_transition(self) -> None:
        listen = braced_block(self.tcp, "pub fn listen(&self)")
        self.assertIn("self.update_state(STATE_CLOSED, STATE_LISTENING", listen)
        is_listening = braced_block(self.tcp, "pub fn is_listening(&self)")
        self.assertIn("self.get_state() == STATE_LISTENING", is_listening)

    def test_unix_stream_dgram_and_socketpair_identity_matrix(self) -> None:
        getsockopt = braced_block(self.fd_socket, "fn sys_getsockopt_local_socket(")
        self.assertIn("SO_DOMAIN_OPT => AF_UNIX_DOMAIN", getsockopt)
        self.assertIn("SO_PROTOCOL_OPT => 0", getsockopt)
        self.assertIn(
            "SO_ACCEPTCONN_OPT => i32::from(local_socket_is_listening(socket.id))",
            getsockopt,
        )

        new_pair = braced_block(self.fd_socket, "pub(super) fn new_pair(")
        self.assertNotIn("local_socket_listeners", new_pair)
        self.assertNotIn("is_listening", new_pair)

        listen = braced_block(self.fd_socket, "fn sys_listen_local_socket(")
        self.assertIn("socket.socktype as u32 != posix_ctypes::SOCK_STREAM", listen)
        self.assertIn("listeners.push(LocalSocketListener", listen)

    def test_unix_acceptconn_uses_listener_existence_not_pending_queue(self) -> None:
        listener_state = braced_block(self.fd_socket, "fn local_socket_is_listening(")
        self.assertIn("listener.owner_id == owner_id", listener_state)
        self.assertNotIn("pending", listener_state)

    def test_scalar_copyout_preserves_linux_short_optlen_semantics(self) -> None:
        copyout = braced_block(self.fd_socket, "fn write_socket_scalar_option(")
        self.assertIn("cmp::min(requested_len, size_of::<i32>())", copyout)
        self.assertIn("&value.to_ne_bytes()[..copy_len]", copyout)
        self.assertIn("let out_len = copy_len as posix_ctypes::socklen_t", copyout)

        inet = braced_block(self.fd_socket, "pub(super) fn sys_getsockopt_bridge(")
        inet_scalar = inet[inet.index("let value = match socket_readonly_scalar") :]
        self.assertNotIn("len < size_of::<i32>()", inet_scalar)
        local = braced_block(self.fd_socket, "fn sys_getsockopt_local_socket(")
        local_scalar = local[local.index("let value = if level == SOL_SOCKET_LEVEL") :]
        self.assertNotIn("len < size_of::<i32>()", local_scalar)
        identity_option = braced_block(self.fd_socket, "fn socket_identity_option(")
        for token in ("SO_DOMAIN_OPT", "SO_PROTOCOL_OPT", "SO_ACCEPTCONN_OPT"):
            self.assertIn(token, identity_option)

    def test_existing_so_error_and_so_type_length_handling_is_unchanged(self) -> None:
        readonly = braced_block(self.fd_socket, "fn socket_readonly_scalar(")
        self.assertIn("SO_ERROR_OPT => Some(0)", readonly)
        local = braced_block(self.fd_socket, "fn sys_getsockopt_local_socket(")
        self.assertIn("SO_ERROR_OPT => 0", local)
        for marker in (
            "pub(super) fn sys_getsockopt_bridge(",
            "fn sys_getsockopt_local_socket(",
        ):
            getsockopt = braced_block(self.fd_socket, marker)
            self.assertIn(
                "if len < size_of::<i32>() && !socket_identity_option(optname)",
                getsockopt,
            )

    def test_invalid_user_pointers_still_fail_through_copy_helpers(self) -> None:
        copyout = braced_block(self.fd_socket, "fn write_socket_scalar_option(")
        self.assertIn("validate_user_write(process, optval, copy_len)", copyout)
        for marker in (
            "pub(super) fn sys_getsockopt_bridge(",
            "fn sys_getsockopt_local_socket(",
        ):
            getsockopt = braced_block(self.fd_socket, marker)
            self.assertIn("if optval == 0 || optlen == 0", getsockopt)
            self.assertIn("read_user_value::<posix_ctypes::socklen_t>(process, optlen)", getsockopt)

    def test_regular_files_remain_enotsock(self) -> None:
        socket_entry = braced_block(self.fd_socket, "pub(super) fn socket_entry(")
        self.assertIn("_ => Err(LinuxError::ENOTSOCK)", socket_entry)
        local = braced_block(self.fd_socket, "fn sys_getsockopt_local_socket(")
        self.assertIn("Ok(_) => return neg_errno(LinuxError::ENOTSOCK)", local)

    def test_unknown_options_remain_errors(self) -> None:
        inet = braced_block(self.fd_socket, "pub(super) fn sys_getsockopt_bridge(")
        self.assertIn("getsockopt_unsupported_errno_code(&socket, level)", inet)
        local = braced_block(self.fd_socket, "fn sys_getsockopt_local_socket(")
        self.assertIn("_ => return neg_errno_code(LINUX_ENOPROTOOPT)", local)
        self.assertNotIn("_ => 0", braced_block(self.fd_socket, "fn socket_readonly_scalar("))

    def test_listener_state_stays_shared_across_dup_and_fork(self) -> None:
        inet_duplicate = braced_block(
            self.fd_socket,
            "pub(super) fn duplicate(&self) -> Result<Self, LinuxError>",
        )
        self.assertIn("arceos_posix_api::sys_dup(self.posix_fd)", inet_duplicate)
        dup_backend = braced_block(self.posix_fd, "fn dup_fd_from(")
        self.assertIn("let f = get_file_like(old_fd)?", dup_backend)
        self.assertIn("FdEntry::with_flags(f, fd_flags)", dup_backend)

        local_duplicate = braced_block(self.fd_socket, "pub(super) fn duplicate(&self) -> Self")
        self.assertIn("id: self.id", local_duplicate)
        fork_duplicate = braced_block(self.fd_table, "pub(super) fn duplicate_for_fork(&self)")
        self.assertIn("Self::Socket(socket) => socket.duplicate().map(Self::Socket)", fork_duplicate)
        self.assertIn(
            "Self::LocalSocket(socket) => Ok(Self::LocalSocket(socket.duplicate()))",
            fork_duplicate,
        )

    def test_no_per_fd_listener_cache_was_added(self) -> None:
        socket_entry = braced_block(self.fd_socket, "pub(super) struct SocketEntry")
        local_entry = braced_block(self.fd_socket, "pub(super) struct LocalSocketEntry")
        self.assertNotIn("listening", socket_entry)
        self.assertNotIn("listening", local_entry)


if __name__ == "__main__":
    unittest.main()
