#!/usr/bin/env python3
"""Regression tests for POSIX timer and interval-timer semantics."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_timer_semantics.py"
TARGET = Path("user/shell/src/uspace/time_abi.rs")


class TimerSemanticsGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="timer-semantics-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        dst = tmp / TARGET
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_text((ROOT / TARGET).read_text(encoding="utf-8"), encoding="utf-8")
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
        path = tree / TARGET
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "value if value == general::SIGEV_THREAD as i32 => Err(LinuxError::EINVAL),",
            "value if value == general::SIGEV_THREAD as i32 => {\n"
            "            // Accept it as a non-delivering kernel timer.\n"
            "            Ok(PosixTimerNotify::None)\n"
            "        },",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SIGEV_THREAD", result.stdout)

    def test_detects_itimer_virtual_prof_rejection(self) -> None:
        tree = self.make_tree()
        path = tree / TARGET
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


if __name__ == "__main__":
    unittest.main()
