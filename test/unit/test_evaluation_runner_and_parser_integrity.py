#!/usr/bin/env python3
"""Regression tests for the evaluation-runner-and-parser static guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_evaluation_runner_and_parser_integrity.py"
TARGETS = [
    Path("user/shell/src/cmd.rs"),
    Path("user/shell/src/uspace/runtime_paths.rs"),
    Path("user/shell/src/uspace/process_lifecycle.rs"),
    Path("user/shell/src/uspace/fd_table.rs"),
    Path("user/shell/src/uspace/program_loader.rs"),
    Path("Makefile"),
    Path("test/evaluation/summarize_ltp_results.py"),
    Path("test/unit/test_ltp_result_summary.py"),
]


class EvaluationRunnerAndParserIntegrityGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="evaluation-runner-and-parser-guard-"))
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

    def replace_once(self, text: str, old: str, new: str) -> str:
        self.assertIn(old, text)
        return text.replace(old, new, 1)

    def replace_nth(self, text: str, old: str, new: str, occurrence: int) -> str:
        self.assertGreaterEqual(occurrence, 1)
        start = 0
        for _ in range(occurrence):
            pos = text.find(old, start)
            self.assertNotEqual(pos, -1)
            start = pos + len(old)
        return text[:pos] + new + text[pos + len(old) :]

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_chdir01_case_specialization(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = self.replace_once(
            text,
            "let needs_case_resource_helper = ltp_case_has_resource_helper(&resource_helper_cases, case);",
            'let needs_case_resource_helper = if case == "chdir01" { true } else { ltp_case_has_resource_helper(&resource_helper_cases, case) };',
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("chdir01", result.stdout)

    def test_detects_first_underscore_resource_helper_parse(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = self.replace_once(
            text,
            "helper_name.strip_prefix(case)",
            "helper_name.split_once('_').map(|(prefix, _)| prefix)",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("first underscore", result.stdout)

    def test_detects_literal_command_success_override(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = self.replace_once(
            text,
            "Ok(status) if expected_status.is_met_by(status) => {",
            'Ok(status) if expected_status.is_met_by(status) || line == "false" => {',
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("literal command lines", result.stdout)

    def test_detects_score_aware_libctest_skip(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            'if DISABLED_OFFICIAL_TEST_GROUPS.contains(&group) {\n'
            '            println!("autorun: skip disabled test group {suite_dir}/{script}");\n'
            "            continue;\n"
            "        }\n",
            'if DISABLED_OFFICIAL_TEST_GROUPS.contains(&group) {\n'
            '            println!("autorun: skip disabled test group {suite_dir}/{script}");\n'
            "            continue;\n"
            "        }\n"
            '        if group == "libctest" && suite_dir != "/musl" {\n'
            '            println!("autorun: skip unscored test group {suite_dir}/{script}: official libctest score is musl-only");\n'
            "            continue;\n"
            "        }\n",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("libctest", result.stdout)

    def test_detects_structural_libctest_suite_dir_skip(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            'if DISABLED_OFFICIAL_TEST_GROUPS.contains(&group) {\n'
            '            println!("autorun: skip disabled test group {suite_dir}/{script}");\n'
            "            continue;\n"
            "        }\n",
            'if DISABLED_OFFICIAL_TEST_GROUPS.contains(&group) {\n'
            '            println!("autorun: skip disabled test group {suite_dir}/{script}");\n'
            "            continue;\n"
            "        }\n"
            '        if group == "libctest" {\n'
            '            if suite_dir.as_str() != "/musl" {\n'
            "                continue;\n"
            "            }\n"
            "        }\n",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("conditionally continue", result.stdout)


    def test_detects_unknown_official_group_silent_skip(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8").replace(
            "if !missing_groups.is_empty() || !disabled_groups.is_empty() {",
            "if false && (!missing_groups.is_empty() || !disabled_groups.is_empty()) {",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unknown or disabled selected official groups", result.stdout)

    def test_detects_suite_specific_script_rewrite_function(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8")
        text += '\nfn rewrite_iperf_daemon_server(script: &str) -> String { script.into() }\n'
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("rewrite_iperf_daemon_server", result.stdout)

    def test_detects_ltp_file_pattern_rewrite(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
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
        path = tree / "user/shell/src/cmd.rs"
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

    def test_detects_pass_ltp_case_wrapper_record(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8").replace(
            "FAIL LTP CASE {case} : 0",
            "PASS LTP CASE {case} : 0",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("PASS LTP CASE", result.stdout)

    def test_detects_busybox_execve_magic_fallback(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/process_lifecycle.rs"
        text = path.read_text(encoding="utf-8") + "\nfn resolve_execve_compat_path() {}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("resolve_execve_compat_path", result.stdout)

    def test_detects_busybox_open_alias_magic(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8") + "\nfn append_busybox_applet_alias_candidates() {}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("append_busybox_applet_alias_candidates", result.stdout)

    def test_detects_missing_runtime_busybox_wrapper_preparation(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = self.replace_once(
            path.read_text(encoding="utf-8"),
            "prepare_suite_runtime_busybox_wrappers(suite_dir)",
            "missing_suite_runtime_busybox_wrappers(suite_dir)",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("busybox wrapper preparation", result.stdout)

    def test_detects_runner_layer_missing_runtime_busybox_wrapper_preparation(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = self.replace_once(
            path.read_text(encoding="utf-8"),
            "prepare_suite_runtime_busybox_wrappers(suite_dir)",
            "missing_suite_runtime_busybox_wrappers(suite_dir)",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("run_busybox_suite", result.stdout)

    def test_detects_ltp_runner_missing_runtime_busybox_wrapper_preparation(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = self.replace_nth(
            path.read_text(encoding="utf-8"),
            "prepare_suite_runtime_busybox_wrappers(suite_dir)",
            "missing_suite_runtime_busybox_wrappers(suite_dir)",
            2,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("run_ltp_suite", result.stdout)

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
        path = tree / "test/evaluation/summarize_ltp_results.py"
        path.write_text(path.read_text(encoding="utf-8").replace("promotion_mode_blocker", "promotion_mode_missing"), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("promotion mode blocker", result.stdout)


if __name__ == "__main__":
    unittest.main()
