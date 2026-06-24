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


def braced_block(text: str, brace_pos: int) -> tuple[str, int]:
    if brace_pos < 0 or brace_pos >= len(text) or text[brace_pos] != "{":
        return "", brace_pos
    depth = 0
    for pos in range(brace_pos, len(text)):
        ch = text[pos]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[brace_pos + 1 : pos], pos + 1
    return text[brace_pos + 1 :], len(text)


def function_body(text: str, name: str) -> str:
    marker = f"fn {name}"
    start = text.find(marker)
    if start < 0:
        return ""
    brace = text.find("{", start)
    body, _ = braced_block(text, brace)
    return body


def rust_function_names(text: str) -> set[str]:
    return set(
        re.findall(
            r"(?:^|\n)\s*(?:pub(?:\([^)]*\))?\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(",
            text,
        )
    )


def scan_busybox_runtime_boundary(root: Path) -> list[str]:
    findings: list[str] = []
    cmd = read(root / "examples/shell/src/cmd.rs")
    runtime_wrappers = function_block(cmd, "ensure_runtime_busybox_wrappers")
    suite_runtime_wrappers = function_block(cmd, "prepare_suite_runtime_busybox_wrappers")
    autorun_body = function_body(cmd, "maybe_run_official_tests")
    busybox_runner = function_body(cmd, "run_busybox_suite")
    ltp_runner = function_body(cmd, "run_ltp_suite")
    for token in (
        "ensure_busybox_applet_wrappers",
        "ensure_runtime_busybox_wrappers",
        "prepare_suite_runtime_busybox_wrappers",
        "PATH_BUSYBOX_APPLETS",
        "LTP_BUSYBOX_APPLETS",
    ):
        if token not in cmd:
            findings.append(f"examples/shell/src/cmd.rs: missing filesystem-visible busybox wrapper support token {token}")
    if not runtime_wrappers:
        findings.append("examples/shell/src/cmd.rs: missing ensure_runtime_busybox_wrappers")
    else:
        wrapper_dir_loop = (
            'for dir in [suite_dir, "/bin", "/usr/bin"]' in runtime_wrappers
            or 'for dir in ["/bin", "/usr/bin", suite_dir]' in runtime_wrappers
        )
        required_wrapper_tokens = (
            "ensure_busybox_applet_wrappers",
            "LTP_BUSYBOX_APPLETS",
        )
        if not wrapper_dir_loop:
            findings.append(
                "examples/shell/src/cmd.rs: runtime busybox support must attempt real wrapper files for suite/bin/usr-bin paths"
            )
        for token in required_wrapper_tokens:
            if token not in runtime_wrappers:
                findings.append(
                    "examples/shell/src/cmd.rs: runtime busybox support must create real wrapper files for suite/bin/usr-bin paths"
                )
                break
    if not suite_runtime_wrappers:
        findings.append("examples/shell/src/cmd.rs: missing suite-level runtime busybox wrapper preparation helper")
    elif (
        'let suite_busybox = join_path(suite_dir, "busybox")' not in suite_runtime_wrappers
        or "ltp_helper_busybox_path(suite_dir, &suite_busybox)" not in suite_runtime_wrappers
        or "ensure_runtime_busybox_wrappers(suite_dir, &wrapper_busybox)" not in suite_runtime_wrappers
    ):
        findings.append(
            "examples/shell/src/cmd.rs: suite-level wrapper preparation must derive the suite busybox and create real runtime wrapper files"
        )
    if not autorun_body or "ensure_runtime_busybox_wrappers(suite_dir, &wrapper_busybox)" not in autorun_body:
        if "prepare_suite_runtime_busybox_wrappers(suite_dir)" not in autorun_body:
            findings.append("examples/shell/src/cmd.rs: official autorun must prepare real busybox wrapper files before executing suites")
    for runner_name, runner_body in (("run_busybox_suite", busybox_runner), ("run_ltp_suite", ltp_runner)):
        if not runner_body:
            findings.append(f"examples/shell/src/cmd.rs: missing {runner_name}")
        elif "prepare_suite_runtime_busybox_wrappers(suite_dir)" not in runner_body:
            findings.append(
                f"examples/shell/src/cmd.rs: {runner_name} must enforce real busybox wrapper preparation instead of relying on outer autorun order"
            )

    forbidden_uspace_tokens = {
        "busybox_applet_target_path": "runtime path/VFS layer must not rewrite applet paths to busybox",
        "is_busybox_applet_name": "kernel runtime must not classify applet names for hidden fallback",
        "append_busybox_applet_alias_candidates": "openat path candidate generation must not add busybox alias fallbacks",
        "busybox_applet_alias_allowed": "openat path candidate generation must not special-case busybox applets",
        "resolve_execve_compat_path": "execve must not rewrite missing /bin or suite applets to busybox",
        "busybox_exec_alias_target": "execve must not synthesize busybox targets for missing paths",
        "existing_busybox_for_exec_root": "execve must not fall back to suite-root busybox invisibly",
        "standard_bin_busybox_applet_name": "execve must not special-case /bin or /usr/bin applet names",
        "rooted_busybox_applet_name": "execve must not special-case /musl or /glibc applet names",
        "find_busybox_for_script": "script loader must not replace missing interpreters with busybox",
        "push_busybox_applet_candidate": "runtime path/VFS layer must not add hidden busybox applet fallbacks",
        "busybox_applet_supported": "runtime path/VFS layer must not classify applet names for hidden fallback",
    }
    for rel in (
        Path("examples/shell/src/uspace/runtime_paths.rs"),
        Path("examples/shell/src/uspace/process_lifecycle.rs"),
        Path("examples/shell/src/uspace/fd_table.rs"),
        Path("examples/shell/src/uspace/program_loader.rs"),
    ):
        text = read(root / rel)
        for token, detail in forbidden_uspace_tokens.items():
            if token in text:
                findings.append(f"{rel}: forbidden busybox magic token {token}: {detail}")
    return findings


def scan_cmd_rs(root: Path) -> list[str]:
    text = read(root / "examples/shell/src/cmd.rs")
    findings: list[str] = []
    env_block = function_block(text, "ltp_case_env")
    autorun_body = function_body(text, "maybe_run_official_tests")
    run_dir_block = function_block(text, "prepare_ltp_case_run_dir")
    helper_cases_block = function_block(text, "ltp_resource_helper_cases")
    copy_block = function_block(text, "copy_script_file")
    unstaged_block = function_block(text, "prepare_unstaged_script_dir")
    forbidden_runner_rewrite_tokens = [
        "rewrite_iperf_daemon_server",
        "rewrite_netperf_daemon_server",
        "restore_unixbench_sort_fixture",
        "normalize_lmbench_stage_wrappers",
        "prepare_libctest_runtest_wrapper",
        "rewrite_libctest_run_script",
        "rewrite_libctest_command",
        "rewrite_ltp_case_line",
        "wrap_ltp_cases",
        "iperf_testcode.sh",
        "netperf_testcode.sh",
        "UNIXBENCH_SORT_SRC",
    ]
    function_names = rust_function_names(text)
    forbidden_functions = {
        token for token in forbidden_runner_rewrite_tokens if token.startswith(("rewrite_", "restore_", "normalize_", "prepare_"))
    }
    for function_name in sorted(function_names & forbidden_functions):
        findings.append(
            f"examples/shell/src/cmd.rs: forbidden suite/script-specific helper still defined: {function_name}"
        )
    for token in forbidden_runner_rewrite_tokens:
        if token in text:
            findings.append(f"examples/shell/src/cmd.rs: suite/script-specific rewrite token is forbidden: {token}")
    if re.search(r"\|\|\s*line\s*==\s*\"", text) or re.search(r"line\s*==\s*\"[^\"]+\"", text):
        findings.append("examples/shell/src/cmd.rs: runner success must not special-case literal command lines")
    if "skip unscored test group" in text or "musl-only" in text:
        findings.append("examples/shell/src/cmd.rs: runner must not skip official groups based on score-only/musl-only policy")
    if re.search(r'group\s*==\s*"libctest"\s*&&\s*suite_dir\s*!=\s*"/musl"', text):
        findings.append("examples/shell/src/cmd.rs: libctest must run for every discovered libc suite instead of score-aware skipping")
    if not autorun_body:
        findings.append("examples/shell/src/cmd.rs: missing maybe_run_official_tests")
    else:
        if 'if group == "libctest"' not in autorun_body or "run_libctest_suite(&suite_dir, &cwd)" not in autorun_body:
            findings.append("examples/shell/src/cmd.rs: libctest dispatch must run the generic libctest suite for each discovered suite directory")
        if "available_groups: BTreeSet<String>" not in autorun_body or "missing_groups" not in autorun_body:
            findings.append("examples/shell/src/cmd.rs: selected official groups must be checked against discovered scripts")
        if "if !missing_groups.is_empty() || !disabled_groups.is_empty()" not in autorun_body:
            findings.append("examples/shell/src/cmd.rs: unknown or disabled selected official groups must fail visibly")
        if "official test group filter matched no available groups" not in autorun_body:
            findings.append("examples/shell/src/cmd.rs: selected official groups must fail if no scripts are available")
        suite_dir_policy = re.compile(
            r'(?:suite_dir(?:\.as_str\(\))?\s*(?:==|!=)\s*"/(?:musl|glibc)")|'
            r'(?:"/(?:musl|glibc)"\s*(?:==|!=)\s*suite_dir(?:\.as_str\(\))?)'
        )
        score_policy = re.compile(r"\b(?:unscored|score-aware|score-only|musl-only)\b", re.IGNORECASE)
        autorun_lines = autorun_body.splitlines()
        for idx, line in enumerate(autorun_lines):
            if "if " not in line or "libctest" not in line:
                continue
            window = "\n".join(autorun_lines[idx : idx + 10])
            if "continue;" in window and "run_libctest_suite" not in window:
                findings.append("examples/shell/src/cmd.rs: libctest dispatch must not conditionally continue/skip discovered suites")
            if suite_dir_policy.search(window):
                findings.append("examples/shell/src/cmd.rs: libctest dispatch must not branch on fixed musl/glibc suite directory policy")
            if score_policy.search(window):
                findings.append("examples/shell/src/cmd.rs: libctest dispatch must not encode score-only or musl-only policy")
    if not copy_block:
        findings.append("examples/shell/src/cmd.rs: missing copy_script_file")
    elif (
        "src.ends_with" in copy_block
        or '"$file"' in copy_block
        or "rewrite_ltp_case_line" in copy_block
        or copy_block.count("rewrite_script_line") != 1
    ):
        findings.append("examples/shell/src/cmd.rs: copy_script_file must stay generic and must not branch on script names or LTP $file patterns")
    if unstaged_block and 'group == "ltp"' in unstaged_block:
        findings.append("examples/shell/src/cmd.rs: unstaged script preparation must not inject LTP-only rewrites")
    if not env_block:
        findings.append("examples/shell/src/cmd.rs: missing ltp_case_env")
    else:
        if '"chdir01"' in env_block or "chdir01" in env_block:
            findings.append("examples/shell/src/cmd.rs: ltp_case_env must not special-case chdir01")
        if "needs_case_resource_helper" not in env_block:
            findings.append("examples/shell/src/cmd.rs: ltp_case_env must be driven by generic helper/device capability")
        if "LTP_SINGLE_FS_TYPE=tmpfs" not in env_block or "LTP_DEV_FS_TYPE=tmpfs" not in env_block:
            findings.append("examples/shell/src/cmd.rs: helper-backed filesystem env boundary is missing")
    if not run_dir_block:
        findings.append("examples/shell/src/cmd.rs: missing prepare_ltp_case_run_dir")
    elif "needs_case_resource_helper" not in run_dir_block:
        findings.append("examples/shell/src/cmd.rs: run-dir selection must reuse generic helper detection")
    if not helper_cases_block:
        findings.append("examples/shell/src/cmd.rs: missing ltp_resource_helper_cases")
    else:
        if "split_once('_')" in helper_cases_block or 'split_once("_")' in helper_cases_block:
            findings.append(
                "examples/shell/src/cmd.rs: ltp_resource_helper_cases must not split helper names at the first underscore; underscore case names need selected-case prefix matching"
            )
        if "strip_prefix(case" not in helper_cases_block or "helper_suffix.starts_with('_')" not in helper_cases_block:
            findings.append(
                "examples/shell/src/cmd.rs: ltp_resource_helper_cases must preserve the generic {case}_ helper boundary for selected cases"
            )
    if "PASS LTP CASE" in text:
        findings.append("examples/shell/src/cmd.rs: runner must not emit PASS LTP CASE wrapper records that can hide later TCONF/TBROK/TFAIL/timeout evidence")
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
        "test_promotion_mode_boundary_allows_stable_file_inline_batch_core_and_blocks_sweep",
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
    findings.extend(scan_busybox_runtime_boundary(root))
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
