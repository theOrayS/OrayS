#!/usr/bin/env python3
"""Static regression guard for G002 fake-success cleanups.

The check is intentionally narrow: it looks only at the Phase-1/G002 surfaces
called out in the 2026-06-07 real-semantics plan and reports source locations
where unsupported POSIX/Linux-visible behavior still returns success instead of
an honest errno/negative errno.  It is a gate, not a fixer.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import argparse
import json
import re
from typing import Iterable

REPO_ROOT = Path(__file__).resolve().parents[1]
AXLIBC_C_GLOB = "ulib/axlibc/c/*.c"
API_IMP_GLOB = "api/arceos_posix_api/src/imp/*.rs"
FD_TABLE_PATH = Path("examples/shell/src/uspace/fd_table.rs")

HIGH_RISK_FD_FUNCS = {
    "truncate",
    "fallocate_allocate",
    "fallocate_punch_hole",
    "fallocate_zero_range",
    "fallocate_collapse_range",
    "fallocate_insert_range",
}

FN_RE = re.compile(r"\bfn\s+([A-Za-z0-9_]+)\s*\(")
RETURN_ZERO_RE = re.compile(r"^return\s+0\s*;")
OK_ZERO_FALLBACK_RE = re.compile(r"_\s*=>\s*Ok\(0\)")
OK_UNIT_FALLBACK_RE = re.compile(r"_\s*=>\s*Ok\(\(\)\)")
FD_UNSUPPORTED_OK_UNIT_RE = re.compile(
    r"FdEntry::(?:DevNull|BlockDevice\(_\)|Rtc)(?:\s*\|\s*FdEntry::(?:DevNull|BlockDevice\(_\)|Rtc))*\s*=>\s*Ok\(\(\)\)"
)


@dataclass(frozen=True)
class Finding:
    path: Path
    line: int
    kind: str
    detail: str

    def display(self) -> str:
        return f"{self.path}:{self.line}: {self.kind}: {self.detail}"

    def as_dict(self) -> dict[str, str | int]:
        return {
            "path": self.path.as_posix(),
            "line": self.line,
            "kind": self.kind,
            "detail": self.detail,
        }


def repo_relative(path: Path, root: Path) -> Path:
    try:
        return path.relative_to(root)
    except ValueError:
        return path


def nearby_success_return(lines: list[str], start_index: int, lookahead: int = 8) -> int | None:
    """Return the 0-based line index of a nearby C `return 0;` success."""

    for idx in range(start_index + 1, min(len(lines), start_index + 1 + lookahead)):
        stripped = lines[idx].strip()
        if stripped.startswith("}"):
            break
        if RETURN_ZERO_RE.search(stripped):
            return idx
    return None


def scan_c_unimplemented_return_success(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    findings: list[Finding] = []
    rel = repo_relative(path, root)
    lines = text.splitlines()
    for idx, line in enumerate(lines):
        if "unimplemented(" not in line:
            continue
        success_idx = nearby_success_return(lines, idx)
        if success_idx is None:
            continue
        findings.append(
            Finding(
                rel,
                success_idx + 1,
                "axlibc-unimplemented-return-zero",
                "unimplemented POSIX/libc stub returns 0 instead of failing honestly",
            )
        )
    return findings


def next_lines_contain_ok_zero(lines: list[str], start_index: int, lookahead: int = 6) -> int | None:
    for idx in range(start_index + 1, min(len(lines), start_index + 1 + lookahead)):
        stripped = lines[idx].strip()
        if "Err(" in stripped:
            return None
        if "Ok(0)" in stripped:
            return idx
    return None


def scan_api_fake_success(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    findings: list[Finding] = []
    rel = repo_relative(path, root)
    lines = text.splitlines()
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if OK_ZERO_FALLBACK_RE.search(stripped):
            findings.append(
                Finding(
                    rel,
                    idx + 1,
                    "api-fallback-ok-zero",
                    "catch-all POSIX API fallback returns Ok(0) for unsupported input",
                )
            )
        if "unsupported" in stripped.lower() or "currently do not support" in stripped.lower():
            ok_idx = next_lines_contain_ok_zero(lines, idx)
            if ok_idx is not None:
                findings.append(
                    Finding(
                        rel,
                        ok_idx + 1,
                        "api-unsupported-ok-zero",
                        "unsupported POSIX API path returns Ok(0) instead of errno",
                    )
                )
        if "stat::default()" in stripped and "TODO" in stripped:
            ok_idx = next_lines_contain_ok_zero(lines, idx)
            if ok_idx is not None:
                findings.append(
                    Finding(
                        rel,
                        ok_idx + 1,
                        "api-default-stat-ok-zero",
                        "default synthetic stat is reported as successful lstat metadata",
                    )
                )
    return findings


def scan_fd_table_fake_success(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    findings: list[Finding] = []
    rel = repo_relative(path, root)
    current_fn: str | None = None
    for idx, line in enumerate(text.splitlines(), start=1):
        if match := FN_RE.search(line):
            current_fn = match.group(1)
        if current_fn not in HIGH_RISK_FD_FUNCS:
            continue
        stripped = line.strip()
        if FD_UNSUPPORTED_OK_UNIT_RE.search(stripped):
            findings.append(
                Finding(
                    rel,
                    idx,
                    "fd-table-unsupported-ok-unit",
                    f"{current_fn} treats dev-null/block/rtc unsupported operation as Ok(())",
                )
            )
        elif OK_UNIT_FALLBACK_RE.search(stripped):
            findings.append(
                Finding(
                    rel,
                    idx,
                    "fd-table-fallback-ok-unit",
                    f"{current_fn} catch-all fallback returns Ok(()) for unsupported fd type",
                )
            )
    return findings


def iter_existing(paths: Iterable[Path]) -> Iterable[Path]:
    for path in paths:
        if path.exists():
            yield path


def scan_repo(root: Path = REPO_ROOT) -> list[Finding]:
    findings: list[Finding] = []
    for path in sorted(root.glob(AXLIBC_C_GLOB)):
        findings.extend(scan_c_unimplemented_return_success(path, path.read_text(), root))
    for path in sorted(root.glob(API_IMP_GLOB)):
        findings.extend(scan_api_fake_success(path, path.read_text(), root))
    for path in iter_existing([root / FD_TABLE_PATH]):
        findings.extend(scan_fd_table_fake_success(path, path.read_text(), root))
    return sorted(findings, key=lambda f: (f.path.as_posix(), f.line, f.kind))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=REPO_ROOT, help="repository root to scan")
    parser.add_argument("--json", action="store_true", help="emit findings as JSON")
    args = parser.parse_args(argv)

    root = args.root.resolve()
    findings = scan_repo(root)
    if args.json:
        print(json.dumps({"finding_count": len(findings), "findings": [f.as_dict() for f in findings]}, indent=2))
    elif findings:
        print(f"G002 fake-success static check: FAIL ({len(findings)} findings)")
        for finding in findings:
            print(finding.display())
    else:
        print("G002 fake-success static check: PASS (0 findings)")
    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
