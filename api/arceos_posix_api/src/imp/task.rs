use core::ffi::c_int;

/// Relinquish the CPU, and switches to another task.
///
/// For single-threaded configuration (`multitask` feature is disabled), we just
/// relax the CPU and wait for incoming interrupts.
pub fn sys_sched_yield() -> c_int {
    #[cfg(feature = "multitask")]
    axtask::yield_now();
    #[cfg(not(feature = "multitask"))]
    if cfg!(feature = "irq") {
        axhal::asm::wait_for_irqs();
    } else {
        core::hint::spin_loop();
    }
    0
}

/// Get the process ID.
///
/// This POSIX API layer has pthread-style tasks but no exposed multi-process
/// object.  Use the scheduler's current task id as the honest native identity
/// for this API surface instead of returning a fixed value that could disagree
/// with other process-aware syscall paths.
pub fn sys_getpid() -> c_int {
    syscall_body!(sys_getpid, {
        #[cfg(feature = "multitask")]
        {
            Ok(axtask::current_may_uninit()
                .map(|task| task.id().as_u64())
                .unwrap_or(1)
                .min(c_int::MAX as u64) as c_int)
        }
        #[cfg(not(feature = "multitask"))]
        {
            Ok(1)
        }
    })
}

/// Exit current task
pub fn sys_exit(exit_code: c_int) -> ! {
    debug!("sys_exit <= {}", exit_code);
    #[cfg(feature = "multitask")]
    axtask::exit(exit_code);
    #[cfg(not(feature = "multitask"))]
    axhal::power::system_off();
}
