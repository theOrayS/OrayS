#!/usr/bin/env python3
"""Static guard for G010 remaining synthetic compatibility semantics."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def rust_function_block(text: str, name: str) -> str:
    markers = [f"fn {name}", f"pub(super) fn {name}", f"pub fn {name}"]
    start = min((pos for marker in markers if (pos := text.find(marker)) >= 0), default=-1)
    if start < 0:
        return ""
    candidates = [
        pos
        for pos in (
            text.find("\nfn ", start + 1),
            text.find("\npub fn ", start + 1),
            text.find("\npub(super) fn ", start + 1),
            text.find("\n#[", start + 1),
        )
        if pos >= 0
    ]
    return text[start : min(candidates) if candidates else len(text)]


def c_function_block(text: str, name: str) -> str:
    match = re.search(rf"(?:^|\n)[A-Za-z_][\w\s\*]*\*?\s*{re.escape(name)}\s*\([^;]*\)\s*\{{", text)
    if not match:
        return ""
    start = match.start()
    brace = text.find("{", match.start())
    depth = 0
    for idx in range(brace, len(text)):
        ch = text[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[start : idx + 1]
    return text[start:]


def scan_api_fd_metadata(root: Path) -> list[str]:
    findings: list[str] = []
    targets = {
        "api/arceos_posix_api/src/imp/stdio.rs": (
            "stdio fd stat must use device/rdev constants and object-derived inodes",
            ("STDIO_STAT_DEV", "STDIN_RDEV", "STDOUT_RDEV", "self.inner as *const"),
        ),
        "api/arceos_posix_api/src/imp/pipe.rs": (
            "pipe fstat must identify the shared pipe object instead of fixed placeholders",
            ("PIPE_STAT_DEV", "PIPE_STAT_BLKSIZE", "Arc::as_ptr(&self.buffer)"),
        ),
        "api/arceos_posix_api/src/imp/io_mpx/epoll.rs": (
            "epoll fstat must identify the epoll instance instead of fixed placeholders",
            ("EPOLL_STAT_DEV", "EPOLL_STAT_BLKSIZE", "self as *const Self"),
        ),
    }
    for rel, (desc, tokens) in targets.items():
        text = read(root / rel)
        if re.search(r"st_ino:\s*1\s*,", text):
            findings.append(f"{rel}: {desc}; found fixed st_ino 1")
        for token in tokens:
            if token not in text:
                findings.append(f"{rel}: {desc}; missing {token}")
    pipe = read(root / "api/arceos_posix_api/src/imp/pipe.rs")
    if "st_uid: 1000" in pipe or "st_gid: 1000" in pipe:
        findings.append("pipe fstat must not fabricate uid/gid 1000 ownership")
    return findings


def scan_fd_status_flags(root: Path) -> list[str]:
    findings: list[str] = []
    fd_ops = read(root / "api/arceos_posix_api/src/imp/fd_ops.rs")
    fs = read(root / "api/arceos_posix_api/src/imp/fs.rs")
    stdio = read(root / "api/arceos_posix_api/src/imp/stdio.rs")
    pipe = read(root / "api/arceos_posix_api/src/imp/pipe.rs")
    epoll = read(root / "api/arceos_posix_api/src/imp/io_mpx/epoll.rs")
    net = read(root / "api/arceos_posix_api/src/imp/net.rs")

    for token in (
        "status_flags: AtomicI32",
        "open_status_flags(flags)",
        "ctypes::O_ACCMODE | ctypes::O_APPEND | ctypes::O_NONBLOCK",
        "File::new(file, filename, flags)",
        "self.status_flags.load(Ordering::Acquire)",
        "self.status_flags.fetch_or(mask, Ordering::AcqRel)",
        "self.status_flags.fetch_and(!mask, Ordering::AcqRel)",
        "fn set_status_flags(&self, flags: c_int) -> LinuxResult",
        "let allowed = (ctypes::O_ACCMODE | ctypes::O_APPEND | ctypes::O_NONBLOCK) as c_int",
        "let mutable = flags & (ctypes::O_APPEND | ctypes::O_NONBLOCK) as c_int",
        "self.status_flags.store(access | mutable, Ordering::Release)",
        "SeekFrom::End(0)",
    ):
        if token not in fs:
            findings.append(f"regular file F_GETFL must preserve open access/status flags; missing {token}")
    file_write = rust_function_block(fs, "write")
    if "SeekFrom::End(0)" not in file_write or "ctypes::O_APPEND" not in file_write:
        findings.append("regular file O_APPEND must seek to end before every write, not only report F_GETFL state")
    for token in (
        "fn set_status_flags(&self, flags: c_int) -> LinuxResult",
        "LinuxError::EOPNOTSUPP",
        "self.set_nonblocking(flags & ctypes::O_NONBLOCK as c_int != 0)",
        "ctypes::F_SETFL => get_file_like(fd)?.set_status_flags(arg as c_int).map(|_| 0)",
    ):
        if token not in fd_ops:
            findings.append(f"F_SETFL must update or reject status flags instead of returning fake success; missing {token}")
    f_setfl = rust_function_block(fd_ops, "sys_fcntl")
    if "set_nonblocking(arg" in f_setfl:
        findings.append("F_SETFL must not only toggle O_NONBLOCK and then return success for dropped status flags")
    for token in (
        "Ok(crate::ctypes::O_RDONLY as c_int)",
        "Ok(crate::ctypes::O_WRONLY as c_int)",
    ):
        if token not in stdio:
            findings.append(f"stdio F_GETFL must expose stdin/stdout access modes for fdopen; missing {token}")
    for token in (
        "ctypes::O_RDONLY as c_int",
        "ctypes::O_WRONLY as c_int",
        "flags |= ctypes::O_NONBLOCK as c_int",
    ):
        if token not in pipe:
            findings.append(f"pipe F_GETFL must expose endpoint access mode plus O_NONBLOCK; missing {token}")
    if "Ok(ctypes::O_RDWR as c_int)" not in epoll:
        findings.append("epoll F_GETFL must expose an O_RDWR anonymous-inode access mode instead of fixed zero")
    for token in (
        "let mut flags = ctypes::O_RDWR as c_int",
        "flags |= ctypes::O_NONBLOCK as c_int",
    ):
        if token not in net:
            findings.append(f"socket F_GETFL must expose O_RDWR plus O_NONBLOCK state; missing {token}")
    return findings


def scan_pipe_nonblocking_semantics(root: Path) -> list[str]:
    findings: list[str] = []
    cargo = read(root / "api/arceos_posix_api/Cargo.toml")
    lib = read(root / "api/arceos_posix_api/src/lib.rs")
    signal = read(root / "api/arceos_posix_api/src/signal.rs")
    pipe = read(root / "api/arceos_posix_api/src/imp/pipe.rs")
    unistd = read(root / "ulib/axlibc/c/unistd.c")
    shell_uspace = read(root / "user/shell/src/uspace/mod.rs")
    for token in (
        'crate_interface = { version = "0.3", features = ["weak_default"] }',
        "#![feature(linkage)]",
        "pub use signal::PosixSignalIf",
        "#[crate_interface::def_interface]",
        "pub trait PosixSignalIf",
        "fn raise_sigpipe() -> bool",
        "false",
        "crate_interface::call_interface!(PosixSignalIf::raise_sigpipe)",
        "struct PosixSignalIfImpl",
        "impl arceos_posix_api::PosixSignalIf for PosixSignalIfImpl",
        "signal_abi::deliver_user_signal",
        "linux_abi::SIGPIPE_NUM",
    ):
        haystack = "\n".join((cargo, lib, signal, shell_uspace))
        if token not in haystack:
            findings.append(f"api pipe EPIPE must have a host-overridable SIGPIPE delivery hook; missing {token}")
    for token in (
        "AtomicBool",
        "AtomicUsize",
        "struct PipePeerCounts",
        "readers: AtomicUsize",
        "writers: AtomicUsize",
        "peer_counts: Arc<PipePeerCounts>",
        "impl Drop for Pipe",
        "writers.load(Ordering::Acquire) == 0",
        "readers.load(Ordering::Acquire) == 0",
        "nonblocking: AtomicBool::new(false)",
        "pub fn read_end_close",
        "self.nonblocking.load(Ordering::Acquire)",
        "LinuxError::EAGAIN",
        "LinuxError::EPIPE",
        "ctypes::O_NONBLOCK",
        "self.nonblocking.store(nonblocking, Ordering::Release)",
        "fn notify_read_end_closed",
        "crate::signal::raise_sigpipe()",
    ):
        if token not in pipe:
            findings.append(f"pipe O_NONBLOCK must be real observable state; missing {token}")
    if "pub fn read_end_close" not in pipe:
        findings.append("pipe write must track closed read ends and return EPIPE instead of fake-success buffering")
    if "Arc::strong_count(&self.buffer) == 1" in pipe:
        findings.append("pipe peer close detection must track read/write endpoints separately, not total buffer Arc strong_count")
    if "Err(LinuxError::EOPNOTSUPP)" in rust_function_block(pipe, "set_nonblocking"):
        findings.append("pipe set_nonblocking must not reject O_NONBLOCK after pipe2 reports success")
    if "fn write(&self, buf: &[u8])" not in pipe or "read_end_close" not in pipe or "LinuxError::EPIPE" not in pipe:
        findings.append("pipe write must return EPIPE when all read ends are closed instead of fake-success buffering")
    if pipe.count("self.notify_read_end_closed();") < 2:
        findings.append("pipe write must raise SIGPIPE on all closed-reader EPIPE paths, not only one branch")
    poll = rust_function_block(pipe, "poll")
    if "buf.available_read() > 0 || self.write_end_close()" not in poll:
        findings.append("pipe poll must report read-end EOF as readable so epoll/select wake instead of hiding closed writers")
    pipe2 = c_function_block(unistd, "pipe2")
    for token in ("errno = EINVAL", "return -1", "apply_pipe2_fcntl", "saved_errno", "close(fd[0])", "close(fd[1])"):
        if token not in pipe2:
            findings.append(f"pipe2 must propagate fcntl/flag errors honestly; missing {token}")
    if "return -EINVAL" in pipe2:
        findings.append("pipe2 must use libc errno + return -1, not raw negative errno")
    if re.search(r"\bfcntl\s*\(\s*fd\[0\].*?\bfcntl\s*\(\s*fd\[1\].*?return\s+0\s*;", pipe2, flags=re.S):
        findings.append("pipe2 must not ignore fcntl failures and then return success")
    return findings


def scan_axlibc_stdio(root: Path) -> list[str]:
    findings: list[str] = []
    header = read(root / "ulib/axlibc/include/stdio.h")
    text = read(root / "ulib/axlibc/c/stdio.c")
    for token in ("uint16_t flags", "F_EOF", "F_ERR"):
        if token not in header:
            findings.append(f"stdio FILE must store observable EOF/error flags; missing {token}")
    for token in ("__mark_file_error", "__mark_file_eof", ".flags = 0", "ssize_t len = read", "ssize_t len = write"):
        if token not in text:
            findings.append(f"stdio must update EOF/error state from real read/write results; missing {token}")
    write_buffer = c_function_block(text, "__write_buffer")
    for token in (
        "while (written < f->buffer_len)",
        "memmove(f->buf, f->buf + written, remaining)",
        "f->buffer_len = remaining",
        "errno = EIO",
        "return -1",
    ):
        if token not in write_buffer:
            findings.append(f"stdio fflush must preserve unwritten bytes and fail on short writes; missing {token}")
    if "__clear_buffer" in text:
        findings.append("stdio must not clear the entire buffer unconditionally after a partial write")
    for name in ("feof", "clearerr", "ferror"):
        block = c_function_block(text, name)
        if not block:
            findings.append(f"stdio missing {name}")
            continue
        if "unimplemented" in block or "ENOSYS" in block or "return -1" in block:
            findings.append(f"stdio {name} still reports unsupported instead of FILE state")
    fgets = c_function_block(text, "fgets")
    fread = c_function_block(text, "fread")
    fwrite = c_function_block(text, "fwrite")
    if "return NULL" not in fgets or "__mark_file_eof" not in fgets:
        findings.append("fgets must return NULL on EOF-before-data and mark EOF")
    if "__mark_file_eof" not in fread or "__mark_file_error" not in fread:
        findings.append("fread must mark EOF/error from real read results")
    if "__mark_file_error" not in fwrite:
        findings.append("fwrite must mark error from real write results")
    fclose_block = c_function_block(text, "fclose")
    if fclose_block and ("fflush(f)" not in fclose_block or "free(f)" not in fclose_block):
        findings.append("stdio fclose must flush buffered data, close fd, and free heap FILE state")
    for name in ("getchar", "fclose", "fseek", "ftello", "ftell", "getc", "__uflow", "getc_unlocked", "fdopen"):
        block = c_function_block(text, name)
        if not block:
            findings.append(f"stdio missing {name}")
            continue
        if "unimplemented" in block or "ENOSYS" in block:
            findings.append(f"stdio {name} still reports unsupported instead of using fd/read/lseek state")
    for token in (
        "return getc(stdin)",
        "__new_file_for_fd",
        "fflush(f)",
        "free(f)",
        "lseek(__stream->fd",
        "return ftello(f)",
        "ssize_t len = read(f->fd, &c, 1)",
        "fcntl(fd, F_GETFL)",
        "fcntl(fd, F_SETFL, fd_flags | O_APPEND)",
        "errno = EOPNOTSUPP",
        "__fd_mode_compatible",
        "errno = EBADF",
    ):
        if token not in text:
            findings.append(f"stdio must implement core FILE/fd state instead of ENOSYS stubs; missing {token}")
    fdopen_block = c_function_block(text, "fdopen")
    for token in (
        "int mode_flags = __fmodeflags(mode)",
        "__fd_mode_compatible(fd_flags, mode_flags)",
        "errno = EBADF",
        "(mode_flags & O_APPEND) && !(fd_flags & O_APPEND)",
        "fcntl(fd, F_SETFL, fd_flags | O_APPEND)",
        "if (!(fd_flags & O_APPEND))",
        "errno = EOPNOTSUPP",
    ):
        if token not in fdopen_block:
            findings.append(f"fdopen append mode must be real or fail honestly; missing {token}")
    fs_gate = text.find("#ifdef AX_CONFIG_FS")
    getc_def = text.find("int getc(FILE *f)")
    if fs_gate >= 0 and (getc_def < 0 or getc_def > fs_gate):
        findings.append("stdio getc/getchar must be read-backed outside AX_CONFIG_FS to avoid no-FS link regressions")
    return findings


def scan_axlibc_syslog(root: Path) -> list[str]:
    text = read(root / "ulib/axlibc/c/syslog.c")
    findings: list[str] = []
    for name in ("openlog", "syslog"):
        block = c_function_block(text, name)
        if not block:
            findings.append(f"syslog.c missing {name}")
            continue
        if "unimplemented" in block:
            findings.append(f"syslog.c {name} still fake-noops through unimplemented()")
    for token in ("syslog_ident", "syslog_option", "syslog_facility", "vfprintf(stderr", "LOG_PID", "LOG_PERROR"):
        if token not in text:
            findings.append(f"syslog/openlog must keep observable logger state/output; missing {token}")
    if "if (!(syslog_option & LOG_PERROR))" not in text:
        findings.append("syslog must not invent default stderr output; mirror to stderr only for LOG_PERROR")
    return findings


def scan_axlibc_locale_pwd_env(root: Path) -> list[str]:
    findings: list[str] = []

    locale = read(root / "ulib/axlibc/c/locale.c")
    setlocale = c_function_block(locale, "setlocale")
    localeconv = c_function_block(locale, "localeconv")
    for name, block in (("setlocale", setlocale), ("localeconv", localeconv)):
        if not block:
            findings.append(f"locale.c missing {name}")
        elif "unimplemented" in block or "ENOSYS" in block:
            findings.append(f"locale.c {name} still fake-noops instead of POSIX C locale state")
    for token in (
        "current_locale",
        "LC_CTYPE",
        "LC_ALL",
        'strcmp(__locale, "C")',
        'strcmp(__locale, "POSIX")',
        "posix_lconv",
        "CHAR_MAX",
    ):
        if token not in locale:
            findings.append(f"locale.c must expose real C/POSIX locale data; missing {token}")

    pwd = read(root / "ulib/axlibc/c/pwd.c")
    for name in ("getpwnam_r", "getpwuid_r"):
        block = c_function_block(pwd, name)
        if not block:
            findings.append(f"pwd.c missing {name}")
        elif "unimplemented" in block or "ENOSYS" in block:
            findings.append(f"pwd.c {name} still reports unsupported instead of supported root user DB")
    for token in (
        "fill_root_passwd",
        'strcmp(name, "root")',
        "pw->pw_uid = 0",
        "pw->pw_gid = 0",
        "ERANGE",
        '"/bin/sh"',
    ):
        if token not in pwd:
            findings.append(f"pwd.c must implement a bounded root passwd record; missing {token}")

    env = read(root / "ulib/axlibc/c/env.c")
    if '"dummy"' in env:
        findings.append("env.c must not seed environ with dummy fake data")
    for name in ("getenv", "setenv", "unsetenv"):
        block = c_function_block(env, name)
        if not block:
            findings.append(f"env.c missing {name}")
        elif "unimplemented" in block or "ENOSYS" in block:
            findings.append(f"env.c {name} still reports unsupported instead of mutable environment state")
    for token in (
        "initial_environ[] = {NULL}",
        "owned_environ",
        "env_ensure_owned",
        "env_strdup",
        "env_count",
        "env_free_owned",
        "env_reserve",
        "malloc(",
        "realloc(",
        "free(",
        "environ_count",
        "environ_capacity",
    ):
        if token not in env:
            findings.append(f"env.c must maintain mutable environ storage; missing {token}")
    if "if (env_ensure_owned() != 0)" not in c_function_block(env, "setenv"):
        findings.append("setenv must adopt/copy external environ before mutating or freeing entries")
    if "if (env_ensure_owned() != 0)" not in c_function_block(env, "unsetenv"):
        findings.append("unsetenv must adopt/copy external environ before mutating or freeing entries")
    return findings


def scan_scheduler_backend_effect(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/resource_sched.rs")
    findings: list[str] = []
    for token in (
        "fn scheduler_backend_priority",
        "fn apply_task_scheduler_state",
        "let _ = axtask::set_task_priority(task, scheduler_backend_priority(process, state));",
        "UserThreadEntry",
        "UserProcessRef::Owned(entry)",
        "scheduler_backend_priority(&entry.process, entry.process.get_sched_state())",
        "apply_task_scheduler_state(&entry.task, &entry.process, state)",
        "general::SCHED_FIFO | general::SCHED_RR",
        "general::SCHED_IDLE => BACKEND_IDLE_PRIORITY",
        "non-negative SCHED_OTHER tasks share the",
    ):
        if token not in text:
            findings.append(f"sched_set* must affect current axtask scheduler priority, not only readback state; missing {token}")
    setter = rust_function_block(text, "set_sched_target_state")
    if "apply_task_scheduler_state(current.as_task_ref(), process, state)" not in setter:
        findings.append("set_sched_target_state must apply scheduler backend state for current task")
    if "apply_task_scheduler_state(&entry.task, &entry.process, state)" not in setter:
        findings.append("set_sched_target_state must apply scheduler backend state for non-current live targets")
    backend = rust_function_block(text, "apply_task_scheduler_state")
    if "Err(LinuxError::EINVAL)" in backend or "return Err" in backend:
        findings.append("scheduler backend priority hook must be best-effort; unsupported FIFO/RR backends must not regress accepted sched_set* calls")
    task_api = read(root / "kernel/task/axtask/src/api.rs")
    run_queue = read(root / "kernel/task/axtask/src/run_queue.rs")
    cfs = read(root / "vendor/cargo/axsched/src/cfs.rs")
    rr = read(root / "vendor/cargo/axsched/src/round_robin.rs")
    fifo = read(root / "vendor/cargo/axsched/src/fifo.rs")
    if "pub fn set_task_priority(task: &AxTaskRef, prio: isize) -> bool" not in task_api:
        findings.append("axtask must expose target-task priority updates for non-current scheduler syscalls")
    if "pub(crate) fn set_task_priority(&mut self, task: &AxTaskRef, prio: isize) -> bool" not in run_queue:
        findings.append("axtask run queue must apply priority to a specified task, not just current")
    for token in (
        "fn refresh_min_vruntime",
        "Arc::ptr_eq",
        "queued_key",
        "self.ready_queue.remove(&key)",
        "queued_task.get_id()",
        "queued_task.get_vruntime()",
    ):
        if token not in cfs:
            findings.append(f"CFS set_priority must remove/reinsert queued tasks so ready_queue keys stay valid; missing {token}")
    for rel, scheduler_text in (
        ("vendor/cargo/axsched/src/round_robin.rs", rr),
        ("vendor/cargo/axsched/src/fifo.rs", fifo),
    ):
        for token in (
            "priority: AtomicIsize",
            "fn priority(&self) -> isize",
            "fn set_priority(&self, prio: isize)",
            "fn pop_highest_priority",
            "cursor.remove_current()",
            "task.set_priority(prio)",
            "valid_backend_priority(prio)",
        ):
            if token not in scheduler_text:
                findings.append(f"{rel}: FIFO/RR scheduler backend must consume accepted priority changes; missing {token}")
        if "fn set_priority(&mut self, _task" in scheduler_text and "\n        false\n" in scheduler_text:
            findings.append(f"{rel}: set_priority must not be a false-only stub")
    for token in (
        "skipped_rounds: AtomicUsize",
        "fn scheduling_class(&self) -> isize",
        "fn effective_priority(&self) -> isize",
        "fn scheduling_key(&self) -> (isize, isize)",
        "NORMAL_SCHEDULING_CLASS",
        "REALTIME_BACKEND_PRIORITY_MAX",
        "fn note_skipped_round(&self)",
        "fn reset_skipped_rounds(&self)",
        "selected.reset_skipped_rounds();",
        "task.note_skipped_round();",
    ):
        if token not in rr:
            findings.append(f"vendor/cargo/axsched/src/round_robin.rs: RR backend must age skipped lower-priority tasks to avoid starvation; missing {token}")
    for token in (
        "BACKEND_RT_BASE_PRIORITY",
        "BACKEND_IDLE_PRIORITY",
        "BACKEND_DEADLINE_BASE_PRIORITY",
        "BACKEND_RT_BASE_PRIORITY - linux_rt_prio as isize",
    ):
        if token not in text:
            findings.append(f"scheduler backend must preserve RT/deadline/idle class ordering; missing {token}")
    return findings


def scan_ltp_specific_comments(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/fd_table.rs")
    findings: list[str] = []
    for token in ("open10", "creat08", "creat09", "LTP's"):
        if token in text:
            findings.append(f"fd_table.rs still contains LTP/case-specific commentary token {token!r}")
    return findings


def scan_user_task_exit_cleanup(root: Path) -> list[str]:
    findings: list[str] = []
    task = read(root / "kernel/task/axtask/src/task.rs")
    lifecycle = read(root / "user/shell/src/uspace/process_lifecycle.rs")
    uspace_mod = read(root / "user/shell/src/uspace/mod.rs")
    memory_map = read(root / "user/shell/src/uspace/memory_map.rs")

    for token in (
        "pub fn try_join(&self) -> Option<i32>",
        "self.state() == TaskState::Exited",
    ):
        if token not in task:
            findings.append(f"axtask must expose non-blocking exited-task cleanup state; missing {token}")
    for token in (
        "const USER_TASK_EXIT_JOIN_GRACE",
        "fn join_user_task_for_cleanup(task: &AxTaskRef) -> bool",
        "task.try_join().is_some()",
        "axhal::time::monotonic_time() < deadline",
        "axtask::reap_exited_tasks();",
        "axtask::yield_now();",
    ):
        if token not in lifecycle:
            findings.append(f"auto-run timeout wrapper must not use unbounded task.join after process exit; missing {token}")
    timeout_runner = rust_function_block(lifecycle, "run_user_program_in_with_timeout")
    some_branch = timeout_runner.split("Some(code)", 1)[1].split("None =>", 1)[0] if "Some(code)" in timeout_runner else ""
    if "let _ = task.join();" in some_branch:
        findings.append("timeout-bound runner still uses unbounded task.join after process exit")

    for token in (
        "impl Drop for UserExecSharedMmapCache",
        "fn disarm_retained_frames(&mut self)",
        "core::mem::take(&mut self.pages)",
        "axmm::release_shared_frame_ref(frame)",
    ):
        if token not in uspace_mod:
            findings.append(f"exec shared mmap cache must be RAII-owned to avoid retained-frame leaks; missing {token}")
    for token in (
        "if let Some(mut cache)",
        "cache.disarm_retained_frames();",
        "aspace.map_retained_shared_frames",
    ):
        if token not in memory_map:
            findings.append(f"exec shared mmap cache transfer must disarm only after successful shared mapping install; missing {token}")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    root = args.root.resolve()
    findings: list[str] = []
    findings.extend(scan_api_fd_metadata(root))
    findings.extend(scan_fd_status_flags(root))
    findings.extend(scan_pipe_nonblocking_semantics(root))
    findings.extend(scan_axlibc_stdio(root))
    findings.extend(scan_axlibc_syslog(root))
    findings.extend(scan_axlibc_locale_pwd_env(root))
    findings.extend(scan_scheduler_backend_effect(root))
    findings.extend(scan_ltp_specific_comments(root))
    findings.extend(scan_user_task_exit_cleanup(root))
    if findings:
        print("G010 real-kernel-semantics static check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("G010 real-kernel-semantics static check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
