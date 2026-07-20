#!/usr/bin/env python3
"""Mutation fixtures for the strict final-2026 CAgent/BuildStorm parser."""

from __future__ import annotations

import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evaluation"))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from parse_final_2026_results import validate_final_2026_output
import run_suite as suite_runner


REPO_ROOT = Path(__file__).resolve().parents[2]


CAGENT_CASES = (
    ("factorial", 13.5, 20_000),
    ("date", 13.5, 20_000),
    ("network", 20.0, 25_000),
    ("cpu", 13.5, 20_000),
    ("kernel", 13.5, 20_000),
    ("fs-create", 20.0, 25_000),
    ("fs-readwrite", 20.0, 30_000),
    ("fs-directory", 20.0, 30_000),
    ("fs-search", 27.0, 35_000),
    ("fs-usage", 20.0, 25_000),
)


def group_output(group: str, body: list[str]) -> str:
    return "\n".join(
        [
            f"#### OS COMP TEST GROUP START {group} ####",
            *body,
            f"#### OS COMP TEST GROUP END {group} ####",
            "",
        ]
    )


def cagent_output(
    *,
    slow: bool = False,
    rejected: str | None = None,
    lifecycle_label: str = "cagent-glibc",
) -> str:
    records = []
    for name, _weight, timeout_ms in reversed(CAGENT_CASES):
        elapsed_ms = timeout_ms // 2 + 1 if slow else timeout_ms // 4
        status = "reject" if name == rejected else "pass"
        records.append(f"testcase cagent {name} {status} {elapsed_ms}")
    return group_output(lifecycle_label, records)


def buildstorm_output(
    *,
    toolchain: str = "ok",
    minibuild: str = "ok",
    compile_ok: str = "true",
    elapsed: str = "100.00",
    cores: str = "8",
    size: str = "600000",
    arch: str = "riscv64",
) -> str:
    compile_record = (
        "BUILDSTORM_COMPILE mode=multi "
        f"ok={compile_ok} elapsed_s={elapsed} cores={cores} bytes={size} arch={arch}"
    )
    if compile_ok == "false":
        compile_record += " rc=1"
    return group_output(
        "buildstorm",
        [
            f"BUILDSTORM_TOOLCHAIN {toolchain}",
            f"BUILDSTORM_MINIBUILD {minibuild}",
            "BUILDSTORM_BEGIN mode=multi",
            compile_record,
        ],
    )


class Final2026ResultValidationTest(unittest.TestCase):
    def validate(
        self,
        stdout: str,
        *,
        group: str,
        arch: str = "riscv64",
        stderr: str = "",
        baseline: float = 400.0,
        group_label: str | None = None,
    ) -> dict[str, object]:
        return validate_final_2026_output(
            stdout,
            stderr,
            expected_group=group,
            expected_group_label=group_label or (
                "cagent-glibc" if group == "cagent" else "buildstorm"
            ),
            expected_arch=arch,
            buildstorm_baseline_seconds=baseline,
        )

    def test_complete_fast_cagent_output_passes_with_reference_score(self) -> None:
        result = self.validate(cagent_output(), group="cagent")
        self.assertEqual(result["status"], "PASS")
        self.assertEqual(result["score"], 199.1)
        self.assertEqual(result["max_scripted_score"], 199.1)
        self.assertEqual(result["observed_case_count"], 10)

    def test_complete_slow_cagent_output_has_no_bonus(self) -> None:
        result = self.validate(cagent_output(slow=True), group="cagent")
        self.assertEqual(result["status"], "PASS")
        self.assertEqual(result["score"], 181.0)

    def test_cagent_reject_is_a_real_failure(self) -> None:
        result = self.validate(cagent_output(rejected="network"), group="cagent")
        self.assertEqual(result["status"], "FAIL")
        self.assertIn("network", result["failed_items"])
        self.assertLess(result["score"], result["max_scripted_score"])

    def test_cagent_missing_identity_is_an_error(self) -> None:
        output = cagent_output().replace("testcase cagent factorial pass 5000\n", "")
        result = self.validate(output, group="cagent")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("missing" in error for error in result["errors"]))

    def test_cagent_duplicate_identity_is_an_error(self) -> None:
        output = cagent_output().replace(
            "#### OS COMP TEST GROUP END cagent-glibc ####",
            "testcase cagent factorial pass 1\n#### OS COMP TEST GROUP END cagent-glibc ####",
        )
        result = self.validate(output, group="cagent")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("duplicate" in error for error in result["errors"]))

    def test_cagent_unknown_identity_is_an_error(self) -> None:
        output = cagent_output().replace(
            "#### OS COMP TEST GROUP END cagent-glibc ####",
            "testcase cagent invented pass 1\n#### OS COMP TEST GROUP END cagent-glibc ####",
        )
        result = self.validate(output, group="cagent")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("unknown" in error for error in result["errors"]))

    def test_cagent_malformed_record_is_an_error(self) -> None:
        output = cagent_output().replace(
            "testcase cagent factorial pass 5000",
            "testcase cagent factorial maybe 5000",
        )
        result = self.validate(output, group="cagent")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("malformed" in error for error in result["errors"]))

    def test_cagent_requires_exact_group_lifecycle(self) -> None:
        output = cagent_output().replace(
            "#### OS COMP TEST GROUP END cagent-glibc ####", ""
        )
        result = self.validate(output, group="cagent")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("lifecycle" in error for error in result["errors"]))

    def test_cagent_protocol_records_on_stderr_are_an_error(self) -> None:
        result = self.validate(
            cagent_output(),
            group="cagent",
            stderr="testcase cagent factorial pass 1\n",
        )
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("stderr" in error for error in result["errors"]))

    def test_cagent_fatal_evidence_after_end_is_an_error(self) -> None:
        output = cagent_output() + "Kernel panic: fatal trap\n"
        result = self.validate(output, group="cagent")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("fatal" in error for error in result["errors"]))

    def test_complete_buildstorm_output_passes_with_reference_score(self) -> None:
        result = self.validate(buildstorm_output(), group="buildstorm")
        self.assertEqual(result["status"], "PASS")
        self.assertEqual(result["score"], 180.0)
        self.assertTrue(result["score_eligible"])

    def test_buildstorm_linear_time_score_matches_reference_judge(self) -> None:
        output = buildstorm_output(elapsed="600.0")
        result = self.validate(output, group="buildstorm")
        self.assertEqual(result["status"], "PASS")
        self.assertEqual(result["score"], 120.0)

    def test_buildstorm_zero_time_score_is_failure_not_pass(self) -> None:
        output = buildstorm_output(elapsed="800.0")
        result = self.validate(output, group="buildstorm")
        self.assertEqual(result["status"], "FAIL")
        self.assertIn("compile-time", result["failed_items"])
        self.assertEqual(result["score"], 60.0)

    def test_buildstorm_toolchain_failure_is_preserved(self) -> None:
        result = self.validate(
            buildstorm_output(toolchain="fail"), group="buildstorm"
        )
        self.assertEqual(result["status"], "FAIL")
        self.assertIn("toolchain", result["failed_items"])

    def test_buildstorm_minibuild_failure_is_preserved(self) -> None:
        result = self.validate(
            buildstorm_output(minibuild="fail"), group="buildstorm"
        )
        self.assertEqual(result["status"], "FAIL")
        self.assertIn("minibuild", result["failed_items"])

    def test_buildstorm_compile_failure_is_preserved(self) -> None:
        result = self.validate(
            buildstorm_output(compile_ok="false", elapsed="12.0", size="0"),
            group="buildstorm",
        )
        self.assertEqual(result["status"], "FAIL")
        self.assertIn("compile", result["failed_items"])

    def test_buildstorm_wrong_core_count_is_ineligible_error(self) -> None:
        result = self.validate(
            buildstorm_output(cores="2"), group="buildstorm"
        )
        self.assertEqual(result["status"], "ERROR")
        self.assertFalse(result["score_eligible"])
        self.assertTrue(any("cores" in error for error in result["errors"]))

    def test_buildstorm_architecture_mismatch_is_an_error(self) -> None:
        result = self.validate(
            buildstorm_output(arch="loongarch64"),
            group="buildstorm",
            arch="riscv64",
        )
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("arch" in error for error in result["errors"]))

    def test_buildstorm_success_cannot_claim_small_artifact(self) -> None:
        result = self.validate(
            buildstorm_output(size="499999"), group="buildstorm"
        )
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("bytes" in error for error in result["errors"]))

    def test_buildstorm_duplicate_compile_record_is_an_error(self) -> None:
        record = (
            "BUILDSTORM_COMPILE mode=multi ok=true elapsed_s=1 "
            "cores=8 bytes=600000 arch=riscv64"
        )
        output = buildstorm_output().replace(
            "#### OS COMP TEST GROUP END buildstorm ####",
            f"{record}\n#### OS COMP TEST GROUP END buildstorm ####",
        )
        result = self.validate(output, group="buildstorm")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("duplicate" in error for error in result["errors"]))

    def test_buildstorm_malformed_numeric_field_is_an_error(self) -> None:
        result = self.validate(
            buildstorm_output(elapsed="nan"), group="buildstorm"
        )
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("elapsed_s" in error for error in result["errors"]))

    def test_buildstorm_requires_group_lifecycle(self) -> None:
        output = buildstorm_output().replace(
            "#### OS COMP TEST GROUP START buildstorm ####\n", ""
        )
        result = self.validate(output, group="buildstorm")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("lifecycle" in error for error in result["errors"]))

    def test_buildstorm_fatal_evidence_after_end_is_an_error(self) -> None:
        result = self.validate(
            buildstorm_output() + "unknown trap after group completion\n",
            group="buildstorm",
        )
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("fatal" in error for error in result["errors"]))

    def test_expected_group_must_match_observed_group(self) -> None:
        result = self.validate(cagent_output(), group="buildstorm")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("group" in error for error in result["errors"]))

    def test_cagent_rejects_noncanonical_lifecycle_label(self) -> None:
        result = self.validate(
            cagent_output(lifecycle_label="cagent"),
            group="cagent",
        )
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("expected group label" in error for error in result["errors"]))

    def runner_case(
        self,
        *,
        code: str = "pass",
        contract: dict[str, object] | None = None,
    ) -> dict[str, object]:
        return {
            "id": "fixture.final-2026",
            "description": "final-2026 runner contract fixture",
            "command": [sys.executable, "-c", code],
            "cwd": "{repo}",
            "timeout_seconds": 5,
            "result_contract": contract
            or {
                "type": "final_2026",
                "expected_group": "cagent",
                "expected_group_label": "cagent-glibc",
                "expected_arch": "riscv64",
                "buildstorm_baseline_seconds": 400.0,
            },
            "required_paths": [],
            "required_commands": [],
        }

    def load_runner_contract(self, contract: dict[str, object]) -> dict[str, object]:
        case = self.runner_case(contract=contract)
        manifest = {
            "schema_version": 1,
            "baseline_ref": "origin/main",
            "profiles": {
                "fixture": {
                    "description": "final-2026 fixture",
                    "arch_policy": "none",
                    "include": [],
                    "cases": [case["id"]],
                    "arch_cases": {},
                }
            },
            "cases": [case],
        }
        with tempfile.TemporaryDirectory(prefix="final-2026-manifest-") as temporary:
            manifest_path = Path(temporary) / "manifest.json"
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            return suite_runner.load_manifest(manifest_path, REPO_ROOT)

    def test_runner_contract_accepts_cagent_pass(self) -> None:
        case = self.runner_case()
        status, _message, details = suite_runner.parse_contract(
            case, cagent_output(), ""
        )
        self.assertEqual(status, "PASS")
        self.assertEqual(details["status"], "PASS")
        self.assertEqual(details["score"], 199.1)

    def test_runner_contract_preserves_cagent_reject(self) -> None:
        case = self.runner_case()
        status, _message, details = suite_runner.parse_contract(
            case, cagent_output(rejected="network"), ""
        )
        self.assertEqual(status, "FAIL")
        self.assertEqual(details["status"], "FAIL")
        self.assertIn("network", details["failed_items"])

    def test_runner_contract_maps_parser_error_to_infrastructure_error(self) -> None:
        case = self.runner_case()
        status, _message, details = suite_runner.parse_contract(
            case,
            cagent_output().replace(
                "#### OS COMP TEST GROUP END cagent-glibc ####", ""
            ),
            "",
        )
        self.assertEqual(status, "INFRA_ERROR")
        self.assertEqual(details["status"], "ERROR")

    def test_runner_manifest_validates_final_contract_fields(self) -> None:
        valid = {
            "type": "final_2026",
            "expected_group": "cagent",
            "expected_group_label": "cagent-glibc",
            "expected_arch": "riscv64",
            "buildstorm_baseline_seconds": 400.0,
        }
        self.assertEqual(
            self.load_runner_contract(valid)["cases"][0]["result_contract"], valid
        )
        mutations = (
            (
                "missing group",
                {
                    key: value
                    for key, value in valid.items()
                    if key != "expected_group"
                },
                "expected_group",
            ),
            ("unknown group", {**valid, "expected_group": "unknown"}, "expected_group"),
            (
                "missing group label",
                {
                    key: value
                    for key, value in valid.items()
                    if key != "expected_group_label"
                },
                "expected_group_label",
            ),
            (
                "wrong group label",
                {**valid, "expected_group_label": "cagent"},
                "expected_group_label",
            ),
            (
                "missing arch",
                {
                    key: value
                    for key, value in valid.items()
                    if key != "expected_arch"
                },
                "expected_arch",
            ),
            ("unknown arch", {**valid, "expected_arch": "x86_64"}, "expected_arch"),
            ("zero baseline", {**valid, "buildstorm_baseline_seconds": 0}, "baseline"),
            (
                "nonfinite baseline",
                {**valid, "buildstorm_baseline_seconds": float("inf")},
                "non-finite",
            ),
        )
        for label, contract, expected_error in mutations:
            with self.subTest(label=label):
                with self.assertRaisesRegex(suite_runner.ManifestError, expected_error):
                    self.load_runner_contract(contract)

    def test_runner_nonzero_exit_still_validates_final_structure(self) -> None:
        code = (
            "import sys; "
            f"print({cagent_output()!r}); "
            "raise SystemExit(7)"
        )
        case = self.runner_case(code=code)
        with tempfile.TemporaryDirectory(prefix="final-2026-run-") as temporary:
            record = suite_runner.run_case(
                case,
                repo=REPO_ROOT,
                output_dir=Path(temporary),
                arch=None,
            )
        self.assertEqual(record["status"], "INFRA_ERROR")
        self.assertEqual(record["return_code"], 7)
        self.assertEqual(record["details"]["process_exit_code"], 7)
        self.assertEqual(record["details"]["status"], "PASS")

    def test_runner_inherits_only_final_provenance_environment(self) -> None:
        allowed = {
            "RV_FINAL_2026_IMG": "/fixtures/final-rv.img",
            "LA_FINAL_2026_IMG": "/fixtures/final-la.img",
            "RV_FINAL_2026_IMG_SHA256": "a" * 64,
            "LA_FINAL_2026_IMG_SHA256": "b" * 64,
            "FINAL_2026_PROTOCOL_ROOT": "/fixtures/final-protocol",
        }
        forbidden = {
            "FINAL_2026_FAKE_PASS": "1",
            "OSCOMP_SKIP_TEST_GROUPS": "cagent",
        }
        original = {name: os.environ.get(name) for name in (*allowed, *forbidden)}
        try:
            os.environ.update(allowed)
            os.environ.update(forbidden)
            environment, error = suite_runner.child_environment(
                self.runner_case(), repo=REPO_ROOT, cwd=REPO_ROOT
            )
        finally:
            for name, value in original.items():
                if value is None:
                    os.environ.pop(name, None)
                else:
                    os.environ[name] = value
        self.assertIsNone(error)
        for name, value in allowed.items():
            self.assertEqual(environment.get(name), value)
        for name in forbidden:
            self.assertNotIn(name, environment)

    def test_runner_final_contract_does_not_accept_official_scouting_inputs(self) -> None:
        case = self.runner_case()
        environment = {"OSCOMP_SKIP_TEST_GROUPS": "cagent"}
        configured = suite_runner.prepare_official_scouting_environment(
            case, environment, invocation_cwd=REPO_ROOT
        )
        self.assertEqual(configured, [])
        self.assertEqual(environment, {"OSCOMP_SKIP_TEST_GROUPS": "cagent"})

    def test_canonical_manifest_registers_four_final_cases(self) -> None:
        manifest = suite_runner.load_manifest(
            REPO_ROOT / "test/suite_manifest.json", REPO_ROOT
        )
        final_cases = {
            case["id"]: case
            for case in manifest["cases"]
            if case["result_contract"]["type"] == "final_2026"
        }
        self.assertEqual(
            set(final_cases),
            {
                "final.cagent.riscv64",
                "final.buildstorm.riscv64",
                "final.cagent.loongarch64",
                "final.buildstorm.loongarch64",
            },
        )
        for case_id, case in final_cases.items():
            group = case_id.split(".")[1]
            arch = case_id.split(".")[2]
            self.assertEqual(case["result_contract"]["expected_group"], group)
            self.assertEqual(case["result_contract"]["expected_arch"], arch)

    def test_final_profile_and_full_select_all_requested_final_cases(self) -> None:
        manifest = suite_runner.load_manifest(
            REPO_ROOT / "test/suite_manifest.json", REPO_ROOT
        )
        final_selection = suite_runner.select_cases(manifest, "final-2026", "all")
        self.assertEqual(
            [case["id"] for case in final_selection.cases],
            [
                "final.cagent.riscv64",
                "final.buildstorm.riscv64",
                "final.cagent.loongarch64",
                "final.buildstorm.loongarch64",
            ],
        )
        full_ids = {
            case["id"]
            for case in suite_runner.select_cases(manifest, "full", "all").cases
        }
        self.assertTrue({"official.riscv64", "official.loongarch64"} <= full_ids)
        self.assertTrue(set(case["id"] for case in final_selection.cases) <= full_ids)

    def test_canonical_final_case_execution_fields_are_locked(self) -> None:
        source = REPO_ROOT / "test/suite_manifest.json"
        manifest = json.loads(source.read_text(encoding="utf-8"))
        case = next(
            case for case in manifest["cases"] if case["id"] == "final.cagent.riscv64"
        )
        mutations = (
            ("command", lambda value: value["command"].append("--untrusted")),
            (
                "environment",
                lambda value: value["environment"].update({"OSCOMP_SKIP_TEST_GROUPS": "cagent"}),
            ),
            ("timeout", lambda value: value.update({"timeout_seconds": 1})),
        )
        for label, mutate in mutations:
            with self.subTest(label=label), tempfile.TemporaryDirectory(
                prefix="final-canonical-mutation-"
            ) as temporary:
                mutated = json.loads(json.dumps(manifest))
                target = next(
                    item
                    for item in mutated["cases"]
                    if item["id"] == "final.cagent.riscv64"
                )
                mutate(target)
                root = Path(temporary)
                manifest_path = root / "manifest.json"
                manifest_path.write_text(json.dumps(mutated), encoding="utf-8")
                with self.assertRaisesRegex(
                    suite_runner.ManifestError, "canonical final-2026"
                ):
                    suite_runner.load_manifest(manifest_path, REPO_ROOT)


if __name__ == "__main__":
    unittest.main()
