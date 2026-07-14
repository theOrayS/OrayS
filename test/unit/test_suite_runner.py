#!/usr/bin/env python3
"""Integration and unit tests for the manifest-driven local suite runner."""

from __future__ import annotations

import json
import os
import signal
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
RUNNER = ROOT / "test/run_suite.py"


def fixture_case(
    case_id: str = "fixture.pass",
    *,
    code: str = "print('CASE_RESULT: PASS')",
    timeout: float = 5,
    contract: dict[str, Any] | None = None,
) -> dict[str, Any]:
    return {
        "id": case_id,
        "description": "tiny runner integrity fixture",
        "command": [sys.executable, "-c", code],
        "cwd": "{repo}",
        "timeout_seconds": timeout,
        "result_contract": contract or {"type": "case_result"},
        "required_paths": [],
        "required_commands": [],
    }


def fixture_manifest(cases: list[dict[str, Any]], case_ids: list[str] | None = None) -> dict[str, Any]:
    return {
        "schema_version": 1,
        "baseline_ref": "origin/main",
        "profiles": {
            "quick": {
                "description": "fixture profile",
                "arch_policy": "none",
                "include": [],
                "cases": case_ids if case_ids is not None else [case["id"] for case in cases],
                "arch_cases": {},
            }
        },
        "cases": cases,
    }


def canonical_official_case(case_id: str = "official.rv") -> dict[str, Any]:
    manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
    template = next(case for case in manifest["cases"] if case["id"] == "official.riscv64")
    case = fixture_case(case_id)
    case["result_contract"] = template["result_contract"]
    case["environment"] = template["environment"]
    return case


class SuiteRunnerTest(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="suite-runner-test-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.manifest_path = self.work / "manifest.json"
        self.output_path = self.work / "output"

    def write_manifest(self, manifest: dict[str, Any]) -> Path:
        self.manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
        return self.manifest_path

    def invoke(
        self,
        manifest: dict[str, Any] | None = None,
        *,
        extra: list[str] | None = None,
        cwd: Path = ROOT,
        environment: dict[str, str] | None = None,
    ) -> subprocess.CompletedProcess[str]:
        if manifest is not None:
            self.write_manifest(manifest)
        command = [
            sys.executable,
            str(RUNNER),
            "--manifest",
            str(self.manifest_path),
            "--profile",
            "quick",
            "--output-dir",
            str(self.output_path),
        ]
        if extra:
            command.extend(extra)
        return subprocess.run(
            command,
            cwd=cwd,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )

    def summary(self) -> dict[str, Any]:
        return json.loads((self.output_path / "summary.json").read_text(encoding="utf-8"))

    def test_missing_manifest_is_infrastructure_error(self) -> None:
        result = self.invoke()
        self.assertEqual(result.returncode, 2)
        self.assertIn("manifest not found", result.stderr)

    def test_malformed_json_is_infrastructure_error(self) -> None:
        self.manifest_path.write_text("{not json", encoding="utf-8")
        result = self.invoke()
        self.assertEqual(result.returncode, 2)
        self.assertIn("malformed manifest JSON", result.stderr)

    def test_non_utf8_manifest_is_infrastructure_error(self) -> None:
        self.manifest_path.write_bytes(b"{\xff}")
        result = self.invoke()
        self.assertEqual(result.returncode, 2)
        self.assertIn("manifest is not valid UTF-8", result.stderr)

    def test_duplicate_json_key_is_rejected(self) -> None:
        self.manifest_path.write_text(
            '{"schema_version": 1, "schema_version": 1, "profiles": {}, "cases": []}',
            encoding="utf-8",
        )
        result = self.invoke()
        self.assertEqual(result.returncode, 2)
        self.assertIn("duplicate JSON key", result.stderr)

    def test_embedded_nul_in_command_argument_is_rejected(self) -> None:
        case = fixture_case()
        case["command"][2] += "\x00pollution"
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("embedded NUL byte", result.stderr)

    def test_embedded_nul_in_environment_value_is_rejected(self) -> None:
        case = fixture_case()
        case["environment"] = {"FIXTURE_VALUE": "bad\x00value"}
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("embedded NUL byte", result.stderr)

    def test_unsupported_schema_version_is_infrastructure_error(self) -> None:
        manifest = fixture_manifest([fixture_case()])
        manifest["schema_version"] = 99
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("unsupported manifest schema_version", result.stderr)

    def test_boolean_schema_version_is_rejected(self) -> None:
        manifest = fixture_manifest([fixture_case()])
        manifest["schema_version"] = True
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("unsupported manifest schema_version", result.stderr)

    def test_empty_profile_cannot_pass(self) -> None:
        case = fixture_case()
        manifest = fixture_manifest([case], case_ids=[])
        manifest["profiles"]["registered"] = {
            "description": "keeps the fixture reachable",
            "arch_policy": "none",
            "include": [],
            "cases": [case["id"]],
            "arch_cases": {},
        }
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("selected zero cases", result.stderr)

    def test_list_rejects_any_zero_selection_profile(self) -> None:
        case = fixture_case()
        manifest = fixture_manifest([case], case_ids=[])
        manifest["profiles"]["registered"] = {
            "description": "keeps the fixture reachable",
            "arch_policy": "none",
            "include": [],
            "cases": [case["id"]],
            "arch_cases": {},
        }
        result = self.invoke(manifest, extra=["--list"])
        self.assertEqual(result.returncode, 2)
        self.assertIn("selected zero cases", result.stderr)

    def test_list_rejects_duplicate_profile_include(self) -> None:
        case = fixture_case()
        manifest = fixture_manifest([case])
        manifest["profiles"]["base"] = manifest["profiles"]["quick"]
        manifest["profiles"]["quick"] = {
            "description": "duplicate include graph",
            "arch_policy": "none",
            "include": ["base", "base"],
            "cases": [],
            "arch_cases": {},
        }
        result = self.invoke(manifest, extra=["--list"])
        self.assertEqual(result.returncode, 2)
        self.assertIn("include contains duplicates", result.stderr)

    def test_unreachable_manifest_case_is_rejected(self) -> None:
        selected = fixture_case("fixture.selected")
        orphan = fixture_case("fixture.orphan")
        result = self.invoke(fixture_manifest([selected, orphan], case_ids=[selected["id"]]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("unreachable", result.stderr)

    def test_duplicate_test_id_is_rejected(self) -> None:
        case = fixture_case()
        manifest = fixture_manifest([case, dict(case)])
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("duplicate test ID", result.stderr)

    def test_infinite_timeout_is_rejected(self) -> None:
        manifest = fixture_manifest([fixture_case()])
        raw = json.dumps(manifest).replace('"timeout_seconds": 5', '"timeout_seconds": 1e999')
        self.manifest_path.write_text(raw, encoding="utf-8")
        result = self.invoke()
        self.assertEqual(result.returncode, 2)
        self.assertIn("finite positive number", result.stderr)

    def test_huge_integer_timeout_is_rejected_without_traceback(self) -> None:
        manifest = fixture_manifest([fixture_case()])
        manifest["cases"][0]["timeout_seconds"] = 10**400
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("finite positive number", result.stderr)
        self.assertNotIn("Traceback", result.stderr)

    def test_cwd_escape_is_rejected_before_execution(self) -> None:
        case = fixture_case()
        case["cwd"] = "{repo}/.."
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("escapes the repository", result.stderr)
        self.assertFalse(self.output_path.exists())

    def test_required_path_escape_is_rejected_before_execution(self) -> None:
        case = fixture_case()
        case["required_paths"] = ["{repo}/../outside-test-asset"]
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("escapes the repository", result.stderr)
        self.assertFalse(self.output_path.exists())

    def test_nonexistent_command_is_infrastructure_error(self) -> None:
        case = fixture_case()
        case["command"] = ["orays-command-that-does-not-exist"]
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        summary = self.summary()
        self.assertEqual(summary["cases"][0]["status"], "INFRA_ERROR")
        self.assertFalse(summary["cases"][0]["executed"])

    def test_nonexistent_registered_script_is_rejected(self) -> None:
        case = fixture_case()
        case["command"] = [sys.executable, "{repo}/test/missing_fixture.py"]
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("not an existing file", result.stderr)

    def test_child_exit_one_is_failure(self) -> None:
        case = fixture_case(code="raise SystemExit(1)")
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(self.summary()["cases"][0]["status"], "FAIL")

    def test_child_signal_termination_is_crash(self) -> None:
        code = "import os, signal; os.kill(os.getpid(), signal.SIGTERM)"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 1)
        case_result = self.summary()["cases"][0]
        self.assertEqual(case_result["status"], "CRASH")
        self.assertEqual(case_result["signal"], signal.SIGTERM)

    def test_timeout_kills_descendant_process_group(self) -> None:
        pid_file = self.work / "descendant.pid"
        ready_file = self.work / "descendant.ready"
        descendant_code = (
            "import pathlib, signal, time; "
            "signal.signal(signal.SIGTERM, signal.SIG_IGN); "
            f"pathlib.Path({str(ready_file)!r}).write_text('ready'); "
            "time.sleep(60)"
        )
        code = (
            "import pathlib, subprocess, sys, time; "
            f"child=subprocess.Popen([sys.executable, '-c', {descendant_code!r}]); "
            f"ready=pathlib.Path({str(ready_file)!r}); "
            "deadline=time.monotonic()+2; "
            "exec('while not ready.exists() and time.monotonic() < deadline:\\n    time.sleep(0.01)'); "
            f"pathlib.Path({str(pid_file)!r}).write_text(str(child.pid)); "
            "time.sleep(60)"
        )
        result = self.invoke(fixture_manifest([fixture_case(code=code, timeout=0.5)]))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(self.summary()["cases"][0]["status"], "TIMEOUT")
        descendant = int(pid_file.read_text(encoding="utf-8"))
        deadline = time.monotonic() + 3
        while time.monotonic() < deadline and self.process_is_live(descendant):
            time.sleep(0.05)
        self.assertFalse(self.process_is_live(descendant), f"descendant {descendant} survived timeout cleanup")

    def test_timeout_kills_descendant_that_escapes_process_group(self) -> None:
        pid_file = self.work / "escaped-descendant.pid"
        child_code = "import signal,time; signal.signal(signal.SIGTERM, signal.SIG_IGN); time.sleep(60)"
        code = (
            "import pathlib,subprocess,sys,time; "
            f"child=subprocess.Popen([sys.executable,'-c',{child_code!r}], start_new_session=True); "
            f"pathlib.Path({str(pid_file)!r}).write_text(str(child.pid)); "
            "time.sleep(60)"
        )
        result = self.invoke(fixture_manifest([fixture_case(code=code, timeout=0.5)]))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(self.summary()["cases"][0]["status"], "TIMEOUT")
        descendant = int(pid_file.read_text(encoding="utf-8"))
        deadline = time.monotonic() + 3
        while time.monotonic() < deadline and self.process_is_live(descendant):
            time.sleep(0.05)
        self.assertFalse(self.process_is_live(descendant), f"escaped descendant {descendant} survived")

    def test_zero_exit_with_live_descendant_is_not_pass(self) -> None:
        pid_file = self.work / "early-exit-descendant.pid"
        child_code = "import time; time.sleep(60)"
        code = (
            "import pathlib,subprocess,sys,time; "
            f"child=subprocess.Popen([sys.executable,'-c',{child_code!r}]); "
            f"pathlib.Path({str(pid_file)!r}).write_text(str(child.pid)); "
            "time.sleep(0.1)"
        )
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 2)
        case_result = self.summary()["cases"][0]
        self.assertEqual(case_result["status"], "INFRA_ERROR")
        self.assertIn("descendant processes", case_result["result"])
        descendant = int(pid_file.read_text(encoding="utf-8"))
        deadline = time.monotonic() + 3
        while time.monotonic() < deadline and self.process_is_live(descendant):
            time.sleep(0.05)
        self.assertFalse(self.process_is_live(descendant), f"descendant {descendant} survived")

    def test_runner_interrupt_kills_active_child_and_finalizes_report(self) -> None:
        for runner_signal in (signal.SIGINT, signal.SIGTERM):
            with self.subTest(runner_signal=runner_signal):
                pid_file = self.work / f"interrupt-{runner_signal}.pid"
                output_path = self.work / f"interrupt-{runner_signal}-output"
                code = (
                    "import os, pathlib, time; "
                    f"pathlib.Path({str(pid_file)!r}).write_text(str(os.getpid())); "
                    "time.sleep(60)"
                )
                self.write_manifest(fixture_manifest([fixture_case(code=code)]))
                process = subprocess.Popen(
                    [
                        sys.executable,
                        str(RUNNER),
                        "--manifest",
                        str(self.manifest_path),
                        "--profile",
                        "quick",
                        "--output-dir",
                        str(output_path),
                    ],
                    cwd=ROOT,
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                )
                deadline = time.monotonic() + 5
                while time.monotonic() < deadline and not pid_file.is_file():
                    time.sleep(0.02)
                self.assertTrue(pid_file.is_file(), "fixture child did not start")
                child_pid = int(pid_file.read_text(encoding="utf-8"))
                os.kill(process.pid, runner_signal)
                stdout, stderr = process.communicate(timeout=10)
                self.assertEqual(process.returncode, 1, stdout + stderr)
                summary = json.loads((output_path / "summary.json").read_text(encoding="utf-8"))
                self.assertEqual(summary["result"], "FAIL")
                self.assertEqual(summary["cases"][0]["status"], "CRASH")
                self.assertEqual(summary["cases"][0]["signal"], runner_signal)
                deadline = time.monotonic() + 3
                while time.monotonic() < deadline and self.process_is_live(child_pid):
                    time.sleep(0.05)
                self.assertFalse(self.process_is_live(child_pid), f"child {child_pid} survived runner interrupt")

    @staticmethod
    def process_is_live(pid: int) -> bool:
        stat_path = Path(f"/proc/{pid}/stat")
        try:
            fields = stat_path.read_text(encoding="utf-8").split()
        except FileNotFoundError:
            return False
        return len(fields) > 2 and fields[2] != "Z"

    def test_stdout_and_stderr_are_captured_separately(self) -> None:
        code = (
            "import sys; print('stdout-evidence'); "
            "print('stderr-evidence', file=sys.stderr)"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "exit_code"})])
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        case_result = self.summary()["cases"][0]
        stdout = Path(case_result["stdout_log"]).read_text(encoding="utf-8")
        stderr = Path(case_result["stderr_log"]).read_text(encoding="utf-8")
        self.assertIn("stdout-evidence", stdout)
        self.assertNotIn("stderr-evidence", stdout)
        self.assertIn("stderr-evidence", stderr)
        self.assertNotIn("stdout-evidence", stderr)

    def test_invalid_utf8_check_output_cannot_pass(self) -> None:
        code = "import os; os.write(1, b'\\xff\\nPASS compliance regression guard\\n')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("not valid UTF-8", self.summary()["cases"][0]["result"])

    def test_invalid_utf8_unittest_output_cannot_pass(self) -> None:
        code = "import os; os.write(2, b'\\xff\\nRan 1 test in 0.001s\\nOK\\n')"
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("not valid UTF-8", self.summary()["cases"][0]["result"])

    def test_nul_in_captured_output_cannot_pass(self) -> None:
        code = "import os; os.write(1, b'\\x00\\nCASE_RESULT: PASS\\n')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("unsupported output character", self.summary()["cases"][0]["result"])

    def test_residual_escape_control_cannot_obscure_failure(self) -> None:
        code = "import os; os.write(1, b'F\\x1bXAIL\\nCASE_RESULT: PASS\\n')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("unsupported output character", self.summary()["cases"][0]["result"])

    def test_unicode_controls_and_formatting_cannot_obscure_failure(self) -> None:
        for index, codepoint in enumerate((0x85, 0x2028, 0x202E)):
            with self.subTest(codepoint=f"U+{codepoint:04X}"):
                self.output_path = self.work / f"unicode-control-{index}"
                code = (
                    f"print('F' + chr({codepoint}) + 'AIL'); "
                    "print('CASE_RESULT: PASS')"
                )
                result = self.invoke(fixture_manifest([fixture_case(code=code)]))
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertIn(
                    f"U+{codepoint:04X}",
                    self.summary()["cases"][0]["result"],
                )

    def test_visible_non_ascii_output_is_allowed(self) -> None:
        code = "print('完整性说明：正常'); print('CASE_RESULT: PASS')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_valid_ansi_sgr_is_normalized_without_changing_status(self) -> None:
        code = "import os; os.write(1, b'\\x1b[32mCASE_RESULT: PASS\\x1b[0m\\n')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_valid_ansi_sgr_cannot_split_and_hide_failure(self) -> None:
        code = "import os; os.write(1, b'F\\x1b[31mAIL\\x1b[0m\\nCASE_RESULT: PASS\\n')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)

    def test_bare_carriage_return_cannot_obscure_status_record(self) -> None:
        code = "import os; os.write(1, b'integrity check: PASS (0 findings)\\rSTATUS: MAYBE\\n')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("bare carriage return", self.summary()["cases"][0]["result"])

    def test_unknown_or_unexecuted_state_cannot_pass_any_contract(self) -> None:
        fixtures = (
            ("exit", "print('NOT_RUN')", {"type": "exit_code"}),
            ("check", "print('integrity check: PASS (0 findings)'); print('UNKNOWN')", {"type": "check"}),
            (
                "unittest",
                "import sys; print('UNRESOLVED', file=sys.stderr); print('Ran 1 test in 0.001s', file=sys.stderr); print('OK', file=sys.stderr)",
                {"type": "unittest", "expected_tests": 1},
            ),
            (
                "cargo",
                "print('UNSUPPORTED'); print('running 1 test'); print('test demo ... ok'); print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')",
                {"type": "cargo_test"},
            ),
            ("case", "print('INCONCLUSIVE'); print('CASE_RESULT: PASS')", {"type": "case_result"}),
        )
        for index, (label, code, contract) in enumerate(fixtures):
            with self.subTest(contract=label):
                self.output_path = self.work / f"unknown-state-{index}"
                result = self.invoke(
                    fixture_manifest([fixture_case(code=code, contract=contract)])
                )
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)

    def test_additional_unexecuted_state_synonyms_cannot_pass(self) -> None:
        for index, state in enumerate(
            (
                "NOT EXECUTED",
                "NOT-RUN",
                "NOT_EXECUTED",
                "UNEXECUTED",
                "DID NOT RUN",
                "NOT ATTEMPTED",
                "INCOMPLETE",
                "PARTIAL",
                "PARTIALLY EXECUTED",
                "INFRA_ERROR",
                "STATUS UNKNOWN",
                "RESULT UNKNOWN",
                "PENDING",
                "CANCELLED",
                "DISABLED",
                "OMITTED",
            )
        ):
            with self.subTest(state=state):
                self.output_path = self.work / f"unexecuted-synonym-{index}"
                code = f"print({state!r}); print('integrity check: PASS (0 findings)')"
                result = self.invoke(
                    fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
                )
                expected = 1 if state in {"INCOMPLETE", "PARTIAL", "PARTIALLY EXECUTED"} else 2
                self.assertEqual(result.returncode, expected, result.stdout + result.stderr)

    def test_explicit_zero_execution_cannot_pass(self) -> None:
        for index, marker in enumerate(
            (
                "NO TESTS RAN",
                "zero tests executed",
                "ran zero tests",
                "0 cases executed",
                "NO CASES RAN",
                "no runnable tests",
                "test suite is empty",
            )
        ):
            with self.subTest(marker=marker):
                self.output_path = self.work / f"zero-execution-{index}"
                code = f"print({marker!r}); print('CASE_RESULT: PASS')"
                result = self.invoke(fixture_manifest([fixture_case(code=code)]))
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)

    def test_signal_and_tap_failure_markers_cannot_pass_strict_contracts(self) -> None:
        fixtures = (
            ("exit", "print('SIGSEGV')", {"type": "exit_code"}),
            (
                "check",
                "print('SIGSEGV'); print('integrity check: PASS (0 findings)')",
                {"type": "check"},
            ),
            (
                "unittest",
                "import sys; print('SIGSEGV', file=sys.stderr); "
                "print('Ran 1 test in 0.001s', file=sys.stderr); "
                "print('OK', file=sys.stderr)",
                {"type": "unittest", "expected_tests": 1},
            ),
            (
                "cargo",
                "print('SIGSEGV'); print('running 1 test'); "
                "print('test demo ... ok'); "
                "print('test result: ok. 1 passed; 0 failed; 0 ignored; "
                "0 measured; 0 filtered out; finished in 0.01s')",
                {"type": "cargo_test"},
            ),
            (
                "case",
                "print('not ok 1 - smoke'); print('CASE_RESULT: PASS')",
                {"type": "case_result"},
            ),
        )
        for index, (label, code, contract) in enumerate(fixtures):
            with self.subTest(contract=label):
                self.output_path = self.work / f"signal-marker-{index}"
                result = self.invoke(
                    fixture_manifest([fixture_case(code=code, contract=contract)])
                )
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)

    def test_unknown_status_record_variants_cannot_pass(self) -> None:
        for index, record in enumerate(
            ("STATUS=MAYBE", "RESULT = MAYBE", "STATE: MAYBE", "CASE_STATUS=MAYBE")
        ):
            with self.subTest(record=record):
                self.output_path = self.work / f"unknown-status-variant-{index}"
                code = f"print({record!r}); print('integrity check: PASS (0 findings)')"
                result = self.invoke(
                    fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
                )
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)

    def test_structured_unknown_states_cannot_pass_any_strict_contract(self) -> None:
        fixtures = (
            (
                "check",
                "print('[UNSUPPORTED] feature demo'); print('integrity check: PASS (0 findings)')",
                {"type": "check"},
            ),
            (
                "unittest",
                "import sys; print('[NOT_RUN] case demo', file=sys.stderr); print('Ran 1 test in 0.001s', file=sys.stderr); print('OK', file=sys.stderr)",
                {"type": "unittest", "expected_tests": 1},
            ),
            (
                "cargo",
                "print('[UNSUPPORTED] demo'); print('running 1 test'); print('test demo ... ok'); print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')",
                {"type": "cargo_test"},
            ),
            (
                "case",
                "print('[NOT_RUN] case demo'); print('CASE_RESULT: PASS')",
                {"type": "case_result"},
            ),
        )
        for index, (label, code, contract) in enumerate(fixtures):
            with self.subTest(contract=label):
                self.output_path = self.work / f"structured-unknown-{index}"
                result = self.invoke(
                    fixture_manifest([fixture_case(code=code, contract=contract)])
                )
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)

    def test_cross_stream_completion_order_cannot_be_forged(self) -> None:
        fixtures = (
            (
                "check",
                "import sys; print('integrity check: PASS (0 findings)', file=sys.stderr, flush=True); print('starting second phase', flush=True)",
                {"type": "check"},
            ),
            (
                "unittest",
                "import sys; print('Ran 1 test in 0.001s', file=sys.stderr, flush=True); print('OK', file=sys.stderr, flush=True); print('starting second phase', flush=True)",
                {"type": "unittest", "expected_tests": 1},
            ),
            (
                "cargo",
                "import sys; print('running 1 test', file=sys.stderr, flush=True); print('test demo ... ok', file=sys.stderr, flush=True); print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s', file=sys.stderr, flush=True); print('starting second phase', flush=True)",
                {"type": "cargo_test"},
            ),
            (
                "case",
                "import sys; print('CASE_RESULT: PASS', file=sys.stderr, flush=True); print('starting second phase', flush=True)",
                {"type": "case_result"},
            ),
        )
        for index, (label, code, contract) in enumerate(fixtures):
            with self.subTest(contract=label):
                self.output_path = self.work / f"cross-stream-{index}"
                result = self.invoke(
                    fixture_manifest([fixture_case(code=code, contract=contract)])
                )
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)

    def test_runner_works_outside_repository_cwd(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case()]), cwd=self.work)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(self.summary()["cases"][0]["cwd"], str(ROOT))

    def test_fail_fast_records_planned_executed_mismatch(self) -> None:
        fail = fixture_case("fixture.fail", code="raise SystemExit(1)")
        passed = fixture_case("fixture.after")
        result = self.invoke(fixture_manifest([fail, passed]), extra=["--fail-fast"])
        self.assertEqual(result.returncode, 1)
        summary = self.summary()
        self.assertEqual(summary["planned_count"], 2)
        self.assertEqual(summary["executed_count"], 1)
        self.assertEqual(summary["totals"]["NOT_RUN"], 1)

    def test_malformed_case_result_is_infrastructure_error(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case(code="print('completed')")]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("expected exactly one CASE_RESULT", self.summary()["cases"][0]["result"])

    def test_case_result_pass_conflicting_with_tfail_is_failure(self) -> None:
        code = "print('internal TFAIL: mismatch'); print('CASE_RESULT: PASS')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 1)
        self.assertIn("conflicts", self.summary()["cases"][0]["result"])

    def test_check_contract_rejects_failure_even_if_pass_word_appears(self) -> None:
        code = "print('check result: FAIL'); print('expected PASS')"
        contract = {"type": "check"}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(self.summary()["cases"][0]["status"], "FAIL")

    def test_check_contract_rejects_inline_fail_after_pass(self) -> None:
        contract = {"type": "check"}
        result = self.invoke(
            fixture_manifest([fixture_case(code="print('PASS despite FAIL')", contract=contract)])
        )
        self.assertEqual(result.returncode, 1)
        self.assertEqual(self.summary()["cases"][0]["status"], "FAIL")

    def test_case_result_pass_rejects_inline_fail_evidence(self) -> None:
        code = "print('CASE_RESULT: PASS'); print('operation FAIL later')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 1)

    def test_case_result_pass_rejects_inline_error_evidence(self) -> None:
        code = "print('CASE_RESULT: PASS'); print('operation ERROR later')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 1)

    def test_case_result_pass_rejects_crash_evidence(self) -> None:
        code = "print('CASE_RESULT: PASS'); print('Segmentation fault (core dumped)')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 1)

    def test_exit_code_zero_rejects_non_pass_evidence(self) -> None:
        code = "print('TFAIL: hidden failure')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "exit_code"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_exit_code_zero_rejects_unknown_status(self) -> None:
        code = "print('STATUS: MAYBE')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "exit_code"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_exit_code_allows_source_diagnostics_that_quote_failure_words(self) -> None:
        code = (
            "print('243 | source diagnostic says receive failed'); "
            "print('5 | source diagnostic mentions error handling')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "exit_code"})])
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_check_contract_requires_explicit_pass_status_line(self) -> None:
        code = "print('documentation mentions PASS without a result')"
        contract = {"type": "check"}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("exactly one explicit PASS", self.summary()["cases"][0]["result"])

    def test_check_positive_findings_cannot_pass(self) -> None:
        result = self.invoke(
            fixture_manifest(
                [
                    fixture_case(
                        code="print('PASS (7 findings)')",
                        contract={"type": "check"},
                    )
                ]
            )
        )
        self.assertEqual(result.returncode, 1)

    def test_check_partial_or_zero_execution_claim_cannot_pass(self) -> None:
        for index, line in enumerate(("PASS partial execution", "PASS zero cases", "PASS no tests ran")):
            with self.subTest(line=line):
                self.output_path = self.work / f"partial-check-{index}"
                result = self.invoke(
                    fixture_manifest(
                        [fixture_case(code=f"print({line!r})", contract={"type": "check"})]
                    )
                )
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_check_traceback_conflicts_with_pass(self) -> None:
        code = "print('PASS compliance regression guard'); print('Traceback (most recent call last):'); print('ValueError: boom')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_check_incomplete_evidence_conflicts_with_pass(self) -> None:
        code = "print('PASS compliance regression guard'); print('INCOMPLETE: 1 of 2 checks')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_check_fatal_evidence_conflicts_with_pass(self) -> None:
        code = "print('PASS compliance regression guard'); print('FATAL: aborted')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_check_unknown_status_record_conflicts_with_pass(self) -> None:
        code = "print('PASS compliance regression guard'); print('STATUS: MAYBE')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_check_pass_record_must_be_terminal(self) -> None:
        code = "print('integrity check: PASS (0 findings)'); print('starting second phase')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "check"})])
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("terminal non-empty", self.summary()["cases"][0]["result"])

    def test_negated_named_check_pass_cannot_pass(self) -> None:
        contract = {"type": "check"}
        result = self.invoke(
            fixture_manifest([fixture_case(code="print('NOT check: PASS')", contract=contract)])
        )
        self.assertEqual(result.returncode, 2)

    def test_negated_result_label_cannot_pass(self) -> None:
        contract = {"type": "check"}
        result = self.invoke(
            fixture_manifest(
                [fixture_case(code="print('NOT result: PASS (0 findings)')", contract=contract)]
            )
        )
        self.assertEqual(result.returncode, 2)

    def test_error_plural_label_cannot_pass(self) -> None:
        contract = {"type": "check"}
        result = self.invoke(
            fixture_manifest([fixture_case(code="print('ERRORS: PASS')", contract=contract)])
        )
        self.assertEqual(result.returncode, 1)

    def test_skipping_label_cannot_pass(self) -> None:
        contract = {"type": "check"}
        result = self.invoke(
            fixture_manifest([fixture_case(code="print('SKIPPING: PASS')", contract=contract)])
        )
        self.assertEqual(result.returncode, 1)

    def test_unknown_case_status_is_infrastructure_error(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case(code="print('CASE_RESULT: MAYBE')")]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("unknown CASE_RESULT status", self.summary()["cases"][0]["result"])

    def test_extra_malformed_case_result_cannot_pass(self) -> None:
        code = "print('CASE_RESULT: PASS'); print('CASE_RESULT: PASS!')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("malformed CASE_RESULT", self.summary()["cases"][0]["result"])

    def test_extra_unknown_case_result_cannot_pass(self) -> None:
        code = "print('CASE_RESULT: PASS'); print('CASE_RESULT: MAYBE extra')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("malformed CASE_RESULT", self.summary()["cases"][0]["result"])

    def test_prefixed_case_result_cannot_pass(self) -> None:
        code = "print('CASE_RESULT: PASS'); print('prefix CASE_RESULT: PASS')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("malformed CASE_RESULT", self.summary()["cases"][0]["result"])

    def test_case_result_record_must_be_terminal(self) -> None:
        code = "print('CASE_RESULT: PASS'); print('starting second phase')"
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("terminal non-empty", self.summary()["cases"][0]["result"])

    def test_official_image_missing_is_infrastructure_error(self) -> None:
        case = canonical_official_case()
        case["required_files"] = [
            {
                "environment": "RV_TESTSUITE_IMG",
                "fallback": str(self.work / "missing-rv.img"),
            }
        ]
        manifest = {
            "schema_version": 1,
            "profiles": {
                "official": {
                    "arch_policy": "one",
                    "include": [],
                    "cases": [],
                    "arch_cases": {"rv": ["official.rv"], "la": []},
                }
            },
            "cases": [case],
        }
        environment = os.environ.copy()
        environment.pop("RV_TESTSUITE_IMG", None)
        environment.pop("TESTSUITE_DIR", None)
        self.write_manifest(manifest)
        result = subprocess.run(
            [
                sys.executable,
                str(RUNNER),
                "--manifest",
                str(self.manifest_path),
                "--profile",
                "official",
                "--arch",
                "rv",
                "--output-dir",
                str(self.output_path),
            ],
            cwd=ROOT,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("required file", self.summary()["cases"][0]["result"])

    def test_official_contract_requires_expected_group_plan(self) -> None:
        case = fixture_case(contract={"type": "official"})
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("expected_group_labels is required", result.stderr)

    def test_official_specialized_groups_require_exact_case_count_plan(self) -> None:
        contract = {
            "type": "official",
            "expected_group_labels": ["busybox-musl"],
        }
        case = fixture_case(contract=contract)
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("must exactly cover LTP/busybox/libctest groups", result.stderr)

    def test_official_case_count_plan_requires_positive_integers(self) -> None:
        contract = {
            "type": "official",
            "expected_group_labels": ["busybox-musl"],
            "expected_group_case_counts": {"busybox-musl": 0},
        }
        case = fixture_case(contract=contract)
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("must be a positive integer", result.stderr)

    def test_unknown_profile_is_rejected(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case()]), extra=["--profile", "unknown"])
        self.assertEqual(result.returncode, 2)
        self.assertIn("unknown profile", result.stderr)

    def test_invalid_architecture_is_rejected(self) -> None:
        case = canonical_official_case()
        manifest = {
            "schema_version": 1,
            "profiles": {
                "official": {
                    "arch_policy": "one",
                    "include": [],
                    "cases": [],
                    "arch_cases": {"rv": ["official.rv"], "la": []},
                }
            },
            "cases": [case],
        }
        self.write_manifest(manifest)
        result = subprocess.run(
            [sys.executable, str(RUNNER), "--manifest", str(self.manifest_path), "--profile", "official", "--arch", "mips"],
            cwd=ROOT,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("requires --arch rv or --arch la", result.stderr)

    def test_official_profile_rejects_coordinated_plan_shrink(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        case = next(case for case in manifest["cases"] if case["id"] == "official.riscv64")
        case["result_contract"] = {
            "type": "official",
            "expected_group_labels": ["ltp-musl", "ltp-glibc"],
            "expected_group_case_counts": {"ltp-musl": 1, "ltp-glibc": 1},
        }
        case["environment"]["OSCOMP_TEST_GROUPS"] = "ltp"
        case["environment"]["LTP_CASES"] = "access01"
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("canonical ordered 24-group official plan", result.stderr)

    def test_canonical_quick_include_graph_cannot_shrink(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        manifest["profiles"]["quick"]["include"] = ["checks"]
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile quick does not match the canonical", result.stderr)

    def test_canonical_baseline_include_graph_cannot_shrink(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        manifest["profiles"]["baseline"]["include"] = []
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile baseline does not match the canonical", result.stderr)

    def test_canonical_full_include_graph_cannot_shrink(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        manifest["profiles"]["full"]["include"] = []
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile full does not match the canonical", result.stderr)

    def test_canonical_case_inventory_cannot_be_parked_in_extra_profile(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        parked = manifest["profiles"]["checks"]["cases"].pop()
        manifest["profiles"]["holding"] = {
            "description": "attempted reachability parking",
            "arch_policy": "none",
            "include": [],
            "cases": [parked],
            "arch_cases": {},
        }
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("canonical manifest profiles must be exactly", result.stderr)

    def test_canonical_check_command_cannot_be_replaced_by_fake_pass(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        case = next(case for case in manifest["cases"] if case["id"] == "check.test_asset_integrity")
        case["command"] = [
            "{python}",
            "-c",
            "print('test asset integrity check: PASS (0 findings)')",
            case["required_paths"][0],
        ]
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("exact canonical Python implementation", result.stderr)

    def test_canonical_python_environment_cannot_disable_assertions(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        case = next(case for case in manifest["cases"] if case["id"] == "unit.no_fake_success")
        case["environment"] = {"PYTHONOPTIMIZE": "1"}
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("exact canonical Python implementation", result.stderr)

    def test_canonical_unit_command_cannot_be_replaced_by_fake_summary(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        case = next(case for case in manifest["cases"] if case["id"] == "unit.suite_runner")
        expected = case["result_contract"]["expected_tests"]
        case["command"] = [
            "{python}",
            "-c",
            f"import sys; print('Ran {expected} tests in 0.1s', file=sys.stderr); print('OK', file=sys.stderr)",
            case["required_paths"][0],
        ]
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("exact canonical Python implementation", result.stderr)

    def test_canonical_unittest_expected_count_cannot_be_lowered(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        case = next(case for case in manifest["cases"] if case["id"] == "unit.no_fake_success")
        case["result_contract"]["expected_tests"] = 1
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("must preserve 10 canonical unittest methods", result.stderr)

    def test_canonical_baseline_command_cannot_be_replaced_by_true(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        case = next(case for case in manifest["cases"] if case["id"] == "baseline.submission_build")
        case["command"] = ["true"]
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("exact canonical baseline command", result.stderr)

    def test_canonical_baseline_reference_cannot_be_retargeted(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        manifest["baseline_ref"] = "HEAD"
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("baseline_ref must be origin/main", result.stderr)

    def test_full_profile_cannot_use_a_noncanonical_official_case(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        template = next(case for case in manifest["cases"] if case["id"] == "official.riscv64")
        alternate = json.loads(json.dumps(template))
        alternate["id"] = "official.partial"
        manifest["cases"].append(alternate)
        manifest["profiles"]["full"]["arch_cases"]["rv"] = [alternate["id"]]
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("canonical manifest case inventory mismatch", result.stderr)

    def test_json_report_totals_are_consistent(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case("fixture.one"), fixture_case("fixture.two")]))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        summary = self.summary()
        self.assertEqual(summary["totals"]["PASS"], 2)
        self.assertEqual(sum(summary["totals"][name] for name in ("PASS", "FAIL", "TIMEOUT", "CRASH", "INFRA_ERROR", "NOT_RUN")), 2)
        self.assertEqual(summary["planned_count"], summary["executed_count"])

    def test_json_report_records_invocation_and_suite_duration(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case()]))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        summary = self.summary()
        self.assertEqual(summary["invocation"][0], sys.executable)
        self.assertIn("--profile", summary["invocation"])
        self.assertIsInstance(summary["duration_seconds"], float)
        self.assertGreaterEqual(summary["duration_seconds"], 0.0)

    def test_child_environment_is_offline_by_default(self) -> None:
        code = (
            "import os; print('offline=' + os.environ.get('CARGO_NET_OFFLINE', '')); "
            "print('CASE_RESULT: PASS')"
        )
        environment = os.environ.copy()
        environment["CARGO_NET_OFFLINE"] = "false"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code)]),
            environment=environment,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        stdout = Path(self.summary()["cases"][0]["stdout_log"]).read_text(encoding="utf-8")
        self.assertIn("offline=true", stdout)

    def test_parent_python_optimize_cannot_disable_child_assertions(self) -> None:
        code = (
            "import unittest; "
            "T=type('T',(unittest.TestCase,),dict(test_assert=lambda self: exec('assert False'))); "
            "unittest.main()"
        )
        contract = {"type": "unittest", "expected_tests": 1}
        environment = os.environ.copy()
        environment["PYTHONOPTIMIZE"] = "1"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract=contract)]),
            environment=environment,
        )
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        stderr = Path(self.summary()["cases"][0]["stderr_log"]).read_text(encoding="utf-8")
        self.assertIn("AssertionError", stderr)

    def test_parent_make_dry_run_flags_are_not_inherited(self) -> None:
        makefile = self.work / "Makefile"
        marker = self.work / "recipe-executed"
        makefile.write_text(
            f"all:\n\t@touch {marker}\n",
            encoding="utf-8",
        )
        for index, variable in enumerate(("MAKEFLAGS", "MFLAGS", "GNUMAKEFLAGS")):
            with self.subTest(variable=variable):
                marker.unlink(missing_ok=True)
                self.output_path = self.work / f"make-environment-{index}"
                case = fixture_case(
                    f"fixture.make-{index}",
                    contract={"type": "exit_code"},
                )
                case["command"] = ["make", "-f", str(makefile), "all"]
                case["required_commands"] = ["make"]
                environment = os.environ.copy()
                environment[variable] = "-n"
                result = self.invoke(
                    fixture_manifest([case]),
                    environment=environment,
                )
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertTrue(marker.is_file(), f"{variable} suppressed the recipe")

    def test_full_all_preserves_each_arch_case_architecture(self) -> None:
        rv = fixture_case("architecture.rv")
        la = fixture_case("architecture.la")
        manifest = {
            "schema_version": 1,
            "baseline_ref": "origin/main",
            "profiles": {
                "full": {
                    "description": "two-architecture fixture",
                    "arch_policy": "one_or_all",
                    "include": [],
                    "cases": [],
                    "arch_cases": {"rv": [rv["id"]], "la": [la["id"]]},
                }
            },
            "cases": [rv, la],
        }
        result = self.invoke(manifest, extra=["--profile", "full", "--arch", "all"])
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        cases = self.summary()["cases"]
        self.assertEqual([case["architecture"] for case in cases], ["rv", "la"])

    def test_existing_output_directory_is_rejected(self) -> None:
        self.output_path.mkdir()
        stale = self.output_path / "stale.log"
        stale.write_text("old evidence", encoding="utf-8")
        result = self.invoke(fixture_manifest([fixture_case()]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("refusing to mix or overwrite evidence", result.stderr)
        self.assertEqual(stale.read_text(encoding="utf-8"), "old evidence")

    def test_log_paths_are_created_for_each_case(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case()]))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        case_result = self.summary()["cases"][0]
        self.assertTrue(Path(case_result["stdout_log"]).is_file())
        self.assertTrue(Path(case_result["stderr_log"]).is_file())

    def test_profile_order_is_deterministic(self) -> None:
        first = fixture_case("fixture.first")
        second = fixture_case("fixture.second")
        manifest = fixture_manifest([first, second], case_ids=["fixture.second", "fixture.first"])
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual([case["id"] for case in self.summary()["cases"]], ["fixture.second", "fixture.first"])

    def test_zero_executed_cases_cannot_pass(self) -> None:
        case = fixture_case()
        case["required_commands"] = ["orays-missing-requirement"]
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        summary = self.summary()
        self.assertEqual(summary["executed_count"], 0)
        self.assertNotEqual(summary["result"], "PASS")

    def test_fail_fast_after_infrastructure_error_returns_two(self) -> None:
        first = fixture_case("fixture.missing")
        first["required_commands"] = ["orays-missing-requirement"]
        second = fixture_case("fixture.after")
        result = self.invoke(fixture_manifest([first, second]), extra=["--fail-fast"])
        self.assertEqual(result.returncode, 2)
        self.assertEqual(self.summary()["totals"]["NOT_RUN"], 1)

    def test_unittest_zero_tests_cannot_pass(self) -> None:
        code = "import sys; print('Ran 0 tests in 0.001s', file=sys.stderr); print('OK', file=sys.stderr)"
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)

    def test_unittest_skip_cannot_pass(self) -> None:
        code = "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); print('OK (skipped=1)', file=sys.stderr)"
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 1)
        self.assertEqual(self.summary()["cases"][0]["status"], "FAIL")

    def test_unittest_expected_count_mismatch_is_error(self) -> None:
        code = "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); print('OK', file=sys.stderr)"
        contract = {"type": "unittest", "expected_tests": 2}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("count mismatch", self.summary()["cases"][0]["result"])

    def test_unittest_exact_count_and_plain_ok_pass(self) -> None:
        code = "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); print('OK', file=sys.stderr)"
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_unittest_malformed_duration_cannot_pass(self) -> None:
        code = "import sys; print('Ran 1 test in bananas', file=sys.stderr); print('OK', file=sys.stderr)"
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("malformed summary", self.summary()["cases"][0]["result"])

    def test_unittest_trailing_summary_garbage_cannot_pass(self) -> None:
        code = "import sys; print('Ran 1 test in 0.001s trailing garbage', file=sys.stderr); print('OK', file=sys.stderr)"
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("malformed summary", self.summary()["cases"][0]["result"])

    def test_unittest_ok_before_summary_cannot_pass(self) -> None:
        code = "import sys; print('OK', file=sys.stderr); print('Ran 1 test in 0.001s', file=sys.stderr)"
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("summary must precede", self.summary()["cases"][0]["result"])

    def test_unittest_extra_malformed_summary_cannot_pass(self) -> None:
        code = (
            "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); "
            "print('Ran 99 tests in bananas', file=sys.stderr); print('OK', file=sys.stderr)"
        )
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)

    def test_unittest_polluted_ok_record_cannot_pass(self) -> None:
        code = (
            "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); "
            "print('OKAY', file=sys.stderr); print('OK', file=sys.stderr)"
        )
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)

    def test_unittest_negated_ok_record_cannot_pass(self) -> None:
        code = (
            "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); "
            "print('NOT OK', file=sys.stderr); print('OK', file=sys.stderr)"
        )
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)

    def test_unittest_traceback_conflicts_with_ok(self) -> None:
        code = (
            "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); "
            "print('Traceback (most recent call last):', file=sys.stderr); "
            "print('ValueError: boom', file=sys.stderr); print('OK', file=sys.stderr)"
        )
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 1)

    def test_unittest_crash_evidence_conflicts_with_ok(self) -> None:
        code = (
            "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); "
            "print('Segmentation fault (core dumped)', file=sys.stderr); "
            "print('OK', file=sys.stderr)"
        )
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 1)

    def test_unittest_unknown_status_record_cannot_pass(self) -> None:
        code = (
            "import sys; print('Ran 1 test in 0.001s', file=sys.stderr); "
            "print('STATUS: MAYBE', file=sys.stderr); print('OK', file=sys.stderr)"
        )
        contract = {"type": "unittest", "expected_tests": 1}
        result = self.invoke(fixture_manifest([fixture_case(code=code, contract=contract)]))
        self.assertEqual(result.returncode, 2)

    def test_cargo_test_complete_nonempty_blocks_pass(self) -> None:
        code = (
            "print('running 1 test'); "
            "print('test demo ... ok'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s'); "
            "print('running 0 tests'); "
            "print('test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_cargo_test_zero_aggregate_cannot_pass(self) -> None:
        code = (
            "print('running 0 tests'); "
            "print('test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_cargo_test_ignored_case_cannot_pass(self) -> None:
        code = (
            "print('running 1 test'); "
            "print('test demo ... ignored'); "
            "print('test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_cargo_test_case_records_cannot_be_redistributed_across_blocks(self) -> None:
        code = (
            "print('running 1 test'); "
            "print('test first ... ok'); print('test stolen ... ok'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s'); "
            "print('running 1 test'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_cargo_test_duplicate_case_identity_cannot_satisfy_count(self) -> None:
        code = (
            "print('running 2 tests'); "
            "print('test same ... ok'); print('test same ... ok'); "
            "print('test result: ok. 2 passed; 0 failed; 0 ignored; "
            "0 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("repeats case identity", self.summary()["cases"][0]["result"])

    def test_cargo_test_measured_count_cannot_be_reported_as_normal_pass(self) -> None:
        code = (
            "print('running 1 test'); print('test demo ... ok'); "
            "print('test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_cargo_test_unaccounted_stdout_after_final_summary_is_error(self) -> None:
        code = (
            "print('running 1 test'); print('test demo ... ok'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s'); "
            "print('beginning unreported second phase')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_cargo_test_accepts_trusted_build_diagnostics_on_stderr(self) -> None:
        code = (
            "import sys; print('warning: demo warning', file=sys.stderr); "
            "print('  --> demo.rs:1:1', file=sys.stderr); "
            "print('    Finished `test` profile in 0.01s', file=sys.stderr); "
            "print('running 1 test'); print('test demo ... ok'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_cargo_test_unaccounted_or_nonpass_stderr_cannot_pass(self) -> None:
        for index, stderr_line in enumerate(
            (
                "starting second phase",
                "panic: fatal",
                "error: test failed",
                "Segmentation fault (core dumped)",
                "TFAIL mismatch",
                "SKIPPED case",
                "Downloading fixture v1.0.0",
                "Downloaded fixture v1.0.0",
            )
        ):
            with self.subTest(stderr=stderr_line):
                self.output_path = self.work / f"cargo-stderr-{index}"
                code = (
                    "import sys; "
                    f"print({stderr_line!r}, file=sys.stderr); "
                    "print('running 1 test'); print('test demo ... ok'); "
                    "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')"
                )
                result = self.invoke(
                    fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
                )
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_cargo_test_planned_executed_mismatch_cannot_pass(self) -> None:
        code = (
            "print('running 2 tests'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_cargo_test_intermediate_failure_conflicts_with_ok_summary(self) -> None:
        code = (
            "print('running 1 test'); "
            "print('test demo ... FAILED'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_cargo_test_intermediate_ignored_conflicts_with_ok_summary(self) -> None:
        code = (
            "print('running 1 test'); "
            "print('test demo ... ignored'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 1)

    def test_cargo_test_malformed_duration_cannot_pass(self) -> None:
        code = (
            "print('running 1 test'); "
            "print('test demo ... ok'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in bananas')"
        )
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "cargo_test"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_checks_profile_contract_cannot_be_downgraded(self) -> None:
        case = fixture_case("fixture.silent", code="pass", contract={"type": "exit_code"})
        manifest = fixture_manifest([case])
        manifest["profiles"] = {"checks": manifest["profiles"]["quick"]}
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile checks requires the check result contract", result.stderr)

    def test_unit_profile_contract_cannot_be_downgraded(self) -> None:
        case = fixture_case("fixture.zero", code="pass", contract={"type": "exit_code"})
        manifest = fixture_manifest([case])
        manifest["profiles"] = {"unit": manifest["profiles"]["quick"]}
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile unit requires the unittest result contract", result.stderr)

    def test_official_profile_contract_cannot_be_downgraded(self) -> None:
        case = fixture_case("fixture.empty", code="pass", contract={"type": "exit_code"})
        manifest = fixture_manifest([case], case_ids=[])
        manifest["profiles"] = {
            "official": {
                "description": "strict official fixture",
                "arch_policy": "one",
                "include": [],
                "cases": [],
                "arch_cases": {"rv": [case["id"]], "la": []},
            }
        }
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile official requires the official result contract", result.stderr)

    def test_unknown_result_contract_is_rejected(self) -> None:
        case = fixture_case()
        case["result_contract"] = {"type": "wishful"}
        result = self.invoke(fixture_manifest([case]))
        self.assertEqual(result.returncode, 2)
        self.assertIn("unsupported", result.stderr)

    def test_list_validates_without_executing(self) -> None:
        manifest = fixture_manifest([fixture_case()])
        self.write_manifest(manifest)
        result = subprocess.run(
            [sys.executable, str(RUNNER), "--manifest", str(self.manifest_path), "--list"],
            cwd=self.work,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("fixture.pass", result.stdout)
        self.assertFalse(self.output_path.exists())


if __name__ == "__main__":
    unittest.main()
