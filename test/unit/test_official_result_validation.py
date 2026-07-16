#!/usr/bin/env python3
"""Unit tests for strict official evaluator result validation."""

from __future__ import annotations

import hashlib
import json
import sys
import subprocess
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evaluation"))

import parse_official_results as validator


def group(label: str, body: str) -> str:
    return (
        f"#### OS COMP TEST GROUP START {label} ####\n"
        f"{body.strip()}\n"
        f"#### OS COMP TEST GROUP END {label} ####\n"
    )


def busybox_frame(ordinal: int, command: str, status: str = "success") -> str:
    return "\n".join(
        (
            f"#### OS COMP BUSYBOX CASE START ordinal={ordinal} ####",
            f"BUSYBOX CASE RESULT ordinal={ordinal} status={status} command={command}",
            f"testcase busybox {command} {status}",
            f"#### OS COMP BUSYBOX CASE END ordinal={ordinal} ####",
        )
    )


def busybox_plan(*commands: str) -> list[validator.BusyBoxCase]:
    return [
        validator.BusyBoxCase(ordinal, command)
        for ordinal, command in enumerate(commands, start=1)
    ]


def complete_ltp(
    *,
    case: str = "access01",
    code: int = 0,
    internal: str = "",
    planned: int = 1,
    summary_passed: int | None = None,
    summary_failed: int | None = None,
) -> str:
    passed = int(code == 0) if summary_passed is None else summary_passed
    failed = int(code != 0) if summary_failed is None else summary_failed
    pass_record = "Pass!\n" if code == 0 else ""
    body = f"""
ltp case list: stable ({planned} cases, timeout 180s)
========== START ltp {case} ==========
RUN LTP CASE {case}
{internal}
FAIL LTP CASE {case} : {code}
{pass_record}LTP CASE RUNTIME {case}: 12 ms
========== END ltp {case} ==========
ltp cases: {passed} passed, {failed} failed, 0 timed out
"""
    return group("ltp-musl", body)


class OfficialResultValidationTest(unittest.TestCase):
    def assert_status(self, text: str, status: str) -> dict[str, object]:
        labels = [
            match.group(1)
            for line in text.splitlines()
            if (match := validator.GROUP_START_RE.fullmatch(line))
        ]
        expected_counts = {
            label: 1
            for label in labels
            if label.startswith(("busybox-", "libctest-"))
        }
        result = validator.validate_official_output(
            text,
            expected_group_case_counts=expected_counts,
        )
        self.assertEqual(result["status"], status, result)
        return result

    def test_empty_output_is_error(self) -> None:
        result = self.assert_status("", "ERROR")
        self.assertGreater(result["error_count"], 0)

    def test_control_byte_in_otherwise_valid_output_is_error(self) -> None:
        text = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0") + "\x00"
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "invalid-output-control" for item in result["errors"]))

    def test_unicode_controls_and_formatting_are_errors(self) -> None:
        for character in ("\u0085", "\u2028", "\u202e"):
            with self.subTest(codepoint=f"U+{ord(character):04X}"):
                text = group(
                    "demo-musl",
                    f"F{character}AIL\nPASS OFFICIAL TEST GROUP demo-musl : 0",
                )
                result = self.assert_status(text, "ERROR")
                self.assertTrue(
                    any(item["kind"] == "invalid-output-control" for item in result["errors"]),
                    result,
                )

    def test_only_trusted_ansi_styling_and_crlf_are_normalized(self) -> None:
        text = group(
            "demo-musl",
            "\x1b[H\x1b[J\x1b[32mPASS OFFICIAL TEST GROUP demo-musl : 0\x1b[0m",
        ).replace("\n", "\r\n")
        self.assert_status(text, "PASS")
        self.assert_status(
            group(
                "demo-musl",
                "F\x1b[H\x1b[J\x1b[31mAIL\x1b[0m\n"
                "PASS OFFICIAL TEST GROUP demo-musl : 0",
            ),
            "FAIL",
        )

    def test_non_styling_csi_and_bare_carriage_return_are_errors(self) -> None:
        for index, control in enumerate(("\x1b[H", "\x1b[J", "\x1b[2J", "\x1b[2A", "\r")):
            with self.subTest(control=index):
                text = group(
                    "demo-musl",
                    f"F{control}AIL\nPASS OFFICIAL TEST GROUP demo-musl : 0",
                )
                self.assert_status(text, "ERROR")

    def test_no_groups_is_error_even_with_success_word(self) -> None:
        self.assert_status("PASS\n", "ERROR")

    def test_missing_group_end_is_error(self) -> None:
        self.assert_status("#### OS COMP TEST GROUP START demo-musl ####\n", "ERROR")

    def test_unmatched_group_end_is_error(self) -> None:
        self.assert_status("#### OS COMP TEST GROUP END demo-musl ####\n", "ERROR")

    def test_mismatched_group_end_is_error(self) -> None:
        self.assert_status(
            "#### OS COMP TEST GROUP START demo-musl ####\n"
            "#### OS COMP TEST GROUP END demo-glibc ####\n",
            "ERROR",
        )

    def test_nested_groups_are_error(self) -> None:
        self.assert_status(
            "#### OS COMP TEST GROUP START one-musl ####\n"
            "#### OS COMP TEST GROUP START two-musl ####\n"
            "#### OS COMP TEST GROUP END one-musl ####\n",
            "ERROR",
        )

    def test_duplicate_groups_are_error(self) -> None:
        text = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0") * 2
        self.assert_status(text, "ERROR")

    def test_generic_group_requires_explicit_success_record(self) -> None:
        self.assert_status(group("demo-musl", "benchmark completed"), "ERROR")

    def test_generic_group_with_explicit_zero_success_passes(self) -> None:
        self.assert_status(group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0"), "PASS")
        controlled_cleanup = (
            "autorun: cyclictest-musl timeout bounded to 900s (nominal 1200s)\n"
            + group(
                "demo-musl",
                "Signal 2 caught, longjmp'ing out!\n"
                "sending SIGTERM to all child processes\n"
                "signaling 240 worker threads to terminate\n"
                "PASS OFFICIAL TEST GROUP demo-musl : 0",
            )
        )
        self.assert_status(controlled_cleanup, "PASS")

    def test_generic_zero_or_incomplete_execution_markers_are_errors(self) -> None:
        for marker in (
            "NO TESTS RAN",
            "running 0 tests",
            "executed 0 tests",
            "tests run: 0",
            "collected 0 items",
            "0 tests collected",
            "no tests were run",
            "zero tests were executed",
            "nothing to run",
            "zero tests executed",
            "ran zero tests",
            "0 cases executed",
            "NO CASES RAN",
            "no runnable tests",
            "test suite is empty",
            "NOT-RUN",
            "NOT_EXECUTED",
            "UNEXECUTED",
            "DID NOT RUN",
            "NOT ATTEMPTED",
            "INCOMPLETE",
            "PARTIAL",
            "PARTIALLY EXECUTED",
            "INFRA_ERROR",
            "UNKNOWN STATUS",
            "status unknown",
            "result unknown",
            "case foo: INCOMPLETE",
            "case foo: PARTIAL",
            "case foo: INFRA_ERROR",
            "case foo: UNEXECUTED",
            "case foo: DID_NOT_RUN",
            "case foo: NOT_ATTEMPTED",
            "test foo: UNKNOWN",
            "STATUS NOT_RUN",
        ):
            with self.subTest(marker=marker):
                self.assert_status(
                    group(
                        "demo-musl",
                        f"{marker}\nPASS OFFICIAL TEST GROUP demo-musl : 0",
                    ),
                    "ERROR",
                )

    def test_generic_timeout_signal_and_tap_failures_cannot_pass(self) -> None:
        for marker in (
            "Killed",
            "Terminated",
            "SIGSEGV",
            "signal 11",
            "command timed out",
            "timed_out",
            "deadline exceeded",
            "watchdog expired",
            "deadline_exceeded",
            "watchdog_expired",
            "command_timed_out",
            "timeout_error",
            "ETIMEDOUT",
            "signal: 11",
            "signal=11",
            "core dump",
            "IllegalInstruction",
            "SegmentationFault",
            "trap 13",
            "not successful",
            "exit status 1",
            "return: 1",
            "TIME_LIMIT_EXCEEDED",
            "not ok 1 - smoke",
        ):
            with self.subTest(marker=marker):
                self.assert_status(
                    group(
                        "demo-musl",
                        f"{marker}\nPASS OFFICIAL TEST GROUP demo-musl : 0",
                    ),
                    "FAIL",
                )

    def test_generic_group_success_record_must_be_terminal(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\nstarting second phase",
        )
        self.assert_status(text, "ERROR")

    def test_generic_unknown_status_record_variants_are_errors(self) -> None:
        for record in (
            "STATUS=MAYBE",
            "RESULT = MAYBE",
            "STATE: MAYBE",
            "CASE_STATUS=MAYBE",
            "CASE_STATUS[MAYBE] demo",
        ):
            with self.subTest(record=record):
                text = group(
                    "demo-musl",
                    f"{record}\nPASS OFFICIAL TEST GROUP demo-musl : 0",
                )
                self.assert_status(text, "ERROR")

    def test_generic_unexecuted_state_synonyms_are_errors(self) -> None:
        for record in (
            "NOT EXECUTED",
            "PENDING",
            "CANCELLED",
            "DISABLED",
            "OMITTED",
            "[NOT_RUN] case demo",
            "[UNSUPPORTED] feature demo",
            "NOT EXECUTED: demo",
            "case demo: PENDING",
        ):
            with self.subTest(record=record):
                text = group(
                    "demo-musl",
                    f"{record}\nPASS OFFICIAL TEST GROUP demo-musl : 0",
                )
                self.assert_status(text, "ERROR")

    def test_generic_group_failure_is_failure_not_pass(self) -> None:
        self.assert_status(group("demo-musl", "FAIL OFFICIAL TEST GROUP demo-musl : 7"), "FAIL")

    def test_generic_group_rejects_extra_nonzero_success_record(self) -> None:
        body = (
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
            "PASS OFFICIAL TEST GROUP demo-musl : 7"
        )
        self.assert_status(group("demo-musl", body), "ERROR")

    def test_generic_group_rejects_success_for_another_label(self) -> None:
        body = (
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
            "PASS OFFICIAL TEST GROUP other-musl : 0"
        )
        self.assert_status(group("demo-musl", body), "ERROR")

    def test_not_pass_text_cannot_match_success_record(self) -> None:
        self.assert_status(group("demo-musl", "NOT PASS OFFICIAL TEST GROUP demo-musl : 0"), "ERROR")

    def test_configured_skip_is_failure(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
            "[CONTEST][OFFICIAL][SKIP] demo-musl: configured skip",
        )
        self.assert_status(text, "FAIL")

    def test_complete_zero_status_ltp_passes(self) -> None:
        result = self.assert_status(complete_ltp(), "PASS")
        self.assertEqual(result["groups"][0]["executed_cases"], 1)
        scoped = validator.validate_ltp_output(
            complete_ltp()
            + group("demo-musl", "FAIL OFFICIAL TEST GROUP demo-musl : 7")
        )
        self.assertEqual(scoped["status"], "PASS", scoped)
        self.assertEqual(scoped["validation_scope"], "ltp")
        self.assertEqual(
            scoped["groups"][0]["cases"],
            [
                {
                    "case": "access01",
                    "code": 0,
                    "events": ["START", "RUN", "RESULT", "PASS", "END"],
                }
            ],
        )
        incomplete = validator.validate_ltp_output(
            complete_ltp().replace("Pass!\n", "")
        )
        self.assertEqual(incomplete["status"], "ERROR", incomplete)
        benign_diagnostics = (
            "tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s\n"
            "signal01 1 TPASS: signal 11 was rejected as expected\n"
            "futex01 1 TPASS: futex failed as expected: ETIMEDOUT (110)\n"
            "     --vm-hang         hang in a sleep loop after memory allocated\n"
            "passed   1\nfailed   0\nbroken   0\nskipped   0\nwarnings 0"
        )
        self.assert_status(complete_ltp(internal=benign_diagnostics), "PASS")

    def test_prefixed_ltp_result_record_is_malformed(self) -> None:
        text = complete_ltp().replace(
            "FAIL LTP CASE access01 : 0",
            "NOT FAIL LTP CASE access01 : 0",
        )
        self.assert_status(text, "ERROR")

    def test_ltp_explicit_official_failure_cannot_pass(self) -> None:
        text = complete_ltp().replace(
            "#### OS COMP TEST GROUP END ltp-musl ####",
            "FAIL OFFICIAL TEST GROUP ltp-musl : 7\n"
            "#### OS COMP TEST GROUP END ltp-musl ####",
        )
        self.assert_status(text, "FAIL")

    def test_ltp_numeric_status_overrides_fail_label(self) -> None:
        result = self.assert_status(complete_ltp(code=0), "PASS")
        self.assertEqual(result["groups"][0]["passed_cases"], 1)

    def test_ltp_nonzero_result_fails(self) -> None:
        self.assert_status(complete_ltp(code=5), "FAIL")

    def test_ltp_tconf_fails_even_with_zero_wrapper_status(self) -> None:
        self.assert_status(complete_ltp(internal="access01 1 TCONF: unsupported"), "FAIL")

    def test_ltp_tfail_fails_even_with_zero_wrapper_status(self) -> None:
        self.assert_status(complete_ltp(internal="access01 1 TFAIL: mismatch"), "FAIL")
        for marker in (
            "access01 1 TBROK: setup failed",
            "access01 1 TPASS: syscall returned ENOSYS",
            "access01 1 TPASS: kernel panic was hidden",
            "access01 1 TINFO: Test timeouted, sending SIGKILL!",
            "failed 1",
            "broken 1",
            "skipped 1",
            "warnings 1",
            "Test timeouted, sending SIGKILL!",
            "kernel panic: fatal",
        ):
            with self.subTest(marker=marker):
                self.assert_status(complete_ltp(internal=marker), "FAIL")

    def test_ltp_zero_case_manifest_is_error(self) -> None:
        text = group("ltp-musl", "ltp case list: stable (0 cases, timeout 180s)\nltp cases: 0 passed, 0 failed, 0 timed out")
        self.assert_status(text, "ERROR")

    def test_ltp_planned_executed_mismatch_is_error(self) -> None:
        self.assert_status(complete_ltp(planned=2, summary_passed=1, summary_failed=0), "ERROR")

    def test_ltp_self_reported_shrink_cannot_satisfy_trusted_plan(self) -> None:
        result = validator.validate_official_output(
            complete_ltp(planned=1),
            expected_group_case_counts={"ltp-musl": 1000},
            expected_ltp_case_list="stable-full",
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "official-reported-plan-mismatch" for item in result["errors"]),
            result,
        )

    def test_ltp_case_list_name_must_match_trusted_selection(self) -> None:
        result = validator.validate_official_output(
            complete_ltp(),
            expected_group_case_counts={"ltp-musl": 1},
            expected_ltp_case_list="stable-full",
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "official-ltp-case-list-mismatch" for item in result["errors"]),
            result,
        )

    def test_ltp_case_identity_must_match_trusted_plan(self) -> None:
        result = validator.validate_official_output(
            complete_ltp(case="invented01"),
            expected_group_case_counts={"ltp-musl": 1},
            expected_ltp_case_list="stable",
            expected_ltp_cases=["access01"],
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "ltp-case-plan-mismatch" for item in result["errors"]),
            result,
        )

    def test_ltp_case_identity_plan_passes_when_exact(self) -> None:
        result = validator.validate_official_output(
            complete_ltp(),
            expected_group_case_counts={"ltp-musl": 1},
            expected_ltp_case_list="stable",
            expected_ltp_cases=["access01"],
        )
        self.assertEqual(result["status"], "PASS", result)

    def test_ltp_nested_case_lifecycles_are_error(self) -> None:
        body = """
ltp case list: stable (2 cases, timeout 180s)
========== START ltp first ==========
RUN LTP CASE first
FAIL LTP CASE first : 0
Pass!
========== START ltp second ==========
RUN LTP CASE second
FAIL LTP CASE second : 0
Pass!
========== END ltp second ==========
========== END ltp first ==========
ltp cases: 2 passed, 0 failed, 0 timed out
"""
        result = validator.validate_official_output(
            group("ltp-musl", body),
            expected_group_case_counts={"ltp-musl": 2},
            expected_ltp_cases=["first", "second"],
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "ltp-nested-case" for item in result["errors"]),
            result,
        )

    def test_ltp_missing_result_is_error(self) -> None:
        text = complete_ltp().replace("FAIL LTP CASE access01 : 0\n", "")
        self.assert_status(text, "ERROR")

    def test_ltp_missing_case_end_is_error(self) -> None:
        text = complete_ltp().replace("========== END ltp access01 ==========\n", "")
        self.assert_status(text, "ERROR")

    def test_ltp_duplicate_run_is_error(self) -> None:
        text = complete_ltp().replace("RUN LTP CASE access01\n", "RUN LTP CASE access01\nRUN LTP CASE access01\n")
        self.assert_status(text, "ERROR")

    def test_ltp_summary_count_mismatch_is_error(self) -> None:
        self.assert_status(complete_ltp(summary_passed=0, summary_failed=0), "ERROR")

    def test_ltp_event_order_is_required(self) -> None:
        text = complete_ltp().replace(
            "========== START ltp access01 ==========\nRUN LTP CASE access01\n",
            "RUN LTP CASE access01\n========== START ltp access01 ==========\n",
        )
        self.assert_status(text, "ERROR")

    def test_ltp_case_list_must_precede_first_case(self) -> None:
        manifest = "ltp case list: stable (1 cases, timeout 180s)"
        text = complete_ltp().replace(f"{manifest}\n", "")
        text = text.replace(
            "ltp cases: 1 passed, 0 failed, 0 timed out",
            f"ltp cases: 1 passed, 0 failed, 0 timed out\n{manifest}",
        )
        self.assert_status(text, "ERROR")

    def test_ltp_summary_must_follow_all_case_ends(self) -> None:
        summary = "ltp cases: 1 passed, 0 failed, 0 timed out"
        text = complete_ltp().replace(f"{summary}\n", "")
        text = text.replace(
            "ltp case list: stable (1 cases, timeout 180s)\n",
            f"ltp case list: stable (1 cases, timeout 180s)\n{summary}\n",
        )
        self.assert_status(text, "ERROR")

    def test_ltp_summary_must_match_numeric_results_exactly(self) -> None:
        text = complete_ltp(code=5, summary_passed=1, summary_failed=0)
        self.assert_status(text, "ERROR")

    def test_busybox_complete_success_passes(self) -> None:
        self.assert_status(group("busybox-musl", busybox_frame(1, "echo ok")), "PASS")
        self.assert_status(
            group(
                "busybox-musl",
                f"cut mktemp timeout\n{busybox_frame(1, 'echo ok')}",
            ),
            "PASS",
        )

    def test_busybox_requires_trusted_expected_count(self) -> None:
        text = group("busybox-musl", busybox_frame(1, "echo ok"))
        result = validator.validate_official_output(text)
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "official-case-count-missing" for item in result["errors"]))

    def test_busybox_partial_execution_cannot_satisfy_expected_count(self) -> None:
        text = group("busybox-musl", busybox_frame(1, "echo ok"))
        result = validator.validate_official_output(
            text,
            expected_group_case_counts={"busybox-musl": 55},
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "official-planned-executed-mismatch" for item in result["errors"]))

    def test_busybox_identity_must_match_trusted_plan(self) -> None:
        text = group("busybox-musl", busybox_frame(1, "invented command"))
        result = validator.validate_official_output(
            text,
            expected_group_case_counts={"busybox-musl": 1},
            expected_busybox_cases=busybox_plan("echo expected"),
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "busybox-case-plan-mismatch" for item in result["errors"]),
            result,
        )

    def test_busybox_identity_plan_passes_when_exact(self) -> None:
        for label in ("busybox-musl", "busybox-glibc"):
            with self.subTest(label=label):
                text = group(label, busybox_frame(1, "echo expected"))
                result = validator.validate_official_output(
                    text,
                    expected_group_case_counts={label: 1},
                    expected_busybox_cases=busybox_plan("echo expected"),
                )
                self.assertEqual(result["status"], "PASS", result)

    def test_busybox_repeated_text_at_distinct_ordinals_is_valid(self) -> None:
        command = 'echo "bbbbbbb" >> test.txt'
        body = f"{busybox_frame(1, command)}\n{busybox_frame(2, command)}"
        result = validator.validate_official_output(
            group("busybox-musl", body),
            expected_group_case_counts={"busybox-musl": 2},
            expected_busybox_cases=busybox_plan(command, command),
        )
        self.assertEqual(result["status"], "PASS", result)
        self.assertEqual(result["planned_case_count"], 2)
        self.assertEqual(result["executed_case_count"], 2)
        self.assertEqual(result["completed_case_count"], 2)
        self.assertEqual(
            [case["ordinal"] for case in result["groups"][0]["cases"]],
            [1, 2],
        )

    def test_busybox_missing_extra_and_order_drift_are_errors(self) -> None:
        expected = busybox_plan("first", "second")
        scenarios = {
            "missing": busybox_frame(1, "first"),
            "extra": "\n".join(
                (
                    busybox_frame(1, "first"),
                    busybox_frame(2, "second"),
                    busybox_frame(3, "third"),
                )
            ),
            "order": f"{busybox_frame(2, 'second')}\n{busybox_frame(1, 'first')}",
        }
        for label, body in scenarios.items():
            with self.subTest(label=label):
                result = validator.validate_official_output(
                    group("busybox-musl", body),
                    expected_group_case_counts={"busybox-musl": 2},
                    expected_busybox_cases=expected,
                )
                self.assertEqual(result["status"], "ERROR", result)
                self.assertTrue(result["errors"], result)

    def test_busybox_malformed_or_incomplete_frames_are_errors(self) -> None:
        scenarios = {
            "zero-identity": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=0 ####",
                    "BUSYBOX CASE RESULT ordinal=0 status=success command=first",
                    "#### OS COMP BUSYBOX CASE END ordinal=0 ####",
                )
            ),
            "result-mismatch": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "BUSYBOX CASE RESULT ordinal=2 status=success command=first",
                    "#### OS COMP BUSYBOX CASE END ordinal=1 ####",
                )
            ),
            "missing-result": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "#### OS COMP BUSYBOX CASE END ordinal=1 ####",
                )
            ),
            "missing-end": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "BUSYBOX CASE RESULT ordinal=1 status=success command=first",
                    "testcase busybox first success",
                )
            ),
            "orphan-result": "BUSYBOX CASE RESULT ordinal=1 status=success command=first",
            "missing-compatibility": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "BUSYBOX CASE RESULT ordinal=1 status=success command=first",
                    "#### OS COMP BUSYBOX CASE END ordinal=1 ####",
                )
            ),
            "compatibility-command-mismatch": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "BUSYBOX CASE RESULT ordinal=1 status=success command=first",
                    "testcase busybox second success",
                    "#### OS COMP BUSYBOX CASE END ordinal=1 ####",
                )
            ),
            "compatibility-status-mismatch": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "BUSYBOX CASE RESULT ordinal=1 status=success command=first",
                    "testcase busybox first fail",
                    "#### OS COMP BUSYBOX CASE END ordinal=1 ####",
                )
            ),
            "compatibility-before-result": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "testcase busybox first success",
                    "BUSYBOX CASE RESULT ordinal=1 status=success command=first",
                    "#### OS COMP BUSYBOX CASE END ordinal=1 ####",
                )
            ),
            "duplicate-compatibility": "\n".join(
                (
                    "#### OS COMP BUSYBOX CASE START ordinal=1 ####",
                    "BUSYBOX CASE RESULT ordinal=1 status=success command=first",
                    "testcase busybox first success",
                    "testcase busybox first success",
                    "#### OS COMP BUSYBOX CASE END ordinal=1 ####",
                )
            ),
            "orphan-compatibility": (
                "testcase busybox first success\n" + busybox_frame(1, "first")
            ),
            "mixed-with-fail": (
                f"{busybox_frame(1, 'first', 'fail')}\n"
                "testcase busybox first success"
            ),
        }
        for label, body in scenarios.items():
            with self.subTest(label=label):
                result = validator.validate_official_output(
                    group("busybox-musl", body),
                    expected_group_case_counts={"busybox-musl": 1},
                    expected_busybox_cases=busybox_plan("first"),
                )
                self.assertEqual(result["status"], "ERROR", result)
                self.assertTrue(result["errors"], result)
                if label == "mixed-with-fail":
                    self.assertIn(
                        "busybox-failure",
                        {item["kind"] for item in result["failures"]},
                    )
                expected_kinds = {
                    "missing-compatibility": "busybox-missing-compatibility-result",
                    "compatibility-command-mismatch": "busybox-compatibility-mismatch",
                    "compatibility-status-mismatch": "busybox-compatibility-mismatch",
                    "compatibility-before-result": "busybox-compatibility-before-result",
                    "duplicate-compatibility": "busybox-duplicate-compatibility-result",
                    "orphan-compatibility": "busybox-orphan-compatibility-result",
                }
                if expected_kind := expected_kinds.get(label):
                    self.assertIn(
                        expected_kind,
                        {item["kind"] for item in result["errors"]},
                        result,
                    )

    def test_busybox_legacy_text_protocol_remains_fail_closed(self) -> None:
        result = validator.validate_official_output(
            group("busybox-musl", "testcase busybox echo expected success"),
            expected_group_case_counts={"busybox-musl": 1},
            expected_busybox_cases=busybox_plan("echo expected"),
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertEqual(result["executed_case_count"], 1)
        self.assertEqual(result["completed_case_count"], 1)
        self.assertIn(
            "busybox-legacy-identity",
            {item["kind"] for item in result["errors"]},
        )

    def test_prefixed_busybox_result_record_is_malformed(self) -> None:
        self.assert_status(
            group(
                "busybox-musl",
                "NOT BUSYBOX CASE RESULT ordinal=1 status=success command=echo",
            ),
            "ERROR",
        )

    def test_duplicate_busybox_case_is_error_even_when_count_matches(self) -> None:
        body = f"{busybox_frame(1, 'echo ok')}\n{busybox_frame(1, 'echo ok')}"
        result = validator.validate_official_output(
            group("busybox-musl", body),
            expected_group_case_counts={"busybox-musl": 2},
            expected_busybox_cases=busybox_plan("echo ok", "echo ok"),
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "busybox-duplicate-identity" for item in result["errors"]),
            result,
        )

    def test_busybox_explicit_official_failure_cannot_pass(self) -> None:
        body = (
            f"{busybox_frame(1, 'echo ok')}\n"
            "FAIL OFFICIAL TEST GROUP busybox-musl : 7"
        )
        self.assert_status(group("busybox-musl", body), "FAIL")

    def test_busybox_empty_group_is_error(self) -> None:
        self.assert_status(group("busybox-musl", "busybox started"), "ERROR")

    def test_busybox_failure_is_failure(self) -> None:
        result = validator.validate_official_output(
            group("busybox-musl", busybox_frame(1, "false", "fail")),
            expected_group_case_counts={"busybox-musl": 1},
            expected_busybox_cases=busybox_plan("false"),
        )
        self.assertEqual(result["status"], "FAIL", result)
        self.assertEqual(result["error_count"], 0, result)
        self.assertGreater(result["failure_count"], 0, result)
        self.assertEqual(result["groups"][0]["failed_cases"], 1, result)
        self.assertEqual(result["groups"][0]["compatibility_result_cases"], 1, result)
        for marker in ("TFAIL: hidden mismatch", "kernel panic: fatal"):
            with self.subTest(marker=marker):
                self.assert_status(
                    group(
                        "busybox-musl",
                        f"{marker}\n{busybox_frame(1, 'echo ok')}",
                    ),
                    "FAIL",
                )

    def test_libctest_complete_success_passes(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        self.assert_status(text, "PASS")

    def test_libctest_summary_must_follow_all_case_ends(self) -> None:
        text = group(
            "libctest-musl",
            "libctest cases: 1 passed, 0 failed, 0 timed out\n"
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs ==========",
        )
        self.assert_status(text, "ERROR")

    def test_libctest_requires_trusted_expected_count(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        result = validator.validate_official_output(text)
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "official-case-count-missing" for item in result["errors"]))

    def test_libctest_partial_execution_cannot_satisfy_expected_count(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        result = validator.validate_official_output(
            text,
            expected_group_case_counts={"libctest-musl": 217},
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "official-planned-executed-mismatch" for item in result["errors"]))

    def test_libctest_identity_must_match_trusted_plan(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe invented ==========\n"
            "Pass!\n"
            "========== END entry-static.exe invented ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        result = validator.validate_official_output(
            text,
            expected_group_case_counts={"libctest-musl": 1},
            expected_libctest_cases=[("entry-static.exe", "expected")],
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "libctest-case-plan-mismatch" for item in result["errors"]),
            result,
        )

    def test_libctest_identity_plan_passes_when_exact(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe expected ==========\n"
            "Pass!\n"
            "========== END entry-static.exe expected ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        result = validator.validate_official_output(
            text,
            expected_group_case_counts={"libctest-musl": 1},
            expected_libctest_cases=[("entry-static.exe", "expected")],
        )
        self.assertEqual(result["status"], "PASS", result)

    def test_libctest_explicit_official_failure_cannot_pass(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out\n"
            "FAIL OFFICIAL TEST GROUP libctest-musl : 7",
        )
        self.assert_status(text, "FAIL")

    def test_duplicate_libctest_case_is_error(self) -> None:
        lifecycle = (
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs =========="
        )
        text = group(
            "libctest-musl",
            f"{lifecycle}\n{lifecycle}\n"
            "libctest cases: 2 passed, 0 failed, 0 timed out",
        )
        self.assert_status(text, "ERROR")

    def test_libctest_zero_execution_is_error(self) -> None:
        self.assert_status(group("libctest-musl", "libctest cases: 0 passed, 0 failed, 0 timed out"), "ERROR")

    def test_libctest_failure_is_failure(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "FAIL libctest entry-static.exe abs: 1\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 0 passed, 1 failed, 0 timed out",
        )
        self.assert_status(text, "FAIL")

    def test_libctest_missing_terminal_is_error(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        self.assert_status(text, "ERROR")

    def test_libctest_missing_case_end_is_error(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        self.assert_status(text, "ERROR")

    def test_expected_group_plan_must_match_order_and_membership(self) -> None:
        text = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
        result = validator.validate_official_output(
            text,
            expected_group_labels=["demo-musl", "demo-glibc"],
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "official-group-plan-mismatch" for item in result["errors"]))

    def test_stderr_cannot_close_stdout_group(self) -> None:
        stdout = (
            "#### OS COMP TEST GROUP START demo-musl ####\n"
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
        )
        stderr = "#### OS COMP TEST GROUP END demo-musl ####\n"
        result = validator.validate_official_output(stdout, stderr)
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "missing-group-end" for item in result["errors"]))

    def test_group_start_on_stderr_is_error(self) -> None:
        stdout = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
        stderr = "#### OS COMP TEST GROUP START stray-musl ####\n"
        result = validator.validate_official_output(stdout, stderr)
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "protocol-record-on-stderr" for item in result["errors"]))

    def test_group_end_on_stderr_is_error(self) -> None:
        stdout = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
        stderr = "#### OS COMP TEST GROUP END stray-musl ####\n"
        result = validator.validate_official_output(stdout, stderr)
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "protocol-record-on-stderr" for item in result["errors"]))

    def test_stderr_non_pass_markers_cannot_pass(self) -> None:
        stdout = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
        for marker in ("TCONF", "TBROK", "TFAIL", "ENOSYS", "TIMEOUT OFFICIAL TEST GROUP"):
            with self.subTest(marker=marker):
                result = validator.validate_official_output(stdout, f"runtime {marker}: evidence\n")
                self.assertEqual(result["status"], "FAIL", result)

    def test_ltp_machine_result_on_stderr_is_error(self) -> None:
        result = validator.validate_official_output(complete_ltp(), "FAIL LTP CASE stray : 7\n")
        self.assertEqual(result["status"], "ERROR", result)

    def test_busybox_machine_result_on_stderr_is_error(self) -> None:
        stdout = group("busybox-musl", busybox_frame(1, "echo ok"))
        result = validator.validate_official_output(
            stdout,
            "BUSYBOX CASE RESULT ordinal=1 status=fail command=false\n",
        )
        self.assertEqual(result["status"], "ERROR", result)

    def test_libctest_machine_result_on_stderr_is_error(self) -> None:
        stdout = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out",
        )
        result = validator.validate_official_output(stdout, "FAIL libctest entry.exe abs: 7\n")
        self.assertEqual(result["status"], "ERROR", result)

    def test_benign_stderr_after_complete_stdout_cannot_pass(self) -> None:
        stdout = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
        result = validator.validate_official_output(
            stdout,
            "starting second phase\n",
            expected_group_labels=["demo-musl"],
        )
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(
            any(item["kind"] == "unaccounted-stderr-output" for item in result["errors"]),
            result,
        )

    def test_trusted_cargo_build_diagnostics_on_stderr_are_allowed(self) -> None:
        stdout = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
        stderr = (
            "warning: fixture warning\n"
            "  --> fixture.rs:1:1\n"
            "   |\n"
            " 1 | fixture\n"
            "   | ^^^^^^^\n"
            "...\n"
            "    Finished `release` profile in 0.01s\n"
        )
        result = validator.validate_official_output(
            stdout,
            stderr,
            expected_group_labels=["demo-musl"],
        )
        self.assertEqual(result["status"], "PASS", result)
        normal_ltp_wording = validator.validate_official_output(
            group(
                "demo-musl",
                "TPASS: clone returned 241\n"
                "TPASS: removexattr failed as expected\n"
                "PASS OFFICIAL TEST GROUP demo-musl : 0",
            ),
            expected_group_labels=["demo-musl"],
        )
        self.assertEqual(normal_ltp_wording["status"], "PASS", normal_ltp_wording)
        for stream, marker in (
            ("stdout", "autorun: smoke exited with status 7"),
            ("stderr", "warning: subprocess exited with status 7"),
            ("stderr", "warning: subprocess exited with code 7"),
            ("stderr", "warning: command returned 7"),
            ("stderr", "warning: qemu exit code 7"),
            ("stderr", "warning: qemu exit code: 7"),
            ("stderr", "warning: subprocess failed with exit code 7"),
            ("stderr", "warning: command returned status 7"),
            ("stderr", "warning: command returned code 7"),
        ):
            with self.subTest(stream=stream):
                marked_stdout = group(
                    "demo-musl",
                    (
                        f"{marker}\nPASS OFFICIAL TEST GROUP demo-musl : 0"
                        if stream == "stdout"
                        else "PASS OFFICIAL TEST GROUP demo-musl : 0"
                    ),
                )
                marked = validator.validate_official_output(
                    marked_stdout,
                    marker + "\n" if stream == "stderr" else "",
                    expected_group_labels=["demo-musl"],
                )
                self.assertEqual(marked["status"], "FAIL", marked)
                self.assertIn(
                    "explicit-nonzero",
                    {finding["kind"] for finding in marked["failures"]},
                )

    def test_ltp_summary_on_stderr_is_error(self) -> None:
        stderr = "ltp cases: 99 passed, 0 failed, 0 timed out\n"
        result = validator.validate_official_output(complete_ltp(), stderr)
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "protocol-record-on-stderr" for item in result["errors"]))

    def test_ltp_lifecycle_on_stderr_is_error(self) -> None:
        for line in (
            "RUN LTP CASE stray",
            "========== START ltp stray ==========",
            "========== END ltp stray ==========",
        ):
            with self.subTest(line=line):
                result = validator.validate_official_output(complete_ltp(), line + "\n")
                self.assertEqual(result["status"], "ERROR", result)
                self.assertTrue(any(item["kind"] == "protocol-record-on-stderr" for item in result["errors"]))

    def test_libctest_summary_on_stderr_is_error(self) -> None:
        stderr = "libctest cases: 1 passed, 0 failed, 0 timed out\n"
        result = validator.validate_official_output(complete_ltp(), stderr)
        self.assertEqual(result["status"], "ERROR", result)
        self.assertTrue(any(item["kind"] == "protocol-record-on-stderr" for item in result["errors"]))

    def test_polluted_group_markers_are_malformed(self) -> None:
        text = (
            "NOT #### OS COMP TEST GROUP START demo-musl ####\n"
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
            "NOT #### OS COMP TEST GROUP END demo-musl ####\n"
        )
        self.assert_status(text, "ERROR")

    def test_extra_prefixed_generic_result_is_malformed(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n"
            "NOT PASS OFFICIAL TEST GROUP demo-musl : 0",
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "malformed-protocol-record" for item in result["errors"]))

    def test_extra_prefixed_ltp_result_is_malformed(self) -> None:
        text = complete_ltp().replace(
            "#### OS COMP TEST GROUP END ltp-musl ####",
            "NOT FAIL LTP CASE stray : 0\n#### OS COMP TEST GROUP END ltp-musl ####",
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "malformed-protocol-record" for item in result["errors"]))

    def test_extra_prefixed_busybox_result_is_malformed(self) -> None:
        text = group(
            "busybox-musl",
            f"{busybox_frame(1, 'echo ok')}\n"
            "NOT BUSYBOX CASE RESULT ordinal=2 status=success command=false",
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "malformed-protocol-record" for item in result["errors"]))

    def test_extra_prefixed_libctest_result_is_malformed(self) -> None:
        text = group(
            "libctest-musl",
            "========== START entry-static.exe abs ==========\n"
            "Pass!\n"
            "========== END entry-static.exe abs ==========\n"
            "libctest cases: 1 passed, 0 failed, 0 timed out\n"
            "NOT FAIL libctest bad.exe abs: 0",
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "malformed-protocol-record" for item in result["errors"]))

    def test_extra_prefixed_group_start_is_malformed(self) -> None:
        text = (
            group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
            + "NOT #### OS COMP TEST GROUP START stray-musl ####\n"
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "malformed-protocol-record" for item in result["errors"]))

    def test_extra_prefixed_ltp_summary_is_malformed(self) -> None:
        text = complete_ltp().replace(
            "#### OS COMP TEST GROUP END ltp-musl ####",
            "NOT ltp cases: 99 passed, 0 failed, 0 timed out\n"
            "#### OS COMP TEST GROUP END ltp-musl ####",
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "malformed-protocol-record" for item in result["errors"]))

    def test_extra_prefixed_ltp_lifecycle_is_malformed(self) -> None:
        text = complete_ltp().replace(
            "#### OS COMP TEST GROUP END ltp-musl ####",
            "NOT ========== START ltp stray ==========\n"
            "#### OS COMP TEST GROUP END ltp-musl ####",
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "malformed-protocol-record" for item in result["errors"]))

    def test_global_forbidden_statuses_cannot_pass(self) -> None:
        for marker in (
            "SKIP benchmark x",
            "SKIPPED case x",
            "XFAIL case x",
            "HANG detected",
            "CRASH detected",
            "PANIC detected",
            "1 ignored",
            "test smoke ignored",
            "NOT_APPLICABLE",
            "benchmark not selected",
        ):
            with self.subTest(marker=marker):
                text = group(
                    "demo-musl",
                    f"PASS OFFICIAL TEST GROUP demo-musl : 0\n{marker}",
                )
                self.assert_status(text, "FAIL")

    def test_lowercase_test_statuses_cannot_pass(self) -> None:
        for marker in (
            "tconf: unsupported",
            "tbrok: setup failed",
            "tfail: mismatch",
            "enosys",
            "timeout",
        ):
            with self.subTest(marker=marker):
                self.assert_status(
                    group(
                        "demo-musl",
                        f"PASS OFFICIAL TEST GROUP demo-musl : 0\n{marker}",
                    ),
                    "FAIL",
                )

    def test_unknown_or_unexecuted_states_cannot_pass(self) -> None:
        for marker in (
            "STATUS: NOT_RUN",
            "STATUS: UNKNOWN",
            "RESULT: MAYBE",
            "UNRESOLVED",
            "UNSUPPORTED",
            "NOT RUN",
            "INCONCLUSIVE",
        ):
            with self.subTest(marker=marker):
                result = self.assert_status(
                    group(
                        "demo-musl",
                        f"PASS OFFICIAL TEST GROUP demo-musl : 0\n{marker}",
                    ),
                    "ERROR",
                )
                self.assertTrue(
                    any(item["kind"] == "unknown-status" for item in result["errors"]),
                    result,
                )

    def test_crash_and_trap_variants_cannot_pass(self) -> None:
        for marker in (
            "process crashed",
            "Unknown trap 0x1",
            "Unhandled user trap IllegalInstruction",
            "panic: fatal",
            "Segmentation fault (core dumped)",
            "Illegal instruction",
            "Bus error",
        ):
            with self.subTest(marker=marker):
                self.assert_status(
                    group(
                        "demo-musl",
                        f"PASS OFFICIAL TEST GROUP demo-musl : 0\n{marker}",
                    ),
                    "FAIL",
                )

    def test_generic_subtest_end_fail_markers_cannot_pass(self) -> None:
        for marker in (
            "====== iperf BASIC_TCP end: fail ======",
            "====== netperf TCP_STREAM end: fail ======",
            "====== cyclictest cyclictest end: fail ======",
            "====== kill hackbench: fail, ignore STRESS result ======",
        ):
            with self.subTest(marker=marker):
                for body in (
                    f"{marker}\nPASS OFFICIAL TEST GROUP demo-musl : 0",
                    f"PASS OFFICIAL TEST GROUP demo-musl : 0\n{marker}",
                ):
                    self.assert_status(group("demo-musl", body), "FAIL")

    def test_residual_escape_control_cannot_obscure_failure(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\nF\x1bXAIL",
        )
        self.assert_status(text, "ERROR")

    def test_cli_requires_the_canonical_official_plan(self) -> None:
        with tempfile.TemporaryDirectory(prefix="official-validator-cli-") as directory:
            stdout_path = Path(directory) / "capture.stdout.log"
            stdout_path.write_text(
                group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0"),
                encoding="utf-8",
            )
            stderr_path = Path(directory) / "capture.stderr.log"
            stderr_path.write_text("", encoding="utf-8")
            result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "parse_official_results.py"),
                    "--stdout",
                    str(stdout_path),
                ],
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )
            paired_result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "parse_official_results.py"),
                    "--stdout",
                    str(stdout_path),
                    "--stderr",
                    str(stderr_path),
                    "--process-exit-code",
                    "0",
                    "--json",
                ],
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )
            nonzero_result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).parents[1] / "evaluation" / "parse_official_results.py"),
                    "--stdout",
                    str(stdout_path),
                    "--stderr",
                    str(stderr_path),
                    "--process-exit-code",
                    "7",
                    "--json",
                ],
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("--stderr", result.stderr)
        self.assertEqual(paired_result.returncode, 2, paired_result.stdout + paired_result.stderr)
        self.assertNotIn("the following arguments are required", paired_result.stderr)
        paired_data = json.loads(paired_result.stdout)
        evidence = paired_data["input_evidence"]
        self.assertEqual(evidence["process_exit_code"], 0)
        self.assertEqual(
            evidence["stdout_sha256"],
            hashlib.sha256(
                group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0").encode(
                    "utf-8"
                )
            ).hexdigest(),
        )
        self.assertEqual(nonzero_result.returncode, 2, nonzero_result.stderr)
        nonzero_data = json.loads(nonzero_result.stdout)
        self.assertIn(
            "evaluator-process-nonzero",
            {finding["kind"] for finding in nonzero_data["failures"]},
        )

    def test_tracked_specialized_identity_plan_loads_exact_counts(self) -> None:
        busybox_cases, libctest_cases = validator.trusted_official_case_plan(
            Path(__file__).resolve().parents[2]
        )
        self.assertEqual(len(busybox_cases), 55)
        self.assertEqual([case.ordinal for case in busybox_cases], list(range(1, 56)))
        self.assertEqual(len(set(busybox_cases)), 55)
        self.assertEqual(len({case.command for case in busybox_cases}), 54)
        self.assertEqual(busybox_cases[36].command, busybox_cases[40].command)
        self.assertEqual(len(libctest_cases), 217)
        self.assertEqual(len(set(libctest_cases)), 217)

    def test_specialized_identity_plan_rejects_duplicate_json_keys(self) -> None:
        with tempfile.TemporaryDirectory(prefix="official-case-plan-") as directory:
            root = Path(directory)
            target = root / validator.OFFICIAL_CASE_PLAN_RELATIVE_PATH
            target.parent.mkdir(parents=True)
            source = (
                Path(__file__).resolve().parents[1]
                / "evaluation"
                / "official_case_plan.json"
            ).read_text(encoding="utf-8")
            target.write_text(
                source.replace(
                    '"schema_version": 2,',
                    '"schema_version": 2,\n  "schema_version": 2,',
                    1,
                ),
                encoding="utf-8",
            )
            with self.assertRaisesRegex(ValueError, "duplicate JSON key"):
                validator.trusted_official_case_plan(root)

    def test_specialized_identity_plan_rejects_duplicate_explicit_ids(self) -> None:
        source = json.loads(
            (
                Path(__file__).resolve().parents[1]
                / "evaluation"
                / "official_case_plan.json"
            ).read_text(encoding="utf-8")
        )
        source["busybox_cases"][0]["id"] = "shared-step"
        source["busybox_cases"][1]["id"] = "shared-step"
        with tempfile.TemporaryDirectory(prefix="official-case-plan-") as directory:
            root = Path(directory)
            target = root / validator.OFFICIAL_CASE_PLAN_RELATIVE_PATH
            target.parent.mkdir(parents=True)
            target.write_text(json.dumps(source), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "duplicate BusyBox explicit ID"):
                validator.trusted_official_case_plan(root)

    def test_unconsumed_explicit_fail_line_cannot_pass(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\nFAIL smoke test",
        )
        self.assert_status(text, "FAIL")

    def test_lowercase_unconsumed_failed_line_cannot_pass(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\nfailed smoke test",
        )
        self.assert_status(text, "FAIL")

    def test_colon_delimited_fail_line_cannot_pass(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\nsmoke test: FAIL",
        )
        self.assert_status(text, "FAIL")

    def test_bracketed_fail_line_cannot_pass(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\n[FAIL] smoke test",
        )
        self.assert_status(text, "FAIL")

    def test_result_equals_failed_line_cannot_pass(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\nresult = failed",
        )
        self.assert_status(text, "FAIL")

    def test_group_result_outside_lifecycle_is_error(self) -> None:
        text = (
            "PASS OFFICIAL TEST GROUP stray-musl : 0\n"
            + group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0")
        )
        result = self.assert_status(text, "ERROR")
        self.assertTrue(any(item["kind"] == "result-outside-group" for item in result["errors"]))

    def test_timeout_marker_cannot_pass(self) -> None:
        text = group(
            "demo-musl",
            "PASS OFFICIAL TEST GROUP demo-musl : 0\nTIMEOUT OFFICIAL TEST GROUP demo-musl after 3s",
        )
        self.assert_status(text, "FAIL")

    def test_panic_marker_cannot_pass(self) -> None:
        text = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0\nkernel panic: fatal")
        self.assert_status(text, "FAIL")

    def test_missing_runtime_artifact_is_error(self) -> None:
        text = group("demo-musl", "PASS OFFICIAL TEST GROUP demo-musl : 0\nmissing libctest entry: /x")
        self.assert_status(text, "ERROR")


if __name__ == "__main__":
    unittest.main()
