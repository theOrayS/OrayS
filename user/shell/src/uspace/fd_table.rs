use core::cmp;
use core::mem::{offset_of, size_of};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};
use core::time::Duration;

use axdriver::prelude::{BaseDriverOps, BlockDriverOps, DevError, DevResult, DeviceType};
use axerrno::LinuxError;
use axfile::{
    DescriptionIdentity, EventDeliveryState, EventObserver, EventSource, EventSubscription,
    FileTableDetach, FileTableGroup, FileTableShareTracker, LevelWaitDecision, OpenFile,
    OpenFileId, ReadyEvents, ReentrancyGate, RegistrationKey, decide_level_wait,
    readiness_deadline_delay,
};
use axfs::fops::{self, Directory, File, FileAttr, OpenOptions};
use axio::SeekFrom;
use axsync::{Mutex as AxMutex, MutexGuard as AxMutexGuard};
use axtask::WaitQueue;
use lazyinit::LazyInit;
use linux_raw_sys::{general, ioctl};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::sync::{Arc, Mutex, Weak};
use std::vec::Vec;

use super::credentials::access_allowed;
use super::fd_object::{FileObject, OpenFileRef, new_open_file, object_as};
use super::fd_pipe::PipeEndpoint;
use super::fd_socket::{LocalSocketEntry, SocketEntry, recv_socket_data_to_user, socket_entry};
use super::linux_abi::{
    ACCESS_R_OK, ACCESS_W_OK, ACCESS_X_OK, CLOSE_RANGE_CLOEXEC, CLOSE_RANGE_UNSHARE,
    FILE_MODE_SET_GID, FILE_MODE_STICKY, MAX_IN_MEMORY_FILE_SIZE, NR_OPEN_LIMIT, O_NOFOLLOW_FLAG,
    O_PATH_FLAG, RLIMIT_FSIZE_RESOURCE, RLIMIT_NOFILE_RESOURCE, RTC_RD_TIME, SEEK_DATA_WHENCE,
    SEEK_HOLE_WHENCE, ST_MODE_BLK, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FIFO, ST_MODE_FILE,
    ST_MODE_LNK, ST_MODE_SOCKET, ST_MODE_TYPE_MASK, SYNTHETIC_BLOCK_DEVICE_SIZE, fd_cloexec_flag,
    neg_errno, posix_ret_i32,
};
use super::memory_map::align_up;
use super::metadata::{
    DEV_CPU_DMA_LATENCY_RDEV, DEV_NULL_RDEV, DEV_ZERO_RDEV, ST_NOSYMFOLLOW_FLAG,
    apply_recorded_path_metadata, canonical_permission_path, dev_cpu_dma_latency_stat,
    dev_null_stat, dev_zero_stat, dirent_type, fd_entry_path, fd_entry_statfs_path,
    file_attr_to_stat, file_type_mode, generic_statfs, path_inode, stdio_stat,
    synthetic_block_stat_for_path, synthetic_char_stat_for_path,
};
use super::perf_counters;
use super::posix_mq::{
    PosixMqDescriptor, ProcMqQueuesMaxEntry, proc_sys_fs_mqueue_fd_entry,
    proc_sys_fs_mqueue_path_entry,
};
use super::program_loader::invalidate_exec_image_cache;
use super::runtime_paths::{
    normalize_path, push_runtime_candidate, runtime_absolute_path_candidates,
    runtime_library_name_candidates,
};
use super::select_fdset::{SelectMode, yield_poll_wait};
use super::signal_abi::{
    current_pending_signal_matches, current_unblocked_signal_pending,
    install_temporary_signal_mask, take_current_pending_signal_matching,
};
use super::synthetic_fs::{
    ProcSysFileEntry, dev_shm_host_path, ensure_dev_shm_dir, is_proc_self_maps_path,
    proc_comm_fd_entry, proc_comm_path_entry, proc_exe_link_target, proc_meminfo_fd_entry,
    proc_meminfo_path_entry, proc_pagemap_fd_entry, proc_pagemap_path_entry,
    proc_pid_stat_fd_entry, proc_pid_stat_path_entry, proc_pid_status_fd_entry,
    proc_pid_status_path_entry, proc_self_maps_fd_entry, proc_self_maps_is_writable_open,
    proc_self_maps_path_entry, proc_smaps_fd_entry, proc_smaps_path_entry, proc_sys_file_fd_entry,
    proc_sys_file_path_entry, proc_sysvipc_msg_fd_entry, proc_sysvipc_msg_path_entry,
    proc_sysvipc_sem_fd_entry, proc_sysvipc_sem_path_entry, proc_sysvipc_shm_fd_entry,
    proc_sysvipc_shm_path_entry, proc_task_dir_fd_entry, proc_task_dir_path_entry,
    proc_timerslack_fd_entry, proc_timerslack_path_entry, synthetic_file_is_writable_open,
    synthetic_kernel_config_content, synthetic_kernel_config_fd_entry,
    synthetic_kernel_config_path_entry, synthetic_proc_sys_content, synthetic_proc_sys_fd_entry,
    synthetic_proc_sys_path_entry, synthetic_proc_version_content, synthetic_proc_version_fd_entry,
    synthetic_proc_version_path_entry, synthetic_userdb_content, synthetic_userdb_fd_entry,
    synthetic_userdb_path_entry,
};
use super::system_info::write_default_winsize;
use super::task_context::current_task_ext;
use super::task_registry::{
    UserThreadEntry, user_thread_entry_by_process_pid, user_thread_entry_for_process,
};
use super::time_abi::{
    clock_gettime_timespec, clock_now_duration, rtc_time_from_wall_time, timespec_to_duration,
};
use super::user_memory::{
    MAX_USER_IO_CHUNK, fill_pseudo_random_bytes, read_cstr, read_iovec_entries, read_user_bytes,
    read_user_bytes_into, read_user_value, validate_user_read, validate_user_write,
    with_readable_user_buffer, with_writable_user_buffer, write_user_bytes, write_user_value,
};
use super::{PathTimes, UserProcess};

pub(super) struct FdTable {
    slots: Vec<Option<FdSlot>>,
}

struct FdSlot {
    entry: FdEntry,
    fd_flags: u32,
    description: Arc<DescriptionIdentity>,
}

impl FdSlot {
    fn new(entry: FdEntry, fd_flags: u32) -> Result<Self, LinuxError> {
        let id = OpenFileId::allocate().map_err(|_| LinuxError::ENFILE)?;
        Ok(Self::with_identity(
            entry,
            fd_flags,
            Arc::new(DescriptionIdentity::new(id)),
        ))
    }

    fn with_identity(entry: FdEntry, fd_flags: u32, description: Arc<DescriptionIdentity>) -> Self {
        Self {
            entry,
            fd_flags: fd_flags & general::FD_CLOEXEC,
            description,
        }
    }

    fn description_id(&self) -> OpenFileId {
        self.description.id()
    }
}

pub(super) struct ClosedFd {
    _entry: FdEntry,
    _description: Arc<DescriptionIdentity>,
}

fn discard_uninstalled_entry(entry: FdEntry) {
    // SocketEntry wraps a raw POSIX descriptor and intentionally has no Drop
    // implementation: installed slots are closed by the FdTable close path.
    // A prepared duplicate that never reaches a slot therefore needs explicit
    // rollback, while all RAII-backed variants only need to be dropped here.
    if let FdEntry::Socket(socket) = &entry {
        let _ = socket.close();
    }
    drop(entry);
}

fn pipe_endpoint(description: &OpenFileRef) -> Result<&PipeEndpoint, LinuxError> {
    object_as(description)
}

pub(super) struct ProcessFdTable {
    state: AxMutex<ProcessFdTableState>,
}

struct ProcessFdTableState {
    base: FdTable,
    sharing: FileTableShareTracker,
    unshared: BTreeMap<i32, FdTable>,
}

pub(super) struct ProcessFdTableGuard<'a> {
    state: AxMutexGuard<'a, ProcessFdTableState>,
    pid: i32,
}

const FD_TABLE_LIMIT: usize = NR_OPEN_LIMIT as usize;
const LINUX_PATH_MAX: usize = 4096;
// axfs_vfs::VfsDirEntry stores 63 bytes of d_name.  Enforce and report that
// real backing limit at the POSIX boundary instead of accepting longer names
// that would later panic during directory enumeration.
const LINUX_NAME_MAX: usize = 63;
const MEMFD_NAME_MAX: usize = 249;
const LINUX_EPOLL_MAX_NEST_DEPTH: usize = 5;

fn epoll_graph_lock() -> &'static Mutex<()> {
    static EPOLL_GRAPH_LOCK: LazyInit<Mutex<()>> = LazyInit::new();
    let _ = EPOLL_GRAPH_LOCK.call_once(|| Mutex::new(()));
    &EPOLL_GRAPH_LOCK
}
// Keep the ramfs/VFS backing for regular files bounded and represent larger
// tmp/scratch extents with the per-process sparse-file overlay below.  The
// ramfs node stores a file in one Vec, so long evaluator runs can fragment the
// byte heap enough that growing even a 1 MiB contiguous backing allocation
// panics.  Bound the physical prefix to one userspace I/O chunk and store the
// rest as chunked sparse extents; reads/mmap merge both sources.
const MAX_PHYSICAL_FILE_BACKING_SIZE: u64 = MAX_USER_IO_CHUNK as u64;
// Linux regular-file offsets are signed.  Keep fallocate from accepting ranges
// that wrap past the largest representable file position even when RLIMIT_FSIZE
// is unlimited.
const LINUX_MAX_FILE_OFFSET: u64 = i64::MAX as u64;
const FALLOC_FL_KEEP_SIZE: usize = 0x01;
const FALLOC_FL_PUNCH_HOLE: usize = 0x02;
const FALLOC_FL_COLLAPSE_RANGE: usize = 0x08;
const FALLOC_FL_ZERO_RANGE: usize = 0x10;
const FALLOC_FL_INSERT_RANGE: usize = 0x20;
const POSIX_FADV_MIN: i32 = 0;
const POSIX_FADV_MAX: i32 = 5;
const SYNTHETIC_CHAR_DEVICE_NAMES: &[&str] = &["cpu_dma_latency"];
const SYNTHETIC_BLOCK_DEVICE_NAMES: &[&str] = &["vda"];

fn current_fd_table_limit() -> usize {
    let Some(task) = current_task_ext() else {
        return FD_TABLE_LIMIT;
    };
    let soft_limit = task.process.get_rlimit(RLIMIT_NOFILE_RESOURCE).current();
    cmp::min(
        FD_TABLE_LIMIT,
        soft_limit.min(FD_TABLE_LIMIT as u64) as usize,
    )
}

fn io_scratch_slice<'a>(scratch: &'a mut Vec<u8>, len: usize) -> Result<&'a mut [u8], LinuxError> {
    if scratch.len() < len {
        scratch
            .try_reserve_exact(len - scratch.len())
            .map_err(|_| LinuxError::ENOMEM)?;
        scratch.resize(len, 0);
    }
    Ok(&mut scratch[..len])
}

pub(super) enum FdEntry {
    Stdin(u32),
    Stdout(u32),
    Stderr(u32),
    DevNull,
    DevZero(u32),
    DevRandom(u32),
    DevCpuDmaLatency(u32),
    BlockDevice(BlockDeviceEntry),
    Rtc,
    File(FileEntry),
    Directory(DirectoryEntry),
    ProcFdDir(ProcFdDirEntry),
    SyntheticDir(SyntheticDirEntry),
    Path(PathEntry),
    MemoryFile(MemoryFileEntry),
    Memfd(MemfdEntry),
    ProcPagemap(ProcPagemapEntry),
    ProcTimerSlack(ProcTimerSlackEntry),
    Pipe(OpenFileRef),
    Socket(SocketEntry),
    LocalSocket(LocalSocketEntry),
    EventFd(OpenFileRef),
    Inotify(InotifyEntry),
    Epoll(EpollEntry),
    TimerFd(OpenFileRef),
    SignalFd(SignalFdEntry),
    PidFd(PidFdEntry),
    PosixMq(PosixMqDescriptor),
    ProcMqQueuesMax(ProcMqQueuesMaxEntry),
    ProcSysFile(ProcSysFileEntry),
}

const STDIN_STATUS_FLAGS: u32 = general::O_RDONLY;
const STDOUT_STATUS_FLAGS: u32 = general::O_WRONLY;
const STDERR_STATUS_FLAGS: u32 = general::O_WRONLY;

#[derive(Clone)]
pub(super) struct FileEntry {
    pub(super) file: File,
    pub(super) path: String,
    pub(super) status_flags: u32,
    offset: Arc<Mutex<u64>>,
}

#[derive(Clone)]
pub(super) struct DirectoryEntry {
    pub(super) dir: Directory,
    pub(super) attr: FileAttr,
    pub(super) path: String,
    next_dirent_cookie: u64,
    next_synthetic_dirent_index: usize,
    synthetic_dirents_emitted: bool,
}

#[derive(Clone)]
pub(super) struct ProcFdDirEntry {
    pub(super) path: String,
    next_dirent_cookie: u64,
}

#[derive(Clone)]
pub(super) struct SyntheticDirEntry {
    pub(super) path: String,
    parent_path: String,
    dirents: Vec<SyntheticDirent>,
    next_dirent_cookie: u64,
}

#[derive(Clone)]
pub(super) struct SyntheticDirent {
    name: String,
    d_type: u8,
    path: String,
}

#[derive(Clone)]
pub(super) struct BlockDeviceEntry {
    pub(super) path: String,
    status_flags: u32,
    offset: Arc<Mutex<u64>>,
    storage: Arc<Mutex<Vec<u8>>>,
}

struct SyntheticBlockDriver {
    name: String,
    storage: Arc<Mutex<Vec<u8>>>,
}

fn checked_seek_offset(base: u64, delta: i64) -> Result<u64, LinuxError> {
    base.checked_add_signed(delta).ok_or(LinuxError::EINVAL)
}

pub(super) fn synthetic_block_device_storage(path: &str) -> Arc<Mutex<Vec<u8>>> {
    static DEVICES: LazyInit<Mutex<BTreeMap<String, Arc<Mutex<Vec<u8>>>>>> = LazyInit::new();
    let _ = DEVICES.call_once(|| Mutex::new(BTreeMap::new()));
    let key = normalize_path("/", path).unwrap_or_else(|| path.to_string());
    let mut devices = DEVICES.lock();
    devices
        .entry(key)
        .or_insert_with(|| Arc::new(Mutex::new(Vec::new())))
        .clone()
}

pub(super) fn synthetic_block_device_for_mount(path: &str) -> Option<axdriver::AxBlockDevice> {
    if !is_synthetic_block_device_path(path) {
        return None;
    }
    Some(Box::new(SyntheticBlockDriver {
        name: normalize_path("/", path).unwrap_or_else(|| path.to_string()),
        storage: synthetic_block_device_storage(path),
    }))
}

pub(super) fn synthetic_block_device_is_uninitialized(path: &str) -> bool {
    is_synthetic_block_device_path(path) && synthetic_block_device_storage(path).lock().is_empty()
}

impl BaseDriverOps for SyntheticBlockDriver {
    fn device_name(&self) -> &str {
        self.name.as_str()
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }
}

impl BlockDriverOps for SyntheticBlockDriver {
    fn num_blocks(&self) -> u64 {
        SYNTHETIC_BLOCK_DEVICE_SIZE / 512
    }

    fn block_size(&self) -> usize {
        512
    }

    fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> DevResult {
        let start = checked_block_offset(block_id, buf.len())?;
        buf.fill(0);
        let storage = self.storage.lock();
        if start < storage.len() {
            let copied = cmp::min(buf.len(), storage.len() - start);
            buf[..copied].copy_from_slice(&storage[start..start + copied]);
        }
        Ok(())
    }

    fn write_block(&mut self, block_id: u64, buf: &[u8]) -> DevResult {
        let start = checked_block_offset(block_id, buf.len())?;
        if buf.is_empty() {
            return Ok(());
        }
        let end = start + buf.len();
        let mut storage = self.storage.lock();
        if storage.len() < end {
            storage.resize(end, 0);
        }
        storage[start..end].copy_from_slice(buf);
        Ok(())
    }

    fn flush(&mut self) -> DevResult {
        Ok(())
    }
}

fn checked_block_offset(block_id: u64, byte_len: usize) -> DevResult<usize> {
    if byte_len % 512 != 0 {
        return Err(DevError::InvalidParam);
    }
    let start = block_id
        .checked_mul(512)
        .and_then(|offset| usize::try_from(offset).ok())
        .ok_or(DevError::InvalidParam)?;
    let end = start.checked_add(byte_len).ok_or(DevError::InvalidParam)?;
    if end > SYNTHETIC_BLOCK_DEVICE_SIZE as usize {
        return Err(DevError::InvalidParam);
    }
    Ok(start)
}

impl BlockDeviceEntry {
    fn new(path: String, flags: u32) -> Self {
        let storage = synthetic_block_device_storage(path.as_str());
        Self {
            path,
            status_flags: fcntl_status_flags(flags),
            offset: Arc::new(Mutex::new(0)),
            storage,
        }
    }

    fn read(&mut self, dst: &mut [u8]) -> Result<usize, LinuxError> {
        if !file_is_readable(self.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let mut offset = self.offset.lock();
        if *offset >= SYNTHETIC_BLOCK_DEVICE_SIZE {
            return Ok(0);
        }
        let count = cmp::min(dst.len() as u64, SYNTHETIC_BLOCK_DEVICE_SIZE - *offset) as usize;
        dst[..count].fill(0);
        let start = *offset as usize;
        let storage = self.storage.lock();
        if start < storage.len() {
            let copied = cmp::min(count, storage.len() - start);
            dst[..copied].copy_from_slice(&storage[start..start + copied]);
        }
        *offset = offset.saturating_add(count as u64);
        Ok(count)
    }

    fn write(&mut self, src: &[u8]) -> Result<usize, LinuxError> {
        if !file_is_writable(self.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let mut offset = self.offset.lock();
        if *offset >= SYNTHETIC_BLOCK_DEVICE_SIZE {
            return Err(LinuxError::ENOSPC);
        }
        let count = cmp::min(src.len() as u64, SYNTHETIC_BLOCK_DEVICE_SIZE - *offset) as usize;
        let start = *offset as usize;
        let end = start.checked_add(count).ok_or(LinuxError::EFBIG)?;
        let mut storage = self.storage.lock();
        if storage.len() < end {
            let additional = end - storage.len();
            storage
                .try_reserve_exact(additional)
                .map_err(|_| LinuxError::ENOMEM)?;
            storage.resize(end, 0);
        }
        storage[start..end].copy_from_slice(&src[..count]);
        *offset = offset.saturating_add(count as u64);
        Ok(count)
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, LinuxError> {
        let mut offset = self.offset.lock();
        let next = match pos {
            SeekFrom::Start(value) => value,
            SeekFrom::Current(delta) => checked_seek_offset(*offset, delta)?,
            SeekFrom::End(delta) => checked_seek_offset(SYNTHETIC_BLOCK_DEVICE_SIZE, delta)?,
        };
        if next > SYNTHETIC_BLOCK_DEVICE_SIZE {
            return Err(LinuxError::EINVAL);
        }
        *offset = next;
        Ok(next)
    }
}

#[derive(Clone)]
pub(super) struct PathEntry {
    pub(super) path: String,
    pub(super) mode: u32,
    pub(super) size: u64,
    pub(super) blocks: u64,
}

#[derive(Clone)]
pub(super) struct MemoryFileEntry {
    pub(super) path: String,
    pub(super) data: Arc<Vec<u8>>,
    pub(super) offset: usize,
}

#[derive(Clone)]
pub(super) struct MemfdEntry {
    name: String,
    status_flags: u32,
    offset: Arc<Mutex<u64>>,
    state: Arc<Mutex<MemfdState>>,
}

#[derive(Clone)]
pub(super) enum MmapFileBacking {
    File(FileEntry),
    Memfd(MemfdEntry),
}

impl MmapFileBacking {
    pub(super) fn same_backing(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::File(left), Self::File(right)) => left.path == right.path,
            (Self::Memfd(left), Self::Memfd(right)) => left.same_backing(right),
            _ => false,
        }
    }
}

struct MemfdState {
    data: Vec<u8>,
    seals: u32,
}

#[derive(Clone)]
pub(super) struct ProcPagemapEntry {
    pub(super) path: String,
    pub(super) present_ranges: Arc<Vec<(u64, u64)>>,
    pub(super) offset: u64,
    pub(super) size: u64,
}

#[derive(Clone)]
pub(super) struct ProcTimerSlackEntry {
    pub(super) path: String,
    pub(super) target_pid: i32,
    pub(super) offset: usize,
    pub(super) status_flags: u32,
}

pub(super) struct EventFdEntry {
    counter: Mutex<u64>,
    semaphore: bool,
    events: EventSource,
    wait: WaitQueue,
}

#[derive(Clone)]
pub(super) struct InotifyEntry {
    status_flags: u32,
}

#[derive(Clone)]
pub(super) struct PidFdEntry {
    target_pid: i32,
    target_process: Weak<UserProcess>,
    status_flags: u32,
}

pub(super) struct TimerFdEntry {
    clock_id: u32,
    state: Mutex<TimerFdState>,
    events: EventSource,
    wait: WaitQueue,
    wait_generation: AtomicU64,
}

#[derive(Clone, Copy)]
struct TimerFdState {
    deadline: Option<Duration>,
    interval: Duration,
    expirations: u64,
}

#[derive(Clone)]
pub(super) struct SignalFdEntry {
    mask: Arc<Mutex<u64>>,
    status_flags: u32,
}

#[derive(Clone)]
pub(super) struct EpollEntry {
    state: Arc<EpollState>,
}

struct EpollState {
    id: OpenFileId,
    registrations: Mutex<BTreeMap<RegistrationKey, EpollRegistration>>,
    events: EventSource,
    wait: WaitQueue,
    generation: AtomicU64,
    next_version: AtomicU64,
    notifying: ReentrancyGate,
}

#[derive(Clone)]
enum EpollTarget {
    Object(Weak<OpenFile<dyn FileObject>>),
    Legacy(Arc<LegacyEpollTarget>),
    Epoll(Weak<EpollState>),
}

/// Epoll's compatibility reference for an unmigrated open description.
///
/// The duplicate is deliberately not part of `DescriptionIdentity`'s alias
/// count: the final userspace descriptor closes that identity and removes this
/// registration. Until then, this snapshot keeps a queryable legacy target
/// even if the process performing `epoll_wait` has closed all of its local
/// aliases while another fork still owns the description.
struct LegacyEpollTarget {
    entry: FdEntry,
}

impl LegacyEpollTarget {
    fn duplicate(entry: &FdEntry) -> Result<Arc<Self>, LinuxError> {
        Ok(Arc::new(Self {
            entry: entry.duplicate_for_fork()?,
        }))
    }
}

impl Drop for LegacyEpollTarget {
    fn drop(&mut self) {
        // Raw POSIX sockets do not have a Drop close. Every duplicate created
        // solely for this compatibility lease is therefore closed here.
        if let FdEntry::Socket(socket) = &self.entry {
            let _ = socket.close();
        }
    }
}

struct EpollRegistration {
    target: EpollTarget,
    event: general::epoll_event,
    delivery: EventDeliveryState,
    version: u64,
    ready_observer: Option<Arc<EpollReadyObserver>>,
    _subscription: Option<EventSubscription>,
    _close_observer: Arc<dyn EventObserver>,
    _close_subscription: EventSubscription,
}

struct EpollCtlUpdate {
    epoll: EpollEntry,
    // A legacy registration may own a raw POSIX socket duplicate. The syscall
    // drops this only after releasing the descriptor-table lock.
    retired: Option<EpollRegistration>,
}

impl EpollRegistration {
    fn new(
        target: EpollTarget,
        event: general::epoll_event,
        version: u64,
        ready_observer: Option<Arc<EpollReadyObserver>>,
        subscription: Option<EventSubscription>,
        close_observer: Arc<dyn EventObserver>,
        close_subscription: EventSubscription,
    ) -> Self {
        Self {
            target,
            event,
            delivery: EventDeliveryState::default(),
            version,
            ready_observer,
            _subscription: subscription,
            _close_observer: close_observer,
            _close_subscription: close_subscription,
        }
    }
}

#[derive(Clone)]
struct EpollRegistrationSnapshot {
    key: RegistrationKey,
    target: EpollTarget,
    event: general::epoll_event,
    delivery: EventDeliveryState,
    ready_observer: Option<Arc<EpollReadyObserver>>,
    version: u64,
}

struct EpollReadyObserver {
    owner: Weak<EpollState>,
    interests: ReadyEvents,
    notification: AtomicU64,
}

impl EpollReadyObserver {
    fn new(owner: &Arc<EpollState>, interests: ReadyEvents) -> Arc<Self> {
        Arc::new(Self {
            owner: Arc::downgrade(owner),
            interests,
            notification: AtomicU64::new(0),
        })
    }

    fn generation(&self) -> u64 {
        self.notification.load(Ordering::Acquire)
    }
}

impl EventObserver for EpollReadyObserver {
    fn on_event(&self, events: ReadyEvents) {
        if !events.intersects(self.interests) {
            return;
        }
        self.notification.fetch_add(1, Ordering::AcqRel);
        if let Some(owner) = self.owner.upgrade() {
            owner.wake();
        }
    }
}

struct EpollCloseObserver {
    owner: Weak<EpollState>,
    key: RegistrationKey,
    version: u64,
}

impl EventObserver for EpollCloseObserver {
    fn on_event(&self, _events: ReadyEvents) {
        if let Some(owner) = self.owner.upgrade() {
            owner.remove_registration_if_version(self.key, self.version);
        }
    }
}

pub(super) fn sys_openat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    flags: usize,
    mode: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let fd = match process.fds.lock().open(
        process,
        dirfd as i32,
        path.as_str(),
        flags as u32,
        mode as u32,
    ) {
        Ok(fd) => fd,
        Err(err) => return neg_errno(err),
    };
    match complete_open_fd(process, fd) {
        Ok(()) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_openat2(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    how: usize,
    size: usize,
) -> isize {
    const OPEN_HOW_SIZE: usize = size_of::<general::open_how>();
    const OPEN_HOW_MAX_EXT_BYTES: usize = 4096;

    if size < OPEN_HOW_SIZE {
        return neg_errno(LinuxError::EINVAL);
    }
    if size - OPEN_HOW_SIZE > OPEN_HOW_MAX_EXT_BYTES {
        return neg_errno(LinuxError::E2BIG);
    }

    let open_how = match read_user_value::<general::open_how>(process, how) {
        Ok(how) => how,
        Err(err) => return neg_errno(err),
    };
    if size > OPEN_HOW_SIZE {
        let extra_ptr = match how.checked_add(OPEN_HOW_SIZE) {
            Some(ptr) => ptr,
            None => return neg_errno(LinuxError::EFAULT),
        };
        let extra = match read_user_bytes(process, extra_ptr, size - OPEN_HOW_SIZE) {
            Ok(bytes) => bytes,
            Err(err) => return neg_errno(err),
        };
        if extra.iter().any(|byte| *byte != 0) {
            return neg_errno(LinuxError::E2BIG);
        }
    }

    if open_how.flags > u32::MAX as u64
        || open_how.mode > u32::MAX as u64
        || open_how.resolve > u32::MAX as u64
    {
        return neg_errno(LinuxError::EINVAL);
    }
    let flags = open_how.flags as u32;
    let mode = open_how.mode as u32;
    let resolve = open_how.resolve as u32;
    let supported_resolve = general::RESOLVE_NO_XDEV
        | general::RESOLVE_NO_MAGICLINKS
        | general::RESOLVE_NO_SYMLINKS
        | general::RESOLVE_BENEATH
        | general::RESOLVE_IN_ROOT
        | general::RESOLVE_CACHED;
    if resolve & !supported_resolve != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let mode_is_allowed = flags & general::O_CREAT != 0 || tmpfile_requested(flags);
    if mode & !0o7777 != 0 || (mode != 0 && !mode_is_allowed) {
        return neg_errno(LinuxError::EINVAL);
    }

    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let fd = {
        let mut table = process.fds.lock();
        if let Err(err) =
            openat2_resolve_guard(process, &table, dirfd as i32, path.as_str(), resolve)
        {
            return neg_errno(err);
        }
        match table.open(process, dirfd as i32, path.as_str(), flags, mode) {
            Ok(fd) => fd,
            Err(err) => return neg_errno(err),
        }
    };
    match complete_open_fd(process, fd) {
        Ok(()) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

fn complete_open_fd(process: &UserProcess, fd: i32) -> Result<(), LinuxError> {
    let fifo = {
        let table = process.fds.lock();
        match table.entry(fd)? {
            FdEntry::Pipe(pipe) => Some(pipe.clone()),
            _ => None,
        }
    };
    if let Some(pipe) = fifo {
        let endpoint = object_as::<PipeEndpoint>(&pipe)?;
        endpoint.notify_open();
        if let Err(err) = endpoint.wait_for_fifo_open_peer(pipe.status_flags()) {
            let closed = { process.fds.lock().close_for_process(process, fd).ok() };
            drop(closed);
            return Err(err);
        }
    }
    Ok(())
}

fn openat2_resolve_guard(
    process: &UserProcess,
    table: &FdTable,
    dirfd: i32,
    path: &str,
    resolve: u32,
) -> Result<(), LinuxError> {
    if resolve == 0 {
        return Ok(());
    }

    if resolve & general::RESOLVE_BENEATH != 0
        && (path.starts_with('/') || openat2_has_parent_escape(path))
    {
        return Err(LinuxError::EXDEV);
    }
    if resolve & general::RESOLVE_IN_ROOT != 0 && path.starts_with('/') {
        return Err(LinuxError::ENOENT);
    }

    let resolved_path = resolve_dirfd_path(process, table, dirfd, path)?;
    if resolve & general::RESOLVE_NO_XDEV != 0 && openat2_is_procfs_path(resolved_path.as_str()) {
        return Err(LinuxError::EXDEV);
    }

    if resolve & (general::RESOLVE_NO_MAGICLINKS | general::RESOLVE_NO_SYMLINKS) != 0
        && openat2_is_proc_magiclink(process, resolved_path.as_str())
    {
        return Err(LinuxError::ELOOP);
    }
    if resolve & general::RESOLVE_NO_SYMLINKS != 0
        && openat2_contains_recorded_symlink(process, resolved_path.as_str())?
    {
        return Err(LinuxError::ELOOP);
    }
    Ok(())
}

fn openat2_has_parent_escape(path: &str) -> bool {
    path.split('/').any(|component| component == "..")
}

fn openat2_is_procfs_path(path: &str) -> bool {
    normalize_path("/", path).is_some_and(|path| path == "/proc" || path.starts_with("/proc/"))
}

fn openat2_is_proc_magiclink(process: &UserProcess, path: &str) -> bool {
    proc_exe_link_target(process, path).is_some()
}

fn openat2_contains_recorded_symlink(
    process: &UserProcess,
    path: &str,
) -> Result<bool, LinuxError> {
    let path = normalize_path("/", path).ok_or(LinuxError::EINVAL)?;
    let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
    let mut prefix = String::new();
    for component in components {
        prefix.push('/');
        prefix.push_str(component);
        if process.path_symlink(prefix.as_str()).is_some() {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn sys_memfd_create(process: &UserProcess, name: usize, flags: usize) -> isize {
    let flags = flags as u32;
    let supported_flags = general::MFD_CLOEXEC | general::MFD_ALLOW_SEALING;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let name = match read_cstr(process, name) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    if name.len() > MEMFD_NAME_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    let entry = FdEntry::Memfd(MemfdEntry::new(
        name,
        general::O_RDWR,
        flags & general::MFD_ALLOW_SEALING != 0,
    ));
    let fd_flags = fd_cloexec_flag(flags & general::MFD_CLOEXEC != 0);
    match process.fds.lock().insert_with_flags(entry, fd_flags) {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_ftruncate(process: &UserProcess, fd: usize, length: usize) -> isize {
    let length = length as isize;
    if length < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let length = length as u64;
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    if length > file_size_limit {
        return neg_errno(LinuxError::EFBIG);
    }
    match process.fds.lock().truncate(process, fd as i32, length) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fallocate(
    process: &UserProcess,
    fd: usize,
    mode: usize,
    offset: usize,
    len: usize,
) -> isize {
    let offset = offset as isize;
    let len = len as isize;
    if offset < 0 || len <= 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let supported_modes = FALLOC_FL_KEEP_SIZE
        | FALLOC_FL_PUNCH_HOLE
        | FALLOC_FL_COLLAPSE_RANGE
        | FALLOC_FL_ZERO_RANGE
        | FALLOC_FL_INSERT_RANGE;
    if mode & !supported_modes != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    let range_ops = [
        FALLOC_FL_PUNCH_HOLE,
        FALLOC_FL_COLLAPSE_RANGE,
        FALLOC_FL_ZERO_RANGE,
        FALLOC_FL_INSERT_RANGE,
    ]
    .into_iter()
    .filter(|flag| mode & *flag != 0)
    .count();
    if range_ops > 1
        || mode & FALLOC_FL_PUNCH_HOLE != 0 && mode != FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE
        || mode & FALLOC_FL_COLLAPSE_RANGE != 0 && mode != FALLOC_FL_COLLAPSE_RANGE
        || mode & FALLOC_FL_INSERT_RANGE != 0 && mode != FALLOC_FL_INSERT_RANGE
    {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    let Some(end) = (offset as u64).checked_add(len as u64) else {
        return neg_errno(LinuxError::EFBIG);
    };
    if end > LINUX_MAX_FILE_OFFSET {
        return neg_errno(LinuxError::EFBIG);
    }
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    if end > file_size_limit {
        return neg_errno(LinuxError::EFBIG);
    }
    let result = if mode & FALLOC_FL_PUNCH_HOLE != 0 {
        process
            .fds
            .lock()
            .fallocate_punch_hole(process, fd as i32, offset as u64, len as u64)
    } else if mode & FALLOC_FL_ZERO_RANGE != 0 {
        process.fds.lock().fallocate_zero_range(
            process,
            fd as i32,
            offset as u64,
            len as u64,
            mode & FALLOC_FL_KEEP_SIZE != 0,
        )
    } else if mode & FALLOC_FL_COLLAPSE_RANGE != 0 {
        process
            .fds
            .lock()
            .fallocate_collapse_range(process, fd as i32, offset as u64, len as u64)
    } else if mode & FALLOC_FL_INSERT_RANGE != 0 {
        process
            .fds
            .lock()
            .fallocate_insert_range(process, fd as i32, offset as u64, len as u64)
    } else if mode == FALLOC_FL_KEEP_SIZE {
        process
            .fds
            .lock()
            .fallocate_allocate(process, fd as i32, offset as u64, len as u64, true)
    } else {
        process
            .fds
            .lock()
            .fallocate_allocate(process, fd as i32, offset as u64, len as u64, false)
    };
    match result {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fadvise64(
    process: &UserProcess,
    fd: usize,
    offset: usize,
    len: usize,
    advice: usize,
) -> isize {
    let offset = offset as isize;
    let len = len as isize;
    if offset < 0 || len < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let advice = advice as i32;
    if !(POSIX_FADV_MIN..=POSIX_FADV_MAX).contains(&advice) {
        return neg_errno(LinuxError::EINVAL);
    }
    match process.fds.lock().fadvise64(fd as i32) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_close(process: &UserProcess, fd: usize) -> isize {
    let result = { process.fds.lock().close_for_process(process, fd as i32) };
    match result {
        Ok(closed) => {
            drop(closed);
            0
        }
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_close_range(
    process: &UserProcess,
    first: usize,
    last: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported = CLOSE_RANGE_UNSHARE | CLOSE_RANGE_CLOEXEC;
    if flags & !supported != 0 || first > last {
        return neg_errno(LinuxError::EINVAL);
    }

    if flags & CLOSE_RANGE_UNSHARE != 0 {
        if let Err(err) = process.fds.unshare_for_pid_if_shared(process.pid()) {
            return neg_errno(err);
        }
    }

    let result = {
        process
            .fds
            .lock_for_pid(process.pid())
            .close_range_for_process(process, first, last, flags)
    };
    match result {
        Ok(closed) => {
            drop(closed);
            0
        }
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_epoll_create1(process: &UserProcess, flags: usize) -> isize {
    if flags & !(general::EPOLL_CLOEXEC as usize) != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    insert_epoll_fd(
        process,
        fd_cloexec_flag(flags & general::EPOLL_CLOEXEC as usize != 0),
    )
}

fn insert_epoll_fd(process: &UserProcess, fd_flags: u32) -> isize {
    let epoll = match EpollEntry::new() {
        Ok(epoll) => epoll,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .insert_with_flags(FdEntry::Epoll(epoll), fd_flags)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_eventfd2(process: &UserProcess, initval: usize, flags: usize) -> isize {
    let flags = flags as u32;
    let supported = general::EFD_CLOEXEC | general::EFD_NONBLOCK | general::EFD_SEMAPHORE;
    if flags & !supported != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let fd_flags = fd_cloexec_flag(flags & general::EFD_CLOEXEC != 0);
    let status_flags = if flags & general::EFD_NONBLOCK != 0 {
        general::O_NONBLOCK
    } else {
        0
    };
    let eventfd = match new_open_file(
        EventFdEntry::new(initval as u64, flags & general::EFD_SEMAPHORE != 0),
        status_flags,
    ) {
        Ok(file) => file,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .insert_with_flags(FdEntry::EventFd(eventfd), fd_flags)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_inotify_init1(process: &UserProcess, flags: usize) -> isize {
    let flags = flags as u32;
    let supported = general::IN_CLOEXEC | general::IN_NONBLOCK;
    if flags & !supported != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let fd_flags = fd_cloexec_flag(flags & general::IN_CLOEXEC != 0);
    let status_flags = if flags & general::IN_NONBLOCK != 0 {
        general::O_NONBLOCK
    } else {
        0
    };
    match process
        .fds
        .lock()
        .insert_with_flags(FdEntry::Inotify(InotifyEntry::new(status_flags)), fd_flags)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_pidfd_open(process: &UserProcess, pid: usize, flags: usize) -> isize {
    let pid = pid as i32;
    let flags = flags as u32;
    if pid <= 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if flags & !general::O_NONBLOCK != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let target_process = if pid == process.pid() {
        match user_thread_entry_for_process(process) {
            Some(entry) => entry.process,
            None => return neg_errno(LinuxError::ESRCH),
        }
    } else {
        match process
            .child_thread_entry_by_pid(pid)
            .or_else(|| user_thread_entry_by_process_pid(pid))
        {
            Some(entry) => entry.process,
            None => return neg_errno(LinuxError::ESRCH),
        }
    };
    match process.fds.lock().insert_with_flags(
        FdEntry::PidFd(PidFdEntry::new(
            pid,
            target_process,
            flags & general::O_NONBLOCK,
        )),
        fd_cloexec_flag(true),
    ) {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_pidfd_getfd(
    process: &UserProcess,
    pidfd: usize,
    targetfd: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let pidfd = pidfd as i32;
    let targetfd = targetfd as i32;
    let target_process = match process.fds.lock().entry(pidfd) {
        Ok(FdEntry::PidFd(pidfd)) => {
            if pidfd.exited() {
                return neg_errno(LinuxError::ESRCH);
            }
            match pidfd.target_process() {
                Some(process) => process,
                None => return neg_errno(LinuxError::ESRCH),
            }
        }
        Ok(_) => return neg_errno(LinuxError::EBADF),
        Err(err) => return neg_errno(err),
    };
    if !process_fd_access_allowed(process, target_process.as_ref()) {
        return neg_errno(LinuxError::EPERM);
    }

    let (entry, description_id) = match target_process.fds.lock().duplicate_entry(targetfd) {
        Ok(duplicate) => duplicate,
        Err(err) => return neg_errno(err),
    };
    match process.fds.lock().insert_min_with_description(
        entry,
        0,
        general::FD_CLOEXEC,
        description_id,
    ) {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_kcmp(
    process: &UserProcess,
    pid1: usize,
    pid2: usize,
    kcmp_type: usize,
    idx1: usize,
    idx2: usize,
) -> isize {
    const KCMP_FILE: usize = 0;

    if kcmp_type != KCMP_FILE {
        return neg_errno(LinuxError::EINVAL);
    }
    let pid1 = pid1 as i32;
    let pid2 = pid2 as i32;
    if pid1 <= 0 || pid2 <= 0 {
        return neg_errno(LinuxError::ESRCH);
    }
    let left = match kcmp_file_description_key(process, pid1, idx1 as i32) {
        Ok(key) => key,
        Err(err) => return neg_errno(err),
    };
    let right = match kcmp_file_description_key(process, pid2, idx2 as i32) {
        Ok(key) => key,
        Err(err) => return neg_errno(err),
    };
    if left.is_some() && left == right {
        0
    } else {
        1
    }
}

pub(super) fn sys_timerfd_create(process: &UserProcess, clock_id: usize, flags: usize) -> isize {
    let clock_id = clock_id as u32;
    if !TimerFdEntry::clock_supported(clock_id) {
        return neg_errno(LinuxError::EINVAL);
    }
    let flags = flags as u32;
    let supported = general::TFD_CLOEXEC | general::TFD_NONBLOCK;
    if flags & !supported != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let fd_flags = fd_cloexec_flag(flags & general::TFD_CLOEXEC != 0);
    let status_flags = if flags & general::TFD_NONBLOCK != 0 {
        general::O_NONBLOCK
    } else {
        0
    };
    let timer = match new_open_file(TimerFdEntry::new(clock_id), status_flags) {
        Ok(file) => file,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .insert_with_flags(FdEntry::TimerFd(timer), fd_flags)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_timerfd_settime(
    process: &UserProcess,
    fd: usize,
    flags: usize,
    new_value: usize,
    old_value: usize,
) -> isize {
    let flags = flags as u32;
    let supported = general::TFD_TIMER_ABSTIME | general::TFD_TIMER_CANCEL_ON_SET;
    if flags & !supported != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let timer = {
        let table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::TimerFd(timer)) => timer.clone(),
            Ok(_) => return neg_errno(LinuxError::EINVAL),
            Err(err) => return neg_errno(err),
        }
    };
    let timer = match object_as::<TimerFdEntry>(&timer) {
        Ok(timer) => timer,
        Err(err) => return neg_errno(err),
    };
    let new_spec = match read_user_value::<general::itimerspec>(process, new_value) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
    let old_spec = match timer.settime(flags, new_spec) {
        Ok(old_spec) => old_spec,
        Err(err) => return neg_errno(err),
    };
    if old_value != 0 {
        let ret = write_user_value(process, old_value, &old_spec);
        if ret != 0 {
            return ret;
        }
    }
    0
}

pub(super) fn sys_timerfd_gettime(process: &UserProcess, fd: usize, curr_value: usize) -> isize {
    let timer = {
        let table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::TimerFd(timer)) => timer.clone(),
            Ok(_) => return neg_errno(LinuxError::EINVAL),
            Err(err) => return neg_errno(err),
        }
    };
    let timer = match object_as::<TimerFdEntry>(&timer) {
        Ok(timer) => timer,
        Err(err) => return neg_errno(err),
    };
    let spec = match timer.gettime() {
        Ok(spec) => spec,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, curr_value, &spec)
}

pub(super) fn sys_signalfd4(
    process: &UserProcess,
    fd: usize,
    mask: usize,
    sigsetsize: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported = general::O_CLOEXEC | general::O_NONBLOCK;
    if flags & !supported != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if sigsetsize != 0 && sigsetsize < super::linux_abi::KERNEL_SIGSET_BYTES {
        return neg_errno(LinuxError::EINVAL);
    }
    let mask_bytes = match read_user_bytes(process, mask, super::linux_abi::KERNEL_SIGSET_BYTES) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    let mut raw = [0u8; super::linux_abi::KERNEL_SIGSET_BYTES];
    raw.copy_from_slice(&mask_bytes);
    let signal_mask = u64::from_ne_bytes(raw);
    let fd = fd as i32;
    if fd == -1 {
        let fd_flags = fd_cloexec_flag(flags & general::O_CLOEXEC != 0);
        let status_flags = flags & general::O_NONBLOCK;
        return match process.fds.lock().insert_with_flags(
            FdEntry::SignalFd(SignalFdEntry::new(signal_mask, status_flags)),
            fd_flags,
        ) {
            Ok(newfd) => newfd as isize,
            Err(err) => neg_errno(err),
        };
    }
    let mut table = process.fds.lock();
    match table.entry_mut(fd) {
        Ok(FdEntry::SignalFd(signal_fd)) => {
            signal_fd.set_mask(signal_mask);
            fd as isize
        }
        Ok(_) => neg_errno(LinuxError::EINVAL),
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_epoll_ctl(
    process: &UserProcess,
    epfd: usize,
    op: usize,
    fd: usize,
    event: usize,
) -> isize {
    let event_value = match op as u32 {
        general::EPOLL_CTL_ADD | general::EPOLL_CTL_MOD => {
            if event == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            match read_user_value::<general::epoll_event>(process, event) {
                Ok(event) => Some(event),
                Err(err) => return neg_errno(err),
            }
        }
        general::EPOLL_CTL_DEL => None,
        _ => return neg_errno(LinuxError::EINVAL),
    };
    let result = {
        let mut table = process.fds.lock();
        table.epoll_ctl(epfd as i32, op as u32, fd as i32, event_value)
    };
    match result {
        Ok(update) => {
            // Observer callbacks may enter nested event code, so wake only
            // after the descriptor-table guard has been dropped. A retired
            // legacy target is likewise closed outside that guard.
            drop(update.retired);
            update.epoll.state.wake();
            0
        }
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_epoll_pwait(
    process: &UserProcess,
    epfd: usize,
    events: usize,
    maxevents: usize,
    timeout_ms: isize,
    sigmask: usize,
    sigsetsize: usize,
) -> isize {
    let timeout = if timeout_ms == 0 {
        EpollWaitTimeout::Immediate
    } else if timeout_ms < 0 {
        EpollWaitTimeout::Infinite
    } else {
        let syscall_start = poll_clock_now();
        EpollWaitTimeout::Until(
            syscall_start + core::time::Duration::from_millis(timeout_ms as u64),
        )
    };
    sys_epoll_wait_with_timeout(
        process, epfd, events, maxevents, timeout, sigmask, sigsetsize,
    )
}

pub(super) fn sys_epoll_pwait2(
    process: &UserProcess,
    epfd: usize,
    events: usize,
    maxevents: usize,
    timeout: usize,
    sigmask: usize,
    sigsetsize: usize,
) -> isize {
    let timeout = if timeout == 0 {
        EpollWaitTimeout::Infinite
    } else {
        let ts = match read_user_value::<general::timespec>(process, timeout) {
            Ok(ts) => ts,
            Err(err) => return neg_errno(err),
        };
        if ts.tv_sec < 0 || !(0..1_000_000_000).contains(&ts.tv_nsec) {
            return neg_errno(LinuxError::EINVAL);
        }
        if ts.tv_sec == 0 && ts.tv_nsec == 0 {
            EpollWaitTimeout::Immediate
        } else {
            let syscall_start = poll_clock_now();
            EpollWaitTimeout::Until(
                syscall_start + core::time::Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32),
            )
        }
    };
    sys_epoll_wait_with_timeout(
        process, epfd, events, maxevents, timeout, sigmask, sigsetsize,
    )
}

#[derive(Clone, Copy)]
enum EpollWaitTimeout {
    Immediate,
    Until(core::time::Duration),
    Infinite,
}

fn poll_clock_now() -> core::time::Duration {
    axhal::time::monotonic_time()
}

fn sys_epoll_wait_with_timeout(
    process: &UserProcess,
    epfd: usize,
    events: usize,
    maxevents: usize,
    timeout: EpollWaitTimeout,
    sigmask: usize,
    sigsetsize: usize,
) -> isize {
    if maxevents == 0 || maxevents > FD_TABLE_LIMIT {
        return neg_errno(LinuxError::EINVAL);
    }
    if events == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    perf_counters::record_epoll_wait();
    let _signal_mask_guard = match install_temporary_signal_mask(process, sigmask, sigsetsize) {
        Ok(guard) => guard,
        Err(err) => return neg_errno(err),
    };
    let epoll = {
        let table = process.fds.lock_for_pid(process.pid());
        match table.epoll_entry(epfd as i32) {
            Ok(epoll) => epoll,
            Err(err) => return neg_errno(err),
        }
    };
    let mut ready = Vec::new();
    if matches!(timeout, EpollWaitTimeout::Immediate) {
        if let Err(err) = FdTable::epoll_collect_ready_for(&epoll, maxevents, &mut ready) {
            return neg_errno(err);
        }
        return copy_epoll_events_to_user(process, events, &ready);
    }
    loop {
        if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
            return neg_errno(LinuxError::EINTR);
        }
        ready.clear();
        let wait_generation = epoll.state.generation();
        if let Err(err) = FdTable::epoll_collect_ready_for(&epoll, maxevents, &mut ready) {
            return neg_errno(err);
        }
        let wait_decision = decide_level_wait(
            !ready.is_empty(),
            matches!(timeout, EpollWaitTimeout::Until(ddl) if poll_clock_now() >= ddl),
        );
        if matches!(wait_decision, LevelWaitDecision::Ready) {
            return copy_epoll_events_to_user(process, events, &ready);
        }
        if matches!(wait_decision, LevelWaitDecision::TimedOut) {
            return 0;
        }
        let wait_deadline = match timeout {
            EpollWaitTimeout::Until(deadline) => Some(deadline),
            EpollWaitTimeout::Infinite => None,
            EpollWaitTimeout::Immediate => unreachable!("immediate epoll waits return before loop"),
        };
        let (has_legacy, object_timeout) =
            match FdTable::epoll_wait_profile(&epoll.state, &mut Vec::new()) {
                Ok(profile) => profile,
                Err(err) => return neg_errno(err),
            };
        let mut delay = if has_legacy {
            Duration::from_millis(1)
        } else {
            Duration::from_millis(10)
        };
        if let Some(timeout) = object_timeout {
            delay = delay.min(timeout);
        }
        if let Some(deadline) = wait_deadline {
            delay = delay.min(deadline.saturating_sub(poll_clock_now()));
        }
        // Empty epolls also wait on this queue: epoll_ctl wakes the state.  For
        // legacy targets, the bounded 1 ms delay preserves the old adapter.
        epoll.state.wait_for_change(process, wait_generation, delay);
        // Always return to the level query.  A notification can happen before
        // the deadline while scheduling resumes this task after it.
    }
}

fn copy_epoll_events_to_user(
    process: &UserProcess,
    events: usize,
    ready: &[general::epoll_event],
) -> isize {
    if ready.is_empty() {
        return 0;
    }
    for (idx, event) in ready.iter().enumerate() {
        let Some(dst) = events.checked_add(idx * size_of::<general::epoll_event>()) else {
            return neg_errno(LinuxError::EFAULT);
        };
        let ret = write_user_value(process, dst, event);
        if ret != 0 {
            return ret;
        }
    }
    ready.len() as isize
}

impl EventFdEntry {
    const COUNTER_MAX: u64 = u64::MAX - 1;

    fn new(initval: u64, semaphore: bool) -> Self {
        Self {
            counter: Mutex::new(initval),
            semaphore,
            events: EventSource::new(),
            wait: WaitQueue::new(),
        }
    }

    fn poll_readable(&self) -> bool {
        *self.counter.lock() > 0
    }

    fn notify_state_change(&self, events: ReadyEvents) {
        self.events.notify(events);
        self.wait.notify_all(false);
    }

    fn interrupted(process: &UserProcess) -> bool {
        process.eval_watchdog_expired() || current_unblocked_signal_pending()
    }

    fn read(
        &self,
        status_flags: u32,
        process: &UserProcess,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        if dst.len() < size_of::<u64>() {
            return Err(LinuxError::EINVAL);
        }
        loop {
            {
                let mut counter = self.counter.lock();
                if *counter > 0 {
                    let value = if self.semaphore { 1 } else { *counter };
                    if self.semaphore {
                        *counter -= 1;
                    } else {
                        *counter = 0;
                    }
                    drop(counter);
                    dst[..size_of::<u64>()].copy_from_slice(&value.to_ne_bytes());
                    self.notify_state_change(ReadyEvents::WRITABLE);
                    return Ok(size_of::<u64>());
                }
            }
            if status_flags & general::O_NONBLOCK != 0 {
                return Err(LinuxError::EAGAIN);
            }
            if Self::interrupted(process) {
                return Err(LinuxError::EINTR);
            }
            process.set_syscall_wait_blocked(true);
            self.wait.wait_timeout_until(Duration::from_millis(10), || {
                self.poll_readable() || Self::interrupted(process)
            });
            process.set_syscall_wait_blocked(false);
        }
    }

    fn write(
        &self,
        status_flags: u32,
        process: &UserProcess,
        src: &[u8],
    ) -> Result<usize, LinuxError> {
        if src.len() < size_of::<u64>() {
            return Err(LinuxError::EINVAL);
        }
        let value = u64::from_ne_bytes(src[..size_of::<u64>()].try_into().unwrap());
        if value == u64::MAX {
            return Err(LinuxError::EINVAL);
        }
        loop {
            {
                let mut counter = self.counter.lock();
                if value <= Self::COUNTER_MAX.saturating_sub(*counter) {
                    *counter += value;
                    drop(counter);
                    if value != 0 {
                        self.notify_state_change(ReadyEvents::READABLE);
                    }
                    return Ok(size_of::<u64>());
                }
            }
            if status_flags & general::O_NONBLOCK != 0 {
                return Err(LinuxError::EAGAIN);
            }
            if Self::interrupted(process) {
                return Err(LinuxError::EINTR);
            }
            process.set_syscall_wait_blocked(true);
            self.wait.wait_timeout_until(Duration::from_millis(10), || {
                value <= Self::COUNTER_MAX.saturating_sub(*self.counter.lock())
                    || Self::interrupted(process)
            });
            process.set_syscall_wait_blocked(false);
        }
    }
}

impl FileObject for EventFdEntry {
    fn read(
        &self,
        description: &OpenFile<dyn FileObject>,
        process: &UserProcess,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        self.read(description.status_flags(), process, dst)
    }

    fn write(
        &self,
        description: &OpenFile<dyn FileObject>,
        process: &UserProcess,
        src: &[u8],
    ) -> Result<usize, LinuxError> {
        self.write(description.status_flags(), process, src)
    }

    fn readiness(&self) -> ReadyEvents {
        let mut events = ReadyEvents::EMPTY;
        let counter = *self.counter.lock();
        if counter > 0 {
            events |= ReadyEvents::READABLE;
        }
        if counter < Self::COUNTER_MAX {
            events |= ReadyEvents::WRITABLE;
        }
        events
    }

    fn event_source(&self) -> &EventSource {
        &self.events
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl InotifyEntry {
    fn new(status_flags: u32) -> Self {
        Self {
            status_flags: status_flags & general::O_NONBLOCK,
        }
    }

    fn status_flags(&self) -> u32 {
        self.status_flags
    }

    fn set_status_flags(&mut self, flags: u32) {
        self.status_flags = flags & general::O_NONBLOCK;
    }

    fn nonblocking(&self) -> bool {
        self.status_flags & general::O_NONBLOCK != 0
    }

    fn read(&self) -> Result<usize, LinuxError> {
        if self.nonblocking() {
            Err(LinuxError::EAGAIN)
        } else {
            Err(LinuxError::EINTR)
        }
    }
}

impl PidFdEntry {
    fn new(target_pid: i32, target_process: Arc<UserProcess>, status_flags: u32) -> Self {
        Self {
            target_pid,
            target_process: Arc::downgrade(&target_process),
            status_flags: status_flags & general::O_NONBLOCK,
        }
    }

    fn target_process(&self) -> Option<Arc<UserProcess>> {
        self.target_process.upgrade()
    }

    fn status_flags(&self) -> u32 {
        self.status_flags
    }

    fn set_status_flags(&mut self, flags: u32) {
        self.status_flags = flags & general::O_NONBLOCK;
    }

    fn nonblocking(&self) -> bool {
        self.status_flags & general::O_NONBLOCK != 0
    }

    fn exited(&self) -> bool {
        self.target_process
            .upgrade()
            .is_none_or(|process| process.live_threads.load(Ordering::Acquire) == 0)
    }
}

impl TimerFdEntry {
    fn new(clock_id: u32) -> Self {
        Self {
            clock_id,
            state: Mutex::new(TimerFdState {
                deadline: None,
                interval: Duration::ZERO,
                expirations: 0,
            }),
            events: EventSource::new(),
            wait: WaitQueue::new(),
            wait_generation: AtomicU64::new(0),
        }
    }

    fn clock_supported(clock_id: u32) -> bool {
        matches!(
            clock_id,
            general::CLOCK_REALTIME
                | general::CLOCK_MONOTONIC
                | general::CLOCK_BOOTTIME
                | general::CLOCK_REALTIME_ALARM
                | general::CLOCK_BOOTTIME_ALARM
        )
    }

    fn notify_state_change(&self) {
        self.wait_generation.fetch_add(1, Ordering::AcqRel);
        self.events.notify(ReadyEvents::READABLE);
        self.wait.notify_all(false);
    }

    fn settime(
        &self,
        flags: u32,
        new_spec: general::itimerspec,
    ) -> Result<general::itimerspec, LinuxError> {
        let new_interval = timespec_to_duration(new_spec.it_interval)?;
        let new_value = timespec_to_duration(new_spec.it_value)?;
        let mut state = self.state.lock();
        self.refresh_locked(&mut state)?;
        let old_spec = self.spec_from_state(&state)?;
        state.interval = new_interval;
        state.expirations = 0;
        state.deadline = if new_value == Duration::ZERO {
            None
        } else if flags & general::TFD_TIMER_ABSTIME != 0 {
            Some(new_value)
        } else {
            Some(saturating_duration_add(
                clock_now_duration(self.clock_id)?,
                new_value,
            ))
        };
        drop(state);
        self.notify_state_change();
        Ok(old_spec)
    }

    fn gettime(&self) -> Result<general::itimerspec, LinuxError> {
        self.refresh()?;
        self.spec_from_state(&self.state.lock())
    }

    fn poll_readable(&self) -> bool {
        self.refresh_silent().is_ok() && self.state.lock().expirations > 0
    }

    fn read(
        &self,
        status_flags: u32,
        process: &UserProcess,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        if dst.len() < size_of::<u64>() {
            return Err(LinuxError::EINVAL);
        }
        loop {
            self.refresh()?;
            {
                let mut state = self.state.lock();
                if state.expirations > 0 {
                    let value = state.expirations;
                    state.expirations = 0;
                    drop(state);
                    dst[..size_of::<u64>()].copy_from_slice(&value.to_ne_bytes());
                    self.notify_state_change();
                    return Ok(size_of::<u64>());
                }
            }
            if status_flags & general::O_NONBLOCK != 0 {
                return Err(LinuxError::EAGAIN);
            }
            if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
                return Err(LinuxError::EINTR);
            }
            let (deadline, generation) = {
                let state = self.state.lock();
                (state.deadline, self.wait_generation.load(Ordering::Acquire))
            };
            let wait_for = match deadline {
                Some(deadline) => deadline
                    .saturating_sub(clock_now_duration(self.clock_id)?)
                    .min(Duration::from_millis(10)),
                None => Duration::from_millis(10),
            };
            process.set_syscall_wait_blocked(true);
            self.wait.wait_timeout_until(wait_for, || {
                self.wait_generation.load(Ordering::Acquire) != generation
                    || process.eval_watchdog_expired()
                    || current_unblocked_signal_pending()
            });
            process.set_syscall_wait_blocked(false);
        }
    }

    fn refresh(&self) -> Result<(), LinuxError> {
        let changed = {
            let mut state = self.state.lock();
            let previous = state.expirations;
            self.refresh_locked(&mut state)?;
            state.expirations != previous
        };
        if changed {
            self.notify_state_change();
        }
        Ok(())
    }

    /// Refresh readiness during a level query without invoking observers from
    /// inside a poll/epoll call (which may hold a compatibility-table lock).
    /// Waiters use `next_timeout`, so passage of the timer deadline does not
    /// rely on this query producing a notification.
    fn refresh_silent(&self) -> Result<(), LinuxError> {
        self.refresh_locked(&mut self.state.lock())
    }

    fn refresh_locked(&self, state: &mut TimerFdState) -> Result<(), LinuxError> {
        let Some(deadline) = state.deadline else {
            return Ok(());
        };
        let now = clock_now_duration(self.clock_id)?;
        if now < deadline {
            return Ok(());
        }
        if state.interval == Duration::ZERO {
            state.expirations = state.expirations.saturating_add(1);
            state.deadline = None;
            return Ok(());
        }
        let elapsed = now.saturating_sub(deadline);
        let period_ns = duration_to_nanos_saturating(state.interval).max(1);
        let periods = elapsed.as_nanos() / period_ns + 1;
        let periods = periods.min(u64::MAX as u128) as u64;
        state.expirations = state.expirations.saturating_add(periods);
        state.deadline = Some(saturating_duration_add(
            deadline,
            duration_mul_saturating(state.interval, periods),
        ));
        Ok(())
    }

    fn spec_from_state(&self, state: &TimerFdState) -> Result<general::itimerspec, LinuxError> {
        let remaining = match state.deadline {
            Some(deadline) => {
                let now = clock_now_duration(self.clock_id)?;
                deadline.saturating_sub(now)
            }
            None => Duration::ZERO,
        };
        Ok(general::itimerspec {
            it_interval: duration_to_timespec(state.interval),
            it_value: duration_to_timespec(remaining),
        })
    }
}

impl FileObject for TimerFdEntry {
    fn read(
        &self,
        description: &OpenFile<dyn FileObject>,
        process: &UserProcess,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        self.read(description.status_flags(), process, dst)
    }

    fn write(
        &self,
        _description: &OpenFile<dyn FileObject>,
        _process: &UserProcess,
        _src: &[u8],
    ) -> Result<usize, LinuxError> {
        Err(LinuxError::EINVAL)
    }

    fn readiness(&self) -> ReadyEvents {
        if self.poll_readable() {
            ReadyEvents::READABLE
        } else {
            ReadyEvents::EMPTY
        }
    }

    fn event_source(&self) -> &EventSource {
        &self.events
    }

    fn next_timeout(&self) -> Option<Duration> {
        let state = self.state.lock();
        let now = clock_now_duration(self.clock_id).ok()?;
        readiness_deadline_delay(state.expirations > 0, state.deadline, now)
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl SignalFdEntry {
    fn new(mask: u64, status_flags: u32) -> Self {
        Self {
            mask: Arc::new(Mutex::new(mask)),
            status_flags: status_flags & general::O_NONBLOCK,
        }
    }

    fn status_flags(&self) -> u32 {
        self.status_flags
    }

    fn set_status_flags(&mut self, flags: u32) {
        self.status_flags = flags & general::O_NONBLOCK;
    }

    fn nonblocking(&self) -> bool {
        self.status_flags & general::O_NONBLOCK != 0
    }

    fn set_mask(&mut self, mask: u64) {
        *self.mask.lock() = mask;
    }

    fn mask(&self) -> u64 {
        *self.mask.lock()
    }

    fn poll_readable(&self) -> bool {
        current_pending_signal_matches(self.mask())
    }

    fn read(&self, process: &UserProcess, dst: &mut [u8]) -> Result<usize, LinuxError> {
        const SIGNALFD_SIGINFO_SIZE: usize = 128;
        if dst.len() < SIGNALFD_SIGINFO_SIZE {
            return Err(LinuxError::EINVAL);
        }
        loop {
            let mask = self.mask();
            if let Some((sig, sender_pid)) = take_current_pending_signal_matching(mask) {
                let mut info = [0u8; SIGNALFD_SIGINFO_SIZE];
                info[0..4].copy_from_slice(&(sig as u32).to_ne_bytes());
                info[8..12].copy_from_slice(&0i32.to_ne_bytes());
                info[12..16].copy_from_slice(&(sender_pid.max(0) as u32).to_ne_bytes());
                dst[..SIGNALFD_SIGINFO_SIZE].copy_from_slice(&info);
                return Ok(SIGNALFD_SIGINFO_SIZE);
            }
            if self.nonblocking() {
                return Err(LinuxError::EAGAIN);
            }
            if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
                return Err(LinuxError::EINTR);
            }
            yield_poll_wait();
        }
    }
}

fn duration_to_timespec(duration: Duration) -> general::timespec {
    general::timespec {
        tv_sec: duration.as_secs().min(i64::MAX as u64) as _,
        tv_nsec: duration.subsec_nanos() as _,
    }
}

fn duration_to_nanos_saturating(duration: Duration) -> u128 {
    duration.as_secs().min((u128::MAX / 1_000_000_000) as u64) as u128 * 1_000_000_000
        + duration.subsec_nanos() as u128
}

fn duration_mul_saturating(duration: Duration, count: u64) -> Duration {
    let nanos = duration_to_nanos_saturating(duration).saturating_mul(count as u128);
    duration_from_nanos_saturating(nanos)
}

fn saturating_duration_add(lhs: Duration, rhs: Duration) -> Duration {
    duration_from_nanos_saturating(
        duration_to_nanos_saturating(lhs).saturating_add(duration_to_nanos_saturating(rhs)),
    )
}

fn duration_from_nanos_saturating(nanos: u128) -> Duration {
    let secs = (nanos / 1_000_000_000).min(u64::MAX as u128) as u64;
    let nsec = (nanos % 1_000_000_000) as u32;
    Duration::new(secs, nsec)
}

impl EpollEntry {
    fn new() -> Result<Self, LinuxError> {
        Ok(Self {
            state: Arc::new(EpollState {
                id: OpenFileId::allocate().map_err(|_| LinuxError::ENFILE)?,
                registrations: Mutex::new(BTreeMap::new()),
                events: EventSource::new(),
                wait: WaitQueue::new(),
                generation: AtomicU64::new(0),
                next_version: AtomicU64::new(1),
                notifying: ReentrancyGate::new(),
            }),
        })
    }

    fn id(&self) -> OpenFileId {
        self.state.id
    }
}

impl EpollState {
    fn wake(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        self.wait.notify_all(false);
        // Cycle validation and insertion share the global graph lock, so the
        // live nested graph is acyclic. Coalesce concurrent/reentrant attempts
        // into another flat walk: this bounds stack recursion without losing
        // the parent wake that arrived during an active callback fan-out.
        let Some(mut notification_guard) = self.notifying.try_enter() else {
            return;
        };
        loop {
            self.events.notify(ReadyEvents::READABLE);
            if !notification_guard.finish_round() {
                break;
            }
        }
    }

    fn remove_registration_if_version(&self, key: RegistrationKey, version: u64) {
        let removed = {
            let mut registrations = self.registrations.lock();
            if registrations
                .get(&key)
                .is_some_and(|registration| registration.version == version)
            {
                registrations.remove(&key)
            } else {
                None
            }
        };
        if removed.is_some() {
            // Drop subscriptions before callbacks are fanned out, and never
            // while holding the registration map lock.
            drop(removed);
            self.wake();
        }
    }

    fn next_version(&self) -> Result<u64, LinuxError> {
        self.next_version
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .map_err(|_| LinuxError::ENFILE)
    }

    fn registration_snapshots(&self) -> Result<Vec<EpollRegistrationSnapshot>, LinuxError> {
        let registrations = self.registrations.lock();
        let mut snapshots = Vec::new();
        snapshots
            .try_reserve_exact(registrations.len())
            .map_err(|_| LinuxError::ENOMEM)?;
        for (&key, registration) in registrations.iter() {
            snapshots.push(EpollRegistrationSnapshot {
                key,
                target: registration.target.clone(),
                event: registration.event,
                delivery: registration.delivery,
                ready_observer: registration.ready_observer.clone(),
                version: registration.version,
            });
        }
        Ok(snapshots)
    }

    fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }

    fn wait_for_change(&self, process: &UserProcess, sample: u64, timeout: Duration) {
        if timeout.is_zero() || self.generation() != sample {
            return;
        }
        process.set_syscall_wait_blocked(true);
        self.wait.wait_timeout_until(timeout, || {
            self.generation() != sample
                || process.eval_watchdog_expired()
                || current_unblocked_signal_pending()
        });
        process.set_syscall_wait_blocked(false);
    }
}

fn read_open_file_to_user(
    process: &UserProcess,
    file: &OpenFileRef,
    buf: usize,
    count: usize,
) -> isize {
    if count == 0 {
        return match file.object().read(file, process, &mut []) {
            Ok(_) => 0,
            Err(err) => neg_errno(err),
        };
    }

    let mut total = 0usize;
    let mut scratch = Vec::new();
    while total < count {
        let Some(base) = buf.checked_add(total) else {
            return if total > 0 {
                total as isize
            } else {
                neg_errno(LinuxError::EFAULT)
            };
        };
        let len = (count - total).min(MAX_USER_IO_CHUNK);
        if let Err(err) = validate_user_write(process, base, len) {
            return if total > 0 {
                total as isize
            } else {
                neg_errno(err)
            };
        }
        let dst = match io_scratch_slice(&mut scratch, len) {
            Ok(dst) => dst,
            Err(err) => {
                return if total > 0 {
                    total as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        let read = match file.object().read(file, process, dst) {
            Ok(read) if read <= len => read,
            Ok(_) => return neg_errno(LinuxError::EINVAL),
            Err(err) => {
                return if total > 0 {
                    total as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        if let Err(err) = write_user_bytes(process, base, &dst[..read]) {
            return if total > 0 {
                total as isize
            } else {
                neg_errno(err)
            };
        }
        total += read;
        if read < len {
            break;
        }
    }
    total as isize
}

fn write_open_file_from_user(
    process: &UserProcess,
    file: &OpenFileRef,
    buf: usize,
    count: usize,
) -> isize {
    if count == 0 {
        return match file.object().write(file, process, &[]) {
            Ok(_) => 0,
            Err(err) => neg_errno(err),
        };
    }

    let mut total = 0usize;
    let mut scratch = Vec::new();
    while total < count {
        let Some(base) = buf.checked_add(total) else {
            return if total > 0 {
                total as isize
            } else {
                neg_errno(LinuxError::EFAULT)
            };
        };
        let len = (count - total).min(MAX_USER_IO_CHUNK);
        if let Err(err) = validate_user_read(process, base, len) {
            return if total > 0 {
                total as isize
            } else {
                neg_errno(err)
            };
        }
        let src = match io_scratch_slice(&mut scratch, len) {
            Ok(src) => src,
            Err(err) => {
                return if total > 0 {
                    total as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        if let Err(err) = read_user_bytes_into(process, base, src) {
            return if total > 0 {
                total as isize
            } else {
                neg_errno(err)
            };
        }
        let written = match file.object().write(file, process, src) {
            Ok(written) if written <= len => written,
            Ok(_) => return neg_errno(LinuxError::EINVAL),
            Err(err) => {
                return if total > 0 {
                    total as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        total += written;
        if written < len {
            break;
        }
    }
    total as isize
}

pub(super) fn sys_read(process: &UserProcess, fd: usize, buf: usize, count: usize) -> isize {
    if let Ok(socket) = socket_entry(process, fd) {
        return recv_socket_data_to_user(process, socket.posix_fd, buf, count, 0);
    }
    let open_file = { process.fds.lock().open_file_ref(fd as i32) };
    match open_file {
        Ok(Some(file)) => return read_open_file_to_user(process, &file, buf, count),
        Ok(None) => {}
        Err(err) => return neg_errno(err),
    }
    if count > MAX_USER_IO_CHUNK {
        let mut total = 0usize;
        let mut scratch = Vec::new();
        while total < count {
            let base = match buf.checked_add(total) {
                Some(base) => base,
                None => {
                    return if total > 0 {
                        total as isize
                    } else {
                        neg_errno(LinuxError::EFAULT)
                    };
                }
            };
            let len = (count - total).min(MAX_USER_IO_CHUNK);
            if let Err(err) = validate_user_write(process, base, len) {
                return if total > 0 {
                    total as isize
                } else {
                    neg_errno(err)
                };
            }
            let bytes = match io_scratch_slice(&mut scratch, len) {
                Ok(bytes) => bytes,
                Err(err) => {
                    return if total > 0 {
                        total as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            let n = match process.fds.lock().read(process, fd as i32, &mut *bytes) {
                Ok(n) => n,
                Err(err) => {
                    return if total > 0 {
                        total as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            if n > len {
                return if total > 0 {
                    total as isize
                } else {
                    neg_errno(LinuxError::EINVAL)
                };
            }
            if let Err(err) = write_user_bytes(process, base, &bytes[..n]) {
                return if total > 0 {
                    total as isize
                } else {
                    neg_errno(err)
                };
            }
            total += n;
            if n < len {
                return total as isize;
            }
        }
        return total as isize;
    }
    with_writable_user_buffer(process, buf, count, |dst| {
        process.fds.lock().read(process, fd as i32, dst)
    })
}

pub(super) fn sys_pread64(
    process: &UserProcess,
    fd: usize,
    buf: usize,
    count: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    with_writable_user_buffer(process, buf, count, |dst| {
        process
            .fds
            .lock()
            .read_file_at_into_fd(process, fd as i32, offset as u64, dst)
    })
}

pub(super) fn sys_write(process: &UserProcess, fd: usize, buf: usize, count: usize) -> isize {
    let open_file = { process.fds.lock().open_file_ref(fd as i32) };
    match open_file {
        Ok(Some(file)) => return write_open_file_from_user(process, &file, buf, count),
        Ok(None) => {}
        Err(err) => return neg_errno(err),
    }
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    if count > MAX_USER_IO_CHUNK {
        let mut written = 0usize;
        let mut scratch = Vec::new();
        while written < count {
            let base = match buf.checked_add(written) {
                Some(base) => base,
                None => {
                    return if written > 0 {
                        written as isize
                    } else {
                        neg_errno(LinuxError::EFAULT)
                    };
                }
            };
            let len = (count - written).min(MAX_USER_IO_CHUNK);
            if let Err(err) = validate_user_read(process, base, len) {
                return if written > 0 {
                    written as isize
                } else {
                    neg_errno(err)
                };
            }
            let src = match io_scratch_slice(&mut scratch, len) {
                Ok(src) => src,
                Err(err) => {
                    return if written > 0 {
                        written as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            if let Err(err) = read_user_bytes_into(process, base, &mut *src) {
                return if written > 0 {
                    written as isize
                } else {
                    neg_errno(err)
                };
            }
            let n = match process
                .fds
                .lock()
                .write(process, fd as i32, src, Some(file_size_limit))
            {
                Ok(v) => v,
                Err(err) => {
                    return if written > 0 {
                        written as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            written += n;
            if n < len {
                return written as isize;
            }
        }
        return written as isize;
    }
    with_readable_user_buffer(process, buf, count, |src| {
        process
            .fds
            .lock()
            .write(process, fd as i32, src, Some(file_size_limit))
    })
}

pub(super) fn sys_pwrite64(
    process: &UserProcess,
    fd: usize,
    buf: usize,
    count: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    with_readable_user_buffer(process, buf, count, |src| {
        let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
        process.fds.lock().write_file_at(
            process,
            fd as i32,
            offset as u64,
            src,
            Some(file_size_limit),
        )
    })
}

pub(super) fn sys_writev(process: &UserProcess, fd: usize, iov: usize, iovcnt: usize) -> isize {
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let open_file = match process.fds.lock().open_file_ref(fd as i32) {
        Ok(file) => file,
        Err(err) => return neg_errno(err),
    };
    let mut written = 0isize;
    let mut scratch = Vec::new();
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_read(process, base, remaining) {
            return if written > 0 { written } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let src = match io_scratch_slice(&mut scratch, len) {
                Ok(src) => src,
                Err(err) => return if written > 0 { written } else { neg_errno(err) },
            };
            if let Err(err) = read_user_bytes_into(process, base, &mut *src) {
                return if written > 0 { written } else { neg_errno(err) };
            }
            let result = match &open_file {
                Some(file) => file.object().write(file, process, src),
                None => {
                    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
                    process
                        .fds
                        .lock()
                        .write(process, fd as i32, src, Some(file_size_limit))
                }
            };
            let n = match result {
                Ok(v) => v,
                Err(err) => return if written > 0 { written } else { neg_errno(err) },
            };
            written += n as isize;
            if n < len {
                return written;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    written
}

pub(super) fn sys_readv(process: &UserProcess, fd: usize, iov: usize, iovcnt: usize) -> isize {
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let open_file = match process.fds.lock().open_file_ref(fd as i32) {
        Ok(file) => file,
        Err(err) => return neg_errno(err),
    };
    let mut total = 0isize;
    let mut scratch = Vec::new();
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_write(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let bytes = match io_scratch_slice(&mut scratch, len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let result = match &open_file {
                Some(file) => file.object().read(file, process, bytes),
                None => process.fds.lock().read(process, fd as i32, bytes),
            };
            let n = match result {
                Ok(v) => v,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            if n > len {
                return if total > 0 {
                    total
                } else {
                    neg_errno(LinuxError::EINVAL)
                };
            }
            if let Err(err) = write_user_bytes(process, base, &bytes[..n]) {
                return if total > 0 { total } else { neg_errno(err) };
            }
            total += n as isize;
            if n < len {
                return total;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    total
}

pub(super) fn sys_preadv(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let mut total = 0isize;
    let mut next_offset = offset as u64;
    let mut scratch = Vec::new();
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_write(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let bytes = match io_scratch_slice(&mut scratch, len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let n = match process.fds.lock().read_file_at_into_fd(
                process,
                fd as i32,
                next_offset,
                &mut *bytes,
            ) {
                Ok(v) => v,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            if let Err(err) = write_user_bytes(process, base, &bytes[..n]) {
                return if total > 0 { total } else { neg_errno(err) };
            }
            total += n as isize;
            next_offset = next_offset.saturating_add(n as u64);
            if n < len {
                return total;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    total
}

fn split_offset_arg(pos_l: usize, pos_h: usize) -> i64 {
    let low = pos_l as u32 as u64;
    let high = pos_h as u32 as u64;
    ((high << 32) | low) as i64
}

pub(super) fn sys_preadv2(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    pos_l: usize,
    pos_h: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    match split_offset_arg(pos_l, pos_h) {
        -1 => sys_readv(process, fd, iov, iovcnt),
        offset if offset < -1 => neg_errno(LinuxError::EINVAL),
        offset => sys_preadv(process, fd, iov, iovcnt, offset as usize),
    }
}

pub(super) fn sys_pwritev(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let mut total = 0isize;
    let mut next_offset = offset as u64;
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    let mut scratch = Vec::new();
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_read(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let src = match io_scratch_slice(&mut scratch, len) {
                Ok(src) => src,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            if let Err(err) = read_user_bytes_into(process, base, &mut *src) {
                return if total > 0 { total } else { neg_errno(err) };
            }
            let n = match process.fds.lock().write_file_at(
                process,
                fd as i32,
                next_offset,
                src,
                Some(file_size_limit),
            ) {
                Ok(v) => v,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            total += n as isize;
            next_offset = next_offset.saturating_add(n as u64);
            if n < len {
                return total;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    total
}

pub(super) fn sys_pwritev2(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    pos_l: usize,
    pos_h: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    match split_offset_arg(pos_l, pos_h) {
        -1 => sys_writev(process, fd, iov, iovcnt),
        offset if offset < -1 => neg_errno(LinuxError::EINVAL),
        offset => sys_pwritev(process, fd, iov, iovcnt, offset as usize),
    }
}

pub(super) fn sys_sendfile(
    process: &UserProcess,
    out_fd: usize,
    in_fd: usize,
    offset_ptr: usize,
    count: usize,
) -> isize {
    let mut offset = if offset_ptr == 0 {
        None
    } else {
        if let Err(err) = validate_user_write(process, offset_ptr, size_of::<i64>()) {
            return neg_errno(err);
        }
        match read_user_value::<i64>(process, offset_ptr) {
            Ok(value) if value >= 0 => Some(value as u64),
            Ok(_) => return neg_errno(LinuxError::EINVAL),
            Err(err) => return neg_errno(err),
        }
    };
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    let output_file = match process.fds.lock().open_file_ref(out_fd as i32) {
        Ok(file) => file,
        Err(err) => return neg_errno(err),
    };
    {
        let mut table = process.fds.lock();
        let input_check = match offset {
            Some(pos) => table.read_file_at_into_fd(process, in_fd as i32, pos, &mut []),
            None => table
                .read_file_at_current_offset_into_fd(process, in_fd as i32, &mut [])
                .map(|(_, read)| read),
        };
        if let Err(err) = input_check {
            return neg_errno(err);
        }
        if output_file.is_none() {
            if let Err(err) = table.write(process, out_fd as i32, &[], Some(file_size_limit)) {
                return neg_errno(err);
            }
        }
    }
    if let Some(file) = &output_file {
        if let Err(err) = file.object().write(file, process, &[]) {
            return neg_errno(err);
        }
    }

    let mut copied = 0usize;
    let mut scratch = Vec::new();
    while copied < count {
        let chunk_len = (count - copied).min(MAX_USER_IO_CHUNK);
        let buf = match io_scratch_slice(&mut scratch, chunk_len) {
            Ok(buf) => buf,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        let read = {
            let mut table = process.fds.lock();
            match offset {
                Some(pos) => table.read_file_at_into_fd(process, in_fd as i32, pos, &mut *buf),
                None => table
                    .read_file_at_current_offset_into_fd(process, in_fd as i32, &mut *buf)
                    .map(|(_, read)| read),
            }
        };
        let read = match read {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        let write_result = match &output_file {
            Some(file) => file.object().write(file, process, &buf[..read]),
            None => process.fds.lock().write(
                process,
                out_fd as i32,
                &buf[..read],
                Some(file_size_limit),
            ),
        };
        let written = match write_result {
            Ok(n) => n,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        if let Some(pos) = offset.as_mut() {
            *pos = pos.saturating_add(written as u64);
        } else if let Err(err) = process
            .fds
            .lock()
            .advance_file_offset_fd(in_fd as i32, written)
        {
            return if copied > 0 {
                copied as isize
            } else {
                neg_errno(err)
            };
        }
        copied += written;
        if written < read {
            break;
        }
    }
    if let Some(pos) = offset {
        let out: i64 = match pos.try_into() {
            Ok(value) => value,
            Err(_) => return neg_errno(LinuxError::EOVERFLOW),
        };
        let ret = write_user_value(process, offset_ptr, &out);
        if ret < 0 {
            return if copied > 0 { copied as isize } else { ret };
        }
    }
    copied as isize
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpliceEndpointKind {
    RegularFile,
    Pipe,
    Stream,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpliceStreamReadiness {
    Data(usize),
    WouldBlock { endpoint_nonblocking: bool },
    Eof,
    Unknown,
}

#[derive(Clone, Copy)]
enum VmspliceDirection {
    ToPipe,
    FromPipe,
}

pub(super) fn sys_splice(
    process: &UserProcess,
    fd_in: usize,
    off_in_ptr: usize,
    fd_out: usize,
    off_out_ptr: usize,
    len: usize,
    flags: usize,
) -> isize {
    if len == 0 {
        return 0;
    }
    let supported_flags = (general::SPLICE_F_MOVE
        | general::SPLICE_F_NONBLOCK
        | general::SPLICE_F_MORE
        | general::SPLICE_F_GIFT) as usize;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let nonblocking = flags & general::SPLICE_F_NONBLOCK as usize != 0;
    let fd_in = fd_in as i32;
    let fd_out = fd_out as i32;

    // Linux pins both descriptors before inspecting user offsets.  Clone migrated
    // pipe descriptions in that same fd-table critical section so a concurrent
    // close/reuse cannot replace an endpoint after it has been classified.
    let ((input_description, input_pipe), (output_description, output_pipe)) = {
        let table = process.fds.lock();
        let input = match table.splice_pipe_snapshot(fd_in) {
            Ok(snapshot) => snapshot,
            Err(err) => return neg_errno(err),
        };
        let output = match table.splice_pipe_snapshot(fd_out) {
            Ok(snapshot) => snapshot,
            Err(err) => return neg_errno(err),
        };
        (input, output)
    };
    // Pipe offsets are rejected by object type, without dereferencing the user
    // pointer.  Both fd lookups above intentionally precede these checks.
    if input_pipe.is_some() && off_in_ptr != 0 {
        return neg_errno(LinuxError::ESPIPE);
    }
    if output_pipe.is_some() && off_out_ptr != 0 {
        return neg_errno(LinuxError::ESPIPE);
    }
    // __do_splice copies the output offset before the input offset.
    let mut off_out = match read_copy_file_range_offset(process, off_out_ptr) {
        Ok(offset) => offset,
        Err(err) => return neg_errno(err),
    };
    let mut off_in = match read_copy_file_range_offset(process, off_in_ptr) {
        Ok(offset) => offset,
        Err(err) => return neg_errno(err),
    };
    // The migrated pipe-to-pipe path uses only the descriptions pinned by the
    // first fd-table critical section.  No later close or descriptor-number
    // reuse can replace either endpoint during validation or transfer.
    if let (Some(input), Some(output)) = (input_pipe.as_ref(), output_pipe.as_ref()) {
        let result = pipe_endpoint(input).and_then(|source| {
            source.splice_to(
                input.status_flags(),
                pipe_endpoint(output)?,
                output.status_flags(),
                len,
                nonblocking,
            )
        });
        return match result {
            Ok(moved) => moved as isize,
            Err(err) => neg_errno(err),
        };
    }
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();

    let (in_kind, out_kind) = {
        let mut table = process.fds.lock();
        // Legacy non-pipe adapters are still table-owned.  Refuse a changed
        // slot rather than classifying or operating on an object installed
        // under the same descriptor number after the entry snapshot.
        if input_pipe.is_none() && !table.splice_snapshot_is_current(fd_in, input_description) {
            return neg_errno(LinuxError::EBADF);
        }
        if output_pipe.is_none() && !table.splice_snapshot_is_current(fd_out, output_description) {
            return neg_errno(LinuxError::EBADF);
        }
        let in_kind = if let Some(pipe) = input_pipe.as_ref() {
            match pipe_endpoint(pipe) {
                Ok(endpoint) if endpoint.readable() => SpliceEndpointKind::Pipe,
                Ok(_) => return neg_errno(LinuxError::EBADF),
                Err(err) => return neg_errno(err),
            }
        } else {
            match splice_input_kind(&table, fd_in, off_in.is_some()) {
                Ok(kind) => kind,
                Err(err) => return neg_errno(err),
            }
        };
        let out_kind = if let Some(pipe) = output_pipe.as_ref() {
            match pipe_endpoint(pipe) {
                Ok(endpoint) if endpoint.writable() => SpliceEndpointKind::Pipe,
                Ok(_) => return neg_errno(LinuxError::EBADF),
                Err(err) => return neg_errno(err),
            }
        } else {
            match splice_output_kind(&table, fd_out, off_out.is_some()) {
                Ok(kind) => kind,
                Err(err) => return neg_errno(err),
            }
        };
        if in_kind != SpliceEndpointKind::Pipe && out_kind != SpliceEndpointKind::Pipe {
            return neg_errno(LinuxError::EINVAL);
        }
        if let Err(err) = validate_splice_output(
            &mut table,
            process,
            fd_out,
            out_kind,
            off_out,
            file_size_limit,
        ) {
            return neg_errno(err);
        }
        if let Err(err) = validate_splice_input(&mut table, process, fd_in, in_kind, off_in) {
            return neg_errno(err);
        }
        (in_kind, out_kind)
    };
    let mut copied = 0usize;
    let mut scratch = Vec::new();
    while copied < len {
        let mut chunk_len = (len - copied).min(MAX_USER_IO_CHUNK);
        if in_kind == SpliceEndpointKind::Pipe {
            let available = match pipe_endpoint(input_pipe.as_ref().unwrap()) {
                Ok(endpoint) => endpoint.available_read(),
                Err(err) => {
                    return if copied > 0 {
                        copied as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            if available == 0 {
                if copied > 0 {
                    break;
                }
                if nonblocking {
                    return neg_errno(LinuxError::EAGAIN);
                }
            }
            if available > 0 {
                chunk_len = chunk_len.min(available);
            }
        }
        if in_kind == SpliceEndpointKind::Stream {
            let readiness = match process.fds.lock().splice_stream_readiness(fd_in) {
                Ok(readiness) => readiness,
                Err(err) => {
                    return if copied > 0 {
                        copied as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            match readiness {
                SpliceStreamReadiness::Data(available) => {
                    chunk_len = chunk_len.min(available);
                }
                SpliceStreamReadiness::WouldBlock {
                    endpoint_nonblocking,
                } => {
                    if copied > 0 {
                        break;
                    }
                    if nonblocking || endpoint_nonblocking {
                        return neg_errno(LinuxError::EAGAIN);
                    }
                    if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
                        return neg_errno(LinuxError::EINTR);
                    }
                    axtask::yield_now();
                    continue;
                }
                SpliceStreamReadiness::Eof => break,
                SpliceStreamReadiness::Unknown => {}
            }
        }
        if out_kind == SpliceEndpointKind::Pipe {
            let output = output_pipe.as_ref().unwrap();
            let endpoint = match pipe_endpoint(output) {
                Ok(endpoint) => endpoint,
                Err(err) => {
                    return if copied > 0 {
                        copied as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            let available = endpoint
                .capacity()
                .saturating_sub(endpoint.available_read());
            if available == 0 {
                if copied > 0 {
                    break;
                }
                if nonblocking {
                    return neg_errno(LinuxError::EAGAIN);
                }
            }
            if available > 0 {
                chunk_len = chunk_len.min(available);
            }
        }
        if out_kind == SpliceEndpointKind::Stream {
            let output_socket = {
                let table = process.fds.lock();
                table.splice_local_socket_output(fd_out)
            };
            let output_socket = match output_socket {
                Ok(output_socket) => output_socket,
                Err(err) => {
                    return if copied > 0 {
                        copied as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            if let Some(output_socket) = output_socket {
                // Reserve the local-socket output capacity before consuming from
                // the splice input.  Otherwise another writer can fill the
                // stream buffer between an availability check and the later
                // write, losing bytes already read from a pipe/socket input.
                // The pipe input was pinned at syscall entry and the socket
                // output is cloned while the fd table is locked; the reservation
                // path then only locks endpoint buffers.  This preserves the
                // repository's normal fd-table -> endpoint lock order and avoids
                // a socket-buffer -> fd-table deadlock.
                let written = output_socket.write_from_pipe_splice_reservation(
                    process,
                    match input_pipe
                        .as_ref()
                        .ok_or(LinuxError::EBADF)
                        .and_then(pipe_endpoint)
                    {
                        Ok(endpoint) => endpoint,
                        Err(err) => {
                            return if copied > 0 {
                                copied as isize
                            } else {
                                neg_errno(err)
                            };
                        }
                    },
                    match input_pipe.as_ref() {
                        Some(pipe) => pipe.status_flags(),
                        None => return neg_errno(LinuxError::EBADF),
                    },
                    chunk_len,
                    nonblocking,
                );
                let written = match written {
                    Ok(written) => written,
                    Err(err) => {
                        return if copied > 0 {
                            copied as isize
                        } else {
                            neg_errno(err)
                        };
                    }
                };
                if in_kind == SpliceEndpointKind::RegularFile {
                    if let Some(pos) = off_in.as_mut() {
                        *pos = pos.saturating_add(written as u64);
                    } else if let Err(err) =
                        process.fds.lock().advance_file_offset_fd(fd_in, written)
                    {
                        return if copied > 0 {
                            copied as isize
                        } else {
                            neg_errno(err)
                        };
                    }
                }
                copied += written;
                if written == 0 || written < chunk_len {
                    break;
                }
                continue;
            }
        }
        if out_kind == SpliceEndpointKind::Stream {
            let available = match process.fds.lock().splice_stream_available_write(fd_out) {
                Ok(available) => available,
                Err(err) => {
                    return if copied > 0 {
                        copied as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
            if available == 0 {
                if copied > 0 {
                    break;
                }
                if nonblocking {
                    return neg_errno(LinuxError::EAGAIN);
                }
                if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
                    return neg_errno(LinuxError::EINTR);
                }
                axtask::yield_now();
                continue;
            }
            if available > 0 {
                chunk_len = chunk_len.min(available);
            }
        }

        let buf = match io_scratch_slice(&mut scratch, chunk_len) {
            Ok(buf) => buf,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        let read = if let Some(pipe) = &input_pipe {
            pipe_endpoint(pipe)
                .and_then(|endpoint| endpoint.read_partial(pipe.status_flags(), buf, nonblocking))
        } else {
            let mut table = process.fds.lock();
            splice_read_input(&mut table, process, fd_in, in_kind, off_in, buf)
        };
        let read = match read {
            Ok(0) => break,
            Ok(read) => read,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };

        let written = if let Some(pipe) = &output_pipe {
            pipe_endpoint(pipe).and_then(|endpoint| {
                endpoint.write_partial(pipe.status_flags(), &buf[..read], nonblocking)
            })
        } else {
            let mut table = process.fds.lock();
            splice_write_output(
                &mut table,
                process,
                fd_out,
                out_kind,
                off_out,
                &buf[..read],
                file_size_limit,
            )
        };
        let written = match written {
            Ok(written) => written,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };

        if in_kind == SpliceEndpointKind::RegularFile {
            if let Some(pos) = off_in.as_mut() {
                *pos = pos.saturating_add(written as u64);
            } else if let Err(err) = process.fds.lock().advance_file_offset_fd(fd_in, written) {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        }
        if out_kind == SpliceEndpointKind::RegularFile {
            if let Some(pos) = off_out.as_mut() {
                *pos = pos.saturating_add(written as u64);
            }
        }
        copied += written;
        if written == 0 || written < read {
            break;
        }
    }

    if let Some(pos) = off_in {
        let out: i64 = match pos.try_into() {
            Ok(value) => value,
            Err(_) => return neg_errno(LinuxError::EOVERFLOW),
        };
        let ret = write_user_value(process, off_in_ptr, &out);
        if ret < 0 {
            return if copied > 0 { copied as isize } else { ret };
        }
    }
    if let Some(pos) = off_out {
        let out: i64 = match pos.try_into() {
            Ok(value) => value,
            Err(_) => return neg_errno(LinuxError::EOVERFLOW),
        };
        let ret = write_user_value(process, off_out_ptr, &out);
        if ret < 0 {
            return if copied > 0 { copied as isize } else { ret };
        }
    }

    copied as isize
}

pub(super) fn sys_tee(
    process: &UserProcess,
    fd_in: usize,
    fd_out: usize,
    len: usize,
    flags: usize,
) -> isize {
    let supported_flags = (general::SPLICE_F_MOVE
        | general::SPLICE_F_NONBLOCK
        | general::SPLICE_F_MORE
        | general::SPLICE_F_GIFT) as usize;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let nonblocking = flags & general::SPLICE_F_NONBLOCK as usize != 0;
    let pipes = {
        let table = process.fds.lock();
        match (
            table.splice_pipe_input(fd_in as i32),
            table.splice_pipe_input(fd_out as i32),
        ) {
            (Ok(src), Ok(dst)) => Ok((src, dst)),
            (Err(err), _) | (_, Err(err)) => Err(err),
        }
    };
    let result = pipes.and_then(|(src, dst)| {
        pipe_endpoint(&src)?.tee_to(
            src.status_flags(),
            pipe_endpoint(&dst)?,
            dst.status_flags(),
            len,
            nonblocking,
        )
    });
    match result {
        Ok(copied) => copied as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_vmsplice(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    flags: usize,
) -> isize {
    let supported_flags = (general::SPLICE_F_MOVE
        | general::SPLICE_F_NONBLOCK
        | general::SPLICE_F_MORE
        | general::SPLICE_F_GIFT) as usize;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let nonblocking = flags & general::SPLICE_F_NONBLOCK as usize != 0;
    let fd = fd as i32;
    let pipe = match process.fds.lock().open_file_ref(fd) {
        Ok(Some(pipe)) if object_as::<PipeEndpoint>(&pipe).is_ok() => pipe,
        Ok(_) => return neg_errno(LinuxError::EBADF),
        Err(err) => return neg_errno(err),
    };
    let endpoint = match pipe_endpoint(&pipe) {
        Ok(endpoint) => endpoint,
        Err(err) => return neg_errno(err),
    };
    let direction = if endpoint.writable() {
        VmspliceDirection::ToPipe
    } else if endpoint.readable() {
        VmspliceDirection::FromPipe
    } else {
        return neg_errno(LinuxError::EBADF);
    };
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };

    let mut total = 0isize;
    let mut scratch = Vec::new();
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        match direction {
            VmspliceDirection::ToPipe => {
                if let Err(err) = validate_user_read(process, base, remaining) {
                    return if total > 0 { total } else { neg_errno(err) };
                }
                while remaining > 0 {
                    let len = remaining.min(MAX_USER_IO_CHUNK);
                    let src = match io_scratch_slice(&mut scratch, len) {
                        Ok(src) => src,
                        Err(err) => return if total > 0 { total } else { neg_errno(err) },
                    };
                    if let Err(err) = read_user_bytes_into(process, base, &mut *src) {
                        return if total > 0 { total } else { neg_errno(err) };
                    }
                    let written =
                        match endpoint.write_partial(pipe.status_flags(), src, nonblocking) {
                            Ok(written) => written,
                            Err(err) => return if total > 0 { total } else { neg_errno(err) },
                        };
                    total += written as isize;
                    if written == 0 {
                        return total;
                    }
                    base = base.saturating_add(written);
                    remaining -= written;
                    if written < len || remaining > 0 {
                        return total;
                    }
                }
            }
            VmspliceDirection::FromPipe => {
                if let Err(err) = validate_user_write(process, base, remaining) {
                    return if total > 0 { total } else { neg_errno(err) };
                }
                while remaining > 0 {
                    let len = remaining.min(MAX_USER_IO_CHUNK);
                    let bytes = match io_scratch_slice(&mut scratch, len) {
                        Ok(bytes) => bytes,
                        Err(err) => return if total > 0 { total } else { neg_errno(err) },
                    };
                    let read = match endpoint.read_partial(pipe.status_flags(), bytes, nonblocking)
                    {
                        Ok(read) => read,
                        Err(err) => return if total > 0 { total } else { neg_errno(err) },
                    };
                    if read > len {
                        return if total > 0 {
                            total
                        } else {
                            neg_errno(LinuxError::EINVAL)
                        };
                    }
                    if let Err(err) = write_user_bytes(process, base, &bytes[..read]) {
                        return if total > 0 { total } else { neg_errno(err) };
                    }
                    total += read as isize;
                    if read == 0 {
                        return total;
                    }
                    base = base.saturating_add(read);
                    remaining -= read;
                    if read < len || remaining > 0 {
                        return total;
                    }
                }
            }
        }
    }

    total
}

fn splice_input_kind(
    table: &FdTable,
    fd: i32,
    has_offset: bool,
) -> Result<SpliceEndpointKind, LinuxError> {
    match table.entry(fd)? {
        FdEntry::File(file) => {
            if !file_is_readable(file.status_flags) {
                return Err(LinuxError::EBADF);
            }
            Ok(SpliceEndpointKind::RegularFile)
        }
        FdEntry::Pipe(pipe) => {
            if has_offset {
                return Err(LinuxError::ESPIPE);
            }
            if !pipe_endpoint(pipe)?.readable() {
                return Err(LinuxError::EBADF);
            }
            Ok(SpliceEndpointKind::Pipe)
        }
        FdEntry::LocalSocket(_) => {
            if has_offset {
                Err(LinuxError::ESPIPE)
            } else {
                Ok(SpliceEndpointKind::Stream)
            }
        }
        FdEntry::ProcSysFile(file) => {
            if has_offset {
                return Err(LinuxError::ESPIPE);
            }
            if !file_is_readable(file.status_flags()) {
                return Err(LinuxError::EBADF);
            }
            Ok(SpliceEndpointKind::Stream)
        }
        FdEntry::Socket(_) => Err(LinuxError::EINVAL),
        FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
            Err(LinuxError::EINVAL)
        }
        _ => Err(LinuxError::EINVAL),
    }
}

fn splice_output_kind(
    table: &FdTable,
    fd: i32,
    has_offset: bool,
) -> Result<SpliceEndpointKind, LinuxError> {
    match table.entry(fd)? {
        FdEntry::File(file) => {
            if !file_is_writable(file.status_flags) {
                return Err(LinuxError::EBADF);
            }
            if file.status_flags & general::O_APPEND != 0 {
                return Err(LinuxError::EINVAL);
            }
            Ok(SpliceEndpointKind::RegularFile)
        }
        FdEntry::Pipe(pipe) => {
            if has_offset {
                return Err(LinuxError::ESPIPE);
            }
            if !pipe_endpoint(pipe)?.writable() {
                return Err(LinuxError::EBADF);
            }
            Ok(SpliceEndpointKind::Pipe)
        }
        FdEntry::LocalSocket(_) => {
            if has_offset {
                Err(LinuxError::ESPIPE)
            } else {
                Ok(SpliceEndpointKind::Stream)
            }
        }
        FdEntry::ProcSysFile(file) => {
            if has_offset {
                return Err(LinuxError::ESPIPE);
            }
            if !file_is_writable(file.status_flags()) {
                return Err(LinuxError::EBADF);
            }
            Ok(SpliceEndpointKind::Stream)
        }
        FdEntry::Socket(_) => Err(LinuxError::EINVAL),
        FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
            Err(LinuxError::EINVAL)
        }
        _ => Err(LinuxError::EINVAL),
    }
}

fn validate_splice_input(
    table: &mut FdTable,
    process: &UserProcess,
    fd: i32,
    kind: SpliceEndpointKind,
    offset: Option<u64>,
) -> Result<(), LinuxError> {
    match kind {
        SpliceEndpointKind::RegularFile => match offset {
            Some(pos) => table
                .read_file_at_into_fd(process, fd, pos, &mut [])
                .map(|_| ()),
            None => table
                .read_file_at_current_offset_into_fd(process, fd, &mut [])
                .map(|_| ()),
        },
        SpliceEndpointKind::Pipe => Ok(()),
        SpliceEndpointKind::Stream => table.read(process, fd, &mut []).map(|_| ()),
    }
}

fn validate_splice_output(
    table: &mut FdTable,
    process: &UserProcess,
    fd: i32,
    kind: SpliceEndpointKind,
    offset: Option<u64>,
    file_size_limit: u64,
) -> Result<(), LinuxError> {
    match kind {
        SpliceEndpointKind::RegularFile => match offset {
            Some(pos) => table.write_file_at(process, fd, pos, &[], Some(file_size_limit)),
            None => table.write(process, fd, &[], Some(file_size_limit)),
        }
        .map(|_| ()),
        SpliceEndpointKind::Pipe => Ok(()),
        SpliceEndpointKind::Stream => table
            .write(process, fd, &[], Some(file_size_limit))
            .map(|_| ()),
    }
}

fn splice_read_input(
    table: &mut FdTable,
    process: &UserProcess,
    fd: i32,
    kind: SpliceEndpointKind,
    offset: Option<u64>,
    dst: &mut [u8],
) -> Result<usize, LinuxError> {
    match kind {
        SpliceEndpointKind::RegularFile => match offset {
            Some(pos) => table.read_file_at_into_fd(process, fd, pos, dst),
            None => table
                .read_file_at_current_offset_into_fd(process, fd, dst)
                .map(|(_, read)| read),
        },
        SpliceEndpointKind::Pipe => table.read(process, fd, dst),
        SpliceEndpointKind::Stream => match table.entry_mut(fd)? {
            FdEntry::LocalSocket(socket) => socket.read_partial(dst),
            FdEntry::ProcSysFile(file) => file.read(dst),
            _ => Err(LinuxError::EINVAL),
        },
    }
}

fn splice_write_output(
    table: &mut FdTable,
    process: &UserProcess,
    fd: i32,
    kind: SpliceEndpointKind,
    offset: Option<u64>,
    src: &[u8],
    file_size_limit: u64,
) -> Result<usize, LinuxError> {
    match kind {
        SpliceEndpointKind::RegularFile => match offset {
            Some(pos) => table.write_file_at(process, fd, pos, src, Some(file_size_limit)),
            None => table.write(process, fd, src, Some(file_size_limit)),
        },
        SpliceEndpointKind::Pipe => table.write(process, fd, src, Some(file_size_limit)),
        SpliceEndpointKind::Stream => match table.entry_mut(fd)? {
            FdEntry::LocalSocket(socket) => socket.write_partial(src),
            FdEntry::ProcSysFile(entry) => entry.write(src),
            _ => Err(LinuxError::EINVAL),
        },
    }
}

pub(super) fn sys_readahead(
    process: &UserProcess,
    fd: usize,
    offset: usize,
    _count: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    match process
        .fds
        .lock()
        .read_file_at_into_fd(process, fd as i32, offset as u64, &mut [])
    {
        Ok(_) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_copy_file_range(
    process: &UserProcess,
    fd_in: usize,
    off_in_ptr: usize,
    fd_out: usize,
    off_out_ptr: usize,
    len: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let mut off_in = match read_copy_file_range_offset(process, off_in_ptr) {
        Ok(offset) => offset,
        Err(err) => return neg_errno(err),
    };
    let mut off_out = match read_copy_file_range_offset(process, off_out_ptr) {
        Ok(offset) => offset,
        Err(err) => return neg_errno(err),
    };
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();

    {
        let mut table = process.fds.lock();
        let input_check = match off_in {
            Some(pos) => table.read_file_at_into_fd(process, fd_in as i32, pos, &mut []),
            None => table
                .read_file_at_current_offset_into_fd(process, fd_in as i32, &mut [])
                .map(|(_, read)| read),
        };
        if let Err(err) = input_check {
            return neg_errno(err);
        }
        let output_check = match off_out {
            Some(pos) => {
                table.write_file_at(process, fd_out as i32, pos, &[], Some(file_size_limit))
            }
            None => table.write(process, fd_out as i32, &[], Some(file_size_limit)),
        };
        if let Err(err) = output_check {
            return neg_errno(err);
        }
    }

    let mut copied = 0usize;
    let mut scratch = Vec::new();
    while copied < len {
        let chunk_len = (len - copied).min(MAX_USER_IO_CHUNK);
        let buf = match io_scratch_slice(&mut scratch, chunk_len) {
            Ok(buf) => buf,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        let read = {
            let mut table = process.fds.lock();
            match off_in {
                Some(pos) => table.read_file_at_into_fd(process, fd_in as i32, pos, &mut *buf),
                None => table
                    .read_file_at_current_offset_into_fd(process, fd_in as i32, &mut *buf)
                    .map(|(_, read)| read),
            }
        };
        let read = match read {
            Ok(0) => break,
            Ok(read) => read,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };

        let written = {
            let mut table = process.fds.lock();
            match off_out {
                Some(pos) => table.write_file_at(
                    process,
                    fd_out as i32,
                    pos,
                    &buf[..read],
                    Some(file_size_limit),
                ),
                None => table.write(process, fd_out as i32, &buf[..read], Some(file_size_limit)),
            }
        };
        let written = match written {
            Ok(written) => written,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };

        if let Some(pos) = off_in.as_mut() {
            *pos = pos.saturating_add(written as u64);
        } else if let Err(err) = process
            .fds
            .lock()
            .advance_file_offset_fd(fd_in as i32, written)
        {
            return if copied > 0 {
                copied as isize
            } else {
                neg_errno(err)
            };
        }
        if let Some(pos) = off_out.as_mut() {
            *pos = pos.saturating_add(written as u64);
        }
        copied += written;
        if written == 0 || written < read {
            break;
        }
    }

    if let Some(pos) = off_in {
        let out: i64 = match pos.try_into() {
            Ok(value) => value,
            Err(_) => return neg_errno(LinuxError::EOVERFLOW),
        };
        let ret = write_user_value(process, off_in_ptr, &out);
        if ret < 0 {
            return if copied > 0 { copied as isize } else { ret };
        }
    }
    if let Some(pos) = off_out {
        let out: i64 = match pos.try_into() {
            Ok(value) => value,
            Err(_) => return neg_errno(LinuxError::EOVERFLOW),
        };
        let ret = write_user_value(process, off_out_ptr, &out);
        if ret < 0 {
            return if copied > 0 { copied as isize } else { ret };
        }
    }

    copied as isize
}

fn read_copy_file_range_offset(
    process: &UserProcess,
    ptr: usize,
) -> Result<Option<u64>, LinuxError> {
    if ptr == 0 {
        return Ok(None);
    }
    validate_user_write(process, ptr, size_of::<i64>())?;
    match read_user_value::<i64>(process, ptr) {
        Ok(value) if value >= 0 => Ok(Some(value as u64)),
        Ok(_) => Err(LinuxError::EINVAL),
        Err(err) => Err(err),
    }
}

pub(super) fn sys_getdents64(process: &UserProcess, fd: usize, dirp: usize, count: usize) -> isize {
    if let Err(err) = validate_user_write(process, dirp, count) {
        return neg_errno(err);
    }
    let bytes = match process.fds.lock().getdents64(process, fd as i32, count) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = write_user_bytes(process, dirp, &bytes) {
        return neg_errno(err);
    }
    bytes.len() as isize
}

pub(super) fn sys_lseek(process: &UserProcess, fd: usize, offset: usize, whence: usize) -> isize {
    match process
        .fds
        .lock()
        .lseek(process, fd as i32, offset as isize as i64, whence as u32)
    {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_dup(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().dup(fd as i32) {
        Ok(new_fd) => new_fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_dup3(process: &UserProcess, oldfd: usize, newfd: usize, flags: usize) -> isize {
    let result = {
        process
            .fds
            .lock()
            .dup3(process, oldfd as i32, newfd as i32, flags as u32)
    };
    match result {
        Ok((fd, closed)) => {
            drop(closed);
            fd as isize
        }
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fcntl(process: &UserProcess, fd: usize, cmd: usize, arg: usize) -> isize {
    let cmd = cmd as u32;
    let fd = fd as i32;
    let result = match cmd {
        cmd @ (general::F_SETOWN | general::F_GETOWN | general::F_SETSIG | general::F_GETSIG)
        | cmd @ (15 | 16) => fcntl_pipe_async_owner(process, fd, cmd, arg),
        general::F_GETLK => fcntl_getlk_record(process, fd, arg, false),
        general::F_SETLK => fcntl_setlk_record(process, fd, arg, false, false),
        general::F_SETLKW => fcntl_setlk_record(process, fd, arg, true, false),
        general::F_OFD_GETLK => fcntl_getlk_record(process, fd, arg, true),
        general::F_OFD_SETLK => fcntl_setlk_record(process, fd, arg, false, true),
        general::F_OFD_SETLKW => fcntl_setlk_record(process, fd, arg, true, true),
        _ => process.fds.lock().fcntl(process, fd, cmd, arg),
    };
    match result {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

fn fcntl_pipe_async_owner(
    process: &UserProcess,
    fd: i32,
    cmd: u32,
    arg: usize,
) -> Result<i32, LinuxError> {
    let pipe = {
        let table = process.fds.lock();
        match table.entry(fd)? {
            FdEntry::Pipe(pipe) => pipe.clone(),
            _ => return Err(LinuxError::EINVAL),
        }
    };
    pipe_endpoint(&pipe)?
        .fcntl_async_owner(process, cmd, arg)?
        .ok_or(LinuxError::EINVAL)
}

fn fcntl_record_lock_fd_exists(process: &UserProcess, fd: i32) -> Result<(), LinuxError> {
    let table = process.fds.lock();
    table.entry(fd).map(|_| ())
}

fn fcntl_record_lock_file(process: &UserProcess, fd: i32) -> Result<FileEntry, LinuxError> {
    let table = process.fds.lock();
    match table.entry(fd)? {
        FdEntry::File(file) => Ok(file.clone()),
        _ => Err(LinuxError::EBADF),
    }
}

fn validate_record_lock_whence(lock: &general::flock) -> Result<(), LinuxError> {
    match lock.l_whence as u32 {
        general::SEEK_SET | general::SEEK_CUR | general::SEEK_END => Ok(()),
        _ => Err(LinuxError::EINVAL),
    }
}

fn fcntl_record_lock_request_for_file(
    process: &UserProcess,
    file: &FileEntry,
    lock: &general::flock,
    ofd: bool,
    require_access: bool,
) -> Result<(u64, PosixRecordLock), LinuxError> {
    if require_access && !record_lock_access_allowed(file, lock.l_type as u32) {
        return Err(LinuxError::EBADF);
    }
    let request = if ofd {
        normalize_ofd_record_lock(file, lock)?
    } else {
        normalize_record_lock(file, process, lock)?
    };
    Ok((record_lock_key(file), request))
}

fn fcntl_getlk_record(
    process: &UserProcess,
    fd: i32,
    arg: usize,
    ofd: bool,
) -> Result<i32, LinuxError> {
    // Linux validates descriptor existence before inspecting the flock payload,
    // but valid non-file descriptors still report copy-in/flock-shape errors
    // before file-type/access errors.  Keep that errno priority while still
    // dropping the fd-table lock before potentially blocking record-lock work.
    fcntl_record_lock_fd_exists(process, fd)?;
    let mut lock: general::flock = read_user_value(process, arg)?;
    validate_flock(&lock)?;
    validate_record_lock_whence(&lock)?;
    let file = fcntl_record_lock_file(process, fd)?;
    let (key, request) = fcntl_record_lock_request_for_file(process, &file, &lock, ofd, false)?;
    if let Some(conflict) = first_record_lock_conflict(key, &request) {
        lock.l_type = conflict.typ;
        lock.l_whence = general::SEEK_SET as _;
        lock.l_start = conflict.start as _;
        lock.l_len = conflict.len as _;
        lock.l_pid = if conflict.owner_id > 0 && conflict.owner_id <= i32::MAX as i64 {
            conflict.owner_id as _
        } else {
            -1
        };
    } else {
        lock.l_type = general::F_UNLCK as _;
    }
    if write_user_value(process, arg, &lock) == 0 {
        Ok(0)
    } else {
        Err(LinuxError::EFAULT)
    }
}

fn fcntl_setlk_record(
    process: &UserProcess,
    fd: i32,
    arg: usize,
    wait: bool,
    ofd: bool,
) -> Result<i32, LinuxError> {
    // Match Linux errno priority: bad descriptors fail before bad user flock
    // pointers, while valid descriptors report copy-in/flock-shape errors before
    // file-type/access errors.  The actual wait still happens after releasing
    // fd-table lock.
    fcntl_record_lock_fd_exists(process, fd)?;
    let lock: general::flock = read_user_value(process, arg)?;
    validate_flock(&lock)?;
    validate_record_lock_whence(&lock)?;
    let file = fcntl_record_lock_file(process, fd)?;
    let (key, request) = fcntl_record_lock_request_for_file(process, &file, &lock, ofd, true)?;
    // F_SETLKW/F_OFD_SETLKW can legitimately block.  Do not hold the fd-table
    // lock while waiting: the current lock owner must be able to re-enter the fd
    // table to write data or issue the unlock that makes this request runnable.
    apply_record_lock(process, key, request, wait)?;
    Ok(0)
}

pub(super) fn sys_flock(process: &UserProcess, fd: usize, operation: usize) -> isize {
    match process.fds.lock().flock(fd as i32, operation as u32) {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fsync(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().entry(fd as i32) {
        Ok(FdEntry::File(file)) => match file.file.flush().map_err(LinuxError::from) {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        },
        Ok(FdEntry::Memfd(_)) => {
            // memfd is RAM-backed state in this compatibility layer.  There is
            // no lower storage device to flush, so all data is already durable
            // for the lifetime of the memfd object.
            0
        }
        Ok(
            FdEntry::Stdin(_)
            | FdEntry::Stdout(_)
            | FdEntry::Stderr(_)
            | FdEntry::DevNull
            | FdEntry::DevZero(_)
            | FdEntry::DevRandom(_)
            | FdEntry::DevCpuDmaLatency(_)
            | FdEntry::BlockDevice(_)
            | FdEntry::Rtc
            | FdEntry::Directory(_)
            | FdEntry::ProcFdDir(_)
            | FdEntry::SyntheticDir(_)
            | FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_)
            | FdEntry::Pipe(_)
            | FdEntry::Socket(_)
            | FdEntry::LocalSocket(_)
            | FdEntry::EventFd(_)
            | FdEntry::Inotify(_)
            | FdEntry::Epoll(_)
            | FdEntry::TimerFd(_)
            | FdEntry::SignalFd(_)
            | FdEntry::PidFd(_)
            | FdEntry::PosixMq(_)
            | FdEntry::ProcMqQueuesMax(_)
            | FdEntry::ProcSysFile(_),
        ) => neg_errno(LinuxError::EINVAL),
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_renameat2(
    process: &UserProcess,
    olddirfd: usize,
    oldpath: usize,
    newdirfd: usize,
    newpath: usize,
    flags: usize,
) -> isize {
    if flags > u32::MAX as usize {
        return neg_errno(LinuxError::EINVAL);
    }
    let flags = flags as u32;
    let supported_flags =
        general::RENAME_NOREPLACE | general::RENAME_EXCHANGE | general::RENAME_WHITEOUT;
    if flags & !supported_flags != 0
        || flags & general::RENAME_EXCHANGE != 0
            && flags & (general::RENAME_NOREPLACE | general::RENAME_WHITEOUT) != 0
        || flags & general::RENAME_WHITEOUT != 0
    {
        return neg_errno(LinuxError::EINVAL);
    }
    let old_path = match read_cstr(process, oldpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let new_path = match read_cstr(process, newpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if old_path.is_empty() || new_path.is_empty() {
        return neg_errno(LinuxError::ENOENT);
    }
    if path_exceeds_linux_limits(old_path.as_str()) || path_exceeds_linux_limits(new_path.as_str())
    {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let (old_abs_path, new_abs_path) = {
        let table = process.fds.lock();
        let old_abs = match resolve_dirfd_path(process, &table, olddirfd as i32, old_path.as_str())
        {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        let new_abs = match resolve_dirfd_path(process, &table, newdirfd as i32, new_path.as_str())
        {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        let old_abs = match process.resolve_parent_symlinks(old_abs.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        let new_abs = match process.resolve_parent_symlinks(new_abs.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        (old_abs, new_abs)
    };
    renameat2_paths(process, old_abs_path, new_abs_path, flags)
        .map_or_else(|err| neg_errno(err), |_| 0)
}

fn renameat2_paths(
    process: &UserProcess,
    old_abs_path: String,
    new_abs_path: String,
    flags: u32,
) -> Result<(), LinuxError> {
    let old_st = rename_target_stat(process, old_abs_path.as_str())?;
    if old_abs_path == new_abs_path {
        return Ok(());
    }
    let new_st = match rename_target_stat(process, new_abs_path.as_str()) {
        Ok(st) => Some(st),
        Err(LinuxError::ENOENT) => None,
        Err(err) => return Err(err),
    };

    if flags & general::RENAME_NOREPLACE != 0 && new_st.is_some() {
        return Err(LinuxError::EEXIST);
    }
    if new_st
        .as_ref()
        .is_some_and(|st| old_st.st_dev == st.st_dev && old_st.st_ino == st.st_ino)
    {
        return Ok(());
    }
    if flags & general::RENAME_EXCHANGE != 0 {
        let new_st = new_st.ok_or(LinuxError::ENOENT)?;
        return rename_exchange(
            process,
            old_abs_path.as_str(),
            new_abs_path.as_str(),
            &old_st,
            &new_st,
        );
    }

    let old_parent_st = check_parent_write_search_permission(process, old_abs_path.as_str())?;
    let new_parent_st = check_parent_write_search_permission(process, new_abs_path.as_str())?;
    check_sticky_parent_permission(process, &old_parent_st, &old_st)?;
    if let Some(st) = new_st.as_ref() {
        check_sticky_parent_permission(process, &new_parent_st, st)?;
    }
    if process.paths_cross_mount(old_abs_path.as_str(), new_abs_path.as_str()) {
        return Err(LinuxError::EXDEV);
    }

    if let Some(backing_path) = process.path_hardlink_backing(old_abs_path.as_str()) {
        if backing_path != old_abs_path {
            if new_st.is_some() {
                return Err(LinuxError::EEXIST);
            }
            process.remove_path_hardlink(old_abs_path.as_str());
            process.remove_path_inode(old_abs_path.as_str());
            process.set_path_hardlink(backing_path.as_str(), new_abs_path, old_st.st_ino as u64);
            invalidate_exec_image_cache(old_abs_path.as_str());
            return Ok(());
        }
    }

    axfs::api::rename(old_abs_path.as_str(), new_abs_path.as_str()).map_err(LinuxError::from)?;
    invalidate_exec_image_cache(old_abs_path.as_str());
    invalidate_exec_image_cache(new_abs_path.as_str());
    process.move_path_metadata(old_abs_path.as_str(), new_abs_path);
    Ok(())
}

fn rename_exchange(
    process: &UserProcess,
    old_abs_path: &str,
    new_abs_path: &str,
    old_st: &general::stat,
    new_st: &general::stat,
) -> Result<(), LinuxError> {
    let old_parent_st = check_parent_write_search_permission(process, old_abs_path)?;
    let new_parent_st = check_parent_write_search_permission(process, new_abs_path)?;
    check_sticky_parent_permission(process, &old_parent_st, old_st)?;
    check_sticky_parent_permission(process, &new_parent_st, new_st)?;
    if process.paths_cross_mount(old_abs_path, new_abs_path) {
        return Err(LinuxError::EXDEV);
    }
    let tmp_path = rename_exchange_tmp_path(process, old_abs_path)?;
    axfs::api::rename(old_abs_path, tmp_path.as_str()).map_err(LinuxError::from)?;
    if let Err(err) = axfs::api::rename(new_abs_path, old_abs_path).map_err(LinuxError::from) {
        let _ = axfs::api::rename(tmp_path.as_str(), old_abs_path);
        return Err(err);
    }
    if let Err(err) = axfs::api::rename(tmp_path.as_str(), new_abs_path).map_err(LinuxError::from) {
        let _ = axfs::api::rename(old_abs_path, new_abs_path);
        let _ = axfs::api::rename(tmp_path.as_str(), old_abs_path);
        return Err(err);
    }
    invalidate_exec_image_cache(old_abs_path);
    invalidate_exec_image_cache(new_abs_path);
    invalidate_exec_image_cache(tmp_path.as_str());
    process.move_path_metadata(old_abs_path, tmp_path.clone());
    process.move_path_metadata(new_abs_path, old_abs_path.to_string());
    process.move_path_metadata(tmp_path.as_str(), new_abs_path.to_string());
    Ok(())
}

fn rename_exchange_tmp_path(
    process: &UserProcess,
    old_abs_path: &str,
) -> Result<String, LinuxError> {
    let parent = parent_path(old_abs_path);
    for attempt in 0..64 {
        let candidate = if parent == "/" {
            format!("/.arceos-rename-exchange-{}-{}", process.pid(), attempt)
        } else {
            format!(
                "{}/.arceos-rename-exchange-{}-{}",
                parent,
                process.pid(),
                attempt
            )
        };
        if path_exceeds_linux_limits(candidate.as_str()) {
            return Err(LinuxError::ENAMETOOLONG);
        }
        if rename_target_stat(process, candidate.as_str()).is_err()
            && process.path_hardlink_backing(candidate.as_str()).is_none()
        {
            return Ok(candidate);
        }
    }
    Err(LinuxError::EEXIST)
}

fn rename_target_stat(process: &UserProcess, path: &str) -> Result<general::stat, LinuxError> {
    if let Some(st) = process.path_symlink_stat(path) {
        Ok(apply_recorded_path_metadata(process, path, st))
    } else {
        stat_absolute_path(process, path)
    }
}

pub(super) fn sys_getcwd(process: &UserProcess, buf: usize, size: usize) -> isize {
    let cwd = match visible_cwd(process) {
        Ok(cwd) => cwd,
        Err(err) => return neg_errno(err),
    };
    let mut bytes = cwd.into_bytes();
    bytes.push(0);
    if bytes.len() > size {
        return neg_errno(LinuxError::ERANGE);
    }
    write_user_bytes(process, buf, &bytes)
        .map_or_else(|err| neg_errno(err), |_| bytes.len() as isize)
}

fn visible_cwd(process: &UserProcess) -> Result<String, LinuxError> {
    let cwd = process.cwd();
    let root = process.fs_root();
    if root == "/" {
        return Ok(cwd);
    }
    if cwd == root {
        return Ok("/".into());
    }
    let mut prefix = root;
    if !prefix.ends_with('/') {
        prefix.push('/');
    }
    cwd.strip_prefix(prefix.as_str())
        .map(|tail| {
            let mut visible = String::from("/");
            visible.push_str(tail);
            visible
        })
        .ok_or(LinuxError::ENOENT)
}

pub(super) fn sys_chdir(process: &UserProcess, pathname: usize) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path_exceeds_linux_limits(path.as_str()) {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let visible_path = {
        let table = process.fds.lock();
        match table.resolve_path(process, general::AT_FDCWD, path.as_str()) {
            Ok(path) => {
                let resolved_path = match process.resolve_path_symlink(path.as_str()) {
                    Ok(Some(target)) => target,
                    Ok(None) => path,
                    Err(err) => return neg_errno(err),
                };
                let stat = match stat_absolute_path(process, resolved_path.as_str()) {
                    Ok(stat) => stat,
                    Err(err) => return neg_errno(err),
                };
                if stat.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
                    return neg_errno(LinuxError::ENOTDIR);
                }
                let uid = process.fs_uid();
                let gid = process.fs_gid();
                let parents_searchable = match parent_dirs_searchable_absolute(
                    process,
                    resolved_path.as_str(),
                    uid,
                    gid,
                ) {
                    Ok(searchable) => searchable,
                    Err(err) => return neg_errno(err),
                };
                if uid != 0
                    && (!parents_searchable || !access_allowed(&stat, ACCESS_X_OK, uid, gid))
                {
                    return neg_errno(LinuxError::EACCES);
                }
                resolved_path
            }
            Err(err) => return neg_errno(err),
        }
    };
    let host_path = process.translate_mount_path(visible_path.as_str());
    if let Err(err) = open_dir_entry(host_path.as_str()) {
        return neg_errno(err);
    }
    process.set_cwd(visible_path);
    0
}

fn can_chroot(process: &UserProcess) -> bool {
    process.uid() == 0
        && general::CAP_SYS_CHROOT <= general::CAP_LAST_CAP
        && process.cap_effective() & (1u64 << general::CAP_SYS_CHROOT) != 0
}

pub(super) fn sys_chroot(process: &UserProcess, pathname: usize) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path_exceeds_linux_limits(path.as_str()) {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }

    let new_root = {
        let table = process.fds.lock();
        let resolved_path = match table.resolve_path(process, general::AT_FDCWD, path.as_str()) {
            Ok(path) => match process.resolve_path_symlink(path.as_str()) {
                Ok(Some(target)) => target,
                Ok(None) => path,
                Err(err) => return neg_errno(err),
            },
            Err(err) => return neg_errno(err),
        };
        if path_exceeds_linux_limits(resolved_path.as_str()) {
            return neg_errno(LinuxError::ENAMETOOLONG);
        }
        let stat = match stat_absolute_path(process, resolved_path.as_str()) {
            Ok(stat) => stat,
            Err(err) => return neg_errno(err),
        };
        if stat.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
            return neg_errno(LinuxError::ENOTDIR);
        }
        let uid = process.fs_uid();
        let gid = process.fs_gid();
        let parents_searchable =
            match parent_dirs_searchable_absolute(process, resolved_path.as_str(), uid, gid) {
                Ok(searchable) => searchable,
                Err(err) => return neg_errno(err),
            };
        if uid != 0 && (!parents_searchable || !access_allowed(&stat, ACCESS_X_OK, uid, gid)) {
            return neg_errno(LinuxError::EACCES);
        }
        resolved_path
    };

    if !can_chroot(process) {
        return neg_errno(LinuxError::EPERM);
    }

    process.set_fs_root(new_root);
    0
}

pub(super) fn sys_mkdirat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .mkdirat(process, dirfd as i32, path.as_str(), mode as u32)
    {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_mknodat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
    dev: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process.fds.lock().mknodat(
        process,
        dirfd as i32,
        path.as_str(),
        mode as u32,
        dev as u64,
    ) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_unlinkat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    flags: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .unlinkat(process, dirfd as i32, path.as_str(), flags as u32)
    {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_linkat(
    process: &UserProcess,
    olddirfd: usize,
    oldpath: usize,
    newdirfd: usize,
    newpath: usize,
    flags: usize,
) -> isize {
    let supported_flags = general::AT_SYMLINK_FOLLOW;
    if flags as u32 & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let old_path = match read_cstr(process, oldpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let new_path = match read_cstr(process, newpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process.fds.lock().linkat(
        process,
        olddirfd as i32,
        old_path.as_str(),
        newdirfd as i32,
        new_path.as_str(),
        flags as u32,
    ) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fchdir(process: &UserProcess, fd: usize) -> isize {
    let new_cwd = {
        let mut table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::Directory(dir)) => {
                let uid = process.fs_uid();
                let gid = process.fs_gid();
                let path = dir.path.clone();
                let stat = apply_recorded_path_metadata(
                    process,
                    path.as_str(),
                    file_attr_to_stat(&dir.attr, Some(path.as_str())),
                );
                let parents_searchable =
                    match table.parent_dirs_searchable(process, path.as_str(), uid, gid) {
                        Ok(searchable) => searchable,
                        Err(err) => return neg_errno(err),
                    };
                if uid != 0
                    && (!parents_searchable || !access_allowed(&stat, ACCESS_X_OK, uid, gid))
                {
                    return neg_errno(LinuxError::EACCES);
                }
                path
            }
            Ok(_) => return neg_errno(LinuxError::ENOTDIR),
            Err(err) => return neg_errno(err),
        }
    };
    process.set_cwd(new_cwd);
    0
}

pub(super) fn sys_ioctl(process: &UserProcess, fd: usize, req: usize, arg: usize) -> isize {
    const BLKGETSIZE64: u32 = 0x8008_1272;
    const FIONREAD: u32 = 0x541b;
    const SIOCATMARK: u32 = 0x8905;
    const SIOCGIFCONF: u32 = 0x8912;
    const SIOCGIFFLAGS: u32 = 0x8913;
    const SIOCSIFFLAGS: u32 = 0x8914;
    match req as u32 {
        SIOCATMARK | SIOCGIFCONF | SIOCGIFFLAGS | SIOCSIFFLAGS => {
            return socket_ioctl(process, fd, req as u32, arg);
        }
        _ => {}
    }
    if req as u32 == BLKGETSIZE64 && process.fds.lock().is_block_device(fd as i32) {
        let size: u64 = SYNTHETIC_BLOCK_DEVICE_SIZE;
        return write_user_value(process, arg, &size);
    }
    if req as u32 == FIONREAD {
        let available = match process.fds.lock().pipe_available_read(fd as i32) {
            Ok(available) => available as i32,
            Err(err) => return neg_errno(err),
        };
        return write_user_value(process, arg, &available);
    }
    if req as u32 == RTC_RD_TIME && process.fds.lock().is_rtc(fd as i32) {
        let rtc = rtc_time_from_wall_time();
        return write_user_value(process, arg, &rtc);
    }
    if req as u32 == ioctl::TIOCGWINSZ {
        if process.fds.lock().is_stdio(fd as i32) {
            return write_default_winsize(process, arg);
        }
    }
    if req as u32 == ioctl::FS_IOC_GETFLAGS || req as u32 == ioctl::FS_IOC_SETFLAGS {
        let path = {
            let table = process.fds.lock();
            match table.entry(fd as i32) {
                Ok(FdEntry::DevCpuDmaLatency(_)) => return neg_errno(LinuxError::ENOTTY),
                Ok(FdEntry::Path(path)) if path.path == "/dev/cpu_dma_latency" => {
                    return neg_errno(LinuxError::ENOTTY);
                }
                Ok(entry) => fd_entry_path(entry).map(ToString::to_string),
                Err(err) => return neg_errno(err),
            }
        };
        let Some(path) = path else {
            return neg_errno(LinuxError::ENOTTY);
        };
        if req as u32 == ioctl::FS_IOC_GETFLAGS {
            let flags = process.path_inode_flags(path.as_str());
            return write_user_value(process, arg, &flags);
        }
        if process.path_on_readonly_mount(path.as_str()) {
            return neg_errno(LinuxError::EROFS);
        }
        let flags: u32 = match read_user_value(process, arg) {
            Ok(flags) => flags,
            Err(err) => return neg_errno(err),
        };
        process.set_path_inode_flags(path, flags);
        return 0;
    }
    neg_errno(LinuxError::ENOTTY)
}

enum IoctlSocketKind {
    Inet(u32),
    Local,
    Other,
}

fn ioctl_socket_kind(process: &UserProcess, fd: usize) -> Result<IoctlSocketKind, LinuxError> {
    let table = process.fds.lock();
    match table.entry(fd as i32) {
        Ok(FdEntry::Socket(socket)) => Ok(IoctlSocketKind::Inet(socket.socktype as u32)),
        Ok(FdEntry::LocalSocket(_)) => Ok(IoctlSocketKind::Local),
        Ok(_) => Ok(IoctlSocketKind::Other),
        Err(err) => Err(err),
    }
}

fn socket_ioctl(process: &UserProcess, fd: usize, req: u32, arg: usize) -> isize {
    const SIOCATMARK: u32 = 0x8905;
    const SIOCGIFCONF: u32 = 0x8912;
    const SIOCGIFFLAGS: u32 = 0x8913;
    const SIOCSIFFLAGS: u32 = 0x8914;
    let kind = match ioctl_socket_kind(process, fd) {
        Ok(kind) => kind,
        Err(err) => return neg_errno(err),
    };
    if matches!(kind, IoctlSocketKind::Other) {
        return neg_errno(LinuxError::ENOTTY);
    }

    match req {
        SIOCATMARK => socket_ioctl_atmark(process, kind, arg),
        SIOCGIFCONF => socket_ioctl_get_ifconf(process, arg),
        SIOCGIFFLAGS => socket_ioctl_get_ifflags(process, arg),
        SIOCSIFFLAGS => socket_ioctl_set_ifflags(process, arg),
        _ => neg_errno(LinuxError::ENOTTY),
    }
}

fn socket_ioctl_atmark(process: &UserProcess, kind: IoctlSocketKind, arg: usize) -> isize {
    const SOCK_DGRAM_KIND: u32 = 2;
    if arg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    if matches!(kind, IoctlSocketKind::Inet(socktype) if socktype == SOCK_DGRAM_KIND) {
        return neg_errno(LinuxError::ENOTTY);
    }
    let value: i32 = 0;
    write_user_value(process, arg, &value)
}

fn socket_ioctl_get_ifconf(process: &UserProcess, arg: usize) -> isize {
    const IFCONF_SIZE: usize = 16;
    const IFCONF_BUF_OFFSET: usize = 8;
    const IFREQ_SIZE: usize = 40;
    const IFREQ_NAME_SIZE: usize = 16;
    if arg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let ifconf = match read_user_bytes(process, arg, IFCONF_SIZE) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    let requested_len =
        i32::from_ne_bytes(ifconf[0..size_of::<i32>()].try_into().unwrap()).max(0) as usize;
    let ifc_buf = usize::from_ne_bytes(
        ifconf[IFCONF_BUF_OFFSET..IFCONF_BUF_OFFSET + size_of::<usize>()]
            .try_into()
            .unwrap(),
    );
    if ifc_buf == 0 || requested_len < IFREQ_SIZE {
        let len: i32 = 0;
        return write_user_bytes_ret(process, arg, &len.to_ne_bytes());
    }

    let mut ifreq = [0u8; IFREQ_SIZE];
    ifreq[..3].copy_from_slice(b"lo\0");
    let family = 2u16.to_ne_bytes();
    ifreq[IFREQ_NAME_SIZE..IFREQ_NAME_SIZE + size_of::<u16>()].copy_from_slice(&family);
    let loopback = u32::from_be_bytes([127, 0, 0, 1]).to_ne_bytes();
    let addr_offset = IFREQ_NAME_SIZE + 4;
    ifreq[addr_offset..addr_offset + size_of::<u32>()].copy_from_slice(&loopback);

    let written = requested_len.min(IFREQ_SIZE);
    if let Err(err) = write_user_bytes(process, ifc_buf, &ifreq[..written]) {
        return neg_errno(err);
    }
    let len = written as i32;
    write_user_bytes_ret(process, arg, &len.to_ne_bytes())
}

fn socket_ioctl_get_ifflags(process: &UserProcess, arg: usize) -> isize {
    const IFREQ_SIZE: usize = 40;
    const IFREQ_FLAGS_OFFSET: usize = 16;
    const IFF_UP: i16 = 0x1;
    const IFF_LOOPBACK: i16 = 0x8;
    const IFF_RUNNING: i16 = 0x40;
    if arg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    if let Err(err) = validate_user_write(process, arg, IFREQ_SIZE) {
        return neg_errno(err);
    }
    let flags = (IFF_UP | IFF_LOOPBACK | IFF_RUNNING).to_ne_bytes();
    write_user_bytes_ret(process, arg + IFREQ_FLAGS_OFFSET, &flags)
}

fn socket_ioctl_set_ifflags(process: &UserProcess, arg: usize) -> isize {
    const IFREQ_SIZE: usize = 40;
    if arg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    if let Err(err) = validate_user_read(process, arg, IFREQ_SIZE) {
        return neg_errno(err);
    }
    if process.uid() != 0 {
        return neg_errno(LinuxError::EPERM);
    }
    // The loopback interface can be reported, but this userspace kernel does not
    // yet have a netdev control plane that can mutate interface flags.  Reject
    // the privileged mutation explicitly instead of validating the ifreq and
    // returning a fake success.
    neg_errno(LinuxError::EOPNOTSUPP)
}

fn write_user_bytes_ret(process: &UserProcess, dst: usize, bytes: &[u8]) -> isize {
    match write_user_bytes(process, dst, bytes) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

impl ProcessFdTable {
    pub(super) fn new() -> Self {
        Self::from_table(FdTable::new())
    }

    pub(super) fn from_table(base: FdTable) -> Self {
        Self {
            state: AxMutex::new(ProcessFdTableState {
                base,
                sharing: FileTableShareTracker::default(),
                unshared: BTreeMap::new(),
            }),
        }
    }

    fn current_pid() -> i32 {
        current_task_ext()
            .map(|ext| ext.process.pid())
            .unwrap_or_default()
    }

    pub(super) fn lock(&self) -> ProcessFdTableGuard<'_> {
        self.lock_for_pid(Self::current_pid())
    }

    pub(super) fn lock_for_pid(&self, pid: i32) -> ProcessFdTableGuard<'_> {
        ProcessFdTableGuard {
            state: self.state.lock(),
            pid,
        }
    }

    pub(super) fn fork_copy_for_pid(&self, pid: i32) -> Result<FdTable, LinuxError> {
        self.lock_for_pid(pid).fork_copy()
    }

    pub(super) fn register_pid(&self, pid: i32) {
        self.state.lock().sharing.register_base(pid);
    }

    pub(super) fn share_for_child_pid(
        self: &Arc<Self>,
        parent_pid: i32,
        child_pid: i32,
    ) -> Arc<Self> {
        let mut state = self.state.lock();
        state.sharing.share(parent_pid, child_pid);
        // Clone while holding the state lock.  exec/close_range take the same
        // lock when splitting a CLONE_FILES table, so a new sharer is ordered
        // either wholly before or wholly after that split.
        Arc::clone(self)
    }

    pub(super) fn unshare_for_pid_if_shared(&self, pid: i32) -> Result<(), LinuxError> {
        let mut state = self.state.lock();
        if !state.sharing.is_shared(pid) {
            return Ok(());
        }
        let copy = match state.sharing.group(pid) {
            FileTableGroup::Private(owner) => state
                .unshared
                .get(&owner)
                .expect("unshared fd table owner must exist")
                .fork_copy()?,
            FileTableGroup::Base => state.base.fork_copy()?,
        };
        if let Some((old_owner, new_owner)) = state.sharing.split(pid) {
            let table = state
                .unshared
                .remove(&old_owner)
                .expect("split private fd table owner must exist");
            state.unshared.insert(new_owner, table);
        }
        state.unshared.insert(pid, copy);
        Ok(())
    }

    pub(super) fn close_all_for_pid(&self, pid: i32) {
        let mut tables = Vec::new();
        {
            let mut state = self.state.lock();
            match state.sharing.detach(pid) {
                FileTableDetach::Keep => {}
                FileTableDetach::DropBase => {
                    tables.push(core::mem::replace(&mut state.base, FdTable::empty()));
                }
                FileTableDetach::DropPrivate(owner) => {
                    if let Some(table) = state.unshared.remove(&owner) {
                        tables.push(table);
                    }
                }
                FileTableDetach::MovePrivate { from, to } => {
                    let table = state
                        .unshared
                        .remove(&from)
                        .expect("moved private fd table owner must exist");
                    state.unshared.insert(to, table);
                }
            }
        }
        for mut table in tables {
            table.close_all();
        }
    }
}

impl Drop for ProcessFdTable {
    fn drop(&mut self) {
        let mut tables = Vec::new();
        {
            let mut state = self.state.lock();
            tables.push(core::mem::replace(&mut state.base, FdTable::empty()));
            for (_, table) in core::mem::take(&mut state.unshared) {
                tables.push(table);
            }
        }
        for mut table in tables {
            table.close_all();
        }
    }
}

impl Deref for ProcessFdTableGuard<'_> {
    type Target = FdTable;

    fn deref(&self) -> &Self::Target {
        match self.state.sharing.group(self.pid) {
            FileTableGroup::Private(owner) => self
                .state
                .unshared
                .get(&owner)
                .expect("unshared fd table owner must exist"),
            FileTableGroup::Base => &self.state.base,
        }
    }
}

impl DerefMut for ProcessFdTableGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.state.sharing.group(self.pid) {
            FileTableGroup::Private(owner) => self
                .state
                .unshared
                .get_mut(&owner)
                .expect("unshared fd table owner must exist"),
            FileTableGroup::Base => &mut self.state.base,
        }
    }
}

impl FdTable {
    pub(super) fn new() -> Self {
        Self {
            slots: vec![
                Some(
                    FdSlot::new(FdEntry::Stdin(STDIN_STATUS_FLAGS), 0)
                        .expect("open-file IDs exhausted while creating stdin"),
                ),
                Some(
                    FdSlot::new(FdEntry::Stdout(STDOUT_STATUS_FLAGS), 0)
                        .expect("open-file IDs exhausted while creating stdout"),
                ),
                Some(
                    FdSlot::new(FdEntry::Stderr(STDERR_STATUS_FLAGS), 0)
                        .expect("open-file IDs exhausted while creating stderr"),
                ),
            ],
        }
    }

    fn empty() -> Self {
        Self { slots: Vec::new() }
    }

    pub(super) fn fork_copy(&self) -> Result<Self, LinuxError> {
        let active_len = self
            .slots
            .iter()
            .rposition(Option::is_some)
            .map_or(0, |idx| idx + 1);
        let mut slots: Vec<Option<FdSlot>> = Vec::with_capacity(active_len);
        for slot in self.slots.iter().take(active_len) {
            slots.push(match slot {
                Some(slot) => {
                    let entry = match slot.entry.duplicate_for_fork() {
                        Ok(entry) => entry,
                        Err(err) => {
                            // Socket duplicates own freshly allocated POSIX
                            // descriptors and do not have an implicit Drop
                            // close.  Roll them back before abandoning a
                            // partially constructed table.
                            for copied in slots.iter().flatten() {
                                if let FdEntry::Socket(socket) = &copied.entry {
                                    let _ = socket.close();
                                }
                            }
                            return Err(err);
                        }
                    };
                    Some(FdSlot::with_identity(
                        entry,
                        slot.fd_flags,
                        Arc::clone(&slot.description),
                    ))
                }
                None => None,
            });
        }
        let table = Self { slots };
        table.track_existing_exec_write_opens();
        Ok(table)
    }

    fn track_existing_exec_write_opens(&self) {
        for slot in self.slots.iter().flatten() {
            track_exec_write_open(&slot.entry);
        }
    }

    pub(super) fn is_stdio(&self, fd: i32) -> bool {
        matches!(
            self.entry(fd),
            Ok(FdEntry::Stdin(_) | FdEntry::Stdout(_) | FdEntry::Stderr(_))
        )
    }

    pub(super) fn is_rtc(&self, fd: i32) -> bool {
        matches!(self.entry(fd), Ok(FdEntry::Rtc))
    }

    pub(super) fn is_block_device(&self, fd: i32) -> bool {
        matches!(self.entry(fd), Ok(FdEntry::BlockDevice(_)))
    }

    pub(super) fn is_dev_zero(&self, fd: i32) -> bool {
        matches!(
            self.entry(fd),
            Ok(FdEntry::File(FileEntry { path, .. })) if path == "/dev/zero"
        ) || matches!(self.entry(fd), Ok(FdEntry::DevZero(_)))
    }

    pub(super) fn pipe_available_read(&self, fd: i32) -> Result<usize, LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(pipe) => Ok(pipe_endpoint(pipe)?.available_read()),
            _ => Err(LinuxError::ENOTTY),
        }
    }

    fn splice_pipe_input(&self, fd: i32) -> Result<OpenFileRef, LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(pipe) => Ok(pipe.clone()),
            _ => Err(LinuxError::EBADF),
        }
    }

    fn splice_pipe_snapshot(
        &self,
        fd: i32,
    ) -> Result<(OpenFileId, Option<OpenFileRef>), LinuxError> {
        let slot = self.slot(fd)?;
        let pipe = match &slot.entry {
            FdEntry::Pipe(pipe) => Some(pipe.clone()),
            _ => None,
        };
        Ok((slot.description_id(), pipe))
    }

    fn splice_snapshot_is_current(&self, fd: i32, description: OpenFileId) -> bool {
        matches!(self.slot(fd), Ok(slot) if slot.description_id() == description)
    }

    fn splice_stream_available_write(&self, fd: i32) -> Result<usize, LinuxError> {
        match self.entry(fd)? {
            FdEntry::LocalSocket(socket) => socket.available_write_after_growth(),
            // Proc/sys writes are synchronous in-kernel updates rather than bounded socket
            // queues, so there is no stream buffer capacity to pre-limit here.
            FdEntry::ProcSysFile(_) => Ok(usize::MAX),
            _ => Err(LinuxError::EBADF),
        }
    }

    fn splice_stream_readiness(&self, fd: i32) -> Result<SpliceStreamReadiness, LinuxError> {
        match self.entry(fd)? {
            FdEntry::LocalSocket(socket) => {
                let (available, peer_open) = socket.available_read_and_peer_open()?;
                if available > 0 {
                    Ok(SpliceStreamReadiness::Data(available))
                } else if peer_open {
                    Ok(SpliceStreamReadiness::WouldBlock {
                        endpoint_nonblocking: socket.status_flags() & general::O_NONBLOCK as i32
                            != 0,
                    })
                } else {
                    Ok(SpliceStreamReadiness::Eof)
                }
            }
            // Proc/sys streams synthesize data synchronously, so no socket buffer
            // readiness can be sampled before the read itself.
            FdEntry::ProcSysFile(_) => Ok(SpliceStreamReadiness::Unknown),
            _ => Err(LinuxError::EBADF),
        }
    }

    fn splice_local_socket_output(&self, fd: i32) -> Result<Option<LocalSocketEntry>, LinuxError> {
        match self.entry(fd)? {
            FdEntry::LocalSocket(socket) => Ok(Some(socket.duplicate())),
            FdEntry::ProcSysFile(_) => Ok(None),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn pipe_capacity(&self, fd: i32) -> Result<usize, LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(pipe) => Ok(pipe_endpoint(pipe)?.capacity()),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn poll(&self, fd: i32, mode: SelectMode) -> bool {
        let Ok(entry) = self.entry(fd) else {
            return matches!(mode, SelectMode::Except);
        };
        Self::poll_entry(entry, mode)
    }

    pub(super) fn poll_entry(entry: &FdEntry, mode: SelectMode) -> bool {
        match mode {
            SelectMode::Read => match entry {
                FdEntry::Stdin(_) => false,
                FdEntry::Stdout(_) | FdEntry::Stderr(_) => false,
                FdEntry::DevNull
                | FdEntry::DevZero(_)
                | FdEntry::DevRandom(_)
                | FdEntry::DevCpuDmaLatency(_)
                | FdEntry::BlockDevice(_)
                | FdEntry::Rtc
                | FdEntry::File(_)
                | FdEntry::Directory(_)
                | FdEntry::ProcFdDir(_)
                | FdEntry::SyntheticDir(_)
                | FdEntry::MemoryFile(_)
                | FdEntry::Memfd(_)
                | FdEntry::ProcPagemap(_)
                | FdEntry::ProcTimerSlack(_)
                | FdEntry::ProcSysFile(_) => true,
                FdEntry::Path(_) => false,
                FdEntry::EventFd(eventfd) => {
                    eventfd.object().readiness().contains(ReadyEvents::READABLE)
                }
                FdEntry::Inotify(_) => false,
                FdEntry::TimerFd(timerfd) => {
                    timerfd.object().readiness().contains(ReadyEvents::READABLE)
                }
                FdEntry::SignalFd(signalfd) => signalfd.poll_readable(),
                FdEntry::PidFd(pidfd) => pidfd.exited(),
                FdEntry::PosixMq(mq) => mq.poll_readable(),
                FdEntry::ProcMqQueuesMax(_) => true,
                FdEntry::Epoll(_) => false,
                FdEntry::Pipe(pipe) => pipe
                    .object()
                    .readiness()
                    .contains(axfile::ReadyEvents::READABLE),
                FdEntry::Socket(socket) => socket.poll(mode),
                FdEntry::LocalSocket(socket) => socket.poll(mode),
            },
            SelectMode::Write => match entry {
                FdEntry::Stdin(_) => false,
                FdEntry::Stdout(_)
                | FdEntry::Stderr(_)
                | FdEntry::DevNull
                | FdEntry::DevZero(_)
                | FdEntry::DevRandom(_)
                | FdEntry::DevCpuDmaLatency(_)
                | FdEntry::BlockDevice(_)
                | FdEntry::Rtc => true,
                FdEntry::File(_) | FdEntry::Memfd(_) => true,
                FdEntry::Directory(_)
                | FdEntry::ProcFdDir(_)
                | FdEntry::SyntheticDir(_)
                | FdEntry::Path(_)
                | FdEntry::MemoryFile(_)
                | FdEntry::ProcPagemap(_)
                | FdEntry::ProcTimerSlack(_)
                | FdEntry::Inotify(_)
                | FdEntry::TimerFd(_)
                | FdEntry::SignalFd(_)
                | FdEntry::PidFd(_)
                | FdEntry::Epoll(_) => false,
                FdEntry::PosixMq(mq) => mq.poll_writable(),
                FdEntry::ProcMqQueuesMax(_) => true,
                FdEntry::ProcSysFile(_) => true,
                FdEntry::EventFd(eventfd) => {
                    eventfd.object().readiness().contains(ReadyEvents::WRITABLE)
                }
                FdEntry::Pipe(pipe) => pipe
                    .object()
                    .readiness()
                    .contains(axfile::ReadyEvents::WRITABLE),
                FdEntry::Socket(socket) => socket.poll(mode),
                FdEntry::LocalSocket(socket) => socket.poll(mode),
            },
            SelectMode::Except => false,
        }
    }

    fn epoll_ctl(
        &mut self,
        epfd: i32,
        op: u32,
        fd: i32,
        event: Option<general::epoll_event>,
    ) -> Result<EpollCtlUpdate, LinuxError> {
        if epfd == fd {
            return Err(LinuxError::EINVAL);
        }
        let epoll = match self.entry(epfd)? {
            FdEntry::Epoll(epoll) => epoll.clone(),
            _ => return Err(LinuxError::EINVAL),
        };
        let target_slot = self.slot(fd)?;
        let key = RegistrationKey::new(fd, target_slot.description_id());
        // Serialize validation and graph mutation across independently forked
        // FdTables that still share EpollState objects.
        let graph_guard = epoll_graph_lock().lock();
        let retired = match op {
            general::EPOLL_CTL_ADD | general::EPOLL_CTL_MOD => {
                let Some(event) = event else {
                    return Err(LinuxError::EFAULT);
                };
                {
                    let registrations = epoll.state.registrations.lock();
                    if op == general::EPOLL_CTL_ADD && registrations.contains_key(&key) {
                        return Err(LinuxError::EEXIST);
                    }
                    if op == general::EPOLL_CTL_MOD && !registrations.contains_key(&key) {
                        return Err(LinuxError::ENOENT);
                    }
                }
                self.validate_epoll_target(&epoll, &target_slot.entry)?;
                let version = epoll.state.next_version()?;
                let close_observer = Arc::new(EpollCloseObserver {
                    owner: Arc::downgrade(&epoll.state),
                    key,
                    version,
                });
                let erased_close_observer: Arc<dyn EventObserver> = close_observer.clone();
                let close_subscription = target_slot
                    .description
                    .close_events()
                    .subscribe(&erased_close_observer)
                    .map_err(|err| match err {
                        axfile::RegistrationError::SourceClosed => LinuxError::EBADF,
                        axfile::RegistrationError::TokenExhausted => LinuxError::ENFILE,
                    })?
                    .1;
                let (target, ready_observer, subscription) = match &target_slot.entry {
                    entry if entry.open_file().is_some() => {
                        let file = entry.open_file().expect("guarded by is_some");
                        let ready_observer = EpollReadyObserver::new(
                            &epoll.state,
                            Self::epoll_observer_interests(event.events),
                        );
                        let observer: Arc<dyn EventObserver> = ready_observer.clone();
                        let subscription = file
                            .object()
                            .event_source()
                            .subscribe(&observer)
                            .map_err(|err| match err {
                                axfile::RegistrationError::SourceClosed => LinuxError::EBADF,
                                axfile::RegistrationError::TokenExhausted => LinuxError::ENFILE,
                            })?
                            .1;
                        (
                            EpollTarget::Object(Arc::downgrade(file)),
                            Some(ready_observer),
                            Some(subscription),
                        )
                    }
                    FdEntry::Epoll(target_epoll) => {
                        let ready_observer = EpollReadyObserver::new(
                            &epoll.state,
                            Self::epoll_observer_interests(event.events),
                        );
                        let observer: Arc<dyn EventObserver> = ready_observer.clone();
                        let subscription = target_epoll
                            .state
                            .events
                            .subscribe(&observer)
                            .map_err(|err| match err {
                                axfile::RegistrationError::SourceClosed => LinuxError::EBADF,
                                axfile::RegistrationError::TokenExhausted => LinuxError::ENFILE,
                            })?
                            .1;
                        (
                            EpollTarget::Epoll(Arc::downgrade(&target_epoll.state)),
                            Some(ready_observer),
                            Some(subscription),
                        )
                    }
                    _ => (
                        EpollTarget::Legacy(LegacyEpollTarget::duplicate(&target_slot.entry)?),
                        None,
                        None,
                    ),
                };
                let registration = EpollRegistration::new(
                    target,
                    event,
                    version,
                    ready_observer,
                    subscription,
                    close_observer,
                    close_subscription,
                );
                epoll.state.registrations.lock().insert(key, registration)
            }
            general::EPOLL_CTL_DEL => {
                let mut registrations = epoll.state.registrations.lock();
                let retired = registrations.remove(&key);
                if retired.is_none() {
                    return Err(LinuxError::ENOENT);
                }
                retired
            }
            _ => return Err(LinuxError::EINVAL),
        };
        drop(graph_guard);
        Ok(EpollCtlUpdate { epoll, retired })
    }

    fn validate_epoll_target(
        &self,
        epoll: &EpollEntry,
        target: &FdEntry,
    ) -> Result<(), LinuxError> {
        match target {
            FdEntry::Pipe(_)
            | FdEntry::Socket(_)
            | FdEntry::LocalSocket(_)
            | FdEntry::EventFd(_)
            | FdEntry::TimerFd(_)
            | FdEntry::SignalFd(_)
            | FdEntry::PidFd(_)
            | FdEntry::PosixMq(_) => Ok(()),
            FdEntry::Epoll(target) => {
                if target.id() == epoll.id() {
                    return Err(LinuxError::EINVAL);
                }
                if Self::epoll_reaches(&target.state, epoll.id(), &mut Vec::new())? {
                    return Err(LinuxError::ELOOP);
                }
                if Self::epoll_nesting_depth(&target.state, &mut Vec::new())?
                    >= LINUX_EPOLL_MAX_NEST_DEPTH
                {
                    return Err(LinuxError::EINVAL);
                }
                Ok(())
            }
            _ => Err(LinuxError::EPERM),
        }
    }

    fn epoll_reaches(
        state: &Arc<EpollState>,
        target_id: OpenFileId,
        visited: &mut Vec<OpenFileId>,
    ) -> Result<bool, LinuxError> {
        if state.id == target_id {
            return Ok(true);
        }
        if visited.contains(&state.id) {
            return Ok(false);
        }
        visited.push(state.id);
        for registration in state.registration_snapshots()? {
            if let EpollTarget::Epoll(child) = registration.target {
                if let Some(child) = child.upgrade() {
                    if Self::epoll_reaches(&child, target_id, visited)? {
                        visited.pop();
                        return Ok(true);
                    }
                }
            }
        }
        visited.pop();
        Ok(false)
    }

    fn epoll_nesting_depth(
        state: &Arc<EpollState>,
        visited: &mut Vec<OpenFileId>,
    ) -> Result<usize, LinuxError> {
        if visited.contains(&state.id) {
            return Ok(LINUX_EPOLL_MAX_NEST_DEPTH);
        }
        visited.push(state.id);
        let mut max_child_depth = 0usize;
        for registration in state.registration_snapshots()? {
            if let EpollTarget::Epoll(child) = registration.target {
                if let Some(child) = child.upgrade() {
                    max_child_depth =
                        cmp::max(max_child_depth, Self::epoll_nesting_depth(&child, visited)?);
                }
            }
        }
        visited.pop();
        Ok(max_child_depth.saturating_add(1))
    }

    pub(super) fn epoll_entry(&self, epfd: i32) -> Result<EpollEntry, LinuxError> {
        match self.entry(epfd)? {
            FdEntry::Epoll(epoll) => Ok(epoll.clone()),
            _ => Err(LinuxError::EINVAL),
        }
    }

    fn epoll_wait_profile(
        state: &Arc<EpollState>,
        visited: &mut Vec<OpenFileId>,
    ) -> Result<(bool, Option<Duration>), LinuxError> {
        if visited.contains(&state.id) {
            return Ok((false, None));
        }
        visited.push(state.id);
        let mut has_legacy = false;
        let mut next_timeout: Option<Duration> = None;
        for snapshot in state.registration_snapshots()? {
            match snapshot.target {
                EpollTarget::Object(file) => {
                    if let Some(file) = file.upgrade() {
                        if let Some(timeout) = file.object().next_timeout() {
                            next_timeout =
                                Some(next_timeout.map_or(timeout, |current| current.min(timeout)));
                        }
                    }
                }
                EpollTarget::Legacy(_) => has_legacy = true,
                EpollTarget::Epoll(child) => {
                    if let Some(child) = child.upgrade() {
                        let (child_legacy, child_timeout) =
                            Self::epoll_wait_profile(&child, visited)?;
                        has_legacy |= child_legacy;
                        if let Some(timeout) = child_timeout {
                            next_timeout =
                                Some(next_timeout.map_or(timeout, |current| current.min(timeout)));
                        }
                    }
                }
            }
        }
        visited.pop();
        Ok((has_legacy, next_timeout))
    }

    fn epoll_collect_ready_for(
        epoll: &EpollEntry,
        maxevents: usize,
        out: &mut Vec<general::epoll_event>,
    ) -> Result<usize, LinuxError> {
        let snapshots = epoll.state.registration_snapshots()?;
        perf_counters::record_epoll_ready_scan(snapshots.len());
        let mut stale = Vec::new();
        for snapshot in snapshots {
            if snapshot.delivery.is_disabled() {
                continue;
            }
            let mut notification = snapshot
                .ready_observer
                .as_ref()
                .map_or(0, |observer| observer.generation());
            loop {
                let Some(ready_events) = Self::epoll_ready_events_for_target(
                    &snapshot.target,
                    snapshot.event.events,
                    &mut Vec::new(),
                )?
                else {
                    stale.push((snapshot.key, snapshot.version));
                    break;
                };
                let observed_after_query = snapshot
                    .ready_observer
                    .as_ref()
                    .map_or(0, |observer| observer.generation());
                if observed_after_query != notification {
                    notification = observed_after_query;
                    continue;
                }
                let has_capacity = out.len() < maxevents;
                if ready_events != 0 && has_capacity {
                    out.try_reserve_exact(1).map_err(|_| LinuxError::ENOMEM)?;
                }
                enum DeliveryClaim {
                    Retry(u64),
                    Done(bool),
                    Gone,
                }
                let claim = {
                    let mut registrations = epoll.state.registrations.lock();
                    let Some(registration) = registrations.get_mut(&snapshot.key) else {
                        break;
                    };
                    if registration.version != snapshot.version {
                        DeliveryClaim::Gone
                    } else {
                        let current_notification = registration
                            .ready_observer
                            .as_ref()
                            .map_or(0, |observer| observer.generation());
                        if current_notification != notification {
                            DeliveryClaim::Retry(current_notification)
                        } else {
                            DeliveryClaim::Done(registration.delivery.claim(
                                ready_events,
                                notification,
                                registration.event.events & general::EPOLLET != 0,
                                registration.event.events & general::EPOLLONESHOT != 0,
                                has_capacity,
                            ))
                        }
                    }
                };
                match claim {
                    DeliveryClaim::Retry(current) => {
                        notification = current;
                    }
                    DeliveryClaim::Done(true) => {
                        out.push(general::epoll_event {
                            events: ready_events,
                            data: snapshot.event.data,
                        });
                        break;
                    }
                    DeliveryClaim::Done(false) | DeliveryClaim::Gone => break,
                }
            }
        }
        if !stale.is_empty() {
            let mut registrations = epoll.state.registrations.lock();
            let mut removed = Vec::new();
            for (key, version) in stale {
                if registrations
                    .get(&key)
                    .is_some_and(|registration| registration.version == version)
                {
                    if let Some(registration) = registrations.remove(&key) {
                        removed.push(registration);
                    }
                }
            }
            drop(registrations);
            drop(removed);
        }
        Ok(epoll.state.registrations.lock().len())
    }

    fn epoll_ready_events_for_target(
        target: &EpollTarget,
        requested: u32,
        visited: &mut Vec<OpenFileId>,
    ) -> Result<Option<u32>, LinuxError> {
        match target {
            EpollTarget::Object(file) => Ok(file
                .upgrade()
                .map(|file| Self::epoll_ready_events_for_object(&file, requested))),
            EpollTarget::Legacy(target) => Ok(Some(Self::epoll_ready_events_for_entry(
                &target.entry,
                requested,
            ))),
            EpollTarget::Epoll(state) => {
                let Some(state) = state.upgrade() else {
                    return Ok(None);
                };
                Ok(Some(if Self::epoll_state_has_ready(&state, visited)? {
                    general::EPOLLIN
                } else {
                    0
                }))
            }
        }
    }

    fn epoll_state_has_ready(
        state: &Arc<EpollState>,
        visited: &mut Vec<OpenFileId>,
    ) -> Result<bool, LinuxError> {
        if visited.contains(&state.id) {
            return Ok(false);
        }
        visited.push(state.id);
        for snapshot in state.registration_snapshots()? {
            if snapshot.delivery.is_disabled() {
                continue;
            }
            if let Some(ready) = Self::epoll_ready_events_for_target(
                &snapshot.target,
                snapshot.event.events,
                visited,
            )? {
                let edge = snapshot.event.events & general::EPOLLET != 0;
                let notification = snapshot
                    .ready_observer
                    .as_ref()
                    .map_or(0, |observer| observer.generation());
                if snapshot.delivery.should_emit(ready, notification, edge) {
                    visited.pop();
                    return Ok(true);
                }
            }
        }
        visited.pop();
        Ok(false)
    }

    fn epoll_observer_interests(requested: u32) -> ReadyEvents {
        let mut interests = ReadyEvents::ERROR | ReadyEvents::HANGUP;
        if requested & general::EPOLLIN != 0 {
            interests |= ReadyEvents::READABLE;
        }
        if requested & general::EPOLLOUT != 0 {
            interests |= ReadyEvents::WRITABLE;
        }
        if requested & general::EPOLLPRI != 0 {
            interests |= ReadyEvents::PRIORITY;
        }
        interests
    }

    fn epoll_ready_events_for_object(file: &OpenFileRef, requested: u32) -> u32 {
        let ready = file.object().readiness();
        let mut ready_events = 0u32;
        if requested & general::EPOLLIN != 0 && ready.intersects(ReadyEvents::READABLE) {
            ready_events |= general::EPOLLIN;
        }
        if requested & general::EPOLLOUT != 0 && ready.intersects(ReadyEvents::WRITABLE) {
            ready_events |= general::EPOLLOUT;
        }
        if ready.intersects(ReadyEvents::ERROR) {
            ready_events |= general::EPOLLERR;
        }
        if ready.intersects(ReadyEvents::HANGUP) {
            ready_events |= general::EPOLLHUP;
        }
        ready_events
    }

    fn epoll_ready_events_for_entry(entry: &FdEntry, requested: u32) -> u32 {
        let mut ready_events = 0u32;
        if requested & general::EPOLLIN != 0 && Self::poll_entry(entry, SelectMode::Read) {
            ready_events |= general::EPOLLIN;
        }
        if requested & general::EPOLLOUT != 0 && Self::poll_entry(entry, SelectMode::Write) {
            ready_events |= general::EPOLLOUT;
        }
        if requested & general::EPOLLRDHUP != 0
            && matches!(entry, FdEntry::Socket(socket) if socket.poll_rdhup())
        {
            ready_events |= general::EPOLLRDHUP;
        }
        ready_events
    }

    pub(super) fn read(
        &mut self,
        process: &UserProcess,
        fd: i32,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdin(_) => Ok(0),
            FdEntry::DevNull => Ok(0),
            FdEntry::DevZero(status_flags) => {
                if !file_is_readable(*status_flags) {
                    return Err(LinuxError::EBADF);
                }
                dst.fill(0);
                Ok(dst.len())
            }
            FdEntry::DevRandom(status_flags) => {
                if !file_is_readable(*status_flags) {
                    return Err(LinuxError::EBADF);
                }
                fill_pseudo_random_bytes(dst);
                Ok(dst.len())
            }
            FdEntry::DevCpuDmaLatency(status_flags) => {
                if !file_is_readable(*status_flags) {
                    return Err(LinuxError::EBADF);
                }
                Ok(0)
            }
            FdEntry::BlockDevice(dev) => dev.read(dst),
            FdEntry::Rtc => Ok(0),
            FdEntry::File(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file_entry_read(process, file, dst)
            }
            FdEntry::MemoryFile(file) => Ok(file.read(dst)),
            FdEntry::Memfd(file) => file.read(dst),
            FdEntry::ProcPagemap(file) => Ok(file.read(dst)),
            FdEntry::ProcTimerSlack(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file.read(process, dst)
            }
            FdEntry::ProcSysFile(file) => file.read(dst),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            // Migrated objects are snapshotted and dispatched before entering
            // this legacy adapter, so a reused slot can never block while the
            // FD-table lock is held.
            FdEntry::Pipe(_) | FdEntry::EventFd(_) | FdEntry::TimerFd(_) => Err(LinuxError::EBADF),
            FdEntry::Socket(socket) => socket.read(dst),
            FdEntry::LocalSocket(socket) => socket.read(process, dst),
            FdEntry::Inotify(inotify) => inotify.read(),
            FdEntry::SignalFd(signalfd) => signalfd.read(process, dst),
            FdEntry::ProcMqQueuesMax(entry) => entry.read(dst),
            FdEntry::PosixMq(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn write(
        &mut self,
        process: &UserProcess,
        fd: i32,
        src: &[u8],
        file_size_limit: Option<u64>,
    ) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdout(_) | FdEntry::Stderr(_) => {
                axhal::console::write_bytes(src);
                Ok(src.len())
            }
            FdEntry::DevNull => Ok(src.len()),
            FdEntry::DevZero(status_flags) => {
                if !file_is_writable(*status_flags) {
                    return Err(LinuxError::EBADF);
                }
                Ok(src.len())
            }
            FdEntry::DevRandom(status_flags) => {
                if !file_is_writable(*status_flags) {
                    return Err(LinuxError::EBADF);
                }
                Ok(src.len())
            }
            FdEntry::DevCpuDmaLatency(status_flags) => {
                if !file_is_writable(*status_flags) {
                    return Err(LinuxError::EBADF);
                }
                // Linux exposes /dev/cpu_dma_latency as a PM QoS request file:
                // userspace writes a binary i32 target or hex string and keeps
                // the fd open while the request is active.  This single-vCPU
                // virtual kernel has no CPU power states to constrain, so a
                // syntactically valid request is accepted as a no-op.
                validate_cpu_dma_latency_request(src)?;
                Ok(src.len())
            }
            FdEntry::BlockDevice(dev) => dev.write(src),
            FdEntry::Rtc => Ok(src.len()),
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file_entry_write(process, file, src, file_size_limit)
            }
            FdEntry::Memfd(file) => file.write(src, file_size_limit),
            FdEntry::Pipe(_) | FdEntry::EventFd(_) => Err(LinuxError::EBADF),
            FdEntry::Socket(socket) => socket.write(src),
            FdEntry::LocalSocket(socket) => socket.write(process, src),
            FdEntry::ProcTimerSlack(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file.write(process, src)
            }
            FdEntry::ProcMqQueuesMax(entry) => entry.write(src),
            FdEntry::ProcSysFile(entry) => entry.write(src),
            FdEntry::TimerFd(_) | FdEntry::SignalFd(_) => Err(LinuxError::EINVAL),
            FdEntry::PosixMq(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn write_file_at(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        src: &[u8],
        file_size_limit: Option<u64>,
    ) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                let base_offset = if file.status_flags & general::O_APPEND != 0 {
                    file_logical_size(process, file)?
                } else {
                    offset
                };
                write_regular_file_at(process, file, base_offset, src, file_size_limit)
            }
            FdEntry::Memfd(file) => file.write_at(offset, src, file_size_limit),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Err(LinuxError::EBADF),
        }
    }

    fn prepare_close_slot(&self, idx: usize) -> Result<(), LinuxError> {
        let Some(slot) = self.slots.get(idx).and_then(Option::as_ref) else {
            return Err(LinuxError::EBADF);
        };
        if let FdEntry::Socket(socket) = &slot.entry {
            socket.close()?;
        }
        Ok(())
    }

    fn take_prepared_slot(&mut self, idx: usize) -> ClosedFd {
        let entry = &self.slots[idx]
            .as_ref()
            .expect("prepared descriptor slot must remain present")
            .entry;
        if let FdEntry::File(file) = entry {
            release_flock_on_last_close(file);
            release_file_lease_on_last_close(file);
            release_ofd_record_locks_on_last_close(file);
        }
        untrack_exec_write_open(entry);
        let slot = self.slots[idx]
            .take()
            .expect("prepared descriptor slot must remain present");
        ClosedFd {
            _entry: slot.entry,
            _description: slot.description,
        }
    }

    fn close_slot(&mut self, idx: usize) -> Result<ClosedFd, LinuxError> {
        self.prepare_close_slot(idx)?;
        Ok(self.take_prepared_slot(idx))
    }

    pub(super) fn close(&mut self, fd: i32) -> Result<ClosedFd, LinuxError> {
        if !(0..self.slots.len() as i32).contains(&fd) || self.slots[fd as usize].is_none() {
            return Err(LinuxError::EBADF);
        }
        self.close_slot(fd as usize)
    }

    pub(super) fn close_for_process(
        &mut self,
        process: &UserProcess,
        fd: i32,
    ) -> Result<ClosedFd, LinuxError> {
        if !(0..self.slots.len() as i32).contains(&fd) || self.slots[fd as usize].is_none() {
            return Err(LinuxError::EBADF);
        }
        if let Some(FdSlot {
            entry: FdEntry::File(file),
            ..
        }) = self.slots[fd as usize].as_ref()
        {
            release_posix_record_locks_for_file_owner(record_lock_key(file), process.pid());
        }
        self.close_slot(fd as usize)
    }

    pub(super) fn close_range_for_process(
        &mut self,
        process: &UserProcess,
        first: usize,
        last: usize,
        flags: u32,
    ) -> Result<Vec<ClosedFd>, LinuxError> {
        let Some(last) = last.min(self.slots.len().saturating_sub(1)).checked_add(1) else {
            return Ok(Vec::new());
        };
        if first >= last {
            return Ok(Vec::new());
        }

        if flags & CLOSE_RANGE_CLOEXEC != 0 {
            for idx in first..last {
                if let Some(slot) = self.slots[idx].as_mut() {
                    slot.fd_flags |= general::FD_CLOEXEC;
                }
            }
            return Ok(Vec::new());
        }

        // CLOSE_RANGE_UNSHARE is applied by ProcessFdTable before selecting the
        // active table for this process.  At this layer only the requested
        // close/CLOEXEC operation remains.
        let _unshare_requested = flags & CLOSE_RANGE_UNSHARE != 0;
        let mut closed = Vec::new();
        closed
            .try_reserve_exact(last - first)
            .map_err(|_| LinuxError::ENOMEM)?;
        // Perform all fallible socket closes before detaching any entries, so
        // an error cannot drop already-detached event sources under the table
        // lock while unwinding this call.
        for idx in first..last {
            if self.slots[idx].is_some() {
                self.prepare_close_slot(idx)?;
            }
        }
        for idx in first..last {
            if self.slots[idx].is_some() {
                if let Some(FdSlot {
                    entry: FdEntry::File(file),
                    ..
                }) = self.slots[idx].as_ref()
                {
                    release_posix_record_locks_for_file_owner(record_lock_key(file), process.pid());
                }
                closed.push(self.take_prepared_slot(idx));
            }
        }
        Ok(closed)
    }

    pub(super) fn close_all(&mut self) {
        for idx in 0..self.slots.len() {
            if let Ok(entry) = self.close_slot(idx) {
                drop(entry);
            }
        }
    }

    pub(super) fn close_cloexec(&mut self) -> Vec<ClosedFd> {
        let close_count = self
            .slots
            .iter()
            .flatten()
            .filter(|slot| slot.fd_flags & general::FD_CLOEXEC != 0)
            .count();
        let mut closed = Vec::with_capacity(close_count);
        for idx in 0..self.slots.len() {
            if self.slots[idx]
                .as_ref()
                .is_none_or(|slot| slot.fd_flags & general::FD_CLOEXEC == 0)
            {
                continue;
            }
            if let Ok(entry) = self.close_slot(idx) {
                closed.push(entry);
            }
        }
        closed
    }

    pub(super) fn truncate(
        &mut self,
        process: &UserProcess,
        fd: i32,
        size: u64,
    ) -> Result<(), LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EINVAL);
                }
                invalidate_exec_image_cache(file.path.as_str());
                let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
                process.ensure_path_data_ranges(file.path.clone(), physical_size);
                resize_regular_file_physical_prefix(file, physical_size, size)?;
                process.truncate_path_sparse_file(file.path.clone(), size);
                Ok(())
            }
            FdEntry::DevNull
            | FdEntry::DevCpuDmaLatency(_)
            | FdEntry::BlockDevice(_)
            | FdEntry::Rtc => Err(LinuxError::EOPNOTSUPP),
            FdEntry::Memfd(file) => file.truncate(size),
            FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn fallocate_allocate(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        len: u64,
        keep_size: bool,
    ) -> Result<(), LinuxError> {
        let end = offset.checked_add(len).ok_or(LinuxError::EFBIG)?;
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                invalidate_exec_image_cache(file.path.as_str());
                let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
                let logical_size = file_logical_size(process, file)?;
                process.ensure_path_data_ranges(file.path.clone(), physical_size);
                ensure_tmpfs_allocated_range(process, file.path.as_str(), offset, len)?;
                if !keep_size {
                    let target_size = logical_size.max(end);
                    resize_regular_file_physical_prefix(file, physical_size, target_size)?;
                    process.set_path_sparse_size(file.path.clone(), target_size);
                }
                process.mark_path_data_range(file.path.clone(), offset, len);
                Ok(())
            }
            FdEntry::DevNull
            | FdEntry::DevCpuDmaLatency(_)
            | FdEntry::BlockDevice(_)
            | FdEntry::Rtc => Err(LinuxError::EOPNOTSUPP),
            FdEntry::Memfd(file) => {
                if keep_size {
                    file.fallocate_keep_size()
                } else {
                    file.truncate(end)
                }
            }
            FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn fallocate_punch_hole(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        len: u64,
    ) -> Result<(), LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Memfd(file) => file.punch_hole(offset, len),
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                invalidate_exec_image_cache(file.path.as_str());
                let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
                process.ensure_path_data_ranges(file.path.clone(), physical_size);
                process.clear_path_data_range(file.path.clone(), offset, len);
                Ok(())
            }
            FdEntry::DevNull
            | FdEntry::DevCpuDmaLatency(_)
            | FdEntry::BlockDevice(_)
            | FdEntry::Rtc => Err(LinuxError::EOPNOTSUPP),
            FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn fallocate_zero_range(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        len: u64,
        keep_size: bool,
    ) -> Result<(), LinuxError> {
        let end = offset.checked_add(len).ok_or(LinuxError::EFBIG)?;
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                invalidate_exec_image_cache(file.path.as_str());
                let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
                let logical_size = file_logical_size(process, file)?;
                process.ensure_path_data_ranges(file.path.clone(), physical_size);
                if !keep_size {
                    let target_size = logical_size.max(end);
                    resize_regular_file_physical_prefix(file, physical_size, target_size)?;
                    process.set_path_sparse_size(file.path.clone(), target_size);
                }
                let zero_len = if keep_size {
                    logical_size.saturating_sub(offset).min(len)
                } else {
                    len
                };
                ensure_tmpfs_allocated_range(process, file.path.as_str(), offset, zero_len)?;
                write_sparse_zero_range(process, file.path.clone(), offset, zero_len)?;
                touch_regular_file_after_write(process, file);
                Ok(())
            }
            FdEntry::Memfd(file) => {
                if !file.writable() {
                    return Err(LinuxError::EBADF);
                }
                if end > MAX_IN_MEMORY_FILE_SIZE {
                    return Err(LinuxError::EFBIG);
                }
                if !keep_size {
                    file.truncate(file.size().max(end))?;
                }
                write_memfd_zero_range(file, offset, len)
            }
            FdEntry::DevNull
            | FdEntry::DevCpuDmaLatency(_)
            | FdEntry::BlockDevice(_)
            | FdEntry::Rtc => Err(LinuxError::EOPNOTSUPP),
            FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn fallocate_collapse_range(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        len: u64,
    ) -> Result<(), LinuxError> {
        let end = offset.checked_add(len).ok_or(LinuxError::EFBIG)?;
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                let size = file_logical_size(process, file)?;
                if end > size {
                    return Err(LinuxError::EINVAL);
                }
                let mut data = read_regular_file_to_vec(process, file, size)?;
                data.drain(offset as usize..end as usize);
                rewrite_regular_file_from_vec(process, file, &data)?;
                Ok(())
            }
            FdEntry::Memfd(file) => {
                if !file.writable() {
                    return Err(LinuxError::EBADF);
                }
                let size = file.size();
                if end > size {
                    return Err(LinuxError::EINVAL);
                }
                let mut data = read_memfd_to_vec(file, size)?;
                data.drain(offset as usize..end as usize);
                file.truncate(0)?;
                file.write_at(0, &data, None).map(|_| ())
            }
            FdEntry::DevNull
            | FdEntry::DevCpuDmaLatency(_)
            | FdEntry::BlockDevice(_)
            | FdEntry::Rtc => Err(LinuxError::EOPNOTSUPP),
            FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn fallocate_insert_range(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        len: u64,
    ) -> Result<(), LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                let size = file_logical_size(process, file)?;
                let new_size = size.checked_add(len).ok_or(LinuxError::EFBIG)?;
                if offset > size || new_size > MAX_IN_MEMORY_FILE_SIZE {
                    return Err(LinuxError::EINVAL);
                }
                let mut data = read_regular_file_to_vec(process, file, size)?;
                insert_zero_range(&mut data, offset as usize, len as usize)?;
                rewrite_regular_file_from_vec(process, file, &data)?;
                Ok(())
            }
            FdEntry::Memfd(file) => {
                if !file.writable() {
                    return Err(LinuxError::EBADF);
                }
                let size = file.size();
                let new_size = size.checked_add(len).ok_or(LinuxError::EFBIG)?;
                if offset > size || new_size > MAX_IN_MEMORY_FILE_SIZE {
                    return Err(LinuxError::EINVAL);
                }
                let mut data = read_memfd_to_vec(file, size)?;
                insert_zero_range(&mut data, offset as usize, len as usize)?;
                file.truncate(0)?;
                file.write_at(0, &data, None).map(|_| ())
            }
            FdEntry::DevNull
            | FdEntry::DevCpuDmaLatency(_)
            | FdEntry::BlockDevice(_)
            | FdEntry::Rtc => Err(LinuxError::EOPNOTSUPP),
            FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn fadvise64(&self, fd: i32) -> Result<(), LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Ok(()),
        }
    }

    pub(super) fn lseek(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: i64,
        whence: u32,
    ) -> Result<u64, LinuxError> {
        if matches!(whence, SEEK_DATA_WHENCE | SEEK_HOLE_WHENCE) {
            if offset < 0 {
                return Err(LinuxError::EINVAL);
            }
            return match self.entry_mut(fd)? {
                FdEntry::File(file) => file_entry_seek_data_or_hole(
                    process,
                    file,
                    offset as u64,
                    whence == SEEK_DATA_WHENCE,
                ),
                FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                    Err(LinuxError::EISDIR)
                }
                FdEntry::Path(_) => Err(LinuxError::EBADF),
                FdEntry::Pipe(_) => Err(LinuxError::ESPIPE),
                FdEntry::Socket(_) | FdEntry::LocalSocket(_) => Err(LinuxError::ESPIPE),
                _ => Err(LinuxError::EINVAL),
            };
        }
        let pos = match whence {
            general::SEEK_SET => {
                if offset < 0 {
                    return Err(LinuxError::EINVAL);
                }
                SeekFrom::Start(offset as u64)
            }
            general::SEEK_CUR => SeekFrom::Current(offset),
            general::SEEK_END => SeekFrom::End(offset),
            _ => return Err(LinuxError::EINVAL),
        };
        match self.entry_mut(fd)? {
            FdEntry::File(file) => file_entry_seek(process, file, pos),
            FdEntry::DevNull => Ok(0),
            FdEntry::DevCpuDmaLatency(_) => Ok(0),
            FdEntry::BlockDevice(dev) => dev.seek(pos),
            FdEntry::Rtc => Ok(0),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Path(_) => Err(LinuxError::EBADF),
            FdEntry::MemoryFile(file) => file.seek(pos),
            FdEntry::Memfd(file) => file.seek(pos),
            FdEntry::ProcPagemap(file) => file.seek(pos),
            FdEntry::ProcTimerSlack(file) => file.seek(pos),
            FdEntry::ProcMqQueuesMax(file) => file.seek(pos),
            FdEntry::ProcSysFile(file) => file.seek(pos),
            FdEntry::PosixMq(_) => Err(LinuxError::ESPIPE),
            FdEntry::Pipe(_) => Err(LinuxError::ESPIPE),
            FdEntry::Socket(_) | FdEntry::LocalSocket(_) => Err(LinuxError::ESPIPE),
            _ => Err(LinuxError::ESPIPE),
        }
    }

    pub(super) fn dup(&mut self, fd: i32) -> Result<i32, LinuxError> {
        self.dup_min(fd, 0)
    }

    fn dup_min(&mut self, fd: i32, min_fd: i32) -> Result<i32, LinuxError> {
        self.dup_min_with_flags(fd, min_fd, 0)
    }

    pub(super) fn dup_min_with_flags(
        &mut self,
        fd: i32,
        min_fd: i32,
        fd_flags: u32,
    ) -> Result<i32, LinuxError> {
        if min_fd < 0 {
            return Err(LinuxError::EINVAL);
        }
        if min_fd as usize >= current_fd_table_limit() {
            return Err(LinuxError::EINVAL);
        }
        let (entry, description) = self.duplicate_entry(fd)?;
        self.insert_min_with_description(
            entry,
            min_fd as usize,
            fd_flags & general::FD_CLOEXEC,
            description,
        )
    }

    pub(super) fn dup3(
        &mut self,
        process: &UserProcess,
        oldfd: i32,
        newfd: i32,
        flags: u32,
    ) -> Result<(i32, Option<ClosedFd>), LinuxError> {
        if oldfd == newfd {
            return Err(LinuxError::EINVAL);
        }
        if flags & !general::O_CLOEXEC != 0 {
            return Err(LinuxError::EINVAL);
        }
        if newfd < 0 {
            return Err(LinuxError::EBADF);
        }
        let newfd = newfd as usize;
        if newfd
            >= cmp::min(
                FD_TABLE_LIMIT,
                process
                    .get_rlimit(RLIMIT_NOFILE_RESOURCE)
                    .current()
                    .min(FD_TABLE_LIMIT as u64) as usize,
            )
        {
            return Err(LinuxError::EBADF);
        }
        let (entry, description) = self.duplicate_entry(oldfd)?;
        let closed = if self.slots.len() <= newfd {
            self.slots.resize_with(newfd + 1, || None);
            None
        } else if self.slots[newfd].is_some() {
            match self.close_for_process(process, newfd as i32) {
                Ok(closed) => Some(closed),
                Err(err) => {
                    discard_uninstalled_entry(entry);
                    return Err(err);
                }
            }
        } else {
            None
        };
        self.slots[newfd] = Some(FdSlot::with_identity(
            entry,
            fd_cloexec_flag(flags & general::O_CLOEXEC != 0),
            description,
        ));
        Ok((newfd as i32, closed))
    }

    pub(super) fn getdents64(
        &mut self,
        process: &UserProcess,
        fd: i32,
        max_len: usize,
    ) -> Result<Vec<u8>, LinuxError> {
        if matches!(self.entry(fd)?, FdEntry::ProcFdDir(_)) {
            let fd_names = self
                .slots
                .iter()
                .enumerate()
                .filter_map(|(idx, slot)| slot.as_ref().map(|_| idx.to_string()))
                .collect::<Vec<_>>();
            let FdEntry::ProcFdDir(dir) = self.entry_mut(fd)? else {
                unreachable!();
            };
            return get_proc_fd_dirents(dir, &fd_names, max_len);
        }
        if matches!(self.entry(fd)?, FdEntry::SyntheticDir(_)) {
            let FdEntry::SyntheticDir(dir) = self.entry_mut(fd)? else {
                unreachable!();
            };
            return get_synthetic_dirents(dir, max_len);
        }
        let entry = self.entry_mut(fd)?;
        let FdEntry::Directory(dir) = entry else {
            return Err(LinuxError::ENOTDIR);
        };
        if axfs::api::metadata(dir.path.as_str()).is_err() {
            return Err(LinuxError::ENOENT);
        }
        let min_reclen = align_up(offset_of!(general::linux_dirent64, d_name) + 1, 8);
        if max_len < min_reclen {
            return Err(LinuxError::EINVAL);
        }
        let mut read_buf: [fops::DirEntry; 16] =
            core::array::from_fn(|_| fops::DirEntry::default());
        let count = dir.dir.read_dir(&mut read_buf).map_err(LinuxError::from)?;
        let mut out = Vec::new();
        let mut seen_names = Vec::new();
        for item in read_buf[..count].iter() {
            let name = item.name_as_bytes();
            let reclen = align_up(
                offset_of!(general::linux_dirent64, d_name) + name.len() + 1,
                8,
            );
            if out.len() + reclen > max_len {
                break;
            }
            let entry_path = core::str::from_utf8(name)
                .ok()
                .and_then(|name| normalize_path(dir.path.as_str(), name));
            if let Ok(name) = core::str::from_utf8(name) {
                seen_names.push(name.to_string());
            }
            dir.next_dirent_cookie = dir.next_dirent_cookie.saturating_add(1);
            let start = out.len();
            out.resize(start + reclen, 0);
            unsafe {
                let dirent = out[start..].as_mut_ptr() as *mut general::linux_dirent64;
                ptr::write_unaligned(
                    dirent,
                    general::linux_dirent64 {
                        d_ino: path_inode(entry_path.as_deref()) as _,
                        d_off: dir.next_dirent_cookie as _,
                        d_reclen: reclen as _,
                        d_type: dirent_type(item.entry_type()) as u8,
                        d_name: Default::default(),
                    },
                );
            }
            let name_start = start + offset_of!(general::linux_dirent64, d_name);
            out[name_start..name_start + name.len()].copy_from_slice(name);
        }
        if !dir.synthetic_dirents_emitted && out.len() < max_len {
            let mut synthetic_dirents = Vec::new();
            for name in process.path_symlink_names_in_dir(dir.path.as_str()) {
                synthetic_dirents.push((name, general::DT_LNK as u8));
            }
            for &name in synthetic_block_device_names_in_dir(dir.path.as_str()) {
                synthetic_dirents.push((name.to_string(), general::DT_BLK as u8));
            }
            for &name in synthetic_char_device_names_in_dir(dir.path.as_str()) {
                synthetic_dirents.push((name.to_string(), general::DT_CHR as u8));
            }
            while dir.next_synthetic_dirent_index < synthetic_dirents.len() {
                let (name, d_type) = &synthetic_dirents[dir.next_synthetic_dirent_index];
                if seen_names.iter().any(|seen| seen == name) {
                    dir.next_synthetic_dirent_index += 1;
                    continue;
                }
                let name_bytes = name.as_bytes();
                let reclen = align_up(
                    offset_of!(general::linux_dirent64, d_name) + name_bytes.len() + 1,
                    8,
                );
                if out.len() + reclen > max_len {
                    break;
                }
                let entry_path = normalize_path(dir.path.as_str(), name);
                dir.next_dirent_cookie = dir.next_dirent_cookie.saturating_add(1);
                let start = out.len();
                out.resize(start + reclen, 0);
                unsafe {
                    let dirent = out[start..].as_mut_ptr() as *mut general::linux_dirent64;
                    ptr::write_unaligned(
                        dirent,
                        general::linux_dirent64 {
                            d_ino: path_inode(entry_path.as_deref()) as _,
                            d_off: dir.next_dirent_cookie as _,
                            d_reclen: reclen as _,
                            d_type: *d_type,
                            d_name: Default::default(),
                        },
                    );
                }
                let name_start = start + offset_of!(general::linux_dirent64, d_name);
                out[name_start..name_start + name_bytes.len()].copy_from_slice(name_bytes);
                dir.next_synthetic_dirent_index += 1;
            }
            if dir.next_synthetic_dirent_index >= synthetic_dirents.len() {
                dir.synthetic_dirents_emitted = true;
            }
        }
        Ok(out)
    }

    pub(super) fn read_file_at_into_fd(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                read_regular_file_at(process, file, offset, dst)
            }
            FdEntry::Memfd(file) => file.read_at(offset, dst),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn read_file_at_current_offset_into_fd(
        &mut self,
        process: &UserProcess,
        fd: i32,
        dst: &mut [u8],
    ) -> Result<(u64, usize), LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                let offset = *file.offset.lock();
                read_regular_file_at(process, file, offset, dst).map(|read| (offset, read))
            }
            FdEntry::Memfd(file) => file.read_at_current_offset(dst),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn advance_file_offset_fd(
        &mut self,
        fd: i32,
        amount: usize,
    ) -> Result<(), LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                let mut offset = file.offset.lock();
                *offset = offset.saturating_add(amount as u64);
                Ok(())
            }
            FdEntry::Memfd(file) => {
                let mut offset = file.offset.lock();
                *offset = offset.saturating_add(amount as u64);
                Ok(())
            }
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn mmap_read_file_at_into_fd(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EACCES);
                }
                read_regular_file_at(process, file, offset, dst)
            }
            FdEntry::Memfd(file) => file.read_at(offset, dst),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn mmap_file_backing(&mut self, fd: i32) -> Result<MmapFileBacking, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EACCES);
                }
                Ok(MmapFileBacking::File(file.clone()))
            }
            FdEntry::Memfd(file) => {
                if !file.readable() {
                    return Err(LinuxError::EACCES);
                }
                Ok(MmapFileBacking::Memfd(file.clone()))
            }
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn mmap_validate_file_fd(&mut self, fd: i32) -> Result<(), LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if file_is_readable(file.status_flags) {
                    Ok(())
                } else {
                    Err(LinuxError::EACCES)
                }
            }
            FdEntry::Memfd(file) => {
                if file.readable() {
                    Ok(())
                } else {
                    Err(LinuxError::EACCES)
                }
            }
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            FdEntry::DevZero(_) => Ok(()),
            FdEntry::DevRandom(_) | FdEntry::DevCpuDmaLatency(_) => Err(LinuxError::ENODEV),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn mmap_fd_allows_shared_write(&self, fd: i32) -> Result<bool, LinuxError> {
        match self.entry(fd)? {
            FdEntry::File(file) => Ok(file_is_writable(file.status_flags)),
            FdEntry::Memfd(file) => Ok(file.allows_shared_write()),
            FdEntry::DevZero(status_flags) => Ok(file_is_writable(*status_flags)),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) | FdEntry::SyntheticDir(_) => {
                Err(LinuxError::EISDIR)
            }
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn mmap_validate_file_fd_exists(&self, fd: i32) -> Result<(), LinuxError> {
        self.entry(fd).map(|_| ())
    }

    pub(super) fn insert_with_flags(
        &mut self,
        entry: FdEntry,
        fd_flags: u32,
    ) -> Result<i32, LinuxError> {
        self.insert_min_with_flags(entry, 0, fd_flags)
    }

    pub(super) fn insert_min_with_flags(
        &mut self,
        entry: FdEntry,
        min_fd: usize,
        fd_flags: u32,
    ) -> Result<i32, LinuxError> {
        if min_fd >= current_fd_table_limit() {
            discard_uninstalled_entry(entry);
            return Err(LinuxError::EMFILE);
        }
        let description_id = match entry.description_id().map_or_else(
            || OpenFileId::allocate().map_err(|_| LinuxError::ENFILE),
            Ok,
        ) {
            Ok(description_id) => description_id,
            Err(err) => {
                discard_uninstalled_entry(entry);
                return Err(err);
            }
        };
        self.insert_min_with_description(
            entry,
            min_fd,
            fd_flags,
            Arc::new(DescriptionIdentity::new(description_id)),
        )
    }

    fn insert_min_with_description(
        &mut self,
        entry: FdEntry,
        min_fd: usize,
        fd_flags: u32,
        description: Arc<DescriptionIdentity>,
    ) -> Result<i32, LinuxError> {
        let fd_limit = current_fd_table_limit();
        if min_fd >= fd_limit {
            discard_uninstalled_entry(entry);
            return Err(LinuxError::EMFILE);
        }
        if self.slots.len() < min_fd {
            self.slots.resize_with(min_fd, || None);
        }
        if let Some((idx, slot)) = self
            .slots
            .iter_mut()
            .enumerate()
            .take(fd_limit)
            .skip(min_fd)
            .find(|(_, slot)| slot.is_none())
        {
            track_exec_write_open(&entry);
            *slot = Some(FdSlot::with_identity(entry, fd_flags, description));
            return Ok(idx as i32);
        }
        if self.slots.len() >= fd_limit {
            discard_uninstalled_entry(entry);
            return Err(LinuxError::EMFILE);
        }
        track_exec_write_open(&entry);
        self.slots
            .push(Some(FdSlot::with_identity(entry, fd_flags, description)));
        Ok((self.slots.len() - 1) as i32)
    }

    pub(super) fn get_fd_flags(&self, fd: i32) -> Result<i32, LinuxError> {
        Ok(self.slot(fd)?.fd_flags as i32)
    }

    pub(super) fn set_fd_flags(&mut self, fd: i32, flags: u32) -> Result<i32, LinuxError> {
        self.slot_mut(fd)?.fd_flags = flags & general::FD_CLOEXEC;
        Ok(0)
    }

    pub(super) fn entry(&self, fd: i32) -> Result<&FdEntry, LinuxError> {
        Ok(&self.slot(fd)?.entry)
    }

    pub(super) fn entry_mut(&mut self, fd: i32) -> Result<&mut FdEntry, LinuxError> {
        Ok(&mut self.slot_mut(fd)?.entry)
    }

    pub(super) fn open_file_ref(&self, fd: i32) -> Result<Option<OpenFileRef>, LinuxError> {
        Ok(self.entry(fd)?.open_file().cloned())
    }

    fn slot(&self, fd: i32) -> Result<&FdSlot, LinuxError> {
        self.slots
            .get(fd as usize)
            .and_then(|slot| slot.as_ref())
            .ok_or(LinuxError::EBADF)
    }

    fn slot_mut(&mut self, fd: i32) -> Result<&mut FdSlot, LinuxError> {
        self.slots
            .get_mut(fd as usize)
            .and_then(|slot| slot.as_mut())
            .ok_or(LinuxError::EBADF)
    }

    fn duplicate_entry(&self, fd: i32) -> Result<(FdEntry, Arc<DescriptionIdentity>), LinuxError> {
        let slot = self.slot(fd)?;
        Ok((
            slot.entry.duplicate_for_fork()?,
            Arc::clone(&slot.description),
        ))
    }

    pub(super) fn file_description_key(&self, fd: i32) -> Result<Option<OpenFileId>, LinuxError> {
        Ok(Some(self.slot(fd)?.description_id()))
    }

    pub(super) fn pidfd_wait_target(&self, fd: i32) -> Result<(i32, bool), LinuxError> {
        match self.entry(fd)? {
            FdEntry::PidFd(pidfd) => Ok((pidfd.target_pid, pidfd.nonblocking())),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn pidfd_signal_target(
        &self,
        process: &UserProcess,
        fd: i32,
    ) -> Result<UserThreadEntry, LinuxError> {
        match self.entry(fd)? {
            FdEntry::PidFd(pidfd) => {
                if pidfd.exited() {
                    return Err(LinuxError::ESRCH);
                }
                let Some(target_process) = pidfd.target_process() else {
                    return Err(LinuxError::ESRCH);
                };
                user_thread_entry_for_process(target_process.as_ref()).ok_or(LinuxError::ESRCH)
            }
            FdEntry::Directory(dir) => proc_pid_signal_target(process, dir.path.as_str()),
            FdEntry::SyntheticDir(dir) => proc_pid_signal_target(process, dir.path.as_str()),
            FdEntry::Path(path) => proc_pid_signal_target(process, path.path.as_str()),
            _ => Err(LinuxError::EBADF),
        }
    }
}

fn proc_pid_signal_target(
    process: &UserProcess,
    path: &str,
) -> Result<UserThreadEntry, LinuxError> {
    const SYNTHETIC_INIT_PID: i32 = 1;

    let normalized = normalize_path("/", path).ok_or(LinuxError::EBADF)?;
    let target_pid = if normalized == "/proc/self" {
        process.pid()
    } else if let Some(rest) = normalized.strip_prefix("/proc/") {
        let Some(component) = rest.split('/').next() else {
            return Err(LinuxError::EBADF);
        };
        component.parse::<i32>().map_err(|_| LinuxError::EBADF)?
    } else {
        return Err(LinuxError::EBADF);
    };
    if target_pid == process.pid() {
        return user_thread_entry_for_process(process).ok_or(LinuxError::ESRCH);
    }
    user_thread_entry_by_process_pid(target_pid).ok_or_else(|| {
        if target_pid == SYNTHETIC_INIT_PID && process.uid() != 0 {
            LinuxError::EPERM
        } else {
            LinuxError::ESRCH
        }
    })
}

fn process_fd_access_allowed(caller: &UserProcess, target: &UserProcess) -> bool {
    if caller.uid() == 0 {
        return true;
    }
    let caller_real = caller.real_uid();
    let caller_effective = caller.uid();
    caller_real == target.real_uid()
        || caller_real == target.saved_uid()
        || caller_effective == target.real_uid()
        || caller_effective == target.saved_uid()
}

fn kcmp_file_description_key(
    process: &UserProcess,
    pid: i32,
    fd: i32,
) -> Result<Option<OpenFileId>, LinuxError> {
    if pid == process.pid() {
        return process.fds.lock().file_description_key(fd);
    }
    let entry = process
        .child_thread_entry_by_pid(pid)
        .or_else(|| user_thread_entry_by_process_pid(pid))
        .ok_or(LinuxError::ESRCH)?;
    if !process_fd_access_allowed(process, entry.process.as_ref()) {
        return Err(LinuxError::EPERM);
    }
    entry.process.fds.lock().file_description_key(fd)
}

fn get_proc_fd_dirents(
    dir: &mut ProcFdDirEntry,
    fd_names: &[String],
    max_len: usize,
) -> Result<Vec<u8>, LinuxError> {
    let min_reclen = align_up(offset_of!(general::linux_dirent64, d_name) + 1, 8);
    if max_len < min_reclen {
        return Err(LinuxError::EINVAL);
    }

    let total_entries = fd_names.len().saturating_add(2);
    let mut out = Vec::new();
    let mut index = dir.next_dirent_cookie as usize;
    while index < total_entries {
        let (name, d_type, entry_path) = if index == 0 {
            (".", general::DT_DIR as u8, dir.path.clone())
        } else if index == 1 {
            ("..", general::DT_DIR as u8, "/proc/self".into())
        } else {
            let name = fd_names[index - 2].as_str();
            let entry_path = format!("{}/{}", dir.path, name);
            (name, general::DT_LNK as u8, entry_path)
        };
        let name_bytes = name.as_bytes();
        let reclen = align_up(
            offset_of!(general::linux_dirent64, d_name) + name_bytes.len() + 1,
            8,
        );
        if out.len() + reclen > max_len {
            break;
        }
        let start = out.len();
        out.resize(start + reclen, 0);
        let next_cookie = index.saturating_add(1) as u64;
        unsafe {
            let dirent = out[start..].as_mut_ptr() as *mut general::linux_dirent64;
            ptr::write_unaligned(
                dirent,
                general::linux_dirent64 {
                    d_ino: path_inode(Some(entry_path.as_str())) as _,
                    d_off: next_cookie as _,
                    d_reclen: reclen as _,
                    d_type,
                    d_name: Default::default(),
                },
            );
        }
        let name_start = start + offset_of!(general::linux_dirent64, d_name);
        out[name_start..name_start + name_bytes.len()].copy_from_slice(name_bytes);
        dir.next_dirent_cookie = next_cookie;
        index += 1;
    }
    Ok(out)
}

fn get_synthetic_dirents(
    dir: &mut SyntheticDirEntry,
    max_len: usize,
) -> Result<Vec<u8>, LinuxError> {
    let min_reclen = align_up(offset_of!(general::linux_dirent64, d_name) + 1, 8);
    if max_len < min_reclen {
        return Err(LinuxError::EINVAL);
    }

    let total_entries = dir.dirents.len().saturating_add(2);
    let mut out = Vec::new();
    let mut index = dir.next_dirent_cookie as usize;
    while index < total_entries {
        let (name, d_type, entry_path): (&str, u8, &str) = if index == 0 {
            (".", general::DT_DIR as u8, dir.path.as_str())
        } else if index == 1 {
            ("..", general::DT_DIR as u8, dir.parent_path.as_str())
        } else {
            let dirent = &dir.dirents[index - 2];
            (dirent.name.as_str(), dirent.d_type, dirent.path.as_str())
        };
        let name_bytes = name.as_bytes();
        let reclen = align_up(
            offset_of!(general::linux_dirent64, d_name) + name_bytes.len() + 1,
            8,
        );
        if out.len() + reclen > max_len {
            break;
        }
        let start = out.len();
        out.resize(start + reclen, 0);
        let next_cookie = index.saturating_add(1) as u64;
        unsafe {
            let dirent = out[start..].as_mut_ptr() as *mut general::linux_dirent64;
            ptr::write_unaligned(
                dirent,
                general::linux_dirent64 {
                    d_ino: path_inode(Some(entry_path)) as _,
                    d_off: next_cookie as _,
                    d_reclen: reclen as _,
                    d_type,
                    d_name: Default::default(),
                },
            );
        }
        let name_start = start + offset_of!(general::linux_dirent64, d_name);
        out[name_start..name_start + name_bytes.len()].copy_from_slice(name_bytes);
        dir.next_dirent_cookie = next_cookie;
        index += 1;
    }
    Ok(out)
}

impl FdTable {
    pub(super) fn open(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        flags: u32,
        mode: u32,
    ) -> Result<i32, LinuxError> {
        let entry = open_fd_entry(process, self, dirfd, path, flags, mode)?;
        self.insert_with_flags(entry, fd_cloexec_flag(flags & general::O_CLOEXEC != 0))
    }

    pub(super) fn mkdirat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        mode: u32,
    ) -> Result<(), LinuxError> {
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        if path_exceeds_linux_limits(path) {
            return Err(LinuxError::ENAMETOOLONG);
        }
        let abs_path = resolve_dirfd_path(process, self, dirfd, path)?;
        let abs_path = process.resolve_parent_symlinks(abs_path.as_str())?;
        if process.path_symlink(abs_path.as_str()).is_some()
            || axfs::api::metadata(abs_path.as_str()).is_ok()
        {
            return Err(LinuxError::EEXIST);
        }
        let parent_st = check_parent_write_search_permission(process, abs_path.as_str())?;
        directory_create_dir(abs_path.as_str())?;
        record_created_path_metadata(process, abs_path, mode, true, &parent_st);
        Ok(())
    }

    pub(super) fn mknodat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        mode: u32,
        dev: u64,
    ) -> Result<(), LinuxError> {
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        if path_exceeds_linux_limits(path) {
            return Err(LinuxError::ENAMETOOLONG);
        }
        let node_type = mode & ST_MODE_TYPE_MASK;
        let node_type = match node_type {
            0 | ST_MODE_FILE => ST_MODE_FILE,
            ST_MODE_FIFO => ST_MODE_FIFO,
            ST_MODE_CHR | ST_MODE_BLK if process.fs_uid() == 0 => node_type,
            ST_MODE_CHR | ST_MODE_BLK => return Err(LinuxError::EPERM),
            ST_MODE_SOCKET if process.fs_uid() == 0 => node_type,
            ST_MODE_SOCKET => return Err(LinuxError::EPERM),
            ST_MODE_DIR | ST_MODE_LNK | ST_MODE_TYPE_MASK => {
                return Err(LinuxError::EINVAL);
            }
            _ => return Err(LinuxError::EINVAL),
        };
        let abs_path = resolve_dirfd_path(process, self, dirfd, path)?;
        let abs_path = process.resolve_parent_symlinks(abs_path.as_str())?;
        if process.path_symlink(abs_path.as_str()).is_some()
            || axfs::api::metadata(abs_path.as_str()).is_ok()
        {
            return Err(LinuxError::EEXIST);
        }
        let parent_st = check_parent_write_search_permission(process, abs_path.as_str())?;
        let mut opts = OpenOptions::new();
        opts.write(true);
        opts.create_new(true);
        File::open(abs_path.as_str(), &opts).map_err(LinuxError::from)?;
        record_created_path_metadata(process, abs_path.clone(), mode, false, &parent_st);
        match node_type {
            ST_MODE_FIFO => {
                process.set_path_special_mode(abs_path.clone(), ST_MODE_FIFO);
                process.remove_path_rdev(abs_path.as_str());
            }
            ST_MODE_CHR | ST_MODE_BLK | ST_MODE_SOCKET => {
                process.set_path_special_mode(abs_path.clone(), node_type);
                if matches!(node_type, ST_MODE_CHR | ST_MODE_BLK) {
                    process.set_path_rdev(abs_path, dev);
                } else {
                    process.remove_path_rdev(abs_path.as_str());
                }
            }
            _ => {
                process.remove_path_special_mode(abs_path.as_str());
                process.remove_path_rdev(abs_path.as_str());
            }
        }
        Ok(())
    }

    pub(super) fn linkat(
        &mut self,
        process: &UserProcess,
        olddirfd: i32,
        oldpath: &str,
        newdirfd: i32,
        newpath: &str,
        flags: u32,
    ) -> Result<(), LinuxError> {
        if flags & !general::AT_SYMLINK_FOLLOW != 0 {
            return Err(LinuxError::EINVAL);
        }
        if oldpath.is_empty() || newpath.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        if path_exceeds_linux_limits(oldpath) || path_exceeds_linux_limits(newpath) {
            return Err(LinuxError::ENAMETOOLONG);
        }

        let old_abs = resolve_dirfd_path(process, self, olddirfd, oldpath)?;
        let old_abs = if flags & general::AT_SYMLINK_FOLLOW != 0 {
            process
                .resolve_path_symlink(old_abs.as_str())?
                .unwrap_or(old_abs)
        } else {
            process.resolve_parent_symlinks(old_abs.as_str())?
        };
        let new_abs = resolve_dirfd_path(process, self, newdirfd, newpath)?;
        let new_abs = process.resolve_parent_symlinks(new_abs.as_str())?;

        if process.path_hardlink_exists(new_abs.as_str())
            || process.path_symlink(new_abs.as_str()).is_some()
            || stat_absolute_path(process, new_abs.as_str()).is_ok()
        {
            return Err(LinuxError::EEXIST);
        }

        // This layer does not hard-link synthetic per-process filesystems.
        // Linux reports EXDEV for a procfs source linked into a regular dir.
        if old_abs.starts_with("/proc/") || old_abs == "/proc" {
            return Err(LinuxError::EXDEV);
        }

        let backing_path = process
            .path_hardlink_backing(old_abs.as_str())
            .unwrap_or_else(|| old_abs.clone());
        let st = stat_absolute_path(process, backing_path.as_str())?;
        if st.st_mode & ST_MODE_TYPE_MASK == ST_MODE_DIR {
            return Err(LinuxError::EPERM);
        }
        if st.st_mode & ST_MODE_TYPE_MASK != ST_MODE_FILE {
            return Err(LinuxError::EPERM);
        }
        if process.paths_cross_mount(backing_path.as_str(), new_abs.as_str()) {
            return Err(LinuxError::EXDEV);
        }
        if process.path_on_readonly_mount(backing_path.as_str())
            || process.path_on_readonly_mount(new_abs.as_str())
        {
            return Err(LinuxError::EROFS);
        }
        check_parent_write_search_permission(process, new_abs.as_str())?;
        process.set_path_hardlink(backing_path.as_str(), new_abs, st.st_ino as u64);
        Ok(())
    }

    pub(super) fn unlinkat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        flags: u32,
    ) -> Result<(), LinuxError> {
        let remove_dir = flags & general::AT_REMOVEDIR != 0;
        let supported_flags = general::AT_REMOVEDIR;
        if flags & !supported_flags != 0 {
            return Err(LinuxError::EINVAL);
        }
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        if path_exceeds_linux_limits(path) {
            return Err(LinuxError::ENAMETOOLONG);
        }
        if remove_dir && last_path_component(path) == Some(".") {
            return Err(LinuxError::EINVAL);
        }
        let abs_path = resolve_dirfd_path(process, self, dirfd, path)?;
        let abs_path = process.resolve_parent_symlinks(abs_path.as_str())?;
        let abs_path = if let Some(backing_path) = dev_shm_host_path(abs_path.as_str()) {
            ensure_dev_shm_dir()?;
            backing_path
        } else {
            abs_path
        };
        let parent_st = check_parent_write_search_permission(process, abs_path.as_str())?;
        if let Some(backing_path) = process.path_hardlink_backing(abs_path.as_str()) {
            if backing_path != abs_path {
                if remove_dir {
                    return Err(LinuxError::ENOTDIR);
                }
                let st = stat_absolute_path(process, backing_path.as_str())?;
                check_inode_flags_allow_unlink(process, backing_path.as_str())?;
                check_sticky_parent_permission(process, &parent_st, &st)?;
                process.remove_path_hardlink(abs_path.as_str());
                process.remove_path_inode(abs_path.as_str());
                process.remove_path_inode_flags(abs_path.as_str());
                process.remove_path_special_mode(abs_path.as_str());
                process.remove_path_rdev(abs_path.as_str());
                process.remove_path_times(abs_path.as_str());
                process.clear_path_sparse_file(abs_path.as_str());
                invalidate_exec_image_cache(abs_path.as_str());
                return Ok(());
            }
        }
        let target_st = if let Some(st) = process.path_symlink_stat(abs_path.as_str()) {
            Some(apply_recorded_path_metadata(process, abs_path.as_str(), st))
        } else {
            match stat_absolute_path(process, abs_path.as_str()) {
                Ok(st) => Some(st),
                Err(LinuxError::ENOENT) if !remove_dir => None,
                Err(err) => return Err(err),
            }
        };
        if let Some(st) = target_st.as_ref() {
            check_inode_flags_allow_unlink(process, abs_path.as_str())?;
            check_sticky_parent_permission(process, &parent_st, st)?;
        }
        if process.path_symlink(abs_path.as_str()).is_some() {
            if remove_dir {
                return Err(LinuxError::ENOTDIR);
            }
            process.remove_path_symlink(abs_path.as_str());
            process.remove_path_inode_flags(abs_path.as_str());
            process.remove_path_times(abs_path.as_str());
            invalidate_exec_image_cache(abs_path.as_str());
            return Ok(());
        }
        let removed = if remove_dir {
            if process.has_mount_point(abs_path.as_str()) {
                return Err(LinuxError::EBUSY);
            }
            directory_remove_dir(abs_path.as_str())
        } else {
            directory_remove_file(abs_path.as_str())
        };
        if removed.is_ok() {
            process.remove_path_hardlink(abs_path.as_str());
            process.remove_path_inode(abs_path.as_str());
            process.remove_path_inode_flags(abs_path.as_str());
            process.remove_path_special_mode(abs_path.as_str());
            process.remove_path_rdev(abs_path.as_str());
            process.remove_path_times(abs_path.as_str());
            process.clear_path_sparse_file(abs_path.as_str());
            invalidate_exec_image_cache(abs_path.as_str());
        }
        removed
    }

    pub(super) fn stat(&mut self, fd: i32) -> Result<general::stat, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdin(_) => Ok(stdio_stat(true)),
            FdEntry::Stdout(_) | FdEntry::Stderr(_) => Ok(stdio_stat(false)),
            FdEntry::DevNull => Ok(dev_null_stat()),
            FdEntry::DevZero(_) => Ok(dev_zero_stat()),
            FdEntry::DevRandom(_) => Ok(PathEntry::synthetic_char("/dev/urandom").stat()),
            FdEntry::DevCpuDmaLatency(_) => Ok(dev_cpu_dma_latency_stat()),
            FdEntry::BlockDevice(dev) => Ok(PathEntry::synthetic_block(dev.path.as_str()).stat()),
            FdEntry::Rtc => Ok(stdio_stat(false)),
            FdEntry::File(file) => Ok(file_attr_to_stat(
                &file.file.get_attr().map_err(LinuxError::from)?,
                Some(file.path.as_str()),
            )),
            FdEntry::Directory(dir) => Ok(file_attr_to_stat(&dir.attr, Some(dir.path.as_str()))),
            FdEntry::ProcFdDir(dir) => Ok(proc_fd_dir_stat(dir.path.as_str())),
            FdEntry::SyntheticDir(dir) => Ok(proc_fd_dir_stat(dir.path.as_str())),
            FdEntry::Path(path) => Ok(path.stat()),
            FdEntry::MemoryFile(file) => Ok(file.stat()),
            FdEntry::Memfd(file) => Ok(file.stat()),
            FdEntry::ProcPagemap(file) => Ok(file.stat()),
            FdEntry::ProcTimerSlack(file) => Ok(file.stat()),
            FdEntry::Pipe(pipe) => Ok(pipe_endpoint(pipe)?.stat()),
            FdEntry::Socket(socket) => Ok(socket.stat()),
            FdEntry::LocalSocket(socket) => Ok(socket.stat()),
            FdEntry::EventFd(_) => Ok(PathEntry::synthetic_file("anon_inode:[eventfd]", 0).stat()),
            FdEntry::Inotify(_) => Ok(PathEntry::synthetic_file("anon_inode:[inotify]", 0).stat()),
            FdEntry::Epoll(_) => Ok(PathEntry::synthetic_file("anon_inode:[eventpoll]", 0).stat()),
            FdEntry::TimerFd(_) => Ok(PathEntry::synthetic_file("anon_inode:[timerfd]", 0).stat()),
            FdEntry::SignalFd(_) => {
                Ok(PathEntry::synthetic_file("anon_inode:[signalfd]", 0).stat())
            }
            FdEntry::PidFd(_) => Ok(PathEntry::synthetic_file("anon_inode:[pidfd]", 0).stat()),
            FdEntry::PosixMq(mq) => Ok(mq.stat()),
            FdEntry::ProcMqQueuesMax(entry) => Ok(entry.stat()),
            FdEntry::ProcSysFile(entry) => Ok(entry.stat()),
        }
    }

    pub(super) fn stat_with_recorded_path(
        &mut self,
        process: &UserProcess,
        fd: i32,
    ) -> Result<(Option<String>, general::stat), LinuxError> {
        let path = fd_entry_path(self.entry(fd)?).map(ToString::to_string);
        let st = self.stat(fd)?;
        let st = match path.as_deref() {
            Some(path) => apply_recorded_path_metadata(process, path, st),
            None => st,
        };
        Ok((path, st))
    }

    pub(super) fn statfs(&self, fd: i32) -> Result<general::statfs, LinuxError> {
        Ok(generic_statfs(fd_entry_statfs_path(self.entry(fd)?)))
    }

    pub(super) fn stat_path(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<general::stat, LinuxError> {
        self.stat_path_inner(process, dirfd, path, true)
    }

    fn stat_path_inner(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        check_parent_search: bool,
    ) -> Result<general::stat, LinuxError> {
        if check_parent_search && process.fs_uid() != 0 {
            let resolved_path = resolve_dirfd_path(process, self, dirfd, path)?;
            match self.parent_dirs_searchable(
                process,
                resolved_path.as_str(),
                process.fs_uid(),
                process.fs_gid(),
            )? {
                true => {}
                false => return Err(LinuxError::EACCES),
            }
        }
        match open_fd_entry(process, self, dirfd, path, O_PATH_FLAG, 0) {
            Ok(FdEntry::DevNull)
            | Ok(FdEntry::DevZero(_))
            | Ok(FdEntry::DevRandom(_))
            | Ok(FdEntry::DevCpuDmaLatency(_))
            | Ok(FdEntry::Rtc) => Ok(stdio_stat(false)),
            Ok(FdEntry::BlockDevice(dev)) => {
                Ok(PathEntry::synthetic_block(dev.path.as_str()).stat())
            }
            Ok(FdEntry::File(file)) => Ok(apply_recorded_path_metadata(
                process,
                file.path.as_str(),
                file_attr_to_stat(
                    &file.file.get_attr().map_err(LinuxError::from)?,
                    Some(file.path.as_str()),
                ),
            )),
            Ok(FdEntry::Directory(dir)) => Ok(apply_recorded_path_metadata(
                process,
                dir.path.as_str(),
                file_attr_to_stat(&dir.attr, Some(dir.path.as_str())),
            )),
            Ok(FdEntry::ProcFdDir(dir)) => Ok(proc_fd_dir_stat(dir.path.as_str())),
            Ok(FdEntry::SyntheticDir(dir)) => Ok(proc_fd_dir_stat(dir.path.as_str())),
            Ok(FdEntry::Path(path)) => Ok(apply_recorded_path_metadata(
                process,
                path.path.as_str(),
                path.stat(),
            )),
            Ok(FdEntry::MemoryFile(file)) => Ok(apply_recorded_path_metadata(
                process,
                file.path.as_str(),
                file.stat(),
            )),
            Ok(FdEntry::Memfd(file)) => Ok(file.stat()),
            Ok(FdEntry::ProcPagemap(file)) => Ok(apply_recorded_path_metadata(
                process,
                file.path.as_str(),
                file.stat(),
            )),
            Ok(FdEntry::ProcTimerSlack(file)) => Ok(apply_recorded_path_metadata(
                process,
                file.path.as_str(),
                file.stat_for_process(process),
            )),
            Ok(_) => Err(LinuxError::EINVAL),
            Err(err) => Err(err),
        }
    }

    pub(super) fn path_stat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<(String, general::stat), LinuxError> {
        let resolved_path = self.resolve_path(process, dirfd, path)?;
        let st = self.stat_path(process, dirfd, path)?;
        Ok((resolved_path, st))
    }

    pub(super) fn resolve_path(
        &self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<String, LinuxError> {
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        let normalized = if path.starts_with('/') {
            process.resolve_fs_absolute_path(path)?
        } else if dirfd == general::AT_FDCWD {
            let cwd = process.cwd();
            normalize_path(cwd.as_str(), path).ok_or(LinuxError::EINVAL)?
        } else {
            let base = match self.entry(dirfd)? {
                FdEntry::Directory(dir) => dir.path.as_str(),
                FdEntry::ProcFdDir(dir) => dir.path.as_str(),
                FdEntry::SyntheticDir(dir) => dir.path.as_str(),
                FdEntry::Path(path_entry) if path_entry.mode & ST_MODE_TYPE_MASK == ST_MODE_DIR => {
                    path_entry.path.as_str()
                }
                _ => return Err(LinuxError::ENOTDIR),
            };
            normalize_path(base, path).ok_or(LinuxError::EINVAL)?
        };
        Ok(canonical_permission_path(normalized))
    }

    pub(super) fn parent_dirs_searchable(
        &mut self,
        process: &UserProcess,
        path: &str,
        uid: u32,
        gid: u32,
    ) -> Result<bool, LinuxError> {
        if uid == 0 {
            return Ok(true);
        }
        let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
        if components.len() <= 1 {
            return Ok(true);
        }
        let mut parent = String::new();
        for component in &components[..components.len() - 1] {
            parent.push('/');
            parent.push_str(component);
            let st = self.stat_path_inner(process, general::AT_FDCWD, parent.as_str(), false)?;
            if st.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
                return Err(LinuxError::ENOTDIR);
            }
            if !access_allowed(&st, ACCESS_X_OK, uid, gid) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(super) fn statfs_path(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<general::statfs, LinuxError> {
        let resolved_path = self.resolve_path(process, dirfd, path)?;
        let entry = open_fd_entry(process, self, dirfd, path, O_PATH_FLAG, 0)?;
        let uid = process.fs_uid();
        if uid != 0 {
            if !self.parent_dirs_searchable(
                process,
                resolved_path.as_str(),
                uid,
                process.fs_gid(),
            )? {
                return Err(LinuxError::EACCES);
            }
        }
        let mut st = generic_statfs(fd_entry_statfs_path(&entry));
        if process.path_on_nosymfollow_mount(resolved_path.as_str()) {
            st.f_flags |= ST_NOSYMFOLLOW_FLAG as general::__kernel_long_t;
        }
        Ok(st)
    }

    pub(super) fn fcntl(
        &mut self,
        process: &UserProcess,
        fd: i32,
        cmd: u32,
        arg: usize,
    ) -> Result<i32, LinuxError> {
        const F_SETPIPE_SZ: u32 = 1031;
        const F_GETPIPE_SZ: u32 = 1032;
        if matches!(self.entry(fd)?, FdEntry::Path(_)) && cmd == general::F_GETFL {
            return Ok(O_PATH_FLAG as i32);
        }
        if matches!(self.entry(fd)?, FdEntry::LocalSocket(_)) {
            return match cmd {
                general::F_DUPFD => self.dup_min_with_flags(fd, arg as i32, 0),
                general::F_DUPFD_CLOEXEC => {
                    self.dup_min_with_flags(fd, arg as i32, general::FD_CLOEXEC)
                }
                general::F_GETFD => self.get_fd_flags(fd),
                general::F_SETFD => self.set_fd_flags(fd, arg as u32),
                general::F_GETFL => match self.entry(fd)? {
                    FdEntry::LocalSocket(socket) => Ok(socket.status_flags()),
                    _ => unreachable!(),
                },
                general::F_SETFL => match self.entry_mut(fd)? {
                    FdEntry::LocalSocket(socket) => {
                        socket.set_status_flags(arg as i32);
                        Ok(0)
                    }
                    _ => unreachable!(),
                },
                _ => Err(LinuxError::EINVAL),
            };
        }
        let socket = match self.entry(fd)? {
            FdEntry::Socket(socket) => Some(socket.clone()),
            _ => None,
        };
        if let Some(socket) = socket {
            return match cmd {
                general::F_DUPFD => self.dup_min_with_flags(fd, arg as i32, 0),
                general::F_DUPFD_CLOEXEC => {
                    self.dup_min_with_flags(fd, arg as i32, general::FD_CLOEXEC)
                }
                general::F_GETFD => self.get_fd_flags(fd),
                general::F_SETFD => self.set_fd_flags(fd, arg as u32),
                general::F_GETFL | general::F_SETFL => posix_ret_i32(arceos_posix_api::sys_fcntl(
                    socket.posix_fd,
                    cmd as i32,
                    arg,
                )),
                _ => Err(LinuxError::EINVAL),
            };
        }
        match cmd {
            general::F_DUPFD => self.dup_min_with_flags(fd, arg as i32, 0),
            general::F_DUPFD_CLOEXEC => {
                self.dup_min_with_flags(fd, arg as i32, general::FD_CLOEXEC)
            }
            general::F_GETFD => self.get_fd_flags(fd),
            general::F_SETFD => self.set_fd_flags(fd, arg as u32),
            general::F_GETFL => match self.entry(fd)? {
                FdEntry::Stdin(status_flags)
                | FdEntry::Stdout(status_flags)
                | FdEntry::Stderr(status_flags)
                | FdEntry::DevCpuDmaLatency(status_flags) => Ok(*status_flags as i32),
                FdEntry::File(file) => Ok(file.status_flags as i32),
                FdEntry::Memfd(file) => Ok(file.status_flags as i32),
                FdEntry::Pipe(pipe) => Ok(pipe.status_flags() as i32),
                FdEntry::EventFd(eventfd) => Ok(eventfd.status_flags() as i32),
                FdEntry::Inotify(inotify) => Ok(inotify.status_flags() as i32),
                FdEntry::TimerFd(timerfd) => Ok(timerfd.status_flags() as i32),
                FdEntry::SignalFd(signalfd) => Ok(signalfd.status_flags() as i32),
                FdEntry::PidFd(pidfd) => Ok(pidfd.status_flags() as i32),
                FdEntry::PosixMq(mq) => Ok(mq.status_flags() as i32),
                FdEntry::ProcMqQueuesMax(entry) => Ok(entry.status_flags() as i32),
                FdEntry::ProcSysFile(entry) => Ok(entry.status_flags() as i32),
                FdEntry::ProcTimerSlack(file) => Ok(file.status_flags as i32),
                _ => Err(LinuxError::EINVAL),
            },
            F_GETPIPE_SZ => Ok(self.pipe_capacity(fd)? as i32),
            F_SETPIPE_SZ => match self.entry(fd)? {
                FdEntry::Pipe(pipe) => Ok(pipe_endpoint(pipe)?.set_capacity(arg as usize)? as i32),
                _ => Err(LinuxError::EBADF),
            },
            general::F_GET_SEALS => match self.entry(fd)? {
                FdEntry::Memfd(file) => Ok(file.seals() as i32),
                _ => Err(LinuxError::EINVAL),
            },
            general::F_ADD_SEALS => match self.entry_mut(fd)? {
                FdEntry::Memfd(file) => {
                    let active_shared_writable_mmap =
                        process.has_shared_writable_mmap_for_memfd(file);
                    file.add_seals(arg as u32, active_shared_writable_mmap)
                }
                _ => Err(LinuxError::EINVAL),
            },
            general::F_SETOWN
            | general::F_GETOWN
            | general::F_SETSIG
            | general::F_GETSIG
            | 15
            | 16 => Err(LinuxError::EINVAL),
            general::F_SETFL => match self.entry_mut(fd)? {
                FdEntry::Stdin(status_flags)
                | FdEntry::Stdout(status_flags)
                | FdEntry::Stderr(status_flags)
                | FdEntry::DevCpuDmaLatency(status_flags) => {
                    *status_flags =
                        (*status_flags & general::O_ACCMODE) | fcntl_setfl_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::File(file) => {
                    file.status_flags =
                        (file.status_flags & general::O_ACCMODE) | fcntl_setfl_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::Memfd(file) => {
                    file.status_flags =
                        (file.status_flags & general::O_ACCMODE) | fcntl_setfl_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::Pipe(pipe) => {
                    let flags = pipe_endpoint(pipe)?
                        .status_flags_after_setfl(pipe.status_flags(), arg as u32);
                    pipe.set_status_flags(flags);
                    Ok(0)
                }
                FdEntry::EventFd(eventfd) => {
                    eventfd.set_status_flags(arg as u32 & general::O_NONBLOCK);
                    Ok(0)
                }
                FdEntry::Inotify(inotify) => {
                    inotify.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::TimerFd(timerfd) => {
                    timerfd.set_status_flags(arg as u32 & general::O_NONBLOCK);
                    Ok(0)
                }
                FdEntry::SignalFd(signalfd) => {
                    signalfd.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::PidFd(pidfd) => {
                    pidfd.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::PosixMq(mq) => {
                    mq.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::ProcMqQueuesMax(entry) => {
                    entry.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::ProcSysFile(entry) => {
                    entry.set_status_flags(arg as u32);
                    Ok(0)
                }
                _ => Err(LinuxError::EINVAL),
            },
            general::F_GETLK => self.fcntl_getlk(process, fd, arg),
            general::F_SETLK => self.fcntl_setlk(process, fd, arg, false),
            general::F_SETLKW => self.fcntl_setlk(process, fd, arg, true),
            general::F_OFD_GETLK => self.fcntl_getlk_ofd(process, fd, arg),
            general::F_OFD_SETLK => self.fcntl_setlk_ofd(process, fd, arg, false),
            general::F_OFD_SETLKW => self.fcntl_setlk_ofd(process, fd, arg, true),
            general::F_GETLEASE => self.fcntl_getlease(fd),
            general::F_SETLEASE => self.fcntl_setlease(fd, arg as u32),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn flock(&mut self, fd: i32, operation: u32) -> Result<i32, LinuxError> {
        let (key, owner) = match self.entry(fd)? {
            FdEntry::File(file) => (flock_key(file), flock_owner(file)),
            _ => return Err(LinuxError::EBADF),
        };
        apply_flock_operation(key, owner, operation)?;
        Ok(0)
    }

    fn fcntl_getlk(
        &mut self,
        process: &UserProcess,
        fd: i32,
        arg: usize,
    ) -> Result<i32, LinuxError> {
        let mut lock: general::flock = read_user_value(process, arg)?;
        validate_flock(&lock)?;
        let (key, request) = match self.entry(fd)? {
            FdEntry::File(file) => (
                record_lock_key(file),
                normalize_record_lock(file, process, &lock)?,
            ),
            _ => return Err(LinuxError::EBADF),
        };
        if let Some(conflict) = first_record_lock_conflict(key, &request) {
            lock.l_type = conflict.typ;
            lock.l_whence = general::SEEK_SET as _;
            lock.l_start = conflict.start as _;
            lock.l_len = conflict.len as _;
            lock.l_pid = if conflict.owner_id > 0 && conflict.owner_id <= i32::MAX as i64 {
                conflict.owner_id as _
            } else {
                -1
            };
        } else {
            lock.l_type = general::F_UNLCK as _;
        }
        if write_user_value(process, arg, &lock) == 0 {
            Ok(0)
        } else {
            Err(LinuxError::EFAULT)
        }
    }

    fn fcntl_setlk(
        &mut self,
        process: &UserProcess,
        fd: i32,
        arg: usize,
        wait: bool,
    ) -> Result<i32, LinuxError> {
        let lock: general::flock = read_user_value(process, arg)?;
        validate_flock(&lock)?;
        let (key, request) = match self.entry(fd)? {
            FdEntry::File(file) => {
                if !record_lock_access_allowed(file, lock.l_type as u32) {
                    return Err(LinuxError::EBADF);
                }
                (
                    record_lock_key(file),
                    normalize_record_lock(file, process, &lock)?,
                )
            }
            _ => return Err(LinuxError::EBADF),
        };
        apply_record_lock(process, key, request, wait)?;
        Ok(0)
    }

    fn fcntl_getlk_ofd(
        &mut self,
        process: &UserProcess,
        fd: i32,
        arg: usize,
    ) -> Result<i32, LinuxError> {
        let mut lock: general::flock = read_user_value(process, arg)?;
        validate_flock(&lock)?;
        let (key, request) = match self.entry(fd)? {
            FdEntry::File(file) => (
                record_lock_key(file),
                normalize_ofd_record_lock(file, &lock)?,
            ),
            _ => return Err(LinuxError::EBADF),
        };
        if let Some(conflict) = first_record_lock_conflict(key, &request) {
            lock.l_type = conflict.typ;
            lock.l_whence = general::SEEK_SET as _;
            lock.l_start = conflict.start as _;
            lock.l_len = conflict.len as _;
            lock.l_pid = if conflict.owner_id > 0 && conflict.owner_id <= i32::MAX as i64 {
                conflict.owner_id as _
            } else {
                -1
            };
        } else {
            lock.l_type = general::F_UNLCK as _;
        }
        if write_user_value(process, arg, &lock) == 0 {
            Ok(0)
        } else {
            Err(LinuxError::EFAULT)
        }
    }

    fn fcntl_setlk_ofd(
        &mut self,
        process: &UserProcess,
        fd: i32,
        arg: usize,
        wait: bool,
    ) -> Result<i32, LinuxError> {
        let lock: general::flock = read_user_value(process, arg)?;
        validate_flock(&lock)?;
        let (key, request) = match self.entry(fd)? {
            FdEntry::File(file) => {
                if !record_lock_access_allowed(file, lock.l_type as u32) {
                    return Err(LinuxError::EBADF);
                }
                (
                    record_lock_key(file),
                    normalize_ofd_record_lock(file, &lock)?,
                )
            }
            _ => return Err(LinuxError::EBADF),
        };
        apply_record_lock(process, key, request, wait)?;
        Ok(0)
    }

    fn fcntl_getlease(&mut self, fd: i32) -> Result<i32, LinuxError> {
        match self.entry(fd)? {
            FdEntry::File(file) => Ok(file_lease_type(file) as i32),
            _ => Err(LinuxError::EINVAL),
        }
    }

    fn fcntl_setlease(&mut self, fd: i32, lease_type: u32) -> Result<i32, LinuxError> {
        let file = match self.entry(fd)? {
            FdEntry::File(file) => file,
            _ => return Err(LinuxError::EINVAL),
        };
        match lease_type {
            general::F_RDLCK | general::F_WRLCK | general::F_UNLCK => {}
            _ => return Err(LinuxError::EINVAL),
        }
        if !file_lease_access_allowed(file, lease_type) {
            return Err(LinuxError::EAGAIN);
        }
        apply_file_lease(file, lease_type)?;
        Ok(0)
    }
}

impl PathEntry {
    pub(super) fn symlink(path: &str) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_LNK | 0o777,
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn from_attr(path: &str, attr: &FileAttr) -> Self {
        Self {
            path: path.into(),
            mode: file_type_mode(attr.file_type()) | attr.perm().bits() as u32,
            size: attr.size(),
            blocks: attr.blocks(),
        }
    }

    pub(super) fn from_metadata(path: &str, metadata: &axfs::api::Metadata) -> Self {
        Self {
            path: path.into(),
            mode: file_type_mode(metadata.file_type()) | metadata.permissions().bits() as u32,
            size: metadata.size(),
            blocks: metadata.blocks(),
        }
    }

    pub(super) fn fifo(path: &str, mode: u32) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_FIFO | (mode & 0o7777),
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn special_node(path: &str, ty: u32, mode: u32) -> Self {
        Self {
            path: path.into(),
            mode: (ty & ST_MODE_TYPE_MASK) | (mode & 0o7777),
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn synthetic_file(path: &str, size: usize) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_FILE | 0o444,
            size: size as u64,
            blocks: (size as u64).div_ceil(512),
        }
    }

    pub(super) fn synthetic_file_with_mode(path: &str, size: usize, mode: u32) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_FILE | (mode & 0o7777),
            size: size as u64,
            blocks: (size as u64).div_ceil(512),
        }
    }

    pub(super) fn synthetic_char(path: &str) -> Self {
        Self::synthetic_char_with_mode(path, 0o440)
    }

    pub(super) fn synthetic_char_with_mode(path: &str, mode: u32) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_CHR | (mode & 0o7777),
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn synthetic_block(path: &str) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_BLK | 0o660,
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn synthetic_dir(path: &str) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_DIR | 0o555,
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn stat(&self) -> general::stat {
        if self.mode & ST_MODE_TYPE_MASK == ST_MODE_CHR {
            return synthetic_char_stat_for_path(self.path.as_str(), self.mode);
        }
        if self.mode & ST_MODE_TYPE_MASK == ST_MODE_BLK {
            return synthetic_block_stat_for_path(self.path.as_str(), self.mode);
        }
        let mut st: general::stat = unsafe { core::mem::zeroed() };
        st.st_dev = 1;
        st.st_ino = path_inode(Some(self.path.as_str()));
        st.st_mode = self.mode;
        st.st_nlink = 1;
        st.st_size = self.size as _;
        st.st_blksize = 512;
        st.st_blocks = self.blocks as _;
        st
    }
}

impl SyntheticDirEntry {
    pub(super) fn new(path: String, parent_path: String, dirents: Vec<SyntheticDirent>) -> Self {
        Self {
            path,
            parent_path,
            dirents,
            next_dirent_cookie: 0,
        }
    }
}

impl SyntheticDirent {
    pub(super) fn new(name: String, d_type: u8, path: String) -> Self {
        Self { name, d_type, path }
    }
}

fn proc_fd_dir_stat(path: &str) -> general::stat {
    let mut st = PathEntry::synthetic_dir(path).stat();
    st.st_nlink = 2;
    st
}

impl MemoryFileEntry {
    pub(super) fn read(&mut self, dst: &mut [u8]) -> usize {
        let start = self.offset.min(self.data.len());
        let end = cmp::min(start + dst.len(), self.data.len());
        let len = end.saturating_sub(start);
        dst[..len].copy_from_slice(&self.data[start..end]);
        self.offset = end;
        len
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file(self.path.as_str(), self.data.len()).stat()
    }

    pub(super) fn seek(&mut self, pos: SeekFrom) -> Result<u64, LinuxError> {
        let next = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::Current(offset) => self.offset as i64 + offset,
            SeekFrom::End(offset) => self.data.len() as i64 + offset,
        };
        if next < 0 {
            return Err(LinuxError::EINVAL);
        }
        self.offset = next as usize;
        Ok(self.offset as u64)
    }
}

impl MemfdEntry {
    const SUPPORTED_SEALS: u32 = general::F_SEAL_SEAL
        | general::F_SEAL_SHRINK
        | general::F_SEAL_GROW
        | general::F_SEAL_WRITE;

    fn new(name: String, status_flags: u32, allow_sealing: bool) -> Self {
        Self {
            name,
            status_flags: fcntl_status_flags(status_flags),
            offset: Arc::new(Mutex::new(0)),
            state: Arc::new(Mutex::new(MemfdState {
                data: Vec::new(),
                seals: if allow_sealing {
                    0
                } else {
                    general::F_SEAL_SEAL
                },
            })),
        }
    }

    fn path(&self) -> String {
        format!("memfd:{} (deleted)", self.name)
    }

    fn reopen(&self, status_flags: u32) -> Self {
        Self {
            name: self.name.clone(),
            status_flags: fcntl_status_flags(status_flags),
            offset: Arc::new(Mutex::new(0)),
            state: self.state.clone(),
        }
    }

    fn readable(&self) -> bool {
        file_is_readable(self.status_flags)
    }

    fn writable(&self) -> bool {
        file_is_writable(self.status_flags)
    }

    fn seals(&self) -> u32 {
        self.state.lock().seals
    }

    fn sealed(&self, seal: u32) -> bool {
        self.seals() & seal != 0
    }

    fn allows_shared_write(&self) -> bool {
        self.writable() && !self.sealed(general::F_SEAL_WRITE)
    }

    pub(super) fn same_backing(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.state, &other.state)
    }

    fn add_seals(
        &mut self,
        seals: u32,
        active_shared_writable_mmap: bool,
    ) -> Result<i32, LinuxError> {
        if seals & !Self::SUPPORTED_SEALS != 0 {
            return Err(LinuxError::EINVAL);
        }
        if !self.writable() {
            return Err(LinuxError::EPERM);
        }
        let mut state = self.state.lock();
        if state.seals & general::F_SEAL_SEAL != 0 {
            return Err(LinuxError::EPERM);
        }
        if seals & general::F_SEAL_WRITE != 0 && active_shared_writable_mmap {
            return Err(LinuxError::EBUSY);
        }
        state.seals |= seals;
        Ok(0)
    }

    fn read(&mut self, dst: &mut [u8]) -> Result<usize, LinuxError> {
        if !self.readable() {
            return Err(LinuxError::EBADF);
        }
        let offset = *self.offset.lock();
        let read = self.read_at(offset, dst)?;
        *self.offset.lock() = offset.saturating_add(read as u64);
        Ok(read)
    }

    fn read_at(&self, offset: u64, dst: &mut [u8]) -> Result<usize, LinuxError> {
        if !self.readable() {
            return Err(LinuxError::EBADF);
        }
        let state = self.state.lock();
        let start = offset.min(usize::MAX as u64) as usize;
        if start >= state.data.len() {
            return Ok(0);
        }
        let len = cmp::min(dst.len(), state.data.len() - start);
        dst[..len].copy_from_slice(&state.data[start..start + len]);
        Ok(len)
    }

    fn read_at_current_offset(&mut self, dst: &mut [u8]) -> Result<(u64, usize), LinuxError> {
        if !self.readable() {
            return Err(LinuxError::EBADF);
        }
        let offset = *self.offset.lock();
        self.read_at(offset, dst).map(|read| (offset, read))
    }

    fn write(&mut self, src: &[u8], file_size_limit: Option<u64>) -> Result<usize, LinuxError> {
        let offset = if self.status_flags & general::O_APPEND != 0 {
            self.size()
        } else {
            *self.offset.lock()
        };
        let written = self.write_at(offset, src, file_size_limit)?;
        *self.offset.lock() = offset.saturating_add(written as u64);
        Ok(written)
    }

    fn write_at(
        &mut self,
        offset: u64,
        src: &[u8],
        file_size_limit: Option<u64>,
    ) -> Result<usize, LinuxError> {
        if !self.writable() {
            return Err(LinuxError::EBADF);
        }
        if self.sealed(general::F_SEAL_WRITE) {
            return Err(LinuxError::EPERM);
        }
        let src = limit_regular_file_write_len(src, file_size_limit, offset)?;
        if src.is_empty() {
            return Ok(0);
        }
        let end = offset
            .checked_add(src.len() as u64)
            .ok_or(LinuxError::EFBIG)?;
        if end > MAX_IN_MEMORY_FILE_SIZE {
            return Err(LinuxError::EFBIG);
        }
        let mut state = self.state.lock();
        if end > state.data.len() as u64 && state.seals & general::F_SEAL_GROW != 0 {
            return Err(LinuxError::EPERM);
        }
        let start = offset as usize;
        let end_usize = end as usize;
        if end_usize > state.data.len() {
            let additional = end_usize - state.data.len();
            state
                .data
                .try_reserve_exact(additional)
                .map_err(|_| LinuxError::ENOMEM)?;
            state.data.resize(end_usize, 0);
        }
        state.data[start..end_usize].copy_from_slice(src);
        Ok(src.len())
    }

    fn truncate(&mut self, size: u64) -> Result<(), LinuxError> {
        if !self.writable() {
            return Err(LinuxError::EINVAL);
        }
        if size > MAX_IN_MEMORY_FILE_SIZE {
            return Err(LinuxError::EFBIG);
        }
        let mut state = self.state.lock();
        let current = state.data.len() as u64;
        if size < current && state.seals & general::F_SEAL_SHRINK != 0 {
            return Err(LinuxError::EPERM);
        }
        if size > current && state.seals & general::F_SEAL_GROW != 0 {
            return Err(LinuxError::EPERM);
        }
        let size = usize::try_from(size).map_err(|_| LinuxError::EFBIG)?;
        if size > state.data.len() {
            let additional = size - state.data.len();
            state
                .data
                .try_reserve_exact(additional)
                .map_err(|_| LinuxError::ENOMEM)?;
        }
        state.data.resize(size, 0);
        Ok(())
    }

    fn fallocate_keep_size(&self) -> Result<(), LinuxError> {
        if !self.writable() {
            return Err(LinuxError::EBADF);
        }
        if self.sealed(general::F_SEAL_WRITE) {
            return Err(LinuxError::EPERM);
        }
        Ok(())
    }

    fn punch_hole(&mut self, offset: u64, len: u64) -> Result<(), LinuxError> {
        if !self.writable() {
            return Err(LinuxError::EBADF);
        }
        if self.sealed(general::F_SEAL_WRITE) {
            return Err(LinuxError::EPERM);
        }
        let end = offset.checked_add(len).ok_or(LinuxError::EINVAL)?;
        let mut state = self.state.lock();
        let start = offset.min(state.data.len() as u64) as usize;
        let end = end.min(state.data.len() as u64) as usize;
        if start < end {
            state.data[start..end].fill(0);
        }
        Ok(())
    }

    fn size(&self) -> u64 {
        self.state.lock().data.len() as u64
    }

    fn stat(&self) -> general::stat {
        PathEntry::synthetic_file(self.path().as_str(), self.size() as usize).stat()
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, LinuxError> {
        let size = self.size();
        let mut offset = self.offset.lock();
        let next = match pos {
            SeekFrom::Start(pos) => Some(pos),
            SeekFrom::Current(off) => (*offset).checked_add_signed(off),
            SeekFrom::End(off) => size.checked_add_signed(off),
        }
        .ok_or(LinuxError::EINVAL)?;
        *offset = next;
        Ok(next)
    }
}

impl MmapFileBacking {
    pub(super) fn is_memfd_backing(&self, file: &MemfdEntry) -> bool {
        match self {
            Self::Memfd(backing) => backing.same_backing(file),
            Self::File(_) => false,
        }
    }
}

impl ProcTimerSlackEntry {
    fn target_timer_slack_ns(&self, process: &UserProcess) -> Result<u64, LinuxError> {
        if self.target_pid == process.pid() {
            return Ok(process.timer_slack_ns());
        }
        user_thread_entry_by_process_pid(self.target_pid)
            .map(|entry| entry.process.timer_slack_ns())
            .ok_or(LinuxError::ESRCH)
    }

    fn target_default_timer_slack_ns(&self, process: &UserProcess) -> Result<u64, LinuxError> {
        if self.target_pid == process.pid() {
            return Ok(process.default_timer_slack_ns());
        }
        user_thread_entry_by_process_pid(self.target_pid)
            .map(|entry| entry.process.default_timer_slack_ns())
            .ok_or(LinuxError::ESRCH)
    }

    fn set_target_timer_slack_ns(
        &self,
        process: &UserProcess,
        value: u64,
    ) -> Result<(), LinuxError> {
        if self.target_pid == process.pid() {
            process.set_timer_slack_ns(value);
            return Ok(());
        }
        let entry = user_thread_entry_by_process_pid(self.target_pid).ok_or(LinuxError::ESRCH)?;
        entry.process.set_timer_slack_ns(value);
        Ok(())
    }

    pub(super) fn read(
        &mut self,
        process: &UserProcess,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        let data = format!("{}\n", self.target_timer_slack_ns(process)?).into_bytes();
        let start = self.offset.min(data.len());
        let end = cmp::min(start + dst.len(), data.len());
        let len = end.saturating_sub(start);
        dst[..len].copy_from_slice(&data[start..end]);
        self.offset = end;
        Ok(len)
    }

    pub(super) fn write(&mut self, process: &UserProcess, src: &[u8]) -> Result<usize, LinuxError> {
        let text = core::str::from_utf8(src).map_err(|_| LinuxError::EINVAL)?;
        let value = text.trim().parse::<u64>().map_err(|_| LinuxError::EINVAL)?;
        let value = if value == 0 {
            self.target_default_timer_slack_ns(process)?
        } else {
            value
        };
        self.set_target_timer_slack_ns(process, value)?;
        self.offset = 0;
        Ok(src.len())
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file_with_mode(self.path.as_str(), 0, 0o644).stat()
    }

    pub(super) fn stat_for_process(&self, process: &UserProcess) -> general::stat {
        let size = self
            .target_timer_slack_ns(process)
            .map(|value| format!("{value}\n").len())
            .unwrap_or(0);
        PathEntry::synthetic_file_with_mode(self.path.as_str(), size, 0o644).stat()
    }

    pub(super) fn seek(&mut self, pos: SeekFrom) -> Result<u64, LinuxError> {
        let next = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::Current(offset) => self.offset as i64 + offset,
            SeekFrom::End(_) => return Err(LinuxError::EINVAL),
        };
        if next < 0 {
            return Err(LinuxError::EINVAL);
        }
        self.offset = next as usize;
        Ok(self.offset as u64)
    }
}

impl ProcPagemapEntry {
    const PRESENT: u64 = 1u64 << 63;
    const ENTRY_SIZE: u64 = size_of::<u64>() as u64;

    pub(super) fn read(&mut self, dst: &mut [u8]) -> usize {
        if self.offset >= self.size {
            return 0;
        }
        let available = (self.size - self.offset).min(dst.len() as u64) as usize;
        let mut written = 0usize;
        while written < available {
            let page_index = self.offset / Self::ENTRY_SIZE;
            let entry_offset = (self.offset % Self::ENTRY_SIZE) as usize;
            let entry = self.page_entry(page_index).to_ne_bytes();
            let copy_len = cmp::min(entry.len() - entry_offset, available - written);
            dst[written..written + copy_len]
                .copy_from_slice(&entry[entry_offset..entry_offset + copy_len]);
            self.offset += copy_len as u64;
            written += copy_len;
        }
        written
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file(self.path.as_str(), self.size as usize).stat()
    }

    pub(super) fn seek(&mut self, pos: SeekFrom) -> Result<u64, LinuxError> {
        let next = match pos {
            SeekFrom::Start(offset) => offset as i128,
            SeekFrom::Current(offset) => self.offset as i128 + offset as i128,
            SeekFrom::End(offset) => self.size as i128 + offset as i128,
        };
        if !(0..=u64::MAX as i128).contains(&next) {
            return Err(LinuxError::EINVAL);
        }
        self.offset = next as u64;
        Ok(self.offset)
    }

    fn page_entry(&self, page_index: u64) -> u64 {
        if self
            .present_ranges
            .iter()
            .any(|(start, end)| *start <= page_index && page_index < *end)
        {
            Self::PRESENT
        } else {
            0
        }
    }
}

fn validate_flock(lock: &general::flock) -> Result<(), LinuxError> {
    match lock.l_type as u32 {
        general::F_RDLCK | general::F_WRLCK | general::F_UNLCK => {}
        _ => return Err(LinuxError::EINVAL),
    }
    match lock.l_whence as u32 {
        general::SEEK_SET | general::SEEK_CUR | general::SEEK_END => Ok(()),
        _ => Err(LinuxError::EINVAL),
    }
}

#[derive(Clone)]
struct PosixRecordLock {
    owner_id: i64,
    typ: i16,
    start: i64,
    len: i64,
}

impl PosixRecordLock {
    fn end(&self) -> i64 {
        if self.len == 0 {
            i64::MAX
        } else {
            self.start.saturating_add(self.len)
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end() && other.start < self.end()
    }

    fn conflicts_with(&self, request: &Self) -> bool {
        if self.owner_id == request.owner_id || !self.overlaps(request) {
            return false;
        }
        match request.typ as u32 {
            general::F_RDLCK => self.typ as u32 == general::F_WRLCK,
            general::F_WRLCK => {
                matches!(self.typ as u32, general::F_RDLCK | general::F_WRLCK)
            }
            _ => false,
        }
    }
}

fn posix_record_lock_table() -> &'static Mutex<BTreeMap<u64, Vec<PosixRecordLock>>> {
    static POSIX_RECORD_LOCKS: LazyInit<Mutex<BTreeMap<u64, Vec<PosixRecordLock>>>> =
        LazyInit::new();
    let _ = POSIX_RECORD_LOCKS.call_once(|| Mutex::new(BTreeMap::new()));
    &POSIX_RECORD_LOCKS
}

fn record_lock_wait_table() -> &'static Mutex<BTreeMap<i64, i64>> {
    static RECORD_LOCK_WAITS: LazyInit<Mutex<BTreeMap<i64, i64>>> = LazyInit::new();
    let _ = RECORD_LOCK_WAITS.call_once(|| Mutex::new(BTreeMap::new()));
    &RECORD_LOCK_WAITS
}

fn record_lock_wait_would_deadlock(
    waits: &BTreeMap<i64, i64>,
    waiting_owner: i64,
    mut blocking_owner: i64,
) -> bool {
    while let Some(next_owner) = waits.get(&blocking_owner).copied() {
        if next_owner == waiting_owner {
            return true;
        }
        blocking_owner = next_owner;
    }
    blocking_owner == waiting_owner
}

fn record_lock_key(file: &FileEntry) -> u64 {
    path_inode(Some(file.path.as_str()))
}

fn ofd_record_lock_owner(file: &FileEntry) -> i64 {
    let ptr = Arc::as_ptr(&file.offset) as usize as u64;
    // Keep OFD owners disjoint from positive process-id POSIX owners.  The
    // descriptor offset Arc is shared by dup/forked file descriptions and unique
    // for independent open() calls, matching Linux open-file-description
    // lock ownership boundaries.
    -((ptr & (i64::MAX as u64)) as i64) - 1
}

fn record_lock_access_allowed(file: &FileEntry, typ: u32) -> bool {
    match typ {
        general::F_RDLCK => file_is_readable(file.status_flags),
        general::F_WRLCK => file_is_writable(file.status_flags),
        general::F_UNLCK => true,
        _ => false,
    }
}

fn normalize_record_lock(
    file: &FileEntry,
    process: &UserProcess,
    lock: &general::flock,
) -> Result<PosixRecordLock, LinuxError> {
    normalize_record_lock_with_owner(file, process.pid() as i64, lock)
}

fn normalize_ofd_record_lock(
    file: &FileEntry,
    lock: &general::flock,
) -> Result<PosixRecordLock, LinuxError> {
    normalize_record_lock_with_owner(file, ofd_record_lock_owner(file), lock)
}

fn normalize_record_lock_with_owner(
    file: &FileEntry,
    owner_id: i64,
    lock: &general::flock,
) -> Result<PosixRecordLock, LinuxError> {
    let base = match lock.l_whence as u32 {
        general::SEEK_SET => 0,
        general::SEEK_CUR => {
            i64::try_from(*file.offset.lock()).map_err(|_| LinuxError::EOVERFLOW)?
        }
        general::SEEK_END => i64::try_from(file.file.get_attr().map_err(LinuxError::from)?.size())
            .map_err(|_| LinuxError::EOVERFLOW)?,
        _ => return Err(LinuxError::EINVAL),
    };
    let mut start = base
        .checked_add(lock.l_start as i64)
        .ok_or(LinuxError::EINVAL)?;
    let mut len = lock.l_len as i64;
    if len < 0 {
        start = start.checked_add(len).ok_or(LinuxError::EINVAL)?;
        len = len.checked_neg().ok_or(LinuxError::EINVAL)?;
    }
    if start < 0 {
        return Err(LinuxError::EINVAL);
    }
    if len != 0 {
        start.checked_add(len).ok_or(LinuxError::EINVAL)?;
    }
    Ok(PosixRecordLock {
        owner_id,
        typ: lock.l_type,
        start,
        len,
    })
}

fn lock_len_from_range(start: i64, end: i64) -> i64 {
    if end == i64::MAX {
        0
    } else {
        end.saturating_sub(start)
    }
}

fn cleanup_dead_record_locks(locks: &mut Vec<PosixRecordLock>, current_owner_id: i64) {
    locks.retain(|lock| {
        lock.owner_id == current_owner_id
            || lock.owner_id < 0
            || (lock.owner_id <= i32::MAX as i64
                && user_thread_entry_by_process_pid(lock.owner_id as i32).is_some())
    });
}

fn merge_record_locks(locks: &mut Vec<PosixRecordLock>) {
    locks.sort_by_key(|lock| (lock.owner_id, lock.typ, lock.start, lock.end()));
    let mut merged: Vec<PosixRecordLock> = Vec::new();
    for lock in locks.drain(..) {
        if let Some(last) = merged.last_mut() {
            if last.owner_id == lock.owner_id && last.typ == lock.typ && lock.start <= last.end() {
                let end = last.end().max(lock.end());
                last.len = lock_len_from_range(last.start, end);
                continue;
            }
        }
        merged.push(lock);
    }
    *locks = merged;
}

fn remove_record_lock_range(locks: &mut Vec<PosixRecordLock>, request: &PosixRecordLock) {
    let request_end = request.end();
    let mut next = Vec::new();
    for lock in locks.drain(..) {
        if lock.owner_id != request.owner_id || !lock.overlaps(request) {
            next.push(lock);
            continue;
        }
        let lock_end = lock.end();
        if lock.start < request.start {
            next.push(PosixRecordLock {
                len: lock_len_from_range(lock.start, request.start),
                ..lock.clone()
            });
        }
        if request_end < lock_end {
            next.push(PosixRecordLock {
                start: request_end,
                len: lock_len_from_range(request_end, lock_end),
                ..lock
            });
        }
    }
    *locks = next;
}

fn first_record_lock_conflict(key: u64, request: &PosixRecordLock) -> Option<PosixRecordLock> {
    let mut table = posix_record_lock_table().lock();
    let locks = table.get_mut(&key)?;
    cleanup_dead_record_locks(locks, request.owner_id);
    locks.sort_by_key(|lock| (lock.start, lock.end(), lock.owner_id));
    let conflict = locks
        .iter()
        .find(|lock| lock.conflicts_with(request))
        .cloned();
    if locks.is_empty() {
        table.remove(&key);
    }
    conflict
}

fn record_lock_wait_interrupted(process: &UserProcess, owner_id: i64) -> bool {
    if process.pending_exit_group().is_some()
        || process.eval_watchdog_expired()
        || current_unblocked_signal_pending()
    {
        record_lock_wait_table().lock().remove(&owner_id);
        true
    } else {
        false
    }
}

fn apply_record_lock(
    process: &UserProcess,
    key: u64,
    request: PosixRecordLock,
    wait: bool,
) -> Result<(), LinuxError> {
    loop {
        if wait && record_lock_wait_interrupted(process, request.owner_id) {
            return Err(LinuxError::EINTR);
        }
        let mut table = posix_record_lock_table().lock();
        let locks = table.entry(key).or_insert_with(Vec::new);
        cleanup_dead_record_locks(locks, request.owner_id);
        let conflict_owner = (request.typ as u32 != general::F_UNLCK)
            .then(|| {
                locks
                    .iter()
                    .find(|lock| lock.conflicts_with(&request))
                    .map(|lock| lock.owner_id)
            })
            .flatten();
        if let Some(conflict_owner) = conflict_owner {
            if !wait {
                if locks.is_empty() {
                    table.remove(&key);
                }
                return Err(LinuxError::EAGAIN);
            }
            if record_lock_wait_interrupted(process, request.owner_id) {
                if locks.is_empty() {
                    table.remove(&key);
                }
                return Err(LinuxError::EINTR);
            }
            {
                let mut waits = record_lock_wait_table().lock();
                if record_lock_wait_would_deadlock(&waits, request.owner_id, conflict_owner) {
                    waits.remove(&request.owner_id);
                    return Err(LinuxError::EDEADLK);
                }
                waits.insert(request.owner_id, conflict_owner);
            }
            drop(table);
            // F_SETLKW is a blocking syscall.  Sleep for the normal poll wait
            // quantum instead of hot-spinning on yield_now(), so the lock owner
            // (and helper signalers used by lock tests) can run and release the
            // conflicting region on single-core evaluator kernels.
            yield_poll_wait();
            continue;
        }
        record_lock_wait_table().lock().remove(&request.owner_id);
        remove_record_lock_range(locks, &request);
        if request.typ as u32 != general::F_UNLCK {
            locks.push(request);
            merge_record_locks(locks);
        }
        if locks.is_empty() {
            table.remove(&key);
        }
        return Ok(());
    }
}

fn release_posix_record_locks_for_file_owner(key: u64, owner_pid: i32) {
    let mut table = posix_record_lock_table().lock();
    let should_remove = if let Some(locks) = table.get_mut(&key) {
        let owner_id = owner_pid as i64;
        locks.retain(|lock| lock.owner_id != owner_id);
        record_lock_wait_table().lock().remove(&owner_id);
        locks.is_empty()
    } else {
        false
    };
    if should_remove {
        table.remove(&key);
    }
}

fn release_ofd_record_locks_on_last_close(file: &FileEntry) {
    if Arc::strong_count(&file.offset) != 1 {
        return;
    }

    let owner_id = ofd_record_lock_owner(file);
    let key = record_lock_key(file);
    record_lock_wait_table().lock().remove(&owner_id);

    let mut table = posix_record_lock_table().lock();
    let should_remove = if let Some(locks) = table.get_mut(&key) {
        locks.retain(|lock| lock.owner_id != owner_id);
        locks.is_empty()
    } else {
        false
    };
    if should_remove {
        table.remove(&key);
    }
}

pub(super) fn release_posix_record_locks_for_process(owner_pid: i32) {
    let mut table = posix_record_lock_table().lock();
    let owner_id = owner_pid as i64;
    record_lock_wait_table().lock().remove(&owner_id);
    let empty_keys: Vec<u64> = table
        .iter_mut()
        .filter_map(|(key, locks)| {
            locks.retain(|lock| lock.owner_id != owner_id);
            locks.is_empty().then_some(*key)
        })
        .collect();
    for key in empty_keys {
        table.remove(&key);
    }
}

struct FileLeaseState {
    exclusive_owner: Option<usize>,
    shared_owners: Vec<usize>,
}

impl FileLeaseState {
    fn new() -> Self {
        Self {
            exclusive_owner: None,
            shared_owners: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.exclusive_owner.is_none() && self.shared_owners.is_empty()
    }

    fn lease_for(&self, owner: usize) -> u32 {
        if self.exclusive_owner == Some(owner) {
            general::F_WRLCK
        } else if self.shared_owners.contains(&owner) {
            general::F_RDLCK
        } else {
            general::F_UNLCK
        }
    }

    fn unlock(&mut self, owner: usize) {
        if self.exclusive_owner == Some(owner) {
            self.exclusive_owner = None;
        }
        self.shared_owners.retain(|held_owner| *held_owner != owner);
    }

    fn lock_shared(&mut self, owner: usize) -> Result<(), LinuxError> {
        if matches!(self.exclusive_owner, Some(held_owner) if held_owner != owner) {
            return Err(LinuxError::EAGAIN);
        }
        self.exclusive_owner = None;
        if !self.shared_owners.contains(&owner) {
            self.shared_owners.push(owner);
        }
        Ok(())
    }

    fn lock_exclusive(&mut self, owner: usize) -> Result<(), LinuxError> {
        if matches!(self.exclusive_owner, Some(held_owner) if held_owner != owner) {
            return Err(LinuxError::EAGAIN);
        }
        if self
            .shared_owners
            .iter()
            .any(|held_owner| *held_owner != owner)
        {
            return Err(LinuxError::EAGAIN);
        }
        self.shared_owners.retain(|held_owner| *held_owner != owner);
        self.exclusive_owner = Some(owner);
        Ok(())
    }
}

fn file_lease_table() -> &'static Mutex<BTreeMap<u64, FileLeaseState>> {
    static FILE_LEASES: LazyInit<Mutex<BTreeMap<u64, FileLeaseState>>> = LazyInit::new();
    let _ = FILE_LEASES.call_once(|| Mutex::new(BTreeMap::new()));
    &FILE_LEASES
}

fn file_lease_key(file: &FileEntry) -> u64 {
    path_inode(Some(file.path.as_str()))
}

fn file_lease_owner(file: &FileEntry) -> usize {
    Arc::as_ptr(&file.offset) as usize
}

fn file_lease_access_allowed(file: &FileEntry, lease_type: u32) -> bool {
    match lease_type {
        general::F_RDLCK => {
            file_is_readable(file.status_flags) && !file_is_writable(file.status_flags)
        }
        general::F_WRLCK => file_is_writable(file.status_flags),
        general::F_UNLCK => true,
        _ => false,
    }
}

fn file_lease_type(file: &FileEntry) -> u32 {
    file_lease_table()
        .lock()
        .get(&file_lease_key(file))
        .map(|state| state.lease_for(file_lease_owner(file)))
        .unwrap_or(general::F_UNLCK)
}

fn apply_file_lease(file: &FileEntry, lease_type: u32) -> Result<(), LinuxError> {
    let key = file_lease_key(file);
    let owner = file_lease_owner(file);
    let mut table = file_lease_table().lock();
    match lease_type {
        general::F_UNLCK => {
            let should_remove = if let Some(state) = table.get_mut(&key) {
                state.unlock(owner);
                state.is_empty()
            } else {
                false
            };
            if should_remove {
                table.remove(&key);
            }
            Ok(())
        }
        general::F_RDLCK | general::F_WRLCK => {
            let state = table.entry(key).or_insert_with(FileLeaseState::new);
            let result = if lease_type == general::F_RDLCK {
                state.lock_shared(owner)
            } else {
                state.lock_exclusive(owner)
            };
            let should_remove = result.is_err() && state.is_empty();
            if should_remove {
                table.remove(&key);
            }
            result
        }
        _ => Err(LinuxError::EINVAL),
    }
}

fn release_file_lease_on_last_close(file: &FileEntry) {
    if Arc::strong_count(&file.offset) == 1 {
        let _ = apply_file_lease(file, general::F_UNLCK);
    }
}

struct FlockState {
    exclusive_owner: Option<usize>,
    shared_owners: Vec<usize>,
}

impl FlockState {
    fn new() -> Self {
        Self {
            exclusive_owner: None,
            shared_owners: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.exclusive_owner.is_none() && self.shared_owners.is_empty()
    }

    fn unlock(&mut self, owner: usize) {
        if self.exclusive_owner == Some(owner) {
            self.exclusive_owner = None;
        }
        self.shared_owners.retain(|held_owner| *held_owner != owner);
    }

    fn lock_shared(&mut self, owner: usize) -> Result<(), LinuxError> {
        if matches!(self.exclusive_owner, Some(held_owner) if held_owner != owner) {
            return Err(LinuxError::EAGAIN);
        }
        self.exclusive_owner = None;
        if !self.shared_owners.contains(&owner) {
            self.shared_owners.push(owner);
        }
        Ok(())
    }

    fn lock_exclusive(&mut self, owner: usize) -> Result<(), LinuxError> {
        if matches!(self.exclusive_owner, Some(held_owner) if held_owner != owner) {
            return Err(LinuxError::EAGAIN);
        }
        if self
            .shared_owners
            .iter()
            .any(|held_owner| *held_owner != owner)
        {
            return Err(LinuxError::EAGAIN);
        }
        self.shared_owners.retain(|held_owner| *held_owner != owner);
        self.exclusive_owner = Some(owner);
        Ok(())
    }
}

fn flock_table() -> &'static Mutex<BTreeMap<u64, FlockState>> {
    static FLOCKS: LazyInit<Mutex<BTreeMap<u64, FlockState>>> = LazyInit::new();
    let _ = FLOCKS.call_once(|| Mutex::new(BTreeMap::new()));
    &FLOCKS
}

fn flock_key(file: &FileEntry) -> u64 {
    path_inode(Some(file.path.as_str()))
}

fn flock_owner(file: &FileEntry) -> usize {
    Arc::as_ptr(&file.offset) as usize
}

fn release_flock_on_last_close(file: &FileEntry) {
    if Arc::strong_count(&file.offset) == 1 {
        release_flock_owner(flock_key(file), flock_owner(file));
    }
}

fn release_flock_owner(key: u64, owner: usize) {
    let mut table = flock_table().lock();
    let should_remove = if let Some(state) = table.get_mut(&key) {
        state.unlock(owner);
        state.is_empty()
    } else {
        false
    };
    if should_remove {
        table.remove(&key);
    }
}

fn apply_flock_operation(key: u64, owner: usize, operation: u32) -> Result<(), LinuxError> {
    if operation & !(general::LOCK_SH | general::LOCK_EX | general::LOCK_NB | general::LOCK_UN) != 0
    {
        return Err(LinuxError::EINVAL);
    }
    let mode = operation & !general::LOCK_NB;
    match mode {
        general::LOCK_UN => {
            release_flock_owner(key, owner);
            Ok(())
        }
        general::LOCK_SH | general::LOCK_EX => {
            let mut table = flock_table().lock();
            let state = table.entry(key).or_insert_with(FlockState::new);
            let ret = if mode == general::LOCK_SH {
                state.lock_shared(owner)
            } else {
                state.lock_exclusive(owner)
            };
            if state.is_empty() {
                table.remove(&key);
            }
            ret
        }
        _ => Err(LinuxError::EINVAL),
    }
}

impl FdEntry {
    fn description_id(&self) -> Option<OpenFileId> {
        match self {
            Self::Pipe(file) | Self::EventFd(file) | Self::TimerFd(file) => Some(file.id()),
            Self::Epoll(epoll) => Some(epoll.id()),
            _ => None,
        }
    }

    fn open_file(&self) -> Option<&OpenFileRef> {
        match self {
            Self::Pipe(file) | Self::EventFd(file) | Self::TimerFd(file) => Some(file),
            _ => None,
        }
    }

    pub(super) fn duplicate_for_fork(&self) -> Result<Self, LinuxError> {
        match self {
            Self::Stdin(status_flags) => Ok(Self::Stdin(*status_flags)),
            Self::Stdout(status_flags) => Ok(Self::Stdout(*status_flags)),
            Self::Stderr(status_flags) => Ok(Self::Stderr(*status_flags)),
            Self::DevNull => Ok(Self::DevNull),
            Self::DevZero(status_flags) => Ok(Self::DevZero(*status_flags)),
            Self::DevRandom(status_flags) => Ok(Self::DevRandom(*status_flags)),
            Self::DevCpuDmaLatency(status_flags) => Ok(Self::DevCpuDmaLatency(*status_flags)),
            Self::BlockDevice(dev) => Ok(Self::BlockDevice(dev.clone())),
            Self::Rtc => Ok(Self::Rtc),
            Self::File(file) => Ok(Self::File(file.clone())),
            Self::Directory(dir) => Ok(Self::Directory(dir.clone())),
            Self::ProcFdDir(dir) => Ok(Self::ProcFdDir(dir.clone())),
            Self::SyntheticDir(dir) => Ok(Self::SyntheticDir(dir.clone())),
            Self::Path(path) => Ok(Self::Path(path.clone())),
            Self::MemoryFile(file) => Ok(Self::MemoryFile(file.clone())),
            Self::Memfd(file) => Ok(Self::Memfd(file.clone())),
            Self::ProcPagemap(file) => Ok(Self::ProcPagemap(file.clone())),
            Self::ProcTimerSlack(file) => Ok(Self::ProcTimerSlack(file.clone())),
            Self::Pipe(pipe) => Ok(Self::Pipe(pipe.clone())),
            Self::Socket(socket) => socket.duplicate().map(Self::Socket),
            Self::LocalSocket(socket) => Ok(Self::LocalSocket(socket.duplicate())),
            Self::EventFd(eventfd) => Ok(Self::EventFd(eventfd.clone())),
            Self::Inotify(inotify) => Ok(Self::Inotify(inotify.clone())),
            Self::Epoll(epoll) => Ok(Self::Epoll(epoll.clone())),
            Self::TimerFd(timerfd) => Ok(Self::TimerFd(timerfd.clone())),
            Self::SignalFd(signalfd) => Ok(Self::SignalFd(signalfd.clone())),
            Self::PidFd(pidfd) => Ok(Self::PidFd(pidfd.clone())),
            Self::PosixMq(mq) => Ok(Self::PosixMq(mq.clone())),
            Self::ProcMqQueuesMax(entry) => Ok(Self::ProcMqQueuesMax(entry.clone())),
            Self::ProcSysFile(entry) => Ok(Self::ProcSysFile(entry.clone())),
        }
    }
}

fn open_fd_entry(
    process: &UserProcess,
    table: &FdTable,
    dirfd: i32,
    path: &str,
    flags: u32,
    mode: u32,
) -> Result<FdEntry, LinuxError> {
    if path_exceeds_linux_limits(path) {
        return Err(LinuxError::ENAMETOOLONG);
    }
    if flags & !supported_open_flags() != 0 {
        return Err(LinuxError::EINVAL);
    }

    let mut opts = OpenOptions::new();
    let access = flags & general::O_ACCMODE;
    if access == general::O_WRONLY {
        opts.write(true);
    } else if access == general::O_RDWR {
        opts.read(true);
        opts.write(true);
    } else {
        opts.read(true);
    }
    if flags & general::O_APPEND != 0 {
        opts.append(true);
    }
    if flags & general::O_TRUNC != 0 {
        opts.truncate(true);
    }
    if flags & general::O_CREAT != 0 {
        opts.create(true);
        if access == general::O_RDONLY {
            opts.write(true);
        }
    }
    if flags & general::O_EXCL != 0 {
        opts.create_new(true);
    }

    let absolute = path.starts_with('/');
    let exec_root = process.exec_root();

    if absolute || dirfd == general::AT_FDCWD {
        let mut candidates = if absolute {
            if process.fs_root() == "/" {
                if let Some(path) = dev_shm_host_path(path) {
                    ensure_dev_shm_dir()?;
                    return open_candidates(process, table, &[path], &opts, flags, mode);
                }
                runtime_absolute_path_candidates(exec_root.as_str(), path)
            } else {
                vec![process.resolve_fs_absolute_path(path)?]
            }
        } else {
            let cwd = process.cwd();
            let primary = normalize_path(cwd.as_str(), path).ok_or(LinuxError::EINVAL)?;
            let mut candidates = vec![primary];
            for extra in runtime_library_name_candidates(exec_root.as_str(), path) {
                push_runtime_candidate(&mut candidates, Some(extra));
            }
            candidates
        };
        translate_mount_candidates(process, &mut candidates);
        if candidates.is_empty() {
            return Err(LinuxError::EINVAL);
        }
        open_candidates(process, table, &candidates, &opts, flags, mode)
    } else {
        let FdEntry::Directory(dir) = table.entry(dirfd)? else {
            return Err(LinuxError::ENOTDIR);
        };
        let primary = normalize_path(dir.path.as_str(), path).ok_or(LinuxError::EINVAL)?;
        let mut candidates = vec![primary];
        for extra in runtime_library_name_candidates(exec_root.as_str(), path) {
            push_runtime_candidate(&mut candidates, Some(extra));
        }
        translate_mount_candidates(process, &mut candidates);
        open_candidates(process, table, &candidates, &opts, flags, mode)
    }
}

fn translate_mount_candidates(process: &UserProcess, candidates: &mut Vec<String>) {
    for candidate in candidates.iter_mut() {
        *candidate = process.translate_mount_path(candidate.as_str());
    }
    let mut deduped = Vec::new();
    for candidate in candidates.drain(..) {
        push_runtime_candidate(&mut deduped, Some(candidate));
    }
    *candidates = deduped;
}

fn file_is_readable(status_flags: u32) -> bool {
    (status_flags & general::O_ACCMODE) != general::O_WRONLY
}

fn file_is_writable(status_flags: u32) -> bool {
    matches!(
        status_flags & general::O_ACCMODE,
        general::O_WRONLY | general::O_RDWR
    )
}

fn exec_write_open_table() -> &'static Mutex<BTreeMap<String, usize>> {
    static EXEC_WRITE_OPEN_COUNTS: LazyInit<Mutex<BTreeMap<String, usize>>> = LazyInit::new();
    let _ = EXEC_WRITE_OPEN_COUNTS.call_once(|| Mutex::new(BTreeMap::new()));
    &EXEC_WRITE_OPEN_COUNTS
}

fn exec_running_table() -> &'static Mutex<BTreeMap<String, usize>> {
    static EXEC_RUNNING_COUNTS: LazyInit<Mutex<BTreeMap<String, usize>>> = LazyInit::new();
    let _ = EXEC_RUNNING_COUNTS.call_once(|| Mutex::new(BTreeMap::new()));
    &EXEC_RUNNING_COUNTS
}

fn track_exec_write_open(entry: &FdEntry) {
    let FdEntry::File(file) = entry else {
        return;
    };
    if !file_is_writable(file.status_flags) {
        return;
    }
    let mut counts = exec_write_open_table().lock();
    *counts.entry(file.path.clone()).or_insert(0) += 1;
}

fn untrack_exec_write_open(entry: &FdEntry) {
    let FdEntry::File(file) = entry else {
        return;
    };
    if !file_is_writable(file.status_flags) {
        return;
    }
    let mut counts = exec_write_open_table().lock();
    if let Some(count) = counts.get_mut(file.path.as_str()) {
        if *count <= 1 {
            counts.remove(file.path.as_str());
        } else {
            *count -= 1;
        }
    }
}

pub(super) fn executable_write_open(path: &str) -> bool {
    exec_write_open_table()
        .lock()
        .get(path)
        .copied()
        .unwrap_or(0)
        > 0
}

pub(super) fn track_running_executable(path: &str) {
    let mut counts = exec_running_table().lock();
    let count = counts.entry(path.into()).or_insert(0);
    *count += 1;
}

pub(super) fn untrack_running_executable(path: &str) {
    let mut counts = exec_running_table().lock();
    if let Some(count) = counts.get_mut(path) {
        if *count <= 1 {
            counts.remove(path);
        } else {
            *count -= 1;
        }
    }
}

fn executable_running(path: &str) -> bool {
    exec_running_table().lock().get(path).copied().unwrap_or(0) > 0
}

fn file_logical_size(process: &UserProcess, file: &FileEntry) -> Result<u64, LinuxError> {
    let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
    Ok(file_logical_size_from_physical(
        process,
        file,
        physical_size,
    ))
}

fn file_logical_size_from_physical(
    process: &UserProcess,
    file: &FileEntry,
    physical_size: u64,
) -> u64 {
    process
        .path_sparse_size(file.path.as_str())
        .unwrap_or(physical_size)
        .max(physical_size)
}

fn read_physical_file_prefix(
    file: &FileEntry,
    offset: u64,
    dst: &mut [u8],
) -> Result<usize, LinuxError> {
    let mut read = 0usize;
    while read < dst.len() {
        let chunk = file
            .file
            .read_at(offset.saturating_add(read as u64), &mut dst[read..])
            .map_err(LinuxError::from)?;
        if chunk == 0 {
            break;
        }
        read += chunk;
    }
    Ok(read)
}

fn read_regular_file_to_vec(
    process: &UserProcess,
    file: &FileEntry,
    size: u64,
) -> Result<Vec<u8>, LinuxError> {
    if size > MAX_IN_MEMORY_FILE_SIZE {
        return Err(LinuxError::EFBIG);
    }
    let size = usize::try_from(size).map_err(|_| LinuxError::EFBIG)?;
    let mut data = Vec::new();
    data.try_reserve_exact(size)
        .map_err(|_| LinuxError::ENOMEM)?;
    data.resize(size, 0);
    let read = read_regular_file_at(process, file, 0, &mut data)?;
    data[read..].fill(0);
    Ok(data)
}

fn read_memfd_to_vec(file: &MemfdEntry, size: u64) -> Result<Vec<u8>, LinuxError> {
    if size > MAX_IN_MEMORY_FILE_SIZE {
        return Err(LinuxError::EFBIG);
    }
    let size = usize::try_from(size).map_err(|_| LinuxError::EFBIG)?;
    let mut data = Vec::new();
    data.try_reserve_exact(size)
        .map_err(|_| LinuxError::ENOMEM)?;
    data.resize(size, 0);
    file.read_at(0, &mut data)?;
    Ok(data)
}

fn insert_zero_range(data: &mut Vec<u8>, offset: usize, len: usize) -> Result<(), LinuxError> {
    if len == 0 {
        return Ok(());
    }
    if offset > data.len() {
        return Err(LinuxError::EINVAL);
    }
    let old_len = data.len();
    let new_len = old_len.checked_add(len).ok_or(LinuxError::EFBIG)?;
    data.try_reserve_exact(len)
        .map_err(|_| LinuxError::ENOMEM)?;
    data.resize(new_len, 0);
    data.copy_within(offset..old_len, offset + len);
    data[offset..offset + len].fill(0);
    Ok(())
}

fn rewrite_regular_file_from_vec(
    process: &UserProcess,
    file: &mut FileEntry,
    data: &[u8],
) -> Result<(), LinuxError> {
    if data.len() as u64 > MAX_IN_MEMORY_FILE_SIZE {
        return Err(LinuxError::EFBIG);
    }
    file.file.truncate(0).map_err(LinuxError::from)?;
    process.truncate_path_sparse_file(file.path.clone(), 0);
    if !data.is_empty() {
        let _ = write_regular_file_at(process, file, 0, data, None)?;
    } else {
        process.set_path_sparse_size(file.path.clone(), 0);
        touch_regular_file_after_write(process, file);
    }
    Ok(())
}

fn write_sparse_zero_range(
    process: &UserProcess,
    path: String,
    offset: u64,
    len: u64,
) -> Result<(), LinuxError> {
    if len == 0 {
        return Ok(());
    }
    const ZERO_CHUNK: usize = 64 * 1024;
    let chunk = ZERO_CHUNK.min(len.min(usize::MAX as u64) as usize);
    let mut zeros = Vec::new();
    zeros
        .try_reserve_exact(chunk)
        .map_err(|_| LinuxError::ENOMEM)?;
    zeros.resize(chunk, 0);

    let mut written = 0u64;
    while written < len {
        let chunk_len = (len - written).min(zeros.len() as u64) as usize;
        let write_offset = offset.checked_add(written).ok_or(LinuxError::EFBIG)?;
        process.write_path_sparse_data(path.clone(), write_offset, &zeros[..chunk_len])?;
        written = written.saturating_add(chunk_len as u64);
    }
    Ok(())
}

fn write_sparse_data_chunked(
    process: &UserProcess,
    path: String,
    offset: u64,
    data: &[u8],
) -> Result<(), LinuxError> {
    let mut written = 0usize;
    while written < data.len() {
        let chunk_len = (data.len() - written).min(MAX_USER_IO_CHUNK);
        let chunk_offset = offset
            .checked_add(written as u64)
            .ok_or(LinuxError::EFBIG)?;
        process.write_path_sparse_data(
            path.clone(),
            chunk_offset,
            &data[written..written + chunk_len],
        )?;
        written += chunk_len;
    }
    Ok(())
}

fn write_memfd_zero_range(file: &mut MemfdEntry, offset: u64, len: u64) -> Result<(), LinuxError> {
    const ZERO_CHUNK: usize = 64 * 1024;
    let chunk = ZERO_CHUNK.min(len.min(usize::MAX as u64) as usize);
    let mut zeros = Vec::new();
    zeros
        .try_reserve_exact(chunk)
        .map_err(|_| LinuxError::ENOMEM)?;
    zeros.resize(chunk, 0);

    let mut written = 0u64;
    while written < len {
        let chunk_len = (len - written).min(zeros.len() as u64) as usize;
        let write_offset = offset.checked_add(written).ok_or(LinuxError::EFBIG)?;
        file.write_at(write_offset, &zeros[..chunk_len], None)?;
        written = written.saturating_add(chunk_len as u64);
    }
    Ok(())
}

fn stat_time(sec: i64, nsec: u64) -> general::timespec {
    const NSEC_PER_SEC: u64 = 1_000_000_000;
    general::timespec {
        tv_sec: sec as _,
        tv_nsec: nsec.min(NSEC_PER_SEC - 1) as _,
    }
}

fn write_timestamp_after(current: general::timespec, now: general::timespec) -> general::timespec {
    let diff_ns = (now.tv_sec as i128 - current.tv_sec as i128) * 1_000_000_000
        + (now.tv_nsec as i128 - current.tv_nsec as i128);
    if diff_ns > 0 && diff_ns <= 30_000_000_000 {
        return now;
    }
    add_timespec_ns(current, 2_000_000_000)
}

fn add_timespec_ns(ts: general::timespec, ns: i128) -> general::timespec {
    const NSEC_PER_SEC: i128 = 1_000_000_000;
    let total = ts.tv_sec as i128 * NSEC_PER_SEC + ts.tv_nsec as i128 + ns;
    if total <= 0 {
        general::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        }
    } else {
        general::timespec {
            tv_sec: (total / NSEC_PER_SEC).min(i64::MAX as i128) as _,
            tv_nsec: (total % NSEC_PER_SEC) as _,
        }
    }
}

fn touch_regular_file_after_write(process: &UserProcess, file: &FileEntry) {
    invalidate_exec_image_cache(file.path.as_str());
    let Ok(now) = clock_gettime_timespec(general::CLOCK_REALTIME) else {
        return;
    };
    let current = process.path_times(file.path.as_str()).unwrap_or_else(|| {
        let st = match file.file.get_attr() {
            Ok(attr) => file_attr_to_stat(&attr, Some(file.path.as_str())),
            Err(_) => {
                return PathTimes {
                    atime: now,
                    mtime: now,
                    ctime: now,
                };
            }
        };
        let st = apply_recorded_path_metadata(process, file.path.as_str(), st);
        PathTimes {
            atime: stat_time(st.st_atime as i64, st.st_atime_nsec),
            mtime: stat_time(st.st_mtime as i64, st.st_mtime_nsec),
            ctime: stat_time(st.st_ctime as i64, st.st_ctime_nsec),
        }
    });
    process.set_path_times(file.path.clone(), {
        let write_time = write_timestamp_after(current.mtime, now);
        PathTimes {
            atime: current.atime,
            mtime: write_time,
            ctime: write_time,
        }
    });
}

fn record_created_path_times(process: &UserProcess, path: String) {
    let Ok(now) = clock_gettime_timespec(general::CLOCK_REALTIME) else {
        return;
    };
    process.set_path_times(
        path,
        PathTimes {
            atime: now,
            mtime: now,
            ctime: now,
        },
    );
}

fn read_regular_file_at(
    process: &UserProcess,
    file: &FileEntry,
    offset: u64,
    dst: &mut [u8],
) -> Result<usize, LinuxError> {
    if dst.is_empty() {
        return Ok(0);
    }
    let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
    let sparse_size = process.path_sparse_size(file.path.as_str());
    let data_ranges = process.path_data_ranges(file.path.as_str());
    let logical_size = sparse_size.unwrap_or(physical_size).max(physical_size);
    if offset >= logical_size {
        return Ok(0);
    }
    let read_len = cmp::min(
        dst.len(),
        logical_size.saturating_sub(offset).min(usize::MAX as u64) as usize,
    );
    if sparse_size.is_none() && data_ranges.is_none() {
        if offset >= physical_size {
            return Ok(0);
        }
        let physical_len = cmp::min(
            read_len,
            physical_size.saturating_sub(offset).min(usize::MAX as u64) as usize,
        );
        return read_physical_file_prefix(file, offset, &mut dst[..physical_len]);
    }
    dst[..read_len].fill(0);
    if let Some(ranges) = data_ranges {
        let read_end = offset.saturating_add(read_len as u64);
        for (range_start, range_end) in ranges {
            let data_start = range_start.max(offset);
            let data_end = range_end.min(read_end);
            if data_start >= data_end || data_start >= physical_size {
                continue;
            }
            let physical_end = data_end.min(physical_size);
            let dst_start = data_start.saturating_sub(offset) as usize;
            let physical_len = physical_end.saturating_sub(data_start) as usize;
            let _ = read_physical_file_prefix(
                file,
                data_start,
                &mut dst[dst_start..dst_start + physical_len],
            )?;
        }
        process.copy_path_sparse_data(file.path.as_str(), offset, &mut dst[..read_len]);
        return Ok(read_len);
    }
    if offset < physical_size {
        let physical_len = cmp::min(
            read_len,
            physical_size.saturating_sub(offset).min(usize::MAX as u64) as usize,
        );
        let physical_read = read_physical_file_prefix(file, offset, &mut dst[..physical_len])?;
        if physical_read < physical_len {
            process.copy_path_sparse_data(file.path.as_str(), offset, &mut dst[..physical_read]);
            return Ok(physical_read);
        }
    }
    process.copy_path_sparse_data(file.path.as_str(), offset, &mut dst[..read_len]);
    Ok(read_len)
}

fn limit_regular_file_write_len<'a>(
    src: &'a [u8],
    file_size_limit: Option<u64>,
    write_offset: u64,
) -> Result<&'a [u8], LinuxError> {
    if src.is_empty() {
        return Ok(src);
    }
    let Some(limit) = file_size_limit else {
        return Ok(src);
    };
    if limit == u64::MAX {
        return Ok(src);
    }
    if write_offset >= limit {
        return Err(LinuxError::EFBIG);
    }
    let allowed = limit.saturating_sub(write_offset) as usize;
    Ok(&src[..src.len().min(allowed)])
}

fn file_data_block_floor(offset: u64) -> u64 {
    offset / 512 * 512
}

fn file_data_block_ceil(offset: u64) -> u64 {
    offset.saturating_add(511) / 512 * 512
}

fn missing_path_data_512_blocks(process: &UserProcess, path: &str, offset: u64, len: u64) -> u64 {
    if len == 0 {
        return 0;
    }
    let start = file_data_block_floor(offset);
    let end = file_data_block_ceil(offset.saturating_add(len));
    if start >= end {
        return 0;
    }
    let Some(missing) = process.missing_path_data_512_blocks(path, start, end) else {
        return end.saturating_sub(start) / 512;
    };
    missing
}

fn can_use_reclaimed_path_blocks(process: &UserProcess, path: &str, offset: u64, len: u64) -> bool {
    let blocks = missing_path_data_512_blocks(process, path, offset, len);
    blocks > 0 && process.path_free_512_blocks(path) >= blocks
}

fn ensure_tmpfs_allocated_range(
    process: &UserProcess,
    path: &str,
    offset: u64,
    len: u64,
) -> Result<(), LinuxError> {
    let Some(free_blocks) = process.tmpfs_free_512_blocks_for_path(path) else {
        return Ok(());
    };
    let missing_blocks = missing_path_data_512_blocks(process, path, offset, len);
    if missing_blocks > free_blocks {
        Err(LinuxError::ENOSPC)
    } else {
        Ok(())
    }
}

fn limit_tmpfs_write_len<'a>(
    process: &UserProcess,
    path: &str,
    src: &'a [u8],
    write_offset: u64,
) -> Result<&'a [u8], LinuxError> {
    let Some(free_blocks) = process.tmpfs_free_512_blocks_for_path(path) else {
        return Ok(src);
    };
    if missing_path_data_512_blocks(process, path, write_offset, src.len() as u64) <= free_blocks {
        return Ok(src);
    }

    let mut low = 0usize;
    let mut high = src.len();
    while low < high {
        let mid = low + (high - low).div_ceil(2);
        if missing_path_data_512_blocks(process, path, write_offset, mid as u64) <= free_blocks {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    if low == 0 {
        Err(LinuxError::ENOSPC)
    } else {
        Ok(&src[..low])
    }
}

fn bytes_are_all_zero(src: &[u8]) -> bool {
    src.iter().all(|byte| *byte == 0)
}

fn resize_regular_file_physical_prefix(
    file: &mut FileEntry,
    physical_size: u64,
    logical_size: u64,
) -> Result<(), LinuxError> {
    let target_physical_size = if logical_size <= MAX_PHYSICAL_FILE_BACKING_SIZE {
        logical_size
    } else {
        physical_size.min(MAX_PHYSICAL_FILE_BACKING_SIZE)
    };
    if target_physical_size != physical_size {
        file.file
            .truncate(target_physical_size)
            .map_err(LinuxError::from)?;
    }
    Ok(())
}

fn write_regular_file_at(
    process: &UserProcess,
    file: &mut FileEntry,
    write_offset: u64,
    src: &[u8],
    file_size_limit: Option<u64>,
) -> Result<usize, LinuxError> {
    let mut src = limit_regular_file_write_len(src, file_size_limit, write_offset)?;
    if src.is_empty() {
        return Ok(0);
    }
    let logical_before = file_logical_size(process, file)?;
    src = limit_tmpfs_write_len(process, file.path.as_str(), src, write_offset)?;
    let physical_before = file.file.get_attr().map_err(LinuxError::from)?.size();
    process.ensure_path_data_ranges(file.path.clone(), physical_before);
    let mut written = 0usize;
    let physical_write_limit = MAX_IN_MEMORY_FILE_SIZE.min(MAX_PHYSICAL_FILE_BACKING_SIZE);
    if write_offset < physical_write_limit {
        let physical_len = cmp::min(
            src.len(),
            physical_write_limit
                .saturating_sub(write_offset)
                .min(usize::MAX as u64) as usize,
        );
        if physical_len > 0 {
            let count = match file.file.write_at(write_offset, &src[..physical_len]) {
                Ok(count) => count,
                Err(err) => {
                    let err = LinuxError::from(err);
                    if err == LinuxError::ENOSPC {
                        if bytes_are_all_zero(&src[..physical_len]) {
                            write_sparse_zero_range(
                                process,
                                file.path.clone(),
                                write_offset,
                                physical_len as u64,
                            )?;
                        } else {
                            write_sparse_data_chunked(
                                process,
                                file.path.clone(),
                                write_offset,
                                &src[..physical_len],
                            )?;
                        }
                        physical_len
                    } else {
                        return Err(err);
                    }
                }
            };
            written += count;
            process.mark_path_data_range(file.path.clone(), write_offset, count as u64);
            if count < physical_len {
                let sparse_offset = write_offset.saturating_add(written as u64);
                let sparse_len = physical_len - count;
                let sparse_end = sparse_offset.saturating_add(sparse_len as u64);
                let preallocated_sparse_extent = process
                    .path_sparse_size(file.path.as_str())
                    .is_some_and(|size| sparse_end <= size);
                let sparse_backed = process.path_data_ranges_cover(
                    file.path.as_str(),
                    sparse_offset,
                    sparse_len as u64,
                ) || preallocated_sparse_extent;
                let reclaimed_sparse_space = if sparse_backed {
                    false
                } else {
                    can_use_reclaimed_path_blocks(
                        process,
                        file.path.as_str(),
                        sparse_offset,
                        sparse_len as u64,
                    )
                };
                if sparse_backed || reclaimed_sparse_space {
                    let tail = &src[written..physical_len];
                    if bytes_are_all_zero(tail) {
                        write_sparse_zero_range(
                            process,
                            file.path.clone(),
                            sparse_offset,
                            sparse_len as u64,
                        )?;
                    } else {
                        write_sparse_data_chunked(process, file.path.clone(), sparse_offset, tail)?;
                    }
                    written = physical_len;
                } else {
                    let logical_after =
                        logical_before.max(write_offset.saturating_add(written as u64));
                    process.set_path_sparse_size(file.path.clone(), logical_after);
                    touch_regular_file_after_write(process, file);
                    return Ok(written);
                }
            }
        }
    }

    if written < src.len() {
        let sparse_offset = write_offset.saturating_add(written as u64);
        let tail = &src[written..];
        if bytes_are_all_zero(tail) {
            write_sparse_zero_range(process, file.path.clone(), sparse_offset, tail.len() as u64)?;
        } else {
            write_sparse_data_chunked(process, file.path.clone(), sparse_offset, tail)?;
        }
        written = src.len();
    }
    let logical_after = logical_before.max(write_offset.saturating_add(written as u64));
    process.set_path_sparse_size(file.path.clone(), logical_after);
    touch_regular_file_after_write(process, file);
    Ok(written)
}

pub(super) fn write_mmap_file_backing(
    process: &UserProcess,
    file: &mut MmapFileBacking,
    write_offset: u64,
    src: &[u8],
) -> Result<usize, LinuxError> {
    match file {
        MmapFileBacking::File(file) => {
            if !file_is_writable(file.status_flags) {
                return Err(LinuxError::EBADF);
            }
            // MAP_SHARED writeback is tied to the mapping offset, not to the current
            // descriptor position. In particular, do not redirect msync writes to EOF
            // for O_APPEND descriptors.
            write_regular_file_at(process, file, write_offset, src, None)
        }
        MmapFileBacking::Memfd(file) => file.write_at(write_offset, src, None),
    }
}

pub(super) fn read_mmap_file_backing(
    process: &UserProcess,
    file: &mut MmapFileBacking,
    read_offset: u64,
    dst: &mut [u8],
) -> Result<usize, LinuxError> {
    match file {
        MmapFileBacking::File(file) => {
            if !file_is_readable(file.status_flags) {
                return Err(LinuxError::EBADF);
            }
            read_regular_file_at(process, file, read_offset, dst)
        }
        MmapFileBacking::Memfd(file) => file.read_at(read_offset, dst),
    }
}

fn file_entry_read(
    process: &UserProcess,
    file: &mut FileEntry,
    dst: &mut [u8],
) -> Result<usize, LinuxError> {
    let current = *file.offset.lock();
    let read = read_regular_file_at(process, file, current, dst)?;
    *file.offset.lock() = current.saturating_add(read as u64);
    Ok(read)
}

fn file_entry_write(
    process: &UserProcess,
    file: &mut FileEntry,
    src: &[u8],
    file_size_limit: Option<u64>,
) -> Result<usize, LinuxError> {
    let write_offset = if file.status_flags & general::O_APPEND != 0 {
        file_logical_size(process, file)?
    } else {
        *file.offset.lock()
    };
    let written = write_regular_file_at(process, file, write_offset, src, file_size_limit)?;
    *file.offset.lock() = write_offset.saturating_add(written as u64);
    Ok(written)
}

fn file_entry_seek(
    process: &UserProcess,
    file: &mut FileEntry,
    pos: SeekFrom,
) -> Result<u64, LinuxError> {
    let size = file_logical_size(process, file)?;
    let mut offset = file.offset.lock();
    let next = match pos {
        SeekFrom::Start(pos) => Some(pos),
        SeekFrom::Current(off) => (*offset).checked_add_signed(off),
        SeekFrom::End(off) => size.checked_add_signed(off),
    }
    .ok_or(LinuxError::EINVAL)?;
    *offset = next;
    Ok(next)
}

fn regular_file_data_ranges(
    process: &UserProcess,
    file: &FileEntry,
    size: u64,
) -> Result<Vec<(u64, u64)>, LinuxError> {
    let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
    let mut ranges = process
        .path_data_ranges(file.path.as_str())
        .unwrap_or_else(|| {
            if physical_size == 0 {
                Vec::new()
            } else {
                vec![(0, physical_size.min(size))]
            }
        });
    ranges.sort_by_key(|(start, _)| *start);

    let mut normalized = Vec::new();
    for (start, end) in ranges {
        let start = start.min(size);
        let end = end.min(size);
        if start >= end {
            continue;
        }
        if let Some((_, prev_end)) = normalized.last_mut() {
            if start <= *prev_end {
                *prev_end = (*prev_end).max(end);
                continue;
            }
        }
        normalized.push((start, end));
    }
    Ok(normalized)
}

fn file_entry_seek_data_or_hole(
    process: &UserProcess,
    file: &mut FileEntry,
    start: u64,
    want_data: bool,
) -> Result<u64, LinuxError> {
    let size = file_logical_size(process, file)?;
    if start >= size {
        return Err(LinuxError::ENXIO);
    }
    let ranges = regular_file_data_ranges(process, file, size)?;
    let next = if want_data {
        ranges
            .iter()
            .find_map(|(range_start, range_end)| {
                if *range_end <= start {
                    None
                } else if *range_start <= start {
                    Some(start)
                } else {
                    Some(*range_start)
                }
            })
            .ok_or(LinuxError::ENXIO)?
    } else {
        let mut cursor = start;
        for (range_start, range_end) in ranges {
            if range_end <= cursor {
                continue;
            }
            if cursor < range_start {
                break;
            }
            cursor = range_end;
            if cursor >= size {
                break;
            }
        }
        cursor.min(size)
    };
    *file.offset.lock() = next;
    Ok(next)
}

fn path_exceeds_linux_limits(path: &str) -> bool {
    path.len() >= LINUX_PATH_MAX
        || path
            .split('/')
            .any(|component| component.len() > LINUX_NAME_MAX)
}

fn parent_path(path: &str) -> &str {
    if path == "/" {
        return "/";
    }
    match path.rsplit_once('/') {
        Some(("", _)) => "/",
        Some((parent, _)) if !parent.is_empty() => parent,
        _ => "/",
    }
}

fn last_path_component(path: &str) -> Option<&str> {
    path.rsplit('/').find(|component| !component.is_empty())
}

fn stat_absolute_path(process: &UserProcess, path: &str) -> Result<general::stat, LinuxError> {
    let stat_path = process
        .path_hardlink_backing(path)
        .unwrap_or_else(|| path.to_string());
    let attr = axfs::api::metadata(stat_path.as_str()).map_err(LinuxError::from)?;
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_dev = 1;
    st.st_ino = path_inode(Some(stat_path.as_str()));
    st.st_mode = file_type_mode(attr.file_type()) | attr.permissions().bits() as u32;
    st.st_nlink = 1;
    st.st_size = attr.size() as _;
    st.st_blksize = 512;
    st.st_blocks = attr.blocks() as _;
    Ok(apply_recorded_path_metadata(
        process,
        stat_path.as_str(),
        st,
    ))
}

fn parent_dirs_searchable_absolute(
    process: &UserProcess,
    path: &str,
    uid: u32,
    gid: u32,
) -> Result<bool, LinuxError> {
    if uid == 0 {
        return Ok(true);
    }
    let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
    if components.len() <= 1 {
        return Ok(true);
    }
    let mut parent = String::new();
    for component in &components[..components.len() - 1] {
        parent.push('/');
        parent.push_str(component);
        let st = stat_absolute_path(process, parent.as_str())?;
        if !access_allowed(&st, ACCESS_X_OK, uid, gid) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) fn check_parent_write_search_permission(
    process: &UserProcess,
    path: &str,
) -> Result<general::stat, LinuxError> {
    let uid = process.fs_uid();
    let gid = process.fs_gid();
    let parent = parent_path(path);
    let parent_st = stat_absolute_path(process, parent)?;
    if parent_st.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
        return Err(LinuxError::ENOTDIR);
    }
    if process.path_on_readonly_mount(parent) {
        return Err(LinuxError::EROFS);
    }
    if uid == 0 {
        return Ok(parent_st);
    }
    if !parent_dirs_searchable_absolute(process, parent, uid, gid)?
        || !access_allowed(&parent_st, ACCESS_W_OK | ACCESS_X_OK, uid, gid)
    {
        return Err(LinuxError::EACCES);
    }
    Ok(parent_st)
}

fn record_created_path_metadata(
    process: &UserProcess,
    path: String,
    requested_mode: u32,
    is_directory: bool,
    parent_st: &general::stat,
) {
    let parent_setgid = parent_st.st_mode & FILE_MODE_SET_GID != 0;
    let mut mode = process.apply_umask(requested_mode);
    if is_directory && parent_setgid {
        mode |= FILE_MODE_SET_GID;
    } else if !is_directory
        && parent_setgid
        && process.fs_uid() != 0
        && !process.has_group(parent_st.st_gid as u32)
    {
        // Linux clears S_ISGID for newly-created non-directories in an SGID
        // directory when the creator is unprivileged and not a member of the
        // inherited group. Root/CAP_FSETID-style creators keep the requested
        // setgid bit; unprivileged creators outside the inherited group lose it.
        mode &= !FILE_MODE_SET_GID;
    }
    let gid = if parent_setgid {
        parent_st.st_gid as u32
    } else {
        process.fs_gid()
    };
    process.set_path_mode(path.clone(), mode);
    process.set_path_owner(path, Some(process.fs_uid()), Some(gid));
}

fn check_sticky_parent_permission(
    process: &UserProcess,
    parent_st: &general::stat,
    target_st: &general::stat,
) -> Result<(), LinuxError> {
    let uid = process.fs_uid();
    if uid == 0 || parent_st.st_mode & FILE_MODE_STICKY == 0 {
        return Ok(());
    }
    if uid == parent_st.st_uid as u32 || uid == target_st.st_uid as u32 {
        Ok(())
    } else {
        Err(LinuxError::EPERM)
    }
}

fn check_inode_flags_allow_unlink(process: &UserProcess, path: &str) -> Result<(), LinuxError> {
    let flags = process.path_inode_flags(path);
    if flags & (general::FS_IMMUTABLE_FL | general::FS_APPEND_FL) != 0 {
        Err(LinuxError::EPERM)
    } else {
        Ok(())
    }
}

fn open_permission_mode(flags: u32) -> usize {
    match flags & general::O_ACCMODE {
        general::O_WRONLY => ACCESS_W_OK,
        general::O_RDWR => ACCESS_R_OK | ACCESS_W_OK,
        _ => ACCESS_R_OK,
    }
}

fn check_open_permission(process: &UserProcess, path: &str, flags: u32) -> Result<(), LinuxError> {
    if flags & O_PATH_FLAG != 0 {
        return Ok(());
    }
    let attr = match axfs::api::metadata(path) {
        Ok(attr) => attr,
        Err(_) => return Ok(()),
    };
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_dev = 1;
    st.st_ino = path_inode(Some(path));
    st.st_mode = file_type_mode(attr.file_type()) | attr.permissions().bits() as u32;
    st.st_nlink = 1;
    st.st_size = attr.size() as _;
    st.st_blksize = 512;
    st.st_blocks = attr.blocks() as _;
    let st = apply_recorded_path_metadata(process, path, st);
    let uid = process.fs_uid();
    let gid = process.fs_gid();
    if !parent_dirs_searchable_absolute(process, path, uid, gid)? {
        return Err(LinuxError::EACCES);
    }
    let mode = open_permission_mode(flags);
    if mode & ACCESS_W_OK != 0 && process.path_on_readonly_mount(path) {
        return Err(LinuxError::EROFS);
    }
    if access_allowed(&st, mode, uid, gid) {
        Ok(())
    } else {
        Err(LinuxError::EACCES)
    }
}

fn check_synthetic_open_permission(
    process: &UserProcess,
    path: &str,
    st: &general::stat,
    flags: u32,
) -> Result<(), LinuxError> {
    if flags & O_PATH_FLAG != 0 {
        return Ok(());
    }
    let uid = process.fs_uid();
    let gid = process.fs_gid();
    if !parent_dirs_searchable_absolute(process, path, uid, gid)? {
        return Err(LinuxError::EACCES);
    }
    let mode = open_permission_mode(flags);
    if mode & ACCESS_W_OK != 0 && process.path_on_readonly_mount(path) {
        return Err(LinuxError::EROFS);
    }
    if access_allowed(st, mode, uid, gid) {
        Ok(())
    } else {
        Err(LinuxError::EACCES)
    }
}

fn fcntl_status_flags(open_flags: u32) -> u32 {
    open_flags
        & (general::O_ACCMODE
            | general::O_APPEND
            | general::O_NONBLOCK
            | general::O_DSYNC
            | general::O_SYNC
            | general::O_DIRECT
            | general::O_NOATIME)
}

fn fcntl_setfl_flags(flags: u32) -> u32 {
    flags & (general::O_APPEND | general::O_NONBLOCK | general::O_DIRECT | general::O_NOATIME)
}

fn validate_cpu_dma_latency_request(src: &[u8]) -> Result<i32, LinuxError> {
    if src.len() == size_of::<i32>() {
        return Ok(i32::from_ne_bytes([src[0], src[1], src[2], src[3]]));
    }

    let text = core::str::from_utf8(src).map_err(|_| LinuxError::EINVAL)?;
    let text = text.trim_matches(|ch: char| ch == '\0' || ch.is_ascii_whitespace());
    if text.is_empty() {
        return Err(LinuxError::EINVAL);
    }
    let (negative, digits) = if let Some(rest) = text.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = text.strip_prefix('+') {
        (false, rest)
    } else {
        (false, text)
    };
    let digits = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
        .unwrap_or(digits);
    if digits.is_empty() {
        return Err(LinuxError::EINVAL);
    }
    let value = i64::from_str_radix(digits, 16).map_err(|_| LinuxError::EINVAL)?;
    let value = if negative { -value } else { value };
    if value < i32::MIN as i64 || value > i32::MAX as i64 {
        return Err(LinuxError::EINVAL);
    }
    Ok(value as i32)
}

fn supported_open_flags() -> u32 {
    general::O_ACCMODE
        | general::O_APPEND
        | general::O_NONBLOCK
        | general::O_DSYNC
        | general::O_SYNC
        | general::O_DIRECT
        | general::O_NOATIME
        | general::O_CREAT
        | general::O_EXCL
        | general::O_TRUNC
        | general::O_CLOEXEC
        | general::O_DIRECTORY
        | general::O_TMPFILE
        | general::O_LARGEFILE
        | general::O_NOCTTY
        | O_PATH_FLAG
        | O_NOFOLLOW_FLAG
}

fn tmpfile_requested(flags: u32) -> bool {
    flags & general::O_TMPFILE == general::O_TMPFILE
}

fn synthetic_readonly_open_writes(flags: u32) -> bool {
    matches!(
        flags & general::O_ACCMODE,
        general::O_WRONLY | general::O_RDWR
    ) || flags & general::O_TRUNC != 0
}

fn open_would_write_or_truncate(flags: u32) -> bool {
    flags & O_PATH_FLAG == 0
        && (matches!(
            flags & general::O_ACCMODE,
            general::O_WRONLY | general::O_RDWR
        ) || flags & general::O_TRUNC != 0)
}

fn open_candidates(
    process: &UserProcess,
    table: &FdTable,
    candidates: &[String],
    opts: &OpenOptions,
    flags: u32,
    mode: u32,
) -> Result<FdEntry, LinuxError> {
    let prefer_dir = flags & general::O_DIRECTORY != 0;
    let wants_tmpfile = tmpfile_requested(flags);
    let path_only = flags & O_PATH_FLAG != 0;
    let mut path_opts = OpenOptions::new();
    if path_only {
        path_opts.read(true);
    }
    let file_opts = if path_only { &path_opts } else { opts };
    let mut last_err = LinuxError::ENOENT;
    for path in candidates {
        if !path_only
            && process.path_on_nosymfollow_mount(path.as_str())
            && process.path_contains_followed_symlink(path.as_str(), true)?
        {
            return Err(LinuxError::ELOOP);
        }
        if flags & O_NOFOLLOW_FLAG != 0 {
            let resolved_path = process.resolve_parent_symlinks(path.as_str())?;
            if resolved_path != *path {
                return open_candidates(process, table, &[resolved_path], opts, flags, mode);
            }
            if process.path_symlink(path.as_str()).is_some() {
                if prefer_dir {
                    return Err(LinuxError::ENOTDIR);
                }
                if path_only {
                    return Ok(FdEntry::Path(PathEntry::symlink(path.as_str())));
                }
                return Err(LinuxError::ELOOP);
            }
        } else if let Some(resolved_path) = process.resolve_path_symlink(path.as_str())? {
            return open_candidates(process, table, &[resolved_path], opts, flags, mode);
        }
        if wants_tmpfile {
            if flags & general::O_ACCMODE == general::O_RDONLY {
                return Err(LinuxError::EINVAL);
            }
            match open_dir_entry(path.as_str()) {
                Ok(_) => return Err(LinuxError::EOPNOTSUPP),
                Err(err) => record_missing_candidate(&mut last_err, err)?,
            }
            continue;
        }
        if let Some(fd) = proc_fd_target_number(process, path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return reopen_proc_fd_entry(table, fd, flags, path_only);
        }
        if let Some(proc_fd_path) = proc_fd_dir_path(process, path.as_str()) {
            if !path_only
                && (matches!(
                    flags & general::O_ACCMODE,
                    general::O_WRONLY | general::O_RDWR
                ) || flags & (general::O_CREAT | general::O_TRUNC) != 0)
            {
                return Err(LinuxError::EISDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_dir(proc_fd_path.as_str()))
            } else {
                FdEntry::ProcFdDir(ProcFdDirEntry {
                    path: proc_fd_path,
                    next_dirent_cookie: 0,
                })
            });
        }
        if let Some(entry) = proc_pid_dir_entry(process, path.as_str(), path_only) {
            if !path_only
                && (matches!(
                    flags & general::O_ACCMODE,
                    general::O_WRONLY | general::O_RDWR
                ) || flags & (general::O_CREAT | general::O_TRUNC) != 0)
            {
                return Err(LinuxError::EISDIR);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_task_dir_path_entry(process, path.as_str())
        } else {
            proc_task_dir_fd_entry(process, path.as_str())
        } {
            if !path_only
                && (matches!(
                    flags & general::O_ACCMODE,
                    general::O_WRONLY | general::O_RDWR
                ) || flags & (general::O_CREAT | general::O_TRUNC) != 0)
            {
                return Err(LinuxError::EISDIR);
            }
            return Ok(entry);
        }
        if let Some(target) = proc_exe_link_target(process, path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & O_NOFOLLOW_FLAG != 0 {
                if path_only {
                    return Ok(FdEntry::Path(PathEntry::symlink(path.as_str())));
                }
                return Err(LinuxError::ELOOP);
            }
            if synthetic_readonly_open_writes(flags) {
                return Err(LinuxError::EPERM);
            }
            let target_flags = flags & !general::O_CREAT;
            return open_fd_entry(
                process,
                table,
                general::AT_FDCWD,
                target.as_str(),
                target_flags,
                mode,
            );
        }
        if let Some((synthetic_path, data)) = synthetic_proc_version_content(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            if synthetic_readonly_open_writes(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(if path_only {
                synthetic_proc_version_path_entry(synthetic_path, data)
            } else {
                synthetic_proc_version_fd_entry(synthetic_path, data)
            });
        }
        if let Some(entry) = if path_only {
            proc_meminfo_path_entry(path.as_str())
        } else {
            proc_meminfo_fd_entry(path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if is_proc_self_maps_path(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && proc_self_maps_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(if path_only {
                proc_self_maps_path_entry(process)
            } else {
                proc_self_maps_fd_entry(process)
            });
        }
        if let Some(entry) = if path_only {
            proc_smaps_path_entry(process, path.as_str())
        } else {
            proc_smaps_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_pagemap_path_entry(process, path.as_str())
        } else {
            proc_pagemap_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_timerslack_path_entry(process, path.as_str())
        } else {
            proc_timerslack_fd_entry(process, path.as_str(), fcntl_status_flags(flags))
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_pid_stat_path_entry(process, path.as_str())
        } else {
            proc_pid_stat_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_pid_status_path_entry(process, path.as_str())
        } else {
            proc_pid_status_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_comm_path_entry(process, path.as_str())
        } else {
            proc_comm_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_sysvipc_msg_path_entry(path.as_str())
        } else {
            proc_sysvipc_msg_fd_entry(path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_sysvipc_sem_path_entry(path.as_str())
        } else {
            proc_sysvipc_sem_fd_entry(path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_sysvipc_shm_path_entry(path.as_str())
        } else {
            proc_sysvipc_shm_fd_entry(path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some((synthetic_path, data)) = synthetic_userdb_content(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(if path_only {
                synthetic_userdb_path_entry(synthetic_path, data)
            } else {
                synthetic_userdb_fd_entry(synthetic_path, data)
            });
        }
        if let Some((synthetic_path, data)) = synthetic_kernel_config_content(path.as_str()) {
            if axfs::api::metadata(synthetic_path).is_err() {
                if prefer_dir {
                    return Err(LinuxError::ENOTDIR);
                }
                if !path_only && synthetic_file_is_writable_open(flags) {
                    return Err(LinuxError::EPERM);
                }
                return Ok(if path_only {
                    synthetic_kernel_config_path_entry(synthetic_path, data)
                } else {
                    synthetic_kernel_config_fd_entry(synthetic_path, data)
                });
            }
        }
        if let Some(entry) = if path_only {
            proc_sys_fs_mqueue_path_entry(path.as_str())
        } else {
            proc_sys_fs_mqueue_fd_entry(path.as_str(), fcntl_status_flags(flags))
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_sys_file_path_entry(path.as_str())
        } else {
            proc_sys_file_fd_entry(path.as_str(), fcntl_status_flags(flags))
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(entry);
        }
        if let Some((synthetic_path, data)) = synthetic_proc_sys_content(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                if synthetic_path != "/proc/sys/kernel/sem" {
                    return Err(LinuxError::EPERM);
                }
            } else {
                return Ok(if path_only {
                    synthetic_proc_sys_path_entry(synthetic_path, data)
                } else {
                    synthetic_proc_sys_fd_entry(synthetic_path, data)
                });
            }
            if axfs::api::metadata(synthetic_path).is_err() {
                return Err(LinuxError::ENOENT);
            }
        }
        if path == "/dev/null" {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_char("/dev/null"))
            } else {
                FdEntry::DevNull
            });
        }
        if path == "/dev/zero" {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_char("/dev/zero"))
            } else {
                FdEntry::DevZero(fcntl_status_flags(flags))
            });
        }
        if path == "/dev/urandom" || path == "/dev/random" {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_char(path.as_str()))
            } else {
                FdEntry::DevRandom(fcntl_status_flags(flags))
            });
        }
        if path == "/dev/cpu_dma_latency" {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            check_synthetic_open_permission(
                process,
                "/dev/cpu_dma_latency",
                &dev_cpu_dma_latency_stat(),
                flags,
            )?;
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_char_with_mode(
                    "/dev/cpu_dma_latency",
                    0o600,
                ))
            } else {
                FdEntry::DevCpuDmaLatency(fcntl_status_flags(flags))
            });
        }
        if is_synthetic_block_device_path(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_block(path.as_str()))
            } else {
                FdEntry::BlockDevice(BlockDeviceEntry::new(path.clone(), flags))
            });
        }
        if path == "/dev/misc/rtc" || path == "/dev/rtc" {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_char(path.as_str()))
            } else {
                FdEntry::Rtc
            });
        }
        if let Some(special_type @ (ST_MODE_CHR | ST_MODE_BLK)) =
            process.path_special_mode(path.as_str())
        {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            check_open_permission(process, path.as_str(), flags)?;
            if path_only {
                let mode = process.path_mode(path.as_str()).unwrap_or(0o600);
                return Ok(FdEntry::Path(PathEntry::special_node(
                    path.as_str(),
                    special_type,
                    mode,
                )));
            }
            match (special_type, process.path_rdev(path.as_str())) {
                (ST_MODE_CHR, Some(DEV_NULL_RDEV)) => return Ok(FdEntry::DevNull),
                (ST_MODE_CHR, Some(DEV_ZERO_RDEV)) => {
                    return Ok(FdEntry::DevZero(fcntl_status_flags(flags)));
                }
                (ST_MODE_CHR, Some(DEV_CPU_DMA_LATENCY_RDEV)) => {
                    return Ok(FdEntry::DevCpuDmaLatency(fcntl_status_flags(flags)));
                }
                (ST_MODE_BLK, _) => {
                    return Ok(FdEntry::BlockDevice(BlockDeviceEntry::new(
                        path.clone(),
                        flags,
                    )));
                }
                _ => {}
            }
            return Err(LinuxError::ENXIO);
        }
        if process.path_special_mode(path.as_str()) == Some(ST_MODE_FIFO) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            check_open_permission(process, path.as_str(), flags)?;
            if flags & general::O_ACCMODE == general::O_WRONLY && flags & general::O_NONBLOCK != 0 {
                if !PipeEndpoint::named_fifo_has_reader(path.as_str()) {
                    return Err(LinuxError::ENXIO);
                }
            }
            if path_only {
                let mode = process.path_mode(path.as_str()).unwrap_or(0o666);
                return Ok(FdEntry::Path(PathEntry::fifo(path.as_str(), mode)));
            }
            let status_flags = flags & (general::O_NONBLOCK | general::O_DIRECT);
            let endpoint = PipeEndpoint::new_named_fifo(path.as_str(), flags & general::O_ACCMODE);
            return Ok(FdEntry::Pipe(new_open_file(
                endpoint,
                (flags & general::O_ACCMODE) | status_flags,
            )?));
        }
        if let Some(backing_path) = process.path_hardlink_backing(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            if open_would_write_or_truncate(flags) && executable_running(backing_path.as_str()) {
                return Err(LinuxError::ETXTBSY);
            }
            check_open_permission(process, backing_path.as_str(), flags)?;
            if path_only {
                let metadata =
                    axfs::api::metadata(backing_path.as_str()).map_err(LinuxError::from)?;
                return Ok(FdEntry::Path(PathEntry::from_metadata(
                    path.as_str(),
                    &metadata,
                )));
            }
            let file = File::open(backing_path.as_str(), file_opts).map_err(LinuxError::from)?;
            if flags & general::O_TRUNC != 0 {
                invalidate_exec_image_cache(backing_path.as_str());
                process.truncate_path_sparse_file(backing_path.clone(), 0);
            }
            return Ok(FdEntry::File(FileEntry {
                file,
                path: backing_path,
                status_flags: fcntl_status_flags(flags),
                offset: Arc::new(Mutex::new(0)),
            }));
        }
        if prefer_dir {
            match open_dir_entry(path.as_str()) {
                Ok(FdEntry::Directory(dir)) if path_only => {
                    return Ok(path_entry_from_directory(dir));
                }
                Ok(entry) if !path_only => return Ok(entry),
                Ok(_) => return Err(LinuxError::EINVAL),
                Err(err) => record_missing_candidate(&mut last_err, err)?,
            }
            continue;
        }
        if flags & O_PATH_FLAG == 0
            && matches!(
                flags & general::O_ACCMODE,
                general::O_WRONLY | general::O_RDWR
            )
            && matches!(open_dir_entry(path.as_str()), Ok(FdEntry::Directory(_)))
        {
            return Err(LinuxError::EISDIR);
        }
        if !path_only && !prefer_dir && flags & general::O_ACCMODE == general::O_RDONLY {
            if let Ok(FdEntry::Directory(dir)) = open_dir_entry(path.as_str()) {
                if flags & general::O_CREAT != 0 {
                    return Err(LinuxError::EISDIR);
                }
                check_open_permission(process, path.as_str(), flags)?;
                return Ok(FdEntry::Directory(dir));
            }
        }
        let created_by_this_open = !path_only
            && flags & general::O_CREAT != 0
            && axfs::api::metadata(path.as_str()).is_err();
        if flags & general::O_NOATIME != 0 && !created_by_this_open && process.uid() != 0 {
            let owner = process
                .path_owner(path.as_str())
                .map(|(uid, _)| uid)
                .unwrap_or(0);
            if owner != process.uid() {
                return Err(LinuxError::EPERM);
            }
        }
        let create_parent_st = if created_by_this_open {
            Some(check_parent_write_search_permission(
                process,
                path.as_str(),
            )?)
        } else {
            if open_would_write_or_truncate(flags) && executable_running(path.as_str()) {
                return Err(LinuxError::ETXTBSY);
            }
            check_open_permission(process, path.as_str(), flags)?;
            None
        };
        if path_only {
            match axfs::api::metadata(path.as_str()) {
                Ok(metadata) => {
                    return Ok(FdEntry::Path(PathEntry::from_metadata(
                        path.as_str(),
                        &metadata,
                    )));
                }
                Err(err) => {
                    record_missing_candidate(&mut last_err, LinuxError::from(err))?;
                    continue;
                }
            }
        }
        match File::open(path.as_str(), file_opts) {
            Ok(file) => {
                if created_by_this_open {
                    if let Some(parent_st) = create_parent_st.as_ref() {
                        record_created_path_metadata(process, path.clone(), mode, false, parent_st);
                    }
                    record_created_path_times(process, path.clone());
                }
                if flags & general::O_TRUNC != 0 {
                    invalidate_exec_image_cache(path.as_str());
                    process.truncate_path_sparse_file(path.clone(), 0);
                }
                return Ok(FdEntry::File(FileEntry {
                    file,
                    path: path.clone(),
                    status_flags: fcntl_status_flags(flags),
                    offset: Arc::new(Mutex::new(0)),
                }));
            }
            Err(err) => {
                let err = LinuxError::from(err);
                if err == LinuxError::EISDIR {
                    return match open_dir_entry(path.as_str())? {
                        FdEntry::Directory(dir) if path_only => Ok(path_entry_from_directory(dir)),
                        entry if !path_only => Ok(entry),
                        _ => Err(LinuxError::EINVAL),
                    };
                }
                record_missing_candidate(&mut last_err, err)?;
            }
        }
    }
    Err(last_err)
}

fn path_entry_from_directory(dir: DirectoryEntry) -> FdEntry {
    FdEntry::Path(PathEntry::from_attr(dir.path.as_str(), &dir.attr))
}

fn is_synthetic_block_device_path(path: &str) -> bool {
    let Some(path) = normalize_path("/", path) else {
        return false;
    };
    let Some(name) = path.strip_prefix("/dev/") else {
        return false;
    };
    is_synthetic_virtio_block_name(name)
}

fn is_synthetic_virtio_block_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes[0] != b'v' || bytes[1] != b'd' || !bytes[2].is_ascii_lowercase() {
        return false;
    }
    bytes[3..].iter().all(|byte| byte.is_ascii_digit())
}

fn synthetic_block_device_names_in_dir(path: &str) -> &'static [&'static str] {
    match normalize_path("/", path).as_deref() {
        Some("/dev") => SYNTHETIC_BLOCK_DEVICE_NAMES,
        _ => &[],
    }
}

fn synthetic_char_device_names_in_dir(path: &str) -> &'static [&'static str] {
    match normalize_path("/", path).as_deref() {
        Some("/dev") => SYNTHETIC_CHAR_DEVICE_NAMES,
        _ => &[],
    }
}

fn record_missing_candidate(last_err: &mut LinuxError, err: LinuxError) -> Result<(), LinuxError> {
    match err {
        LinuxError::ENOENT => Ok(()),
        LinuxError::ENOTDIR => {
            // Runtime loader paths often probe absolute locations such as
            // `/lib/libc.so.6` before this compatibility layer redirects them
            // to the suite-local runtime root (`/glibc/lib/libc.so.6`,
            // `/musl/lib/libc.so`, etc.).  A missing leading directory is a
            // failed candidate, not proof that later runtime candidates are
            // invalid.  Preserve ENOTDIR as the final error if every candidate
            // misses, but keep searching the candidate list.
            if *last_err == LinuxError::ENOENT {
                *last_err = err;
            }
            Ok(())
        }
        _ => {
            *last_err = err;
            Err(err)
        }
    }
}

pub(super) fn open_dir_entry(path: &str) -> Result<FdEntry, LinuxError> {
    let mut opts = OpenOptions::new();
    opts.read(true);
    let dir = Directory::open_dir(path, &opts).map_err(LinuxError::from)?;
    let attr = dir.get_attr().map_err(LinuxError::from)?;
    Ok(FdEntry::Directory(DirectoryEntry {
        dir,
        attr,
        path: path.into(),
        next_dirent_cookie: 0,
        next_synthetic_dirent_index: 0,
        synthetic_dirents_emitted: false,
    }))
}

fn proc_fd_dir_path(process: &UserProcess, path: &str) -> Option<String> {
    let normalized = normalize_path("/", path)?;
    if normalized == "/proc/self/fd" || normalized == "/dev/fd" {
        return Some(normalized);
    }
    let pid_path = format!("/proc/{}/fd", process.pid());
    (normalized == pid_path).then_some(normalized)
}

fn proc_pid_dir_entry(process: &UserProcess, path: &str, path_only: bool) -> Option<FdEntry> {
    const SYNTHETIC_INIT_PID: i32 = 1;

    let normalized = normalize_path("/", path)?;
    let pid = if normalized == "/proc/self" {
        process.pid()
    } else {
        let rest = normalized.strip_prefix("/proc/")?;
        if rest.contains('/') {
            return None;
        }
        rest.parse::<i32>().ok()?
    };
    if pid != process.pid()
        && pid != SYNTHETIC_INIT_PID
        && process.child_thread_entry_by_pid(pid).is_none()
        && user_thread_entry_by_process_pid(pid).is_none()
    {
        return None;
    }

    if path_only {
        return Some(FdEntry::Path(PathEntry::synthetic_dir(normalized.as_str())));
    }

    let dirents = [
        ("stat", general::DT_REG as u8),
        ("status", general::DT_REG as u8),
        ("task", general::DT_DIR as u8),
        ("comm", general::DT_REG as u8),
    ]
    .into_iter()
    .map(|(name, d_type)| {
        SyntheticDirent::new(
            name.into(),
            d_type,
            format!("{}/{}", normalized.as_str(), name),
        )
    })
    .collect();
    Some(FdEntry::SyntheticDir(SyntheticDirEntry::new(
        normalized.clone(),
        parent_path(normalized.as_str()).into(),
        dirents,
    )))
}

fn proc_fd_target_number(process: &UserProcess, path: &str) -> Option<i32> {
    let normalized = normalize_path("/", path)?;
    let rest = normalized
        .strip_prefix("/proc/self/fd/")
        .or_else(|| normalized.strip_prefix("/dev/fd/"))
        .or_else(|| normalized.strip_prefix(format!("/proc/{}/fd/", process.pid()).as_str()))?;
    if rest.is_empty() || rest.contains('/') {
        return None;
    }
    rest.parse().ok()
}

fn reopen_proc_fd_entry(
    table: &FdTable,
    fd: i32,
    flags: u32,
    path_only: bool,
) -> Result<FdEntry, LinuxError> {
    match table.entry(fd)? {
        FdEntry::Memfd(file) => {
            let mut reopened = file.reopen(fcntl_status_flags(flags));
            if flags & general::O_TRUNC != 0 {
                if !reopened.writable() {
                    return Err(LinuxError::EACCES);
                }
                reopened.truncate(0)?;
            }
            if path_only {
                Ok(FdEntry::Path(PathEntry::synthetic_file(
                    reopened.path().as_str(),
                    reopened.size() as usize,
                )))
            } else {
                Ok(FdEntry::Memfd(reopened))
            }
        }
        _ => Err(LinuxError::EINVAL),
    }
}

fn directory_create_dir(path: &str) -> Result<(), LinuxError> {
    axfs::api::create_dir(path).map_err(LinuxError::from)
}

fn directory_remove_file(path: &str) -> Result<(), LinuxError> {
    axfs::api::remove_file(path).map_err(LinuxError::from)
}

fn directory_remove_dir(path: &str) -> Result<(), LinuxError> {
    axfs::api::remove_dir(path).map_err(LinuxError::from)
}

pub(super) fn resolve_dirfd_path(
    process: &UserProcess,
    table: &FdTable,
    dirfd: i32,
    path: &str,
) -> Result<String, LinuxError> {
    if path_exceeds_linux_limits(path) {
        return Err(LinuxError::ENAMETOOLONG);
    }
    if path.starts_with('/') {
        return process
            .resolve_fs_absolute_path(path)
            .and_then(|path| translate_checked_path(process, path));
    }
    if dirfd == general::AT_FDCWD {
        let cwd = process.cwd();
        return normalize_path(cwd.as_str(), path)
            .ok_or(LinuxError::EINVAL)
            .and_then(|path| translate_checked_path(process, path));
    }
    let FdEntry::Directory(dir) = table.entry(dirfd)? else {
        return Err(LinuxError::ENOTDIR);
    };
    if axfs::api::metadata(dir.path.as_str()).is_err() {
        return Err(LinuxError::ENOENT);
    }
    normalize_path(dir.path.as_str(), path)
        .ok_or(LinuxError::EINVAL)
        .and_then(|path| translate_checked_path(process, path))
}

fn translate_checked_path(process: &UserProcess, path: String) -> Result<String, LinuxError> {
    if path_exceeds_linux_limits(path.as_str()) {
        return Err(LinuxError::ENAMETOOLONG);
    }
    let translated = process.translate_mount_path(path.as_str());
    if path_exceeds_linux_limits(translated.as_str()) {
        return Err(LinuxError::ENAMETOOLONG);
    }
    Ok(translated)
}
