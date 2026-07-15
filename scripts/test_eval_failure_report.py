#!/usr/bin/env python3
"""Regression tests for deterministic, fail-closed evaluator failure reports."""

from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("eval_failure_report.py")


def valid_log(*, code: int = 0, payload: tuple[str, ...] = ()) -> str:
    return "\n".join(
        (
            "#### OS COMP TEST GROUP START ltp-musl ####",
            "ltp case list: inline (1 cases, timeout 30s)",
            "RUN LTP CASE access01",
            *payload,
            f"FAIL LTP CASE access01 : {code}",
            (
                "ltp cases: 1 passed, 0 failed, 0 timed out"
                if code == 0
                else "ltp cases: 0 passed, 1 failed, 0 timed out"
            ),
            "#### OS COMP TEST GROUP END ltp-musl ####",
            "",
        )
    )


class EvalFailureReportTest(unittest.TestCase):
    def run_report(
        self,
        payload: bytes,
        *,
        require_clean: bool = False,
        judge_dir: Path | None = None,
    ) -> tuple[subprocess.CompletedProcess[str], bytes]:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            log = root / "rv.log"
            output = root / "report.md"
            log.write_bytes(payload)
            command = [sys.executable, str(SCRIPT), str(log), "-o", str(output)]
            if require_clean:
                command.append("--require-clean")
            if judge_dir is not None:
                command.extend(("--judge-dir", str(judge_dir)))
            result = subprocess.run(command, check=False, capture_output=True, text=True)
            rendered = output.read_bytes() if output.exists() else b""
            return result, rendered

    def test_valid_pass_is_clean_and_report_is_deterministic(self) -> None:
        payload = valid_log().encode()
        first, first_report = self.run_report(payload, require_clean=True)
        second, second_report = self.run_report(payload, require_clean=True)
        self.assertEqual(first.returncode, 0, first.stderr)
        self.assertEqual(second.returncode, 0, second.stderr)
        self.assertEqual(first_report, second_report)
        text = first_report.decode()
        self.assertIn("Protocol state: **PASS**", text)
        self.assertIn("Official judge status: not-requested", text)
        self.assertNotIn("生成时间", text)

    def test_valid_semantic_failure_is_reportable_but_required_gate_fails(self) -> None:
        payload = valid_log(code=5).encode()
        report_only, rendered = self.run_report(payload)
        required, _ = self.run_report(payload, require_clean=True)
        self.assertEqual(report_only.returncode, 0, report_only.stderr)
        self.assertEqual(required.returncode, 1, required.stderr)
        self.assertIn("Protocol state: **FAIL**", rendered.decode())
        self.assertIn("access01", rendered.decode())

    def test_tconf_is_not_pass(self) -> None:
        payload = valid_log(payload=("access01 1 TCONF : unsupported",)).encode()
        result, rendered = self.run_report(payload, require_clean=True)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn("Protocol state: **SKIPPED**", rendered.decode())
        self.assertIn("TCONF-only", rendered.decode())

    def test_markdown_escapes_untrusted_protocol_fields(self) -> None:
        payload = (
            valid_log(code=5)
            .replace("access01", "`case|<x>`")
            .encode()
        )
        result, rendered = self.run_report(payload, require_clean=True)
        self.assertEqual(result.returncode, 1, result.stderr)
        text = rendered.decode()
        self.assertNotIn("<x>", text)
        self.assertIn("&lt;x&gt;", text)
        self.assertIn("\\|", text)
        self.assertIn("\\`", text)

    def test_empty_truncated_duplicate_and_invalid_byte_are_integrity_errors(self) -> None:
        malformed = (
            b"",
            b"RUN LTP CASE access01\n",
            valid_log().replace(
                "FAIL LTP CASE access01 : 0",
                "FAIL LTP CASE access01 : 5\nFAIL LTP CASE access01 : 0",
            ).encode(),
            valid_log().replace("FAIL LTP CASE access01 : 0", "FAIL LTP CASE access01 : \xff0").encode("latin-1"),
            b"ltp cases: 0 passed, 1 failed, 0 timed out\n",
            b"orphan 1 TFAIL : failure\n",
        )
        for payload in malformed:
            with self.subTest(payload=payload):
                result, rendered = self.run_report(payload)
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertIn("Evaluator protocol integrity errors", rendered.decode())

    def test_requested_missing_judge_directory_is_error_and_never_zero(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            missing = Path(tmp) / "developer-home" / "missing-pr3-judge-dir"
            result, rendered = self.run_report(valid_log().encode(), judge_dir=missing)
            self.assertEqual(result.returncode, 2, result.stderr)
            text = rendered.decode()
            self.assertIn("Official judge status: error", text)
            self.assertIn("N/A (error)", text)
            self.assertIn("requested judge directory is unavailable", text)
            self.assertNotIn(Path(tmp).as_posix(), text)

    def test_output_cannot_alias_or_overwrite_raw_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            raw = root / "raw.log"
            original = valid_log().encode()
            raw.write_bytes(original)
            aliases = [raw, root / "hardlink.log"]
            aliases[1].hardlink_to(raw)
            for output in aliases:
                with self.subTest(output=output):
                    result = subprocess.run(
                        [sys.executable, str(SCRIPT), str(raw), "-o", str(output), "--require-clean"],
                        check=False,
                        capture_output=True,
                        text=True,
                    )
                    self.assertEqual(result.returncode, 2, result)
                    self.assertEqual(raw.read_bytes(), original)

    def test_invalid_input_removes_stale_owned_output(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            output = root / "report.md"
            missing = root / "missing.log"
            oversized = root / "oversized.log"
            with oversized.open("wb") as stream:
                stream.truncate(64 * 1024 * 1024 + 1)

            for raw in (missing, oversized):
                with self.subTest(raw=raw):
                    output.write_text("stale green report\n", encoding="utf-8")
                    result = subprocess.run(
                        [sys.executable, str(SCRIPT), str(raw), "-o", str(output)],
                        check=False,
                        capture_output=True,
                        text=True,
                    )
                    self.assertEqual(result.returncode, 2, result)
                    self.assertFalse(output.exists())


if __name__ == "__main__":
    unittest.main()
