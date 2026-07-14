#!/usr/bin/env python3
"""Regression tests for NUMA memory-policy syscall semantics."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_memory_policy_semantics.py"
TARGETS = [
    Path("user/shell/src/uspace/memory_policy.rs"),
    Path("user/shell/src/uspace/syscall_dispatch.rs"),
]


class MemoryPolicySemanticsGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="memory-policy-semantics-guard-"))
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

    def test_detects_ignored_memory_policy_mode(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/memory_policy.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "match default_policy_only(process, mode, nodemask, maxnode) {\n"
            "        Ok(()) => 0,\n"
            "        Err(err) => neg_errno(err),\n"
            "    }",
            "let _ = mode;\n    validate_mempolicy_request(process, nodemask, maxnode)",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("mbind", result.stdout)

    def test_detects_missing_mbind_flags_dispatch(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/syscall_dispatch.rs"
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
        path.write_text(text.replace(old, new, 1), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("arg5", result.stdout)


if __name__ == "__main__":
    unittest.main()
