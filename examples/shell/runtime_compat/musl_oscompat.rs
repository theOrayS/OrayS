#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const ENOMEM: i32 = 12;
const EACCES: i32 = 13;
const EFAULT: i32 = 14;
const EPERM: i32 = 1;
const ENAMETOOLONG: i32 = 36;
const PRIO_PROCESS: usize = 0;
const SYS_EPOLL_PWAIT: usize = 22;
const SYS_SETPRIORITY: usize = 140;
const SYS_SCHED_SETPARAM: usize = 118;
const SYS_SCHED_SETSCHEDULER: usize = 119;
const SYS_SCHED_GETSCHEDULER: usize = 120;
const SYS_SCHED_GETPARAM: usize = 121;
const SYS_SCHED_GET_PRIORITY_MAX: usize = 125;
const SYS_SCHED_GET_PRIORITY_MIN: usize = 126;
const SYS_SCHED_RR_GET_INTERVAL: usize = 127;
const SYS_GETPRIORITY: usize = 141;
const SYS_READLINKAT: usize = 78;
const SYS_UNAME: usize = 160;
const SYS_BRK: usize = 214;
const AT_FDCWD: i32 = -100;

unsafe extern "C" {
    fn __errno_location() -> *mut i32;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[inline]
unsafe fn set_errno(err: i32) {
    let errno = unsafe { __errno_location() };
    if !errno.is_null() {
        unsafe {
            *errno = err;
        }
    }
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
unsafe fn syscall1(number: usize, arg0: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
unsafe fn syscall2(number: usize, arg0: usize, arg1: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
unsafe fn syscall3(number: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
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

#[cfg(target_arch = "riscv64")]
#[inline(always)]
unsafe fn syscall4(number: usize, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
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
unsafe fn syscall1(number: usize, arg0: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a0") arg0 => ret,
            in("$a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
unsafe fn syscall2(number: usize, arg0: usize, arg1: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a0") arg0 => ret,
            in("$a1") arg1,
            in("$a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
unsafe fn syscall3(number: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
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

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
unsafe fn syscall4(number: usize, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a0") arg0 => ret,
            in("$a1") arg1,
            in("$a2") arg2,
            in("$a3") arg3,
            in("$a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
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

#[inline]
fn syscall_error(ret: isize) -> Option<i32> {
    if (-4095..0).contains(&ret) {
        Some((-ret) as i32)
    } else {
        None
    }
}

#[repr(C)]
pub struct SchedParam {
    sched_priority: i32,
}

#[repr(C)]
pub struct Timespec {
    tv_sec: isize,
    tv_nsec: isize,
}

#[inline]
unsafe fn syscall_ret_i32(ret: isize) -> i32 {
    if let Some(err) = syscall_error(ret) {
        unsafe { set_errno(err) };
        -1
    } else {
        ret as i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn sched_setparam(pid: i32, param: *const SchedParam) -> i32 {
    unsafe { syscall_ret_i32(syscall2(SYS_SCHED_SETPARAM, pid as usize, param as usize)) }
}

#[no_mangle]
pub unsafe extern "C" fn sched_getparam(pid: i32, param: *mut SchedParam) -> i32 {
    unsafe { syscall_ret_i32(syscall2(SYS_SCHED_GETPARAM, pid as usize, param as usize)) }
}

#[no_mangle]
pub unsafe extern "C" fn sched_setscheduler(
    pid: i32,
    policy: i32,
    param: *const SchedParam,
) -> i32 {
    unsafe {
        syscall_ret_i32(syscall3(
            SYS_SCHED_SETSCHEDULER,
            pid as usize,
            policy as usize,
            param as usize,
        ))
    }
}

#[no_mangle]
pub unsafe extern "C" fn sched_getscheduler(pid: i32) -> i32 {
    unsafe { syscall_ret_i32(syscall1(SYS_SCHED_GETSCHEDULER, pid as usize)) }
}

#[no_mangle]
pub unsafe extern "C" fn sched_get_priority_max(policy: i32) -> i32 {
    unsafe { syscall_ret_i32(syscall1(SYS_SCHED_GET_PRIORITY_MAX, policy as usize)) }
}

#[no_mangle]
pub unsafe extern "C" fn sched_get_priority_min(policy: i32) -> i32 {
    unsafe { syscall_ret_i32(syscall1(SYS_SCHED_GET_PRIORITY_MIN, policy as usize)) }
}

#[no_mangle]
pub unsafe extern "C" fn sched_rr_get_interval(pid: i32, interval: *mut Timespec) -> i32 {
    unsafe {
        syscall_ret_i32(syscall2(
            SYS_SCHED_RR_GET_INTERVAL,
            pid as usize,
            interval as usize,
        ))
    }
}

#[cfg(target_arch = "loongarch64")]
#[no_mangle]
pub unsafe extern "C" fn epoll_wait(
    epfd: i32,
    events: *mut u8,
    maxevents: i32,
    timeout: i32,
) -> i32 {
    unsafe {
        syscall_ret_i32(syscall6(
            SYS_EPOLL_PWAIT,
            epfd as usize,
            events as usize,
            maxevents as usize,
            timeout as isize as usize,
            0,
            0,
        ))
    }
}

#[no_mangle]
pub unsafe extern "C" fn readlinkat(
    dirfd: i32,
    path: *const u8,
    buf: *mut u8,
    bufsiz: usize,
) -> isize {
    let ret = unsafe {
        syscall4(
            SYS_READLINKAT,
            dirfd as usize,
            path as usize,
            buf as usize,
            bufsiz,
        )
    };
    if let Some(err) = syscall_error(ret) {
        unsafe { set_errno(err) };
        -1
    } else {
        ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn readlink(path: *const u8, buf: *mut u8, bufsiz: usize) -> isize {
    unsafe { readlinkat(AT_FDCWD, path, buf, bufsiz) }
}

static BRK_LOCK: AtomicBool = AtomicBool::new(false);
static CURRENT_BRK: AtomicUsize = AtomicUsize::new(0);

struct BrkGuard;

impl BrkGuard {
    fn lock() -> Self {
        while BRK_LOCK
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        Self
    }
}

impl Drop for BrkGuard {
    fn drop(&mut self) {
        BRK_LOCK.store(false, Ordering::Release);
    }
}

#[no_mangle]
pub unsafe extern "C" fn brk(addr: *mut u8) -> i32 {
    let _guard = BrkGuard::lock();
    let target = addr as usize;
    let ret = unsafe { syscall1(SYS_BRK, target) } as usize;
    if ret == target {
        CURRENT_BRK.store(ret, Ordering::Release);
        0
    } else {
        unsafe { set_errno(ENOMEM) };
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn sbrk(increment: isize) -> *mut u8 {
    let _guard = BrkGuard::lock();
    let mut current = CURRENT_BRK.load(Ordering::Acquire);
    if current == 0 {
        let ret = unsafe { syscall1(SYS_BRK, 0) };
        if syscall_error(ret).is_some() {
            unsafe { set_errno(ENOMEM) };
            return (-1isize) as *mut u8;
        }
        current = ret as usize;
        CURRENT_BRK.store(current, Ordering::Release);
    }
    if increment == 0 {
        return current as *mut u8;
    }
    let target = if increment > 0 {
        current.checked_add(increment as usize)
    } else if increment == isize::MIN {
        None
    } else {
        current.checked_sub((-increment) as usize)
    };
    let Some(target) = target else {
        unsafe { set_errno(ENOMEM) };
        return (-1isize) as *mut u8;
    };
    let ret = unsafe { syscall1(SYS_BRK, target) } as usize;
    if ret == target {
        let old = current;
        CURRENT_BRK.store(ret, Ordering::Release);
        old as *mut u8
    } else {
        unsafe { set_errno(ENOMEM) };
        (-1isize) as *mut u8
    }
}

#[no_mangle]
pub unsafe extern "C" fn nice(increment: i32) -> i32 {
    let priority = unsafe { syscall2(SYS_GETPRIORITY, PRIO_PROCESS, 0) };
    if let Some(err) = syscall_error(priority) {
        unsafe { set_errno(err) };
        return -1;
    }
    let current_nice = 20i32.saturating_sub(priority as i32);
    let target_nice = current_nice.saturating_add(increment);
    let ret = unsafe { syscall3(SYS_SETPRIORITY, PRIO_PROCESS, 0, target_nice as usize) };
    if let Some(mut err) = syscall_error(ret) {
        if err == EACCES {
            err = EPERM;
        }
        unsafe { set_errno(err) };
        return -1;
    }
    let priority = unsafe { syscall2(SYS_GETPRIORITY, PRIO_PROCESS, 0) };
    if let Some(err) = syscall_error(priority) {
        unsafe { set_errno(err) };
        return -1;
    }
    20i32.saturating_sub(priority as i32)
}

#[repr(C)]
struct UtsName {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

#[no_mangle]
pub unsafe extern "C" fn gethostname(name: *mut u8, len: usize) -> i32 {
    if name.is_null() {
        unsafe { set_errno(EFAULT) };
        return -1;
    }
    let mut uts = UtsName {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let ret = unsafe { syscall1(SYS_UNAME, (&mut uts as *mut UtsName) as usize) };
    if let Some(err) = syscall_error(ret) {
        unsafe { set_errno(err) };
        return -1;
    }
    let mut used = 0usize;
    while used < uts.nodename.len() && uts.nodename[used] != 0 {
        used += 1;
    }
    if len <= used {
        unsafe { set_errno(ENAMETOOLONG) };
        return -1;
    }
    let mut index = 0usize;
    while index < used {
        unsafe {
            *name.add(index) = uts.nodename[index];
        }
        index += 1;
    }
    unsafe {
        *name.add(used) = 0;
    }
    0
}
