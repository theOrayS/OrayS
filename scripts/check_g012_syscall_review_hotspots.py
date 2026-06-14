#!/usr/bin/env python3
"""Static guard for the 2026-06-08 syscall review fake-implementation hotspots."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def read(root: Path, rel: str) -> str:
    return (root / rel).read_text(encoding="utf-8", errors="ignore")


def rust_function_block(text: str, name: str) -> str:
    match = re.search(rf"(?:^|\n)(?:pub\([^)]*\)\s+|pub\s+)?fn\s+{re.escape(name)}\s*\([^{{]*\)\s*(?:->[^{{]+)?\{{", text)
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
    mod = read(root, "examples/shell/src/uspace/mod.rs")
    macro_match = re.search(r"macro_rules!\s+user_trace\s*\{(?P<body>.*?)\n\}", mod, re.S)
    if not macro_match:
        findings.append("uspace mod: missing central user_trace macro")
    else:
        body = macro_match.group("body")
        if "=> {};" in body or "format_args" not in body:
            findings.append("uspace mod: user_trace macro must not be an empty shell")
    for rel in (
        "examples/shell/src/uspace/futex.rs",
        "examples/shell/src/uspace/signal_abi.rs",
        "examples/shell/src/uspace/memory_map.rs",
        "examples/shell/src/uspace/process_lifecycle.rs",
        "examples/shell/src/uspace/task_context.rs",
    ):
        if "macro_rules! user_trace" in read(root, rel):
            findings.append(f"{rel}: local empty user_trace macro must not shadow central trace")
    block = rust_function_block(read(root, "examples/shell/src/uspace/user_memory.rs"), "log_read_cstr_efault")
    require_tokens(
        findings,
        block,
        "log_read_cstr_efault must emit gated diagnostic context",
        ("user_trace!", "process.pid()", "fault_addr", "reason", "query_address"),
    )
    if re.search(r"fn\s+log_read_cstr_efault\s*\([^)]*\)\s*\{\s*\}", block):
        findings.append("log_read_cstr_efault is still empty")
    return findings


def scan_high_hotspots(root: Path) -> list[str]:
    findings: list[str] = []
    mount = read(root, "examples/shell/src/uspace/mount_abi.rs")
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

    fd = read(root, "examples/shell/src/uspace/fd_table.rs")
    fsync = rust_function_block(fd, "sys_fsync")
    require_tokens(
        findings,
        fsync,
        "sys_fsync must flush real files and reject unsupported fd classes",
        ("FdEntry::File", "file.file.flush()", "FdEntry::Memfd", "LinuxError::EINVAL"),
    )
    if "Ok(_) => 0" in fsync:
        findings.append("sys_fsync still has a catch-all success arm")
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


def scan_medium_hotspots(root: Path) -> list[str]:
    findings: list[str] = []
    sysinfo = read(root, "examples/shell/src/uspace/system_info.rs")
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

    time = read(root, "examples/shell/src/uspace/time_abi.rs")
    process_times = rust_function_block(time, "process_times")
    if "elapsed / 2" in process_times or "saturating_sub(user_ticks)" in process_times:
        findings.append("process_times still fabricates a half user/system split")
    require_tokens(
        findings,
        process_times,
        "process_times must use measured syscall runtime instead of fabricated CPU splits",
        (
            "lacks hardware-mode CPU accounting",
            "syscall_runtime_micros",
            "micros_to_ticks",
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

    sched = read(root, "examples/shell/src/uspace/resource_sched.rs")
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
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    root = args.root.resolve()
    findings = scan_user_trace(root) + scan_high_hotspots(root) + scan_medium_hotspots(root)
    if findings:
        print("G012 syscall review hotspot guard: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("G012 syscall review hotspot guard: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
