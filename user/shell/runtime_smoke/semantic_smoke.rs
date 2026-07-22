#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This freestanding program is the userspace side of the repository-contained ABI
// smoke, so using a higher-level syscall or libc wrapper would test that wrapper
// instead of the kernel boundary. The unsafe surface is deliberately limited to the
// architecture syscall instruction and the two compiler-required C memory symbols.
// The semantic-evidence manifest builds and executes this same source on RV64 and
// LA64, and requires the ordered syscall assertions plus clean guest shutdown.

const SYS_DUP: usize = 23;
const SYS_FCNTL: usize = 25;
const SYS_OPENAT: usize = 56;
const SYS_CLOSE: usize = 57;
const SYS_PIPE2: usize = 59;
const SYS_READ: usize = 63;
const SYS_WRITE: usize = 64;
const SYS_VMSPLICE: usize = 75;
const SYS_SPLICE: usize = 76;
const SYS_TEE: usize = 77;
const SYS_EXIT: usize = 93;
const SYS_UNAME: usize = 160;
const SYS_GETPID: usize = 172;

const NEG_EBADF: isize = -9;
const NEG_EAGAIN: isize = -11;
const NEG_EINVAL: isize = -22;
const NEG_ESPIPE: isize = -29;
const AT_FDCWD: isize = -100;
const F_GETFL: usize = 3;
const O_RDONLY: usize = 0;
const O_WRONLY: usize = 1;
const O_NONBLOCK: usize = 0o4000;

// The first vector is exactly the largest pipe capacity supported by OrayS. A
// blocking vmsplice that fills it must return its progress rather than wait on
// the following vector in the same syscall. The one-byte second vector then
// proves that the caller can drain the pipe and resume without data loss.
const VMSPLICE_FIRST_LEN: usize = 64 * 1024;
const VMSPLICE_TOTAL_LEN: usize = VMSPLICE_FIRST_LEN + 1;
static VMSPLICE_FIRST: [u8; VMSPLICE_FIRST_LEN] = [0x3c; VMSPLICE_FIRST_LEN];
static VMSPLICE_SECOND: [u8; 1] = [0xa5];

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
const ASSERT_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 ASSERT splice_pipe PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 ASSERT splice_pipe PASS arch=loongarch64\n";
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
const USER_FAIL_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL splice_pipe arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL splice_pipe arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_TEE_DEVICE_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_open arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_TEE_DEVICE_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_open arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_TEE_DEVICE_MODE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_mode arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_TEE_DEVICE_MODE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_mode arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_TEE_DEVICE_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_close arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_TEE_DEVICE_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_close arch=loongarch64\n";
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

#[repr(C)]
struct IoVec {
    base: usize,
    len: usize,
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
/// Issues a six-argument syscall using the Linux RV64 userspace ABI.
///
/// # Safety
///
/// `number` must identify a syscall whose six raw arguments have the layouts
/// supplied in `arg0..=arg5`. A pointer may intentionally be invalid when exercising
/// kernel rejection and errno precedence, but Rust must never dereference such a value;
/// whenever the syscall is expected to access memory, the complete range must remain
/// valid with the required access and writable memory must not be aliased until `ecall`
/// returns. The caller must interpret the Linux return value, including negative errno.
/// This instruction binding uses `a0..a5` for arguments/return and `a7` for the number;
/// it intentionally does not claim `nomem` because the kernel may access caller memory.
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
    // SAFETY: the caller upholds the raw syscall argument contract documented above,
    // and this target-specific block names the Linux RV64 syscall ABI registers.
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

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
/// Issues a six-argument syscall using the Linux LA64 userspace ABI.
///
/// # Safety
///
/// `number` must identify a syscall whose six raw arguments have the layouts
/// supplied in `arg0..=arg5`. A pointer may intentionally be invalid when exercising
/// kernel rejection and errno precedence, but Rust must never dereference such a value;
/// whenever the syscall is expected to access memory, the complete range must remain
/// valid with the required access and writable memory must not be aliased until
/// `syscall 0` returns. The caller must interpret the Linux return value, including
/// negative errno. This binding uses `$a0..$a5` for arguments/return and `$a7` for the
/// number; it intentionally does not claim `nomem` because the kernel may access caller
/// memory.
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
    // SAFETY: the caller upholds the raw syscall argument contract documented above,
    // and this target-specific block names the Linux LA64 syscall ABI registers.
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

#[inline(always)]
fn write(bytes: &[u8]) -> isize {
    // SAFETY: `bytes` is readable for exactly `len` bytes and remains live until the
    // synchronous write returns. fd 1 and the length are scalar SYS_WRITE arguments.
    unsafe { syscall6(SYS_WRITE, 1, bytes.as_ptr() as usize, bytes.len(), 0, 0, 0) }
}

#[inline(always)]
fn pipe2(pipe: &mut [i32; 2], flags: usize) -> isize {
    // SAFETY: `pipe` is uniquely borrowed and writable for two i32 values until the
    // synchronous syscall returns; flags and unused argument slots are scalar values.
    unsafe {
        syscall6(
            SYS_PIPE2,
            pipe.as_mut_ptr() as usize,
            flags,
            0,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn pipe_write(fd: i32, bytes: &[u8]) -> isize {
    // SAFETY: `bytes` remains readable for its complete length until the synchronous
    // syscall returns; the descriptor and unused argument slots are scalar values.
    unsafe {
        syscall6(
            SYS_WRITE,
            fd as usize,
            bytes.as_ptr() as usize,
            bytes.len(),
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn pipe_read(fd: i32, bytes: &mut [u8]) -> isize {
    // SAFETY: `bytes` is uniquely borrowed and writable for its complete length until
    // the synchronous syscall returns; no Rust reference observes it during the call.
    unsafe {
        syscall6(
            SYS_READ,
            fd as usize,
            bytes.as_mut_ptr() as usize,
            bytes.len(),
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn tee(fd_in: i32, fd_out: i32, len: usize, flags: usize) -> isize {
    // SAFETY: tee consumes only scalar descriptors, length, and flags; it has no
    // userspace pointer arguments.
    unsafe {
        syscall6(
            SYS_TEE,
            fd_in as usize,
            fd_out as usize,
            len,
            flags,
            0,
            0,
        )
    }
}

#[inline(always)]
fn vmsplice(fd: i32, iovecs: &[IoVec], flags: usize) -> isize {
    // SAFETY: the iovec array and every described byte range remain readable until
    // this synchronous vmsplice returns. The descriptors, count, and flags are
    // scalar arguments, and the immutable backing arrays outlive the call.
    unsafe {
        syscall6(
            SYS_VMSPLICE,
            fd as usize,
            iovecs.as_ptr() as usize,
            iovecs.len(),
            flags,
            0,
            0,
        )
    }
}

#[inline(always)]
fn splice(fd_in: i32, fd_out: i32, len: usize, flags: usize) -> isize {
    // SAFETY: both offset pointers are null by contract, descriptors/length/flags are
    // scalars, and the kernel validates descriptor direction and available data.
    unsafe {
        syscall6(
            SYS_SPLICE,
            fd_in as usize,
            0,
            fd_out as usize,
            0,
            len,
            flags,
        )
    }
}

#[inline(always)]
fn close(fd: i32) -> isize {
    // SAFETY: SYS_CLOSE consumes only the scalar descriptor and ignores the remaining
    // argument slots. Callers ensure each successfully installed descriptor is closed
    // at most once in the success path.
    unsafe { syscall6(SYS_CLOSE, fd as usize, 0, 0, 0, 0, 0) }
}

#[inline(always)]
fn dup(fd: i32) -> isize {
    // SAFETY: SYS_DUP consumes only the scalar descriptor and ignores the remaining
    // argument slots.
    unsafe { syscall6(SYS_DUP, fd as usize, 0, 0, 0, 0, 0) }
}

#[inline(always)]
fn fcntl_getfl(fd: i32) -> isize {
    // SAFETY: SYS_FCNTL/F_GETFL consumes only the scalar descriptor and command; the
    // remaining argument slots are ignored.
    unsafe { syscall6(SYS_FCNTL, fd as usize, F_GETFL, 0, 0, 0, 0) }
}

#[inline(always)]
fn openat(path: &[u8], flags: usize) -> isize {
    // SAFETY: callers provide a readable NUL-terminated pathname that remains live
    // until this synchronous syscall returns. AT_FDCWD, flags, and mode are scalars;
    // mode is ignored because these probes do not request O_CREAT.
    unsafe {
        syscall6(
            SYS_OPENAT,
            AT_FDCWD as usize,
            path.as_ptr() as usize,
            flags,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn exit(code: usize) -> ! {
    // SAFETY: SYS_EXIT consumes only the scalar exit code; its other raw arguments are
    // ignored. If a defective kernel returns, the fallback loop preserves `!`.
    unsafe {
        let _ = syscall6(SYS_EXIT, code, 0, 0, 0, 0, 0);
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

fn expected_vmsplice_byte(offset: usize) -> u8 {
    if offset < VMSPLICE_FIRST_LEN {
        0x3c
    } else {
        0xa5
    }
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
    let pid = unsafe { syscall6(SYS_GETPID, 0, 0, 0, 0, 0, 0) };
    if pid <= 0 {
        fail(USER_FAIL_GETPID, 103);
    }
    if write(ASSERT_GETPID) != ASSERT_GETPID.len() as isize {
        fail(USER_FAIL_WRITE, 104);
    }

    // Linux resolves a zero-length splice before flags, descriptors, or user offsets.
    // The deliberately invalid scalar pointer values must therefore never be touched.
    // SAFETY: no Rust pointer is constructed or dereferenced; this call intentionally
    // supplies invalid raw values to verify that the zero-length ABI returns first.
    if unsafe {
        syscall6(
            SYS_SPLICE,
            usize::MAX,
            1,
            usize::MAX,
            1,
            0,
            usize::MAX,
        )
    } != 0
    {
        fail(USER_FAIL_SPLICE_PIPE, 105);
    }

    let mut source_pipe = [-1_i32; 2];
    let mut destination_pipe = [-1_i32; 2];
    if pipe2(&mut source_pipe, 0) != 0 || pipe2(&mut destination_pipe, 0) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 106);
    }
    const SPLICE_PAYLOAD: &[u8; 12] = b"splice-smoke";
    if pipe_write(source_pipe[1], SPLICE_PAYLOAD) != SPLICE_PAYLOAD.len() as isize {
        fail(USER_FAIL_SPLICE_PIPE, 107);
    }

    // fd acquisition precedes offset copying. An invalid input fd plus an invalid raw
    // offset must report EBADF without dereferencing the offset.
    // SAFETY: the raw value 1 is intentionally not a Rust pointer and must be rejected
    // before access because fd_in is invalid; all other arguments are bounded scalars.
    if unsafe {
        syscall6(
            SYS_SPLICE,
            usize::MAX,
            1,
            destination_pipe[1] as usize,
            0,
            1,
            0,
        )
    } != NEG_EBADF
    {
        fail(USER_FAIL_SPLICE_PIPE, 108);
    }

    // Pipe offsets are rejected as ESPIPE before the kernel reads the pointed-to value.
    // SAFETY: the deliberately invalid raw offset is never made into a Rust reference;
    // the live pipe endpoint requires the kernel to reject it before user-memory access.
    if unsafe {
        syscall6(
            SYS_SPLICE,
            source_pipe[0] as usize,
            1,
            destination_pipe[1] as usize,
            0,
            1,
            0,
        )
    } != NEG_ESPIPE
    {
        fail(USER_FAIL_SPLICE_PIPE, 109);
    }

    if splice(source_pipe[1], destination_pipe[1], 1, 0) != NEG_EBADF {
        fail(USER_FAIL_SPLICE_PIPE, 110);
    }

    // Different descriptors can still identify the same pipe backing object. The
    // rejected transfer must leave the source bytes available for the valid splice.
    if splice(
        source_pipe[0],
        source_pipe[1],
        SPLICE_PAYLOAD.len(),
        0,
    ) != NEG_EINVAL
    {
        fail(USER_FAIL_SPLICE_PIPE, 111);
    }
    if splice(
        source_pipe[0],
        destination_pipe[1],
        SPLICE_PAYLOAD.len(),
        0,
    ) != SPLICE_PAYLOAD.len() as isize
    {
        fail(USER_FAIL_SPLICE_PIPE, 112);
    }
    let mut spliced = [0_u8; SPLICE_PAYLOAD.len()];
    if pipe_read(destination_pipe[0], &mut spliced) != spliced.len() as isize
        || spliced != *SPLICE_PAYLOAD
    {
        fail(USER_FAIL_SPLICE_PIPE, 113);
    }

    // Close both destination descriptors, install a replacement pipe (which may reuse
    // either fd number), and prove the next transfer uses the newly installed objects.
    if close(destination_pipe[0]) != 0 || close(destination_pipe[1]) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 114);
    }
    destination_pipe = [-1; 2];
    if pipe2(&mut destination_pipe, 0) != 0
        || pipe_write(source_pipe[1], SPLICE_PAYLOAD) != SPLICE_PAYLOAD.len() as isize
        || splice(
            source_pipe[0],
            destination_pipe[1],
            SPLICE_PAYLOAD.len(),
            0,
        ) != SPLICE_PAYLOAD.len() as isize
    {
        fail(USER_FAIL_SPLICE_PIPE, 115);
    }
    spliced = [0; SPLICE_PAYLOAD.len()];
    if pipe_read(destination_pipe[0], &mut spliced) != spliced.len() as isize
        || spliced != *SPLICE_PAYLOAD
    {
        fail(USER_FAIL_SPLICE_PIPE, 116);
    }
    for fd in source_pipe.into_iter().chain(destination_pipe) {
        if close(fd) != 0 {
            fail(USER_FAIL_SPLICE_PIPE, 117);
        }
    }

    // A splice into a full O_NONBLOCK destination must not consume its source even
    // without SPLICE_F_NONBLOCK. Filling the destination through its normal writer
    // exercises capacity contention without a timing-dependent race; the subsequent
    // source read proves byte preservation.
    let mut preserved_source = [-1_i32; 2];
    let mut full_destination = [-1_i32; 2];
    if pipe2(&mut preserved_source, 0) != 0
        || pipe2(&mut full_destination, O_NONBLOCK) != 0
        || pipe_write(preserved_source[1], SPLICE_PAYLOAD) != SPLICE_PAYLOAD.len() as isize
    {
        fail(USER_FAIL_SPLICE_PIPE, 118);
    }
    let fill = [0x5a_u8; 512];
    let mut filled = 0usize;
    loop {
        let result = pipe_write(full_destination[1], &fill);
        if result > 0 {
            filled = filled.saturating_add(result as usize);
            if filled > 1024 * 1024 {
                fail(USER_FAIL_SPLICE_PIPE, 119);
            }
        } else if result == NEG_EAGAIN {
            break;
        } else {
            fail(USER_FAIL_SPLICE_PIPE, 120);
        }
    }
    if filled == 0
        || splice(
            preserved_source[0],
            full_destination[1],
            SPLICE_PAYLOAD.len(),
            0,
        ) != NEG_EAGAIN
    {
        fail(USER_FAIL_SPLICE_PIPE, 121);
    }
    let mut preserved = [0_u8; SPLICE_PAYLOAD.len()];
    if pipe_read(preserved_source[0], &mut preserved) != preserved.len() as isize
        || preserved != *SPLICE_PAYLOAD
    {
        fail(USER_FAIL_SPLICE_PIPE, 122);
    }
    for fd in preserved_source.into_iter().chain(full_destination) {
        if close(fd) != 0 {
            fail(USER_FAIL_SPLICE_PIPE, 123);
        }
    }

    // Linux validates tee flags first, returns zero for a zero-length request before
    // resolving either descriptor, then resolves both descriptors before checking
    // access modes and finally pipe type/backing identity. Stdio supplies stable
    // live non-pipe descriptions with known access modes: fd 0 is read-only and fd 1
    // is write-only. Every rejection below must precede any wait on the empty pipe.
    let mut tee_pipe = [-1_i32; 2];
    if pipe2(&mut tee_pipe, 0) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 124);
    }
    if tee(-1, -1, 0, 0) != 0 || tee(-1, -1, 0, usize::MAX) != NEG_EINVAL {
        fail(USER_FAIL_SPLICE_PIPE, 224);
    }
    if tee(1, tee_pipe[1], 1, 0) != NEG_EBADF
        || tee(tee_pipe[0], 0, 1, 0) != NEG_EBADF
        || tee(0, -1, 1, 0) != NEG_EBADF
        || tee(0, tee_pipe[0], 1, 0) != NEG_EBADF
        || tee(tee_pipe[1], 1, 1, 0) != NEG_EBADF
    {
        fail(USER_FAIL_SPLICE_PIPE, 225);
    }
    if tee(0, tee_pipe[1], 1, 0) != NEG_EINVAL
        || tee(tee_pipe[0], 1, 1, 0) != NEG_EINVAL
        || tee(tee_pipe[0], tee_pipe[1], 1, 0) != NEG_EINVAL
    {
        fail(USER_FAIL_SPLICE_PIPE, 125);
    }

    // Device-backed descriptions must retain the access mode supplied to openat.
    // Wrong-direction endpoints fail with EBADF before the correctly directed live
    // non-pipe combinations reach the EINVAL type check.
    let dev_null_read = openat(b"/dev/null\0", O_RDONLY);
    let dev_null_write = openat(b"/dev/null\0", O_WRONLY);
    if dev_null_read < 0 || dev_null_write < 0 {
        fail(USER_FAIL_TEE_DEVICE_OPEN, 226);
    }
    let dev_null_read = dev_null_read as i32;
    let dev_null_write = dev_null_write as i32;
    if tee(tee_pipe[0], dev_null_read, 1, 0) != NEG_EBADF
        || tee(dev_null_write, tee_pipe[1], 1, 0) != NEG_EBADF
    {
        fail(USER_FAIL_TEE_DEVICE_MODE, 227);
    }
    if tee(dev_null_read, tee_pipe[1], 1, 0) != NEG_EINVAL
        || tee(tee_pipe[0], dev_null_write, 1, 0) != NEG_EINVAL
    {
        fail(USER_FAIL_TEE_DEVICE_MODE, 228);
    }
    // The preserved open(2) access mode must also gate plain reads and writes on
    // the device descriptions: a write-only description rejects reads and a
    // read-only description rejects writes with EBADF, while the correctly
    // directed operations succeed.
    let mut dev_mode_buf = [0_u8; 1];
    if pipe_read(dev_null_write, &mut dev_mode_buf) != NEG_EBADF
        || pipe_write(dev_null_read, &dev_mode_buf) != NEG_EBADF
    {
        fail(USER_FAIL_TEE_DEVICE_MODE, 230);
    }
    if pipe_read(dev_null_read, &mut dev_mode_buf) != 0
        || pipe_write(dev_null_write, &dev_mode_buf) != 1
    {
        fail(USER_FAIL_TEE_DEVICE_MODE, 231);
    }
    // Duplicated descriptors share the same open file description, so the
    // preserved access mode and the tee errno behavior survive dup(2), and
    // F_GETFL reports the mode selected at open time.
    let dev_null_read_dup = dup(dev_null_read);
    let dev_null_write_dup = dup(dev_null_write);
    if dev_null_read_dup < 0 || dev_null_write_dup < 0 {
        fail(USER_FAIL_TEE_DEVICE_MODE, 232);
    }
    if tee(tee_pipe[0], dev_null_read_dup as i32, 1, 0) != NEG_EBADF
        || tee(dev_null_write_dup as i32, tee_pipe[1], 1, 0) != NEG_EBADF
        || fcntl_getfl(dev_null_read_dup as i32) != O_RDONLY as isize
        || fcntl_getfl(dev_null_write_dup as i32) != O_WRONLY as isize
    {
        fail(USER_FAIL_TEE_DEVICE_MODE, 233);
    }
    if close(dev_null_read_dup as i32) != 0 || close(dev_null_write_dup as i32) != 0 {
        fail(USER_FAIL_TEE_DEVICE_MODE, 234);
    }
    if close(dev_null_read) != 0 || close(dev_null_write) != 0 {
        fail(USER_FAIL_TEE_DEVICE_CLOSE, 229);
    }
    if close(tee_pipe[0]) != 0 || close(tee_pipe[1]) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 126);
    }

    // Exercise blocking vmsplice across an iovec boundary larger than a pipe.
    // Each successful partial result is drained before retrying, and every byte is
    // checked in order. A syscall that has already copied bytes must not wait for
    // capacity while advancing to the next vector.
    let mut vmsplice_pipe = [-1_i32; 2];
    if pipe2(&mut vmsplice_pipe, 0) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 127);
    }
    let mut total = 0usize;
    let mut calls = 0usize;
    while total < VMSPLICE_TOTAL_LEN {
        let (iovecs, count) = if total < VMSPLICE_FIRST_LEN {
            let first = &VMSPLICE_FIRST[total..];
            (
                [
                    IoVec {
                        base: first.as_ptr() as usize,
                        len: first.len(),
                    },
                    IoVec {
                        base: VMSPLICE_SECOND.as_ptr() as usize,
                        len: VMSPLICE_SECOND.len(),
                    },
                ],
                2,
            )
        } else {
            (
                [
                    IoVec {
                        base: VMSPLICE_SECOND.as_ptr() as usize,
                        len: VMSPLICE_SECOND.len(),
                    },
                    IoVec { base: 0, len: 0 },
                ],
                1,
            )
        };
        let result = vmsplice(vmsplice_pipe[1], &iovecs[..count], 0);
        let remaining = VMSPLICE_TOTAL_LEN - total;
        if result <= 0 || result as usize > remaining {
            fail(USER_FAIL_SPLICE_PIPE, 128);
        }
        let moved = result as usize;
        let mut drained = 0usize;
        let mut read_buffer = [0_u8; 512];
        while drained < moved {
            let requested = (moved - drained).min(read_buffer.len());
            let read = pipe_read(vmsplice_pipe[0], &mut read_buffer[..requested]);
            if read <= 0 || read as usize > requested {
                fail(USER_FAIL_SPLICE_PIPE, 129);
            }
            let read = read as usize;
            for (index, byte) in read_buffer[..read].iter().enumerate() {
                if *byte != expected_vmsplice_byte(total + drained + index) {
                    fail(USER_FAIL_SPLICE_PIPE, 130);
                }
            }
            drained += read;
        }

        total += moved;
        calls += 1;
        if calls > 1024 {
            fail(USER_FAIL_SPLICE_PIPE, 131);
        }
    }
    if close(vmsplice_pipe[0]) != 0 || close(vmsplice_pipe[1]) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 132);
    }
    if write(ASSERT_SPLICE_PIPE) != ASSERT_SPLICE_PIPE.len() as isize {
        fail(USER_FAIL_WRITE, 133);
    }

    let mut uts = UtsName::zeroed();
    // SAFETY: `uts` is a live, uniquely borrowed, writable `repr(C)` Linux utsname
    // buffer for the duration of SYS_UNAME; no Rust reference observes it until the
    // synchronous syscall returns. The remaining argument slots are ignored.
    let uname_result =
        unsafe { syscall6(SYS_UNAME, &mut uts as *mut UtsName as usize, 0, 0, 0, 0, 0) };
    if uname_result != 0 {
        fail(USER_FAIL_UNAME, 134);
    }
    if !c_field_equals(&uts.sysname, b"Linux") {
        fail(USER_FAIL_SYSNAME, 135);
    }
    if write(ASSERT_UNAME_SYSNAME) != ASSERT_UNAME_SYSNAME.len() as isize {
        fail(USER_FAIL_WRITE, 136);
    }
    if !c_field_equals(&uts.machine, EXPECTED_MACHINE) {
        fail(USER_FAIL_MACHINE, 137);
    }
    if write(ASSERT_UNAME_MACHINE) != ASSERT_UNAME_MACHINE.len() as isize {
        fail(USER_FAIL_WRITE, 138);
    }
    if write(USER_PASS) != USER_PASS.len() as isize {
        fail(USER_FAIL_WRITE, 139);
    }
    exit(0)
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    fail(USER_FAIL_PANIC, 120)
}
