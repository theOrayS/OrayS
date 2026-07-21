from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/desktop/runtime-identity.py"
POLICY = ROOT / "test/desktop/runtime-policy.json"
CONTRACT = ROOT / "scripts/desktop/runtime_evidence_contract.py"
COLLECTOR = ROOT / "scripts/desktop/collect-runtime-metadata.py"


class RuntimeIdentityTests(unittest.TestCase):
    def invoke(self, *args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, "-B", str(SCRIPT), *args],
            check=False,
            capture_output=True,
            text=True,
        )

    def load_module(self):
        spec = importlib.util.spec_from_file_location("desktop_runtime_identity", SCRIPT)
        assert spec is not None and spec.loader is not None
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    def load_contract(self):
        spec = importlib.util.spec_from_file_location(
            "desktop_runtime_evidence_contract", CONTRACT
        )
        assert spec is not None and spec.loader is not None
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    def make_bound_identity(self, root: Path) -> tuple[dict, Path, Path, Path]:
        repository = root / "repo"
        run = repository / "test/output/desktop/run"
        artifact = repository / "build/desktop/rv/artifacts/orays-desktop-rv.bin"
        qemu = root / "approved-qemu"
        run.mkdir(parents=True)
        artifact.parent.mkdir(parents=True)
        artifact.write_bytes(b"guest-artifact")
        qemu.write_bytes(b"approved-qemu")
        qemu.chmod(0o755)
        qemu_digest = hashlib.sha256(qemu.read_bytes()).hexdigest()
        policy = {
            "schema": 1,
            "qemu_version": "9.2.4",
            "architectures": {
                "rv": {
                    "qemu_binary": "approved-qemu",
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
        commit = "a" * 40
        identity = {
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
                "source_commit": commit,
            },
            "build_invocation": ["scripts/desktop/build.sh", "rv"],
            "qemu_launch_argv": [
                str(qemu),
                "-machine",
                "virt",
                "-kernel",
                str(artifact),
            ],
        }
        metadata = {
            "source_commit_before": commit,
            "repository_root": str(repository),
            "runtime_identity": identity,
        }
        return metadata, policy_path, qemu, artifact

    def make_collector_fixture(self, root: Path) -> tuple[Path, Path, Path, Path, Path]:
        repository = root / "repo"
        repository.mkdir()
        subprocess.run(["git", "init", "-q"], cwd=repository, check=True)
        subprocess.run(
            ["git", "config", "user.email", "desktop-test@example.invalid"],
            cwd=repository,
            check=True,
        )
        subprocess.run(
            ["git", "config", "user.name", "Desktop Test"],
            cwd=repository,
            check=True,
        )
        artifact = repository / "build/desktop/rv/artifacts/orays-desktop-rv.bin"
        artifact.parent.mkdir(parents=True)
        artifact.write_bytes(b"guest-artifact")
        qemu = root / "qemu-system-riscv64"
        qemu.write_text(
            "#!/bin/sh\necho 'QEMU emulator version 9.2.4'\n", encoding="utf-8"
        )
        qemu.chmod(0o755)
        policy = repository / "test/desktop/runtime-policy.json"
        policy.parent.mkdir(parents=True)
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
        (repository / ".gitignore").write_text("test/output/\n", encoding="utf-8")
        subprocess.run(["git", "add", "."], cwd=repository, check=True)
        subprocess.run(["git", "commit", "-qm", "fixture"], cwd=repository, check=True)
        run = repository / "test/output/desktop/run"
        run.mkdir(parents=True)
        metadata = run / "runtime-metadata.json"
        return repository, run, metadata, qemu, artifact

    def invoke_collector(self, *args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, "-B", str(COLLECTOR), *args],
            check=False,
            capture_output=True,
            text=True,
        )

    def collect_and_bind(
        self, repository: Path, run: Path, metadata: Path, qemu: Path, artifact: Path
    ) -> None:
        initial = self.invoke_collector(
            "--repo-root",
            str(repository),
            "--output",
            str(metadata),
            "--arch",
            "rv",
            "--scenario",
            "boot",
            "--qemu-binary",
            qemu.name,
            "--qemu-path",
            str(qemu),
            "--required-qemu-version",
            "9.2.4",
            "--runtime-policy",
            str(repository / "test/desktop/runtime-policy.json"),
            "--run-dir",
            str(run),
        )
        self.assertEqual(initial.returncode, 0, initial.stdout + initial.stderr)
        launch = [str(qemu), "-machine", "virt", "-kernel", str(artifact)]
        bound = self.invoke_collector(
            "--repo-root",
            str(repository),
            "--output",
            str(metadata),
            "--bind-runtime",
            "--artifact",
            str(artifact),
            "--qemu-launch-argv-json",
            json.dumps(launch),
        )
        self.assertEqual(bound.returncode, 0, bound.stdout + bound.stderr)

    def test_tracked_policy_approves_exact_qemu_digests(self) -> None:
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        self.assertEqual(policy["schema"], 1)
        self.assertEqual(policy["qemu_version"], "9.2.4")
        self.assertEqual(
            policy["architectures"]["rv"]["qemu_sha256"],
            "194d645ab5063833b35512c2d15364070401f63a4f97baf4b7da2244d44efeee",
        )
        self.assertEqual(
            policy["architectures"]["la"]["qemu_sha256"],
            "9df8237128d340e34c491fdf76616240a604b3f3fbdbca769a1ab991f4d4a35a",
        )

    def test_fake_canonical_banner_cannot_bypass_digest_policy(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            fake = Path(directory) / "qemu-system-riscv64"
            fake.write_text(
                "#!/bin/sh\necho 'QEMU emulator version 9.2.4'\n",
                encoding="utf-8",
            )
            fake.chmod(0o755)
            result = self.invoke(
                "exec",
                "--canonical-path",
                str(fake),
                "--required-sha256",
                "0" * 64,
                "--",
                "--version",
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("digest mismatch", result.stderr)
            self.assertNotIn("QEMU emulator version 9.2.4", result.stdout)

    def test_symlink_is_resolved_once_to_an_absolute_canonical_path(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "approved-qemu"
            target.write_bytes(b"approved-object")
            link = root / "qemu"
            link.symlink_to(target)
            result = self.invoke("resolve", "--candidate", str(link))
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(Path(result.stdout.strip()), target.resolve(strict=True))

    def test_open_verified_object_survives_path_replacement_without_toctou(self) -> None:
        module = self.load_module()
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "qemu"
            approved = b"approved-object"
            path.write_bytes(approved)
            path.chmod(0o755)
            required = hashlib.sha256(approved).hexdigest()
            descriptor, observed = module.open_verified_executable(path, required)
            try:
                replacement = path.with_name("replacement")
                replacement.write_bytes(b"replacement-object")
                replacement.chmod(0o755)
                os.replace(replacement, path)
                self.assertEqual(observed, required)
                self.assertEqual(module.digest_descriptor(descriptor), required)
                self.assertNotEqual(os.fstat(descriptor).st_ino, path.stat().st_ino)
            finally:
                os.close(descriptor)

    def test_runtime_contract_accepts_complete_bound_identity(self) -> None:
        contract = self.load_contract()
        with tempfile.TemporaryDirectory() as directory:
            metadata, policy, _, _ = self.make_bound_identity(Path(directory))
            contract.validate_runtime_identity(
                metadata, policy, "rv", verify_files=True
            )

    def test_runtime_contract_accepts_only_honest_partial_prestart_identity(self) -> None:
        contract = self.load_contract()
        with tempfile.TemporaryDirectory() as directory:
            metadata, policy, _, _ = self.make_bound_identity(Path(directory))
            qemu = metadata["runtime_identity"]["qemu"]
            qemu["canonical_path"] = None
            qemu["observed_banner"] = None
            qemu["observed_sha256"] = None
            metadata["qemu_binary"] = "approved-qemu"
            metadata["runtime_identity"]["guest_artifact"] = None
            metadata["runtime_identity"]["qemu_launch_argv"] = None
            contract.validate_runtime_identity(
                metadata,
                policy,
                "rv",
                verify_files=False,
                require_complete=False,
            )
            with self.assertRaisesRegex(ValueError, "no verified QEMU"):
                contract.validate_runtime_identity(
                    metadata,
                    policy,
                    "rv",
                    verify_files=False,
                    require_complete=True,
                )
            qemu["observed_banner"] = "QEMU emulator version 9.2.4"
            with self.assertRaisesRegex(ValueError, "unverified observation"):
                contract.validate_runtime_identity(
                    metadata,
                    policy,
                    "rv",
                    verify_files=False,
                    require_complete=False,
                )

    def test_runtime_contract_rejects_qemu_digest_mismatch(self) -> None:
        contract = self.load_contract()
        with tempfile.TemporaryDirectory() as directory:
            metadata, policy, _, _ = self.make_bound_identity(Path(directory))
            metadata["runtime_identity"]["qemu"]["observed_sha256"] = "0" * 64
            with self.assertRaisesRegex(ValueError, "QEMU digest"):
                contract.validate_runtime_identity(
                    metadata, policy, "rv", verify_files=True
                )

    def test_runtime_contract_rejects_artifact_digest_path_and_argv_tampering(self) -> None:
        contract = self.load_contract()
        mutations = (
            (
                lambda identity: identity["guest_artifact"].__setitem__(
                    "sha256", "0" * 64
                ),
                "artifact digest",
            ),
            (
                lambda identity: identity["guest_artifact"].__setitem__(
                    "canonical_path", "/tmp/not-the-built-artifact"
                ),
                "artifact canonical path",
            ),
            (
                lambda identity: identity["qemu_launch_argv"].__setitem__(
                    identity["qemu_launch_argv"].index("-kernel") + 1,
                    "/tmp/not-the-launched-artifact",
                ),
                "QEMU launch artifact",
            ),
        )
        for mutate, message in mutations:
            with self.subTest(message=message), tempfile.TemporaryDirectory() as directory:
                metadata, policy, _, _ = self.make_bound_identity(Path(directory))
                mutate(metadata["runtime_identity"])
                with self.assertRaisesRegex(ValueError, message):
                    contract.validate_runtime_identity(
                        metadata, policy, "rv", verify_files=True
                    )

    def test_collector_binds_and_finalizes_verified_runtime_identity(self) -> None:
        contract = self.load_contract()
        with tempfile.TemporaryDirectory() as directory:
            repository, run, metadata, qemu, artifact = self.make_collector_fixture(
                Path(directory)
            )
            self.collect_and_bind(repository, run, metadata, qemu, artifact)
            finalized = self.invoke_collector(
                "--repo-root",
                str(repository),
                "--output",
                str(metadata),
                "--finalize",
            )
            self.assertEqual(finalized.returncode, 0, finalized.stdout + finalized.stderr)
            value = json.loads(metadata.read_text(encoding="utf-8"))
            self.assertEqual(value["schema"], 4)
            self.assertTrue(value["provenance_stable"])
            contract.validate_runtime_identity(
                value, run / "runtime-policy.json", "rv", verify_files=True
            )

    def test_collector_finalize_rejects_replaced_qemu_and_artifact(self) -> None:
        for target_name, expected in (
            ("qemu", "QEMU digest"),
            ("artifact", "artifact digest"),
        ):
            with self.subTest(target=target_name), tempfile.TemporaryDirectory() as directory:
                repository, run, metadata, qemu, artifact = self.make_collector_fixture(
                    Path(directory)
                )
                self.collect_and_bind(repository, run, metadata, qemu, artifact)
                target = qemu if target_name == "qemu" else artifact
                target.write_bytes(b"replacement-object")
                target.chmod(0o755)
                finalized = self.invoke_collector(
                    "--repo-root",
                    str(repository),
                    "--output",
                    str(metadata),
                    "--finalize",
                )
                self.assertNotEqual(
                    finalized.returncode, 0, finalized.stdout + finalized.stderr
                )
                value = json.loads(metadata.read_text(encoding="utf-8"))
                self.assertFalse(value["provenance_stable"])
                self.assertTrue(
                    any(expected in error for error in value["collection_errors"]),
                    value["collection_errors"],
                )


if __name__ == "__main__":
    unittest.main()
