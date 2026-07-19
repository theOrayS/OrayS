#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/desktop/check-scope.py"


class ScopeRunnerTests(unittest.TestCase):
    def test_root_and_subdirectory_invocations_are_identical(self) -> None:
        def invoke(cwd: Path) -> subprocess.CompletedProcess[str]:
            return subprocess.run(
                [sys.executable, "-B", str(SCRIPT)],
                cwd=cwd,
                check=False,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )

        root = invoke(ROOT)
        subdirectory = invoke(ROOT / "user/desktop")
        self.assertEqual(subdirectory.returncode, root.returncode)
        self.assertEqual(subdirectory.stdout, root.stdout)
        self.assertEqual(subdirectory.stderr, root.stderr)


if __name__ == "__main__":
    unittest.main()
