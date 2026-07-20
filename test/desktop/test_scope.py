#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/desktop/check-scope.py"
SPEC = importlib.util.spec_from_file_location("desktop_scope", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ScopeRunnerTests(unittest.TestCase):
    def test_changed_paths_preserve_unicode_names_without_git_quoting(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            subprocess.run(["git", "init", "-q", str(repo)], check=True)
            subprocess.run(
                ["git", "config", "user.email", "desktop-test@example.invalid"],
                cwd=repo,
                check=True,
            )
            subprocess.run(
                ["git", "config", "user.name", "Desktop Test"], cwd=repo, check=True
            )
            (repo / "tracked").write_text("base\n", encoding="utf-8")
            subprocess.run(["git", "add", "tracked"], cwd=repo, check=True)
            subprocess.run(["git", "commit", "-qm", "base"], cwd=repo, check=True)
            name = "人工审查报告.md"
            (repo / name).write_text("review\n", encoding="utf-8")

            self.assertIn(name, MODULE.changed_paths(repo, "HEAD"))

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
