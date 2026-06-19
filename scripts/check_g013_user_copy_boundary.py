#!/usr/bin/env python3
"""Static guard for keeping POSIX syscall user copy-in/out behind shared helpers.

This guard is intentionally source-policy oriented: it does not claim MMU-backed
fault isolation, but it prevents reviewed syscall implementations from
re-introducing scattered raw pointer copy/read/write primitives that self-check
review already identified as a risk.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT_REL = Path("api/arceos_posix_api/src")
UTILS_REL = ROOT_REL / "utils.rs"
IMP_REL = ROOT_REL / "imp"

HELPER_TOKENS = (
    "pub unsafe fn read_user_value",
    "pub unsafe fn write_user_value",
    "pub unsafe fn readable_user_buffer",
    "pub unsafe fn writable_user_buffer",
    "pub unsafe fn readable_user_slice",
    "pub unsafe fn writable_user_slice",
    "check_null_ptr(ptr)?;",
    "check_null_mut_ptr(ptr)?;",
)

RAW_MEMORY_PRIMITIVE_RE = re.compile(
    r"\b(?:(?:core::)?ptr::(?:read|write)(?:_unaligned|_volatile)?|"
    r"(?:core::)?ptr::copy(?:_nonoverlapping)?|"
    r"(?:read|write)(?:_unaligned|_volatile)|copy_nonoverlapping)"
    r"\s*(?:::<[^>]+>)?\s*\("
)

RAW_SLICE_PRIMITIVE_RE = re.compile(r"\b(?:core::slice|Vec)::from_raw_parts(?:_mut)?\s*\(")
UNSAFE_DEREF_RE = re.compile(r"unsafe\s*\{\s*(?:&\s*)?\*")
UNSAFE_DEREF_MULTILINE_RE = re.compile(r"unsafe\s*\{\s*\n\s*(?:&\s*)?\*")
UNSAFE_PTR_METHOD_RE = re.compile(
    r"unsafe\s*\{[^\n{}]*(?:\b\w*ptr\b|\b\w+_ptr\b)\s*\.\s*"
    r"(?:read|write)(?:_unaligned|_volatile)?\s*\("
)


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def is_allowed_raw_copy(rel: Path, line: str) -> bool:
    rel_s = rel.as_posix()
    if rel == UTILS_REL:
        return True
    # freeaddrinfo reclaims an addrinfo buffer allocated by sys_getaddrinfo.
    if rel_s == "api/arceos_posix_api/src/imp/net.rs" and "Vec::from_raw_parts(aibuf_ptr" in line:
        return True
    return False


def is_allowed_unsafe_deref(rel: Path, line: str) -> bool:
    rel_s = rel.as_posix()
    if rel_s == "api/arceos_posix_api/src/imp/pthread/mutex.rs" and "mutex.cast::<Self>()" in line:
        return True
    if rel_s == "api/arceos_posix_api/src/imp/pthread/mod.rs" and "result.get()" in line:
        return True
    if rel_s == "api/arceos_posix_api/src/imp/net.rs" and "(*aibuf_ptr)" in line:
        return True
    return False


def has_raw_memory_primitive(line: str) -> bool:
    return bool(RAW_MEMORY_PRIMITIVE_RE.search(line) or RAW_SLICE_PRIMITIVE_RE.search(line))


def rust_function_block(text: str, name: str) -> str:
    match = re.search(
        rf"(?:^|\n)\s*(?:pub\s+)?(?:unsafe\s+)?fn\s+{re.escape(name)}\s*\([^{{]*\)\s*(?:->[^{{]+)?\{{",
        text,
    )
    if not match:
        return ""
    brace = text.find("{", match.start())
    depth = 0
    for idx in range(brace, len(text)):
        ch = text[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[match.start() : idx + 1]
    return text[match.start() :]


def scan(root: Path) -> list[str]:
    findings: list[str] = []
    utils_path = root / UTILS_REL
    if not utils_path.exists():
        findings.append(f"{UTILS_REL}: missing shared user-copy helper module")
        return findings

    utils = read(utils_path)
    for token in HELPER_TOKENS:
        if token not in utils:
            findings.append(f"{UTILS_REL}: missing helper contract token {token!r}")

    imp_root = root / IMP_REL
    if not imp_root.exists():
        findings.append(f"{IMP_REL}: missing syscall implementation tree")
        return findings

    for path in sorted(imp_root.rglob("*.rs")):
        rel = path.relative_to(root)
        text = read(path)
        for lineno, line in enumerate(text.splitlines(), 1):
            if has_raw_memory_primitive(line) and not is_allowed_raw_copy(rel, line):
                findings.append(f"{rel}:{lineno}: raw memory copy/slice primitive must use utils user-copy helpers")
            if UNSAFE_DEREF_RE.search(line) and not is_allowed_unsafe_deref(rel, line):
                findings.append(f"{rel}:{lineno}: unsafe raw deref in syscall impl must be copied through utils or documented as kernel-owned")
            if UNSAFE_PTR_METHOD_RE.search(line) and not is_allowed_unsafe_deref(rel, line):
                findings.append(f"{rel}:{lineno}: unsafe raw pointer method read/write must use utils user-copy helpers")
        for match in UNSAFE_DEREF_MULTILINE_RE.finditer(text):
            lineno = text.count("\n", 0, match.start()) + 1
            snippet = text[match.start() : match.start() + 120].replace("\n", " ")
            if not is_allowed_unsafe_deref(rel, snippet):
                findings.append(f"{rel}:{lineno}: multiline unsafe raw deref must use utils user-copy helpers")

    epoll = read(root / IMP_REL / "io_mpx" / "epoll.rs")
    epoll_ctl = rust_function_block(epoll, "sys_epoll_ctl")
    if "read_user_value(event as *const ctypes::epoll_event)?" not in epoll_ctl:
        findings.append("sys_epoll_ctl: EPOLL_CTL_ADD/MOD event copy-in must use read_user_value")

    net = read(root / IMP_REL / "net.rs")
    hints = rust_function_block(net, "from_ptr")
    if "read_user_value(hints)" not in hints or "ctypes::EAI_SYSTEM" not in hints:
        findings.append("ResolvedAddrInfoHints::from_ptr: hints copy-in must use read_user_value and map pointer faults visibly")

    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    findings = scan(args.root.resolve())
    if findings:
        print("G013 user-copy boundary static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("G013 user-copy boundary static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
