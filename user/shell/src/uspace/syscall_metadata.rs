//! Declarative audit metadata for architecture-sensitive dispatcher entries.
//!
//! The existing hand-written dispatcher remains the executable source of
//! routing behavior. The PR1 guard compares these declarations with it.

use orays_linux::abi::syscall::numbers;
#[cfg(target_arch = "loongarch64")]
use orays_linux::abi::syscall::{LOONGARCH_LEGACY_GETRLIMIT, LOONGARCH_LEGACY_SETRLIMIT};
use orays_linux::syscall::{SyscallArchitecture, SyscallAvailability, SyscallMeta, SyscallNumber};

#[cfg(not(any(
    target_arch = "riscv64",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
const POLL_EXCLUDED_ARCHITECTURES: &[SyscallArchitecture] = &[
    SyscallArchitecture::Riscv64,
    SyscallArchitecture::Aarch64,
    SyscallArchitecture::LoongArch64,
];

const CLONE: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(numbers::__NR_clone),
    "clone",
    5,
    SyscallAvailability::All,
    "sys_clone",
    None,
    "clone-argument-order",
);

const FSYNC: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(numbers::__NR_fsync),
    "fsync",
    1,
    SyscallAvailability::All,
    "sys_fsync",
    None,
    "fsync-fdatasync-handler",
);

const FDATASYNC: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(numbers::__NR_fdatasync),
    "fdatasync",
    1,
    SyscallAvailability::All,
    "sys_fsync",
    Some("fsync"),
    "fsync-fdatasync-handler",
);

#[cfg(not(any(
    target_arch = "riscv64",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
const POLL: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(numbers::__NR_poll),
    "poll",
    3,
    SyscallAvailability::Except(POLL_EXCLUDED_ARCHITECTURES),
    "sys_poll",
    None,
    "poll-architecture-cfg",
);

#[cfg(target_arch = "riscv64")]
const RISCV64_GETRLIMIT: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(numbers::__NR_getrlimit),
    "getrlimit",
    2,
    SyscallAvailability::Only(SyscallArchitecture::Riscv64),
    "sys_getrlimit",
    None,
    "riscv64-rlimit-number",
);

#[cfg(target_arch = "riscv64")]
const RISCV64_SETRLIMIT: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(numbers::__NR_setrlimit),
    "setrlimit",
    2,
    SyscallAvailability::Only(SyscallArchitecture::Riscv64),
    "sys_setrlimit",
    None,
    "riscv64-rlimit-number",
);

#[cfg(target_arch = "loongarch64")]
const LOONGARCH64_GETRLIMIT: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(LOONGARCH_LEGACY_GETRLIMIT),
    "getrlimit",
    2,
    SyscallAvailability::Only(SyscallArchitecture::LoongArch64),
    "sys_getrlimit",
    None,
    "loongarch64-legacy-rlimit-number",
);

#[cfg(target_arch = "loongarch64")]
const LOONGARCH64_SETRLIMIT: SyscallMeta = SyscallMeta::new(
    SyscallNumber::new(LOONGARCH_LEGACY_SETRLIMIT),
    "setrlimit",
    2,
    SyscallAvailability::Only(SyscallArchitecture::LoongArch64),
    "sys_setrlimit",
    None,
    "loongarch64-legacy-rlimit-number",
);

#[used]
static SYSCALL_METADATA: &[SyscallMeta] = &[
    CLONE,
    FSYNC,
    FDATASYNC,
    #[cfg(not(any(
        target_arch = "riscv64",
        target_arch = "aarch64",
        target_arch = "loongarch64"
    )))]
    POLL,
    #[cfg(target_arch = "riscv64")]
    RISCV64_GETRLIMIT,
    #[cfg(target_arch = "riscv64")]
    RISCV64_SETRLIMIT,
    #[cfg(target_arch = "loongarch64")]
    LOONGARCH64_GETRLIMIT,
    #[cfg(target_arch = "loongarch64")]
    LOONGARCH64_SETRLIMIT,
];

const _: () = {
    assert!(CLONE.argument_count() == 5);
    assert!(FSYNC.argument_count() == 1);
    assert!(FDATASYNC.argument_count() == 1);
    assert!(FDATASYNC.alias_of().is_some());
};

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
const _: () = assert!(SYSCALL_METADATA.len() == 5);

#[cfg(target_arch = "aarch64")]
const _: () = assert!(SYSCALL_METADATA.len() == 3);

#[cfg(not(any(
    target_arch = "riscv64",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
const _: () = assert!(SYSCALL_METADATA.len() == 4);
