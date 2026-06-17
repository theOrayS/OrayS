#!/usr/bin/env python3
"""Regression tests for LTP evaluator summary semantics."""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ltp_summary


class LtpSummarySemanticsTest(unittest.TestCase):
    def compact(self, log: str) -> dict:
        return ltp_summary.compact(ltp_summary.parse_log(log), arch="rv")

    def promotion_report(self, rv_log: str, la_log: str) -> dict:
        rows = []
        for arch, log in (("rv", rv_log), ("la", la_log)):
            raw_summary = ltp_summary.parse_log(log)
            data = ltp_summary.compact(raw_summary, arch=arch)
            rows.extend(ltp_summary.promotion_rows(raw_summary, data, arch))
        return ltp_summary.promotion_report(
            rows,
            required_arches={"rv", "la"},
            required_libcs={"musl", "glibc"},
        )

    def two_libc_pass_log(self, case: str) -> str:
        return "\n".join(
            [
                "#### OS COMP TEST GROUP START ltp-musl ####",
                "ltp case list: inline (1 cases, timeout 30s)",
                f"RUN LTP CASE {case}",
                f"FAIL LTP CASE {case} : 0",
                "#### OS COMP TEST GROUP END ltp-musl ####",
                "#### OS COMP TEST GROUP START ltp-glibc ####",
                "ltp case list: inline (1 cases, timeout 30s)",
                f"RUN LTP CASE {case}",
                f"FAIL LTP CASE {case} : 0",
                "#### OS COMP TEST GROUP END ltp-glibc ####",
            ]
        )

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

        self.assertEqual(data["pass_count"], 1)
        self.assertEqual(data["fail_count"], 0)
        self.assertEqual(data["case_matrix"]["access01"]["rv"]["musl"]["status"], "PASS")

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

        self.assertEqual(data["pass_count"], 1)
        self.assertEqual(data["fail_count"], 0)
        self.assertEqual(data["case_matrix"]["access01"]["rv"]["musl"]["status"], "PASS")

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

        self.assertEqual(data["pass_count"], 0)
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

        self.assertEqual(data["pass_count"], 0)
        self.assertEqual(data["timeouts"], 1)
        row = data["case_matrix"]["nanosleep01"]["rv"]["musl"]
        self.assertEqual(row["status"], "TIMEOUT")
        self.assertEqual(row["timeouts"], 1)
        self.assertEqual(data["categories"]["pass_clean"], [])
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

    def test_promotion_candidate_requires_four_way_clean_matrix(self) -> None:
        report = self.promotion_report(
            rv_log=self.two_libc_pass_log("openat02"),
            la_log=self.two_libc_pass_log("openat02"),
        )

        self.assertEqual(report["candidate_count"], 1)
        self.assertEqual(report["blocked_count"], 0)
        self.assertEqual(report["candidates"][0]["case"], "openat02")
        self.assertEqual(len(report["candidates"][0]["combos"]), 4)

    def test_promotion_candidate_blocks_missing_arch_libc_combo(self) -> None:
        report = self.promotion_report(
            rv_log=self.two_libc_pass_log("rename01"),
            la_log="\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE rename01",
                    "PASS LTP CASE rename01 : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                ]
            ),
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
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE readlinkat02",
                    "PASS LTP CASE readlinkat02 : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "RUN LTP CASE readlinkat02",
                    "readlinkat02 1 TCONF : setup skipped part of the test",
                    "PASS LTP CASE readlinkat02 : 0",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
                ]
            ),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        blocker = report["blocked"][0]["blockers"][0]
        self.assertEqual(blocker["arch"], "la")
        self.assertEqual(blocker["libc"], "glibc")
        self.assertEqual(blocker["reasons"], ["TCONF=1"])

    def test_promotion_candidate_blocks_blacklist_selection_mode(self) -> None:
        def blacklist_log(case: str) -> str:
            return "\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "ltp case list: stable-plus-all-minus-blacklist stable=1000 extra=2 deduped=1000 skipped=3 (1002 cases, timeout 30s)",
                    f"RUN LTP CASE {case}",
                    f"PASS LTP CASE {case} : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "ltp case list: stable-plus-all-minus-blacklist stable=1000 extra=2 deduped=1000 skipped=3 (1002 cases, timeout 30s)",
                    f"RUN LTP CASE {case}",
                    f"PASS LTP CASE {case} : 0",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
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
        report = self.promotion_report(
            rv_log="\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE fchmod02",
                    "FAIL LTP CASE fchmod02 : 5",
                    "PASS LTP CASE fchmod02 : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "RUN LTP CASE fchmod02",
                    "PASS LTP CASE fchmod02 : 0",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
                ]
            ),
            la_log=self.two_libc_pass_log("fchmod02"),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        reasons = report["blocked"][0]["blockers"][0]["reasons"]
        self.assertEqual(reasons, ["event-failures=1"])

    def test_cli_requires_promotion_mode_for_multiple_logs(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            rv_log = Path(tmp) / "rv.log"
            la_log = Path(tmp) / "la.log"
            rv_log.write_text(self.two_libc_pass_log("chmod06"))
            la_log.write_text(self.two_libc_pass_log("chmod06"))

            result = subprocess.run(
                [sys.executable, str(Path(__file__).with_name("ltp_summary.py")), str(rv_log), str(la_log)],
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
            rv_log.write_text(self.two_libc_pass_log("chmod06"))
            la_log.write_text(self.two_libc_pass_log("chmod06"))

            result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).with_name("ltp_summary.py")),
                    "--promotion-candidates",
                    "--json",
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )

        self.assertEqual(result.returncode, 0, result.stderr)
        data = json.loads(result.stdout)
        self.assertEqual(data["candidate_count"], 1)
        self.assertEqual(data["blocked_count"], 0)
        self.assertEqual(data["candidates"][0]["case"], "chmod06")


if __name__ == "__main__":
    unittest.main()
