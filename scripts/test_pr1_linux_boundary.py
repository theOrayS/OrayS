#!/usr/bin/env python3
"""Regression tests for the PR1 Linux boundary static guard."""

from __future__ import annotations

import importlib.util
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_pr1_linux_boundary.py"
TARGETS = (
    Path("api/orays_linux"),
    Path("api/orays_linux_abi"),
    Path("user/shell/Cargo.toml"),
    Path("user/shell/src/uspace"),
)

SPEC = importlib.util.spec_from_file_location("check_pr1_linux_boundary", GUARD)
assert SPEC is not None and SPEC.loader is not None
GUARD_MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GUARD_MODULE)


class Pr1LinuxBoundaryGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="pr1-linux-boundary-"))
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

    def assert_guard_fails(self, tree: Path, message: str) -> None:
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn(message, result.stdout)

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_reverse_or_implementation_dependency(self) -> None:
        tree = self.make_tree()
        path = tree / "api/orays_linux/Cargo.toml"
        text = path.read_text(encoding="utf-8").replace(
            "orays-linux-abi = { workspace = true }",
            'orays-linux-abi = { workspace = true }\narceos-shell = { path = "../../user/shell" }',
            1,
        )
        path.write_text(text, encoding="utf-8")
        self.assert_guard_fails(tree, "reverse/implementation dependency")

    def test_detects_target_specific_reverse_dependency(self) -> None:
        tree = self.make_tree()
        path = tree / "api/orays_linux/Cargo.toml"
        path.write_text(
            path.read_text(encoding="utf-8")
            + "\n[target.'cfg(target_arch = \"riscv64\")'.dependencies]\n"
            + 'arceos-shell = { path = "../../user/shell" }\n',
            encoding="utf-8",
        )
        self.assert_guard_fails(tree, "reverse/implementation dependency")

    def test_detects_unsafe_in_boundary_crate(self) -> None:
        tree = self.make_tree()
        path = tree / "api/orays_linux/src/syscall.rs"
        path.write_text(path.read_text(encoding="utf-8") + "\nfn spread() { unsafe {} }\n", encoding="utf-8")
        self.assert_guard_fails(tree, "unsafe is forbidden")

    def test_detects_legacy_user_copy_caller_growth(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/credentials.rs"
        path.write_text(path.read_text(encoding="utf-8") + "\n// read_user_value(process, ptr)\n", encoding="utf-8")
        self.assert_guard_fails(tree, "caller inventory changed for read_user_value")

    def test_detects_legacy_user_copy_caller_loss(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        call = "validate_user_write(process, base, len)"
        self.assertIn(call, text)
        path.write_text(
            text.replace(call, "validate_user_write_removed(process, base, len)", 1),
            encoding="utf-8",
        )
        self.assert_guard_fails(tree, "caller inventory changed for validate_user_write")

    def test_detects_raw_boundary_visibility_growth(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/user_memory.rs"
        text = path.read_text(encoding="utf-8").replace(
            "fn write_user_bytes_raw(",
            "pub(super) fn write_user_bytes_raw(",
            1,
        )
        path.write_text(text, encoding="utf-8")
        self.assert_guard_fails(tree, "must remain a module-private low-level function")

    def test_detects_unsafe_boundary_spread(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/user_memory.rs"
        path.write_text(path.read_text(encoding="utf-8") + "\nfn spread() { unsafe {} }\n", encoding="utf-8")
        self.assert_guard_fails(tree, "expected 5 audited unsafe blocks")

    def test_detects_metadata_handler_drift(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/syscall_metadata.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace('"sys_clone"', '"sys_fork"', 1),
            encoding="utf-8",
        )
        self.assert_guard_fails(tree, "CLONE declaration drifted")

    def test_detects_unaliased_duplicate_number(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/syscall_metadata.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("numbers::__NR_fdatasync", "numbers::__NR_fsync", 1)
        text = text.replace('Some("fsync")', "None", 1)
        path.write_text(text, encoding="utf-8")
        self.assert_guard_fails(tree, "duplicate syscall number 82")

    def test_explicit_same_handler_alias_allows_duplicate_number(self) -> None:
        registrations = [
            ("fsync", 82, "sys_fsync", None),
            ("fdatasync", 82, "sys_fsync", "fsync"),
        ]
        self.assertEqual(GUARD_MODULE.validate_registration_set("test", registrations), [])

    def test_detects_missing_metadata_table_registration(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/syscall_metadata.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace("    FDATASYNC,\n", "", 1),
            encoding="utf-8",
        )
        self.assert_guard_fails(tree, "cfg/table registration token missing")

    def test_detects_loongarch_clone_argument_order_drift(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/syscall_dispatch.rs"
        text = path.read_text(encoding="utf-8")
        old = """            tf.arg2(),
            tf.arg4(),
            tf.arg3(),
        ),
        general::__NR_execve"""
        new = """            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_execve"""
        self.assertIn(old, text)
        path.write_text(text.replace(old, new, 1), encoding="utf-8")
        self.assert_guard_fails(tree, "architecture/handler route changed")

    def test_detects_loongarch_legacy_number_drift(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/syscall_dispatch.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "const LOONGARCH_LEGACY_GETRLIMIT: u32 = 163;",
                "const LOONGARCH_LEGACY_GETRLIMIT: u32 = 165;",
                1,
            ),
            encoding="utf-8",
        )
        self.assert_guard_fails(tree, "architecture/handler route changed")

    def test_detects_abi_number_guard_drift(self) -> None:
        tree = self.make_tree()
        path = tree / "api/orays_linux_abi/src/syscall.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "    assert!(numbers::__NR_clone == 220);\n",
                "",
                1,
            ),
            encoding="utf-8",
        )
        self.assert_guard_fails(tree, "syscall number guard missing")

    def test_detects_abi_layout_guard_drift(self) -> None:
        tree = self.make_tree()
        path = tree / "api/orays_linux_abi/src/time.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "    assert!(align_of::<RtcTime>() == align_of::<i32>());\n",
                "",
                1,
            ),
            encoding="utf-8",
        )
        self.assert_guard_fails(tree, "time layout guard missing")


if __name__ == "__main__":
    unittest.main()
