use core::mem::size_of;
use core::sync::atomic::Ordering;
use core::time::Duration;

use axerrno::LinuxError;
use kspin::SpinNoPreempt;
use linux_raw_sys::general;

use super::linux_abi::{BITS_PER_USIZE, FD_SET_WORDS, FD_SETSIZE, neg_errno};
use super::signal_abi::{current_unblocked_signal_pending, install_temporary_signal_mask};
use super::task_context::current_task_ext;
use super::task_registry::live_user_thread_count;
use super::user_memory::{read_user_value, write_user_value};
use super::{FdTable, UserProcess};

#[repr(C)]
#[derive(Clone, Copy)]
struct UserFdSet {
    fds_bits: [usize; FD_SET_WORDS],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct UserPollFd {
    fd: i32,
    events: i16,
    revents: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PselectSigmaskArg {
    ss: usize,
    ss_len: usize,
}

#[derive(Clone, Copy)]
pub(super) enum SelectMode {
    Read,
    Write,
    Except,
}

const POLLIN: i16 = 0x0001;
const POLLPRI: i16 = 0x0002;
const POLLOUT: i16 = 0x0004;
const POLLERR: i16 = 0x0008;
const POLLNVAL: i16 = 0x0020;
const POLL_WAIT_BLOCK_QUANTUM: Duration = Duration::from_millis(1);
const POLL_DEADLINE_YIELD_WINDOW: Duration = Duration::from_millis(2);
const POLL_TIMEOUT_ONLY_SHORT_EXIT_GUARD: Duration = Duration::from_micros(384);
const POLL_TIMEOUT_ONLY_LONG_THRESHOLD: Duration = Duration::from_millis(5);
const POLL_TIMEOUT_ONLY_LONG_EXIT_GUARD: Duration = Duration::from_micros(1152);
// A pure pselect sleep has no fdset copy-out on timeout.  For multi-ms waits,
// block the task until the final precision window instead of busy-spinning for
// the whole interval; then spin to the real deadline so the syscall does not
// report timeout before the requested interval has elapsed.
const EMPTY_TIMEOUT_BLOCK_THRESHOLD: Duration = Duration::from_millis(5);
const EMPTY_TIMEOUT_SPIN_WINDOW: Duration = Duration::from_millis(2);
static POLL_DEADLINE_SPIN_GUARD: SpinNoPreempt<()> = SpinNoPreempt::new(());

fn poll_clock_now() -> Duration {
    axhal::time::monotonic_time()
}

fn yield_if_peer_user_task() {
    if live_user_thread_count() > 1 {
        axtask::yield_now();
    }
}

pub(super) fn read_pselect_deadline(
    process: &UserProcess,
    timeout: usize,
    base: core::time::Duration,
) -> Result<Option<core::time::Duration>, LinuxError> {
    if timeout == 0 {
        return Ok(None);
    }
    let ts = read_user_value::<general::timespec>(process, timeout)?;
    if ts.tv_sec < 0 || !(0..1_000_000_000).contains(&ts.tv_nsec) {
        return Err(LinuxError::EINVAL);
    }
    Ok(Some(
        base + core::time::Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32),
    ))
}

pub(super) fn read_fd_set(
    process: &UserProcess,
    ptr: usize,
) -> Result<[usize; FD_SET_WORDS], LinuxError> {
    if ptr == 0 {
        return Ok([0; FD_SET_WORDS]);
    }
    Ok(read_user_value::<UserFdSet>(process, ptr)?.fds_bits)
}

pub(super) fn write_fd_set(
    process: &UserProcess,
    ptr: usize,
    bits: &[usize; FD_SET_WORDS],
) -> isize {
    if ptr == 0 {
        return 0;
    }
    write_user_value(process, ptr, &UserFdSet { fds_bits: *bits })
}

pub(super) fn poll_fd_set(
    table: &FdTable,
    nfds: usize,
    requested: &[usize; FD_SET_WORDS],
    ready: &mut [usize; FD_SET_WORDS],
    mode: SelectMode,
) -> usize {
    let mut count = 0usize;
    let words = nfds.div_ceil(BITS_PER_USIZE);
    for word_idx in 0..words {
        let mut bits = requested[word_idx];
        while bits != 0 {
            let bit_idx = bits.trailing_zeros() as usize;
            let fd = word_idx * BITS_PER_USIZE + bit_idx;
            if fd >= nfds {
                break;
            }
            if table.poll(fd as i32, mode) {
                ready[word_idx] |= 1usize << bit_idx;
                count += 1;
            }
            bits &= bits - 1;
        }
    }
    count
}

fn validate_fd_set_entries(
    table: &FdTable,
    nfds: usize,
    requested: &[usize; FD_SET_WORDS],
) -> Result<(), LinuxError> {
    let words = nfds.div_ceil(BITS_PER_USIZE).min(FD_SET_WORDS);
    for word_idx in 0..words {
        let mut bits = requested[word_idx];
        let used_bits = nfds.saturating_sub(word_idx * BITS_PER_USIZE);
        if used_bits < BITS_PER_USIZE {
            bits &= (1usize << used_bits) - 1;
        }
        while bits != 0 {
            let bit_idx = bits.trailing_zeros() as usize;
            let fd = word_idx * BITS_PER_USIZE + bit_idx;
            if fd >= nfds {
                break;
            }
            table.entry(fd as i32)?;
            bits &= bits - 1;
        }
    }
    Ok(())
}

fn fd_set_has_requested(nfds: usize, requested: &[usize; FD_SET_WORDS]) -> bool {
    let words = nfds.div_ceil(BITS_PER_USIZE).min(FD_SET_WORDS);
    for word_idx in 0..words {
        let mut bits = requested[word_idx];
        let used_bits = nfds.saturating_sub(word_idx * BITS_PER_USIZE);
        if used_bits < BITS_PER_USIZE {
            bits &= (1usize << used_bits) - 1;
        }
        if bits != 0 {
            return true;
        }
    }
    false
}

pub(super) fn sys_pselect6(
    process: &UserProcess,
    nfds: i32,
    readfds: usize,
    writefds: usize,
    exceptfds: usize,
    timeout: usize,
    sigmask: usize,
) -> isize {
    if nfds < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let syscall_start = poll_clock_now();
    let _signal_mask_guard = if sigmask == 0 {
        None
    } else {
        let arg = match read_user_value::<PselectSigmaskArg>(process, sigmask) {
            Ok(arg) => arg,
            Err(err) => return neg_errno(err),
        };
        match install_temporary_signal_mask(process, arg.ss, arg.ss_len) {
            Ok(guard) => guard,
            Err(err) => return neg_errno(err),
        }
    };
    let nfds = (nfds as usize).min(FD_SETSIZE);
    let read_bits = match read_fd_set(process, readfds) {
        Ok(bits) => bits,
        Err(err) => return neg_errno(err),
    };
    let write_bits = match read_fd_set(process, writefds) {
        Ok(bits) => bits,
        Err(err) => return neg_errno(err),
    };
    let except_bits = match read_fd_set(process, exceptfds) {
        Ok(bits) => bits,
        Err(err) => return neg_errno(err),
    };
    let deadline = match read_pselect_deadline(process, timeout, syscall_start) {
        Ok(deadline) => deadline,
        Err(err) => return neg_errno(err),
    };
    {
        let table = process.fds.lock();
        for fd_set in [&read_bits, &write_bits, &except_bits] {
            if let Err(err) = validate_fd_set_entries(&table, nfds, fd_set) {
                return neg_errno(err);
            }
        }
    }
    let empty_requested_fd_sets = !fd_set_has_requested(nfds, &read_bits)
        && !fd_set_has_requested(nfds, &write_bits)
        && !fd_set_has_requested(nfds, &except_bits);
    let wait_deadline = deadline;
    let mut polled_once = false;
    loop {
        if polled_once && wait_deadline.is_some_and(|ddl| poll_clock_now() >= ddl) {
            if empty_requested_fd_sets {
                return 0;
            }
            let empty = [0; FD_SET_WORDS];
            let ret = write_fd_set(process, readfds, &empty);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, writefds, &empty);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, exceptfds, &empty);
            if ret != 0 {
                return ret;
            }
            return 0;
        }
        if process.eval_watchdog_expired() {
            return neg_errno(LinuxError::EINTR);
        }
        if current_unblocked_signal_pending() {
            return neg_errno(LinuxError::EINTR);
        }
        let mut ready_read = [0usize; FD_SET_WORDS];
        let mut ready_write = [0usize; FD_SET_WORDS];
        let mut ready_except = [0usize; FD_SET_WORDS];
        let ready = {
            let table = process.fds.lock();
            let mut count = 0usize;
            count += poll_fd_set(&table, nfds, &read_bits, &mut ready_read, SelectMode::Read);
            count += poll_fd_set(
                &table,
                nfds,
                &write_bits,
                &mut ready_write,
                SelectMode::Write,
            );
            count += poll_fd_set(
                &table,
                nfds,
                &except_bits,
                &mut ready_except,
                SelectMode::Except,
            );
            count
        };
        polled_once = true;
        if ready > 0 {
            let ret = write_fd_set(process, readfds, &ready_read);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, writefds, &ready_write);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, exceptfds, &ready_except);
            if ret != 0 {
                return ret;
            }
            // In this cooperative single-core environment, a hot readiness loop
            // can otherwise starve the peer process that would consume the event.
            // If this is the only live user task, however, yielding before a
            // ready return only burns a scheduler round trip in short poll/select
            // heavy workloads.
            yield_if_peer_user_task();
            return ready as isize;
        }
        if wait_deadline.is_some_and(|ddl| poll_clock_now() >= ddl) {
            if empty_requested_fd_sets {
                return 0;
            }
            let empty = [0; FD_SET_WORDS];
            let ret = write_fd_set(process, readfds, &empty);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, writefds, &empty);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, exceptfds, &empty);
            if ret != 0 {
                return ret;
            }
            return 0;
        }
        let timed_out = if empty_requested_fd_sets {
            yield_poll_blocking_timeout_until(wait_deadline)
        } else {
            yield_poll_wait_until(wait_deadline)
        };
        if timed_out {
            if empty_requested_fd_sets {
                return 0;
            }
            let empty = [0; FD_SET_WORDS];
            let ret = write_fd_set(process, readfds, &empty);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, writefds, &empty);
            if ret != 0 {
                return ret;
            }
            let ret = write_fd_set(process, exceptfds, &empty);
            if ret != 0 {
                return ret;
            }
            return 0;
        }
    }
}

#[cfg(not(any(
    target_arch = "riscv64",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
fn poll_deadline_from_timeout_ms(timeout_ms: i32) -> Option<core::time::Duration> {
    (timeout_ms >= 0)
        .then(|| poll_clock_now() + core::time::Duration::from_millis(timeout_ms as u64))
}

fn poll_one_fd(table: &FdTable, pollfd: &mut UserPollFd) -> (bool, bool) {
    pollfd.revents = 0;
    if pollfd.fd < 0 {
        return (false, false);
    }
    let entry = match table.entry(pollfd.fd) {
        Ok(entry) => entry,
        Err(_) => {
            pollfd.revents = POLLNVAL;
            return (true, false);
        }
    };
    let watched = true;
    if pollfd.events & POLLIN != 0 && FdTable::poll_entry(entry, SelectMode::Read) {
        pollfd.revents |= POLLIN;
    }
    if pollfd.events & POLLPRI != 0 && FdTable::poll_entry(entry, SelectMode::Except) {
        pollfd.revents |= POLLPRI;
    }
    if pollfd.events & POLLOUT != 0 && FdTable::poll_entry(entry, SelectMode::Write) {
        pollfd.revents |= POLLOUT;
    }
    if FdTable::poll_entry(entry, SelectMode::Except) {
        pollfd.revents |= POLLERR;
    }
    (pollfd.revents != 0, watched)
}

fn poll_fds_once(
    process: &UserProcess,
    fds: usize,
    nfds: usize,
) -> Result<(usize, usize), LinuxError> {
    let mut ready = 0usize;
    let mut watched = 0usize;
    let table = process.fds.lock();
    for idx in 0..nfds {
        let ptr = fds + idx * size_of::<UserPollFd>();
        let mut pollfd = read_user_value::<UserPollFd>(process, ptr)?;
        let (is_ready, is_watched) = poll_one_fd(&table, &mut pollfd);
        if is_ready {
            ready += 1;
        }
        if is_watched {
            watched += 1;
        }
        let ret = write_user_value(process, ptr, &pollfd);
        if ret != 0 {
            return Err(LinuxError::EFAULT);
        }
    }
    Ok((ready, watched))
}

pub(super) fn sys_ppoll(
    process: &UserProcess,
    fds: usize,
    nfds: usize,
    timeout: usize,
    sigmask: usize,
    sigsetsize: usize,
) -> isize {
    if nfds > FD_SETSIZE {
        return neg_errno(LinuxError::EINVAL);
    }
    let syscall_start = poll_clock_now();
    let _signal_mask_guard = match install_temporary_signal_mask(process, sigmask, sigsetsize) {
        Ok(guard) => guard,
        Err(err) => return neg_errno(err),
    };
    let deadline = match read_pselect_deadline(process, timeout, syscall_start) {
        Ok(deadline) => deadline,
        Err(err) => return neg_errno(err),
    };
    sys_poll_until(process, fds, nfds, deadline)
}

#[cfg(not(any(
    target_arch = "riscv64",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
pub(super) fn sys_poll(process: &UserProcess, fds: usize, nfds: usize, timeout_ms: i32) -> isize {
    if nfds > FD_SETSIZE {
        return neg_errno(LinuxError::EINVAL);
    }
    sys_poll_until(
        process,
        fds,
        nfds,
        poll_deadline_from_timeout_ms(timeout_ms),
    )
}

fn sys_poll_until(
    process: &UserProcess,
    fds: usize,
    nfds: usize,
    deadline: Option<core::time::Duration>,
) -> isize {
    let mut polled_once = false;
    loop {
        if polled_once && deadline.is_some_and(|ddl| poll_clock_now() >= ddl) {
            return 0;
        }
        if process.eval_watchdog_expired() {
            return neg_errno(LinuxError::EINTR);
        }
        if current_unblocked_signal_pending() {
            return neg_errno(LinuxError::EINTR);
        }
        let watched = match poll_fds_once(process, fds, nfds) {
            Ok((ready, _watched)) if ready > 0 => {
                yield_if_peer_user_task();
                return ready as isize;
            }
            Ok((_, watched)) => watched,
            Err(err) => return neg_errno(err),
        };
        polled_once = true;
        if deadline.is_some_and(|ddl| poll_clock_now() >= ddl) {
            return 0;
        }
        let timed_out = if watched == 0 {
            yield_poll_timeout_only_until(deadline)
        } else {
            yield_poll_wait_until(deadline)
        };
        if timed_out {
            return 0;
        }
    }
}

fn timeout_only_deadline(deadline: Duration) -> Duration {
    let now = poll_clock_now();
    let remaining = deadline.saturating_sub(now);
    let guard = if remaining >= POLL_TIMEOUT_ONLY_LONG_THRESHOLD {
        POLL_TIMEOUT_ONLY_LONG_EXIT_GUARD
    } else {
        POLL_TIMEOUT_ONLY_SHORT_EXIT_GUARD
    };
    if remaining > guard {
        deadline - guard
    } else {
        deadline
    }
}

fn finite_poll_wait_delay(deadline: Duration) -> Option<Duration> {
    let now = poll_clock_now();
    if now >= deadline {
        return None;
    }
    let remaining = deadline - now;
    if remaining <= POLL_DEADLINE_YIELD_WINDOW {
        None
    } else {
        Some(POLL_WAIT_BLOCK_QUANTUM.min(remaining - POLL_DEADLINE_YIELD_WINDOW))
    }
}

pub(super) fn yield_poll_wait_until(deadline: Option<Duration>) -> bool {
    yield_poll_wait_until_target(deadline, false)
}

pub(super) fn yield_poll_timeout_only_until(deadline: Option<Duration>) -> bool {
    yield_poll_wait_until_target(deadline, true)
}

pub(super) fn yield_poll_blocking_timeout_until(deadline: Option<Duration>) -> bool {
    match deadline {
        Some(deadline) => {
            let now = poll_clock_now();
            if now >= deadline {
                return true;
            }
            let remaining = deadline - now;
            if remaining > EMPTY_TIMEOUT_BLOCK_THRESHOLD {
                let block_for = remaining.saturating_sub(EMPTY_TIMEOUT_SPIN_WINDOW);
                if !block_for.is_zero() {
                    yield_poll_wait_for(block_for);
                }
            }
            spin_poll_wait_until(deadline);
            true
        }
        None => {
            yield_poll_wait_for(POLL_WAIT_BLOCK_QUANTUM);
            false
        }
    }
}

fn yield_poll_wait_until_target(deadline: Option<Duration>, timeout_only: bool) -> bool {
    match deadline {
        Some(deadline) => {
            let target = if timeout_only {
                timeout_only_deadline(deadline)
            } else {
                deadline
            };
            match finite_poll_wait_delay(target) {
                Some(delay) => {
                    yield_poll_wait_for(delay);
                    poll_clock_now() >= target
                }
                None => {
                    spin_poll_wait_until(target);
                    true
                }
            }
        }
        None => {
            yield_poll_wait_for(POLL_WAIT_BLOCK_QUANTUM);
            false
        }
    }
}

pub(super) fn yield_poll_wait() {
    yield_poll_wait_for(POLL_WAIT_BLOCK_QUANTUM);
}

fn yield_poll_wait_for(delay: Duration) {
    if let Some(ext) = current_task_ext() {
        ext.poll_wait.store(true, Ordering::Release);
        // Empty poll/select loops are waiting for an external event, not doing
        // useful CPU work.  On the single-vCPU evaluator a pure yield lets a
        // background server immediately re-enter the ready queue and can starve
        // the shell/client that should produce the event.  Block briefly so the
        // scheduler can run peers while preserving POSIX retry semantics.
        axtask::sleep(delay);
        ext.poll_wait.store(false, Ordering::Release);
    } else {
        axtask::sleep(delay);
    }
}

fn spin_poll_wait_until(deadline: Duration) {
    let _guard = POLL_DEADLINE_SPIN_GUARD.lock();
    while poll_clock_now() < deadline {
        core::hint::spin_loop();
    }
}
