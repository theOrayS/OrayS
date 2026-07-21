#!/usr/bin/env python3
"""Regression tests for the evaluation-runner-and-parser static guard."""

from __future__ import annotations

import json
import os
import shlex
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
    Path("test/evaluation/parse_official_results.py"),
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
        makefiles_marker = directory / "makefiles-injection-loaded"
        makefiles_injection = directory / "untrusted-injected.mk"
        makefiles_injection.write_text(
            f"injected := $(shell touch {makefiles_marker})\n",
            encoding="utf-8",
        )
        make_script = bin_dir / "make"
        make_script.write_text(
            "#!/bin/sh\n"
            f"printf '%s\\n' \"$@\" > {shlex.quote(str(args_log))}\n"
            "{\n"
            "  printf '%s\\n' LTP_BLACKLIST_BEGIN\n"
            "  printf '%s\\n' \"${LTP_BLACKLIST-}\"\n"
            "  printf '%s\\n' LTP_BLACKLIST_END\n"
            "  env | LC_ALL=C sort\n"
            f"}} > {shlex.quote(str(environment_log))}\n"
            f"exit {make_status}\n",
            encoding="utf-8",
        )
        make_script.chmod(0o755)
        for command in (
            "cargo",
            "qemu-img",
            "qemu-system-riscv64",
            "qemu-system-loongarch64",
        ):
            path = bin_dir / command
            path.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
            path.chmod(0o755)
        image_directory = directory / "images"
        image_directory.mkdir()
        for image_name in ("sdcard-rv.img", "sdcard-la.img"):
            (image_directory / image_name).write_bytes(b"fixture")
        environment = os.environ.copy()
        for name in (
            "LTP_BLACKLIST",
            "LTP_BLACKLIST_FILE",
            "LTP_BLACKLIST_COMMON_FILE",
            "LTP_BLACKLIST_RV_FILE",
            "LTP_BLACKLIST_LA_FILE",
            "LTP_BLACKLIST_RV",
            "LTP_BLACKLIST_RISCV64",
            "LTP_BLACKLIST_LA",
            "LTP_BLACKLIST_LOONGARCH64",
            "OSCOMP_SKIP_TEST_GROUPS",
        ):
            environment.pop(name, None)
        environment.update(
            {
                "PATH": f"{bin_dir}:{environment['PATH']}",
                "RV_TESTSUITE_IMG": "images/sdcard-rv.img",
                "LA_TESTSUITE_IMG": "images/sdcard-la.img",
                "ORAYS_TEST_OUTPUT_DIR": "out",
                "OSCOMP_TEST_GROUPS": "all",
                "OSCOMP_SKIP_TEST_GROUPS": "none",
                "OSCOMP_GROUP_TIMEOUT_CEILING_SECS": "900",
                "LTP_CASES": "stable-full",
                "LTP_CASE_TIMEOUT_SECS": "180",
                ".SHELLFLAGS": "-n -c",
                "AXCONFIG_GEN": "/bin/false",
                "CARGO_NET_OFFLINE": "false",
                "PYTHONNOUSERSITE": "",
                "PYTHONDONTWRITEBYTECODE": "",
                "PYTHONPYCACHEPREFIX": "/tmp/untrusted-python-cache",
                "MAKE": "/bin/false",
                "MAKEFILES": str(makefiles_injection),
                "MAKEFLAGS": "-n",
                "MAKEOVERRIDES": "MAKE=/bin/false",
                "BASH_ENV": str(bash_environment),
                "ENV": str(bash_environment),
                "BASH_FUNC_make%%": "() { printf 'exported make function was not removed\\n'; return 0; }",
                "KERNEL_APP": "untrusted/app",
                "KERNEL_RV_FEATURES": "untrusted-features",
                "KERNEL_RV_APP_FEATURES": "untrusted-app-features",
                "KERNEL_LA_FEATURES": "untrusted-la-features",
                "KERNEL_LA_APP_FEATURES": "untrusted-la-app-features",
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

    def test_detects_missing_system_information_path_applets(self) -> None:
        for applet in ("df", "nproc"):
            with self.subTest(applet=applet):
                tree = self.make_tree()
                path = tree / "user/shell/src/cmd.rs"
                text = path.read_text(encoding="utf-8")
                token = f'"{applet}", '
                self.assertIn(token, text)
                path.write_text(text.replace(token, "", 1), encoding="utf-8")
                result = self.run_guard(tree)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(f"BusyBox {applet} applet", result.stdout)

    def test_official_executor_absolutizes_paths_and_fixes_consumed_resources(self) -> None:
        directory, environment, args_log, environment_log = self.fake_official_environment()
        blacklist_directory = directory / "blacklists"
        blacklist_directory.mkdir()
        contents = {
            "generic-one.txt": "generic-one",
            "generic-two.txt": "generic-two",
            "common.txt": "common",
            "rv.txt": "rv-file",
            "la.txt": "la-file",
        }
        for name, content in contents.items():
            (blacklist_directory / name).write_text(f"{content}\n", encoding="utf-8")
        environment.update(
            {
                "LTP_BLACKLIST": "inline-base",
                "LTP_BLACKLIST_FILE": "blacklists/generic-one.txt blacklists/generic-two.txt",
                "LTP_BLACKLIST_COMMON_FILE": "blacklists/common.txt",
                "LTP_BLACKLIST_RV_FILE": "blacklists/rv.txt",
                "LTP_BLACKLIST_LA_FILE": "blacklists/la.txt",
                "LTP_BLACKLIST_RV": "rv-inline",
                "LTP_BLACKLIST_RISCV64": "riscv64-inline",
                "LTP_BLACKLIST_LA": "la-inline",
                "LTP_BLACKLIST_LOONGARCH64": "loongarch64-inline",
            }
        )
        for arch, image_variable, run_image_variable, memory_variable, arch_content in (
            ("rv", "RV_TESTSUITE_IMG", "RV_TESTSUITE_RUN_IMG", "RV_MEM", "rv-file"),
            ("la", "LA_TESTSUITE_IMG", "LA_TESTSUITE_RUN_IMG", "LA_MEM", "la-file"),
        ):
            with self.subTest(arch=arch):
                result = subprocess.run(
                    [str(ROOT / "test/evaluation/run_official_evaluation.sh"), arch],
                    cwd=directory,
                    env=environment,
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    check=False,
                )
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                arguments = args_log.read_text(encoding="utf-8").splitlines()
                self.assertIn(
                    f"{image_variable}={directory}/images/sdcard-{arch}.img",
                    arguments,
                )
                self.assertTrue(
                    any(
                        value.startswith(f"{run_image_variable}={directory}/out/")
                        for value in arguments
                    ),
                    arguments,
                )
                self.assertIn("KERNEL_SMP=1", arguments)
                self.assertIn(f"{memory_variable}=1G", arguments)
                self.assertFalse(
                    any(value.startswith(("SMP=", "MEM=")) for value in arguments),
                    arguments,
                )
                environment_text = environment_log.read_text(encoding="utf-8")
                composed = environment_text.split("LTP_BLACKLIST_BEGIN\n", 1)[1].split(
                    "\nLTP_BLACKLIST_END\n", 1
                )[0]
                self.assertEqual(
                    composed.splitlines(),
                    ["inline-base", "generic-one", "generic-two", "common", arch_content],
                )
                self.assertNotIn("la-file" if arch == "rv" else "rv-file", composed)
                self.assertIn("CARGO_NET_OFFLINE=true", environment_text)
                self.assertIn("PYTHONNOUSERSITE=1", environment_text)
                self.assertIn("PYTHONDONTWRITEBYTECODE=1", environment_text)
                self.assertIn("PYTHONPYCACHEPREFIX=/dev/null", environment_text)
                for variable, value in (
                    ("OSCOMP_TEST_GROUPS", "all"),
                    ("OSCOMP_SKIP_TEST_GROUPS", "none"),
                    ("OSCOMP_GROUP_TIMEOUT_CEILING_SECS", "900"),
                    ("LTP_CASES", "stable-full"),
                    ("LTP_CASE_TIMEOUT_SECS", "180"),
                ):
                    self.assertIn(f"{variable}={value}", environment_text)
                for variable, value in (
                    ("LTP_BLACKLIST_RV", "rv-inline"),
                    ("LTP_BLACKLIST_RISCV64", "riscv64-inline"),
                    ("LTP_BLACKLIST_LA", "la-inline"),
                    ("LTP_BLACKLIST_LOONGARCH64", "loongarch64-inline"),
                ):
                    self.assertIn(f"{variable}={value}", environment_text)
                for variable in (
                    ".SHELLFLAGS",
                    "AXCONFIG_GEN",
                    "MAKE",
                    "MAKEFILES",
                    "MAKEFLAGS",
                    "MAKEOVERRIDES",
                    "BASH_ENV",
                    "ENV",
                    "BASH_FUNC_make%%",
                    "KERNEL_APP",
                    "KERNEL_RV_FEATURES",
                    "KERNEL_RV_APP_FEATURES",
                    "KERNEL_LA_FEATURES",
                    "KERNEL_LA_APP_FEATURES",
                    "KERNEL_MODE",
                    "PLAT_CONFIG",
                    "KERNEL_RV",
                ):
                    self.assertNotIn(f"{variable}=", environment_text)
                self.assertFalse(
                    (directory / "makefiles-injection-loaded").exists(),
                    "untrusted MAKEFILES injection was loaded",
                )

        workspace = directory / "workspace"
        copied_repo = workspace / "repo"
        copied_executor = copied_repo / "test/evaluation/run_official_evaluation.sh"
        copied_executor.parent.mkdir(parents=True)
        shutil.copy2(ROOT / "test/evaluation/run_official_evaluation.sh", copied_executor)
        for image_name in ("sdcard-rv.img", "sdcard-la.img"):
            (workspace / image_name).write_bytes(b"parent-default-fixture")
        default_environment = environment.copy()
        for name in (
            "ORAYS_WORKSPACE_ROOT",
            "TESTSUITE_DIR",
            "RV_TESTSUITE_IMG",
            "LA_TESTSUITE_IMG",
        ):
            default_environment.pop(name, None)
        default_environment["ORAYS_TEST_OUTPUT_DIR"] = str(directory / "parent-default-out")
        for arch, image_variable in (
            ("rv", "RV_TESTSUITE_IMG"),
            ("la", "LA_TESTSUITE_IMG"),
        ):
            with self.subTest(parent_default_arch=arch):
                result = subprocess.run(
                    [str(copied_executor), arch],
                    cwd=directory,
                    env=default_environment,
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    check=False,
                )
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                image_argument = next(
                    value
                    for value in args_log.read_text(encoding="utf-8").splitlines()
                    if value.startswith(f"{image_variable}=")
                )
                self.assertEqual(
                    Path(image_argument.split("=", 1)[1]).resolve(),
                    workspace / f"sdcard-{arch}.img",
                )

    def test_official_executor_reserves_125_for_preflight_infrastructure(self) -> None:
        directory, environment, args_log, _environment_log = self.fake_official_environment()
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

        (directory / "images/sdcard-rv.img").write_bytes(b"fixture")
        environment["LTP_BLACKLIST_FILE"] = "missing-blacklist.txt"
        missing_blacklist = subprocess.run(
            [str(ROOT / "test/evaluation/run_official_evaluation.sh"), "rv"],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(
            missing_blacklist.returncode,
            125,
            missing_blacklist.stdout + missing_blacklist.stderr,
        )
        self.assertIn("LTP_BLACKLIST_FILE", missing_blacklist.stderr)
        self.assertIn("missing or unreadable blacklist file", missing_blacklist.stderr)

        environment.pop("LTP_BLACKLIST_FILE")
        malicious_image = directory / "images/sdcard-rv.img;printf-injected;#"
        malicious_image.write_bytes(b"fixture")
        environment["RV_TESTSUITE_IMG"] = str(malicious_image)
        unsafe_path = subprocess.run(
            [str(ROOT / "test/evaluation/run_official_evaluation.sh"), "rv"],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(unsafe_path.returncode, 125, unsafe_path.stdout + unsafe_path.stderr)
        self.assertIn("not a safe absolute path for Make", unsafe_path.stderr)
        self.assertFalse(args_log.exists(), "an unsafe image path reached Make")

        environment["RV_TESTSUITE_IMG"] = "images/sdcard-rv.img"
        make_expansion_marker = directory / "selector-make-expansion-ran"
        environment["LTP_BLACKLIST"] = f"$(shell touch {make_expansion_marker})"
        unsafe_selector = subprocess.run(
            [str(ROOT / "test/evaluation/run_official_evaluation.sh"), "rv"],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(
            unsafe_selector.returncode,
            125,
            unsafe_selector.stdout + unsafe_selector.stderr,
        )
        self.assertIn("dollar sign that Make could expand", unsafe_selector.stderr)
        self.assertFalse(args_log.exists(), "an unsafe selector reached Make")
        self.assertFalse(
            make_expansion_marker.exists(),
            "Make expanded an untrusted official selector",
        )

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
        directory, environment, _args_log, environment_log = self.fake_official_environment()
        blacklist_directory = directory / "blacklists"
        blacklist_directory.mkdir()
        for name, content in (
            ("generic.txt", "generic-entry"),
            ("common.txt", "common-entry"),
            ("rv.txt", "rv-entry"),
        ):
            (blacklist_directory / name).write_text(f"{content}\n", encoding="utf-8")
        environment.update(
            {
                "LTP_BLACKLIST": "inline-entry",
                "LTP_BLACKLIST_FILE": "blacklists/generic.txt",
                "LTP_BLACKLIST_COMMON_FILE": "blacklists/common.txt",
                "LTP_BLACKLIST_RV_FILE": "blacklists/rv.txt",
            }
        )
        fake_make = directory / "bin/make"
        fake_make.write_text(
            "#!/bin/sh\n"
            "{\n"
            "  printf '%s\\n' LTP_BLACKLIST_BEGIN\n"
            "  printf '%s\\n' \"${LTP_BLACKLIST-}\"\n"
            "  printf '%s\\n' LTP_BLACKLIST_END\n"
            f"}} > {shlex.quote(str(environment_log))}\n"
            "printf '%s\\n' "
            "'#### OS COMP TEST GROUP START demo-musl ####' "
            "'FAIL OFFICIAL TEST GROUP demo-musl : 7' "
            "'#### OS COMP TEST GROUP END demo-musl ####'\n"
            "exit 0\n",
            encoding="utf-8",
        )
        fake_make.chmod(0o755)
        canonical_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        fixture_case = json.loads(
            json.dumps(
                next(
                    case
                    for case in canonical_manifest["cases"]
                    if case["id"] == "official.riscv64"
                )
            )
        )
        fixture_case["id"] = "official.fixture-guest-failure"
        fixture_case["result_contract"] = {
            "type": "official",
            "expected_group_labels": ["demo-musl"],
            "expected_group_case_counts": {},
        }
        fixture_manifest = {
            "schema_version": 1,
            "baseline_ref": "origin/main",
            "profiles": {
                "fixture": {
                    "description": "official failure propagation fixture",
                    "arch_policy": "none",
                    "include": [],
                    "cases": [fixture_case["id"]],
                    "arch_cases": {},
                }
            },
            "cases": [fixture_case],
        }
        fixture_manifest_path = directory / "official-fixture-manifest.json"
        fixture_manifest_path.write_text(
            json.dumps(fixture_manifest),
            encoding="utf-8",
        )
        environment["RV_TESTSUITE_IMG"] = str(
            (directory / "images/sdcard-rv.img").resolve()
        )
        output_dir = directory / "suite-output"
        result = subprocess.run(
            [
                sys.executable,
                "-I",
                "-S",
                "-B",
                "-X",
                "pycache_prefix=/dev/null",
                str(ROOT / "test/run_suite.py"),
                "--manifest",
                str(fixture_manifest_path),
                "--profile",
                "fixture",
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
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        summary = json.loads((output_dir / "summary.json").read_text(encoding="utf-8"))
        self.assertIn(summary["result"], {"FAIL", "INFRA_ERROR"}, summary)
        self.assertIn(summary["cases"][0]["status"], {"FAIL", "INFRA_ERROR"}, summary)
        self.assertNotEqual(summary["cases"][0]["status"], "PASS", summary)
        self.assertEqual(
            environment_log.read_text(encoding="utf-8").splitlines(),
            [
                "LTP_BLACKLIST_BEGIN",
                "inline-entry",
                "generic-entry",
                "common-entry",
                "rv-entry",
                "LTP_BLACKLIST_END",
            ],
        )
        self.assertEqual(
            summary["cases"][0]["details"]["noncanonical_official_environment"],
            [
                "LTP_BLACKLIST",
                "LTP_BLACKLIST_FILE",
                "LTP_BLACKLIST_COMMON_FILE",
                "LTP_BLACKLIST_RV_FILE",
            ],
        )

        environment_log.unlink()
        unsafe_output = directory / "suite-output;printf-injected;#"
        unsafe_result = subprocess.run(
            [
                sys.executable,
                "-I",
                "-S",
                "-B",
                "-X",
                "pycache_prefix=/dev/null",
                str(ROOT / "test/run_suite.py"),
                "--manifest",
                str(fixture_manifest_path),
                "--profile",
                "fixture",
                "--output-dir",
                str(unsafe_output),
            ],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(unsafe_result.returncode, 2, unsafe_result.stdout + unsafe_result.stderr)
        unsafe_summary = json.loads(
            (unsafe_output / "summary.json").read_text(encoding="utf-8")
        )
        self.assertEqual(unsafe_summary["cases"][0]["status"], "INFRA_ERROR")
        unsafe_stderr = Path(
            unsafe_summary["cases"][0]["stderr_log"]
        ).read_text(encoding="utf-8")
        self.assertIn(
            "not a safe absolute path for Make",
            unsafe_stderr,
        )
        self.assertFalse(environment_log.exists(), "an unsafe output path reached Make")

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
                "-I",
                "-S",
                "-B",
                "-X",
                "pycache_prefix=/dev/null",
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

        default_result = subprocess.run(
            [str(ROOT / "run-eval.sh")],
            cwd=directory,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(default_result.returncode, 37, default_result.stdout + default_result.stderr)
        self.assertFalse(hook_marker.exists(), "root wrapper executed inherited BASH_ENV")
        self.assertEqual(
            args_log.read_text(encoding="utf-8").splitlines(),
            [
                "-I",
                "-S",
                "-B",
                "-X",
                "pycache_prefix=/dev/null",
                str(ROOT / "test/run_suite.py"),
                "--profile",
                "official",
                "--arch",
                "rv",
            ],
        )

        for arguments in (
            ["rv", "--manifest", str(directory / "alternate.json")],
            ["rv", "--profile", "quick"],
            ["rv", "--arch", "la"],
            ["rv", "--list"],
        ):
            with self.subTest(arguments=arguments):
                args_log.unlink(missing_ok=True)
                rejected = subprocess.run(
                    [str(ROOT / "run-eval.sh"), *arguments],
                    cwd=directory,
                    env=environment,
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    check=False,
                )
                self.assertEqual(2, rejected.returncode, rejected.stdout + rejected.stderr)
                self.assertIn("unsupported official entry argument", rejected.stderr)
                self.assertFalse(args_log.exists(), "rejected options reached Python")

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
            'Ok(status) if expected_status.is_met_by(status) => "success",',
            'Ok(status) if expected_status.is_met_by(status) || line == "false" => "success",',
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

        framing_tree = self.make_tree()
        framing_path = framing_tree / "user/shell/src/cmd.rs"
        framing_text = self.replace_once(
            framing_path.read_text(encoding="utf-8"),
            'println!("PASS OFFICIAL TEST GROUP {label} : 0");',
            'println!("official group completed");',
        )
        framing_path.write_text(framing_text, encoding="utf-8")
        framing_result = self.run_guard(framing_tree)
        self.assertNotEqual(framing_result.returncode, 0)
        self.assertIn("status-bound PASS/FAIL", framing_result.stdout)

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

    def test_detects_removal_of_debian_posix_shell_support(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8") + (
            '\nfn official_shell_for_suite(suite_dir: &str) { '
            'let _ = join_path(suite_dir, "busybox"); }\n'
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("real POSIX /bin/sh", result.stdout)

    def test_detects_busybox_only_official_autorun(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = path.read_text(encoding="utf-8").replace(
            "run_official_shell_command",
            "missing_official_shell_command",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("ordinary POSIX shells", result.stdout)

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

    def test_detects_missing_busybox_ordinal_identity_framing(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/cmd.rs"
        text = self.replace_once(
            path.read_text(encoding="utf-8"),
            "case_ordinal += 1;",
            "case_ordinal += 0;",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("stable ordinal START/RESULT/END", result.stdout)

        compatibility_tree = self.make_tree()
        compatibility_path = compatibility_tree / "user/shell/src/cmd.rs"
        compatibility_text = self.replace_once(
            compatibility_path.read_text(encoding="utf-8"),
            "testcase busybox {label_line} {case_status}",
            "compatibility record removed",
        )
        compatibility_path.write_text(compatibility_text, encoding="utf-8")
        compatibility_result = self.run_guard(compatibility_tree)
        self.assertNotEqual(compatibility_result.returncode, 0)
        self.assertIn("scorer-compatible result projection", compatibility_result.stdout)

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

        image_tree = self.make_tree()
        image_makefile = image_tree / "Makefile"
        image_makefile.write_text(
            self.replace_once(
                image_makefile.read_text(encoding="utf-8"),
                "RV_TESTSUITE_IMG ?= $(ORAYS_WORKSPACE_ROOT)/sdcard-rv.img",
                "RV_TESTSUITE_IMG ?= /tmp/fixed-rv.img",
            ),
            encoding="utf-8",
        )
        image_result = self.run_guard(image_tree)
        self.assertNotEqual(image_result.returncode, 0)
        self.assertIn("repository-parent workspace root", image_result.stdout)

    def test_detects_missing_promotion_mode_blocker(self) -> None:
        tree = self.make_tree()
        path = tree / "test/evaluation/summarize_ltp_results.py"
        path.write_text(path.read_text(encoding="utf-8").replace("promotion_mode_blocker", "promotion_mode_missing"), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("promotion mode blocker", result.stdout)

        lifecycle_tree = self.make_tree()
        lifecycle_path = lifecycle_tree / "test/evaluation/summarize_ltp_results.py"
        lifecycle_path.write_text(
            lifecycle_path.read_text(encoding="utf-8").replace(
                "validate_ltp_output",
                "validate_unscoped_output",
            ),
            encoding="utf-8",
        )
        lifecycle_result = self.run_guard(lifecycle_tree)
        self.assertNotEqual(lifecycle_result.returncode, 0)
        self.assertIn("LTP-scoped validation", lifecycle_result.stdout)

        dimension_tree = self.make_tree()
        dimension_path = dimension_tree / "test/evaluation/summarize_ltp_results.py"
        dimension_path.write_text(
            dimension_path.read_text(encoding="utf-8").replace(
                "validate_promotion_dimensions",
                "accept_empty_promotion_dimensions",
            ),
            encoding="utf-8",
        )
        dimension_result = self.run_guard(dimension_tree)
        self.assertNotEqual(dimension_result.returncode, 0)
        self.assertIn("nonempty known promotion dimensions", dimension_result.stdout)

        pair_tree = self.make_tree()
        pair_summary = pair_tree / "test/evaluation/summarize_ltp_results.py"
        pair_summary.write_text(
            pair_summary.read_text(encoding="utf-8").replace(
                "validate_promotion_input_pairs",
                "accept_unpaired_promotion_inputs",
            ),
            encoding="utf-8",
        )
        pair_result = self.run_guard(pair_tree)
        self.assertNotEqual(pair_result.returncode, 0)
        self.assertIn("strict promotion stdout/stderr identity pairing", pair_result.stdout)

        source_key_tree = self.make_tree()
        source_key_summary = source_key_tree / "test/evaluation/summarize_ltp_results.py"
        source_key_summary.write_text(
            source_key_summary.read_text(encoding="utf-8").replace(
                "capture_source_key",
                "stream_identity_key",
            ),
            encoding="utf-8",
        )
        source_key_result = self.run_guard(source_key_tree)
        self.assertNotEqual(source_key_result.returncode, 0)
        self.assertIn("exact promotion capture source keys", source_key_result.stdout)

        digest_tree = self.make_tree()
        digest_summary = digest_tree / "test/evaluation/summarize_ltp_results.py"
        digest_summary.write_text(
            digest_summary.read_text(encoding="utf-8").replace(
                "hashlib.sha256(raw).hexdigest()",
                "hashlib.sha1(raw).hexdigest()",
            ),
            encoding="utf-8",
        )
        digest_result = self.run_guard(digest_tree)
        self.assertNotEqual(digest_result.returncode, 0)
        self.assertIn("raw stdout SHA-256 provenance", digest_result.stdout)

        stderr_digest_tree = self.make_tree()
        stderr_digest_summary = (
            stderr_digest_tree / "test/evaluation/summarize_ltp_results.py"
        )
        stderr_digest_summary.write_text(
            stderr_digest_summary.read_text(encoding="utf-8").replace(
                "hashlib.sha256(stderr_raw).hexdigest()",
                "hashlib.sha1(stderr_raw).hexdigest()",
            ),
            encoding="utf-8",
        )
        stderr_digest_result = self.run_guard(stderr_digest_tree)
        self.assertNotEqual(stderr_digest_result.returncode, 0)
        self.assertIn("raw stderr SHA-256 provenance", stderr_digest_result.stdout)

        isolation_tree = self.make_tree()
        isolation_summary = isolation_tree / "test/evaluation/summarize_ltp_results.py"
        isolation_summary.write_text(
            isolation_summary.read_text(encoding="utf-8").replace(
                "or not _bootstrap_sys.flags.no_site",
                "or False",
                1,
            ),
            encoding="utf-8",
        )
        isolation_result = self.run_guard(isolation_tree)
        self.assertNotEqual(isolation_result.returncode, 0)
        self.assertIn(
            "isolated result-tool startup without site initialization",
            isolation_result.stdout,
        )

        cache_tree = self.make_tree()
        cache_summary = cache_tree / "test/evaluation/summarize_ltp_results.py"
        cache_summary.write_text(
            cache_summary.read_text(encoding="utf-8").replace(
                'or _bootstrap_sys.pycache_prefix != "/dev/null"',
                "or False",
                1,
            ),
            encoding="utf-8",
        )
        cache_result = self.run_guard(cache_tree)
        self.assertNotEqual(cache_result.returncode, 0)
        self.assertIn("isolated result-tool bytecode cache boundary", cache_result.stdout)

        stderr_tree = self.make_tree()
        stderr_summary = stderr_tree / "test/evaluation/summarize_ltp_results.py"
        stderr_summary.write_text(
            stderr_summary.read_text(encoding="utf-8").replace(
                '"--stderr-log"',
                '"--optional-stderr-log"',
            ),
            encoding="utf-8",
        )
        stderr_result = self.run_guard(stderr_tree)
        self.assertNotEqual(stderr_result.returncode, 0)
        self.assertIn("mandatory stderr companion input", stderr_result.stdout)

        strict_stderr_tree = self.make_tree()
        strict_stderr_summary = (
            strict_stderr_tree / "test/evaluation/summarize_ltp_results.py"
        )
        strict_stderr_summary.write_text(
            strict_stderr_summary.read_text(encoding="utf-8").replace(
                "args.strict and not args.promotion_candidates",
                "False",
            ),
            encoding="utf-8",
        )
        strict_stderr_result = self.run_guard(strict_stderr_tree)
        self.assertNotEqual(strict_stderr_result.returncode, 0)
        self.assertIn(
            "mandatory strict-mode stderr companion",
            strict_stderr_result.stdout,
        )

        group_tree = self.make_tree()
        group_summary = group_tree / "test/evaluation/summarize_ltp_results.py"
        group_summary.write_text(
            group_summary.read_text(encoding="utf-8").replace(
                "noncanonical-ltp-group=",
                "permissive-ltp-group=",
            ),
            encoding="utf-8",
        )
        group_result = self.run_guard(group_tree)
        self.assertNotEqual(group_result.returncode, 0)
        self.assertIn("exact canonical LTP group eligibility blocker", group_result.stdout)

        subset_tree = self.make_tree()
        subset_summary = subset_tree / "test/evaluation/summarize_ltp_results.py"
        subset_summary.write_text(
            subset_summary.read_text(encoding="utf-8").replace(
                "required_arches != KNOWN_PROMOTION_ARCHES",
                "False",
            ),
            encoding="utf-8",
        )
        subset_result = self.run_guard(subset_tree)
        self.assertNotEqual(subset_result.returncode, 0)
        self.assertIn("full RV/LA promotion matrix gate", subset_result.stdout)

        libc_subset_tree = self.make_tree()
        libc_subset_summary = (
            libc_subset_tree / "test/evaluation/summarize_ltp_results.py"
        )
        libc_subset_summary.write_text(
            libc_subset_summary.read_text(encoding="utf-8").replace(
                "required_libcs != KNOWN_PROMOTION_LIBCS",
                "False",
            ),
            encoding="utf-8",
        )
        libc_subset_result = self.run_guard(libc_subset_tree)
        self.assertNotEqual(libc_subset_result.returncode, 0)
        self.assertIn("full musl/glibc promotion matrix gate", libc_subset_result.stdout)

        strict_binding_tree = self.make_tree()
        strict_binding_summary = (
            strict_binding_tree / "test/evaluation/summarize_ltp_results.py"
        )
        strict_binding_summary.write_text(
            strict_binding_summary.read_text(encoding="utf-8").replace(
                "strict_case_binding",
                "unchecked_case_binding",
            ),
            encoding="utf-8",
        )
        strict_binding_result = self.run_guard(strict_binding_tree)
        self.assertNotEqual(strict_binding_result.returncode, 0)
        self.assertIn("source/group/case lifecycle binding", strict_binding_result.stdout)

        promotion_branch_tree = self.make_tree()
        promotion_branch_summary = (
            promotion_branch_tree / "test/evaluation/summarize_ltp_results.py"
        )
        promotion_branch_summary.write_text(
            promotion_branch_summary.read_text(encoding="utf-8").replace(
                "args.strict or args.promotion_candidates",
                "args.strict",
            ),
            encoding="utf-8",
        )
        promotion_branch_result = self.run_guard(promotion_branch_tree)
        self.assertNotEqual(promotion_branch_result.returncode, 0)
        self.assertIn("mandatory promotion validation branch", promotion_branch_result.stdout)

        lifecycle_test_tree = self.make_tree()
        lifecycle_test_path = lifecycle_test_tree / "test/unit/test_ltp_result_summary.py"
        lifecycle_test_path.write_text(
            lifecycle_test_path.read_text(encoding="utf-8").replace(
                "test_promotion_candidate_requires_complete_lifecycle",
                "test_promotion_candidate_accepts_incomplete_lifecycle",
            ),
            encoding="utf-8",
        )
        lifecycle_test_result = self.run_guard(lifecycle_test_tree)
        self.assertNotEqual(lifecycle_test_result.returncode, 0)
        self.assertIn(
            "test_promotion_candidate_requires_complete_lifecycle",
            lifecycle_test_result.stdout,
        )

        malformed_protocol_tree = self.make_tree()
        malformed_protocol_path = (
            malformed_protocol_tree / "test/unit/test_ltp_result_summary.py"
        )
        malformed_protocol_path.write_text(
            malformed_protocol_path.read_text(encoding="utf-8").replace(
                "strict-malformed-protocol-record",
                "ignored-malformed-protocol-record",
            ),
            encoding="utf-8",
        )
        malformed_protocol_result = self.run_guard(malformed_protocol_tree)
        self.assertNotEqual(malformed_protocol_result.returncode, 0)
        self.assertIn("malformed protocol promotion assertion", malformed_protocol_result.stdout)

        outside_assertion_tree = self.make_tree()
        outside_assertion_path = (
            outside_assertion_tree / "test/unit/test_ltp_result_summary.py"
        )
        outside_assertion_path.write_text(
            outside_assertion_path.read_text(encoding="utf-8").replace(
                '"outside-ltp-group" in reason',
                '"ignored-outside-ltp-group" in reason',
                1,
            ),
            encoding="utf-8",
        )
        outside_assertion_result = self.run_guard(outside_assertion_tree)
        self.assertNotEqual(outside_assertion_result.returncode, 0)
        self.assertIn("outside-group quality-signal assertion", outside_assertion_result.stdout)

        invalid_dimension_tree = self.make_tree()
        invalid_dimension_path = (
            invalid_dimension_tree / "test/unit/test_ltp_result_summary.py"
        )
        invalid_dimension_path.write_text(
            invalid_dimension_path.read_text(encoding="utf-8").replace(
                "self.assertEqual(invalid.returncode, 2",
                "self.assertEqual(invalid.returncode, 0",
                1,
            ),
            encoding="utf-8",
        )
        invalid_dimension_result = self.run_guard(invalid_dimension_tree)
        self.assertNotEqual(invalid_dimension_result.returncode, 0)
        self.assertIn("invalid-dimension CLI assertion", invalid_dimension_result.stdout)

        outside_tree = self.make_tree()
        outside_parser = outside_tree / "test/evaluation/parse_official_results.py"
        outside_parser.write_text(
            outside_parser.read_text(encoding="utf-8").replace(
                "outside-ltp-group",
                "ignored-outside-ltp-signal",
            ),
            encoding="utf-8",
        )
        outside_result = self.run_guard(outside_tree)
        self.assertNotEqual(outside_result.returncode, 0)
        self.assertIn("outside-group LTP quality blocker", outside_result.stdout)


if __name__ == "__main__":
    unittest.main()
