#!/usr/bin/env python3
"""Unit tests for honest official evaluation failure reporting."""

from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evaluation"))

import report_evaluation_failures as reporter


class EvaluationFailureReportTest(unittest.TestCase):
    def parse(self, text: str) -> reporter.LogReport:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "official.log"
            path.write_text(text, encoding="utf-8")
            return reporter.parse_log(path)

    def test_started_case_without_result_is_failed(self) -> None:
        report = self.parse(
            "#### OS COMP TEST GROUP START ltp-musl ####\n"
            "RUN LTP CASE access01\n"
        )
        failed = reporter.ltp_failed_cases(report)
        self.assertEqual([(case.group, case.name) for case in failed], [("ltp-musl", "access01")])
        self.assertIsNone(failed[0].code)

    def test_zero_numeric_result_is_clean_wrapper_success(self) -> None:
        report = self.parse("RUN LTP CASE access01\nFAIL LTP CASE access01 : 0\n")
        self.assertEqual(reporter.ltp_failed_cases(report), [])

    def test_nonzero_numeric_result_is_failed(self) -> None:
        report = self.parse("RUN LTP CASE access01\nFAIL LTP CASE access01 : 5\n")
        self.assertEqual(len(reporter.ltp_failed_cases(report)), 1)

    def test_timeout_overrides_zero_numeric_result(self) -> None:
        report = self.parse(
            "RUN LTP CASE access01\n"
            "FAIL LTP CASE access01 : 0\n"
            "TIMEOUT LTP CASE access01 after 3s\n"
        )
        self.assertEqual(len(reporter.ltp_failed_cases(report)), 1)

    def test_tconf_only_is_separate_and_never_described_as_pass(self) -> None:
        report = self.parse(
            "RUN LTP CASE access01\n"
            "access01 1 TCONF: unsupported\n"
            "FAIL LTP CASE access01 : 0\n"
        )
        self.assertEqual(len(reporter.ltp_tconf_cases(report)), 1)
        markdown = reporter.render_markdown([report])
        self.assertIn("TCONF-only（配置性未通过/跳过，未计为 PASS）", markdown)

    def test_render_includes_incomplete_case(self) -> None:
        report = self.parse("RUN LTP CASE access01\n")
        markdown = reporter.render_markdown([report])
        self.assertIn("失败/不完整", markdown)
        self.assertIn("access01", markdown)


if __name__ == "__main__":
    unittest.main()
