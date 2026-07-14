#!/usr/bin/env python3
"""Guard NUMA memory-policy syscalls against ignored-argument success."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

from source_scan import function_block, read


def scan_memory_policy(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/memory_policy.rs")
    dispatch = read(root / "user/shell/src/uspace/syscall_dispatch.rs")
    findings: list[str] = []
    if "fn nodemask_is_empty" not in text or "fn default_policy_only" not in text:
        findings.append("memory policy must validate default-only state with nodemask contents")
    if "LinuxError::EOPNOTSUPP" not in text:
        findings.append("memory policy must return EOPNOTSUPP for unsupported NUMA policy changes")
    mbind_block = function_block(text, "sys_mbind")
    set_block = function_block(text, "sys_set_mempolicy")
    get_block = function_block(text, "sys_get_mempolicy")
    if "default_policy_only" not in mbind_block:
        findings.append("mbind must reject non-default mempolicy modes instead of validating only nodemask")
    if "flags: usize" not in mbind_block or "flags != MEMBIND_SUPPORTED_FLAGS" not in mbind_block:
        findings.append("mbind must consume and reject unsupported flags instead of ignoring arg5")
    dispatch_block = dispatch[
        dispatch.find("general::__NR_mbind") : dispatch.find("general::__NR_get_mempolicy")
    ]
    if "tf.arg5()" not in dispatch_block:
        findings.append("syscall_dispatch must pass mbind arg5 flags to sys_mbind")
    if re.search(
        r"let\s+_\s*=\s*\(start,\s*len,\s*mode\);\s*validate_mempolicy_request",
        mbind_block,
    ):
        findings.append("mbind still ignores range/mode and validates only the nodemask")
    if "default_policy_only" not in set_block or re.search(
        r"let\s+_\s*=\s*mode;\s*validate_mempolicy_request", set_block
    ):
        findings.append("set_mempolicy must not accept every validated mode as success")
    if "flags != 0" not in get_block or "LinuxError::EOPNOTSUPP" not in get_block:
        findings.append("get_mempolicy must reject unsupported flags instead of clearing outputs")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    findings = scan_memory_policy(args.root.resolve())
    if findings:
        print("memory policy semantics check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("memory policy semantics check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
