use core::cmp;
use core::mem::size_of;

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use linux_raw_sys::general;

use super::linux_abi::{
    DEFAULT_NOFILE_LIMIT, RLIMIT_NOFILE_RESOURCE, RLIMIT_STACK_RESOURCE, USER_STACK_SIZE, neg_errno,
};
use super::user_memory::{
    clear_user_bytes, read_user_value, validate_user_read, write_user_bytes, write_user_value,
};
use super::{UserProcess, task_context::current_tid};

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct UserRlimit {
    rlim_cur: u64,
    rlim_max: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct UserSchedParam {
    sched_priority: i32,
}

pub(super) fn default_rlimit(resource: u32) -> UserRlimit {
    match resource {
        RLIMIT_STACK_RESOURCE => UserRlimit {
            rlim_cur: USER_STACK_SIZE as u64,
            rlim_max: USER_STACK_SIZE as u64,
        },
        RLIMIT_NOFILE_RESOURCE => UserRlimit {
            rlim_cur: DEFAULT_NOFILE_LIMIT,
            rlim_max: DEFAULT_NOFILE_LIMIT,
        },
        _ => UserRlimit {
            rlim_cur: u64::MAX,
            rlim_max: u64::MAX,
        },
    }
}

pub(super) fn rlimit_is_valid(limit: UserRlimit) -> bool {
    limit.rlim_cur <= limit.rlim_max
}

pub(super) fn prlimit_target_valid(pid: i32) -> bool {
    pid == 0 || pid == current_tid()
}

pub(super) fn default_sched_param() -> UserSchedParam {
    UserSchedParam { sched_priority: 0 }
}

pub(super) fn sched_param_accepts_setparam(param: UserSchedParam) -> bool {
    param.sched_priority == 0
}

pub(super) fn sched_param_accepts_policy(policy: i32, param: UserSchedParam) -> bool {
    match policy as u32 {
        0 if param.sched_priority == 0 => true,
        general::SCHED_FIFO | general::SCHED_RR if (1..=99).contains(&param.sched_priority) => true,
        general::SCHED_BATCH | general::SCHED_IDLE if param.sched_priority == 0 => true,
        _ => false,
    }
}

pub(super) fn is_same_sched_target(process: &UserProcess, pid: i32) -> bool {
    pid == 0 || pid == current_tid() || pid == process.pid()
}

pub(super) fn sched_affinity_accepts_current_cpu(first_mask_byte: u8) -> bool {
    first_mask_byte & 1 != 0
}

pub(super) fn sched_affinity_result_len(cpusetsize: usize) -> usize {
    cmp::min(cpusetsize, size_of::<usize>())
}

pub(super) fn sys_sched_yield(_tf: &TrapFrame) -> isize {
    axtask::yield_now();
    0
}

pub(super) fn sys_sched_setparam(process: &UserProcess, pid: i32, param: usize) -> isize {
    if !is_same_sched_target(process, pid) {
        return neg_errno(LinuxError::ESRCH);
    }
    if param == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    match read_user_value::<UserSchedParam>(process, param) {
        Ok(value) if sched_param_accepts_setparam(value) => 0,
        Ok(_) => neg_errno(LinuxError::EINVAL),
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_sched_getparam(process: &UserProcess, pid: i32, param: usize) -> isize {
    if !is_same_sched_target(process, pid) {
        return neg_errno(LinuxError::ESRCH);
    }
    if param == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let value = default_sched_param();
    write_user_value(process, param, &value)
}

pub(super) fn sys_sched_setscheduler(
    process: &UserProcess,
    pid: i32,
    policy: i32,
    param: usize,
) -> isize {
    if !is_same_sched_target(process, pid) {
        return neg_errno(LinuxError::ESRCH);
    }
    if param == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let param = match read_user_value::<UserSchedParam>(process, param) {
        Ok(param) => param,
        Err(err) => return neg_errno(err),
    };
    if sched_param_accepts_policy(policy, param) {
        0
    } else {
        neg_errno(LinuxError::EINVAL)
    }
}

pub(super) fn sys_sched_getscheduler(process: &UserProcess, pid: i32) -> isize {
    if !is_same_sched_target(process, pid) {
        return neg_errno(LinuxError::ESRCH);
    }
    0
}

pub(super) fn sys_sched_setaffinity(
    process: &UserProcess,
    pid: i32,
    cpusetsize: usize,
    mask: usize,
) -> isize {
    if !is_same_sched_target(process, pid) {
        return neg_errno(LinuxError::ESRCH);
    }
    if cpusetsize == 0 || mask == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_read(process, mask, cpusetsize) {
        return neg_errno(err);
    }
    match read_user_value::<u8>(process, mask) {
        Ok(first) if sched_affinity_accepts_current_cpu(first) => 0,
        Ok(_) => neg_errno(LinuxError::EINVAL),
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_sched_getaffinity(
    process: &UserProcess,
    pid: i32,
    cpusetsize: usize,
    mask: usize,
) -> isize {
    if !is_same_sched_target(process, pid) {
        return neg_errno(LinuxError::ESRCH);
    }
    if cpusetsize == 0 || mask == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = clear_user_bytes(process, mask, cpusetsize) {
        return neg_errno(err);
    }
    if let Err(err) = write_user_bytes(process, mask, &[1]) {
        return neg_errno(err);
    }
    sched_affinity_result_len(cpusetsize) as isize
}

pub(super) fn sys_prlimit64(
    process: &UserProcess,
    pid: i32,
    resource: u32,
    new_limit: usize,
    old_limit: usize,
) -> isize {
    if !prlimit_target_valid(pid) {
        return neg_errno(LinuxError::ESRCH);
    }

    if old_limit != 0 {
        let current = process.get_rlimit(resource);
        let ret = write_user_value(process, old_limit, &current);
        if ret != 0 {
            return ret;
        }
    }

    if new_limit != 0 {
        let limit = match read_user_value::<UserRlimit>(process, new_limit) {
            Ok(limit) => limit,
            Err(err) => return neg_errno(err),
        };
        if !rlimit_is_valid(limit) {
            return neg_errno(LinuxError::EINVAL);
        }
        process.set_rlimit(resource, limit);
    }

    0
}
