use core::mem::size_of;
use core::sync::atomic::{AtomicU32, Ordering};

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axsync::Mutex;
use axtask::{AxTaskRef, WaitQueue};
use lazyinit::LazyInit;
use linux_raw_sys::general;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::UserProcess;
use super::linux_abi::{USER_MMAP_BASE, neg_errno};
use super::signal_abi::current_sigcancel_pending;
use super::task_context::current_task_ext;
use super::user_memory::read_user_value;

macro_rules! user_trace {
    ($($arg:tt)*) => {};
}

pub(super) struct FutexState {
    pub(super) seq: AtomicU32,
    pub(super) queue: WaitQueue,
}

fn table() -> &'static Mutex<BTreeMap<usize, Arc<FutexState>>> {
    static FUTEXES: LazyInit<Mutex<BTreeMap<usize, Arc<FutexState>>>> = LazyInit::new();
    if !FUTEXES.is_inited() {
        FUTEXES.init_once(Mutex::new(BTreeMap::new()));
    }
    &FUTEXES
}

fn state(uaddr: usize) -> Arc<FutexState> {
    let mut table = table().lock();
    table
        .entry(uaddr)
        .or_insert_with(|| {
            Arc::new(FutexState {
                seq: AtomicU32::new(0),
                queue: WaitQueue::new(),
            })
        })
        .clone()
}

pub(super) fn wake_addr(uaddr: usize, count: usize) -> usize {
    let Some(state) = table().lock().get(&uaddr).cloned() else {
        return 0;
    };
    state.seq.fetch_add(1, Ordering::Release);
    let mut woken = 0usize;
    for _ in 0..count {
        if !state.queue.notify_one(true) {
            break;
        }
        woken += 1;
    }
    woken
}

pub(super) fn wake_task(uaddr: usize, task: &AxTaskRef) {
    if let Some(state) = table().lock().get(&uaddr).cloned() {
        state.seq.fetch_add(1, Ordering::Release);
        let _ = state.queue.notify_task(true, task);
    }
}

pub(super) fn sys_futex(
    process: &UserProcess,
    _tf: &TrapFrame,
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
        general::FUTEX_WAIT => {
            let current = match read_user_value::<u32>(process, uaddr) {
                Ok(value) => value,
                Err(err) => return neg_errno(err),
            };
            if current != val as u32 {
                return neg_errno(LinuxError::EAGAIN);
            }
            let state = state(uaddr);
            let seq = state.seq.load(Ordering::Acquire);
            if let Some(ext) = current_task_ext() {
                ext.futex_wait.store(uaddr, Ordering::Release);
            }
            let wait_cond = || {
                state.seq.load(Ordering::Acquire) != seq
                    || read_user_value::<u32>(process, uaddr)
                        .map_or(true, |value| value != val as u32)
                    || current_sigcancel_pending()
            };
            if timeout != 0 {
                let ts = match read_user_value::<general::timespec>(process, timeout) {
                    Ok(value) => value,
                    Err(err) => return neg_errno(err),
                };
                let dur = core::time::Duration::new(
                    ts.tv_sec.max(0) as u64,
                    ts.tv_nsec.clamp(0, 999_999_999) as u32,
                );
                if state.queue.wait_timeout_until(dur, wait_cond) {
                    if let Some(ext) = current_task_ext() {
                        ext.futex_wait.store(0, Ordering::Release);
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
            state.queue.wait_until(wait_cond);
            if let Some(ext) = current_task_ext() {
                ext.futex_wait.store(0, Ordering::Release);
            }
            if current_sigcancel_pending() {
                return neg_errno(LinuxError::EINTR);
            }
            0
        }
        general::FUTEX_WAKE => wake_addr(uaddr, val) as isize,
        _ => neg_errno(LinuxError::ENOSYS),
    }
}
