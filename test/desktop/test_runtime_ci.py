from pathlib import Path
import hashlib
import json
import os
import signal
import shutil
import subprocess
import sys
import tempfile
import time
import unittest


REPO_ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = REPO_ROOT / ".github/workflows/desktop.yml"
RUNNER = REPO_ROOT / "scripts/desktop/run-headless-qemu.sh"
QMP_RUNTIME_DIR = REPO_ROOT / "scripts/desktop/create-qmp-runtime-dir.py"
METADATA_COLLECTOR = REPO_ROOT / "scripts/desktop/collect-runtime-metadata.py"
DESKTOP_APP = REPO_ROOT / "user/desktop/src/app.rs"


class DesktopRuntimeCiTests(unittest.TestCase):
    def assert_process_gone(self, process_id: int, start_time: str) -> None:
        deadline = time.monotonic() + 2.0
        while True:
            try:
                fields = Path(f"/proc/{process_id}/stat").read_text(
                    encoding="utf-8"
                ).split()
            except FileNotFoundError:
                return
            if len(fields) < 22 or fields[21] != start_time:
                return
            if time.monotonic() >= deadline:
                self.fail(
                    f"runtime child {process_id} with start time {start_time} "
                    "still exists after cleanup deadline"
                )
            time.sleep(0.02)

    def make_runner_fixture(self, root: Path) -> tuple[Path, Path, dict[str, str]]:
        repo = root / "repo"
        scripts = repo / "scripts/desktop"
        shutil.copytree(
            REPO_ROOT / "scripts/desktop",
            scripts,
            ignore=shutil.ignore_patterns("__pycache__", ".agents", ".codex"),
        )
        fixture = repo / "test/desktop/fixtures/input/boot.json"
        fixture.parent.mkdir(parents=True)
        shutil.copyfile(REPO_ROOT / "test/desktop/fixtures/input/boot.json", fixture)
        fake_bin = root / "fake-bin"
        fake_bin.mkdir()
        qemu = fake_bin / "qemu-system-riscv64"
        qemu.write_text(
            "#!/bin/sh\n"
            "if [ \"${1:-}\" = --version ]; then\n"
            "  echo 'QEMU emulator version 9.2.4'\n"
            "  exit 0\n"
            "fi\n"
            "if [ -n \"${FAKE_QEMU_PID_FILE:-}\" ]; then\n"
            "  echo $$ > \"$FAKE_QEMU_PID_FILE\"\n"
            "fi\n"
            "if [ -n \"${FAKE_QEMU_STAT_FILE:-}\" ]; then\n"
            "  cat /proc/$$/stat > \"$FAKE_QEMU_STAT_FILE\"\n"
            "fi\n"
            "if [ \"${FAKE_QEMU_MODE:-exit}\" = sleep ]; then\n"
            "  trap 'exit 143' TERM\n"
            "  trap 'exit 130' INT\n"
            "  while :; do sleep 1; done\n"
            "fi\n"
            "exit \"${FAKE_QEMU_EXIT:-0}\"\n",
            encoding="utf-8",
        )
        qemu.chmod(0o755)
        for name, body in (
            (
                "qemu-img",
                "#!/bin/sh\nfor last do :; done\n: > \"$last\"\n",
            ),
            ("mkfs.fat", "#!/bin/sh\nexit 0\n"),
        ):
            path = fake_bin / name
            path.write_text(body, encoding="utf-8")
            path.chmod(0o755)
        build = scripts / "build.sh"
        build.write_text(
            "#!/bin/sh\n"
            "if [ \"${FAKE_BUILD_EXIT:-0}\" -ne 0 ]; then\n"
            "  exit \"$FAKE_BUILD_EXIT\"\n"
            "fi\n"
            "mkdir -p \"$repo_root/build/desktop/rv/artifacts\"\n"
            "printf guest-artifact > \"$repo_root/build/desktop/rv/artifacts/orays-desktop-rv.bin\"\n",
            encoding="utf-8",
        )
        # The fake build runs from an absolute path, so bind the repository path
        # without adding a caller-controlled production override.
        build.write_text(
            build.read_text(encoding="utf-8").replace(
                "$repo_root", str(repo)
            ),
            encoding="utf-8",
        )
        build.chmod(0o755)
        qmp_helper = scripts / "create-qmp-runtime-dir.py"
        qmp_helper.write_text(
            "#!/usr/bin/env python3\n"
            "import os, pathlib, tempfile\n"
            "runtime = pathlib.Path(tempfile.mkdtemp(prefix='orays-qmp.', dir='/tmp'))\n"
            "record = os.environ.get('FAKE_QMP_RECORD')\n"
            "if record:\n"
            "    pathlib.Path(record).write_text(str(runtime), encoding='utf-8')\n"
            "print(runtime)\n",
            encoding="utf-8",
        )
        qmp_helper.chmod(0o755)
        policy = repo / "test/desktop/runtime-policy.json"
        policy.parent.mkdir(parents=True, exist_ok=True)
        policy.write_text(
            json.dumps(
                {
                    "schema": 1,
                    "qemu_version": "9.2.4",
                    "architectures": {
                        "rv": {
                            "qemu_binary": qemu.name,
                            "qemu_sha256": hashlib.sha256(qemu.read_bytes()).hexdigest(),
                            "artifact": "build/desktop/rv/artifacts/orays-desktop-rv.bin",
                            "build_invocation": ["scripts/desktop/build.sh", "rv"],
                        }
                    },
                },
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
        )
        (repo / ".gitignore").write_text(
            "build/\ntest/output/\n", encoding="utf-8"
        )
        (repo / "test/output").mkdir(parents=True)
        subprocess.run(["git", "init", "-q"], cwd=repo, check=True)
        subprocess.run(
            ["git", "config", "user.email", "desktop-test@example.invalid"],
            cwd=repo,
            check=True,
        )
        subprocess.run(
            ["git", "config", "user.name", "Desktop Test"], cwd=repo, check=True
        )
        subprocess.run(["git", "add", "."], cwd=repo, check=True)
        subprocess.run(["git", "commit", "-qm", "fixture"], cwd=repo, check=True)
        environment = os.environ.copy()
        environment["PATH"] = f"{fake_bin}:{environment['PATH']}"
        return repo, fake_bin, environment

    def invoke_fixture_runner(
        self,
        repo: Path,
        environment: dict[str, str],
        name: str,
    ) -> tuple[subprocess.CompletedProcess[str], Path]:
        output = repo / f"test/output/desktop/{name}"
        result = subprocess.run(
            [
                "bash",
                str(repo / "scripts/desktop/run-headless-qemu.sh"),
                "--arch",
                "rv",
                "--scenario",
                "boot",
                "--output",
                str(output),
            ],
            cwd=repo,
            env=environment,
            check=False,
            capture_output=True,
            text=True,
            timeout=30,
        )
        return result, output

    def test_workflow_has_no_persistent_self_hosted_runtime_job(self) -> None:
        text = WORKFLOW.read_text(encoding="utf-8")
        self.assertIn("pull_request:", text)
        self.assertNotIn("FIXED_QEMU_VERSION", text)
        self.assertNotIn("  desktop-runtime:", text)
        self.assertNotIn("self-hosted", text)

    def test_workflow_keeps_only_the_hosted_desktop_job(self) -> None:
        text = WORKFLOW.read_text(encoding="utf-8")
        jobs = text.split("jobs:\n", 1)[1]
        self.assertIn("  desktop:\n", jobs)
        self.assertIn("runs-on: ubuntu-24.04", jobs)
        self.assertEqual(jobs.count("\n  desktop"), 1)

    def test_runner_exit_trap_finalizes_pass_and_failure_evidence(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertIn("trap finalize EXIT", runner)
        self.assertIn("finalize-runtime-evidence.py", runner)
        self.assertIn("--runner-exit \"$runner_exit\"", runner)
        self.assertIn('if [[ -n "$qemu_pid" ]]; then', runner)
        self.assertNotIn('if [[ -n "$qemu_pid" ]] && kill -0', runner)
        self.assertIn('if wait "$qemu_pid"; then', runner)
        self.assertIn('if wait "$qemu_pid" 2>/dev/null; then', runner)
        self.assertIn("qemu_pid=", runner)
        self.assertNotIn("package-review-evidence.py\" --run-dir", runner)
        self.assertIn(
            "trap 'failure_stage=runner-signal; failure_reason=signal_interrupted; exit 130' INT",
            runner,
        )
        self.assertIn(
            "trap 'failure_stage=runner-signal; failure_reason=signal_terminated; exit 143' TERM",
            runner,
        )
        self.assertIn('rm -f "$qmp_runtime_dir/qmp.sock"', runner)
        self.assertIn('rmdir "$qmp_runtime_dir"', runner)

    def test_runner_uses_short_temporary_qmp_socket(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertIn("create-qmp-runtime-dir.py", runner)
        self.assertIn("qmp_runtime_dir", runner)
        self.assertIn('qmp_socket="$qmp_runtime_dir/qmp.sock"', runner)
        self.assertNotIn('qmp_socket="$run_dir/qmp.sock"', runner)
        self.assertNotIn("TMPDIR", runner)

    def test_qmp_runtime_dir_ignores_unsafe_tmpdir_and_is_qemu_safe(self) -> None:
        environment = os.environ.copy()
        with tempfile.TemporaryDirectory() as directory:
            environment["TMPDIR"] = str(Path(directory) / (("long," * 30) + "value:tail"))
            result = subprocess.run(
                [sys.executable, "-B", str(QMP_RUNTIME_DIR)],
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        runtime_dir = Path(result.stdout.strip())
        try:
            socket_path = runtime_dir / "qmp.sock"
            self.assertEqual(runtime_dir.parent, Path("/tmp"))
            self.assertNotIn(",", str(socket_path))
            self.assertNotIn(":", str(socket_path))
            self.assertLessEqual(len(os.fsencode(socket_path)), 107)
        finally:
            runtime_dir.rmdir()

    def test_runner_records_qemu_start_state_and_required_version(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertIn('required_qemu_version=9.2.4', runner)
        self.assertIn('DESKTOP_REQUIRED_QEMU_VERSION', runner)
        self.assertNotIn('${DESKTOP_REQUIRED_QEMU_VERSION:-9.2.4}', runner)
        self.assertIn(
            'if [[ "$qemu_version" != "QEMU emulator version ${required_qemu_version}" ]]',
            runner,
        )
        self.assertIn('--required-qemu-version "$required_qemu_version"', runner)
        self.assertIn('--qemu-started "$qemu_started"', runner)
        self.assertIn("failure_reason=qemu_version_mismatch", runner)

    def test_state_reset_is_handled_before_shell_dispatch(self) -> None:
        app = DESKTOP_APP.read_text(encoding="utf-8")
        reset = app.index("if matches!(event, InputEvent::StateReset)")
        shell_dispatch = app.index("self.compositor.shell_mut().handle_input(event, bounds)")
        self.assertLess(reset, shell_dispatch)

    def test_metadata_collector_rejects_noncanonical_required_version(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            run = root / "run"
            run.mkdir()
            output = root / "runtime-metadata.json"
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(METADATA_COLLECTOR),
                    "--repo-root",
                    str(REPO_ROOT),
                    "--output",
                    str(output),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                    "--qemu-binary",
                    "qemu-system-riscv64",
                    "--required-qemu-version",
                    "6.2.0",
                    "--run-dir",
                    str(run),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
            self.assertIn("must be exactly 9.2.4", result.stderr)
            self.assertFalse(output.exists())

    def test_runner_records_early_clean_and_nonzero_qemu_exit_once(self) -> None:
        for qemu_exit in (0, 7):
            with self.subTest(qemu_exit=qemu_exit), tempfile.TemporaryDirectory() as directory:
                repo, _, environment = self.make_runner_fixture(Path(directory))
                environment["FAKE_QEMU_EXIT"] = str(qemu_exit)
                result, output = self.invoke_fixture_runner(
                    repo, environment, f"qemu-exit-{qemu_exit}"
                )
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
                summary_path = output / "review-package/summary.json"
                self.assertTrue(summary_path.is_file(), result.stdout + result.stderr)
                summary = json.loads(summary_path.read_text(encoding="utf-8"))
                self.assertTrue(summary["qemu_started"])
                self.assertEqual(summary["qemu_exit"], qemu_exit)
                self.assertEqual(summary["failure_stage"], "qemu-boot")
                validation = subprocess.run(
                    [
                        sys.executable,
                        "-B",
                        str(repo / "scripts/desktop/validate-review-package.py"),
                        "--package",
                        str(output / "review-package"),
                    ],
                    check=False,
                    capture_output=True,
                    text=True,
                )
                self.assertEqual(
                    validation.returncode, 0, validation.stdout + validation.stderr
                )
                self.assertIn("VALID_FAIL", validation.stdout)

    def test_runner_build_failure_still_finalizes_filtered_package(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo, _, environment = self.make_runner_fixture(Path(directory))
            environment["FAKE_BUILD_EXIT"] = "9"
            result, output = self.invoke_fixture_runner(
                repo, environment, "build-failure"
            )
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            summary_path = output / "review-package/summary.json"
            self.assertTrue(summary_path.is_file(), result.stdout + result.stderr)
            summary = json.loads(summary_path.read_text(encoding="utf-8"))
            self.assertFalse(summary["qemu_started"])
            self.assertIsNone(summary["qemu_exit"])
            self.assertEqual(summary["runner_exit"], 9)
            self.assertEqual(summary["failure_stage"], "desktop-build")

    def test_output_creation_failure_has_explicit_no_evidence_contract(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo, _, environment = self.make_runner_fixture(Path(directory))
            outside = Path(directory) / "outside"
            result = subprocess.run(
                [
                    "bash",
                    str(repo / "scripts/desktop/run-headless-qemu.sh"),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                    "--output",
                    str(outside),
                ],
                cwd=repo,
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("DESKTOP_RUNTIME_EVIDENCE=UNAVAILABLE", result.stderr)
            self.assertIn("stage=output-setup", result.stderr)
            self.assertFalse(outside.exists())

    def test_missing_python_has_explicit_no_evidence_contract(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            repo, _, _ = self.make_runner_fixture(root)
            limited = root / "limited-path"
            limited.mkdir()
            for name in ("git", "dirname"):
                target = shutil.which(name)
                self.assertIsNotNone(target)
                (limited / name).symlink_to(target)
            environment = os.environ.copy()
            environment["PATH"] = str(limited)
            result = subprocess.run(
                [
                    "/bin/bash",
                    str(repo / "scripts/desktop/run-headless-qemu.sh"),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                ],
                cwd=repo,
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 127, result.stdout + result.stderr)
            self.assertIn("DESKTOP_RUNTIME_EVIDENCE=UNAVAILABLE", result.stderr)
            self.assertIn("reason=missing_python", result.stderr)

    def test_runner_timeout_finalizes_and_cleans_child_and_qmp_dir(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            repo, _, environment = self.make_runner_fixture(root)
            pid_file = root / "qemu.pid"
            stat_file = root / "qemu.stat"
            qmp_record = root / "qmp-path"
            environment.update(
                {
                    "FAKE_QEMU_MODE": "sleep",
                    "FAKE_QEMU_PID_FILE": str(pid_file),
                    "FAKE_QEMU_STAT_FILE": str(stat_file),
                    "FAKE_QMP_RECORD": str(qmp_record),
                    "DESKTOP_QEMU_TIMEOUT_SECS": "1",
                }
            )
            result, output = self.invoke_fixture_runner(repo, environment, "timeout")
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            summary = json.loads(
                (output / "review-package/summary.json").read_text(encoding="utf-8")
            )
            self.assertEqual(summary["failure_stage"], "qemu-boot")
            self.assertEqual(summary["qemu_exit"], 124)
            child = int(pid_file.read_text(encoding="utf-8"))
            start_time = stat_file.read_text(encoding="utf-8").split()[21]
            self.assert_process_gone(child, start_time)
            qmp_dir = Path(qmp_record.read_text(encoding="utf-8"))
            self.assertFalse(qmp_dir.exists())

    def test_runner_signals_finalize_and_clean_child_and_qmp_dir(self) -> None:
        for delivered, expected_exit, expected_reason in (
            (signal.SIGINT, 130, "signal_interrupted"),
            (signal.SIGTERM, 143, "signal_terminated"),
        ):
            with self.subTest(signal=delivered), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                repo, _, environment = self.make_runner_fixture(root)
                pid_file = root / "qemu.pid"
                stat_file = root / "qemu.stat"
                qmp_record = root / "qmp-path"
                output = repo / f"test/output/desktop/signal-{delivered}"
                environment.update(
                    {
                        "FAKE_QEMU_MODE": "sleep",
                        "FAKE_QEMU_PID_FILE": str(pid_file),
                        "FAKE_QEMU_STAT_FILE": str(stat_file),
                        "FAKE_QMP_RECORD": str(qmp_record),
                    }
                )
                process = subprocess.Popen(
                    [
                        "bash",
                        str(repo / "scripts/desktop/run-headless-qemu.sh"),
                        "--arch",
                        "rv",
                        "--scenario",
                        "boot",
                        "--output",
                        str(output),
                    ],
                    cwd=repo,
                    env=environment,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                )
                self.assertIsNotNone(process.stdout)
                line = process.stdout.readline()
                self.assertIn("QEMU_RUN_DIR=", line)
                startup_deadline = time.monotonic() + 2.0
                while not pid_file.is_file() or not stat_file.is_file():
                    if time.monotonic() >= startup_deadline:
                        process.kill()
                        stdout, stderr = process.communicate(timeout=5)
                        self.fail(
                            "fake QEMU did not confirm startup before signal: "
                            + line
                            + stdout
                            + stderr
                        )
                    time.sleep(0.01)
                process.send_signal(delivered)
                stdout, stderr = process.communicate(timeout=15)
                self.assertEqual(
                    process.returncode,
                    expected_exit,
                    line + stdout + stderr,
                )
                summary = json.loads(
                    (output / "review-package/summary.json").read_text(encoding="utf-8")
                )
                self.assertEqual(summary["runner_exit"], expected_exit)
                self.assertEqual(summary["failure_stage"], "runner-signal")
                self.assertEqual(summary["failure_reason"], expected_reason)
                child = int(pid_file.read_text(encoding="utf-8"))
                start_time = stat_file.read_text(encoding="utf-8").split()[21]
                self.assert_process_gone(child, start_time)
                qmp_dir = Path(qmp_record.read_text(encoding="utf-8"))
                self.assertFalse(qmp_dir.exists())


if __name__ == "__main__":
    unittest.main()
