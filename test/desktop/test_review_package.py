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
PACKAGE_SCRIPT = REPO_ROOT / "scripts/desktop/package-review-evidence.py"
SUMMARY_SCRIPT = REPO_ROOT / "scripts/desktop/summarize-run.py"
VALIDATOR_SCRIPT = REPO_ROOT / "scripts/desktop/validate-review-package.py"
FINALIZER_SCRIPT = REPO_ROOT / "scripts/desktop/finalize-runtime-evidence.py"


def qmp_exchange(commands: list[dict]) -> bytes:
    rows = [{"direction": "receive", "message": {"QMP": {}}}]
    for command in commands:
        rows.append({"direction": "send", "message": command})
        rows.append({"direction": "receive", "message": {"return": {}}})
    return "".join(json.dumps(row) + "\n" for row in rows).encode()


class ReviewPackageTests(unittest.TestCase):
    def make_run(self, root: Path) -> Path:
        run = root / "run"
        run.mkdir()
        serial = (
            "ORAYS_DESKTOP_DISPLAY width=2 height=1\n"
            "ORAYS_DESKTOP_INPUT_READY devices=2\n"
            "ORAYS_DESKTOP_FRAME boot 1\n"
        ).encode()
        (run / "serial.log").write_bytes(serial)
        (run / "display-geometry.txt").write_text(
            "DISPLAY_GEOMETRY=2x1\n", encoding="utf-8"
        )
        (run / "input-sequence.json").write_text(
            '[{"label":"wait","wait_ms":1}]\n', encoding="utf-8"
        )
        (run / "qmp-input.jsonl").write_bytes(
            qmp_exchange([{"execute": "qmp_capabilities"}])
        )
        frame = run / "frame.ppm"
        frame.write_bytes(b"P6\n2 1\n255\n\x01\x02\x03\x04\x05\x06")
        (run / "qmp-capture.jsonl").write_bytes(
            qmp_exchange(
                [
                    {"execute": "qmp_capabilities"},
                    {"execute": "screendump", "arguments": {"filename": str(frame)}},
                    {"execute": "quit"},
                ]
            )
        )
        (run / "capture-precondition.json").write_text(
            json.dumps(
                {
                    "schema": 1,
                    "kind": "required-markers",
                    "serial_prefix_bytes": len(serial),
                    "serial_prefix_sha256": hashlib.sha256(serial).hexdigest(),
                    "markers": [
                        {
                            "marker": "ORAYS_DESKTOP_DISPLAY width=2 height=1",
                            "line": 1,
                        }
                    ],
                }
            ),
            encoding="utf-8",
        )
        artifact = run / "orays-desktop-rv.bin"
        metadata = {
            "schema": 4,
            "created_at_utc": "2026-07-19T00:00:00+00:00",
            "finalized_at_utc": "2026-07-19T00:01:00+00:00",
            "source_commit": "b" * 40,
            "source_dirty": False,
            "source_status": [],
            "source_commit_before": "b" * 40,
            "source_dirty_before": False,
            "source_status_before": [],
            "source_commit_after": "b" * 40,
            "source_dirty_after": False,
            "source_status_after": [],
            "provenance_stable": True,
            "collection_errors": [],
            "architecture": "rv",
            "scenario": "boot",
            "run_dir": str(run),
            "qemu_binary": "/opt/orays-test-qemu/bin/qemu-system-riscv64",
            "qemu_version": "QEMU emulator version 9.2.4",
            "qemu_sha256": "a" * 64,
            "qemu_digest_policy": "unpinned",
            "qemu_authorized_sha256": None,
            "qemu_digest_matches_authorized": None,
            "qemu_argv": [
                "/opt/orays-test-qemu/bin/qemu-system-riscv64",
                "-machine",
                "virt",
                "-kernel",
                str(artifact),
            ],
            "guest_artifact": {
                "path": str(artifact),
                "type": "raw-binary",
                "size": 4096,
                "sha256": "c" * 64,
                "architecture": "rv",
            },
            "runner_inputs": {"vnc_display": 42, "qemu_timeout_seconds": 90},
            "required_qemu_version": "9.2.4",
            "observed_qemu_version": "QEMU emulator version 9.2.4",
            "qemu_version_matches_required": True,
            "toolchain_versions": {
                "rustc": "rustc test",
                "cargo": "cargo test",
                "python": "Python test",
            },
            "generation_command": [
                "scripts/desktop/run-headless-qemu.sh",
                "--arch",
                "rv",
                "--scenario",
                "boot",
                "--output",
                str(run),
            ],
        }
        (run / "runtime-metadata.json").write_text(
            json.dumps(metadata, sort_keys=True) + "\n", encoding="utf-8"
        )
        summary = subprocess.run(
            [
                sys.executable,
                "-B",
                str(SUMMARY_SCRIPT),
                "--run-dir",
                str(run),
                "--arch",
                "rv",
                "--scenario",
                "boot",
                "--qemu-exit",
                "0",
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(summary.returncode, 0, summary.stdout + summary.stderr)
        return run

    def invoke_package(self, run: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, "-B", str(PACKAGE_SCRIPT), "--run-dir", str(run)],
            check=False,
            capture_output=True,
            text=True,
        )

    def invoke_validator(self, package: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, "-B", str(VALIDATOR_SCRIPT), "--package", str(package)],
            check=False,
            capture_output=True,
            text=True,
        )

    def rewrite_packaged_summary(
        self, package: Path, summary: dict, package_updates: dict
    ) -> None:
        summary_path = package / "summary.json"
        summary_path.write_text(
            json.dumps(summary, sort_keys=True) + "\n", encoding="utf-8"
        )
        package_path = package / "review-package.json"
        manifest = json.loads(package_path.read_text(encoding="utf-8"))
        manifest.update(package_updates)
        manifest["files"]["summary.json"] = hashlib.sha256(
            summary_path.read_bytes()
        ).hexdigest()
        package_path.write_text(
            json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        (package / "package-files.sha256").write_text(
            "".join(
                f"{digest}  {name}\n"
                for name, digest in sorted(manifest["files"].items())
            ),
            encoding="utf-8",
        )

    def test_complete_raw_evidence_is_packaged_with_outer_hashes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            result = self.invoke_package(run)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            package = run / "review-package"
            self.assertTrue((package / "review-package.json").is_file())
            self.assertTrue((package / "package-files.sha256").is_file())
            self.assertEqual(
                json.loads((package / "runtime-metadata.json").read_text(encoding="utf-8"))["schema"],
                4,
            )
            self.assertEqual(
                json.loads((package / "summary.json").read_text(encoding="utf-8"))["schema"],
                2,
            )
            self.assertEqual(
                json.loads((package / "review-package.json").read_text(encoding="utf-8"))["schema"],
                4,
            )
            self.assertFalse((package / "disk.img").exists())
            validation = self.invoke_validator(package)
            self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
            self.assertIn("VALID_PASS", validation.stdout)

    def test_relocated_package_reproduces_semantic_pass(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            run = self.make_run(root)
            self.assertEqual(self.invoke_package(run).returncode, 0)
            relocated_parent = root / "relocated"
            relocated_parent.mkdir()
            relocated = Path(shutil.move(str(run / "review-package"), relocated_parent))
            shutil.rmtree(run)
            result = self.invoke_validator(relocated)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("VALID_PASS", result.stdout)

    def test_pass_package_rejects_qemu_version_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            metadata_path = run / "runtime-metadata.json"
            metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
            metadata["qemu_version"] = "QEMU emulator version 6.2.0"
            metadata["observed_qemu_version"] = "QEMU emulator version 6.2.0"
            metadata["qemu_version_matches_required"] = False
            metadata_path.write_text(json.dumps(metadata, sort_keys=True) + "\n", encoding="utf-8")
            summary = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(SUMMARY_SCRIPT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                    "--qemu-started",
                    "true",
                    "--qemu-exit",
                    "0",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(summary.returncode, 0)
            self.assertIn("QEMU version", (run / "summary.json").read_text(encoding="utf-8"))

    def test_dirty_tree_cannot_produce_semantic_pass(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            metadata_path = run / "runtime-metadata.json"
            metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
            metadata.update(
                {
                    "source_dirty": True,
                    "source_status": [" M tracked.rs"],
                    "source_dirty_before": True,
                    "source_status_before": [" M tracked.rs"],
                    "source_dirty_after": True,
                    "source_status_after": [" M tracked.rs"],
                    "provenance_stable": False,
                }
            )
            metadata_path.write_text(json.dumps(metadata, sort_keys=True) + "\n", encoding="utf-8")
            summary = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(SUMMARY_SCRIPT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                    "--qemu-started",
                    "true",
                    "--qemu-exit",
                    "0",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(summary.returncode, 0)
            summary_value = json.loads((run / "summary.json").read_text(encoding="utf-8"))
            self.assertEqual(summary_value["result"], "FAIL")
            self.assertTrue(
                any("changed during the run" in failure for failure in summary_value["failures"])
            )

    def test_packager_rejects_qemu_not_started_with_zero_runner_exit(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            summary_path = run / "summary.json"
            summary = json.loads(summary_path.read_text(encoding="utf-8"))
            summary.update(
                {
                    "qemu_started": False,
                    "qemu_exit": None,
                    "runner_exit": 0,
                    "failure_stage": "complete",
                    "failure_reason": "none",
                }
            )
            summary_path.write_text(json.dumps(summary, sort_keys=True) + "\n", encoding="utf-8")
            result = self.invoke_package(run)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("QEMU not started", result.stderr)

    def test_validator_rejects_forged_qemu_not_started_pass(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            self.assertEqual(self.invoke_package(run).returncode, 0)
            package = run / "review-package"
            summary = json.loads(
                (package / "summary.json").read_text(encoding="utf-8")
            )
            summary.update(
                {
                    "qemu_started": False,
                    "qemu_exit": None,
                    "runner_exit": 0,
                    "failure_stage": "complete",
                    "failure_reason": "none",
                }
            )
            self.rewrite_packaged_summary(
                package,
                summary,
                {
                    "qemu_started": False,
                    "qemu_exit": None,
                    "runner_exit": 0,
                    "failure_stage": "complete",
                    "failure_reason": "none",
                },
            )
            result = self.invoke_validator(package)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("requires a nonzero runner_exit", result.stderr)

    def test_packager_rejects_legacy_summary_schema(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            summary_path = run / "summary.json"
            summary = json.loads(summary_path.read_text(encoding="utf-8"))
            summary["schema"] = 1
            summary_path.write_text(json.dumps(summary), encoding="utf-8")
            result = self.invoke_package(run)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("summary schema", result.stderr)

    def test_relocated_package_tampering_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            run = self.make_run(root)
            self.assertEqual(self.invoke_package(run).returncode, 0)
            relocated_parent = root / "relocated"
            relocated_parent.mkdir()
            relocated = Path(shutil.move(str(run / "review-package"), relocated_parent))
            with (relocated / "frame.ppm").open("ab") as frame:
                frame.write(b"tampered")
            result = self.invoke_validator(relocated)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("digest mismatch", result.stderr)

    def test_capture_binding_path_escape_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            self.assertEqual(self.invoke_package(run).returncode, 0)
            package = run / "review-package"
            manifest_path = package / "review-package.json"
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            manifest["capture_binding"]["evidence_relative_filename"] = "../frame.ppm"
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            result = self.invoke_validator(package)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("relative filename", result.stderr)

    def test_missing_raw_file_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            (run / "serial.log").unlink()
            result = self.invoke_package(run)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("serial.log", result.stderr)

    def test_tampered_raw_file_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            (run / "frame.ppm").write_bytes(b"tampered")
            result = self.invoke_package(run)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("digest mismatch", result.stderr)

    def test_runner_failure_still_produces_valid_filtered_failure_package(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory) / "failed-run"
            run.mkdir()
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(FINALIZER_SCRIPT),
                    "--repo-root",
                    str(REPO_ROOT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                    "--qemu-binary",
                    "qemu-system-riscv64",
                    "--qemu-started",
                    "false",
                    "--runner-exit",
                    "1",
                    "--failure-stage",
                    "desktop-build",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(result.returncode, 0)
            package = run / "review-package"
            self.assertTrue(package.is_dir(), result.stdout + result.stderr)
            for name in (
                "summary.json",
                "serial.log",
                "qmp-input.jsonl",
                "qmp-capture.jsonl",
                "runtime-metadata.json",
            ):
                self.assertTrue((package / name).is_file(), name)
            self.assertFalse((package / "disk.img").exists())
            metadata = json.loads(
                (package / "runtime-metadata.json").read_text(encoding="utf-8")
            )
            self.assertEqual(
                metadata["source_commit_before"], metadata["source_commit_after"]
            )
            self.assertEqual(
                metadata["source_status_before"], metadata["source_status_after"]
            )
            self.assertEqual(
                metadata["provenance_stable"],
                not metadata["source_dirty_before"]
                and not metadata["source_dirty_after"],
            )
            validation = self.invoke_validator(package)
            self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
            self.assertIn("VALID_FAIL", validation.stdout)

    def make_failure_package(self, root: Path, *finalizer_args: str) -> Path:
        run = root / "failed-qemu"
        run.mkdir()
        result = subprocess.run(
            [
                sys.executable,
                "-B",
                str(FINALIZER_SCRIPT),
                "--repo-root",
                str(REPO_ROOT),
                "--run-dir",
                str(run),
                "--arch",
                "la",
                "--scenario",
                "boot",
                "--qemu-binary",
                "qemu-system-loongarch64",
                *finalizer_args,
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(result.returncode, 0)
        package = run / "review-package"
        self.assertTrue(package.is_dir(), result.stdout + result.stderr)
        return package

    def test_qemu_zero_exit_before_guest_evidence_is_a_valid_fail_package(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            package = self.make_failure_package(
                Path(directory),
                "--qemu-exit",
                "0",
                "--runner-exit",
                "1",
                "--failure-stage",
                "qemu-boot",
            )
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            self.assertEqual(summary["qemu_exit"], 0)
            self.assertEqual(summary["result"], "FAIL")
            self.assertTrue(summary["failures"])
            validation = self.invoke_validator(package)
            self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
            self.assertIn("VALID_FAIL", validation.stdout)

    def test_validator_rejects_different_failure_with_same_colon_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            package = self.make_failure_package(
                Path(directory),
                "--qemu-exit",
                "42",
                "--runner-exit",
                "1",
                "--failure-stage",
                "qemu-boot",
            )
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            original = "invalid capture evidence: QMP transcript is empty"
            self.assertIn(original, summary["failures"])
            summary["failures"] = [
                (
                    "invalid capture evidence: screendump target does not match "
                    "the captured frame"
                    if failure == original
                    else failure
                )
                for failure in summary["failures"]
            ]
            self.rewrite_packaged_summary(package, summary, {})
            result = self.invoke_validator(package)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("do not exactly match", result.stderr)

    def test_validator_rejects_dropped_or_added_failure(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            package = self.make_failure_package(
                Path(directory),
                "--qemu-exit",
                "42",
                "--runner-exit",
                "1",
                "--failure-stage",
                "qemu-boot",
            )
            original_summary = json.loads(
                (package / "summary.json").read_text(encoding="utf-8")
            )
            dropped = dict(original_summary)
            dropped["failures"] = original_summary["failures"][1:]
            self.rewrite_packaged_summary(package, dropped, {})
            result = self.invoke_validator(package)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("do not exactly match", result.stderr)

        with tempfile.TemporaryDirectory() as directory:
            package = self.make_failure_package(
                Path(directory),
                "--qemu-exit",
                "42",
                "--runner-exit",
                "1",
                "--failure-stage",
                "qemu-boot",
            )
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            summary["failures"] = summary["failures"] + ["invalid capture evidence: extra"]
            self.rewrite_packaged_summary(package, summary, {})
            result = self.invoke_validator(package)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("do not exactly match", result.stderr)

    def test_validator_rejects_qemu_identity_tamper_in_package_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            package = self.make_failure_package(
                Path(directory),
                "--qemu-exit",
                "42",
                "--runner-exit",
                "1",
                "--failure-stage",
                "qemu-boot",
            )
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            self.rewrite_packaged_summary(package, summary, {"qemu_sha256": "f" * 64})
            result = self.invoke_validator(package)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("does not match runtime metadata", result.stderr)

    def test_validator_rejects_guest_artifact_tamper_in_package_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            package = self.make_failure_package(
                Path(directory),
                "--qemu-exit",
                "42",
                "--runner-exit",
                "1",
                "--failure-stage",
                "qemu-boot",
            )
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            artifact = dict(
                json.loads((package / "runtime-metadata.json").read_text(encoding="utf-8"))[
                    "guest_artifact"
                ]
                or {"path": "/x", "type": "elf", "size": 1, "sha256": "0" * 64,
                    "architecture": "la"}
            )
            artifact["sha256"] = "f" * 64
            self.rewrite_packaged_summary(package, summary, {"guest_artifact": artifact})
            result = self.invoke_validator(package)
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("does not match runtime metadata", result.stderr)

    def test_qemu_abnormal_exit_is_preserved_in_failure_package(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory) / "failed-qemu"
            run.mkdir()
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(FINALIZER_SCRIPT),
                    "--repo-root",
                    str(REPO_ROOT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "la",
                    "--scenario",
                    "boot",
                    "--qemu-binary",
                    "qemu-system-loongarch64",
                    "--qemu-exit",
                    "42",
                    "--runner-exit",
                    "1",
                    "--failure-stage",
                    "qemu-boot",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(result.returncode, 0)
            package = run / "review-package"
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            self.assertEqual(summary["qemu_exit"], 42)
            self.assertEqual(summary["failure_stage"], "qemu-boot")
            self.assertTrue(
                any("qemu exit was 42, expected 0" in failure for failure in summary["failures"])
            )
            validation = self.invoke_validator(package)
            self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
            self.assertIn("VALID_FAIL", validation.stdout)

    def test_qemu_not_started_uses_null_exit_and_structured_reason(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory) / "qemu-not-started"
            run.mkdir()
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(FINALIZER_SCRIPT),
                    "--repo-root",
                    str(REPO_ROOT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                    "--qemu-binary",
                    "qemu-system-riscv64",
                    "--required-qemu-version",
                    "9.2.4",
                    "--qemu-started",
                    "false",
                    "--runner-exit",
                    "3",
                    "--failure-stage",
                    "runtime-prerequisites",
                    "--failure-reason",
                    "qemu_version_mismatch",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertNotEqual(result.returncode, 70, result.stdout + result.stderr)
            package = run / "review-package"
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            self.assertFalse(summary["qemu_started"])
            self.assertIsNone(summary["qemu_exit"])
            self.assertEqual(summary["failure_reason"], "qemu_version_mismatch")
            validation = self.invoke_validator(package)
            self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
            self.assertIn("VALID_FAIL", validation.stdout)

    def test_missing_toolchain_still_produces_auditable_failure_package(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            tool_bin = root / "bin"
            tool_bin.mkdir()
            git = shutil.which("git")
            self.assertIsNotNone(git)
            (tool_bin / "git").symlink_to(git)
            (tool_bin / "python3").symlink_to(sys.executable)
            qemu = tool_bin / "qemu-system-riscv64"
            qemu.write_text(
                "#!/bin/sh\necho 'QEMU emulator version 9.2.4'\n",
                encoding="utf-8",
            )
            qemu.chmod(0o755)
            run = root / "missing-toolchain"
            run.mkdir()
            environment = os.environ.copy()
            environment["PATH"] = str(tool_bin)
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(FINALIZER_SCRIPT),
                    "--repo-root",
                    str(REPO_ROOT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "rv",
                    "--scenario",
                    "boot",
                    "--qemu-binary",
                    "qemu-system-riscv64",
                    "--qemu-started",
                    "false",
                    "--runner-exit",
                    "1",
                    "--failure-stage",
                    "runtime-prerequisites",
                ],
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertNotEqual(result.returncode, 70, result.stdout + result.stderr)
            package = run / "review-package"
            metadata = json.loads(
                (package / "runtime-metadata.json").read_text(encoding="utf-8")
            )
            self.assertTrue(
                any("rustc version collection failed" in error for error in metadata["collection_errors"])
            )
            self.assertTrue(
                any("cargo version collection failed" in error for error in metadata["collection_errors"])
            )
            validation = self.invoke_validator(package)
            self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
            self.assertIn("VALID_FAIL", validation.stdout)


if __name__ == "__main__":
    unittest.main()
