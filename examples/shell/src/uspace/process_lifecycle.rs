use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering};

use axalloc::frame_allocator_stats;
use axerrno::LinuxError;
use axhal::context::{TrapFrame, UspaceContext};
use axhal::paging::MappingFlags;
use axmm::AddrSpace;
use axsync::Mutex;
use axtask::{self, AxTaskRef, TaskInner, WaitQueue};
use linux_raw_sys::general;
use memory_addr::VirtAddr;
use std::collections::BTreeMap;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

use super::futex;
use super::linux_abi::{SIGCHLD_NUM, USER_ASPACE_BASE, USER_ASPACE_SIZE, neg_errno};
use super::program_loader::load_program_image;
use super::resource_sched::default_sched_state;
use super::runtime_paths::current_cwd;
use super::signal_abi::{all_application_signal_mask, ensure_user_return_hook_registered};
#[cfg(target_arch = "riscv64")]
use super::task_context::fixup_riscv_clone_child_return;
use super::task_context::{
    UserTaskExt, child_trap_frame, current_task_ext, current_tid, make_uspace_context, task_ext,
    user_pc,
};
#[cfg(feature = "auto-run-tests")]
use super::task_registry::live_user_thread_entries;
use super::task_registry::{
    UserThreadEntry, live_user_thread_count, register_user_task, unregister_user_task,
    user_thread_entries_by_process_pid, user_thread_entry_by_process_pid,
};
use super::user_memory::{
    read_cstr, read_execve_argv, read_execve_envp, write_user_bytes, write_user_value,
};
use super::{ChildTask, FdTable, NO_EXIT_GROUP_CODE, UserProcess};

const MAX_LIVE_USER_THREADS: usize = 512;
const MIN_FORK_FREE_FRAMES: usize = 8192;
const USER_TASK_KSTACK_SIZE: usize = 16 * 1024;

macro_rules! user_trace {
    ($($arg:tt)*) => {};
}

pub(super) struct ProcessTeardown {
    done: AtomicBool,
}

impl ProcessTeardown {
    pub(super) fn new() -> Self {
        Self {
            done: AtomicBool::new(false),
        }
    }

    pub(super) fn run(
        &self,
        _pid: i32,
        aspace: &Mutex<AddrSpace>,
        fds: &Mutex<FdTable>,
        children: &Mutex<Vec<ChildTask>>,
    ) {
        if self.done.swap(true, Ordering::AcqRel) {
            return;
        }

        aspace.lock().clear();
        {
            let mut fds = fds.lock();
            fds.close_all();
            *fds = FdTable::new();
        }
        children.lock().clear();
        axtask::reap_exited_tasks();
    }
}

struct LoadedProgram {
    process: Arc<UserProcess>,
    context: UspaceContext,
}

pub fn run_user_program(argv: &[&str]) -> Result<i32, String> {
    run_user_program_in(current_cwd().as_str(), argv)
}

pub fn run_user_program_in(cwd: &str, argv: &[&str]) -> Result<i32, String> {
    run_user_program_in_with_timeout(cwd, argv, None)
}

#[cfg(feature = "auto-run-tests")]
pub fn run_user_program_in_timeout(
    cwd: &str,
    argv: &[&str],
    timeout_secs: u64,
) -> Result<i32, String> {
    run_user_program_in_with_timeout(cwd, argv, Some(timeout_secs))
}

#[cfg(feature = "auto-run-tests")]
pub fn cleanup_user_processes() {
    for _ in 0..16 {
        let mut seen = Vec::new();
        let entries = live_user_thread_entries();
        if entries.is_empty() {
            break;
        }
        for entry in entries {
            let pid = entry.process.pid();
            if seen.contains(&pid) {
                continue;
            }
            seen.push(pid);
            entry.process.request_exit_group(137);
        }
        yield_for_task_gc();
        if live_user_thread_count() == 0 {
            break;
        }
    }
    yield_for_task_gc();
}

fn run_user_program_in_with_timeout(
    cwd: &str,
    argv: &[&str],
    timeout_secs: Option<u64>,
) -> Result<i32, String> {
    ensure_user_return_hook_registered();
    let loaded = load_program(cwd, argv)?;
    let process = loaded.process.clone();
    if let Some(timeout_secs) = timeout_secs {
        let deadline_us = axhal::time::monotonic_time()
            .as_micros()
            .try_into()
            .unwrap_or(u64::MAX)
            .saturating_add(timeout_secs.saturating_mul(1_000_000));
        process
            .eval_watchdog_deadline_us
            .store(deadline_us, Ordering::Release);
    }
    let task_process = process.clone();
    let context = loaded.context;
    let mut task = TaskInner::new(
        move || user_task_entry(task_process, context),
        format!("user:{}", argv[0]),
        USER_TASK_KSTACK_SIZE,
    );
    let root = loaded.process.aspace.lock().page_table_root();
    task.ctx_mut().set_page_table_root(root);
    task.init_task_ext(UserTaskExt::new(loaded.process.clone(), 0, 0));
    let task = axtask::spawn_task(task);
    process.set_pid(task.id().as_u64() as i32);
    register_user_task(task.clone(), process.clone());
    let exit_code = if let Some(timeout_secs) = timeout_secs {
        match process.wait_for_exit_timeout(core::time::Duration::from_secs(
            timeout_secs.saturating_add(2),
        )) {
            Some(code) => {
                let _ = task.join();
                process.teardown();
                code
            }
            None => {
                process.request_exit_group(137);
                process.teardown();
                137
            }
        }
    } else {
        let code = process.wait_for_exit();
        let _ = task.join();
        process.teardown();
        code
    };
    drop(task);
    yield_for_task_gc();
    Ok(exit_code)
}

fn yield_for_task_gc() {
    for _ in 0..64 {
        axtask::reap_exited_tasks();
        axtask::yield_now();
    }
    axtask::reap_exited_tasks();
}

fn ensure_user_task_capacity() -> Result<(), LinuxError> {
    if live_user_thread_count() >= MAX_LIVE_USER_THREADS {
        return Err(LinuxError::EAGAIN);
    }
    if frame_allocator_stats().free_frames < MIN_FORK_FREE_FRAMES {
        return Err(LinuxError::EAGAIN);
    }
    Ok(())
}

fn user_task_entry(_process: Arc<UserProcess>, context: UspaceContext) {
    let curr = axtask::current();
    let kstack_top = curr
        .kernel_stack_top()
        .expect("user task must have a kernel stack");
    unsafe { context.enter_uspace(kstack_top) }
}

fn user_thread_entry(process: Arc<UserProcess>, context: UspaceContext, child_tid_ptr: usize) {
    if child_tid_ptr != 0 {
        let tid = axtask::current().id().as_u64() as i32;
        let _ = write_user_value(process.as_ref(), child_tid_ptr, &tid);
    }
    user_task_entry(process, context)
}

fn load_program(cwd: &str, argv: &[&str]) -> Result<LoadedProgram, String> {
    let mut aspace = axmm::new_user_aspace(VirtAddr::from(USER_ASPACE_BASE), USER_ASPACE_SIZE)
        .map_err(|err| format!("failed to create user address space: {err}"))?;
    let image = load_program_image(&mut aspace, cwd, argv[0], argv, None)?;

    let process = Arc::new(UserProcess {
        aspace: Mutex::new(aspace),
        brk: Mutex::new(image.brk),
        shared_mmap_ranges: Mutex::new(Vec::new()),
        fds: Mutex::new(FdTable::new()),
        cwd: Mutex::new(cwd.into()),
        exec_root: Mutex::new(image.exec_root.clone()),
        exec_path: Mutex::new(image.exec_path.clone()),
        hostname: Mutex::new(String::from("arceos")),
        prctl_name: Mutex::new(String::from("arceos")),
        children: Mutex::new(Vec::new()),
        child_exit_wait: WaitQueue::new(),
        rlimits: Mutex::new(BTreeMap::new()),
        sched_state: Mutex::new(default_sched_state()),
        nice: AtomicI32::new(0),
        signal_actions: Mutex::new(BTreeMap::new()),
        path_modes: Mutex::new(BTreeMap::new()),
        path_special_modes: Mutex::new(BTreeMap::new()),
        path_owners: Mutex::new(BTreeMap::new()),
        path_symlinks: Mutex::new(BTreeMap::new()),
        umask: AtomicU32::new(0),
        mount_points: Arc::new(Mutex::new(BTreeMap::new())),
        shm_attachments: Mutex::new(BTreeMap::new()),
        real_uid: AtomicU32::new(0),
        uid: AtomicU32::new(0),
        saved_uid: AtomicU32::new(0),
        fs_uid: AtomicU32::new(0),
        real_gid: AtomicU32::new(0),
        gid: AtomicU32::new(0),
        saved_gid: AtomicU32::new(0),
        fs_gid: AtomicU32::new(0),
        groups: Mutex::new(Vec::new()),
        credential_generation: AtomicUsize::new(0),
        personality: AtomicUsize::new(0),
        parent_death_signal: AtomicI32::new(0),
        real_timer_generation: AtomicU64::new(0),
        real_timer_deadline_us: AtomicU64::new(0),
        real_timer_interval_us: AtomicU64::new(0),
        eval_watchdog_deadline_us: AtomicU64::new(0),
        child_wait_blocked: AtomicBool::new(false),
        syscall_wait_blocked: AtomicBool::new(false),
        pid: AtomicI32::new(0),
        pgid: AtomicI32::new(0),
        sid: AtomicI32::new(0),
        ppid: 1,
        live_threads: AtomicUsize::new(1),
        exit_group_code: AtomicI32::new(NO_EXIT_GROUP_CODE),
        exit_code: AtomicI32::new(0),
        term_signal: AtomicI32::new(0),
        exit_wait: WaitQueue::new(),
        teardown: ProcessTeardown::new(),
    });

    Ok(LoadedProgram {
        process,
        context: make_uspace_context(image.entry, image.stack_ptr, image.argc),
    })
}

fn exec_program(
    process: &UserProcess,
    cwd: &str,
    path: &str,
    argv: &[String],
    env: &[String],
) -> Result<(usize, usize, usize), String> {
    let argv_refs = argv.iter().map(String::as_str).collect::<Vec<_>>();
    let image = {
        let mut aspace = process.aspace.lock();
        load_program_image(&mut aspace, cwd, path, &argv_refs, Some(env))?
    };
    *process.brk.lock() = image.brk;
    process.shared_mmap_ranges.lock().clear();
    process.set_exec_root(image.exec_root);
    process.set_exec_path(image.exec_path);
    Ok((image.entry, image.stack_ptr, image.argc))
}

fn existing_busybox_for_exec_root(exec_root: &str) -> Option<String> {
    let mut candidates = Vec::new();
    if exec_root == "/glibc" {
        candidates.push("/glibc/busybox");
        candidates.push("/musl/busybox");
    } else {
        candidates.push("/musl/busybox");
        candidates.push("/glibc/busybox");
    }
    candidates.into_iter().find_map(|path| {
        matches!(std::fs::metadata(path), Ok(meta) if meta.is_file()).then(|| path.into())
    })
}

fn resolve_execve_compat_path(process: &UserProcess, path: String, argv: &mut [String]) -> String {
    let needs_busybox = matches!(path.as_str(), "/bin/sh" | "/busybox" | "/bin/busybox")
        && !matches!(std::fs::metadata(path.as_str()), Ok(meta) if meta.is_file());
    if !needs_busybox {
        return path;
    }

    let Some(busybox) = existing_busybox_for_exec_root(process.exec_root().as_str()) else {
        return path;
    };
    if path == "/bin/sh" {
        if let Some(argv0) = argv.first_mut() {
            if argv0 == "/bin/sh" || argv0.ends_with("/sh") {
                *argv0 = "sh".into();
            }
        }
    }
    busybox
}

impl UserProcess {
    pub(super) fn cwd(&self) -> String {
        self.cwd.lock().clone()
    }

    pub(super) fn exec_root(&self) -> String {
        self.exec_root.lock().clone()
    }

    pub(super) fn exec_path(&self) -> String {
        self.exec_path.lock().clone()
    }

    pub(super) fn hostname(&self) -> String {
        self.hostname.lock().clone()
    }

    pub(super) fn set_hostname(&self, hostname: String) {
        *self.hostname.lock() = hostname;
    }

    pub(super) fn prctl_name(&self) -> String {
        self.prctl_name.lock().clone()
    }

    pub(super) fn set_prctl_name(&self, name: String) {
        *self.prctl_name.lock() = name;
    }

    pub(super) fn set_cwd(&self, cwd: String) {
        *self.cwd.lock() = cwd;
    }

    fn set_exec_root(&self, exec_root: String) {
        *self.exec_root.lock() = exec_root;
    }

    fn set_exec_path(&self, exec_path: String) {
        *self.exec_path.lock() = exec_path;
    }

    pub(super) fn ppid(&self) -> i32 {
        self.ppid
    }

    pub(super) fn pid(&self) -> i32 {
        self.pid.load(Ordering::Acquire)
    }

    pub(super) fn set_pid(&self, pid: i32) {
        self.pid.store(pid, Ordering::Release);
        let _ = self
            .pgid
            .compare_exchange(0, pid, Ordering::AcqRel, Ordering::Acquire);
        let _ = self
            .sid
            .compare_exchange(0, pid, Ordering::AcqRel, Ordering::Acquire);
    }

    pub(super) fn pgid(&self) -> i32 {
        self.pgid.load(Ordering::Acquire)
    }

    pub(super) fn set_pgid(&self, pgid: i32) {
        self.pgid.store(pgid, Ordering::Release);
    }

    pub(super) fn sid(&self) -> i32 {
        self.sid.load(Ordering::Acquire)
    }

    pub(super) fn set_sid(&self, sid: i32) {
        self.sid.store(sid, Ordering::Release);
    }

    pub(super) fn credential_generation(&self) -> usize {
        self.credential_generation.load(Ordering::Acquire)
    }

    pub(super) fn teardown(&self) {
        self.teardown
            .run(self.pid(), &self.aspace, &self.fds, &self.children);
    }

    pub(super) fn add_thread(&self) {
        self.live_threads.fetch_add(1, Ordering::AcqRel);
    }

    pub(super) fn note_thread_exit(&self, code: i32) {
        self.exit_code.store(code, Ordering::Release);
        let live_before = self.live_threads.fetch_sub(1, Ordering::AcqRel);
        if live_before == 1 {
            self.teardown();
            self.exit_wait.notify_all(false);
            notify_parent_child_exit(self.ppid);
        }
    }

    pub(super) fn request_exit_group(&self, code: i32) {
        self.request_exit_group_inner(code, 0);
    }

    pub(super) fn request_signal_exit_group(&self, sig: i32) {
        self.request_exit_group_inner(128 + sig, signal_wait_status(sig));
    }

    fn request_exit_group_inner(&self, code: i32, term_signal: i32) {
        let _ = self.exit_group_code.compare_exchange(
            NO_EXIT_GROUP_CODE,
            code,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
        if term_signal != 0 {
            let _ = self.term_signal.compare_exchange(
                0,
                term_signal,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
        }
        self.exit_code.store(code, Ordering::Release);
        self.child_exit_wait.notify_all(false);
        self.exit_wait.notify_all(false);
        for entry in user_thread_entries_by_process_pid(self.pid()) {
            if let Some(ext) = task_ext(&entry.task) {
                let futex_wait = ext.futex_wait.load(Ordering::Acquire);
                if futex_wait != 0 {
                    futex::wake_task(ext.process.as_ref(), futex_wait, &entry.task);
                }
            }
        }
    }

    pub(super) fn pending_exit_group(&self) -> Option<i32> {
        let code = self.exit_group_code.load(Ordering::Acquire);
        (code != NO_EXIT_GROUP_CODE).then_some(code)
    }

    pub(super) fn eval_watchdog_deadline_us(&self) -> u64 {
        self.eval_watchdog_deadline_us.load(Ordering::Acquire)
    }

    pub(super) fn eval_watchdog_expired(&self) -> bool {
        let deadline = self.eval_watchdog_deadline_us();
        deadline != 0 && axhal::time::monotonic_time().as_micros() >= deadline as u128
    }

    pub(super) fn eval_watchdog_remaining(&self) -> Option<core::time::Duration> {
        let deadline = self.eval_watchdog_deadline_us();
        if deadline == 0 {
            return None;
        }
        let now = axhal::time::monotonic_time()
            .as_micros()
            .min(u64::MAX as u128) as u64;
        Some(super::time_abi::micros_to_duration(
            deadline.saturating_sub(now),
        ))
    }

    pub(super) fn is_child_wait_blocked(&self) -> bool {
        self.child_wait_blocked.load(Ordering::Acquire)
    }

    pub(super) fn set_syscall_wait_blocked(&self, blocked: bool) {
        self.syscall_wait_blocked.store(blocked, Ordering::Release);
    }

    pub(super) fn is_syscall_wait_blocked(&self) -> bool {
        self.syscall_wait_blocked.load(Ordering::Acquire)
    }

    pub(super) fn record_shared_mmap(&self, start: usize, size: usize, flags: MappingFlags) {
        self.shared_mmap_ranges.lock().push((start, size, flags));
    }

    pub(super) fn forget_mmap_range(&self, start: usize, end: usize) {
        self.shared_mmap_ranges
            .lock()
            .retain(|(range_start, size, _)| {
                let range_end = range_start.saturating_add(*size);
                range_end <= start || *range_start >= end
            });
    }

    fn shared_mmap_ranges(&self) -> Vec<(usize, usize, MappingFlags)> {
        self.shared_mmap_ranges.lock().clone()
    }

    fn wait_for_exit(&self) -> i32 {
        self.exit_wait
            .wait_until(|| self.live_threads.load(Ordering::Acquire) == 0);
        self.exit_code.load(Ordering::Acquire)
    }

    fn wait_for_exit_timeout(&self, timeout: core::time::Duration) -> Option<i32> {
        let timed_out = self
            .exit_wait
            .wait_timeout_until(timeout, || self.live_threads.load(Ordering::Acquire) == 0);
        (!timed_out).then(|| self.exit_code.load(Ordering::Acquire))
    }

    pub(super) fn fork(&self) -> Result<Arc<UserProcess>, LinuxError> {
        let mut aspace = axmm::new_user_aspace(VirtAddr::from(USER_ASPACE_BASE), USER_ASPACE_SIZE)
            .map_err(LinuxError::from)?;
        {
            let mut parent_aspace = self.aspace.lock();
            aspace
                .clone_user_mappings_from(&mut parent_aspace)
                .map_err(LinuxError::from)?;
            for (start, size, flags) in self.shared_mmap_ranges() {
                let start = VirtAddr::from(start);
                parent_aspace
                    .protect(start, size, flags)
                    .map_err(LinuxError::from)?;
                aspace
                    .protect(start, size, flags)
                    .map_err(LinuxError::from)?;
            }
        }

        Ok(Arc::new(UserProcess {
            aspace: Mutex::new(aspace),
            brk: Mutex::new(*self.brk.lock()),
            shared_mmap_ranges: Mutex::new(self.shared_mmap_ranges()),
            fds: Mutex::new(self.fds.lock().fork_copy()?),
            cwd: Mutex::new(self.cwd()),
            exec_root: Mutex::new(self.exec_root()),
            exec_path: Mutex::new(self.exec_path()),
            hostname: Mutex::new(self.hostname()),
            prctl_name: Mutex::new(self.prctl_name()),
            children: Mutex::new(Vec::new()),
            child_exit_wait: WaitQueue::new(),
            rlimits: Mutex::new(self.rlimits.lock().clone()),
            sched_state: Mutex::new(self.get_sched_state()),
            nice: AtomicI32::new(self.nice()),
            signal_actions: Mutex::new(self.signal_actions.lock().clone()),
            path_modes: Mutex::new(self.path_modes.lock().clone()),
            path_special_modes: Mutex::new(self.path_special_modes.lock().clone()),
            path_owners: Mutex::new(self.path_owners.lock().clone()),
            path_symlinks: Mutex::new(self.path_symlinks.lock().clone()),
            umask: AtomicU32::new(self.umask.load(Ordering::Acquire)),
            mount_points: self.mount_points.clone(),
            shm_attachments: Mutex::new(self.shm_attachments.lock().clone()),
            real_uid: AtomicU32::new(self.real_uid()),
            uid: AtomicU32::new(self.uid()),
            saved_uid: AtomicU32::new(self.saved_uid()),
            fs_uid: AtomicU32::new(self.fs_uid()),
            real_gid: AtomicU32::new(self.real_gid()),
            gid: AtomicU32::new(self.gid()),
            saved_gid: AtomicU32::new(self.saved_gid()),
            fs_gid: AtomicU32::new(self.fs_gid()),
            groups: Mutex::new(self.groups()),
            credential_generation: AtomicUsize::new(self.credential_generation()),
            personality: AtomicUsize::new(self.personality()),
            parent_death_signal: AtomicI32::new(self.parent_death_signal.load(Ordering::Acquire)),
            real_timer_generation: AtomicU64::new(0),
            real_timer_deadline_us: AtomicU64::new(0),
            real_timer_interval_us: AtomicU64::new(0),
            eval_watchdog_deadline_us: AtomicU64::new(
                self.eval_watchdog_deadline_us.load(Ordering::Acquire),
            ),
            child_wait_blocked: AtomicBool::new(false),
            syscall_wait_blocked: AtomicBool::new(false),
            pid: AtomicI32::new(0),
            pgid: AtomicI32::new(self.pgid()),
            sid: AtomicI32::new(self.sid()),
            ppid: axtask::current().id().as_u64() as i32,
            live_threads: AtomicUsize::new(1),
            exit_group_code: AtomicI32::new(NO_EXIT_GROUP_CODE),
            exit_code: AtomicI32::new(0),
            term_signal: AtomicI32::new(0),
            exit_wait: WaitQueue::new(),
            teardown: ProcessTeardown::new(),
        }))
    }

    pub(super) fn add_child(&self, task: AxTaskRef, process: Arc<UserProcess>) -> i32 {
        let pid = task.id().as_u64() as i32;
        self.children.lock().push(ChildTask { pid, task, process });
        pid
    }

    pub(super) fn wait_child(
        &self,
        pid: i32,
        nohang: bool,
    ) -> Result<Option<(i32, i32)>, LinuxError> {
        fn is_exited(child: &ChildTask) -> bool {
            child.process.live_threads.load(Ordering::Acquire) == 0
        }

        fn wait_pid_matches(child: &ChildTask, pid: i32, current_pgid: i32) -> bool {
            match pid {
                -1 => true,
                0 => child.process.pgid() == current_pgid,
                p if p > 0 => child.pid == p,
                p => p
                    .checked_neg()
                    .is_some_and(|target_pgid| child.process.pgid() == target_pgid),
            }
        }

        if pid < -1 && pid.checked_neg().is_none() {
            return Err(LinuxError::ESRCH);
        }
        let current_pgid = self.pgid();

        let child = loop {
            let maybe_child = {
                let mut children = self.children.lock();
                if children.is_empty() {
                    return Err(LinuxError::ECHILD);
                }

                if !children
                    .iter()
                    .any(|child| wait_pid_matches(child, pid, current_pgid))
                {
                    return Err(LinuxError::ECHILD);
                }
                let exited_index = children.iter().position(|child| {
                    wait_pid_matches(child, pid, current_pgid) && is_exited(child)
                });

                if let Some(index) = exited_index {
                    Some(children.remove(index))
                } else if nohang {
                    // WNOHANG must not block, but yielding before reporting
                    // "no child changed state" avoids monopolizing a
                    // single-core cooperative/preempt-limited run when user
                    // code polls waitpid() in a tight loop while child tasks
                    // need CPU time to reach exit.
                    drop(children);
                    axtask::yield_now();
                    return Ok(None);
                } else {
                    None
                }
            };

            if let Some(child) = maybe_child {
                break child;
            }
            if self.pending_exit_group().is_some() || self.eval_watchdog_expired() {
                return Err(LinuxError::EINTR);
            }
            let wait_condition = || {
                let children = self.children.lock();
                children.is_empty()
                    || !children
                        .iter()
                        .any(|child| wait_pid_matches(child, pid, current_pgid))
                    || children
                        .iter()
                        .any(|child| wait_pid_matches(child, pid, current_pgid) && is_exited(child))
                    || self.pending_exit_group().is_some()
                    || self.eval_watchdog_expired()
            };
            if let Some(timeout) = self.eval_watchdog_remaining() {
                self.child_wait_blocked.store(true, Ordering::Release);
                if self
                    .child_exit_wait
                    .wait_timeout_until(timeout, wait_condition)
                {
                    self.child_wait_blocked.store(false, Ordering::Release);
                    return Err(LinuxError::EINTR);
                }
                self.child_wait_blocked.store(false, Ordering::Release);
            } else {
                self.child_wait_blocked.store(true, Ordering::Release);
                self.child_exit_wait.wait_until(wait_condition);
                self.child_wait_blocked.store(false, Ordering::Release);
            }
        };
        let status = child.process.wait_status();
        let child_pid = child.pid;
        let _ = child.task.join();
        child.process.teardown();
        drop(child);
        yield_for_task_gc();
        Ok(Some((child_pid, status)))
    }

    fn wait_status(&self) -> i32 {
        let sig = self.term_signal.load(Ordering::Acquire);
        if sig != 0 {
            sig
        } else {
            (self.exit_code.load(Ordering::Acquire) & 0xff) << 8
        }
    }

    pub(super) fn child_thread_entry_by_pid(&self, pid: i32) -> Option<UserThreadEntry> {
        let children = self.children.lock();
        children
            .iter()
            .find(|child| {
                child.pid == pid && child.process.live_threads.load(Ordering::Acquire) != 0
            })
            .map(|child| UserThreadEntry {
                task: child.task.clone(),
                process: child.process.clone(),
            })
    }
}

fn notify_parent_child_exit(ppid: i32) {
    if let Some(parent) = user_thread_entry_by_process_pid(ppid) {
        parent.process.child_exit_wait.notify_all(false);
    }
}

pub(super) fn sys_execve(
    process: &UserProcess,
    _tf: &TrapFrame,
    pathname: usize,
    argv: usize,
    _envp: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let mut argv = match read_execve_argv(process, argv, path.as_str()) {
        Ok(argv) => argv,
        Err(err) => return neg_errno(err),
    };
    let path = resolve_execve_compat_path(process, path, &mut argv);
    let env = match read_execve_envp(process, _envp) {
        Ok(env) => env,
        Err(err) => return neg_errno(err),
    };
    let cwd = process.cwd();
    let (entry, stack_ptr, argc) =
        match exec_program(process, cwd.as_str(), path.as_str(), &argv, &env) {
            Ok(image) => image,
            Err(err) => {
                let errno = if err.contains("Entity not found") {
                    LinuxError::ENOENT
                } else if err.contains("Not a directory") {
                    LinuxError::ENOTDIR
                } else {
                    LinuxError::ENOEXEC
                };
                if errno == LinuxError::ENOEXEC {
                    println!("execve-load-failure: path={path} err={err}");
                }
                return neg_errno(errno);
            }
        };
    let context = make_uspace_context(entry, stack_ptr, argc);
    process.fds.lock().close_cloexec();
    let kstack_top = axtask::current()
        .kernel_stack_top()
        .expect("user task must have a kernel stack");
    unsafe { context.enter_uspace(kstack_top) }
}

pub(super) fn sys_clone(
    process: &Arc<UserProcess>,
    tf: &TrapFrame,
    flags: usize,
    child_stack: usize,
    ptid: usize,
    tls: usize,
    ctid: usize,
) -> isize {
    let exit_signal = flags & 0xff;
    let clone_flags = flags & !0xff;
    user_trace!(
        "thrclone: tid={} pid={} flags={flags:#x} clone_flags={clone_flags:#x} exit_signal={exit_signal} stack={child_stack:#x} ptid={ptid:#x} tls={tls:#x} ctid={ctid:#x} pc={:#x} sp={:#x} tp={:#x}",
        current_tid(),
        process.pid(),
        user_pc(tf),
        tf.regs.sp,
        tf.regs.tp,
    );
    let (inherited_signal_mask, fork_signal_mask_restore) = current_task_ext()
        .map(|ext| {
            (
                ext.signal_mask.load(Ordering::Acquire),
                ext.fork_signal_mask_restore.load(Ordering::Acquire),
            )
        })
        .unwrap_or((0, u64::MAX));
    let vfork_flags = general::CLONE_VM as usize | general::CLONE_VFORK as usize;
    let process_allowed_flags = vfork_flags
        | general::CLONE_SETTLS as usize
        | general::CLONE_PARENT_SETTID as usize
        | general::CLONE_CHILD_SETTID as usize
        | general::CLONE_CHILD_CLEARTID as usize;
    let fork_like_flags = clone_flags & !process_allowed_flags == 0
        && (clone_flags & general::CLONE_VM as usize == 0
            || clone_flags & vfork_flags == vfork_flags);
    if fork_like_flags {
        if let Err(err) = ensure_user_task_capacity() {
            return neg_errno(err);
        }
        if !matches!(exit_signal, 0) && exit_signal != SIGCHLD_NUM as usize {
            return neg_errno(LinuxError::ENOSYS);
        }
        if clone_flags & general::CLONE_PARENT_SETTID as usize != 0 && ptid == 0 {
            return neg_errno(LinuxError::EFAULT);
        }
        if clone_flags
            & (general::CLONE_CHILD_SETTID as usize | general::CLONE_CHILD_CLEARTID as usize)
            != 0
            && ctid == 0
        {
            return neg_errno(LinuxError::EFAULT);
        }

        let child_process = match process.fork() {
            Ok(process) => process,
            Err(err) => {
                println!(
                    "clone-failure-diagnostic: err={err:?} flags={flags:#x} clone_flags={clone_flags:#x} exit_signal={exit_signal} child_stack={child_stack:#x} parent_sp={:#x} parent_pc={:#x} clone_vm={} clone_vfork={}",
                    tf.regs.sp,
                    user_pc(tf),
                    clone_flags & general::CLONE_VM as usize != 0,
                    clone_flags & general::CLONE_VFORK as usize != 0,
                );
                return neg_errno(err);
            }
        };
        let mut child_tf = child_trap_frame(tf, child_stack);
        if clone_flags & general::CLONE_SETTLS as usize != 0 {
            child_tf.regs.tp = tls;
        }
        #[cfg(target_arch = "riscv64")]
        fixup_riscv_clone_child_return(process.as_ref(), &mut child_tf);
        let child_context = UspaceContext::from(&child_tf);
        let task_process = child_process.clone();
        let mut task = TaskInner::new(
            move || user_task_entry(task_process, child_context),
            "user:fork".into(),
            USER_TASK_KSTACK_SIZE,
        );
        let pid = task.id().as_u64() as i32;
        child_process.set_pid(pid);
        if clone_flags & general::CLONE_PARENT_SETTID as usize != 0 {
            let ret = write_user_value(process.as_ref(), ptid, &pid);
            if ret != 0 {
                return ret;
            }
        }
        if clone_flags & general::CLONE_CHILD_SETTID as usize != 0 {
            let ret = write_user_value(child_process.as_ref(), ctid, &pid);
            if ret != 0 {
                return ret;
            }
        }
        let child_clear_tid = if clone_flags & general::CLONE_CHILD_CLEARTID as usize != 0 {
            ctid
        } else {
            0
        };
        let root = child_process.aspace.lock().page_table_root();
        task.ctx_mut().set_page_table_root(root);
        // Fork-like process children should inherit the caller's stable signal
        // mask, not libc's transient all-signals-blocked fork critical section.
        // Thread clones keep the live mask through the thread path above.
        let child_signal_mask = if inherited_signal_mask == all_application_signal_mask()
            && fork_signal_mask_restore != u64::MAX
        {
            fork_signal_mask_restore
        } else {
            inherited_signal_mask
        };
        task.init_task_ext(UserTaskExt::new(
            child_process.clone(),
            child_clear_tid,
            child_signal_mask,
        ));
        let task = axtask::spawn_task(task);
        register_user_task(task.clone(), child_process.clone());
        process.add_child(task, child_process);
        return pid as isize;
    }

    const THREAD_REQUIRED_FLAGS: usize = general::CLONE_VM as usize
        | general::CLONE_FS as usize
        | general::CLONE_FILES as usize
        | general::CLONE_SIGHAND as usize
        | general::CLONE_SYSVSEM as usize
        | general::CLONE_THREAD as usize;
    const THREAD_ALLOWED_FLAGS: usize = THREAD_REQUIRED_FLAGS
        | general::CLONE_SETTLS as usize
        | general::CLONE_PARENT_SETTID as usize
        | general::CLONE_CHILD_CLEARTID as usize
        | general::CLONE_CHILD_SETTID as usize
        | general::CLONE_DETACHED as usize
        | general::CLONE_UNTRACED as usize;

    if exit_signal != 0
        || clone_flags & THREAD_REQUIRED_FLAGS != THREAD_REQUIRED_FLAGS
        || clone_flags & !THREAD_ALLOWED_FLAGS != 0
        || child_stack == 0
    {
        return neg_errno(LinuxError::ENOSYS);
    }

    if let Err(err) = ensure_user_task_capacity() {
        return neg_errno(err);
    }

    if clone_flags & general::CLONE_PARENT_SETTID as usize != 0 && ptid == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    if clone_flags & (general::CLONE_CHILD_SETTID as usize | general::CLONE_CHILD_CLEARTID as usize)
        != 0
        && ctid == 0
    {
        return neg_errno(LinuxError::EFAULT);
    }

    let mut child_tf = child_trap_frame(tf, child_stack);
    if clone_flags & general::CLONE_SETTLS as usize != 0 {
        child_tf.regs.tp = tls;
    }
    #[cfg(target_arch = "riscv64")]
    fixup_riscv_clone_child_return(process.as_ref(), &mut child_tf);
    let child_context = UspaceContext::from(&child_tf);
    let child_set_tid = if clone_flags & general::CLONE_CHILD_SETTID as usize != 0 {
        ctid
    } else {
        0
    };
    let child_clear_tid = if clone_flags & general::CLONE_CHILD_CLEARTID as usize != 0 {
        ctid
    } else {
        0
    };
    let task_process = process.clone();
    let mut task = TaskInner::new(
        move || user_thread_entry(task_process, child_context, child_set_tid),
        "user:thread".into(),
        USER_TASK_KSTACK_SIZE,
    );
    let tid = task.id().as_u64() as i32;
    let root = process.aspace.lock().page_table_root();
    task.ctx_mut().set_page_table_root(root);
    task.init_task_ext(UserTaskExt::new(
        process.clone(),
        child_clear_tid,
        inherited_signal_mask,
    ));

    if clone_flags & general::CLONE_PARENT_SETTID as usize != 0 {
        let ret = write_user_value(process.as_ref(), ptid, &tid);
        if ret != 0 {
            return ret;
        }
    }
    process.add_thread();
    let spawned = axtask::spawn_task(task);
    register_user_task(spawned, process.clone());
    tid as isize
}

pub(super) fn sys_wait4(
    process: &UserProcess,
    pid: i32,
    status: usize,
    options: usize,
    _rusage: usize,
) -> isize {
    const SUPPORTED_WAIT_OPTIONS: u32 = general::WNOHANG
        | general::WUNTRACED
        | general::WCONTINUED
        | general::__WNOTHREAD
        | general::__WALL
        | general::__WCLONE;

    let options = options as u32;
    if options & !SUPPORTED_WAIT_OPTIONS != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let nohang = options & general::WNOHANG != 0;
    let Some((child_pid, exit_code)) = (match process.wait_child(pid, nohang) {
        Ok(result) => result,
        Err(err) => return neg_errno(err),
    }) else {
        return 0;
    };
    user_trace!("user-wait4: requested pid={pid}, child={child_pid}, exit={exit_code}");
    if status != 0 {
        let wait_status = exit_code;
        let ret = write_user_value(process, status, &wait_status);
        if ret != 0 {
            return ret;
        }
    }
    child_pid as isize
}

fn waitid_pid_filter(idtype: u32, id: i32) -> Result<i32, LinuxError> {
    match idtype {
        general::P_ALL => Ok(-1),
        general::P_PID => Ok(id),
        general::P_PGID => {
            if id == 0 {
                Ok(0)
            } else if id > 0 {
                id.checked_neg().ok_or(LinuxError::ESRCH)
            } else {
                Err(LinuxError::EINVAL)
            }
        }
        _ => Err(LinuxError::EINVAL),
    }
}

fn waitid_siginfo(child_pid: i32, status: i32) -> [u8; 128] {
    let mut info = [0u8; 128];
    let signal_status = status & 0x7f;
    let (code, child_status) = if signal_status != 0 {
        let code = if status & 0x80 != 0 {
            general::CLD_DUMPED as i32
        } else {
            general::CLD_KILLED as i32
        };
        (code, signal_status)
    } else {
        (general::CLD_EXITED as i32, (status >> 8) & 0xff)
    };

    info[0..4].copy_from_slice(&(general::SIGCHLD as i32).to_ne_bytes());
    info[4..8].copy_from_slice(&0i32.to_ne_bytes());
    info[8..12].copy_from_slice(&code.to_ne_bytes());
    info[16..20].copy_from_slice(&child_pid.to_ne_bytes());
    info[20..24].copy_from_slice(&0u32.to_ne_bytes());
    info[24..28].copy_from_slice(&child_status.to_ne_bytes());
    info
}

fn signal_wait_status(sig: i32) -> i32 {
    let core_dumped = matches!(sig, 3 | 4 | 5 | 6 | 7 | 8 | 11 | 24 | 25 | 31);
    (sig & 0x7f) | if core_dumped { 0x80 } else { 0 }
}

pub(super) fn sys_waitid(
    process: &UserProcess,
    idtype: u32,
    id: i32,
    infop: usize,
    options: usize,
    _rusage: usize,
) -> isize {
    const SUPPORTED_WAITID_OPTIONS: u32 =
        general::WNOHANG | general::WEXITED | general::__WNOTHREAD | general::__WALL;

    let options = options as u32;
    if options & !SUPPORTED_WAITID_OPTIONS != 0 || options & general::WEXITED == 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let pid_filter = match waitid_pid_filter(idtype, id) {
        Ok(pid_filter) => pid_filter,
        Err(err) => return neg_errno(err),
    };
    let nohang = options & general::WNOHANG != 0;
    let wait_result = match process.wait_child(pid_filter, nohang) {
        Ok(result) => result,
        Err(err) => return neg_errno(err),
    };

    let info = if let Some((child_pid, status)) = wait_result {
        waitid_siginfo(child_pid, status)
    } else {
        [0u8; 128]
    };
    if infop != 0 {
        if let Err(err) = write_user_bytes(process, infop, &info) {
            return neg_errno(err);
        }
    }
    0
}

pub(super) fn sys_exit(process: &UserProcess, _tf: &TrapFrame, code: i32) -> ! {
    user_trace!(
        "user-exit: tid={} code={code} sp={:#x} tp={:#x} ra={:#x} pc={:#x}",
        current_tid(),
        tf.regs.sp,
        tf.regs.tp,
        tf.regs.ra,
        user_pc(tf),
    );
    terminate_current_thread(process, code)
}

pub(super) fn sys_exit_group(process: &UserProcess, _tf: &TrapFrame, code: i32) -> ! {
    user_trace!(
        "user-exit-group: tid={} code={code} sp={:#x} tp={:#x} ra={:#x} pc={:#x}",
        current_tid(),
        tf.regs.sp,
        tf.regs.tp,
        tf.regs.ra,
        user_pc(tf),
    );
    process.request_exit_group(code);
    terminate_current_thread_for_exit_group(process, code)
}

pub(super) fn terminate_current_thread(process: &UserProcess, code: i32) -> ! {
    terminate_current_thread_inner(process, code, false)
}

pub(super) fn terminate_current_thread_for_exit_group(process: &UserProcess, code: i32) -> ! {
    terminate_current_thread_inner(process, code, true)
}

fn terminate_current_thread_inner(process: &UserProcess, code: i32, teardown_now: bool) -> ! {
    clear_current_tid_and_wake();
    perform_deferred_self_unmap();
    unregister_user_task(current_tid());
    if teardown_now {
        process.teardown();
    }
    process.note_thread_exit(code);
    axtask::exit(code)
}

fn clear_current_tid_and_wake() {
    let Some(ext) = current_task_ext() else {
        return;
    };
    let clear_tid = ext.clear_child_tid.swap(0, Ordering::AcqRel);
    if clear_tid == 0 {
        return;
    }
    user_trace!(
        "user-clear-tid: tid={} clear_tid={clear_tid:#x}",
        current_tid()
    );
    let zero: i32 = 0;
    let _ = write_user_value(ext.process.as_ref(), clear_tid, &zero);
    let _ = futex::wake_addr(ext.process.as_ref(), clear_tid, 1);
}

fn perform_deferred_self_unmap() {
    let Some(ext) = current_task_ext() else {
        return;
    };
    let start = ext.deferred_unmap_start.swap(0, Ordering::AcqRel);
    let len = ext.deferred_unmap_len.swap(0, Ordering::AcqRel);
    if start == 0 || len == 0 {
        return;
    }
    let _ = ext.process.aspace.lock().unmap(VirtAddr::from(start), len);
}
