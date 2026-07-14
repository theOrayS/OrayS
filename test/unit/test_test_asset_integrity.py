#!/usr/bin/env python3
"""Unit tests for canonical test asset and registration integrity."""

from __future__ import annotations

import json
import shutil
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "checks"))

import check_test_asset_integrity as guard

REPO_ROOT = Path(__file__).resolve().parents[2]


class TestAssetIntegrityTest(unittest.TestCase):
    def make_tree(self) -> Path:
        temporary = Path(tempfile.mkdtemp(prefix="test-asset-integrity-"))
        self.addCleanup(lambda: shutil.rmtree(temporary, ignore_errors=True))
        shutil.copytree(REPO_ROOT / "test", temporary / "test", ignore=shutil.ignore_patterns("output", "__pycache__"))
        (temporary / "test/output").mkdir(parents=True)
        shutil.copy2(REPO_ROOT / "test/output/.gitignore", temporary / "test/output/.gitignore")
        (temporary / "scripts").mkdir()
        shutil.copy2(REPO_ROOT / "run-eval.sh", temporary / "run-eval.sh")
        return temporary

    def load_tree_manifest(self, tree: Path) -> tuple[Path, dict[str, object]]:
        path = tree / "test/suite_manifest.json"
        return path, json.loads(path.read_text(encoding="utf-8"))

    def write_tree_manifest(self, path: Path, manifest: dict[str, object]) -> None:
        path.write_text(json.dumps(manifest), encoding="utf-8")

    def test_current_repository_passes(self) -> None:
        self.assertEqual(guard.scan_repo(REPO_ROOT), [])

    def test_unregistered_check_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/checks/check_unregistered_behavior.py").write_text("print('PASS')\n", encoding="utf-8")
        findings = guard.scan_repo(tree)
        self.assertTrue(any("unregistered check" in finding for finding in findings), findings)

    def test_legacy_sequence_named_script_is_detected(self) -> None:
        tree = self.make_tree()
        legacy_suffix = "g" + "099"
        (tree / f"scripts/check_{legacy_suffix}_demo.py").write_text(
            "print('PASS')\n", encoding="utf-8"
        )
        findings = guard.scan_repo(tree)
        self.assertTrue(any("legacy sequence-named" in finding for finding in findings), findings)

    def test_each_retired_non_sequence_asset_is_detected(self) -> None:
        for retired_path in guard.RETIRED_SCRIPT_ASSETS:
            with self.subTest(path=retired_path):
                tree = self.make_tree()
                path = tree / retired_path
                if retired_path.suffix:
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_text("retired implementation\n", encoding="utf-8")
                else:
                    path.mkdir(parents=True)
                    (path / "retired.md").write_text("retired fixture\n", encoding="utf-8")
                findings = guard.scan_repo(tree)
                self.assertTrue(
                    any("retired test/evaluation assets" in finding for finding in findings),
                    findings,
                )

    def test_historical_id_in_canonical_content_is_detected(self) -> None:
        tree = self.make_tree()
        historical_id = "G" + "099"
        (tree / "test/README.md").write_text(
            f"normal output {historical_id}\n", encoding="utf-8"
        )
        findings = guard.scan_repo(tree)
        self.assertTrue(any("historical sequence ID" in finding for finding in findings), findings)

    def test_underscored_historical_id_in_canonical_filename_is_detected(self) -> None:
        tree = self.make_tree()
        historical_suffix = "g" + "099"
        path = tree / f"test/checks/check_{historical_suffix}_demo.py"
        path.write_text("print('PASS')\n", encoding="utf-8")
        findings = guard.scan_repo(tree)
        self.assertTrue(
            any("historical sequence ID in canonical path" in finding for finding in findings),
            findings,
        )

    def test_underscored_historical_id_in_canonical_content_is_detected(self) -> None:
        tree = self.make_tree()
        historical_suffix = "g" + "099"
        (tree / "test/README.md").write_text(
            f"normal output check_{historical_suffix}_demo\n",
            encoding="utf-8",
        )
        findings = guard.scan_repo(tree)
        self.assertTrue(
            any("historical sequence ID in canonical content" in finding for finding in findings),
            findings,
        )

    def test_missing_manifest_registration_is_detected(self) -> None:
        tree = self.make_tree()
        manifest_path = tree / "test/suite_manifest.json"
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        manifest["profiles"]["checks"]["cases"].pop()
        manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
        findings = guard.scan_repo(tree)
        self.assertTrue(any("unregistered check" in finding for finding in findings), findings)

    def test_output_ignore_contract_is_required(self) -> None:
        tree = self.make_tree()
        (tree / "test/output/.gitignore").write_text("\n", encoding="utf-8")
        findings = guard.scan_repo(tree)
        self.assertTrue(any("ignore all generated output" in finding for finding in findings), findings)

    def test_malformed_manifest_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/suite_manifest.json").write_text("{malformed", encoding="utf-8")
        findings = guard.scan_repo(tree)
        self.assertTrue(any("cannot read canonical manifest" in finding for finding in findings), findings)

    def test_non_utf8_manifest_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/suite_manifest.json").write_bytes(b"{\xff}\n")
        findings = guard.scan_repo(tree)
        self.assertTrue(any("cannot read canonical manifest" in finding for finding in findings), findings)

    def test_non_object_manifest_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/suite_manifest.json").write_text("[]\n", encoding="utf-8")
        findings = guard.scan_repo(tree)
        self.assertTrue(any("manifest must be a JSON object" in finding for finding in findings), findings)

    def test_missing_profile_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        del manifest["profiles"]["checks"]  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("usable checks profile" in finding for finding in findings), findings)

    def test_missing_full_profile_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        del manifest["profiles"]["full"]  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("missing required profiles" in finding for finding in findings), findings)

    def test_unknown_registered_case_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        manifest["profiles"]["checks"]["cases"][0] = "check.unknown"  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("references unknown case" in finding for finding in findings), findings)

    def test_registration_without_argv_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        manifest["cases"][0]["command"] = None  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("has no argv command" in finding for finding in findings), findings)

    def test_registration_without_python_path_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        manifest["cases"][0]["command"] = ["python3", "-c", "print('PASS')"]  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("exactly one Python implementation path" in finding for finding in findings), findings)

    def test_registration_with_two_python_paths_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        manifest["cases"][0]["command"].append("{repo}/test/checks/source_scan.py")  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("exactly one Python implementation path" in finding for finding in findings), findings)

    def test_registration_path_escape_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        manifest["cases"][0]["command"][-1] = "{repo}/../outside.py"  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("implementation escapes repository" in finding for finding in findings), findings)

    def test_stale_registration_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        manifest["cases"][0]["command"][-1] = "{repo}/test/checks/check_missing.py"  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("registers missing check implementations" in finding for finding in findings), findings)

    def test_duplicate_implementation_registration_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        duplicate = dict(manifest["cases"][0])  # type: ignore[index]
        duplicate["id"] = "check.duplicate_registration"
        manifest["cases"].append(duplicate)  # type: ignore[union-attr]
        manifest["profiles"]["checks"]["cases"].append(duplicate["id"])  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("implementation path registered more than once" in finding for finding in findings), findings)

    def test_migrated_pair_cannot_disappear_with_its_registration(self) -> None:
        tree = self.make_tree()
        check_id = "check.timer_semantics"
        unit_id = "unit.timer_semantics"
        (tree / "test/checks/check_timer_semantics.py").unlink()
        (tree / "test/unit/test_timer_semantics.py").unlink()
        path, manifest = self.load_tree_manifest(tree)
        manifest["cases"] = [  # type: ignore[index]
            case
            for case in manifest["cases"]  # type: ignore[union-attr]
            if case["id"] not in {check_id, unit_id}
        ]
        manifest["profiles"]["checks"]["cases"].remove(check_id)  # type: ignore[index]
        manifest["profiles"]["unit"]["cases"].remove(unit_id)  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(
            any("missing canonical check implementations" in finding for finding in findings),
            findings,
        )
        self.assertTrue(
            any("missing canonical unit implementations" in finding for finding in findings),
            findings,
        )

    def test_missing_canonical_asset_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/evaluation/report_evaluation_failures.py").unlink()
        findings = guard.scan_repo(tree)
        self.assertTrue(any("required canonical test asset is missing" in finding for finding in findings), findings)

    def test_missing_baseline_validation_report_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/docs/baseline_validation.md").unlink()
        findings = guard.scan_repo(tree)
        self.assertTrue(
            any("test/docs/baseline_validation.md" in finding for finding in findings),
            findings,
        )

    def test_missing_output_ignore_file_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/output/.gitignore").unlink()
        findings = guard.scan_repo(tree)
        self.assertIn("test/output/.gitignore is missing", findings)

    def test_missing_root_wrapper_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "run-eval.sh").unlink()
        findings = guard.scan_repo(tree)
        self.assertIn("root official compatibility wrapper is missing", findings)

    def test_non_delegating_root_wrapper_is_detected(self) -> None:
        tree = self.make_tree()
        wrapper = tree / "run-eval.sh"
        wrapper.write_text("#!/bin/sh\nexec true\n", encoding="utf-8")
        wrapper.chmod(0o755)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("does not exec the strict canonical profile" in finding for finding in findings), findings)

    def test_root_wrapper_with_duplicated_logic_is_detected(self) -> None:
        tree = self.make_tree()
        wrapper = tree / "run-eval.sh"
        wrapper.write_text(
            "#!/bin/sh\nexec python3 test/run_suite.py --profile official --arch \"$@\"\nmake all\n",
            encoding="utf-8",
        )
        wrapper.chmod(0o755)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("contains duplicated evaluation logic" in finding for finding in findings), findings)

    def test_non_executable_root_wrapper_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "run-eval.sh").chmod(0o644)
        findings = guard.scan_repo(tree)
        self.assertIn("root official compatibility wrapper is not executable", findings)

    def test_non_executable_canonical_runner_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/evaluation/run_official_evaluation.sh").chmod(0o644)
        findings = guard.scan_repo(tree)
        self.assertIn("canonical official evaluation runner is not executable", findings)

    def test_non_executable_local_suite_runner_is_detected(self) -> None:
        tree = self.make_tree()
        (tree / "test/run_suite.py").chmod(0o644)
        findings = guard.scan_repo(tree)
        self.assertIn("canonical local suite runner is not executable", findings)

    def test_check_contract_downgrade_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        manifest["cases"][0]["result_contract"] = {"type": "exit_code"}  # type: ignore[index]
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("must use check result contract" in finding for finding in findings), findings)

    def test_unit_contract_downgrade_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        unit_id = manifest["profiles"]["unit"]["cases"][0]  # type: ignore[index]
        unit_case = next(case for case in manifest["cases"] if case["id"] == unit_id)  # type: ignore[union-attr]
        unit_case["result_contract"] = {"type": "exit_code"}
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("must use unittest result contract" in finding for finding in findings), findings)

    def test_official_contract_downgrade_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        official_id = manifest["profiles"]["official"]["arch_cases"]["rv"][0]  # type: ignore[index]
        official_case = next(case for case in manifest["cases"] if case["id"] == official_id)  # type: ignore[union-attr]
        official_case["result_contract"] = {"type": "exit_code"}
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(any("must use official result contract" in finding for finding in findings), findings)

    def test_workspace_unit_contract_downgrade_is_detected(self) -> None:
        tree = self.make_tree()
        path, manifest = self.load_tree_manifest(tree)
        unit_case = next(
            case
            for case in manifest["cases"]  # type: ignore[union-attr]
            if case["id"] == "baseline.workspace_unit_tests"
        )
        unit_case["result_contract"] = {"type": "exit_code"}
        self.write_tree_manifest(path, manifest)
        findings = guard.scan_repo(tree)
        self.assertTrue(
            any("baseline.workspace_unit_tests must use cargo_test" in finding for finding in findings),
            findings,
        )

    def test_missing_canonical_test_directory_is_detected(self) -> None:
        tree = self.make_tree()
        shutil.rmtree(tree / "test")
        findings = guard.scan_repo(tree)
        self.assertIn("canonical test directory is missing", findings)


if __name__ == "__main__":
    unittest.main()
