#!/usr/bin/env python3
"""Static regression guard for stat-metadata stat/lstat metadata semantics.

The guard is intentionally narrow: it scans the POSIX API VFS surface that stat-metadata
is changing and reports source patterns that can make stat/lstat appear to
succeed without real metadata semantics.  It is a gate, not a fixer.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import argparse
import json
import re

REPO_ROOT = Path(__file__).resolve().parents[2]
FS_RS_PATH = Path("api/arceos_posix_api/src/imp/fs.rs")

FN_RE_TEMPLATE = r"\bfn\s+{name}\s*\("
STAT_DEFAULT_RE = re.compile(
    r"(?:\bctypes::stat::default\s*\(|\bstat::default\s*\(|\bmem::zeroed\s*\(|\bDefault::default\s*\()"
)
STAT_OPEN_RE = re.compile(r"\b(?:axfs::fops::)?File::open\s*\(")
API_METADATA_RE = re.compile(r"\baxfs::api::metadata\s*\(")

LSTAT_EXPLICIT_BOUNDARY_TERMS = (
    "symlink_metadata",
    "metadata_no_follow",
    "metadata_nofollow",
    "lstat_metadata",
    "path_symlink_stat",
    "read_link",
    "readlink",
    "O_NOFOLLOW",
    "NOFOLLOW",
    "no_follow",
    "nofollow",
    "no-follow",
)
LSTAT_HONEST_ERROR_TERMS = (
    "LinuxError::ENOSYS",
    "LinuxError::EOPNOTSUPP",
    "LinuxError::ELOOP",
    "Err(LinuxError::ENOSYS)",
    "Err(LinuxError::EOPNOTSUPP)",
    "Err(LinuxError::ELOOP)",
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


@dataclass(frozen=True)
class RustFunction:
    name: str
    start_line: int
    text: str

    def iter_lines(self):
        for offset, line in enumerate(self.text.splitlines()):
            yield self.start_line + offset, line


def repo_relative(path: Path, root: Path) -> Path:
    try:
        return path.relative_to(root)
    except ValueError:
        return path


def brace_delta(line: str) -> int:
    """Count Rust brace balance for ordinary source lines.

    This intentionally ignores braces inside double-quoted or single-quoted
    literals so debug format strings do not break function extraction.
    """

    delta = 0
    in_string = False
    in_char = False
    escaped = False
    idx = 0
    while idx < len(line):
        ch = line[idx]
        nxt = line[idx + 1] if idx + 1 < len(line) else ""
        if not in_string and not in_char and ch == "/" and nxt == "/":
            break
        if escaped:
            escaped = False
        elif ch == "\\" and (in_string or in_char):
            escaped = True
        elif ch == '"' and not in_char:
            in_string = not in_string
        elif ch == "'" and not in_string:
            in_char = not in_char
        elif not in_string and not in_char:
            if ch == "{":
                delta += 1
            elif ch == "}":
                delta -= 1
        idx += 1
    return delta


def extract_function(text: str, name: str) -> RustFunction | None:
    lines = text.splitlines()
    fn_re = re.compile(FN_RE_TEMPLATE.format(name=re.escape(name)))
    start_idx = next((idx for idx, line in enumerate(lines) if fn_re.search(line)), None)
    if start_idx is None:
        return None

    depth = 0
    saw_open = False
    for idx in range(start_idx, len(lines)):
        line_delta = brace_delta(lines[idx])
        if "{" in lines[idx]:
            saw_open = True
        depth += line_delta
        if saw_open and depth == 0:
            return RustFunction(name=name, start_line=start_idx + 1, text="\n".join(lines[start_idx : idx + 1]))
    return RustFunction(name=name, start_line=start_idx + 1, text="\n".join(lines[start_idx:]))


def first_matching_line(block: RustFunction, regex: re.Pattern[str]) -> int | None:
    for line_no, line in block.iter_lines():
        if regex.search(line):
            return line_no
    return None


def default_stat_line(block: RustFunction) -> int | None:
    for line_no, line in block.iter_lines():
        stripped = line.strip()
        if stripped.startswith("..Default::default"):
            continue
        if STAT_DEFAULT_RE.search(stripped):
            return line_no
    return None


def lstat_has_explicit_boundary(block: RustFunction) -> bool:
    body = block.text
    if any(term in body for term in LSTAT_EXPLICIT_BOUNDARY_TERMS):
        return True
    body_lower = body.lower()
    return "symlink" in body_lower and any(term in body for term in LSTAT_HONEST_ERROR_TERMS)


def scan_fs_rs(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    rel = repo_relative(path, root)
    findings: list[Finding] = []
    sys_stat = extract_function(text, "sys_stat")
    sys_lstat = extract_function(text, "sys_lstat")

    for block in (sys_stat, sys_lstat):
        if block is None:
            continue
        if default_line := default_stat_line(block):
            findings.append(
                Finding(
                    rel,
                    default_line,
                    "stat-metadata-default-stat-success-risk",
                    f"{block.name} builds a default/zeroed stat value on a success path",
                )
            )

    if sys_stat is not None:
        if open_line := first_matching_line(sys_stat, STAT_OPEN_RE):
            findings.append(
                Finding(
                    rel,
                    open_line,
                    "stat-metadata-stat-via-read-open",
                    "sys_stat obtains path metadata by opening the file, which can reject directories or unreadable files before stat semantics run",
                )
            )

    if sys_lstat is not None:
        for line_no, line in sys_lstat.iter_lines():
            if re.search(r"\bsys_stat\s*\(", line):
                findings.append(
                    Finding(
                        rel,
                        line_no,
                        "stat-metadata-lstat-delegates-to-stat",
                        "sys_lstat delegates to stat/follow semantics instead of a no-follow boundary or honest unsupported error",
                    )
                )
                break
        if (first_matching_line(sys_lstat, API_METADATA_RE) or first_matching_line(sys_lstat, STAT_OPEN_RE)) and not lstat_has_explicit_boundary(sys_lstat):
            metadata_line = first_matching_line(sys_lstat, API_METADATA_RE) or first_matching_line(sys_lstat, STAT_OPEN_RE)
            findings.append(
                Finding(
                    rel,
                    metadata_line or sys_lstat.start_line,
                    "stat-metadata-lstat-without-nofollow-boundary",
                    "sys_lstat returns ordinary path metadata without an explicit symlink no-follow boundary or honest unsupported errno",
                )
            )

    return findings


def scan_repo(root: Path = REPO_ROOT) -> list[Finding]:
    path = root / FS_RS_PATH
    if not path.exists():
        return [
            Finding(
                FS_RS_PATH,
                1,
                "stat-metadata-missing-fs-rs",
                "expected POSIX API VFS implementation file is missing",
            )
        ]
    return sorted(scan_fs_rs(path, path.read_text(), root), key=lambda f: (f.path.as_posix(), f.line, f.kind))


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
        print(f"stat-metadata stat metadata static check: FAIL ({len(findings)} findings)")
        for finding in findings:
            print(finding.display())
    else:
        print("stat-metadata stat metadata static check: PASS (0 findings)")
    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
