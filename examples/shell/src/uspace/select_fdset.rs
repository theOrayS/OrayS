use core::mem::size_of;

use axerrno::LinuxError;
use linux_raw_sys::general;

use super::linux_abi::{BITS_PER_USIZE, FD_SET_WORDS, FD_SETSIZE, neg_errno};
use super::signal_abi::current_unblocked_signal_pending;
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

pub(super) fn read_pselect_deadline(
    process: &UserProcess,
    timeout: usize,
) -> Result<Option<core::time::Duration>, LinuxError> {
    if timeout == 0 {
        return Ok(None);
    }
    let ts = read_user_value::<general::timespec>(process, timeout)?;
    if ts.tv_sec < 0 || !(0..1_000_000_000).contains(&ts.tv_nsec) {
        return Err(LinuxError::EINVAL);
    }
    Ok(Some(
        axhal::time::wall_time() + core::time::Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32),
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

pub(super) fn sys_pselect6(
    process: &UserProcess,
    nfds: i32,
    readfds: usize,
    writefds: usize,
    exceptfds: usize,
    timeout: usize,
    _sigmask: usize,
) -> isize {
    if nfds < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
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
    let deadline = match read_pselect_deadline(process, timeout) {
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
    loop {
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
            axtask::yield_now();
            return ready as isize;
        }
        if deadline.is_some_and(|ddl| axhal::time::wall_time() >= ddl) {
            axtask::yield_now();
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
        axtask::yield_now();
    }
}

#[cfg(not(any(
    target_arch = "riscv64",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
fn poll_deadline_from_timeout_ms(timeout_ms: i32) -> Option<core::time::Duration> {
    (timeout_ms >= 0)
        .then(|| axhal::time::wall_time() + core::time::Duration::from_millis(timeout_ms as u64))
}

fn poll_one_fd(table: &FdTable, pollfd: &mut UserPollFd) -> bool {
    pollfd.revents = 0;
    if pollfd.fd < 0 {
        return false;
    }
    if table.entry(pollfd.fd).is_err() {
        pollfd.revents = POLLNVAL;
        return true;
    }
    if pollfd.events & (POLLIN | POLLPRI) != 0 && table.poll(pollfd.fd, SelectMode::Read) {
        pollfd.revents |= pollfd.events & (POLLIN | POLLPRI);
    }
    if pollfd.events & POLLOUT != 0 && table.poll(pollfd.fd, SelectMode::Write) {
        pollfd.revents |= POLLOUT;
    }
    if table.poll(pollfd.fd, SelectMode::Except) {
        pollfd.revents |= POLLERR;
    }
    pollfd.revents != 0
}

fn poll_fds_once(process: &UserProcess, fds: usize, nfds: usize) -> Result<usize, LinuxError> {
    let mut ready = 0usize;
    let table = process.fds.lock();
    for idx in 0..nfds {
        let ptr = fds + idx * size_of::<UserPollFd>();
        let mut pollfd = read_user_value::<UserPollFd>(process, ptr)?;
        if poll_one_fd(&table, &mut pollfd) {
            ready += 1;
        }
        let ret = write_user_value(process, ptr, &pollfd);
        if ret != 0 {
            return Err(LinuxError::EFAULT);
        }
    }
    Ok(ready)
}

pub(super) fn sys_ppoll(
    process: &UserProcess,
    fds: usize,
    nfds: usize,
    timeout: usize,
    _sigmask: usize,
    _sigsetsize: usize,
) -> isize {
    if nfds > FD_SETSIZE {
        return neg_errno(LinuxError::EINVAL);
    }
    let deadline = match read_pselect_deadline(process, timeout) {
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
    loop {
        if process.eval_watchdog_expired() {
            return neg_errno(LinuxError::EINTR);
        }
        if current_unblocked_signal_pending() {
            return neg_errno(LinuxError::EINTR);
        }
        match poll_fds_once(process, fds, nfds) {
            Ok(ready) if ready > 0 => {
                axtask::yield_now();
                return ready as isize;
            }
            Ok(_) => {}
            Err(err) => return neg_errno(err),
        }
        if deadline.is_some_and(|ddl| axhal::time::wall_time() >= ddl) {
            axtask::yield_now();
            return 0;
        }
        axtask::yield_now();
    }
}
