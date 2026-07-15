#!/usr/bin/env python3
"""Mutation tests for the competition semantic-evidence workflow guard."""

from __future__ import annotations

import shutil
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "checks"))

import check_competition_semantic_evidence as guard


ROOT = Path(__file__).resolve().parents[2]


class CompetitionSemanticEvidenceWorkflowGuardTest(unittest.TestCase):
    def copy_tree(self, temporary: str) -> Path:
        root = Path(temporary)
        destination = root / ".github" / "workflows"
        destination.mkdir(parents=True)
        for name in ("build.yml", "test.yml", "docs.yml"):
            shutil.copy2(ROOT / ".github" / "workflows" / name, destination / name)
        shutil.copy2(ROOT / "Makefile", root / "Makefile")
        evidence = root / "test" / "evidence"
        evidence.mkdir(parents=True)
        for name in ("setup_qemu.sh", "semantic_evidence_manifest.json"):
            shutil.copy2(ROOT / "test" / "evidence" / name, evidence / name)
        docs = root / "docs"
        docs.mkdir()
        shutil.copy2(
            ROOT / "docs" / "pr3-semantic-evidence.md",
            docs / "pr3-semantic-evidence.md",
        )
        return root

    def mutate(self, root: Path, name: str, old: str, new: str) -> list[str]:
        path = root / ".github" / "workflows" / name
        text = path.read_text(encoding="utf-8")
        self.assertIn(old, text)
        path.write_text(text.replace(old, new, 1), encoding="utf-8")
        return guard.scan(root)

    def mutate_path(self, root: Path, relative: str, old: str, new: str) -> list[str]:
        path = root / relative
        text = path.read_text(encoding="utf-8")
        self.assertIn(old, text)
        path.write_text(text.replace(old, new, 1), encoding="utf-8")
        return guard.scan(root)

    def test_current_tree_passes(self) -> None:
        self.assertEqual(guard.scan(ROOT), [])

    def test_detects_mutable_action_reference(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5",
                "actions/checkout@v4",
            )
            self.assertTrue(any("mutable" in item for item in findings))

    def test_detects_continue_on_error_in_required_job(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "  pr3-infrastructure:\n",
                "  pr3-infrastructure:\n    continue-on-error: true\n",
            )
            self.assertTrue(any("pr3-infrastructure uses continue-on-error" in item for item in findings))

    def test_detects_fixed_application_tests_weakened_to_observational(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "  app-test:\n    name: Application tests (${{ matrix.arch }}, fixed-required)\n",
                "  app-test:\n    name: Application tests (${{ matrix.arch }}, fixed-required)\n"
                "    continue-on-error: true\n",
            )
            self.assertTrue(any("application tests must remain hard-fail" in item for item in findings))

    def test_detects_missing_architecture_runtime_check(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "PR3 LA64 fixed build + runtime smoke (required)",
                "LA smoke",
            )
            self.assertTrue(any("missing architecture check name" in item for item in findings))

    def test_detects_incomplete_aggregate_shards(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "path: build/pr3-evidence/la64",
                "path: build/pr3-evidence/missing-la64",
            )
            self.assertTrue(any("build/pr3-evidence/la64" in item for item in findings))

    def test_detects_missing_job_summary(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "$GITHUB_STEP_SUMMARY",
                "$REMOVED_STEP_SUMMARY",
            )
            self.assertTrue(any("GITHUB_STEP_SUMMARY" in item for item in findings))

    def test_detects_workspace_local_qemu_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "${{ runner.temp }}/orays-pr3-qemu-9.2.4",
                "${{ github.workspace }}/.cache/qemu-9.2.4",
            )
            self.assertTrue(any("exactly three" in item for item in findings))

    def test_detects_runner_context_in_workflow_env(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "env:\n  FIXED_RUST_TOOLCHAIN:",
                "env:\n  QEMU_PREFIX: ${{ runner.temp }}/forbidden\n  FIXED_RUST_TOOLCHAIN:",
            )
            self.assertTrue(any("workflow-level" in item for item in findings))

    def test_detects_runner_context_in_job_env(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "    runs-on: ubuntu-24.04\n",
                "    runs-on: ubuntu-24.04\n    env:\n      QEMU_PREFIX: ${{ runner.temp }}/forbidden\n",
            )
            self.assertTrue(any("job-level env" in item for item in findings))

    def test_detects_cached_qemu_binary_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "          path: build/qemu-source/qemu-9.2.4.tar.xz\n",
                "          path: |\n"
                "            ${{ runner.temp }}/orays-pr3-qemu-9.2.4\n"
                "            build/qemu-source/qemu-9.2.4.tar.xz\n",
            )
            self.assertTrue(any("binary prefix must never be cached" in item for item in findings))

    def test_detects_unbounded_qemu_build_or_non_always_upload(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(root, "test.yml", "--timeout 4800", "--timeout 9999")
            self.assertTrue(any("bounded/log-preserving" in item for item in findings))
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "name: Upload verified QEMU\n        if: ${{ always() }}",
                "name: Upload verified QEMU\n        if: ${{ success() }}",
            )
            self.assertTrue(any("bounded/log-preserving" in item for item in findings))

    def test_detects_missing_offline_qemu_fdt_dependency(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "libpixman-1-dev libfdt-dev zlib1g-dev",
                "libpixman-1-dev zlib1g-dev",
            )
            self.assertTrue(
                any("QEMU baseline" in item and "libfdt-dev" in item for item in findings)
            )

    def test_detects_missing_qemu_consumer_runtime_dependency(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "libfdt1 libglib2.0-0t64 libpixman-1-0 zlib1g",
                "libglib2.0-0t64 libpixman-1-0 zlib1g",
            )
            self.assertTrue(
                any("consumer runtime dependency" in item and "libfdt1" in item for item in findings)
            )

    def test_detects_qemu_consumer_owner_restore_regression(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                'tar --no-same-owner -C "$QEMU_PREFIX" -xzf ',
                'tar -C "$QEMU_PREFIX" -xzf ',
            )
            self.assertTrue(
                any("consumer artifact extraction" in item for item in findings)
            )

    def test_safe_consumer_extractor_comment_cannot_mask_unsafe_command(self) -> None:
        safe = (
            'tar --no-same-owner -C "$QEMU_PREFIX" -xzf '
            "build/pr3-qemu-artifact/qemu-9.2.4.tar.gz"
        )
        unsafe = (
            'tar -C "$QEMU_PREFIX" -xzf '
            "build/pr3-qemu-artifact/qemu-9.2.4.tar.gz"
        )
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                safe,
                unsafe + "\n          # " + safe,
            )
            self.assertTrue(any("consumer artifact extraction" in item for item in findings))

    def test_extra_unsafe_consumer_extractor_is_rejected(self) -> None:
        safe_command = (
            'tar --no-same-owner -C "$QEMU_PREFIX" -xzf '
            "build/pr3-qemu-artifact/qemu-9.2.4.tar.gz"
        )
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                safe_command,
                safe_command
                + '\n          tar -C "$QEMU_PREFIX/extra" -xzf '
                + "build/pr3-qemu-artifact/qemu-9.2.4.tar.gz",
            )
            self.assertTrue(any("consumer artifact extraction" in item for item in findings))

    def test_detects_missing_or_wrong_manifest_qemu_version_constraint(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "test/evidence/semantic_evidence_manifest.json",
                '      "required_version": "QEMU emulator version 9.2.4"\n',
                '      "required_version": "QEMU emulator version 6.2.0"\n',
            )
            self.assertTrue(any("exact required QEMU version" in item for item in findings))
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "test/evidence/semantic_evidence_manifest.json",
                '        "qemu-rv64"\n',
                '        "python3"\n',
            )
            self.assertTrue(any("smoke.rv64.abi must require qemu-rv64" in item for item in findings))

    def test_detects_inexact_documented_branch_protection_names(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "docs/pr3-semantic-evidence.md",
                "`Application tests (riscv64, fixed-required)`",
                "`Application tests (<arch>, fixed-required)`",
            )
            self.assertTrue(
                any("missing exact CI check classification" in item for item in findings)
            )
            self.assertTrue(any("<arch> placeholder" in item for item in findings))

    def test_detects_runtime_skipped_success_on_qemu_producer_failure(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "needs: pr3-qemu-baseline\n    if: ${{ always() }}",
                "needs: pr3-qemu-baseline\n    if: ${{ success() }}",
            )
            self.assertTrue(any("skipped-success" in item for item in findings))

    def test_detects_lost_qemu_producer_conclusion(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                'test "${{ needs.pr3-qemu-baseline.result }}" = success',
                "true # producer conclusion lost",
            )
            self.assertTrue(any("preserve the QEMU producer conclusion" in item for item in findings))

    def test_detects_timeout_budget_regression(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "test.yml",
                "  pr3-runtime:\n    name:",
                "  pr3-runtime:\n    timeout-minutes: 10\n    name:",
            )
            self.assertTrue(any("pr3-runtime timeout" in item for item in findings))

    def test_detects_qemu_path_or_internal_network_bypass(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "Makefile",
                '"$$PR3_QEMU_RV_BIN" -machine virt',
                "qemu-system-riscv64 -machine virt",
            )
            self.assertTrue(any("exact runtime token" in item for item in findings))
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "Makefile",
                "-netdev hubport,id=net,hubid=0",
                "-netdev user,id=net",
            )
            self.assertTrue(any("internal hub" in item or "exact runtime token" in item for item in findings))

    def test_detects_setup_network_download_reenable(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "test/evidence/setup_qemu.sh",
                "--disable-download",
                "--enable-download",
            )
            self.assertTrue(any("offline configure" in item for item in findings))

    def test_detects_qemu_archive_owner_restore_regression(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "test/evidence/setup_qemu.sh",
                'tar --no-same-owner -xf "$archive" -C "$work_dir"',
                'tar -xf "$archive" -C "$work_dir"',
            )
            self.assertTrue(any("active archive extraction" in item for item in findings))

    def test_safe_qemu_extractor_comment_cannot_mask_unsafe_command(self) -> None:
        safe = 'tar --no-same-owner -xf "$archive" -C "$work_dir"'
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "test/evidence/setup_qemu.sh",
                safe,
                'tar -xf "$archive" -C "$work_dir"\n# ' + safe,
            )
            self.assertTrue(any("active archive extraction" in item for item in findings))

    def test_extra_unsafe_qemu_extractor_is_rejected(self) -> None:
        safe = 'tar --no-same-owner -xf "$archive" -C "$work_dir"'
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate_path(
                root,
                "test/evidence/setup_qemu.sh",
                safe,
                safe + '\ntar -xf "$archive" -C "$work_dir/extra"',
            )
            self.assertTrue(any("active archive extraction" in item for item in findings))

    def test_detects_implicit_make_default_build(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "build.yml",
                "run: make ARCH=${{ matrix.arch }} A=user/shell build",
                "run: make ARCH=${{ matrix.arch }} A=user/shell",
            )
            self.assertTrue(any("explicit build target" in item for item in findings))

    def test_detects_fixed_compatibility_lane_weakened_to_observational(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "build.yml",
                "  build-for-other-platforms:\n    name: Other platforms (${{ matrix.lane }})\n"
                "    runs-on: ubuntu-24.04\n    timeout-minutes: 60\n"
                "    continue-on-error: ${{ matrix.lane == 'moving-nightly-observational' }}",
                "  build-for-other-platforms:\n    name: Other platforms (${{ matrix.lane }})\n"
                "    runs-on: ubuntu-24.04\n    timeout-minutes: 60\n"
                "    continue-on-error: true",
            )
            self.assertTrue(
                any("only moving nightly observational" in item for item in findings)
            )

    def test_detects_moving_other_platform_dependency(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "build.yml",
                'CARGO_HOME="$PWD/cargo-home" cargo add --offline --path '
                "vendor/cargo/axplat-aarch64-raspi",
                'CARGO_HOME="$PWD/cargo-home" cargo add --offline '
                "axplat-aarch64-raspi --git https://github.com/"
                "arceos-org/axhal_crates.git",
            )
            self.assertTrue(
                any("repository vendor" in item or "moving git dependency" in item for item in findings)
            )

    def test_detects_host_only_cargo_axplat_plugin_dependency(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "build.yml",
                'CARGO_HOME="$PWD/cargo-home" cargo add --offline --path '
                "vendor/cargo/axplat-aarch64-raspi",
                "cargo axplat add --path vendor/cargo/axplat-aarch64-raspi",
            )
            self.assertTrue(any("host cargo-axplat plugin" in item for item in findings))

    def test_detects_online_or_host_cargo_home_platform_add(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "build.yml",
                'CARGO_HOME="$PWD/cargo-home" cargo add --offline --path '
                "vendor/cargo/axplat-aarch64-raspi",
                "cargo add --path vendor/cargo/axplat-aarch64-raspi",
            )
            self.assertTrue(
                any("offline repository cargo home" in item for item in findings)
            )

    def test_detects_docs_write_permission_expansion(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_tree(temporary)
            findings = self.mutate(
                root,
                "docs.yml",
                "permissions:\n  contents: read",
                "permissions:\n  contents: write",
            )
            self.assertTrue(any("contents: read" in item or "exactly once" in item for item in findings))


if __name__ == "__main__":
    unittest.main()
