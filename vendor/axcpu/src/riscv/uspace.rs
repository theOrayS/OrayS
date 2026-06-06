//! Structures and functions for user space.

use core::arch::naked_asm;
use memory_addr::VirtAddr;
use riscv::register::sstatus::Sstatus;
#[cfg(feature = "fp-simd")]
use riscv::register::sstatus::FS;

use crate::{GeneralRegisters, TrapFrame};

/// Context to enter user space.
pub struct UspaceContext(TrapFrame);

impl UspaceContext {
    /// Creates an empty context with all registers set to zero.
    pub const fn empty() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }

    /// Creates a new context with the given entry point, user stack pointer,
    /// and the argument.
    pub fn new(entry: usize, ustack_top: VirtAddr, arg0: usize) -> Self {
        let mut sstatus = Sstatus::from_bits(0);
        sstatus.set_spie(true); // enable interrupts
        sstatus.set_sum(true); // enable user memory access in supervisor mode
        #[cfg(feature = "fp-simd")]
        {
            sstatus.set_fs(FS::Initial); // set the FPU to initial state
        }

        Self(TrapFrame {
            regs: GeneralRegisters {
                a0: arg0,
                sp: ustack_top.as_usize(),
                ..Default::default()
            },
            sepc: entry,
            sstatus,
        })
    }

    /// Creates a new context from the given [`TrapFrame`].
    pub const fn from(trap_frame: &TrapFrame) -> Self {
        Self(*trap_frame)
    }

    /// Gets the instruction pointer.
    pub const fn get_ip(&self) -> usize {
        self.0.sepc
    }

    /// Gets the stack pointer.
    pub const fn get_sp(&self) -> usize {
        self.0.regs.sp
    }

    /// Sets the instruction pointer.
    pub const fn set_ip(&mut self, pc: usize) {
        self.0.sepc = pc;
    }

    /// Sets the stack pointer.
    pub const fn set_sp(&mut self, sp: usize) {
        self.0.regs.sp = sp;
    }

    /// Sets the return value register.
    pub const fn set_retval(&mut self, a0: usize) {
        self.0.regs.a0 = a0;
    }

    /// Enters user space.
    ///
    /// It restores the user registers and jumps to the user entry point
    /// (saved in `sepc`).
    /// When an exception or syscall occurs, the kernel stack pointer is
    /// switched to `kstack_top`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it changes processor mode and the stack.
    pub unsafe fn enter_uspace(&self, kstack_top: VirtAddr) -> ! {
        crate::asm::disable_irqs();
        unsafe { enter_uspace_raw(core::ptr::addr_of!(self.0), kstack_top.as_usize()) }
    }
}

#[unsafe(naked)]
unsafe extern "C" fn enter_uspace_raw(_tf: *const TrapFrame, _kstack_top: usize) -> ! {
    naked_asm!(
        include_asm_macros!(),
        "
        csrw    sscratch, a1
        addi    a1, a1, -{trapframe_size}
        LDR     t0, a0, 32
        csrw    sepc, t0

        STR     gp, a1, 3
        LDR     gp, a0, 3

        STR     tp, a1, 4
        LDR     tp, a0, 4

        LDR     t0, a0, 33
        csrw    sstatus, t0
        mv      sp, a0
        POP_GENERAL_REGS
        LDR     sp, sp, 2
        sret
        ",
        trapframe_size = const core::mem::size_of::<TrapFrame>(),
    )
}
