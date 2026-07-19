from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "scripts/desktop/create-run-dir.py"


class OutputPathTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="orays-output-path-")
        self.repo = Path(self.temporary.name) / "repo"
        self.output = self.repo / "test/output/desktop"
        self.output.mkdir(parents=True)

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def invoke(self, *arguments: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [
                sys.executable,
                "-B",
                str(SCRIPT),
                "--repo-root",
                str(self.repo),
                *arguments,
            ],
            check=False,
            capture_output=True,
            text=True,
        )

    def test_parent_escape_is_rejected_without_creation(self) -> None:
        escaped = self.output / "../../../escape"
        result = self.invoke("--candidate", str(escaped))
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertFalse((self.repo / "escape").exists())

    def test_symlink_parent_escape_is_rejected(self) -> None:
        outside = Path(self.temporary.name) / "outside"
        outside.mkdir()
        (self.output / "link").symlink_to(outside, target_is_directory=True)
        result = self.invoke("--candidate", str(self.output / "link/run"))
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertFalse((outside / "run").exists())

    def test_legal_new_directory_is_created_below_output_root(self) -> None:
        candidate = self.output / "legal/run"
        result = self.invoke("--candidate", str(candidate))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(Path(result.stdout.strip()), candidate)
        self.assertTrue(candidate.is_dir())

    def test_existing_directory_is_refused(self) -> None:
        candidate = self.output / "existing"
        candidate.mkdir()
        result = self.invoke("--candidate", str(candidate))
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("refusing to overwrite", result.stderr)

    def test_random_prefix_creates_one_real_child(self) -> None:
        result = self.invoke("--prefix", "qemu-rv-boot.")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        created = Path(result.stdout.strip())
        self.assertEqual(created.parent, self.output)
        self.assertTrue(created.is_dir())
        self.assertFalse(created.is_symlink())

    def test_missing_desktop_root_is_created_from_tracked_output_parent(self) -> None:
        self.output.rmdir()
        (self.repo / "test/output/.gitignore").write_text("*\n", encoding="utf-8")
        result = self.invoke("--prefix", "qemu-rv-boot.")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        created = Path(result.stdout.strip())
        self.assertEqual(created.parent, self.output)
        self.assertTrue(created.is_dir())
        self.assertFalse(self.output.is_symlink())


if __name__ == "__main__":
    unittest.main()
