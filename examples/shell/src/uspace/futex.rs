use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering};

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axsync::Mutex;
use axtask::{AxTaskRef, WaitQueue};
use lazyinit::LazyInit;
use linux_raw_sys::general;
use memory_addr::VirtAddr;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::linux_abi::{neg_errno, USER_MMAP_BASE};
use super::signal_abi::current_sigcancel_pending;
use super::task_context::{current_task_ext, current_tid, user_pc};
use super::time_abi::{clock_now_duration, timespec_to_duration};
use super::user_memory::read_user_value;
use super::UserProcess;

pub(super) struct FutexState {
    pub(super) seq: AtomicU32,
    pub(super) queue: WaitQueue,
}

fn table() -> &'static Mutex<BTreeMap<usize, Arc<FutexState>>> {
    static FUTEXES: LazyInit<Mutex<BTreeMap<usize, Arc<FutexState>>>> = LazyInit::new();
    let _ = FUTEXES.call_once(|| Mutex::new(BTreeMap::new()));
    &FUTEXES
}

fn futex_key(process: &UserProcess, uaddr: usize) -> Result<usize, LinuxError> {
    // Futex wait queues are keyed by the backing frame, not just the user
    // virtual address: independent LTP processes commonly reuse the same mmap
    // addresses, while forked MAP_SHARED checkpoint pages retain the same
    // physical frame and must still rendezvous across parent/child processes.
    let query = process.aspace.lock().query_address(VirtAddr::from(uaddr));
    if !query.pte_mapped {
        return Err(LinuxError::EFAULT);
    }
    Ok(query.paddr | (uaddr & 0xfff))
}

fn state(key: usize) -> Arc<FutexState> {
    let mut table = table().lock();
    table
        .entry(key)
        .or_insert_with(|| {
            Arc::new(FutexState {
                seq: AtomicU32::new(0),
                queue: WaitQueue::new(),
            })
        })
        .clone()
}

fn prune_empty_key(key: usize) {
    let mut table = table().lock();
    let should_remove = table
        .get(&key)
        .is_some_and(|state| state.queue.is_empty() && Arc::strong_count(state) == 1);
    if should_remove {
        table.remove(&key);
    }
}

pub(super) fn prune_empty_futexes() {
    let mut table = table().lock();
    table.retain(|_, state| !state.queue.is_empty() || Arc::strong_count(state) > 1);
}

#[cfg(feature = "auto-run-tests")]
pub fn futex_table_stats() -> (usize, usize) {
    let table = table().lock();
    let queued = table.values().map(|state| state.queue.len()).sum();
    (table.len(), queued)
}

fn wake_addr_checked(
    process: &UserProcess,
    uaddr: usize,
    count: usize,
) -> Result<usize, LinuxError> {
    let key = futex_key(process, uaddr)?;
    let Some(state) = table().lock().get(&key).cloned() else {
        return Ok(0);
    };
    state.seq.fetch_add(1, Ordering::Release);
    let woken = state.queue.notify_many_unique(count, true);
    drop(state);
    prune_empty_key(key);
    Ok(woken)
}

fn wake_requeue_addr_checked(
    process: &UserProcess,
    uaddr: usize,
    wake_count: usize,
    requeue_count: usize,
    uaddr2: usize,
    cmp: Option<u32>,
) -> Result<usize, LinuxError> {
    if uaddr2 == 0 || uaddr2 % size_of::<u32>() != 0 {
        return Err(LinuxError::EINVAL);
    }
    if let Some(expected) = cmp {
        let current = read_user_value::<u32>(process, uaddr)?;
        if current != expected {
            return Err(LinuxError::EAGAIN);
        }
    }
    // Validate the destination futex even though this implementation wakes the
    // source waiters directly. For non-PI condition-variable requeue users this
    // is a correctness-preserving fallback: awakened waiters return to libc and
    // contend on the mutex futex in userspace instead of being moved by the
    // kernel as an optimization.
    let _ = futex_key(process, uaddr2)?;
    wake_addr_checked(process, uaddr, wake_count.saturating_add(requeue_count))
}

fn read_futex_timeout(
    process: &UserProcess,
    timeout: usize,
    absolute: bool,
    realtime: bool,
) -> Result<Option<core::time::Duration>, LinuxError> {
    if timeout == 0 {
        return Ok(None);
    }

    let ts = read_user_value::<general::timespec>(process, timeout)?;
    let mut dur = timespec_to_duration(ts)?;
    if absolute {
        let clockid = if realtime {
            general::CLOCK_REALTIME
        } else {
            general::CLOCK_MONOTONIC
        };
        dur = dur
            .checked_sub(clock_now_duration(clockid)?)
            .unwrap_or_else(|| core::time::Duration::from_secs(0));
    }
    Ok(Some(dur))
}

fn wait_addr(
    process: &UserProcess,
    uaddr: usize,
    val: usize,
    timeout: usize,
    absolute_timeout: bool,
    realtime_timeout: bool,
) -> isize {
    let current = match read_user_value::<u32>(process, uaddr) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
    if current != val as u32 {
        return neg_errno(LinuxError::EAGAIN);
    }
    let key = match futex_key(process, uaddr) {
        Ok(key) => key,
        Err(err) => return neg_errno(err),
    };
    let timeout = match read_futex_timeout(process, timeout, absolute_timeout, realtime_timeout) {
        Ok(timeout) => timeout,
        Err(err) => return neg_errno(err),
    };
    let state = state(key);
    let seq = state.seq.load(Ordering::Acquire);
    if let Some(ext) = current_task_ext() {
        ext.futex_wait.store(uaddr, Ordering::Release);
    }
    let wait_cond = || {
        state.seq.load(Ordering::Acquire) != seq
            || read_user_value::<u32>(process, uaddr).map_or(true, |value| value != val as u32)
            || current_sigcancel_pending()
            || process.pending_exit_group().is_some()
            || process.eval_watchdog_expired()
    };
    if let Some(dur) = timeout {
        let dur = process
            .eval_watchdog_remaining()
            .map_or(dur, |remaining| remaining.min(dur));
        let timed_out = state.queue.wait_timeout_until(dur, wait_cond);
        drop(state);
        prune_empty_key(key);
        if timed_out {
            if let Some(ext) = current_task_ext() {
                ext.futex_wait.store(0, Ordering::Release);
            }
            if process.eval_watchdog_expired() || process.pending_exit_group().is_some() {
                return neg_errno(LinuxError::EINTR);
            }
            return neg_errno(LinuxError::ETIMEDOUT);
        }
        if let Some(ext) = current_task_ext() {
            ext.futex_wait.store(0, Ordering::Release);
        }
        if current_sigcancel_pending() {
            return neg_errno(LinuxError::EINTR);
        }
        return 0;
    }
    if let Some(dur) = process.eval_watchdog_remaining() {
        let _ = state.queue.wait_timeout_until(dur, wait_cond);
    } else {
        state.queue.wait_until(wait_cond);
    }
    drop(state);
    prune_empty_key(key);
    if let Some(ext) = current_task_ext() {
        ext.futex_wait.store(0, Ordering::Release);
    }
    if current_sigcancel_pending()
        || process.pending_exit_group().is_some()
        || process.eval_watchdog_expired()
    {
        return neg_errno(LinuxError::EINTR);
    }
    0
}

pub(super) fn wake_addr(process: &UserProcess, uaddr: usize, count: usize) -> usize {
    // Best-effort kernel-internal wake path for teardown/cancellation cleanup:
    // if the user address has already been unmapped, there is no futex queue to
    // wake and no syscall return value to report.
    wake_addr_checked(process, uaddr, count).unwrap_or(0)
}

pub(super) fn futex_waiter_is_queued(process: &UserProcess, uaddr: usize) -> bool {
    let Ok(key) = futex_key(process, uaddr) else {
        return false;
    };
    table()
        .lock()
        .get(&key)
        .map(|state| !state.queue.is_empty())
        .unwrap_or(false)
}

pub(super) fn wake_task(process: &UserProcess, uaddr: usize, task: &AxTaskRef) {
    let Ok(key) = futex_key(process, uaddr) else {
        return;
    };
    if let Some(state) = table().lock().get(&key).cloned() {
        state.seq.fetch_add(1, Ordering::Release);
        let _ = state.queue.notify_task(true, task);
    }
}

pub(super) fn sys_futex(
    process: &UserProcess,
    tf: &TrapFrame,
    uaddr: usize,
    futex_op: usize,
    val: usize,
    timeout: usize,
    _uaddr2: usize,
    _val3: usize,
) -> isize {
    if uaddr == 0 || uaddr % size_of::<u32>() != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let op = futex_op as u32;
    let cmd = op & general::FUTEX_CMD_MASK as u32;
    if uaddr < USER_MMAP_BASE || (uaddr >= USER_MMAP_BASE && val <= 8) {
        user_trace!(
            "user-futex: tid={} cmd={cmd:#x} op={op:#x} uaddr={uaddr:#x} val={val:#x} timeout={timeout:#x} sp={:#x} tp={:#x} ra={:#x} pc={:#x}",
            current_tid(),
            tf.regs.sp,
            tf.regs.tp,
            tf.regs.ra,
            user_pc(tf),
        );
    }
    match cmd {
        general::FUTEX_WAIT => wait_addr(process, uaddr, val, timeout, false, false),
        general::FUTEX_WAIT_BITSET => {
            if _val3 == 0 {
                return neg_errno(LinuxError::EINVAL);
            }
            wait_addr(
                process,
                uaddr,
                val,
                timeout,
                true,
                op & general::FUTEX_CLOCK_REALTIME != 0,
            )
        }
        general::FUTEX_WAKE => match wake_addr_checked(process, uaddr, val) {
            Ok(woken) => woken as isize,
            Err(err) => neg_errno(err),
        },
        general::FUTEX_REQUEUE => {
            match wake_requeue_addr_checked(process, uaddr, val, timeout, _uaddr2, None) {
                Ok(woken) => woken as isize,
                Err(err) => neg_errno(err),
            }
        }
        general::FUTEX_CMP_REQUEUE => match wake_requeue_addr_checked(
            process,
            uaddr,
            val,
            timeout,
            _uaddr2,
            Some(_val3 as u32),
        ) {
            Ok(woken) => woken as isize,
            Err(err) => neg_errno(err),
        },
        general::FUTEX_WAKE_BITSET => {
            if _val3 == 0 {
                return neg_errno(LinuxError::EINVAL);
            }
            match wake_addr_checked(process, uaddr, val) {
                Ok(woken) => woken as isize,
                Err(err) => neg_errno(err),
            }
        }
        _ => neg_errno(LinuxError::ENOSYS),
    }
}
