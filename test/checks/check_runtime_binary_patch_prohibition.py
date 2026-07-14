#!/usr/bin/env python3
"""Static guard for runtime-binary-patch-prohibition: musl runtime ELF patching is retired."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


FORBIDDEN_PROGRAM_LOADER_TOKENS = [
    "MUSL_PATCH_RETIREMENT_DIRECTIVE",
    "type MuslPatchManifestEntry",
    "RISCV_MUSL_PATCH_MANIFEST",
    "LOONGARCH_MUSL_PATCH_MANIFEST",
    "validate_musl_patch_manifest",
    "ensure_musl_patch_manifest_symbol",
    "patch_riscv_musl",
    "patch_loongarch_musl",
    "find_dynsym_file_offset",
    "find_symbol_file_offset",
    "reserve_elf_rx_patch_area",
    "ENOSYS_BRK_STUB",
    "ENOMEM_SBRK_STUB",
    "patched_prefix_sha256",
    "original_prefix_sha256",
]

REQUIRED_RETIRED_DOC_TOKENS = [
    "Status: retired",
    "runtime byte patching is prohibited",
    "do not patch ELF bytes",
    "raw syscall",
    "musl",
    "glibc",
    "TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap",
    "rebuild or replace the runtime",
]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")



def rust_function_names(text: str) -> set[str]:
    return set(
        re.findall(
            r"(?:^|\n)\s*(?:pub(?:\([^)]*\))?\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(",
            text,
        )
    )


def rust_const_names(text: str) -> set[str]:
    return set(
        re.findall(
            r"(?:^|\n)\s*(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Za-z_][A-Za-z0-9_]*)\s*:",
            text,
        )
    )

def scan_program_loader(root: Path) -> list[str]:
    path = root / "user/shell/src/uspace/program_loader.rs"
    text = read(path)
    findings: list[str] = []
    function_names = rust_function_names(text)
    const_names = rust_const_names(text)
    for name in sorted(function_names):
        if ("musl" in name and "patch" in name) or name.startswith(("find_dynsym_file_offset", "find_symbol_file_offset", "reserve_elf_rx_patch_area")):
            findings.append(f"program_loader: retired patch helper function is still defined: {name}")
    for name in sorted(const_names):
        if "MUSL" in name and "PATCH" in name:
            findings.append(f"program_loader: retired musl patch constant is still defined: {name}")
    for token in FORBIDDEN_PROGRAM_LOADER_TOKENS:
        if token in text:
            findings.append(
                f"program_loader: retired musl runtime patch token is still present: {token}"
            )
    load_block_start = text.find("pub(super) fn load_program_image")
    if load_block_start < 0:
        findings.append("program_loader: missing load_program_image")
    else:
        load_block_end = text.find("\nfn effective_exec_root", load_block_start)
        load_block = text[load_block_start:load_block_end if load_block_end >= 0 else len(text)]
        for token in ("patch_riscv", "patch_loongarch", "as_mut_slice()"):
            if token in load_block:
                findings.append(
                    f"program_loader: load_program_image still mutates loaded ELF bytes through {token}"
                )
    return findings


def scan_docs(root: Path) -> list[str]:
    manifest = root / "test/fixtures/runtime_binary_patch_prohibition/musl_patch_retirement_contract.md"
    gate = root / "test/fixtures/runtime_binary_patch_prohibition/stable_promotion_reproof_contract.md"
    findings: list[str] = []
    if not manifest.exists():
        findings.append("docs: missing musl-runtime-patch-manifest.md retirement note")
        return findings
    if not gate.exists():
        findings.append("docs: missing stable-reproof-gate.md")
        return findings
    manifest_text = read(manifest)
    for token in REQUIRED_RETIRED_DOC_TOKENS:
        if token not in manifest_text:
            findings.append(f"retired musl patch doc missing required token: {token}")
    gate_text = read(gate)
    required_gate_tokens = [
        "Total entries | 1000",
        "Unique cases | 1000",
        "RV64 | musl",
        "RV64 | glibc",
        "LA64 | musl",
        "LA64 | glibc",
        "--promotion-candidates",
        "--promotion-arches rv,la",
        "--promotion-libcs musl,glibc",
        "TCONF",
        "TBROK",
        "TFAIL",
        "ENOSYS/not implemented",
        "timeout",
        "panic",
        "trap",
        "prior fail event",
        "status0-only result",
        "named LTP case/path/process special handling",
        "does not add or remove entries from `LTP_STABLE_CASES`",
    ]
    for token in required_gate_tokens:
        if token not in gate_text:
            findings.append(f"stable re-proof gate doc missing required token: {token}")
    return findings


def scan_parser_gate(root: Path) -> list[str]:
    summary = read(root / "test/evaluation/summarize_ltp_results.py")
    tests = read(root / "test/unit/test_ltp_result_summary.py")
    findings: list[str] = []
    if 'default="rv,la"' not in summary:
        findings.append("ltp_summary: promotion arches must default to rv,la")
    if 'default="musl,glibc"' not in summary:
        findings.append("ltp_summary: promotion libcs must default to musl,glibc")
    for token in ("blacklist", "sweep:", "all-minus-blacklist"):
        if token not in summary:
            findings.append(f"ltp_summary: promotion mode blocker missing token {token}")
    for test_name in (
        "test_promotion_candidate_requires_four_way_clean_matrix",
        "test_promotion_candidate_blocks_missing_arch_libc_combo",
        "test_promotion_candidate_blocks_pass_with_internal_tconf",
        "test_promotion_candidate_blocks_blacklist_selection_mode",
        "test_promotion_candidate_blocks_prior_failure_event_even_if_later_passes",
    ):
        if test_name not in tests:
            findings.append(f"test_ltp_summary: missing promotion gate regression {test_name}")
    required_test_contracts = [
        'self.assertEqual(report["candidate_count"], 1)',
        'self.assertEqual(len(report["candidates"][0]["combos"]), 4)',
        'self.assertEqual(report["blocked"][0]["missing"], [{"arch": "la", "libc": "glibc"}])',
        'self.assertEqual(blocker["reasons"], ["TCONF=1"])',
        'selection-mode=stable-plus-all-minus-blacklist',
        'self.assertEqual(reasons, ["event-failures=1"])',
    ]
    for token in required_test_contracts:
        if token not in tests:
            findings.append(f"test_ltp_summary: weakened promotion gate assertion/token: {token}")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    root = args.root.resolve()
    findings: list[str] = []
    findings.extend(scan_program_loader(root))
    findings.extend(scan_docs(root))
    findings.extend(scan_parser_gate(root))
    if findings:
        print("runtime-binary-patch-prohibition musl runtime patch retirement static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("runtime-binary-patch-prohibition musl runtime patch retirement static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
