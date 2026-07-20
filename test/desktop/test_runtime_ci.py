from pathlib import Path
import os
import subprocess
import sys
import tempfile
import unittest


REPO_ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = REPO_ROOT / ".github/workflows/desktop.yml"
RUNNER = REPO_ROOT / "scripts/desktop/run-headless-qemu.sh"
QMP_RUNTIME_DIR = REPO_ROOT / "scripts/desktop/create-qmp-runtime-dir.py"
METADATA_COLLECTOR = REPO_ROOT / "scripts/desktop/collect-runtime-metadata.py"
DESKTOP_APP = REPO_ROOT / "user/desktop/src/app.rs"


class DesktopRuntimeCiTests(unittest.TestCase):
    def runtime_job(self) -> str:
        text = WORKFLOW.read_text(encoding="utf-8")
        return text.split("  desktop-runtime:\n", 1)[1]

    def test_self_hosted_runtime_never_runs_for_pull_request_events(self) -> None:
        text = WORKFLOW.read_text(encoding="utf-8")
        runtime = self.runtime_job()
        self.assertIn("pull_request:", text)
        self.assertIn("runs-on: [self-hosted, Linux, X64, orays-desktop-qemu-9.2.4]", runtime)
        self.assertIn("github.event_name == 'workflow_dispatch'", runtime)
        self.assertIn("github.event_name == 'push'", runtime)
        self.assertIn("refs/heads/feature/orays-desktop-environment", runtime)
        self.assertIn("refs/heads/develop/post-integration-next", runtime)
        self.assertNotIn("github.event_name == 'pull_request'", runtime)

    def test_runtime_architectures_are_independent_and_version_pinned(self) -> None:
        runtime = self.runtime_job()
        self.assertIn("fail-fast: false", runtime)
        self.assertIn("arch: [rv, la]", runtime)
        self.assertIn("FIXED_QEMU_VERSION: 9.2.4", WORKFLOW.read_text(encoding="utf-8"))
        self.assertIn("DESKTOP_REQUIRED_QEMU_VERSION: ${{ env.FIXED_QEMU_VERSION }}", runtime)
        self.assertIn("--arch ${{ matrix.arch }} --scenario boot", runtime)
        self.assertNotIn("--arch rv --scenario boot", runtime)
        self.assertNotIn("--arch la --scenario boot", runtime)

    def test_runtime_job_always_uploads_only_filtered_per_arch_package(self) -> None:
        runtime = self.runtime_job()
        self.assertIn(
            "actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02",
            runtime,
        )
        self.assertIn("if: ${{ always() }}", runtime)
        self.assertIn("${{ matrix.arch }}-boot/review-package", runtime)
        self.assertIn("if-no-files-found: error", runtime)
        self.assertNotIn("disk.img", runtime)
        self.assertNotIn("qmp.sock", runtime)

    def test_runner_exit_trap_finalizes_pass_and_failure_evidence(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertIn("trap finalize EXIT", runner)
        self.assertIn("finalize-runtime-evidence.py", runner)
        self.assertIn("--runner-exit \"$runner_exit\"", runner)
        self.assertIn('if [[ -n "$qemu_pid" ]]; then', runner)
        self.assertNotIn('if [[ -n "$qemu_pid" ]] && kill -0', runner)
        self.assertIn('wait "$qemu_pid" 2>/dev/null || stopped_exit=$?', runner)
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


if __name__ == "__main__":
    unittest.main()
