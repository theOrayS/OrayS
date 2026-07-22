from pathlib import Path
import hashlib
import json
import os
import shutil
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


class DesktopWorkflowPolicyTests(unittest.TestCase):
    def test_no_persistent_self_hosted_runtime_job_anywhere(self) -> None:
        text = WORKFLOW.read_text(encoding="utf-8")
        self.assertNotIn("desktop-runtime:", text)
        for workflow in sorted((REPO_ROOT / ".github/workflows").glob("*.yml")):
            with self.subTest(workflow=workflow.name):
                self.assertNotIn("self-hosted", workflow.read_text(encoding="utf-8"))

    def test_local_qemu_evidence_procedure_is_documented(self) -> None:
        doc = (REPO_ROOT / "docs/references/desktop-headless-development.md").read_text(
            encoding="utf-8"
        )
        for arch in ("rv", "la"):
            self.assertIn(f"run-headless-qemu.sh --arch {arch} --scenario boot", doc)
        self.assertIn("DESKTOP_QEMU_AUTHORIZED_SHA256", doc)
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertIn("required_qemu_version=9.2.4", runner)

    def test_desktop_job_keeps_host_and_dual_architecture_gates(self) -> None:
        text = WORKFLOW.read_text(encoding="utf-8")
        self.assertIn("scripts/desktop/build.sh host-test", text)
        self.assertIn("python3 -B -m unittest discover -s test/desktop", text)
        self.assertIn("scripts/desktop/build.sh golden-check", text)
        self.assertIn("scripts/desktop/build.sh rv", text)
        self.assertIn("scripts/desktop/build.sh la", text)
        self.assertIn("check-scope.py --base", text)

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

    def test_runner_records_real_qemu_exit_before_every_wait(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertIn('if [[ -z "$qemu_exit" ]]; then', runner)
        self.assertNotIn('if [[ -z "$qemu_exit" || "$qemu_exit" == 0 ]]; then', runner)
        waits = runner.count('wait "$qemu_pid" || qemu_exit=$?')
        self.assertGreaterEqual(waits, 7)
        self.assertGreaterEqual(runner.count("qemu_exit=0"), waits)

    def test_runner_executes_resolved_verified_qemu_identity(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertIn(
            'qemu_binary_resolved=$(readlink -f "$(command -v "$qemu_binary")")', runner
        )
        self.assertIn('qemu_version=$("$qemu_binary_resolved" --version', runner)
        self.assertIn('"$qemu_binary_resolved" "${qemu_args[@]}"', runner)
        self.assertIn('--qemu-binary "$qemu_binary_resolved"', runner)
        self.assertIn("--record-invocation", runner)
        self.assertIn('--guest-artifact "$artifact"', runner)
        self.assertIn('--qemu-argv "$qemu_binary_resolved" "${qemu_args[@]}"', runner)
        self.assertIn("DESKTOP_QEMU_AUTHORIZED_SHA256", runner)
        self.assertIn("qemu_digest_mismatch", runner)
        self.assertIn("qemu_authorized_digest_invalid", runner)
        self.assertNotIn('"$qemu_binary" "${qemu_args[@]}"', runner)

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


class RunnerQemuIdentityTests(unittest.TestCase):
    def make_fake_qemu(
        self, root: Path, banner: str = "QEMU emulator version 9.2.4"
    ) -> Path:
        fakebin = root / "bin"
        fakebin.mkdir()
        qemu = fakebin / "qemu-system-riscv64"
        qemu.write_text(f"#!/bin/sh\necho '{banner}'\n", encoding="utf-8")
        qemu.chmod(0o755)
        return fakebin

    def run_runner(
        self, root: Path, fakebin: Path, extra_env: dict[str, str] | None = None
    ) -> tuple[subprocess.CompletedProcess[str], Path]:
        environment = os.environ.copy()
        environment["PATH"] = str(fakebin) + os.pathsep + environment["PATH"]
        if extra_env:
            environment.update(extra_env)
        output_root = REPO_ROOT / "test/output/desktop"
        output_root.mkdir(parents=True, exist_ok=True)
        run_dir = Path(tempfile.mkdtemp(prefix="runner-test-", dir=output_root))
        run_dir.rmdir()
        self.addCleanup(shutil.rmtree, run_dir, True)
        result = subprocess.run(
            [str(RUNNER), "--arch", "rv", "--scenario", "boot", "--output", str(run_dir)],
            check=False,
            capture_output=True,
            text=True,
            env=environment,
            timeout=120,
        )
        return result, run_dir

    def assert_valid_fail_package(self, run_dir: Path) -> dict:
        package = run_dir / "review-package"
        self.assertTrue(package.is_dir())
        validation = subprocess.run(
            [
                sys.executable,
                "-B",
                str(REPO_ROOT / "scripts/desktop/validate-review-package.py"),
                "--package",
                str(package),
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
        self.assertIn("VALID_FAIL", validation.stdout)
        return json.loads((package / "summary.json").read_text(encoding="utf-8"))

    def test_runner_rejects_wrong_qemu_banner_with_valid_fail_package(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = self.make_fake_qemu(root, "QEMU emulator version 6.2.0")
            result, run_dir = self.run_runner(root, fakebin)
            self.assertEqual(result.returncode, 3, result.stdout + result.stderr)
            summary = self.assert_valid_fail_package(run_dir)
            self.assertFalse(summary["qemu_started"])
            self.assertIsNone(summary["qemu_exit"])
            self.assertEqual(summary["failure_stage"], "runtime-prerequisites")
            self.assertEqual(summary["failure_reason"], "qemu_version_mismatch")

    def test_runner_rejects_unauthorized_qemu_digest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = self.make_fake_qemu(root)
            result, run_dir = self.run_runner(
                root, fakebin, {"DESKTOP_QEMU_AUTHORIZED_SHA256": "0" * 64}
            )
            self.assertEqual(result.returncode, 3, result.stdout + result.stderr)
            summary = self.assert_valid_fail_package(run_dir)
            self.assertEqual(summary["failure_reason"], "qemu_digest_mismatch")
            metadata = json.loads(
                (run_dir / "review-package/runtime-metadata.json").read_text(
                    encoding="utf-8"
                )
            )
            self.assertEqual(metadata["qemu_digest_policy"], "authorized-sha256")
            self.assertEqual(metadata["qemu_authorized_sha256"], "0" * 64)
            self.assertFalse(metadata["qemu_digest_matches_authorized"])

    def test_runner_rejects_malformed_authorized_qemu_digest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = self.make_fake_qemu(root)
            result, run_dir = self.run_runner(
                root, fakebin, {"DESKTOP_QEMU_AUTHORIZED_SHA256": "not-a-digest"}
            )
            self.assertEqual(result.returncode, 3, result.stdout + result.stderr)
            summary = self.assert_valid_fail_package(run_dir)
            self.assertEqual(summary["failure_reason"], "qemu_authorized_digest_invalid")


class MetadataInvocationTests(unittest.TestCase):
    def collect_initial(self, root: Path, fakebin: Path) -> Path:
        run = root / "run"
        run.mkdir()
        output = root / "runtime-metadata.json"
        environment = os.environ.copy()
        environment["PATH"] = str(fakebin) + os.pathsep + environment["PATH"]
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
                "9.2.4",
                "--run-dir",
                str(run),
            ],
            check=False,
            capture_output=True,
            text=True,
            env=environment,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        return output

    def make_artifact(self, root: Path) -> Path:
        artifact = root / "orays-desktop-rv.bin"
        artifact.write_bytes(b"\x00\x01\x02\x03" * 64)
        return artifact

    def record_invocation(
        self, output: Path, qemu: Path, artifact: Path, *extra: str
    ) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [
                sys.executable,
                "-B",
                str(METADATA_COLLECTOR),
                "--repo-root",
                str(REPO_ROOT),
                "--output",
                str(output),
                "--record-invocation",
                "--guest-artifact",
                str(artifact),
                "--vnc-display",
                "42",
                "--qemu-timeout-seconds",
                "90",
                "--qemu-argv",
                str(qemu),
                "-machine",
                "virt",
                "-kernel",
                str(artifact),
                *extra,
            ],
            check=False,
            capture_output=True,
            text=True,
        )

    def test_initial_collection_records_digest_identity(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            output = self.collect_initial(root, fakebin)
            metadata = json.loads(output.read_text(encoding="utf-8"))
            resolved = str((fakebin / "qemu-system-riscv64").resolve())
            self.assertEqual(metadata["schema"], 4)
            self.assertEqual(metadata["qemu_binary"], resolved)
            self.assertEqual(
                metadata["qemu_sha256"],
                hashlib.sha256((fakebin / "qemu-system-riscv64").read_bytes()).hexdigest(),
            )
            self.assertEqual(metadata["qemu_digest_policy"], "unpinned")
            self.assertIsNone(metadata["qemu_authorized_sha256"])
            self.assertIsNone(metadata["qemu_digest_matches_authorized"])
            self.assertIsNone(metadata["qemu_argv"])
            self.assertIsNone(metadata["guest_artifact"])

    def test_initial_collection_records_authorized_digest_match(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            digest = hashlib.sha256(
                (fakebin / "qemu-system-riscv64").read_bytes()
            ).hexdigest()
            run = root / "run"
            run.mkdir()
            output = root / "runtime-metadata.json"
            environment = os.environ.copy()
            environment["PATH"] = str(fakebin) + os.pathsep + environment["PATH"]
            environment["DESKTOP_QEMU_AUTHORIZED_SHA256"] = digest
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
                    "9.2.4",
                    "--run-dir",
                    str(run),
                ],
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            metadata = json.loads(output.read_text(encoding="utf-8"))
            self.assertEqual(metadata["qemu_digest_policy"], "authorized-sha256")
            self.assertEqual(metadata["qemu_authorized_sha256"], digest)
            self.assertTrue(metadata["qemu_digest_matches_authorized"])
            self.assertEqual(metadata["collection_errors"], [])

    def test_initial_collection_records_authorized_digest_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            run = root / "run"
            run.mkdir()
            output = root / "runtime-metadata.json"
            environment = os.environ.copy()
            environment["PATH"] = str(fakebin) + os.pathsep + environment["PATH"]
            environment["DESKTOP_QEMU_AUTHORIZED_SHA256"] = "0" * 64
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
                    "9.2.4",
                    "--run-dir",
                    str(run),
                ],
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            metadata = json.loads(output.read_text(encoding="utf-8"))
            self.assertEqual(metadata["qemu_digest_policy"], "authorized-sha256")
            self.assertFalse(metadata["qemu_digest_matches_authorized"])
            self.assertTrue(metadata["collection_errors"])

    def test_record_invocation_binds_argv_and_artifact(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            output = self.collect_initial(root, fakebin)
            artifact = self.make_artifact(root)
            qemu = (fakebin / "qemu-system-riscv64").resolve()
            result = self.record_invocation(output, qemu, artifact, "-serial", "stdio")
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            metadata = json.loads(output.read_text(encoding="utf-8"))
            self.assertEqual(
                metadata["qemu_argv"],
                [str(qemu), "-machine", "virt", "-kernel", str(artifact), "-serial", "stdio"],
            )
            guest = metadata["guest_artifact"]
            self.assertEqual(guest["path"], str(artifact))
            self.assertEqual(guest["type"], "raw-binary")
            self.assertEqual(guest["size"], artifact.stat().st_size)
            self.assertEqual(
                guest["sha256"], hashlib.sha256(artifact.read_bytes()).hexdigest()
            )
            self.assertEqual(guest["architecture"], "rv")
            self.assertEqual(
                metadata["runner_inputs"], {"vnc_display": 42, "qemu_timeout_seconds": 90}
            )

    def test_record_invocation_rejects_argv_zero_substitution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            output = self.collect_initial(root, fakebin)
            artifact = self.make_artifact(root)
            result = self.record_invocation(output, Path("/usr/bin/fake-qemu"), artifact)
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            self.assertIn("argv[0]", result.stderr)

    def test_record_invocation_rejects_kernel_artifact_substitution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            output = self.collect_initial(root, fakebin)
            artifact = self.make_artifact(root)
            other = root / "other.bin"
            other.write_bytes(b"\xff" * 16)
            qemu = (fakebin / "qemu-system-riscv64").resolve()
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(METADATA_COLLECTOR),
                    "--repo-root",
                    str(REPO_ROOT),
                    "--output",
                    str(output),
                    "--record-invocation",
                    "--guest-artifact",
                    str(artifact),
                    "--vnc-display",
                    "42",
                    "--qemu-timeout-seconds",
                    "90",
                    "--qemu-argv",
                    str(qemu),
                    "-kernel",
                    str(other),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            self.assertIn("-kernel", result.stderr)

    def test_record_invocation_rejects_qemu_binary_substitution(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            output = self.collect_initial(root, fakebin)
            artifact = self.make_artifact(root)
            qemu = fakebin / "qemu-system-riscv64"
            qemu.write_text("#!/bin/sh\necho 'QEMU emulator version 9.2.4'\n# changed\n")
            result = self.record_invocation(output, qemu.resolve(), artifact)
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            self.assertIn("changed between identity collection and execution", result.stderr)

    def test_record_invocation_detects_elf_artifact_type(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fakebin = RunnerQemuIdentityTests.make_fake_qemu(self, root)
            output = self.collect_initial(root, fakebin)
            artifact = root / "orays-desktop-rv.elf"
            artifact.write_bytes(b"\x7fELF" + b"\x00" * 60)
            qemu = (fakebin / "qemu-system-riscv64").resolve()
            result = self.record_invocation(output, qemu, artifact)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            metadata = json.loads(output.read_text(encoding="utf-8"))
            self.assertEqual(metadata["guest_artifact"]["type"], "elf")


if __name__ == "__main__":
    unittest.main()
