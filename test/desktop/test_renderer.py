#!/usr/bin/env python3
import hashlib
import os
from pathlib import Path
import subprocess
import unittest


REPO_ROOT = Path(__file__).resolve().parents[2]
SCENE_ROOT = REPO_ROOT / "test/output/desktop/scenes"
GOLDEN_FILE = REPO_ROOT / "test/desktop/fixtures/golden/shell-scenes.sha256"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def expected_hashes() -> dict[str, str]:
    hashes: dict[str, str] = {}
    for line in GOLDEN_FILE.read_text(encoding="utf-8").splitlines():
        digest, filename = line.split()
        hashes[filename] = digest
    return hashes


class HeadlessRendererTests(unittest.TestCase):
    def run_scene(self, scene: str) -> subprocess.CompletedProcess[str]:
        environment = os.environ.copy()
        environment.pop("DISPLAY", None)
        environment.pop("WAYLAND_DISPLAY", None)
        return subprocess.run(
            [str(REPO_ROOT / "scripts/desktop/build.sh"), "scene", scene],
            cwd=REPO_ROOT,
            env=environment,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=120,
            check=False,
        )

    def test_launcher_scene_is_deterministic_and_matches_golden(self) -> None:
        first = self.run_scene("launcher")
        self.assertEqual(first.returncode, 0, first.stdout)
        path = SCENE_ROOT / "launcher.ppm"
        first_bytes = path.read_bytes()
        second = self.run_scene("launcher")
        self.assertEqual(second.returncode, 0, second.stdout)
        self.assertEqual(path.read_bytes(), first_bytes)
        self.assertEqual(sha256(path), expected_hashes()["launcher.ppm"])
        self.assertTrue(first_bytes.startswith(b"P6\n1280 720\n255\n"))
        self.assertEqual(len(first_bytes), len(b"P6\n1280 720\n255\n") + 1280 * 720 * 3)

    def test_unknown_scene_fails_closed(self) -> None:
        result = self.run_scene("does-not-exist")
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("unsupported desktop scene", result.stdout)

    def test_real_application_scene_is_deterministic_and_matches_golden(self) -> None:
        first = self.run_scene("applications")
        self.assertEqual(first.returncode, 0, first.stdout)
        path = SCENE_ROOT / "applications.ppm"
        first_bytes = path.read_bytes()
        second = self.run_scene("applications")
        self.assertEqual(second.returncode, 0, second.stdout)
        self.assertEqual(path.read_bytes(), first_bytes)
        self.assertEqual(sha256(path), expected_hashes()["applications.ppm"])


if __name__ == "__main__":
    unittest.main()
