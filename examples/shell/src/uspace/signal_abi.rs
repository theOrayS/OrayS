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
    SIGCANCEL_NUM, SIGCHLD_NUM, SIGFPE_NUM, SIGILL_NUM, SIGINT_NUM, SIGKILL_NUM, SIGPIPE_NUM,
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

macro_rules! user_trace {
    ($($arg:tt)*) => {};
}

const NO_SIGSUSPEND_RESTORE_MASK: u64 = u64::MAX;

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
    matches!(
        sig,
        SIGINT_NUM
            | SIGQUIT_NUM
            | SIGILL_NUM
            | SIGABRT_NUM
            | SIGFPE_NUM
            | SIGKILL_NUM
            | SIGSEGV_NUM
            | SIGPIPE_NUM
            | SIGALRM_NUM
            | SIGTERM_NUM
    )
}

pub(super) fn signal_is_blocked(ext: &UserTaskExt, sig: i32) -> bool {
    if sig == SIGKILL_NUM {
        return false;
    }
    let bit = signal_mask_bit(sig);
    bit != 0 && ext.signal_mask.load(Ordering::Acquire) & bit != 0
}

pub(super) fn current_sigcancel_pending() -> bool {
    current_task_ext().is_some_and(|ext| {
        ext.pending_signal.load(Ordering::Acquire) == SIGCANCEL_NUM
            && !signal_is_blocked(ext, SIGCANCEL_NUM)
    })
}

pub(super) fn current_unblocked_signal_pending() -> bool {
    current_task_ext().is_some_and(|ext| {
        let sig = ext.pending_signal.load(Ordering::Acquire);
        sig != 0 && !signal_is_blocked(ext, sig)
    })
}

pub(super) fn deliver_user_signal(
    entry: &UserThreadEntry,
    sig: i32,
    sender_pid: i32,
) -> Result<(), LinuxError> {
    if sig == 0 {
        return Ok(());
    }
    let ext = super::task_context::task_ext(&entry.task).ok_or(LinuxError::ESRCH)?;
    if sig == SIGKILL_NUM {
        ext.process.request_exit_group(128 + sig);
    }
    ext.pending_signal_sender
        .store(sender_pid, Ordering::Release);
    ext.pending_signal.store(sig, Ordering::Release);
    if sig == SIGCANCEL_NUM {
        user_trace!(
            "sigdbg: deliver tid={} blocked={} futex_wait={:#x}",
            entry.task.id().as_u64(),
            signal_is_blocked(ext, sig),
            ext.futex_wait.load(Ordering::Acquire),
        );
    }
    if sig == SIGCANCEL_NUM && !signal_is_blocked(ext, sig) {
        let futex_wait = ext.futex_wait.load(Ordering::Acquire);
        if futex_wait != 0 {
            futex::wake_task(futex_wait, &entry.task);
        }
    }
    Ok(())
}

fn deliver_user_signal_result(entry: &UserThreadEntry, sig: i32, sender_pid: i32) -> isize {
    match deliver_user_signal(entry, sig, sender_pid) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

fn deliver_process_group_signal(pgid: i32, sig: i32, sender_pid: i32) -> isize {
    let entries = user_thread_entries_by_process_group(pgid);
    if entries.is_empty() {
        return neg_errno(LinuxError::ESRCH);
    }
    for entry in entries {
        if let Err(err) = deliver_user_signal(&entry, sig, sender_pid) {
            return neg_errno(err);
        }
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
    ext.process.request_exit_group(128 + signal);
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
        ext.process.request_exit_group(137);
        terminate_current_thread_for_exit_group(ext.process.as_ref(), 137);
    }
    if let Some(code) = ext.process.pending_exit_group() {
        terminate_current_thread_for_exit_group(ext.process.as_ref(), code);
    }
    if ext.pending_signal.load(Ordering::Acquire) == SIGCANCEL_NUM
        && !signal_is_blocked(ext, SIGCANCEL_NUM)
    {
        ext.pending_signal.store(0, Ordering::Release);
        terminate_current_thread(ext.process.as_ref(), 0);
    }
    if ext.signal_frame.load(Ordering::Acquire) == 0 {
        if let Some(restored) = ext.pending_sigreturn.lock().take() {
            *tf = restored;
            return;
        }
    }
    let _ = ext.process.consume_expired_real_timer();
    #[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
    if ext.signal_frame.load(Ordering::Acquire) == 0 {
        let sig = ext.pending_signal.load(Ordering::Acquire);
        if sig != 0 && !signal_is_blocked(ext, sig) {
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
        ext.pending_signal.store(0, Ordering::Release);
        let current_mask = ext.signal_mask.load(Ordering::Acquire);
        let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
        if handler == 0 && default_signal_terminates(sig) {
            ext.process.request_exit_group(128 + sig);
            terminate_current_thread_for_exit_group(ext.process.as_ref(), 128 + sig);
        }
        ext.signal_mask.store(restore_mask, Ordering::Release);
        return Ok(());
    }
    let current_mask = ext.signal_mask.load(Ordering::Acquire);
    let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
    let frame_size = riscv_signal_frame_size();
    let frame_addr = align_down(tf.regs.sp.saturating_sub(frame_size), 16);
    ensure_signal_frame_pages(ext.process.as_ref(), frame_addr, frame_size)?;

    let sender_pid = ext.pending_signal_sender.load(Ordering::Acquire);
    let frame = make_riscv_signal_frame(
        sig,
        SI_TKILL_CODE,
        sender_pid,
        restore_mask,
        SS_DISABLE,
        RISCV_SIGTRAMP_CODE,
        trap_frame_to_riscv_sigcontext(tf),
    );

    let frame_ret = write_user_value(ext.process.as_ref(), frame_addr, &frame);
    if frame_ret != 0 {
        return Err(LinuxError::EFAULT);
    }

    *ext.pending_sigreturn.lock() = Some(*tf);
    ext.signal_frame.store(frame_addr, Ordering::Release);
    ext.pending_signal.store(0, Ordering::Release);
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
        ext.pending_signal.store(0, Ordering::Release);
        let current_mask = ext.signal_mask.load(Ordering::Acquire);
        let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
        if handler == 0 && default_signal_terminates(sig) {
            ext.process.request_exit_group(128 + sig);
            terminate_current_thread_for_exit_group(ext.process.as_ref(), 128 + sig);
        }
        ext.signal_mask.store(restore_mask, Ordering::Release);
        return Ok(());
    }

    let current_mask = ext.signal_mask.load(Ordering::Acquire);
    let restore_mask = take_sigsuspend_restore_mask(ext, current_mask);
    let frame_size = loongarch_signal_frame_size();
    let frame_addr = align_down(tf.regs.sp.saturating_sub(frame_size), 16);
    ensure_signal_frame_pages(ext.process.as_ref(), frame_addr, frame_size)?;

    let sender_pid = ext.pending_signal_sender.load(Ordering::Acquire);
    let frame = make_loongarch_signal_frame(
        sig,
        SI_TKILL_CODE,
        sender_pid,
        restore_mask,
        SS_DISABLE,
        LOONGARCH_SIGTRAMP_CODE,
        trap_frame_to_loongarch_sigcontext(tf),
    );
    let frame_ret = write_user_value(ext.process.as_ref(), frame_addr, &frame);
    if frame_ret != 0 {
        return Err(LinuxError::EFAULT);
    }

    *ext.pending_sigreturn.lock() = Some(*tf);
    ext.signal_frame.store(frame_addr, Ordering::Release);
    ext.pending_signal.store(0, Ordering::Release);
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
    _sigsetsize: usize,
) -> isize {
    if signum == 0 || signum >= 65 {
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
    }

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
        if ext.pending_signal.load(Ordering::Acquire) == 0 {
            user_trace!(
                "sigdbg: rt_sigreturn tid={} frame={frame_addr:#x} restore_sp={:#x} restore_tp={:#x} restore_pc={:#x}",
                current_tid(),
                restored.regs.sp,
                restored.regs.tp,
                restored.sepc,
            );
        }
        ext.signal_frame.store(0, Ordering::Release);
        *ext.pending_sigreturn.lock() = Some(restored);
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
        *ext.pending_sigreturn.lock() = Some(restored);
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
        ext.signal_mask.store(next_mask, Ordering::Release);
    }
    0
}

pub(super) fn sys_rt_sigpending(
    process: &UserProcess,
    set: usize,
    sigsetsize: usize,
) -> isize {
    let Some(ext) = current_task_ext() else {
        return neg_errno(LinuxError::EINVAL);
    };
    if set == 0 || sigsetsize < KERNEL_SIGSET_BYTES {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = clear_user_bytes(process, set, sigsetsize) {
        return neg_errno(err);
    }

    let pending_sig = ext.pending_signal.load(Ordering::Acquire);
    let pending_mask = signal_mask_bit(pending_sig);
    write_user_bytes(process, set, &pending_mask.to_ne_bytes())
        .map(|()| 0)
        .unwrap_or_else(neg_errno)
}

pub(super) fn sys_rt_sigsuspend(
    process: &UserProcess,
    set: usize,
    sigsetsize: usize,
) -> isize {
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

    while !current_unblocked_signal_pending() {
        if let Some(code) = ext.process.pending_exit_group() {
            ext.sigsuspend_restore_mask
                .store(NO_SIGSUSPEND_RESTORE_MASK, Ordering::Release);
            ext.signal_mask.store(old_mask, Ordering::Release);
            terminate_current_thread_for_exit_group(ext.process.as_ref(), code);
        }
        let _ = ext.process.consume_expired_real_timer();
        if current_unblocked_signal_pending() {
            break;
        }
        axtask::yield_now();
    }

    neg_errno(LinuxError::EINTR)
}

pub(super) fn sys_rt_sigtimedwait(
    process: &UserProcess,
    _set: usize,
    info: usize,
    timeout: usize,
    _sigsetsize: usize,
) -> isize {
    if timeout != 0 {
        if let Err(err) = read_user_value::<general::timespec>(process, timeout) {
            return neg_errno(err);
        }
    }
    if info != 0 {
        if let Err(err) = clear_user_bytes(process, info, 128) {
            return neg_errno(err);
        }
    }
    SIGCHLD_NUM
}

pub(super) fn sys_kill(process: &UserProcess, pid: i32, sig: i32) -> isize {
    if let Err(err) = validate_signal_target(sig) {
        return neg_errno(err);
    }
    if pid == 0 {
        return deliver_process_group_signal(process.pgid(), sig, process.pid());
    }
    if pid < -1 {
        return deliver_process_group_signal(-pid, sig, process.pid());
    }
    if pid == -1 {
        return neg_errno(LinuxError::EPERM);
    }
    if pid == process.pid() || pid == current_tid() {
        let Some(entry) = user_thread_entry_for_process(process) else {
            return neg_errno(LinuxError::ESRCH);
        };
        return deliver_user_signal_result(&entry, sig, process.pid());
    }
    let Some(entry) = process
        .child_thread_entry_by_pid(pid)
        .or_else(|| user_thread_entry_by_process_pid(pid))
    else {
        return neg_errno(LinuxError::ESRCH);
    };
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
    deliver_user_signal_result(&entry, sig, process.pid())
}

pub(super) fn sys_tgkill(process: &UserProcess, tgid: i32, tid: i32, sig: i32) -> isize {
    if tgid <= 0 || tid <= 0 {
        return neg_errno(LinuxError::EINVAL);
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
    deliver_user_signal_result(&entry, sig, process.pid())
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
fn make_riscv_siginfo(sig: i32, code: i32, tid: i32) -> RiscvSignalInfo {
    let mut info = RiscvSignalInfo { bytes: [0; 128] };
    info.bytes[0..4].copy_from_slice(&sig.to_ne_bytes());
    info.bytes[4..8].copy_from_slice(&0i32.to_ne_bytes());
    info.bytes[8..12].copy_from_slice(&code.to_ne_bytes());
    info.bytes[16..20].copy_from_slice(&tid.to_ne_bytes());
    info.bytes[20..24].copy_from_slice(&0u32.to_ne_bytes());
    info
}

#[cfg(target_arch = "riscv64")]
pub(super) fn make_riscv_signal_frame(
    sig: i32,
    code: i32,
    tid: i32,
    current_mask: u64,
    stack_flags: i32,
    trampoline: [u32; 3],
    mcontext: RiscvSignalSigcontext,
) -> RiscvSignalFrame {
    RiscvSignalFrame {
        info: make_riscv_siginfo(sig, code, tid),
        ucontext: RiscvSignalUcontext {
            flags: 0,
            link: 0,
            stack: RiscvSignalStack {
                sp: 0,
                stack_flags,
                stack_pad: 0,
                size: 0,
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
fn make_loongarch_siginfo(sig: i32, code: i32, tid: i32) -> LoongArchSignalInfo {
    let mut info = LoongArchSignalInfo { bytes: [0; 128] };
    info.bytes[0..4].copy_from_slice(&sig.to_ne_bytes());
    info.bytes[4..8].copy_from_slice(&0i32.to_ne_bytes());
    info.bytes[8..12].copy_from_slice(&code.to_ne_bytes());
    info.bytes[16..20].copy_from_slice(&tid.to_ne_bytes());
    info.bytes[20..24].copy_from_slice(&0u32.to_ne_bytes());
    info
}

#[cfg(target_arch = "loongarch64")]
pub(super) fn make_loongarch_signal_frame(
    sig: i32,
    code: i32,
    tid: i32,
    current_mask: u64,
    stack_flags: i32,
    trampoline: [u32; 3],
    mcontext: LoongArchSigcontext,
) -> LoongArchSignalFrame {
    LoongArchSignalFrame {
        info: make_loongarch_siginfo(sig, code, tid),
        ucontext: LoongArchUcontext {
            flags: 0,
            link: 0,
            stack: LoongArchSignalStack {
                sp: 0,
                stack_flags,
                stack_pad: 0,
                size: 0,
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
