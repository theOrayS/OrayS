use core::mem::size_of;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering};

use axalloc::frame_allocator_stats;
use axerrno::LinuxError;
use axhal::context::{TrapFrame, UspaceContext};
use axhal::paging::MappingFlags;
use axhal::trap::PageFaultFlags;
use axmm::AddrSpace;
use axsync::Mutex;
use axtask::{self, AxTaskRef, TaskInner, WaitQueue};
#[cfg(feature = "auto-run-tests")]
use lazyinit::LazyInit;
use linux_raw_sys::general;
use memory_addr::{PAGE_SIZE_4K, PageIter4K, VirtAddr};
use std::collections::BTreeMap;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

use super::credentials::{self, access_allowed};
use super::fd_table::{
    executable_write_open, release_posix_record_locks_for_process, track_running_executable,
    untrack_running_executable,
};
use super::futex;
use super::linux_abi::{
    ACCESS_X_OK, SIGCHLD_NUM, ST_MODE_DIR, ST_MODE_TYPE_MASK, USER_ASPACE_BASE, USER_ASPACE_SIZE,
    neg_errno,
};
use super::metadata::{apply_recorded_path_metadata, file_type_mode, path_inode};
use super::program_loader::{LoadedMapping, load_program_image};
use super::resource_sched::default_sched_state;
use super::runtime_paths::{
    busybox_applet_target_path, current_cwd, is_busybox_applet_name, normalize_path,
};
use super::signal_abi::{all_application_signal_mask, ensure_user_return_hook_registered};
use super::sysv_shm;
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
    MAX_USER_IO_CHUNK, read_cstr, read_execve_argv, read_execve_envp, read_user_value,
    write_user_bytes, write_user_value,
};
use super::{ChildTask, DEFAULT_TIMER_SLACK_NS, NO_EXIT_GROUP_CODE, ProcessFdTable, UserProcess};

const MAX_LIVE_USER_THREADS: usize = 512;
const MIN_FORK_FREE_FRAMES: usize = 8192;
const USER_TASK_KSTACK_SIZE: usize = 16 * 1024;
const EXEC_PATH_MAX: usize = 4096;
const EXEC_NAME_MAX: usize = 255;

fn zero_child_wipe_on_fork_ranges(
    process: &UserProcess,
    ranges: &[(usize, usize)],
) -> Result<(), LinuxError> {
    if ranges.is_empty() {
        return Ok(());
    }
    let chunk = MAX_USER_IO_CHUNK.min(PAGE_SIZE_4K * 16);
    let mut zeros = Vec::new();
    zeros
        .try_reserve_exact(chunk)
        .map_err(|_| LinuxError::ENOMEM)?;
    zeros.resize(chunk, 0);

    for (start, end) in ranges.iter().copied() {
        let mut cursor = start;
        while cursor < end {
            let len = (end - cursor).min(zeros.len());
            process
                .aspace
                .lock()
                .write(VirtAddr::from(cursor), &zeros[..len])
                .map_err(LinuxError::from)?;
            cursor += len;
        }
    }
    Ok(())
}

fn populate_committed_brk_for_fork(aspace: &mut AddrSpace, brk_start: usize, brk_end: usize) {
    if brk_end <= brk_start {
        return;
    }
    let start = brk_start / PAGE_SIZE_4K * PAGE_SIZE_4K;
    let end = brk_end.saturating_add(PAGE_SIZE_4K - 1) / PAGE_SIZE_4K * PAGE_SIZE_4K;
    let mut page = start;
    while page < end {
        let _ = aspace.handle_page_fault(VirtAddr::from(page), PageFaultFlags::WRITE);
        page = page.saturating_add(PAGE_SIZE_4K);
    }
}

fn prefault_clone_child_stack_for_fork(aspace: &mut AddrSpace, child_stack: Option<usize>) {
    let Some(child_stack) = child_stack.filter(|stack| *stack != 0) else {
        return;
    };
    let start = child_stack.saturating_sub(PAGE_SIZE_4K * 2) / PAGE_SIZE_4K * PAGE_SIZE_4K;
    let end = child_stack.saturating_add(PAGE_SIZE_4K - 1) / PAGE_SIZE_4K * PAGE_SIZE_4K;
    if end <= start {
        return;
    }
    let mut page = start;
    while page < end {
        let _ = aspace.handle_page_fault(VirtAddr::from(page), PageFaultFlags::WRITE);
        page = page.saturating_add(PAGE_SIZE_4K);
    }
}

fn initial_path_modes() -> BTreeMap<String, u32> {
    let mut modes = BTreeMap::new();
    // The ramfs mount at /tmp should behave like a normal Linux temporary
    // directory for user programs that switch credentials and then create
    // scratch subdirectories.  The LTP runner's TMPDIR is a shared scratch root
    // created by the kernel-side harness before each case; seed it with the
    // same world-writable sticky mode so forked/setuid test children inherit a
    // writable view instead of the backing ramfs default directory mode.
    modes.insert(String::from("/tmp"), 0o1777);
    modes.insert(String::from("/tmp/ltp-work"), 0o1777);
    #[cfg(feature = "auto-run-tests")]
    {
        for (path, mode) in initial_path_mode_overrides().lock().iter() {
            modes.insert(path.clone(), *mode);
        }
    }
    modes
}

#[cfg(feature = "auto-run-tests")]
static INITIAL_PATH_MODE_OVERRIDES: LazyInit<Mutex<BTreeMap<String, u32>>> = LazyInit::new();

#[cfg(feature = "auto-run-tests")]
fn initial_path_mode_overrides() -> &'static Mutex<BTreeMap<String, u32>> {
    let _ = INITIAL_PATH_MODE_OVERRIDES.call_once(|| Mutex::new(BTreeMap::new()));
    &INITIAL_PATH_MODE_OVERRIDES
}

#[cfg(feature = "auto-run-tests")]
pub fn seed_initial_path_mode(path: &str, mode: u32) {
    let normalized = normalize_path("/", path).unwrap_or_else(|| String::from(path));
    initial_path_mode_overrides()
        .lock()
        .insert(normalized, mode & 0o7777);
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
        pid: i32,
        exec_path: &Mutex<String>,
        aspace: &Mutex<AddrSpace>,
        clear_aspace: bool,
        fds: &ProcessFdTable,
        close_fd_base: bool,
        children: &Mutex<Vec<ChildTask>>,
    ) {
        if self.done.swap(true, Ordering::AcqRel) {
            return;
        }

        untrack_running_executable(exec_path.lock().as_str());
        release_posix_record_locks_for_process(pid);
        if clear_aspace {
            aspace.lock().clear();
        }
        fds.close_all_for_pid(pid, close_fd_base);
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
    let exec_path = image.exec_path.clone();

    let process = Arc::new(UserProcess {
        aspace: Arc::new(Mutex::new(aspace)),
        owns_aspace: true,
        brk: Mutex::new(image.brk),
        shared_mmap_ranges: Mutex::new(Vec::new()),
        mmap_sigbus_ranges: Mutex::new(Vec::new()),
        mmap_ranges: Mutex::new(Vec::new()),
        exec_shared_mmap_cache: Mutex::new(Vec::new()),
        mlock_future: AtomicBool::new(false),
        mlockall_accounted_kb: AtomicUsize::new(0),
        fds: Arc::new(ProcessFdTable::new()),
        cwd: Mutex::new(cwd.into()),
        fs_root: Mutex::new(String::from("/")),
        exec_root: Mutex::new(image.exec_root.clone()),
        exec_path: Mutex::new(image.exec_path.clone()),
        hostname: Arc::new(Mutex::new(String::from("arceos"))),
        domainname: Arc::new(Mutex::new(String::from("localdomain"))),
        prctl_name: Mutex::new(String::from("arceos")),
        children: Mutex::new(Vec::new()),
        child_exit_wait: WaitQueue::new(),
        rlimits: Mutex::new(BTreeMap::new()),
        sched_state: Mutex::new(default_sched_state()),
        nice: AtomicI32::new(0),
        ioprio: AtomicU32::new(super::resource_sched::default_ioprio()),
        signal_actions: Mutex::new(BTreeMap::new()),
        path_modes: Mutex::new(initial_path_modes()),
        path_inodes: Mutex::new(BTreeMap::new()),
        path_special_modes: Mutex::new(BTreeMap::new()),
        path_rdevs: Mutex::new(BTreeMap::new()),
        path_owners: Mutex::new(BTreeMap::new()),
        path_symlinks: Mutex::new(BTreeMap::new()),
        path_hardlinks: Mutex::new(BTreeMap::new()),
        path_hardlink_counts: Mutex::new(BTreeMap::new()),
        path_inode_flags: Mutex::new(BTreeMap::new()),
        path_xattrs: Mutex::new(BTreeMap::new()),
        path_times: Mutex::new(BTreeMap::new()),
        path_sparse_sizes: Mutex::new(BTreeMap::new()),
        path_sparse_data: Mutex::new(BTreeMap::new()),
        path_data_ranges: Mutex::new(BTreeMap::new()),
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
        cap_effective: AtomicU64::new(credentials::ALL_CAPABILITIES_MASK),
        cap_permitted: AtomicU64::new(credentials::ALL_CAPABILITIES_MASK),
        cap_inheritable: AtomicU64::new(0),
        cap_bounding: AtomicU64::new(credentials::ALL_CAPABILITIES_MASK),
        personality: AtomicUsize::new(0),
        parent_death_signal: AtomicI32::new(0),
        default_timer_slack_ns: AtomicU64::new(DEFAULT_TIMER_SLACK_NS),
        timer_slack_ns: AtomicU64::new(DEFAULT_TIMER_SLACK_NS),
        posix_timers: Mutex::new(BTreeMap::new()),
        next_posix_timer_id: AtomicI32::new(1),
        real_timer_generation: AtomicU64::new(0),
        real_timer_deadline_us: AtomicU64::new(0),
        real_timer_interval_us: AtomicU64::new(0),
        virtual_timer_deadline_us: AtomicU64::new(0),
        virtual_timer_interval_us: AtomicU64::new(0),
        prof_timer_deadline_us: AtomicU64::new(0),
        prof_timer_interval_us: AtomicU64::new(0),
        start_clock_ticks: AtomicU64::new(super::time_abi::clock_ticks_now()),
        waited_child_user_ticks: AtomicU64::new(0),
        waited_child_system_ticks: AtomicU64::new(0),
        max_rss_kb: AtomicUsize::new(0),
        waited_child_maxrss_kb: AtomicUsize::new(0),
        eval_watchdog_deadline_us: AtomicU64::new(0),
        child_wait_blocked: AtomicBool::new(false),
        syscall_wait_blocked: AtomicBool::new(false),
        vfork_exec_done: AtomicBool::new(false),
        pid: AtomicI32::new(0),
        pgid: AtomicI32::new(0),
        sid: AtomicI32::new(0),
        ppid: 1,
        live_threads: AtomicUsize::new(1),
        exit_group_code: AtomicI32::new(NO_EXIT_GROUP_CODE),
        exit_code: AtomicI32::new(0),
        term_signal: AtomicI32::new(0),
        wait_stopped_signal: AtomicI32::new(0),
        wait_continued_signal: AtomicI32::new(0),
        exit_wait: WaitQueue::new(),
        teardown: ProcessTeardown::new(),
    });
    track_running_executable(exec_path.as_str());
    record_loaded_image_mappings(process.as_ref(), &image.mappings);

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
    let mut new_aspace = axmm::new_user_aspace(VirtAddr::from(USER_ASPACE_BASE), USER_ASPACE_SIZE)
        .map_err(|err| format!("failed to create exec address space: {err:?}"))?;
    let image = load_program_image(&mut new_aspace, cwd, path, &argv_refs, Some(env))?;
    let new_root = new_aspace.page_table_root();
    {
        let mut aspace = process.aspace.lock();
        process.cache_exec_shared_file_mappings(&mut aspace);
        core::mem::swap(&mut *aspace, &mut new_aspace);
    }
    // The old address space now lives in `new_aspace`; switch both the live CPU
    // and the saved task context to the successfully built image before the old
    // page table is dropped. Failed execve paths never mutate the live image.
    unsafe { axhal::asm::write_user_page_table(new_root) };
    axtask::current().set_page_table_root(new_root);
    axhal::asm::flush_tlb(None);
    *process.brk.lock() = image.brk;
    process.shared_mmap_ranges.lock().clear();
    process.mmap_sigbus_ranges.lock().clear();
    process.mmap_ranges.lock().clear();
    record_loaded_image_mappings(process, &image.mappings);
    process.mlock_future.store(false, Ordering::Release);
    process.mlockall_accounted_kb.store(0, Ordering::Release);
    process.clear_posix_timers();
    let old_exec_path = process.exec_path();
    if old_exec_path != image.exec_path {
        untrack_running_executable(old_exec_path.as_str());
        track_running_executable(image.exec_path.as_str());
    }
    process.set_exec_root(image.exec_root);
    process.set_exec_path(image.exec_path);
    process.vfork_exec_done.store(true, Ordering::Release);
    process.exit_wait.notify_all(false);
    Ok((image.entry, image.stack_ptr, image.argc))
}

fn record_loaded_image_mappings(process: &UserProcess, mappings: &[LoadedMapping]) {
    for mapping in mappings {
        process.record_mmap_region(
            mapping.start,
            mapping.size,
            mapping.prot,
            false,
            false,
            false,
            false,
            mapping.prot & general::PROT_WRITE != 0,
            None,
        );
    }
}

fn existing_busybox_for_exec_root(exec_root: &str) -> Option<String> {
    let mut candidates = Vec::new();
    if exec_root == "/glibc" {
        // The glibc runtime image may contain a minimal busybox without the
        // shell/applet set used by LTP helpers.  Prefer the packaged musl
        // busybox for generic /bin applet fallback; it is statically linked
        // and is safe to use as a tool runner for glibc payloads.
        candidates.push("/musl/busybox");
        candidates.push("/glibc/busybox");
    } else {
        candidates.push("/musl/busybox");
        candidates.push("/glibc/busybox");
    }
    candidates.into_iter().find_map(|path| {
        matches!(std::fs::metadata(path), Ok(meta) if meta.is_file()).then(|| path.into())
    })
}

fn standard_bin_busybox_applet_name(path: &str) -> Option<&str> {
    let applet = path
        .strip_prefix("/bin/")
        .or_else(|| path.strip_prefix("/usr/bin/"))?;
    if applet.is_empty() || applet.contains('/') || !is_busybox_applet_name(applet) {
        None
    } else {
        Some(applet)
    }
}

fn rooted_busybox_applet_name(path: &str) -> Option<&str> {
    let applet = path
        .strip_prefix("/musl/")
        .or_else(|| path.strip_prefix("/glibc/"))?;
    if applet.is_empty()
        || applet.contains('/')
        || applet == "busybox"
        || !is_busybox_applet_name(applet)
    {
        None
    } else {
        Some(applet)
    }
}

fn busybox_exec_alias_target(process: &UserProcess, path: &str) -> Option<String> {
    busybox_applet_target_path(path)
        .filter(|target| matches!(std::fs::metadata(target), Ok(meta) if meta.is_file()))
        .or_else(|| existing_busybox_for_exec_root(process.exec_root().as_str()))
}

fn resolve_execve_compat_path(process: &UserProcess, path: String, argv: &mut [String]) -> String {
    let applet_name = standard_bin_busybox_applet_name(path.as_str())
        .or_else(|| rooted_busybox_applet_name(path.as_str()));
    let needs_busybox = (matches!(path.as_str(), "/busybox" | "/bin/busybox")
        || applet_name.is_some())
        && !matches!(std::fs::metadata(path.as_str()), Ok(meta) if meta.is_file());
    if !needs_busybox {
        return path;
    }

    let Some(busybox) = busybox_exec_alias_target(process, path.as_str()) else {
        return path;
    };
    if let Some(applet) = applet_name {
        if let Some(argv0) = argv.first_mut() {
            if argv0 == path.as_str() || argv0.rsplit('/').next() == Some(applet) {
                *argv0 = applet.into();
            }
        }
    }
    busybox
}

impl UserProcess {
    pub(super) fn cwd(&self) -> String {
        self.cwd.lock().clone()
    }

    pub(super) fn fs_root(&self) -> String {
        self.fs_root.lock().clone()
    }

    pub(super) fn resolve_fs_absolute_path(&self, path: &str) -> Result<String, LinuxError> {
        let normalized = normalize_path("/", path).ok_or(LinuxError::EINVAL)?;
        let root = self.fs_root();
        if root == "/" {
            return Ok(normalized);
        }
        if normalized == "/" {
            return Ok(root);
        }
        let tail = normalized.trim_start_matches('/');
        normalize_path(root.as_str(), tail).ok_or(LinuxError::EINVAL)
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

    pub(super) fn domainname(&self) -> String {
        self.domainname.lock().clone()
    }

    pub(super) fn set_domainname(&self, domainname: String) {
        *self.domainname.lock() = domainname;
    }

    pub(super) fn prctl_name(&self) -> String {
        self.prctl_name.lock().clone()
    }

    pub(super) fn set_prctl_name(&self, name: String) {
        *self.prctl_name.lock() = name;
    }

    pub(super) fn timer_slack_ns(&self) -> u64 {
        self.timer_slack_ns.load(Ordering::Acquire)
    }

    pub(super) fn default_timer_slack_ns(&self) -> u64 {
        self.default_timer_slack_ns.load(Ordering::Acquire)
    }

    pub(super) fn set_timer_slack_ns(&self, value: u64) {
        self.timer_slack_ns.store(value, Ordering::Release);
    }

    pub(super) fn set_cwd(&self, cwd: String) {
        *self.cwd.lock() = cwd;
    }

    pub(super) fn set_fs_root(&self, fs_root: String) {
        *self.fs_root.lock() = fs_root;
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
        self.release_exec_shared_mmap_cache();
        sysv_shm::release_process_attachments(self);
        self.teardown.run(
            self.pid(),
            &self.exec_path,
            self.aspace.as_ref(),
            self.owns_aspace,
            self.fds.as_ref(),
            Arc::strong_count(&self.fds) == 1,
            &self.children,
        );
    }

    pub(super) fn release_exec_shared_mmap_cache(&self) {
        let mut cache = self.exec_shared_mmap_cache.lock();
        for entry in core::mem::take(&mut *cache) {
            entry.release_retained_frames();
        }
    }

    pub(super) fn cache_exec_shared_file_mappings(&self, aspace: &mut AddrSpace) {
        self.release_exec_shared_mmap_cache();
        let mut next_cache = Vec::new();
        for region in self.mmap_regions() {
            if !region.shared {
                continue;
            }
            let Some(backing) = region.file_backing.clone() else {
                continue;
            };
            if region.size == 0 || region.size % PAGE_SIZE_4K != 0 {
                continue;
            }
            let Some(iter) =
                PageIter4K::new(VirtAddr::from(region.start), VirtAddr::from(region.end()))
            else {
                continue;
            };
            let mut pages = Vec::new();
            for page in iter {
                let Ok((frame, flags, page_size)) = aspace.page_table().query(page) else {
                    continue;
                };
                if page_size.is_huge() {
                    continue;
                }
                axmm::retain_shared_frame_ref(frame);
                pages.push((page.as_usize() - region.start, frame, flags));
            }
            if pages.len() != region.size / PAGE_SIZE_4K {
                for (_, frame, _) in pages {
                    axmm::release_shared_frame_ref(frame);
                }
                continue;
            }
            next_cache.push(super::UserExecSharedMmapCache {
                file: backing.file,
                offset: backing.offset,
                size: region.size,
                valid_len: backing.valid_len,
                pages,
            });
        }
        *self.exec_shared_mmap_cache.lock() = next_cache;
    }

    pub(super) fn take_exec_shared_mmap_cache(
        &self,
        file: &super::fd_table::MmapFileBacking,
        offset: u64,
        size: usize,
    ) -> Option<super::UserExecSharedMmapCache> {
        let mut cache = self.exec_shared_mmap_cache.lock();
        let index = cache.iter().position(|entry| {
            entry.offset == offset && entry.size == size && entry.file.same_backing(file)
        })?;
        Some(cache.remove(index))
    }

    fn atomic_record_max(cell: &AtomicUsize, value: usize) {
        let mut current = cell.load(Ordering::Acquire);
        while value > current {
            match cell.compare_exchange(current, value, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => return,
                Err(observed) => current = observed,
            }
        }
    }

    pub(super) fn record_self_maxrss_kb(&self, value: usize) {
        Self::atomic_record_max(&self.max_rss_kb, value);
    }

    pub(super) fn self_maxrss_kb(&self) -> usize {
        self.max_rss_kb.load(Ordering::Acquire)
    }

    pub(super) fn child_maxrss_kb(&self) -> usize {
        self.waited_child_maxrss_kb.load(Ordering::Acquire)
    }

    fn record_waited_child_maxrss(&self, value: usize) {
        Self::atomic_record_max(&self.waited_child_maxrss_kb, value);
    }

    pub(super) fn sigchld_discards_wait_status(&self) -> bool {
        let action = self
            .signal_actions
            .lock()
            .get(&(SIGCHLD_NUM as usize))
            .copied()
            .unwrap_or_else(|| unsafe { core::mem::zeroed() });
        let handler = action
            .sa_handler_kernel
            .map(|func| func as usize)
            .unwrap_or(0);
        handler == 1 || action.sa_flags & general::SA_NOCLDWAIT as u64 != 0
    }

    pub(super) fn reap_ignored_child(&self, child_pid: i32) -> bool {
        let mut children = self.children.lock();
        let Some(index) = children.iter().position(|child| {
            child.pid == child_pid && child.process.live_threads.load(Ordering::Acquire) == 0
        }) else {
            return false;
        };
        let _child = children.remove(index);
        drop(children);
        self.child_exit_wait.notify_all(false);
        axtask::reap_exited_tasks();
        true
    }

    pub(super) fn reap_exited_ignored_children(&self) {
        let mut children = self.children.lock();
        let mut index = 0;
        let mut reaped = false;
        while index < children.len() {
            if children[index].process.live_threads.load(Ordering::Acquire) == 0 {
                let _child = children.remove(index);
                reaped = true;
            } else {
                index += 1;
            }
        }
        drop(children);
        if reaped {
            self.child_exit_wait.notify_all(false);
            axtask::reap_exited_tasks();
        }
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
            notify_parent_child_exit(self.ppid, self.pid());
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

    pub(super) fn protect_shared_mmap_range(&self, start: usize, end: usize, flags: MappingFlags) {
        let mut ranges = self.shared_mmap_ranges.lock();
        let old = core::mem::take(&mut *ranges);
        for (range_start, size, range_flags) in old {
            let Some(range_end) = range_start.checked_add(size) else {
                continue;
            };
            if range_end <= start || range_start >= end {
                ranges.push((range_start, size, range_flags));
                continue;
            }
            if range_start < start {
                ranges.push((range_start, start - range_start, range_flags));
            }
            let protected_start = range_start.max(start);
            let protected_end = range_end.min(end);
            if protected_end > protected_start {
                ranges.push((protected_start, protected_end - protected_start, flags));
            }
            if range_end > end {
                ranges.push((end, range_end - end, range_flags));
            }
        }
        ranges.sort_by_key(|(range_start, _, _)| *range_start);
    }

    pub(super) fn record_mmap_sigbus_range(&self, start: usize, end: usize) {
        if end <= start {
            return;
        }
        let mut ranges = self.mmap_sigbus_ranges.lock();
        ranges.push((start, end));
        ranges.sort_by_key(|(range_start, _)| *range_start);
    }

    pub(super) fn fault_in_mmap_sigbus_range(&self, addr: usize) -> bool {
        self.mmap_sigbus_ranges
            .lock()
            .iter()
            .any(|(start, end)| addr >= *start && addr < *end)
    }

    pub(super) fn handle_mmap_grow_down_fault(
        &self,
        addr: usize,
        fault_flags: PageFaultFlags,
    ) -> bool {
        const STACK_GUARD_GAP: usize = 256 * PAGE_SIZE_4K;

        let fault_page = addr & !(PAGE_SIZE_4K - 1);
        let requested_access = MappingFlags::from_bits_truncate(fault_flags.bits());
        let candidate = {
            let ranges = self.mmap_ranges.lock();
            let Some(region) = ranges
                .iter()
                .filter(|region| region.grow_down && fault_page < region.start)
                .min_by_key(|region| region.start - fault_page)
                .cloned()
            else {
                return false;
            };
            let grow_end = region.start;
            if grow_end <= fault_page {
                return false;
            }
            let map_flags = super::memory_map::mmap_prot_to_flags(region.prot);
            if !map_flags.contains(requested_access) {
                return false;
            }
            if ranges.iter().any(|other| {
                other.start < grow_end && other.end() > fault_page && other.start != region.start
            }) {
                return false;
            }
            let guard_floor = fault_page.saturating_sub(STACK_GUARD_GAP);
            if ranges
                .iter()
                .any(|other| other.end() <= fault_page && other.end() > guard_floor)
            {
                return false;
            }
            (region.start, region.end(), grow_end - fault_page, map_flags)
        };

        let (old_start, old_end, grow_size, map_flags) = candidate;
        if self
            .aspace
            .lock()
            .map_alloc(VirtAddr::from(fault_page), grow_size, map_flags, false)
            .is_err()
        {
            return false;
        }

        let mut ranges = self.mmap_ranges.lock();
        if let Some(region) = ranges
            .iter_mut()
            .find(|region| region.grow_down && region.start == old_start && region.end() == old_end)
        {
            region.start = fault_page;
            region.size = old_end - fault_page;
            true
        } else {
            false
        }
    }

    fn forget_mmap_sigbus_range(&self, start: usize, end: usize) {
        let mut ranges = self.mmap_sigbus_ranges.lock();
        let old = core::mem::take(&mut *ranges);
        for (range_start, range_end) in old {
            if range_end <= start || range_start >= end {
                ranges.push((range_start, range_end));
                continue;
            }
            if range_start < start {
                ranges.push((range_start, start));
            }
            if range_end > end {
                ranges.push((end, range_end));
            }
        }
        ranges.sort_by_key(|(range_start, _)| *range_start);
    }

    pub(super) fn record_mmap_region(
        &self,
        start: usize,
        size: usize,
        prot: u32,
        shared: bool,
        anonymous: bool,
        locked: bool,
        grow_down: bool,
        may_write: bool,
        file_backing: Option<super::UserMmapFileBacking>,
    ) {
        let Some(end) = start.checked_add(size) else {
            return;
        };
        self.forget_mmap_region(start, end);
        {
            let mut ranges = self.mmap_ranges.lock();
            ranges.push(super::UserMmapRegion {
                start,
                size,
                prot,
                shared,
                anonymous,
                locked,
                wipe_on_fork: false,
                grow_down,
                may_write,
                file_backing,
            });
            ranges.sort_by_key(|region| region.start);
        }
    }

    pub(super) fn mmap_range_denies_write(&self, start: usize, end: usize) -> bool {
        self.mmap_ranges.lock().iter().any(|region| {
            region.shared && !region.may_write && region.start < end && region.end() > start
        })
    }

    pub(super) fn has_shared_writable_mmap_for_memfd(
        &self,
        file: &super::fd_table::MemfdEntry,
    ) -> bool {
        self.mmap_ranges.lock().iter().any(|region| {
            region.shared
                && region.may_write
                && region.prot & general::PROT_WRITE != 0
                && region
                    .file_backing
                    .as_ref()
                    .map_or(false, |backing| backing.file.is_memfd_backing(file))
        })
    }

    pub(super) fn mmap_range_has_locked(&self, start: usize, end: usize) -> bool {
        self.mmap_ranges
            .lock()
            .iter()
            .any(|region| region.locked && region.start < end && region.end() > start)
    }

    pub(super) fn set_all_mmap_locked(&self, locked: bool) {
        for region in self.mmap_ranges.lock().iter_mut() {
            region.locked = locked;
        }
    }

    pub(super) fn set_mmap_lock_range(&self, start: usize, end: usize, locked: bool) {
        let mut ranges = self.mmap_ranges.lock();
        let old = core::mem::take(&mut *ranges);
        for region in old {
            let region_end = region.end();
            if region_end <= start || region.start >= end {
                ranges.push(region);
                continue;
            }
            if region.start < start {
                ranges.push(region.subregion_with_lock(region.start, start, region.locked));
            }
            let locked_start = region.start.max(start);
            let locked_end = region_end.min(end);
            if locked_end > locked_start {
                ranges.push(region.subregion_with_lock(locked_start, locked_end, locked));
            }
            if region_end > end {
                ranges.push(region.subregion_with_lock(end, region_end, region.locked));
            }
        }
        ranges.sort_by_key(|region| region.start);
    }

    pub(super) fn set_mmap_wipe_on_fork_range(&self, start: usize, end: usize, enabled: bool) {
        let mut ranges = self.mmap_ranges.lock();
        let old = core::mem::take(&mut *ranges);
        for region in old {
            let region_end = region.end();
            if region_end <= start || region.start >= end {
                ranges.push(region);
                continue;
            }
            if region.start < start {
                ranges.push(region.subregion(region.start, start, region.prot));
            }
            let selected_start = region.start.max(start);
            let selected_end = region_end.min(end);
            if selected_end > selected_start {
                let mut selected = region.subregion(selected_start, selected_end, region.prot);
                selected.wipe_on_fork = enabled;
                ranges.push(selected);
            }
            if region_end > end {
                ranges.push(region.subregion(end, region_end, region.prot));
            }
        }
        ranges.sort_by_key(|region| region.start);
    }

    pub(super) fn protect_mmap_region(&self, start: usize, end: usize, prot: u32) {
        let mut ranges = self.mmap_ranges.lock();
        let old = core::mem::take(&mut *ranges);
        for region in old {
            let region_end = region.end();
            if region_end <= start || region.start >= end {
                ranges.push(region);
                continue;
            }
            if region.start < start {
                ranges.push(region.subregion(region.start, start, region.prot));
            }
            let protected_start = region.start.max(start);
            let protected_end = region_end.min(end);
            if protected_end > protected_start {
                ranges.push(region.subregion(protected_start, protected_end, prot));
            }
            if region_end > end {
                ranges.push(region.subregion(end, region_end, region.prot));
            }
        }
        ranges.sort_by_key(|region| region.start);
    }

    pub(super) fn forget_mmap_region(&self, start: usize, end: usize) {
        self.forget_mmap_sigbus_range(start, end);
        let mut ranges = self.mmap_ranges.lock();
        let old = core::mem::take(&mut *ranges);
        for region in old {
            let region_end = region.end();
            if region_end <= start || region.start >= end {
                ranges.push(region);
                continue;
            }
            if region.start < start {
                ranges.push(region.subregion(region.start, start, region.prot));
            }
            if region_end > end {
                ranges.push(region.subregion(end, region_end, region.prot));
            }
        }
        ranges.sort_by_key(|region| region.start);
    }

    pub(super) fn mmap_regions(&self) -> Vec<super::UserMmapRegion> {
        self.mmap_ranges.lock().clone()
    }

    fn mmap_sigbus_ranges(&self) -> Vec<(usize, usize)> {
        self.mmap_sigbus_ranges.lock().clone()
    }

    pub(super) fn locked_mmap_kb(&self) -> usize {
        let mmap_kb = self
            .mmap_ranges
            .lock()
            .iter()
            .filter(|region| region.locked)
            .map(|region| region.size / 1024)
            .sum::<usize>();
        mmap_kb + self.mlockall_accounted_kb.load(Ordering::Acquire)
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
        self.fork_with_fd_sharing(false, false, false, None)
    }

    pub(super) fn fork_with_fd_sharing(
        &self,
        share_fds: bool,
        share_vm: bool,
        share_writable_mappings: bool,
        child_stack: Option<usize>,
    ) -> Result<Arc<UserProcess>, LinuxError> {
        let child_aspace = if share_vm {
            self.aspace.clone()
        } else {
            let mut aspace =
                axmm::new_user_aspace(VirtAddr::from(USER_ASPACE_BASE), USER_ASPACE_SIZE)
                    .map_err(LinuxError::from)?;
            let brk_snapshot = *self.brk.lock();
            {
                let mut parent_aspace = self.aspace.lock();
                populate_committed_brk_for_fork(
                    &mut parent_aspace,
                    brk_snapshot.start,
                    brk_snapshot.end,
                );
                prefault_clone_child_stack_for_fork(&mut parent_aspace, child_stack);
                if share_writable_mappings {
                    aspace
                        .share_user_mappings_from(&mut parent_aspace)
                        .map_err(LinuxError::from)?;
                } else {
                    aspace
                        .clone_user_mappings_from(&mut parent_aspace)
                        .map_err(LinuxError::from)?;
                }
                for (start, size, flags) in self.shared_mmap_ranges() {
                    let start = VirtAddr::from(start);
                    parent_aspace
                        .protect(start, size, flags)
                        .map_err(LinuxError::from)?;
                    aspace
                        .protect(start, size, flags)
                        .map_err(LinuxError::from)?;
                }
                for (start, end) in self.mmap_sigbus_ranges() {
                    let start = VirtAddr::from(start);
                    let size = end - start.as_usize();
                    parent_aspace
                        .protect(start, size, MappingFlags::USER)
                        .map_err(LinuxError::from)?;
                    aspace
                        .protect(start, size, MappingFlags::USER)
                        .map_err(LinuxError::from)?;
                }
            }
            Arc::new(Mutex::new(aspace))
        };

        let shm_attachments = self.shm_attachments.lock().clone();
        sysv_shm::retain_attachments(&shm_attachments);
        let mut child_mmap_ranges = self.mmap_regions();
        for region in child_mmap_ranges.iter_mut() {
            region.locked = false;
        }
        let wipe_on_fork_ranges: Vec<(usize, usize)> = child_mmap_ranges
            .iter()
            .filter(|region| region.wipe_on_fork)
            .map(|region| (region.start, region.end()))
            .collect();
        let child_fds = if share_fds {
            self.fds.clone()
        } else {
            Arc::new(ProcessFdTable::from_table(
                self.fds.fork_copy_for_pid(self.pid())?,
            ))
        };

        let child = Arc::new(UserProcess {
            aspace: child_aspace,
            owns_aspace: !share_vm,
            brk: Mutex::new(*self.brk.lock()),
            shared_mmap_ranges: Mutex::new(self.shared_mmap_ranges()),
            mmap_sigbus_ranges: Mutex::new(self.mmap_sigbus_ranges()),
            mmap_ranges: Mutex::new(child_mmap_ranges),
            exec_shared_mmap_cache: Mutex::new(Vec::new()),
            mlock_future: AtomicBool::new(false),
            mlockall_accounted_kb: AtomicUsize::new(0),
            fds: child_fds,
            cwd: Mutex::new(self.cwd()),
            fs_root: Mutex::new(self.fs_root()),
            exec_root: Mutex::new(self.exec_root()),
            exec_path: Mutex::new(self.exec_path()),
            hostname: self.hostname.clone(),
            domainname: self.domainname.clone(),
            prctl_name: Mutex::new(self.prctl_name()),
            children: Mutex::new(Vec::new()),
            child_exit_wait: WaitQueue::new(),
            rlimits: Mutex::new(self.rlimits.lock().clone()),
            sched_state: Mutex::new(self.get_sched_state()),
            nice: AtomicI32::new(self.nice()),
            ioprio: AtomicU32::new(self.ioprio()),
            signal_actions: Mutex::new(self.signal_actions.lock().clone()),
            path_modes: Mutex::new(self.path_modes.lock().clone()),
            path_inodes: Mutex::new(self.path_inodes.lock().clone()),
            path_special_modes: Mutex::new(self.path_special_modes.lock().clone()),
            path_rdevs: Mutex::new(self.path_rdevs.lock().clone()),
            path_owners: Mutex::new(self.path_owners.lock().clone()),
            path_symlinks: Mutex::new(self.path_symlinks.lock().clone()),
            path_hardlinks: Mutex::new(self.path_hardlinks.lock().clone()),
            path_hardlink_counts: Mutex::new(self.path_hardlink_counts.lock().clone()),
            path_inode_flags: Mutex::new(self.path_inode_flags.lock().clone()),
            path_xattrs: Mutex::new(self.path_xattrs.lock().clone()),
            path_times: Mutex::new(self.path_times.lock().clone()),
            path_sparse_sizes: Mutex::new(self.path_sparse_sizes.lock().clone()),
            path_sparse_data: Mutex::new(self.path_sparse_data.lock().clone()),
            path_data_ranges: Mutex::new(self.path_data_ranges.lock().clone()),
            umask: AtomicU32::new(self.umask.load(Ordering::Acquire)),
            mount_points: self.mount_points.clone(),
            shm_attachments: Mutex::new(shm_attachments),
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
            cap_effective: AtomicU64::new(self.cap_effective()),
            cap_permitted: AtomicU64::new(self.cap_permitted()),
            cap_inheritable: AtomicU64::new(self.cap_inheritable()),
            cap_bounding: AtomicU64::new(self.cap_bounding()),
            personality: AtomicUsize::new(self.personality()),
            parent_death_signal: AtomicI32::new(self.parent_death_signal.load(Ordering::Acquire)),
            default_timer_slack_ns: AtomicU64::new(self.timer_slack_ns()),
            timer_slack_ns: AtomicU64::new(self.timer_slack_ns()),
            posix_timers: Mutex::new(BTreeMap::new()),
            next_posix_timer_id: AtomicI32::new(1),
            real_timer_generation: AtomicU64::new(0),
            real_timer_deadline_us: AtomicU64::new(0),
            real_timer_interval_us: AtomicU64::new(0),
            virtual_timer_deadline_us: AtomicU64::new(0),
            virtual_timer_interval_us: AtomicU64::new(0),
            prof_timer_deadline_us: AtomicU64::new(0),
            prof_timer_interval_us: AtomicU64::new(0),
            start_clock_ticks: AtomicU64::new(super::time_abi::clock_ticks_now()),
            waited_child_user_ticks: AtomicU64::new(0),
            waited_child_system_ticks: AtomicU64::new(0),
            max_rss_kb: AtomicUsize::new(self.self_maxrss_kb()),
            waited_child_maxrss_kb: AtomicUsize::new(0),
            eval_watchdog_deadline_us: AtomicU64::new(
                self.eval_watchdog_deadline_us.load(Ordering::Acquire),
            ),
            child_wait_blocked: AtomicBool::new(false),
            syscall_wait_blocked: AtomicBool::new(false),
            vfork_exec_done: AtomicBool::new(false),
            pid: AtomicI32::new(0),
            pgid: AtomicI32::new(self.pgid()),
            sid: AtomicI32::new(self.sid()),
            ppid: axtask::current().id().as_u64() as i32,
            live_threads: AtomicUsize::new(1),
            exit_group_code: AtomicI32::new(NO_EXIT_GROUP_CODE),
            exit_code: AtomicI32::new(0),
            term_signal: AtomicI32::new(0),
            wait_stopped_signal: AtomicI32::new(0),
            wait_continued_signal: AtomicI32::new(0),
            exit_wait: WaitQueue::new(),
            teardown: ProcessTeardown::new(),
        });
        if !share_vm {
            zero_child_wipe_on_fork_ranges(child.as_ref(), &wipe_on_fork_ranges)?;
        }
        track_running_executable(child.exec_path().as_str());
        Ok(child)
    }

    pub(super) fn add_child(&self, task: AxTaskRef, process: Arc<UserProcess>) -> i32 {
        let pid = task.id().as_u64() as i32;
        self.children.lock().push(ChildTask { pid, task, process });
        pid
    }

    pub(super) fn record_wait_stopped(&self, sig: i32) {
        self.wait_continued_signal.store(0, Ordering::Release);
        self.wait_stopped_signal.store(sig, Ordering::Release);
        notify_parent_child_exit(self.ppid, self.pid());
    }

    pub(super) fn record_wait_continued(&self, sig: i32) {
        self.wait_stopped_signal.store(0, Ordering::Release);
        self.wait_continued_signal.store(sig, Ordering::Release);
        notify_parent_child_exit(self.ppid, self.pid());
    }

    fn wait_child_signal_event(
        &self,
        pid: i32,
        nohang: bool,
        want_stopped: bool,
        want_continued: bool,
        consume: bool,
    ) -> Result<Option<(i32, i32, i32)>, LinuxError> {
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

        fn child_event(
            child: &ChildTask,
            want_stopped: bool,
            want_continued: bool,
        ) -> Option<(i32, i32)> {
            if want_stopped {
                let sig = child.process.wait_stopped_signal.load(Ordering::Acquire);
                if sig != 0 {
                    return Some((sig, general::CLD_STOPPED as i32));
                }
            }
            if want_continued {
                let sig = child.process.wait_continued_signal.load(Ordering::Acquire);
                if sig != 0 {
                    return Some((sig, general::CLD_CONTINUED as i32));
                }
            }
            None
        }

        if pid < -1 && pid.checked_neg().is_none() {
            return Err(LinuxError::ESRCH);
        }
        let current_pgid = self.pgid();

        loop {
            let maybe_event = {
                let children = self.children.lock();
                if children.is_empty() {
                    return Err(LinuxError::ECHILD);
                }
                if !children
                    .iter()
                    .any(|child| wait_pid_matches(child, pid, current_pgid))
                {
                    return Err(LinuxError::ECHILD);
                }
                children
                    .iter()
                    .find(|child| {
                        wait_pid_matches(child, pid, current_pgid)
                            && child_event(child, want_stopped, want_continued).is_some()
                    })
                    .and_then(|child| {
                        child_event(child, want_stopped, want_continued)
                            .map(|(sig, code)| (child.pid, sig, code))
                    })
            };

            if let Some((child_pid, sig, code)) = maybe_event {
                if consume {
                    let children = self.children.lock();
                    if let Some(child) = children.iter().find(|child| child.pid == child_pid) {
                        if code == general::CLD_STOPPED as i32 {
                            child
                                .process
                                .wait_stopped_signal
                                .store(0, Ordering::Release);
                        } else {
                            child
                                .process
                                .wait_continued_signal
                                .store(0, Ordering::Release);
                        }
                    }
                }
                return Ok(Some((child_pid, sig, code)));
            }

            if nohang {
                axtask::yield_now();
                return Ok(None);
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
                    || children.iter().any(|child| {
                        wait_pid_matches(child, pid, current_pgid)
                            && child_event(child, want_stopped, want_continued).is_some()
                    })
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
        }
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
        let child_usage = super::time_abi::process_times(child.process.as_ref());
        self.waited_child_user_ticks.fetch_add(
            child_usage
                .tms_utime
                .saturating_add(child_usage.tms_cutime)
                .max(0) as u64,
            Ordering::AcqRel,
        );
        self.waited_child_system_ticks.fetch_add(
            child_usage
                .tms_stime
                .saturating_add(child_usage.tms_cstime)
                .max(0) as u64,
            Ordering::AcqRel,
        );
        let child_maxrss = child
            .process
            .self_maxrss_kb()
            .max(child.process.child_maxrss_kb());
        self.record_waited_child_maxrss(child_maxrss);
        let _ = child.task.join();
        child.process.teardown();
        drop(child);
        yield_for_task_gc();
        Ok(Some((child_pid, status)))
    }

    pub(super) fn child_exited(&self, pid: i32) -> Result<bool, LinuxError> {
        let children = self.children.lock();
        if children.is_empty() {
            return Err(LinuxError::ECHILD);
        }
        children
            .iter()
            .find(|child| child.pid == pid)
            .map(|child| child.process.live_threads.load(Ordering::Acquire) == 0)
            .ok_or(LinuxError::ECHILD)
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
            .find(|child| child.pid == pid)
            .map(|child| UserThreadEntry {
                task: child.task.clone(),
                process: child.process.clone(),
            })
    }
}

fn notify_parent_child_exit(ppid: i32, child_pid: i32) {
    if let Some(parent) = user_thread_entry_by_process_pid(ppid) {
        if parent.process.sigchld_discards_wait_status()
            && parent.process.reap_ignored_child(child_pid)
        {
            return;
        }
        parent.process.child_exit_wait.notify_all(false);
    }
}

fn exec_path_exceeds_linux_limits(path: &str) -> bool {
    path.len() >= EXEC_PATH_MAX
        || path
            .split('/')
            .any(|component| component.len() > EXEC_NAME_MAX)
}

fn exec_path_stat(process: &UserProcess, path: &str) -> Result<(general::stat, bool), LinuxError> {
    let attr = axfs::api::metadata(path).map_err(LinuxError::from)?;
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_dev = 1;
    st.st_ino = path_inode(Some(path));
    st.st_mode = file_type_mode(attr.file_type()) | attr.permissions().bits() as u32;
    st.st_nlink = 1;
    st.st_size = attr.size() as _;
    st.st_blksize = 512;
    st.st_blocks = attr.blocks() as _;
    let has_recorded_mode = process.path_mode(path).is_some();
    Ok((
        apply_recorded_path_metadata(process, path, st),
        has_recorded_mode,
    ))
}

fn check_exec_parent_components(
    process: &UserProcess,
    path: &str,
    uid: u32,
    gid: u32,
) -> Result<(), LinuxError> {
    let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
    if components.len() <= 1 {
        return Ok(());
    }

    let mut current = String::new();
    for component in &components[..components.len() - 1] {
        current.push('/');
        current.push_str(component);
        let (st, has_recorded_mode) = exec_path_stat(process, current.as_str())?;
        if st.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
            return Err(LinuxError::ENOTDIR);
        }
        if (has_recorded_mode || st.st_mode & 0o111 != 0)
            && !access_allowed(&st, ACCESS_X_OK, uid, gid)
        {
            return Err(LinuxError::EACCES);
        }
    }
    Ok(())
}

fn validate_execve_target(process: &UserProcess, cwd: &str, path: &str) -> Result<(), LinuxError> {
    if path.is_empty() {
        return Err(LinuxError::ENOENT);
    }
    if exec_path_exceeds_linux_limits(path) {
        return Err(LinuxError::ENAMETOOLONG);
    }

    let normalized = normalize_path(cwd, path).ok_or(LinuxError::EINVAL)?;
    if exec_path_exceeds_linux_limits(normalized.as_str()) {
        return Err(LinuxError::ENAMETOOLONG);
    }
    let resolved = process
        .resolve_path_symlink(normalized.as_str())?
        .unwrap_or(normalized);
    let physical = process
        .path_hardlink_backing(resolved.as_str())
        .unwrap_or_else(|| resolved.clone());
    let uid = process.fs_uid();
    let gid = process.fs_gid();
    check_exec_parent_components(process, resolved.as_str(), uid, gid)?;
    let (st, has_recorded_mode) = exec_path_stat(process, physical.as_str())?;
    if (has_recorded_mode || st.st_mode & 0o111 != 0) && !access_allowed(&st, ACCESS_X_OK, uid, gid)
    {
        return Err(LinuxError::EACCES);
    }
    if executable_write_open(physical.as_str()) {
        return Err(LinuxError::ETXTBSY);
    }
    Ok(())
}

pub(super) fn sys_execve(
    process: &UserProcess,
    _tf: &TrapFrame,
    pathname: usize,
    argv: usize,
    _envp: usize,
) -> isize {
    let raw_path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let mut argv = match read_execve_argv(process, argv, raw_path.as_str()) {
        Ok(argv) => argv,
        Err(err) => return neg_errno(err),
    };
    let path = resolve_execve_compat_path(process, raw_path.clone(), &mut argv);
    let env = match read_execve_envp(process, _envp) {
        Ok(env) => env,
        Err(err) => return neg_errno(err),
    };
    let cwd = process.cwd();
    if let Err(err) = validate_execve_target(process, cwd.as_str(), path.as_str()) {
        return neg_errno(err);
    }
    if !process.owns_aspace {
        return neg_errno(LinuxError::EAGAIN);
    }
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
        | general::CLONE_FILES as usize
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

        let share_fds = clone_flags & general::CLONE_FILES as usize != 0;
        let vfork_requested = clone_flags & vfork_flags == vfork_flags;
        // Treat vfork-style process creation as fork-like for address-space
        // ownership while still blocking the parent below.  The child gets a
        // separate page table with writable pages shared, preserving vfork's
        // pre-exec memory visibility without letting child exec/exit clear the
        // parent's AddrSpace.
        let share_vm = false;
        let share_writable_mappings = vfork_requested;
        let child_process = match process.fork_with_fd_sharing(
            share_fds,
            share_vm,
            share_writable_mappings,
            (!share_vm).then_some(child_stack),
        ) {
            Ok(process) => process,
            Err(err) => return neg_errno(err),
        };
        let mut child_tf = child_trap_frame(tf, child_stack);
        if clone_flags & general::CLONE_SETTLS as usize != 0 {
            child_tf.regs.tp = tls;
        }
        #[cfg(target_arch = "riscv64")]
        if child_stack != 0 {
            fixup_riscv_clone_child_return(process.as_ref(), &mut child_tf);
        }
        let child_context = UspaceContext::from(&child_tf);
        let task_process = child_process.clone();
        let mut task = TaskInner::new(
            move || user_task_entry(task_process, child_context),
            "user:fork".into(),
            USER_TASK_KSTACK_SIZE,
        );
        let pid = task.id().as_u64() as i32;
        child_process.set_pid(pid);
        if share_fds {
            process.fds.share_table_for_child_pid(process.pid(), pid);
        }
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
        process.add_child(task, child_process.clone());
        if vfork_requested {
            let wait_condition = || {
                child_process.live_threads.load(Ordering::Acquire) == 0
                    || child_process.vfork_exec_done.load(Ordering::Acquire)
                    || process.pending_exit_group().is_some()
                    || process.eval_watchdog_expired()
            };
            if let Some(timeout) = process.eval_watchdog_remaining() {
                if child_process
                    .exit_wait
                    .wait_timeout_until(timeout, wait_condition)
                {
                    return neg_errno(LinuxError::EINTR);
                }
            } else {
                child_process.exit_wait.wait_until(wait_condition);
            }
            if process.pending_exit_group().is_some() || process.eval_watchdog_expired() {
                return neg_errno(LinuxError::EINTR);
            }
        }
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

fn waitid_signal_siginfo(child_pid: i32, sig: i32, code: i32) -> [u8; 128] {
    let mut info = [0u8; 128];
    info[0..4].copy_from_slice(&(general::SIGCHLD as i32).to_ne_bytes());
    info[4..8].copy_from_slice(&0i32.to_ne_bytes());
    info[8..12].copy_from_slice(&code.to_ne_bytes());
    info[16..20].copy_from_slice(&child_pid.to_ne_bytes());
    info[20..24].copy_from_slice(&0u32.to_ne_bytes());
    info[24..28].copy_from_slice(&sig.to_ne_bytes());
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
    const SUPPORTED_WAITID_OPTIONS: u32 = general::WNOHANG
        | general::WEXITED
        | general::WSTOPPED
        | general::WCONTINUED
        | general::WNOWAIT
        | general::__WNOTHREAD
        | general::__WALL;

    let options = options as u32;
    let waitable = general::WEXITED | general::WSTOPPED | general::WCONTINUED;
    if options & !SUPPORTED_WAITID_OPTIONS != 0 || options & waitable == 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let nohang = options & general::WNOHANG != 0;
    let (pid_filter, pidfd_nonblock) = if idtype == general::P_PIDFD {
        match process.fds.lock().pidfd_wait_target(id) {
            Ok((pid, nonblock)) => (pid, nonblock),
            Err(err) => return neg_errno(err),
        }
    } else {
        match waitid_pid_filter(idtype, id) {
            Ok(pid_filter) => (pid_filter, false),
            Err(err) => return neg_errno(err),
        }
    };
    let want_stopped = options & general::WSTOPPED != 0;
    let want_continued = options & general::WCONTINUED != 0;
    if pidfd_nonblock && !nohang && !want_stopped && !want_continued {
        match process.child_exited(pid_filter) {
            Ok(true) => {}
            Ok(false) => return neg_errno(LinuxError::EAGAIN),
            Err(err) => return neg_errno(err),
        }
    }
    if want_stopped || want_continued {
        let wait_result = match process.wait_child_signal_event(
            pid_filter,
            nohang,
            want_stopped,
            want_continued,
            options & general::WNOWAIT == 0,
        ) {
            Ok(result) => result,
            Err(err) => return neg_errno(err),
        };
        let info = if let Some((child_pid, sig, code)) = wait_result {
            waitid_signal_siginfo(child_pid, sig, code)
        } else {
            [0u8; 128]
        };
        if infop != 0 {
            if let Err(err) = write_user_bytes(process, infop, &info) {
                return neg_errno(err);
            }
        }
        return 0;
    }

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

pub(super) fn sys_exit(process: &UserProcess, tf: &TrapFrame, code: i32) -> ! {
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

pub(super) fn sys_exit_group(process: &UserProcess, tf: &TrapFrame, code: i32) -> ! {
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
    release_current_robust_futexes();
    clear_current_tid_and_wake();
    perform_deferred_self_unmap();
    unregister_user_task(current_tid());
    if teardown_now {
        process.teardown();
    }
    process.note_thread_exit(code);
    axtask::exit(code)
}

fn release_current_robust_futexes() {
    const ROBUST_LIST_NEXT_OFFSET: usize = 0;
    const ROBUST_LIST_FUTEX_OFFSET_OFFSET: usize = size_of::<usize>();
    const ROBUST_LIST_PENDING_OFFSET: usize = size_of::<usize>() * 2;
    const MAX_ROBUST_LIST_NODES: usize = 2048;

    let Some(ext) = current_task_ext() else {
        return;
    };
    let head = ext.robust_list_head.load(Ordering::Acquire);
    if head == 0 {
        return;
    }
    let process = ext.process.as_ref();
    let tid = current_tid() as u32;
    let head_list = head + ROBUST_LIST_NEXT_OFFSET;
    let futex_offset =
        match read_user_value::<isize>(process, head + ROBUST_LIST_FUTEX_OFFSET_OFFSET) {
            Ok(offset) => offset,
            Err(_) => return,
        };
    let pending = read_user_value::<usize>(process, head + ROBUST_LIST_PENDING_OFFSET).unwrap_or(0);

    let mut node = match read_user_value::<usize>(process, head_list) {
        Ok(next) => next,
        Err(_) => return,
    };
    let mut visited = 0usize;
    while node != 0 && node != head_list && visited < MAX_ROBUST_LIST_NODES {
        let next = read_user_value::<usize>(process, node).unwrap_or(0);
        release_robust_futex_node(process, node, futex_offset, tid);
        node = next;
        visited += 1;
    }
    if pending != 0 {
        release_robust_futex_node(process, pending, futex_offset, tid);
    }
}

fn release_robust_futex_node(process: &UserProcess, node: usize, futex_offset: isize, tid: u32) {
    let Some(futex_addr) = node.checked_add_signed(futex_offset) else {
        return;
    };
    let Ok(owner) = read_user_value::<u32>(process, futex_addr) else {
        return;
    };
    if owner & general::FUTEX_TID_MASK != tid {
        return;
    }
    let next = (owner & general::FUTEX_WAITERS) | general::FUTEX_OWNER_DIED;
    if write_user_value(process, futex_addr, &next) == 0 && owner & general::FUTEX_WAITERS != 0 {
        let _ = futex::wake_addr(process, futex_addr, usize::MAX);
    }
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
