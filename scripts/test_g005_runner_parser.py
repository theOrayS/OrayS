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

    def test_detects_literal_command_success_override(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "Ok(0) => {\n                println!(\"testcase busybox {line} success\");",
            "Ok(status) if status == 0 || line == \"false\" => {\n                println!(\"testcase busybox {line} success\");",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("literal command lines", result.stdout)

    def test_detects_suite_specific_script_rewrite_function(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text += '\nfn rewrite_iperf_daemon_server(script: &str) -> String { script.into() }\n'
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("rewrite_iperf_daemon_server", result.stdout)

    def test_detects_ltp_file_pattern_rewrite(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            ".map(|line| rewrite_script_line(line, busybox_path, rewrite_busybox_path))",
            ".map(|line| if line.trim_start() == \"\"$file\"\" { rewrite_script_line(line, busybox_path, rewrite_busybox_path) } else { rewrite_script_line(line, busybox_path, rewrite_busybox_path) })",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("$file", result.stdout)

    def test_detects_exact_test_script_name_branch(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "if raw_script.ends_with('\\n') {",
            "if src.ends_with(\"iperf_testcode.sh\") { script.push_str(\"# rewrite\"); }\n    if raw_script.ends_with('\\n') {",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("iperf_testcode.sh", result.stdout)

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
