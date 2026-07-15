#!/usr/bin/env python3
"""Regression tests for the synthetic-capability synthetic capability guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_synthetic_capability_integrity.py"
TARGETS = [
    Path("user/shell/src/uspace/synthetic_fs.rs"),
    Path("user/shell/src/uspace/fd_table.rs"),
    Path("user/shell/src/uspace/metadata.rs"),
    Path("user/shell/src/uspace/linux_abi.rs"),
]


class SyntheticCapabilityIntegrityGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="synthetic-capability-guard-"))
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

    def test_detects_ltp_cmdline_marker(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/synthetic_fs.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            'b"root=/dev/vda rw console=ttyS0\\n"',
            'b"root=/dev/vda rw console=ttyS0 ltp.oskernel2026=1\\n"',
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("LTP", result.stdout)

    def test_detects_linux_abi_ltp_marker(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/linux_abi.rs"
        path.write_text(
            path.read_text(encoding="utf-8") + "\n// LTP-aware sizing marker must not be reintroduced.\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("linux_abi.rs", result.stdout)

    def test_detects_extra_block_device_alias(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            'const SYNTHETIC_BLOCK_DEVICE_NAMES: &[&str] = &["vda"];',
            'const SYNTHETIC_BLOCK_DEVICE_NAMES: &[&str] = &["vda", "sda"];',
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("synthetic block devices", result.stdout)

    def test_detects_unbacked_config_comment(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/synthetic_fs.rs"
        text = path.read_text(encoding="utf-8").replace(
            "implemented Linux ABI surfaces",
            "LTP feature probes",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("synthetic proc/config", result.stdout)


if __name__ == "__main__":
    unittest.main()
