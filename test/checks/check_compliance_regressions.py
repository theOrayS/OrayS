#!/usr/bin/env python3
"""Static guard for compliance regression regression.

This guard intentionally checks source semantics, not testcase names.  It
prevents reintroducing fixed fake kernel capabilities, silent runner fallbacks,
and libc/API wrappers that report success while dropping errno or user-pointer
validation.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def read(root: Path, rel: str) -> str:
    return (root / rel).read_text(encoding="utf-8", errors="ignore")


def function_block(text: str, name: str) -> str:
    marker = re.search(rf"\b(?:pub\s+)?(?:unsafe\s+)?fn\s+{re.escape(name)}(?:\s*<[^>]*>)?\s*\(", text)
    if not marker:
        marker = re.search(rf"\b(?:int|mode_t|FILE\s*\*)\s+{re.escape(name)}\s*\(", text)
    if not marker:
        return ""
    brace = text.find("{", marker.start())
    if brace < 0:
        return ""
    depth = 0
    in_string = False
    in_char = False
    escaped = False
    for pos in range(brace, len(text)):
        ch = text[pos]
        if escaped:
            escaped = False
            continue
        if ch == "\\" and (in_string or in_char):
            escaped = True
            continue
        if ch == '"' and not in_char:
            in_string = not in_string
            continue
        if ch == "'" and not in_string:
            in_char = not in_char
            continue
        if in_string or in_char:
            continue
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[marker.start() : pos + 1]
    return text[marker.start() :]


def require(condition: bool, findings: list[str], detail: str) -> None:
    if not condition:
        findings.append(detail)


def scan(root: Path) -> list[str]:
    findings: list[str] = []

    cargo_toml = read(root, "Cargo.toml")
    require(
        'axfs_vfs = { path = "vendor/axfs_vfs" }' in cargo_toml,
        findings,
        "workspace must patch axfs_vfs to the tracked audited vendored source when extending VfsNodeOps",
    )
    require(
        "vendor/cargo/axfs_vfs" not in cargo_toml,
        findings,
        "workspace must not depend on ignored vendor/cargo/axfs_vfs for audited VfsNodeOps changes",
    )

    mounts = read(root, "kernel/fs/axfs/src/mounts.rs")
    for token, detail in [
        ('foo_dir.add("bar"', "devfs must not register synthetic /dev/foo/bar"),
        ("proc_self_stat", "procfs must not expose fixed /proc/self/stat"),
        ("current_clocksource", "sysfs must not expose fixed current_clocksource text"),
        ("tsc\\n", "sysfs must not hard-code x86 tsc clocksource"),
        ("always [madvise] never", "sysfs must not hard-code THP capability"),
        ("overcommit_memory", "procfs must not hard-code VM overcommit capability"),
        ("pipe-max-size", "procfs must not hard-code pipe limit capability"),
        ("somaxconn", "procfs must not hard-code network tunable capability"),
    ]:
        require(token not in mounts, findings, detail)

    cmd = read(root, "user/shell/src/cmd.rs")
    selected_ltp = function_block(cmd, "selected_ltp_cases")
    run_ltp = function_block(cmd, "run_ltp_suite")
    official = function_block(cmd, "selected_official_test_groups")
    maybe = function_block(cmd, "maybe_run_official_tests")
    require("Result<Option<Vec<String>>, String>" in official, findings, "official group filter must return Result, not collapse errors to all")
    require("invalid official test group filter" in official or "official test group filter did not contain" in official, findings, "official group filter must preserve malformed/empty errors")
    require("FAIL OFFICIAL TEST GROUP FILTER" in maybe and "std::process::exit(1)" in maybe, findings, "malformed official group filter must be visible and fail nonzero")
    require("available_groups: BTreeSet<String>" in maybe and "missing_groups" in maybe and "unknown official test groups" in maybe, findings, "selected official groups must be validated against discovered script groups")
    require("if !missing_groups.is_empty() || !disabled_groups.is_empty()" in maybe, findings, "unknown or disabled selected official groups must fail visibly")
    require("official test group filter matched no available groups" in maybe and "if selected_groups.is_some()" in maybe, findings, "selected official groups must fail visibly when no script can run")
    require("invalid LTP_CASES selection" in selected_ltp and "String::from(\"core\")" not in selected_ltp.split("invalid LTP_CASES selection")[-1], findings, "malformed LTP_CASES must not fall back to core")
    require("LTP_CASES selection did not contain any cases" in selected_ltp, findings, "explicit empty LTP_CASES must fail")
    start_pos = run_ltp.find("OS COMP TEST GROUP START")
    setup_pos = min([p for p in [run_ltp.find("prepare_suite_runtime_busybox_wrappers"), run_ltp.find("selected_ltp_cases")] if p >= 0], default=-1)
    require(start_pos >= 0 and setup_pos >= 0 and start_pos < setup_pos, findings, "LTP group START must be printed before setup/selection can fail")
    for token in ("FAIL LTP SETUP", "ltp setup failed", "ltp cases: 0 passed, 1 failed, 0 timed out", "OS COMP TEST GROUP END"):
        require(token in run_ltp, findings, f"LTP setup/selection failure path missing {token}")
    require("PASS LTP CASE" not in cmd, findings, "runner must not synthesize PASS LTP CASE")

    program_loader = read(root, "user/shell/src/uspace/program_loader.rs")
    effective_exec_root = function_block(program_loader, "effective_exec_root")
    require(
        "ld-linux-" not in effective_exec_root
        and "ld-musl-" not in effective_exec_root
        and "exec_loader_owned_string(path_root" in effective_exec_root,
        findings,
        "a rootfs ELF must retain its standard runtime root instead of being remapped by interpreter basename",
    )

    runtime_paths = read(root, "user/shell/src/uspace/runtime_paths.rs")
    runtime_roots = function_block(runtime_paths, "runtime_root_candidates")
    try_runtime_roots = function_block(runtime_paths, "try_runtime_root_candidates")
    standard_root = runtime_roots.find('if exec_root == "/"')
    glibc_fallback = runtime_roots.find("is_glibc_runtime_name")
    require(
        standard_root >= 0
        and glibc_fallback >= 0
        and standard_root < glibc_fallback
        and "push(exec_root)" in runtime_roots[standard_root:glibc_fallback],
        findings,
        "bare runtime SONAME lookup must search the standard root before compatibility roots",
    )
    try_standard_root = try_runtime_roots.find('if exec_root == "/"')
    try_glibc_fallback = try_runtime_roots.find("is_glibc_runtime_name")
    require(
        try_standard_root >= 0
        and try_glibc_fallback >= 0
        and try_standard_root < try_glibc_fallback
        and "try_push_candidate_from_str" in try_runtime_roots[
            try_standard_root:try_glibc_fallback
        ],
        findings,
        "fallible runtime SONAME lookup must search the standard root before compatibility roots",
    )

    utils = read(root, "api/arceos_posix_api/src/utils.rs")
    for token in ("validate_user_range", "checked_mul", "checked_add", "align_of", "user_ref"):
        require(token in utils, findings, f"user pointer helpers missing {token}")
    mutex = read(root, "api/arceos_posix_api/src/imp/pthread/mutex.rs")
    from_user = function_block(mutex, "from_user")
    require("user_ref" in from_user and "&*mutex.cast" not in from_user, findings, "pthread mutex must borrow through shared user_ref helper")

    task = read(root, "api/arceos_posix_api/src/imp/task.rs")
    getpid = function_block(task, "sys_getpid")
    require("current_may_uninit" not in getpid and ".id()" not in getpid, findings, "native getpid must not expose per-thread scheduler id")

    net = read(root, "api/arceos_posix_api/src/imp/net.rs")
    sendto = function_block(net, "sys_sendto")
    tcp_pos = sendto.find("Socket::Tcp")
    validate_pos = sendto.find("from_sockaddr", tcp_pos)
    send_pos = sendto.find("socket.send(buf)", tcp_pos)
    require(tcp_pos >= 0 and validate_pos >= 0 and send_pos >= 0 and validate_pos < send_pos, findings, "TCP sendto must validate non-null destination before stream send")

    api_stdio = read(root, "api/arceos_posix_api/src/imp/stdio.rs")
    poll = function_block(api_stdio, "poll")
    require("fill_buf" in poll and "readable: true" not in poll, findings, "stdin poll must not report constant readiness")

    signal = read(root, "api/arceos_posix_api/src/signal.rs")
    time = read(root, "api/arceos_posix_api/src/imp/time.rs")
    nanosleep = function_block(time, "sys_nanosleep")
    require("has_interrupting_signal" in signal and "has_interrupting_signal" in nanosleep, findings, "nanosleep must use signal interrupt hook")
    require("Duration::ZERO" in nanosleep and "write_optional_timespec(rem" in nanosleep, findings, "nanosleep must write rem on interrupt and zero rem on success")
    require("TODO: should be woken by signals" not in time, findings, "nanosleep signal TODO must not remain as hidden unsupported semantics")

    api_fs = read(root, "api/arceos_posix_api/src/imp/fs.rs")
    flags_to_options = function_block(api_fs, "flags_to_options")
    require(not re.search(r"\b_mode\b", flags_to_options) and "options.mode" in flags_to_options and "FILE_MODE_UMASK" in flags_to_options, findings, "open mode must be consumed with umask instead of ignored")
    require("pub fn sys_umask" in api_fs, findings, "POSIX API must expose a real umask state")

    fops = read(root, "kernel/fs/axfs/src/fops.rs")
    open_at = function_block(fops, "_open_at")
    require(re.search(r"if\s+!created_new\s*&&\s*!perm_to_cap", open_at) is not None, findings, "O_CREAT must not reject the returned fd using the freshly-created file mode")

    root_rs = read(root, "kernel/fs/axfs/src/root.rs")
    root_rename_start = root_rs.rfind("pub(crate) fn rename")
    root_rename = function_block(root_rs[root_rename_start:], "rename") if root_rename_start >= 0 else ""
    require("remove_file" not in root_rename and "remove_dir" not in root_rename, findings, "root rename must not pre-delete destination")
    require("create_file_with_perm" in root_rs and "set_perm" in root_rs, findings, "root create must propagate create mode when supported")

    vfs = read(root, "vendor/axfs_vfs/src/lib.rs")
    ram_file = read(root, "vendor/axfs_ramfs/src/file.rs")
    ram_dir = read(root, "vendor/axfs_ramfs/src/dir.rs")
    require("fn set_perm" in vfs and "fn set_perm" in ram_file and "fn set_perm" in ram_dir, findings, "ramfs must store observable permission bits")
    rename_entry = function_block(ram_dir, "rename_entry")
    require("AlreadyExists" not in rename_entry and "DirectoryNotEmpty" in rename_entry and "insert(dst_name.into(), node)" in rename_entry, findings, "ramfs rename must support atomic-compatible replacement checks")

    libc_fs = read(root, "ulib/axlibc/src/fs.rs")
    getcwd = function_block(libc_fs, "getcwd")
    require("set_errno" in getcwd and "null_mut" in getcwd, findings, "axlibc getcwd must translate negative errno to NULL + errno")
    require("ax_umask" in libc_fs and "sys_umask" in libc_fs, findings, "axlibc must route umask to POSIX API state")

    libc_fd = read(root, "ulib/axlibc/src/fd_ops.rs")
    dup3 = function_block(libc_fd, "dup3")
    for token in ("!ctypes::O_CLOEXEC", "LinuxError::EINVAL", "F_SETFD", "FD_CLOEXEC", "sys_close"):
        require(token in dup3, findings, f"dup3 missing honest flag/error handling token {token}")

    libc_stdio = read(root, "ulib/axlibc/c/stdio.c")
    fflush = function_block(libc_stdio, "fflush")
    puts = function_block(libc_stdio, "puts")
    require("if (!f)" in fflush and "__fflush(stdout)" in fflush and "__fflush(stderr)" in fflush, findings, "fflush(NULL) must flush known output streams instead of dereferencing NULL")
    require("out(stdout" in puts and "write(1" not in puts and "return EOF" in puts, findings, "puts must use FILE output path and propagate failure")

    libc_socket = read(root, "ulib/axlibc/c/socket.c")
    accept4 = function_block(libc_socket, "accept4")
    for token in ("F_GETFL", "current_flags | O_NONBLOCK", "saved_errno", "close(ret)"):
        require(token in accept4, findings, f"accept4 missing flag failure cleanup token {token}")

    libc_stat = read(root, "ulib/axlibc/c/stat.c")
    umask = function_block(libc_stat, "umask")
    require("ax_umask" in umask and "ENOSYS" not in umask, findings, "libc umask must not be an ENOSYS stub")

    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    findings = scan(args.root)
    if findings:
        print("FAIL compliance regression guard")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("compliance regressions check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
