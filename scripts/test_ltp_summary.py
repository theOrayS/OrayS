#!/usr/bin/env python3
"""Regression tests for LTP evaluator summary semantics."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ltp_summary


class LtpSummarySemanticsTest(unittest.TestCase):
    def compact(self, log: str) -> dict:
        return ltp_summary.compact(ltp_summary.parse_log(log), arch="rv")

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


if __name__ == "__main__":
    unittest.main()
