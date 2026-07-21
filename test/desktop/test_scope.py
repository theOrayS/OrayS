#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import os
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
    def make_repo(self, directory: str) -> Path:
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
        return repo

    def test_changed_paths_preserve_unicode_names_without_git_quoting(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = self.make_repo(directory)
            name = "人工审查报告.md"
            (repo / name).write_text("review\n", encoding="utf-8")

            self.assertIn(name, MODULE.changed_paths(repo, "HEAD"))

    def test_changed_paths_preserve_delimiters_and_non_utf8_bytes_exactly(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = self.make_repo(directory)
            (repo / "scripts/desktop").mkdir(parents=True)
            names = {
                " scripts/desktop/evil.py",
                "scripts/desktop/trailing.py ",
                "scripts/desktop/tab\tname.py",
                "scripts/desktop/line\nbreak.py",
                os.fsdecode(b"scripts/desktop/non-utf8-\xff.py"),
            }
            for name in names:
                encoded_path = os.path.join(os.fsencode(repo), os.fsencode(name))
                os.makedirs(os.path.dirname(encoded_path), exist_ok=True)
                descriptor = os.open(
                    encoded_path,
                    os.O_WRONLY | os.O_CREAT | os.O_EXCL,
                    0o600,
                )
                os.close(descriptor)

            observed = MODULE.changed_paths(repo, "HEAD")
            self.assertTrue(names <= observed, (names, observed))
            self.assertNotIn("scripts/desktop/evil.py", observed)
            self.assertFalse(
                MODULE.any_match(" scripts/desktop/evil.py", ["scripts/desktop/**"])
            )

    def test_default_base_fails_closed_without_trusted_remote(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = self.make_repo(directory)
            with self.assertRaisesRegex(RuntimeError, "unable to resolve desktop base"):
                MODULE.default_base(repo)

    def test_ignored_mutable_state_cannot_select_scope_base(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = self.make_repo(directory)
            state = repo / ".codex/state/desktop-base-sha"
            state.parent.mkdir(parents=True)
            state.write_text("HEAD\n", encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "unable to resolve desktop base"):
                MODULE.default_base(repo)

    def test_default_base_uses_verified_remote_development_base(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = self.make_repo(directory)
            commit = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=repo,
                check=True,
                text=True,
                stdout=subprocess.PIPE,
            ).stdout.strip()
            subprocess.run(
                [
                    "git",
                    "update-ref",
                    "refs/remotes/origin/develop/post-integration-next",
                    commit,
                ],
                cwd=repo,
                check=True,
            )
            self.assertEqual(MODULE.default_base(repo), commit)

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
