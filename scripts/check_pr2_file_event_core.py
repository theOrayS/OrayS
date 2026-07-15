#!/usr/bin/env python3
"""Integration guard for PR2 file-object and readiness invariants.

The executable Rust tests prove the reusable state machines.  This guard binds
those state machines to the actual exec, FD-table, epoll, poll/select, timerfd,
and pipe call sites so a later edit cannot bypass the tested path unnoticed.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


EXPECTED_FD_VARIANTS = {
    "Stdin", "Stdout", "Stderr", "DevNull", "DevZero", "DevRandom",
    "DevCpuDmaLatency", "BlockDevice", "Rtc", "File", "Directory",
    "ProcFdDir", "SyntheticDir", "Path", "MemoryFile", "Memfd",
    "ProcPagemap", "ProcTimerSlack", "Pipe", "Socket", "LocalSocket",
    "EventFd", "Inotify", "Epoll", "TimerFd", "SignalFd", "PidFd",
    "PosixMq", "ProcMqQueuesMax", "ProcSysFile",
}


def read(root: Path, relative: str) -> str:
    return (root / relative).read_text(encoding="utf-8")


def require(condition: bool, findings: list[str], message: str) -> None:
    if not condition:
        findings.append(message)


def require_order(
    source: str,
    tokens: tuple[str, ...],
    findings: list[str],
    message: str,
) -> None:
    positions = [source.find(token) for token in tokens]
    require(
        all(position >= 0 for position in positions)
        and positions == sorted(positions)
        and len(set(positions)) == len(positions),
        findings,
        message,
    )


def block_after(source: str, marker: str) -> str:
    start = source.find(marker)
    if start < 0:
        return ""
    opening = source.find("{", start + len(marker))
    if opening < 0:
        return ""
    depth = 0
    for index in range(opening, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                return source[start:index + 1]
    return ""


def enum_variants(source: str) -> set[str]:
    match = re.search(r"pub\(super\) enum FdEntry\s*\{(?P<body>.*?)\n\}", source, re.S)
    if not match:
        return set()
    return set(re.findall(r"^\s{4}([A-Z][A-Za-z0-9_]*)", match.group("body"), re.M))


def check(root: Path) -> list[str]:
    findings: list[str] = []
    core = read(root, "kernel/fs/axfile/src/lib.rs")
    objects = read(root, "user/shell/src/uspace/fd_object.rs")
    table = read(root, "user/shell/src/uspace/fd_table.rs")
    pipe = read(root, "user/shell/src/uspace/fd_pipe.rs")
    socket = read(root, "user/shell/src/uspace/fd_socket.rs")
    select = read(root, "user/shell/src/uspace/select_fdset.rs")
    lifecycle = read(root, "user/shell/src/uspace/process_lifecycle.rs")
    cases = read(root, "user/shell/src/cmd.rs")
    makefile = read(root, "Makefile")
    test_workflow = read(root, ".github/workflows/test.yml")

    # Compatibility boundary and stable descriptor/open-description split.
    require(enum_variants(table) == EXPECTED_FD_VARIANTS, findings,
            "FdEntry discriminants changed outside the PR2 migration slice")
    for payload in ("Pipe(OpenFileRef)", "EventFd(OpenFileRef)", "TimerFd(OpenFileRef)"):
        require(payload in table, findings, f"migrated payload missing: {payload}")
    for payload in ("File(FileEntry)", "Socket(SocketEntry)", "SignalFd(SignalFdEntry)",
                    "PidFd(PidFdEntry)", "PosixMq(PosixMqDescriptor)"):
        require(payload in table, findings,
                f"legacy adapter boundary expanded unexpectedly: {payload}")
    for token in ("struct OpenFileId", "fetch_update", "struct RegistrationKey",
                  "Weak<dyn EventObserver>", "EventSubscription", "#![forbid(unsafe_code)]"):
        require(token in core, findings, f"axfile core invariant missing: {token}")
    for token in ("struct FdSlot", "fd_flags: u32",
                  "description: Arc<DescriptionIdentity>"):
        require(token in table, findings, f"descriptor/open-description split missing: {token}")
    require("registered_fd: i32" in core and "open_file_id: OpenFileId" in core,
            findings, "registration key must combine FD and stable open-file identity")
    require("Arc::downgrade(observer)" in core, findings,
            "event sources must retain observers weakly")
    require("EpollTarget::Object(Arc::downgrade(file))" in table, findings,
            "epoll migrated target must be weak")
    require("BTreeMap<i32, EpollRegistration>" not in table, findings,
            "epoll regressed to an integer-FD-only registration map")

    # Review findings 1-2: exec transaction, partial-copy rollback, and explicit
    # CLONE_FILES membership rather than a racy Arc count.
    exec_program = block_after(lifecycle, "fn exec_program(")
    require_order(
        exec_program,
        ("load_program_image(", ".unshare_for_pid_if_shared(process.pid())",
         "core::mem::swap(&mut *aspace, &mut new_aspace)"),
        findings,
        "exec must finish loading and split CLONE_FILES before committing the new image",
    )
    require("ExecProgramError::FdTable" in lifecycle, findings,
            "exec must return a fallible fd-table split without committing the image")
    require(not re.search(r"Arc::strong_count\([^\n)]*fds", lifecycle + table), findings,
            "CLONE_FILES decisions must not use Arc::strong_count")
    require("struct FileTableShareTracker" in core and
            "sharing: FileTableShareTracker" in table, findings,
            "explicit CLONE_FILES membership tracker is not wired into ProcessFdTable")
    share_child = block_after(table, "pub(super) fn share_for_child_pid(")
    require_order(share_child, ("let mut state = self.state.lock()",
                                "state.sharing.share(parent_pid, child_pid)",
                                "Arc::clone(self)"), findings,
                  "CLONE_FILES membership and Arc publication must share one critical section")
    unshare = block_after(table, "pub(super) fn unshare_for_pid_if_shared(")
    require_order(unshare, (".fork_copy()?", "state.sharing.split(pid)"), findings,
                  "fallible fd-table copy must precede the membership split commit")
    fork_copy = block_after(table, "pub(super) fn fork_copy(&self)")
    require("for copied in slots.iter().flatten()" in fork_copy and
            "socket.close()" in fork_copy, findings,
            "partial fork-copy failure must close already duplicated raw sockets")
    discard_uninstalled = block_after(table, "fn discard_uninstalled_entry(")
    insert_new = block_after(table, "pub(super) fn insert_min_with_flags(")
    insert_alias = block_after(table, "fn insert_min_with_description(")
    dup3 = block_after(table, "pub(super) fn dup3(")
    require("FdEntry::Socket(socket)" in discard_uninstalled and
            "socket.close()" in discard_uninstalled and
            insert_new.count("discard_uninstalled_entry(entry)") == 2 and
            insert_alias.count("discard_uninstalled_entry(entry)") == 2 and
            "discard_uninstalled_entry(entry)" in dup3,
            findings,
            "failed duplicate installation must roll back raw legacy socket ownership")
    insert_socket = block_after(socket, "pub(super) fn insert_socket_entry(")
    transferred_socket = insert_socket.split(
        "match process.fds.lock().insert_with_flags", 1
    )[-1]
    require("sys_close(posix_fd)" not in transferred_socket, findings,
            "socket insertion failure must not double-close table-owned raw descriptors")

    # Review findings 3-4 and 9: description-owned legacy compatibility lease,
    # per-registration edge generations, and proactive final-close cleanup.
    legacy_ready = block_after(table, "fn epoll_ready_events_for_target(")
    require("struct LegacyEpollTarget" in table and
            "Legacy(Arc<LegacyEpollTarget>)" in table and
            "LegacyEpollTarget::duplicate(&target_slot.entry)?" in table and
            "&target.entry" in legacy_ready and "self.slots" not in legacy_ready,
            findings,
            "legacy epoll readiness must use a fork-stable compatibility lease")
    epoll_syscall = block_after(table, "pub(super) fn sys_epoll_ctl(")
    require_order(epoll_syscall, ("let result = {", "drop(update.retired)",
                                  "update.epoll.state.wake()"), findings,
                  "retired legacy epoll targets must drop after the FD-table guard")
    ready_observer = block_after(table, "struct EpollReadyObserver")
    require("struct EventDeliveryState" in core and
            "notification: AtomicU64" in ready_observer and
            "events.intersects(self.interests)" in table and
            "registration.delivery.claim(" in table,
            findings,
            "epoll must atomically claim only relevant per-registration notifications")
    require("stale.push((snapshot.key, snapshot.version))" in table and
            "registration.version == version" in table,
            findings, "a stale scan must not remove a newer MOD registration version")
    require("struct DescriptionIdentity" in core and "struct ClosedFd" in table and
            "struct EpollCloseObserver" in table and
            ".close_events()" in table and "remove_registration_if_version" in table,
            findings, "last-description close must remove the exact epoll registration version")

    # Review finding 5: every blocking consumer performs a final level query
    # before declaring a deadline timeout.
    require("polled_once" not in table + select, findings,
            "poll/select/epoll must not timeout before their final level query")
    for source, marker, label in (
        (select, "pub(super) fn sys_pselect6(", "pselect"),
        (select, "fn sys_poll_until(", "poll"),
        (table, "fn sys_epoll_wait_with_timeout(", "epoll"),
    ):
        require("decide_level_wait(" in block_after(source, marker), findings,
                f"{label} does not route final readiness through decide_level_wait")
    epoll_wait = block_after(table, "fn sys_epoll_wait_with_timeout(")
    require(epoll_wait.count("FdTable::epoll_collect_ready_for(") == 2 and
            "table.epoll_collect_ready_for(" not in epoll_wait,
            findings, "epoll must query registration targets without an FD-table guard")
    for token in ("struct ObjectWaitSet", "generation()", "wait_for_change",
                  "event_source().subscribe"):
        require(token in objects, findings, f"poll/select wait protocol missing: {token}")

    # Review finding 6: cycle validation and graph mutation are serialized, and
    # synchronous observer recursion is bounded by the tested core gate.
    epoll_ctl = block_after(table, "fn epoll_ctl(")
    require_order(epoll_ctl, ("epoll_graph_lock().lock()",
                              "self.validate_epoll_target(",
                              ".insert(key, registration)"), findings,
                  "nested epoll validation and insertion must hold the global graph lock")
    epoll_wake = block_after(table, "fn wake(&self)")
    require("notifying: ReentrancyGate" in table and
            "self.notifying.try_enter()" in epoll_wake and
            "notification_guard.finish_round()" in epoll_wake, findings,
            "nested epoll callbacks must coalesce concurrent notifications")

    # Review findings 7, 10, and 11: reciprocal pipe locking, OFD-local async
    # state, and no user copy under the async-state lock.
    tee_to = block_after(pipe, "pub(super) fn tee_to(")
    require("with_ordered_arc_mutex_pair(" in tee_to, findings,
            "pipe tee must use the canonical reciprocal lock-order helper")
    require("async_state: Arc<Mutex<PipeAsyncState>>" in pipe and
            "async_listeners: Arc<Mutex<Vec<Weak<Mutex<PipeAsyncState>>>>>" in pipe and
            "let read_async_state = Arc::new(Mutex::new(PipeAsyncState::new(true)))" in pipe and
            "let write_async_state = Arc::new(Mutex::new(PipeAsyncState::new(false)))" in pipe,
            findings, "pipe async owner/enabled state must be local to each open description")
    getown_ex = block_after(pipe, "F_GETOWN_EX =>")
    owner_start = getown_ex.find("let owner = {")
    require(owner_start >= 0 and
            "self.async_state.lock()" not in getown_ex[:owner_start] and
            "let owner = {\n                    let state = self.async_state.lock();" in getown_ex and
            getown_ex.find("write_user_value(process, arg, &owner)") >
            getown_ex.find("\n                };") >= 0,
            findings, "F_GETOWN_EX must release the async-state lock before user-memory copy")

    # Review finding 8: timer profile calculation must never consume an expiry;
    # an expired deadline requests an immediate outer-loop re-query.
    timer_timeout = block_after(table, "fn next_timeout(&self)")
    require("readiness_deadline_delay(" in timer_timeout and
            "refresh_silent" not in timer_timeout, findings,
            "timerfd wait profiling must be non-mutating and force expired deadlines to rescan")

    # Review finding 12: executable semantic tests and mutation guards are part
    # of both local make unittest variants and therefore the existing Test CI.
    semantic_tests = (
        "description_close_notification_waits_for_the_last_alias",
        "legacy_target_lease_survives_fork_alias_and_drops_on_final_close",
        "readiness_hints_preserve_the_changed_event_class",
        "edge_delivery_keeps_a_notification_between_level_scans",
        "one_shot_claim_is_single_winner_across_concurrent_waiters",
        "final_level_query_wins_over_a_reached_deadline",
        "ready_or_expired_object_requests_an_immediate_rescan",
        "notification_reentrancy_is_bounded_and_released_by_drop",
        "notification_reentrancy_coalesces_without_losing_a_round",
        "reciprocal_pair_operations_share_one_lock_order",
        "object_state_is_shared_only_by_description_aliases",
        "failed_split_preparation_leaves_shared_membership_unchanged",
    )
    for test in semantic_tests:
        require(f"fn {test}()" in core, findings, f"semantic regression test missing: {test}")
    require("pr2-check:" in makefile and
            "python3 scripts/check_pr2_file_event_core.py" in makefile and
            "python3 scripts/test_pr2_file_event_core.py" in makefile and
            "cargo test -p axfile --lib" in makefile and
            "unittest: pr2-check" in makefile and
            "unittest_no_fail_fast: pr2-check" in makefile,
            findings, "PR2 semantic and mutation tests are not wired into both unittest targets")
    require("make unittest_no_fail_fast" in test_workflow, findings,
            "Test CI no longer executes the unittest target containing PR2 regressions")

    # Runtime cases remain a separate image/QEMU gate; names are inventory only,
    # never a substitute for the executable core and integration mutation tests.
    for case in ("pipe2_01", "dup3_01", "epoll_ctl01", "epoll_wait01",
                 "eventfd01", "eventfd2_01", "timerfd_create01", "timerfd_settime01"):
        require(f'"{case}"' in cases, findings, f"runtime regression case missing: {case}")

    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    findings = check(args.root.resolve())
    if findings:
        for finding in findings:
            print(f"PR2-CHECK: {finding}")
        return 1
    print("PR2-CHECK: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
