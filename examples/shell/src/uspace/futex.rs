use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axsync::Mutex;
use axtask::{AxTaskRef, WaitQueue};
use lazyinit::LazyInit;
use linux_raw_sys::general;
use memory_addr::VirtAddr;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::UserProcess;
use super::linux_abi::{USER_MMAP_BASE, neg_errno};
use super::perf_counters;
use super::signal_abi::current_unblocked_signal_pending;
use super::task_context::{current_task_ext, current_tid, task_ext, user_pc};
use super::time_abi::{clock_now_duration, timespec_to_duration};
use super::user_memory::{fault_in_user_read, read_user_value};

pub(super) struct FutexState {
    pub(super) seq: AtomicU32,
    pub(super) queue: WaitQueue,
}

// Keep short timed futex waits out of the task timer queue.  The timer backend
// intentionally protects near-expired one-shot deadlines with a one-millisecond
// minimum programming window; if a 1ms userspace futex timeout first blocks for
// the tiny `remaining - spin_window` tail, the wakeup can overshoot the Linux
// timer-test tolerance.  Spinning only for sub-few-millisecond futex timeouts
// preserves the POSIX rule that timeouts are not reported early while avoiding
// scheduler/timer rounding on precision probes.
const FUTEX_TIMEOUT_SPIN_WINDOW: Duration = Duration::from_millis(2);

fn table() -> &'static Mutex<BTreeMap<usize, Arc<FutexState>>> {
    static FUTEXES: LazyInit<Mutex<BTreeMap<usize, Arc<FutexState>>>> = LazyInit::new();
    let _ = FUTEXES.call_once(|| Mutex::new(BTreeMap::new()));
    &FUTEXES
}

fn mapped_futex_key(process: &UserProcess, uaddr: usize) -> Result<usize, LinuxError> {
    let query = process.aspace.lock().query_address(VirtAddr::from(uaddr));
    if !query.pte_mapped {
        return Err(LinuxError::EFAULT);
    }
    Ok(query.paddr | (uaddr & 0xfff))
}

fn futex_key(process: &UserProcess, uaddr: usize) -> Result<usize, LinuxError> {
    // Futex wait queues are keyed by the backing frame, not just the user
    // virtual address: independent processes may reuse the same mmap addresses,
    // while forked MAP_SHARED checkpoint pages retain the same
    // physical frame and must still rendezvous across parent/child processes.
    fault_in_user_read(process, uaddr, size_of::<u32>())?;
    mapped_futex_key(process, uaddr)
}

fn mapped_futex_state(process: &UserProcess, uaddr: usize) -> Option<(usize, Arc<FutexState>)> {
    let Ok(key) = mapped_futex_key(process, uaddr) else {
        return None;
    };
    table().lock().get(&key).cloned().map(|state| (key, state))
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

fn clear_current_futex_wait(process: &UserProcess, original_uaddr: usize) {
    let Some(ext) = current_task_ext() else {
        return;
    };
    let queued_uaddr = ext.futex_wait.swap(0, Ordering::AcqRel);
    ext.futex_bitset.store(0, Ordering::Release);
    if queued_uaddr == 0 || queued_uaddr == original_uaddr {
        return;
    }
    let current = axtask::current();
    let Some((key, state)) = mapped_futex_state(process, queued_uaddr) else {
        return;
    };
    state.queue.remove_task(current.as_task_ref());
    drop(state);
    prune_empty_key(key);
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
    wake_addr_bitset_checked(process, uaddr, count, u32::MAX)
}

fn futex_waiter_matches(task: &AxTaskRef, bitset: u32) -> bool {
    task_ext(task).is_some_and(|ext| ext.futex_bitset.load(Ordering::Acquire) & bitset != 0)
}

fn clear_futex_waiter(task: &AxTaskRef) {
    if let Some(ext) = task_ext(task) {
        ext.futex_wait.store(0, Ordering::Release);
        ext.futex_bitset.store(0, Ordering::Release);
    }
}

fn wake_addr_bitset_checked(
    process: &UserProcess,
    uaddr: usize,
    count: usize,
    bitset: u32,
) -> Result<usize, LinuxError> {
    let key = futex_key(process, uaddr)?;
    let Some(state) = table().lock().get(&key).cloned() else {
        return Ok(0);
    };
    state.seq.fetch_add(1, Ordering::Release);
    let woken = state.queue.notify_many_where(
        count,
        true,
        |task| futex_waiter_matches(task, bitset),
        |task| {
            clear_futex_waiter(task);
        },
    );
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
) -> Result<(usize, usize), LinuxError> {
    if uaddr2 == 0 || uaddr2 % size_of::<u32>() != 0 {
        return Err(LinuxError::EINVAL);
    }
    if let Some(expected) = cmp {
        let current = read_user_value::<u32>(process, uaddr)?;
        if current != expected {
            return Err(LinuxError::EAGAIN);
        }
    }
    let source_key = futex_key(process, uaddr)?;
    let target_key = futex_key(process, uaddr2)?;
    let Some(source) = table().lock().get(&source_key).cloned() else {
        return Ok((0, 0));
    };
    let target = if source_key == target_key {
        source.clone()
    } else {
        state(target_key)
    };

    source.seq.fetch_add(1, Ordering::Release);
    let (woken, requeued) = source.queue.notify_and_requeue_where(
        wake_count,
        requeue_count,
        &target.queue,
        true,
        |task| futex_waiter_matches(task, u32::MAX),
        |task| {
            clear_futex_waiter(task);
        },
        |task| {
            if let Some(ext) = task_ext(task) {
                ext.futex_wait.store(uaddr2, Ordering::Release);
            }
        },
    );
    drop(source);
    drop(target);
    prune_empty_key(source_key);
    prune_empty_key(target_key);
    Ok((woken, requeued))
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

fn wait_timeout_until_precise<F>(state: &FutexState, dur: Duration, condition: F) -> bool
where
    F: Fn() -> bool,
{
    if condition() {
        return false;
    }
    if dur.is_zero() {
        return true;
    }
    let Some(deadline) = axhal::time::monotonic_time().checked_add(dur) else {
        return state.queue.wait_timeout_until(dur, condition);
    };

    loop {
        if condition() {
            return false;
        }
        let now = axhal::time::monotonic_time();
        if now >= deadline {
            return true;
        }
        let remaining = deadline.saturating_sub(now);
        if remaining <= FUTEX_TIMEOUT_SPIN_WINDOW {
            while axhal::time::monotonic_time() < deadline {
                if condition() {
                    return false;
                }
                core::hint::spin_loop();
            }
            return !condition();
        }

        let block_for = remaining.saturating_sub(FUTEX_TIMEOUT_SPIN_WINDOW);
        if !state.queue.wait_timeout_until(block_for, || condition()) {
            return false;
        }
    }
}

fn wait_addr(
    process: &UserProcess,
    uaddr: usize,
    val: usize,
    timeout: usize,
    absolute_timeout: bool,
    realtime_timeout: bool,
    bitset: u32,
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
    if let Some(ext) = current_task_ext() {
        ext.futex_wait.store(uaddr, Ordering::Release);
        ext.futex_bitset.store(bitset, Ordering::Release);
    }
    let wait_cond = || {
        current_task_ext().is_some_and(|ext| ext.futex_wait.load(Ordering::Acquire) != uaddr)
            || read_user_value::<u32>(process, uaddr).map_or(true, |value| value != val as u32)
            || current_unblocked_signal_pending()
            || process.pending_exit_group().is_some()
            || process.eval_watchdog_expired()
    };
    if let Some(dur) = timeout {
        let dur = process
            .eval_watchdog_remaining()
            .map_or(dur, |remaining| remaining.min(dur));
        let timed_out = wait_timeout_until_precise(&state, dur, wait_cond);
        drop(state);
        prune_empty_key(key);
        if timed_out {
            clear_current_futex_wait(process, uaddr);
            if current_unblocked_signal_pending()
                || process.eval_watchdog_expired()
                || process.pending_exit_group().is_some()
            {
                return neg_errno(LinuxError::EINTR);
            }
            return neg_errno(LinuxError::ETIMEDOUT);
        }
        clear_current_futex_wait(process, uaddr);
        if current_unblocked_signal_pending() {
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
    clear_current_futex_wait(process, uaddr);
    if current_unblocked_signal_pending()
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
    let Some((key, state)) = mapped_futex_state(process, uaddr) else {
        return 0;
    };
    state.seq.fetch_add(1, Ordering::Release);
    let woken = state.queue.notify_many_where(
        count,
        true,
        |task| futex_waiter_matches(task, u32::MAX),
        |task| {
            clear_futex_waiter(task);
        },
    );
    drop(state);
    prune_empty_key(key);
    woken
}

pub(super) fn futex_waiter_is_queued(process: &UserProcess, uaddr: usize) -> bool {
    mapped_futex_state(process, uaddr)
        .map(|(_, state)| !state.queue.is_empty())
        .unwrap_or(false)
}

pub(super) fn wake_task(process: &UserProcess, uaddr: usize, task: &AxTaskRef) {
    let Some((_, state)) = mapped_futex_state(process, uaddr) else {
        return;
    };
    state.seq.fetch_add(1, Ordering::Release);
    if let Some(ext) = task_ext(task) {
        ext.futex_wait.store(0, Ordering::Release);
        ext.futex_bitset.store(0, Ordering::Release);
    }
    let _ = state.queue.notify_task(true, task);
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
    perf_counters::record_futex_call(matches!(
        cmd,
        general::FUTEX_WAIT | general::FUTEX_WAIT_BITSET
    ));
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
        general::FUTEX_WAIT => wait_addr(process, uaddr, val, timeout, false, false, u32::MAX),
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
                _val3 as u32,
            )
        }
        general::FUTEX_WAKE => match wake_addr_checked(process, uaddr, val) {
            Ok(woken) => woken as isize,
            Err(err) => neg_errno(err),
        },
        general::FUTEX_REQUEUE => {
            match wake_requeue_addr_checked(process, uaddr, val, timeout, _uaddr2, None) {
                Ok((woken, _requeued)) => woken as isize,
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
            Ok((woken, requeued)) => woken.saturating_add(requeued) as isize,
            Err(err) => neg_errno(err),
        },
        general::FUTEX_WAKE_BITSET => {
            if _val3 == 0 {
                return neg_errno(LinuxError::EINVAL);
            }
            match wake_addr_bitset_checked(process, uaddr, val, _val3 as u32) {
                Ok(woken) => woken as isize,
                Err(err) => neg_errno(err),
            }
        }
        _ => neg_errno(LinuxError::ENOSYS),
    }
}
