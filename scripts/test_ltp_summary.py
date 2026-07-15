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
            raw_summary = ltp_summary.parse_log_bytes(log.encode())
            data = ltp_summary.compact(raw_summary, arch=arch)
            rows.extend(ltp_summary.promotion_rows(raw_summary, data, arch))
        return ltp_summary.promotion_report(
            rows,
            required_arches={"rv", "la"},
            required_libcs={"musl", "glibc"},
        )

    def test_markdown_escape_neutralizes_structure_and_html(self) -> None:
        escaped = ltp_summary.markdown_escape("`case|<script>`\nnext")
        self.assertEqual(escaped, "\\`case\\|&lt;script&gt;\\` next")

    def test_stable_path_labels_hide_parents_and_disambiguate_basenames(self) -> None:
        labels = ltp_summary.stable_path_labels(
            [Path("/home/runner/one/evaluator.log"), Path("/root/two/evaluator.log")]
        )
        self.assertEqual(
            labels,
            ["input-1:evaluator.log", "input-2:evaluator.log"],
        )
        self.assertNotIn("/home", " ".join(labels))
        self.assertNotIn("/root", " ".join(labels))

        collision_labels = ltp_summary.stable_path_labels(
            [
                Path("/one/evaluator.log"),
                Path("/two/evaluator.log"),
                Path("/three/evaluator.log#1"),
            ]
        )
        self.assertEqual(len(collision_labels), len(set(collision_labels)))
        self.assertEqual(
            collision_labels,
            [
                "input-1:evaluator.log",
                "input-2:evaluator.log",
                "input-3:evaluator.log#1",
            ],
        )

    def two_libc_pass_log(self, case: str) -> str:
        return "\n".join(
            [
                "#### OS COMP TEST GROUP START ltp-musl ####",
                "ltp case list: inline (1 cases, timeout 30s)",
                f"RUN LTP CASE {case}",
                f"FAIL LTP CASE {case} : 0",
                "ltp cases: 1 passed, 0 failed, 0 timed out",
                "#### OS COMP TEST GROUP END ltp-musl ####",
                "#### OS COMP TEST GROUP START ltp-glibc ####",
                "ltp case list: inline (1 cases, timeout 30s)",
                f"RUN LTP CASE {case}",
                f"FAIL LTP CASE {case} : 0",
                "ltp cases: 1 passed, 0 failed, 0 timed out",
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

        self.assertEqual(ltp_summary.promotion_mode_blocker(None), "missing-case-list")

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

    def test_promotion_candidate_blocks_missing_arch_libc_combo(self) -> None:
        report = self.promotion_report(
            rv_log=self.two_libc_pass_log("rename01"),
            la_log="\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "ltp case list: inline (1 cases, timeout 30s)",
                    "RUN LTP CASE rename01",
                    "PASS LTP CASE rename01 : 0",
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
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
                    "ltp case list: inline (1 cases, timeout 30s)",
                    "RUN LTP CASE readlinkat02",
                    "PASS LTP CASE readlinkat02 : 0",
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "ltp case list: inline (1 cases, timeout 30s)",
                    "RUN LTP CASE readlinkat02",
                    "readlinkat02 1 TCONF : setup skipped part of the test",
                    "PASS LTP CASE readlinkat02 : 0",
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
                ]
            ),
        )

        self.assertEqual(report["candidate_count"], 0)
        self.assertEqual(report["blocked_count"], 1)
        blocker = report["blocked"][0]["blockers"][0]
        self.assertEqual(blocker["arch"], "la")
        self.assertEqual(blocker["libc"], "glibc")
        self.assertIn("TCONF=1", blocker["reasons"])

    def test_tconf_problem_marker_contract_remains_explicit(self) -> None:
        blocker = {
            "reasons": ltp_summary.row_problem_markers(
                {
                    "status": "PASS",
                    "internal": {"TCONF": 1},
                    "timeouts": 0,
                    "enosys": 0,
                    "panic_trap": 0,
                }
            )
        }
        self.assertEqual(blocker["reasons"], ["TCONF=1"])

    def test_promotion_candidate_blocks_blacklist_selection_mode(self) -> None:
        def blacklist_log(case: str) -> str:
            return "\n".join(
                [
                    "#### OS COMP TEST GROUP START ltp-musl ####",
                    "ltp case list: stable-plus-all-minus-blacklist stable=1000 extra=2 deduped=1000 skipped=3 (1 cases, timeout 30s)",
                    f"RUN LTP CASE {case}",
                    f"PASS LTP CASE {case} : 0",
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "ltp case list: stable-plus-all-minus-blacklist stable=1000 extra=2 deduped=1000 skipped=3 (1 cases, timeout 30s)",
                    f"RUN LTP CASE {case}",
                    f"PASS LTP CASE {case} : 0",
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
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
                    "ltp case list: inline (1 cases, timeout 30s)",
                    "RUN LTP CASE fchmod02",
                    "FAIL LTP CASE fchmod02 : 5",
                    "PASS LTP CASE fchmod02 : 0",
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
                    "#### OS COMP TEST GROUP END ltp-musl ####",
                    "#### OS COMP TEST GROUP START ltp-glibc ####",
                    "ltp case list: inline (1 cases, timeout 30s)",
                    "RUN LTP CASE fchmod02",
                    "PASS LTP CASE fchmod02 : 0",
                    "ltp cases: 1 passed, 0 failed, 0 timed out",
                    "#### OS COMP TEST GROUP END ltp-glibc ####",
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

    def test_problem_marker_contract_for_prior_failure_event(self) -> None:
        row = {
            "status": "PASS",
            "internal": {},
            "timeouts": 0,
            "enosys": 0,
            "panic_trap": 0,
            "event_failures": 1,
        }
        reasons = ltp_summary.row_problem_markers(row)
        self.assertEqual(reasons, ["event-failures=1"])

    def test_promotion_rejects_empty_axes_and_duplicate_combo(self) -> None:
        with self.assertRaises(ValueError):
            ltp_summary.promotion_report([], set(), {"musl"})

        raw = ltp_summary.parse_log_bytes(self.two_libc_pass_log("dup01").encode())
        data = ltp_summary.compact(raw, arch="rv")
        rows = ltp_summary.promotion_rows(raw, data, "rv")
        report = ltp_summary.promotion_report(
            rows + [dict(rows[0])], {"rv"}, {"musl", "glibc"}
        )
        self.assertEqual(report["candidate_count"], 0)
        self.assertIn(
            "duplicate-combo=2",
            report["blocked"][0]["blockers"][0]["reasons"],
        )

    def test_cli_empty_or_malformed_log_is_error(self) -> None:
        for payload in (b"", b"RUN LTP CASE access01\n", b"FAIL LTP CASE access01 : \xff0\n"):
            with self.subTest(payload=payload), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp) / "rv.log"
                path.write_bytes(payload)
                result = subprocess.run(
                    [sys.executable, str(Path(__file__).with_name("ltp_summary.py")), "--json", str(path)],
                    check=False,
                    capture_output=True,
                    text=True,
                )
                self.assertEqual(result.returncode, 2, result)

    def test_cli_oversized_log_and_numeric_field_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            oversized = Path(tmp) / "oversized.log"
            with oversized.open("wb") as stream:
                stream.truncate(ltp_summary.MAX_EVALUATOR_LOG_BYTES + 1)
            huge_numeric = Path(tmp) / "huge-numeric.log"
            huge_numeric.write_bytes(
                self.two_libc_pass_log("access01")
                .replace("FAIL LTP CASE access01 : 0", "FAIL LTP CASE access01 : " + "9" * 5000, 1)
                .encode()
            )
            for path in (oversized, huge_numeric):
                with self.subTest(path=path):
                    result = subprocess.run(
                        [sys.executable, str(Path(__file__).with_name("ltp_summary.py")), "--require-clean", "--json", str(path)],
                        check=False,
                        capture_output=True,
                        text=True,
                    )
                    self.assertEqual(result.returncode, 2, result)
                    self.assertNotIn("Traceback", result.stderr)

    def test_cli_require_clean_distinguishes_valid_failure(self) -> None:
        failed = self.two_libc_pass_log("access01").replace(
            "FAIL LTP CASE access01 : 0\nltp cases: 1 passed, 0 failed",
            "FAIL LTP CASE access01 : 5\nltp cases: 0 passed, 1 failed",
            1,
        )
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "rv.log"
            path.write_text(failed)
            report_only = subprocess.run(
                [sys.executable, str(Path(__file__).with_name("ltp_summary.py")), "--json", str(path)],
                check=False,
                capture_output=True,
                text=True,
            )
            required = subprocess.run(
                [sys.executable, str(Path(__file__).with_name("ltp_summary.py")), "--require-clean", "--json", str(path)],
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(report_only.returncode, 0, report_only.stderr)
        self.assertEqual(required.returncode, 1, required.stderr)

    def test_zero_case_ltp_is_integrity_error_in_normal_and_promotion_modes(self) -> None:
        empty = "\n".join(
            (
                "#### OS COMP TEST GROUP START ltp-musl ####",
                "ltp case list: empty (0 cases, timeout 30s)",
                "ltp cases: 0 passed, 0 failed, 0 timed out",
                "#### OS COMP TEST GROUP END ltp-musl ####",
                "",
            )
        )
        with tempfile.TemporaryDirectory() as tmp:
            rv_log = Path(tmp) / "rv.log"
            la_log = Path(tmp) / "la.log"
            rv_log.write_text(empty)
            la_log.write_text(empty)
            normal = subprocess.run(
                [sys.executable, str(Path(__file__).with_name("ltp_summary.py")), "--require-clean", "--json", str(rv_log)],
                check=False,
                capture_output=True,
                text=True,
            )
            promotion = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).with_name("ltp_summary.py")),
                    "--promotion-candidates",
                    "--require-clean",
                    "--json",
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(normal.returncode, 2, normal)
        self.assertEqual(promotion.returncode, 2, promotion)
        self.assertGreater(json.loads(promotion.stdout)["blocked_count"], 0)

    def test_promotion_require_clean_blocks_missing_required_architecture(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            rv_log = Path(tmp) / "rv.log"
            rv_log.write_text(self.two_libc_pass_log("chmod06"))
            result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).with_name("ltp_summary.py")),
                    "--promotion-candidates",
                    "--require-clean",
                    "--json",
                    str(rv_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(result.returncode, 1, result)
        self.assertEqual(json.loads(result.stdout)["blocked_count"], 1)

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
            la_log.write_text(
                "runtime architecture fixture: la\n" + self.two_libc_pass_log("chmod06")
            )

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

    def test_markdown_reports_do_not_embed_absolute_input_paths(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rv_log = root / "rv.log"
            la_log = root / "la.log"
            rv_log.write_text(self.two_libc_pass_log("chmod06"))
            la_log.write_text(
                "runtime architecture fixture: la\n"
                + self.two_libc_pass_log("chmod06")
            )
            normal = subprocess.run(
                [sys.executable, str(Path(__file__).with_name("ltp_summary.py")), str(rv_log)],
                check=False,
                capture_output=True,
                text=True,
            )
            promotion = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).with_name("ltp_summary.py")),
                    "--promotion-candidates",
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(normal.returncode, 0, normal)
            self.assertEqual(promotion.returncode, 0, promotion)
            for rendered in (normal.stdout, promotion.stdout):
                self.assertNotIn(root.as_posix(), rendered)
                self.assertNotIn("/root/", rendered)
            self.assertIn("# LTP summary: rv.log", normal.stdout)
            self.assertIn("- Inputs: rv.log, la.log", promotion.stdout)

    def test_promotion_rejects_identical_raw_log_bound_to_two_architectures(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            rv_log = Path(tmp) / "rv.log"
            la_log = Path(tmp) / "la.log"
            payload = self.two_libc_pass_log("chmod06")
            rv_log.write_text(payload)
            la_log.write_text(payload)
            result = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).with_name("ltp_summary.py")),
                    "--promotion-candidates",
                    "--require-clean",
                    "--json",
                    str(rv_log),
                    str(la_log),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
        self.assertEqual(result.returncode, 2, result)
        data = json.loads(result.stdout)
        self.assertEqual(data["candidate_count"], 0)
        self.assertTrue(
            any(item["case"] == "<architecture-provenance>" for item in data["blocked"])
        )
        self.assertNotIn(Path(tmp).as_posix(), result.stdout)
        self.assertIn("rv.log", result.stdout)
        self.assertIn("la.log", result.stdout)


if __name__ == "__main__":
    unittest.main()
