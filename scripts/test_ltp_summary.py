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
                f"RUN LTP CASE {case}",
                f"PASS LTP CASE {case} : 0",
                "#### OS COMP TEST GROUP END ltp-musl ####",
                "#### OS COMP TEST GROUP START ltp-glibc ####",
                f"RUN LTP CASE {case}",
                f"PASS LTP CASE {case} : 0",
                "#### OS COMP TEST GROUP END ltp-glibc ####",
            ]
        )

    def test_zero_status_pass_token_is_real_pass_for_current_wire_format(self) -> None:
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

    def test_zero_status_fail_token_remains_legacy_pass_compatible(self) -> None:
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

    def test_promotion_candidate_blocks_timeout_even_after_wrapper_pass(self) -> None:
        report = self.promotion_report(
            rv_log="\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE kill02",
                    "PASS LTP CASE kill02 : 0",
                    "TIMEOUT LTP CASE kill02 after 60s",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "RUN LTP CASE kill02",
                    "PASS LTP CASE kill02 : 0",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
                ]
            ),
            la_log=self.two_libc_pass_log("kill02"),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        blocker = report["blocked"][0]["blockers"][0]
        self.assertEqual(blocker["arch"], "rv")
        self.assertEqual(blocker["libc"], "musl")
        self.assertEqual(blocker["reasons"], ["timeout=1", "status=TIMEOUT"])

    def test_promotion_candidate_blocks_enosys_and_panic_trap_markers(self) -> None:
        report = self.promotion_report(
            rv_log="\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "RUN LTP CASE getcpu01",
                    "getcpu01: not implemented",
                    "PASS LTP CASE getcpu01 : 0",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "RUN LTP CASE getcpu01",
                    "kernel trap while handling getcpu01",
                    "PASS LTP CASE getcpu01 : 0",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
                ]
            ),
            la_log=self.two_libc_pass_log("getcpu01"),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        blockers = {
            (blocker["arch"], blocker["libc"]): blocker["reasons"]
            for blocker in report["blocked"][0]["blockers"]
        }
        self.assertEqual(blockers[("rv", "musl")], ["ENOSYS=1"])
        self.assertEqual(blockers[("rv", "glibc")], ["panic/trap=1"])

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
