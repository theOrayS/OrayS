use core::{cmp, mem::size_of};
use std::string::String;

use axalloc::global_allocator;
use axerrno::LinuxError;
use linux_raw_sys::{general, system};

use super::task_registry::live_user_thread_count;
use super::time_abi::{USER_HZ, process_times};
use super::user_memory::{
    read_user_bytes, validate_user_write, write_user_bytes, write_user_value,
};
use super::{UserProcess, neg_errno};

pub(super) enum SyslogAction {
    Close,
    Open,
    Read,
    ReadAll,
    ReadClear,
    Clear,
    ConsoleOff,
    ConsoleOn,
    SizeBuffer,
    ConsoleLevel,
    Invalid,
}

pub(super) fn syslog_action(log_type: i32) -> SyslogAction {
    match log_type {
        // SYSLOG_ACTION_CLOSE / OPEN.
        0 => SyslogAction::Close,
        1 => SyslogAction::Open,
        // SYSLOG_ACTION_READ. The kernel log is modelled as empty, but the
        // read operation still validates arguments and privileged access.
        2 => SyslogAction::Read,
        // SYSLOG_ACTION_READ_ALL and READ_CLEAR. READ_ALL can expose an empty
        // kernel log; READ_CLEAR needs a mutable ring-buffer backend and is
        // rejected explicitly below until that backend exists.
        3 => SyslogAction::ReadAll,
        4 => SyslogAction::ReadClear,
        // SYSLOG_ACTION_CLEAR / CONSOLE_OFF / CONSOLE_ON.
        5 => SyslogAction::Clear,
        6 => SyslogAction::ConsoleOff,
        7 => SyslogAction::ConsoleOn,
        // SYSLOG_ACTION_SIZE_BUFFER.
        10 => SyslogAction::SizeBuffer,
        // SYSLOG_ACTION_CONSOLE_LEVEL uses len as the requested 1..=8 level.
        8 => SyslogAction::ConsoleLevel,
        _ => SyslogAction::Invalid,
    }
}

fn unsupported_privileged_syslog(process: &UserProcess) -> isize {
    if process.uid() != 0 {
        return neg_errno(LinuxError::EPERM);
    }
    neg_errno(LinuxError::EOPNOTSUPP)
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

fn timeval_from_ticks(ticks: isize) -> general::__kernel_old_timeval {
    let ticks = ticks.max(0) as i128;
    let user_hz = USER_HZ as i128;
    general::__kernel_old_timeval {
        tv_sec: (ticks / user_hz) as _,
        tv_usec: ((ticks % user_hz) * 1_000_000 / user_hz) as _,
    }
}

fn rusage_target_valid(who: i32) -> bool {
    who == general::RUSAGE_SELF as i32
        || who == general::RUSAGE_THREAD as i32
        || who == general::RUSAGE_CHILDREN
}

fn process_rusage(process: &UserProcess, who: i32) -> Result<general::rusage, LinuxError> {
    if !rusage_target_valid(who) {
        return Err(LinuxError::EINVAL);
    }
    let mut value = default_rusage();
    let times = process_times(process);
    match who {
        who if who == general::RUSAGE_CHILDREN => {
            value.ru_utime = timeval_from_ticks(times.tms_cutime as isize);
            value.ru_stime = timeval_from_ticks(times.tms_cstime as isize);
            value.ru_maxrss = process.child_maxrss_kb() as _;
        }
        _ => {
            value.ru_utime = timeval_from_ticks(times.tms_utime as isize);
            value.ru_stime = timeval_from_ticks(times.tms_stime as isize);
            value.ru_maxrss = process.self_maxrss_kb() as _;
        }
    }
    Ok(value)
}

pub(super) fn write_default_rusage(process: &UserProcess, who: i32, usage: usize) -> isize {
    let value = match process_rusage(process, who) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
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
    let mut value = default_utsname();
    let hostname = process.hostname();
    write_c_string(&mut value.nodename, hostname.as_bytes());
    let domainname = process.domainname();
    write_c_string(&mut value.domainname, domainname.as_bytes());
    write_user_value(process, buf, &value)
}

pub(super) fn sys_syslog(process: &UserProcess, log_type: i32, buf: usize, len: usize) -> isize {
    match syslog_action(log_type) {
        SyslogAction::Close | SyslogAction::Open => unsupported_privileged_syslog(process),
        SyslogAction::Read => {
            if (len as isize) < 0 || buf == 0 {
                return neg_errno(LinuxError::EINVAL);
            }
            if process.uid() != 0 {
                return neg_errno(LinuxError::EPERM);
            }
            if len > 0 {
                if let Err(err) = validate_user_write(process, buf, len) {
                    return neg_errno(err);
                }
            }
            0
        }
        SyslogAction::ReadAll => {
            if (len as isize) < 0 {
                return neg_errno(LinuxError::EINVAL);
            }
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
        SyslogAction::ReadClear => {
            if (len as isize) < 0 {
                return neg_errno(LinuxError::EINVAL);
            }
            unsupported_privileged_syslog(process)
        }
        SyslogAction::SizeBuffer => 0,
        SyslogAction::Clear | SyslogAction::ConsoleOff | SyslogAction::ConsoleOn => {
            unsupported_privileged_syslog(process)
        }
        SyslogAction::ConsoleLevel => {
            if (len as isize) < 0 || !(1..=8).contains(&len) {
                return neg_errno(LinuxError::EINVAL);
            }
            unsupported_privileged_syslog(process)
        }
        SyslogAction::Invalid => neg_errno(LinuxError::EINVAL),
    }
}

pub(super) fn sys_getcpu(process: &UserProcess, cpu: usize, node: usize) -> isize {
    let value = 0u32;
    if cpu != 0 {
        let ret = write_user_value(process, cpu, &value);
        if ret != 0 {
            return ret;
        }
    }
    if node != 0 {
        let ret = write_user_value(process, node, &value);
        if ret != 0 {
            return ret;
        }
    }
    0
}

pub(super) fn sys_getrusage(process: &UserProcess, who: i32, usage: usize) -> isize {
    write_default_rusage(process, who, usage)
}

pub(super) fn sys_uname(process: &UserProcess, buf: usize) -> isize {
    write_default_utsname(process, buf)
}

const HOST_NAME_MAX: usize = 64;
const TASK_COMM_LEN: usize = 16;
const PR_SET_PDEATHSIG: usize = 1;
const PR_GET_PDEATHSIG: usize = 2;
const PR_SET_NAME: usize = 15;
const PR_GET_NAME: usize = 16;
const PR_CAPBSET_READ: usize = 23;
const PR_CAPBSET_DROP: usize = 24;
const PR_SET_TIMERSLACK: usize = 29;
const PR_GET_TIMERSLACK: usize = 30;

pub(super) fn sys_sethostname(process: &UserProcess, name: usize, len: usize) -> isize {
    if len > HOST_NAME_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if process.uid() != 0 {
        return neg_errno(LinuxError::EPERM);
    }
    if len > 0 && name == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let hostname = if len > 0 {
        let Ok(bytes) = read_user_bytes(process, name, len) else {
            return neg_errno(LinuxError::EFAULT);
        };
        String::from_utf8_lossy(&bytes).into_owned()
    } else {
        String::new()
    };
    process.set_hostname(hostname);
    0
}

pub(super) fn sys_setdomainname(process: &UserProcess, name: usize, len: usize) -> isize {
    if len > HOST_NAME_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if process.uid() != 0 {
        return neg_errno(LinuxError::EPERM);
    }
    if len > 0 && name == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let domainname = if len > 0 {
        let Ok(bytes) = read_user_bytes(process, name, len) else {
            return neg_errno(LinuxError::EFAULT);
        };
        String::from_utf8_lossy(&bytes).into_owned()
    } else {
        String::new()
    };
    process.set_domainname(domainname);
    0
}

pub(super) fn sys_prctl(
    process: &UserProcess,
    option: usize,
    arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match option {
        PR_SET_PDEATHSIG => {
            if arg2 > 64 {
                return neg_errno(LinuxError::EINVAL);
            }
            process
                .parent_death_signal
                .store(arg2 as i32, core::sync::atomic::Ordering::Release);
            0
        }
        PR_GET_PDEATHSIG => {
            if arg2 == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let value = process
                .parent_death_signal
                .load(core::sync::atomic::Ordering::Acquire);
            write_user_value(process, arg2, &value)
        }
        PR_SET_NAME => {
            if arg2 == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let bytes = match read_user_bytes(process, arg2, TASK_COMM_LEN) {
                Ok(bytes) => bytes,
                Err(err) => return neg_errno(err),
            };
            let name_len = bytes
                .iter()
                .position(|&byte| byte == 0)
                .unwrap_or(TASK_COMM_LEN - 1)
                .min(TASK_COMM_LEN - 1);
            process.set_prctl_name(String::from_utf8_lossy(&bytes[..name_len]).into_owned());
            0
        }
        PR_GET_NAME => {
            if arg2 == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let name = process.prctl_name();
            let mut bytes = [0u8; TASK_COMM_LEN];
            let copy_len = cmp::min(name.as_bytes().len(), TASK_COMM_LEN - 1);
            bytes[..copy_len].copy_from_slice(&name.as_bytes()[..copy_len]);
            write_user_bytes(process, arg2, &bytes).map_or_else(|err| neg_errno(err), |_| 0)
        }
        PR_CAPBSET_READ => match process.capability_in_bounding_set(arg2 as u32) {
            Some(true) => 1,
            Some(false) => 0,
            None => neg_errno(LinuxError::EINVAL),
        },
        PR_CAPBSET_DROP => match process.drop_capability_from_bounding_set(arg2 as u32) {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        },
        PR_SET_TIMERSLACK => {
            let slack = if arg2 == 0 {
                process.default_timer_slack_ns()
            } else {
                arg2 as u64
            };
            process.set_timer_slack_ns(slack);
            0
        }
        PR_GET_TIMERSLACK => cmp::min(process.timer_slack_ns(), isize::MAX as u64) as isize,
        _ => neg_errno(LinuxError::EINVAL),
    }
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
