#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
import socket
import tempfile
import threading
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/desktop/inject-input.py"
SPEC = importlib.util.spec_from_file_location("desktop_inject_input", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class InputSequenceTests(unittest.TestCase):
    def test_repository_fixture_is_valid(self) -> None:
        expected_steps = {
            "boot.json": 1,
            "basic.json": 3,
            "launcher.json": 1,
            "overlap.json": 1,
            "applications.json": 4,
        }
        for name, expected in expected_steps.items():
            with self.subTest(name=name):
                steps = MODULE.load_sequence(ROOT / "test/desktop/fixtures/input" / name)
                self.assertEqual(len(steps), expected)

    def test_invalid_event_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "invalid.json"
            path.write_text('[{"events":[{"type":"mystery","data":{}}]}]', encoding="utf-8")
            with self.assertRaises(MODULE.InputInjectionError):
                MODULE.load_sequence(path)

    def test_pixel_coordinates_resolve_for_both_guest_geometries(self) -> None:
        sequence = MODULE.load_sequence(
            ROOT / "test/desktop/fixtures/input/applications.json"
        )
        events = [event for step in sequence for event in step.get("events", [])]
        for display_size, expected_pixels in (
            ((1024, 768), ((708, 728), (398, 235))),
            ((1280, 800), ((836, 760), (398, 235))),
        ):
            resolved = [MODULE.resolve_event(event, display_size) for event in events]
            points = []
            for index in (0, 4):
                x = resolved[index]["data"]["value"] * (display_size[0] - 1) // 32767
                y = resolved[index + 1]["data"]["value"] * (display_size[1] - 1) // 32767
                points.append((x, y))
            self.assertEqual(tuple(points), expected_pixels)

    def test_qmp_handshake_and_input_command_are_verified(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            socket_path = Path(directory) / "qmp.sock"
            received: list[dict] = []

            def server() -> None:
                with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as listener:
                    listener.bind(str(socket_path))
                    listener.listen(1)
                    connection, _ = listener.accept()
                    with connection:
                        stream = connection.makefile("rwb", buffering=0)
                        stream.write(b'{"QMP":{"version":{},"capabilities":[]}}\r\n')
                        for _ in range(2):
                            request = json.loads(stream.readline().decode("utf-8"))
                            received.append(request)
                            stream.write(b'{"return":{}}\r\n')

            thread = threading.Thread(target=server)
            thread.start()
            steps = [
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
            transcript = MODULE.inject(socket_path, steps, 2.0)
            thread.join(timeout=2.0)

            self.assertFalse(thread.is_alive())
            self.assertEqual(received[0]["execute"], "qmp_capabilities")
            self.assertEqual(received[1]["execute"], "input-send-event")
            self.assertTrue(any(row["direction"] == "receive" for row in transcript))


if __name__ == "__main__":
    unittest.main()
