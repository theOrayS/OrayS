#!/usr/bin/env python3
import os
from pathlib import Path
import subprocess
import unittest


REPO_ROOT = Path(__file__).resolve().parents[2]


class WindowManagerHostContractTests(unittest.TestCase):
    def test_rust_window_shell_and_damage_contracts(self) -> None:
        environment = os.environ.copy()
        environment.pop("DISPLAY", None)
        environment.pop("WAYLAND_DISPLAY", None)
        result = subprocess.run(
            [str(REPO_ROOT / "scripts/desktop/build.sh"), "host-test"],
            cwd=REPO_ROOT,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=180,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("test result: ok", result.stdout)
        self.assertIn("animated_damage_matches_fresh_composition", result.stdout)
        self.assertIn("launcher_shortcut_animates", result.stdout)


if __name__ == "__main__":
    unittest.main()
