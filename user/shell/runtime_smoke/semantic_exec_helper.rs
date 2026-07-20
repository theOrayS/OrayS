#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This second freestanding image is execve'd by the semantic smoke after stdout
// has been redirected to a pipe. Raw syscalls keep the check independent of libc
// while exercising the same fork/exec/pipe/wait boundary used by popen callers.
const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const PAYLOAD: &[u8] = b"orays-exec-helper\n";

/// Issues one Linux syscall using the RISC-V64 userspace ABI.
///
/// # Safety
///
/// The caller must provide arguments valid for `number`, including keeping every
/// pointed-to range live and correctly accessible until the synchronous syscall
/// returns. Negative results are Linux errno values and must be handled by the
/// caller.
#[cfg(target_arch = "riscv64")]
unsafe fn syscall6(
    number: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the raw syscall contract above; these are the
    // registers prescribed by the RISC-V64 Linux syscall ABI.
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a7") number,
            options(nostack)
        );
    }
    ret
}

/// Issues one Linux syscall using the LoongArch64 userspace ABI.
///
/// # Safety
///
/// The caller must provide arguments valid for `number`, including keeping every
/// pointed-to range live and correctly accessible until the synchronous syscall
/// returns. Negative results are Linux errno values and must be handled by the
/// caller.
#[cfg(target_arch = "loongarch64")]
unsafe fn syscall6(
    number: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the raw syscall contract above; these are the
    // registers prescribed by the LoongArch64 Linux syscall ABI.
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a0") arg0 => ret,
            in("$a1") arg1,
            in("$a2") arg2,
            in("$a3") arg3,
            in("$a4") arg4,
            in("$a5") arg5,
            in("$a7") number,
            options(nostack)
        );
    }
    ret
}

fn exit(code: usize) -> ! {
    // SAFETY: SYS_EXIT consumes a scalar status and does not dereference memory.
    unsafe {
        let _ = syscall6(SYS_EXIT, code, 0, 0, 0, 0, 0);
    }
    loop {
        core::hint::spin_loop();
    }
}

// SAFETY: the private linker script selects this symbol as the sole ELF entry;
// the C ABI and non-returning signature match that contract.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut written = 0usize;
    while written < PAYLOAD.len() {
        // SAFETY: the remaining suffix is readable and live for the synchronous
        // write. fd 1 is the stdout pipe installed by the parent smoke process.
        let result = unsafe {
            syscall6(
                SYS_WRITE,
                1,
                PAYLOAD.as_ptr().add(written) as usize,
                PAYLOAD.len() - written,
                0,
                0,
                0,
            )
        };
        if result <= 0 || result as usize > PAYLOAD.len() - written {
            exit(1);
        }
        written += result as usize;
    }
    exit(0)
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    exit(2)
}
