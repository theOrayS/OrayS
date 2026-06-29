#!/usr/bin/env python3
"""Regression tests for the G010 real-kernel-semantics guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_g010_real_kernel_semantics.py"
TARGETS = [
    Path("api/arceos_posix_api/Cargo.toml"),
    Path("api/arceos_posix_api/src/imp/fd_ops.rs"),
    Path("api/arceos_posix_api/src/imp/fs.rs"),
    Path("api/arceos_posix_api/src/imp/stdio.rs"),
    Path("api/arceos_posix_api/src/imp/pipe.rs"),
    Path("api/arceos_posix_api/src/imp/io_mpx/epoll.rs"),
    Path("api/arceos_posix_api/src/imp/net.rs"),
    Path("api/arceos_posix_api/src/lib.rs"),
    Path("api/arceos_posix_api/src/signal.rs"),
    Path("kernel/task/axtask/src/api.rs"),
    Path("kernel/task/axtask/src/run_queue.rs"),
    Path("kernel/task/axtask/src/task.rs"),
    Path("vendor/cargo/axsched/src/cfs.rs"),
    Path("vendor/cargo/axsched/src/round_robin.rs"),
    Path("vendor/cargo/axsched/src/fifo.rs"),
    Path("ulib/axlibc/include/stdio.h"),
    Path("ulib/axlibc/c/stdio.c"),
    Path("ulib/axlibc/c/syslog.c"),
    Path("ulib/axlibc/c/unistd.c"),
    Path("ulib/axlibc/c/locale.c"),
    Path("ulib/axlibc/c/pwd.c"),
    Path("ulib/axlibc/c/env.c"),
    Path("user/shell/src/uspace/mod.rs"),
    Path("user/shell/src/uspace/memory_map.rs"),
    Path("user/shell/src/uspace/process_lifecycle.rs"),
    Path("user/shell/src/uspace/resource_sched.rs"),
    Path("user/shell/src/uspace/fd_table.rs"),
]


class G010RealKernelSemanticsGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="g010-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(encoding="utf-8"), encoding="utf-8")
        return tmp

    def run_guard(self, tree: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(GUARD), "--root", str(tree)],
            check=False,
            capture_output=True,
            text=True,
        )

    def replace_once(self, text: str, old: str, new: str) -> str:
        self.assertIn(old, text, f"test fixture drifted; missing mutation target: {old!r}")
        return text.replace(old, new, 1)

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_fixed_stdio_inode(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/stdio.rs"
        text = path.read_text(encoding="utf-8").replace("st_ino,", "st_ino: 1,", 1)
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("st_ino 1", result.stdout)

    def test_detects_pipe_fabricated_owner(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/pipe.rs"
        text = path.read_text(encoding="utf-8").replace("st_uid: 0,", "st_uid: 1000,", 1)
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("uid/gid 1000", result.stdout)

    def test_detects_regular_file_open_flags_dropped(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/fs.rs"
        text = self.replace_once(
            path.read_text(encoding="utf-8"),
            "File::new(file, filename, flags).add_to_fd_table(open_fd_flags(flags))",
            "File::new(file, filename, 0).add_to_fd_table(open_fd_flags(flags))",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("regular file F_GETFL", result.stdout)

    def test_detects_f_setfl_only_toggling_nonblock_fake_success(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/fd_ops.rs"
        text = path.read_text(encoding="utf-8").replace(
            "ctypes::F_SETFL => get_file_like(fd)?.set_status_flags(arg as c_int).map(|_| 0),",
            """ctypes::F_SETFL => {
                get_file_like(fd)?.set_nonblocking(arg & (ctypes::O_NONBLOCK as usize) > 0)?;
                Ok(0)
            }""",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("F_SETFL", result.stdout)

    def test_detects_regular_file_append_state_dropped_by_setfl(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/fs.rs"
        text = path.read_text(encoding="utf-8").replace(
            "let mutable = flags & (ctypes::O_APPEND | ctypes::O_NONBLOCK) as c_int;",
            "let mutable = flags & ctypes::O_NONBLOCK as c_int;",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("regular file F_GETFL", result.stdout)

    def test_detects_regular_file_append_write_without_seek_end(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/fs.rs"
        text = path.read_text(encoding="utf-8").replace(
            """        if self.status_flags.load(Ordering::Acquire) & ctypes::O_APPEND as c_int != 0 {
            inner.seek(SeekFrom::End(0))?;
        }
""",
            "",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("O_APPEND", result.stdout)

    def test_detects_stdio_access_mode_zero_for_stdout(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/stdio.rs"
        text = path.read_text(encoding="utf-8").replace(
            "Ok(crate::ctypes::O_WRONLY as c_int)",
            "Ok(0)",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("stdio F_GETFL", result.stdout)

    def test_detects_socket_access_mode_zero(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/net.rs"
        text = path.read_text(encoding="utf-8").replace(
            "let mut flags = ctypes::O_RDWR as c_int;",
            "let mut flags = 0;",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("socket F_GETFL", result.stdout)

    def test_detects_pipe_nonblocking_rejected_after_pipe2_success(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/pipe.rs"
        text = self.replace_once(
            path.read_text(encoding="utf-8"),
            """        self.nonblocking.store(nonblocking, Ordering::Release);
        if nonblocking {
            self.notify_readable();
            self.notify_writable();
        }
        Ok(())
""",
            "if nonblocking {\n            Err(LinuxError::EOPNOTSUPP)\n        } else {\n            Ok(())\n        }",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("O_NONBLOCK", result.stdout)

    def test_detects_pipe_write_without_reader_fake_success(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/pipe.rs"
        text = path.read_text(encoding="utf-8").replace(
            "    pub fn read_end_close(&self) -> bool {\n        self.peer_counts.readers.load(Ordering::Acquire) == 0\n    }\n",
            "",
        )
        text = text.replace(
            """            if self.read_end_close() {
                return if write_size == 0 {
                    Err(LinuxError::EPIPE)
                } else {
                    Ok(write_size)
                };
            }
""",
            "",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("EPIPE", result.stdout)

    def test_detects_api_pipe_epipe_without_sigpipe_hook(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/pipe.rs"
        text = path.read_text(encoding="utf-8").replace(
            "        self.notify_read_end_closed();\n",
            "",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SIGPIPE", result.stdout)

    def test_detects_missing_host_sigpipe_delivery(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/mod.rs"
        text = path.read_text(encoding="utf-8").replace(
            "signal_abi::deliver_user_signal(&entry, linux_abi::SIGPIPE_NUM, ext.process.pid()).is_ok()",
            "false",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("SIGPIPE", result.stdout)

    def test_detects_pipe_poll_hiding_eof(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/pipe.rs"
        text = path.read_text(encoding="utf-8").replace(
            "readable: self.readable() && (buf.available_read() > 0 || self.write_end_close()),",
            "readable: self.readable() && buf.available_read() > 0,",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("poll", result.stdout)

    def test_detects_directionless_pipe_peer_count(self) -> None:
        tree = self.make_tree()
        path = tree / "api/arceos_posix_api/src/imp/pipe.rs"
        text = path.read_text(encoding="utf-8").replace(
            "self.peer_counts.writers.load(Ordering::Acquire) == 0",
            "Arc::strong_count(&self.buffer) == 1",
            1,
        )
        text = text.replace(
            "self.peer_counts.readers.load(Ordering::Acquire) == 0",
            "Arc::strong_count(&self.buffer) == 1",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("strong_count", result.stdout)

    def test_detects_pipe2_ignored_fcntl_failure(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/unistd.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("int pipe2(")
        end = text.index("\n#endif // AX_CONFIG_PIPE", start)
        replacement = """int pipe2(int fd[2], int flag)
{
    if (!flag)
        return pipe(fd);
    if (flag & ~(O_CLOEXEC | O_NONBLOCK))
        return -EINVAL;

    int res = pipe(fd);
    if (res != 0)
        return res;
    fcntl(fd[0], F_SETFL, O_NONBLOCK);
    fcntl(fd[1], F_SETFL, O_NONBLOCK);
    return 0;
}
"""
        path.write_text(text[:start] + replacement + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("pipe2", result.stdout)

    def test_detects_stdio_feof_enosys_stub(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8").replace(
            "return f && (f->flags & F_EOF) ? 1 : 0;",
            "unimplemented();\n    errno = ENOSYS;\n    return -1;",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("feof", result.stdout)

    def test_detects_stdio_fclose_without_flush_free(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("int fclose(")
        end = text.index("\nint fileno(", start)
        replacement = """int fclose(FILE *f)
{
    return close(f->fd);
}
"""
        path.write_text(text[:start] + replacement + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("fclose", result.stdout)

    def test_detects_stdio_fflush_clearing_partial_write(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("static int __write_buffer(")
        end = text.index("\nstatic int __fflush(", start)
        replacement = """static int __write_buffer(FILE *f)
{
    int r = write(f->fd, f->buf, f->buffer_len);
    f->buffer_len = 0;
    return r >= 0 ? 0 : r;
}
"""
        path.write_text(text[:start] + replacement + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("short writes", result.stdout)

    def test_detects_stdio_getc_enosys_stub(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8").replace(
            "return getc(stdin);",
            "unimplemented();\n    errno = ENOSYS;\n    return EOF;",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("getchar", result.stdout)

    def test_detects_stdio_getc_inside_fs_gate(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("int getc(FILE *f)")
        end = text.index("\nint fflush(", start)
        getc_block = text[start:end] + "\n"
        text = text[:start] + text[end + 1 :]
        fs_gate = text.index("#ifdef AX_CONFIG_FS")
        text = text[:fs_gate] + "#ifdef AX_CONFIG_FS\n\n" + getc_block + text[fs_gate + len("#ifdef AX_CONFIG_FS\n") :]
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no-FS", result.stdout)

    def test_detects_stdio_fdopen_enosys_stub(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8")
        old = "return __new_file_for_fd(fd);"
        pos = text.rindex(old)
        text = text[:pos] + "unimplemented();\n    errno = ENOSYS;\n    return NULL;" + text[pos + len(old) :]
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("fdopen", result.stdout)

    def test_detects_stdio_fdopen_without_mode_check(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8").replace(
            """    if (!__fd_mode_compatible(fd_flags, mode_flags)) {
        errno = EBADF;
        return NULL;
    }
""",
            "",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("fd", result.stdout)

    def test_detects_stdio_fdopen_append_without_real_append(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdio.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("    if ((mode_flags & O_APPEND) && !(fd_flags & O_APPEND)) {")
        end = text.index("    return __new_file_for_fd(fd);", start)
        text = text[:start] + text[end:]
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("append", result.stdout)

    def test_detects_syslog_unimplemented_noop(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/syslog.c"
        text = path.read_text(encoding="utf-8").replace("vfprintf(stderr, __fmt ? __fmt : \"\", ap);", "unimplemented();")
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("syslog", result.stdout)

    def test_detects_syslog_default_stderr_facade(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/syslog.c"
        text = path.read_text(encoding="utf-8").replace(
            "    if (!(syslog_option & LOG_PERROR))\n        return;\n\n",
            "",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("LOG_PERROR", result.stdout)

    def test_detects_locale_unimplemented_stub(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/locale.c"
        text = path.read_text(encoding="utf-8").replace(
            "if (!__locale)\n        return current_locale;",
            "unimplemented();\n    errno = ENOSYS;\n    return NULL;",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("locale.c setlocale", result.stdout)

    def test_detects_pwd_enosys_stub(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/pwd.c"
        text = path.read_text(encoding="utf-8").replace(
            "return fill_root_passwd(pw, buf, size, res);",
            "return ENOSYS;",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("pwd.c getpwnam_r", result.stdout)

    def test_detects_dummy_environment_seed(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/env.c"
        text = path.read_text(encoding="utf-8").replace(
            "static char *initial_environ[] = {NULL};",
            'char *environ_[2] = {"dummy", NULL};',
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("dummy", result.stdout)

    def test_detects_environment_without_external_adoption(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/env.c"
        text = path.read_text(encoding="utf-8").replace(
            "    if (env_ensure_owned() != 0)\n        return -1;\n\n",
            "",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("setenv", result.stdout)

    def test_detects_scheduler_readback_only_facade(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/resource_sched.rs"
        text = path.read_text(encoding="utf-8").replace(
            "    apply_task_scheduler_state(&entry.task, &entry.process, state);\n",
            "    Ok(())\n",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("scheduler backend", result.stdout)

    def test_detects_setpriority_noncurrent_readback_only(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/resource_sched.rs"
        text = self.replace_once(
            path.read_text(encoding="utf-8"),
            "    apply_task_scheduler_state(&entry.task, &entry.process, state);\n",
            "",
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("non-current live targets", result.stdout)

    def test_detects_scheduler_backend_failure_regression(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/resource_sched.rs"
        text = path.read_text(encoding="utf-8").replace(
            "let _ = axtask::set_task_priority(task, scheduler_backend_priority(process, state));",
            "if !axtask::set_task_priority(task, scheduler_backend_priority(process, state)) {\n        return Err(LinuxError::EINVAL);\n    }",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("best-effort", result.stdout)

    def test_detects_cfs_priority_without_rekey(self) -> None:
        tree = self.make_tree()
        path = tree / "vendor/cargo/axsched/src/cfs.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("    fn set_priority(&mut self, task: &Self::SchedItem, prio: isize) -> bool {")
        end = text.index("\n    }\n}", start) + len("\n    }")
        replacement = """    fn set_priority(&mut self, task: &Self::SchedItem, prio: isize) -> bool {
        if (-20..=19).contains(&prio) {
            task.set_priority(prio);
            true
        } else {
            false
        }
    }"""
        path.write_text(text[:start] + replacement + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("CFS set_priority", result.stdout)

    def test_detects_rr_priority_false_stub(self) -> None:
        tree = self.make_tree()
        path = tree / "vendor/cargo/axsched/src/round_robin.rs"
        text = path.read_text(encoding="utf-8")
        start = text.index("    fn set_priority(&mut self, task: &Self::SchedItem, prio: isize) -> bool {")
        end = text.index("\n    }\n}", start) + len("\n    }")
        replacement = """    fn set_priority(&mut self, _task: &Self::SchedItem, _prio: isize) -> bool {
        false
    }"""
        path.write_text(text[:start] + replacement + text[end:], encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("round_robin", result.stdout)

    def test_detects_ltp_case_specific_commentary(self) -> None:
        tree = self.make_tree()
        path = tree / "user/shell/src/uspace/fd_table.rs"
        text = path.read_text(encoding="utf-8") + "\n// open10-specific compatibility must not be reintroduced\n"
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("open10", result.stdout)


if __name__ == "__main__":
    unittest.main()
