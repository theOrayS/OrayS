use core::mem::size_of;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use core::time::Duration;

use axalloc::frame_allocator_stats;
use axerrno::LinuxError;
use axhal::context::{TrapFrame, UspaceContext};
use axhal::paging::MappingFlags;
use axhal::trap::PageFaultFlags;
use axmm::AddrSpace;
use axsync::Mutex;
use axtask::{self, AxTaskRef, TaskInner, WaitQueue};
use lazyinit::LazyInit;
use linux_raw_sys::general;
use memory_addr::{PAGE_SIZE_4K, PageIter4K, VirtAddr};
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::sync::Arc;
#[cfg(feature = "auto-run-tests")]
use std::sync::Weak;
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
use super::program_loader::{
    EXEC_LOADER_ENOMEM_PREFIX, LoadedMapping, exec_loader_axerr, exec_loader_string_refs,
    load_program_image,
};
use super::resource_sched::{
    apply_process_scheduler_state_to_task, child_sched_state_from_parent, default_sched_state,
};
use super::runtime_paths::{
    current_cwd, normalize_path, try_normalize_path, try_runtime_absolute_path_candidates,
    try_staged_cwd_absolute_path_candidates,
};
use super::signal_abi::{
    all_application_signal_mask, current_unblocked_signal_pending, deliver_user_signal,
    ensure_user_return_hook_registered, thread_waits_for_signal,
};
use super::sysv_shm;
use super::task_context::{
    UserTaskExt, child_trap_frame, current_task_ext, current_tid, make_uspace_context, task_ext,
    user_pc,
};
#[cfg(feature = "auto-run-tests")]
use super::task_registry::live_user_thread_entries;
use super::task_registry::{
    UserThreadEntry, live_user_thread_count, prune_exited_user_tasks, register_user_task,
    unregister_user_task_with_runtime, user_thread_entries_by_process_pid,
    user_thread_entry_by_process_pid,
};
use super::user_memory::{
    MAX_USER_IO_CHUNK, read_cstr, read_execve_argv, read_execve_envp, read_user_value,
    write_user_bytes, write_user_value,
};
use super::{ChildTask, DEFAULT_TIMER_SLACK_NS, NO_EXIT_GROUP_CODE, ProcessFdTable, UserProcess};

const MAX_LIVE_USER_THREADS: usize = 512;
const MIN_FORK_FREE_FRAMES: usize = 8192;
const USER_TASK_EXIT_JOIN_GRACE: Duration = Duration::from_secs(2);
// User tasks execute all Linux syscall emulation and the underlying ArceOS
// networking/filesystem paths on their kernel stack.  glibc network workloads
// such as netperf's TCP_RR/CRR path nest socket, scheduler, and userspace-copy
// helpers deeply enough that the old 16 KiB stack could corrupt adjacent heap
// state before task exit.  Use a larger real kernel stack rather than special
// casing any benchmark.
const USER_TASK_KSTACK_SIZE: usize = 64 * 1024;
const EXEC_PATH_MAX: usize = 4096;
const EXEC_NAME_MAX: usize = 255;

#[cfg(feature = "auto-run-tests")]
static USER_PROCESS_OBJECTS: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "auto-run-tests")]
static USER_PROCESS_CREATED: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "auto-run-tests")]
static USER_PROCESS_DROPPED: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "auto-run-tests")]
static USER_PROCESS_WEAKS: LazyInit<Mutex<Vec<Weak<UserProcess>>>> = LazyInit::new();

#[cfg(feature = "auto-run-tests")]
fn user_process_weaks() -> &'static Mutex<Vec<Weak<UserProcess>>> {
    let _ = USER_PROCESS_WEAKS.call_once(|| Mutex::new(Vec::new()));
    &USER_PROCESS_WEAKS
}

static GLOBAL_PATH_SPARSE_SIZES: LazyInit<Mutex<BTreeMap<String, u64>>> = LazyInit::new();
static GLOBAL_PATH_SPARSE_DATA: LazyInit<Mutex<BTreeMap<String, Vec<(u64, Vec<u8>)>>>> =
    LazyInit::new();
static GLOBAL_PATH_SPARSE_REPEATS: LazyInit<Mutex<BTreeMap<String, Vec<(u64, u64, u8)>>>> =
    LazyInit::new();
static GLOBAL_PATH_DATA_RANGES: LazyInit<Mutex<BTreeMap<String, Vec<(u64, u64)>>>> =
    LazyInit::new();

fn global_path_sparse_sizes() -> &'static Mutex<BTreeMap<String, u64>> {
    let _ = GLOBAL_PATH_SPARSE_SIZES.call_once(|| Mutex::new(BTreeMap::new()));
    &GLOBAL_PATH_SPARSE_SIZES
}

fn global_path_sparse_data() -> &'static Mutex<BTreeMap<String, Vec<(u64, Vec<u8>)>>> {
    let _ = GLOBAL_PATH_SPARSE_DATA.call_once(|| Mutex::new(BTreeMap::new()));
    &GLOBAL_PATH_SPARSE_DATA
}

fn global_path_sparse_repeats() -> &'static Mutex<BTreeMap<String, Vec<(u64, u64, u8)>>> {
    let _ = GLOBAL_PATH_SPARSE_REPEATS.call_once(|| Mutex::new(BTreeMap::new()));
    &GLOBAL_PATH_SPARSE_REPEATS
}

fn global_path_data_ranges() -> &'static Mutex<BTreeMap<String, Vec<(u64, u64)>>> {
    let _ = GLOBAL_PATH_DATA_RANGES.call_once(|| Mutex::new(BTreeMap::new()));
    &GLOBAL_PATH_DATA_RANGES
}

fn first_overlapping_mmap_region(ranges: &[super::UserMmapRegion], start: usize) -> usize {
    ranges.partition_point(|region| region.end() <= start)
}

fn overlapping_mmap_regions<'a>(
    ranges: &'a [super::UserMmapRegion],
    start: usize,
    end: usize,
) -> impl Iterator<Item = &'a super::UserMmapRegion> + 'a {
    let first = first_overlapping_mmap_region(ranges, start);
    ranges[first..]
        .iter()
        .take_while(move |region| region.start < end)
}

#[cfg(feature = "auto-run-tests")]
fn record_user_process_created(process: &Arc<UserProcess>) {
    USER_PROCESS_OBJECTS.fetch_add(1, Ordering::AcqRel);
    USER_PROCESS_CREATED.fetch_add(1, Ordering::AcqRel);
    user_process_weaks().lock().push(Arc::downgrade(process));
}

#[cfg(feature = "auto-run-tests")]
pub fn user_process_object_stats() -> (usize, usize, usize) {
    (
        USER_PROCESS_OBJECTS.load(Ordering::Acquire),
        USER_PROCESS_CREATED.load(Ordering::Acquire),
        USER_PROCESS_DROPPED.load(Ordering::Acquire),
    )
}

#[cfg(feature = "auto-run-tests")]
pub fn user_process_retention_stats() -> (usize, usize, usize, usize, usize, usize, usize) {
    let mut weaks = user_process_weaks().lock();
    let mut retained = 0usize;
    let mut live_threads_nonzero = 0usize;
    let mut teardown_done = 0usize;
    let mut exit_pending = 0usize;
    let mut total_child_edges = 0usize;
    let mut max_child_edges = 0usize;
    let mut max_strong_count = 0usize;

    let mut index = 0;
    while index < weaks.len() {
        let Some(process) = weaks[index].upgrade() else {
            weaks.remove(index);
            continue;
        };
        retained += 1;
        if process.live_threads.load(Ordering::Acquire) != 0 {
            live_threads_nonzero += 1;
        }
        if process.teardown.is_done() {
            teardown_done += 1;
        }
        if process.pending_exit_group().is_some() {
            exit_pending += 1;
        }
        let child_edges = process.children.lock().len();
        total_child_edges += child_edges;
        max_child_edges = max_child_edges.max(child_edges);
        max_strong_count = max_strong_count.max(Arc::strong_count(&process).saturating_sub(1));
        index += 1;
    }

    (
        retained,
        live_threads_nonzero,
        teardown_done,
        exit_pending,
        total_child_edges,
        max_child_edges,
        max_strong_count,
    )
}

fn zero_child_wipe_on_fork_ranges(
    aspace: &Arc<Mutex<AddrSpace>>,
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
            aspace
                .lock()
                .write(VirtAddr::from(cursor), &zeros[..len])
                .map_err(LinuxError::from)?;
            cursor += len;
        }
    }
    Ok(())
}

fn range_segments_excluding(
    start: usize,
    end: usize,
    excluded: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    if end <= start {
        return Vec::new();
    }
    let mut segments = vec![(start, end)];
    for (excluded_start, excluded_end) in excluded {
        if excluded_end <= excluded_start {
            continue;
        }
        let mut next = Vec::new();
        for (segment_start, segment_end) in segments {
            if segment_end <= *excluded_start || segment_start >= *excluded_end {
                next.push((segment_start, segment_end));
                continue;
            }
            if segment_start < *excluded_start {
                next.push((segment_start, (*excluded_start).min(segment_end)));
            }
            if segment_end > *excluded_end {
                next.push(((*excluded_end).max(segment_start), segment_end));
            }
        }
        segments = next;
        if segments.is_empty() {
            break;
        }
    }
    segments
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
    // scratch subdirectories.
    modes.insert(String::from("/tmp"), 0o1777);
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

    pub(super) fn is_done(&self) -> bool {
        self.done.load(Ordering::Acquire)
    }

    pub(super) fn run(
        &self,
        pid: i32,
        exec_path: &Mutex<String>,
        aspace: &Mutex<AddrSpace>,
        clear_aspace: bool,
        fds: &ProcessFdTable,
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
        fds.close_all_for_pid(pid);
        children.lock().clear();
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
pub fn run_user_program_in_timeout_with_env(
    cwd: &str,
    argv: &[&str],
    env: &[String],
    timeout_secs: u64,
) -> Result<i32, String> {
    run_user_program_in_with_env_and_timeout(cwd, argv, Some(env), Some(timeout_secs))
}

#[cfg(feature = "auto-run-tests")]
pub fn cleanup_user_processes() {
    for _ in 0..16 {
        let mut seen = Vec::new();
        let mut processes = Vec::new();
        for entry in live_user_thread_entries() {
            let pid = entry.process.pid();
            if seen.contains(&pid) {
                continue;
            }
            seen.push(pid);
            processes.push(entry.process);
        }
        if processes.is_empty() {
            break;
        }
        for process in &processes {
            process.request_eval_exit_tree(137);
        }
        yield_for_task_gc();

        // Some benchmark/helper programs intentionally daemonize or block in a
        // socket syscall after their launcher exits.  The auto-runner cleanup is
        // an evaluator boundary, not Linux exit_group(2) behavior: once the
        // whole launched process tree has been asked to terminate, release live
        // descriptor resources that would otherwise keep listeners, pipes, or
        // files visible to the next independent group.  Do not clear the address
        // space here; the task may still be unwinding from a blocking syscall and
        // will run the full teardown when it observes the pending exit request.
        for process in &processes {
            if process.live_threads.load(Ordering::Acquire) != 0
                && process.pending_exit_group().is_some()
            {
                process.close_eval_file_descriptors();
            }
        }
        yield_for_task_gc();
        if live_user_thread_count() == 0 {
            break;
        }
    }
    yield_for_task_gc();
    super::futex::prune_empty_futexes();
}

#[cfg(feature = "auto-run-tests")]
pub fn live_user_task_count_for_diagnostics() -> usize {
    live_user_thread_count()
}

fn run_user_program_in_with_timeout(
    cwd: &str,
    argv: &[&str],
    timeout_secs: Option<u64>,
) -> Result<i32, String> {
    run_user_program_in_with_env_and_timeout(cwd, argv, None, timeout_secs)
}

fn run_user_program_in_with_env_and_timeout(
    cwd: &str,
    argv: &[&str],
    env: Option<&[String]>,
    timeout_secs: Option<u64>,
) -> Result<i32, String> {
    ensure_user_return_hook_registered();
    let loaded = if let Some(env) = env {
        load_program_with_env(cwd, argv, Some(env))?
    } else {
        load_program(cwd, argv)?
    };
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
    let context = loaded.context;
    let mut task = TaskInner::new(
        user_task_entry,
        format!("user:{}", argv[0]),
        USER_TASK_KSTACK_SIZE,
    );
    let root = loaded.process.aspace.lock().page_table_root();
    task.ctx_mut().set_page_table_root(root);
    task.init_task_ext(UserTaskExt::new(loaded.process.clone(), context, 0, 0));
    let task = axtask::spawn_task(task);
    process.set_pid(task.id().as_u64() as i32);
    register_user_task(task.clone(), process.clone());
    let exit_code = if let Some(timeout_secs) = timeout_secs {
        match process.wait_for_exit_timeout(core::time::Duration::from_secs(
            timeout_secs.saturating_add(2),
        )) {
            Some(code) => {
                let expired = process.eval_watchdog_expired();
                if expired {
                    process.request_eval_exit_tree(137);
                }
                join_user_task_for_cleanup(&task);
                process.teardown();
                if expired { 137 } else { code }
            }
            None => {
                process.request_eval_exit_tree(137);
                join_user_task_for_cleanup(&task);
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
    settle_task_gc_after_join();
    Ok(exit_code)
}

fn join_user_task_for_cleanup(task: &AxTaskRef) -> bool {
    if task.try_join().is_some() {
        return true;
    }

    let deadline = axhal::time::monotonic_time() + USER_TASK_EXIT_JOIN_GRACE;
    while axhal::time::monotonic_time() < deadline {
        reap_user_runtime_once();
        if task.try_join().is_some() {
            return true;
        }
        axtask::yield_now();
    }

    reap_user_runtime_once();
    task.try_join().is_some()
}

fn reap_user_runtime_once() {
    // Drop registry/process references before scanning axtask's exited queue so
    // exited user tasks can be reclaimed in the same evaluator boundary.  The
    // old order left tasks whose final registry reference was pruned after the
    // scan retained until a later program, which accumulates badly across full
    // musl+glibc LTP sweeps.
    prune_exited_user_tasks();
    axtask::reap_exited_tasks();
}

fn settle_task_gc_after_join() {
    reap_user_runtime_once();
    // A first pass can drop user-process child edges/FdTables whose drops make
    // more exited tasks single-owned.  A second pass is bounded and keeps this
    // cleanup local to the just-finished program instead of leaking pressure to
    // later LTP cases.
    reap_user_runtime_once();
}

fn yield_for_task_gc() {
    settle_task_gc_after_join();
    for _ in 0..64 {
        if live_user_thread_count() == 0 {
            break;
        }
        axtask::yield_now();
        settle_task_gc_after_join();
    }
    settle_task_gc_after_join();
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

fn user_task_entry() {
    let context = current_task_ext()
        .and_then(|ext| ext.initial_context.lock().take())
        .expect("user task must have an initial userspace context");
    let curr = axtask::current();
    let kstack_top = curr
        .kernel_stack_top()
        .expect("user task must have a kernel stack");
    unsafe { context.enter_uspace(kstack_top) }
}

fn load_program(cwd: &str, argv: &[&str]) -> Result<LoadedProgram, String> {
    load_program_with_env(cwd, argv, None)
}

fn load_program_with_env(
    cwd: &str,
    argv: &[&str],
    env: Option<&[String]>,
) -> Result<LoadedProgram, String> {
    let mut aspace = axmm::new_user_aspace(VirtAddr::from(USER_ASPACE_BASE), USER_ASPACE_SIZE)
        .map_err(|err| format!("failed to create user address space: {err}"))?;
    let image = load_program_image(None, &mut aspace, cwd, argv[0], argv, env)?;
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
        timer_wait: WaitQueue::new(),
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
        path_sparse_sizes: global_path_sparse_sizes(),
        path_sparse_data: global_path_sparse_data(),
        path_sparse_repeats: global_path_sparse_repeats(),
        path_data_ranges: global_path_data_ranges(),
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
        syscall_runtime_micros: AtomicU64::new(0),
        last_reported_user_micros: AtomicU64::new(0),
        last_reported_system_micros: AtomicU64::new(0),
        completed_thread_runtime_ticks: AtomicU64::new(0),
        last_reported_user_ticks: AtomicU64::new(0),
        last_reported_system_ticks: AtomicU64::new(0),
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
    #[cfg(feature = "auto-run-tests")]
    record_user_process_created(&process);
    track_running_executable(exec_path.as_str());
    record_loaded_image_mappings(process.as_ref(), &image.mappings);

    Ok(LoadedProgram {
        process,
        context: make_uspace_context(image.entry, image.stack_ptr, image.argc),
    })
}

enum ExecProgramError {
    Loader(String),
    FdTable(LinuxError),
}

fn exec_program(
    process: &UserProcess,
    cwd: &str,
    path: &str,
    argv: &[String],
    env: &[String],
) -> Result<(usize, usize, usize), ExecProgramError> {
    let argv_refs = exec_loader_string_refs(argv, "prepare exec argv references")
        .map_err(ExecProgramError::Loader)?;
    let mut new_aspace = axmm::new_user_aspace(VirtAddr::from(USER_ASPACE_BASE), USER_ASPACE_SIZE)
        .map_err(|err| exec_loader_axerr("failed to create exec address space".to_string(), err))
        .map_err(ExecProgramError::Loader)?;
    let image = load_program_image(
        Some(process),
        &mut new_aspace,
        cwd,
        path,
        &argv_refs,
        Some(env),
    )
    .map_err(ExecProgramError::Loader)?;
    let new_root = new_aspace.page_table_root();
    // This is the last fallible step.  It must complete before the live
    // address space is replaced: a failed CLONE_FILES split remains an
    // ordinary execve error returned to the still-intact caller image.
    process
        .fds
        .unshare_for_pid_if_shared(process.pid())
        .map_err(ExecProgramError::FdTable)?;
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
    process.exit_wait.notify_all(true);
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

impl UserProcess {
    fn child_creation_interrupted(&self) -> Option<i32> {
        if let Some(code) = self.pending_exit_group() {
            return Some(code);
        }
        if self.eval_watchdog_expired() || self.teardown.is_done() {
            return Some(137);
        }
        None
    }

    fn wait_interrupt_pending(&self) -> bool {
        if self.consume_expired_real_timer() && current_unblocked_signal_pending() {
            return true;
        }
        current_unblocked_signal_pending()
            || self.pending_exit_group().is_some()
            || self.eval_watchdog_expired()
    }

    fn blocking_wait_remaining(&self) -> Option<Duration> {
        let mut remaining = self.eval_watchdog_remaining();
        if let Some(timer_remaining) = self.real_timer_remaining() {
            remaining =
                Some(remaining.map_or(timer_remaining, |current| current.min(timer_remaining)));
        }
        remaining
    }

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
        self.fds.register_pid(pid);
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
        self.clear_real_itimer();
        self.clear_posix_timers();
        self.release_exec_shared_mmap_cache();
        sysv_shm::release_process_attachments(self);
        self.teardown.run(
            self.pid(),
            &self.exec_path,
            self.aspace.as_ref(),
            self.owns_aspace,
            self.fds.as_ref(),
            &self.children,
        );
    }

    #[cfg(feature = "auto-run-tests")]
    fn close_eval_file_descriptors(&self) {
        self.fds.close_all_for_pid(self.pid());
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

    fn sigchld_has_user_handler(&self) -> bool {
        let action = self
            .signal_actions
            .lock()
            .get(&(SIGCHLD_NUM as usize))
            .copied()
            .unwrap_or_else(|| unsafe { core::mem::zeroed() });
        action
            .sa_handler_kernel
            .map(|func| func as usize)
            .unwrap_or(0)
            > 1
    }

    fn reset_caught_signal_handlers_for_exec(&self) {
        // POSIX/Linux execve preserves SIG_DFL and SIG_IGN dispositions, but
        // every caught signal must become default in the new image.  User-space
        // handler addresses belong to the old program; carrying them across
        // exec can jump into unmapped or non-executable memory when a pending
        // signal is delivered in the replacement image.
        self.signal_actions.lock().retain(|_, action| {
            action
                .sa_handler_kernel
                .map(|handler| handler as usize)
                .unwrap_or(0)
                <= 1
        });
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
        self.child_exit_wait.notify_all(true);
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
            self.child_exit_wait.notify_all(true);
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
            // The last thread has made the process wait condition true.  Wake
            // waiters with reschedule requested so the runner can observe the
            // state before the exiting task continues through final task GC.
            self.exit_wait.notify_all(true);
            notify_parent_child_exit(self.ppid, self.pid());
        }
    }

    pub(super) fn request_exit_group(&self, code: i32) {
        self.request_exit_group_inner(code, 0);
    }

    pub(super) fn request_signal_exit_group(&self, sig: i32) {
        self.request_exit_group_inner(128 + sig, signal_wait_status(sig));
    }

    pub(super) fn request_eval_exit_tree(&self, code: i32) {
        self.request_eval_exit_tree_inner(code, 0);
    }

    fn request_eval_exit_tree_inner(&self, code: i32, depth: usize) {
        const MAX_EVAL_EXIT_TREE_DEPTH: usize = 64;

        // The auto-runner launches a benchmark script as one bounded unit.
        // Those scripts may fork process trees (for example hackbench/iozone).
        // When the runner watchdog or post-group cleanup fires, recursively
        // requesting descendant exit is evaluator hygiene, not Linux exit_group
        // syscall behavior.  Stop the current process first so it cannot fork
        // more children, then snapshot descendants without holding the lock.
        self.request_exit_group(code);
        if depth >= MAX_EVAL_EXIT_TREE_DEPTH {
            return;
        }
        let children = self
            .children
            .lock()
            .iter()
            .map(|child| child.process.clone())
            .collect::<Vec<_>>();
        for child in children {
            child.request_eval_exit_tree_inner(code, depth + 1);
        }
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
        self.child_exit_wait.notify_all(true);
        self.exit_wait.notify_all(true);
        // Threads in nanosleep/clock_nanosleep wait on the process timer queue.
        // An exit-group request makes those waits interruptible: wake them now so
        // they can observe pending_exit_group() instead of sleeping until their
        // original timeout and delaying group teardown.
        self.timer_wait.notify_all(true);
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
        let ranges = self.mmap_sigbus_ranges.lock();
        let cutoff = ranges.partition_point(|(start, _)| *start <= addr);
        ranges[..cutoff]
            .iter()
            .rev()
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
            let mut candidate = None;
            for region in ranges.iter() {
                if region.grow_down
                    && fault_page < region.start
                    && candidate
                        .as_ref()
                        .map_or(true, |best: &super::UserMmapRegion| {
                            region.start < best.start
                        })
                {
                    candidate = Some(region.clone());
                }
            }
            let Some(region) = candidate else {
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
            let guard_floor = fault_page.saturating_sub(STACK_GUARD_GAP);
            for other in ranges.iter() {
                let other_end = other.end();
                let same_region = other.start == region.start && other_end == region.end();
                if !same_region && other.start < grow_end && other_end > fault_page {
                    return false;
                }
                if other_end <= fault_page && other_end > guard_floor {
                    return false;
                }
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

    pub(super) fn mmap_sigbus_segments(&self, start: usize, end: usize) -> Vec<(usize, usize)> {
        if end <= start {
            return Vec::new();
        }
        let ranges = self.mmap_sigbus_ranges.lock();
        let cutoff = ranges.partition_point(|(range_start, _)| *range_start < end);
        ranges[..cutoff]
            .iter()
            .filter_map(|(range_start, range_end)| {
                let segment_start = (*range_start).max(start);
                let segment_end = (*range_end).min(end);
                (segment_end > segment_start).then_some((segment_start, segment_end))
            })
            .collect()
    }

    pub(super) fn record_mmap_sigbus_ranges<I>(&self, ranges: I)
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        let mut sigbus_ranges = self.mmap_sigbus_ranges.lock();
        for (start, end) in ranges {
            if end > start {
                sigbus_ranges.push((start, end));
            }
        }
        sigbus_ranges.sort_by_key(|(range_start, _)| *range_start);
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
        if start.checked_add(size).is_none() {
            return;
        }
        self.record_mmap_region_entry(super::UserMmapRegion {
            start,
            size,
            prot,
            shared,
            anonymous,
            locked,
            dont_fork: false,
            wipe_on_fork: false,
            grow_down,
            may_write,
            file_backing,
        });
    }

    pub(super) fn record_mmap_region_entry(&self, region: super::UserMmapRegion) {
        let start = region.start;
        let Some(end) = region.start.checked_add(region.size) else {
            return;
        };
        if end <= start {
            return;
        }
        self.forget_mmap_region(start, end);
        {
            let mut ranges = self.mmap_ranges.lock();
            ranges.push(region);
            ranges.sort_by_key(|region| region.start);
        }
    }

    pub(super) fn mmap_range_denies_write(&self, start: usize, end: usize) -> bool {
        if end <= start {
            return false;
        }
        let ranges = self.mmap_ranges.lock();
        overlapping_mmap_regions(&ranges, start, end).any(|region| {
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
        if end <= start {
            return false;
        }
        let ranges = self.mmap_ranges.lock();
        overlapping_mmap_regions(&ranges, start, end)
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

    pub(super) fn set_mmap_dont_fork_range(&self, start: usize, end: usize, enabled: bool) {
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
                selected.dont_fork = enabled;
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
        let mut ranges = self.shared_mmap_ranges.lock();
        let old = core::mem::take(&mut *ranges);
        for (range_start, size, flags) in old {
            let range_end = range_start.saturating_add(size);
            if range_end <= start || range_start >= end {
                ranges.push((range_start, size, flags));
                continue;
            }
            if range_start < start {
                ranges.push((range_start, start - range_start, flags));
            }
            if range_end > end {
                ranges.push((end, range_end - end, flags));
            }
        }
        ranges.sort_by_key(|(range_start, _, _)| *range_start);
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
        self.fork_with_fd_sharing(false, false, false, None, None)
    }

    pub(super) fn fork_with_fd_sharing(
        &self,
        share_fds: bool,
        share_vm: bool,
        share_writable_mappings: bool,
        child_stack: Option<usize>,
        child_pid: Option<i32>,
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
        let dont_fork_ranges: Vec<(usize, usize)> = if share_vm {
            Vec::new()
        } else {
            child_mmap_ranges
                .iter()
                .filter(|region| region.dont_fork)
                .map(|region| (region.start, region.end()))
                .collect()
        };
        if !dont_fork_ranges.is_empty() {
            {
                let mut aspace = child_aspace.lock();
                for (start, end) in &dont_fork_ranges {
                    aspace
                        .unmap(VirtAddr::from(*start), end.saturating_sub(*start))
                        .map_err(LinuxError::from)?;
                }
            }
            child_mmap_ranges.retain(|region| !region.dont_fork);
        }
        let wipe_on_fork_ranges: Vec<(usize, usize)> = child_mmap_ranges
            .iter()
            .filter(|region| region.wipe_on_fork)
            .map(|region| (region.start, region.end()))
            .collect();
        if !share_vm {
            zero_child_wipe_on_fork_ranges(&child_aspace, &wipe_on_fork_ranges)?;
        }
        let child_fds = if share_fds {
            self.fds.share_for_child_pid(
                self.pid(),
                child_pid.expect("CLONE_FILES process child must have an allocated pid"),
            )
        } else {
            Arc::new(ProcessFdTable::from_table(
                self.fds.fork_copy_for_pid(self.pid())?,
            ))
        };

        let child_shared_mmap_ranges: Vec<_> = self
            .shared_mmap_ranges()
            .into_iter()
            .flat_map(|(start, size, flags)| {
                let end = start.saturating_add(size);
                range_segments_excluding(start, end, &dont_fork_ranges)
                    .into_iter()
                    .map(move |(segment_start, segment_end)| {
                        (segment_start, segment_end - segment_start, flags)
                    })
            })
            .collect();
        let child_mmap_sigbus_ranges: Vec<_> = self
            .mmap_sigbus_ranges()
            .into_iter()
            .flat_map(|(start, end)| range_segments_excluding(start, end, &dont_fork_ranges))
            .collect();
        let cwd = self.cwd();
        let fs_root = self.fs_root();
        let exec_root = self.exec_root();
        let exec_path = self.exec_path();
        let prctl_name = self.prctl_name();
        let groups = self.groups();
        let timer_slack_ns = self.timer_slack_ns();
        let parent_task_id = axtask::current().id().as_u64() as i32;

        let child = Arc::new(UserProcess {
            aspace: child_aspace,
            owns_aspace: !share_vm,
            brk: Mutex::new(*self.brk.lock()),
            shared_mmap_ranges: Mutex::new(child_shared_mmap_ranges),
            mmap_sigbus_ranges: Mutex::new(child_mmap_sigbus_ranges),
            mmap_ranges: Mutex::new(child_mmap_ranges),
            exec_shared_mmap_cache: Mutex::new(Vec::new()),
            mlock_future: AtomicBool::new(false),
            mlockall_accounted_kb: AtomicUsize::new(0),
            fds: child_fds,
            cwd: Mutex::new(cwd),
            fs_root: Mutex::new(fs_root),
            exec_root: Mutex::new(exec_root),
            exec_path: Mutex::new(exec_path.clone()),
            hostname: self.hostname.clone(),
            domainname: self.domainname.clone(),
            prctl_name: Mutex::new(prctl_name),
            children: Mutex::new(Vec::new()),
            child_exit_wait: WaitQueue::new(),
            timer_wait: WaitQueue::new(),
            rlimits: Mutex::new(self.rlimits.lock().clone()),
            sched_state: Mutex::new(child_sched_state_from_parent(self.get_sched_state())),
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
            path_sparse_sizes: self.path_sparse_sizes,
            path_sparse_data: self.path_sparse_data,
            path_sparse_repeats: self.path_sparse_repeats,
            path_data_ranges: self.path_data_ranges,
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
            groups: Mutex::new(groups),
            credential_generation: AtomicUsize::new(self.credential_generation()),
            cap_effective: AtomicU64::new(self.cap_effective()),
            cap_permitted: AtomicU64::new(self.cap_permitted()),
            cap_inheritable: AtomicU64::new(self.cap_inheritable()),
            cap_bounding: AtomicU64::new(self.cap_bounding()),
            personality: AtomicUsize::new(self.personality()),
            parent_death_signal: AtomicI32::new(self.parent_death_signal.load(Ordering::Acquire)),
            default_timer_slack_ns: AtomicU64::new(timer_slack_ns),
            timer_slack_ns: AtomicU64::new(timer_slack_ns),
            posix_timers: Mutex::new(BTreeMap::new()),
            next_posix_timer_id: AtomicI32::new(1),
            real_timer_generation: AtomicU64::new(0),
            real_timer_deadline_us: AtomicU64::new(0),
            real_timer_interval_us: AtomicU64::new(0),
            virtual_timer_deadline_us: AtomicU64::new(0),
            virtual_timer_interval_us: AtomicU64::new(0),
            prof_timer_deadline_us: AtomicU64::new(0),
            prof_timer_interval_us: AtomicU64::new(0),
            syscall_runtime_micros: AtomicU64::new(0),
            last_reported_user_micros: AtomicU64::new(0),
            last_reported_system_micros: AtomicU64::new(0),
            completed_thread_runtime_ticks: AtomicU64::new(0),
            last_reported_user_ticks: AtomicU64::new(0),
            last_reported_system_ticks: AtomicU64::new(0),
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
            ppid: parent_task_id,
            live_threads: AtomicUsize::new(1),
            exit_group_code: AtomicI32::new(NO_EXIT_GROUP_CODE),
            exit_code: AtomicI32::new(0),
            term_signal: AtomicI32::new(0),
            wait_stopped_signal: AtomicI32::new(0),
            wait_continued_signal: AtomicI32::new(0),
            exit_wait: WaitQueue::new(),
            teardown: ProcessTeardown::new(),
        });
        #[cfg(feature = "auto-run-tests")]
        record_user_process_created(&child);
        track_running_executable(exec_path.as_str());
        Ok(child)
    }

    pub(super) fn add_child(&self, task: AxTaskRef, process: Arc<UserProcess>) -> i32 {
        let pid = task.id().as_u64() as i32;
        let mut children = self.children.lock();
        if let Some(code) = self.child_creation_interrupted() {
            drop(children);
            process.request_exit_group(code);
            return pid;
        }
        children.push(ChildTask { pid, task, process });
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
            if self.wait_interrupt_pending() {
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
                    || current_unblocked_signal_pending()
                    || self.pending_exit_group().is_some()
                    || self.eval_watchdog_expired()
            };
            if let Some(timeout) = self.blocking_wait_remaining() {
                self.child_wait_blocked.store(true, Ordering::Release);
                if self
                    .child_exit_wait
                    .wait_timeout_until(timeout, wait_condition)
                {
                    self.child_wait_blocked.store(false, Ordering::Release);
                    if self.wait_interrupt_pending() {
                        return Err(LinuxError::EINTR);
                    }
                    continue;
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
    ) -> Result<Option<(i32, i32, general::rusage)>, LinuxError> {
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
            if self.wait_interrupt_pending() {
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
                    || current_unblocked_signal_pending()
                    || self.pending_exit_group().is_some()
                    || self.eval_watchdog_expired()
            };
            if let Some(timeout) = self.blocking_wait_remaining() {
                self.child_wait_blocked.store(true, Ordering::Release);
                if self
                    .child_exit_wait
                    .wait_timeout_until(timeout, wait_condition)
                {
                    self.child_wait_blocked.store(false, Ordering::Release);
                    if self.wait_interrupt_pending() {
                        return Err(LinuxError::EINTR);
                    }
                    continue;
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
        let child_rusage = rusage_from_child_usage(child_usage, child_maxrss);
        self.record_waited_child_maxrss(child_maxrss);
        let _ = child.task.join();
        child.process.teardown();
        drop(child);
        yield_for_task_gc();
        Ok(Some((child_pid, status, child_rusage)))
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

fn empty_rusage() -> general::rusage {
    unsafe { core::mem::zeroed() }
}

fn rusage_timeval_from_user_ticks(ticks: i64) -> general::__kernel_old_timeval {
    let ticks = ticks.max(0) as i128;
    let user_hz = super::time_abi::USER_HZ as i128;
    general::__kernel_old_timeval {
        tv_sec: (ticks / user_hz) as _,
        tv_usec: ((ticks % user_hz) * 1_000_000 / user_hz) as _,
    }
}

fn rusage_from_child_usage(usage: super::time_abi::Tms, maxrss_kb: usize) -> general::rusage {
    let mut rusage = empty_rusage();
    rusage.ru_utime =
        rusage_timeval_from_user_ticks(usage.tms_utime.saturating_add(usage.tms_cutime));
    rusage.ru_stime =
        rusage_timeval_from_user_ticks(usage.tms_stime.saturating_add(usage.tms_cstime));
    rusage.ru_maxrss = maxrss_kb as _;
    rusage
}

#[cfg(feature = "auto-run-tests")]
impl Drop for UserProcess {
    fn drop(&mut self) {
        USER_PROCESS_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        USER_PROCESS_DROPPED.fetch_add(1, Ordering::AcqRel);
    }
}

fn notify_parent_child_exit(ppid: i32, child_pid: i32) {
    if let Some(parent) = user_thread_entry_by_process_pid(ppid) {
        if parent.process.sigchld_discards_wait_status()
            && parent.process.reap_ignored_child(child_pid)
        {
            return;
        }
        if parent.process.sigchld_has_user_handler()
            || thread_waits_for_signal(&parent, SIGCHLD_NUM as i32)
        {
            let _ = deliver_user_signal(&parent, SIGCHLD_NUM as i32, child_pid);
        }
        // A child-status transition makes wait4()/waitid() predicates true.
        // Request rescheduling so the woken parent can reap promptly even when
        // the exiting/signalling child is part of a fork-heavy workload.
        parent.process.child_exit_wait.notify_all(true);
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

fn validate_execve_candidate(process: &UserProcess, path: &str) -> Result<(), LinuxError> {
    if exec_path_exceeds_linux_limits(path) {
        return Err(LinuxError::ENAMETOOLONG);
    }
    let resolved = process
        .resolve_path_symlink(path)?
        .unwrap_or_else(|| path.to_string());
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

fn validate_execve_target(process: &UserProcess, cwd: &str, path: &str) -> Result<(), LinuxError> {
    if path.is_empty() {
        return Err(LinuxError::ENOENT);
    }
    if exec_path_exceeds_linux_limits(path) {
        return Err(LinuxError::ENAMETOOLONG);
    }

    let normalized = try_normalize_path(cwd, path)
        .map_err(|_| LinuxError::ENOMEM)?
        .ok_or(LinuxError::EINVAL)?;
    if exec_path_exceeds_linux_limits(normalized.as_str()) {
        return Err(LinuxError::ENAMETOOLONG);
    }
    let mut candidates = Vec::new();
    candidates
        .try_reserve_exact(1)
        .map_err(|_| LinuxError::ENOMEM)?;
    let mut primary = String::new();
    primary
        .try_reserve_exact(normalized.len())
        .map_err(|_| LinuxError::ENOMEM)?;
    primary.push_str(normalized.as_str());
    candidates.push(primary);
    if path.starts_with('/') {
        for candidate in try_staged_cwd_absolute_path_candidates(cwd, normalized.as_str())
            .map_err(|_| LinuxError::ENOMEM)?
        {
            if !candidates.iter().any(|item| item == &candidate) {
                candidates
                    .try_reserve_exact(1)
                    .map_err(|_| LinuxError::ENOMEM)?;
                candidates.push(candidate);
            }
        }
        for candidate in
            try_runtime_absolute_path_candidates(process.exec_root().as_str(), normalized.as_str())
                .map_err(|_| LinuxError::ENOMEM)?
        {
            if !candidates.iter().any(|item| item == &candidate) {
                candidates
                    .try_reserve_exact(1)
                    .map_err(|_| LinuxError::ENOMEM)?;
                candidates.push(candidate);
            }
        }
    }
    let mut missing_error = LinuxError::ENOENT;
    for candidate in candidates {
        match validate_execve_candidate(process, candidate.as_str()) {
            Ok(()) => return Ok(()),
            Err(LinuxError::ENOENT) => {
                if missing_error != LinuxError::ENOTDIR {
                    missing_error = LinuxError::ENOENT;
                }
            }
            Err(LinuxError::ENOTDIR) => missing_error = LinuxError::ENOTDIR,
            Err(err) => return Err(err),
        }
    }
    Err(missing_error)
}

fn contains_ascii_case_insensitive(haystack: &str, needle: &str) -> bool {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|window| {
        window
            .iter()
            .zip(needle.iter())
            .all(|(&actual, &expected)| ascii_lower(actual) == ascii_lower(expected))
    })
}

fn ascii_lower(byte: u8) -> u8 {
    if byte.is_ascii_uppercase() {
        byte + (b'a' - b'A')
    } else {
        byte
    }
}

fn exec_loader_errno_from_message(err: &str) -> LinuxError {
    if err.starts_with(EXEC_LOADER_ENOMEM_PREFIX) {
        LinuxError::ENOMEM
    } else if contains_ascii_case_insensitive(err, "not a directory") {
        LinuxError::ENOTDIR
    } else if contains_ascii_case_insensitive(err, "entity not found")
        || contains_ascii_case_insensitive(err, "not found")
    {
        LinuxError::ENOENT
    } else {
        LinuxError::ENOEXEC
    }
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
    let argv = match read_execve_argv(process, argv, raw_path.as_str()) {
        Ok(argv) => argv,
        Err(err) => return neg_errno(err),
    };
    let env = match read_execve_envp(process, _envp) {
        Ok(env) => env,
        Err(err) => return neg_errno(err),
    };
    let cwd = process.cwd();
    if let Err(err) = validate_execve_target(process, cwd.as_str(), raw_path.as_str()) {
        return neg_errno(err);
    }
    if !process.owns_aspace {
        return neg_errno(LinuxError::EAGAIN);
    }
    let (entry, stack_ptr, argc) =
        match exec_program(process, cwd.as_str(), raw_path.as_str(), &argv, &env) {
            Ok(image) => image,
            Err(ExecProgramError::Loader(err)) => {
                return neg_errno(exec_loader_errno_from_message(err.as_str()));
            }
            Err(ExecProgramError::FdTable(err)) => return neg_errno(err),
        };
    process.reset_caught_signal_handlers_for_exec();
    let context = make_uspace_context(entry, stack_ptr, argc);
    let closed_cloexec = { process.fds.lock().close_cloexec() };
    drop(closed_cloexec);
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
        if process.child_creation_interrupted().is_some() {
            return neg_errno(LinuxError::EINTR);
        }
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
        let mut task = TaskInner::new(user_task_entry, "user:fork".into(), USER_TASK_KSTACK_SIZE);
        let pid = task.id().as_u64() as i32;
        let child_process = match process.fork_with_fd_sharing(
            share_fds,
            share_vm,
            share_writable_mappings,
            (!share_vm).then_some(child_stack),
            Some(pid),
        ) {
            Ok(process) => process,
            Err(err) => return neg_errno(err),
        };
        // CLONE_FILES membership was registered with this pid while creating
        // the child.  Publish it before any rollback path can call teardown.
        child_process.set_pid(pid);
        if process.child_creation_interrupted().is_some() {
            child_process.request_exit_group(137);
            child_process.teardown();
            return neg_errno(LinuxError::EINTR);
        }
        let mut child_tf = child_trap_frame(tf, child_stack);
        if clone_flags & general::CLONE_SETTLS as usize != 0 {
            child_tf.regs.tp = tls;
        }
        let child_context = UspaceContext::from(&child_tf);
        if clone_flags & general::CLONE_PARENT_SETTID as usize != 0 {
            let ret = write_user_value(process.as_ref(), ptid, &pid);
            if ret != 0 {
                child_process.teardown();
                return ret;
            }
        }
        if clone_flags & general::CLONE_CHILD_SETTID as usize != 0 {
            let ret = write_user_value(child_process.as_ref(), ctid, &pid);
            if ret != 0 {
                child_process.teardown();
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
            child_context,
            child_clear_tid,
            child_signal_mask,
        ));
        let task = axtask::spawn_task(task);
        apply_process_scheduler_state_to_task(child_process.as_ref(), &task);
        register_user_task(task.clone(), child_process.clone());
        let parent_interrupted = process.child_creation_interrupted().is_some();
        process.add_child(task, child_process.clone());
        if parent_interrupted {
            return neg_errno(LinuxError::EINTR);
        }
        if vfork_requested {
            let wait_condition = || {
                child_process.live_threads.load(Ordering::Acquire) == 0
                    || child_process.vfork_exec_done.load(Ordering::Acquire)
                    // A vfork parent may resume once the child has committed to
                    // process exit.  The child can still be finishing kernel-side
                    // teardown before live_threads reaches zero; waiting for that
                    // cleanup here turns short child _exit() paths into watchdog
                    // timeouts even though no user code can run in the child.
                    || child_process.pending_exit_group().is_some()
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
        // A fork storm should not let the forking parent monopolize a single
        // CPU until it has created every child. Linux may schedule the child or
        // another ready task immediately after fork returns; yield here after
        // all parent-visible state is installed so woken parents/sleepers can
        // run promptly under process-heavy workloads.
        axtask::yield_now();
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

    if process.child_creation_interrupted().is_some() {
        return neg_errno(LinuxError::EINTR);
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
    let mut task = TaskInner::new(user_task_entry, "user:thread".into(), USER_TASK_KSTACK_SIZE);
    let tid = task.id().as_u64() as i32;
    let root = process.aspace.lock().page_table_root();
    task.ctx_mut().set_page_table_root(root);
    task.init_task_ext(UserTaskExt::new(
        process.clone(),
        child_context,
        child_clear_tid,
        inherited_signal_mask,
    ));

    if clone_flags & general::CLONE_PARENT_SETTID as usize != 0 {
        let ret = write_user_value(process.as_ref(), ptid, &tid);
        if ret != 0 {
            return ret;
        }
    }
    if child_set_tid != 0 {
        let ret = write_user_value(process.as_ref(), child_set_tid, &tid);
        if ret != 0 {
            return ret;
        }
    }
    if process.child_creation_interrupted().is_some() {
        return neg_errno(LinuxError::EINTR);
    }
    process.add_thread();
    let spawned = axtask::spawn_task(task);
    apply_process_scheduler_state_to_task(process.as_ref(), &spawned);
    register_user_task(spawned, process.clone());
    // Match the fork path's fairness point for pthread/CLONE_THREAD storms.
    // Linux may run a newly-created sibling before the creator continues; on a
    // single-vCPU RR system this prevents one thread creator from monopolizing
    // CPU while hundreds of runnable peers are being made ready.
    axtask::yield_now();
    tid as isize
}

pub(super) fn sys_wait4(
    process: &UserProcess,
    pid: i32,
    status: usize,
    options: usize,
    rusage: usize,
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
    let Some((child_pid, exit_code, child_rusage)) = (match process.wait_child(pid, nohang) {
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
    if rusage != 0 {
        let ret = write_user_value(process, rusage, &child_rusage);
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
    rusage: usize,
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
        if rusage != 0 {
            let usage = empty_rusage();
            let ret = write_user_value(process, rusage, &usage);
            if ret != 0 {
                return ret;
            }
        }
        return 0;
    }

    let wait_result = match process.wait_child(pid_filter, nohang) {
        Ok(result) => result,
        Err(err) => return neg_errno(err),
    };

    let (info, child_rusage) = if let Some((child_pid, status, usage)) = wait_result {
        (waitid_siginfo(child_pid, status), usage)
    } else {
        ([0u8; 128], empty_rusage())
    };
    if infop != 0 {
        if let Err(err) = write_user_bytes(process, infop, &info) {
            return neg_errno(err);
        }
    }
    if rusage != 0 {
        let ret = write_user_value(process, rusage, &child_rusage);
        if ret != 0 {
            return ret;
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
    unregister_user_task_with_runtime(
        current_tid(),
        process,
        axtask::current().cpu_runtime_ticks(),
    );
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
