#!/usr/bin/env python3
"""Regression tests for the G013 user-copy boundary guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_g013_user_copy_boundary.py"
TARGETS = [
    Path("api/arceos_posix_api/src/utils.rs"),
    Path("api/arceos_posix_api/src/imp"),
]


class G013UserCopyBoundaryGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="g013-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            src = ROOT / rel
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            if src.is_dir():
                shutil.copytree(src, dst, dirs_exist_ok=True)
            else:
                shutil.copy2(src, dst)
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

    def test_detects_raw_copy_primitive_in_syscall_impl(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/io.rs"
        path.write_text(
            path.read_text(encoding="utf-8")
            + "\nunsafe fn bad_copy(src: *const u8, dst: *mut u8) { core::ptr::copy_nonoverlapping(src, dst, 1); }\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("raw memory copy", result.stdout)

    def test_detects_core_ptr_write_in_syscall_impl(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/io.rs"
        path.write_text(
            path.read_text(encoding="utf-8")
            + "\nunsafe fn bad_write(dst: *mut u8) { core::ptr::write(dst, 1); }\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("raw memory copy", result.stdout)

    def test_detects_ptr_method_write_in_syscall_impl(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/io.rs"
        path.write_text(
            path.read_text(encoding="utf-8")
            + "\nunsafe fn bad_method(buf_ptr: *mut u8) { unsafe { buf_ptr.write(1); } }\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("raw pointer method", result.stdout)

    def test_detects_multiline_unsafe_raw_deref(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/io.rs"
        path.write_text(
            path.read_text(encoding="utf-8")
            + "\nunsafe fn bad_multiline(src: *const u8) -> u8 { unsafe {\n    *src\n} }\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("multiline unsafe raw deref", result.stdout)

    def test_detects_epoll_ctl_raw_event_deref(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/io_mpx/epoll.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "Some(unsafe { read_user_value(event as *const ctypes::epoll_event)? })",
            "Some(unsafe { *event })",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sys_epoll_ctl", result.stdout)

    def test_detects_getaddrinfo_hints_raw_deref(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "unsafe { read_user_value(hints) }.map_err(|_| ctypes::EAI_SYSTEM)?",
            "unsafe { *hints }",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("ResolvedAddrInfoHints", result.stdout)

    def test_detects_missing_shared_helper_contract(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/utils.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace("pub unsafe fn read_user_value", "pub unsafe fn read_abi_value", 1),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing helper contract", result.stdout)


if __name__ == "__main__":
    unittest.main()
