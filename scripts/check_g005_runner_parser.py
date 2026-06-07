#!/usr/bin/env python3
"""Static guard for G005 LTP runner/parser honesty boundaries."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


class Finding(list):
    pass


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def function_block(text: str, name: str) -> str:
    marker = f"fn {name}"
    start = text.find(marker)
    if start < 0:
        return ""
    next_fn = text.find("\nfn ", start + len(marker))
    next_cfg_fn = text.find("\n#[cfg", start + len(marker))
    candidates = [pos for pos in (next_fn, next_cfg_fn) if pos >= 0]
    end = min(candidates) if candidates else len(text)
    return text[start:end]


def scan_cmd_rs(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/cmd.rs")
    findings: list[str] = []
    env_block = function_block(text, "ltp_case_env")
    run_dir_block = function_block(text, "prepare_ltp_case_run_dir")
    if not env_block:
        findings.append("examples/shell/src/cmd.rs: missing ltp_case_env")
    else:
        if '"chdir01"' in env_block or "chdir01" in env_block:
            findings.append("examples/shell/src/cmd.rs: ltp_case_env must not special-case chdir01")
        if "needs_case_resource_helper" not in env_block:
            findings.append("examples/shell/src/cmd.rs: ltp_case_env must be driven by generic helper/device capability")
        if "LTP_FORCE_SINGLE_FS_TYPE=tmpfs" not in env_block or "LTP_DEV_FS_TYPE=tmpfs" not in env_block:
            findings.append("examples/shell/src/cmd.rs: helper-backed filesystem env boundary is missing")
    if not run_dir_block:
        findings.append("examples/shell/src/cmd.rs: missing prepare_ltp_case_run_dir")
    elif "needs_case_resource_helper" not in run_dir_block:
        findings.append("examples/shell/src/cmd.rs: run-dir selection must reuse generic helper detection")
    if re.search(r"if\s+case\s*==\s*\"chdir01\"", text):
        findings.append("examples/shell/src/cmd.rs: case-name branch for chdir01 is forbidden")
    return findings


def scan_makefile(root: Path) -> list[str]:
    text = read(root / "Makefile")
    findings: list[str] = []
    if not re.search(r"^REMOTE_LTP_CASES\s*\?=\s*stable\s*$", text, re.MULTILINE):
        findings.append("Makefile: REMOTE_LTP_CASES default must be stable")
    if re.search(r"^REMOTE_LTP_CASES\s*\?=\s*stable-plus-blacklist\s*$", text, re.MULTILINE):
        findings.append("Makefile: stable-plus-blacklist must not be the remote default")
    return findings


def scan_ltp_summary(root: Path) -> list[str]:
    text = read(root / "scripts/ltp_summary.py")
    findings: list[str] = []
    required_tokens = {
        "CASE_LIST_RE": "case-list manifest regex",
        "case_list_manifests": "case-list manifests in compact output",
        "promotion_mode_blocker": "promotion mode blocker",
        "selection-mode=": "visible selection-mode blocker reason",
        "blacklist": "blacklist promotion blocker token",
        "all-minus-blacklist": "all-minus-blacklist promotion blocker token",
    }
    for token, label in required_tokens.items():
        if token not in text:
            findings.append(f"scripts/ltp_summary.py: missing {label}")
    tests = read(root / "scripts/test_ltp_summary.py")
    for test_name in (
        "test_case_list_manifest_is_reported",
        "test_promotion_candidate_blocks_blacklist_selection_mode",
    ):
        if test_name not in tests:
            findings.append(f"scripts/test_ltp_summary.py: missing {test_name}")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    root = args.root.resolve()
    findings: list[str] = []
    findings.extend(scan_cmd_rs(root))
    findings.extend(scan_makefile(root))
    findings.extend(scan_ltp_summary(root))
    if findings:
        print("G005 runner/parser static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("G005 runner/parser static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
