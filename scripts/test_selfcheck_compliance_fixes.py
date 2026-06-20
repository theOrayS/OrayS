#!/usr/bin/env python3
"""Mutation tests for check_selfcheck_compliance_fixes.py."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_selfcheck_compliance_fixes.py"
TARGETS = [
    Path("Cargo.toml"),
    Path("kernel/fs/axfs/src/mounts.rs"),
    Path("examples/shell/src/cmd.rs"),
    Path("api/arceos_posix_api/src/utils.rs"),
    Path("api/arceos_posix_api/src/imp/pthread/mutex.rs"),
    Path("api/arceos_posix_api/src/imp/task.rs"),
    Path("api/arceos_posix_api/src/imp/net.rs"),
    Path("api/arceos_posix_api/src/imp/stdio.rs"),
    Path("api/arceos_posix_api/src/imp/time.rs"),
    Path("api/arceos_posix_api/src/imp/fs.rs"),
    Path("api/arceos_posix_api/src/signal.rs"),
    Path("kernel/fs/axfs/src/root.rs"),
    Path("kernel/fs/axfs/src/fops.rs"),
    Path("vendor/axfs_vfs/src/lib.rs"),
    Path("vendor/axfs_ramfs/src/file.rs"),
    Path("vendor/axfs_ramfs/src/dir.rs"),
    Path("ulib/axlibc/src/fs.rs"),
    Path("ulib/axlibc/src/fd_ops.rs"),
    Path("ulib/axlibc/c/stdio.c"),
    Path("ulib/axlibc/c/socket.c"),
    Path("ulib/axlibc/c/stat.c"),
]


class SelfCheckComplianceGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="selfcheck-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(encoding="utf-8"), encoding="utf-8")
        return tmp

    def run_guard(self, tree: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run([sys.executable, str(GUARD), "--root", str(tree)], text=True, capture_output=True)

    def mutate(self, tree: Path, rel: str, old: str, new: str) -> None:
        path = tree / rel
        text = path.read_text(encoding="utf-8")
        self.assertIn(old, text, f"fixture drifted: missing {old!r}")
        path.write_text(text.replace(old, new, 1), encoding="utf-8")

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_detects_ltp_core_fallback(self) -> None:
        tree = self.make_tree()
        self.mutate(
            tree,
            "examples/shell/src/cmd.rs",
            'Err(format!(\n        "invalid LTP_CASES selection \'{spec}\': no valid cases parsed"\n    ))',
            'Ok((String::from("core"), ltp_cases_from_slice(LTP_CORE_CASES)?))',
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("LTP_CASES", result.stdout)

    def test_detects_getpid_task_id(self) -> None:
        tree = self.make_tree()
        self.mutate(tree, "api/arceos_posix_api/src/imp/task.rs", "Ok(1)", "Ok(axtask::current().id().as_u64() as c_int)")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("getpid", result.stdout)

    def test_detects_root_rename_predelete(self) -> None:
        tree = self.make_tree()
        self.mutate(tree, "kernel/fs/axfs/src/root.rs", "parent_node_of(None, old).rename(old, new)", "remove_file(None, new)?;\n    parent_node_of(None, old).rename(old, new)")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("pre-delete", result.stdout)


    def test_detects_official_unknown_group_skip(self) -> None:
        tree = self.make_tree()
        self.mutate(
            tree,
            "examples/shell/src/cmd.rs",
            "if !missing_groups.is_empty() || !disabled_groups.is_empty() {",
            "if false && (!missing_groups.is_empty() || !disabled_groups.is_empty()) {",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("official groups", result.stdout)

    def test_detects_ocreat_mode_reject_regression(self) -> None:
        tree = self.make_tree()
        self.mutate(
            tree,
            "kernel/fs/axfs/src/fops.rs",
            "if !created_new && !perm_to_cap(attr.perm()).contains(access_cap) {",
            "if !perm_to_cap(attr.perm()).contains(access_cap) {",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("O_CREAT", result.stdout)

    def test_detects_accept4_ignored_fcntl(self) -> None:
        tree = self.make_tree()
        self.mutate(tree, "ulib/axlibc/c/socket.c", "int current_flags = fcntl(ret, F_GETFL);", "int current_flags = 0;")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("accept4", result.stdout)


if __name__ == "__main__":
    unittest.main()
