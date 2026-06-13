#!/usr/bin/env python3
"""Regression tests for the G012 syscall review hotspot guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_g012_syscall_review_hotspots.py"
TARGETS = [
    Path("examples/shell/src/uspace/mod.rs"),
    Path("examples/shell/src/uspace/futex.rs"),
    Path("examples/shell/src/uspace/signal_abi.rs"),
    Path("examples/shell/src/uspace/memory_map.rs"),
    Path("examples/shell/src/uspace/process_lifecycle.rs"),
    Path("examples/shell/src/uspace/task_context.rs"),
    Path("examples/shell/src/uspace/user_memory.rs"),
    Path("examples/shell/src/uspace/mount_abi.rs"),
    Path("examples/shell/src/uspace/fd_table.rs"),
    Path("examples/shell/src/uspace/system_info.rs"),
    Path("examples/shell/src/uspace/time_abi.rs"),
    Path("examples/shell/src/uspace/resource_sched.rs"),
]


class G012SyscallReviewHotspotGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="g012-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(encoding="utf-8"), encoding="utf-8")
        return tmp

    def run_guard(self, tree: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run([sys.executable, str(GUARD), "--root", str(tree)], check=False, capture_output=True, text=True)

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_empty_log_read_cstr(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/user_memory.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn log_read_cstr_efault")
        text = text[:start] + "fn log_read_cstr_efault() {\n}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("log_read_cstr_efault", result.stdout)

    def test_detects_empty_central_user_trace(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/mod.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "let _ = core::format_args!($($arg)*);",
                "",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("user_trace", result.stdout)

    def test_detects_local_user_trace_shadow(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/futex.rs"
        path.write_text(
            path.read_text(encoding="utf-8") + "\nmacro_rules! user_trace { ($($arg:tt)*) => {}; }\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("local empty user_trace", result.stdout)

    def test_detects_mount_root_alias(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/mount_abi.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            'axfs::api::mount_fatfs(mount_path, dev, format).map_err(LinuxError::from)?;\n            Ok(target_path.into())',
            'Ok("/".into())',
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("mount", result.stdout)

    def test_detects_fsync_catch_all_success(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("pub(super) fn sys_fsync")
        end = text.index("pub(super) fn sys_renameat2", start)
        block = text[start:end].replace(
            "Err(err) => neg_errno(err),",
            "Ok(_) => 0,\n        Err(err) => neg_errno(err),",
            1,
        )
        path.write_text(text[:start] + block + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sys_fsync", result.stdout)

    def test_detects_siocsifflags_validate_success(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn socket_ioctl_set_ifflags")
        end = text.index("fn write_user_bytes_ret", start)
        fake_success = """fn socket_ioctl_set_ifflags(process: &UserProcess, arg: usize) -> isize {
    const IFREQ_SIZE: usize = 40;
    if arg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    match validate_user_read(process, arg, IFREQ_SIZE) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

"""
        path.write_text(text[:start] + fake_success + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SIOCSIFFLAGS", result.stdout)

    def test_detects_syslog_privileged_noop(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        path.write_text(path.read_text(encoding="utf-8") + "\n// PrivilegedNoop\n", encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("PrivilegedNoop", result.stdout)

    def test_detects_syslog_write_only_state(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        path.write_text(
            path.read_text(encoding="utf-8") + "\nstatic SYSLOG_OPEN: usize = 0;\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SYSLOG", result.stdout)

    def test_detects_syslog_missing_explicit_unsupported(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn unsupported_privileged_syslog")
        end = text.index("pub(super) fn sys_getcpu", start)
        block = text[start:end].replace("LinuxError::EOPNOTSUPP", "LinuxError::EINVAL", 1)
        path.write_text(text[:start] + block + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("EOPNOTSUPP", result.stdout)

    def test_detects_syslog_state_action_success_arm(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(
            text.replace(
                "SyslogAction::Close | SyslogAction::Open => unsupported_privileged_syslog(process),",
                "SyslogAction::Close | SyslogAction::Open => 0,",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SyslogAction::Close", result.stdout)

    def test_detects_times_half_split(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/time_abi.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "let user_ticks = elapsed;\n    let system_ticks = 0;",
                "let user_ticks = elapsed / 2;\n    let system_ticks = elapsed.saturating_sub(user_ticks);",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("process_times", result.stdout)

    def test_detects_sched_deadline_stored_without_backend(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/resource_sched.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn sched_state_from_attr")
        end = text.index("pub(super) fn sys_sched_getattr", start)
        block = text[start:end].replace("LinuxError::EOPNOTSUPP", "LinuxError::EINVAL", 1)
        path.write_text(text[:start] + block + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sched_setattr SCHED_DEADLINE", result.stdout)


if __name__ == "__main__":
    unittest.main()
