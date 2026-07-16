#!/usr/bin/env python3
"""Integration and unit tests for the manifest-driven local suite runner."""

from __future__ import annotations

import ast
import json
import os
import py_compile
import shutil
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
UNITTEST_HARNESS = ROOT / "test/run_unittest_suite.py"
sys.path.insert(0, str(ROOT / "test"))
import run_suite as runner_implementation


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
            "fixture": {
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
            "fixture",
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
        manifest["profiles"]["base"] = manifest["profiles"]["fixture"]
        manifest["profiles"]["fixture"] = {
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

        old_infrastructure_markers = [
            "unknown target triple 'loongarch64-unknown-none'",
            "libclang error; possible causes include:",
            "Host vs. target architecture mismatch",
        ]
        capability_id = "clang.target.loongarch64-unknown-none"
        fake_bin = self.work / "fake-clang-bin"
        fake_bin.mkdir()
        fake_clang = fake_bin / "clang"
        probe_environment = {**os.environ, "PATH": str(fake_bin)}
        sentinel = self.work / "main-command-ran"

        def write_fake_clang(source: str) -> None:
            fake_clang.write_text("#!/bin/sh\n" + source, encoding="utf-8")
            fake_clang.chmod(0o755)

        def capability_case(*, code: str | None = None) -> dict[str, Any]:
            selected = fixture_case(
                code=code
                or (
                    "import pathlib; "
                    f"pathlib.Path({str(sentinel)!r}).write_text('ran'); "
                    "print('primary command completed')"
                ),
                contract={"type": "exit_code"},
            )
            selected["required_commands"] = ["clang"]
            selected["required_capabilities"] = [capability_id]
            return selected

        write_fake_clang(
            "printf 'probe stdout\\n'\n"
            "printf 'probe stderr\\n' >&2\n"
            "exit 0\n"
        )
        self.output_path = self.work / "capability-available"
        available_result = self.invoke(
            fixture_manifest([capability_case()]),
            environment=probe_environment,
        )
        self.assertEqual(
            available_result.returncode,
            0,
            available_result.stdout + available_result.stderr,
        )
        available_record = self.summary()["cases"][0]
        self.assertTrue(available_record["executed"])
        self.assertTrue(available_record["details"]["capability_executed"])
        available_probe = available_record["details"]["capability_probes"][0]
        self.assertEqual(available_probe["status"], "PASS")
        self.assertEqual(available_probe["return_code"], 0)
        self.assertEqual(Path(available_probe["stdout_log"]).read_text(), "probe stdout\n")
        self.assertEqual(Path(available_probe["stderr_log"]).read_text(), "probe stderr\n")
        self.assertTrue(sentinel.exists())

        self.output_path = self.work / "main-failure-after-capability"
        marker_failure_case = capability_case(
            code=(
                "import sys; "
                f"print({old_infrastructure_markers[0]!r}, file=sys.stderr); "
                f"print({old_infrastructure_markers[1]!r}, file=sys.stderr); "
                f"print({old_infrastructure_markers[2]!r}, file=sys.stderr); "
                "print('error: real production regression', file=sys.stderr); "
                "raise SystemExit(2)"
            )
        )
        marker_failure_result = self.invoke(
            fixture_manifest([marker_failure_case]),
            environment=probe_environment,
        )
        self.assertEqual(
            marker_failure_result.returncode,
            1,
            marker_failure_result.stdout + marker_failure_result.stderr,
        )
        marker_failure_record = self.summary()["cases"][0]
        self.assertEqual(marker_failure_record["status"], "FAIL")
        self.assertEqual(
            marker_failure_record["details"]["capability_probes"][0]["status"],
            "PASS",
        )

        sentinel.unlink()
        write_fake_clang(
            "printf 'probe unavailable stdout\\n'\n"
            "printf 'probe unavailable stderr\\n' >&2\n"
            "exit 7\n"
        )
        self.output_path = self.work / "capability-unavailable"
        unavailable_result = self.invoke(
            fixture_manifest([capability_case()]),
            environment=probe_environment,
        )
        self.assertEqual(
            unavailable_result.returncode,
            2,
            unavailable_result.stdout + unavailable_result.stderr,
        )
        unavailable_record = self.summary()["cases"][0]
        self.assertEqual(unavailable_record["status"], "INFRA_ERROR")
        self.assertFalse(unavailable_record["executed"])
        unavailable_probe = unavailable_record["details"]["capability_probes"][0]
        self.assertEqual(unavailable_probe["return_code"], 7)
        self.assertEqual(
            Path(unavailable_probe["stderr_log"]).read_text(),
            "probe unavailable stderr\n",
        )
        self.assertFalse(sentinel.exists())

        unknown_case = capability_case()
        unknown_case["required_capabilities"] = ["clang.target.unknown"]
        self.output_path = self.work / "unknown-capability"
        unknown_result = self.invoke(fixture_manifest([unknown_case]))
        self.assertEqual(unknown_result.returncode, 2)
        self.assertIn("unsupported capability IDs", unknown_result.stderr)

        duplicate_case = capability_case()
        duplicate_case["required_capabilities"] = [capability_id, capability_id]
        self.output_path = self.work / "duplicate-capability"
        duplicate_result = self.invoke(fixture_manifest([duplicate_case]))
        self.assertEqual(duplicate_result.returncode, 2)
        self.assertIn("contains duplicates", duplicate_result.stderr)

        missing_required_command_case = capability_case()
        missing_required_command_case["required_commands"] = []
        self.output_path = self.work / "capability-command-not-declared"
        missing_required_command_result = self.invoke(
            fixture_manifest([missing_required_command_case])
        )
        self.assertEqual(missing_required_command_result.returncode, 2)
        self.assertIn("in required_commands", missing_required_command_result.stderr)

        malformed_capability_case = capability_case()
        malformed_capability_case["required_capabilities"] = capability_id
        self.output_path = self.work / "malformed-capability-list"
        malformed_capability_result = self.invoke(
            fixture_manifest([malformed_capability_case])
        )
        self.assertEqual(malformed_capability_result.returncode, 2)
        self.assertIn("required_capabilities must be list", malformed_capability_result.stderr)

        fake_clang.unlink()
        self.output_path = self.work / "capability-command-missing"
        missing_command_result = self.invoke(
            fixture_manifest([capability_case()]),
            environment=probe_environment,
        )
        self.assertEqual(missing_command_result.returncode, 2)
        missing_command_record = self.summary()["cases"][0]
        self.assertEqual(missing_command_record["status"], "INFRA_ERROR")
        self.assertFalse(missing_command_record["executed"])
        self.assertIn("required command not found", missing_command_record["result"])
        self.assertFalse(sentinel.exists())

        write_fake_clang("kill -TERM $$\n")
        self.output_path = self.work / "capability-signal"
        signal_result = self.invoke(
            fixture_manifest([capability_case()]),
            environment=probe_environment,
        )
        self.assertEqual(signal_result.returncode, 2, signal_result.stdout + signal_result.stderr)
        signal_record = self.summary()["cases"][0]
        self.assertFalse(signal_record["executed"])
        signal_probe = signal_record["details"]["capability_probes"][0]
        self.assertEqual(signal_probe["status"], "CRASH")
        self.assertEqual(signal_probe["signal"], signal.SIGTERM)
        self.assertFalse(sentinel.exists())

        escaped_probe_pid = self.work / "escaped-capability-probe.pid"
        write_fake_clang(
            f"/usr/bin/setsid /bin/sleep 60 &\nprintf '%s' \"$!\" > {str(escaped_probe_pid)!r}\nwait\n"
        )
        self.output_path = self.work / "capability-timeout"
        timeout_result = self.invoke(
            fixture_manifest([capability_case()]),
            environment=probe_environment,
        )
        self.assertEqual(
            timeout_result.returncode,
            2,
            timeout_result.stdout + timeout_result.stderr,
        )
        timeout_record = self.summary()["cases"][0]
        self.assertFalse(timeout_record["executed"])
        timeout_probe = timeout_record["details"]["capability_probes"][0]
        self.assertEqual(timeout_probe["status"], "TIMEOUT")
        self.assertTrue(escaped_probe_pid.is_file())
        escaped_pid = int(escaped_probe_pid.read_text(encoding="utf-8"))
        deadline = time.monotonic() + 3
        while time.monotonic() < deadline and self.process_is_live(escaped_pid):
            time.sleep(0.05)
        self.assertFalse(
            self.process_is_live(escaped_pid),
            f"capability probe descendant {escaped_pid} survived timeout cleanup",
        )
        self.assertFalse(sentinel.exists())

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

        direct_case = fixture_case(
            code="print('primary command must not run')",
            contract={"type": "exit_code"},
        )
        original_snapshot = runner_implementation._proc_snapshot
        try:
            runner_implementation._proc_snapshot = lambda: {}
            unavailable_record = runner_implementation.run_case(
                direct_case,
                repo=ROOT,
                output_dir=self.work / "unavailable-proc-snapshot",
                arch=None,
            )
            self.assertEqual(unavailable_record["status"], "INFRA_ERROR")
            self.assertFalse(unavailable_record["executed"])
            self.assertIn("process snapshot", unavailable_record["result"])

            runner_implementation._proc_snapshot = lambda: {
                os.getpid(): (os.getppid() + 1, os.getpgrp())
            }
            partial_record = runner_implementation.run_case(
                direct_case,
                repo=ROOT,
                output_dir=self.work / "partial-proc-snapshot",
                arch=None,
            )
            self.assertEqual(partial_record["status"], "INFRA_ERROR")
            self.assertFalse(partial_record["executed"])
            self.assertIn("does not accurately describe", partial_record["result"])
        finally:
            runner_implementation._proc_snapshot = original_snapshot

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
                        "fixture",
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
        code = (
            "import os; "
            "os.write(1, b'\\x1b[H\\x1b[J\\x1b[32mCASE_RESULT: PASS\\x1b[0m\\n')"
        )
        result = self.invoke(fixture_manifest([fixture_case(code=code)]))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_valid_ansi_sgr_cannot_split_and_hide_failure(self) -> None:
        code = (
            "import os; "
            "os.write(1, b'F\\x1b[H\\x1b[J\\x1b[31mAIL\\x1b[0m\\nCASE_RESULT: PASS\\n')"
        )
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
        for index, line in enumerate(
            (
                "TFAIL: hidden failure",
                "suite: TFAIL hidden",
                "build panic: kernel died",
                "prefix TIMEOUT",
                "prefix HANG detected",
                "worker ABORTED unexpectedly",
                "prefix FATAL error",
                "phase INCOMPLETE after 1 of 2 steps",
                "probe STATUS: UNKNOWN after execution",
                "suite: UNKNOWN",
                "guest reached a FATAL TRAP during shutdown",
                "worker terminated by SIGNAL 9",
                "prefix hang detected",
                "worker aborted unexpectedly",
                "prefix fatal error",
                "phase incomplete after 1 of 2 steps",
                "probe tfail hidden",
                "suite: unknown",
                "operation FAILED",
                "prefix ERROR occurred",
                "test failed as expected",
                "worker aborted as expected",
                "warning: TFAIL hidden failure",
                "warning: TIMEOUT occurred",
                "warning: ENOSYS result",
                "warning: test SKIPPED",
                "warning: STATUS: UNKNOWN",
                "Running TFAIL hidden failure",
                "Finished with ERROR",
                "Building failed",
                "Checking TIMEOUT",
                "Compiling panic failure",
                "warning: kernel panic occurred",
                "warning: worker aborted unexpectedly",
                "warning: process killed by signal",
                "warning: test failed unexpectedly",
                "warning: suite failure detected",
                "warning: case ERROR occurred",
                "warning: build failed unexpectedly",
                "warning: operation FAILED",
                "command exited with status 1",
                "process exit code: 2",
                "return code 127",
                "non-zero exit status",
                "exited unsuccessfully",
                "command was unsuccessful",
            )
        ):
            with self.subTest(line=line):
                self.output_path = self.work / f"exit-hard-failure-{index}"
                result = self.invoke(
                    fixture_manifest(
                        [
                            fixture_case(
                                code=f"print({line!r})",
                                contract={"type": "exit_code"},
                            )
                        ]
                    )
                )
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)

        self.output_path = self.work / "exit-empty-rejected"
        empty_result = self.invoke(
            fixture_manifest(
                [fixture_case(code="pass", contract={"type": "exit_code"})]
            )
        )
        self.assertEqual(empty_result.returncode, 2, empty_result.stdout + empty_result.stderr)
        self.assertEqual(self.summary()["cases"][0]["status"], "INFRA_ERROR")

        self.output_path = self.work / "exit-empty-explicitly-allowed"
        allowed_empty = self.invoke(
            fixture_manifest(
                [
                    fixture_case(
                        code="pass",
                        contract={"type": "exit_code", "allow_empty_output": True},
                    )
                ]
            )
        )
        self.assertEqual(allowed_empty.returncode, 0, allowed_empty.stdout + allowed_empty.stderr)

    def test_exit_code_zero_rejects_unknown_status(self) -> None:
        code = "print('STATUS: MAYBE')"
        result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract={"type": "exit_code"})])
        )
        self.assertEqual(result.returncode, 2)

    def test_exit_code_allows_source_diagnostics_that_quote_failure_words(self) -> None:
        code = (
            "print('243 | source diagnostic says receive failed'); "
            "print('5 | source diagnostic mentions error handling'); "
            "print('308 | sys_epoll_wait(epfd, maxevents, timeout)'); "
            "print('309 | let timeout = request.timeout'); "
            "print('5408 ~ && let Some(timeout) = file.object().next_timeout()'); "
            "print('command was aborted? no'); "
            "print('error count: 0'); "
            "print('failed attempts: 0'); "
            "print('0 failures'); "
            "print('none failed'); "
            "print('10 passed; 0 failed'); "
            "print('0 tests failed'); "
            "print('operation did not fail'); "
            "print('warning: error handling path was checked'); "
            "print('warning: panic message contains unused formatting placeholders'); "
            "print('warning: function abort is deprecated'); "
            "print('   Compiling proc-macro-error-attr2 v2.0.0'); "
            "print('     Running tests/error-path.rs (target/debug/error-path)'); "
            "print('completed without errors'); "
            "print('completed without any failures'); "
            "print('error-free completion'); "
            "print('failure-free completion'); "
            "print('command exited with status 0'); "
            "print('process exit code: 0'); "
            "print('return code 0')"
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
        canonical_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        canonical_case = next(
            case
            for case in canonical_manifest["cases"]
            if case["id"] == "official.riscv64"
        )
        case = canonical_official_case("official.fixture-missing-image")
        case["required_files"] = canonical_case["required_files"]
        environment = os.environ.copy()
        environment["RV_TESTSUITE_IMG"] = str(self.work / "missing-rv.img")
        result = self.invoke(
            fixture_manifest([case]),
            environment=environment,
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("required file", self.summary()["cases"][0]["result"])

    def test_noncanonical_official_scouting_configuration_cannot_pass(self) -> None:
        pass_transcript = (
            "#### OS COMP TEST GROUP START demo-musl ####\n"
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
            "#### OS COMP TEST GROUP END demo-musl ####"
        )
        fail_transcript = (
            "#### OS COMP TEST GROUP START demo-musl ####\n"
            "FAIL OFFICIAL TEST GROUP demo-musl : 7\n"
            "#### OS COMP TEST GROUP END demo-musl ####"
        )
        contract = {
            "type": "official",
            "expected_group_labels": ["demo-musl"],
            "expected_group_case_counts": {},
        }
        base_environment = os.environ.copy()
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
            base_environment.pop(name, None)

        scenarios = (
            ("blacklist-pass", "LTP_BLACKLIST", "no-such-case", pass_transcript, 2, "INFRA_ERROR"),
            (
                "skip-pass",
                "OSCOMP_SKIP_TEST_GROUPS",
                "demo-musl",
                pass_transcript,
                2,
                "INFRA_ERROR",
            ),
            ("blacklist-fail", "LTP_BLACKLIST", "no-such-case", fail_transcript, 1, "FAIL"),
        )
        for label, variable, value, transcript, expected_exit, expected_status in scenarios:
            with self.subTest(label=label):
                self.output_path = self.work / f"official-scouting-{label}"
                environment = base_environment.copy()
                environment[variable] = value
                case = fixture_case(
                    "fixture.official-result",
                    code=f"print({transcript!r})",
                    contract=contract,
                )
                result = self.invoke(
                    fixture_manifest([case]),
                    environment=environment,
                )
                self.assertEqual(expected_exit, result.returncode, result.stdout + result.stderr)
                record = self.summary()["cases"][0]
                self.assertEqual(expected_status, record["status"], record)
                self.assertEqual(
                    [variable],
                    record["details"]["noncanonical_official_environment"],
                )
                self.assertNotEqual("PASS", record["status"], record)

    def test_official_nonzero_exit_still_requires_structural_validation(self) -> None:
        contract = {
            "type": "official",
            "expected_group_labels": ["demo-musl"],
            "expected_group_case_counts": {},
        }
        pass_transcript = (
            "#### OS COMP TEST GROUP START demo-musl ####\n"
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
            "#### OS COMP TEST GROUP END demo-musl ####"
        )
        fail_transcript = (
            "#### OS COMP TEST GROUP START demo-musl ####\n"
            "FAIL OFFICIAL TEST GROUP demo-musl : 7\n"
            "#### OS COMP TEST GROUP END demo-musl ####"
        )
        scenarios = (
            (
                "truncated",
                "#### OS COMP TEST GROUP START demo-musl ####",
                2,
                "INFRA_ERROR",
                "ERROR",
            ),
            (
                "malformed",
                "NOT #### OS COMP TEST GROUP START demo-musl ####",
                2,
                "INFRA_ERROR",
                "ERROR",
            ),
            ("semantic-failure", fail_transcript, 1, "FAIL", "FAIL"),
            ("pass-exit-conflict", pass_transcript, 2, "INFRA_ERROR", "PASS"),
        )
        for label, transcript, runner_exit, status, parser_status in scenarios:
            with self.subTest(label=label):
                self.output_path = self.work / f"official-nonzero-{label}"
                case = fixture_case(
                    f"fixture.official-nonzero-{label}",
                    code=(
                        "import sys; "
                        f"print({transcript!r}); "
                        "raise SystemExit(1)"
                    ),
                    contract=contract,
                )
                result = self.invoke(fixture_manifest([case]))
                self.assertEqual(
                    runner_exit,
                    result.returncode,
                    result.stdout + result.stderr,
                )
                record = self.summary()["cases"][0]
                self.assertEqual(status, record["status"], record)
                self.assertEqual(1, record["return_code"], record)
                self.assertEqual(1, record["details"]["process_exit_code"], record)
                self.assertEqual(parser_status, record["details"]["status"], record)
                if parser_status == "ERROR":
                    self.assertGreater(record["details"]["error_count"], 0, record)

        self.output_path = self.work / "official-nonzero-invalid-utf8"
        invalid_utf8 = fixture_case(
            "fixture.official-nonzero-invalid-utf8",
            code=(
                "import sys; "
                "sys.stdout.buffer.write(bytes([255])); "
                "sys.stdout.flush(); "
                "raise SystemExit(1)"
            ),
            contract=contract,
        )
        invalid_utf8_result = self.invoke(fixture_manifest([invalid_utf8]))
        self.assertEqual(
            2,
            invalid_utf8_result.returncode,
            invalid_utf8_result.stdout + invalid_utf8_result.stderr,
        )
        invalid_utf8_record = self.summary()["cases"][0]
        self.assertEqual("INFRA_ERROR", invalid_utf8_record["status"], invalid_utf8_record)
        self.assertEqual(1, invalid_utf8_record["return_code"], invalid_utf8_record)
        self.assertEqual(
            1,
            invalid_utf8_record["details"]["process_exit_code"],
            invalid_utf8_record,
        )
        self.assertIn(
            "not valid UTF-8",
            invalid_utf8_record["details"]["output_integrity_error"],
        )

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

    def test_official_busybox_semantic_failure_and_replay_keep_distinct_statuses(self) -> None:
        busybox_cases, _libctest_cases = runner_implementation.trusted_official_case_plan(
            ROOT
        )

        def frame(case: Any, status: str = "success") -> str:
            return "\n".join(
                (
                    f"#### OS COMP BUSYBOX CASE START ordinal={case.ordinal} ####",
                    f"BUSYBOX CASE RESULT ordinal={case.ordinal} "
                    f"status={status} command={case.command}",
                    f"testcase busybox {case.command} {status}",
                    f"#### OS COMP BUSYBOX CASE END ordinal={case.ordinal} ####",
                )
            )

        contract = {
            "type": "official",
            "expected_group_labels": ["busybox-musl"],
            "expected_group_case_counts": {"busybox-musl": len(busybox_cases)},
        }
        case = fixture_case("fixture.official-busybox", contract=contract)
        semantic_body = "\n".join(
            frame(item, "fail" if item.ordinal == 1 else "success")
            for item in busybox_cases
        )
        semantic_stdout = (
            "#### OS COMP TEST GROUP START busybox-musl ####\n"
            f"{semantic_body}\n"
            "#### OS COMP TEST GROUP END busybox-musl ####\n"
        )
        status, _result, details = runner_implementation.parse_contract(
            case, semantic_stdout, ""
        )
        self.assertEqual(status, "FAIL", details)
        self.assertEqual(details["error_count"], 0, details)
        self.assertEqual(details["planned_case_count"], len(busybox_cases))
        self.assertEqual(details["executed_case_count"], len(busybox_cases))
        self.assertEqual(details["completed_case_count"], len(busybox_cases))

        replay_body = "\n".join(
            [frame(busybox_cases[0]), *(frame(item) for item in busybox_cases)]
        )
        replay_stdout = (
            "#### OS COMP TEST GROUP START busybox-musl ####\n"
            f"{replay_body}\n"
            "#### OS COMP TEST GROUP END busybox-musl ####\n"
        )
        replay_status, _replay_result, replay_details = (
            runner_implementation.parse_contract(case, replay_stdout, "")
        )
        self.assertEqual(replay_status, "INFRA_ERROR", replay_details)
        self.assertIn(
            "busybox-duplicate-identity",
            {finding["kind"] for finding in replay_details["errors"]},
        )

    def test_unknown_profile_is_rejected(self) -> None:
        result = self.invoke(fixture_manifest([fixture_case()]), extra=["--profile", "unknown"])
        self.assertEqual(result.returncode, 2)
        self.assertIn("unknown profile", result.stderr)

    def test_invalid_architecture_is_rejected(self) -> None:
        rv = fixture_case("architecture.rv")
        la = fixture_case("architecture.la")
        manifest = fixture_manifest([rv, la], case_ids=[])
        manifest["profiles"]["fixture"].update(
            {
                "arch_policy": "one",
                "arch_cases": {"rv": [rv["id"]], "la": [la["id"]]},
            }
        )
        result = self.invoke(manifest, extra=["--arch", "mips"])
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile fixture requires --arch rv or --arch la", result.stderr)

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

    def test_canonical_official_execution_and_preflight_fields_are_locked(self) -> None:
        canonical_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        for case_id, expected_fallback in (
            ("official.riscv64", "{repo}/../sdcard-rv.img"),
            ("official.loongarch64", "{repo}/../sdcard-la.img"),
        ):
            case = next(
                case for case in canonical_manifest["cases"] if case["id"] == case_id
            )
            self.assertEqual(case["required_files"][0]["fallback"], expected_fallback)

        mutations = (
            ("timeout_seconds", 1),
            ("required_paths", []),
            ("required_commands", []),
            ("required_files", []),
            ("infrastructure_exit_codes", [2]),
        )
        for index, (field, value) in enumerate(mutations):
            with self.subTest(field=field):
                self.output_path = self.work / f"official-field-{index}"
                manifest = json.loads(
                    (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
                )
                case = next(
                    case
                    for case in manifest["cases"]
                    if case["id"] == "official.riscv64"
                )
                case[field] = value
                result = self.invoke(manifest)
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertIn(f"canonical official {field}", result.stderr)

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
        self.assertIn("must preserve and identity-bind 10", result.stderr)

        dependency_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        reporter_case = next(
            case
            for case in dependency_manifest["cases"]
            if case["id"] == "unit.evaluation_failure_report"
        )
        reporter_case["required_paths"].pop()
        dependency_result = self.invoke(dependency_manifest)
        self.assertEqual(dependency_result.returncode, 2)
        self.assertIn("exact canonical Python implementation", dependency_result.stderr)

        valid_source = (
            "import unittest\n"
            "class T(unittest.TestCase):\n"
            "    def test_assertion(self):\n"
            "        self.assertTrue(True)\n"
            "if __name__ == '__main__':\n"
            "    unittest.main()\n"
        )
        self.assertEqual(
            runner_implementation.canonical_unittest_method_count(
                ast.parse(valid_source),
                Path("valid.py"),
            ),
            1,
        )
        invalid_sources = (
            (
                valid_source.replace("def test_assertion", "async def test_assertion"),
                "async test method",
            ),
            (
                valid_source.replace(
                    "        self.assertTrue(True)",
                    "        yield self.fail('never executed')",
                ),
                "generator test method",
            ),
            (
                valid_source.replace(
                    "    def test_assertion",
                    "    def testFailure(self):\n"
                    "        self.fail('default unittest discovery must not be ignored')\n"
                    "    def test_assertion",
                ),
                "discoverable test names without the test_ prefix",
            ),
            (
                valid_source.replace(
                    "class T(unittest.TestCase):",
                    "class T(unittest.TestCase):\n"
                    "    def setUp(self):\n"
                    "        yield self.fail('generator setup body must execute')",
                ),
                "lifecycle hook setUp must execute synchronously",
            ),
            (
                valid_source.replace(
                    "    def test_assertion",
                    "    async def tearDown(self):\n"
                    "        self.fail('async teardown body must execute')\n"
                    "    def test_assertion",
                ),
                "lifecycle hook tearDown must execute synchronously",
            ),
            (
                valid_source.replace(
                    "    def test_assertion",
                    "    @staticmethod\n    def test_assertion",
                ),
                "decorated test method",
            ),
            (
                valid_source.replace(
                    "class T(unittest.TestCase):",
                    "@staticmethod\nclass T(unittest.TestCase):",
                ),
                "undecorated direct unittest.TestCase",
            ),
            (
                valid_source.replace(
                    "class T(unittest.TestCase):",
                    "class Mixin:\n    pass\nclass T(Mixin, unittest.TestCase):",
                ),
                "without mixins",
            ),
            (
                valid_source.replace(
                    "    def test_assertion",
                    "    def _callTestMethod(self, method):\n"
                    "        return None\n"
                    "    def test_assertion",
                ),
                "overrides unittest execution hooks",
            ),
            (
                valid_source.replace(
                    "if __name__ == '__main__':",
                    "def load_tests(loader, tests, pattern):\n"
                    "    return tests\n"
                    "if __name__ == '__main__':",
                ),
                "must not bind load_tests",
            ),
            (
                valid_source.replace(
                    "if __name__ == '__main__':",
                    "if True:\n"
                    "    def load_tests(loader, tests, pattern):\n"
                    "        return unittest.TestSuite([unittest.FunctionTestCase(lambda: None)])\n"
                    "if __name__ == '__main__':",
                ),
                "must not bind load_tests",
            ),
            (
                "import unittest\n"
                "def test_not_discovered():\n"
                "    raise AssertionError\n"
                "if __name__ == '__main__':\n"
                "    unittest.main()\n",
                "outside a direct unittest.TestCase",
            ),
            (
                valid_source.replace("unittest.main()", "unittest.main(defaultTest='T')"),
                "plain unittest",
            ),
        )
        for source, message in invalid_sources:
            with self.subTest(canonical_unittest_bypass=message):
                with self.assertRaisesRegex(runner_implementation.ManifestError, message):
                    runner_implementation.canonical_unittest_method_count(
                        ast.parse(source),
                        Path("mutated.py"),
                    )

        harness_source = self.work / "harness-valid.py"
        harness_source.write_text(valid_source, encoding="utf-8")
        harness_command = [
            sys.executable,
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
            str(UNITTEST_HARNESS),
        ]
        harness_result = subprocess.run(
            [*harness_command, str(harness_source)],
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertEqual(harness_result.returncode, 0, harness_result.stderr)
        binding_line = (
            "UNITTEST_BINDING: planned=1 started=1 executed=1 stopped=1"
        )
        harness_lines = harness_result.stderr.splitlines()
        self.assertEqual(harness_lines.count(binding_line), 1)
        self.assertIn(".", harness_lines)

        positive_semantics_sources = (
            valid_source.replace(
                "        self.assertTrue(True)",
                "        def values():\n"
                "            yield 1\n"
                "        self.assertEqual(list(values()), [1])",
            ),
            valid_source.replace(
                "    def test_assertion(self):",
                "    def test_assertion(self):\n"
                "        \"documented test\"",
            ).replace(
                "        self.assertTrue(True)",
                "        self.assertEqual(self.shortDescription(), 'documented test')",
            ),
            valid_source.replace(
                "        self.assertTrue(True)",
                "        self.assertEqual(__name__, '__main__')\n"
                "        self.assertEqual(self.__class__.__module__, '__main__')",
            ),
            valid_source.replace(
                "import unittest\n",
                "import unittest\nimport os\n",
            ).replace(
                "        self.assertTrue(True)",
                "        self.assertEqual(os.environ.get('PYTHONDONTWRITEBYTECODE'), '1')\n"
                "        self.assertEqual(os.environ.get('PYTHONPYCACHEPREFIX'), '/dev/null')",
            ),
        )
        for index, positive_source in enumerate(positive_semantics_sources):
            with self.subTest(native_semantics_positive=index):
                path = self.work / f"harness-positive-{index}.py"
                path.write_text(positive_source, encoding="utf-8")
                positive_result = subprocess.run(
                    [*harness_command, str(path)],
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    timeout=15,
                    check=False,
                )
                self.assertEqual(
                    positive_result.returncode,
                    0,
                    positive_result.stdout + positive_result.stderr,
                )

        nested_module_dir = self.work / "nested-bytecode"
        nested_module_dir.mkdir()
        nested_module = nested_module_dir / "bytecode_probe.py"
        nested_module.write_text("VALUE = 'pyc'\n", encoding="utf-8")
        source_stat = nested_module.stat()
        nested_cache_dir = nested_module_dir / "__pycache__"
        nested_cache_dir.mkdir()
        nested_pyc = nested_cache_dir / (
            f"bytecode_probe.{sys.implementation.cache_tag}.pyc"
        )
        py_compile.compile(
            str(nested_module),
            cfile=str(nested_pyc),
            doraise=True,
        )
        nested_module.write_text("VALUE = 'src'\n", encoding="utf-8")
        os.utime(
            nested_module,
            ns=(source_stat.st_atime_ns, source_stat.st_mtime_ns),
        )
        poisoned_pyc = nested_pyc.read_bytes()
        control_environment = os.environ.copy()
        control_environment.pop("PYTHONDONTWRITEBYTECODE", None)
        control_environment.pop("PYTHONPYCACHEPREFIX", None)
        control_result = subprocess.run(
            [sys.executable, "-c", "import bytecode_probe; print(bytecode_probe.VALUE)"],
            cwd=nested_module_dir,
            env=control_environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertEqual(control_result.returncode, 0, control_result.stderr)
        self.assertEqual(control_result.stdout.strip(), "pyc")
        bytecode_source = valid_source.replace(
            "import unittest\n",
            "import unittest\nimport subprocess\nimport sys\n",
        ).replace(
            "        self.assertTrue(True)",
            "        result = subprocess.run(\n"
            "            [sys.executable, '-c', "
            "'import bytecode_probe, sys; print(bytecode_probe.VALUE); print(sys.pycache_prefix)'],\n"
            f"            cwd={str(nested_module_dir)!r}, text=True, capture_output=True, check=False,\n"
            "        )\n"
            "        self.assertEqual(result.returncode, 0, result.stderr)\n"
            "        self.assertEqual(result.stdout.splitlines(), ['src', '/dev/null'])",
        )
        bytecode_fixture = self.work / "harness-bytecode.py"
        bytecode_fixture.write_text(bytecode_source, encoding="utf-8")
        bytecode_result = subprocess.run(
            [*harness_command, str(bytecode_fixture)],
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertEqual(
            bytecode_result.returncode,
            0,
            bytecode_result.stdout + bytecode_result.stderr,
        )
        self.assertEqual(nested_pyc.read_bytes(), poisoned_pyc)
        self.assertEqual(list(nested_pyc.parent.iterdir()), [nested_pyc])

        failing_source = valid_source.replace(
            "self.assertTrue(True)", "self.fail('original test body must execute')"
        )
        failing_path = self.work / "harness-failing.py"
        failing_path.write_text(failing_source, encoding="utf-8")
        failing_result = subprocess.run(
            [*harness_command, str(failing_path)],
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertEqual(failing_result.returncode, 1, failing_result.stderr)
        self.assertIn(
            "non-success outcome events are not complete success",
            failing_result.stderr,
        )
        self.assertNotIn(
            "skip/expected-failure outcomes are not complete success",
            failing_result.stderr,
        )
        runtime_bypasses = (
            failing_source.replace(
                "if __name__ == '__main__':",
                "T.test_assertion = lambda self: None\nif __name__ == '__main__':",
            ),
            failing_source.replace(
                "    def test_assertion",
                "    def __getattribute__(self, name):\n"
                "        if name.startswith('test_'):\n"
                "            return lambda: None\n"
                "        return super().__getattribute__(name)\n"
                "    def test_assertion",
            ),
            failing_source.replace(
                "if __name__ == '__main__':",
                "unittest.main = lambda: print('Ran 1 test in 0.001s\\nOK')\n"
                "if __name__ == '__main__':",
            ),
            failing_source.replace(
                "if __name__ == '__main__':",
                "def __getattr__(name):\n"
                "    if name == 'load_tests':\n"
                "        return lambda *args: unittest.TestSuite([unittest.FunctionTestCase(lambda: None)])\n"
                "    raise AttributeError(name)\n"
                "if __name__ == '__main__':",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "def setUpModule():\n"
                "    raise RuntimeError('module setup must execute')\n"
                "class T(unittest.TestCase):",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "def tearDownModule():\n"
                "    raise RuntimeError('module teardown must execute')\n"
                "class T(unittest.TestCase):",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "def setUpModule():\n"
                "    def cleanup():\n"
                "        raise RuntimeError('module cleanup must execute')\n"
                "    unittest.addModuleCleanup(cleanup)\n"
                "class T(unittest.TestCase):",
            ),
            valid_source.replace(
                "if __name__ == '__main__':",
                "Alias = T\nif __name__ == '__main__':",
            ).replace(
                "        self.assertTrue(True)",
                "        count = getattr(T, 'count', 0) + 1\n"
                "        T.count = count\n"
                "        self.assertEqual(count, 1)",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "class RunOnly(unittest.TestCase):\n"
                "    def runTest(self):\n"
                "        self.fail('runTest must not be omitted')\n"
                "class T(unittest.TestCase):",
            ),
            valid_source.replace(
                "if __name__ == '__main__':",
                "class Hidden(unittest.TestCase):\n"
                "    pass\n"
                "def hidden_failure(self):\n"
                "    self.fail('dynamic test must not be omitted')\n"
                "Hidden.test_failure = hidden_failure\n"
                "if __name__ == '__main__':",
            ),
            valid_source.replace(
                "if __name__ == '__main__':",
                "def dynamic_loader(loader, tests, pattern):\n"
                "    return unittest.TestSuite([unittest.FunctionTestCase(\n"
                "        lambda: (_ for _ in ()).throw(AssertionError('dynamic failure'))\n"
                "    )])\n"
                "globals()['load_tests'] = dynamic_loader\n"
                "if __name__ == '__main__':",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "class T(unittest.TestCase):\n"
                "    def setUp(self):\n"
                "        self.addCleanup(self.lazy_cleanup)\n"
                "    def lazy_cleanup(self):\n"
                "        yield self.fail('generator cleanup body must execute')",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "class T(unittest.TestCase):\n"
                "    @classmethod\n"
                "    def setUpClass(cls):\n"
                "        cls.addClassCleanup(cls.lazy_cleanup)\n"
                "    @classmethod\n"
                "    def lazy_cleanup(cls):\n"
                "        yield cls.fail('generator class cleanup body must execute')",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "def lazy_module_cleanup():\n"
                "    yield AssertionError('generator module cleanup body must execute')\n"
                "def setUpModule():\n"
                "    unittest.addModuleCleanup(lazy_module_cleanup)\n"
                "class T(unittest.TestCase):",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "register_module_cleanup = unittest.addModuleCleanup\n"
                "def lazy_aliased_module_cleanup():\n"
                "    yield AssertionError('aliased module cleanup body must execute')\n"
                "class T(unittest.TestCase):",
            ).replace(
                "        self.assertTrue(True)",
                "        register_module_cleanup(lazy_aliased_module_cleanup)\n"
                "        self.assertTrue(True)",
            ),
            valid_source.replace(
                "if __name__ == '__main__':",
                "register_class_cleanup = T.addClassCleanup\n"
                "def lazy_aliased_class_cleanup():\n"
                "    yield AssertionError('aliased class cleanup body must execute')\n"
                "if __name__ == '__main__':",
            ).replace(
                "        self.assertTrue(True)",
                "        register_class_cleanup(lazy_aliased_class_cleanup)\n"
                "        self.assertTrue(True)",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "def lazy_case_module_cleanup():\n"
                "    yield AssertionError('unittest.case module cleanup body must execute')\n"
                "class T(unittest.TestCase):",
            ).replace(
                "        self.assertTrue(True)",
                "        unittest.case.addModuleCleanup(lazy_case_module_cleanup)\n"
                "        self.assertTrue(True)",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "class T(unittest.TestCase):\n"
                "    def setUp(self):\n"
                "        self.addCleanup(self.lazy_cleanup)\n"
                "    async def lazy_cleanup(self):\n"
                "        self.fail('async cleanup body must execute')",
            ),
            failing_source.replace(
                "        self.fail('original test body must execute')",
                "        self.addCleanup(lambda: self._outcome.errors.clear())\n"
                "        self.fail('outcome mutation must not hide failure')",
            ),
            valid_source.replace(
                "        self.assertTrue(True)",
                "        raise unittest.case._ShouldStop()",
            ),
            valid_source.replace(
                "class T(unittest.TestCase):",
                "class T(unittest.TestCase):\n"
                "    marker = False\n"
                "    def test_z_prime(self):\n"
                "        T.marker = True\n"
                "    def test_a_observe(self):\n"
                "        self.assertTrue(T.marker)\n"
                "class Ignored(unittest.TestCase):",
            ).replace(
                "class Ignored(unittest.TestCase):\n"
                "    def test_assertion(self):",
                "class Ignored(unittest.TestCase):\n"
                "    def test_assertion(self):",
            ),
        )
        for index, malicious_source in enumerate(runtime_bypasses):
            with self.subTest(runtime_identity_bypass=index):
                path = self.work / f"harness-bypass-{index}.py"
                path.write_text(malicious_source, encoding="utf-8")
                bypass_result = subprocess.run(
                    [*harness_command, str(path)],
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    timeout=15,
                    check=False,
                )
                self.assertNotEqual(
                    bypass_result.returncode,
                    0,
                    bypass_result.stdout + bypass_result.stderr,
                )
                self.assertNotIn(
                    "UNITTEST_BINDING: planned=1 started=1 executed=1 stopped=1",
                    bypass_result.stderr,
                )

        import_fd_source = valid_source.replace(
            "import unittest\n",
            "import unittest\nimport os\nos.write(2, b'IMPORT-FD-MARKER\\n')\n",
        )
        import_fd_path = self.work / "harness-import-fd.py"
        import_fd_path.write_text(import_fd_source, encoding="utf-8")
        import_fd_result = subprocess.run(
            [*harness_command, str(import_fd_path)],
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=15,
            check=False,
        )
        self.assertNotEqual(import_fd_result.returncode, 0)
        self.assertIn("IMPORT-FD-MARKER", import_fd_result.stderr)
        self.assertIn("side-effect free", import_fd_result.stderr)

        relay_pid_path = self.work / "harness-signal-relay.pids"
        signal_source = valid_source.replace(
            "import unittest\n",
            "import unittest\nimport os\nimport pathlib\nimport signal\nimport sys\n",
        ).replace(
            "        self.assertTrue(True)",
            "        print('stdout-before-signal', flush=True)\n"
            "        print('stderr-before-signal', file=sys.stderr, flush=True)\n"
            "        os.write(1, b'fd-stdout-before-signal\\n')\n"
            "        os.write(2, b'fd-stderr-before-signal\\n')\n"
            "        relay_children = pathlib.Path(\n"
            "            f'/proc/{os.getpid()}/task/{os.getpid()}/children'\n"
            "        ).read_text(encoding='utf-8')\n"
            f"        pathlib.Path({str(relay_pid_path)!r}).write_text(\n"
            "            relay_children, encoding='utf-8'\n"
            "        )\n"
            "        os.kill(os.getpid(), signal.SIGTERM)",
        )
        signal_path = self.work / "harness-signal.py"
        signal_path.write_text(signal_source, encoding="utf-8")
        signal_case = fixture_case(
            "fixture.signal_harness",
            contract={
                "type": "unittest",
                "expected_tests": 1,
                "identity_binding": True,
            },
        )
        signal_case["command"] = [*harness_command, str(signal_path)]
        self.output_path = self.work / "harness-signal-output"
        signal_result = self.invoke(fixture_manifest([signal_case]))
        self.assertEqual(
            signal_result.returncode,
            2,
            signal_result.stdout + signal_result.stderr,
        )
        signal_summary = self.summary()
        self.assertEqual(signal_summary["result"], "INFRA_ERROR")
        self.assertEqual(
            (
                signal_summary["planned_count"],
                signal_summary["executed_count"],
                signal_summary["completed_count"],
            ),
            (1, 1, 1),
        )
        self.assertEqual(signal_summary["totals"]["INFRA_ERROR"], 1)
        self.assertEqual(signal_summary["totals"]["PASS"], 0)
        signal_record = signal_summary["cases"][0]
        self.assertTrue(signal_record["executed"])
        self.assertEqual(signal_record["status"], "INFRA_ERROR")
        self.assertEqual(signal_record["return_code"], -signal.SIGTERM)
        self.assertIn("descendant processes", signal_record["result"])
        self.assertEqual(signal_record["details"]["surviving_descendant_count"], 2)
        signal_stdout = Path(signal_record["stdout_log"]).read_text(encoding="utf-8")
        signal_stderr = Path(signal_record["stderr_log"]).read_text(encoding="utf-8")
        self.assertIn("stdout-before-signal", signal_stdout)
        self.assertIn("stderr-before-signal", signal_stderr)
        self.assertIn("fd-stdout-before-signal", signal_stdout)
        self.assertIn("fd-stderr-before-signal", signal_stderr)
        self.assertNotIn("UNITTEST_BINDING:", signal_stdout + signal_stderr)
        relay_pids = [int(pid) for pid in relay_pid_path.read_text().split()]
        self.assertEqual(len(relay_pids), 2)
        for relay_pid in relay_pids:
            self.assertFalse(Path(f"/proc/{relay_pid}").exists())

    def test_canonical_baseline_command_cannot_be_replaced_by_true(self) -> None:
        manifest = json.loads((ROOT / "test/suite_manifest.json").read_text(encoding="utf-8"))
        case = next(case for case in manifest["cases"] if case["id"] == "baseline.submission_build")
        case["command"] = ["true"]
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("exact canonical baseline command", result.stderr)

        capability_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        capability_case = next(
            case
            for case in capability_manifest["cases"]
            if case["id"] == "baseline.clippy_loongarch64"
        )
        capability_case["required_capabilities"] = []
        capability_result = self.invoke(capability_manifest)
        self.assertEqual(capability_result.returncode, 2)
        self.assertIn("capability requirements", capability_result.stderr)

        required_command_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        required_command_case = next(
            case
            for case in required_command_manifest["cases"]
            if case["id"] == "baseline.clippy_loongarch64"
        )
        required_command_case["required_commands"].remove("clang")
        self.output_path = self.work / "canonical-capability-command-mutation"
        required_command_result = self.invoke(required_command_manifest)
        self.assertEqual(required_command_result.returncode, 2)
        self.assertIn("in required_commands", required_command_result.stderr)

        result_contract_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        result_contract_case = next(
            case
            for case in result_contract_manifest["cases"]
            if case["id"] == "baseline.kernel_riscv64"
        )
        result_contract_case["result_contract"]["allow_empty_output"] = True
        self.output_path = self.work / "canonical-baseline-contract-mutation"
        result_contract_result = self.invoke(result_contract_manifest)
        self.assertEqual(result_contract_result.returncode, 2)
        self.assertIn("result/capability requirements", result_contract_result.stderr)

        infrastructure_code_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        infrastructure_code_case = next(
            case
            for case in infrastructure_code_manifest["cases"]
            if case["id"] == "baseline.kernel_riscv64"
        )
        infrastructure_code_case["infrastructure_exit_codes"] = [1]
        self.output_path = self.work / "canonical-baseline-infra-code-mutation"
        infrastructure_code_result = self.invoke(infrastructure_code_manifest)
        self.assertEqual(infrastructure_code_result.returncode, 2)
        self.assertIn("result/capability requirements", infrastructure_code_result.stderr)

        required_inputs_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        required_inputs_case = next(
            case
            for case in required_inputs_manifest["cases"]
            if case["id"] == "baseline.kernel_riscv64"
        )
        required_inputs_case["required_commands"].append("orays-nonexistent-poison")
        self.output_path = self.work / "canonical-baseline-required-command-mutation"
        required_inputs_result = self.invoke(required_inputs_manifest)
        self.assertEqual(required_inputs_result.returncode, 2)
        self.assertIn("result/capability requirements", required_inputs_result.stderr)

        required_path_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        required_path_case = next(
            case
            for case in required_path_manifest["cases"]
            if case["id"] == "baseline.kernel_riscv64"
        )
        required_path_case["required_paths"] = ["{repo}/test/README.md"]
        self.output_path = self.work / "canonical-baseline-required-path-mutation"
        required_path_result = self.invoke(required_path_manifest)
        self.assertEqual(required_path_result.returncode, 2)
        self.assertIn("result/capability requirements", required_path_result.stderr)

        timeout_manifest = json.loads(
            (ROOT / "test/suite_manifest.json").read_text(encoding="utf-8")
        )
        timeout_case = next(
            case
            for case in timeout_manifest["cases"]
            if case["id"] == "baseline.kernel_riscv64"
        )
        timeout_case["timeout_seconds"] = 1
        self.output_path = self.work / "canonical-baseline-timeout-mutation"
        timeout_result = self.invoke(timeout_manifest)
        self.assertEqual(timeout_result.returncode, 2)
        self.assertIn("result/capability requirements", timeout_result.stderr)

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
        self.assertIsInstance(summary["runner_dirty"], bool)
        self.assertIsInstance(summary["runner_status"], list)
        self.assertEqual(summary["runner_commit_final"], summary["runner_commit"])
        self.assertEqual(summary["runner_dirty_final"], summary["runner_dirty"])
        self.assertEqual(summary["runner_status_final"], summary["runner_status"])
        self.assertTrue(summary["runner_provenance_stable"])
        self.assertEqual(
            summary["python_runtime"],
            {
                "isolated": True,
                "no_site": True,
                "dont_write_bytecode": True,
                "pycache_prefix": "/dev/null",
            },
        )

        for label, startup_flags in (
            ("missing-no-site-and-prefix", ["-I", "-B"]),
            ("missing-prefix", ["-I", "-S", "-B"]),
            (
                "wrong-prefix",
                ["-I", "-S", "-B", "-X", f"pycache_prefix={self.work / 'wrong-cache'}"],
            ),
        ):
            with self.subTest(startup_isolation=label):
                output_path = self.work / f"startup-{label}"
                startup_result = subprocess.run(
                    [
                        sys.executable,
                        *startup_flags,
                        str(RUNNER),
                        "--manifest",
                        str(self.manifest_path),
                        "--profile",
                        "fixture",
                        "--output-dir",
                        str(output_path),
                    ],
                    cwd=ROOT,
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    timeout=15,
                    check=False,
                )
                self.assertEqual(
                    startup_result.returncode,
                    0,
                    startup_result.stdout + startup_result.stderr,
                )
                startup_summary = json.loads(
                    (output_path / "summary.json").read_text(encoding="utf-8")
                )
                self.assertEqual(startup_summary["python_runtime"], summary["python_runtime"])
                self.assertTrue(startup_summary["runner_provenance_stable"])

        probe_repo = self.work / "git-provenance"
        probe_repo.mkdir()
        probe_environment = runner_implementation.git_probe_environment(probe_repo)
        git_command = shutil.which("git", path=probe_environment["PATH"])
        self.assertIsNotNone(git_command)
        resolved_git = str(Path(git_command).resolve())
        for command in (
            [resolved_git, "init", "--quiet"],
            [resolved_git, "config", "user.name", "Suite Runner Test"],
            [resolved_git, "config", "user.email", "suite-runner@example.invalid"],
        ):
            subprocess.run(
                command,
                cwd=probe_repo,
                env=probe_environment,
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
        tracked = probe_repo / "tracked.txt"
        tracked.write_text("clean\n", encoding="utf-8")
        for command in (
            [resolved_git, "add", "tracked.txt"],
            [resolved_git, "commit", "--quiet", "-m", "fixture"],
        ):
            subprocess.run(
                command,
                cwd=probe_repo,
                env=probe_environment,
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
        self.assertEqual(runner_implementation.git_worktree_status(probe_repo), "")
        initial_probe_commit = runner_implementation.baseline_commit(probe_repo, "HEAD")
        (probe_repo / "untracked.txt").write_text("dirty\n", encoding="utf-8")
        self.assertIn(
            "?? untracked.txt",
            runner_implementation.git_worktree_status(probe_repo) or "",
        )
        self.assertEqual(
            runner_implementation.baseline_commit(probe_repo, "HEAD"),
            initial_probe_commit,
        )
        self.assertNotEqual(runner_implementation.git_worktree_status(probe_repo), "")

        clean_git_environment = {
            name: value
            for name, value in os.environ.items()
            if not name.startswith("GIT_")
        }
        expected_baseline = subprocess.run(
            ["git", "-C", str(ROOT), "rev-parse", "--verify", "origin/main^{commit}"],
            env=clean_git_environment,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        expected_runner = subprocess.run(
            ["git", "-C", str(ROOT), "rev-parse", "--verify", "HEAD^{commit}"],
            env=clean_git_environment,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        hostile_environment = os.environ.copy()
        hostile_environment.update(
            {
                "GIT_DIR": str(self.work / "attacker.git"),
                "GIT_WORK_TREE": str(self.work / "attacker-worktree"),
                "GIT_CONFIG_GLOBAL": str(self.work / "attacker-gitconfig"),
            }
        )
        self.output_path = self.work / "git-environment-isolation"
        isolated = self.invoke(
            fixture_manifest([fixture_case()]),
            environment=hostile_environment,
        )
        self.assertEqual(isolated.returncode, 0, isolated.stdout + isolated.stderr)
        isolated_summary = self.summary()
        self.assertEqual(isolated_summary["baseline_commit"], expected_baseline)
        self.assertEqual(isolated_summary["runner_commit"], expected_runner)
        self.assertEqual(isolated_summary["runner_commit_final"], expected_runner)
        self.assertTrue(isolated_summary["runner_provenance_stable"])

    def test_child_environment_is_offline_by_default(self) -> None:
        code = (
            "import os; print('offline=' + os.environ.get('CARGO_NET_OFFLINE', '')); "
            "print('home=' + os.environ.get('HOME', '')); "
            "print('path=' + os.environ.get('PATH', '')); "
            "print('manifest=' + os.environ.get('EXPLICIT_CASE_CONTROL', '')); "
            "print('cargo_home=' + os.environ.get('CARGO_HOME', '')); "
            "print('no_bytecode=' + os.environ.get('PYTHONDONTWRITEBYTECODE', '')); "
            "print('pycache_prefix=' + os.environ.get('PYTHONPYCACHEPREFIX', '')); "
            "print('no_user_site=' + os.environ.get('PYTHONNOUSERSITE', '')); "
            "print('bash_env=' + os.environ.get('BASH_ENV', '')); "
            "print('rustfmt=' + os.environ.get('RUSTFMT', '')); "
            "print('remote_ltp=' + os.environ.get('REMOTE_LTP_CASES', '')); "
            "print('ambient=' + os.environ.get('AMBIENT_FUTURE_CONTROL', '')); "
            "print('shell_functions=' + ','.join(sorted(name for name in os.environ if name.startswith('BASH_FUNC_')))); "
            "print('CASE_RESULT: PASS')"
        )
        environment = os.environ.copy()
        environment["CARGO_NET_OFFLINE"] = "false"
        environment["BASH_ENV"] = "/tmp/untrusted-bash-env"
        environment["RUSTFMT"] = "/bin/true"
        environment["CARGO_HOME"] = "/tmp/untrusted-cargo-home"
        environment["REMOTE_LTP_CASES"] = '"; :; #'
        environment["AMBIENT_FUTURE_CONTROL"] = "must-not-propagate"
        environment["BASH_FUNC_fake%%"] = "() { return 0; }"
        case = fixture_case(code=code)
        case["environment"] = {
            "EXPLICIT_CASE_CONTROL": "manifest-owned",
            "PYTHONNOUSERSITE": "",
            "PYTHONDONTWRITEBYTECODE": "",
            "PYTHONPYCACHEPREFIX": str(self.work / "untrusted-cache"),
        }
        result = self.invoke(
            fixture_manifest([case]),
            environment=environment,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        stdout = Path(self.summary()["cases"][0]["stdout_log"]).read_text(encoding="utf-8")
        self.assertIn("offline=true", stdout)
        self.assertIn(f"home={environment.get('HOME', str(Path.home()))}\n", stdout)
        self.assertIn(f"path={environment.get('PATH', os.defpath)}\n", stdout)
        self.assertIn("manifest=manifest-owned\n", stdout)
        self.assertIn(f"cargo_home={ROOT / 'cargo-home'}\n", stdout)
        self.assertIn("no_bytecode=1\n", stdout)
        self.assertIn("pycache_prefix=/dev/null\n", stdout)
        self.assertIn("no_user_site=1\n", stdout)
        self.assertIn("bash_env=\n", stdout)
        self.assertIn("rustfmt=\n", stdout)
        self.assertIn("remote_ltp=\n", stdout)
        self.assertIn("ambient=\n", stdout)
        self.assertIn("shell_functions=\n", stdout)

        workspace_override = str(self.work / "official-workspace")
        previous_workspace = os.environ.get("ORAYS_WORKSPACE_ROOT")
        previous_testsuite = os.environ.get("TESTSUITE_DIR")
        try:
            os.environ["ORAYS_WORKSPACE_ROOT"] = workspace_override
            os.environ.pop("TESTSUITE_DIR", None)
            official_environment, error = runner_implementation.child_environment(
                canonical_official_case(), repo=ROOT, cwd=ROOT
            )
            self.assertIsNone(error)
            self.assertEqual(
                official_environment["ORAYS_WORKSPACE_ROOT"], workspace_override
            )
            self.assertEqual(official_environment["TESTSUITE_DIR"], workspace_override)
        finally:
            if previous_workspace is None:
                os.environ.pop("ORAYS_WORKSPACE_ROOT", None)
            else:
                os.environ["ORAYS_WORKSPACE_ROOT"] = previous_workspace
            if previous_testsuite is None:
                os.environ.pop("TESTSUITE_DIR", None)
            else:
                os.environ["TESTSUITE_DIR"] = previous_testsuite

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

    def test_parent_make_control_environment_is_not_inherited(self) -> None:
        makefile = self.work / "Makefile"
        middle_makefile = self.work / "Middle.mk"
        child_makefile = self.work / "Child.mk"
        marker = self.work / "recipe-executed"
        injection_marker = self.work / "makefiles-injection-loaded"
        makefiles_injection = self.work / "untrusted-injected.mk"
        makefile.write_text(
            f"all:\n\t@$(MAKE) -f {middle_makefile} middle\n",
            encoding="utf-8",
        )
        middle_makefile.write_text(
            f"middle:\n\t@$(MAKE) -f {child_makefile} child\n",
            encoding="utf-8",
        )
        child_makefile.write_text(
            f"child:\n\t@touch {marker}\n",
            encoding="utf-8",
        )
        makefiles_injection.write_text(
            f"injected := $(shell touch {injection_marker})\n",
            encoding="utf-8",
        )
        inherited_values = (
            (".SHELLFLAGS", "-n -c"),
            ("MAKE", "/bin/false"),
            ("MAKEFILES", str(makefiles_injection)),
            ("MAKEFLAGS", "-n"),
            ("MAKEOVERRIDES", "MAKE=/bin/false"),
            ("MFLAGS", "-n"),
            ("GNUMAKEFLAGS", "-n"),
        )
        for index, (variable, value) in enumerate(inherited_values):
            with self.subTest(variable=variable):
                marker.unlink(missing_ok=True)
                injection_marker.unlink(missing_ok=True)
                self.output_path = self.work / f"make-environment-{index}"
                case = fixture_case(
                    f"fixture.make-{index}",
                    contract={"type": "exit_code"},
                )
                case["command"] = ["make", "-f", str(makefile), "all"]
                case["required_commands"] = ["make"]
                environment = os.environ.copy()
                environment[variable] = value
                result = self.invoke(
                    fixture_manifest([case]),
                    environment=environment,
                )
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertTrue(marker.is_file(), f"{variable} suppressed the recipe")
                self.assertFalse(
                    injection_marker.exists(),
                    f"{variable} loaded the untrusted makefile",
                )
                resolved = self.summary()["cases"][0]["required_command_paths"]
                self.assertEqual(Path(resolved["make"]), Path(shutil.which("make") or "").resolve())

        bypass_makefile = self.work / "AmbientBypass.mk"
        bypass_marker = self.work / "ambient-bypass-recipe-continued"
        bypass_makefile.write_text(
            f"all:\n\t@false $(AMBIENT_FUTURE_CONTROL)\n\t@touch {bypass_marker}\n",
            encoding="utf-8",
        )
        self.output_path = self.work / "unknown-make-environment"
        bypass_case = fixture_case(
            "fixture.unknown-make-environment",
            contract={"type": "exit_code"},
        )
        bypass_case["command"] = ["make", "-f", str(bypass_makefile), "all"]
        bypass_case["required_commands"] = ["make"]
        environment = os.environ.copy()
        environment["AMBIENT_FUTURE_CONTROL"] = "; :; #"
        result = self.invoke(
            fixture_manifest([bypass_case]),
            environment=environment,
        )
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual(self.summary()["cases"][0]["status"], "FAIL")
        self.assertFalse(
            bypass_marker.exists(),
            "an unknown inherited Make variable converted a failing recipe into PASS",
        )

        path_expansion_marker = self.work / "ambient-path-expansion-ran"
        self.output_path = self.work / "make-path-expansion"
        environment = os.environ.copy()
        environment["PATH"] = (
            f"$(shell touch {path_expansion_marker}):"
            + environment.get("PATH", os.defpath)
        )
        result = self.invoke(
            fixture_manifest([bypass_case]),
            environment=environment,
        )
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertEqual(self.summary()["cases"][0]["status"], "INFRA_ERROR")
        self.assertIn(
            "PATH contains control or Make-expansion syntax",
            self.summary()["cases"][0]["result"],
        )
        self.assertFalse(
            path_expansion_marker.exists(),
            "Make expanded caller-controlled PATH syntax",
        )

    def test_one_or_all_profile_preserves_each_arch_case_architecture(self) -> None:
        rv = fixture_case("architecture.rv")
        la = fixture_case("architecture.la")
        manifest = {
            "schema_version": 1,
            "baseline_ref": "origin/main",
            "profiles": {
                "registered-all": {
                    "description": "two-architecture fixture",
                    "arch_policy": "one_or_all",
                    "include": [],
                    "cases": [],
                    "arch_cases": {"rv": [rv["id"]], "la": [la["id"]]},
                }
            },
            "cases": [rv, la],
        }
        result = self.invoke(manifest, extra=["--profile", "registered-all", "--arch", "all"])
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

        self.output_path = self.work / "identity-bound-output"
        bound_code = (
            "import sys; "
            "print('UNITTEST_BINDING: planned=1 started=1 executed=1 stopped=1', file=sys.stderr); "
            "print('Ran 1 test in 0.001s', file=sys.stderr); print('OK', file=sys.stderr)"
        )
        bound_contract = {
            "type": "unittest",
            "expected_tests": 1,
            "identity_binding": True,
        }
        bound_result = self.invoke(
            fixture_manifest([fixture_case(code=bound_code, contract=bound_contract)])
        )
        self.assertEqual(bound_result.returncode, 0, bound_result.stdout + bound_result.stderr)
        self.assertEqual(self.summary()["cases"][0]["details"]["executed_tests"], 1)

        self.output_path = self.work / "glued-binding-output"
        glued_code = (
            "import sys; "
            "sys.stderr.write('.UNITTEST_BINDING: planned=1 started=1 executed=1 stopped=1\\n'); "
            "print('Ran 1 test in 0.001s', file=sys.stderr); print('OK', file=sys.stderr)"
        )
        glued_result = self.invoke(
            fixture_manifest([fixture_case(code=glued_code, contract=bound_contract)])
        )
        self.assertEqual(glued_result.returncode, 2)
        self.assertIn("binding record", self.summary()["cases"][0]["result"])

        e2e_module = self.work / "identity-bound-e2e.py"
        e2e_module.write_text(
            "import unittest\n"
            "class T(unittest.TestCase):\n"
            "    def test_assertion(self):\n"
            "        self.assertTrue(True)\n"
            "if __name__ == '__main__':\n"
            "    unittest.main()\n",
            encoding="utf-8",
        )
        e2e_case = fixture_case(contract=bound_contract)
        e2e_case["command"] = [
            sys.executable,
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
            str(UNITTEST_HARNESS),
            str(e2e_module),
        ]
        self.output_path = self.work / "identity-bound-e2e-output"
        e2e_result = self.invoke(fixture_manifest([e2e_case]))
        self.assertEqual(e2e_result.returncode, 0, e2e_result.stdout + e2e_result.stderr)
        self.assertEqual(self.summary()["cases"][0]["status"], "PASS")

        duplicate_module = self.work / "identity-bound-duplicate.py"
        duplicate_module.write_text(
            "import sys\n"
            "import unittest\n"
            "class T(unittest.TestCase):\n"
            "    def test_assertion(self):\n"
            "        print('UNITTEST_BINDING: planned=1 started=1 executed=1 stopped=1', file=sys.stderr)\n"
            "        self.assertTrue(True)\n"
            "if __name__ == '__main__':\n"
            "    unittest.main()\n",
            encoding="utf-8",
        )
        duplicate_case = json.loads(json.dumps(e2e_case))
        duplicate_case["command"][-1] = str(duplicate_module)
        self.output_path = self.work / "identity-bound-duplicate-output"
        duplicate_result = self.invoke(fixture_manifest([duplicate_case]))
        self.assertEqual(duplicate_result.returncode, 2)
        self.assertIn("found 2", self.summary()["cases"][0]["result"])

        self.output_path = self.work / "missing-binding-output"
        missing_result = self.invoke(
            fixture_manifest([fixture_case(code=code, contract=bound_contract)])
        )
        self.assertEqual(missing_result.returncode, 2)
        self.assertIn("binding record", self.summary()["cases"][0]["result"])

        self.output_path = self.work / "mismatched-binding-output"
        mismatched_code = bound_code.replace("executed=1", "executed=0")
        mismatched_result = self.invoke(
            fixture_manifest([fixture_case(code=mismatched_code, contract=bound_contract)])
        )
        self.assertEqual(mismatched_result.returncode, 2)
        self.assertIn("binding count mismatch", self.summary()["cases"][0]["result"])

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

        self.output_path = self.work / "async-unittest"
        async_code = (
            "import unittest\n"
            "class T(unittest.TestCase):\n"
            "    async def test_never_runs(self):\n"
            "        self.fail('not awaited')\n"
            "unittest.main()\n"
        )
        async_result = self.invoke(
            fixture_manifest(
                [fixture_case(code=async_code, contract={"type": "unittest", "expected_tests": 1})]
            )
        )
        self.assertEqual(async_result.returncode, 2, async_result.stdout + async_result.stderr)
        self.assertIn("coroutine test", self.summary()["cases"][0]["result"])

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

        self.output_path = self.work / "cargo-identity-unittest"
        identity_unittest = (
            "import sys; "
            "print('..', file=sys.stderr); "
            "print('UNITTEST_BINDING: planned=2 started=2 executed=2 stopped=2', file=sys.stderr); "
            "print('', file=sys.stderr); print('-' * 70, file=sys.stderr); "
            "print('Ran 2 tests in 0.01s', file=sys.stderr); "
            "print('', file=sys.stderr); print('OK', file=sys.stderr); "
            "print('cargo test -p demo --no-fail-fast'); "
            "print('running 1 test'); print('test demo ... ok'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest(
                [fixture_case(code=identity_unittest, contract={"type": "cargo_test"})]
            )
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(
            self.summary()["cases"][0]["details"]["accounted_identity_unittests"],
            [2],
        )

        identity_nonpass_mutations = (
            ("started=2", "started=1"),
            (
                "print('', file=sys.stderr); print('OK', file=sys.stderr);",
                "print('', file=sys.stderr); print('TFAIL hidden', file=sys.stderr); print('OK', file=sys.stderr);",
            ),
            ("--no-fail-fast", "--no-fail-fast failure"),
        )
        for index, (old, new) in enumerate(identity_nonpass_mutations):
            with self.subTest(identity_nonpass_mutation=(old, new)):
                self.output_path = self.work / f"cargo-identity-unittest-bad-{index}"
                result = self.invoke(
                    fixture_manifest(
                        [
                            fixture_case(
                                code=identity_unittest.replace(old, new),
                                contract={"type": "cargo_test"},
                            )
                        ]
                    )
                )
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)

        self.output_path = self.work / "cargo-expected-panic"
        expected_panic = (
            "import sys; "
            "print(\"thread 'demo::rejects_invalid_input' panicked at demo.rs:1:1:\", file=sys.stderr); "
            "print('invalid input', file=sys.stderr); "
            "print('note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace', file=sys.stderr); "
            "print('running 1 test'); "
            "print('test demo::rejects_invalid_input - should panic ... ok'); "
            "print('test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s')"
        )
        result = self.invoke(
            fixture_manifest(
                [fixture_case(code=expected_panic, contract={"type": "cargo_test"})]
            )
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(
            self.summary()["cases"][0]["details"]["accounted_expected_panics"],
            ["demo::rejects_invalid_input"],
        )

        injected_nonpass = (
            ("TCONF unavailable", "before-note"),
            ("TBROK setup failed", "before-note"),
            ("TFAIL mismatch", "before-note"),
            ("ENOSYS unsupported", "before-note"),
            ("TIMEOUT expired", "before-note"),
            ("STATUS: UNKNOWN", "before-note"),
            ("error: test failed", "before-note"),
            ("panic: extra", "before-note"),
            ("panic: extra", "after-note"),
        )
        for index, (extra_stderr, position) in enumerate(injected_nonpass):
            with self.subTest(extra_stderr=extra_stderr, position=position):
                self.output_path = self.work / f"cargo-expected-panic-extra-{index}"
                anchor = (
                    "print('note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace', file=sys.stderr);"
                    if position == "before-note"
                    else "print('running 1 test');"
                )
                code_with_extra = expected_panic.replace(
                    anchor,
                    f"print({extra_stderr!r}, file=sys.stderr); {anchor}",
                )
                result = self.invoke(
                    fixture_manifest(
                        [
                            fixture_case(
                                code=code_with_extra,
                                contract={"type": "cargo_test"},
                            )
                        ]
                    )
                )
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)

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
        manifest["profiles"] = {"checks": manifest["profiles"]["fixture"]}
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile checks requires the check result contract", result.stderr)

    def test_unit_profile_contract_cannot_be_downgraded(self) -> None:
        case = fixture_case("fixture.zero", code="pass", contract={"type": "exit_code"})
        manifest = fixture_manifest([case])
        manifest["profiles"] = {"unit": manifest["profiles"]["fixture"]}
        result = self.invoke(manifest)
        self.assertEqual(result.returncode, 2)
        self.assertIn("profile unit requires the unittest result contract", result.stderr)

    def test_alternate_official_profile_cannot_bypass_canonical_contract(self) -> None:
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

        strict_case = fixture_case(
            "fixture.strict-official",
            code=(
                "print('#### OS COMP TEST GROUP START demo-musl ####\\n'"
                "      'PASS OFFICIAL TEST GROUP demo-musl : 0\\n'"
                "      '#### OS COMP TEST GROUP END demo-musl ####')"
            ),
            contract={
                "type": "official",
                "expected_group_labels": ["demo-musl"],
                "expected_group_case_counts": {},
            },
        )
        manifest = {
            "schema_version": 1,
            "baseline_ref": "origin/main",
            "profiles": {
                "official": {
                    "description": "maliciously reduced but internally complete official fixture",
                    "arch_policy": "one",
                    "include": [],
                    "cases": [],
                    "arch_cases": {"rv": [strict_case["id"]], "la": []},
                }
            },
            "cases": [strict_case],
        }
        self.output_path = self.work / "alternate-official-output"
        result = self.invoke(manifest, extra=["--profile", "official", "--arch", "rv"])
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("reserved canonical profile names", result.stderr)
        self.assertFalse(self.output_path.exists(), "alternate official evidence directory was created")

        for reserved_name, contract in (
            ("checks", {"type": "check"}),
            ("unit", {"type": "unittest", "expected_tests": 1}),
            ("quick", {"type": "case_result"}),
            ("baseline", {"type": "case_result"}),
            (
                "official",
                {
                    "type": "official",
                    "expected_group_labels": ["demo-musl"],
                    "expected_group_case_counts": {},
                },
            ),
            ("full", {"type": "case_result"}),
        ):
            with self.subTest(reserved_profile=reserved_name):
                reserved_case = fixture_case(
                    f"fixture.reserved-{reserved_name}",
                    contract=contract,
                )
                reserved_manifest = fixture_manifest([reserved_case])
                reserved_manifest["profiles"] = {
                    reserved_name: reserved_manifest["profiles"]["fixture"]
                }
                self.output_path = self.work / f"reserved-profile-{reserved_name}"
                reserved_result = self.invoke(reserved_manifest, extra=["--list"])
                self.assertEqual(
                    reserved_result.returncode,
                    2,
                    reserved_result.stdout + reserved_result.stderr,
                )
                self.assertIn("reserved canonical profile names", reserved_result.stderr)
                self.assertFalse(self.output_path.exists())

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
