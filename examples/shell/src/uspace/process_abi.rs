use core::sync::atomic::Ordering;

use axerrno::LinuxError;

use super::linux_abi::{LINUX_PERSONALITY_MASK, LINUX_PERSONALITY_QUERY};
use super::task_registry::user_thread_entry_by_process_pid;
use super::{UserProcess, neg_errno};

impl UserProcess {
    pub(super) fn personality(&self) -> usize {
        self.personality.load(Ordering::Acquire)
    }

    pub(super) fn set_personality(&self, persona: usize) {
        self.personality
            .store(persona & LINUX_PERSONALITY_MASK, Ordering::Release);
    }
}

pub(super) fn sys_setpgid(process: &UserProcess, pid: usize, pgid: usize) -> isize {
    let pid = pid as i32;
    let pgid = pgid as i32;
    if pid < 0 || pgid < 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let current = process.pid();
    let (target, target_process) = if pid == 0 || pid == current {
        (current, None)
    } else {
        let Some(entry) = user_thread_entry_by_process_pid(pid) else {
            return neg_errno(LinuxError::ESRCH);
        };
        if entry.process.ppid() != current {
            return neg_errno(LinuxError::ESRCH);
        }
        (entry.process.pid(), Some(entry.process))
    };

    let group = if pgid == 0 { target } else { pgid };
    if group <= 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if group != target && group != process.pgid() {
        return neg_errno(LinuxError::EPERM);
    }
    if let Some(target_process) = target_process {
        target_process.set_pgid(group);
    } else {
        process.set_pgid(group);
    }

    0
}

pub(super) fn sys_getpgid(process: &UserProcess, pid: usize) -> isize {
    let pid = pid as i32;
    if pid < 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let current = process.pid();
    let target = if pid == 0 { current } else { pid };
    if target == current {
        return process.pgid() as isize;
    }
    let Some(entry) = user_thread_entry_by_process_pid(target) else {
        return neg_errno(LinuxError::ESRCH);
    };
    entry.process.pgid() as isize
}

pub(super) fn sys_setsid(process: &UserProcess) -> isize {
    process.set_pgid(process.pid());
    process.pid() as isize
}

pub(super) fn sys_personality(process: &UserProcess, persona: usize) -> isize {
    apply_personality_request(process, persona) as isize
}

pub(super) fn apply_personality_request(process: &UserProcess, persona: usize) -> usize {
    let old = process.personality();
    if persona != LINUX_PERSONALITY_QUERY {
        process.set_personality(persona);
    }
    old
}
