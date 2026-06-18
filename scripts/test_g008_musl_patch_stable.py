#!/usr/bin/env python3
"""Regression tests for the G008 musl runtime patch retirement guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_g008_musl_patch_stable.py"
TARGETS = [
    Path("examples/shell/src/uspace/program_loader.rs"),
    Path("docs/ltp-real-semantics-repair-2026-06-07/musl-runtime-patch-manifest.md"),
    Path("docs/ltp-real-semantics-repair-2026-06-07/stable-reproof-gate.md"),
    Path("scripts/ltp_summary.py"),
    Path("scripts/test_ltp_summary.py"),
]


class G008MuslPatchStableGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="g008-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(encoding="utf-8"), encoding="utf-8")
        return tmp

    def run_guard(self, tree: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(GUARD), "--root", str(tree)],
            check=False,
            capture_output=True,
            text=True,
        )

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_reintroduced_runtime_patch_function(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/program_loader.rs"
        text = path.read_text(encoding="utf-8")
        text += "\nfn patch_riscv_musl_syscall_stubs(image: &mut [u8]) { let _ = image; }\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("patch_riscv_musl", result.stdout)

    def test_detects_reintroduced_symbol_lookup(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/program_loader.rs"
        text = path.read_text(encoding="utf-8")
        text += "\nfn find_dynsym_file_offset() {}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("find_dynsym_file_offset", result.stdout)

    def test_detects_reintroduced_rx_patch_area(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/program_loader.rs"
        text = path.read_text(encoding="utf-8")
        text += "\nfn reserve_elf_rx_patch_area() {}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("reserve_elf_rx_patch_area", result.stdout)

    def test_detects_missing_retirement_doc_contract(self) -> None:
        tree = self.make_tree()
        path = tree / "docs/ltp-real-semantics-repair-2026-06-07/musl-runtime-patch-manifest.md"
        text = path.read_text(encoding="utf-8").replace("runtime byte patching is prohibited", "runtime patching may continue")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("runtime byte patching is prohibited", result.stdout)

    def test_detects_weakened_promotion_arch_default(self) -> None:
        tree = self.make_tree()
        path = tree / "scripts/ltp_summary.py"
        text = path.read_text(encoding="utf-8").replace('default="rv,la"', 'default="rv"', 1)
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("rv,la", result.stdout)

    def test_detects_missing_four_combo_gate_doc(self) -> None:
        tree = self.make_tree()
        path = tree / "docs/ltp-real-semantics-repair-2026-06-07/stable-reproof-gate.md"
        text = path.read_text(encoding="utf-8").replace("LA64 | glibc", "LA64 | skipped")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("LA64 | glibc", result.stdout)

    def test_detects_missing_quality_signal_doc_tokens(self) -> None:
        tree = self.make_tree()
        path = tree / "docs/ltp-real-semantics-repair-2026-06-07/stable-reproof-gate.md"
        text = path.read_text(encoding="utf-8").replace(
            "TCONF, TBROK, TFAIL, ENOSYS/not implemented, timeout, panic, trap, or prior fail event",
            "generic failure signals",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("TCONF", result.stdout)

    def test_detects_name_only_parser_test_contract(self) -> None:
        tree = self.make_tree()
        path = tree / "scripts/test_ltp_summary.py"
        text = path.read_text(encoding="utf-8").replace(
            'self.assertEqual(len(report["candidates"][0]["combos"]), 4)',
            "self.assertTrue(True)",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("combos", result.stdout)


if __name__ == "__main__":
    unittest.main()
