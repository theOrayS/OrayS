use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize};

use axhal::paging::MappingFlags;
use axmm::AddrSpace;
use axns::AxNamespace;
use axsync::Mutex;
use axtask::{AxTaskRef, WaitQueue};
use linux_raw_sys::general;
use std::collections::BTreeMap;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

macro_rules! user_trace {
    ($($arg:tt)*) => {};
}

mod credentials;
mod fd_pipe;
mod fd_socket;
mod fd_table;
mod futex;
mod linux_abi;
mod memory_map;
mod memory_policy;
mod metadata;
mod mount_abi;
mod process_abi;
mod process_lifecycle;
mod program_loader;
mod resource_sched;
mod runtime_paths;
mod select_fdset;
mod signal_abi;
mod synthetic_fs;
mod syscall_dispatch;
mod system_info;
mod sysv_shm;
mod task_context;
mod task_registry;
mod time_abi;
mod user_memory;

use fd_table::FdTable;
use linux_abi::*;
use process_lifecycle::ProcessTeardown;
#[cfg(feature = "auto-run-tests")]
pub use process_lifecycle::cleanup_user_processes;
pub use process_lifecycle::run_user_program;
#[cfg(feature = "auto-run-tests")]
pub use process_lifecycle::run_user_program_in;
#[cfg(feature = "auto-run-tests")]
pub use process_lifecycle::run_user_program_in_timeout;
use resource_sched::{UserRlimit, UserSchedState};
use select_fdset::SelectMode;

struct AxNamespaceImpl;

const DEFAULT_TIMER_SLACK_NS: u64 = 50_000;

#[derive(Clone)]
struct MountPoint {
    source_root: String,
    readonly: bool,
}

#[derive(Clone, Copy)]
struct PathTimes {
    atime: general::timespec,
    mtime: general::timespec,
    ctime: general::timespec,
}

struct UserProcess {
    aspace: Mutex<AddrSpace>,
    brk: Mutex<BrkState>,
    shared_mmap_ranges: Mutex<Vec<(usize, usize, MappingFlags)>>,
    mmap_sigbus_ranges: Mutex<Vec<(usize, usize)>>,
    mmap_ranges: Mutex<Vec<UserMmapRegion>>,
    fds: Mutex<FdTable>,
    cwd: Mutex<String>,
    exec_root: Mutex<String>,
    exec_path: Mutex<String>,
    hostname: Mutex<String>,
    prctl_name: Mutex<String>,
    children: Mutex<Vec<ChildTask>>,
    child_exit_wait: WaitQueue,
    rlimits: Mutex<BTreeMap<u32, UserRlimit>>,
    sched_state: Mutex<UserSchedState>,
    nice: AtomicI32,
    signal_actions: Mutex<BTreeMap<usize, general::kernel_sigaction>>,
    path_modes: Mutex<BTreeMap<String, u32>>,
    path_inodes: Mutex<BTreeMap<String, u64>>,
    path_special_modes: Mutex<BTreeMap<String, u32>>,
    path_rdevs: Mutex<BTreeMap<String, u64>>,
    path_owners: Mutex<BTreeMap<String, (u32, u32)>>,
    path_symlinks: Mutex<BTreeMap<String, String>>,
    path_hardlinks: Mutex<BTreeMap<String, String>>,
    path_hardlink_counts: Mutex<BTreeMap<String, u64>>,
    path_xattrs: Mutex<BTreeMap<String, BTreeMap<String, Vec<u8>>>>,
    path_times: Mutex<BTreeMap<String, PathTimes>>,
    path_sparse_sizes: Mutex<BTreeMap<String, u64>>,
    path_sparse_data: Mutex<BTreeMap<String, Vec<(u64, Vec<u8>)>>>,
    umask: AtomicU32,
    mount_points: Arc<Mutex<BTreeMap<String, MountPoint>>>,
    shm_attachments: Mutex<BTreeMap<usize, (i32, usize)>>,
    real_uid: AtomicU32,
    uid: AtomicU32,
    saved_uid: AtomicU32,
    fs_uid: AtomicU32,
    real_gid: AtomicU32,
    gid: AtomicU32,
    saved_gid: AtomicU32,
    fs_gid: AtomicU32,
    groups: Mutex<Vec<u32>>,
    credential_generation: AtomicUsize,
    personality: AtomicUsize,
    parent_death_signal: AtomicI32,
    default_timer_slack_ns: AtomicU64,
    timer_slack_ns: AtomicU64,
    real_timer_generation: AtomicU64,
    real_timer_deadline_us: AtomicU64,
    real_timer_interval_us: AtomicU64,
    virtual_timer_deadline_us: AtomicU64,
    virtual_timer_interval_us: AtomicU64,
    prof_timer_deadline_us: AtomicU64,
    prof_timer_interval_us: AtomicU64,
    start_clock_ticks: AtomicU64,
    waited_child_user_ticks: AtomicU64,
    waited_child_system_ticks: AtomicU64,
    eval_watchdog_deadline_us: AtomicU64,
    child_wait_blocked: AtomicBool,
    syscall_wait_blocked: AtomicBool,
    pid: AtomicI32,
    pgid: AtomicI32,
    sid: AtomicI32,
    ppid: i32,
    live_threads: AtomicUsize,
    exit_group_code: AtomicI32,
    exit_code: AtomicI32,
    term_signal: AtomicI32,
    wait_stopped_signal: AtomicI32,
    wait_continued_signal: AtomicI32,
    exit_wait: WaitQueue,
    teardown: ProcessTeardown,
}

#[derive(Clone, Copy)]
struct BrkState {
    start: usize,
    end: usize,
    limit: usize,
    next_mmap: usize,
}

#[derive(Clone, Copy)]
struct UserMmapRegion {
    start: usize,
    size: usize,
    prot: u32,
    shared: bool,
    locked: bool,
}

impl UserMmapRegion {
    fn end(&self) -> usize {
        self.start.saturating_add(self.size)
    }
}

struct ChildTask {
    pid: i32,
    task: AxTaskRef,
    process: Arc<UserProcess>,
}

const NO_EXIT_GROUP_CODE: i32 = i32::MIN;

#[crate_interface::impl_interface]
impl axns::AxNamespaceIf for AxNamespaceImpl {
    fn current_namespace_base() -> *mut u8 {
        AxNamespace::global().base()
    }
}
