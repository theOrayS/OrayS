#!/usr/bin/env python3
"""Regression tests for the evaluation-runner-and-parser static guard."""

from __future__ import annotations

import json
import os
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
    def fake_official_environment(
        self,
        *,
        make_status: int = 0,
    ) -> tuple[Path, dict[str, str], Path, Path]:
        directory = Path(tempfile.mkdtemp(prefix="official-wrapper-fixture-"))
        self.addCleanup(lambda: shutil.rmtree(directory, ignore_errors=True))
        bin_dir = directory / "bin"
        bin_dir.mkdir()
        args_log = directory / "make-args.log"
        environment_log = directory / "make-environment.log"
        bash_environment = directory / "bash-environment.sh"
        bash_environment.write_text(
            "make() { printf 'BASH_ENV make function was not removed\\n'; return 0; }\n",
            encoding="utf-8",
        )
        make_script = bin_dir / "make"
        make_script.write_text(
            "#!/bin/sh\n"
            "printf '%s\\n' \"$@\" > \"$FAKE_MAKE_ARGS\"\n"
            "env | LC_ALL=C sort > \"$FAKE_MAKE_ENVIRONMENT\"\n"
            "exit \"$FAKE_MAKE_STATUS\"\n",
            encoding="utf-8",
        )
        make_script.chmod(0o755)
        for command in ("cargo", "qemu-img", "qemu-system-riscv64"):
            path = bin_dir / command
            path.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
            path.chmod(0o755)
        image = directory / "images/sdcard-rv.img"
        image.parent.mkdir()
        image.write_bytes(b"fixture")
        environment = os.environ.copy()
        environment.update(
            {
                "PATH": f"{bin_dir}:{environment['PATH']}",
                "RV_TESTSUITE_IMG": "images/sdcard-rv.img",
                "ORAYS_TEST_OUTPUT_DIR": "out",
                "FAKE_MAKE_ARGS": str(args_log),
                "FAKE_MAKE_ENVIRONMENT": str(environment_log),
                "FAKE_MAKE_STATUS": str(make_status),
                "CARGO_NET_OFFLINE": "false",
                "MAKEFLAGS": "-n",
                "BASH_ENV": str(bash_environment),
                "ENV": str(bash_environment),
                "BASH_FUNC_make%%": "() { printf 'exported make function was not removed\\n'; return 0; }",
                "KERNEL_APP": "untrusted/app",
                "KERNEL_RV_FEATURES": "untrusted-features",
                "KERNEL_RV_APP_FEATURES": "untrusted-app-features",
                "KERNEL_MODE": "debug",
                "PLAT_CONFIG": "/untrusted/platform.toml",
                "KERNEL_RV": "/untrusted/kernel-rv",
                "KERNEL_SMP": "9",
                "RV_MEM": "9G",
            }
        )
        return directory, environment, args_log, environment_log

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

    def test_official_executor_absolutizes_paths_and_fixes_consumed_resources(self) -> None:
        directory, environment, args_log, environment_log = self.fake_official_environment()
        result = subprocess.run(
            [str(ROOT / "test/evaluation/run_official_evaluation.sh"), "rv"],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        arguments = args_log.read_text(encoding="utf-8").splitlines()
        self.assertIn(f"RV_TESTSUITE_IMG={directory}/images/sdcard-rv.img", arguments)
        self.assertTrue(
            any(value.startswith(f"RV_TESTSUITE_RUN_IMG={directory}/out/") for value in arguments),
            arguments,
        )
        self.assertIn("KERNEL_SMP=1", arguments)
        self.assertIn("RV_MEM=1G", arguments)
        self.assertFalse(any(value.startswith(("SMP=", "MEM=")) for value in arguments), arguments)
        environment_text = environment_log.read_text(encoding="utf-8")
        self.assertIn("CARGO_NET_OFFLINE=true", environment_text)
        for variable in (
            "MAKEFLAGS",
            "BASH_ENV",
            "ENV",
            "BASH_FUNC_make%%",
            "KERNEL_APP",
            "KERNEL_RV_FEATURES",
            "KERNEL_RV_APP_FEATURES",
            "KERNEL_MODE",
            "PLAT_CONFIG",
            "KERNEL_RV",
        ):
            self.assertNotIn(f"{variable}=", environment_text)

    def test_official_executor_reserves_125_for_preflight_infrastructure(self) -> None:
        directory, environment, _args_log, _environment_log = self.fake_official_environment()
        (directory / "images/sdcard-rv.img").unlink()
        result = subprocess.run(
            [str(ROOT / "test/evaluation/run_official_evaluation.sh"), "rv"],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(result.returncode, 125, result.stdout + result.stderr)
        self.assertIn("infrastructure error", result.stderr)

    def test_official_executor_preserves_make_exit_two_as_test_failure(self) -> None:
        directory, environment, _args_log, _environment_log = self.fake_official_environment(
            make_status=2
        )
        result = subprocess.run(
            [str(ROOT / "test/evaluation/run_official_evaluation.sh"), "rv"],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)

    def test_public_official_entry_cannot_pass_explicit_guest_failure(self) -> None:
        directory, environment, _args_log, _environment_log = self.fake_official_environment()
        fake_make = directory / "bin/make"
        fake_make.write_text(
            "#!/bin/sh\n"
            "printf '%s\\n' "
            "'#### OS COMP TEST GROUP START demo-musl ####' "
            "'FAIL OFFICIAL TEST GROUP demo-musl : 7' "
            "'#### OS COMP TEST GROUP END demo-musl ####'\n"
            "exit 0\n",
            encoding="utf-8",
        )
        fake_make.chmod(0o755)
        output_dir = directory / "suite-output"
        result = subprocess.run(
            [
                str(ROOT / "run-eval.sh"),
                "rv",
                "--output-dir",
                str(output_dir),
            ],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        summary = json.loads((output_dir / "summary.json").read_text(encoding="utf-8"))
        self.assertEqual(summary["result"], "INFRA_ERROR", summary)
        self.assertEqual(summary["cases"][0]["status"], "INFRA_ERROR", summary)
        self.assertNotEqual(summary["cases"][0]["status"], "PASS", summary)

    def test_public_official_entry_ignores_startup_hooks_and_preserves_environment(self) -> None:
        directory = Path(tempfile.mkdtemp(prefix="official-public-wrapper-fixture-"))
        self.addCleanup(lambda: shutil.rmtree(directory, ignore_errors=True))
        bin_dir = directory / "bin"
        bin_dir.mkdir()
        args_log = directory / "python-args.log"
        environment_log = directory / "python-environment.log"
        hook_marker = directory / "bash-env-ran"
        bash_environment = directory / "bash-environment.sh"
        bash_environment.write_text(
            'printf "hook ran\\n" > "$WRAPPER_HOOK_MARKER"\nexit 0\n',
            encoding="utf-8",
        )
        fake_python = bin_dir / "python3"
        fake_python.write_text(
            "#!/bin/sh\n"
            'printf "%s\\n" "$@" > "$WRAPPER_ARGS_LOG"\n'
            "printf 'BASH_ENV=%s\\nENV=%s\\nPYTHONPATH=%s\\n' "
            '"${BASH_ENV-}" "${ENV-}" "${PYTHONPATH-}" '
            '> "$WRAPPER_ENVIRONMENT_LOG"\n'
            "exit 37\n",
            encoding="utf-8",
        )
        fake_python.chmod(0o755)
        environment = os.environ.copy()
        environment.update(
            {
                "PATH": f"{bin_dir}:{environment.get('PATH', '')}",
                "BASH_ENV": str(bash_environment),
                "ENV": "preserved-env-value",
                "PYTHONPATH": "preserved-python-path",
                "WRAPPER_ARGS_LOG": str(args_log),
                "WRAPPER_ENVIRONMENT_LOG": str(environment_log),
                "WRAPPER_HOOK_MARKER": str(hook_marker),
            }
        )
        result = subprocess.run(
            [str(ROOT / "run-eval.sh"), "la", "--output-dir", str(directory / "output")],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(result.returncode, 37, result.stdout + result.stderr)
        self.assertFalse(hook_marker.exists(), "root wrapper executed inherited BASH_ENV")
        self.assertEqual(
            args_log.read_text(encoding="utf-8").splitlines(),
            [
                "-B",
                "-E",
                "-s",
                str(ROOT / "test/run_suite.py"),
                "--profile",
                "official",
                "--arch",
                "la",
                "--output-dir",
                str(directory / "output"),
            ],
        )
        self.assertEqual(
            environment_log.read_text(encoding="utf-8").splitlines(),
            [
                f"BASH_ENV={bash_environment}",
                "ENV=preserved-env-value",
                "PYTHONPATH=preserved-python-path",
            ],
        )

        python_hook_directory = directory / "python-hook"
        python_hook_directory.mkdir()
        python_hook_marker = directory / "python-hook-ran"
        (python_hook_directory / "sitecustomize.py").write_text(
            "import os\n"
            "from pathlib import Path\n"
            "Path(os.environ['PYTHON_HOOK_MARKER']).write_text('hook ran\\n')\n"
            "os._exit(0)\n",
            encoding="utf-8",
        )
        real_environment = os.environ.copy()
        real_environment.update(
            {
                "BASH_ENV": str(bash_environment),
                "ENV": "preserved-env-value",
                "PYTHONPATH": str(python_hook_directory),
                "PYTHON_HOOK_MARKER": str(python_hook_marker),
                "WRAPPER_HOOK_MARKER": str(hook_marker),
            }
        )
        real_result = subprocess.run(
            [str(ROOT / "run-eval.sh"), "invalid-architecture"],
            cwd=directory,
            env=real_environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(real_result.returncode, 2, real_result.stdout + real_result.stderr)
        self.assertFalse(hook_marker.exists(), "root wrapper executed inherited BASH_ENV")
        self.assertFalse(python_hook_marker.exists(), "root wrapper imported PYTHONPATH sitecustomize")

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
