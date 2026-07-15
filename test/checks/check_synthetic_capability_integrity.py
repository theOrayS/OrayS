#!/usr/bin/env python3
"""Static guard for synthetic-capability synthetic proc/dev/config honesty boundaries."""

from __future__ import annotations

import argparse
import ast
import re
from pathlib import Path


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def rust_const_block(text: str, name: str) -> str:
    marker = f"const {name}:"
    start = text.find(marker)
    if start < 0:
        return ""
    end = text.find(";", start)
    if end < 0:
        return text[start:]
    return text[start : end + 1]


def decode_rust_byte_literals(block: str) -> str:
    chunks = re.findall(r'b"(?:\\.|[^"\\])*"', block, re.DOTALL)
    out = bytearray()
    for chunk in chunks:
        try:
            out.extend(ast.literal_eval(chunk))
        except (SyntaxError, ValueError):
            continue
    return out.decode("utf-8", errors="ignore")


def scan_synthetic_fs(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/synthetic_fs.rs")
    findings: list[str] = []
    lowered = text.lower()
    if "ltp" in lowered or "oskernel2026" in lowered:
        findings.append(
            "user/shell/src/uspace/synthetic_fs.rs: synthetic proc/config content must not carry LTP/oskernel2026 markers"
        )

    cmdline_block = rust_const_block(text, "SYNTHETIC_PROC_CMDLINE_CONTENT")
    if not cmdline_block:
        findings.append(
            "user/shell/src/uspace/synthetic_fs.rs: missing SYNTHETIC_PROC_CMDLINE_CONTENT"
        )
    else:
        cmdline = decode_rust_byte_literals(cmdline_block)
        if "ltp" in cmdline.lower() or "oskernel2026" in cmdline.lower():
            findings.append("/proc/cmdline synthetic content still contains LTP-specific marker")
        if "root=/dev/vda" not in cmdline:
            findings.append("/proc/cmdline must point at the single supported synthetic block device")

    config_block = rust_const_block(text, "SYNTHETIC_KERNEL_CONFIG_CONTENT")
    if not config_block:
        findings.append(
            "user/shell/src/uspace/synthetic_fs.rs: missing SYNTHETIC_KERNEL_CONFIG_CONTENT"
        )
    else:
        if "CONFIG_EVENTFD=y" not in config_block:
            findings.append("synthetic kernel config lost the backed eventfd capability")
        if "LTP" in config_block:
            findings.append("synthetic kernel config must not describe itself as an LTP probe")
        if "implemented Linux ABI surfaces" not in config_block:
            findings.append("synthetic kernel config must state its capability-backed boundary")
    return findings


def scan_block_devices(root: Path) -> list[str]:
    fd_table = read(root / "user/shell/src/uspace/fd_table.rs")
    metadata = read(root / "user/shell/src/uspace/metadata.rs")
    findings: list[str] = []
    match = re.search(
        r"const\s+SYNTHETIC_BLOCK_DEVICE_NAMES\s*:\s*&\[[^\]]+\]\s*=\s*&\[(?P<body>[^\]]*)\]",
        fd_table,
        re.DOTALL,
    )
    if not match:
        findings.append("user/shell/src/uspace/fd_table.rs: missing synthetic block device capability list")
    else:
        names = re.findall(r'"([^"]+)"', match.group("body"))
        if names != ["vda"]:
            findings.append(
                f"user/shell/src/uspace/fd_table.rs: synthetic block devices must be exactly ['vda'], got {names!r}"
            )
    for forbidden in ("DEV_SDA_RDEV", "DEV_XVDA_RDEV", '"/dev/sda"', '"/dev/xvda"'):
        if forbidden in metadata or forbidden in fd_table:
            findings.append(f"synthetic block device alias is still exposed: {forbidden}")
    return findings


def scan_linux_abi_markers(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/linux_abi.rs")
    findings: list[str] = []
    lowered = text.lower()
    for token in ("ltp", "oskernel2026"):
        if token in lowered:
            findings.append(
                f"user/shell/src/uspace/linux_abi.rs: Linux ABI constants must not carry {token.upper()}-aware markers"
            )
    if "SYNTHETIC_BLOCK_DEVICE_SIZE" not in text:
        findings.append("user/shell/src/uspace/linux_abi.rs: missing synthetic block device size ABI constant")
    return findings


def scan_backing_evidence(root: Path) -> list[str]:
    synthetic_fs = read(root / "user/shell/src/uspace/synthetic_fs.rs")
    fd_table = read(root / "user/shell/src/uspace/fd_table.rs")
    findings: list[str] = []
    if "CONFIG_EVENTFD=y" in synthetic_fs:
        for token in ("sys_eventfd2", "FdEntry::EventFd"):
            if token not in fd_table:
                findings.append(f"CONFIG_EVENTFD is exposed without backing evidence token {token}")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    root = args.root.resolve()

    findings: list[str] = []
    findings.extend(scan_synthetic_fs(root))
    findings.extend(scan_block_devices(root))
    findings.extend(scan_linux_abi_markers(root))
    findings.extend(scan_backing_evidence(root))
    if findings:
        print("synthetic-capability synthetic capability static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("synthetic-capability synthetic capability static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
