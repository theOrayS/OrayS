use core::cmp;
use core::mem::size_of;

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use linux_raw_sys::general;
use std::vec::Vec;

use super::linux_abi::{
    DEFAULT_NOFILE_LIMIT, NR_OPEN_LIMIT, RLIMIT_NOFILE_RESOURCE, RLIMIT_STACK_RESOURCE,
    USER_STACK_SIZE, neg_errno,
};
use super::task_registry::{
    live_user_process_entries, user_thread_entries_by_process_group,
    user_thread_entry_by_process_pid,
};
use super::user_memory::{
    clear_user_bytes, read_user_bytes, read_user_value, validate_user_read, validate_user_write,
    write_user_bytes, write_user_value,
};
use super::{UserProcess, task_context::current_tid};

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct UserRlimit {
    rlim_cur: u64,
    rlim_max: u64,
}

impl UserRlimit {
    pub(super) fn current(&self) -> u64 {
        self.rlim_cur
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct UserSchedParam {
    sched_priority: i32,
}

#[derive(Clone, Copy)]
pub(super) struct UserSchedState {
    policy: i32,
    param: UserSchedParam,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct UserSchedAttr {
    size: u32,
    sched_policy: u32,
    sched_flags: u64,
    sched_nice: i32,
    sched_priority: u32,
    sched_runtime: u64,
    sched_deadline: u64,
    sched_period: u64,
    sched_util_min: u32,
    sched_util_max: u32,
}

const SCHED_ATTR_BASE_SIZE: usize = 48;

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

impl UserProcess {
    pub(super) fn get_rlimit(&self, resource: u32) -> UserRlimit {
        self.rlimits
            .lock()
            .get(&resource)
            .copied()
            .unwrap_or_else(|| default_rlimit(resource))
    }

    pub(super) fn set_rlimit(&self, resource: u32, limit: UserRlimit) {
        self.rlimits.lock().insert(resource, limit);
    }
}

pub(super) fn rlimit_is_valid(limit: UserRlimit) -> bool {
    limit.rlim_cur <= limit.rlim_max
}

fn resource_is_valid(resource: u32) -> bool {
    resource <= general::RLIMIT_RTTIME
}

pub(super) fn prlimit_target_valid(pid: i32) -> bool {
    pid == 0 || pid == current_tid()
}

pub(super) fn default_sched_param() -> UserSchedParam {
    UserSchedParam { sched_priority: 0 }
}

pub(super) fn default_sched_state() -> UserSchedState {
    UserSchedState {
        policy: 0,
        param: default_sched_param(),
    }
}

impl UserProcess {
    pub(super) fn get_sched_state(&self) -> UserSchedState {
        *self.sched_state.lock()
    }

    pub(super) fn set_sched_state(&self, state: UserSchedState) {
        *self.sched_state.lock() = state;
    }

    pub(super) fn nice(&self) -> i32 {
        self.nice.load(core::sync::atomic::Ordering::Acquire)
    }

    pub(super) fn set_nice(&self, nice: i32) {
        self.nice.store(
            nice.clamp(MIN_NICE, MAX_NICE),
            core::sync::atomic::Ordering::Release,
        );
    }
}

const MIN_NICE: i32 = -20;
const MAX_NICE: i32 = 19;
const DEFAULT_NICE: i32 = 0;

fn clamp_nice(nice: i32) -> i32 {
    nice.clamp(MIN_NICE, MAX_NICE)
}

fn linux_priority_from_nice(nice: i32) -> isize {
    (20 - clamp_nice(nice)) as isize
}

fn priority_targets(
    process: &UserProcess,
    which: u32,
    who: i32,
) -> Result<Vec<UserProcessRef>, LinuxError> {
    let mut targets = Vec::new();
    match which {
        general::PRIO_PROCESS => {
            let target = if who == 0 { process.pid() } else { who };
            if target < 0 {
                return Err(LinuxError::ESRCH);
            }
            if target == process.pid() {
                targets.push(UserProcessRef::Borrowed(process));
            } else if target == 1 {
                targets.push(UserProcessRef::InitProcess);
            } else if let Some(entry) = user_thread_entry_by_process_pid(target) {
                targets.push(UserProcessRef::Owned(entry.process));
            }
        }
        general::PRIO_PGRP => {
            let target = if who == 0 { process.pgid() } else { who };
            if target < 0 {
                return Err(LinuxError::ESRCH);
            }
            for entry in user_thread_entries_by_process_group(target) {
                targets.push(UserProcessRef::Owned(entry.process));
            }
        }
        general::PRIO_USER => {
            let target = if who == 0 { process.uid() } else { who as u32 };
            if who < 0 {
                return Err(LinuxError::ESRCH);
            }
            if process.uid() == target {
                targets.push(UserProcessRef::Borrowed(process));
            }
            for entry in live_user_process_entries() {
                if entry.process.pid() != process.pid() && entry.process.uid() == target {
                    targets.push(UserProcessRef::Owned(entry.process));
                }
            }
        }
        _ => return Err(LinuxError::EINVAL),
    }
    if targets.is_empty() {
        Err(LinuxError::ESRCH)
    } else {
        Ok(targets)
    }
}

enum UserProcessRef<'a> {
    Borrowed(&'a UserProcess),
    Owned(std::sync::Arc<UserProcess>),
    InitProcess,
}

impl UserProcessRef<'_> {
    fn nice(&self) -> i32 {
        match self {
            UserProcessRef::Borrowed(process) => process.nice(),
            UserProcessRef::Owned(process) => process.nice(),
            UserProcessRef::InitProcess => DEFAULT_NICE,
        }
    }

    fn uid(&self) -> u32 {
        match self {
            UserProcessRef::Borrowed(process) => process.uid(),
            UserProcessRef::Owned(process) => process.uid(),
            UserProcessRef::InitProcess => 0,
        }
    }

    fn set_nice(&self, nice: i32) {
        match self {
            UserProcessRef::Borrowed(process) => process.set_nice(nice),
            UserProcessRef::Owned(process) => process.set_nice(nice),
            UserProcessRef::InitProcess => {}
        }
    }
}

pub(super) fn sys_getpriority(process: &UserProcess, which: u32, who: i32) -> isize {
    let targets = match priority_targets(process, which, who) {
        Ok(targets) => targets,
        Err(err) => return neg_errno(err),
    };
    let best = targets
        .iter()
        .map(|target| target.nice())
        .min()
        .unwrap_or(DEFAULT_NICE);
    linux_priority_from_nice(best)
}

pub(super) fn sys_setpriority(process: &UserProcess, which: u32, who: i32, nice: i32) -> isize {
    let targets = match priority_targets(process, which, who) {
        Ok(targets) => targets,
        Err(err) => return neg_errno(err),
    };
    let nice = clamp_nice(nice);
    for target in &targets {
        if process.uid() != 0 && process.uid() != target.uid() {
            return neg_errno(LinuxError::EPERM);
        }
        if process.uid() != 0 && nice < target.nice() {
            return neg_errno(LinuxError::EACCES);
        }
    }
    for target in targets {
        target.set_nice(nice);
    }
    0
}

pub(super) fn sched_param_accepts_policy(policy: i32, param: UserSchedParam) -> bool {
    match policy as u32 {
        0 if param.sched_priority == 0 => true,
        general::SCHED_FIFO | general::SCHED_RR if (1..=99).contains(&param.sched_priority) => true,
        general::SCHED_BATCH | general::SCHED_IDLE if param.sched_priority == 0 => true,
        _ => false,
    }
}

fn sched_priority_bounds(policy: i32) -> Option<(i32, i32)> {
    match policy as u32 {
        0 | general::SCHED_BATCH | general::SCHED_IDLE | general::SCHED_DEADLINE => Some((0, 0)),
        general::SCHED_FIFO | general::SCHED_RR => Some((1, 99)),
        _ => None,
    }
}

fn sched_target_state(process: &UserProcess, pid: i32) -> Result<UserSchedState, LinuxError> {
    if pid < 0 {
        return Err(LinuxError::EINVAL);
    }
    if pid == 0 || pid == current_tid() || pid == process.pid() {
        return Ok(process.get_sched_state());
    }
    user_thread_entry_by_process_pid(pid)
        .map(|entry| entry.process.get_sched_state())
        .ok_or(LinuxError::ESRCH)
}

fn set_sched_target_state(
    process: &UserProcess,
    pid: i32,
    state: UserSchedState,
) -> Result<(), LinuxError> {
    if pid < 0 {
        return Err(LinuxError::EINVAL);
    }
    if pid == 0 || pid == current_tid() || pid == process.pid() {
        process.set_sched_state(state);
        return Ok(());
    }
    let entry = user_thread_entry_by_process_pid(pid).ok_or(LinuxError::ESRCH)?;
    entry.process.set_sched_state(state);
    Ok(())
}

pub(super) fn is_same_sched_target(process: &UserProcess, pid: i32) -> bool {
    sched_target_state(process, pid).is_ok()
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
    if param == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let mut state = match sched_target_state(process, pid) {
        Ok(state) => state,
        Err(err) => return neg_errno(err),
    };
    match read_user_value::<UserSchedParam>(process, param) {
        Ok(value) if sched_param_accepts_policy(state.policy, value) => {
            state.param = value;
            match set_sched_target_state(process, pid, state) {
                Ok(()) => 0,
                Err(err) => neg_errno(err),
            }
        }
        Ok(_) => neg_errno(LinuxError::EINVAL),
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_sched_getparam(process: &UserProcess, pid: i32, param: usize) -> isize {
    if param == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let value = match sched_target_state(process, pid) {
        Ok(state) => state.param,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, param, &value)
}

pub(super) fn sys_sched_setscheduler(
    process: &UserProcess,
    pid: i32,
    policy: i32,
    param: usize,
) -> isize {
    if param == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = sched_target_state(process, pid) {
        return neg_errno(err);
    }
    let param = match read_user_value::<UserSchedParam>(process, param) {
        Ok(param) => param,
        Err(err) => return neg_errno(err),
    };
    if sched_param_accepts_policy(policy, param) {
        match set_sched_target_state(process, pid, UserSchedState { policy, param }) {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        }
    } else {
        neg_errno(LinuxError::EINVAL)
    }
}

pub(super) fn sys_sched_getscheduler(process: &UserProcess, pid: i32) -> isize {
    match sched_target_state(process, pid) {
        Ok(state) => state.policy as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_sched_get_priority_max(policy: i32) -> isize {
    match sched_priority_bounds(policy) {
        Some((_, max)) => max as isize,
        None => neg_errno(LinuxError::EINVAL),
    }
}

pub(super) fn sys_sched_get_priority_min(policy: i32) -> isize {
    match sched_priority_bounds(policy) {
        Some((min, _)) => min as isize,
        None => neg_errno(LinuxError::EINVAL),
    }
}

pub(super) fn sys_sched_rr_get_interval(process: &UserProcess, pid: i32, interval: usize) -> isize {
    let state = match sched_target_state(process, pid) {
        Ok(state) => state,
        Err(err) => return neg_errno(err),
    };
    let tv_nsec = if state.policy as u32 == general::SCHED_FIFO {
        0
    } else {
        10_000_000
    };
    let quantum = general::timespec { tv_sec: 0, tv_nsec };
    write_user_value(process, interval, &quantum)
}

fn sched_attr_from_state(state: UserSchedState) -> UserSchedAttr {
    UserSchedAttr {
        size: size_of::<UserSchedAttr>() as u32,
        sched_policy: state.policy as u32,
        sched_flags: 0,
        sched_nice: 0,
        sched_priority: state.param.sched_priority as u32,
        sched_runtime: 0,
        sched_deadline: 0,
        sched_period: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    }
}

fn sched_state_from_attr(attr: UserSchedAttr) -> Result<UserSchedState, LinuxError> {
    let param = UserSchedParam {
        sched_priority: attr.sched_priority as i32,
    };
    let policy = attr.sched_policy as i32;
    if attr.sched_flags != 0 || !sched_param_accepts_policy(policy, param) {
        return Err(LinuxError::EINVAL);
    }
    Ok(UserSchedState { policy, param })
}

pub(super) fn sys_sched_getattr(
    process: &UserProcess,
    pid: i32,
    attr: usize,
    size: usize,
    flags: usize,
) -> isize {
    let state = match sched_target_state(process, pid) {
        Ok(state) => state,
        Err(err) => return neg_errno(err),
    };
    if flags != 0 || attr == 0 || size < SCHED_ATTR_BASE_SIZE {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_write(process, attr, size) {
        return neg_errno(err);
    }
    let value = sched_attr_from_state(state);
    let bytes = unsafe {
        core::slice::from_raw_parts(
            &value as *const UserSchedAttr as *const u8,
            size_of::<UserSchedAttr>(),
        )
    };
    let copy_len = cmp::min(size, bytes.len());
    if let Err(err) = clear_user_bytes(process, attr, size) {
        return neg_errno(err);
    }
    match write_user_bytes(process, attr, &bytes[..copy_len]) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_sched_setattr(
    process: &UserProcess,
    pid: i32,
    attr: usize,
    flags: usize,
) -> isize {
    if let Err(err) = sched_target_state(process, pid) {
        return neg_errno(err);
    }
    if flags != 0 || attr == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let user_size = match read_user_value::<u32>(process, attr) {
        Ok(size) => size as usize,
        Err(err) => return neg_errno(err),
    };
    if user_size < SCHED_ATTR_BASE_SIZE {
        return neg_errno(LinuxError::EINVAL);
    }
    let read_len = cmp::min(user_size, size_of::<UserSchedAttr>());
    let user_bytes = match read_user_bytes(process, attr, read_len) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    let mut attr_bytes = [0u8; size_of::<UserSchedAttr>()];
    attr_bytes[..user_bytes.len()].copy_from_slice(&user_bytes);
    let sched_attr =
        unsafe { core::ptr::read_unaligned(attr_bytes.as_ptr() as *const UserSchedAttr) };
    match sched_state_from_attr(sched_attr) {
        Ok(state) => match set_sched_target_state(process, pid, state) {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        },
        Err(err) => neg_errno(err),
    }
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
    if !resource_is_valid(resource) {
        return neg_errno(LinuxError::EINVAL);
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
        if resource == RLIMIT_NOFILE_RESOURCE && limit.rlim_max > NR_OPEN_LIMIT {
            return neg_errno(LinuxError::EPERM);
        }
        let current = process.get_rlimit(resource);
        if process.uid() != 0 && limit.rlim_max > current.rlim_max {
            return neg_errno(LinuxError::EPERM);
        }
        process.set_rlimit(resource, limit);
    }

    0
}

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
pub(super) fn sys_getrlimit(process: &UserProcess, resource: u32, old_limit: usize) -> isize {
    sys_prlimit64(process, 0, resource, 0, old_limit)
}

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
pub(super) fn sys_setrlimit(process: &UserProcess, resource: u32, new_limit: usize) -> isize {
    sys_prlimit64(process, 0, resource, new_limit, 0)
}
