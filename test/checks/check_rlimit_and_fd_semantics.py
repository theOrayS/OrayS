#!/usr/bin/env python3
"""Static regression guard for rlimit-and-fd rlimit/sysconf/fcntl/FD semantics.

The guard is intentionally narrow: it scans only the rlimit-and-fd surfaces called out by
team context and reports source patterns that can make unsupported or untracked
rlimit/sysconf/fcntl/FD-flag behavior look successful.  It is source-pattern
based, contains no LTP case/path/process-name knowledge, and is a gate rather
than a fixer.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import argparse
import json
import re

REPO_ROOT = Path(__file__).resolve().parents[2]
API_SYS_PATH = Path("api/arceos_posix_api/src/imp/sys.rs")
API_RESOURCES_PATH = Path("api/arceos_posix_api/src/imp/resources.rs")
API_FD_OPS_PATH = Path("api/arceos_posix_api/src/imp/fd_ops.rs")
SHELL_RESOURCE_PATH = Path("user/shell/src/uspace/resource_sched.rs")
SHELL_FD_TABLE_PATH = Path("user/shell/src/uspace/fd_table.rs")

FN_RE_TEMPLATE = r"\bfn\s+{name}\s*\("
MATCH_FALLBACK_OK_ZERO_RE = re.compile(r"_\s*=>\s*(?:return\s+)?Ok\(0\)")
MATCH_FALLBACK_RETURN_ZERO_RE = re.compile(r"_\s*=>\s*(?:return\s+)?0\b")
UNSUPPORTED_TEXT_RE = re.compile(r"unsupported|do not support|ignored|hard-code|hardcoded", re.I)
RETURN_OK_ZERO_RE = re.compile(r"(?:return\s+)?Ok\(0\)")
RETURN_ZERO_RE = re.compile(r"return\s+0\s*;")


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
    """Count Rust braces while ignoring braces in ordinary literals/comments."""

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
        if "{" in lines[idx]:
            saw_open = True
        depth += brace_delta(lines[idx])
        if saw_open and depth == 0:
            return RustFunction(name=name, start_line=start_idx + 1, text="\n".join(lines[start_idx : idx + 1]))
    return RustFunction(name=name, start_line=start_idx + 1, text="\n".join(lines[start_idx:]))


def missing_function_finding(rel: Path, name: str) -> Finding:
    return Finding(rel, 1, "rlimit-and-fd-missing-function", f"expected Rust function {name} is missing")


def nearby_ok_zero(lines: list[str], start: int, lookahead: int = 8) -> int | None:
    for idx in range(start + 1, min(len(lines), start + 1 + lookahead)):
        stripped = lines[idx].strip()
        if "Err(" in stripped or "neg_errno(" in stripped:
            return None
        if RETURN_OK_ZERO_RE.search(stripped) or RETURN_ZERO_RE.search(stripped):
            return idx
    return None


def branch_text(block: RustFunction, marker: str, max_lines: int = 10) -> tuple[int, str] | None:
    lines = block.text.splitlines()
    for idx, line in enumerate(lines):
        if marker not in line:
            continue
        chunk = []
        for offset in range(idx, min(len(lines), idx + max_lines)):
            if offset > idx and re.search(r"^\s*(?:ctypes::|general::|_)\w*", lines[offset]):
                break
            chunk.append(lines[offset])
        return block.start_line + idx, "\n".join(chunk)
    return None


def scan_sysconf(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    rel = repo_relative(path, root)
    block = extract_function(text, "sys_sysconf")
    if block is None:
        return [missing_function_finding(rel, "sys_sysconf")]

    findings: list[Finding] = []
    has_honest_fallback = False
    saw_fallback = False
    for line_no, line in block.iter_lines():
        stripped = line.strip()
        if re.search(r"_\s*=>", stripped):
            has_fallback_branch = True
        if MATCH_FALLBACK_OK_ZERO_RE.search(stripped):
            saw_fallback = True
            findings.append(
                Finding(
                    rel,
                    line_no,
                    "rlimit-and-fd-sysconf-fallback-ok-zero",
                    "unsupported sysconf names must fail honestly instead of returning 0",
                )
            )
        if re.search(r"_\s*=>\s*(?:return\s+)?Err\(", stripped):
            saw_fallback = True
            has_honest_fallback = True
    if not has_honest_fallback and not saw_fallback:
        findings.append(
            Finding(
                rel,
                block.start_line,
                "rlimit-and-fd-sysconf-missing-honest-fallback",
                "sys_sysconf should have an explicit errno fallback for unsupported names",
            )
        )
    return findings


def scan_api_resources(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    rel = repo_relative(path, root)
    findings: list[Finding] = []
    for name in ("sys_getrlimit", "sys_setrlimit"):
        block = extract_function(text, name)
        if block is None:
            findings.append(missing_function_finding(rel, name))
            continue
        has_invalid_resource_errno = False
        has_null_efault = "is_null()" in block.text and "LinuxError::EFAULT" in block.text
        lines = block.text.splitlines()
        for idx, line in enumerate(lines):
            line_no = block.start_line + idx
            stripped = line.strip()
            if re.search(r"_\s*=>\s*(?:return\s+)?Err", stripped):
                has_invalid_resource_errno = True
            if MATCH_FALLBACK_OK_ZERO_RE.search(stripped) or MATCH_FALLBACK_RETURN_ZERO_RE.search(stripped):
                findings.append(
                    Finding(
                        rel,
                        line_no,
                        "rlimit-and-fd-rlimit-fallback-success",
                        f"{name} fallback returns success for an unsupported resource",
                    )
                )
            if UNSUPPORTED_TEXT_RE.search(stripped):
                ok_idx = nearby_ok_zero(lines, idx)
                if ok_idx is not None:
                    findings.append(
                        Finding(
                            rel,
                            block.start_line + ok_idx,
                            "rlimit-and-fd-rlimit-unsupported-ok-zero",
                            f"{name} unsupported path returns success instead of errno",
                        )
                    )
        if not has_invalid_resource_errno:
            findings.append(
                Finding(
                    rel,
                    block.start_line,
                    "rlimit-and-fd-rlimit-missing-invalid-resource-errno",
                    f"{name} should reject unsupported resources with errno",
                )
            )
        if not has_null_efault:
            findings.append(
                Finding(
                    rel,
                    block.start_line,
                    "rlimit-and-fd-rlimit-missing-efault-boundary",
                    f"{name} should preserve null user-pointer EFAULT behavior",
                )
            )
    return findings


def scan_api_fd_ops(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    rel = repo_relative(path, root)
    findings: list[Finding] = []

    for idx, line in enumerate(text.splitlines(), start=1):
        if "fn status_flags" in line:
            block = extract_function("\n".join(text.splitlines()[idx - 1 :]), "status_flags")
            if block and "Ok(0)" in block.text:
                findings.append(
                    Finding(
                        rel,
                        idx,
                        "rlimit-and-fd-api-filelike-default-status-ok-zero",
                        "FileLike::status_flags default reports success without per-FD status state",
                    )
                )
            break

    block = extract_function(text, "sys_fcntl")
    if block is None:
        findings.append(missing_function_finding(rel, "sys_fcntl"))
        return findings

    for cmd in ("ctypes::F_GETFD", "ctypes::F_SETFD"):
        if cmd not in block.text:
            findings.append(
                Finding(
                    rel,
                    block.start_line,
                    "rlimit-and-fd-api-fcntl-missing-fd-command",
                    f"sys_fcntl must implement {cmd} using readable FD flag state",
                )
            )

    dup_cloexec = branch_text(block, "ctypes::F_DUPFD_CLOEXEC")
    if dup_cloexec is not None:
        line_no, chunk = dup_cloexec
        chunk_without_cmd_name = chunk.replace("F_DUPFD_CLOEXEC", "")
        if "dup_fd(fd)" in chunk and "FD_CLOEXEC" not in chunk_without_cmd_name:
            findings.append(
                Finding(
                    rel,
                    line_no,
                    "rlimit-and-fd-api-dupfd-cloexec-drops-flag",
                    "F_DUPFD_CLOEXEC duplicates without setting FD_CLOEXEC state",
                )
            )

    lines = block.text.splitlines()
    for idx, line in enumerate(lines):
        line_no = block.start_line + idx
        stripped = line.strip()
        if "fd == 0" in stripped and "fd == 1" in stripped and "fd == 2" in stripped:
            ok_idx = nearby_ok_zero(lines, idx, lookahead=4)
            if ok_idx is not None:
                findings.append(
                    Finding(
                        rel,
                        block.start_line + ok_idx,
                        "rlimit-and-fd-api-fcntl-stdio-hardcoded-ok-zero",
                        "stdio fcntl flags should be represented as FD/status state, not hard-coded success",
                    )
                )
        if UNSUPPORTED_TEXT_RE.search(stripped):
            ok_idx = nearby_ok_zero(lines, idx)
            if ok_idx is not None:
                findings.append(
                    Finding(
                        rel,
                        block.start_line + ok_idx,
                        "rlimit-and-fd-api-fcntl-unsupported-ok-zero",
                        "unsupported fcntl command returns success instead of errno",
                    )
                )
        if MATCH_FALLBACK_OK_ZERO_RE.search(stripped):
            findings.append(
                Finding(
                    rel,
                    line_no,
                    "rlimit-and-fd-api-fcntl-fallback-ok-zero",
                    "fcntl catch-all fallback returns Ok(0) for unsupported input",
                )
            )
    return findings


def scan_shell_resource(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    rel = repo_relative(path, root)
    block = extract_function(text, "sys_prlimit64")
    if block is None:
        return [missing_function_finding(rel, "sys_prlimit64")]

    findings: list[Finding] = []
    if "resource_is_valid" not in block.text or "LinuxError::EINVAL" not in block.text:
        findings.append(
            Finding(
                rel,
                block.start_line,
                "rlimit-and-fd-prlimit-missing-invalid-resource-check",
                "sys_prlimit64 should reject invalid resources with EINVAL",
            )
        )
    lines = block.text.splitlines()
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if "resource_is_valid" in stripped:
            ok_idx = nearby_ok_zero(lines, idx)
            if ok_idx is not None:
                findings.append(
                    Finding(
                        rel,
                        block.start_line + ok_idx,
                        "rlimit-and-fd-prlimit-invalid-resource-success",
                        "invalid prlimit resource path returns success instead of errno",
                    )
                )
    return findings


def scan_shell_fd_table(path: Path, text: str, root: Path = REPO_ROOT) -> list[Finding]:
    rel = repo_relative(path, root)
    findings: list[Finding] = []

    if re.search(r"const\s+FD_TABLE_LIMIT\s*:\s*usize\s*=\s*DEFAULT_NOFILE_LIMIT\s+as\s+usize", text):
        findings.append(
            Finding(
                rel,
                1,
                "rlimit-and-fd-fd-table-limit-default-nofile",
                "FdTable capacity must not be capped at the default soft RLIMIT_NOFILE value",
            )
        )
    if "DEFAULT_NOFILE_LIMIT" in text:
        findings.append(
            Finding(
                rel,
                1,
                "rlimit-and-fd-fd-table-imports-default-nofile",
                "FdTable should use NR_OPEN_LIMIT for physical capacity and enforce RLIMIT_NOFILE separately",
            )
        )
    if "const FD_TABLE_LIMIT: usize = NR_OPEN_LIMIT as usize" not in text:
        findings.append(
            Finding(
                rel,
                1,
                "rlimit-and-fd-fd-table-limit-missing-nr-open",
                "FdTable capacity should be backed by NR_OPEN_LIMIT, not the default soft limit",
            )
        )

    limit_block = extract_function(text, "current_fd_table_limit")
    if limit_block is None:
        findings.append(missing_function_finding(rel, "current_fd_table_limit"))
    else:
        for token, detail in (
            ("RLIMIT_NOFILE_RESOURCE", "current_fd_table_limit must read the per-process soft RLIMIT_NOFILE"),
            ("soft_limit.min(FD_TABLE_LIMIT as u64)", "current_fd_table_limit must clamp the soft limit to NR_OPEN capacity"),
            ("cmp::min(", "current_fd_table_limit must enforce both soft and physical limits"),
        ):
            if token not in limit_block.text:
                findings.append(Finding(rel, limit_block.start_line, "rlimit-and-fd-fd-soft-limit-clamp-missing", detail))

    required_terms = {
        "fd_flags": "FdTable should carry per-descriptor FD_CLOEXEC state",
        "get_fd_flags": "F_GETFD should read per-descriptor FD flags",
        "set_fd_flags": "F_SETFD should update per-descriptor FD flags",
        "insert_with_flags": "FD creation should preserve requested FD flags",
    }
    for term, detail in required_terms.items():
        if term not in text:
            findings.append(Finding(rel, 1, "rlimit-and-fd-fd-state-missing", detail))

    stdio_status_terms = {
        "Stdin(u32)": "stdin should carry status flags so F_GETFL/F_SETFL report real fd state",
        "Stdout(u32)": "stdout should carry status flags so F_GETFL/F_SETFL report real fd state",
        "Stderr(u32)": "stderr should carry status flags so F_GETFL/F_SETFL report real fd state",
    }
    for term, detail in stdio_status_terms.items():
        if term not in text:
            findings.append(Finding(rel, 1, "rlimit-and-fd-stdio-status-state-missing", detail))

    block = extract_function(text, "fcntl")
    if block is None:
        findings.append(missing_function_finding(rel, "fcntl"))
        return findings

    for cmd, handler in (
        ("general::F_GETFD", "get_fd_flags"),
        ("general::F_SETFD", "set_fd_flags"),
    ):
        if cmd not in block.text or handler not in block.text:
            findings.append(
                Finding(
                    rel,
                    block.start_line,
                    "rlimit-and-fd-shell-fcntl-missing-fd-command",
                    f"FdTable::fcntl must route {cmd} to {handler}",
                )
            )

    for variant in ("FdEntry::Stdin(status_flags)", "FdEntry::Stdout(status_flags)", "FdEntry::Stderr(status_flags)"):
        if variant not in block.text:
            findings.append(
                Finding(
                    rel,
                    block.start_line,
                    "rlimit-and-fd-shell-fcntl-stdio-status-missing",
                    "FdTable::fcntl must route stdio F_GETFL/F_SETFL through status_flags state",
                )
            )
            break
    if "fcntl_setfl_flags(arg as u32)" not in block.text:
        findings.append(
            Finding(
                rel,
                block.start_line,
                "rlimit-and-fd-shell-fcntl-setfl-mask-missing",
                "F_SETFL must mask to mutable Linux status flags instead of accepting arbitrary bits",
            )
        )

    for line_no, line in block.iter_lines():
        stripped = line.strip()
        if MATCH_FALLBACK_OK_ZERO_RE.search(stripped):
            findings.append(
                Finding(
                    rel,
                    line_no,
                    "rlimit-and-fd-shell-fcntl-fallback-ok-zero",
                    "FdTable::fcntl catch-all/unsupported branch returns Ok(0)",
                )
            )
    return findings


def scan_repo(root: Path = REPO_ROOT) -> list[Finding]:
    root = root.resolve()
    scanners = (
        (API_SYS_PATH, scan_sysconf),
        (API_RESOURCES_PATH, scan_api_resources),
        (API_FD_OPS_PATH, scan_api_fd_ops),
        (SHELL_RESOURCE_PATH, scan_shell_resource),
        (SHELL_FD_TABLE_PATH, scan_shell_fd_table),
    )
    findings: list[Finding] = []
    for rel_path, scanner in scanners:
        path = root / rel_path
        if path.exists():
            findings.extend(scanner(path, path.read_text(), root))
        else:
            findings.append(Finding(rel_path, 1, "rlimit-and-fd-missing-file", "expected rlimit-and-fd surface file is missing"))
    return sorted(findings, key=lambda f: (f.path.as_posix(), f.line, f.kind))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=REPO_ROOT, help="repository root to scan")
    parser.add_argument("--json", action="store_true", help="emit findings as JSON")
    args = parser.parse_args(argv)

    findings = scan_repo(args.root.resolve())
    if args.json:
        print(json.dumps({"finding_count": len(findings), "findings": [f.as_dict() for f in findings]}, indent=2))
    elif findings:
        print(f"rlimit-and-fd rlimit/sysconf/fcntl/FD static check: FAIL ({len(findings)} findings)")
        for finding in findings:
            print(finding.display())
    else:
        print("rlimit-and-fd rlimit/sysconf/fcntl/FD static check: PASS (0 findings)")
    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
