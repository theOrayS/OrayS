use core::cmp;
use core::mem::{offset_of, size_of};
use core::ptr;
use core::time::Duration;

use axerrno::LinuxError;
use axfs::fops::{self, Directory, File, FileAttr, OpenOptions};
use axio::SeekFrom;
use lazyinit::LazyInit;
use linux_raw_sys::{general, ioctl};
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use super::credentials::access_allowed;
use super::fd_pipe::PipeEndpoint;
use super::fd_socket::{LocalSocketEntry, SocketEntry, recv_socket_data_to_user, socket_entry};
use super::linux_abi::{
    ACCESS_R_OK, ACCESS_W_OK, ACCESS_X_OK, DEFAULT_NOFILE_LIMIT, FILE_MODE_SET_GID,
    FILE_MODE_STICKY, MAX_IN_MEMORY_FILE_SIZE, O_NOFOLLOW_FLAG, O_PATH_FLAG, RLIMIT_FSIZE_RESOURCE,
    RTC_RD_TIME, ST_MODE_BLK, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FIFO, ST_MODE_FILE, ST_MODE_LNK,
    ST_MODE_SOCKET, ST_MODE_TYPE_MASK, fd_cloexec_flag, neg_errno, posix_ret_i32,
};
use super::memory_map::align_up;
use super::metadata::{
    DEV_NULL_RDEV, DEV_ZERO_RDEV, apply_recorded_path_metadata, canonical_permission_path,
    dev_null_stat, dev_zero_stat, dirent_type, fd_entry_path, fd_entry_statfs_path,
    file_attr_to_stat, file_type_mode, generic_statfs, path_inode, stdio_stat,
    synthetic_block_stat_for_path, synthetic_char_stat_for_path,
};
use super::runtime_paths::{
    busybox_applet_target_path, normalize_path, push_runtime_candidate,
    runtime_absolute_path_candidates, runtime_library_name_candidates,
};
use super::select_fdset::{SelectMode, yield_poll_wait};
use super::signal_abi::{
    current_pending_signal_matches, current_unblocked_signal_pending,
    install_temporary_signal_mask, take_current_pending_signal_matching,
};
use super::synthetic_fs::{
    dev_shm_host_path, ensure_dev_shm_dir, is_proc_self_maps_path, proc_comm_fd_entry,
    proc_comm_path_entry, proc_pagemap_fd_entry, proc_pagemap_path_entry, proc_pid_stat_fd_entry,
    proc_pid_stat_path_entry, proc_pid_status_fd_entry, proc_pid_status_path_entry,
    proc_self_maps_fd_entry, proc_self_maps_is_writable_open, proc_self_maps_path_entry,
    proc_timerslack_fd_entry, proc_timerslack_path_entry, synthetic_file_is_writable_open,
    synthetic_kernel_config_content, synthetic_kernel_config_fd_entry,
    synthetic_kernel_config_path_entry, synthetic_proc_sys_content, synthetic_proc_sys_fd_entry,
    synthetic_proc_sys_path_entry, synthetic_userdb_content, synthetic_userdb_fd_entry,
    synthetic_userdb_path_entry,
};
use super::system_info::write_default_winsize;
use super::task_registry::user_thread_entry_by_process_pid;
use super::time_abi::{
    clock_gettime_timespec, clock_now_duration, rtc_time_from_wall_time, timespec_to_duration,
};
use super::user_memory::{
    MAX_USER_IO_CHUNK, read_cstr, read_iovec_entries, read_user_bytes, read_user_value,
    user_io_buffer, validate_user_read, validate_user_write, with_readable_user_buffer,
    with_writable_user_buffer, write_user_bytes, write_user_value,
};
use super::{PathTimes, UserProcess};

pub(super) struct FdTable {
    pub(super) entries: Vec<Option<FdEntry>>,
    pub(super) fd_flags: Vec<u32>,
}

const FD_TABLE_LIMIT: usize = DEFAULT_NOFILE_LIMIT as usize;
const LINUX_PATH_MAX: usize = 4096;
// axfs_vfs::VfsDirEntry stores 63 bytes of d_name.  Enforce and report that
// real backing limit at the POSIX boundary instead of accepting longer names
// that would later panic during directory enumeration.
const LINUX_NAME_MAX: usize = 63;
const LINUX_EPOLL_MAX_NEST_DEPTH: usize = 5;
const SYNTHETIC_BLOCK_DEVICE_NAMES: &[&str] = &["vda", "sda", "xvda"];

pub(super) enum FdEntry {
    Stdin,
    Stdout,
    Stderr,
    DevNull,
    DevZero,
    BlockDevice(BlockDeviceEntry),
    Rtc,
    File(FileEntry),
    Directory(DirectoryEntry),
    ProcFdDir(ProcFdDirEntry),
    Path(PathEntry),
    MemoryFile(MemoryFileEntry),
    ProcPagemap(ProcPagemapEntry),
    ProcTimerSlack(ProcTimerSlackEntry),
    Pipe(PipeEndpoint),
    Socket(SocketEntry),
    LocalSocket(LocalSocketEntry),
    EventFd(EventFdEntry),
    Epoll(EpollEntry),
    TimerFd(TimerFdEntry),
    SignalFd(SignalFdEntry),
}

#[derive(Clone)]
pub(super) struct FileEntry {
    pub(super) file: File,
    pub(super) path: String,
    pub(super) status_flags: u32,
    offset: Arc<Mutex<u64>>,
    lease_type: Arc<Mutex<u32>>,
}

#[derive(Clone)]
pub(super) struct DirectoryEntry {
    pub(super) dir: Directory,
    pub(super) attr: FileAttr,
    pub(super) path: String,
    next_dirent_cookie: u64,
    synthetic_dirents_emitted: bool,
}

#[derive(Clone)]
pub(super) struct ProcFdDirEntry {
    pub(super) path: String,
    next_dirent_cookie: u64,
}

#[derive(Clone)]
pub(super) struct BlockDeviceEntry {
    pub(super) path: String,
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

#[derive(Clone)]
pub(super) struct EventFdEntry {
    counter: Arc<Mutex<u64>>,
    status_flags: u32,
    semaphore: bool,
}

#[derive(Clone)]
pub(super) struct TimerFdEntry {
    clock_id: u32,
    status_flags: u32,
    state: Arc<Mutex<TimerFdState>>,
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
    registrations: Arc<Mutex<BTreeMap<i32, EpollRegistration>>>,
}

#[derive(Clone, Copy)]
struct EpollRegistration {
    event: general::epoll_event,
    last_ready: u32,
    disabled: bool,
}

impl EpollRegistration {
    const fn new(event: general::epoll_event) -> Self {
        Self {
            event,
            last_ready: 0,
            disabled: false,
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
    match process.fds.lock().open(
        process,
        dirfd as i32,
        path.as_str(),
        flags as u32,
        mode as u32,
    ) {
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
    if mode != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    let Some(end) = (offset as u64).checked_add(len as u64) else {
        return neg_errno(LinuxError::EFBIG);
    };
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    if end > file_size_limit {
        return neg_errno(LinuxError::EFBIG);
    }
    match process.fds.lock().truncate(process, fd as i32, end) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_close(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().close_for_process(process, fd as i32) {
        Ok(()) => 0,
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
    match process
        .fds
        .lock()
        .insert_with_flags(FdEntry::Epoll(EpollEntry::new()), fd_flags)
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
    match process.fds.lock().insert_with_flags(
        FdEntry::EventFd(EventFdEntry::new(
            initval as u64,
            status_flags,
            flags & general::EFD_SEMAPHORE != 0,
        )),
        fd_flags,
    ) {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
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
    match process.fds.lock().insert_with_flags(
        FdEntry::TimerFd(TimerFdEntry::new(clock_id, status_flags)),
        fd_flags,
    ) {
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
    let mut table = process.fds.lock();
    match table.epoll_ctl(epfd as i32, op as u32, fd as i32, event_value) {
        Ok(()) => 0,
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
        EpollWaitTimeout::Until(
            axhal::time::wall_time() + core::time::Duration::from_millis(timeout_ms as u64),
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
            EpollWaitTimeout::Until(
                axhal::time::wall_time()
                    + core::time::Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32),
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
    if matches!(timeout, EpollWaitTimeout::Immediate) && sigmask == 0 {
        let table = process.fds.lock();
        match table.epoll_try_fast_no_ready(epfd as i32) {
            Ok(Some(true)) => return 0,
            Ok(Some(false) | None) => {}
            Err(err) => return neg_errno(err),
        }
    }
    let _signal_mask_guard = match install_temporary_signal_mask(process, sigmask, sigsetsize) {
        Ok(guard) => guard,
        Err(err) => return neg_errno(err),
    };
    let mut ready = Vec::new();
    if matches!(timeout, EpollWaitTimeout::Immediate) {
        {
            let table = process.fds.lock();
            match table.epoll_collect_ready(epfd as i32, maxevents, &mut ready) {
                Ok(()) => {}
                Err(err) => return neg_errno(err),
            }
        }
        return copy_epoll_events_to_user(process, events, &ready);
    }
    loop {
        if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
            return neg_errno(LinuxError::EINTR);
        }
        ready.clear();
        {
            let table = process.fds.lock();
            match table.epoll_collect_ready(epfd as i32, maxevents, &mut ready) {
                Ok(()) => {}
                Err(err) => return neg_errno(err),
            }
        }
        if !ready.is_empty() {
            return copy_epoll_events_to_user(process, events, &ready);
        }
        if matches!(timeout, EpollWaitTimeout::Until(ddl) if axhal::time::wall_time() >= ddl) {
            return 0;
        }
        yield_poll_wait();
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

    fn new(initval: u64, status_flags: u32, semaphore: bool) -> Self {
        Self {
            counter: Arc::new(Mutex::new(initval)),
            status_flags: status_flags & general::O_NONBLOCK,
            semaphore,
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

    fn poll_readable(&self) -> bool {
        *self.counter.lock() > 0
    }

    fn poll_writable(&self) -> bool {
        *self.counter.lock() < Self::COUNTER_MAX
    }

    fn read(&self, process: &UserProcess, dst: &mut [u8]) -> Result<usize, LinuxError> {
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
                    dst[..size_of::<u64>()].copy_from_slice(&value.to_ne_bytes());
                    return Ok(size_of::<u64>());
                }
            }
            if self.nonblocking() {
                return Err(LinuxError::EAGAIN);
            }
            if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
                return Err(LinuxError::EINTR);
            }
            axtask::yield_now();
        }
    }

    fn write(&self, process: &UserProcess, src: &[u8]) -> Result<usize, LinuxError> {
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
                    return Ok(size_of::<u64>());
                }
            }
            if self.nonblocking() {
                return Err(LinuxError::EAGAIN);
            }
            if process.eval_watchdog_expired() || current_unblocked_signal_pending() {
                return Err(LinuxError::EINTR);
            }
            axtask::yield_now();
        }
    }
}

impl TimerFdEntry {
    fn new(clock_id: u32, status_flags: u32) -> Self {
        Self {
            clock_id,
            status_flags: status_flags & general::O_NONBLOCK,
            state: Arc::new(Mutex::new(TimerFdState {
                deadline: None,
                interval: Duration::ZERO,
                expirations: 0,
            })),
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

    fn status_flags(&self) -> u32 {
        self.status_flags
    }

    fn set_status_flags(&mut self, flags: u32) {
        self.status_flags = flags & general::O_NONBLOCK;
    }

    fn nonblocking(&self) -> bool {
        self.status_flags & general::O_NONBLOCK != 0
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
        Ok(old_spec)
    }

    fn gettime(&self) -> Result<general::itimerspec, LinuxError> {
        let mut state = self.state.lock();
        self.refresh_locked(&mut state)?;
        self.spec_from_state(&state)
    }

    fn poll_readable(&self) -> bool {
        let mut state = self.state.lock();
        self.refresh_locked(&mut state)
            .is_ok_and(|()| state.expirations > 0)
    }

    fn read(&self, process: &UserProcess, dst: &mut [u8]) -> Result<usize, LinuxError> {
        if dst.len() < size_of::<u64>() {
            return Err(LinuxError::EINVAL);
        }
        loop {
            {
                let mut state = self.state.lock();
                self.refresh_locked(&mut state)?;
                if state.expirations > 0 {
                    let value = state.expirations;
                    state.expirations = 0;
                    dst[..size_of::<u64>()].copy_from_slice(&value.to_ne_bytes());
                    return Ok(size_of::<u64>());
                }
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
    fn new() -> Self {
        Self {
            registrations: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}

pub(super) fn sys_read(process: &UserProcess, fd: usize, buf: usize, count: usize) -> isize {
    if let Ok(socket) = socket_entry(process, fd) {
        return recv_socket_data_to_user(process, socket.posix_fd, buf, count, 0);
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
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
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
    let mut written = 0isize;
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_read(process, base, remaining) {
            return if written > 0 { written } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let src = match read_user_bytes(process, base, len) {
                Ok(bytes) => bytes,
                Err(err) => return if written > 0 { written } else { neg_errno(err) },
            };
            let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
            let n = match process
                .fds
                .lock()
                .write(process, fd as i32, &src, Some(file_size_limit))
            {
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
    let mut total = 0isize;
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_write(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let mut bytes = match user_io_buffer(len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let n = match process.fds.lock().read(process, fd as i32, &mut bytes) {
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
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_write(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let mut bytes = match user_io_buffer(len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let n = match process.fds.lock().read_file_at_into_fd(
                process,
                fd as i32,
                next_offset,
                &mut bytes,
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
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_read(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let src = match read_user_bytes(process, base, len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let n = match process.fds.lock().write_file_at(
                process,
                fd as i32,
                next_offset,
                &src,
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
        if let Err(err) = table.write(process, out_fd as i32, &[], Some(file_size_limit)) {
            return neg_errno(err);
        }
    }

    let mut copied = 0usize;
    while copied < count {
        let chunk_len = (count - copied).min(MAX_USER_IO_CHUNK);
        let mut buf = match user_io_buffer(chunk_len) {
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
                Some(pos) => table.read_file_at_into_fd(process, in_fd as i32, pos, &mut buf),
                None => table
                    .read_file_at_current_offset_into_fd(process, in_fd as i32, &mut buf)
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
        let written = match process.fds.lock().write(
            process,
            out_fd as i32,
            &buf[..read],
            Some(file_size_limit),
        ) {
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
    while copied < len {
        let chunk_len = (len - copied).min(MAX_USER_IO_CHUNK);
        let mut buf = match user_io_buffer(chunk_len) {
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
                Some(pos) => table.read_file_at_into_fd(process, fd_in as i32, pos, &mut buf),
                None => table
                    .read_file_at_current_offset_into_fd(process, fd_in as i32, &mut buf)
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
    match process
        .fds
        .lock()
        .dup3(process, oldfd as i32, newfd as i32, flags as u32)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fcntl(process: &UserProcess, fd: usize, cmd: usize, arg: usize) -> isize {
    match process
        .fds
        .lock()
        .fcntl(process, fd as i32, cmd as u32, arg)
    {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_flock(process: &UserProcess, fd: usize, operation: usize) -> isize {
    match process.fds.lock().flock(fd as i32, operation as u32) {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fsync(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().entry(fd as i32) {
        Ok(FdEntry::DevNull | FdEntry::DevZero | FdEntry::BlockDevice(_) | FdEntry::Rtc) => {
            neg_errno(LinuxError::EINVAL)
        }
        Ok(_) => 0,
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
            return Ok(());
        }
    }

    axfs::api::rename(old_abs_path.as_str(), new_abs_path.as_str()).map_err(LinuxError::from)?;
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
    let cwd = process.cwd();
    let mut bytes = cwd.into_bytes();
    bytes.push(0);
    if bytes.len() > size {
        return neg_errno(LinuxError::ERANGE);
    }
    write_user_bytes(process, buf, &bytes)
        .map_or_else(|err| neg_errno(err), |_| bytes.len() as isize)
}

pub(super) fn sys_chdir(process: &UserProcess, pathname: usize) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let visible_path = {
        let mut table = process.fds.lock();
        match table.resolve_path(process, general::AT_FDCWD, path.as_str()) {
            Ok(path) => {
                let stat = match table.stat_path(process, general::AT_FDCWD, path.as_str()) {
                    Ok(stat) => stat,
                    Err(err) => return neg_errno(err),
                };
                if stat.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
                    return neg_errno(LinuxError::ENOTDIR);
                }
                let uid = process.fs_uid();
                let gid = process.fs_gid();
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
    if req as u32 == BLKGETSIZE64 && process.fds.lock().is_block_device(fd as i32) {
        let size: u64 = 512 * 1024 * 1024;
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

impl FdTable {
    pub(super) fn new() -> Self {
        Self {
            entries: vec![
                Some(FdEntry::Stdin),
                Some(FdEntry::Stdout),
                Some(FdEntry::Stderr),
            ],
            fd_flags: vec![0, 0, 0],
        }
    }

    pub(super) fn fork_copy(&self) -> Result<Self, LinuxError> {
        let mut entries = Vec::with_capacity(self.entries.len());
        let mut fd_flags = Vec::with_capacity(self.entries.len());
        for (idx, entry) in self.entries.iter().enumerate() {
            entries.push(match entry {
                Some(entry) => Some(entry.duplicate_for_fork()?),
                None => None,
            });
            fd_flags.push(if entry.is_some() {
                self.fd_flags.get(idx).copied().unwrap_or(0)
            } else {
                0
            });
        }
        Ok(Self { entries, fd_flags })
    }

    pub(super) fn is_stdio(&self, fd: i32) -> bool {
        matches!(fd, 0..=2)
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
        ) || matches!(self.entry(fd), Ok(FdEntry::DevZero))
    }

    pub(super) fn pipe_available_read(&self, fd: i32) -> Result<usize, LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(pipe) => Ok(pipe.available_read()),
            _ => Err(LinuxError::ENOTTY),
        }
    }

    pub(super) fn pipe_capacity(&self, fd: i32) -> Result<usize, LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(pipe) => Ok(pipe.capacity()),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn poll(&self, fd: i32, mode: SelectMode) -> bool {
        let Ok(entry) = self.entry(fd) else {
            return matches!(mode, SelectMode::Except);
        };
        match mode {
            SelectMode::Read => match entry {
                FdEntry::Stdin => false,
                FdEntry::Stdout | FdEntry::Stderr => false,
                FdEntry::DevNull
                | FdEntry::DevZero
                | FdEntry::BlockDevice(_)
                | FdEntry::Rtc
                | FdEntry::File(_)
                | FdEntry::Directory(_)
                | FdEntry::ProcFdDir(_)
                | FdEntry::MemoryFile(_)
                | FdEntry::ProcPagemap(_)
                | FdEntry::ProcTimerSlack(_) => true,
                FdEntry::Path(_) => false,
                FdEntry::EventFd(eventfd) => eventfd.poll_readable(),
                FdEntry::TimerFd(timerfd) => timerfd.poll_readable(),
                FdEntry::SignalFd(signalfd) => signalfd.poll_readable(),
                FdEntry::Epoll(_) => false,
                FdEntry::Pipe(pipe) => pipe.poll_readable(),
                FdEntry::Socket(socket) => socket.poll(mode),
                FdEntry::LocalSocket(socket) => socket.poll(mode),
            },
            SelectMode::Write => match entry {
                FdEntry::Stdin => false,
                FdEntry::Stdout
                | FdEntry::Stderr
                | FdEntry::DevNull
                | FdEntry::DevZero
                | FdEntry::BlockDevice(_)
                | FdEntry::Rtc => true,
                FdEntry::File(_) => true,
                FdEntry::Directory(_)
                | FdEntry::ProcFdDir(_)
                | FdEntry::Path(_)
                | FdEntry::MemoryFile(_)
                | FdEntry::ProcPagemap(_)
                | FdEntry::ProcTimerSlack(_)
                | FdEntry::TimerFd(_)
                | FdEntry::SignalFd(_)
                | FdEntry::Epoll(_) => false,
                FdEntry::EventFd(eventfd) => eventfd.poll_writable(),
                FdEntry::Pipe(pipe) => pipe.poll_writable(),
                FdEntry::Socket(socket) => socket.poll(mode),
                FdEntry::LocalSocket(socket) => socket.poll(mode),
            },
            SelectMode::Except => false,
        }
    }

    pub(super) fn epoll_ctl(
        &mut self,
        epfd: i32,
        op: u32,
        fd: i32,
        event: Option<general::epoll_event>,
    ) -> Result<(), LinuxError> {
        if epfd == fd {
            return Err(LinuxError::EINVAL);
        }
        let epoll = match self.entry(epfd)? {
            FdEntry::Epoll(epoll) => epoll.clone(),
            _ => return Err(LinuxError::EINVAL),
        };
        match op {
            general::EPOLL_CTL_ADD | general::EPOLL_CTL_MOD => {
                let Some(event) = event else {
                    return Err(LinuxError::EFAULT);
                };
                self.validate_epoll_target(epfd, fd)?;
                let mut registrations = epoll.registrations.lock();
                match op {
                    general::EPOLL_CTL_ADD => {
                        if registrations.contains_key(&fd) {
                            return Err(LinuxError::EEXIST);
                        }
                        registrations.insert(fd, EpollRegistration::new(event));
                    }
                    general::EPOLL_CTL_MOD => {
                        if !registrations.contains_key(&fd) {
                            return Err(LinuxError::ENOENT);
                        }
                        registrations.insert(fd, EpollRegistration::new(event));
                    }
                    _ => unreachable!(),
                }
            }
            general::EPOLL_CTL_DEL => {
                let mut registrations = epoll.registrations.lock();
                if registrations.remove(&fd).is_none() {
                    return Err(LinuxError::ENOENT);
                }
            }
            _ => return Err(LinuxError::EINVAL),
        }
        Ok(())
    }

    fn validate_epoll_target(&self, epfd: i32, fd: i32) -> Result<(), LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(_)
            | FdEntry::Socket(_)
            | FdEntry::LocalSocket(_)
            | FdEntry::EventFd(_)
            | FdEntry::TimerFd(_)
            | FdEntry::SignalFd(_) => Ok(()),
            FdEntry::Epoll(_) => {
                if self.epoll_reaches(fd, epfd, &mut Vec::new()) {
                    return Err(LinuxError::ELOOP);
                }
                if self.epoll_nesting_depth(fd, &mut Vec::new()) >= LINUX_EPOLL_MAX_NEST_DEPTH {
                    return Err(LinuxError::EINVAL);
                }
                Ok(())
            }
            _ => Err(LinuxError::EPERM),
        }
    }

    fn epoll_reaches(&self, start_epfd: i32, target_epfd: i32, visited: &mut Vec<i32>) -> bool {
        if start_epfd == target_epfd {
            return true;
        }
        if visited.contains(&start_epfd) {
            return false;
        }
        visited.push(start_epfd);
        let registrations = match self.entry(start_epfd) {
            Ok(FdEntry::Epoll(epoll)) => epoll.registrations.lock().clone(),
            _ => {
                visited.pop();
                return false;
            }
        };
        for fd in registrations.keys() {
            if self.epoll_reaches(*fd, target_epfd, visited) {
                return true;
            }
        }
        visited.pop();
        false
    }

    fn epoll_nesting_depth(&self, epfd: i32, visited: &mut Vec<i32>) -> usize {
        if visited.contains(&epfd) {
            return LINUX_EPOLL_MAX_NEST_DEPTH;
        }
        visited.push(epfd);
        let registrations = match self.entry(epfd) {
            Ok(FdEntry::Epoll(epoll)) => epoll.registrations.lock().clone(),
            _ => {
                visited.pop();
                return 0;
            }
        };
        let mut max_child_depth = 0usize;
        for fd in registrations.keys() {
            max_child_depth = cmp::max(max_child_depth, self.epoll_nesting_depth(*fd, visited));
        }
        visited.pop();
        max_child_depth.saturating_add(1)
    }

    pub(super) fn epoll_collect_ready(
        &self,
        epfd: i32,
        maxevents: usize,
        out: &mut Vec<general::epoll_event>,
    ) -> Result<(), LinuxError> {
        let registrations = match self.entry(epfd)? {
            FdEntry::Epoll(epoll) => epoll.registrations.clone(),
            _ => return Err(LinuxError::EINVAL),
        };
        let mut registrations = registrations.lock();
        for (&fd, registration) in registrations.iter_mut() {
            if registration.disabled {
                continue;
            }
            let ready_events = self.epoll_ready_events(fd, registration.event.events);
            let edge_triggered = registration.event.events & general::EPOLLET != 0;
            let became_ready = ready_events & !registration.last_ready != 0;
            let should_emit = ready_events != 0 && (!edge_triggered || became_ready);
            let has_capacity = out.len() < maxevents;
            if should_emit && has_capacity {
                out.push(general::epoll_event {
                    events: ready_events,
                    data: registration.event.data,
                });
                if registration.event.events & general::EPOLLONESHOT != 0 {
                    registration.disabled = true;
                }
            }
            if !should_emit || has_capacity {
                registration.last_ready = ready_events;
            }
        }
        Ok(())
    }

    pub(super) fn epoll_try_fast_no_ready(&self, epfd: i32) -> Result<Option<bool>, LinuxError> {
        let registrations = match self.entry(epfd)? {
            FdEntry::Epoll(epoll) => epoll.registrations.clone(),
            _ => return Err(LinuxError::EINVAL),
        };
        let mut registrations = registrations.lock();
        for (&fd, registration) in registrations.iter_mut() {
            if registration.disabled {
                continue;
            }
            if registration.event.events & (general::EPOLLET | general::EPOLLONESHOT) != 0 {
                return Ok(None);
            }
            let ready_events = self.epoll_ready_events(fd, registration.event.events);
            if ready_events != 0 {
                return Ok(Some(false));
            }
            registration.last_ready = 0;
        }
        Ok(Some(true))
    }

    fn epoll_ready_events(&self, fd: i32, requested: u32) -> u32 {
        let mut ready_events = 0u32;
        if requested & general::EPOLLIN != 0 && self.poll(fd, SelectMode::Read) {
            ready_events |= general::EPOLLIN;
        }
        if requested & general::EPOLLOUT != 0 && self.poll(fd, SelectMode::Write) {
            ready_events |= general::EPOLLOUT;
        }
        if requested & general::EPOLLRDHUP != 0 && self.poll_rdhup(fd) {
            ready_events |= general::EPOLLRDHUP;
        }
        if self.entry(fd).is_err() {
            ready_events |= general::EPOLLNVAL;
        }
        ready_events
    }

    fn poll_rdhup(&self, fd: i32) -> bool {
        matches!(self.entry(fd), Ok(FdEntry::Socket(socket)) if socket.poll_rdhup())
    }

    pub(super) fn read(
        &mut self,
        process: &UserProcess,
        fd: i32,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdin => Ok(0),
            FdEntry::DevNull => Ok(0),
            FdEntry::DevZero => {
                dst.fill(0);
                Ok(dst.len())
            }
            FdEntry::BlockDevice(_) => {
                dst.fill(0);
                Ok(dst.len())
            }
            FdEntry::Rtc => Ok(0),
            FdEntry::File(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file_entry_read(process, file, dst)
            }
            FdEntry::MemoryFile(file) => Ok(file.read(dst)),
            FdEntry::ProcPagemap(file) => Ok(file.read(dst)),
            FdEntry::ProcTimerSlack(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file.read(process, dst)
            }
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) => Err(LinuxError::EISDIR),
            FdEntry::Pipe(pipe) => pipe.read(dst),
            FdEntry::Socket(socket) => socket.read(dst),
            FdEntry::LocalSocket(socket) => socket.read(dst),
            FdEntry::EventFd(eventfd) => eventfd.read(process, dst),
            FdEntry::TimerFd(timerfd) => timerfd.read(process, dst),
            FdEntry::SignalFd(signalfd) => signalfd.read(process, dst),
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
            FdEntry::Stdout | FdEntry::Stderr => {
                axhal::console::write_bytes(src);
                Ok(src.len())
            }
            FdEntry::DevNull => Ok(src.len()),
            FdEntry::DevZero => Ok(src.len()),
            FdEntry::BlockDevice(_) => Ok(src.len()),
            FdEntry::Rtc => Ok(src.len()),
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file_entry_write(process, file, src, file_size_limit)
            }
            FdEntry::Pipe(pipe) => pipe.write(src),
            FdEntry::Socket(socket) => socket.write(src),
            FdEntry::LocalSocket(socket) => socket.write(src),
            FdEntry::EventFd(eventfd) => eventfd.write(process, src),
            FdEntry::ProcTimerSlack(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file.write(process, src)
            }
            FdEntry::TimerFd(_) | FdEntry::SignalFd(_) => Err(LinuxError::EINVAL),
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
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
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

    fn close_slot(&mut self, idx: usize) -> Result<(), LinuxError> {
        if let Some(FdEntry::File(file)) = self.entries[idx].as_ref() {
            release_flock_on_last_close(file);
        }
        if let Some(FdEntry::Socket(socket)) = self.entries[idx].as_ref() {
            socket.close()?;
        }
        self.entries[idx] = None;
        if let Some(flags) = self.fd_flags.get_mut(idx) {
            *flags = 0;
        }
        Ok(())
    }

    pub(super) fn close(&mut self, fd: i32) -> Result<(), LinuxError> {
        if !(0..self.entries.len() as i32).contains(&fd) || self.entries[fd as usize].is_none() {
            return Err(LinuxError::EBADF);
        }
        self.close_slot(fd as usize)
    }

    pub(super) fn close_for_process(
        &mut self,
        process: &UserProcess,
        fd: i32,
    ) -> Result<(), LinuxError> {
        if !(0..self.entries.len() as i32).contains(&fd) || self.entries[fd as usize].is_none() {
            return Err(LinuxError::EBADF);
        }
        if let Some(FdEntry::File(file)) = self.entries[fd as usize].as_ref() {
            release_posix_record_locks_for_file_owner(record_lock_key(file), process.pid());
        }
        self.close_slot(fd as usize)
    }

    pub(super) fn close_all(&mut self) {
        for idx in 0..self.entries.len() {
            let _ = self.close_slot(idx);
        }
    }

    pub(super) fn close_cloexec(&mut self) {
        for idx in 0..self.entries.len() {
            if self.fd_flags.get(idx).copied().unwrap_or(0) & general::FD_CLOEXEC == 0 {
                continue;
            }
            let _ = self.close_slot(idx);
        }
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
                let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
                if size <= physical_size || size < MAX_IN_MEMORY_FILE_SIZE {
                    file.file.truncate(size).map_err(LinuxError::from)?;
                }
                process.truncate_path_sparse_file(file.path.clone(), size);
                Ok(())
            }
            FdEntry::DevNull => Ok(()),
            FdEntry::BlockDevice(_) => Ok(()),
            FdEntry::Rtc => Ok(()),
            FdEntry::Path(_)
            | FdEntry::MemoryFile(_)
            | FdEntry::ProcPagemap(_)
            | FdEntry::ProcTimerSlack(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn lseek(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: i64,
        whence: u32,
    ) -> Result<u64, LinuxError> {
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
            FdEntry::BlockDevice(_) => Ok(0),
            FdEntry::Rtc => Ok(0),
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) => Err(LinuxError::EISDIR),
            FdEntry::Path(_) => Err(LinuxError::EBADF),
            FdEntry::MemoryFile(file) => file.seek(pos),
            FdEntry::ProcPagemap(file) => file.seek(pos),
            FdEntry::ProcTimerSlack(file) => file.seek(pos),
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
        if min_fd as usize >= FD_TABLE_LIMIT {
            return Err(LinuxError::EINVAL);
        }
        let entry = self.entry(fd)?.duplicate_for_fork()?;
        self.insert_min_with_flags(entry, min_fd as usize, fd_flags & general::FD_CLOEXEC)
    }

    pub(super) fn dup3(
        &mut self,
        process: &UserProcess,
        oldfd: i32,
        newfd: i32,
        flags: u32,
    ) -> Result<i32, LinuxError> {
        if oldfd == newfd {
            return Err(LinuxError::EINVAL);
        }
        if flags & !general::O_CLOEXEC != 0 {
            return Err(LinuxError::EINVAL);
        }
        let entry = self.entry(oldfd)?.duplicate_for_fork()?;
        if newfd < 0 {
            return Err(LinuxError::EBADF);
        }
        let newfd = newfd as usize;
        if newfd >= FD_TABLE_LIMIT {
            return Err(LinuxError::EBADF);
        }
        if self.entries.len() <= newfd {
            self.entries.resize_with(newfd + 1, || None);
            self.fd_flags.resize(newfd + 1, 0);
        } else if self.entries[newfd].is_some() {
            let _ = self.close_for_process(process, newfd as i32);
        }
        if self.fd_flags.len() <= newfd {
            self.fd_flags.resize(newfd + 1, 0);
        }
        self.entries[newfd] = Some(entry);
        self.fd_flags[newfd] = fd_cloexec_flag(flags & general::O_CLOEXEC != 0);
        Ok(newfd as i32)
    }

    pub(super) fn getdents64(
        &mut self,
        process: &UserProcess,
        fd: i32,
        max_len: usize,
    ) -> Result<Vec<u8>, LinuxError> {
        if matches!(self.entry(fd)?, FdEntry::ProcFdDir(_)) {
            let fd_names = self
                .entries
                .iter()
                .enumerate()
                .filter_map(|(idx, entry)| entry.as_ref().map(|_| idx.to_string()))
                .collect::<Vec<_>>();
            let FdEntry::ProcFdDir(dir) = self.entry_mut(fd)? else {
                unreachable!();
            };
            return get_proc_fd_dirents(dir, &fd_names, max_len);
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
            for name in process.path_symlink_names_in_dir(dir.path.as_str()) {
                if seen_names.iter().any(|seen| seen == &name) {
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
                let entry_path = normalize_path(dir.path.as_str(), name.as_str());
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
                            d_type: general::DT_LNK as u8,
                            d_name: Default::default(),
                        },
                    );
                }
                let name_start = start + offset_of!(general::linux_dirent64, d_name);
                out[name_start..name_start + name_bytes.len()].copy_from_slice(name_bytes);
            }
            for &name in synthetic_block_device_names_in_dir(dir.path.as_str()) {
                if seen_names.iter().any(|seen| seen == name) {
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
                            d_type: general::DT_BLK as u8,
                            d_name: Default::default(),
                        },
                    );
                }
                let name_start = start + offset_of!(general::linux_dirent64, d_name);
                out[name_start..name_start + name_bytes.len()].copy_from_slice(name_bytes);
            }
            dir.synthetic_dirents_emitted = true;
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
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) | FdEntry::ProcFdDir(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        if !file_is_readable(file.status_flags) {
            return Err(LinuxError::EBADF);
        }
        read_regular_file_at(process, file, offset, dst)
    }

    pub(super) fn read_file_at_current_offset_into_fd(
        &mut self,
        process: &UserProcess,
        fd: i32,
        dst: &mut [u8],
    ) -> Result<(u64, usize), LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) | FdEntry::ProcFdDir(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        if !file_is_readable(file.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let offset = *file.offset.lock();
        read_regular_file_at(process, file, offset, dst).map(|read| (offset, read))
    }

    pub(super) fn advance_file_offset_fd(
        &mut self,
        fd: i32,
        amount: usize,
    ) -> Result<(), LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) | FdEntry::ProcFdDir(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        let mut offset = file.offset.lock();
        *offset = offset.saturating_add(amount as u64);
        Ok(())
    }

    pub(super) fn mmap_read_file_at_into_fd(
        &mut self,
        process: &UserProcess,
        fd: i32,
        offset: u64,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) | FdEntry::ProcFdDir(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        if !file_is_readable(file.status_flags) {
            return Err(LinuxError::EACCES);
        }
        read_regular_file_at(process, file, offset, dst)
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
            FdEntry::Directory(_) | FdEntry::ProcFdDir(_) => Err(LinuxError::EISDIR),
            FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                Err(LinuxError::ESPIPE)
            }
            FdEntry::DevZero => Ok(()),
            _ => Err(LinuxError::EBADF),
        }
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
        if min_fd >= FD_TABLE_LIMIT {
            return Err(LinuxError::EMFILE);
        }
        if self.entries.len() < min_fd {
            self.entries.resize_with(min_fd, || None);
            self.fd_flags.resize(min_fd, 0);
        }
        if self.fd_flags.len() < self.entries.len() {
            self.fd_flags.resize(self.entries.len(), 0);
        }
        if let Some((idx, slot)) = self
            .entries
            .iter_mut()
            .enumerate()
            .take(FD_TABLE_LIMIT)
            .skip(min_fd)
            .find(|(_, slot)| slot.is_none())
        {
            *slot = Some(entry);
            self.fd_flags[idx] = fd_flags & general::FD_CLOEXEC;
            return Ok(idx as i32);
        }
        if self.entries.len() >= FD_TABLE_LIMIT {
            return Err(LinuxError::EMFILE);
        }
        self.entries.push(Some(entry));
        self.fd_flags.push(fd_flags & general::FD_CLOEXEC);
        Ok((self.entries.len() - 1) as i32)
    }

    pub(super) fn get_fd_flags(&self, fd: i32) -> Result<i32, LinuxError> {
        self.entry(fd)?;
        Ok(self.fd_flags.get(fd as usize).copied().unwrap_or(0) as i32)
    }

    pub(super) fn set_fd_flags(&mut self, fd: i32, flags: u32) -> Result<i32, LinuxError> {
        self.entry(fd)?;
        let idx = fd as usize;
        if self.fd_flags.len() <= idx {
            self.fd_flags.resize(idx + 1, 0);
        }
        self.fd_flags[idx] = flags & general::FD_CLOEXEC;
        Ok(0)
    }

    pub(super) fn entry(&self, fd: i32) -> Result<&FdEntry, LinuxError> {
        self.entries
            .get(fd as usize)
            .and_then(|entry| entry.as_ref())
            .ok_or(LinuxError::EBADF)
    }

    pub(super) fn entry_mut(&mut self, fd: i32) -> Result<&mut FdEntry, LinuxError> {
        self.entries
            .get_mut(fd as usize)
            .and_then(|entry| entry.as_mut())
            .ok_or(LinuxError::EBADF)
    }
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
        }
        removed
    }

    pub(super) fn stat(&mut self, fd: i32) -> Result<general::stat, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdin => Ok(stdio_stat(true)),
            FdEntry::Stdout | FdEntry::Stderr => Ok(stdio_stat(false)),
            FdEntry::DevNull => Ok(dev_null_stat()),
            FdEntry::DevZero => Ok(dev_zero_stat()),
            FdEntry::BlockDevice(dev) => Ok(PathEntry::synthetic_block(dev.path.as_str()).stat()),
            FdEntry::Rtc => Ok(stdio_stat(false)),
            FdEntry::File(file) => Ok(file_attr_to_stat(
                &file.file.get_attr().map_err(LinuxError::from)?,
                Some(file.path.as_str()),
            )),
            FdEntry::Directory(dir) => Ok(file_attr_to_stat(&dir.attr, Some(dir.path.as_str()))),
            FdEntry::ProcFdDir(dir) => Ok(proc_fd_dir_stat(dir.path.as_str())),
            FdEntry::Path(path) => Ok(path.stat()),
            FdEntry::MemoryFile(file) => Ok(file.stat()),
            FdEntry::ProcPagemap(file) => Ok(file.stat()),
            FdEntry::ProcTimerSlack(file) => Ok(file.stat()),
            FdEntry::Pipe(pipe) => Ok(pipe.stat()),
            FdEntry::Socket(socket) => Ok(socket.stat()),
            FdEntry::LocalSocket(socket) => Ok(socket.stat()),
            FdEntry::EventFd(_) => Ok(PathEntry::synthetic_file("anon_inode:[eventfd]", 0).stat()),
            FdEntry::Epoll(_) => Ok(PathEntry::synthetic_file("anon_inode:[eventpoll]", 0).stat()),
            FdEntry::TimerFd(_) => Ok(PathEntry::synthetic_file("anon_inode:[timerfd]", 0).stat()),
            FdEntry::SignalFd(_) => {
                Ok(PathEntry::synthetic_file("anon_inode:[signalfd]", 0).stat())
            }
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
            Ok(FdEntry::DevNull) | Ok(FdEntry::DevZero) | Ok(FdEntry::Rtc) => Ok(stdio_stat(false)),
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
            normalize_path("/", path).ok_or(LinuxError::EINVAL)?
        } else if dirfd == general::AT_FDCWD {
            let cwd = process.cwd();
            normalize_path(cwd.as_str(), path).ok_or(LinuxError::EINVAL)?
        } else {
            let base = match self.entry(dirfd)? {
                FdEntry::Directory(dir) => dir.path.as_str(),
                FdEntry::ProcFdDir(dir) => dir.path.as_str(),
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
        let entry = open_fd_entry(process, self, dirfd, path, O_PATH_FLAG, 0)?;
        let uid = process.fs_uid();
        if uid != 0 {
            let resolved_path = self.resolve_path(process, dirfd, path)?;
            if !self.parent_dirs_searchable(
                process,
                resolved_path.as_str(),
                uid,
                process.fs_gid(),
            )? {
                return Err(LinuxError::EACCES);
            }
        }
        Ok(generic_statfs(fd_entry_statfs_path(&entry)))
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
                general::F_DUPFD_CLOEXEC => self.insert_min_with_flags(
                    self.entry(fd)?.duplicate_for_fork()?,
                    arg,
                    general::FD_CLOEXEC,
                ),
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
                _ => Ok(0),
            };
        }
        let socket = match self.entry(fd)? {
            FdEntry::Socket(socket) => Some(socket.clone()),
            _ => None,
        };
        if let Some(socket) = socket {
            return match cmd {
                general::F_DUPFD => {
                    self.insert_min_with_flags(FdEntry::Socket(socket.duplicate()?), arg, 0)
                }
                general::F_DUPFD_CLOEXEC => self.insert_min_with_flags(
                    FdEntry::Socket(socket.duplicate()?),
                    arg,
                    general::FD_CLOEXEC,
                ),
                general::F_GETFD => self.get_fd_flags(fd),
                general::F_SETFD => self.set_fd_flags(fd, arg as u32),
                general::F_GETFL | general::F_SETFL => posix_ret_i32(arceos_posix_api::sys_fcntl(
                    socket.posix_fd,
                    cmd as i32,
                    arg,
                )),
                _ => Ok(0),
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
                FdEntry::File(file) => Ok(file.status_flags as i32),
                FdEntry::Pipe(pipe) => Ok(pipe.status_flags() as i32),
                FdEntry::EventFd(eventfd) => Ok(eventfd.status_flags() as i32),
                FdEntry::TimerFd(timerfd) => Ok(timerfd.status_flags() as i32),
                FdEntry::SignalFd(signalfd) => Ok(signalfd.status_flags() as i32),
                FdEntry::ProcTimerSlack(file) => Ok(file.status_flags as i32),
                _ => Ok(0),
            },
            F_GETPIPE_SZ => Ok(self.pipe_capacity(fd)? as i32),
            F_SETPIPE_SZ => match self.entry(fd)? {
                FdEntry::Pipe(pipe) => Ok(pipe.set_capacity(arg as usize)? as i32),
                _ => Err(LinuxError::EBADF),
            },
            general::F_SETFL => match self.entry_mut(fd)? {
                FdEntry::File(file) => {
                    file.status_flags =
                        (file.status_flags & general::O_ACCMODE) | fcntl_setfl_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::Pipe(pipe) => {
                    pipe.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::EventFd(eventfd) => {
                    eventfd.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::TimerFd(timerfd) => {
                    timerfd.set_status_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::SignalFd(signalfd) => {
                    signalfd.set_status_flags(arg as u32);
                    Ok(0)
                }
                _ => Ok(0),
            },
            general::F_GETLK => self.fcntl_getlk(process, fd, arg),
            general::F_SETLK => self.fcntl_setlk(process, fd, arg, false),
            general::F_SETLKW => self.fcntl_setlk(process, fd, arg, true),
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
            lock.l_pid = conflict.owner_pid as _;
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
        apply_record_lock(key, request, wait)?;
        Ok(0)
    }

    fn fcntl_getlease(&mut self, fd: i32) -> Result<i32, LinuxError> {
        match self.entry(fd)? {
            FdEntry::File(file) => Ok(*file.lease_type.lock() as i32),
            _ => Err(LinuxError::EINVAL),
        }
    }

    fn fcntl_setlease(&mut self, fd: i32, lease_type: u32) -> Result<i32, LinuxError> {
        match self.entry(fd)? {
            FdEntry::File(file) => match lease_type {
                general::F_RDLCK => {
                    if file.status_flags & general::O_ACCMODE != general::O_RDONLY {
                        return Err(LinuxError::EAGAIN);
                    }
                    *file.lease_type.lock() = lease_type;
                    Ok(())
                }
                general::F_WRLCK | general::F_UNLCK => {
                    *file.lease_type.lock() = lease_type;
                    Ok(())
                }
                _ => Err(LinuxError::EINVAL),
            },
            _ => return Err(LinuxError::EINVAL),
        }?;
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
        Self {
            path: path.into(),
            mode: ST_MODE_CHR | 0o440,
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
    owner_pid: i32,
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
        if self.owner_pid == request.owner_pid || !self.overlaps(request) {
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
    if !POSIX_RECORD_LOCKS.is_inited() {
        POSIX_RECORD_LOCKS.init_once(Mutex::new(BTreeMap::new()));
    }
    &POSIX_RECORD_LOCKS
}

fn record_lock_key(file: &FileEntry) -> u64 {
    path_inode(Some(file.path.as_str()))
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
        owner_pid: process.pid(),
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

fn cleanup_dead_record_locks(locks: &mut Vec<PosixRecordLock>, current_pid: i32) {
    locks.retain(|lock| {
        lock.owner_pid == current_pid || user_thread_entry_by_process_pid(lock.owner_pid).is_some()
    });
}

fn merge_record_locks(locks: &mut Vec<PosixRecordLock>) {
    locks.sort_by_key(|lock| (lock.owner_pid, lock.typ, lock.start, lock.end()));
    let mut merged: Vec<PosixRecordLock> = Vec::new();
    for lock in locks.drain(..) {
        if let Some(last) = merged.last_mut() {
            if last.owner_pid == lock.owner_pid && last.typ == lock.typ && lock.start <= last.end()
            {
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
        if lock.owner_pid != request.owner_pid || !lock.overlaps(request) {
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
    cleanup_dead_record_locks(locks, request.owner_pid);
    locks.sort_by_key(|lock| (lock.start, lock.end(), lock.owner_pid));
    let conflict = locks
        .iter()
        .find(|lock| lock.conflicts_with(request))
        .cloned();
    if locks.is_empty() {
        table.remove(&key);
    }
    conflict
}

fn apply_record_lock(key: u64, request: PosixRecordLock, wait: bool) -> Result<(), LinuxError> {
    loop {
        let mut table = posix_record_lock_table().lock();
        let locks = table.entry(key).or_insert_with(Vec::new);
        cleanup_dead_record_locks(locks, request.owner_pid);
        if request.typ as u32 != general::F_UNLCK
            && locks.iter().any(|lock| lock.conflicts_with(&request))
        {
            if !wait {
                if locks.is_empty() {
                    table.remove(&key);
                }
                return Err(LinuxError::EAGAIN);
            }
            drop(table);
            axtask::yield_now();
            continue;
        }
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
        locks.retain(|lock| lock.owner_pid != owner_pid);
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
    let empty_keys: Vec<u64> = table
        .iter_mut()
        .filter_map(|(key, locks)| {
            locks.retain(|lock| lock.owner_pid != owner_pid);
            locks.is_empty().then_some(*key)
        })
        .collect();
    for key in empty_keys {
        table.remove(&key);
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
    if !FLOCKS.is_inited() {
        FLOCKS.init_once(Mutex::new(BTreeMap::new()));
    }
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
    pub(super) fn duplicate_for_fork(&self) -> Result<Self, LinuxError> {
        match self {
            Self::Stdin => Ok(Self::Stdin),
            Self::Stdout => Ok(Self::Stdout),
            Self::Stderr => Ok(Self::Stderr),
            Self::DevNull => Ok(Self::DevNull),
            Self::DevZero => Ok(Self::DevZero),
            Self::BlockDevice(dev) => Ok(Self::BlockDevice(dev.clone())),
            Self::Rtc => Ok(Self::Rtc),
            Self::File(file) => Ok(Self::File(file.clone())),
            Self::Directory(dir) => Ok(Self::Directory(dir.clone())),
            Self::ProcFdDir(dir) => Ok(Self::ProcFdDir(dir.clone())),
            Self::Path(path) => Ok(Self::Path(path.clone())),
            Self::MemoryFile(file) => Ok(Self::MemoryFile(file.clone())),
            Self::ProcPagemap(file) => Ok(Self::ProcPagemap(file.clone())),
            Self::ProcTimerSlack(file) => Ok(Self::ProcTimerSlack(file.clone())),
            Self::Pipe(pipe) => Ok(Self::Pipe(pipe.clone())),
            Self::Socket(socket) => socket.duplicate().map(Self::Socket),
            Self::LocalSocket(socket) => Ok(Self::LocalSocket(socket.duplicate())),
            Self::EventFd(eventfd) => Ok(Self::EventFd(eventfd.clone())),
            Self::Epoll(epoll) => Ok(Self::Epoll(epoll.clone())),
            Self::TimerFd(timerfd) => Ok(Self::TimerFd(timerfd.clone())),
            Self::SignalFd(signalfd) => Ok(Self::SignalFd(signalfd.clone())),
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
    let add_busybox_aliases = busybox_applet_alias_allowed(flags, access);

    if absolute || dirfd == general::AT_FDCWD {
        let mut candidates = if absolute {
            if let Some(path) = dev_shm_host_path(path) {
                ensure_dev_shm_dir()?;
                return open_candidates(process, &[path], &opts, flags, mode);
            }
            runtime_absolute_path_candidates(exec_root.as_str(), path)
        } else {
            let cwd = process.cwd();
            let primary = normalize_path(cwd.as_str(), path).ok_or(LinuxError::EINVAL)?;
            let mut candidates = vec![primary];
            for extra in runtime_library_name_candidates(exec_root.as_str(), path) {
                push_runtime_candidate(&mut candidates, Some(extra));
            }
            candidates
        };
        if add_busybox_aliases {
            append_busybox_applet_alias_candidates(&mut candidates);
        }
        translate_mount_candidates(process, &mut candidates);
        if candidates.is_empty() {
            return Err(LinuxError::EINVAL);
        }
        open_candidates(process, &candidates, &opts, flags, mode)
    } else {
        let FdEntry::Directory(dir) = table.entry(dirfd)? else {
            return Err(LinuxError::ENOTDIR);
        };
        let primary = normalize_path(dir.path.as_str(), path).ok_or(LinuxError::EINVAL)?;
        let mut candidates = vec![primary];
        for extra in runtime_library_name_candidates(exec_root.as_str(), path) {
            push_runtime_candidate(&mut candidates, Some(extra));
        }
        if add_busybox_aliases {
            append_busybox_applet_alias_candidates(&mut candidates);
        }
        translate_mount_candidates(process, &mut candidates);
        open_candidates(process, &candidates, &opts, flags, mode)
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

fn busybox_applet_alias_allowed(flags: u32, access: u32) -> bool {
    access != general::O_WRONLY
        && access != general::O_RDWR
        && flags & (general::O_CREAT | general::O_TRUNC | general::O_APPEND) == 0
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

fn file_logical_size(process: &UserProcess, file: &FileEntry) -> Result<u64, LinuxError> {
    let physical_size = file.file.get_attr().map_err(LinuxError::from)?.size();
    Ok(process
        .path_sparse_size(file.path.as_str())
        .unwrap_or(physical_size)
        .max(physical_size))
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
    let logical_size = file_logical_size(process, file)?;
    if offset >= logical_size {
        return Ok(0);
    }
    let read_len = cmp::min(
        dst.len(),
        logical_size.saturating_sub(offset).min(usize::MAX as u64) as usize,
    );
    dst[..read_len].fill(0);
    if offset < physical_size {
        let physical_len = cmp::min(
            read_len,
            physical_size.saturating_sub(offset).min(usize::MAX as u64) as usize,
        );
        let mut physical_read = 0usize;
        while physical_read < physical_len {
            let chunk = file
                .file
                .read_at(
                    offset.saturating_add(physical_read as u64),
                    &mut dst[physical_read..physical_len],
                )
                .map_err(LinuxError::from)?;
            if chunk == 0 {
                break;
            }
            physical_read += chunk;
        }
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

fn write_regular_file_at(
    process: &UserProcess,
    file: &mut FileEntry,
    write_offset: u64,
    src: &[u8],
    file_size_limit: Option<u64>,
) -> Result<usize, LinuxError> {
    let src = limit_regular_file_write_len(src, file_size_limit, write_offset)?;
    if src.is_empty() {
        return Ok(0);
    }
    let logical_before = file_logical_size(process, file)?;
    let mut written = 0usize;
    if write_offset < MAX_IN_MEMORY_FILE_SIZE {
        let physical_len = cmp::min(
            src.len(),
            MAX_IN_MEMORY_FILE_SIZE
                .saturating_sub(write_offset)
                .min(usize::MAX as u64) as usize,
        );
        if physical_len > 0 {
            let count = file
                .file
                .write_at(write_offset, &src[..physical_len])
                .map_err(LinuxError::from)?;
            written += count;
            if count < physical_len {
                let logical_after = logical_before.max(write_offset.saturating_add(written as u64));
                process.set_path_sparse_size(file.path.clone(), logical_after);
                touch_regular_file_after_write(process, file);
                return Ok(written);
            }
        }
    }

    if written < src.len() {
        let sparse_offset = write_offset.saturating_add(written as u64);
        process.write_path_sparse_data(file.path.clone(), sparse_offset, &src[written..]);
        written = src.len();
    }
    let logical_after = logical_before.max(write_offset.saturating_add(written as u64));
    process.set_path_sparse_size(file.path.clone(), logical_after);
    touch_regular_file_after_write(process, file);
    Ok(written)
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
        // inherited group. Root/CAP_FSETID-style creators keep requested
        // setgid, which preserves the open10/creat08 root_setgid case while
        // still clearing it for creat09's unprivileged mismatch.
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

fn append_busybox_applet_alias_candidates(candidates: &mut Vec<String>) {
    for candidate in candidates.clone() {
        push_runtime_candidate(candidates, busybox_applet_target_path(candidate.as_str()));
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

fn tmpfile_requested(flags: u32) -> bool {
    flags & general::O_TMPFILE == general::O_TMPFILE
}

fn open_candidates(
    process: &UserProcess,
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
        if flags & O_NOFOLLOW_FLAG != 0 {
            let resolved_path = process.resolve_parent_symlinks(path.as_str())?;
            if resolved_path != *path {
                return open_candidates(process, &[resolved_path], opts, flags, mode);
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
            return open_candidates(process, &[resolved_path], opts, flags, mode);
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
        if let Some((synthetic_path, data)) = synthetic_proc_sys_content(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(if path_only {
                synthetic_proc_sys_path_entry(synthetic_path, data)
            } else {
                synthetic_proc_sys_fd_entry(synthetic_path, data)
            });
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
                FdEntry::DevZero
            });
        }
        if is_synthetic_block_device_path(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_block(path.as_str()))
            } else {
                FdEntry::BlockDevice(BlockDeviceEntry { path: path.clone() })
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
                (ST_MODE_CHR, Some(DEV_ZERO_RDEV)) => return Ok(FdEntry::DevZero),
                (ST_MODE_BLK, _) => {
                    return Ok(FdEntry::BlockDevice(BlockDeviceEntry {
                        path: path.clone(),
                    }));
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
                // This compatibility layer does not keep a rendezvous table
                // for named FIFO opens.  A nonblocking writer therefore has
                // no observable reader and must fail like Linux open(2).
                return Err(LinuxError::ENXIO);
            }
            if path_only {
                let mode = process.path_mode(path.as_str()).unwrap_or(0o666);
                return Ok(FdEntry::Path(PathEntry::fifo(path.as_str(), mode)));
            }
            let status_flags = flags & (general::O_NONBLOCK | general::O_DIRECT);
            return Ok(match flags & general::O_ACCMODE {
                general::O_WRONLY => {
                    let (_, write_end) = PipeEndpoint::new_pair(status_flags);
                    FdEntry::Pipe(write_end)
                }
                // Opening a FIFO with O_RDWR is Linux-specific but common in
                // tests to avoid blocking; any pipe endpoint is non-seekable,
                // preserving the required ESPIPE semantics.
                general::O_RDWR => FdEntry::Pipe(PipeEndpoint::new_fifo_readwrite(status_flags)),
                _ => {
                    let (read_end, _) = PipeEndpoint::new_pair(status_flags);
                    FdEntry::Pipe(read_end)
                }
            });
        }
        if let Some(backing_path) = process.path_hardlink_backing(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            check_open_permission(process, backing_path.as_str(), flags)?;
            let file = File::open(backing_path.as_str(), file_opts).map_err(LinuxError::from)?;
            if path_only {
                let attr = file.get_attr().map_err(LinuxError::from)?;
                return Ok(FdEntry::Path(PathEntry::from_attr(path.as_str(), &attr)));
            }
            if flags & general::O_TRUNC != 0 {
                process.truncate_path_sparse_file(backing_path.clone(), 0);
            }
            return Ok(FdEntry::File(FileEntry {
                file,
                path: backing_path,
                status_flags: fcntl_status_flags(flags),
                offset: Arc::new(Mutex::new(0)),
                lease_type: Arc::new(Mutex::new(general::F_UNLCK)),
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
            check_open_permission(process, path.as_str(), flags)?;
            None
        };
        match File::open(path.as_str(), file_opts) {
            Ok(file) if path_only => {
                let attr = file.get_attr().map_err(LinuxError::from)?;
                return Ok(FdEntry::Path(PathEntry::from_attr(path.as_str(), &attr)));
            }
            Ok(file) => {
                if created_by_this_open {
                    if let Some(parent_st) = create_parent_st.as_ref() {
                        record_created_path_metadata(process, path.clone(), mode, false, parent_st);
                    }
                    record_created_path_times(process, path.clone());
                }
                if flags & general::O_TRUNC != 0 {
                    process.truncate_path_sparse_file(path.clone(), 0);
                }
                return Ok(FdEntry::File(FileEntry {
                    file,
                    path: path.clone(),
                    status_flags: fcntl_status_flags(flags),
                    offset: Arc::new(Mutex::new(0)),
                    lease_type: Arc::new(Mutex::new(general::F_UNLCK)),
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
    !name.contains('/')
        && SYNTHETIC_BLOCK_DEVICE_NAMES
            .iter()
            .any(|candidate| *candidate == name)
}

fn synthetic_block_device_names_in_dir(path: &str) -> &'static [&'static str] {
    match normalize_path("/", path).as_deref() {
        Some("/dev") => SYNTHETIC_BLOCK_DEVICE_NAMES,
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
        return normalize_path("/", path)
            .ok_or(LinuxError::EINVAL)
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
