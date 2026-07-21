#!/usr/bin/env python3
"""Static guard for high-risk syscall boundary regressions."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def read(root: Path, rel: str) -> str:
    return (root / rel).read_text(encoding="utf-8", errors="ignore")


def rust_function_block(text: str, name: str) -> str:
    match = re.search(
        rf"(?:^|\n)(?:pub\([^)]*\)\s+|pub\s+)?fn\s+{re.escape(name)}(?:<[^>{{}}]*>)?\s*\([^{{]*\)\s*(?:->[^{{]+)?\{{",
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


def require_tokens(findings: list[str], block: str, desc: str, tokens: tuple[str, ...]) -> None:
    if not block:
        findings.append(f"{desc}: function block missing")
        return
    for token in tokens:
        if token not in block:
            findings.append(f"{desc}: missing {token!r}")


def rust_match_body(block: str, match_expr: str) -> str:
    start = block.find(match_expr)
    if start == -1:
        return ""
    brace = block.find("{", start)
    if brace == -1:
        return ""
    depth = 0
    for idx in range(brace, len(block)):
        ch = block[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return block[brace + 1 : idx]
    return block[brace + 1 :]


def split_rust_match_arms(match_body: str) -> list[str]:
    arms: list[str] = []
    start = 0
    brace_depth = 0
    paren_depth = 0
    bracket_depth = 0
    for idx, ch in enumerate(match_body):
        if ch == "{":
            brace_depth += 1
        elif ch == "}":
            brace_depth -= 1
            if brace_depth == 0 and paren_depth == 0 and bracket_depth == 0:
                lookahead = idx + 1
                while lookahead < len(match_body) and match_body[lookahead].isspace():
                    lookahead += 1
                if match_body.startswith("SyslogAction::", lookahead):
                    arm = match_body[start : idx + 1].strip()
                    if arm:
                        arms.append(arm)
                    start = lookahead
        elif ch == "(":
            paren_depth += 1
        elif ch == ")":
            paren_depth -= 1
        elif ch == "[":
            bracket_depth += 1
        elif ch == "]":
            bracket_depth -= 1
        elif ch == "," and brace_depth == 0 and paren_depth == 0 and bracket_depth == 0:
            arm = match_body[start:idx].strip()
            if arm:
                arms.append(arm)
            start = idx + 1
    tail = match_body[start:].strip()
    if tail:
        arms.append(tail)
    return arms


def require_syslog_state_actions_privileged(findings: list[str], syslog: str) -> None:
    state_actions = (
        "Close",
        "Open",
        "Clear",
        "ConsoleOff",
        "ConsoleOn",
        "ConsoleLevel",
    )
    match_body = rust_match_body(syslog, "match syslog_action(log_type)")
    arms = split_rust_match_arms(match_body)
    handled = {action: False for action in state_actions}
    for arm in arms:
        if "=>" not in arm:
            continue
        lhs, rhs = arm.split("=>", 1)
        for action in state_actions:
            if f"SyslogAction::{action}" not in lhs:
                continue
            handled[action] = True
            if "privileged_syslog_control(process," not in rhs:
                findings.append(
                    f"sys_syslog state-changing action SyslogAction::{action} must route to privileged_syslog_control(process)"
                )
    for action, was_handled in handled.items():
        if not was_handled:
            findings.append(f"sys_syslog missing explicit match arm for state-changing SyslogAction::{action}")


def scan_user_trace(root: Path) -> list[str]:
    findings: list[str] = []
    mod = read(root, "user/shell/src/uspace/mod.rs")
    macro_match = re.search(r"macro_rules!\s+user_trace\s*\{(?P<body>.*?)\n\}", mod, re.S)
    if not macro_match:
        findings.append("uspace mod: missing central user_trace macro")
    else:
        body = macro_match.group("body")
        if (
            'option_env!("USER_TRACE")' not in body
            or "Some(_) => println!($($arg)*)" not in body
        ):
            findings.append("uspace mod: user_trace macro must not be an empty shell")
    for rel in (
        "user/shell/src/uspace/futex.rs",
        "user/shell/src/uspace/signal_abi.rs",
        "user/shell/src/uspace/memory_map.rs",
        "user/shell/src/uspace/process_lifecycle.rs",
        "user/shell/src/uspace/task_context.rs",
    ):
        if "macro_rules! user_trace" in read(root, rel):
            findings.append(f"{rel}: local empty user_trace macro must not shadow central trace")
    block = rust_function_block(read(root, "user/shell/src/uspace/user_memory.rs"), "log_read_cstr_efault")
    require_tokens(
        findings,
        block,
        "log_read_cstr_efault must emit gated diagnostic context",
        ("user_trace!", "process.pid()", "fault_addr", "reason", "query_address"),
    )
    if re.search(r"fn\s+log_read_cstr_efault\s*\([^)]*\)\s*\{\s*\}", block):
        findings.append("log_read_cstr_efault is still empty")
    return findings


def scan_high_risk_boundaries(root: Path) -> list[str]:
    findings: list[str] = []
    mount = read(root, "user/shell/src/uspace/mount_abi.rs")
    resolve = rust_function_block(mount, "resolve_mount_source")
    require_tokens(
        findings,
        resolve,
        "vfat mount must attach a real mounted filesystem instead of aliasing an existing path",
        ('"vfat"', "synthetic_block_device_for_mount", "axfs::api::mount_fatfs", "Ok(target_path.into())"),
    )
    require_tokens(
        findings,
        resolve,
        "unsupported block filesystems must still fail explicitly",
        ('"ext2"', '"ext3"', '"ext4"', "LinuxError::EOPNOTSUPP"),
    )
    if "is_supported_block_device_name" in mount:
        findings.append("mount_abi: block-device name alias helper is still present")
    if re.search(r'"vfat"[\s\S]*?Ok\("/"\.into\(\)\)', resolve):
        findings.append("mount_abi: block-filesystem mount still aliases to root")

    fd = read(root, "user/shell/src/uspace/fd_table.rs")
    fsync = rust_function_block(fd, "sys_fsync")
    require_tokens(
        findings,
        fsync,
        "sys_fsync must flush real files and reject unsupported fd classes",
        ("FdEntry::File", "file.file.flush()", "FdEntry::Memfd", "LinuxError::EINVAL"),
    )
    if "Ok(_) => 0" in fsync:
        findings.append("sys_fsync still has a catch-all success arm")
    open_entry = rust_function_block(fd, "open_fd_entry")
    require_tokens(
        findings,
        fd,
        "uspace openat/openat2 must reject unknown open flags instead of only validating API/libc wrappers",
        ("fn supported_open_flags", "general::O_TMPFILE", "O_PATH_FLAG", "O_NOFOLLOW_FLAG"),
    )
    require_tokens(
        findings,
        open_entry,
        "open_fd_entry must apply the supported open flag mask before using flags",
        ("flags & !supported_open_flags() != 0", "LinuxError::EINVAL"),
    )
    set_flags = rust_function_block(fd, "socket_ioctl_set_ifflags")
    require_tokens(
        findings,
        set_flags,
        "SIOCSIFFLAGS must not validate and fake-success",
        ("validate_user_read", "LinuxError::EPERM", "LinuxError::EOPNOTSUPP"),
    )
    if re.search(r"Ok\(\(\)\)\s*=>\s*0", set_flags):
        findings.append("SIOCSIFFLAGS still returns success after validation only")
    return findings


def scan_stateful_boundaries(root: Path) -> list[str]:
    findings: list[str] = []
    sysinfo = read(root, "user/shell/src/uspace/system_info.rs")
    syslog = rust_function_block(sysinfo, "sys_syslog")
    if "PrivilegedNoop" in sysinfo:
        findings.append("sys_syslog still has a PrivilegedNoop action")
    if re.search(r"SYSLOG_(?:OPEN|CLEARED|CONSOLE_ENABLED|CONSOLE_LEVEL)", sysinfo):
        findings.append("sys_syslog still carries write-only SYSLOG_* state instead of a real backend or explicit error")
    helper = rust_function_block(sysinfo, "privileged_syslog_control")
    snapshot = rust_function_block(sysinfo, "klog_snapshot_bytes")
    copy_snapshot = rust_function_block(sysinfo, "copy_klog_snapshot")
    require_tokens(
        findings,
        helper,
        "sys_syslog privileged control actions must enforce root and update modeled klog/console state",
        (
            "LinuxError::EPERM",
            "return neg_errno",
            "KLOG_CONTROL_STATE.open.store",
            "KLOG_CONTROL_STATE.console_level.store",
            "clear_generation",
        ),
    )
    require_tokens(
        findings,
        snapshot,
        "sys_syslog control state must be consumed by the syscall-visible klog snapshot, not just stored",
        (
            "KLOG_CONTROL_STATE.open.load",
            "KLOG_CONTROL_STATE.console_enabled.load",
            "console_level = KLOG_CONTROL_STATE",
            "KLOG_CONTROL_STATE.clear_generation.load",
        ),
    )
    require_tokens(
        findings,
        copy_snapshot,
        "sys_syslog read/read-clear paths must expose and clear the modeled klog buffer",
        (
            "klog_snapshot_bytes",
            "write_user_bytes",
            "clear_after_read",
            "KLOG_CONTROL_STATE",
            "fetch_add",
        ),
    )
    require_tokens(
        findings,
        syslog,
        "sys_syslog state-changing actions must not bypass privileged control validation and visible read/size effects",
        (
            "SyslogAction::ReadClear",
            "SyslogAction::Clear",
            "SyslogAction::ConsoleOff",
            "privileged_syslog_control(process,",
            "copy_klog_snapshot(process, buf, len, true)",
            "KLOG_BUFFER_CAPACITY",
        ),
    )
    require_syslog_state_actions_privileged(findings, syslog)

    time = read(root, "user/shell/src/uspace/time_abi.rs")
    process_times = rust_function_block(time, "process_times")
    if "elapsed / 2" in process_times or "saturating_sub(user_ticks)" in process_times:
        findings.append("process_times still fabricates a half user/system split")
    if "start_clock_ticks" in process_times or "clock_ticks_now()" in process_times:
        findings.append("process_times must not use wall-clock lifetime as CPU time")
    require_tokens(
        findings,
        process_times,
        "process_times must use scheduler runtime as CPU total and only cap system split with measured syscall runtime",
        (
            "process_runtime_duration",
            "duration_to_user_hz_ticks",
            "total_runtime_ticks",
            "syscall_runtime_micros",
            "micros_to_ticks",
            ".min(total_runtime_ticks)",
            "last_reported_user_ticks",
            "last_reported_system_ticks",
        ),
    )
    require_tokens(
        findings,
        time,
        "adjtimex frequency/tick discipline must affect CLOCK_REALTIME instead of only storing fields",
        (
            "discipline_extra_ns_for_raw",
            "epoch_raw_ns",
            "epoch_extra_ns",
            "ADJ_FREQUENCY | ADJ_TICK",
            "reset_discipline_epoch",
        ),
    )

    sched = read(root, "user/shell/src/uspace/resource_sched.rs")
    sched_param_accepts = rust_function_block(sched, "sched_param_accepts_policy")
    require_tokens(
        findings,
        sched_param_accepts,
        "sched_setscheduler must validate policy-specific sched_param without accepting deadline-only policy tuples",
        ("SCHED_FIFO", "SCHED_RR", "SCHED_BATCH", "SCHED_IDLE", "_ => false"),
    )
    if "SCHED_DEADLINE" in sched_param_accepts:
        findings.append(
            "sys_sched_setscheduler must not accept SCHED_DEADLINE through sched_param-only validation; use sched_setattr for deadline tuples"
        )
    sched_setscheduler = rust_function_block(sched, "sys_sched_setscheduler")
    require_tokens(
        findings,
        sched_setscheduler,
        "sys_sched_setscheduler must reject policy/param mismatch visibly",
        ("!sched_param_accepts_policy(base_policy, param)", "LinuxError::EINVAL"),
    )
    sched_attr = rust_function_block(sched, "sched_state_from_attr")
    require_tokens(
        findings,
        sched_attr,
        "sched_setattr SCHED_DEADLINE must validate and preserve API-visible deadline attributes",
        (
            "general::SCHED_DEADLINE",
            "attr.sched_runtime == 0",
            "attr.sched_runtime > attr.sched_deadline",
            "attr.sched_deadline > attr.sched_period",
            "sched_runtime: attr.sched_runtime",
            "sched_deadline: attr.sched_deadline",
            "sched_period: attr.sched_period",
        ),
    )
    require_tokens(
        findings,
        sched,
        "SCHED_DEADLINE must affect backend scheduling priority rather than only being stored",
        (
            "fn deadline_scheduler_backend_priority",
            "general::SCHED_DEADLINE => deadline_scheduler_backend_priority",
            "sched_runtime",
            "sched_period",
        ),
    )
    if "scheduled with the normal nice-based priority" in sched:
        findings.append("SCHED_DEADLINE comment still admits normal-priority fake success")

    memory = read(root, "user/shell/src/uspace/memory_map.rs")
    require_tokens(
        findings,
        memory,
        "MADV_DONTFORK/DOFORK must fail visibly when fork-policy metadata cannot be tracked",
        (
            "fn madvise_range_is_tracked",
            "general::MADV_DONTFORK | general::MADV_DOFORK",
            "!madvise_range_is_tracked(process, addr, end)",
            "process.set_mmap_dont_fork_range",
        ),
    )
    for name in ("mremap_shrink_in_place", "mremap_try_expand_in_place", "mremap_move"):
        block = rust_function_block(memory, name)
        require_tokens(
            findings,
            block,
            f"{name} must preserve full mmap metadata including DONTFORK/WIPEONFORK and SIGBUS poison ranges",
            (
                "mmap_sigbus_segments",
                "record_mmap_region_entry(region)",
                "record_mmap_sigbus_ranges",
            ),
        )
    futex = read(root, "user/shell/src/uspace/futex.rs")
    wake_requeue = rust_function_block(futex, "wake_requeue_addr_checked")
    require_tokens(
        findings,
        wake_requeue,
        "futex requeue helper must return wake and requeue counts separately",
        (
            "Result<(usize, usize), LinuxError>",
            "return Ok((0, 0));",
            "Ok((woken, requeued))",
        ),
    )
    require_tokens(
        findings,
        wake_requeue,
        "FUTEX_CMP_REQUEUE must validate both futex keys before comparing the source value",
        (
            "if uaddr2 % size_of::<u32>() != 0",
            "let source_key = futex_key(process, uaddr, private)?;",
            "let target_key = futex_key(process, uaddr2, private)?;",
            "if let Some(expected) = cmp",
        ),
    )
    if "uaddr2 == 0" in wake_requeue:
        findings.append("FUTEX_CMP_REQUEUE null target must be reported by futex_key as EFAULT")
    requeue_source_key = wake_requeue.find("let source_key = futex_key(process, uaddr, private)?;")
    requeue_target_key = wake_requeue.find("let target_key = futex_key(process, uaddr2, private)?;")
    requeue_compare = wake_requeue.find("if let Some(expected) = cmp")
    if not (0 <= requeue_source_key < requeue_target_key < requeue_compare):
        findings.append("FUTEX_CMP_REQUEUE compares the source value before validating both futex keys")
    sys_futex = rust_function_block(futex, "sys_futex")
    requeue_start = sys_futex.find("general::FUTEX_REQUEUE =>")
    cmp_requeue_start = sys_futex.find("general::FUTEX_CMP_REQUEUE =>")
    wake_bitset_start = sys_futex.find("general::FUTEX_WAKE_BITSET =>")
    requeue_arm = (
        sys_futex[requeue_start:cmp_requeue_start]
        if 0 <= requeue_start < cmp_requeue_start
        else ""
    )
    cmp_requeue_arm = (
        sys_futex[cmp_requeue_start:wake_bitset_start]
        if 0 <= cmp_requeue_start < wake_bitset_start
        else ""
    )
    require_tokens(
        findings,
        requeue_arm,
        "FUTEX_REQUEUE must return woken+requeued affected waiters",
        (
            "general::FUTEX_REQUEUE",
            "Ok((woken, requeued)) => woken.saturating_add(requeued) as isize",
        ),
    )
    require_tokens(
        findings,
        cmp_requeue_arm,
        "FUTEX_CMP_REQUEUE must return woken+requeued affected waiters",
        (
            "general::FUTEX_CMP_REQUEUE",
            "Ok((woken, requeued)) => woken.saturating_add(requeued) as isize",
        ),
    )
    wait_queue = read(root, "kernel/task/axtask/src/wait_queue.rs")
    same_queue_requeue = rust_function_block(wait_queue, "count_same_queue_requeues")
    require_tokens(
        findings,
        same_queue_requeue,
        "same-address futex requeue must count matching waiters without dropping them",
        (
            "already_selected.contains(&key)",
            "requeued_len < requeue_count && predicate(&task)",
            "on_requeue(&task);",
            "requeued_len = requeued_len.saturating_add(1);",
            "source.push_back(task);",
        ),
    )
    notify_requeue = wait_queue
    require_tokens(
        findings,
        notify_requeue,
        "same-target WaitQueue requeue must use the in-place counting path",
        (
            "count_same_queue_requeues(",
            "if core::ptr::eq(self, target)",
            "return operate(&mut source, None);",
        ),
    )

    lifecycle = read(root, "user/shell/src/uspace/process_lifecycle.rs")
    require_tokens(
        findings,
        lifecycle,
        "mmap SIGBUS metadata helpers must support preserving poisoned ranges across mremap",
        (
            "pub(super) fn mmap_sigbus_segments",
            "pub(super) fn record_mmap_sigbus_ranges",
        ),
    )
    require_tokens(
        findings,
        lifecycle,
        "wait child reaping must return resource usage for wait4/waitid",
        (
            "Result<Option<(i32, i32, general::rusage)>, LinuxError>",
            "rusage_from_child_usage(child_usage, child_maxrss)",
            "Ok(Some((child_pid, status, child_rusage)))",
        ),
    )
    registry = read(root, "user/shell/src/uspace/task_registry.rs")
    unregister = rust_function_block(registry, "unregister_user_task_with_runtime")
    require_tokens(
        findings,
        unregister,
        "thread unregister must commit completed runtime under the table lock before removing the live entry",
        (
            "let mut table = user_thread_table().lock();",
            "completed_thread_runtime_ticks",
            ".fetch_add(runtime_ticks, Ordering::AcqRel)",
            "table.remove(&tid)",
        ),
    )
    if unregister.find(".fetch_add(runtime_ticks, Ordering::AcqRel)") > unregister.find("table.remove(&tid)"):
        findings.append(
            "unregister_user_task_with_runtime removes the live task before committing completed runtime"
        )

    linux_abi = read(root, "user/shell/src/uspace/linux_abi.rs")
    process_abi = read(root, "user/shell/src/uspace/process_abi.rs")
    if "LINUX_PERSONALITY_MASK" in linux_abi or "LINUX_PERSONALITY_MASK" in process_abi:
        findings.append("personality must not silently mask/accept arbitrary persona values")
    require_tokens(
        findings,
        linux_abi,
        "personality ABI should name only the supported Linux persona instead of a broad accept mask",
        ("LINUX_PERSONALITY_QUERY", "PER_LINUX"),
    )
    sys_personality = rust_function_block(process_abi, "sys_personality")
    require_tokens(
        findings,
        sys_personality,
        "sys_personality must return errno for unsupported persona requests",
        ("apply_personality_request", "Err(err) => neg_errno(err)"),
    )
    apply_personality = rust_function_block(process_abi, "apply_personality_request")
    require_tokens(
        findings,
        apply_personality,
        "personality changes must be validated before updating process state",
        ("persona != LINUX_PERSONALITY_QUERY", "validate_personality(persona)?", "process.set_personality(persona)"),
    )
    validate_personality = rust_function_block(process_abi, "validate_personality")
    require_tokens(
        findings,
        validate_personality,
        "personality validation must accept only named Linux domains/flags and reject unknown bits",
        (
            "PERSONALITY_PER_MASK",
            "PERSONALITY_MAX_KNOWN_DOMAIN",
            "PERSONALITY_KNOWN_FLAGS",
            "Err(LinuxError::EINVAL)",
            "Ok(persona)",
        ),
    )

    wait4 = rust_function_block(lifecycle, "sys_wait4")
    if "_rusage:" in wait4:
        findings.append("sys_wait4 must not mark rusage as intentionally ignored")
    require_tokens(
        findings,
        wait4,
        "sys_wait4 must write caller-visible rusage instead of ignoring the argument",
        (
            "rusage: usize",
            "child_rusage",
            "write_user_value(process, rusage, &child_rusage)",
        ),
    )
    waitid = rust_function_block(lifecycle, "sys_waitid")
    if "_rusage:" in waitid:
        findings.append("sys_waitid must not mark rusage as intentionally ignored")
    require_tokens(
        findings,
        waitid,
        "sys_waitid must write caller-visible rusage instead of ignoring the argument",
        (
            "rusage: usize",
            "child_rusage",
            "write_user_value(process, rusage, &child_rusage)",
        ),
    )

    pthread = read(root, "api/arceos_posix_api/src/imp/pthread/mod.rs")
    require_tokens(
        findings,
        pthread,
        "pthread_create must register pthread_t before user start routine can run",
        (
            "registration_ready",
            "while !child_registration_ready.load(Ordering::Acquire)",
            "axtask::yield_now()",
            "TID_TO_PTHREAD.write().insert(tid, thread)",
            "registration_ready.store(true, Ordering::Release)",
        ),
    )
    require_tokens(
        findings,
        pthread,
        "missing pthread registration must be visible rather than a silent null/default path",
        (
            "error!(\"pthread_self: missing pthread registration",
            "error!(\"pthread_exit: missing pthread registration",
        ),
    )
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    root = args.root.resolve()
    findings = scan_user_trace(root) + scan_high_risk_boundaries(root) + scan_stateful_boundaries(root)
    if findings:
        print("syscall boundary regressions check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("syscall boundary regressions check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
