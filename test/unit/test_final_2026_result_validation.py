#!/usr/bin/env python3
"""Mutation fixtures for the strict final-2026 CAgent/BuildStorm parser."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evaluation"))

from parse_final_2026_results import validate_final_2026_output


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


def cagent_output(*, slow: bool = False, rejected: str | None = None) -> str:
    records = []
    for name, _weight, timeout_ms in reversed(CAGENT_CASES):
        elapsed_ms = timeout_ms // 2 + 1 if slow else timeout_ms // 4
        status = "reject" if name == rejected else "pass"
        records.append(f"testcase cagent {name} {status} {elapsed_ms}")
    return group_output("cagent", records)


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
    ) -> dict[str, object]:
        return validate_final_2026_output(
            stdout,
            stderr,
            expected_group=group,
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
            "#### OS COMP TEST GROUP END cagent ####",
            "testcase cagent factorial pass 1\n#### OS COMP TEST GROUP END cagent ####",
        )
        result = self.validate(output, group="cagent")
        self.assertEqual(result["status"], "ERROR")
        self.assertTrue(any("duplicate" in error for error in result["errors"]))

    def test_cagent_unknown_identity_is_an_error(self) -> None:
        output = cagent_output().replace(
            "#### OS COMP TEST GROUP END cagent ####",
            "testcase cagent invented pass 1\n#### OS COMP TEST GROUP END cagent ####",
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
            "#### OS COMP TEST GROUP END cagent ####", ""
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


if __name__ == "__main__":
    unittest.main()
