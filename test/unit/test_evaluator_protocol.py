#!/usr/bin/env python3
"""Adversarial tests for the byte-preserving evaluator protocol parser."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evidence"))

from evaluator_protocol import parse_evaluator_bytes


def one_case(
    *,
    result: bytes = b"FAIL LTP CASE access01 : 0",
    payload: tuple[bytes, ...] = (),
    summary: bytes = b"ltp cases: 1 passed, 0 failed, 0 timed out",
    close: bool = True,
) -> bytes:
    lines = [
        b"#### OS COMP TEST GROUP START ltp-musl ####",
        b"ltp case list: inline (1 cases, timeout 30s)",
        b"RUN LTP CASE access01",
        *payload,
        result,
        summary,
    ]
    if close:
        lines.append(b"#### OS COMP TEST GROUP END ltp-musl ####")
    return b"\n".join(lines) + b"\n"


class EvaluatorProtocolTest(unittest.TestCase):
    def parse(self, data: bytes) -> dict:
        return parse_evaluator_bytes(data)

    def assert_state(self, expected: str, data: bytes) -> dict:
        result = self.parse(data)
        self.assertEqual(result["state"], expected, result)
        return result

    def test_official_fail_token_with_zero_status_is_pass(self) -> None:
        result = self.assert_state("pass", one_case())
        self.assertEqual(result["diagnostics"], [])
        self.assertEqual(result["cases"][0]["raw_status"], "FAIL")
        self.assertEqual(result["cases"][0]["code"], 0)

    def test_misleading_pass_token_with_nonzero_status_is_fail(self) -> None:
        result = self.assert_state(
            "fail",
            one_case(
                result=b"PASS LTP CASE access01 : 5",
                summary=b"ltp cases: 0 passed, 1 failed, 0 timed out",
            ),
        )
        self.assertEqual(result["cases"][0]["state"], "fail")

    def test_internal_markers_never_become_pass(self) -> None:
        expectations = {
            b"access01 1 TFAIL : assertion": "fail",
            b"access01 1 TBROK : setup": "error",
            b"access01 1 TCONF : unsupported": "skipped",
            b"access01: ENOSYS": "fail",
        }
        for line, expected in expectations.items():
            with self.subTest(line=line):
                self.assert_state(expected, one_case(payload=(line,)))

    def test_internal_ltp_summary_is_semantic_evidence(self) -> None:
        fields = (
            b"Summary:",
            b"passed 0",
            b"failed 1",
            b"broken 0",
            b"skipped 0",
            b"warnings 0",
        )
        result = self.assert_state("fail", one_case(payload=fields))
        self.assertEqual(result["cases"][0]["signals"]["ltp_failed"], 1)

    def test_incomplete_internal_ltp_summary_is_error(self) -> None:
        result = self.assert_state(
            "error", one_case(payload=(b"Summary:", b"passed 1", b"failed 0"))
        )
        self.assertIn(
            "incomplete_internal_summary",
            {item["code"] for item in result["diagnostics"]},
        )

    def test_internal_summary_outside_case_is_never_ignored(self) -> None:
        summary = b"\n".join(
            (
                b"Summary:",
                b"passed 0",
                b"failed 1",
                b"broken 0",
                b"skipped 0",
                b"warnings 0",
            )
        )
        valid = one_case()
        variants = (
            valid.replace(b"RUN LTP CASE access01", summary + b"\nRUN LTP CASE access01"),
            valid.replace(
                b"#### OS COMP TEST GROUP END ltp-musl ####",
                summary + b"\n#### OS COMP TEST GROUP END ltp-musl ####",
            ),
        )
        for payload in variants:
            with self.subTest(payload=payload):
                result = self.assert_state("error", payload)
                codes = {item["code"] for item in result["diagnostics"]}
                self.assertIn("internal_summary_outside_case", codes)
                self.assertIn("orphan_internal_summary_field", codes)

    def test_nonzero_result_followed_by_timeout_is_timeout(self) -> None:
        result = self.assert_state(
            "timeout",
            one_case(
                result=b"FAIL LTP CASE access01 : 137\nTIMEOUT LTP CASE access01 after 30s",
                summary=b"ltp cases: 0 passed, 1 failed, 1 timed out",
            ),
        )
        self.assertTrue(result["cases"][0]["timed_out"])

    def test_fatal_signal_anywhere_in_log_is_error(self) -> None:
        variants = (
            b"kernel panicked before boot\n" + one_case(),
            one_case(payload=(b"fatal trap in testcase",)),
            one_case() + b"InstructionNotExist after group\n",
        )
        for data in variants:
            with self.subTest(data=data[-60:]):
                self.assert_state("error", data)

    def test_ansi_csi_and_osc_do_not_hide_fatal_signal(self) -> None:
        for fatal in (
            b"\x1b[38:5:1mpanicked\x1b[0m",
            b"\x1b]0;title\x07kernel trap",
            b"\x1b[?25lTBROK\x1b[?25h",
        ):
            with self.subTest(fatal=fatal):
                self.assert_state("error", one_case(payload=(fatal,)))

    def test_terminal_controls_cannot_split_or_hide_fatal_tokens(self) -> None:
        variants = (
            b"T\x1b[31mFAIL",
            b"T\x1b]title\x07FAIL",
            b"pa\x1bPignored\x1b\\nic",
            b"pa\0nic",
            b"T\x07FAIL",
            b"panX\x08ic",
            b"panX\x1b[Dic",
        )
        for fatal in variants:
            with self.subTest(fatal=fatal):
                result = self.parse(one_case(payload=(fatal,)))
                self.assertNotEqual(result["state"], "pass", result)

    def test_completed_payload_terminal_controls_fail_closed(self) -> None:
        variants = (
            b"\x1b]0;TFAIL assertion hidden in OSC\x07",
            b"\x1bPkernel panic hidden in DCS\x1b\\",
        )
        for payload in variants:
            with self.subTest(payload=payload):
                result = self.assert_state("error", one_case(payload=(payload,)))
                self.assertIn(
                    "malformed_terminal_control",
                    {item["code"] for item in result["diagnostics"]},
                )

    def test_unterminated_terminal_controls_fail_closed(self) -> None:
        variants = (b"T\x1b]0;hiddenFAIL", b"pa\x1bPignorednic", b"\x1b[31")
        for fatal in variants:
            with self.subTest(fatal=fatal):
                result = self.assert_state("error", one_case(payload=(fatal,)))
                self.assertIn(
                    "malformed_terminal_control",
                    {item["code"] for item in result["diagnostics"]},
                )

    def test_non_ltp_group_signals_contribute_to_overall_state(self) -> None:
        def clean_group(payload: bytes) -> bytes:
            if payload.startswith(b"[CONTEST][OFFICIAL]"):
                group = b"official-selection"
            elif b"libctest" in payload:
                group = b"libctest-musl"
            elif payload.startswith(b"testcase busybox"):
                group = b"busybox-musl"
            else:
                group = b"official-selection"
            body = [
                b"#### OS COMP TEST GROUP START " + group + b" ####",
            ]
            if payload.startswith(b"FAIL libctest"):
                body.extend(
                    (
                        b"========== START musl foo ==========",
                        payload,
                        b"========== END musl foo ==========",
                        b"libctest cases: 0 passed, 1 failed, 0 timed out",
                    )
                )
            elif payload == b"libctest cases: 12 passed, 0 failed, 0 timed out":
                body.extend(
                    (
                        b"========== START entry-static.exe foo ==========" ,
                        b"Pass!",
                        b"========== END entry-static.exe foo ==========" ,
                        b"libctest cases: 1 passed, 0 failed, 0 timed out",
                    )
                )
            else:
                body.append(payload)
            body.extend(
            (
                b"#### OS COMP TEST GROUP END " + group + b" ####",
                one_case().rstrip(b"\n"),
                b"",
            )
            )
            return b"\n".join(body)
        expectations = {
            b"FAIL OFFICIAL TEST GROUP busybox : 1": "fail",
            b"kernel panicked in busybox": "error",
            b"busybox returned ENOSYS": "fail",
            b"TIMEOUT OFFICIAL TEST GROUP busybox after 30s": "timeout",
            b"autorun: busybox exited with status 7": "fail",
            b"autorun: /bin/busybox failed: missing binary": "error",
            b"autorun: busybox shell not found": "error",
            b"FAIL libctest musl foo: failed assertion": "fail",
            (
                b"========== START entry-static.exe foo ==========\n"
                b"FAIL libctest entry-static.exe foo: 1\n"
                b"========== END entry-static.exe foo ==========\n"
                b"libctest cases: 0 passed, 1 failed, 0 timed out"
            ): "fail",
            (
                b"========== START entry-static.exe foo ==========\n"
                b"FAIL libctest entry-static.exe foo: timeout\n"
                b"========== END entry-static.exe foo ==========\n"
                b"libctest cases: 0 passed, 1 failed, 1 timed out"
            ): "timeout",
            b"testcase busybox echo fail": "fail",
            b"[CONTEST][OFFICIAL][SKIP] libctest-glibc: configured skip": "skipped",
        }
        for payload, expected in expectations.items():
            with self.subTest(payload=payload):
                result = self.assert_state(expected, clean_group(payload))
                self.assertEqual(result["groups"][-1]["name"], "ltp-musl")

        for payload in (
            b"autorun: busybox exited with status 0",
            b"libctest cases: 12 passed, 0 failed, 0 timed out",
        ):
            with self.subTest(clean_payload=payload):
                self.assert_state("pass", clean_group(payload))

    def test_typed_non_ltp_groups_require_real_completed_work(self) -> None:
        variants = (
            b"#### OS COMP TEST GROUP START libctest-musl ####\n"
            b"#### OS COMP TEST GROUP END libctest-musl ####\n",
            b"#### OS COMP TEST GROUP START libctest-musl ####\n"
            b"libctest cases: 0 passed, 0 failed, 0 timed out\n"
            b"#### OS COMP TEST GROUP END libctest-musl ####\n",
            b"#### OS COMP TEST GROUP START busybox-musl ####\n"
            b"#### OS COMP TEST GROUP END busybox-musl ####\n",
        )
        for group in variants:
            with self.subTest(group=group):
                self.assert_state("error", group + one_case())

        libctest_read_failure = (
            b"#### OS COMP TEST GROUP START libctest-musl ####\n"
            b"libctest: read /musl/run-static.sh failed: NotFound\n"
            b"libctest cases: 0 passed, 1 failed, 0 timed out\n"
            b"#### OS COMP TEST GROUP END libctest-musl ####\n"
        )
        self.assert_state("fail", libctest_read_failure + one_case())

        fabricated_summary = (
            b"#### OS COMP TEST GROUP START libctest-musl ####\n"
            b"libctest cases: 12 passed, 0 failed, 0 timed out\n"
            b"#### OS COMP TEST GROUP END libctest-musl ####\n"
        )
        result = self.assert_state("error", fabricated_summary + one_case())
        self.assertIn(
            "libctest_summary_mismatch",
            {item["code"] for item in result["diagnostics"]},
        )

        for orphan in (
            b"libctest cases: 12 passed, 0 failed, 0 timed out\n",
            b"testcase busybox echo success\n",
        ):
            with self.subTest(orphan=orphan):
                self.assert_state("error", one_case() + orphan)

    def test_zero_case_ltp_group_is_integrity_error(self) -> None:
        empty = b"\n".join(
            (
                b"#### OS COMP TEST GROUP START ltp-musl ####",
                b"ltp case list: empty (0 cases, timeout 30s)",
                b"ltp cases: 0 passed, 0 failed, 0 timed out",
                b"#### OS COMP TEST GROUP END ltp-musl ####",
                b"",
            )
        )
        result = self.assert_state("error", empty)
        self.assertIn("empty_case_list", {item["code"] for item in result["diagnostics"]})

    def test_malformed_official_failure_record_is_integrity_error(self) -> None:
        result = self.assert_state(
            "error", b"FAIL OFFICIAL TEST GROUP busybox : nope\n" + one_case()
        )
        self.assertIn(
            "malformed_control_record",
            {item["code"] for item in result["diagnostics"]},
        )

    def test_malformed_non_ltp_control_records_are_integrity_errors(self) -> None:
        variants = (
            b"testcase busybox echo fa\xffil",
            b"FAIL libctest musl foo: fa\xffil",
            b"libctest cases: 1 passed, x failed, 0 timed out",
            b"autorun: busybox exited with status x",
            b"FAIL OFFICIAL TEST GR\xffOUP busybox : 1",
            b"testcase busy\xffbox echo fail",
            b"autor\xffun: busybox exited with status 7",
            b"TF\xffAIL",
            b"pa\xffnic",
            b"ENO\xffSYS",
            b"TIME\xffOUT",
        )
        for payload in variants:
            with self.subTest(payload=payload):
                result = self.assert_state("error", payload + b"\n" + one_case())
                self.assertIn(
                    "malformed_control_record",
                    {item["code"] for item in result["diagnostics"]},
                )

    def test_non_ascii_case_name_is_integrity_error(self) -> None:
        result = self.assert_state(
            "error",
            one_case().replace(b"access01", b"access\xff01"),
        )
        self.assertIn(
            "malformed_control_record",
            {item["code"] for item in result["diagnostics"]},
        )

    def test_empty_and_truncated_logs_are_errors(self) -> None:
        variants = (
            b"",
            b"RUN LTP CASE access01\n",
            b"#### OS COMP TEST GROUP START ltp-musl ####\nRUN LTP CASE access01\n",
            one_case(close=False),
            one_case(summary=b"unrelated output"),
        )
        for data in variants:
            with self.subTest(data=data):
                result = self.assert_state("error", data)
                self.assertTrue(result["diagnostics"])

    def test_non_ascii_byte_cannot_be_deleted_into_status_zero(self) -> None:
        result = self.assert_state(
            "error",
            one_case(
                result=b"FAIL LTP CASE access01 : \xff0",
                summary=b"ltp cases: 0 passed, 1 failed, 0 timed out",
            ),
        )
        self.assertIn(
            "malformed_control_record",
            {item["code"] for item in result["diagnostics"]},
        )
        self.assertIsNone(result["cases"][0]["code"])

    def test_display_ignorable_byte_cannot_reconstruct_status_zero(self) -> None:
        result = self.assert_state(
            "error",
            one_case(
                result=b"FAIL LTP CASE access01 : \0 0",
                summary=b"ltp cases: 1 passed, 0 failed, 0 timed out",
            ),
        )
        self.assertIn(
            "malformed_control_record",
            {item["code"] for item in result["diagnostics"]},
        )
        self.assertIsNone(result["cases"][0]["code"])

    def test_oversized_numeric_control_field_is_integrity_error(self) -> None:
        result = self.assert_state(
            "error",
            one_case(
                result=b"FAIL LTP CASE access01 : " + b"9" * 5000,
                summary=b"ltp cases: 0 passed, 1 failed, 0 timed out",
            ),
        )
        self.assertIn(
            "malformed_control_record",
            {item["code"] for item in result["diagnostics"]},
        )

    def test_firmware_banner_nul_outside_control_record_is_tolerated(self) -> None:
        result = self.assert_state("pass", b"OpenSBI |___\0/_____|\n" + one_case())
        self.assertEqual(result["diagnostics"], [])

    def test_duplicate_and_mismatched_records_are_errors(self) -> None:
        variants = (
            one_case(result=b"FAIL LTP CASE access01 : 0\nFAIL LTP CASE access01 : 0"),
            one_case(result=b"FAIL LTP CASE other01 : 0"),
            one_case(
                result=b"FAIL LTP CASE access01 : 137\nTIMEOUT LTP CASE access01 after 30s\nTIMEOUT LTP CASE access01 after 30s",
                summary=b"ltp cases: 0 passed, 1 failed, 1 timed out",
            ),
            one_case() + one_case(),
        )
        for data in variants:
            with self.subTest(data=data):
                self.assert_state("error", data)

    def test_declared_and_summary_counts_must_match_observed_cases(self) -> None:
        bad_declared = one_case().replace(b"(1 cases", b"(2 cases")
        bad_summary = one_case(summary=b"ltp cases: 0 passed, 0 failed, 0 timed out")
        for data in (bad_declared, bad_summary):
            with self.subTest(data=data):
                self.assert_state("error", data)

    def test_setup_failure_is_not_clean(self) -> None:
        data = b"\n".join(
            (
                b"#### OS COMP TEST GROUP START ltp-musl ####",
                b"FAIL LTP SETUP ltp-musl : -1",
                b"ltp cases: 0 passed, 1 failed, 0 timed out",
                b"#### OS COMP TEST GROUP END ltp-musl ####",
            )
        )
        self.assert_state("fail", data)

    def test_non_utf8_payload_is_preserved_by_raw_digest_without_parse_error(self) -> None:
        result = self.assert_state("pass", one_case(payload=(b"payload:\xff",)))
        self.assertEqual(result["size_bytes"], len(one_case(payload=(b"payload:\xff",))))
        self.assertEqual(len(result["raw_sha256"]), 64)


if __name__ == "__main__":
    unittest.main()
