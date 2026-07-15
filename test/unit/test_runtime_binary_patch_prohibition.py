#!/usr/bin/env python3
"""Regression tests for the runtime-binary-patch-prohibition musl runtime patch retirement guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_runtime_binary_patch_prohibition.py"
TARGETS = [
    Path("user/shell/src/uspace/program_loader.rs"),
    Path("test/fixtures/runtime_binary_patch_prohibition/musl_patch_retirement_contract.md"),
    Path("test/fixtures/runtime_binary_patch_prohibition/stable_promotion_reproof_contract.md"),
    Path("test/evaluation/summarize_ltp_results.py"),
    Path("test/unit/test_ltp_result_summary.py"),
]


class RuntimeBinaryPatchProhibitionGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="runtime-binary-patch-prohibition-guard-"))
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
        path = tree / "user/shell/src/uspace/program_loader.rs"
        text = path.read_text(encoding="utf-8")
        text += "\nfn patch_riscv_musl_syscall_stubs(image: &mut [u8]) { let _ = image; }\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("patch_riscv_musl", result.stdout)

    def test_detects_reintroduced_symbol_lookup(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/program_loader.rs"
        text = path.read_text(encoding="utf-8")
        text += "\nfn find_dynsym_file_offset() {}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("find_dynsym_file_offset", result.stdout)

    def test_detects_reintroduced_rx_patch_area(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/program_loader.rs"
        text = path.read_text(encoding="utf-8")
        text += "\nfn reserve_elf_rx_patch_area() {}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("reserve_elf_rx_patch_area", result.stdout)

    def test_detects_missing_retirement_doc_contract(self) -> None:
        tree = self.make_tree()
        path = tree / "test/fixtures/runtime_binary_patch_prohibition/musl_patch_retirement_contract.md"
        text = path.read_text(encoding="utf-8").replace("runtime byte patching is prohibited", "runtime patching may continue")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("runtime byte patching is prohibited", result.stdout)

    def test_detects_weakened_promotion_arch_default(self) -> None:
        tree = self.make_tree()
        path = tree / "test/evaluation/summarize_ltp_results.py"
        text = path.read_text(encoding="utf-8").replace('default="rv,la"', 'default="rv"', 1)
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("rv,la", result.stdout)

    def test_detects_missing_four_combo_gate_doc(self) -> None:
        tree = self.make_tree()
        path = tree / "test/fixtures/runtime_binary_patch_prohibition/stable_promotion_reproof_contract.md"
        text = path.read_text(encoding="utf-8").replace("LA64 | glibc", "LA64 | skipped")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("LA64 | glibc", result.stdout)

    def test_detects_missing_quality_signal_doc_tokens(self) -> None:
        tree = self.make_tree()
        path = tree / "test/fixtures/runtime_binary_patch_prohibition/stable_promotion_reproof_contract.md"
        text = path.read_text(encoding="utf-8").replace(
            "TCONF, TBROK, TFAIL, ENOSYS/not implemented, timeout, panic, trap, or prior fail event",
            "generic failure signals",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("TCONF", result.stdout)

        for token in (
            "python3 -I -S -B -X pycache_prefix=/dev/null",
            "--stderr-log <rv-stderr-log>",
            "--stderr-log <la-stderr-log>",
            "--process-exit-code <rv-evaluator-exit-code>",
            "--process-exit-code <la-evaluator-exit-code>",
            "exact full sets `rv,la` and `musl,glibc`",
            "empty or unknown promotion dimension",
            "outside an LTP group",
            "invalid UTF-8",
            "START/RUN/result/Pass!/END",
            "planned/executed mismatch",
            "missing, duplicate, late, or mismatched LTP summary",
            "`ERROR` or `FAIL`",
        ):
            with self.subTest(token=token):
                token_tree = self.make_tree()
                token_path = (
                    token_tree
                    / "test/fixtures/runtime_binary_patch_prohibition/stable_promotion_reproof_contract.md"
                )
                token_path.write_text(
                    token_path.read_text(encoding="utf-8").replace(token, "removed contract", 1),
                    encoding="utf-8",
                )
                token_result = self.run_guard(token_tree)
                self.assertNotEqual(token_result.returncode, 0)
                self.assertIn(token, token_result.stdout)

    def test_detects_name_only_parser_test_contract(self) -> None:
        tree = self.make_tree()
        path = tree / "test/unit/test_ltp_result_summary.py"
        text = path.read_text(encoding="utf-8").replace(
            'self.assertEqual(len(report["candidates"][0]["combos"]), 4)',
            "self.assertTrue(True)",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("combos", result.stdout)

        lifecycle_tree = self.make_tree()
        lifecycle_path = lifecycle_tree / "test/unit/test_ltp_result_summary.py"
        lifecycle_path.write_text(
            lifecycle_path.read_text(encoding="utf-8").replace(
                "test_promotion_candidate_requires_complete_lifecycle",
                "test_promotion_candidate_accepts_partial_lifecycle",
                1,
            ),
            encoding="utf-8",
        )
        lifecycle_result = self.run_guard(lifecycle_tree)
        self.assertNotEqual(lifecycle_result.returncode, 0)
        self.assertIn("requires_complete_lifecycle", lifecycle_result.stdout)

        dimension_tree = self.make_tree()
        dimension_path = dimension_tree / "test/evaluation/summarize_ltp_results.py"
        dimension_path.write_text(
            dimension_path.read_text(encoding="utf-8").replace(
                "validate_promotion_dimensions",
                "accept_empty_promotion_dimensions",
            ),
            encoding="utf-8",
        )
        dimension_result = self.run_guard(dimension_tree)
        self.assertNotEqual(dimension_result.returncode, 0)
        self.assertIn("mandatory lifecycle binding token", dimension_result.stdout)

        stderr_tree = self.make_tree()
        stderr_path = stderr_tree / "test/evaluation/summarize_ltp_results.py"
        stderr_path.write_text(
            stderr_path.read_text(encoding="utf-8").replace(
                "stderr_path=str(stderr_path)",
                "stderr_path='omitted'",
                1,
            ),
            encoding="utf-8",
        )
        stderr_result = self.run_guard(stderr_tree)
        self.assertNotEqual(stderr_result.returncode, 0)
        self.assertIn("mandatory lifecycle binding token", stderr_result.stdout)

        for token, replacement in (
            ('"TCONF=1" in candidate["reasons"]', '"TCONF=0" in candidate["reasons"]'),
            (
                'self.assertIn("event-failures=1", reasons)',
                'self.assertIn("event-failures=0", reasons)',
            ),
            (
                'self.assertIn("strict-malformed-protocol-record", outside_reasons)',
                'self.assertIn("ignored-malformed-protocol-record", outside_reasons)',
            ),
        ):
            with self.subTest(test_contract_token=token):
                contract_tree = self.make_tree()
                contract_path = contract_tree / "test/unit/test_ltp_result_summary.py"
                contract_text = contract_path.read_text(encoding="utf-8")
                self.assertIn(token, contract_text)
                contract_path.write_text(
                    contract_text.replace(token, replacement, 1),
                    encoding="utf-8",
                )
                contract_result = self.run_guard(contract_tree)
                self.assertNotEqual(contract_result.returncode, 0)
                self.assertIn("weakened promotion gate assertion/token", contract_result.stdout)

        for token, replacement in (
            ('validation=data["strict_validation"]', "validation={}"),
            ('item["strict_case_binding"]', 'item["unchecked_case_binding"]'),
            (
                "if args.strict or args.promotion_candidates or stderr_path is not None:",
                "if stderr_path is not None:",
            ),
        ):
            with self.subTest(summary_binding_token=token):
                binding_tree = self.make_tree()
                binding_path = binding_tree / "test/evaluation/summarize_ltp_results.py"
                binding_text = binding_path.read_text(encoding="utf-8")
                self.assertIn(token, binding_text)
                binding_path.write_text(
                    binding_text.replace(token, replacement, 1),
                    encoding="utf-8",
                )
                binding_result = self.run_guard(binding_tree)
                self.assertNotEqual(binding_result.returncode, 0)
                self.assertIn("mandatory lifecycle binding token", binding_result.stdout)


if __name__ == "__main__":
    unittest.main()
