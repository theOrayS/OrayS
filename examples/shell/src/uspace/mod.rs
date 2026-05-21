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

struct UserProcess {
    aspace: Mutex<AddrSpace>,
    brk: Mutex<BrkState>,
    shared_mmap_ranges: Mutex<Vec<(usize, usize, MappingFlags)>>,
    fds: Mutex<FdTable>,
    cwd: Mutex<String>,
    exec_root: Mutex<String>,
    exec_path: Mutex<String>,
    children: Mutex<Vec<ChildTask>>,
    child_exit_wait: WaitQueue,
    rlimits: Mutex<BTreeMap<u32, UserRlimit>>,
    sched_state: Mutex<UserSchedState>,
    signal_actions: Mutex<BTreeMap<usize, general::kernel_sigaction>>,
    path_modes: Mutex<BTreeMap<String, u32>>,
    path_owners: Mutex<BTreeMap<String, (u32, u32)>>,
    path_symlinks: Mutex<BTreeMap<String, String>>,
    umask: AtomicU32,
    mount_points: Arc<Mutex<BTreeMap<String, String>>>,
    shm_attachments: Mutex<BTreeMap<usize, (i32, usize)>>,
    real_uid: AtomicU32,
    uid: AtomicU32,
    saved_uid: AtomicU32,
    real_gid: AtomicU32,
    gid: AtomicU32,
    saved_gid: AtomicU32,
    groups: Mutex<Vec<u32>>,
    credential_generation: AtomicUsize,
    personality: AtomicUsize,
    real_timer_generation: AtomicU64,
    real_timer_deadline_us: AtomicU64,
    real_timer_interval_us: AtomicU64,
    eval_watchdog_deadline_us: AtomicU64,
    child_wait_blocked: AtomicBool,
    pid: AtomicI32,
    pgid: AtomicI32,
    ppid: i32,
    live_threads: AtomicUsize,
    exit_group_code: AtomicI32,
    exit_code: AtomicI32,
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
