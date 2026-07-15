#!/usr/bin/env python3
"""Regression tests for the POSIX state integrity guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_posix_state_integrity.py"
TARGETS = [
    Path("api/arceos_posix_api/src/imp/fd_ops.rs"),
    Path("api/arceos_posix_api/src/imp/fs.rs"),
    Path("api/arceos_posix_api/src/imp/net.rs"),
    Path("ulib/axlibc/src/net.rs"),
    Path("user/shell/src/uspace/fd_table.rs"),
    Path("user/shell/src/uspace/linux_abi.rs"),
    Path("user/shell/src/uspace/metadata.rs"),
    Path("user/shell/src/uspace/time_abi.rs"),
    Path("user/shell/src/uspace/resource_sched.rs"),
]


class PosixStateIntegrityGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="posix-state-integrity-guard-"))
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

    def test_detects_stdio_close_fake_success(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/fd_ops.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "syscall_body!(sys_close, close_file_like(fd).map(|_| 0))",
            "if (0..=2).contains(&fd) { return 0; }\n    syscall_body!(sys_close, close_file_like(fd).map(|_| 0))",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("stdin/stdout/stderr", result.stdout)

    def test_detects_numeric_stdio_ioctl_fake(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "matches!(\n            self.entry(fd),\n            Ok(FdEntry::Stdin(_) | FdEntry::Stdout(_) | FdEntry::Stderr(_))\n        )",
            "matches!(fd, 0..=2)",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("is_stdio", result.stdout)

    def test_detects_block_write_bitbucket(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("FdEntry::BlockDevice(dev) => dev.write(src),", "FdEntry::BlockDevice(_) => Ok(src.len()),")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("bit-buckets", result.stdout)

    def test_detects_block_capacity_mismatch(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "let size: u64 = SYNTHETIC_BLOCK_DEVICE_SIZE;",
            "let size: u64 = 512 * 1024 * 1024;",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("BLKGETSIZE64", result.stdout)

    def test_detects_getaddrinfo_ignored_hints_and_panic(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("hints: *const ctypes::addrinfo", "_hints: *const ctypes::addrinfo", 1)
        text += '\nfn fake_ipv6() { panic!("IPv6 is not supported"); }\n'
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("IPv6", result.stdout)

    def test_detects_getaddrinfo_unreserved_internal_pointers(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("Vec::with_capacity(max_results)", "Vec::new()")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("internal pointers", result.stdout)

    def test_detects_getaddrinfo_missing_canonname_storage(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("copy_canonname_to_aibuf(&mut out[i], canonname.as_deref());", "")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("AI_CANONNAME", result.stdout)

    def test_detects_socket_stat_inode_one(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("st_ino: inode,", "st_ino: 1,")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("socket inode", result.stdout)

    def test_detects_adjtimex_uid_gate(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/time_abi.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("!can_set_system_time(process)", "process.uid() != 0")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("CAP_SYS_TIME", result.stdout)

    def test_detects_clock_tai_ignoring_adjtimex_tai(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/time_abi.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "general::CLOCK_TAI => Ok(adjusted_wall_time_with_extra_ns(\n            TIME_DISCIPLINE.tai.load(Ordering::Acquire) as i128 * NSEC_PER_SEC,\n        )),",
            "general::CLOCK_TAI => Ok(adjusted_wall_time()),",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("ADJ_TAI", result.stdout)


    def test_detects_per_fd_fake_lease_state(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("offset: Arc<Mutex<u64>>,", "offset: Arc<Mutex<u64>>,\n    lease_type: Arc<Mutex<u32>>,")
        text = text.replace("file_lease_type(file) as i32", "*file.lease_type.lock() as i32")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("F_SETLEASE", result.stdout)

    def test_detects_synthetic_pid1_noop_success(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/resource_sched.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "UserProcessRef::InitProcess => {\n                SYNTHETIC_INIT_NICE.store(clamp_nice(nice), Ordering::Release)\n            }",
            "UserProcessRef::InitProcess => {}",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("PID1", result.stdout)

    def test_detects_api_stat_inode_one(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/fs.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("st_ino: path_inode(path),", "st_ino: 1,")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("inode", result.stdout)

    def test_detects_api_fstat_missing_opened_path(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/fs.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "Ok(file_attr_to_stat(metadata, Some(self.path.as_str())))",
            "Ok(file_attr_to_stat(metadata, None))",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("fstat", result.stdout)


if __name__ == "__main__":
    unittest.main()
