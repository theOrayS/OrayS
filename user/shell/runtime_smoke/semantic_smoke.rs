#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This freestanding program is the userspace side of the repository-contained ABI
// smoke, so using a higher-level syscall or libc wrapper would test that wrapper
// instead of the kernel boundary. The unsafe surface is deliberately limited to the
// architecture syscall instruction and the two compiler-required C memory symbols.
// The semantic-evidence manifest builds and executes this same source on RV64 and
// LA64, and requires the ordered syscall assertions plus clean guest shutdown.

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const SYS_UNAME: usize = 160;
const SYS_GETPID: usize = 172;

#[cfg(target_arch = "riscv64")]
const USER_START: &[u8] = b"PR3_SMOKE_V1 USER_START arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_START: &[u8] = b"PR3_SMOKE_V1 USER_START arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_WRITE: &[u8] = b"PR3_SMOKE_V1 ASSERT write PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_WRITE: &[u8] = b"PR3_SMOKE_V1 ASSERT write PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_GETPID: &[u8] = b"PR3_SMOKE_V1 ASSERT getpid PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_GETPID: &[u8] = b"PR3_SMOKE_V1 ASSERT getpid PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_UNAME_SYSNAME: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_sysname PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_UNAME_SYSNAME: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_sysname PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_UNAME_MACHINE: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_machine PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_UNAME_MACHINE: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_machine PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_PASS: &[u8] = b"PR3_SMOKE_V1 USER_PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_PASS: &[u8] = b"PR3_SMOKE_V1 USER_PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_WRITE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL write arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_WRITE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL write arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_GETPID: &[u8] = b"PR3_SMOKE_V1 USER_FAIL getpid arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_GETPID: &[u8] = b"PR3_SMOKE_V1 USER_FAIL getpid arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_UNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_UNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_SYSNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_sysname arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_SYSNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_sysname arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_MACHINE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_machine arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_MACHINE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_machine arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PANIC: &[u8] = b"PR3_SMOKE_V1 USER_FAIL panic arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PANIC: &[u8] = b"PR3_SMOKE_V1 USER_FAIL panic arch=loongarch64\n";

#[cfg(target_arch = "riscv64")]
const EXPECTED_MACHINE: &[u8] = b"riscv64";
#[cfg(target_arch = "loongarch64")]
const EXPECTED_MACHINE: &[u8] = b"loongarch64";

#[repr(C)]
struct UtsName {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

impl UtsName {
    const fn zeroed() -> Self {
        Self {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        }
    }
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
/// Issues a three-argument syscall using the Linux RV64 userspace ABI.
///
/// # Safety
///
/// `number` must identify a syscall whose first three raw arguments have the layouts
/// supplied in `arg0..=arg2`. Any pointer/length pair must remain valid with the
/// syscall's required read or write access until `ecall` returns, and writable memory
/// must not be aliased for the duration of the call. The caller must interpret the
/// returned Linux value, including negative errno. This instruction binding uses
/// `a0..a2` for arguments/return and `a7` for the number; it intentionally does not
/// claim `nomem` because the kernel may access caller-provided memory.
unsafe fn syscall3(number: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the raw syscall argument contract documented above,
    // and this target-specific block names the Linux RV64 syscall ABI registers.
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
/// Issues a three-argument syscall using the Linux LA64 userspace ABI.
///
/// # Safety
///
/// `number` must identify a syscall whose first three raw arguments have the layouts
/// supplied in `arg0..=arg2`. Any pointer/length pair must remain valid with the
/// syscall's required read or write access until `syscall 0` returns, and writable
/// memory must not be aliased for the duration of the call. The caller must interpret
/// the returned Linux value, including negative errno. This instruction binding uses
/// `$a0..$a2` for arguments/return and `$a7` for the number; it intentionally does not
/// claim `nomem` because the kernel may access caller-provided memory.
unsafe fn syscall3(number: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the raw syscall argument contract documented above,
    // and this target-specific block names the Linux LA64 syscall ABI registers.
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a0") arg0 => ret,
            in("$a1") arg1,
            in("$a2") arg2,
            in("$a7") number,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
fn write(bytes: &[u8]) -> isize {
    // SAFETY: `bytes` is readable for exactly `len` bytes and remains live until the
    // synchronous write returns. fd 1 and the length are scalar SYS_WRITE arguments.
    unsafe { syscall3(SYS_WRITE, 1, bytes.as_ptr() as usize, bytes.len()) }
}

#[inline(always)]
fn exit(code: usize) -> ! {
    // SAFETY: SYS_EXIT consumes only the scalar exit code; its other raw arguments are
    // ignored. If a defective kernel returns, the fallback loop preserves `!`.
    unsafe {
        let _ = syscall3(SYS_EXIT, code, 0, 0);
    }
    loop {
        core::hint::spin_loop();
    }
}

#[inline(never)]
fn fail(marker: &[u8], code: usize) -> ! {
    let _ = write(marker);
    exit(code)
}

fn c_field_equals(field: &[u8; 65], expected: &[u8]) -> bool {
    expected.len() < field.len()
        && &field[..expected.len()] == expected
        && field[expected.len()] == 0
}

// SAFETY: this freestanding ELF provides the sole `memset` definition selected by its
// private linker script, with the C ABI/signature rustc expects for compiler lowering.
#[unsafe(no_mangle)]
/// Fills a caller-provided byte range for compiler-generated freestanding code.
///
/// # Safety
///
/// For `count > 0`, `destination` must be non-null and valid for writes of `count`
/// bytes, and no concurrent or aliased access may violate Rust's memory rules for that
/// range. A zero count performs no pointer arithmetic or dereference.
unsafe extern "C" fn memset(destination: *mut u8, value: i32, count: usize) -> *mut u8 {
    for offset in 0..count {
        // SAFETY: the function contract makes the complete range writable, and
        // `offset < count` keeps this byte within that range (`u8` has alignment 1).
        unsafe {
            destination.add(offset).write_volatile(value as u8);
        }
    }
    destination
}

// SAFETY: this freestanding ELF provides the sole `memcmp` definition selected by its
// private linker script, with the C ABI/signature rustc expects for compiler lowering.
#[unsafe(no_mangle)]
/// Compares two caller-provided byte ranges for compiler-generated freestanding code.
///
/// # Safety
///
/// For `count > 0`, both pointers must be non-null and valid for reads of `count`
/// bytes, and neither range may be mutated concurrently. The ranges may overlap because
/// this function only reads them. A zero count performs no pointer arithmetic or
/// dereference.
unsafe extern "C" fn memcmp(left: *const u8, right: *const u8, count: usize) -> i32 {
    for offset in 0..count {
        // SAFETY: the function contract makes both complete ranges readable, and
        // `offset < count` keeps each byte within its range (`u8` has alignment 1).
        let left_byte = unsafe { left.add(offset).read_volatile() };
        let right_byte = unsafe { right.add(offset).read_volatile() };
        if left_byte != right_byte {
            return left_byte as i32 - right_byte as i32;
        }
    }
    0
}

// SAFETY: the private linker script selects this sole `_start` symbol as the entry of
// the freestanding smoke ELF; its C ABI and non-returning signature match that role.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if write(USER_START) != USER_START.len() as isize {
        fail(USER_FAIL_WRITE, 101);
    }
    if write(ASSERT_WRITE) != ASSERT_WRITE.len() as isize {
        fail(USER_FAIL_WRITE, 102);
    }

    // SAFETY: SYS_GETPID has no pointer arguments; all three raw argument slots are
    // ignored, and the returned scalar is checked before use.
    let pid = unsafe { syscall3(SYS_GETPID, 0, 0, 0) };
    if pid <= 0 {
        fail(USER_FAIL_GETPID, 103);
    }
    if write(ASSERT_GETPID) != ASSERT_GETPID.len() as isize {
        fail(USER_FAIL_WRITE, 104);
    }

    let mut uts = UtsName::zeroed();
    // SAFETY: `uts` is a live, uniquely borrowed, writable `repr(C)` Linux utsname
    // buffer for the duration of SYS_UNAME; no Rust reference observes it until the
    // synchronous syscall returns. The remaining argument slots are ignored.
    let uname_result = unsafe { syscall3(SYS_UNAME, &mut uts as *mut UtsName as usize, 0, 0) };
    if uname_result != 0 {
        fail(USER_FAIL_UNAME, 105);
    }
    if !c_field_equals(&uts.sysname, b"Linux") {
        fail(USER_FAIL_SYSNAME, 106);
    }
    if write(ASSERT_UNAME_SYSNAME) != ASSERT_UNAME_SYSNAME.len() as isize {
        fail(USER_FAIL_WRITE, 107);
    }
    if !c_field_equals(&uts.machine, EXPECTED_MACHINE) {
        fail(USER_FAIL_MACHINE, 108);
    }
    if write(ASSERT_UNAME_MACHINE) != ASSERT_UNAME_MACHINE.len() as isize {
        fail(USER_FAIL_WRITE, 109);
    }
    if write(USER_PASS) != USER_PASS.len() as isize {
        fail(USER_FAIL_WRITE, 110);
    }
    exit(0)
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    fail(USER_FAIL_PANIC, 120)
}
