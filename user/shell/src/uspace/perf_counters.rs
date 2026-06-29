use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Default)]
pub(crate) struct PerfSnapshot {
    pub(crate) syscalls: u64,
    pub(crate) fs_syscalls: u64,
    pub(crate) exec_syscalls: u64,
    pub(crate) mmap_syscalls: u64,
    pub(crate) iovec_syscalls: u64,
    pub(crate) epoll_syscalls: u64,
    pub(crate) futex_syscalls: u64,
    pub(crate) user_copy_faults: u64,
    pub(crate) user_copy_read_bytes: u64,
    pub(crate) user_copy_write_bytes: u64,
    pub(crate) iovec_tables: u64,
    pub(crate) iovec_entries: u64,
    pub(crate) exec_images: u64,
    pub(crate) exec_image_bytes: u64,
    pub(crate) mmap_calls: u64,
    pub(crate) mmap_file_backed_calls: u64,
    pub(crate) mmap_page_faults: u64,
    pub(crate) epoll_waits: u64,
    pub(crate) epoll_ready_scans: u64,
    pub(crate) futex_calls: u64,
    pub(crate) futex_waits: u64,
    pub(crate) poll_fd_scans: u64,
    pub(crate) poll_waits: u64,
}

struct PerfCounters {
    syscalls: AtomicU64,
    fs_syscalls: AtomicU64,
    exec_syscalls: AtomicU64,
    mmap_syscalls: AtomicU64,
    iovec_syscalls: AtomicU64,
    epoll_syscalls: AtomicU64,
    futex_syscalls: AtomicU64,
    user_copy_faults: AtomicU64,
    user_copy_read_bytes: AtomicU64,
    user_copy_write_bytes: AtomicU64,
    iovec_tables: AtomicU64,
    iovec_entries: AtomicU64,
    exec_images: AtomicU64,
    exec_image_bytes: AtomicU64,
    mmap_calls: AtomicU64,
    mmap_file_backed_calls: AtomicU64,
    mmap_page_faults: AtomicU64,
    epoll_waits: AtomicU64,
    epoll_ready_scans: AtomicU64,
    futex_calls: AtomicU64,
    futex_waits: AtomicU64,
    poll_fd_scans: AtomicU64,
    poll_waits: AtomicU64,
}

static PERF: PerfCounters = PerfCounters {
    syscalls: AtomicU64::new(0),
    fs_syscalls: AtomicU64::new(0),
    exec_syscalls: AtomicU64::new(0),
    mmap_syscalls: AtomicU64::new(0),
    iovec_syscalls: AtomicU64::new(0),
    epoll_syscalls: AtomicU64::new(0),
    futex_syscalls: AtomicU64::new(0),
    user_copy_faults: AtomicU64::new(0),
    user_copy_read_bytes: AtomicU64::new(0),
    user_copy_write_bytes: AtomicU64::new(0),
    iovec_tables: AtomicU64::new(0),
    iovec_entries: AtomicU64::new(0),
    exec_images: AtomicU64::new(0),
    exec_image_bytes: AtomicU64::new(0),
    mmap_calls: AtomicU64::new(0),
    mmap_file_backed_calls: AtomicU64::new(0),
    mmap_page_faults: AtomicU64::new(0),
    epoll_waits: AtomicU64::new(0),
    epoll_ready_scans: AtomicU64::new(0),
    futex_calls: AtomicU64::new(0),
    futex_waits: AtomicU64::new(0),
    poll_fd_scans: AtomicU64::new(0),
    poll_waits: AtomicU64::new(0),
};

#[inline(always)]
fn enabled() -> bool {
    option_env!("USER_PERF") == Some("1")
}

#[inline(always)]
fn add(counter: &AtomicU64, value: u64) {
    if enabled() && value != 0 {
        counter.fetch_add(value, Ordering::Relaxed);
    }
}

#[inline(always)]
pub(super) fn record_syscall(syscall_num: u32) {
    if !enabled() {
        return;
    }
    add(&PERF.syscalls, 1);
    match syscall_num {
        linux_raw_sys::general::__NR_execve => add(&PERF.exec_syscalls, 1),
        linux_raw_sys::general::__NR_mmap
        | linux_raw_sys::general::__NR_munmap
        | linux_raw_sys::general::__NR_mprotect
        | linux_raw_sys::general::__NR_mremap
        | linux_raw_sys::general::__NR_msync
        | linux_raw_sys::general::__NR_madvise
        | linux_raw_sys::general::__NR_mincore
        | linux_raw_sys::general::__NR_brk => add(&PERF.mmap_syscalls, 1),
        linux_raw_sys::general::__NR_readv
        | linux_raw_sys::general::__NR_writev
        | linux_raw_sys::general::__NR_preadv
        | linux_raw_sys::general::__NR_pwritev
        | linux_raw_sys::general::__NR_preadv2
        | linux_raw_sys::general::__NR_pwritev2 => add(&PERF.iovec_syscalls, 1),
        linux_raw_sys::general::__NR_epoll_create1
        | linux_raw_sys::general::__NR_epoll_ctl
        | linux_raw_sys::general::__NR_epoll_pwait
        | linux_raw_sys::general::__NR_epoll_pwait2 => add(&PERF.epoll_syscalls, 1),
        linux_raw_sys::general::__NR_futex => add(&PERF.futex_syscalls, 1),
        linux_raw_sys::general::__NR_openat
        | linux_raw_sys::general::__NR_openat2
        | linux_raw_sys::general::__NR_close
        | linux_raw_sys::general::__NR_read
        | linux_raw_sys::general::__NR_write
        | linux_raw_sys::general::__NR_pread64
        | linux_raw_sys::general::__NR_pwrite64
        | linux_raw_sys::general::__NR_newfstatat
        | linux_raw_sys::general::__NR_fstat
        | linux_raw_sys::general::__NR_statx
        | linux_raw_sys::general::__NR_getdents64
        | linux_raw_sys::general::__NR_lseek
        | linux_raw_sys::general::__NR_fsync
        | linux_raw_sys::general::__NR_fdatasync
        | linux_raw_sys::general::__NR_ftruncate
        | linux_raw_sys::general::__NR_truncate
        | linux_raw_sys::general::__NR_unlinkat
        | linux_raw_sys::general::__NR_renameat2
        | linux_raw_sys::general::__NR_mkdirat
        | linux_raw_sys::general::__NR_readlinkat => add(&PERF.fs_syscalls, 1),
        _ => {}
    }
}

#[inline(always)]
pub(super) fn record_user_copy_fault() {
    add(&PERF.user_copy_faults, 1);
}

#[inline(always)]
pub(super) fn record_user_copy_read(len: usize) {
    add(&PERF.user_copy_read_bytes, len as u64);
}

#[inline(always)]
pub(super) fn record_user_copy_write(len: usize) {
    add(&PERF.user_copy_write_bytes, len as u64);
}

#[inline(always)]
pub(super) fn record_iovec_table(entries: usize) {
    add(&PERF.iovec_tables, 1);
    add(&PERF.iovec_entries, entries as u64);
}

#[inline(always)]
pub(super) fn record_exec_image(bytes: usize) {
    add(&PERF.exec_images, 1);
    add(&PERF.exec_image_bytes, bytes as u64);
}

#[inline(always)]
pub(super) fn record_mmap(file_backed: bool) {
    add(&PERF.mmap_calls, 1);
    if file_backed {
        add(&PERF.mmap_file_backed_calls, 1);
    }
}

#[inline(always)]
pub(super) fn record_mmap_page_fault() {
    add(&PERF.mmap_page_faults, 1);
}

#[inline(always)]
pub(super) fn record_epoll_wait() {
    add(&PERF.epoll_waits, 1);
}

#[inline(always)]
pub(super) fn record_epoll_ready_scan(registrations: usize) {
    add(&PERF.epoll_ready_scans, registrations as u64);
}

#[inline(always)]
pub(super) fn record_futex_call(wait: bool) {
    add(&PERF.futex_calls, 1);
    if wait {
        add(&PERF.futex_waits, 1);
    }
}

#[inline(always)]
pub(super) fn record_poll_fd_scan(nfds: usize) {
    add(&PERF.poll_fd_scans, nfds as u64);
}

#[inline(always)]
pub(super) fn record_poll_wait() {
    add(&PERF.poll_waits, 1);
}

#[cfg(feature = "auto-run-tests")]
pub(crate) fn perf_snapshot() -> PerfSnapshot {
    PerfSnapshot {
        syscalls: PERF.syscalls.load(Ordering::Relaxed),
        fs_syscalls: PERF.fs_syscalls.load(Ordering::Relaxed),
        exec_syscalls: PERF.exec_syscalls.load(Ordering::Relaxed),
        mmap_syscalls: PERF.mmap_syscalls.load(Ordering::Relaxed),
        iovec_syscalls: PERF.iovec_syscalls.load(Ordering::Relaxed),
        epoll_syscalls: PERF.epoll_syscalls.load(Ordering::Relaxed),
        futex_syscalls: PERF.futex_syscalls.load(Ordering::Relaxed),
        user_copy_faults: PERF.user_copy_faults.load(Ordering::Relaxed),
        user_copy_read_bytes: PERF.user_copy_read_bytes.load(Ordering::Relaxed),
        user_copy_write_bytes: PERF.user_copy_write_bytes.load(Ordering::Relaxed),
        iovec_tables: PERF.iovec_tables.load(Ordering::Relaxed),
        iovec_entries: PERF.iovec_entries.load(Ordering::Relaxed),
        exec_images: PERF.exec_images.load(Ordering::Relaxed),
        exec_image_bytes: PERF.exec_image_bytes.load(Ordering::Relaxed),
        mmap_calls: PERF.mmap_calls.load(Ordering::Relaxed),
        mmap_file_backed_calls: PERF.mmap_file_backed_calls.load(Ordering::Relaxed),
        mmap_page_faults: PERF.mmap_page_faults.load(Ordering::Relaxed),
        epoll_waits: PERF.epoll_waits.load(Ordering::Relaxed),
        epoll_ready_scans: PERF.epoll_ready_scans.load(Ordering::Relaxed),
        futex_calls: PERF.futex_calls.load(Ordering::Relaxed),
        futex_waits: PERF.futex_waits.load(Ordering::Relaxed),
        poll_fd_scans: PERF.poll_fd_scans.load(Ordering::Relaxed),
        poll_waits: PERF.poll_waits.load(Ordering::Relaxed),
    }
}
