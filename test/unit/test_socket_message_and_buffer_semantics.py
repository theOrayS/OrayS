#!/usr/bin/env python3
"""Regression tests for socket message and buffer semantics."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_socket_message_and_buffer_semantics.py"
TARGETS = [
    Path("api/arceos_posix_api/src/imp/net.rs"),
    Path("user/shell/src/uspace/fd_socket.rs"),
    Path("kernel/net/axnet/src/smoltcp_impl/mod.rs"),
    Path("kernel/net/axnet/src/smoltcp_impl/tcp.rs"),
    Path("kernel/net/axnet/src/smoltcp_impl/udp.rs"),
    Path("kernel/net/axnet/src/smoltcp_impl/loopback.rs"),
    Path("kernel/net/axnet/src/smoltcp_impl/udp_loopback.rs"),
    Path("kernel/net/axnet/src/smoltcp_impl/listen_table.rs"),
]


class SocketMessageAndBufferSemanticsGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="socket-semantics-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(encoding="utf-8"), encoding="utf-8")
        return tmp

    def run_guard(self, tree: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(GUARD), "--root", str(tree)],
            check=False,
            capture_output=True,
            text=True,
        )

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_recvmsg_first_iov_only(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_socket.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "let receive_len = capped_iovec_write_len(&iov_entries);",
            "let Some(first_iov) = iov_entries.first() else { return 0; };\n    let receive_len = first_iov.iov_len as usize;",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("recvmsg", result.stdout)

    def test_detects_sockopt_unbacked_advertisement(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_socket.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "    } else {\n        neg_errno_code(setsockopt_unsupported_errno_code(level_i32))\n    }",
            "    } else if level_i32 == SOL_SOCKET_LEVEL && optname_i32 == SO_REUSEPORT_OPT {\n        0\n    } else {\n        neg_errno_code(setsockopt_unsupported_errno_code(level_i32))\n    }",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SO_REUSEPORT_OPT", result.stdout)

    def test_detects_socket_buffer_backend_without_listener_plumbing(self) -> None:
        tree = self.make_tree()
        path = tree / "kernel/net/axnet/src/smoltcp_impl/listen_table.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "SocketSetWrapper::new_tcp_socket_with_buffer_lengths(\n                entry.recv_buffer_size,\n                entry.send_buffer_size,\n            )",
            "SocketSetWrapper::new_tcp_socket()",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SO_SNDBUF/SO_RCVBUF backend is incomplete", result.stdout)

    def test_detects_socket_buffer_resize_enosys_mapping(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("LinuxError::ENOPROTOOPT", "LinuxError::EOPNOTSUPP")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("LinuxError::ENOPROTOOPT", result.stdout)

    def test_detects_socket_buffer_zero_rejection(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("if size < 0", "if size < 1", 1)
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("clamp zero/small requests", result.stdout)

    def test_detects_udp_buffer_metadata_not_resized(self) -> None:
        tree = self.make_tree()
        path = tree / "kernel/net/axnet/src/smoltcp_impl/mod.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("udp_packet_metadata_len(recv_len)", "8", 1)
        text = text.replace("udp_packet_metadata_len(send_len)", "8", 1)
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("udp_packet_metadata_len", result.stdout)

    def test_detects_tcp_active_resize_capacity_clamp(self) -> None:
        tree = self.make_tree()
        path = tree / "kernel/net/axnet/src/smoltcp_impl/tcp.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "normalize_socket_buffer_len(size, TCP_RX_BUF_LEN)",
            "normalize_socket_buffer_len(size, self.recv_capacity_limit()?)",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("active TCP resize", result.stdout)

    def test_detects_tcp_loopback_without_buffer_limits(self) -> None:
        tree = self.make_tree()
        path = tree / "kernel/net/axnet/src/smoltcp_impl/loopback.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("client_to_server_limit", "client_to_server_unbounded")
        text = text.replace("server_to_client_limit", "server_to_client_unbounded")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("client_to_server_limit", result.stdout)

    def test_detects_udp_loopback_without_buffer_limits(self) -> None:
        tree = self.make_tree()
        path = tree / "kernel/net/axnet/src/smoltcp_impl/udp_loopback.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("byte_limit", "unbounded_bytes")
        text = text.replace("packet_limit", "unbounded_packets")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("byte_limit", result.stdout)

if __name__ == "__main__":
    unittest.main()
