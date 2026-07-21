#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
use core::mem::{offset_of, size_of};
use core::sync::atomic::{AtomicBool, Ordering};

use axerrno::LinuxError;
use axhal::context::TrapFrame;
#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
use axhal::trap::PageFaultFlags;
use axhal::trap::{USER_EXCEPTION, register_trap_handler, register_user_return_handler};
use linux_raw_sys::general;
#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
use memory_addr::{PAGE_SIZE_4K, VirtAddr};

use super::UserProcess;
use super::futex;
use super::linux_abi::{
    KERNEL_SIGSET_BYTES, SIG_BLOCK_HOW, SIG_SETMASK_HOW, SIG_UNBLOCK_HOW, SIGABRT_NUM, SIGALRM_NUM,
    SIGCANCEL_NUM, SIGCONT_NUM, SIGFPE_NUM, SIGILL_NUM, SIGINT_NUM, SIGKILL_NUM, SIGPIPE_NUM,
    SIGQUIT_NUM, SIGSEGV_NUM, SIGSTOP_NUM, SIGTERM_NUM, neg_errno,
};
#[cfg(target_arch = "loongarch64")]
use super::linux_abi::{LOONGARCH_SIGTRAMP_CODE, SA_NODEFER_FLAG, SI_TKILL_CODE, SS_DISABLE};
#[cfg(target_arch = "riscv64")]
use super::linux_abi::{RISCV_SIGTRAMP_CODE, SA_NODEFER_FLAG, SI_TKILL_CODE, SS_DISABLE};
#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
use super::memory_map::{align_down, align_up, user_mapping_flags};
#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
use super::process_lifecycle::{terminate_current_thread, terminate_current_thread_for_exit_group};
use super::task_context::{UserTaskExt, current_task_ext, current_tid};
use super::task_registry::{
    UserThreadEntry, user_thread_entries_by_process_group, user_thread_entry_by_process_pid,
    user_thread_entry_by_tid, user_thread_entry_for_process,
};
use super::user_memory::{
    clear_user_bytes, read_user_bytes, read_user_value, write_user_bytes, write_user_value,
};

const NO_SIGSUSPEND_RESTORE_MASK: u64 = u64::MAX;

pub(super) struct TemporarySignalMaskGuard {
    ext: &'static UserTaskExt,
    old_mask: u64,
}

impl Drop for TemporarySignalMaskGuard {
    fn drop(&mut self) {
        self.ext.signal_mask.store(self.old_mask, Ordering::Release);
    }
}

#[cfg(target_arch = "riscv64")]
use super::linux_abi::{RISCV_SIGNAL_FPSTATE_BYTES, RISCV_SIGNAL_SIGSET_RESERVED_BYTES};

static USER_RETURN_HOOK_REGISTERED: AtomicBool = AtomicBool::new(false);

pub(super) fn validate_signal_target(sig: i32) -> Result<(), LinuxError> {
    if sig < 0 || sig > 64 {
        return Err(LinuxError::EINVAL);
    }
    Ok(())
}

pub(super) fn signal_mask_bit(sig: i32) -> u64 {
    if (1..=64).contains(&sig) {
        1u64 << ((sig - 1) as u32)
    } else {
        0
    }
}

fn unmaskable_signal_bits() -> u64 {
    signal_mask_bit(SIGKILL_NUM) | signal_mask_bit(SIGSTOP_NUM)
}

fn first_signal_from_mask(mask: u64) -> i32 {
    if mask == 0 {
        0
    } else {
        mask.trailing_zeros() as i32 + 1
    }
}

fn pending_signal_mask(ext: &UserTaskExt) -> u64 {
    ext.pending_signal_mask.load(Ordering::Acquire)
}

fn set_pending_signal_hint(ext: &UserTaskExt, mask: u64) {
    ext.pending_signal
        .store(first_signal_from_mask(mask), Ordering::Release);
}

fn queue_pending_signal(ext: &UserTaskExt, sig: i32, sender_pid: i32) {
    queue_pending_signal_info(ext, sig, sender_pid, SI_TKILL_CODE, 0, 0);
}

fn queue_pending_signal_info(
    ext: &UserTaskExt,
    sig: i32,
    sender_pid: i32,
    code: i32,
    sender_uid: u32,
    value: usize,
) {
    let bit = signal_mask_bit(sig);
    if bit == 0 {
        return;
    }
    ext.pending_signal_sender
        .store(sender_pid, Ordering::Release);
    ext.pending_signal_code.store(code, Ordering::Release);
    ext.pending_signal_uid.store(sender_uid, Ordering::Release);
    ext.pending_signal_value.store(value, Ordering::Release);
    ext.pending_signal_mask.fetch_or(bit, Ordering::AcqRel);
    ext.pending_signal.store(sig, Ordering::Release);
}

fn clear_pending_signal(ext: &UserTaskExt, sig: i32) {
    let bit = signal_mask_bit(sig);
    if bit == 0 {
        return;
    }
    let next = ext.pending_signal_mask.fetch_and(!bit, Ordering::AcqRel) & !bit;
    set_pending_signal_hint(ext, next);
}

fn first_unblocked_pending_signal(ext: &UserTaskExt) -> i32 {
    let mut mask = pending_signal_mask(ext);
    while mask != 0 {
        let sig = first_signal_from_mask(mask);
        if sig == 0 {
            break;
        }
        let bit = signal_mask_bit(sig);
        if !signal_is_blocked(ext, sig) {
            return sig;
        }
        mask &= !bit;
    }
    0
}

fn restartable_blocking_syscall(tf: &TrapFrame, syscall_num: u32) -> bool {
    if matches!(syscall_num, general::__NR_wait4 | general::__NR_waitid) {
        return true;
    }
    if syscall_num != general::__NR_futex {
        return false;
    }

    // FUTEX_WAIT_BITSET uses an absolute timeout (or no timeout), so replaying
    // the original syscall frame after an SA_RESTART handler preserves its
    // deadline. FUTEX_WAIT instead accepts a relative timeout; replaying that
    // pointer would extend the wait, so timed WAIT needs a dedicated Linux-style
    // restart block before it can be included here.
    tf.arg1() as u32 & general::FUTEX_CMD_MASK as u32 == general::FUTEX_WAIT_BITSET
}

pub(super) fn note_syscall_restart_candidate(tf: &TrapFrame, syscall_num: u32, ret: isize) {
    let Some(ext) = current_task_ext() else {
        return;
    };
    if ret == neg_errno(LinuxError::EINTR) && restartable_blocking_syscall(tf, syscall_num) {
        ext.store_syscall_restart_frame(*tf);
    } else {
        // A restart frame is tied to the syscall that was just interrupted.
        // Once any later syscall reaches a non-restartable result, stale wait
        // state must not be reused by a future SA_RESTART signal delivery.
        ext.clear_syscall_restart_frame();
    }
}

fn take_syscall_restart_frame(ext: &UserTaskExt) -> Option<TrapFrame> {
    ext.take_syscall_restart_frame()
}

fn clear_syscall_restart_frame(ext: &UserTaskExt) {
    ext.clear_syscall_restart_frame();
}

fn signal_action_restarts_syscall(action: &general::kernel_sigaction) -> bool {
    action.sa_flags & general::SA_RESTART as u64 != 0
}

pub(super) fn all_application_signal_mask() -> u64 {
    !unmaskable_signal_bits()
}

fn take_sigsuspend_restore_mask(ext: &UserTaskExt, current_mask: u64) -> u64 {
    let saved = ext
        .sigsuspend_restore_mask
        .swap(NO_SIGSUSPEND_RESTORE_MASK, Ordering::AcqRel);
    if saved == NO_SIGSUSPEND_RESTORE_MASK {
        current_mask
    } else {
        saved
    }
}

fn default_signal_terminates(sig: i32) -> bool {
    // Linux default actions for standard signals: 1..=16 terminate or dump
    // core, 17/18/23/28 are ignored by default, 19..=22 stop, and 24..=31
    // terminate or dump core.  Keep this independent of test names so wait
    // status reflects the signal that actually ended the process.
    matches!(
        sig,
        1 | SIGINT_NUM
            | SIGQUIT_NUM
            | SIGILL_NUM
            | 5
            | SIGABRT_NUM
            | 7
            | SIGFPE_NUM
            | SIGKILL_NUM
            | 10
            | SIGSEGV_NUM
            | 12
            | SIGPIPE_NUM
            | SIGALRM_NUM
            | SIGTERM_NUM
            | 16
            | 24..=31
    )
}

pub(super) fn signal_is_blocked(ext: &UserTaskExt, sig: i32) -> bool {
    if sig == SIGKILL_NUM {
        return false;
    }
    let bit = signal_mask_bit(sig);
    bit != 0 && ext.signal_mask.load(Ordering::Acquire) & bit != 0
}

fn realtime_signal_queue_blocked_by_rlimit(entry: &UserThreadEntry, sig: i32) -> bool {
    if sig < 32 {
        return false;
    }
    let Some(ext) = super::task_context::task_ext(&entry.task) else {
        return false;
    };
    signal_is_blocked(ext, sig) && ext.process.get_rlimit(general::RLIMIT_SIGPENDING).current() == 0
}

pub(super) fn queue_current_synchronous_signal(sig: i32) -> bool {
    let Some(ext) = current_task_ext() else {
        return false;
    };
    if signal_is_blocked(ext, sig)
        || ext.signal_frame.load(Ordering::Acquire) != 0
        || pending_signal_mask(ext) != 0
    {
        return false;
    }
    let action = ext
        .process
        .signal_actions
        .lock()
        .get(&(sig as usize))
        .copied()
        .unwrap_or_else(|| unsafe { core::mem::zeroed() });
    let handler = action
        .sa_handler_kernel
        .map(|func| func as usize)
        .unwrap_or(0);
    if handler <= 1 {
        return false;
    }
    let Some(entry) = user_thread_entry_by_tid(current_tid()) else {
        return false;
    };
    deliver_user_signal(&entry, sig, 0).is_ok()
}

pub(super) fn current_unblocked_signal_pending() -> bool {
    current_task_ext().is_some_and(|ext| first_unblocked_pending_signal(ext) != 0)
}

pub(super) fn current_pending_signal_matches(mask: u64) -> bool {
    current_task_ext().is_some_and(|ext| pending_signal_mask(ext) & mask != 0)
}

pub(super) fn thread_waits_for_signal(entry: &UserThreadEntry, sig: i32) -> bool {
    let Some(ext) = super::task_context::task_ext(&entry.task) else {
        return false;
    };
    ext.signal_wait.load(Ordering::Acquire)
        && ext.signal_wait_mask.load(Ordering::Acquire) & signal_mask_bit(sig) != 0
}

fn enter_signal_wait(ext: &UserTaskExt, wait_mask: u64) {
    ext.signal_wait_mask.store(wait_mask, Ordering::Release);
    ext.signal_wait.store(true, Ordering::Release);
}

fn leave_signal_wait(ext: &UserTaskExt) {
    ext.signal_wait.store(false, Ordering::Release);
    ext.signal_wait_mask.store(0, Ordering::Release);
}

pub(super) fn take_current_pending_signal_matching(mask: u64) -> Option<(i32, i32)> {
    let ext = current_task_ext()?;
    loop {
        let pending = pending_signal_mask(ext);
        let matched = pending & mask;
        if matched == 0 {
            return None;
        }
        let sig = first_signal_from_mask(matched);
        let bit = signal_mask_bit(sig);
        let next = pending & !bit;
        if ext
            .pending_signal_mask
            .compare_exchange(pending, next, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            set_pending_signal_hint(ext, next);
            let sender_pid = ext.pending_signal_sender.load(Ordering::Acquire);
            return Some((sig, sender_pid));
        }
    }
}

pub(super) fn install_temporary_signal_mask(
    process: &UserProcess,
    set: usize,
    sigsetsize: usize,
) -> Result<Option<TemporarySignalMaskGuard>, LinuxError> {
    if set == 0 {
        return Ok(None);
    }
    if sigsetsize != 0 && sigsetsize < KERNEL_SIGSET_BYTES {
        return Err(LinuxError::EINVAL);
    }
    let src = read_user_bytes(process, set, KERNEL_SIGSET_BYTES)?;
    let mut set_bytes = [0u8; KERNEL_SIGSET_BYTES];
    set_bytes.copy_from_slice(&src);
    let mask = u64::from_ne_bytes(set_bytes) & !unmaskable_signal_bits();
    let ext = current_task_ext().ok_or(LinuxError::EINVAL)?;
    let old_mask = ext.signal_mask.swap(mask, Ordering::AcqRel);
    Ok(Some(TemporarySignalMaskGuard { ext, old_mask }))
}

pub(super) fn deliver_user_signal(
    entry: &UserThreadEntry,
    sig: i32,
    sender_pid: i32,
) -> Result<(), LinuxError> {
    deliver_user_signal_inner(entry, sig, sender_pid, SI_TKILL_CODE, 0, 0)
}

pub(super) fn deliver_user_signal_with_siginfo(
    entry: &UserThreadEntry,
    sig: i32,
    sender_pid: i32,
    code: i32,
    sender_uid: u32,
    value: i32,
) -> Result<(), LinuxError> {
    deliver_user_signal_inner(entry, sig, sender_pid, code, sender_uid, value as usize)
}

fn deliver_user_signal_inner(
    entry: &UserThreadEntry,
    sig: i32,
    sender_pid: i32,
    code: i32,
    sender_uid: u32,
    value: usize,
) -> Result<(), LinuxError> {
    if sig == 0 {
        return Ok(());
    }
    let ext = super::task_context::task_ext(&entry.task).ok_or(LinuxError::ESRCH)?;
    let action = ext
        .process
        .signal_actions
        .lock()
        .get(&(sig as usize))
        .copied()
        .unwrap_or_else(|| unsafe { core::mem::zeroed() });
    let handler = action
        .sa_handler_kernel
        .map(|func| func as usize)
        .unwrap_or(0);
    if handler == 0 && default_signal_terminates(sig) && !signal_is_blocked(ext, sig) {
        ext.process.request_signal_exit_group(sig);
    }
    if handler == 0 && !signal_is_blocked(ext, sig) {
        if sig == SIGSTOP_NUM {
            ext.process.record_wait_stopped(sig);
        } else if sig == SIGCONT_NUM {
            ext.process.record_wait_continued(sig);
        }
    }
    queue_pending_signal_info(ext, sig, sender_pid, code, sender_uid, value);
    let wakes_signal_wait = ext.signal_wait.load(Ordering::Acquire)
        && ext.signal_wait_mask.load(Ordering::Acquire) & signal_mask_bit(sig) != 0;
    if !signal_is_blocked(ext, sig) || wakes_signal_wait {
        // A newly-pending unblocked signal interrupts sleep-like syscalls such
        // as wait4()/waitid() and nanosleep().  A blocked signal selected by
        // sigtimedwait() must also wake that syscall even though it remains
        // blocked for normal handler delivery.
        ext.process.child_exit_wait.notify_all(true);
        ext.process.timer_wait.notify_all(true);
    }
    if sig == SIGCANCEL_NUM {
        user_trace!(
            "sigdbg: deliver tid={} blocked={} futex_wait={:#x}",
            entry.task.id().as_u64(),
            signal_is_blocked(ext, sig),
            ext.futex_wait.load(Ordering::Acquire),
        );
    }
    if !signal_is_blocked(ext, sig) {
        let futex_wait = ext.futex_wait.load(Ordering::Acquire);
        if futex_wait != 0 {
            futex::wake_task(ext.process.as_ref(), futex_wait, &entry.task);
        }
    }
    Ok(())
}

fn request_pending_default_terminate_signal(ext: &UserTaskExt) {
    let mut mask = pending_signal_mask(ext);
    while mask != 0 {
        let sig = first_signal_from_mask(mask);
        if sig == 0 {
            return;
        }
        let bit = signal_mask_bit(sig);
        mask &= !bit;
        if signal_is_blocked(ext, sig) {
            continue;
        }
        let action = ext
            .process
            .signal_actions
            .lock()
            .get(&(sig as usize))
            .copied()
            .unwrap_or_else(|| unsafe { core::mem::zeroed() });
        let handler = action
            .sa_handler_kernel
            .map(|func| func as usize)
            .unwrap_or(0);
        if handler == 0 && default_signal_terminates(sig) {
            ext.process.request_signal_exit_group(sig);
            return;
        }
    }
}

fn terminate_current_if_exit_group_pending(process: &UserProcess) -> ! {
    if let Some(code) = process.pending_exit_group() {
        terminate_current_thread_for_exit_group(process, code);
    }
    unreachable!("exit-group termination requested before synchronous signal termination");
}

fn deliver_user_signal_result(entry: &UserThreadEntry, sig: i32, sender_pid: i32) -> isize {
    match deliver_user_signal(entry, sig, sender_pid) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

fn signal_permission_allowed(sender: &UserProcess, target: &UserProcess, sig: i32) -> bool {
    if sender.uid() == 0 {
        return true;
    }
    if sig == SIGCONT_NUM && sender.sid() == target.sid() {
        return true;
    }
    let sender_real = sender.real_uid();
    let sender_effective = sender.uid();
    sender_real == target.real_uid()
        || sender_real == target.saved_uid()
        || sender_effective == target.real_uid()
        || sender_effective == target.saved_uid()
}

fn deliver_process_group_signal(process: &UserProcess, pgid: i32, sig: i32) -> isize {
    let entries = user_thread_entries_by_process_group(pgid);
    if entries.is_empty() {
        return neg_errno(LinuxError::ESRCH);
    }
    let mut delivered = false;
    for entry in entries {
        if !signal_permission_allowed(process, entry.process.as_ref(), sig) {
            continue;
        }
        if let Err(err) = deliver_user_signal(&entry, sig, process.pid()) {
            return neg_errno(err);
        }
        delivered = true;
    }
    if !delivered {
        return neg_errno(LinuxError::EPERM);
    }
    0
}

#[register_trap_handler(USER_EXCEPTION)]
fn user_exception(_tf: &TrapFrame, signal: usize) -> bool {
    let Some(ext) = current_task_ext() else {
        return false;
    };
    let signal = match signal as i32 {
        SIGILL_NUM | SIGSEGV_NUM => signal as i32,
        _ => SIGSEGV_NUM,
    };
    ext.process.request_signal_exit_group(signal);
    terminate_current_thread(ext.process.as_ref(), 128 + signal)
}

pub(super) fn ensure_user_return_hook_registered() {
    if !USER_RETURN_HOOK_REGISTERED.swap(true, Ordering::AcqRel) {
        register_user_return_handler(user_return_hook);
    }
}

fn user_return_hook(tf: &mut TrapFrame) {
    #[cfg(target_arch = "loongarch64")]
    {
        tf.prmd = 0x7;
    }
    let Some(ext) = current_task_ext() else {
        return;
    };
    let eval_deadline_us = ext
        .process
        .eval_watchdog_deadline_us
        .load(Ordering::Acquire);
    if eval_deadline_us != 0
        && (axhal::time::monotonic_time().as_micros() as u64) >= eval_deadline_us
    {
        ext.process.request_eval_exit_tree(137);
        terminate_current_thread_for_exit_group(ext.process.as_ref(), 137);
    }
    if let Some(code) = ext.process.pending_exit_group() {
        terminate_current_thread_for_exit_group(ext.process.as_ref(), code);
    }
    if ext.signal_frame.load(Ordering::Acquire) == 0 {
        if let Some(restored) = ext.pending_sigreturn.lock().take() {
            *tf = restored;
        }
    }
    let _ = ext.process.consume_expired_real_timer();
    let _ = ext.process.consume_expired_cpu_timers();
    #[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
    if ext.signal_frame.load(Ordering::Acquire) == 0 {
        let sig = first_unblocked_pending_signal(ext);
        if sig != 0 {
            let _ = inject_pending_signal(tf, ext, sig);
        }
    }
}

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
fn ensure_signal_frame_pages(
    process: &UserProcess,
    start: usize,
    len: usize,
) -> Result<(), LinuxError> {
    let end = start.checked_add(len).ok_or(LinuxError::EFAULT)?;
    let page_start = align_down(start, PAGE_SIZE_4K);
    let page_end = align_up(end, PAGE_SIZE_4K);
    let mut aspace = process.aspace.lock();
    for page in (page_start..page_end).step_by(PAGE_SIZE_4K) {
        let _ = aspace.handle_page_fault(VirtAddr::from(page), PageFaultFlags::WRITE);
    }
    aspace
        .protect(
            VirtAddr::from(page_start),
            page_end - page_start,
            user_mapping_flags(true, true, true),
        )
        .map_err(LinuxError::from)
}

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
fn sigaltstack_range(ext: &UserTaskExt) -> Option<(usize, usize)> {
    let flags = ext.sigaltstack_flags.load(Ordering::Acquire) as u32;
    if flags & SS_DISABLE as u32 != 0 {
        return None;
    }
    let start = ext.sigaltstack_sp.load(Ordering::Acquire);
    let size = usize::try_from(ext.sigaltstack_size.load(Ordering::Acquire)).ok()?;
    let end = start.checked_add(size)?;
    (start < end).then_some((start, end))
}

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
fn pointer_on_sigaltstack(ext: &UserTaskExt, pointer: usize) -> bool {
    pointer != 0
        && sigaltstack_range(ext).is_some_and(|(start, end)| (start..end).contains(&pointer))
}

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
fn sigaltstack_at_pointer(ext: &UserTaskExt, pointer: usize) -> general::sigaltstack {
    let mut stack = general::sigaltstack {
        ss_sp: ext.sigaltstack_sp.load(Ordering::Acquire) as *mut core::ffi::c_void,
        ss_flags: ext.sigaltstack_flags.load(Ordering::Acquire),
        ss_size: ext.sigaltstack_size.load(Ordering::Acquire),
    };
    if stack.ss_flags as u32 & SS_DISABLE as u32 == 0 && pointer_on_sigaltstack(ext, pointer) {
        stack.ss_flags |= general::SS_ONSTACK as i32;
    }
    stack
}

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
fn signal_frame_layout(
    ext: &UserTaskExt,
    action_flags: u64,
    current_sp: usize,
    frame_size: usize,
) -> Result<(usize, general::sigaltstack), LinuxError> {
    let interrupted_stack = sigaltstack_at_pointer(ext, current_sp);
    let already_on_altstack = interrupted_stack.ss_flags & general::SS_ONSTACK as i32 != 0;
    let use_altstack = action_flags & general::SA_ONSTACK as u64 != 0
        && interrupted_stack.ss_flags & SS_DISABLE == 0
        && !already_on_altstack;
    let (stack_top, altstack_start) = if use_altstack {
        let (start, end) = sigaltstack_range(ext).ok_or(LinuxError::EFAULT)?;
        (end, Some(start))
    } else {
        (current_sp, None)
    };
    let frame_addr = stack_top
        .checked_sub(frame_size)
        .map(|address| align_down(address, 16))
        .ok_or(LinuxError::EFAULT)?;
    if altstack_start.is_some_and(|start| frame_addr < start) {
        return Err(LinuxError::EFAULT);
    }
    Ok((frame_addr, interrupted_stack))
}

#[cfg(target_arch = "riscv64")]
fn inject_pending_signal(
    tf: &mut TrapFrame,
    ext: &UserTaskExt,
    sig: i32,
) -> Result<(), LinuxError> {
    let action = ext
        .process
        .signal_actions
        .lock()
        .get(&(sig as usize))
        .copied()
        .unwrap_or_else(|| unsafe { core::mem::zeroed() });
    let handler = action
        .sa_handler_kernel
        .map(|func| func as usize)
        .unwrap_or(0);
    if sig >= 32 {
        user_trace!(
            "sigdbg: inject tid={} sig={sig} handler={handler:#x} flags={:#x} sp={:#x} tp={:#x}",
            current_tid(),
            action.sa_flags,
            tf.regs.sp,
            tf.regs.tp,
        );
    }
    if handler <= 1 {
        clear_syscall_restart_frame(ext);
        clear_pending_signal(ext, sig);
        let current_mask = ext.signal_mask.load(Ordering::Acquire);
        let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
        if handler == 0 && default_signal_terminates(sig) {
            ext.process.request_signal_exit_group(sig);
            terminate_current_thread_for_exit_group(ext.process.as_ref(), 128 + sig);
        }
        ext.signal_mask.store(restore_mask, Ordering::Release);
        return Ok(());
    }
    let current_mask = ext.signal_mask.load(Ordering::Acquire);
    let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
    let saved_tf = if signal_action_restarts_syscall(&action) {
        take_syscall_restart_frame(ext).unwrap_or(*tf)
    } else {
        clear_syscall_restart_frame(ext);
        *tf
    };
    let frame_size = riscv_signal_frame_size();
    let (frame_addr, interrupted_stack) =
        signal_frame_layout(ext, action.sa_flags, tf.regs.sp, frame_size)?;
    ensure_signal_frame_pages(ext.process.as_ref(), frame_addr, frame_size)?;

    let sender_pid = ext.pending_signal_sender.load(Ordering::Acquire);
    let code = ext.pending_signal_code.load(Ordering::Acquire);
    let sender_uid = ext.pending_signal_uid.load(Ordering::Acquire);
    let value = ext.pending_signal_value.load(Ordering::Acquire);
    let frame = make_riscv_signal_frame(
        sig,
        code,
        sender_pid,
        sender_uid,
        value,
        restore_mask,
        interrupted_stack.ss_sp as usize,
        interrupted_stack.ss_flags,
        usize::try_from(interrupted_stack.ss_size).map_err(|_| LinuxError::EFAULT)?,
        RISCV_SIGTRAMP_CODE,
        trap_frame_to_riscv_sigcontext(&saved_tf),
    );

    let frame_ret = write_user_value(ext.process.as_ref(), frame_addr, &frame);
    if frame_ret != 0 {
        return Err(LinuxError::EFAULT);
    }

    *ext.pending_sigreturn.lock() = Some(saved_tf);
    ext.signal_frame.store(frame_addr, Ordering::Release);
    clear_pending_signal(ext, sig);
    let mut next_mask = current_mask | action.sa_mask.sig[0];
    if action.sa_flags & SA_NODEFER_FLAG == 0 {
        next_mask |= signal_mask_bit(sig);
    }
    ext.signal_mask.store(next_mask, Ordering::Release);
    if sig >= 32 {
        user_trace!(
            "sigdbg: frame tid={} sig={sig} frame_addr={frame_addr:#x} size={frame_size:#x}",
            current_tid(),
        );
    }

    let frame_offsets = riscv_signal_frame_offsets();
    tf.regs.sp = frame_addr;
    tf.regs.ra = frame_addr + frame_offsets.trampoline;
    tf.regs.a0 = sig as usize;
    tf.regs.a1 = frame_addr + frame_offsets.info;
    tf.regs.a2 = frame_addr + frame_offsets.ucontext;
    tf.sepc = handler;
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn inject_pending_signal(
    tf: &mut TrapFrame,
    ext: &UserTaskExt,
    sig: i32,
) -> Result<(), LinuxError> {
    let action = ext
        .process
        .signal_actions
        .lock()
        .get(&(sig as usize))
        .copied()
        .unwrap_or_else(|| unsafe { core::mem::zeroed() });
    let handler = action
        .sa_handler_kernel
        .map(|func| func as usize)
        .unwrap_or(0);
    if handler <= 1 {
        clear_syscall_restart_frame(ext);
        clear_pending_signal(ext, sig);
        let current_mask = ext.signal_mask.load(Ordering::Acquire);
        let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
        if handler == 0 && default_signal_terminates(sig) {
            ext.process.request_signal_exit_group(sig);
            terminate_current_thread_for_exit_group(ext.process.as_ref(), 128 + sig);
        }
        ext.signal_mask.store(restore_mask, Ordering::Release);
        return Ok(());
    }

    let current_mask = ext.signal_mask.load(Ordering::Acquire);
    let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
    let saved_tf = if signal_action_restarts_syscall(&action) {
        take_syscall_restart_frame(ext).unwrap_or(*tf)
    } else {
        clear_syscall_restart_frame(ext);
        *tf
    };
    let frame_size = loongarch_signal_frame_size();
    let (frame_addr, interrupted_stack) =
        signal_frame_layout(ext, action.sa_flags, tf.regs.sp, frame_size)?;
    ensure_signal_frame_pages(ext.process.as_ref(), frame_addr, frame_size)?;

    let sender_pid = ext.pending_signal_sender.load(Ordering::Acquire);
    let code = ext.pending_signal_code.load(Ordering::Acquire);
    let sender_uid = ext.pending_signal_uid.load(Ordering::Acquire);
    let value = ext.pending_signal_value.load(Ordering::Acquire);
    let frame = make_loongarch_signal_frame(
        sig,
        code,
        sender_pid,
        sender_uid,
        value,
        restore_mask,
        interrupted_stack.ss_sp as usize,
        interrupted_stack.ss_flags,
        usize::try_from(interrupted_stack.ss_size).map_err(|_| LinuxError::EFAULT)?,
        LOONGARCH_SIGTRAMP_CODE,
        trap_frame_to_loongarch_sigcontext(&saved_tf),
    );
    let frame_ret = write_user_value(ext.process.as_ref(), frame_addr, &frame);
    if frame_ret != 0 {
        return Err(LinuxError::EFAULT);
    }

    *ext.pending_sigreturn.lock() = Some(saved_tf);
    ext.signal_frame.store(frame_addr, Ordering::Release);
    clear_pending_signal(ext, sig);
    let mut next_mask = current_mask | action.sa_mask.sig[0];
    if action.sa_flags & SA_NODEFER_FLAG == 0 {
        next_mask |= signal_mask_bit(sig);
    }
    ext.signal_mask.store(next_mask, Ordering::Release);

    let frame_offsets = loongarch_signal_frame_offsets();
    tf.regs.sp = frame_addr;
    tf.regs.ra = frame_addr + frame_offsets.trampoline;
    tf.regs.a0 = sig as usize;
    tf.regs.a1 = frame_addr + frame_offsets.info;
    tf.regs.a2 = frame_addr + frame_offsets.ucontext;
    tf.era = handler;
    Ok(())
}

pub(super) fn sys_rt_sigaction(
    process: &UserProcess,
    signum: usize,
    act: usize,
    oldact: usize,
    sigsetsize: usize,
) -> isize {
    if sigsetsize != KERNEL_SIGSET_BYTES {
        return neg_errno(LinuxError::EINVAL);
    }

    if signum == 0 || signum >= 65 {
        return neg_errno(LinuxError::EINVAL);
    }
    if signum as i32 == SIGKILL_NUM || signum as i32 == SIGSTOP_NUM {
        return neg_errno(LinuxError::EINVAL);
    }

    let new_action = if act != 0 {
        match read_user_value::<general::kernel_sigaction>(process, act) {
            Ok(value) => Some(value),
            Err(err) => return neg_errno(err),
        }
    } else {
        None
    };

    if oldact != 0 {
        let old = process
            .signal_actions
            .lock()
            .get(&signum)
            .copied()
            .unwrap_or_else(|| unsafe { core::mem::zeroed() });
        let ret = write_user_value(process, oldact, &old);
        if ret != 0 {
            return ret;
        }
    }

    if let Some(new_action) = new_action {
        if signum >= 32 {
            let _handler = new_action
                .sa_handler_kernel
                .map(|func| func as usize)
                .unwrap_or(0);
            user_trace!(
                "sigdbg: rt_sigaction tid={} sig={} handler={_handler:#x} flags={:#x} mask={:#x}",
                current_tid(),
                signum,
                new_action.sa_flags,
                new_action.sa_mask.sig[0],
            );
        }
        process.signal_actions.lock().insert(signum, new_action);
        if signum == general::SIGCHLD as usize && process.sigchld_discards_wait_status() {
            process.reap_exited_ignored_children();
        }
    }

    0
}

fn current_sigaltstack(ext: &UserTaskExt) -> general::sigaltstack {
    sigaltstack_at_pointer(ext, ext.signal_frame.load(Ordering::Acquire))
}

pub(super) fn sys_sigaltstack(process: &UserProcess, ss: usize, old_ss: usize) -> isize {
    let Some(ext) = current_task_ext() else {
        return neg_errno(LinuxError::EINVAL);
    };

    if old_ss != 0 {
        let old = current_sigaltstack(ext);
        let ret = write_user_value(process, old_ss, &old);
        if ret != 0 {
            return ret;
        }
    }

    if ss == 0 {
        return 0;
    }
    if pointer_on_sigaltstack(ext, ext.signal_frame.load(Ordering::Acquire)) {
        return neg_errno(LinuxError::EPERM);
    }

    let next = match read_user_value::<general::sigaltstack>(process, ss) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
    let allowed_flags = (SS_DISABLE as u32) | general::SS_AUTODISARM;
    let next_flags = next.ss_flags as u32;
    if next_flags & !allowed_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if next_flags & SS_DISABLE as u32 == 0 && next.ss_size < general::MINSIGSTKSZ as u64 {
        return neg_errno(LinuxError::ENOMEM);
    }

    ext.sigaltstack_sp
        .store(next.ss_sp as usize, Ordering::Release);
    ext.sigaltstack_flags
        .store(next.ss_flags, Ordering::Release);
    ext.sigaltstack_size.store(next.ss_size, Ordering::Release);
    0
}

pub(super) fn sys_rt_sigreturn(process: &UserProcess) -> isize {
    #[cfg(target_arch = "riscv64")]
    {
        let Some(ext) = current_task_ext() else {
            return neg_errno(LinuxError::EINVAL);
        };
        let frame_addr = ext.signal_frame.load(Ordering::Acquire);
        if frame_addr == 0 {
            return neg_errno(LinuxError::EINVAL);
        }
        let frame = match read_user_value::<RiscvSignalFrame>(process, frame_addr) {
            Ok(frame) => frame,
            Err(err) => return neg_errno(err),
        };
        let Some(mut restored) = ext.pending_sigreturn.lock().take() else {
            return neg_errno(LinuxError::EINVAL);
        };
        apply_riscv_sigcontext(&mut restored, &frame.ucontext.mcontext);
        ext.signal_mask
            .store(frame.ucontext.sigmask.sig[0], Ordering::Release);
        if pending_signal_mask(ext) == 0 {
            user_trace!(
                "sigdbg: rt_sigreturn tid={} frame={frame_addr:#x} restore_sp={:#x} restore_tp={:#x} restore_pc={:#x}",
                current_tid(),
                restored.regs.sp,
                restored.regs.tp,
                restored.sepc,
            );
        }
        ext.signal_frame.store(0, Ordering::Release);
        // A restart frame is only valid for the signal being delivered.  Normal
        // SA_RESTART delivery consumes it while building the signal frame; if it
        // is still present when the handler returns, it is stale and must not be
        // reused by a later signal.
        clear_syscall_restart_frame(ext);
        *ext.pending_sigreturn.lock() = Some(restored);
        axtask::yield_now();
        0
    }
    #[cfg(target_arch = "loongarch64")]
    {
        let Some(ext) = current_task_ext() else {
            return neg_errno(LinuxError::EINVAL);
        };
        let frame_addr = ext.signal_frame.load(Ordering::Acquire);
        if frame_addr == 0 {
            return neg_errno(LinuxError::EINVAL);
        }
        let frame = match read_user_value::<LoongArchSignalFrame>(process, frame_addr) {
            Ok(frame) => frame,
            Err(err) => return neg_errno(err),
        };
        let Some(mut restored) = ext.pending_sigreturn.lock().take() else {
            return neg_errno(LinuxError::EINVAL);
        };
        apply_loongarch_sigcontext(&mut restored, &frame.ucontext.mcontext);
        restored.prmd = 0x7;
        ext.signal_mask
            .store(frame.ucontext.sigmask.sig[0], Ordering::Release);
        ext.signal_frame.store(0, Ordering::Release);
        // A restart frame is only valid for the signal being delivered.  Normal
        // SA_RESTART delivery consumes it while building the signal frame; if it
        // is still present when the handler returns, it is stale and must not be
        // reused by a later signal.
        clear_syscall_restart_frame(ext);
        *ext.pending_sigreturn.lock() = Some(restored);
        axtask::yield_now();
        0
    }
    #[cfg(not(any(target_arch = "riscv64", target_arch = "loongarch64")))]
    {
        let _ = process;
        neg_errno(LinuxError::ENOSYS)
    }
}

pub(super) fn sys_rt_sigprocmask(
    process: &UserProcess,
    how: usize,
    set: usize,
    oldset: usize,
    sigsetsize: usize,
) -> isize {
    let Some(ext) = current_task_ext() else {
        return neg_errno(LinuxError::EINVAL);
    };
    if sigsetsize != 0 && sigsetsize < KERNEL_SIGSET_BYTES {
        return neg_errno(LinuxError::EINVAL);
    }
    let current_mask = ext.signal_mask.load(Ordering::Acquire);
    if oldset != 0 {
        if let Err(err) = clear_user_bytes(process, oldset, sigsetsize) {
            return neg_errno(err);
        }
        if sigsetsize >= KERNEL_SIGSET_BYTES {
            if let Err(err) = write_user_bytes(process, oldset, &current_mask.to_ne_bytes()) {
                return neg_errno(err);
            }
        }
    }
    if set != 0 {
        let src = match read_user_bytes(process, set, KERNEL_SIGSET_BYTES) {
            Ok(src) => src,
            Err(err) => return neg_errno(err),
        };
        let mut set_bytes = [0u8; KERNEL_SIGSET_BYTES];
        set_bytes.copy_from_slice(&src);
        let set_mask = u64::from_ne_bytes(set_bytes) & !unmaskable_signal_bits();
        let next_mask = match how {
            SIG_BLOCK_HOW => current_mask | set_mask,
            SIG_UNBLOCK_HOW => current_mask & !set_mask,
            SIG_SETMASK_HOW => set_mask,
            _ => return neg_errno(LinuxError::EINVAL),
        } & !unmaskable_signal_bits();
        if (current_mask | set_mask | next_mask) & signal_mask_bit(SIGCANCEL_NUM) != 0 {
            user_trace!(
                "sigdbg: rt_sigprocmask tid={} how={} set={set_mask:#x} old={current_mask:#x} new={next_mask:#x}",
                current_tid(),
                how,
            );
        }
        if next_mask == all_application_signal_mask() {
            // musl's fork path temporarily blocks every maskable signal before
            // cloning. A process child should not inherit that transient mask
            // as its long-lived runtime state, or default-fatal self-signals
            // can be delayed until the child exits normally.
            ext.fork_signal_mask_restore
                .store(current_mask, Ordering::Release);
        } else {
            ext.fork_signal_mask_restore
                .store(u64::MAX, Ordering::Release);
        }
        ext.signal_mask.store(next_mask, Ordering::Release);
        request_pending_default_terminate_signal(ext);
        if ext.process.pending_exit_group().is_some() {
            terminate_current_if_exit_group_pending(ext.process.as_ref());
        }
    }
    0
}

pub(super) fn sys_rt_sigpending(process: &UserProcess, set: usize, sigsetsize: usize) -> isize {
    let Some(ext) = current_task_ext() else {
        return neg_errno(LinuxError::EINVAL);
    };
    if set == 0 || sigsetsize < KERNEL_SIGSET_BYTES {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = clear_user_bytes(process, set, sigsetsize) {
        return neg_errno(err);
    }

    let pending_mask = pending_signal_mask(ext);
    write_user_bytes(process, set, &pending_mask.to_ne_bytes())
        .map(|()| 0)
        .unwrap_or_else(neg_errno)
}

pub(super) fn sys_rt_sigsuspend(process: &UserProcess, set: usize, sigsetsize: usize) -> isize {
    let Some(ext) = current_task_ext() else {
        return neg_errno(LinuxError::EINVAL);
    };
    if sigsetsize < KERNEL_SIGSET_BYTES {
        return neg_errno(LinuxError::EINVAL);
    }
    let src = match read_user_bytes(process, set, KERNEL_SIGSET_BYTES) {
        Ok(src) => src,
        Err(err) => return neg_errno(err),
    };
    let mut set_bytes = [0u8; KERNEL_SIGSET_BYTES];
    set_bytes.copy_from_slice(&src);
    let old_mask = ext.signal_mask.load(Ordering::Acquire);
    let suspend_mask = u64::from_ne_bytes(set_bytes) & !unmaskable_signal_bits();
    ext.sigsuspend_restore_mask
        .store(old_mask, Ordering::Release);
    ext.signal_mask.store(suspend_mask, Ordering::Release);
    enter_signal_wait(ext, 0);

    while !current_unblocked_signal_pending() {
        if let Some(code) = ext.process.pending_exit_group() {
            leave_signal_wait(ext);
            ext.sigsuspend_restore_mask
                .store(NO_SIGSUSPEND_RESTORE_MASK, Ordering::Release);
            ext.signal_mask.store(old_mask, Ordering::Release);
            terminate_current_thread_for_exit_group(ext.process.as_ref(), code);
        }
        let _ = ext.process.consume_expired_real_timer();
        if current_unblocked_signal_pending() {
            break;
        }
        let mut wait_slice = core::time::Duration::from_millis(10);
        if let Some(remaining) = ext.process.eval_watchdog_remaining() {
            wait_slice = wait_slice.min(remaining);
        }
        ext.process.timer_wait.wait_timeout_until(wait_slice, || {
            current_unblocked_signal_pending()
                || ext.process.pending_exit_group().is_some()
                || ext.process.eval_watchdog_expired()
        });
    }
    leave_signal_wait(ext);

    neg_errno(LinuxError::EINTR)
}

pub(super) fn sys_rt_sigtimedwait(
    process: &UserProcess,
    set: usize,
    info: usize,
    timeout: usize,
    sigsetsize: usize,
) -> isize {
    if set == 0 || sigsetsize < KERNEL_SIGSET_BYTES {
        return neg_errno(LinuxError::EINVAL);
    }
    let src = match read_user_bytes(process, set, KERNEL_SIGSET_BYTES) {
        Ok(src) => src,
        Err(err) => return neg_errno(err),
    };
    let mut set_bytes = [0u8; KERNEL_SIGSET_BYTES];
    set_bytes.copy_from_slice(&src);
    let wait_mask = u64::from_ne_bytes(set_bytes) & !unmaskable_signal_bits();

    let deadline_us = if timeout != 0 {
        let timeout = match read_user_value::<general::timespec>(process, timeout) {
            Ok(timeout) => timeout,
            Err(err) => return neg_errno(err),
        };
        if timeout.tv_sec < 0 || timeout.tv_nsec < 0 || timeout.tv_nsec >= 1_000_000_000 {
            return neg_errno(LinuxError::EINVAL);
        }
        let timeout_us = (timeout.tv_sec as u64)
            .saturating_mul(1_000_000)
            .saturating_add(((timeout.tv_nsec as u64).saturating_add(999)) / 1_000);
        Some(
            (axhal::time::monotonic_time()
                .as_micros()
                .min(u64::MAX as u128) as u64)
                .saturating_add(timeout_us),
        )
    } else {
        None
    };

    let Some(ext) = current_task_ext() else {
        return neg_errno(LinuxError::EINVAL);
    };
    enter_signal_wait(ext, wait_mask);
    loop {
        if let Some((sig, sender_pid)) = take_current_pending_signal_matching(wait_mask) {
            leave_signal_wait(ext);
            if info != 0 {
                let mut siginfo = [0u8; 128];
                siginfo[0..4].copy_from_slice(&sig.to_ne_bytes());
                siginfo[4..8].copy_from_slice(&0i32.to_ne_bytes());
                siginfo[8..12].copy_from_slice(&0i32.to_ne_bytes());
                siginfo[16..20].copy_from_slice(&sender_pid.to_ne_bytes());
                if let Err(err) = write_user_bytes(process, info, &siginfo) {
                    return neg_errno(err);
                }
            }
            return sig as isize;
        }

        if let Some(deadline_us) = deadline_us {
            let now_us = axhal::time::monotonic_time()
                .as_micros()
                .min(u64::MAX as u128) as u64;
            if now_us >= deadline_us {
                leave_signal_wait(ext);
                return neg_errno(LinuxError::EAGAIN);
            }
        }

        if let Some(code) = ext.process.pending_exit_group() {
            leave_signal_wait(ext);
            terminate_current_thread_for_exit_group(ext.process.as_ref(), code);
        }
        let _ = ext.process.consume_expired_real_timer();
        if ext.process.eval_watchdog_expired() {
            leave_signal_wait(ext);
            return neg_errno(LinuxError::EINTR);
        }
        if current_unblocked_signal_pending() && !current_pending_signal_matches(wait_mask) {
            leave_signal_wait(ext);
            return neg_errno(LinuxError::EINTR);
        }
        let mut wait_slice = core::time::Duration::from_millis(10);
        if let Some(deadline_us) = deadline_us {
            let now_us = axhal::time::monotonic_time()
                .as_micros()
                .min(u64::MAX as u128) as u64;
            let remaining_us = deadline_us.saturating_sub(now_us);
            if remaining_us == 0 {
                leave_signal_wait(ext);
                return neg_errno(LinuxError::EAGAIN);
            }
            wait_slice = wait_slice.min(core::time::Duration::from_micros(remaining_us));
        }
        if let Some(remaining) = ext.process.eval_watchdog_remaining() {
            wait_slice = wait_slice.min(remaining);
        }
        ext.process.timer_wait.wait_timeout_until(wait_slice, || {
            current_pending_signal_matches(wait_mask)
                || ext.process.pending_exit_group().is_some()
                || ext.process.eval_watchdog_expired()
                || (current_unblocked_signal_pending()
                    && !current_pending_signal_matches(wait_mask))
        });
    }
}

pub(super) fn sys_kill(process: &UserProcess, pid: i32, sig: i32) -> isize {
    if let Err(err) = validate_signal_target(sig) {
        return neg_errno(err);
    }
    if pid == 0 {
        return deliver_process_group_signal(process, process.pgid(), sig);
    }
    if pid < -1 {
        return deliver_process_group_signal(process, -pid, sig);
    }
    if pid == -1 {
        return neg_errno(LinuxError::EPERM);
    }
    if pid == process.pid() || pid == current_tid() {
        let Some(entry) = user_thread_entry_for_process(process) else {
            return neg_errno(LinuxError::ESRCH);
        };
        let ret = deliver_user_signal_result(&entry, sig, process.pid());
        let terminate_self = ret == 0 && process.pending_exit_group().is_some();
        // `terminate_current_if_exit_group_pending()` never returns.  Do not
        // keep `UserThreadEntry` (and therefore its AxTaskRef/UserProcess Arc)
        // live across that no-return edge, otherwise self-signals leave the
        // exited task permanently retained in the axtask GC queue.
        drop(entry);
        if terminate_self {
            terminate_current_if_exit_group_pending(process);
        }
        return ret;
    }
    let Some(entry) = process
        .child_thread_entry_by_pid(pid)
        .or_else(|| user_thread_entry_by_process_pid(pid))
    else {
        return neg_errno(LinuxError::ESRCH);
    };
    if !signal_permission_allowed(process, entry.process.as_ref(), sig) {
        return neg_errno(LinuxError::EPERM);
    }
    deliver_user_signal_result(&entry, sig, process.pid())
}

pub(super) fn sys_tkill(process: &UserProcess, tid: i32, sig: i32) -> isize {
    if tid <= 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_signal_target(sig) {
        return neg_errno(err);
    }
    let entry = match user_thread_entry_by_tid(tid) {
        Some(entry) => entry,
        None => return neg_errno(LinuxError::ESRCH),
    };
    if entry.process.pid() != process.pid() {
        return neg_errno(LinuxError::ESRCH);
    }
    if sig >= 32 {
        user_trace!(
            "sigdbg: tkill from tid={} to tid={tid} sig={sig}",
            current_tid()
        );
    }
    let ret = deliver_user_signal_result(&entry, sig, process.pid());
    let terminate_self = ret == 0 && tid == current_tid() && process.pending_exit_group().is_some();
    drop(entry);
    if terminate_self {
        terminate_current_if_exit_group_pending(process);
    }
    ret
}

pub(super) fn sys_tgkill(process: &UserProcess, tgid: i32, tid: i32, sig: i32) -> isize {
    if tgid <= 0 || tid <= 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_signal_target(sig) {
        return neg_errno(err);
    }
    let entry = match user_thread_entry_by_tid(tid) {
        Some(entry) => entry,
        None => return neg_errno(LinuxError::ESRCH),
    };
    if entry.process.pid() != process.pid() || entry.process.pid() != tgid {
        return neg_errno(LinuxError::ESRCH);
    }
    if sig >= 32 {
        user_trace!(
            "sigdbg: tgkill from tid={} tgid={} to tid={tid} sig={sig}",
            current_tid(),
            tgid,
        );
    }
    if realtime_signal_queue_blocked_by_rlimit(&entry, sig) {
        return neg_errno(LinuxError::EAGAIN);
    }
    let ret = deliver_user_signal_result(&entry, sig, process.pid());
    let terminate_self = ret == 0 && tid == current_tid() && process.pending_exit_group().is_some();
    drop(entry);
    if terminate_self {
        terminate_current_if_exit_group_pending(process);
    }
    ret
}

pub(super) fn sys_pidfd_send_signal(
    process: &UserProcess,
    pidfd: i32,
    sig: i32,
    info: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_signal_target(sig) {
        return neg_errno(err);
    }

    let entry = match process.fds.lock().pidfd_signal_target(process, pidfd) {
        Ok(entry) => entry,
        Err(err) => return neg_errno(err),
    };
    if !signal_permission_allowed(process, entry.process.as_ref(), sig) {
        return neg_errno(LinuxError::EPERM);
    }
    if sig == 0 {
        return 0;
    }

    let result = if info != 0 {
        let siginfo = match read_user_bytes(process, info, 128) {
            Ok(bytes) => bytes,
            Err(err) => return neg_errno(err),
        };
        let si_signo = i32::from_ne_bytes(siginfo[0..4].try_into().unwrap());
        if si_signo != sig {
            return neg_errno(LinuxError::EINVAL);
        }
        let si_code = i32::from_ne_bytes(siginfo[8..12].try_into().unwrap());
        let si_pid = i32::from_ne_bytes(siginfo[16..20].try_into().unwrap());
        let si_uid = u32::from_ne_bytes(siginfo[20..24].try_into().unwrap());
        let si_value = i32::from_ne_bytes(siginfo[24..28].try_into().unwrap());
        deliver_user_signal_with_siginfo(&entry, sig, si_pid, si_code, si_uid, si_value)
    } else {
        deliver_user_signal(&entry, sig, process.pid())
    };
    result.map_or_else(neg_errno, |_| 0)
}

#[cfg(target_arch = "riscv64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct RiscvSignalInfo {
    pub(super) bytes: [u8; 128],
}

#[cfg(target_arch = "riscv64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct RiscvSignalStack {
    pub(super) sp: usize,
    pub(super) stack_flags: i32,
    pub(super) stack_pad: i32,
    pub(super) size: usize,
}

#[cfg(target_arch = "riscv64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct RiscvKernelSigset {
    pub(super) sig: [u64; 1],
    pub(super) reserved: [u8; RISCV_SIGNAL_SIGSET_RESERVED_BYTES],
}

#[cfg(target_arch = "riscv64")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub(super) struct RiscvSignalFpState {
    pub(super) bytes: [u8; RISCV_SIGNAL_FPSTATE_BYTES],
}

#[cfg(target_arch = "riscv64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct RiscvSignalSigcontext {
    pub(super) gregs: [usize; 32],
    pub(super) fpstate: RiscvSignalFpState,
}

#[cfg(target_arch = "riscv64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct RiscvSignalUcontext {
    pub(super) flags: usize,
    pub(super) link: usize,
    pub(super) stack: RiscvSignalStack,
    pub(super) sigmask: RiscvKernelSigset,
    pub(super) mcontext: RiscvSignalSigcontext,
}

#[cfg(target_arch = "riscv64")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub(super) struct RiscvSignalFrame {
    pub(super) info: RiscvSignalInfo,
    pub(super) ucontext: RiscvSignalUcontext,
    pub(super) trampoline: [u32; 3],
}

#[cfg(target_arch = "riscv64")]
pub(super) struct RiscvSignalFrameOffsets {
    pub(super) info: usize,
    pub(super) ucontext: usize,
    pub(super) trampoline: usize,
}

#[cfg(target_arch = "riscv64")]
pub(super) fn riscv_signal_frame_size() -> usize {
    size_of::<RiscvSignalFrame>()
}

#[cfg(target_arch = "riscv64")]
pub(super) fn riscv_signal_frame_offsets() -> RiscvSignalFrameOffsets {
    RiscvSignalFrameOffsets {
        info: offset_of!(RiscvSignalFrame, info),
        ucontext: offset_of!(RiscvSignalFrame, ucontext),
        trampoline: offset_of!(RiscvSignalFrame, trampoline),
    }
}

#[cfg(target_arch = "riscv64")]
pub(super) fn trap_frame_to_riscv_sigcontext(tf: &TrapFrame) -> RiscvSignalSigcontext {
    RiscvSignalSigcontext {
        gregs: [
            tf.sepc,
            tf.regs.ra,
            tf.regs.sp,
            tf.regs.gp,
            tf.regs.tp,
            tf.regs.t0,
            tf.regs.t1,
            tf.regs.t2,
            tf.regs.s0,
            tf.regs.s1,
            tf.regs.a0,
            tf.regs.a1,
            tf.regs.a2,
            tf.regs.a3,
            tf.regs.a4,
            tf.regs.a5,
            tf.regs.a6,
            tf.regs.a7,
            tf.regs.s2,
            tf.regs.s3,
            tf.regs.s4,
            tf.regs.s5,
            tf.regs.s6,
            tf.regs.s7,
            tf.regs.s8,
            tf.regs.s9,
            tf.regs.s10,
            tf.regs.s11,
            tf.regs.t3,
            tf.regs.t4,
            tf.regs.t5,
            tf.regs.t6,
        ],
        fpstate: RiscvSignalFpState {
            bytes: [0; RISCV_SIGNAL_FPSTATE_BYTES],
        },
    }
}

#[cfg(target_arch = "riscv64")]
pub(super) fn apply_riscv_sigcontext(tf: &mut TrapFrame, sigcontext: &RiscvSignalSigcontext) {
    tf.sepc = sigcontext.gregs[0];
    tf.regs.zero = 0;
    tf.regs.ra = sigcontext.gregs[1];
    tf.regs.sp = sigcontext.gregs[2];
    tf.regs.gp = sigcontext.gregs[3];
    tf.regs.tp = sigcontext.gregs[4];
    tf.regs.t0 = sigcontext.gregs[5];
    tf.regs.t1 = sigcontext.gregs[6];
    tf.regs.t2 = sigcontext.gregs[7];
    tf.regs.s0 = sigcontext.gregs[8];
    tf.regs.s1 = sigcontext.gregs[9];
    tf.regs.a0 = sigcontext.gregs[10];
    tf.regs.a1 = sigcontext.gregs[11];
    tf.regs.a2 = sigcontext.gregs[12];
    tf.regs.a3 = sigcontext.gregs[13];
    tf.regs.a4 = sigcontext.gregs[14];
    tf.regs.a5 = sigcontext.gregs[15];
    tf.regs.a6 = sigcontext.gregs[16];
    tf.regs.a7 = sigcontext.gregs[17];
    tf.regs.s2 = sigcontext.gregs[18];
    tf.regs.s3 = sigcontext.gregs[19];
    tf.regs.s4 = sigcontext.gregs[20];
    tf.regs.s5 = sigcontext.gregs[21];
    tf.regs.s6 = sigcontext.gregs[22];
    tf.regs.s7 = sigcontext.gregs[23];
    tf.regs.s8 = sigcontext.gregs[24];
    tf.regs.s9 = sigcontext.gregs[25];
    tf.regs.s10 = sigcontext.gregs[26];
    tf.regs.s11 = sigcontext.gregs[27];
    tf.regs.t3 = sigcontext.gregs[28];
    tf.regs.t4 = sigcontext.gregs[29];
    tf.regs.t5 = sigcontext.gregs[30];
    tf.regs.t6 = sigcontext.gregs[31];
}

#[cfg(target_arch = "riscv64")]
fn make_riscv_siginfo(sig: i32, code: i32, tid: i32, uid: u32, value: usize) -> RiscvSignalInfo {
    let mut info = RiscvSignalInfo { bytes: [0; 128] };
    info.bytes[0..4].copy_from_slice(&sig.to_ne_bytes());
    info.bytes[4..8].copy_from_slice(&0i32.to_ne_bytes());
    info.bytes[8..12].copy_from_slice(&code.to_ne_bytes());
    info.bytes[16..20].copy_from_slice(&tid.to_ne_bytes());
    info.bytes[20..24].copy_from_slice(&uid.to_ne_bytes());
    info.bytes[24..32].copy_from_slice(&value.to_ne_bytes());
    info
}

#[cfg(target_arch = "riscv64")]
pub(super) fn make_riscv_signal_frame(
    sig: i32,
    code: i32,
    tid: i32,
    uid: u32,
    value: usize,
    current_mask: u64,
    stack_sp: usize,
    stack_flags: i32,
    stack_size: usize,
    trampoline: [u32; 3],
    mcontext: RiscvSignalSigcontext,
) -> RiscvSignalFrame {
    RiscvSignalFrame {
        info: make_riscv_siginfo(sig, code, tid, uid, value),
        ucontext: RiscvSignalUcontext {
            flags: 0,
            link: 0,
            stack: RiscvSignalStack {
                sp: stack_sp,
                stack_flags,
                stack_pad: 0,
                size: stack_size,
            },
            sigmask: RiscvKernelSigset {
                sig: [current_mask],
                reserved: [0; RISCV_SIGNAL_SIGSET_RESERVED_BYTES],
            },
            mcontext,
        },
        trampoline,
    }
}

#[cfg(target_arch = "loongarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct LoongArchSignalInfo {
    pub(super) bytes: [u8; 128],
}

#[cfg(target_arch = "loongarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct LoongArchSignalStack {
    pub(super) sp: usize,
    pub(super) stack_flags: i32,
    pub(super) stack_pad: i32,
    pub(super) size: usize,
}

#[cfg(target_arch = "loongarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct LoongArchKernelSigset {
    pub(super) sig: [u64; 1],
    pub(super) reserved: [u8; 120],
}

#[cfg(target_arch = "loongarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct LoongArchSigcontext {
    pub(super) pc: usize,
    pub(super) gregs: [usize; 32],
}

#[cfg(target_arch = "loongarch64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct LoongArchUcontext {
    pub(super) flags: usize,
    pub(super) link: usize,
    pub(super) stack: LoongArchSignalStack,
    pub(super) sigmask: LoongArchKernelSigset,
    pub(super) mcontext: LoongArchSigcontext,
}

#[cfg(target_arch = "loongarch64")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub(super) struct LoongArchSignalFrame {
    pub(super) info: LoongArchSignalInfo,
    pub(super) ucontext: LoongArchUcontext,
    pub(super) trampoline: [u32; 3],
}

#[cfg(target_arch = "loongarch64")]
pub(super) struct LoongArchSignalFrameOffsets {
    pub(super) info: usize,
    pub(super) ucontext: usize,
    pub(super) trampoline: usize,
}

#[cfg(target_arch = "loongarch64")]
pub(super) fn loongarch_signal_frame_size() -> usize {
    size_of::<LoongArchSignalFrame>()
}

#[cfg(target_arch = "loongarch64")]
pub(super) fn loongarch_signal_frame_offsets() -> LoongArchSignalFrameOffsets {
    LoongArchSignalFrameOffsets {
        info: offset_of!(LoongArchSignalFrame, info),
        ucontext: offset_of!(LoongArchSignalFrame, ucontext),
        trampoline: offset_of!(LoongArchSignalFrame, trampoline),
    }
}

#[cfg(target_arch = "loongarch64")]
pub(super) fn trap_frame_to_loongarch_sigcontext(tf: &TrapFrame) -> LoongArchSigcontext {
    let mut gregs = [0usize; 32];
    gregs[1] = tf.regs.ra;
    gregs[2] = tf.regs.tp;
    gregs[3] = tf.regs.sp;
    gregs[4] = tf.regs.a0;
    gregs[5] = tf.regs.a1;
    gregs[6] = tf.regs.a2;
    gregs[7] = tf.regs.a3;
    gregs[8] = tf.regs.a4;
    gregs[9] = tf.regs.a5;
    gregs[10] = tf.regs.a6;
    gregs[11] = tf.regs.a7;
    gregs[12] = tf.regs.t0;
    gregs[13] = tf.regs.t1;
    gregs[14] = tf.regs.t2;
    gregs[15] = tf.regs.t3;
    gregs[16] = tf.regs.t4;
    gregs[17] = tf.regs.t5;
    gregs[18] = tf.regs.t6;
    gregs[19] = tf.regs.t7;
    gregs[20] = tf.regs.t8;
    gregs[21] = tf.regs.u0;
    gregs[22] = tf.regs.fp;
    gregs[23] = tf.regs.s0;
    gregs[24] = tf.regs.s1;
    gregs[25] = tf.regs.s2;
    gregs[26] = tf.regs.s3;
    gregs[27] = tf.regs.s4;
    gregs[28] = tf.regs.s5;
    gregs[29] = tf.regs.s6;
    gregs[30] = tf.regs.s7;
    gregs[31] = tf.regs.s8;
    LoongArchSigcontext { pc: tf.era, gregs }
}

#[cfg(target_arch = "loongarch64")]
pub(super) fn apply_loongarch_sigcontext(tf: &mut TrapFrame, sigcontext: &LoongArchSigcontext) {
    tf.era = sigcontext.pc;
    tf.regs.zero = 0;
    tf.regs.ra = sigcontext.gregs[1];
    tf.regs.tp = sigcontext.gregs[2];
    tf.regs.sp = sigcontext.gregs[3];
    tf.regs.a0 = sigcontext.gregs[4];
    tf.regs.a1 = sigcontext.gregs[5];
    tf.regs.a2 = sigcontext.gregs[6];
    tf.regs.a3 = sigcontext.gregs[7];
    tf.regs.a4 = sigcontext.gregs[8];
    tf.regs.a5 = sigcontext.gregs[9];
    tf.regs.a6 = sigcontext.gregs[10];
    tf.regs.a7 = sigcontext.gregs[11];
    tf.regs.t0 = sigcontext.gregs[12];
    tf.regs.t1 = sigcontext.gregs[13];
    tf.regs.t2 = sigcontext.gregs[14];
    tf.regs.t3 = sigcontext.gregs[15];
    tf.regs.t4 = sigcontext.gregs[16];
    tf.regs.t5 = sigcontext.gregs[17];
    tf.regs.t6 = sigcontext.gregs[18];
    tf.regs.t7 = sigcontext.gregs[19];
    tf.regs.t8 = sigcontext.gregs[20];
    tf.regs.u0 = sigcontext.gregs[21];
    tf.regs.fp = sigcontext.gregs[22];
    tf.regs.s0 = sigcontext.gregs[23];
    tf.regs.s1 = sigcontext.gregs[24];
    tf.regs.s2 = sigcontext.gregs[25];
    tf.regs.s3 = sigcontext.gregs[26];
    tf.regs.s4 = sigcontext.gregs[27];
    tf.regs.s5 = sigcontext.gregs[28];
    tf.regs.s6 = sigcontext.gregs[29];
    tf.regs.s7 = sigcontext.gregs[30];
    tf.regs.s8 = sigcontext.gregs[31];
}

#[cfg(target_arch = "loongarch64")]
fn make_loongarch_siginfo(
    sig: i32,
    code: i32,
    tid: i32,
    uid: u32,
    value: usize,
) -> LoongArchSignalInfo {
    let mut info = LoongArchSignalInfo { bytes: [0; 128] };
    info.bytes[0..4].copy_from_slice(&sig.to_ne_bytes());
    info.bytes[4..8].copy_from_slice(&0i32.to_ne_bytes());
    info.bytes[8..12].copy_from_slice(&code.to_ne_bytes());
    info.bytes[16..20].copy_from_slice(&tid.to_ne_bytes());
    info.bytes[20..24].copy_from_slice(&uid.to_ne_bytes());
    info.bytes[24..32].copy_from_slice(&value.to_ne_bytes());
    info
}

#[cfg(target_arch = "loongarch64")]
pub(super) fn make_loongarch_signal_frame(
    sig: i32,
    code: i32,
    tid: i32,
    uid: u32,
    value: usize,
    current_mask: u64,
    stack_sp: usize,
    stack_flags: i32,
    stack_size: usize,
    trampoline: [u32; 3],
    mcontext: LoongArchSigcontext,
) -> LoongArchSignalFrame {
    LoongArchSignalFrame {
        info: make_loongarch_siginfo(sig, code, tid, uid, value),
        ucontext: LoongArchUcontext {
            flags: 0,
            link: 0,
            stack: LoongArchSignalStack {
                sp: stack_sp,
                stack_flags,
                stack_pad: 0,
                size: stack_size,
            },
            sigmask: LoongArchKernelSigset {
                sig: [current_mask],
                reserved: [0; 120],
            },
            mcontext,
        },
        trampoline,
    }
}

#[cfg(target_arch = "riscv64")]
const _: [(); RISCV_SIGNAL_FPSTATE_BYTES] = [(); size_of::<RiscvSignalFpState>()];
#[cfg(target_arch = "riscv64")]
const _: [(); 784] = [(); size_of::<RiscvSignalSigcontext>()];
#[cfg(target_arch = "riscv64")]
const _: [(); 960] = [(); size_of::<RiscvSignalUcontext>()];
#[cfg(target_arch = "riscv64")]
const _: [(); 1104] = [(); size_of::<RiscvSignalFrame>()];
