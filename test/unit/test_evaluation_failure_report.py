#!/usr/bin/env python3
"""Unit tests for honest official evaluation failure reporting."""

from __future__ import annotations

import hashlib
import os
import sys
import subprocess
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evaluation"))

import report_evaluation_failures as reporter


class EvaluationFailureReportTest(unittest.TestCase):
    def parse(self, text: str) -> reporter.LogReport:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "official.log"
            stderr_path = Path(temporary) / "official.stderr.log"
            path.write_text(text, encoding="utf-8")
            stderr_path.write_text("", encoding="utf-8")
            return reporter.parse_log(path, stderr_path, 0)

    def complete_ltp_log(
        self,
        *,
        code: int = 0,
        marker: str = "",
        timed_out: bool = False,
    ) -> str:
        lines = [
            "#### OS COMP TEST GROUP START ltp-musl ####",
            "ltp case list: inline (1 cases, timeout 30s)",
            "========== START ltp access01 ==========",
            "RUN LTP CASE access01",
        ]
        if marker:
            lines.append(marker)
        lines.append(f"FAIL LTP CASE access01 : {code}")
        if code == 0:
            lines.append("Pass!")
        if timed_out:
            lines.append("TIMEOUT LTP CASE access01 after 3s")
        lines.extend(
            [
                "LTP CASE RUNTIME access01: 1 ms",
                "========== END ltp access01 ==========",
                (
                    f"ltp cases: {1 if code == 0 else 0} passed, "
                    f"{0 if code == 0 else 1} failed, {1 if timed_out else 0} timed out"
                ),
                "#### OS COMP TEST GROUP END ltp-musl ####",
            ]
        )
        return "\n".join(lines) + "\n"

    def isolated_cli(self, script: Path, *arguments: str) -> list[str]:
        return [
            sys.executable,
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
            str(script),
            *arguments,
        ]

    def test_started_case_without_result_is_failed(self) -> None:
        report = self.parse(
            "#### OS COMP TEST GROUP START ltp-musl ####\n"
            "RUN LTP CASE access01\n"
        )
        failed = reporter.ltp_failed_cases(report)
        self.assertEqual([(case.group, case.name) for case in failed], [("ltp-musl", "access01")])
        self.assertIsNone(failed[0].code)
        self.assertFalse(failed[0].lifecycle_complete)
        self.assertEqual(report.ltp_validation["status"], "ERROR")
        self.assertIn(
            "missing-group-end",
            {finding["kind"] for finding in report.ltp_validation["errors"]},
        )

    def test_zero_numeric_result_is_clean_wrapper_success(self) -> None:
        report = self.parse(self.complete_ltp_log())
        self.assertEqual(reporter.ltp_failed_cases(report), [])
        self.assertEqual(report.ltp_validation["status"], "PASS")
        self.assertTrue(next(iter(report.ltp_cases.values())).lifecycle_complete)

    def test_nonzero_numeric_result_is_failed(self) -> None:
        report = self.parse(self.complete_ltp_log(code=5))
        self.assertEqual(len(reporter.ltp_failed_cases(report)), 1)
        self.assertEqual(report.ltp_validation["status"], "FAIL")
        self.assertTrue(next(iter(report.ltp_cases.values())).lifecycle_complete)

    def test_timeout_overrides_zero_numeric_result(self) -> None:
        report = self.parse(self.complete_ltp_log(timed_out=True))
        self.assertEqual(len(reporter.ltp_failed_cases(report)), 1)
        self.assertTrue(next(iter(report.ltp_cases.values())).timed_out)

    def test_tconf_only_is_separate_and_never_described_as_pass(self) -> None:
        report = self.parse(self.complete_ltp_log(marker="access01 1 TCONF: unsupported"))
        self.assertEqual(len(reporter.ltp_tconf_cases(report)), 1)
        self.assertEqual(report.ltp_validation["status"], "FAIL")
        self.assertIn(
            "forbidden-status",
            {finding["kind"] for finding in report.ltp_validation["failures"]},
        )
        markdown = reporter.render_markdown([report])
        self.assertIn("TCONF-only（配置性未通过/跳过，未计为 PASS）", markdown)

    def test_render_includes_incomplete_case(self) -> None:
        report = self.parse("RUN LTP CASE access01\n")
        markdown = reporter.render_markdown([report])
        self.assertIn("失败/不完整", markdown)
        self.assertIn("access01", markdown)
        self.assertIn("**ERROR**", markdown)
        empty_markdown = reporter.render_markdown([self.parse("")])
        self.assertIn("empty-output", empty_markdown)
        self.assertIn("zero-ltp-groups", empty_markdown)
        self.assertIn("不得据此判定 PASS", empty_markdown)
        generic_failure = self.parse(
            self.complete_ltp_log()
            + "#### OS COMP TEST GROUP START netperf-musl ####\n"
            + "====== netperf TCP_STREAM end: fail ======\n"
            + "PASS OFFICIAL TEST GROUP netperf-musl : 0\n"
            + "#### OS COMP TEST GROUP END netperf-musl ####\n"
        )
        self.assertEqual(generic_failure.ltp_validation["status"], "PASS")
        self.assertEqual(generic_failure.overall_validation["status"], "FAIL")
        self.assertIn(
            "generic-subtest-failure",
            {
                finding["kind"]
                for finding in generic_failure.overall_validation["failures"]
            },
        )
        self.assertIn("全流可见故障扫描", reporter.render_markdown([generic_failure]))

        with tempfile.TemporaryDirectory() as temporary:
            process_stdout = Path(temporary) / "process.log"
            process_stderr = Path(temporary) / "process.stderr.log"
            process_stdout.write_text(self.complete_ltp_log(), encoding="utf-8")
            process_stderr.write_text("", encoding="utf-8")
            nonzero_process = reporter.parse_log(process_stdout, process_stderr, 7)
        self.assertEqual(nonzero_process.ltp_validation["status"], "FAIL")
        self.assertEqual(nonzero_process.overall_validation["status"], "FAIL")
        self.assertEqual(nonzero_process.process_exit_code, 7)
        scoped_markdown = reporter.render_markdown([self.parse(self.complete_ltp_log())])
        self.assertNotIn("status: **PASS**", scoped_markdown)
        self.assertIn("LTP_SCOPE_COMPLETE_NOT_OFFICIAL_VERDICT", scoped_markdown)
        self.assertIn("NO_VISIBLE_FAILURE_UNVALIDATED", scoped_markdown)

    def test_busybox_failure_report_preserves_ordinal_and_command(self) -> None:
        report = self.parse(
            "#### OS COMP TEST GROUP START busybox-musl ####\n"
            "#### OS COMP BUSYBOX CASE START ordinal=7 ####\n"
            "BUSYBOX CASE RESULT ordinal=7 status=fail command=false\n"
            "#### OS COMP BUSYBOX CASE END ordinal=7 ####\n"
            "#### OS COMP TEST GROUP END busybox-musl ####\n"
        )
        self.assertEqual(
            report.busybox_failures,
            ["busybox-musl: ordinal 7: false"],
        )
        self.assertIn("ordinal 7: false", reporter.render_markdown([report]))

    def test_cli_writes_report_for_clean_utf8_input(self) -> None:
        script = Path(__file__).parents[1] / "evaluation" / "report_evaluation_failures.py"
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            source = directory / "official.log"
            stderr_source = directory / "official.stderr.log"
            output = directory / "report.md"
            source.write_text(self.complete_ltp_log(code=5), encoding="utf-8")
            stderr_source.write_text("", encoding="utf-8")
            result = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(stderr_source),
                    "--process-exit-code",
                    "0",
                    str(source),
                    "--output",
                    str(output),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn(f"wrote {output}", result.stdout)
            rendered = output.read_text(encoding="utf-8")
            self.assertIn("access01", rendered)
            self.assertIn("status: **FAIL**", rendered)
            self.assertIn(str(stderr_source), rendered)
            self.assertIn(hashlib.sha256(source.read_bytes()).hexdigest(), rendered)
            self.assertIn(hashlib.sha256(stderr_source.read_bytes()).hexdigest(), rendered)

            wrong_stderr = directory / "wrong.stderr.log"
            wrong_stderr.write_text("", encoding="utf-8")
            wrong_output = directory / "wrong.md"
            wrong_pair = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(wrong_stderr),
                    "--process-exit-code",
                    "0",
                    str(source),
                    "--output",
                    str(wrong_output),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(wrong_pair.returncode, 2, wrong_pair.stderr)
            self.assertFalse(wrong_output.exists())

            original_capture = source.read_bytes()
            direct_collision = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(stderr_source),
                    "--process-exit-code",
                    "0",
                    str(source),
                    "--output",
                    str(source),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            symlink_output = directory / "capture-symlink.md"
            symlink_output.symlink_to(source)
            symlink_collision = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(stderr_source),
                    "--process-exit-code",
                    "0",
                    str(source),
                    "--output",
                    str(symlink_output),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            hardlink_output = directory / "capture-hardlink.md"
            os.link(source, hardlink_output)
            hardlink_collision = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(stderr_source),
                    "--process-exit-code",
                    "0",
                    str(source),
                    "--output",
                    str(hardlink_output),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            for collision in (direct_collision, symlink_collision, hardlink_collision):
                self.assertEqual(collision.returncode, 2, collision.stderr)
                self.assertIn("must not", collision.stderr)
            self.assertEqual(source.read_bytes(), original_capture)

    def test_cli_rejects_invalid_utf8_log_and_judge_without_report(self) -> None:
        script = Path(__file__).parents[1] / "evaluation" / "report_evaluation_failures.py"
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            malformed = directory / "malformed.log"
            stderr_log = directory / "malformed.stderr.log"
            malformed.write_bytes(b"RUN LTP CASE access\xff01\n")
            stderr_log.write_bytes(b"")
            output = directory / "malformed-report.md"
            malformed_result = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(stderr_log),
                    "--process-exit-code",
                    "0",
                    str(malformed),
                    "--output",
                    str(output),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(malformed_result.returncode, 2, malformed_result.stderr)
            self.assertIn(str(malformed), malformed_result.stderr)
            self.assertIn("invalid UTF-8 at byte offset", malformed_result.stderr)
            self.assertIn("no report was written", malformed_result.stderr)
            self.assertNotIn("wrote", malformed_result.stdout)
            self.assertFalse(output.exists())

            missing_companion_output = directory / "missing-companion.md"
            missing_companion = subprocess.run(
                self.isolated_cli(
                    script,
                    str(malformed),
                    "--output",
                    str(missing_companion_output),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(missing_companion.returncode, 2)
            self.assertFalse(missing_companion_output.exists())

            valid_stdout = directory / "stderr-invalid.log"
            invalid_stderr = directory / "stderr-invalid.stderr.log"
            invalid_stderr_output = directory / "stderr-invalid-report.md"
            valid_stdout.write_text(self.complete_ltp_log(), encoding="utf-8")
            invalid_stderr.write_bytes(b"panic\xff")
            invalid_stderr_result = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(invalid_stderr),
                    "--process-exit-code",
                    "0",
                    str(valid_stdout),
                    "--output",
                    str(invalid_stderr_output),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(invalid_stderr_result.returncode, 2)
            self.assertIn(str(invalid_stderr), invalid_stderr_result.stderr)
            self.assertFalse(invalid_stderr_output.exists())

            bare_cr = directory / "bare-cr.log"
            bare_cr_stderr = directory / "bare-cr.stderr.log"
            bare_cr.write_bytes(self.complete_ltp_log().replace("\n", "\r").encode("utf-8"))
            bare_cr_stderr.write_bytes(b"")
            bare_cr_report = reporter.parse_log(bare_cr, bare_cr_stderr, 0)
            self.assertEqual(bare_cr_report.ltp_validation["status"], "ERROR")
            self.assertIn(
                "invalid-output-control",
                {
                    finding["kind"]
                    for finding in bare_cr_report.ltp_validation["errors"]
                },
            )

            clean_log = directory / "libctest.log"
            clean_stderr = directory / "libctest.stderr.log"
            clean_log.write_text(
                "#### OS COMP TEST GROUP START libctest-musl ####\n"
                "START entry-static.exe demo\nPass!\n"
                "#### OS COMP TEST GROUP END libctest-musl ####\n",
                encoding="utf-8",
            )
            clean_stderr.write_text("", encoding="utf-8")
            judge_dir = directory / "judge"
            judge_dir.mkdir()
            malformed_judge = judge_dir / "judge_libctest-musl.py"
            missing_judge_output = directory / "missing-judge-report.md"
            missing_judge_result = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(clean_stderr),
                    "--process-exit-code",
                    "0",
                    str(clean_log),
                    "--output",
                    str(missing_judge_output),
                    "--judge-dir",
                    str(judge_dir),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(missing_judge_result.returncode, 2)
            self.assertIn("required official judge file is missing", missing_judge_result.stderr)
            self.assertFalse(missing_judge_output.exists())

            malformed_judge.write_bytes(b'libctest_baseline = """\xff"""\n')
            judge_output = directory / "judge-report.md"
            judge_result = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(clean_stderr),
                    "--process-exit-code",
                    "0",
                    str(clean_log),
                    "--output",
                    str(judge_output),
                    "--judge-dir",
                    str(judge_dir),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(judge_result.returncode, 2, judge_result.stderr)
            self.assertIn(str(malformed_judge), judge_result.stderr)
            self.assertIn("invalid UTF-8 at byte offset", judge_result.stderr)
            self.assertFalse(judge_output.exists())

            malformed_judge.write_text(
                'libctest_baseline = """"""\n',
                encoding="utf-8",
            )
            empty_output = directory / "empty-judge-report.md"
            empty_result = subprocess.run(
                self.isolated_cli(
                    script,
                    "--stderr-log",
                    str(clean_stderr),
                    "--process-exit-code",
                    "0",
                    str(clean_log),
                    "--output",
                    str(empty_output),
                    "--judge-dir",
                    str(judge_dir),
                ),
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(empty_result.returncode, 2)
            self.assertIn("baseline is missing or empty", empty_result.stderr)
            self.assertFalse(empty_output.exists())


if __name__ == "__main__":
    unittest.main()
