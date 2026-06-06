use core::sync::atomic::Ordering;

use axerrno::LinuxError;

use super::linux_abi::{LINUX_PERSONALITY_MASK, LINUX_PERSONALITY_QUERY};
use super::task_registry::{live_user_process_entries, user_thread_entry_by_process_pid};
use super::{neg_errno, UserProcess};

const SYNTHETIC_INIT_PID: i32 = 1;

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
    let target_sid = target_process
        .as_ref()
        .map(|entry| entry.sid())
        .unwrap_or_else(|| process.sid());
    if group != target && !process_group_exists_in_session(group, target_sid) {
        return neg_errno(LinuxError::EPERM);
    }
    if let Some(target_process) = target_process {
        target_process.set_pgid(group);
    } else {
        process.set_pgid(group);
    }

    0
}

fn process_group_exists_in_session(pgid: i32, sid: i32) -> bool {
    live_user_process_entries()
        .into_iter()
        .any(|entry| entry.process.pgid() == pgid && entry.process.sid() == sid)
}

fn visible_process_group_and_session(
    process: &UserProcess,
    pid: usize,
) -> Result<(i32, i32), LinuxError> {
    let pid = pid as i32;
    if pid < 0 {
        return Err(LinuxError::ESRCH);
    }

    let current = process.pid();
    let target = if pid == 0 { current } else { pid };
    if target == current {
        return Ok((process.pgid(), process.sid()));
    }
    let Some(entry) = user_thread_entry_by_process_pid(target) else {
        if target == SYNTHETIC_INIT_PID {
            return Ok((SYNTHETIC_INIT_PID, SYNTHETIC_INIT_PID));
        }
        return Err(LinuxError::ESRCH);
    };
    Ok((entry.process.pgid(), entry.process.sid()))
}

pub(super) fn sys_getpgid(process: &UserProcess, pid: usize) -> isize {
    match visible_process_group_and_session(process, pid) {
        Ok((pgid, _)) => pgid as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_getsid(process: &UserProcess, pid: usize) -> isize {
    match visible_process_group_and_session(process, pid) {
        Ok((_, sid)) => sid as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_setsid(process: &UserProcess) -> isize {
    let sid = process.pid();
    if process_group_exists_in_session(sid, process.sid()) {
        return neg_errno(LinuxError::EPERM);
    }
    process.set_pgid(sid);
    process.set_sid(sid);
    sid as isize
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
