use core::{cmp, mem::size_of};

use axalloc::global_allocator;
use axerrno::LinuxError;
use linux_raw_sys::{general, system};

use super::task_registry::live_user_thread_count;
use super::user_memory::{validate_user_write, write_user_bytes, write_user_value};
use super::{neg_errno, UserProcess};

pub(super) enum SyslogAction {
    EmptyRead,
    SizeBuffer,
    ConsoleControl,
    Invalid,
}

pub(super) fn syslog_action(log_type: i32) -> SyslogAction {
    match log_type {
        // SYSLOG_ACTION_READ_ALL and READ_CLEAR. Expose an empty kernel log.
        3 | 4 => SyslogAction::EmptyRead,
        // SYSLOG_ACTION_SIZE_BUFFER.
        10 => SyslogAction::SizeBuffer,
        // Console control operations are accepted as no-ops.
        6..=8 => SyslogAction::ConsoleControl,
        _ => SyslogAction::Invalid,
    }
}

pub(super) fn syslog_empty_read_bytes(buf: usize, len: usize) -> Option<&'static [u8]> {
    if len > 0 && buf != 0 {
        Some(&[0])
    } else {
        None
    }
}

fn default_rusage() -> general::rusage {
    unsafe { core::mem::zeroed() }
}

fn rusage_target_valid(who: i32) -> bool {
    who == general::RUSAGE_SELF as i32
        || who == general::RUSAGE_THREAD as i32
        || who == general::RUSAGE_CHILDREN
}

pub(super) fn write_default_rusage(process: &UserProcess, who: i32, usage: usize) -> isize {
    if !rusage_target_valid(who) {
        return neg_errno(LinuxError::EINVAL);
    }
    let value = default_rusage();
    write_user_value(process, usage, &value)
}

fn default_winsize() -> general::winsize {
    general::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    }
}

pub(super) fn write_default_winsize(process: &UserProcess, buf: usize) -> isize {
    let value = default_winsize();
    write_user_value(process, buf, &value)
}

fn default_utsname() -> system::new_utsname {
    let mut uts = system::new_utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    write_c_string(&mut uts.sysname, b"Linux");
    write_c_string(&mut uts.nodename, b"arceos");
    write_c_string(&mut uts.release, b"6.0.0");
    write_c_string(&mut uts.version, b"ArceOS");
    #[cfg(target_arch = "riscv64")]
    write_c_string(&mut uts.machine, b"riscv64");
    #[cfg(target_arch = "loongarch64")]
    write_c_string(&mut uts.machine, b"loongarch64");
    write_c_string(&mut uts.domainname, b"localdomain");
    uts
}

pub(super) fn write_default_utsname(process: &UserProcess, buf: usize) -> isize {
    let value = default_utsname();
    write_user_value(process, buf, &value)
}

pub(super) fn sys_syslog(process: &UserProcess, log_type: i32, buf: usize, len: usize) -> isize {
    match syslog_action(log_type) {
        SyslogAction::EmptyRead => {
            if let Some(bytes) = syslog_empty_read_bytes(buf, len) {
                if let Err(err) = validate_user_write(process, buf, len) {
                    return neg_errno(err);
                }
                if let Err(err) = write_user_bytes(process, buf, bytes) {
                    return neg_errno(err);
                }
            }
            0
        }
        SyslogAction::SizeBuffer | SyslogAction::ConsoleControl => 0,
        SyslogAction::Invalid => neg_errno(LinuxError::EINVAL),
    }
}

pub(super) fn sys_getrusage(process: &UserProcess, who: i32, usage: usize) -> isize {
    write_default_rusage(process, who, usage)
}

pub(super) fn sys_uname(process: &UserProcess, buf: usize) -> isize {
    write_default_utsname(process, buf)
}

fn default_sysinfo() -> system::sysinfo {
    let alloc = global_allocator();
    let free_pages = alloc.available_pages() as u64;
    let total_pages = (alloc.used_pages() as u64 + free_pages).max(1);
    let procs = live_user_thread_count().clamp(1, u16::MAX as usize) as u16;
    let mut info: system::sysinfo = unsafe { core::mem::zeroed() };
    info.uptime = axhal::time::monotonic_time().as_secs() as _;
    info.loads = [0; 3];
    info.totalram = total_pages as _;
    info.freeram = free_pages as _;
    info.sharedram = 0;
    info.bufferram = 0;
    info.totalswap = 0;
    info.freeswap = 0;
    info.procs = procs;
    info.totalhigh = 0;
    info.freehigh = 0;
    info.mem_unit = 4096;
    info
}

pub(super) fn sys_sysinfo(process: &UserProcess, info: usize) -> isize {
    let value = default_sysinfo();
    let bytes = unsafe {
        core::slice::from_raw_parts(
            (&value as *const system::sysinfo).cast::<u8>(),
            size_of::<system::sysinfo>(),
        )
    };
    write_user_bytes(process, info, bytes).map_or_else(|err| neg_errno(err), |_| 0)
}

trait CCharSlot: Copy {
    fn from_byte(byte: u8) -> Self;
}

impl CCharSlot for u8 {
    fn from_byte(byte: u8) -> Self {
        byte
    }
}

impl CCharSlot for i8 {
    fn from_byte(byte: u8) -> Self {
        byte as i8
    }
}

fn write_c_string<T: CCharSlot>(dst: &mut [T], src: &[u8]) {
    let len = cmp::min(dst.len().saturating_sub(1), src.len());
    for (idx, byte) in src[..len].iter().enumerate() {
        dst[idx] = T::from_byte(*byte);
    }
    if !dst.is_empty() {
        dst[len] = T::from_byte(0);
    }
}
