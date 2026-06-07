#!/usr/bin/env python3
"""Regression tests for the G005 static guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_g005_runner_parser.py"
TARGETS = [
    Path("examples/shell/src/cmd.rs"),
    Path("Makefile"),
    Path("scripts/ltp_summary.py"),
    Path("scripts/test_ltp_summary.py"),
]


class G005RunnerParserGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="g005-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(), encoding="utf-8")
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

    def test_detects_chdir01_case_specialization(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "if needs_case_resource_helper {",
            "if case == \"chdir01\" {",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("chdir01", result.stdout)

    def test_detects_blacklist_default(self) -> None:
        tree = self.make_tree()
        path = tree / "Makefile"
        text = path.read_text(encoding="utf-8").replace(
            "REMOTE_LTP_CASES ?= stable",
            "REMOTE_LTP_CASES ?= stable-plus-blacklist",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("stable-plus-blacklist", result.stdout)

    def test_detects_missing_promotion_mode_blocker(self) -> None:
        tree = self.make_tree()
        path = tree / "scripts/ltp_summary.py"
        path.write_text(path.read_text(encoding="utf-8").replace("promotion_mode_blocker", "promotion_mode_missing"), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("promotion mode blocker", result.stdout)


if __name__ == "__main__":
    unittest.main()
