#!/usr/bin/env python3
"""Deterministic tests for the PR3 semantic evidence pipeline."""

from __future__ import annotations

import copy
import hashlib
import json
import os
import signal
import subprocess
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path
from unittest import mock

EVIDENCE_DIR = Path(__file__).resolve().parents[1] / "evidence"
sys.path.insert(0, str(EVIDENCE_DIR))

import render_semantic_evidence as render
import semantic_evidence as evidence


SCRIPT_DIR = EVIDENCE_DIR
REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = SCRIPT_DIR / "semantic_evidence_manifest.json"
FIXTURES = Path(__file__).resolve().parents[1] / "fixtures" / "semantic_evidence"


def process_result(**overrides: object) -> dict[str, object]:
    result: dict[str, object] = {
        "spawned": True,
        "exit_code": 0,
        "signal": None,
        "timed_out": False,
        "residual_processes_killed": False,
        "cleanup_complete": True,
        "cleanup_diagnostics": [],
        "term_sent": False,
        "kill_sent": False,
        "reaped": True,
        "spawn_error": None,
    }
    result.update(overrides)
    return result


def result_case(
    case_id: str,
    state: str,
    *,
    policy: str = "required",
    target: str = "runtime_semantic",
    observed: str | None = None,
    reason: str = "fixture reason <script>|line",
    log_ref: dict[str, object],
) -> dict[str, object]:
    if observed is None and state == "pass":
        observed = target
    if state in {"blocked", "skipped"}:
        process = process_result(spawned=False, exit_code=None, reaped=False)
    elif state == "timeout":
        process = process_result(
            exit_code=-9,
            signal=9,
            timed_out=True,
            term_sent=True,
            kill_sent=True,
        )
    elif state in {"fail", "error"}:
        process = process_result(exit_code=1)
    else:
        process = process_result()
    return {
        "case_id": case_id,
        "title": f"Fixture {case_id}",
        "category": "fixture",
        "architecture": "host",
        "policy": policy,
        "target_evidence": target,
        "observed_evidence": observed,
        "state": state,
        "reason_code": f"fixture_{state}",
        "reason": reason,
        "command": ["python3", "-c", "pass"],
        "cwd": ".",
        "started_at": "2026-01-01T00:00:00.000Z",
        "ended_at": "2026-01-01T00:00:01.000Z",
        "duration_seconds": 1.0,
        "process": process,
        "artifacts": [],
        "logs": {"stdout": None, "stderr": None, "raw": log_ref},
        "provenance": ["test/unit/test_semantic_evidence.py"],
    }


def result_document(bundle: Path, cases: list[dict[str, object]]) -> dict[str, object]:
    cases.sort(key=lambda item: (str(item["case_id"]), 0))
    expected_instances = sorted(
        f"{item['case_id']}@{item['architecture']}" for item in cases
    )
    document: dict[str, object] = {
        "schema_version": 1,
        "manifest": {
            "schema_version": 1,
            "suite_id": "fixture-suite",
            "sha256": "a" * 64,
            "path": "test/evidence/semantic_evidence_manifest.json",
        },
        "repository": {
            "revision": "b" * 40,
            "dirty": False,
            "content_sha256": "c" * 64,
        },
        "tools": {},
        "run": {
            "started_at": "2026-01-01T00:00:00.000Z",
            "ended_at": "2026-01-01T00:00:01.000Z",
            "duration_seconds": 1.0,
            "repository_before": {
                "revision": "b" * 40,
                "dirty": False,
                "content_sha256": "c" * 64,
            },
            "selected_case_count": len(cases),
            "expected_instances": expected_instances,
            "selection": {
                "requested_instances": expected_instances,
                "included_dependency_instances": [],
                "full_instance_count": len(cases),
                "full_required_instance_count": sum(
                    item["policy"] == "required" for item in cases
                ),
                "complete_suite": True,
                "complete_required": True,
            },
        },
        "cases": cases,
    }
    document["summary"] = evidence._summarize_cases(cases)  # noqa: SLF001
    return document


def make_log_reference(bundle: Path, text: str = "fixture raw evidence\n") -> dict[str, object]:
    log = bundle / "logs" / "fixture.log"
    log.parent.mkdir(parents=True, exist_ok=True)
    data = text.encode("utf-8")
    log.write_bytes(data)
    return {
        "path": "logs/fixture.log",
        "size_bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
    }


class StrictJsonTests(unittest.TestCase):
    def test_rejects_duplicate_keys(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "duplicate.json"
            path.write_text('{"a": 1, "a": 2}', encoding="utf-8")
            with self.assertRaisesRegex(evidence.JsonInputError, "duplicate JSON key"):
                evidence.strict_json_load(path)

    def test_rejects_non_finite_numbers(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "nan.json"
            path.write_text('{"value": NaN}', encoding="utf-8")
            with self.assertRaisesRegex(evidence.JsonInputError, "non-finite"):
                evidence.strict_json_load(path)

    def test_rejects_invalid_utf8(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "bad.json"
            path.write_bytes(b"{\xff}")
            with self.assertRaisesRegex(evidence.JsonInputError, "not valid UTF-8"):
                evidence.strict_json_load(path)

    def test_rejects_oversized_deep_or_surrogate_json_without_traceback(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            oversized = root / "oversized.json"
            with oversized.open("wb") as stream:
                stream.truncate(evidence.MAX_JSON_INPUT_BYTES + 1)
            deep = root / "deep.json"
            deep.write_text("[" * 5000 + "]" * 5000)
            surrogate = root / "surrogate.json"
            surrogate.write_text('"\\ud800"')
            for path in (oversized, deep, surrogate):
                with self.subTest(path=path), self.assertRaises(evidence.JsonInputError):
                    evidence.strict_json_load(path)

    def test_rejects_fifo_without_blocking(self) -> None:
        if not hasattr(os, "mkfifo"):
            self.skipTest("FIFO creation is unavailable")
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "manifest.fifo"
            os.mkfifo(path)
            with self.assertRaisesRegex(evidence.JsonInputError, "not a regular file"):
                evidence.strict_json_load(path)


class ManifestTests(unittest.TestCase):
    def setUp(self) -> None:
        self.document = evidence.strict_json_load(MANIFEST_PATH)

    def test_repository_manifest_expands_complete_guard_inventory(self) -> None:
        manifest = evidence.validate_manifest(self.document, REPO_ROOT)
        self.assertEqual(len(manifest["expanded_cases"]), 14)
        ids = [case["id"] for case in manifest["expanded_cases"]]
        self.assertIn("infra.competition-workflow", ids)
        self.assertIn("infra.competition-workflow-tests", ids)
        self.assertIn("smoke.rv64.abi", ids)
        self.assertIn("smoke.la64.abi", ids)

    def test_unknown_field_fails_closed(self) -> None:
        document = copy.deepcopy(self.document)
        document["surprise"] = True
        with self.assertRaisesRegex(evidence.ManifestError, "unknown fields"):
            evidence.validate_manifest(document, REPO_ROOT)

    def test_bool_timeout_is_not_an_integer(self) -> None:
        document = copy.deepcopy(self.document)
        document["cases"][0]["timeout_seconds"] = True
        with self.assertRaisesRegex(evidence.ManifestError, "must be an integer"):
            evidence.validate_manifest(document, REPO_ROOT)

    def test_bool_schema_version_and_invalid_contract_values_are_rejected(self) -> None:
        mutations = (
            ("schema", lambda document: document.__setitem__("schema_version", True)),
            (
                "evidence",
                lambda document: document["cases"][0].__setitem__(
                    "evidence_level", "imaginary"
                ),
            ),
            (
                "timeout-low",
                lambda document: document["cases"][0].__setitem__("timeout_seconds", 0),
            ),
            (
                "timeout-high",
                lambda document: document["cases"][0].__setitem__(
                    "timeout_seconds", evidence.MAX_TIMEOUT_SECONDS + 1
                ),
            ),
            (
                "dependency",
                lambda document: document["cases"][0].__setitem__(
                    "depends_on", ["missing.case"]
                ),
            ),
        )
        for name, mutate in mutations:
            with self.subTest(name=name):
                document = copy.deepcopy(self.document)
                mutate(document)
                with self.assertRaises(evidence.ManifestError):
                    evidence.validate_manifest(document, REPO_ROOT)

    def test_duplicate_case_id_is_rejected(self) -> None:
        document = copy.deepcopy(self.document)
        document["cases"][1]["id"] = document["cases"][0]["id"]
        with self.assertRaisesRegex(evidence.ManifestError, "must be sorted|duplicate"):
            evidence.validate_manifest(document, REPO_ROOT)

    def test_bad_architecture_is_rejected(self) -> None:
        document = copy.deepcopy(self.document)
        document["cases"][0]["architectures"] = ["mips64"]
        with self.assertRaisesRegex(evidence.ManifestError, "architectures"):
            evidence.validate_manifest(document, REPO_ROOT)

    def test_exit_classifier_cannot_claim_runtime_evidence(self) -> None:
        document = copy.deepcopy(self.document)
        document["cases"][0]["evidence_level"] = "runtime_semantic"
        with self.assertRaisesRegex(evidence.ManifestError, "cannot establish"):
            evidence.validate_manifest(document, REPO_ROOT)

    def test_required_case_cannot_use_external_capability(self) -> None:
        document = copy.deepcopy(self.document)
        document["capabilities"][0]["external"] = True
        with self.assertRaisesRegex(evidence.ManifestError, "required but depends on external"):
            evidence.validate_manifest(document, REPO_ROOT)

    def test_required_version_is_exact_and_tool_only(self) -> None:
        expected = "QEMU emulator version 9.2.4"
        self.assertEqual(
            evidence.schema_document()["$defs"]["capability"]["dependentSchemas"],
            {
                "required_version": {
                    "properties": {"kind": {"const": "tool"}},
                },
            },
        )
        qemu_capabilities = {
            capability["id"]: capability
            for capability in self.document["capabilities"]
            if capability["id"] in {"qemu-la64", "qemu-rv64"}
        }
        self.assertEqual(set(qemu_capabilities), {"qemu-la64", "qemu-rv64"})
        self.assertTrue(
            all(item.get("required_version") == expected for item in qemu_capabilities.values())
        )

        wrong_kind = copy.deepcopy(self.document)
        executable = next(
            item for item in wrong_kind["capabilities"] if item["kind"] == "executable"
        )
        executable["required_version"] = expected
        with self.assertRaisesRegex(evidence.ManifestError, "only for tool"):
            evidence.validate_manifest(wrong_kind, REPO_ROOT)

        empty = copy.deepcopy(self.document)
        qemu = next(item for item in empty["capabilities"] if item["id"] == "qemu-rv64")
        qemu["required_version"] = ""
        with self.assertRaisesRegex(evidence.ManifestError, "non-empty string"):
            evidence.validate_manifest(empty, REPO_ROOT)

    def test_inventory_detects_deleted_complete_pair(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            scripts = root / "scripts"
            scripts.mkdir()
            for stem in ("beta_three",):
                (scripts / f"check_{stem}.py").write_text("", encoding="utf-8")
                (scripts / f"test_{stem}.py").write_text("", encoding="utf-8")
            manifest = {
                "schema_version": 1,
                "suite_id": "fixture",
                "capabilities": [
                    {"id": "python3", "kind": "tool", "value": "python3", "external": False}
                ],
                "runners": [
                    {
                        "id": "static",
                        "kind": "process",
                        "classifier_kind": "guard_protocol",
                        "max_evidence": "static_checked",
                        "combine_output": True,
                        "grace_seconds": 1.0,
                    }
                ],
                "inventories": [
                    {
                        "id": "guards",
                        "kind": "python_guard_pairs",
                        "check_glob": "scripts/check_*.py",
                        "test_glob": "scripts/test_*.py",
                        "runner_id": "static",
                        "category": "compliance",
                        "policy": "required",
                        "timeout_seconds": 10,
                        "requires": ["python3"],
                        "expected_ids": ["alpha", "beta"],
                    }
                ],
                "cases": [],
            }
            with self.assertRaisesRegex(evidence.ManifestError, "missing expected guard ids: alpha"):
                evidence.validate_manifest(manifest, root)

    def test_generated_schema_matches_checked_in_file(self) -> None:
        expected = evidence.canonical_json_bytes(evidence.schema_document())
        self.assertEqual(expected, (SCRIPT_DIR / "semantic_evidence_schema.v1.json").read_bytes())


class ClassifierTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        manifest, _ = evidence.load_and_validate_manifest(MANIFEST_PATH, REPO_ROOT)
        cls.smoke = next(
            case for case in manifest["expanded_cases"] if case["id"] == "smoke.rv64.abi"
        )
        # Keep protocol-classifier mutation coverage independent of whichever
        # repository guards are registered in the specialized PR3 collector.
        cls.guard_case = {
            "evidence_level": "static_checked",
            "timeout_seconds": 10,
            "classifier": {
                "kind": "guard_protocol",
                "pass_pattern": (
                    "^(?:PASS: fixture guard found no success-without-state "
                    "shells|fixture guard: PASS)$"
                ),
                "fail_pattern": "^fixture guard: FAIL$",
            },
        }
        cls.infrastructure = next(
            case
            for case in manifest["expanded_cases"]
            if case["id"] == "infra.semantic-evidence"
        )

    def classify(self, fixture: str, **process_overrides: object) -> dict[str, object]:
        return evidence.classify_process(
            case=self.smoke,
            arch="riscv64",
            process=process_result(**process_overrides),
            raw_log=FIXTURES / fixture,
        )

    def test_complete_runtime_protocol_passes(self) -> None:
        result = self.classify("smoke-rv64-pass.txt")
        self.assertEqual(result["state"], "pass")
        self.assertEqual(result["observed_evidence"], "runtime_semantic")

    def test_guard_prefix_pass_protocol_is_accepted_once(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            log = Path(temporary) / "guard.log"
            log.write_text(
                "PASS: fixture guard found no success-without-state shells\n",
                encoding="utf-8",
            )
            result = evidence.classify_process(
                case=self.guard_case,
                arch="host",
                process=process_result(),
                raw_log=log,
            )
            self.assertEqual(result["state"], "pass")

    def test_guard_mixed_pass_protocol_is_ambiguous(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            log = Path(temporary) / "guard.log"
            log.write_text(
                "PASS: fixture guard found no success-without-state shells\n"
                "fixture guard: PASS\n",
                encoding="utf-8",
            )
            result = evidence.classify_process(
                case=self.guard_case,
                arch="host",
                process=process_result(),
                raw_log=log,
            )
            self.assertEqual(result["state"], "error")
            self.assertEqual(result["reason_code"], "ambiguous_guard_protocol")

    def test_guard_duplicate_prefix_pass_protocol_is_ambiguous(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            log = Path(temporary) / "guard.log"
            line = "PASS: fixture guard found no success-without-state shells\n"
            log.write_text(line + line, encoding="utf-8")
            result = evidence.classify_process(
                case=self.guard_case,
                arch="host",
                process=process_result(),
                raw_log=log,
            )
            self.assertEqual(result["state"], "error")

    def test_guard_pass_and_fail_protocol_is_ambiguous(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            log = Path(temporary) / "guard.log"
            log.write_text(
                "PASS: fixture guard found no success-without-state shells\n"
                "fixture guard: FAIL\n",
                encoding="utf-8",
            )
            result = evidence.classify_process(
                case=self.guard_case,
                arch="host",
                process=process_result(),
                raw_log=log,
            )
            self.assertEqual(result["state"], "error")

    def test_guard_protocol_rejects_non_ascii_text(self) -> None:
        pass_line = b"PASS: fixture guard found no success-without-state shells\n"
        for hidden_fail in (b"F\xc2\xadAIL hidden\n", b"F\xef\xbb\xbfAIL hidden\n"):
            with self.subTest(hidden_fail=hidden_fail), tempfile.TemporaryDirectory() as temporary:
                log = Path(temporary) / "guard.log"
                log.write_bytes(pass_line + hidden_fail)
                result = evidence.classify_process(
                    case=self.guard_case,
                    arch="host",
                    process=process_result(),
                    raw_log=log,
                )
                self.assertEqual(result["state"], "error", result)
                self.assertEqual(result["reason_code"], "malformed_log", result)

    def test_display_ignorable_bytes_cannot_hide_guard_failure(self) -> None:
        pass_line = b"PASS: fixture guard found no success-without-state shells\n"
        for hidden_fail in (b"fixture guard: F\0AIL\n", b"fixture guard: F\aAIL\n"):
            with self.subTest(hidden_fail=hidden_fail), tempfile.TemporaryDirectory() as temporary:
                log = Path(temporary) / "guard.log"
                log.write_bytes(pass_line + hidden_fail)
                result = evidence.classify_process(
                    case=self.guard_case,
                    arch="host",
                    process=process_result(),
                    raw_log=log,
                )
                self.assertEqual(result["state"], "error", result)
                self.assertEqual(result["reason_code"], "ambiguous_guard_protocol", result)

    def test_unittest_zero_tests_cannot_pass(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            log = Path(temporary) / "unittest.log"
            log.write_text("Ran 0 tests in 0.000s\n\nOK\n", encoding="utf-8")
            result = evidence.classify_process(
                case=self.infrastructure,
                arch="host",
                process=process_result(),
                raw_log=log,
            )
            self.assertEqual(result["state"], "error")
            self.assertEqual(result["reason_code"], "ambiguous_guard_protocol")

    def test_duplicate_marker_is_error(self) -> None:
        self.assertEqual(self.classify("smoke-rv64-duplicate.txt")["state"], "error")

    def test_truncated_log_is_error(self) -> None:
        result = self.classify("smoke-rv64-truncated.txt")
        self.assertEqual(result["reason_code"], "marker_count")

    def test_panic_after_user_pass_is_error(self) -> None:
        result = self.classify("smoke-rv64-panic-after-pass.txt")
        self.assertEqual(result["reason_code"], "fatal_runtime_signal")

    def test_ansi_colored_panic_is_error(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "colored-panic.log"
            path.write_text(
                "\x1b[31mpanicked at kernel/net/axnet/src/lib.rs:44\x1b[0m\n",
                encoding="utf-8",
            )
            result = evidence.classify_process(
                case=self.smoke,
                arch="riscv64",
                process=process_result(),
                raw_log=path,
            )
            self.assertEqual(result["reason_code"], "fatal_runtime_signal")

    def test_terminal_sequences_cannot_split_or_hide_runtime_fatals(self) -> None:
        base = (FIXTURES / "smoke-rv64-pass.txt").read_bytes()
        suffixes = (
            b"T\x1b[31mFAIL after markers\n",
            b"unexpected trap after markers\n",
            b"operation not implemented after markers\n",
            b"pa\0nic after markers\n",
            b"T\x07FAIL after markers\n",
        )
        for suffix in suffixes:
            with self.subTest(suffix=suffix), tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "fatal.log"
                path.write_bytes(base + suffix)
                result = evidence.classify_process(
                    case=self.smoke,
                    arch="riscv64",
                    process=process_result(),
                    raw_log=path,
                )
                self.assertEqual(result["reason_code"], "fatal_runtime_signal", result)

        for suffix in (
            b"T\x1b]title\x07FAIL after markers\n",
            b"pa\x1bPignored\x1b\\nic after markers\n",
            b"panX\x08ic after markers\n",
            b"panX\x1b[Dic after markers\n",
        ):
            with self.subTest(unsafe_control=suffix), tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "unsafe-control.log"
                path.write_bytes(base + suffix)
                result = evidence.classify_process(
                    case=self.smoke,
                    arch="riscv64",
                    process=process_result(),
                    raw_log=path,
                )
                self.assertEqual(result["reason_code"], "malformed_log", result)

    def test_runtime_protocol_rejects_non_ascii_text(self) -> None:
        base = (FIXTURES / "smoke-rv64-pass.txt").read_bytes()
        for hidden_panic in (b"pa\xc2\xadnic after markers\n", b"pa\xef\xbb\xbfnic after markers\n"):
            with self.subTest(hidden_panic=hidden_panic), tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "unicode-fatal.log"
                path.write_bytes(base + hidden_panic)
                result = evidence.classify_process(
                    case=self.smoke,
                    arch="riscv64",
                    process=process_result(),
                    raw_log=path,
                )
                self.assertEqual(result["state"], "error", result)
                self.assertEqual(result["reason_code"], "malformed_log", result)

    def test_incomplete_terminal_control_after_markers_is_error(self) -> None:
        suffixes = (b"T\x1b]0;hiddenFAIL", b"pa\x1bPignorednic", b"\x1b[31")
        for suffix in suffixes:
            with self.subTest(suffix=suffix), tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "malformed-control.log"
                path.write_bytes((FIXTURES / "smoke-rv64-pass.txt").read_bytes() + suffix)
                result = evidence.classify_process(
                    case=self.smoke,
                    arch="riscv64",
                    process=process_result(),
                    raw_log=path,
                )
                self.assertEqual(result["reason_code"], "malformed_log", result)

    def test_completed_payload_terminal_controls_cannot_hide_runtime_fatals(self) -> None:
        suffixes = (
            b"\x1b]0;kernel panic hidden in OSC\x07\n",
            b"\x1bPTFAIL hidden in DCS\x1b\\\n",
        )
        for suffix in suffixes:
            with self.subTest(suffix=suffix), tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "payload-control.log"
                path.write_bytes(
                    (FIXTURES / "smoke-rv64-pass.txt").read_bytes() + suffix
                )
                result = evidence.classify_process(
                    case=self.smoke,
                    arch="riscv64",
                    process=process_result(),
                    raw_log=path,
                )
                self.assertEqual(result["state"], "error", result)
                self.assertEqual(result["reason_code"], "malformed_log", result)

    def test_display_ignorable_bytes_cannot_reconstruct_runtime_markers(self) -> None:
        base = (FIXTURES / "smoke-rv64-pass.txt").read_bytes()
        for control in (b"\0", b"\a"):
            with self.subTest(control=control), tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "split-marker.log"
                path.write_bytes(base.replace(b"USER_PASS", b"USER_" + control + b"PASS"))
                result = evidence.classify_process(
                    case=self.smoke,
                    arch="riscv64",
                    process=process_result(),
                    raw_log=path,
                )
                self.assertNotEqual(result["state"], "pass", result)

        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "firmware-nul.log"
            path.write_bytes(b"OpenSBI |___\0/_____|\n" + base)
            result = evidence.classify_process(
                case=self.smoke,
                arch="riscv64",
                process=process_result(),
                raw_log=path,
            )
            self.assertEqual(result["state"], "pass", result)

    def test_timeout_wins_over_complete_pass_markers(self) -> None:
        result = self.classify("smoke-rv64-pass.txt", timed_out=True)
        self.assertEqual(result["state"], "timeout")

    def test_nonzero_exit_after_markers_is_failure(self) -> None:
        result = self.classify("smoke-rv64-pass.txt", exit_code=5)
        self.assertEqual(result["state"], "fail")

    def test_wrong_architecture_is_not_a_pass(self) -> None:
        result = evidence.classify_process(
            case=self.smoke,
            arch="loongarch64",
            process=process_result(),
            raw_log=FIXTURES / "smoke-rv64-pass.txt",
        )
        self.assertEqual(result["state"], "error")

    def test_empty_log_is_error(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "empty.log"
            path.write_bytes(b"")
            result = evidence.classify_process(
                case=self.smoke,
                arch="riscv64",
                process=process_result(),
                raw_log=path,
            )
            self.assertEqual(result["reason_code"], "empty_log")

    def test_invalid_utf8_log_is_error(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "invalid.log"
            path.write_bytes(b"\xff")
            result = evidence.classify_process(
                case=self.smoke,
                arch="riscv64",
                process=process_result(),
                raw_log=path,
            )
            self.assertEqual(result["reason_code"], "malformed_log")

    def test_oversized_log_is_error_without_loading_it(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "oversized.log"
            with path.open("wb") as stream:
                stream.truncate(evidence.MAX_CLASSIFIABLE_LOG_BYTES + 1)
            result = evidence.classify_process(
                case=self.smoke,
                arch="riscv64",
                process=process_result(),
                raw_log=path,
            )
            self.assertEqual(result["reason_code"], "malformed_log")
            self.assertIn("classification limit", result["reason"])

    def test_cleanup_residual_is_never_pass(self) -> None:
        result = self.classify("smoke-rv64-pass.txt", residual_processes_killed=True)
        self.assertEqual(result["reason_code"], "residual_process")

    def test_guard_requires_nonempty_unambiguous_protocol(self) -> None:
        case = {
            "evidence_level": "static_checked",
            "timeout_seconds": 10,
            "classifier": {
                "kind": "guard_protocol",
                "pass_pattern": r"^G999 .*: PASS(?: .*)?$",
                "fail_pattern": r": FAIL\b",
            },
        }
        passed = evidence.classify_process(
            case=case,
            arch="host",
            process=process_result(),
            raw_log=FIXTURES / "guard-pass.txt",
        )
        ambiguous = evidence.classify_process(
            case=case,
            arch="host",
            process=process_result(),
            raw_log=FIXTURES / "guard-ambiguous.txt",
        )
        self.assertEqual(passed["state"], "pass")
        self.assertEqual(ambiguous["state"], "error")

    def test_mutation_unittest_protocol_rejects_zero_or_ambiguous_tests(self) -> None:
        case = {
            "evidence_level": "static_checked",
            "timeout_seconds": 10,
            "classifier": {
                "kind": "guard_protocol",
                "pass_pattern": r"^OK$",
                "fail_pattern": r"(?:^FAILED \(|Traceback \(most recent call last\):)",
                "min_tests": 1,
            },
        }
        fixtures = {
            "zero": "Ran 0 tests in 0.000s\n\nOK\n",
            "missing-count": "OK\n",
            "duplicate-count": "Ran 1 test in 0.001s\nRan 1 test in 0.001s\n\nOK\n",
            "nonterminal-ok": "Ran 1 test in 0.001s\n\nOK\nextra\n",
        }
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name, content in fixtures.items():
                path = root / f"{name}.log"
                path.write_text(content, encoding="utf-8")
                result = evidence.classify_process(
                    case=case,
                    arch="host",
                    process=process_result(),
                    raw_log=path,
                )
                self.assertEqual(result["state"], "error", (name, result))

            valid = root / "valid.log"
            valid.write_text(".\n----------------------------------------------------------------------\nRan 1 test in 0.001s\n\nOK\n")
            result = evidence.classify_process(
                case=case,
                arch="host",
                process=process_result(),
                raw_log=valid,
            )
            self.assertEqual(result["state"], "pass", result)


class ProcessSupervisorTests(unittest.TestCase):
    def test_timeout_kills_term_ignoring_process_group(self) -> None:
        code = (
            "import signal,subprocess,sys,time;"
            "signal.signal(signal.SIGTERM, signal.SIG_IGN);"
            "subprocess.Popen([sys.executable,'-c',"
            "'import signal,time; signal.signal(signal.SIGTERM, signal.SIG_IGN); time.sleep(60)']);"
            "print('ready', flush=True);time.sleep(60)"
        )
        with tempfile.TemporaryDirectory() as temporary:
            result = evidence.run_process(
                command=[sys.executable, "-c", code],
                cwd=REPO_ROOT,
                environment=dict(os.environ),
                timeout_seconds=1,
                grace_seconds=0.1,
                combine_output=True,
                log_dir=Path(temporary),
                slug="timeout",
            )
            self.assertTrue(result["timed_out"])
            self.assertTrue(result["term_sent"])
            self.assertTrue(result["kill_sent"])
            self.assertTrue(result["reaped"])
            self.assertTrue(result["cleanup_complete"])

    def test_background_child_after_leader_exit_is_error_evidence(self) -> None:
        code = (
            "import subprocess,sys;"
            "subprocess.Popen([sys.executable,'-c','import time; time.sleep(60)']);"
            "print('leader exits', flush=True)"
        )
        with tempfile.TemporaryDirectory() as temporary:
            result = evidence.run_process(
                command=[sys.executable, "-c", code],
                cwd=REPO_ROOT,
                environment=dict(os.environ),
                timeout_seconds=5,
                grace_seconds=0.1,
                combine_output=True,
                log_dir=Path(temporary),
                slug="residual",
            )
            self.assertTrue(result["residual_processes_killed"])
            self.assertTrue(result["cleanup_complete"])

    def test_setsid_child_cannot_escape_supervisor(self) -> None:
        if not sys.platform.startswith("linux"):
            self.skipTest("Linux subreaper test")
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            pid_file = root / "detached.pid"
            child = (
                "import os,signal,time;"
                "os.setsid();"
                "signal.signal(signal.SIGTERM,signal.SIG_IGN);"
                "time.sleep(60)"
            )
            code = (
                "import subprocess,sys;"
                f"p=subprocess.Popen([sys.executable,'-c',{child!r}]);"
                f"open({str(pid_file)!r},'w').write(str(p.pid));"
                "print('leader exits',flush=True)"
            )
            result = evidence.run_process(
                command=[sys.executable, "-c", code],
                cwd=REPO_ROOT,
                environment=dict(os.environ),
                timeout_seconds=5,
                grace_seconds=0.1,
                combine_output=True,
                log_dir=root,
                slug="detached",
            )
            detached_pid = int(pid_file.read_text(encoding="ascii"))
            self.assertTrue(result["residual_processes_killed"])
            self.assertTrue(result["cleanup_complete"])
            self.assertFalse(Path(f"/proc/{detached_pid}").exists())


class ResultAndRendererTests(unittest.TestCase):
    def make_bundle(self, root: Path) -> tuple[Path, dict[str, object]]:
        log_ref = make_log_reference(root)
        cases = [
            result_case("a-pass", "pass", log_ref=log_ref),
            result_case("b-fail", "fail", log_ref=log_ref),
            result_case("c-error", "error", log_ref=log_ref),
            result_case("d-timeout", "timeout", log_ref=log_ref),
            result_case("e-blocked", "blocked", log_ref=log_ref),
            result_case("f-skipped", "skipped", policy="observational", log_ref=log_ref),
            result_case("g-observational-fail", "fail", policy="observational", log_ref=log_ref),
        ]
        document = result_document(root, cases)
        path = root / "semantic-evidence-v1.json"
        path.write_bytes(evidence.canonical_json_bytes(document))
        return path, document

    def test_result_checks_exact_case_coverage_and_summary(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, document = self.make_bundle(root)
            evidence.load_and_validate_result(path)
            broken = copy.deepcopy(document)
            broken["run"]["expected_instances"].append("missing@host")
            with self.assertRaisesRegex(evidence.ResultError, "exactly cover"):
                evidence.validate_result_document(broken, bundle_root=root)
            broken = copy.deepcopy(document)
            broken["summary"]["total"] += 1
            with self.assertRaisesRegex(evidence.ResultError, "summary"):
                evidence.validate_result_document(broken, bundle_root=root)
            broken = copy.deepcopy(document)
            broken["summary"]["total"] = True
            broken["summary"]["required_nonpass"] = True
            broken["summary"]["states"] = {
                key: bool(value) for key, value in broken["summary"]["states"].items()
            }
            broken["summary"]["policies"] = {
                key: bool(value) for key, value in broken["summary"]["policies"].items()
            }
            with self.assertRaisesRegex(evidence.ResultError, "non-negative integer"):
                evidence.validate_result_document(broken, bundle_root=root)

    def test_result_rejects_log_digest_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, _ = self.make_bundle(root)
            (root / "logs" / "fixture.log").write_text("tampered\n", encoding="utf-8")
            with self.assertRaisesRegex(evidence.ResultError, "size_bytes mismatch|sha256 mismatch"):
                evidence.load_and_validate_result(path)

    def test_result_rejects_evidence_overclaim(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            _, document = self.make_bundle(root)
            document["cases"][0]["target_evidence"] = "built"
            document["cases"][0]["observed_evidence"] = "runtime_semantic"
            with self.assertRaisesRegex(evidence.ResultError, "overclaims"):
                evidence.validate_result_document(document, bundle_root=root)

    def test_result_rejects_bool_version_and_invalid_state(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            _, document = self.make_bundle(root)
            for name, mutate in (
                ("version", lambda value: value.__setitem__("schema_version", True)),
                ("state", lambda value: value["cases"][0].__setitem__("state", "green")),
            ):
                with self.subTest(name=name):
                    broken = copy.deepcopy(document)
                    mutate(broken)
                    with self.assertRaises(evidence.ResultError):
                        evidence.validate_result_document(broken, bundle_root=root)

    def test_result_rejects_absolute_or_parent_tool_paths(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            _, document = self.make_bundle(root)
            for invalid_path in ("/home/developer/.rustup/bin/rustc", "../rustc", ""):
                with self.subTest(path=invalid_path):
                    broken = copy.deepcopy(document)
                    broken["tools"] = {
                        "rustc": {
                            "path": invalid_path,
                            "version": "rustc fixture",
                            "sha256": "d" * 64,
                        }
                    }
                    with self.assertRaisesRegex(
                        evidence.ResultError, "normalized relative logical name"
                    ):
                        evidence.validate_result_document(broken, bundle_root=root)

    def test_result_rejects_invalid_or_reversed_timestamps(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            _, document = self.make_bundle(root)
            invalid = copy.deepcopy(document)
            invalid["run"]["started_at"] = "9999-99-99T99:99:99.999Z"
            with self.assertRaisesRegex(evidence.ResultError, "valid millisecond"):
                evidence.validate_result_document(invalid, bundle_root=root)

            reversed_run = copy.deepcopy(document)
            reversed_run["run"]["ended_at"] = "2025-12-31T23:59:59.999Z"
            with self.assertRaisesRegex(evidence.ResultError, "must not precede"):
                evidence.validate_result_document(reversed_run, bundle_root=root)

            escaped_case = copy.deepcopy(document)
            escaped_case["cases"][0]["ended_at"] = "2026-01-01T00:00:02.000Z"
            with self.assertRaisesRegex(evidence.ResultError, "contained by result.run"):
                evidence.validate_result_document(escaped_case, bundle_root=root)

    def test_blocked_and_skipped_cannot_claim_observed_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            _, document = self.make_bundle(root)
            for case_index in (4, 5):
                broken = copy.deepcopy(document)
                broken["cases"][case_index]["observed_evidence"] = "runtime_semantic"
                with self.subTest(case=broken["cases"][case_index]["case_id"]):
                    with self.assertRaisesRegex(
                        evidence.ResultError, "blocked/skipped state cannot claim"
                    ):
                        evidence.validate_result_document(broken, bundle_root=root)

    def test_junit_counts_and_xml_escaping(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, _ = self.make_bundle(root)
            output = root / "rendered"
            render.render_all(path, output, allow_partial=True)
            xml_root = ET.parse(output / render.JUNIT_NAME).getroot()
            self.assertEqual(xml_root.attrib["tests"], "7")
            self.assertEqual(xml_root.attrib["failures"], "2")
            self.assertEqual(xml_root.attrib["errors"], "3")
            self.assertEqual(xml_root.attrib["skipped"], "1")

    def test_junit_sanitizes_control_characters_in_classname(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, document = self.make_bundle(root)
            document["cases"][0]["category"] = "fixture\u0001category"
            path.write_bytes(evidence.canonical_json_bytes(document))
            output = root / "rendered"
            render.render_all(path, output, allow_partial=True)
            parsed = ET.parse(output / render.JUNIT_NAME)
            classname = parsed.find(".//testcase").attrib["classname"]
            self.assertIn("\ufffd", classname)

    def test_partial_required_result_is_visibly_non_green(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, document = self.make_bundle(root)
            document["run"]["selection"]["complete_required"] = False
            path.write_bytes(evidence.canonical_json_bytes(document))
            output = root / "rendered"
            render.render_all(path, output, allow_partial=True)
            xml_root = ET.parse(output / render.JUNIT_NAME).getroot()
            self.assertEqual(xml_root.attrib["errors"], "4")
            self.assertIsNotNone(
                xml_root.find(".//testcase[@name='required-suite-completeness']/error")
            )
            self.assertIn(
                "INCOMPLETE / NOT A REQUIRED GATE",
                (output / render.HTML_NAME).read_text(encoding="utf-8"),
            )

    def test_html_log_link_resolves_to_the_canonical_bundle(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, _ = self.make_bundle(root)
            output = root / "nested" / "reports"
            render.render_all(path, output, allow_partial=True)
            text = (output / render.HTML_NAME).read_text(encoding="utf-8")
            expected = os.path.relpath(root / "logs" / "fixture.log", output).replace(
                os.sep, "/"
            )
            self.assertIn(f'href="{expected}"', text)

    def test_html_is_offline_and_escapes_untrusted_values(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, _ = self.make_bundle(root)
            output = root / "rendered"
            render.render_all(path, output, allow_partial=True)
            text = (output / render.HTML_NAME).read_text(encoding="utf-8")
            self.assertNotIn("<script>", text)
            self.assertIn("&lt;script&gt;", text)
            self.assertNotIn("cdn", text.lower())
            self.assertNotIn("http://", text.lower())
            self.assertNotIn("https://", text.lower())

    def test_markdown_escapes_pipe_and_html(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, _ = self.make_bundle(root)
            output = root / "rendered"
            render.render_all(path, output, allow_partial=True)
            text = (output / render.MATRIX_NAME).read_text(encoding="utf-8")
            self.assertIn("\\|", text)
            self.assertIn("&lt;script&gt;", text)

    def test_human_reports_bind_source_identity_and_resolve_raw_logs(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, document = self.make_bundle(root)
            document["repository"]["dirty"] = True
            document["repository"]["content_sha256"] = "d" * 64
            path.write_bytes(evidence.canonical_json_bytes(document))
            output = root / "nested" / "reports"
            render.render_all(path, output, allow_partial=True)
            expected = os.path.relpath(root / "logs" / "fixture.log", output).replace(
                os.sep, "/"
            )
            html_text = (output / render.HTML_NAME).read_text(encoding="utf-8")
            matrix_text = (output / render.MATRIX_NAME).read_text(encoding="utf-8")
            self.assertIn("<strong>Repository dirty:</strong> true", html_text)
            self.assertIn(f"<code>{'d' * 64}</code>", html_text)
            self.assertIn("- Repository dirty: true", matrix_text)
            self.assertIn(f"- Source content SHA-256: {'d' * 64}", matrix_text)
            self.assertIn(f"[raw log]({expected})", matrix_text)
            self.assertTrue((output / expected).resolve().is_file())

    def test_repeated_render_is_byte_identical(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, _ = self.make_bundle(root)
            first = root / "first"
            second = root / "second"
            render.render_all(path, first, allow_partial=True)
            render.render_all(path, second, allow_partial=True)
            for name in (render.JUNIT_NAME, render.HTML_NAME, render.MATRIX_NAME):
                self.assertEqual((first / name).read_bytes(), (second / name).read_bytes())

    def test_failed_rerender_removes_stale_owned_reports(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path, _ = self.make_bundle(root)
            output = root / "rendered"
            render.render_all(path, output, allow_partial=True)
            path.write_text("{}\n", encoding="utf-8")
            with self.assertRaises(evidence.ResultError):
                render.render_all(path, output, allow_partial=True)
            for name in (render.JUNIT_NAME, render.HTML_NAME, render.MATRIX_NAME):
                self.assertFalse((output / name).exists())


class PipelineIntegrationTests(unittest.TestCase):
    def test_required_environment_is_allowlisted_and_toolchain_pinned(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "rust-toolchain.toml").write_text(
                '[toolchain]\nchannel = "nightly-2025-05-20"\n', encoding="utf-8"
            )
            injected = {
                "ARCH": "attacker-arch",
                "CARGO_HOME": "/tmp/attacker-cargo-home",
                "MAKEFLAGS": "-e",
                "PR3_SMOKE_APP_FEATURES": "fake-pass",
                "RUSTC": "/tmp/attacker-rustc",
                "RUSTFLAGS": "--cfg fake_pass",
                "RUSTUP_TOOLCHAIN": "nightly",
                "RUST_LLD": "/tmp/attacker-linker",
                "SECRET_TOKEN": "must-not-reach-child",
            }
            with mock.patch.dict(os.environ, injected, clear=False):
                environment = evidence._baseline_environment(root)  # noqa: SLF001
            for name in injected:
                if name == "RUSTUP_TOOLCHAIN":
                    continue
                self.assertNotIn(name, environment)
            self.assertEqual(environment["RUSTUP_TOOLCHAIN"], "nightly-2025-05-20")
            self.assertEqual(environment["LC_ALL"], "C.UTF-8")
            self.assertEqual(environment["TZ"], "UTC")
            self.assertEqual(environment["PATH"], os.environ["PATH"])

    def test_rustup_proxy_identity_uses_dispatched_tool_binary(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            rustup = root / "rustup"
            rustc = root / "rustc"
            actual = root / "actual-rustc"
            rustup.write_text(
                "#!/bin/sh\n"
                "if [ \"${0##*/}\" = rustc ]; then\n"
                "  echo 'rustc 1.2.3-fixture'\n"
                "elif [ \"$1\" = which ] && [ \"$2\" = rustc ]; then\n"
                "  printf '%s\\n' \"$PR3_FIXTURE_ACTUAL_RUSTC\"\n"
                "else\n"
                "  exit 9\n"
                "fi\n",
                encoding="utf-8",
            )
            actual.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
            rustup.chmod(0o755)
            actual.chmod(0o755)
            rustc.symlink_to("rustup")
            environment = dict(os.environ)
            environment["PATH"] = root.as_posix()
            environment["PR3_FIXTURE_ACTUAL_RUSTC"] = actual.as_posix()

            identity = evidence._tool_identity("rustc", environment)  # noqa: SLF001

            self.assertEqual(identity["path"], "rustc")
            self.assertEqual(identity["version"], "rustc 1.2.3-fixture")
            self.assertEqual(
                identity["sha256"], hashlib.sha256(actual.read_bytes()).hexdigest()
            )
            self.assertNotEqual(
                identity["sha256"], hashlib.sha256(rustup.read_bytes()).hexdigest()
            )

    def test_repository_wrapper_identity_hashes_effective_binary(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            scripts = root / "scripts"
            scripts.mkdir()
            actual = root / "actual-tool"
            wrapper = scripts / "tool.sh"
            actual.write_text(
                "#!/bin/sh\necho 'actual-tool 7.8.9'\n", encoding="utf-8"
            )
            wrapper.write_text(
                "#!/bin/sh\n"
                "if [ \"$1\" = --pr3-print-effective-tool ]; then\n"
                "  printf '%s\\n' \"$PR3_FIXTURE_ACTUAL_TOOL\"\n"
                "  exit 0\n"
                "fi\n"
                "exec \"$PR3_FIXTURE_ACTUAL_TOOL\" \"$@\"\n",
                encoding="utf-8",
            )
            actual.chmod(0o755)
            wrapper.chmod(0o755)
            environment = dict(os.environ)
            environment["PR3_FIXTURE_ACTUAL_TOOL"] = actual.as_posix()

            identity = evidence._repository_executable_identity(  # noqa: SLF001
                "scripts/tool.sh", root, environment
            )

            self.assertEqual(identity["path"], "actual-tool")
            self.assertEqual(identity["version"], "actual-tool 7.8.9")
            self.assertEqual(
                identity["sha256"], hashlib.sha256(actual.read_bytes()).hexdigest()
            )
            self.assertNotEqual(
                identity["sha256"], hashlib.sha256(wrapper.read_bytes()).hexdigest()
            )

    def test_required_tool_version_blocks_execution_and_pass_forgery(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest_path = self.make_repository(root)
            tool_dir = root / "fixture-bin"
            tool_dir.mkdir()
            tool = tool_dir / "qemu-fixture"

            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            manifest["capabilities"].insert(
                0,
                {
                    "id": "pinned-qemu",
                    "kind": "tool",
                    "value": "qemu-fixture",
                    "external": False,
                    "required_version": "QEMU emulator version 9.2.4",
                },
            )
            host_case = next(case for case in manifest["cases"] if case["id"] == "a.host")
            host_case["requires"] = ["pinned-qemu", "python3"]
            manifest_path.write_text(
                json.dumps(manifest, indent=2) + "\n", encoding="utf-8"
            )
            environment = {
                "PATH": tool_dir.as_posix() + os.pathsep + os.environ["PATH"]
            }

            tool.write_text(
                "#!/bin/sh\necho 'QEMU emulator version 6.2.0'\n", encoding="utf-8"
            )
            tool.chmod(0o755)
            with mock.patch.dict(os.environ, environment, clear=False):
                blocked, blocked_status = evidence.run_manifest(
                    manifest_path=manifest_path,
                    output_dir=root / "evidence" / "wrong-version",
                    repo_root=root,
                    case_filters=set(),
                    policy_filter=None,
                    category_filter=None,
                    arch_filter="host",
                )
            self.assertEqual(blocked_status, 1)
            self.assertEqual(blocked["cases"][0]["state"], "blocked")
            self.assertEqual(blocked["cases"][0]["reason_code"], "missing_prerequisite")
            self.assertIn("tool version mismatch", blocked["cases"][0]["reason"])

            tool.write_text(
                "#!/bin/sh\necho 'QEMU emulator version 9.2.4'\n", encoding="utf-8"
            )
            tool.chmod(0o755)
            output = root / "evidence" / "correct-version"
            with mock.patch.dict(os.environ, environment, clear=False):
                passed, passed_status = evidence.run_manifest(
                    manifest_path=manifest_path,
                    output_dir=output,
                    repo_root=root,
                    case_filters=set(),
                    policy_filter=None,
                    category_filter=None,
                    arch_filter="host",
                )
            self.assertEqual(passed_status, 0)
            self.assertEqual(passed["cases"][0]["state"], "pass")
            self.assertEqual(
                passed["tools"]["qemu-fixture"]["version"],
                "QEMU emulator version 9.2.4",
            )

            forged = copy.deepcopy(passed)
            forged["tools"]["qemu-fixture"]["version"] = (
                "QEMU emulator version 6.2.0"
            )
            with self.assertRaisesRegex(evidence.ResultError, "tool version mismatch"):
                evidence.validate_result_against_manifest(
                    forged,
                    manifest_path=manifest_path,
                    repo_root=root,
                    bundle_root=output,
                    require_full_required=False,
                )

    def make_repository(self, root: Path, *, fail_la: bool = False) -> Path:
        (root / "scripts").mkdir()
        (root / ".gitignore").write_text(
            "/build/\n/evidence/\n/aggregate*/\n", encoding="utf-8"
        )
        (root / "rust-toolchain.toml").write_text(
            '[toolchain]\nchannel = "nightly-2025-05-20"\n', encoding="utf-8"
        )
        (root / "scripts" / "host.py").write_text("print('host pass')\n", encoding="utf-8")
        (root / "scripts" / "rv.py").write_text(
            "from pathlib import Path\n"
            "Path('build').mkdir(exist_ok=True)\n"
            "Path('build/rv.bin').write_bytes(b'fresh-rv-artifact')\n"
            "print('rv build pass')\n",
            encoding="utf-8",
        )
        (root / "scripts" / "la.py").write_text(
            "print('la result')\n" + ("raise SystemExit(7)\n" if fail_la else ""),
            encoding="utf-8",
        )
        manifest = {
            "schema_version": 1,
            "suite_id": "merge-fixture",
            "capabilities": [
                {"id": "python3", "kind": "tool", "value": "python3", "external": False}
            ],
            "runners": [
                {
                    "id": "build-command",
                    "kind": "process",
                    "classifier_kind": "exit_code",
                    "max_evidence": "built",
                    "combine_output": True,
                    "grace_seconds": 0.1,
                },
                {
                    "id": "unit-test",
                    "kind": "process",
                    "classifier_kind": "exit_code",
                    "max_evidence": "static_checked",
                    "combine_output": True,
                    "grace_seconds": 0.1,
                },
            ],
            "inventories": [],
            "cases": [
                {
                    "id": "a.host",
                    "title": "Host fixture",
                    "category": "fixture",
                    "architectures": ["host"],
                    "runner_id": "unit-test",
                    "evidence_level": "static_checked",
                    "policy": "required",
                    "timeout_seconds": 10,
                    "requires": ["python3"],
                    "depends_on": [],
                    "command": ["python3", "scripts/host.py"],
                    "classifier": {"kind": "exit_code"},
                    "provenance": ["scripts/host.py"],
                },
                {
                    "id": "b.rv-build",
                    "title": "RV fixture build",
                    "category": "fixture",
                    "architectures": ["riscv64"],
                    "runner_id": "build-command",
                    "evidence_level": "built",
                    "policy": "required",
                    "timeout_seconds": 10,
                    "requires": ["python3"],
                    "depends_on": [],
                    "command": ["python3", "scripts/rv.py"],
                    "artifacts": ["build/rv.bin"],
                    "classifier": {"kind": "exit_code"},
                    "provenance": ["scripts/rv.py"],
                },
                {
                    "id": "c.la",
                    "title": "LA fixture",
                    "category": "fixture",
                    "architectures": ["loongarch64"],
                    "runner_id": "unit-test",
                    "evidence_level": "static_checked",
                    "policy": "required",
                    "timeout_seconds": 10,
                    "requires": ["python3"],
                    "depends_on": [],
                    "command": ["python3", "scripts/la.py"],
                    "classifier": {"kind": "exit_code"},
                    "provenance": ["scripts/la.py"],
                },
            ],
        }
        manifest_path = root / "scripts" / "manifest.json"
        manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
        subprocess.run(["git", "init", "-q"], cwd=root, check=True)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(
            [
                "git",
                "-c",
                "user.name=PR3 Test",
                "-c",
                "user.email=pr3@example.invalid",
                "commit",
                "-qm",
                "fixture",
            ],
            cwd=root,
            check=True,
        )
        return manifest_path

    def test_repository_local_unignored_output_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            with self.assertRaisesRegex(evidence.EvidenceError, "must be ignored"):
                evidence.run_manifest(
                    manifest_path=manifest,
                    output_dir=root / "source-evidence",
                    repo_root=root,
                    case_filters=set(),
                    policy_filter=None,
                    category_filter=None,
                    arch_filter="host",
                )

    def test_root_or_symlinked_output_is_rejected_without_external_deletion(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "repo"
            root.mkdir()
            manifest = self.make_repository(root)
            with self.assertRaisesRegex(evidence.EvidenceError, "filesystem root"):
                evidence.run_manifest(
                    manifest_path=manifest,
                    output_dir=Path("/"),
                    repo_root=root,
                    case_filters=set(),
                    policy_filter=None,
                    category_filter=None,
                    arch_filter="host",
                )

            external = Path(temporary) / "external"
            external.mkdir()
            sentinel = external / "semantic-evidence-v1.json"
            sentinel.write_text("do not delete")
            link = root / "build"
            link.symlink_to(external, target_is_directory=True)
            with self.assertRaisesRegex(evidence.EvidenceError, "symbolic links"):
                evidence.run_manifest(
                    manifest_path=manifest,
                    output_dir=link,
                    repo_root=root,
                    case_filters=set(),
                    policy_filter=None,
                    category_filter=None,
                    arch_filter="host",
                )
            self.assertEqual(sentinel.read_text(), "do not delete")

    def test_manifest_aware_result_rejects_source_content_drift(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            shard = self.run_shards(root, manifest)[0]
            (root / "scripts" / "host.py").write_text(
                "print('mutated after collection')\n", encoding="utf-8"
            )
            with self.assertRaisesRegex(evidence.ResultError, "content does not match"):
                evidence.load_validate_result_with_manifest(
                    shard,
                    manifest_path=manifest,
                    repo_root=root,
                    require_full_required=False,
                )

    def test_preexecution_block_with_declared_artifact_writes_canonical(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            document = json.loads(manifest.read_text(encoding="utf-8"))
            document["capabilities"].insert(
                0,
                {
                    "id": "missing-tool",
                    "kind": "tool",
                    "value": "orays-definitely-missing-tool",
                    "external": False,
                },
            )
            rv_case = next(case for case in document["cases"] if case["id"] == "b.rv-build")
            rv_case["requires"] = ["missing-tool", "python3"]
            manifest.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")
            output = root / "evidence" / "blocked"
            result, status = evidence.run_manifest(
                manifest_path=manifest,
                output_dir=output,
                repo_root=root,
                case_filters=set(),
                policy_filter=None,
                category_filter=None,
                arch_filter="riscv64",
            )
            self.assertEqual(status, 1)
            case = result["cases"][0]
            self.assertEqual((case["state"], case["reason_code"]), ("blocked", "missing_prerequisite"))
            self.assertEqual(case["artifacts"], [])
            self.assertTrue((output / "semantic-evidence-v1.json").is_file())
            self.assertTrue((output / case["logs"]["raw"]["path"]).is_file())

    def test_repository_mutation_during_run_is_canonical_error(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            (root / "scripts" / "host.py").write_text(
                "from pathlib import Path\n"
                "path = Path('scripts/host.py')\n"
                "path.write_text(path.read_text() + '# changed during run\\n')\n"
                "print('host command completed')\n",
                encoding="utf-8",
            )
            subprocess.run(["git", "add", "scripts/host.py"], cwd=root, check=True)
            subprocess.run(
                [
                    "git",
                    "-c",
                    "user.name=PR3 Test",
                    "-c",
                    "user.email=pr3@example.invalid",
                    "commit",
                    "-qm",
                    "mutating fixture",
                ],
                cwd=root,
                check=True,
            )
            output = root / "evidence" / "mutating"
            result, status = evidence.run_manifest(
                manifest_path=manifest,
                output_dir=output,
                repo_root=root,
                case_filters=set(),
                policy_filter=None,
                category_filter=None,
                arch_filter="host",
            )
            self.assertEqual(status, 1)
            case = result["cases"][0]
            self.assertEqual(
                (case["state"], case["reason_code"]),
                ("error", "repository_changed_during_run"),
            )
            self.assertTrue((output / "semantic-evidence-v1.json").is_file())
            self.assertTrue((output / case["logs"]["raw"]["path"]).is_file())

    def test_failed_build_without_declared_artifact_writes_canonical(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            (root / "scripts" / "rv.py").write_text(
                "print('forced build failure')\nraise SystemExit(7)\n",
                encoding="utf-8",
            )
            output = root / "evidence" / "failed-build"
            result, status = evidence.run_manifest(
                manifest_path=manifest,
                output_dir=output,
                repo_root=root,
                case_filters=set(),
                policy_filter=None,
                category_filter=None,
                arch_filter="riscv64",
            )
            self.assertEqual(status, 1)
            case = result["cases"][0]
            self.assertEqual((case["state"], case["reason_code"]), ("error", "artifact_missing"))
            self.assertEqual(case["artifacts"], [])
            self.assertTrue((output / "semantic-evidence-v1.json").is_file())
            self.assertTrue((output / case["logs"]["raw"]["path"]).is_file())

    def run_shards(self, root: Path, manifest: Path) -> list[Path]:
        shards = []
        for arch, name in (("host", "host"), ("riscv64", "rv"), ("loongarch64", "la")):
            output = root / "evidence" / name
            evidence.run_manifest(
                manifest_path=manifest,
                output_dir=output,
                repo_root=root,
                case_filters=set(),
                policy_filter=None,
                category_filter=None,
                arch_filter=arch,
            )
            shards.append(output / "semantic-evidence-v1.json")
        return shards

    def test_shard_merge_is_complete_artifact_bound_and_order_stable(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            shards = self.run_shards(root, manifest)
            first_dir = root / "build" / "aggregate-first"
            second_dir = root / "build" / "aggregate-second"
            first, first_status = evidence.merge_result_shards(
                shard_paths=shards,
                manifest_path=manifest,
                output_dir=first_dir,
                repo_root=root,
            )
            second, second_status = evidence.merge_result_shards(
                shard_paths=list(reversed(shards)),
                manifest_path=manifest,
                output_dir=second_dir,
                repo_root=root,
            )
            self.assertEqual((first_status, second_status), (0, 0))
            self.assertTrue(first["run"]["selection"]["complete_required"])
            self.assertEqual(first, second)
            self.assertEqual(
                (first_dir / "semantic-evidence-v1.json").read_bytes(),
                (second_dir / "semantic-evidence-v1.json").read_bytes(),
            )
            rv = next(case for case in first["cases"] if case["case_id"] == "b.rv-build")
            self.assertEqual(rv["artifacts"][0]["size_bytes"], len(b"fresh-rv-artifact"))
            self.assertTrue((first_dir / rv["artifacts"][0]["path"]).is_file())

    def test_merge_rejects_missing_and_duplicate_shards(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            shards = self.run_shards(root, manifest)
            with self.assertRaisesRegex(evidence.ResultError, "required shard instances are missing"):
                evidence.merge_result_shards(
                    shard_paths=shards[:2],
                    manifest_path=manifest,
                    output_dir=root / "build" / "missing",
                    repo_root=root,
                )

    def test_failed_merge_removes_stale_aggregate(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root)
            shards = self.run_shards(root, manifest)
            output = root / "build" / "stale-aggregate"
            evidence.merge_result_shards(
                shard_paths=shards,
                manifest_path=manifest,
                output_dir=output,
                repo_root=root,
            )
            (output / "reports").mkdir()
            (output / "reports" / "stale.html").write_text("old green")
            shards[-1].unlink()
            with self.assertRaises(evidence.EvidenceError):
                evidence.merge_result_shards(
                    shard_paths=shards,
                    manifest_path=manifest,
                    output_dir=output,
                    repo_root=root,
                )
            self.assertFalse((output / "semantic-evidence-v1.json").exists())
            self.assertFalse((output / "reports").exists())
            with self.assertRaisesRegex(evidence.ResultError, "must be unique"):
                evidence.merge_result_shards(
                    shard_paths=[shards[0], shards[0]],
                    manifest_path=manifest,
                    output_dir=root / "build" / "duplicate",
                    repo_root=root,
                )

    def test_complete_merge_preserves_required_nonpass_exit_status(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            manifest = self.make_repository(root, fail_la=True)
            shards = self.run_shards(root, manifest)
            merged, status = evidence.merge_result_shards(
                shard_paths=shards,
                manifest_path=manifest,
                output_dir=root / "build" / "aggregate",
                repo_root=root,
            )
            self.assertEqual(status, 1)
            self.assertEqual(merged["summary"]["required_nonpass"], 1)
            la = next(case for case in merged["cases"] if case["case_id"] == "c.la")
            self.assertEqual(la["state"], "fail")


if __name__ == "__main__":
    unittest.main()
