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


def make_repo(root: Path) -> None:
    subprocess.run(["git", "init", "-q", str(root)], check=True)
    subprocess.run(
        ["git", "config", "user.email", "desktop-test@example.invalid"],
        cwd=root,
        check=True,
    )
    subprocess.run(["git", "config", "user.name", "Desktop Test"], cwd=root, check=True)
    (root / "tracked").write_text("base\n", encoding="utf-8")
    subprocess.run(["git", "add", "tracked"], cwd=root, check=True)
    subprocess.run(["git", "commit", "-qm", "base"], cwd=root, check=True)


class ScopeRunnerTests(unittest.TestCase):
    def test_changed_paths_preserve_unicode_names_without_git_quoting(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            make_repo(repo)
            name = "人工审查报告.md"
            (repo / name).write_text("review\n", encoding="utf-8")

            self.assertIn(name, MODULE.changed_paths(repo, "HEAD"))

    def test_changed_paths_return_adversarial_names_exactly(self) -> None:
        adversarial = [
            "user/shell/evil\n.rs",
            "user/shell/evil\rcarriage.rs",
            "user/desktop/tab\tname.rs",
            "user/desktop/ trailing.rs ",
            " user/desktop/leading.rs",
            'user/desktop/quote"double.rs',
            "user/desktop/quote'single.rs",
            "user/desktop/backslash\\.rs",
        ]
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            make_repo(repo)
            for name in adversarial:
                path = repo / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("x\n", encoding="utf-8")

            changed = MODULE.changed_paths(repo, "HEAD")
            for name in adversarial:
                self.assertIn(name, changed)

    def test_classification_never_strips_or_rewrites_paths(self) -> None:
        allowed = ["user/desktop/**", "docs/allowed.md"]
        bridges = ["kernel/drivers/axdriver/src/input.rs"]
        rejected = [
            " user/desktop/src/lib.rs",
            "docs/allowed.md ",
            "docs/allowed.md\n",
            "docs/allowed.md\r",
            "kernel/drivers/axdriver/src/input.rs ",
        ]
        for path in rejected:
            with self.subTest(path=path):
                failures, bridge_changed = MODULE.classify([path], allowed, bridges)
                self.assertTrue(failures, f"path bypassed the allowlist: {path!r}")
                self.assertEqual(bridge_changed, [])
        # Byte-exact unusual names inside an allowed /** tree are still in
        # that tree; only mutation (strip/unquote) could misplace them.
        for path in ("user/desktop/src/lib.rs ", "user/desktop/src/lib.rs\t"):
            with self.subTest(path=path):
                failures, _ = MODULE.classify([path], allowed, bridges)
                self.assertEqual(failures, [])

    def test_classification_matches_exact_and_prefix_rules(self) -> None:
        allowed = ["user/desktop/**", "docs/allowed.md"]
        bridges = ["kernel/drivers/axdriver/src/input.rs"]
        failures, bridge_changed = MODULE.classify(
            [
                "user/desktop/src/lib.rs",
                "docs/allowed.md",
                "kernel/drivers/axdriver/src/input.rs",
                "user/shell/main.rs",
                "vendor/x.rs",
                "unrelated/file.md",
            ],
            allowed,
            bridges,
        )
        self.assertEqual(bridge_changed, ["kernel/drivers/axdriver/src/input.rs"])
        self.assertTrue(any("user/shell/main.rs" in item for item in failures))
        self.assertTrue(any("vendor/x.rs" in item for item in failures))
        self.assertTrue(any("unrelated/file.md" in item for item in failures))
        self.assertEqual(len(failures), 3)

    def test_newline_in_allowed_tree_name_is_still_inside_that_tree(self) -> None:
        allowed = ["user/desktop/**"]
        failures, _ = MODULE.classify(["user/desktop/a\nb.rs"], allowed, [])
        self.assertEqual(failures, [])

    def test_missing_base_argument_and_state_file_is_a_hard_failure(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            make_repo(repo)
            with self.assertRaises(RuntimeError):
                MODULE.resolve_base(repo, None)

    def test_invalid_base_is_a_hard_failure(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            make_repo(repo)
            for candidate in ("0" * 40, "does-not-exist", "-n", "..", ""):
                with self.subTest(candidate=candidate):
                    with self.assertRaises(RuntimeError):
                        MODULE.resolve_base(repo, candidate)

    def test_resolve_base_expands_to_auditable_commit_identity(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            make_repo(repo)
            head = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=repo,
                check=True,
                text=True,
                stdout=subprocess.PIPE,
            ).stdout.strip()
            resolved, source = MODULE.resolve_base(repo, head[:12])
            self.assertEqual(resolved, head)
            self.assertEqual(source, "cli")
            state = repo / ".codex/state/desktop-base-sha"
            state.parent.mkdir(parents=True)
            state.write_text(head + "\n", encoding="utf-8")
            resolved, source = MODULE.resolve_base(repo, None)
            self.assertEqual(resolved, head)
            self.assertEqual(source, "state-file")

    def test_script_fails_closed_without_any_authorized_base(self) -> None:
        state = ROOT / ".codex/state/desktop-base-sha"
        if state.exists():
            self.skipTest("local state file overrides the CLI-less invocation")
        result = subprocess.run(
            [sys.executable, "-B", str(SCRIPT)],
            cwd=ROOT,
            check=False,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertNotIn("DESKTOP_SCOPE=PASS", result.stdout)

    def test_bridge_churn_counts_paths_with_unusual_names(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            make_repo(repo)
            name = "sub/weird\nname.rs"
            path = repo / name
            path.parent.mkdir(parents=True)
            path.write_text("a\nb\n", encoding="utf-8")
            subprocess.run(["git", "add", "-A"], cwd=repo, check=True)
            subprocess.run(["git", "commit", "-qm", "add"], cwd=repo, check=True)
            path.write_text("a\nb\nc\nd\n", encoding="utf-8")

            self.assertEqual(MODULE.bridge_churn(repo, "HEAD", [name]), 2)

    def test_root_and_subdirectory_invocations_are_identical(self) -> None:
        def invoke(cwd: Path) -> subprocess.CompletedProcess[str]:
            return subprocess.run(
                [sys.executable, "-B", str(SCRIPT), "--base", "HEAD"],
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
