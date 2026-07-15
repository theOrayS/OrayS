#!/usr/bin/env python3
"""Mutation tests proving that the PR2 integration guard covers each finding."""

from __future__ import annotations

import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CHECK = ROOT / "scripts/check_pr2_file_event_core.py"
FILES = (
    "Makefile",
    ".github/workflows/test.yml",
    "kernel/fs/axfile/src/lib.rs",
    "user/shell/src/uspace/fd_object.rs",
    "user/shell/src/uspace/fd_pipe.rs",
    "user/shell/src/uspace/fd_socket.rs",
    "user/shell/src/uspace/fd_table.rs",
    "user/shell/src/uspace/select_fdset.rs",
    "user/shell/src/uspace/process_lifecycle.rs",
    "user/shell/src/cmd.rs",
)


class Pr2FileEventCoreGuardTests(unittest.TestCase):
    def fixture(self) -> Path:
        temp = Path(tempfile.mkdtemp(prefix="orays-pr2-check-"))
        self.addCleanup(shutil.rmtree, temp)
        for relative in FILES:
            destination = temp / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)
        return temp

    def mutate(self, root: Path, relative: str, old: str, new: str) -> None:
        path = root / relative
        text = path.read_text(encoding="utf-8")
        self.assertIn(old, text, f"mutation anchor missing: {relative}: {old!r}")
        path.write_text(text.replace(old, new, 1), encoding="utf-8")

    def run_check(self, root: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["python3", str(CHECK), "--root", str(root)],
            check=False,
            text=True,
            capture_output=True,
        )

    def assert_rejected(self, root: Path, message: str) -> None:
        result = self.run_check(root)
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn(message, result.stdout)

    def test_repository_passes(self) -> None:
        result = self.run_check(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_detects_feature_sensitive_axfile_lock(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "kernel/fs/axfile/src/lib.rs",
            "use axsync::spin::SpinNoIrq as EventRegistryMutex;",
            "use axsync::Mutex as EventRegistryMutex;",
        )
        self.assert_rejected(
            root,
            "axfile event-registry locks must remain feature-invariant non-sleeping locks",
        )

    def test_detects_exec_split_bypassing_precommit_path(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/process_lifecycle.rs",
            ".unshare_for_pid_if_shared(process.pid())",
            ".unshare_after_image_commit(process.pid())",
        )
        self.assert_rejected(root, "split CLONE_FILES before committing")

    def test_detects_partial_fork_copy_socket_leak(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "for copied in slots.iter().flatten() {\n"
            "                                if let FdEntry::Socket(socket) = &copied.entry {\n"
            "                                    let _ = socket.close();",
            "for copied in slots.iter().flatten() {\n"
            "                                if let FdEntry::Socket(socket) = &copied.entry {\n"
            "                                    let _leaked_socket = socket;",
        )
        self.assert_rejected(root, "must close already duplicated raw sockets")

    def test_detects_failed_dup_install_socket_leak(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "fn discard_uninstalled_entry(entry: FdEntry) {\n"
            "    // SocketEntry wraps a raw POSIX descriptor and intentionally has no Drop\n"
            "    // implementation: installed slots are closed by the FdTable close path.\n"
            "    // A prepared duplicate that never reaches a slot therefore needs explicit\n"
            "    // rollback, while all RAII-backed variants only need to be dropped here.\n"
            "    if let FdEntry::Socket(socket) = &entry {\n"
            "        let _ = socket.close();",
            "fn discard_uninstalled_entry(entry: FdEntry) {\n"
            "    // SocketEntry wraps a raw POSIX descriptor and intentionally has no Drop\n"
            "    // implementation: installed slots are closed by the FdTable close path.\n"
            "    // A prepared duplicate that never reaches a slot therefore needs explicit\n"
            "    // rollback, while all RAII-backed variants only need to be dropped here.\n"
            "    if let FdEntry::Socket(socket) = &entry {\n"
            "        let _leaked_socket = socket;",
        )
        self.assert_rejected(root, "must roll back raw legacy socket ownership")

    def test_detects_socket_insert_double_close(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_socket.rs",
            "        Err(err) => neg_errno(err),",
            "        Err(err) => {\n"
            "            let _ = arceos_posix_api::sys_close(posix_fd);\n"
            "            neg_errno(err)\n"
            "        },",
        )
        self.assert_rejected(root, "must not double-close table-owned raw descriptors")

    def test_detects_arc_count_used_for_clone_files(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/process_lifecycle.rs",
            "let share_fds = clone_flags & general::CLONE_FILES as usize != 0;",
            "let share_fds = Arc::strong_count(&process.fds) > 1;",
        )
        self.assert_rejected(root, "must not use Arc::strong_count")

    def test_detects_legacy_epoll_fork_lease_removal(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "LegacyEpollTarget::duplicate(&target_slot.entry)?",
            "legacy_fd_only_target(fd, target_slot.description_id())",
        )
        self.assert_rejected(root, "fork-stable compatibility lease")

    def test_detects_legacy_target_drop_under_fd_table_lock(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "drop(update.retired);\n            update.epoll.state.wake();",
            "update.epoll.state.wake();\n            drop(update.retired);",
        )
        self.assert_rejected(root, "must drop after the FD-table guard")

    def test_detects_shared_edge_generation(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "notification: AtomicU64,",
            "notification: u64,",
        )
        self.assert_rejected(root, "relevant per-registration notifications")

    def test_detects_cross_class_edge_retrigger(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "if !events.intersects(self.interests) {",
            "if events.is_empty() {",
        )
        self.assert_rejected(root, "only relevant per-registration notifications")

    def test_detects_non_atomic_one_shot_delivery(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "registration.delivery.claim(",
            "snapshot.delivery.claim(",
        )
        self.assert_rejected(root, "atomically claim")

    def test_detects_stale_scan_removing_newer_mod(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "stale.push((snapshot.key, snapshot.version));",
            "stale.push((snapshot.key, 0));",
        )
        self.assert_rejected(root, "must not remove a newer MOD")

    def test_detects_deadline_first_poll_path(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/select_fdset.rs",
            "decide_level_wait(ready, deadline.is_some_and",
            "legacy_deadline_first(ready, deadline.is_some_and",
        )
        self.assert_rejected(root, "poll does not route final readiness")

    def test_detects_epoll_readiness_under_fd_table_lock(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "FdTable::epoll_collect_ready_for(&epoll, maxevents, &mut ready)",
            "process.fds.lock_for_pid(process.pid()).epoll_collect_ready_for("
            "&epoll, maxevents, &mut ready)",
        )
        self.assert_rejected(root, "without an FD-table guard")

    def test_detects_unserialized_nested_epoll_graph(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "let graph_guard = epoll_graph_lock().lock();",
            "let graph_guard = unlocked_epoll_graph();",
        )
        self.assert_rejected(root, "must hold the global graph lock")

    def test_detects_reentrant_epoll_callback_path(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "self.notifying.try_enter()",
            "legacy_recursive_notify()",
        )
        self.assert_rejected(root, "must coalesce concurrent notifications")

    def test_detects_lost_nested_epoll_notification_round(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "notification_guard.finish_round()",
            "notification_guard.release_without_replay()",
        )
        self.assert_rejected(root, "must coalesce concurrent notifications")

    def test_detects_reciprocal_tee_lock_order_regression(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_pipe.rs",
            "with_ordered_arc_mutex_pair(",
            "with_source_first_mutex_pair(",
        )
        self.assert_rejected(root, "canonical reciprocal lock-order helper")

    def test_detects_mutating_timer_wait_profile(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            "readiness_deadline_delay(state.expirations > 0, state.deadline, now)",
            "self.refresh_silent().ok().map(|_| Duration::ZERO)",
        )
        self.assert_rejected(root, "wait profiling must be non-mutating")

    def test_detects_missing_last_close_cleanup(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_table.rs",
            ".close_events()",
            ".stale_close_events()",
        )
        self.assert_rejected(root, "last-description close must remove")

    def test_detects_pipe_async_state_shared_between_open_descriptions(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_pipe.rs",
            "let write_async_state = Arc::new(Mutex::new(PipeAsyncState::new(false)));",
            "let write_async_state = read_async_state.clone();",
        )
        self.assert_rejected(root, "state must be local to each open description")

    def test_detects_user_copy_under_pipe_async_lock(self) -> None:
        root = self.fixture()
        self.mutate(
            root,
            "user/shell/src/uspace/fd_pipe.rs",
            "F_GETOWN_EX => {\n                let owner = {",
            "F_GETOWN_EX => {\n                let _held = self.async_state.lock();\n                let owner = {",
        )
        self.assert_rejected(root, "release the async-state lock before user-memory copy")

    def test_detects_pr2_tests_removed_from_make_unittest(self) -> None:
        root = self.fixture()
        self.mutate(root, "Makefile", "unittest: pr2-check", "unittest:")
        self.assert_rejected(root, "not wired into both unittest targets")


if __name__ == "__main__":
    unittest.main()
