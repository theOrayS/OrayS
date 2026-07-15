//! Architecture-selected Linux syscall numbers.

/// The target-specific syscall-number namespace supplied by Linux UAPI.
pub use linux_raw_sys::general as numbers;

#[cfg(target_arch = "loongarch64")]
pub const LOONGARCH_LEGACY_GETRLIMIT: u32 = 163;
#[cfg(target_arch = "loongarch64")]
pub const LOONGARCH_LEGACY_SETRLIMIT: u32 = 164;

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
const _: () = {
    assert!(numbers::__NR_read == 63);
    assert!(numbers::__NR_write == 64);
    assert!(numbers::__NR_ppoll == 73);
    assert!(numbers::__NR_clone == 220);
};

#[cfg(target_arch = "riscv64")]
const _: () = {
    assert!(numbers::__NR_getrlimit == 163);
    assert!(numbers::__NR_setrlimit == 164);
};

#[cfg(target_arch = "loongarch64")]
const _: () = {
    assert!(LOONGARCH_LEGACY_GETRLIMIT == 163);
    assert!(LOONGARCH_LEGACY_SETRLIMIT == 164);
};
