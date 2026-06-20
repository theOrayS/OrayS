#!/usr/bin/env python3
"""Regression tests for the G012 syscall review hotspot guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_g012_syscall_review_hotspots.py"
TARGETS = [
    Path("examples/shell/src/uspace/mod.rs"),
    Path("examples/shell/src/uspace/futex.rs"),
    Path("examples/shell/src/uspace/signal_abi.rs"),
    Path("examples/shell/src/uspace/memory_map.rs"),
    Path("examples/shell/src/uspace/process_lifecycle.rs"),
    Path("examples/shell/src/uspace/task_registry.rs"),
    Path("examples/shell/src/uspace/task_context.rs"),
    Path("examples/shell/src/uspace/user_memory.rs"),
    Path("examples/shell/src/uspace/mount_abi.rs"),
    Path("examples/shell/src/uspace/fd_table.rs"),
    Path("examples/shell/src/uspace/system_info.rs"),
    Path("examples/shell/src/uspace/time_abi.rs"),
    Path("examples/shell/src/uspace/resource_sched.rs"),
    Path("examples/shell/src/uspace/linux_abi.rs"),
    Path("examples/shell/src/uspace/process_abi.rs"),
    Path("api/arceos_posix_api/src/imp/pthread/mod.rs"),
]


class G012SyscallReviewHotspotGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="g012-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(encoding="utf-8"), encoding="utf-8")
        return tmp

    def run_guard(self, tree: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run([sys.executable, str(GUARD), "--root", str(tree)], check=False, capture_output=True, text=True)

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_empty_log_read_cstr(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/user_memory.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn log_read_cstr_efault")
        text = text[:start] + "fn log_read_cstr_efault() {\n}\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("log_read_cstr_efault", result.stdout)

    def test_detects_empty_central_user_trace(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/mod.rs"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "let _ = core::format_args!($($arg)*);",
                "",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("user_trace", result.stdout)

    def test_detects_local_user_trace_shadow(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/futex.rs"
        path.write_text(
            path.read_text(encoding="utf-8") + "\nmacro_rules! user_trace { ($($arg:tt)*) => {}; }\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("local empty user_trace", result.stdout)

    def test_detects_mount_root_alias(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/mount_abi.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            'axfs::api::mount_fatfs(mount_path, dev, format).map_err(LinuxError::from)?;\n            Ok(target_path.into())',
            'Ok("/".into())',
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("mount", result.stdout)

    def test_detects_fsync_catch_all_success(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("pub(super) fn sys_fsync")
        end = text.index("pub(super) fn sys_renameat2", start)
        block = text[start:end].replace(
            "Err(err) => neg_errno(err),",
            "Ok(_) => 0,\n        Err(err) => neg_errno(err),",
            1,
        )
        path.write_text(text[:start] + block + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sys_fsync", result.stdout)

    def test_detects_openat_unknown_flag_acceptance(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "    if flags & !supported_open_flags() != 0 {\n        return Err(LinuxError::EINVAL);\n    }\n",
            "",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("open_fd_entry", result.stdout)

    def test_detects_siocsifflags_validate_success(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn socket_ioctl_set_ifflags")
        end = text.index("fn write_user_bytes_ret", start)
        fake_success = """fn socket_ioctl_set_ifflags(process: &UserProcess, arg: usize) -> isize {
    const IFREQ_SIZE: usize = 40;
    if arg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    match validate_user_read(process, arg, IFREQ_SIZE) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

"""
        path.write_text(text[:start] + fake_success + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SIOCSIFFLAGS", result.stdout)

    def test_detects_syslog_privileged_noop(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        path.write_text(path.read_text(encoding="utf-8") + "\n// PrivilegedNoop\n", encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("PrivilegedNoop", result.stdout)

    def test_detects_syslog_write_only_state(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        path.write_text(
            path.read_text(encoding="utf-8") + "\nstatic SYSLOG_OPEN: usize = 0;\n",
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SYSLOG", result.stdout)

    def test_detects_syslog_control_state_without_snapshot_consumer(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("KLOG_CONTROL_STATE.open.load", "KLOG_CONTROL_STATE.open_no_consumer")
        text = text.replace(
            "KLOG_CONTROL_STATE.console_enabled.load",
            "KLOG_CONTROL_STATE.console_enabled_no_consumer",
        )
        text = text.replace(
            "KLOG_CONTROL_STATE.clear_generation.load",
            "KLOG_CONTROL_STATE.clear_generation_no_consumer",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("klog snapshot", result.stdout)

    def test_detects_syslog_missing_privilege_gate(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn privileged_syslog_control")
        end = text.index("pub(super) fn sys_getcpu", start)
        block = text[start:end].replace("LinuxError::EPERM", "LinuxError::EINVAL", 1)
        path.write_text(text[:start] + block + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("privileged control", result.stdout)

    def test_detects_syslog_state_action_success_arm(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/system_info.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(
            text.replace(
                "SyslogAction::Close | SyslogAction::Open => {\n            privileged_syslog_control(process, syslog_action(log_type), len)\n        }",
                "SyslogAction::Close | SyslogAction::Open => 0,",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SyslogAction::Close", result.stdout)

    def test_detects_times_half_split(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/time_abi.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("pub(super) fn process_times")
        end = text.index("pub(super) fn sys_times", start)
        fake_split = """pub(super) fn process_times(process: &UserProcess) -> Tms {
    let elapsed = clock_ticks_now()
        .saturating_sub(process.start_clock_ticks.load(Ordering::Acquire))
        .min(c_long::MAX as u64) as c_long;
    let user_ticks = elapsed / 2;
    let system_ticks = elapsed.saturating_sub(user_ticks);
    Tms {
        tms_utime: user_ticks,
        tms_stime: system_ticks,
        tms_cutime: 0,
        tms_cstime: 0,
    }
}

"""
        path.write_text(text[:start] + fake_split + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("process_times", result.stdout)

    def test_detects_madvise_dontfork_without_tracked_metadata_gate(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/memory_map.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(
            text.replace(
                "            if !madvise_range_is_tracked(process, addr, end) {\n                return neg_errno(LinuxError::ENOMEM);\n            }\n",
                "",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("MADV_DONTFORK", result.stdout)

    def test_detects_mremap_metadata_reset(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/memory_map.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(
            text.replace(
                "process.record_mmap_region_entry(region);",
                "process.record_mmap_region(region.start, region.size, region.prot, region.shared, region.anonymous, region.locked, region.grow_down, region.may_write, region.file_backing);",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("mremap_shrink_in_place", result.stdout)

    def test_detects_mremap_sigbus_metadata_drop(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/memory_map.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(
            text.replace("    process.record_mmap_sigbus_ranges(preserved_sigbus);\n", "", 1),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SIGBUS", result.stdout)

    def test_detects_futex_requeue_total_return(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/futex.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(
            text.replace(
                "Ok((woken, _requeued)) => woken as isize",
                "Ok((woken, requeued)) => woken.saturating_add(requeued) as isize",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("FUTEX_REQUEUE", result.stdout)

    def test_detects_wait4_ignored_rusage(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/process_lifecycle.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(text.replace("    rusage: usize,\n) -> isize {\n", "    _rusage: usize,\n) -> isize {\n", 1), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sys_wait4", result.stdout)

    def test_detects_pthread_registration_barrier_removal(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/pthread/mod.rs"
        text = path.read_text(encoding="utf-8")
        path.write_text(
            text.replace("        registration_ready.store(true, Ordering::Release);\n", "", 1),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("pthread_create", result.stdout)

    def test_detects_runtime_unregister_before_accounting(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/task_registry.rs"
        text = path.read_text(encoding="utf-8")
        old = """    process
        .completed_thread_runtime_ticks
        .fetch_add(runtime_ticks, Ordering::AcqRel);
    table.remove(&tid);
"""
        new = """    table.remove(&tid);
    process
        .completed_thread_runtime_ticks
        .fetch_add(runtime_ticks, Ordering::AcqRel);
"""
        path.write_text(text.replace(old, new, 1), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unregister_user_task_with_runtime", result.stdout)

    def test_detects_sched_deadline_attribute_drop(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/resource_sched.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn sched_state_from_attr")
        end = text.index("pub(super) fn sys_sched_getattr", start)
        block = text[start:end].replace("sched_runtime: attr.sched_runtime", "sched_runtime: 0", 1)
        path.write_text(text[:start] + block + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sched_setattr SCHED_DEADLINE", result.stdout)

    def test_detects_sched_deadline_normal_priority_backend(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/resource_sched.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("fn deadline_scheduler_backend_priority")
        end = text.index("fn apply_task_scheduler_state", start)
        text = text[:start] + text[end:]
        text = text.replace(
            "general::SCHED_DEADLINE => deadline_scheduler_backend_priority(state),",
            "general::SCHED_DEADLINE => process.nice() as isize,",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SCHED_DEADLINE", result.stdout)

    def test_detects_sched_setscheduler_deadline_param_acceptance(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/resource_sched.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "general::SCHED_BATCH | general::SCHED_IDLE if param.sched_priority == 0 => true,",
            "general::SCHED_BATCH | general::SCHED_IDLE | general::SCHED_DEADLINE if param.sched_priority == 0 => true,",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sched_param-only", result.stdout)

    def test_detects_personality_mask_acceptance(self) -> None:
        tree = self.make_tree()
        linux_abi = tree / "examples/shell/src/uspace/linux_abi.rs"
        process_abi = tree / "examples/shell/src/uspace/process_abi.rs"
        linux_abi.write_text(
            linux_abi.read_text(encoding="utf-8").replace(
                "pub(super) const PER_LINUX: usize = 0;",
                "pub(super) const LINUX_PERSONALITY_MASK: usize = 0xffff_ffff;",
                1,
            ),
            encoding="utf-8",
        )
        process_abi.write_text(
            process_abi.read_text(encoding="utf-8").replace(
                "let persona = validate_personality(persona)?;\n        process.set_personality(persona);",
                "process.set_personality(persona & LINUX_PERSONALITY_MASK);",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("personality", result.stdout)

    def test_detects_adjtimex_field_only_discipline(self) -> None:
        tree = self.make_tree()
        path = tree / "examples/shell/src/uspace/time_abi.rs"
        text = path.read_text(encoding="utf-8")
        text = text.replace("discipline_extra_ns_for_raw", "field_only_timex_state")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("adjtimex", result.stdout)


if __name__ == "__main__":
    unittest.main()
