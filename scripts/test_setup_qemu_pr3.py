#!/usr/bin/env python3
"""Offline contract tests for the pinned PR3 QEMU source setup script."""

from __future__ import annotations

import hashlib
import subprocess
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("setup_qemu_pr3.sh")
VERSION = "9.2.4"
SHA256 = "f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a"


class SetupQemuPr3Test(unittest.TestCase):
    def make_structurally_valid_transferred_prefix(self, root: Path) -> Path:
        prefix = root / "qemu"
        binary_dir = prefix / "bin"
        binary_dir.mkdir(parents=True)
        binary_hashes = {}
        for name in ("qemu-system-riscv64", "qemu-system-loongarch64"):
            binary = binary_dir / name
            binary.write_text(f"#!/usr/bin/env sh\necho 'QEMU emulator version {VERSION}'\n")
            binary.chmod(0o755)
            binary_hashes[name] = hashlib.sha256(binary.read_bytes()).hexdigest()
        (prefix / f".orays-pr3-qemu-{VERSION}").write_text(
            "\n".join(
                (
                    f"version={VERSION}",
                    f"source_url=https://download.qemu.org/qemu-{VERSION}.tar.xz",
                    f"source_sha256={SHA256}",
                    "source_size_bytes=134782772",
                    "target_list=riscv64-softmmu,loongarch64-softmmu",
                    "configure_profile=--disable-docs --disable-werror --disable-download --disable-slirp",
                    f"qemu-system-riscv64_sha256={binary_hashes['qemu-system-riscv64']}",
                    f"qemu-system-loongarch64_sha256={binary_hashes['qemu-system-loongarch64']}",
                    "",
                )
            )
        )
        return prefix

    def test_verify_only_checks_transferred_artifact_structure(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prefix = self.make_structurally_valid_transferred_prefix(root)
            result = subprocess.run(
                ["bash", str(SCRIPT), "--verify-only", str(prefix), str(root / "cache")],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 0, result.stderr)

    def test_verify_only_rejects_mutated_stamp(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prefix = self.make_structurally_valid_transferred_prefix(root)
            stamp = prefix / f".orays-pr3-qemu-{VERSION}"
            stamp.write_text(stamp.read_text().replace(SHA256, "0" * 64))
            result = subprocess.run(
                ["bash", str(SCRIPT), "--verify-only", str(prefix), str(root / "cache")],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 1)

    def test_verify_only_rejects_binary_changed_after_stamp(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prefix = self.make_structurally_valid_transferred_prefix(root)
            with (prefix / "bin" / "qemu-system-riscv64").open("a") as binary:
                binary.write("# changed\n")
            result = subprocess.run(
                ["bash", str(SCRIPT), "--verify-only", str(prefix), str(root / "cache")],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 1)

    def test_normal_mode_rejects_forged_prepopulated_prefix(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prefix = self.make_structurally_valid_transferred_prefix(root)
            result = subprocess.run(
                ["bash", str(SCRIPT), str(prefix), str(root / "cache")],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 2)
            self.assertIn("refusing to reuse", result.stderr)

    def test_refuses_nonempty_unverified_prefix_before_network(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prefix = root / "qemu"
            prefix.mkdir()
            (prefix / "unknown").write_text("do not delete")
            result = subprocess.run(
                ["bash", str(SCRIPT), str(prefix), str(root / "cache")],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 2)
            self.assertTrue((prefix / "unknown").is_file())

    def test_refuses_root_or_empty_paths(self) -> None:
        result = subprocess.run(
            ["bash", str(SCRIPT), "/", "/tmp/cache"],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 2)

    def test_refuses_symlinked_prefix_or_cache_before_network(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            prefix_link = root / "prefix-link"
            cache_link = root / "cache-link"
            prefix_link.symlink_to("/")
            cache_link.symlink_to("/")
            for prefix, cache in (
                (prefix_link, root / "cache"),
                (root / "prefix", cache_link),
            ):
                with self.subTest(prefix=prefix, cache=cache):
                    result = subprocess.run(
                        ["bash", str(SCRIPT), str(prefix), str(cache)],
                        check=False,
                        capture_output=True,
                        text=True,
                    )
                    self.assertEqual(result.returncode, 2)
                    self.assertIn("symlinked", result.stderr)

    def test_required_profile_disables_downloads_and_slirp(self) -> None:
        text = SCRIPT.read_text(encoding="utf-8")
        self.assertIn("--disable-download", text)
        self.assertIn("--disable-slirp", text)

    def test_archive_extraction_does_not_restore_foreign_ownership(self) -> None:
        text = SCRIPT.read_text(encoding="utf-8")
        self.assertIn('tar --no-same-owner -xf "$archive" -C "$work_dir"', text)


if __name__ == "__main__":
    unittest.main()
