#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/desktop/summarize-run.py"
SPEC = importlib.util.spec_from_file_location("desktop_summarize_run", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
QMP_SCRIPT = ROOT / "scripts/desktop/qmp_screendump.py"
QMP_SPEC = importlib.util.spec_from_file_location("desktop_qmp_screendump", QMP_SCRIPT)
assert QMP_SPEC is not None and QMP_SPEC.loader is not None
QMP_MODULE = importlib.util.module_from_spec(QMP_SPEC)
QMP_SPEC.loader.exec_module(QMP_MODULE)


def write_json_lines(path: Path, rows: list[dict]) -> None:
    path.write_text("".join(json.dumps(row) + "\n" for row in rows), encoding="utf-8")


def qmp_exchange(commands: list[dict]) -> list[dict]:
    rows = [
        {
            "direction": "receive",
            "message": {"QMP": {"version": {}, "capabilities": []}},
        }
    ]
    for command in commands:
        rows.append({"direction": "send", "message": command})
        rows.append({"direction": "receive", "message": {"return": {}}})
    return rows


class SummarizeRunTests(unittest.TestCase):
    def make_run(self, root: Path) -> Path:
        run = root / "run"
        run.mkdir()
        (run / "serial.log").write_text(
            "\n".join(
                [
                    "ORAYS_DESKTOP_DISPLAY width=2 height=2",
                    "ORAYS_DESKTOP_INPUT_READY devices=2",
                    "ORAYS_DESKTOP_FRAME boot 123",
                    "ORAYS_DESKTOP_ACTION LAUNCHER OPEN",
                    "ORAYS_DESKTOP_FRAME animation 456",
                    "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE",
                ]
            )
            + "\n",
            encoding="utf-8",
        )
        (run / "display-geometry.txt").write_text(
            "DISPLAY_GEOMETRY=2x2\n", encoding="utf-8"
        )
        qemu = root / "qemu-system-riscv64"
        qemu.write_bytes(b"approved-qemu-object")
        qemu.chmod(0o755)
        artifact = root / "build/desktop/rv/artifacts/orays-desktop-rv.bin"
        artifact.parent.mkdir(parents=True)
        artifact.write_bytes(b"guest-artifact-object")
        qemu_digest = hashlib.sha256(qemu.read_bytes()).hexdigest()
        policy = {
            "schema": 1,
            "qemu_version": "9.2.4",
            "architectures": {
                "rv": {
                    "qemu_binary": qemu.name,
                    "qemu_sha256": qemu_digest,
                    "artifact": "build/desktop/rv/artifacts/orays-desktop-rv.bin",
                    "build_invocation": ["scripts/desktop/build.sh", "rv"],
                }
            },
        }
        policy_path = run / "runtime-policy.json"
        policy_path.write_text(
            json.dumps(policy, sort_keys=True) + "\n", encoding="utf-8"
        )
        (run / "runtime-metadata.json").write_text(
            json.dumps(
                {
                    "schema": 4,
                    "created_at_utc": "2026-07-19T00:00:00+00:00",
                    "finalized_at_utc": "2026-07-19T00:01:00+00:00",
                    "source_commit": "a" * 40,
                    "source_dirty": False,
                    "source_status": [],
                    "source_commit_before": "a" * 40,
                    "source_dirty_before": False,
                    "source_status_before": [],
                    "source_commit_after": "a" * 40,
                    "source_dirty_after": False,
                    "source_status_after": [],
                    "provenance_stable": True,
                    "collection_errors": [],
                    "repository_root": str(root),
                    "architecture": "rv",
                    "scenario": "launcher",
                    "run_dir": str(run),
                    "qemu_binary": str(qemu),
                    "qemu_version": "QEMU emulator version 9.2.4",
                    "required_qemu_version": "9.2.4",
                    "observed_qemu_version": "QEMU emulator version 9.2.4",
                    "qemu_version_matches_required": True,
                    "toolchain_versions": {
                        "rustc": "rustc test",
                        "cargo": "cargo test",
                        "python": "Python 3.10",
                    },
                    "generation_command": [
                        "scripts/desktop/run-headless-qemu.sh",
                        "--arch",
                        "rv",
                        "--scenario",
                        "launcher",
                        "--output",
                        "test/output/desktop/run",
                    ],
                    "runtime_identity": {
                        "schema": 1,
                        "policy_repository_path": "test/desktop/runtime-policy.json",
                        "policy_sha256": hashlib.sha256(
                            policy_path.read_bytes()
                        ).hexdigest(),
                        "qemu": {
                            "canonical_path": str(qemu),
                            "required_version": "9.2.4",
                            "observed_banner": "QEMU emulator version 9.2.4",
                            "required_sha256": qemu_digest,
                            "observed_sha256": qemu_digest,
                        },
                        "guest_artifact": {
                            "architecture": "rv",
                            "repository_path": "build/desktop/rv/artifacts/orays-desktop-rv.bin",
                            "canonical_path": str(artifact),
                            "sha256": hashlib.sha256(
                                artifact.read_bytes()
                            ).hexdigest(),
                            "source_commit": "a" * 40,
                        },
                        "build_invocation": ["scripts/desktop/build.sh", "rv"],
                        "qemu_launch_argv": [
                            str(qemu),
                            "-machine",
                            "virt",
                            "-kernel",
                            str(artifact),
                        ],
                    },
                }
            ),
            encoding="utf-8",
        )
        serial = (run / "serial.log").read_bytes()
        (run / "capture-precondition.json").write_text(
            json.dumps(
                {
                    "schema": 1,
                    "action_marker": "ORAYS_DESKTOP_ACTION LAUNCHER OPEN",
                    "action_line": 4,
                    "stable_marker": "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE",
                    "stable_line": 6,
                    "serial_prefix_bytes": len(serial),
                    "serial_prefix_sha256": hashlib.sha256(serial).hexdigest(),
                }
            ),
            encoding="utf-8",
        )
        sequence = [
            {
                "events": [
                    {
                        "type": "key",
                        "data": {
                            "down": True,
                            "key": {"type": "qcode", "data": "a"},
                        },
                    }
                ]
            }
        ]
        (run / "input-sequence.json").write_text(json.dumps(sequence), encoding="utf-8")
        write_json_lines(
            run / "qmp-input.jsonl",
            qmp_exchange(
                [
                    {"execute": "qmp_capabilities"},
                    {
                        "execute": "input-send-event",
                        "arguments": {"events": sequence[0]["events"]},
                    },
                ]
            ),
        )
        frame = run / "frame.ppm"
        frame.write_bytes(b"P6\n2 2\n255\n" + b"\0\0\0\xff\0\0\0\xff\0\0\0\xff")
        write_json_lines(
            run / "qmp-capture.jsonl",
            qmp_exchange(
                [
                    {"execute": "qmp_capabilities"},
                    {"execute": "screendump", "arguments": {"filename": str(frame)}},
                    {"execute": "quit"},
                ]
            ),
        )
        return run

    def test_complete_evidence_is_accepted(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            failures, geometry, hashes = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertEqual(failures, [])
            self.assertEqual(geometry, (2, 2))
            self.assertEqual(set(hashes), {
                "serial.log",
                "qmp-capture.jsonl",
                "qmp-input.jsonl",
                "input-sequence.json",
                "frame.ppm",
                "capture-precondition.json",
                "runtime-metadata.json",
                "runtime-policy.json",
                "display-geometry.txt",
            })

    def test_cli_writes_pass_summary_for_complete_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(SCRIPT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "rv",
                    "--scenario",
                    "launcher",
                    "--qemu-exit",
                    "0",
                ],
                check=False,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            summary = json.loads((run / "summary.json").read_text(encoding="utf-8"))
            self.assertEqual(summary["schema"], 3)
            self.assertEqual(summary["result"], "PASS")
            self.assertEqual(summary["failure_stage"], "complete")
            self.assertEqual(summary["failure_reason"], "none")
            self.assertEqual(summary["post_action_state_marker_count"], 1)

    def test_qemu_not_started_cannot_pass_with_complete_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            result = subprocess.run(
                [
                    sys.executable,
                    "-B",
                    str(SCRIPT),
                    "--run-dir",
                    str(run),
                    "--arch",
                    "rv",
                    "--scenario",
                    "launcher",
                    "--qemu-started",
                    "false",
                    "--runner-exit",
                    "0",
                ],
                check=False,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
            if (run / "summary.json").exists():
                summary = json.loads((run / "summary.json").read_text(encoding="utf-8"))
                self.assertEqual(summary["result"], "FAIL")

    def test_status_tokens_reject_empty_illegal_unknown_and_inconsistent_values(self) -> None:
        cases = (
            ("", "qemu_version_mismatch"),
            ("   ", "qemu_version_mismatch"),
            ("runtime prerequisites", "qemu_version_mismatch"),
            ("unknown-stage", "qemu_version_mismatch"),
            ("runtime-prerequisites", ""),
            ("runtime-prerequisites", "   "),
            ("runtime-prerequisites", "reason with spaces"),
            ("runtime-prerequisites", "desktop_build_failure"),
        )
        for stage, reason in cases:
            with self.subTest(stage=stage, reason=reason), tempfile.TemporaryDirectory() as directory:
                run = self.make_run(Path(directory))
                (run / "summary.json").unlink(missing_ok=True)
                (run / "hashes.sha256").unlink(missing_ok=True)
                result = subprocess.run(
                    [
                        sys.executable,
                        "-B",
                        str(SCRIPT),
                        "--run-dir",
                        str(run),
                        "--arch",
                        "rv",
                        "--scenario",
                        "launcher",
                        "--qemu-started",
                        "false",
                        "--runner-exit",
                        "3",
                        "--failure-stage",
                        stage,
                        "--failure-reason",
                        reason,
                    ],
                    check=False,
                    text=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                )
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertFalse((run / "summary.json").exists())

    def test_only_exact_canonical_qemu_version_is_accepted(self) -> None:
        cases = (
            ("9.2.4", "QEMU emulator version 9.2.40"),
            ("9.2.4", "QEMU emulator version 9.2.4-rc0"),
            ("6.2.0", "QEMU emulator version 6.2.0"),
        )
        for required, observed in cases:
            with self.subTest(required=required, observed=observed), tempfile.TemporaryDirectory() as directory:
                run = self.make_run(Path(directory))
                metadata_path = run / "runtime-metadata.json"
                metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
                metadata.update(
                    {
                        "required_qemu_version": required,
                        "observed_qemu_version": observed,
                        "qemu_version": observed,
                        "qemu_version_matches_required": True,
                    }
                )
                metadata_path.write_text(json.dumps(metadata), encoding="utf-8")
                failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
                self.assertTrue(
                    any("QEMU version" in failure for failure in failures),
                    failures,
                )

    def test_missing_stable_marker_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            serial = run / "serial.log"
            serial.write_text(
                serial.read_text(encoding="utf-8").replace(
                    "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE\n", ""
                ),
                encoding="utf-8",
            )
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertTrue(any("stable marker" in failure for failure in failures))

    def test_source_change_during_run_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            path = run / "runtime-metadata.json"
            metadata = json.loads(path.read_text(encoding="utf-8"))
            metadata["source_commit_after"] = "b" * 40
            metadata["provenance_stable"] = False
            path.write_text(json.dumps(metadata), encoding="utf-8")
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertTrue(any("changed during the run" in failure for failure in failures))

    def test_stable_marker_before_action_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            serial = run / "serial.log"
            text = serial.read_text(encoding="utf-8")
            text = text.replace(
                "ORAYS_DESKTOP_ACTION LAUNCHER OPEN\nORAYS_DESKTOP_FRAME animation 456\n"
                "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE\n",
                "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE\n"
                "ORAYS_DESKTOP_ACTION LAUNCHER OPEN\nORAYS_DESKTOP_FRAME animation 456\n",
            )
            serial.write_text(text, encoding="utf-8")
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertTrue(any("stable marker" in failure for failure in failures))

    def test_serial_prefix_tampering_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            serial = run / "serial.log"
            serial.write_text(
                serial.read_text(encoding="utf-8").replace(
                    "ORAYS_DESKTOP_FRAME boot 123", "ORAYS_DESKTOP_FRAME boot 124"
                ),
                encoding="utf-8",
            )
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertTrue(any("prefix hash" in failure for failure in failures))

    def test_capture_precondition_binds_an_append_safe_serial_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            serial = root / "serial.log"
            serial.write_text(
                "ORAYS_DESKTOP_ACTION LAUNCHER OPEN\n"
                "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE\n",
                encoding="utf-8",
            )
            precondition = root / "capture-precondition.json"
            QMP_MODULE.record_capture_precondition(
                serial,
                "ORAYS_DESKTOP_ACTION LAUNCHER OPEN",
                "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE",
                precondition,
            )
            with serial.open("a", encoding="utf-8") as stream:
                stream.write("QEMU shutdown completed\n")
            MODULE.validate_capture_precondition(
                precondition,
                serial,
                "ORAYS_DESKTOP_ACTION LAUNCHER OPEN",
                "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE",
            )

    def test_capture_precondition_rejects_stable_before_action(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            serial = root / "serial.log"
            serial.write_text(
                "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE\n"
                "ORAYS_DESKTOP_ACTION LAUNCHER OPEN\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(QMP_MODULE.QmpError, "does not follow"):
                QMP_MODULE.record_capture_precondition(
                    serial,
                    "ORAYS_DESKTOP_ACTION LAUNCHER OPEN",
                    "ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE",
                    root / "capture-precondition.json",
                )

    def test_generic_capture_precondition_binds_ordered_markers(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            serial = root / "serial.log"
            markers = ["DISPLAY READY", "ACTION COMPLETE"]
            serial.write_text("DISPLAY READY\nACTION COMPLETE\n", encoding="utf-8")
            precondition = root / "capture-precondition.json"
            QMP_MODULE.record_required_markers_precondition(
                serial, markers, precondition
            )
            with serial.open("a", encoding="utf-8") as stream:
                stream.write("QEMU shutdown completed\n")
            MODULE.validate_required_markers_precondition(
                precondition, serial, markers
            )

    def test_resize_input_marker_requires_a_preceding_presented_frame(self) -> None:
        display = "ORAYS_DESKTOP_DISPLAY_CHANGED width=900 height=650"
        pointer = (
            "ORAYS_DESKTOP_INPUT PointerMoved { position: Point { x: 450, y: 325 }, "
            "delta_x: 450, delta_y: 325 }"
        )
        MODULE.validate_presented_input_order(
            [display, "ORAYS_DESKTOP_FRAME input 456", pointer], display, pointer
        )
        with self.assertRaisesRegex(ValueError, "no presented input frame"):
            MODULE.validate_presented_input_order(
                [display, pointer, "ORAYS_DESKTOP_FRAME input 456"], display, pointer
            )

    def test_missing_input_readiness_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            serial = run / "serial.log"
            serial.write_text(
                serial.read_text(encoding="utf-8").replace(
                    "ORAYS_DESKTOP_INPUT_READY devices=2\n", ""
                ),
                encoding="utf-8",
            )
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertTrue(any("input readiness" in failure for failure in failures))

    def test_qmp_error_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            rows = qmp_exchange(
                [
                    {"execute": "qmp_capabilities"},
                    {"execute": "input-send-event", "arguments": {"events": []}},
                ]
            )
            rows[-1] = {"direction": "receive", "message": {"error": {"class": "GenericError"}}}
            write_json_lines(run / "qmp-input.jsonl", rows)
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertTrue(any("input evidence" in failure for failure in failures))

    def test_sequence_mismatch_fails_closed_without_qmp_error(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            rows = qmp_exchange(
                [
                    {"execute": "qmp_capabilities"},
                    {
                        "execute": "input-send-event",
                        "arguments": {
                            "events": [
                                {
                                    "type": "key",
                                    "data": {
                                        "down": False,
                                        "key": {"type": "qcode", "data": "b"},
                                    },
                                }
                            ]
                        },
                    },
                ]
            )
            write_json_lines(run / "qmp-input.jsonl", rows)
            with self.assertRaisesRegex(ValueError, "does not match the sequence"):
                MODULE.validate_input_evidence(
                    run / "input-sequence.json",
                    run / "qmp-input.jsonl",
                    (2, 2),
                )

    def test_geometry_mismatch_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            (run / "frame.ppm").write_bytes(
                b"P6\n3 2\n255\n" + b"\0\0\0\xff\0\0\0\xff\0\0\0\xff\xff\0\0\0\xff"
            )
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertIn("screenshot geometry does not match the guest display marker", failures)

    def test_ambiguous_guest_geometry_marker_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            serial = run / "serial.log"
            serial.write_text(
                serial.read_text(encoding="utf-8")
                + "ORAYS_DESKTOP_DISPLAY width=2 height=2\n",
                encoding="utf-8",
            )
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertIn("guest display geometry marker missing or ambiguous", failures)

    def test_resize_evidence_binds_local_vnc_request(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "vnc-resize.json"
            path.write_text(
                json.dumps(
                    {
                        "schema": 1,
                        "transport": "RFB SetDesktopSize",
                        "endpoint": "127.0.0.1:5942",
                        "server_version": "RFB 003.008",
                        "security_type": "None",
                        "initial_geometry": [1024, 768],
                        "requested_geometry": [900, 650],
                        "screen_count": 1,
                        "extended_desktop_size_encoding": -308,
                    }
                ),
                encoding="utf-8",
            )
            MODULE.validate_resize_evidence(path, (1024, 768), (900, 650))
            value = json.loads(path.read_text(encoding="utf-8"))
            value["endpoint"] = "0.0.0.0:5942"
            path.write_text(json.dumps(value), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "not localhost"):
                MODULE.validate_resize_evidence(path, (1024, 768), (900, 650))

    def test_uniform_frame_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            run = self.make_run(Path(directory))
            (run / "frame.ppm").write_bytes(b"P6\n2 2\n255\n" + b"\0" * 12)
            failures, _, _ = MODULE.validate_run(run, "rv", "launcher", 0)
            self.assertIn("screenshot is a uniform frame", failures)


if __name__ == "__main__":
    unittest.main()
