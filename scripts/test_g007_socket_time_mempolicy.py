#!/usr/bin/env python3
"""Regression tests for the G007 socket/time/mempolicy static guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_g007_socket_time_mempolicy.py"
TARGETS = [
    Path("examples/shell/src/uspace/fd_socket.rs"),
    Path("examples/shell/src/uspace/time_abi.rs"),
    Path("examples/shell/src/uspace/memory_policy.rs"),
    Path("examples/shell/src/uspace/syscall_dispatch.rs"),
]


class G007SocketTimeMempolicyGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="g007-guard-"))
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

    def test_detects_sigev_thread_fake_success(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/time_abi.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "value if value == general::SIGEV_THREAD as i32 => Err(LinuxError::EINVAL),",
            "value if value == general::SIGEV_THREAD as i32 => {\n            // Accept it as a non-delivering kernel timer.\n            Ok(PosixTimerNotify::None)\n        },",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SIGEV_THREAD", result.stdout)

    def test_detects_itimer_virtual_prof_rejection(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/time_abi.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "    interval_cell.store(interval_us, Ordering::Release);",
            "    if which != general::ITIMER_REAL as i32 && (first_us != 0 || interval_us != 0) {\n"
            "        return neg_errno(LinuxError::EOPNOTSUPP);\n"
            "    }\n\n"
            "    interval_cell.store(interval_us, Ordering::Release);",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("ITIMER_VIRTUAL/PROF", result.stdout)

    def test_detects_recvmsg_first_iov_only(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/fd_socket.rs"
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
        path = tree / "examples/shell/src/uspace/fd_socket.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "    } else {\n        neg_errno_code(setsockopt_unsupported_errno_code(level_i32))\n    }",
            "    } else if level_i32 == SOL_SOCKET_LEVEL && optname_i32 == SO_REUSEADDR_OPT {\n        0\n    } else {\n        neg_errno_code(setsockopt_unsupported_errno_code(level_i32))\n    }",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SO_REUSEADDR_OPT", result.stdout)

    def test_detects_mempolicy_ignored_mode(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/memory_policy.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "match default_policy_only(process, mode, nodemask, maxnode) {\n        Ok(()) => 0,\n        Err(err) => neg_errno(err),\n    }",
            "let _ = mode;\n    validate_mempolicy_request(process, nodemask, maxnode)",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("mbind", result.stdout)

    def test_detects_missing_mbind_flags_dispatch(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/syscall_dispatch.rs"
        text = path.read_text(encoding="utf-8")
        old = """general::__NR_mbind => sys_mbind(
            process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),"""
        new = """general::__NR_mbind => sys_mbind(
            process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),"""
        self.assertIn(old, text)
        text = text.replace(old, new, 1)
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("arg5", result.stdout)


if __name__ == "__main__":
    unittest.main()
