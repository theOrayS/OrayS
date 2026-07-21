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
    def bind_runtime_identity(self, run: Path) -> None:
        repository = run.parent
        qemu = repository / "qemu-system-riscv64"
        artifact = repository / "build/desktop/rv/artifacts/orays-desktop-rv.bin"
        artifact.parent.mkdir(parents=True, exist_ok=True)
        qemu.write_bytes(b"approved-qemu-object")
        qemu.chmod(0o755)
        artifact.write_bytes(b"guest-artifact-object")
        qemu_digest = hashlib.sha256(qemu.read_bytes()).hexdigest()
        policy = {
            "schema": 1,
            "qemu_version": "9.2.4",
            "architectures": {
                "rv": {
                    "qemu_binary": "qemu-system-riscv64",
                    "qemu_sha256": qemu_digest,
                    "artifact": "build/desktop/rv/artifacts/orays-desktop-rv.bin",
                    "build_invocation": ["scripts/desktop/build.sh", "rv"],
                }
            },
        }
        policy_path = run / "runtime-policy.json"
        policy_path.write_text(
            json.dumps(policy, sort_keys=True) + "\n", encoding="utf-8"
        )
        metadata_path = run / "runtime-metadata.json"
        metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
        metadata.update(
            {
                "schema": 4,
                "repository_root": str(repository),
                "runtime_identity": {
                    "schema": 1,
                    "policy_repository_path": "test/desktop/runtime-policy.json",
                    "policy_sha256": hashlib.sha256(policy_path.read_bytes()).hexdigest(),
                    "qemu": {
                        "canonical_path": str(qemu),
                        "required_version": "9.2.4",
                        "observed_banner": "QEMU emulator version 9.2.4",
                        "required_sha256": qemu_digest,
                        "observed_sha256": qemu_digest,
                    },
                    "guest_artifact": {
                        "architecture": "rv",
                        "repository_path": "build/desktop/rv/artifacts/orays-desktop-rv.bin",
                        "canonical_path": str(artifact),
                        "sha256": hashlib.sha256(artifact.read_bytes()).hexdigest(),
                        "source_commit": metadata["source_commit_before"],
                    },
                    "build_invocation": ["scripts/desktop/build.sh", "rv"],
                    "qemu_launch_argv": [
                        str(qemu),
                        "-machine",
                        "virt",
                        "-kernel",
                        str(artifact),
                    ],
                },
            }
        )
        metadata_path.write_text(
            json.dumps(metadata, sort_keys=True) + "\n", encoding="utf-8"
        )

    def summarize_bound_run(self, run: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
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
        metadata = {
            "schema": 3,
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
            "qemu_binary": "/usr/bin/qemu-system-riscv64",
            "qemu_version": "QEMU emulator version 9.2.4",
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
        self.bind_runtime_identity(run)
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

    def rehash_packaged_file(self, package: Path, name: str) -> None:
        manifest_path = package / "review-package.json"
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        manifest["files"][name] = hashlib.sha256((package / name).read_bytes()).hexdigest()
        manifest_path.write_text(
            json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        (package / "package-files.sha256").write_text(
            "".join(
                f"{digest}  {filename}\n"
                for filename, digest in sorted(manifest["files"].items())
            ),
            encoding="utf-8",
        )

    def test_bound_runtime_identity_survives_full_package_chain(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            summary = self.summarize_bound_run(run)
            self.assertEqual(summary.returncode, 0, summary.stdout + summary.stderr)
            self.assertEqual(self.invoke_package(run).returncode, 0)
            package = run / "review-package"
            summary_value = json.loads(
                (package / "summary.json").read_text(encoding="utf-8")
            )
            package_value = json.loads(
                (package / "review-package.json").read_text(encoding="utf-8")
            )
            self.assertEqual(summary_value["schema"], 3)
            self.assertEqual(package_value["schema"], 4)
            self.assertEqual(
                package_value["runtime_identity"], summary_value["runtime_identity"]
            )
            self.assertIn("runtime-policy.json", package_value["files"])
            validation = self.invoke_validator(package)
            self.assertEqual(validation.returncode, 0, validation.stdout + validation.stderr)
            self.assertIn("VALID_PASS", validation.stdout)

    def test_validator_rejects_packaged_runtime_identity_tampering(self) -> None:
        mutations = (
            (
                lambda identity: identity["qemu"].__setitem__(
                    "observed_sha256", "0" * 64
                ),
                "QEMU digest",
            ),
            (
                lambda identity: identity["guest_artifact"].__setitem__(
                    "sha256", "0" * 64
                ),
                "runtime identity differs",
            ),
            (
                lambda identity: identity["guest_artifact"].__setitem__(
                    "canonical_path", "/tmp/forged-artifact"
                ),
                "artifact canonical path",
            ),
            (
                lambda identity: identity["qemu_launch_argv"].__setitem__(
                    identity["qemu_launch_argv"].index("-kernel") + 1,
                    "/tmp/forged-kernel",
                ),
                "QEMU launch artifact",
            ),
        )
        for mutate, message in mutations:
            with self.subTest(message=message), tempfile.TemporaryDirectory() as directory:
                run = self.make_run(Path(directory))
                self.assertEqual(self.summarize_bound_run(run).returncode, 0)
                self.assertEqual(self.invoke_package(run).returncode, 0)
                package = run / "review-package"
                metadata_path = package / "runtime-metadata.json"
                metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
                mutate(metadata["runtime_identity"])
                metadata_path.write_text(
                    json.dumps(metadata, sort_keys=True) + "\n", encoding="utf-8"
                )
                self.rehash_packaged_file(package, "runtime-metadata.json")
                result = self.invoke_validator(package)
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertIn(message, result.stderr)

    def test_validator_rejects_failure_detail_with_matching_category(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            (run / "input-sequence.json").write_text("not-json\n", encoding="utf-8")
            summary_result = self.summarize_bound_run(run)
            self.assertEqual(summary_result.returncode, 1, summary_result.stdout)
            self.assertEqual(self.invoke_package(run).returncode, 0)
            package = run / "review-package"
            summary = json.loads(
                (package / "summary.json").read_text(encoding="utf-8")
            )
            index = next(
                index
                for index, failure in enumerate(summary["failures"])
                if failure.startswith("invalid input evidence:")
            )
            summary["failures"][index] = (
                "invalid input evidence: forged detail with the same category"
            )
            self.rewrite_packaged_summary(package, summary, {})
            validation = self.invoke_validator(package)
            self.assertNotEqual(
                validation.returncode, 0, validation.stdout + validation.stderr
            )
            self.assertIn("exact deterministic failures", validation.stderr)

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
                3,
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

    def test_qemu_started_without_prebound_identity_fails_closed(self) -> None:
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
            self.assertEqual(result.returncode, 70, result.stdout + result.stderr)
            package = run / "review-package"
            summary = json.loads((package / "summary.json").read_text(encoding="utf-8"))
            self.assertEqual(summary["qemu_exit"], 42)
            self.assertEqual(summary["failure_stage"], "qemu-boot")
            self.assertTrue(
                any("qemu exit was 42, expected 0" in failure for failure in summary["failures"])
            )
            validation = self.invoke_validator(package)
            self.assertNotEqual(
                validation.returncode, 0, validation.stdout + validation.stderr
            )
            self.assertIn("no verified QEMU object", validation.stderr)

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
