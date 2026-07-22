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
use super::task_context::{
    RelativeFutexRestartKey, UserTaskExt, current_task_ext, current_tid, task_ext, user_pc,
};
use super::time_abi::{clock_now_duration, timespec_to_duration};
use super::user_memory::{fault_in_user_read, read_user_value};

pub(super) struct FutexState {
    pub(super) seq: AtomicU32,
    pub(super) queue: WaitQueue,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum FutexKey {
    // Linux private futexes are scoped to the shared address-space object and
    // virtual address.  Their identity must survive a private page's COW remap.
    Private { aspace: usize, uaddr: usize },
    // Process-shared futexes rendezvous through the backing frame even when
    // different processes map it at different virtual addresses.
    Shared { paddr: usize },
}

// Keep short timed futex waits out of the task timer queue.  The timer backend
// intentionally protects near-expired one-shot deadlines with a one-millisecond
// minimum programming window; if a 1ms userspace futex timeout first blocks for
// the tiny `remaining - spin_window` tail, the wakeup can overshoot the Linux
// timer-test tolerance.  Spinning only for sub-few-millisecond futex timeouts
// preserves the POSIX rule that timeouts are not reported early while avoiding
// scheduler/timer rounding on precision probes.
const FUTEX_TIMEOUT_SPIN_WINDOW: Duration = Duration::from_millis(2);

fn table() -> &'static Mutex<BTreeMap<FutexKey, Arc<FutexState>>> {
    static FUTEXES: LazyInit<Mutex<BTreeMap<FutexKey, Arc<FutexState>>>> = LazyInit::new();
    let _ = FUTEXES.call_once(|| Mutex::new(BTreeMap::new()));
    &FUTEXES
}

fn mapped_futex_key(
    process: &UserProcess,
    uaddr: usize,
    private: bool,
) -> Result<FutexKey, LinuxError> {
    let query = process.aspace.lock().query_address(VirtAddr::from(uaddr));
    if !query.pte_mapped {
        return Err(LinuxError::EFAULT);
    }
    if private {
        Ok(FutexKey::Private {
            aspace: Arc::as_ptr(&process.aspace) as usize,
            uaddr,
        })
    } else {
        Ok(FutexKey::Shared {
            paddr: query.paddr | (uaddr & 0xfff),
        })
    }
}

fn futex_key(process: &UserProcess, uaddr: usize, private: bool) -> Result<FutexKey, LinuxError> {
    fault_in_user_read(process, uaddr, size_of::<u32>())?;
    mapped_futex_key(process, uaddr, private)
}

fn mapped_futex_state(
    process: &UserProcess,
    uaddr: usize,
    private: bool,
) -> Option<(FutexKey, Arc<FutexState>)> {
    let Ok(key) = mapped_futex_key(process, uaddr, private) else {
        return None;
    };
    table().lock().get(&key).cloned().map(|state| (key, state))
}

fn state(key: FutexKey) -> Arc<FutexState> {
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

fn prune_empty_key(key: FutexKey) {
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

fn clear_current_futex_wait(process: &UserProcess, original_uaddr: usize, private: bool) {
    let Some(ext) = current_task_ext() else {
        return;
    };
    let queued_uaddr = ext.futex_wait.swap(0, Ordering::AcqRel);
    ext.futex_bitset.store(0, Ordering::Release);
    if queued_uaddr == 0 || queued_uaddr == original_uaddr {
        return;
    }
    let current = axtask::current();
    let Some((key, state)) = mapped_futex_state(process, queued_uaddr, private) else {
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
    private: bool,
) -> Result<usize, LinuxError> {
    wake_addr_bitset_checked(process, uaddr, count, u32::MAX, private)
}

fn legacy_futex_wake_count(value: u32) -> usize {
    // The original futex syscall exposes u32 `val`, but the legacy wake helper
    // receives it as a signed int. Without FUTEX2's strict-count flag, Linux's
    // historical loop wakes one matching waiter for zero or negative counts.
    let signed = value as i32;
    if signed <= 0 { 1 } else { signed as usize }
}

fn futex_requeue_count(value: u32) -> Result<usize, LinuxError> {
    // Unlike the legacy wake loop, futex_requeue rejects either signed count
    // before looking up or changing a futex queue.
    let signed = value as i32;
    if signed < 0 {
        Err(LinuxError::EINVAL)
    } else {
        Ok(signed as usize)
    }
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
    private: bool,
) -> Result<usize, LinuxError> {
    let key = futex_key(process, uaddr, private)?;
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
    private: bool,
) -> Result<(usize, usize), LinuxError> {
    if uaddr2 % size_of::<u32>() != 0 {
        return Err(LinuxError::EINVAL);
    }
    let source_key = futex_key(process, uaddr, private)?;
    let target_key = futex_key(process, uaddr2, private)?;
    let source = if cmp.is_some() {
        state(source_key)
    } else {
        let Some(source) = table().lock().get(&source_key).cloned() else {
            return Ok((0, 0));
        };
        source
    };
    let target = if source_key == target_key {
        source.clone()
    } else {
        state(target_key)
    };

    let outcome = source.queue.notify_and_requeue_where_checked(
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
        || {
            if let Some(expected) = cmp {
                let current = read_user_value::<u32>(process, uaddr)?;
                if current != expected {
                    return Err(LinuxError::EAGAIN);
                }
            }
            source.seq.fetch_add(1, Ordering::Release);
            Ok(())
        },
    );
    drop(source);
    drop(target);
    prune_empty_key(source_key);
    prune_empty_key(target_key);
    let (woken, requeued) = outcome?;
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

fn relative_restart_key(
    uaddr: usize,
    futex_op: usize,
    val: usize,
    timeout: usize,
) -> RelativeFutexRestartKey {
    RelativeFutexRestartKey {
        uaddr,
        futex_op,
        val,
        timeout,
    }
}

fn read_relative_futex_timeout(
    process: &UserProcess,
    timeout: usize,
    restart_key: RelativeFutexRestartKey,
) -> Result<Option<Duration>, LinuxError> {
    let Some(ext) = current_task_ext() else {
        return read_futex_timeout(process, timeout, false, false);
    };
    let now = axhal::time::monotonic_time();
    // A handler may issue unrelated syscalls while an interrupted wait's signal
    // frame is active. Do not let those syscalls consume or replace the outer
    // wait's restart deadline.
    if ext.signal_frame.load(Ordering::Acquire) == 0 {
        if let Some(remaining) = ext.resume_relative_futex_wait(restart_key, now) {
            return Ok(Some(remaining));
        }
    }

    let requested = read_futex_timeout(process, timeout, false, false)?;
    if ext.signal_frame.load(Ordering::Acquire) == 0 {
        if let Some(requested) = requested {
            ext.start_relative_futex_wait(restart_key, now, requested);
        }
    }
    Ok(requested)
}

fn finish_relative_futex_wait(restart_key: Option<RelativeFutexRestartKey>) {
    let (Some(ext), Some(restart_key)) = (current_task_ext(), restart_key) else {
        return;
    };
    ext.finish_relative_futex_wait(restart_key);
}

pub(super) fn arm_relative_futex_restart(ext: &UserTaskExt, tf: &TrapFrame) -> bool {
    ext.arm_relative_futex_restart(relative_restart_key(
        tf.arg0(),
        tf.arg1(),
        tf.arg2() as u32 as usize,
        tf.arg3(),
    ))
}

pub(super) fn clear_relative_futex_restart(ext: &UserTaskExt) {
    ext.clear_relative_futex_restart();
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
    val: u32,
    timeout: Option<Duration>,
    bitset: u32,
    private: bool,
    restart_key: Option<RelativeFutexRestartKey>,
) -> isize {
    let current = match read_user_value::<u32>(process, uaddr) {
        Ok(value) => value,
        Err(err) => {
            finish_relative_futex_wait(restart_key);
            return neg_errno(err);
        }
    };
    if current != val {
        finish_relative_futex_wait(restart_key);
        return neg_errno(LinuxError::EAGAIN);
    }
    let key = match futex_key(process, uaddr, private) {
        Ok(key) => key,
        Err(err) => {
            finish_relative_futex_wait(restart_key);
            return neg_errno(err);
        }
    };
    let state = state(key);
    if let Some(ext) = current_task_ext() {
        ext.futex_wait.store(uaddr, Ordering::Release);
        ext.futex_bitset.store(bitset, Ordering::Release);
    }
    let wait_cond = || {
        current_task_ext().is_some_and(|ext| ext.futex_wait.load(Ordering::Acquire) != uaddr)
            || read_user_value::<u32>(process, uaddr).map_or(true, |value| value != val)
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
            clear_current_futex_wait(process, uaddr, private);
            if current_unblocked_signal_pending()
                || process.eval_watchdog_expired()
                || process.pending_exit_group().is_some()
            {
                return neg_errno(LinuxError::EINTR);
            }
            finish_relative_futex_wait(restart_key);
            return neg_errno(LinuxError::ETIMEDOUT);
        }
        clear_current_futex_wait(process, uaddr, private);
        if current_unblocked_signal_pending() {
            return neg_errno(LinuxError::EINTR);
        }
        finish_relative_futex_wait(restart_key);
        return 0;
    }
    if let Some(dur) = process.eval_watchdog_remaining() {
        let _ = state.queue.wait_timeout_until(dur, wait_cond);
    } else {
        state.queue.wait_until(wait_cond);
    }
    drop(state);
    prune_empty_key(key);
    clear_current_futex_wait(process, uaddr, private);
    if current_unblocked_signal_pending()
        || process.pending_exit_group().is_some()
        || process.eval_watchdog_expired()
    {
        return neg_errno(LinuxError::EINTR);
    }
    finish_relative_futex_wait(restart_key);
    0
}

pub(super) fn wake_addr(process: &UserProcess, uaddr: usize, count: usize) -> usize {
    // Best-effort kernel-internal wake path for teardown/cancellation cleanup:
    // if the user address has already been unmapped, there is no futex queue to
    // wake and no syscall return value to report.
    // Linux's kernel-generated clear-child-tid and robust-list wakes use the
    // process-shared futex identity, matching the non-private glibc join wait.
    let Some((key, state)) = mapped_futex_state(process, uaddr, false) else {
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
    [true, false].into_iter().any(|private| {
        mapped_futex_state(process, uaddr, private)
            .map(|(_, state)| !state.queue.is_empty())
            .unwrap_or(false)
    })
}

pub(super) fn wake_task(process: &UserProcess, uaddr: usize, task: &AxTaskRef) {
    for private in [true, false] {
        let Some((key, state)) = mapped_futex_state(process, uaddr, private) else {
            continue;
        };
        state.seq.fetch_add(1, Ordering::Release);
        let woken = state.queue.notify_many_where(
            1,
            true,
            |queued| Arc::ptr_eq(queued, task),
            |queued| clear_futex_waiter(queued),
        );
        drop(state);
        prune_empty_key(key);
        if woken != 0 {
            return;
        }
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
    let op = futex_op as u32;
    let val_u32 = val as u32;
    let val3_u32 = _val3 as u32;
    let cmd = op & general::FUTEX_CMD_MASK as u32;
    let allowed_operation_bits = general::FUTEX_CMD_MASK as u32
        | general::FUTEX_PRIVATE_FLAG as u32
        | general::FUTEX_CLOCK_REALTIME as u32;
    let supported_command = matches!(
        cmd,
        general::FUTEX_WAIT
            | general::FUTEX_WAIT_BITSET
            | general::FUTEX_WAKE
            | general::FUTEX_WAKE_BITSET
            | general::FUTEX_REQUEUE
            | general::FUTEX_CMP_REQUEUE
    );
    let private = op & general::FUTEX_PRIVATE_FLAG as u32 != 0;
    let restart_key = (cmd == general::FUTEX_WAIT && timeout != 0)
        .then(|| relative_restart_key(uaddr, futex_op, val_u32 as usize, timeout));
    let timeout_result = match cmd {
        general::FUTEX_WAIT => match restart_key {
            Some(restart_key) => read_relative_futex_timeout(process, timeout, restart_key),
            None => Ok(None),
        },
        general::FUTEX_LOCK_PI
        | general::FUTEX_LOCK_PI2
        | general::FUTEX_WAIT_BITSET
        | general::FUTEX_WAIT_REQUEUE_PI => read_futex_timeout(
            process,
            timeout,
            true,
            op & general::FUTEX_CLOCK_REALTIME as u32 != 0,
        ),
        _ => Ok(None),
    };
    let parsed_timeout = match timeout_result {
        Ok(timeout) => timeout,
        Err(err) => {
            finish_relative_futex_wait(restart_key);
            return neg_errno(err);
        }
    };
    let unsupported_operation = op & !allowed_operation_bits != 0 || !supported_command;
    if unsupported_operation {
        finish_relative_futex_wait(restart_key);
        return neg_errno(LinuxError::ENOSYS);
    }
    let unsupported_clock_operation =
        op & general::FUTEX_CLOCK_REALTIME as u32 != 0 && cmd != general::FUTEX_WAIT_BITSET;
    if unsupported_clock_operation {
        finish_relative_futex_wait(restart_key);
        return neg_errno(LinuxError::ENOSYS);
    }
    let requeue_counts = match cmd {
        general::FUTEX_REQUEUE | general::FUTEX_CMP_REQUEUE => {
            match (
                futex_requeue_count(val_u32),
                futex_requeue_count(timeout as u32),
            ) {
                (Ok(wake_count), Ok(requeue_count)) => Some((wake_count, requeue_count)),
                _ => {
                    finish_relative_futex_wait(restart_key);
                    return neg_errno(LinuxError::EINVAL);
                }
            }
        }
        _ => None,
    };
    if uaddr % size_of::<u32>() != 0 {
        finish_relative_futex_wait(restart_key);
        return neg_errno(LinuxError::EINVAL);
    }
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
        general::FUTEX_WAIT => wait_addr(
            process,
            uaddr,
            val_u32,
            parsed_timeout,
            u32::MAX,
            private,
            restart_key,
        ),
        general::FUTEX_WAIT_BITSET => {
            if val3_u32 == 0 {
                return neg_errno(LinuxError::EINVAL);
            }
            wait_addr(
                process,
                uaddr,
                val_u32,
                parsed_timeout,
                val3_u32,
                private,
                None,
            )
        }
        general::FUTEX_WAKE => {
            match wake_addr_checked(process, uaddr, legacy_futex_wake_count(val_u32), private) {
                Ok(woken) => woken as isize,
                Err(err) => neg_errno(err),
            }
        }
        general::FUTEX_REQUEUE => {
            let Some((wake_count, requeue_count)) = requeue_counts else {
                return neg_errno(LinuxError::ENOSYS);
            };
            match wake_requeue_addr_checked(
                process,
                uaddr,
                wake_count,
                requeue_count,
                _uaddr2,
                None,
                private,
            ) {
                Ok((woken, requeued)) => woken.saturating_add(requeued) as isize,
                Err(err) => neg_errno(err),
            }
        }
        general::FUTEX_CMP_REQUEUE => {
            let Some((wake_count, requeue_count)) = requeue_counts else {
                return neg_errno(LinuxError::ENOSYS);
            };
            match wake_requeue_addr_checked(
                process,
                uaddr,
                wake_count,
                requeue_count,
                _uaddr2,
                Some(val3_u32),
                private,
            ) {
                Ok((woken, requeued)) => woken.saturating_add(requeued) as isize,
                Err(err) => neg_errno(err),
            }
        }
        general::FUTEX_WAKE_BITSET => {
            if val3_u32 == 0 {
                return neg_errno(LinuxError::EINVAL);
            }
            match wake_addr_bitset_checked(
                process,
                uaddr,
                legacy_futex_wake_count(val_u32),
                val3_u32,
                private,
            ) {
                Ok(woken) => woken as isize,
                Err(err) => neg_errno(err),
            }
        }
        _ => neg_errno(LinuxError::ENOSYS),
    }
}
