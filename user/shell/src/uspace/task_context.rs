use core::mem::size_of;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering};

use axerrno::LinuxError;
use axhal::context::{TrapFrame, UspaceContext};
use axsync::Mutex;
use axtask::AxTaskRef;
use std::sync::Arc;

#[cfg(target_arch = "riscv64")]
use riscv::register::sstatus::{FS, Sstatus};

use super::UserProcess;
use super::linux_abi::neg_errno;
use super::task_registry::user_thread_entry_by_tid;
use super::user_memory::write_user_value;

// Linux validates set_robust_list(2) against the userspace robust_list_head
// layout. Both supported 64-bit userspace ABIs here use a header of three
// pointer-sized words: list head pointer, futex offset, and pending-list pointer.
const ROBUST_LIST_HEAD_LEN: usize = size_of::<usize>() * 3;
const SYNTHETIC_INIT_PID: i32 = 1;

#[cfg(feature = "auto-run-tests")]
static USER_TASK_EXT_LIVE: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "auto-run-tests")]
static USER_TASK_EXT_CREATED: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "auto-run-tests")]
static USER_TASK_EXT_DROPPED: AtomicUsize = AtomicUsize::new(0);

pub(super) struct UserTaskExt {
    pub(super) process: Arc<UserProcess>,
    pub(super) initial_context: Mutex<Option<UspaceContext>>,
    pub(super) clear_child_tid: AtomicUsize,
    pub(super) pending_signal_mask: AtomicU64,
    pub(super) pending_signal: AtomicI32,
    pub(super) pending_signal_sender: AtomicI32,
    pub(super) pending_signal_code: AtomicI32,
    pub(super) pending_signal_uid: AtomicU32,
    pub(super) pending_signal_value: AtomicUsize,
    pub(super) signal_mask: AtomicU64,
    /// Mask to restore for fork-like children when libc temporarily blocks all
    /// maskable signals around fork. `u64::MAX` means no restore is pending.
    pub(super) fork_signal_mask_restore: AtomicU64,
    pub(super) sigsuspend_restore_mask: AtomicU64,
    pub(super) signal_wait: AtomicBool,
    pub(super) signal_wait_mask: AtomicU64,
    pub(super) poll_wait: AtomicBool,
    pub(super) futex_wait: AtomicUsize,
    pub(super) futex_bitset: AtomicU32,
    pub(super) robust_list_head: AtomicUsize,
    pub(super) robust_list_len: AtomicUsize,
    pub(super) deferred_unmap_start: AtomicUsize,
    pub(super) deferred_unmap_len: AtomicUsize,
    pub(super) sigaltstack_sp: AtomicUsize,
    pub(super) sigaltstack_flags: AtomicI32,
    pub(super) sigaltstack_size: AtomicU64,
    pub(super) signal_frame: AtomicUsize,
    pub(super) last_user_pc: AtomicUsize,
    pub(super) pending_sigreturn: Mutex<Option<TrapFrame>>,
    // Keep syscall restart state behind the helper methods below.  The bool is
    // the hot-path predicate that lets ordinary syscalls avoid taking the
    // mutex; the mutex remains the owner of the actual trap frame.
    syscall_restart_frame_valid: AtomicBool,
    syscall_restart_frame: Mutex<Option<TrapFrame>>,
    pub(super) syscall_runtime_micros: AtomicU64,
    pub(super) last_reported_user_micros: AtomicU64,
    pub(super) last_reported_system_micros: AtomicU64,
}

impl UserTaskExt {
    pub(super) fn new(
        process: Arc<UserProcess>,
        initial_context: UspaceContext,
        clear_child_tid: usize,
        signal_mask: u64,
    ) -> Self {
        #[cfg(feature = "auto-run-tests")]
        {
            USER_TASK_EXT_LIVE.fetch_add(1, Ordering::AcqRel);
            USER_TASK_EXT_CREATED.fetch_add(1, Ordering::AcqRel);
        }
        Self {
            process,
            initial_context: Mutex::new(Some(initial_context)),
            clear_child_tid: AtomicUsize::new(clear_child_tid),
            pending_signal_mask: AtomicU64::new(0),
            pending_signal: AtomicI32::new(0),
            pending_signal_sender: AtomicI32::new(0),
            pending_signal_code: AtomicI32::new(super::linux_abi::SI_TKILL_CODE),
            pending_signal_uid: AtomicU32::new(0),
            pending_signal_value: AtomicUsize::new(0),
            signal_mask: AtomicU64::new(signal_mask),
            fork_signal_mask_restore: AtomicU64::new(u64::MAX),
            sigsuspend_restore_mask: AtomicU64::new(u64::MAX),
            signal_wait: AtomicBool::new(false),
            signal_wait_mask: AtomicU64::new(0),
            poll_wait: AtomicBool::new(false),
            futex_wait: AtomicUsize::new(0),
            futex_bitset: AtomicU32::new(0),
            robust_list_head: AtomicUsize::new(0),
            robust_list_len: AtomicUsize::new(0),
            deferred_unmap_start: AtomicUsize::new(0),
            deferred_unmap_len: AtomicUsize::new(0),
            sigaltstack_sp: AtomicUsize::new(0),
            sigaltstack_flags: AtomicI32::new(linux_raw_sys::general::SS_DISABLE as i32),
            sigaltstack_size: AtomicU64::new(0),
            signal_frame: AtomicUsize::new(0),
            last_user_pc: AtomicUsize::new(0),
            pending_sigreturn: Mutex::new(None),
            syscall_restart_frame_valid: AtomicBool::new(false),
            syscall_restart_frame: Mutex::new(None),
            syscall_runtime_micros: AtomicU64::new(0),
            last_reported_user_micros: AtomicU64::new(0),
            last_reported_system_micros: AtomicU64::new(0),
        }
    }

    pub(super) fn store_syscall_restart_frame(&self, frame: TrapFrame) {
        *self.syscall_restart_frame.lock() = Some(frame);
        self.syscall_restart_frame_valid
            .store(true, Ordering::Release);
    }

    pub(super) fn take_syscall_restart_frame(&self) -> Option<TrapFrame> {
        self.syscall_restart_frame_valid
            .swap(false, Ordering::AcqRel)
            .then(|| self.syscall_restart_frame.lock().take())
            .flatten()
    }

    pub(super) fn clear_syscall_restart_frame(&self) {
        if self.syscall_restart_frame_valid.load(Ordering::Acquire)
            && self
                .syscall_restart_frame_valid
                .swap(false, Ordering::AcqRel)
        {
            *self.syscall_restart_frame.lock() = None;
        }
    }
}

#[cfg(feature = "auto-run-tests")]
impl Drop for UserTaskExt {
    fn drop(&mut self) {
        USER_TASK_EXT_LIVE.fetch_sub(1, Ordering::AcqRel);
        USER_TASK_EXT_DROPPED.fetch_add(1, Ordering::AcqRel);
    }
}

#[cfg(feature = "auto-run-tests")]
pub fn user_task_ext_stats() -> (usize, usize, usize) {
    (
        USER_TASK_EXT_LIVE.load(Ordering::Acquire),
        USER_TASK_EXT_CREATED.load(Ordering::Acquire),
        USER_TASK_EXT_DROPPED.load(Ordering::Acquire),
    )
}

axtask::def_task_ext!(UserTaskExt);

pub(super) fn current_task_ext() -> Option<&'static UserTaskExt> {
    let curr = axtask::current_may_uninit()?;
    let ptr = unsafe { curr.task_ext_ptr() };
    if ptr.is_null() {
        return None;
    }
    let ext = unsafe { &*(ptr as *const UserTaskExt) };
    Some(ext)
}

pub(super) fn task_ext(task: &AxTaskRef) -> Option<&UserTaskExt> {
    let ptr = unsafe { task.task_ext_ptr() };
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { &*(ptr as *const UserTaskExt) })
}

pub(super) fn set_current_clear_child_tid(tidptr: usize) {
    if let Some(ext) = current_task_ext() {
        ext.clear_child_tid.store(tidptr, Ordering::Release);
    }
}

pub(super) fn set_current_robust_list(head: usize, len: usize) -> Result<(), LinuxError> {
    if len != ROBUST_LIST_HEAD_LEN {
        return Err(LinuxError::EINVAL);
    }
    let Some(ext) = current_task_ext() else {
        return Err(LinuxError::EINVAL);
    };
    ext.robust_list_head.store(head, Ordering::Release);
    ext.robust_list_len.store(len, Ordering::Release);
    Ok(())
}

pub(super) fn robust_list_for_task(task: &AxTaskRef) -> Option<(usize, usize)> {
    let ext = task_ext(task)?;
    Some((
        ext.robust_list_head.load(Ordering::Acquire),
        ext.robust_list_len.load(Ordering::Acquire),
    ))
}

pub(super) fn current_tid() -> i32 {
    axtask::current().id().as_u64() as i32
}

pub(super) fn set_current_user_pc(pc: usize) {
    if let Some(ext) = current_task_ext() {
        ext.last_user_pc.store(pc, Ordering::Release);
    }
}

#[cfg(target_arch = "riscv64")]
#[allow(dead_code)]
pub(super) fn user_pc(tf: &TrapFrame) -> usize {
    tf.sepc
}

#[cfg(target_arch = "loongarch64")]
#[allow(dead_code)]
pub(super) fn user_pc(tf: &TrapFrame) -> usize {
    tf.era
}

#[cfg(target_arch = "x86_64")]
#[allow(dead_code)]
pub(super) fn user_pc(tf: &TrapFrame) -> usize {
    tf.rip as usize
}

#[cfg(target_arch = "aarch64")]
#[allow(dead_code)]
pub(super) fn user_pc(tf: &TrapFrame) -> usize {
    tf.elr as usize
}

pub(super) fn sys_set_tid_address(tf: &TrapFrame, tidptr: usize) -> isize {
    set_current_clear_child_tid(tidptr);
    user_trace!(
        "user-set-tid: tid={} tidptr={tidptr:#x} sp={:#x} tp={:#x} ra={:#x} pc={:#x}",
        current_tid(),
        tf.regs.sp,
        tf.regs.tp,
        tf.regs.ra,
        user_pc(tf),
    );
    axtask::current().id().as_u64() as isize
}

pub(super) fn sys_set_robust_list(head: usize, len: usize) -> isize {
    set_current_robust_list(head, len).map_or_else(neg_errno, |_| 0)
}

pub(super) fn sys_get_robust_list(
    process: &UserProcess,
    pid: i32,
    head_ptr: usize,
    len_ptr: usize,
) -> isize {
    let tid = if pid == 0 { current_tid() } else { pid };
    let Some(entry) = user_thread_entry_by_tid(tid) else {
        // PID 1 is modelled as a visible synthetic init process by /proc and
        // process-group syscalls. It has no real task extension to expose here,
        // but callers that lost permission after setuid should see EPERM for an
        // existing foreign process rather than ESRCH for a missing process.
        if tid == SYNTHETIC_INIT_PID {
            return neg_errno(LinuxError::EPERM);
        }
        return neg_errno(LinuxError::ESRCH);
    };
    if entry.process.pid() != process.pid() {
        return neg_errno(LinuxError::EPERM);
    }
    let Some((head, len)) = robust_list_for_task(&entry.task) else {
        return neg_errno(LinuxError::ESRCH);
    };
    let ret = write_user_value(process, head_ptr, &head);
    if ret != 0 {
        return ret;
    }
    write_user_value(process, len_ptr, &len)
}

pub(super) fn make_uspace_context(entry: usize, stack_ptr: usize, argc: usize) -> UspaceContext {
    #[cfg(target_arch = "riscv64")]
    {
        let mut sstatus = Sstatus::from_bits(0);
        sstatus.set_spie(true);
        sstatus.set_sum(true);
        sstatus.set_fs(FS::Initial);
        let mut tf = TrapFrame {
            regs: axhal::context::TrapFrame::default().regs,
            sepc: entry,
            sstatus,
        };
        tf.regs.sp = stack_ptr;
        // RISC-V glibc crt1 treats entry a0 as rtld_fini, while argc/argv/envp
        // are read from the initial stack. Passing argc here makes static glibc
        // call argc as an exit handler.
        tf.regs.a0 = 0;
        tf.regs.a1 = stack_ptr + size_of::<usize>();
        tf.regs.a2 = stack_ptr + (argc + 2) * size_of::<usize>();
        UspaceContext::from(&tf)
    }
    #[cfg(target_arch = "loongarch64")]
    {
        let mut tf = TrapFrame::default();
        tf.prmd = 0x7;
        tf.era = entry;
        tf.regs.sp = stack_ptr;
        // LoongArch glibc has the same crt1 convention: a0 is rtld_fini, not
        // argc. The argument vector starts on the user stack.
        tf.regs.a0 = 0;
        tf.regs.a1 = stack_ptr + size_of::<usize>();
        tf.regs.a2 = stack_ptr + (argc + 2) * size_of::<usize>();
        UspaceContext::from(&tf)
    }
}

pub(super) fn child_trap_frame(parent: &TrapFrame, child_stack: usize) -> TrapFrame {
    let mut child = *parent;
    child.regs.a0 = 0;
    if child_stack != 0 {
        child.regs.sp = child_stack;
    }
    #[cfg(target_arch = "loongarch64")]
    {
        child.prmd = 0x7;
    }
    advance_syscall_pc(&mut child);
    child
}

fn advance_syscall_pc(tf: &mut TrapFrame) {
    #[cfg(target_arch = "riscv64")]
    {
        tf.sepc += 4;
    }
    #[cfg(target_arch = "loongarch64")]
    {
        tf.era += 4;
    }
}
