#!/usr/bin/env python3
"""Regression tests for LTP evaluator summary semantics."""

from __future__ import annotations

import json
import hashlib
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evaluation"))
import summarize_ltp_results as ltp_summary


class LtpSummarySemanticsTest(unittest.TestCase):
    def arch_banner(self, arch: str) -> str:
        if arch == "rv":
            return (
                "Building App: shell, Arch: riscv64, "
                "Platform: riscv64-qemu-virt, App type: rust\n"
            )
        return (
            "Building App: shell, Arch: loongarch64, "
            "Platform: loongarch64-qemu-virt, App type: rust\n"
        )

    def compact(self, log: str) -> dict:
        return ltp_summary.compact(ltp_summary.parse_log(log), arch="rv")

    def promotion_report(
        self,
        rv_log: str,
        la_log: str,
        *,
        extra_validations: list[dict] | None = None,
    ) -> dict:
        rows = []
        input_validations = []
        for arch, log in (("rv", rv_log), ("la", la_log)):
            raw_summary = ltp_summary.parse_log(log)
            data = ltp_summary.compact(raw_summary, arch=arch)
            validation = ltp_summary.strict_ltp_validation(log)
            input_validations.append(
                {
                    "path": f"{arch}.log",
                    "stderr_path": f"{arch}.stderr.log",
                    "arch": arch,
                    "status": validation["status"],
                    "group_count": validation["group_count"],
                    "error_count": validation["error_count"],
                    "failure_count": validation["failure_count"],
                }
            )
            rows.extend(
                ltp_summary.promotion_rows(
                    raw_summary,
                    data,
                    arch,
                    source_path=f"{arch}.log",
                    stderr_path=f"{arch}.stderr.log",
                    validation=validation,
                )
            )
        return ltp_summary.promotion_report(
            rows,
            required_arches={"rv", "la"},
            required_libcs={"musl", "glibc"},
            input_validations=[*input_validations, *(extra_validations or [])],
        )

    def complete_group(
        self,
        group: str,
        case: str,
        *,
        mode: str = "inline",
        marker: str = "",
    ) -> str:
        return "\n".join(
            [
                f"#### OS COMP TEST GROUP START {group} ####",
                f"ltp case list: {mode} (1 cases, timeout 30s)",
                f"========== START ltp {case} ==========",
                f"RUN LTP CASE {case}",
                marker,
                f"FAIL LTP CASE {case} : 0",
                "Pass!",
                f"LTP CASE RUNTIME {case}: 1 ms",
                f"========== END ltp {case} ==========",
                "ltp cases: 1 passed, 0 failed, 0 timed out",
                f"#### OS COMP TEST GROUP END {group} ####",
            ]
        )

    def two_libc_pass_log(self, case: str) -> str:
        return "\n".join(
            [
                self.complete_group("ltp-musl", case),
                self.complete_group("ltp-glibc", case),
            ]
        )

    def complete_strict_log(self, marker: str = "") -> str:
        return "\n".join(
            [
                "#### OS COMP TEST GROUP START ltp-musl ####",
                "ltp case list: inline (1 cases, timeout 30s)",
                "========== START ltp access01 ==========",
                "RUN LTP CASE access01",
                marker,
                "FAIL LTP CASE access01 : 0",
                "Pass!",
                "LTP CASE RUNTIME access01: 1 ms",
                "========== END ltp access01 ==========",
                "ltp cases: 1 passed, 0 failed, 0 timed out",
                "#### OS COMP TEST GROUP END ltp-musl ####",
            ]
        )

    def test_strict_validation_accepts_complete_clean_log(self) -> None:
        validation = ltp_summary.strict_ltp_validation(self.complete_strict_log())
        self.assertEqual(validation["status"], "PASS", validation)
        self.assertEqual(validation["validation_scope"], "ltp")
        self.assertEqual(validation["group_count"], 1)
        self.assertEqual(
            validation["groups"][0]["cases"],
            [
                {
                    "case": "access01",
                    "code": 0,
                    "events": ["START", "RUN", "RESULT", "PASS", "END"],
                }
            ],
        )
        two_group = ltp_summary.strict_ltp_validation(
            self.two_libc_pass_log("access01")
        )
        self.assertEqual(two_group["status"], "PASS", two_group)
        self.assertEqual(two_group["group_count"], 2)

    def test_strict_validation_rejects_empty_log(self) -> None:
        validation = ltp_summary.strict_ltp_validation("")
        self.assertEqual(validation["status"], "ERROR", validation)

    def test_strict_validation_rejects_missing_case_end(self) -> None:
        log = self.complete_strict_log().replace("========== END ltp access01 ==========\n", "")
        validation = ltp_summary.strict_ltp_validation(log)
        self.assertEqual(validation["status"], "ERROR", validation)

    def test_strict_validation_rejects_planned_executed_mismatch(self) -> None:
        log = self.complete_strict_log().replace("inline (1 cases", "inline (2 cases")
        validation = ltp_summary.strict_ltp_validation(log)
        self.assertEqual(validation["status"], "ERROR", validation)

    def test_strict_validation_rejects_tconf(self) -> None:
        validation = ltp_summary.strict_ltp_validation(
            self.complete_strict_log("access01 1 TCONF: not configured")
        )
        self.assertEqual(validation["status"], "FAIL", validation)
        stderr_validation = ltp_summary.strict_ltp_validation(
            self.complete_strict_log(),
            "kernel panic on stderr",
        )
        self.assertNotEqual(stderr_validation["status"], "PASS", stderr_validation)

    def test_strict_cli_rejects_invalid_utf8_without_hiding_clean_positive(self) -> None:
        script = Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"
        with tempfile.TemporaryDirectory() as tmp:
            clean = Path(tmp) / "rv-clean.log"
            malformed = Path(tmp) / "rv-malformed.log"
            clean_stderr_log = Path(tmp) / "rv-clean.stderr.log"
            malformed_stdout_stderr_log = Path(tmp) / "rv-malformed.stderr.log"
            malformed_stderr_stdout_log = Path(tmp) / "rv-stderr-malformed.log"
            malformed_stderr_log = Path(tmp) / "rv-stderr-malformed.stderr.log"
            la_promotion_log = Path(tmp) / "la-clean.log"
            la_stderr_log = Path(tmp) / "la-clean.stderr.log"
            wrong_strict_stderr = Path(tmp) / "la-wrong.stderr.log"
            loose_log = Path(tmp) / "loose.log"
            encoded = self.complete_strict_log().encode("utf-8")
            clean.write_bytes(encoded)
            malformed.write_bytes(encoded + b"\xff")
            clean_stderr_log.write_bytes(b"")
            malformed_stdout_stderr_log.write_bytes(b"")
            malformed_stderr_stdout_log.write_bytes(encoded)
            malformed_stderr_log.write_bytes(b"panic\xff")
            la_promotion_log.write_text(
                self.arch_banner("la") + self.two_libc_pass_log("access01")
            )
            la_stderr_log.write_bytes(b"")
            wrong_strict_stderr.write_bytes(b"")
            loose_log.write_text("RUN LTP CASE partial\nFAIL LTP CASE partial : 0\n")

            clean_result = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--strict",
                    "--json",
                    "--stderr-log",
                    str(clean_stderr_log),
                    "--process-exit-code",
                    "0",
                    str(clean),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            malformed_result = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--strict",
                    "--json",
                    "--stderr-log",
                    str(malformed_stdout_stderr_log),
                    "--process-exit-code",
                    "0",
                    str(malformed),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            missing_strict_stderr = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--strict",
                    "--json",
                    "--process-exit-code",
                    "0",
                    str(clean),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            wrong_strict_pair = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--strict",
                    "--json",
                    "--stderr-log",
                    str(wrong_strict_stderr),
                    "--process-exit-code",
                    "0",
                    str(clean),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            loose_result = subprocess.run(
                [sys.executable, str(script), "--json", str(loose_log)],
                check=False,
                capture_output=True,
                text=True,
            )
            malformed_promotion = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(malformed_stdout_stderr_log),
                    "--stderr-log",
                    str(la_stderr_log),
                    "--process-exit-code",
                    "0",
                    "--process-exit-code",
                    "0",
                    str(malformed),
                    str(la_promotion_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            malformed_stderr_promotion = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(malformed_stderr_log),
                    "--stderr-log",
                    str(la_stderr_log),
                    "--process-exit-code",
                    "0",
                    "--process-exit-code",
                    "0",
                    str(malformed_stderr_stdout_log),
                    str(la_promotion_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            clean_stderr_log.write_text("kernel panic on stderr\n")
            paired_loose = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--json",
                    "--stderr-log",
                    str(clean_stderr_log),
                    "--process-exit-code",
                    "0",
                    str(clean),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

        self.assertEqual(clean_result.returncode, 0, clean_result.stderr)
        clean_data = json.loads(clean_result.stdout)
        self.assertEqual(clean_data["strict_validation"]["status"], "PASS", clean_data)
        self.assertNotIn("decode_error", clean_data)
        self.assertEqual(clean_data["validation_mode"], "STRICT_LTP")
        self.assertEqual(clean_data["input_provenance"]["pair_id"], "rv:rv-clean")
        self.assertEqual(
            clean_data["input_provenance"]["stdout_sha256"],
            hashlib.sha256(encoded).hexdigest(),
        )
        self.assertEqual(missing_strict_stderr.returncode, 2)
        self.assertEqual(missing_strict_stderr.stdout, "")
        self.assertIn("--strict requires exactly one", missing_strict_stderr.stderr)
        self.assertEqual(wrong_strict_pair.returncode, 2, wrong_strict_pair.stderr)
        self.assertEqual(wrong_strict_pair.stdout, "")
        self.assertEqual(loose_result.returncode, 0, loose_result.stderr)
        loose_data = json.loads(loose_result.stdout)
        self.assertEqual(loose_data["validation_mode"], "FORENSIC_UNVALIDATED")
        self.assertEqual(loose_data["zero_exit_record_count"], 1)
        self.assertNotIn("pass_count", loose_data)
        self.assertNotIn("pass_clean", loose_data["categories"])
        self.assertEqual(
            loose_data["case_matrix_rows"][0]["status"], "ZERO_EXIT_RECORD"
        )
        self.assertNotEqual(paired_loose.returncode, 0, paired_loose.stdout)
        paired_loose_data = json.loads(paired_loose.stdout)
        self.assertEqual(paired_loose_data["validation_mode"], "STRICT_LTP")
        self.assertNotEqual(
            paired_loose_data["strict_validation"]["status"], "PASS"
        )

        self.assertEqual(malformed_result.returncode, 2, malformed_result.stderr)
        malformed_data = json.loads(malformed_result.stdout)
        self.assertEqual(malformed_data["decode_error"]["kind"], "invalid-utf8")
        self.assertEqual(malformed_data["strict_validation"]["status"], "ERROR")
        self.assertNotEqual(malformed_data["strict_validation"]["status"], "PASS")
        self.assertIn(
            "invalid-utf8", ltp_summary.render_markdown(malformed, malformed_data)
        )
        self.assertEqual(malformed_promotion.returncode, 2, malformed_promotion.stderr)
        malformed_promotion_data = json.loads(malformed_promotion.stdout)
        self.assertEqual(malformed_promotion_data["candidate_count"], 0)
        self.assertEqual(
            malformed_promotion_data["input_validations"][0]["status"], "ERROR"
        )
        self.assertEqual(
            malformed_promotion_data["input_validations"][0]["errors"][0]["kind"],
            "invalid-utf8",
        )
        self.assertIn(
            "invalid-utf8",
            ltp_summary.render_promotion_markdown(
                malformed_promotion_data,
                [
                    (malformed, malformed_stdout_stderr_log),
                    (la_promotion_log, la_stderr_log),
                ],
            ),
        )
        self.assertEqual(
            malformed_stderr_promotion.returncode,
            2,
            malformed_stderr_promotion.stderr,
        )
        malformed_stderr_data = json.loads(malformed_stderr_promotion.stdout)
        self.assertEqual(malformed_stderr_data["candidate_count"], 0)
        self.assertEqual(malformed_stderr_data["input_errors"][0]["stream"], "stderr")

    def test_zero_status_fail_token_is_real_pass_for_official_wire_format(self) -> None:
        data = self.compact(
            "\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE access01",
                    "FAIL LTP CASE access01 : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                ]
            )
        )

        self.assertEqual(data["zero_exit_record_count"], 1)
        self.assertEqual(data["fail_count"], 0)
        self.assertEqual(
            data["case_matrix"]["access01"]["rv"]["musl"]["status"],
            "ZERO_EXIT_RECORD",
        )

    def test_zero_status_pass_token_remains_intermediate_log_compatible(self) -> None:
        data = self.compact(
            "\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE access01",
                    "PASS LTP CASE access01 : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                ]
            )
        )

        self.assertEqual(data["zero_exit_record_count"], 1)
        self.assertEqual(data["fail_count"], 0)
        self.assertEqual(
            data["case_matrix"]["access01"]["rv"]["musl"]["status"],
            "ZERO_EXIT_RECORD",
        )

    def test_nonzero_pass_token_is_not_a_fake_pass(self) -> None:
        data = self.compact(
            "\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "RUN LTP CASE read01",
                    "PASS LTP CASE read01 : 5",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
                ]
            )
        )

        self.assertEqual(data["zero_exit_record_count"], 0)
        self.assertEqual(data["fail_count"], 1)
        row = data["case_matrix"]["read01"]["rv"]["glibc"]
        self.assertEqual(row["status"], "FAIL")
        self.assertEqual(row["code"], 5)

    def test_timeout_marker_removes_prior_pass_classification(self) -> None:
        data = self.compact(
            "\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE nanosleep01",
                    "FAIL LTP CASE nanosleep01 : 0",
                    "TIMEOUT LTP CASE nanosleep01 after 15s",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                ]
            )
        )

        self.assertEqual(data["zero_exit_record_count"], 0)
        self.assertEqual(data["timeouts"], 1)
        row = data["case_matrix"]["nanosleep01"]["rv"]["musl"]
        self.assertEqual(row["status"], "TIMEOUT")
        self.assertEqual(row["timeouts"], 1)
        self.assertEqual(data["categories"]["zero_exit_without_detected_blocker"], [])
        self.assertEqual(data["categories"]["timeout"], ["rv:musl:nanosleep01"])

    def test_case_list_manifest_is_reported(self) -> None:
        data = self.compact(
            "\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "ltp case list: stable (1000 cases, timeout 15s)",
                    "RUN LTP CASE access01",
                    "PASS LTP CASE access01 : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                ]
            )
        )

        self.assertEqual(
            data["case_list_manifests"],
            [
                {
                    "group": "ltp-musl",
                    "name": "stable",
                    "case_count": 1000,
                    "timeout_secs": 15,
                }
            ],
        )
        row = data["case_matrix"]["access01"]["rv"]["musl"]
        self.assertEqual(row["case_list"]["name"], "stable")


    def test_promotion_mode_boundary_allows_stable_file_inline_batch_core_and_blocks_sweep(self) -> None:
        allowed_modes = [
            "stable",
            "file:/tmp/ltp_cases.txt",
            "inline",
            "batch:smoke",
            "core",
        ]
        for mode in allowed_modes:
            with self.subTest(mode=mode):
                self.assertIsNone(ltp_summary.promotion_mode_blocker({"name": mode}))

        blocked_modes = [
            "all",
            "sweep:all",
            "all-minus-blacklist skipped=3",
            "stable-plus-all-minus-blacklist stable=1000 extra=2 deduped=0 skipped=1",
        ]
        for mode in blocked_modes:
            with self.subTest(mode=mode):
                self.assertEqual(
                    ltp_summary.promotion_mode_blocker({"name": mode}),
                    f"selection-mode={mode}",
                )

    def test_promotion_candidate_requires_four_way_clean_matrix(self) -> None:
        report = self.promotion_report(
            rv_log=self.two_libc_pass_log("openat02"),
            la_log=self.two_libc_pass_log("openat02"),
        )

        self.assertEqual(report["candidate_count"], 1)
        self.assertEqual(report["blocked_count"], 0)
        self.assertEqual(report["candidates"][0]["case"], "openat02")
        self.assertEqual(len(report["candidates"][0]["combos"]), 4)
        for combo in report["candidates"][0]["combos"]:
            self.assertIn(combo["source_path"], {"rv.log", "la.log"})
            self.assertIn(combo["stderr_path"], {"rv.stderr.log", "la.stderr.log"})
            self.assertEqual(combo["strict_case_binding"]["case"], "openat02")
            self.assertEqual(
                combo["strict_case_binding"]["events"],
                ["START", "RUN", "RESULT", "PASS", "END"],
            )
        for arches, libcs, message in (
            (set(), {"musl"}, "at least one known architecture"),
            ({"rv"}, set(), "at least one known libc"),
            ({"mips"}, {"musl"}, "unknown promotion arches"),
            ({"rv"}, {"uclibc"}, "unknown promotion libcs"),
            ({"rv"}, {"musl", "glibc"}, "exactly rv,la"),
            ({"rv", "la"}, {"musl"}, "exactly musl,glibc"),
        ):
            with self.subTest(arches=arches, libcs=libcs):
                with self.assertRaisesRegex(ValueError, message):
                    ltp_summary.promotion_report([], arches, libcs, [])
        with self.assertRaisesRegex(ValueError, "requires input lifecycle validations"):
            ltp_summary.promotion_report(
                [],
                {"rv", "la"},
                {"musl", "glibc"},
                [],
            )

    def test_promotion_candidate_requires_complete_lifecycle(self) -> None:
        clean = self.two_libc_pass_log("openat02")
        mutations = {
            "manifest": "ltp case list: inline (1 cases, timeout 30s)\n",
            "case-start": "========== START ltp openat02 ==========\n",
            "run": "RUN LTP CASE openat02\n",
            "result": "FAIL LTP CASE openat02 : 0\n",
            "pass": "Pass!\n",
            "case-end": "========== END ltp openat02 ==========\n",
            "summary": "ltp cases: 1 passed, 0 failed, 0 timed out\n",
        }
        for label, removed in mutations.items():
            with self.subTest(label=label):
                malformed = clean.replace(removed, "", 1)
                report = self.promotion_report(malformed, clean)
                self.assertEqual(report["candidate_count"], 0, report)
                self.assertEqual(report["blocked_count"], 1, report)
                reasons = [
                    reason
                    for blocker in report["blocked"][0]["blockers"]
                    for reason in blocker["reasons"]
                ]
                self.assertTrue(
                    any(reason.startswith("strict-") for reason in reasons),
                    reasons,
                )

        for label, malformed in (
            ("planned", clean.replace("inline (1 cases", "inline (2 cases", 1)),
            (
                "summary-mismatch",
                clean.replace(
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
                    "ltp cases: 0 passed, 1 failed, 0 timed out",
                    1,
                ),
            ),
        ):
            with self.subTest(label=label):
                report = self.promotion_report(malformed, clean)
                self.assertEqual(report["candidate_count"], 0, report)
                self.assertEqual(report["blocked_count"], 1, report)

        for label, prefix in (
            ("ltp-record", "RUN LTP CASE\n"),
            ("group-frame", "#### OS COMP TEST GROUP START ltp-bogus ###\n"),
        ):
            with self.subTest(label=label):
                outside_report = self.promotion_report(prefix + clean, clean)
                self.assertEqual(outside_report["candidate_count"], 0, outside_report)
                outside_reasons = [
                    reason
                    for blocker in outside_report["blocked"][0]["blockers"]
                    for reason in blocker["reasons"]
                ]
                self.assertIn("strict-malformed-protocol-record", outside_reasons)

        for label, malformed_protocol in (
            ("official-bad-code", "PASS OFFICIAL TEST GROUP ltp-musl : nope"),
            ("official-missing-code", "PASS OFFICIAL TEST GROUP ltp-musl"),
            ("busybox-extra", "testcase busybox fake success extra"),
            ("pass-extra", "Pass! extra"),
        ):
            with self.subTest(outside_malformed_protocol=label):
                outside_report = self.promotion_report(
                    clean + "\n" + malformed_protocol,
                    clean,
                )
                self.assertEqual(outside_report["candidate_count"], 0, outside_report)
                reasons = [
                    reason
                    for blocker in outside_report["blocked"][0]["blockers"]
                    for reason in blocker["reasons"]
                ]
                self.assertIn("strict-malformed-protocol-record", reasons)

        duplicate_non_ltp_group = "\n".join(
            [
                clean,
                "#### OS COMP TEST GROUP START custom-smoke ####",
                "#### OS COMP TEST GROUP END custom-smoke ####",
                "#### OS COMP TEST GROUP START custom-smoke ####",
                "#### OS COMP TEST GROUP END custom-smoke ####",
            ]
        )
        duplicate_group_report = self.promotion_report(duplicate_non_ltp_group, clean)
        self.assertEqual(duplicate_group_report["candidate_count"], 0)
        duplicate_reasons = [
            reason
            for blocker in duplicate_group_report["blocked"][0]["blockers"]
            for reason in blocker["reasons"]
        ]
        self.assertIn("strict-duplicate-group", duplicate_reasons)

        report_level_failure = self.promotion_report(
            clean,
            clean,
            extra_validations=[
                {
                    "path": "extra.log",
                    "stderr_path": "extra.stderr.log",
                    "arch": "rv",
                    "status": "FAIL",
                    "group_count": 1,
                    "error_count": 0,
                    "failure_count": 1,
                }
            ],
        )
        self.assertEqual(report_level_failure["candidate_count"], 0)
        self.assertEqual(report_level_failure["blocked_count"], 1)
        self.assertEqual(report_level_failure["input_blockers"][0]["status"], "FAIL")

        scouting_groups = clean.replace("ltp-musl", "ltp-musl-scout").replace(
            "ltp-glibc",
            "ltp-glibc-scout",
        )
        scouting_report = self.promotion_report(scouting_groups, clean)
        self.assertEqual(scouting_report["candidate_count"], 0, scouting_report)
        scouting_reasons = [
            reason
            for blocker in scouting_report["blocked"][0]["blockers"]
            for reason in blocker["reasons"]
        ]
        self.assertTrue(
            any(reason.startswith("noncanonical-ltp-group=") for reason in scouting_reasons),
            scouting_reasons,
        )

        for label, suffix in (
            ("tfail", "access01 1 TFAIL: late failure"),
            ("tbrok", "access01 1 TBROK: late breakage"),
            ("tconf", "access01 1 TCONF: late skip"),
            ("enosys", "access01: ENOSYS"),
            ("timeout", "command timed out after the LTP groups"),
            ("panic", "kernel panic after the LTP groups"),
            ("unknown", "STATUS: UNKNOWN"),
            ("zero-execution", "0 tests ran"),
        ):
            with self.subTest(outside_quality_signal=label):
                outside_report = self.promotion_report(clean + "\n" + suffix, clean)
                self.assertEqual(outside_report["candidate_count"], 0, outside_report)
                self.assertEqual(outside_report["blocked_count"], 1, outside_report)
                reasons = [
                    reason
                    for blocker in outside_report["blocked"][0]["blockers"]
                    for reason in blocker["reasons"]
                ]
                self.assertTrue(
                    any("outside-ltp-group" in reason for reason in reasons),
                    reasons,
                )

        for label, protocol_record in (
            ("official-pass", "PASS OFFICIAL TEST GROUP ltp-musl : 0"),
            ("official-fail", "FAIL OFFICIAL TEST GROUP ltp-musl : 1"),
        ):
            with self.subTest(outside_protocol=label):
                outside_report = self.promotion_report(
                    clean + "\n" + protocol_record,
                    clean,
                )
                self.assertEqual(outside_report["candidate_count"], 0, outside_report)
                reasons = [
                    reason
                    for blocker in outside_report["blocked"][0]["blockers"]
                    for reason in blocker["reasons"]
                ]
                self.assertIn(
                    "strict-protocol-record-outside-ltp-group",
                    reasons,
                )

        non_ltp_failure = "\n".join(
            [
                clean,
                "#### OS COMP TEST GROUP START custom-smoke ####",
                "TFAIL belongs to a non-LTP group",
                "#### OS COMP TEST GROUP END custom-smoke ####",
            ]
        )
        scoped_report = self.promotion_report(non_ltp_failure, clean)
        self.assertEqual(scoped_report["candidate_count"], 1, scoped_report)

        owned_non_ltp_failure = "\n".join(
            [
                clean,
                "#### OS COMP TEST GROUP START custom-smoke ####",
                "FAIL OFFICIAL TEST GROUP custom-smoke : 1",
                "#### OS COMP TEST GROUP END custom-smoke ####",
            ]
        )
        owned_report = self.promotion_report(owned_non_ltp_failure, clean)
        self.assertEqual(owned_report["candidate_count"], 1, owned_report)

        mislabeled_non_ltp_failure = owned_non_ltp_failure.replace(
            "FAIL OFFICIAL TEST GROUP custom-smoke : 1",
            "FAIL OFFICIAL TEST GROUP ltp-musl : 1",
        )
        mislabeled_report = self.promotion_report(mislabeled_non_ltp_failure, clean)
        self.assertEqual(mislabeled_report["candidate_count"], 0, mislabeled_report)
        mislabeled_reasons = [
            reason
            for blocker in mislabeled_report["blocked"][0]["blockers"]
            for reason in blocker["reasons"]
        ]
        self.assertIn("strict-protocol-group-label-mismatch", mislabeled_reasons)

    def test_promotion_candidate_blocks_missing_arch_libc_combo(self) -> None:
        report = self.promotion_report(
            rv_log=self.two_libc_pass_log("rename01"),
            la_log=self.complete_group("ltp-musl", "rename01"),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        self.assertEqual(report["blocked"][0]["case"], "rename01")
        self.assertEqual(report["blocked"][0]["missing"], [{"arch": "la", "libc": "glibc"}])

    def test_promotion_candidate_blocks_pass_with_internal_tconf(self) -> None:
        report = self.promotion_report(
            rv_log=self.two_libc_pass_log("readlinkat02"),
            la_log="\n".join(
                [
                    self.complete_group("ltp-musl", "readlinkat02"),
                    self.complete_group(
                        "ltp-glibc",
                        "readlinkat02",
                        marker="readlinkat02 1 TCONF : setup skipped part of the test",
                    ),
                ]
            ),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        blocker = report["blocked"][0]["blockers"][0]
        self.assertEqual(blocker["arch"], "la")
        self.assertIn("strict-validation=FAIL", blocker["reasons"])
        self.assertTrue(
            any(
                candidate["libc"] == "glibc" and "TCONF=1" in candidate["reasons"]
                for candidate in report["blocked"][0]["blockers"]
            )
        )

    def test_promotion_candidate_blocks_blacklist_selection_mode(self) -> None:
        def blacklist_log(case: str) -> str:
            return "\n".join(
                [
                    self.complete_group(
                        "ltp-musl",
                        case,
                        mode="stable-plus-all-minus-blacklist stable=1 extra=0 deduped=1 skipped=1",
                    ),
                    self.complete_group(
                        "ltp-glibc",
                        case,
                        mode="stable-plus-all-minus-blacklist stable=1 extra=0 deduped=1 skipped=1",
                    ),
                ]
            )

        report = self.promotion_report(
            rv_log=blacklist_log("chmod06"),
            la_log=blacklist_log("chmod06"),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        blockers = report["blocked"][0]["blockers"]
        self.assertEqual(len(blockers), 4)
        self.assertTrue(
            all(
                any(reason.startswith("selection-mode=stable-plus-all-minus-blacklist") for reason in blocker["reasons"])
                for blocker in blockers
            )
        )

    def test_promotion_candidate_blocks_prior_failure_event_even_if_later_passes(self) -> None:
        malicious_group = self.complete_group("ltp-musl", "fchmod02").replace(
            "FAIL LTP CASE fchmod02 : 0",
            "FAIL LTP CASE fchmod02 : 5\nFAIL LTP CASE fchmod02 : 0",
        )
        report = self.promotion_report(
            rv_log="\n".join(
                [
                    malicious_group,
                    self.complete_group("ltp-glibc", "fchmod02"),
                ]
            ),
            la_log=self.two_libc_pass_log("fchmod02"),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        reasons = [
            reason
            for blocker in report["blocked"][0]["blockers"]
            for reason in blocker["reasons"]
        ]
        self.assertIn("event-failures=1", reasons)
        self.assertIn("strict-validation=ERROR", reasons)
        self.assertTrue(any(reason.startswith("strict-ltp-") for reason in reasons))

    def test_cli_requires_promotion_mode_for_multiple_logs(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            rv_log = Path(tmp) / "rv.log"
            la_log = Path(tmp) / "la.log"
            rv_stderr = Path(tmp) / "rv.stderr.log"
            la_stderr = Path(tmp) / "la.stderr.log"
            rv_text = self.arch_banner("rv") + self.two_libc_pass_log("chmod06")
            la_text = self.arch_banner("la") + self.two_libc_pass_log("chmod06").replace(
                "LTP CASE RUNTIME chmod06: 1 ms",
                "LTP CASE RUNTIME chmod06: 2 ms",
            )
            rv_log.write_text(rv_text)
            la_log.write_text(la_text)
            rv_stderr.write_text("")
            la_stderr.write_text("")

            result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

        self.assertNotEqual(result.returncode, 0)
        self.assertIn("multiple logs require --promotion-candidates", result.stderr)

    def test_cli_promotion_candidates_json_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            rv_log = Path(tmp) / "rv.log"
            la_log = Path(tmp) / "la.log"
            rv_stderr = Path(tmp) / "rv.stderr.log"
            la_stderr = Path(tmp) / "la.stderr.log"
            rv_text = self.arch_banner("rv") + self.two_libc_pass_log("chmod06")
            la_text = self.arch_banner("la") + self.two_libc_pass_log("chmod06").replace(
                "LTP CASE RUNTIME chmod06: 1 ms",
                "LTP CASE RUNTIME chmod06: 2 ms",
            )
            process_args = ["--process-exit-code", "0", "--process-exit-code", "0"]
            three_process_args = [*process_args, "--process-exit-code", "0"]
            rv_log.write_text(rv_text)
            la_log.write_text(la_text)
            rv_stderr.write_text("")
            la_stderr.write_text("")

            wrong_stem_stderr = Path(tmp) / "rv-wrong.stderr.log"
            wrong_stem_stderr.write_text("")
            other_directory = Path(tmp) / "other"
            other_directory.mkdir()
            other_rv_stderr = other_directory / "rv.stderr.log"
            other_rv_stderr.write_text("")
            ambiguous_log = Path(tmp) / "rv-la.log"
            ambiguous_stderr = Path(tmp) / "rv-la.stderr.log"
            ambiguous_log.write_text(rv_text)
            ambiguous_stderr.write_text("")
            case_mismatch_log = Path(tmp) / "RV.log"
            case_mismatch_log.write_text(rv_text)
            la_alias_log = Path(tmp) / "la-alias.log"
            la_alias_stderr = Path(tmp) / "la-alias.stderr.log"
            la_alias_log.write_text(la_text)
            os.link(rv_stderr, la_alias_stderr)
            la_copy_log = Path(tmp) / "la-copy.log"
            la_copy_stderr = Path(tmp) / "la-copy.stderr.log"
            la_copy_log.write_text(rv_text)
            la_copy_stderr.write_text("")

            result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(rv_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    *process_args,
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            missing_stderr = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    *process_args,
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            swapped_stderr = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(la_stderr),
                    "--stderr-log",
                    str(rv_stderr),
                    *process_args,
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            wrong_stem = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(wrong_stem_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    *process_args,
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            different_directory = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(other_rv_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    *process_args,
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            ambiguous_arch = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(ambiguous_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    *process_args,
                    str(ambiguous_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            case_mismatch = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(rv_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    *process_args,
                    str(case_mismatch_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            hardlink_alias = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(rv_stderr),
                    "--stderr-log",
                    str(la_alias_stderr),
                    *process_args,
                    str(rv_log),
                    str(la_alias_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            copied_cross_arch = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(rv_stderr),
                    "--stderr-log",
                    str(la_copy_stderr),
                    *process_args,
                    str(rv_log),
                    str(la_copy_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            nonzero_process = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(rv_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    "--process-exit-code",
                    "7",
                    "--process-exit-code",
                    "0",
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            rv_stderr.write_text("kernel panic on stderr\n")
            contaminated_stderr = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(rv_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    *process_args,
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            mystery_log = Path(tmp) / "mystery.log"
            mystery_stderr = Path(tmp) / "mystery.stderr.log"
            mystery_log.write_text(self.two_libc_pass_log("chmod06") + "\nTFAIL outside\n")
            mystery_stderr.write_text("")
            extra_unknown_input = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "summarize_ltp_results.py"),
                    "--promotion-candidates",
                    "--json",
                    "--stderr-log",
                    str(rv_stderr),
                    "--stderr-log",
                    str(la_stderr),
                    "--stderr-log",
                    str(mystery_stderr),
                    *three_process_args,
                    str(rv_log),
                    str(la_log),
                    str(mystery_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

            invalid_dimensions = []
            for option, value in (
                ("--promotion-arches", ""),
                ("--promotion-arches", ",,,"),
                ("--promotion-libcs", ""),
                ("--promotion-libcs", ",,,"),
                ("--promotion-arches", "mips"),
                ("--promotion-libcs", "uclibc"),
                ("--promotion-arches", "rv"),
                ("--promotion-libcs", "musl"),
            ):
                invalid_dimensions.append(
                    (
                        option,
                        value,
                        subprocess.run(
                            [
                                sys.executable,
                                str(
                                    Path(__file__).parents[1]
                                    / "evaluation"
                                    / "summarize_ltp_results.py"
                                ),
                                "--promotion-candidates",
                                "--json",
                                option,
                                value,
                                "--stderr-log",
                                str(rv_stderr),
                                "--stderr-log",
                                str(la_stderr),
                                *process_args,
                                str(rv_log),
                                str(la_log),
                            ],
                            check=False,
                            capture_output=True,
                            text=True,
                        ),
                    )
                )

        self.assertEqual(result.returncode, 0, result.stderr)
        data = json.loads(result.stdout)
        self.assertEqual(data["candidate_count"], 1)
        self.assertEqual(data["blocked_count"], 0)
        self.assertEqual(data["candidates"][0]["case"], "chmod06")
        self.assertEqual(data["validation_scope"], "ltp")
        self.assertEqual(
            [validation["status"] for validation in data["input_validations"]],
            ["PASS", "PASS"],
        )
        expected_stdout_hashes = {
            "rv:rv": hashlib.sha256(rv_text.encode("utf-8")).hexdigest(),
            "la:la": hashlib.sha256(la_text.encode("utf-8")).hexdigest(),
        }
        expected_empty_hash = hashlib.sha256(b"").hexdigest()
        self.assertEqual(len(data["input_pairs"]), 2)
        self.assertEqual(
            {pair["pair_id"] for pair in data["input_pairs"]},
            {"rv:rv", "la:la"},
        )
        for pair in data["input_pairs"]:
            self.assertEqual(
                pair["stdout_sha256"], expected_stdout_hashes[pair["pair_id"]]
            )
            self.assertEqual(pair["stderr_sha256"], expected_empty_hash)
            self.assertGreater(pair["stdout_size_bytes"], 0)
            self.assertEqual(pair["stderr_size_bytes"], 0)
        self.assertTrue(
            all(
                combo["strict_case_binding"]["events"]
                == ["START", "RUN", "RESULT", "PASS", "END"]
                for combo in data["candidates"][0]["combos"]
            )
        )
        pair_hashes = {
            pair["pair_id"]: (pair["stdout_sha256"], pair["stderr_sha256"])
            for pair in data["input_pairs"]
        }
        for combo in data["candidates"][0]["combos"]:
            self.assertEqual(
                (combo["stdout_sha256"], combo["stderr_sha256"]),
                pair_hashes[combo["pair_id"]],
            )
        self.assertEqual(missing_stderr.returncode, 2, missing_stderr.stderr)
        self.assertIn("requires exactly one --stderr-log", missing_stderr.stderr)
        self.assertEqual(missing_stderr.stdout, "")
        self.assertEqual(extra_unknown_input.returncode, 2, extra_unknown_input.stderr)
        self.assertEqual(extra_unknown_input.stdout, "")
        self.assertIn("exactly one rv or la architecture", extra_unknown_input.stderr)
        for invalid_pair in (
            swapped_stderr,
            wrong_stem,
            different_directory,
            ambiguous_arch,
            case_mismatch,
            hardlink_alias,
        ):
            self.assertEqual(
                invalid_pair.returncode,
                2,
                invalid_pair.stdout + invalid_pair.stderr,
            )
            self.assertEqual(invalid_pair.stdout, "")
        self.assertEqual(
            contaminated_stderr.returncode,
            1,
            contaminated_stderr.stdout + contaminated_stderr.stderr,
        )
        contaminated_data = json.loads(contaminated_stderr.stdout)
        self.assertEqual(contaminated_data["candidate_count"], 0, contaminated_data)
        self.assertEqual(contaminated_data["input_validations"][0]["status"], "FAIL")
        self.assertTrue(contaminated_data["input_validations"][0]["failures"])
        self.assertIn(
            contaminated_data["input_validations"][0]["failures"][0]["kind"],
            ltp_summary.render_promotion_markdown(
                contaminated_data, [(rv_log, rv_stderr), (la_log, la_stderr)]
            ),
        )
        self.assertEqual(copied_cross_arch.returncode, 2, copied_cross_arch.stderr)
        copied_data = json.loads(copied_cross_arch.stdout)
        self.assertEqual(copied_data["candidate_count"], 0)
        self.assertTrue(
            {
                "capture-architecture-provenance",
                "cross-arch-identical-stdout-digest",
            }.issubset({finding["kind"] for finding in copied_data["input_errors"]})
        )
        self.assertEqual(nonzero_process.returncode, 1, nonzero_process.stderr)
        nonzero_data = json.loads(nonzero_process.stdout)
        self.assertEqual(nonzero_data["candidate_count"], 0)
        self.assertEqual(nonzero_data["input_pairs"][0]["process_exit_code"], 7)
        self.assertEqual(nonzero_data["input_validations"][0]["status"], "FAIL")
        for option, value, invalid in invalid_dimensions:
            with self.subTest(option=option, value=value):
                self.assertEqual(invalid.returncode, 2, invalid.stdout + invalid.stderr)
                self.assertEqual(invalid.stdout, "")
                self.assertNotIn('"candidates"', invalid.stderr)


if __name__ == "__main__":
    unittest.main()
