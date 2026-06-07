#!/usr/bin/env python3
"""Static guard for G008 musl patch manifest and stable re-proof gates."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

RISCV_MANIFEST_ENTRIES = [
    ("main-executable", "brk"),
    ("interpreter", "brk"),
    ("interpreter", "sbrk"),
    ("interpreter", "nice"),
    ("interpreter", "gethostname"),
]
LOONGARCH_MANIFEST_ENTRIES = [
    ("main-executable", "brk"),
    ("interpreter", "sched_setparam"),
    ("interpreter", "sched_getparam"),
    ("interpreter", "sched_setscheduler"),
    ("interpreter", "sched_getscheduler"),
    ("interpreter", "brk"),
    ("interpreter", "sbrk"),
    ("interpreter", "gethostname"),
    ("interpreter", "readlink"),
    ("interpreter", "readlinkat"),
]
RISCV_SYMBOLS = sorted({symbol for _target, symbol in RISCV_MANIFEST_ENTRIES})
LOONGARCH_SYMBOLS = sorted({symbol for _target, symbol in LOONGARCH_MANIFEST_ENTRIES})
SCHED_SYMBOLS = {
    "sched_setparam",
    "sched_getparam",
    "sched_setscheduler",
    "sched_getscheduler",
}


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def const_block(text: str, name: str) -> str:
    marker = f"const {name}"
    start = text.find(marker)
    if start < 0:
        return ""
    end = text.find(";\n", start)
    return text[start : end + 2] if end >= 0 else text[start:]


def manifest_entry_exists(block: str, target: str, symbol: str) -> bool:
    return (
        re.search(
            rf'\(\s*"{re.escape(target)}"\s*,\s*"{re.escape(symbol)}"\s*,',
            block,
            flags=re.MULTILINE,
        )
        is not None
    )


def scan_program_loader(root: Path) -> list[str]:
    path = root / "examples/shell/src/uspace/program_loader.rs"
    text = read(path)
    findings: list[str] = []
    if "MUSL_PATCH_RETIREMENT_DIRECTIVE" not in text or "raw syscall + musl + glibc" not in text:
        findings.append("program_loader: missing musl patch retirement directive with raw/musl/glibc evidence")
    if "type MuslPatchManifestEntry" not in text:
        findings.append("program_loader: missing explicit musl patch manifest entry type")
    for const_name, manifest_entries, symbols in (
        ("RISCV_MUSL_PATCH_MANIFEST", RISCV_MANIFEST_ENTRIES, RISCV_SYMBOLS),
        ("LOONGARCH_MUSL_PATCH_MANIFEST", LOONGARCH_MANIFEST_ENTRIES, LOONGARCH_SYMBOLS),
    ):
        block = const_block(text, const_name)
        if not block:
            findings.append(f"program_loader: missing {const_name}")
            continue
        for target, symbol in manifest_entries:
            if not manifest_entry_exists(block, target, symbol):
                findings.append(
                    f"program_loader: {const_name} missing target-specific manifest entry "
                    f'target={target} symbol={symbol}'
                )
        for symbol in symbols:
            if f'"{symbol}"' not in block:
                findings.append(f"program_loader: {const_name} missing symbol {symbol}")
            if symbol in SCHED_SYMBOLS:
                if "ensure_loongarch_musl_patch_manifest_symbol(name)" not in text:
                    findings.append("program_loader: sched raw patch loop must validate symbols through the manifest")
            else:
                ensure_name = "ensure_riscv_musl_patch_manifest_symbol" if const_name.startswith("RISCV") else "ensure_loongarch_musl_patch_manifest_symbol"
                if f'{ensure_name}("{symbol}")' not in text:
                    findings.append(f"program_loader: patch symbol {symbol} is not guarded by {ensure_name}")
    hidden_patch_calls = re.findall(r'find_(?:dynsym|symbol)_file_offset\(&elf,\s*"([^"]+)"\)', text)
    manifest_symbols = set(RISCV_SYMBOLS) | set(LOONGARCH_SYMBOLS) | {
        "__errno_location",
        "__syscall_ret",
        "getpriority",
        "setpriority",
    }
    for symbol in hidden_patch_calls:
        if symbol not in manifest_symbols:
            findings.append(
                f"program_loader: symbol lookup for {symbol} is not covered by the G008 manifest guard"
            )
    return findings


def scan_docs(root: Path) -> list[str]:
    manifest = root / "docs/ltp-real-semantics-repair-2026-06-07/musl-runtime-patch-manifest.md"
    gate = root / "docs/ltp-real-semantics-repair-2026-06-07/stable-reproof-gate.md"
    findings: list[str] = []
    if not manifest.exists():
        findings.append("docs: missing musl-runtime-patch-manifest.md")
        return findings
    if not gate.exists():
        findings.append("docs: missing stable-reproof-gate.md")
        return findings
    manifest_text = read(manifest)
    gate_text = read(gate)
    required_manifest_tokens = [
        "temporary compatibility bridge",
        "raw syscall",
        "musl",
        "glibc",
        "runtime-required",
        "original_prefix_sha256",
        "patched_prefix_sha256",
    ]
    for token in required_manifest_tokens:
        if token not in manifest_text:
            findings.append(f"musl manifest doc missing required token: {token}")
    for symbol in sorted(set(RISCV_SYMBOLS + LOONGARCH_SYMBOLS)):
        if f"`{symbol}`" not in manifest_text:
            findings.append(f"musl manifest doc missing symbol {symbol}")
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
    summary = read(root / "scripts/ltp_summary.py")
    tests = read(root / "scripts/test_ltp_summary.py")
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
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    root = args.root.resolve()
    findings: list[str] = []
    findings.extend(scan_program_loader(root))
    findings.extend(scan_docs(root))
    findings.extend(scan_parser_gate(root))
    if findings:
        print("G008 musl patch/stable gate static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("G008 musl patch/stable gate static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
