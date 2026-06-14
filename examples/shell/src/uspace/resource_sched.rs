use core::cmp;
use core::mem::size_of;
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use linux_raw_sys::general;
use std::vec::Vec;

use super::linux_abi::{
    neg_errno, DEFAULT_NOFILE_LIMIT, NR_OPEN_LIMIT, RLIMIT_NOFILE_RESOURCE, RLIMIT_STACK_RESOURCE,
    USER_STACK_SIZE,
};
use super::task_registry::{
    live_user_process_entries, user_thread_entries_by_process_group,
    user_thread_entry_by_process_pid, UserThreadEntry,
};
use super::user_memory::{
    clear_user_bytes, read_user_bytes, read_user_value, validate_user_read, validate_user_write,
    write_user_bytes, write_user_value,
};
use super::{task_context::current_tid, UserProcess};

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
    reset_on_fork: bool,
    sched_runtime: u64,
    sched_deadline: u64,
    sched_period: u64,
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

pub(super) fn prlimit_target_valid(process: &UserProcess, pid: i32) -> bool {
    pid == 0 || pid == current_tid() || pid == process.pid()
}

pub(super) fn default_sched_param() -> UserSchedParam {
    UserSchedParam { sched_priority: 0 }
}

pub(super) fn default_sched_state() -> UserSchedState {
    UserSchedState {
        policy: 0,
        param: default_sched_param(),
        reset_on_fork: false,
        sched_runtime: 0,
        sched_deadline: 0,
        sched_period: 0,
    }
}

pub(super) fn child_sched_state_from_parent(parent: UserSchedState) -> UserSchedState {
    if parent.reset_on_fork {
        default_sched_state()
    } else {
        parent
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
        self.nice.load(Ordering::Acquire)
    }

    pub(super) fn set_nice(&self, nice: i32) {
        self.nice
            .store(nice.clamp(MIN_NICE, MAX_NICE), Ordering::Release);
    }

    pub(super) fn ioprio(&self) -> u32 {
        self.ioprio.load(Ordering::Acquire)
    }

    pub(super) fn set_ioprio(&self, ioprio: u32) {
        self.ioprio.store(ioprio, Ordering::Release);
    }
}

const MIN_NICE: i32 = -20;
const MAX_NICE: i32 = 19;
const DEFAULT_NICE: i32 = 0;
const IOPRIO_WHO_PROCESS: u32 = 1;
const IOPRIO_WHO_PGRP: u32 = 2;
const IOPRIO_WHO_USER: u32 = 3;
const IOPRIO_CLASS_SHIFT: u32 = 13;
const IOPRIO_PRIO_MASK: u32 = (1 << IOPRIO_CLASS_SHIFT) - 1;
const IOPRIO_CLASS_NONE: u32 = 0;
const IOPRIO_CLASS_RT: u32 = 1;
const IOPRIO_CLASS_BE: u32 = 2;
const IOPRIO_CLASS_IDLE: u32 = 3;
const IOPRIO_NR_LEVELS: u32 = 8;
const DEFAULT_IOPRIO: u32 = (IOPRIO_CLASS_BE << IOPRIO_CLASS_SHIFT) | 4;
const SCHED_RESET_ON_FORK_FLAG: u32 = 0x4000_0000;
const SCHED_ATTR_FLAG_RESET_ON_FORK: u64 = 0x1;

static SYNTHETIC_INIT_NICE: AtomicI32 = AtomicI32::new(DEFAULT_NICE);
static SYNTHETIC_INIT_IOPRIO: AtomicU32 = AtomicU32::new(DEFAULT_IOPRIO);

pub(super) fn default_ioprio() -> u32 {
    DEFAULT_IOPRIO
}

fn encode_ioprio(class: u32, data: u32) -> u32 {
    (class << IOPRIO_CLASS_SHIFT) | data
}

fn ioprio_class(ioprio: u32) -> u32 {
    ioprio >> IOPRIO_CLASS_SHIFT
}

fn ioprio_data(ioprio: u32) -> u32 {
    ioprio & IOPRIO_PRIO_MASK
}

fn validate_ioprio(ioprio: u32) -> Result<(), LinuxError> {
    let class = ioprio_class(ioprio);
    let data = ioprio_data(ioprio);
    match class {
        IOPRIO_CLASS_NONE if data == 0 => Ok(()),
        IOPRIO_CLASS_RT | IOPRIO_CLASS_BE | IOPRIO_CLASS_IDLE if data < IOPRIO_NR_LEVELS => Ok(()),
        _ => Err(LinuxError::EINVAL),
    }
}

fn ioprio_rank(ioprio: u32) -> (u32, u32) {
    let class_rank = match ioprio_class(ioprio) {
        IOPRIO_CLASS_RT => 0,
        IOPRIO_CLASS_BE => 1,
        IOPRIO_CLASS_IDLE => 2,
        _ => 3,
    };
    (class_rank, ioprio_data(ioprio))
}

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
                targets.push(UserProcessRef::Owned(entry));
            }
        }
        general::PRIO_PGRP => {
            let target = if who == 0 { process.pgid() } else { who };
            if target < 0 {
                return Err(LinuxError::ESRCH);
            }
            for entry in user_thread_entries_by_process_group(target) {
                targets.push(UserProcessRef::Owned(entry));
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
                    targets.push(UserProcessRef::Owned(entry));
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

fn ioprio_targets(
    process: &UserProcess,
    which: u32,
    who: i32,
) -> Result<Vec<UserProcessRef>, LinuxError> {
    let mut targets = Vec::new();
    match which {
        IOPRIO_WHO_PROCESS => {
            let target = if who == 0 { process.pid() } else { who };
            if target < 0 {
                return Err(LinuxError::ESRCH);
            }
            if target == process.pid() || target == current_tid() {
                targets.push(UserProcessRef::Borrowed(process));
            } else if target == 1 {
                targets.push(UserProcessRef::InitProcess);
            } else if let Some(entry) = user_thread_entry_by_process_pid(target) {
                targets.push(UserProcessRef::Owned(entry));
            }
        }
        IOPRIO_WHO_PGRP => {
            let target = if who == 0 { process.pgid() } else { who };
            if target < 0 {
                return Err(LinuxError::ESRCH);
            }
            for entry in user_thread_entries_by_process_group(target) {
                targets.push(UserProcessRef::Owned(entry));
            }
        }
        IOPRIO_WHO_USER => {
            if who < 0 {
                return Err(LinuxError::ESRCH);
            }
            let target = if who == 0 { process.uid() } else { who as u32 };
            if process.uid() == target {
                targets.push(UserProcessRef::Borrowed(process));
            }
            for entry in live_user_process_entries() {
                if entry.process.pid() != process.pid() && entry.process.uid() == target {
                    targets.push(UserProcessRef::Owned(entry));
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
    Owned(UserThreadEntry),
    InitProcess,
}

impl UserProcessRef<'_> {
    fn nice(&self) -> i32 {
        match self {
            UserProcessRef::Borrowed(process) => process.nice(),
            UserProcessRef::Owned(entry) => entry.process.nice(),
            UserProcessRef::InitProcess => SYNTHETIC_INIT_NICE.load(Ordering::Acquire),
        }
    }

    fn uid(&self) -> u32 {
        match self {
            UserProcessRef::Borrowed(process) => process.uid(),
            UserProcessRef::Owned(entry) => entry.process.uid(),
            UserProcessRef::InitProcess => 0,
        }
    }

    fn set_nice(&self, nice: i32) {
        match self {
            UserProcessRef::Borrowed(process) => {
                process.set_nice(nice);
                let _ = axtask::set_priority(nice as isize);
            }
            UserProcessRef::Owned(entry) => {
                entry.process.set_nice(nice);
                let _ = axtask::set_task_priority(&entry.task, nice as isize);
            }
            UserProcessRef::InitProcess => {
                SYNTHETIC_INIT_NICE.store(clamp_nice(nice), Ordering::Release)
            }
        }
    }

    fn ioprio(&self) -> u32 {
        match self {
            UserProcessRef::Borrowed(process) => process.ioprio(),
            UserProcessRef::Owned(entry) => entry.process.ioprio(),
            UserProcessRef::InitProcess => SYNTHETIC_INIT_IOPRIO.load(Ordering::Acquire),
        }
    }

    fn set_ioprio(&self, ioprio: u32) {
        match self {
            UserProcessRef::Borrowed(process) => process.set_ioprio(ioprio),
            UserProcessRef::Owned(entry) => entry.process.set_ioprio(ioprio),
            UserProcessRef::InitProcess => SYNTHETIC_INIT_IOPRIO.store(ioprio, Ordering::Release),
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

pub(super) fn sys_ioprio_get(process: &UserProcess, which: u32, who: i32) -> isize {
    let targets = match ioprio_targets(process, which, who) {
        Ok(targets) => targets,
        Err(err) => return neg_errno(err),
    };
    targets
        .iter()
        .map(|target| target.ioprio())
        .min_by_key(|ioprio| ioprio_rank(*ioprio))
        .unwrap_or_else(default_ioprio) as isize
}

pub(super) fn sys_ioprio_set(process: &UserProcess, which: u32, who: i32, ioprio: u32) -> isize {
    if let Err(err) = validate_ioprio(ioprio) {
        return neg_errno(err);
    }
    let targets = match ioprio_targets(process, which, who) {
        Ok(targets) => targets,
        Err(err) => return neg_errno(err),
    };
    for target in &targets {
        if process.uid() != 0 && process.uid() != target.uid() {
            return neg_errno(LinuxError::EPERM);
        }
        if process.uid() != 0 && ioprio_class(ioprio) == IOPRIO_CLASS_RT {
            return neg_errno(LinuxError::EPERM);
        }
    }
    for target in targets {
        target.set_ioprio(ioprio);
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

fn sched_policy_needs_privilege(policy: i32) -> bool {
    matches!(
        policy as u32,
        general::SCHED_FIFO | general::SCHED_RR | general::SCHED_DEADLINE
    )
}

fn split_sched_policy(policy: i32) -> Result<(i32, bool), LinuxError> {
    if policy < 0 {
        return Err(LinuxError::EINVAL);
    }
    let raw_policy = policy as u32;
    let reset_on_fork = (raw_policy & SCHED_RESET_ON_FORK_FLAG) != 0;
    let base_policy = raw_policy & !SCHED_RESET_ON_FORK_FLAG;
    let base_policy = base_policy as i32;
    if sched_priority_bounds(base_policy).is_none() {
        return Err(LinuxError::EINVAL);
    }
    Ok((base_policy, reset_on_fork))
}

fn scheduler_backend_priority(process: &UserProcess, state: UserSchedState) -> isize {
    match state.policy as u32 {
        general::SCHED_FIFO | general::SCHED_RR => {
            let linux_rt_prio = state.param.sched_priority.clamp(1, 99);
            -1 - (((linux_rt_prio - 1) * 19) / 98) as isize
        }
        general::SCHED_DEADLINE => deadline_scheduler_backend_priority(state),
        general::SCHED_IDLE => 19,
        _ => process.nice() as isize,
    }
}

fn deadline_scheduler_backend_priority(state: UserSchedState) -> isize {
    // ArceOS currently exposes a priority/nice backend rather than a full EDF
    // scheduler.  Do not claim deadline attributes as a pure bookkeeping no-op:
    // accepted SCHED_DEADLINE reservations affect the runnable priority in
    // proportion to their runtime/period utilization, while the ABI-visible
    // reservation tuple remains available through sched_getattr().
    let runtime = state.sched_runtime.min(state.sched_period);
    let utilization_percent = if state.sched_period == 0 {
        100
    } else {
        ((runtime as u128).saturating_mul(100) / state.sched_period as u128) as i32
    };
    -1 - ((utilization_percent.clamp(1, 100) - 1) * 19 / 99) as isize
}

fn apply_task_scheduler_state(
    task: &axtask::AxTaskRef,
    process: &UserProcess,
    state: UserSchedState,
) {
    // axtask exposes a CFS-style nice hook.  Linux policy state is still
    // preserved for get* calls; map accepted policy/priority into the nearest
    // available backend priority so sched_set* changes affect scheduling
    // where the configured scheduler supports it.
    let _ = axtask::set_task_priority(task, scheduler_backend_priority(process, state));
}

pub(super) fn apply_process_scheduler_state_to_task(
    process: &UserProcess,
    task: &axtask::AxTaskRef,
) {
    let state = process.get_sched_state();
    apply_task_scheduler_state(task, process, state);
}

fn sched_target_state(process: &UserProcess, pid: i32) -> Result<UserSchedState, LinuxError> {
    if pid < 0 {
        return Err(LinuxError::EINVAL);
    }
    if pid == 0 || pid == current_tid() || pid == process.pid() {
        return Ok(process.get_sched_state());
    }
    if let Some(entry) = process.child_thread_entry_by_pid(pid) {
        return Ok(entry.process.get_sched_state());
    }
    user_thread_entry_by_process_pid(pid)
        .map(|entry| entry.process.get_sched_state())
        .ok_or(LinuxError::ESRCH)
}

fn sched_target_uid(process: &UserProcess, pid: i32) -> Result<u32, LinuxError> {
    if pid < 0 {
        return Err(LinuxError::EINVAL);
    }
    if pid == 0 || pid == current_tid() || pid == process.pid() {
        return Ok(process.uid());
    }
    if pid == 1 {
        return Ok(0);
    }
    if let Some(entry) = process.child_thread_entry_by_pid(pid) {
        return Ok(entry.process.uid());
    }
    user_thread_entry_by_process_pid(pid)
        .map(|entry| entry.process.uid())
        .ok_or(LinuxError::ESRCH)
}

fn can_set_sched_target(process: &UserProcess, pid: i32) -> Result<(), LinuxError> {
    let target_uid = sched_target_uid(process, pid)?;
    if process.uid() == 0 || process.uid() == target_uid {
        Ok(())
    } else {
        Err(LinuxError::EPERM)
    }
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
        let current = axtask::current();
        apply_task_scheduler_state(current.as_task_ref(), process, state);
        process.set_sched_state(state);
        return Ok(());
    }
    let entry = user_thread_entry_by_process_pid(pid).ok_or(LinuxError::ESRCH)?;
    apply_task_scheduler_state(&entry.task, &entry.process, state);
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
            if let Err(err) = can_set_sched_target(process, pid) {
                return neg_errno(err);
            }
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
    let (base_policy, reset_on_fork) = match split_sched_policy(policy) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
    let param = match read_user_value::<UserSchedParam>(process, param) {
        Ok(param) => param,
        Err(err) => return neg_errno(err),
    };
    if !sched_param_accepts_policy(base_policy, param) {
        return neg_errno(LinuxError::EINVAL);
    }
    match sched_target_state(process, pid) {
        Ok(_) => {}
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = can_set_sched_target(process, pid) {
        return neg_errno(err);
    }
    if process.uid() != 0 && sched_policy_needs_privilege(base_policy) {
        return neg_errno(LinuxError::EPERM);
    }
    match set_sched_target_state(
        process,
        pid,
        UserSchedState {
            policy: base_policy,
            param,
            reset_on_fork,
            sched_runtime: 0,
            sched_deadline: 0,
            sched_period: 0,
        },
    ) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
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
        sched_flags: if state.reset_on_fork {
            SCHED_ATTR_FLAG_RESET_ON_FORK
        } else {
            0
        },
        sched_nice: 0,
        sched_priority: state.param.sched_priority as u32,
        sched_runtime: state.sched_runtime,
        sched_deadline: state.sched_deadline,
        sched_period: state.sched_period,
        sched_util_min: 0,
        sched_util_max: 0,
    }
}

fn sched_state_from_attr(attr: UserSchedAttr) -> Result<UserSchedState, LinuxError> {
    let param = UserSchedParam {
        sched_priority: attr.sched_priority as i32,
    };
    let policy = attr.sched_policy as i32;
    let allowed_flags = SCHED_ATTR_FLAG_RESET_ON_FORK;
    if attr.sched_flags & !allowed_flags != 0 {
        return Err(LinuxError::EINVAL);
    }
    let reset_on_fork = (attr.sched_flags & allowed_flags) != 0;
    if policy as u32 == general::SCHED_DEADLINE {
        if param.sched_priority != 0
            || attr.sched_runtime == 0
            || attr.sched_deadline == 0
            || attr.sched_period == 0
            || attr.sched_runtime > attr.sched_deadline
            || attr.sched_deadline > attr.sched_period
        {
            return Err(LinuxError::EINVAL);
        }
        return Ok(UserSchedState {
            policy,
            param,
            reset_on_fork,
            sched_runtime: attr.sched_runtime,
            sched_deadline: attr.sched_deadline,
            sched_period: attr.sched_period,
        });
    }
    if !sched_param_accepts_policy(policy, param) {
        return Err(LinuxError::EINVAL);
    }
    Ok(UserSchedState {
        policy,
        param,
        reset_on_fork,
        sched_runtime: 0,
        sched_deadline: 0,
        sched_period: 0,
    })
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
        Ok(state) => {
            if let Err(err) = can_set_sched_target(process, pid) {
                return neg_errno(err);
            }
            if process.uid() != 0 && sched_policy_needs_privilege(state.policy) {
                return neg_errno(LinuxError::EPERM);
            }
            match set_sched_target_state(process, pid, state) {
                Ok(()) => 0,
                Err(err) => neg_errno(err),
            }
        }
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
        Ok(first) if sched_affinity_accepts_current_cpu(first) => {
            match can_set_sched_target(process, pid) {
                Ok(()) => 0,
                Err(err) => neg_errno(err),
            }
        }
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
    if !prlimit_target_valid(process, pid) {
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
