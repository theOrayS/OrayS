use core::mem::size_of;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use core::time::Duration;

use axerrno::LinuxError;
use axsync::Mutex;
use axtask::WaitQueue;
use lazyinit::LazyInit;
use std::collections::BTreeMap;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

use super::linux_abi::{
    neg_errno, SYSV_IPC_CREAT, SYSV_IPC_EXCL, SYSV_IPC_INFO, SYSV_IPC_PRIVATE, SYSV_IPC_RMID,
    SYSV_IPC_SET, SYSV_IPC_STAT,
};
use super::signal_abi::current_unblocked_signal_pending;
use super::task_context::current_task_ext;
use super::time_abi::timespec_to_duration;
use super::user_memory::{
    read_user_value, validate_user_write, write_user_bytes, write_user_value,
};
use super::UserProcess;

const SYSV_SEM_MAX_SETS: usize = 128;
const SYSV_SEM_MAX_SEMS_PER_SET: usize = 32_000;
const SYSV_SEM_MAX_TOTAL_SEMS: usize = SYSV_SEM_MAX_SETS * 10;
const SYSV_SEM_MAX_OPS: usize = 500;
const SYSV_SEM_MAX_VALUE: i32 = 32_767;
const SYSV_SEM_UNDO_MAX: i32 = SYSV_SEM_MAX_VALUE;
const SYSV_IPC_NOWAIT: i16 = 0o4000;
const SYSV_SEM_UNDO: i16 = 0x1000;
const SYSV_SEM_KNOWN_FLAGS: i16 = SYSV_IPC_NOWAIT | SYSV_SEM_UNDO;

const SYSV_SEM_GETPID: i32 = 11;
const SYSV_SEM_GETVAL: i32 = 12;
const SYSV_SEM_GETALL: i32 = 13;
const SYSV_SEM_GETNCNT: i32 = 14;
const SYSV_SEM_GETZCNT: i32 = 15;
const SYSV_SEM_SETVAL: i32 = 16;
const SYSV_SEM_SETALL: i32 = 17;
const SYSV_SEM_STAT: i32 = 18;
const SYSV_SEM_INFO: i32 = 19;
const SYSV_SEM_STAT_ANY: i32 = 20;

const PROC_SYS_KERNEL_SEM_PATH: &str = "/proc/sys/kernel/sem";
const PROC_SYS_KERNEL_SEM_CONTENT: &[u8] = b"32000 1280 500 128\n";

#[derive(Clone, Copy, Default)]
struct SysvSemaphore {
    value: u16,
    last_pid: i32,
    ncnt: usize,
    zcnt: usize,
}

struct SysvSemSetState {
    key: i32,
    mode: u32,
    uid: u32,
    gid: u32,
    cuid: u32,
    cgid: u32,
    sems: Vec<SysvSemaphore>,
    otime: isize,
    ctime: isize,
}

struct SysvSemSet {
    state: Mutex<SysvSemSetState>,
    queue: WaitQueue,
    generation: AtomicU32,
    removed: AtomicBool,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserIpcPerm64 {
    key: i32,
    uid: u32,
    gid: u32,
    cuid: u32,
    cgid: u32,
    mode: u32,
    seq: u16,
    pad2: u16,
    unused1: usize,
    unused2: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserSemidDs64 {
    sem_perm: UserIpcPerm64,
    sem_otime: isize,
    sem_ctime: isize,
    sem_nsems: usize,
    unused3: usize,
    unused4: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserSeminfo {
    semmap: i32,
    semmni: i32,
    semmns: i32,
    semmnu: i32,
    semmsl: i32,
    semopm: i32,
    semume: i32,
    semusz: i32,
    semvmx: i32,
    semaem: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserSembuf {
    sem_num: u16,
    sem_op: i16,
    sem_flg: i16,
}

const _: [(); 48] = [(); size_of::<UserIpcPerm64>()];
// Linux asm-generic 64-bit semid_ds/time64 helper layout used by RISC-V and
// LoongArch libc ABIs: sem_nsems is immediately after sem_ctime.
const _: [(); 88] = [(); size_of::<UserSemidDs64>()];
const _: [(); 40] = [(); size_of::<UserSeminfo>()];
const _: [(); 6] = [(); size_of::<UserSembuf>()];

static NEXT_SYSV_SEM_ID: AtomicI32 = AtomicI32::new(1);

fn table() -> &'static Mutex<BTreeMap<i32, Arc<SysvSemSet>>> {
    static SYSV_SEM: LazyInit<Mutex<BTreeMap<i32, Arc<SysvSemSet>>>> = LazyInit::new();
    let _ = SYSV_SEM.call_once(|| Mutex::new(BTreeMap::new()));
    &SYSV_SEM
}

fn current_time_secs() -> isize {
    axhal::time::wall_time().as_secs().min(isize::MAX as u64) as isize
}

fn requested_access(flags: i32) -> (bool, bool) {
    let mode = flags as u32 & 0o777;
    (mode & 0o444 != 0, mode & 0o222 != 0)
}

fn mode_allows(state: &SysvSemSetState, process: &UserProcess, read: bool, write: bool) -> bool {
    if process.uid() == 0 {
        return true;
    }
    let shift = if process.uid() == state.uid {
        6
    } else if process.gid() == state.gid {
        3
    } else {
        0
    };
    let perms = (state.mode >> shift) & 0o7;
    (!read || perms & 0o4 != 0) && (!write || perms & 0o2 != 0)
}

fn control_allowed(state: &SysvSemSetState, process: &UserProcess) -> bool {
    process.uid() == 0 || process.uid() == state.uid || process.uid() == state.cuid
}

fn active_snapshot_locked(table: &BTreeMap<i32, Arc<SysvSemSet>>) -> (i32, i32, i32) {
    let max_id = table.keys().copied().max().unwrap_or(0);
    let used_sets = table.len().min(i32::MAX as usize) as i32;
    let sem_count = table
        .values()
        .map(|set| set.state.lock().sems.len())
        .sum::<usize>()
        .min(i32::MAX as usize) as i32;
    (max_id, used_sets, sem_count)
}

fn seminfo(used_sets: i32, sem_count: i32, info_mode: bool) -> UserSeminfo {
    UserSeminfo {
        semmap: SYSV_SEM_MAX_SETS as i32,
        semmni: SYSV_SEM_MAX_SETS as i32,
        semmns: SYSV_SEM_MAX_TOTAL_SEMS as i32,
        semmnu: SYSV_SEM_MAX_SETS as i32,
        semmsl: SYSV_SEM_MAX_SEMS_PER_SET as i32,
        semopm: SYSV_SEM_MAX_OPS as i32,
        semume: SYSV_SEM_MAX_OPS as i32,
        semusz: if info_mode {
            used_sets
        } else {
            size_of::<SysvSemSetState>() as i32
        },
        semvmx: SYSV_SEM_MAX_VALUE,
        semaem: if info_mode {
            sem_count
        } else {
            SYSV_SEM_UNDO_MAX
        },
    }
}

fn stat_from_state(state: &SysvSemSetState) -> UserSemidDs64 {
    UserSemidDs64 {
        sem_perm: UserIpcPerm64 {
            key: state.key,
            uid: state.uid,
            gid: state.gid,
            cuid: state.cuid,
            cgid: state.cgid,
            mode: state.mode,
            ..UserIpcPerm64::default()
        },
        sem_otime: state.otime,
        sem_ctime: state.ctime,
        sem_nsems: state.sems.len(),
        ..UserSemidDs64::default()
    }
}

fn lookup_set(semid: i32) -> Result<Arc<SysvSemSet>, LinuxError> {
    table()
        .lock()
        .get(&semid)
        .cloned()
        .ok_or(LinuxError::EINVAL)
}

fn create_set(
    process: &UserProcess,
    key: i32,
    nsems: usize,
    flags: i32,
) -> Result<i32, LinuxError> {
    if nsems == 0 || nsems > SYSV_SEM_MAX_SEMS_PER_SET {
        return Err(LinuxError::EINVAL);
    }
    let mut table = table().lock();
    if table.len() >= SYSV_SEM_MAX_SETS {
        return Err(LinuxError::ENOSPC);
    }
    let current_total = table
        .values()
        .map(|set| set.state.lock().sems.len())
        .sum::<usize>();
    if current_total.saturating_add(nsems) > SYSV_SEM_MAX_TOTAL_SEMS {
        return Err(LinuxError::ENOSPC);
    }
    let semid = NEXT_SYSV_SEM_ID.fetch_add(1, Ordering::Relaxed);
    table.insert(
        semid,
        Arc::new(SysvSemSet {
            state: Mutex::new(SysvSemSetState {
                key,
                mode: (flags as u32) & 0o777,
                uid: process.uid(),
                gid: process.gid(),
                cuid: process.uid(),
                cgid: process.gid(),
                sems: vec![SysvSemaphore::default(); nsems],
                otime: 0,
                ctime: current_time_secs(),
            }),
            queue: WaitQueue::new(),
            generation: AtomicU32::new(0),
            removed: AtomicBool::new(false),
        }),
    );
    Ok(semid)
}

fn get_or_create(
    process: &UserProcess,
    key: usize,
    nsems: usize,
    semflg: usize,
) -> Result<i32, LinuxError> {
    let key = key as i32;
    let flags = semflg as i32;
    if key != SYSV_IPC_PRIVATE {
        let table = table().lock();
        if let Some((semid, set)) = table.iter().find(|(_, set)| set.state.lock().key == key) {
            if flags & SYSV_IPC_CREAT != 0 && flags & SYSV_IPC_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            let state = set.state.lock();
            if nsems > state.sems.len() {
                return Err(LinuxError::EINVAL);
            }
            let (read, write) = requested_access(flags);
            if (read || write) && !mode_allows(&state, process, read, write) {
                return Err(LinuxError::EACCES);
            }
            return Ok(*semid);
        }
        if flags & SYSV_IPC_CREAT == 0 {
            return Err(LinuxError::ENOENT);
        }
    }
    create_set(process, key, nsems, flags)
}

fn validate_semnum(state: &SysvSemSetState, semnum: usize) -> Result<usize, LinuxError> {
    if semnum >= state.sems.len() {
        Err(LinuxError::EFBIG)
    } else {
        Ok(semnum)
    }
}

fn write_semid(process: &UserProcess, ptr: usize, state: &SysvSemSetState) -> isize {
    if ptr == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let stat = stat_from_state(state);
    write_user_value(process, ptr, &stat)
}

fn read_semop_timeout(
    process: &UserProcess,
    timeout: usize,
) -> Result<Option<Duration>, LinuxError> {
    if timeout == 0 {
        return Ok(None);
    }
    let ts = read_user_value::<linux_raw_sys::general::timespec>(process, timeout)?;
    Ok(Some(timespec_to_duration(ts)?))
}

fn semop_interrupted(process: &UserProcess) -> bool {
    current_unblocked_signal_pending()
        || process.pending_exit_group().is_some()
        || process.eval_watchdog_expired()
}

fn can_apply_ops(state: &SysvSemSetState, ops: &[UserSembuf]) -> Result<Option<usize>, LinuxError> {
    let mut values: Vec<i32> = state.sems.iter().map(|sem| sem.value as i32).collect();
    for (idx, op) in ops.iter().enumerate() {
        let semnum = op.sem_num as usize;
        if semnum >= state.sems.len() {
            return Err(LinuxError::EFBIG);
        }
        if op.sem_flg & !SYSV_SEM_KNOWN_FLAGS != 0 {
            return Err(LinuxError::EINVAL);
        }
        let value = values[semnum];
        if op.sem_op > 0 {
            let new_value = value + op.sem_op as i32;
            if new_value > SYSV_SEM_MAX_VALUE {
                return Err(LinuxError::ERANGE);
            }
            values[semnum] = new_value;
        } else if op.sem_op < 0 {
            let needed = -(op.sem_op as i32);
            if value < needed {
                return Ok(Some(idx));
            }
            values[semnum] = value - needed;
        } else if value != 0 {
            return Ok(Some(idx));
        }
    }
    Ok(None)
}

fn apply_ops(state: &mut SysvSemSetState, ops: &[UserSembuf], pid: i32) {
    for op in ops {
        let sem = &mut state.sems[op.sem_num as usize];
        if op.sem_op > 0 {
            sem.value = (sem.value as i32 + op.sem_op as i32) as u16;
            sem.last_pid = pid;
        } else if op.sem_op < 0 {
            sem.value = (sem.value as i32 + op.sem_op as i32) as u16;
            sem.last_pid = pid;
        }
    }
    state.otime = current_time_secs();
}

fn try_apply_ops(
    set: &Arc<SysvSemSet>,
    ops: &[UserSembuf],
    process: &UserProcess,
) -> Result<(), LinuxError> {
    if set.removed.load(Ordering::Acquire) {
        return Err(LinuxError::EIDRM);
    }
    let mut state = set.state.lock();
    let mut need_read = false;
    let mut need_write = false;
    for op in ops {
        need_read |= op.sem_op == 0;
        need_write |= op.sem_op != 0;
    }
    if !mode_allows(&state, process, need_read, need_write) {
        return Err(LinuxError::EACCES);
    }
    if can_apply_ops(&state, ops)?.is_some() {
        return Err(LinuxError::EAGAIN);
    }
    apply_ops(&mut state, ops, process.pid());
    set.generation.fetch_add(1, Ordering::Release);
    set.queue.notify_all(true);
    Ok(())
}

fn blocking_semop(
    set: Arc<SysvSemSet>,
    ops: Vec<UserSembuf>,
    process: &UserProcess,
    timeout: Option<Duration>,
) -> Result<(), LinuxError> {
    let Some(blocked_idx) = ({
        let state = set.state.lock();
        can_apply_ops(&state, &ops)?
    }) else {
        return try_apply_ops(&set, &ops, process);
    };
    let blocked_sem = ops[blocked_idx].sem_num as usize;
    let wait_for_zero = ops[blocked_idx].sem_op == 0;
    {
        let mut state = set.state.lock();
        if blocked_sem < state.sems.len() {
            if wait_for_zero {
                state.sems[blocked_sem].zcnt = state.sems[blocked_sem].zcnt.saturating_add(1);
            } else {
                state.sems[blocked_sem].ncnt = state.sems[blocked_sem].ncnt.saturating_add(1);
            }
        }
    }

    if let Some(ext) = current_task_ext() {
        ext.process.set_syscall_wait_blocked(true);
    }

    let wait_started = axhal::time::wall_time();
    let result = loop {
        if set.removed.load(Ordering::Acquire) {
            break Err(LinuxError::EIDRM);
        }
        if semop_interrupted(process) {
            break Err(LinuxError::EINTR);
        }
        match try_apply_ops(&set, &ops, process) {
            Ok(()) => {
                break Ok(());
            }
            Err(LinuxError::EAGAIN) => {}
            Err(err) => {
                break Err(err);
            }
        }
        let Some(timeout) = timeout else {
            let _ = set.queue.wait_timeout_until(Duration::from_millis(20), || {
                set.removed.load(Ordering::Acquire) || semop_interrupted(process) || {
                    let state = set.state.lock();
                    can_apply_ops(&state, &ops).map_or(true, |blocked| blocked.is_none())
                }
            });
            continue;
        };
        let elapsed = axhal::time::wall_time().saturating_sub(wait_started);
        if elapsed >= timeout {
            break Err(LinuxError::EAGAIN);
        }
        let remaining = timeout
            .saturating_sub(elapsed)
            .min(Duration::from_millis(20));
        let timed_out = set.queue.wait_timeout_until(remaining, || {
            set.removed.load(Ordering::Acquire) || semop_interrupted(process) || {
                let state = set.state.lock();
                can_apply_ops(&state, &ops).map_or(true, |blocked| blocked.is_none())
            }
        });
        if timed_out && remaining >= timeout.saturating_sub(elapsed) {
            break Err(LinuxError::EAGAIN);
        }
    };

    if let Some(ext) = current_task_ext() {
        ext.process.set_syscall_wait_blocked(false);
    }
    {
        let mut state = set.state.lock();
        if blocked_sem < state.sems.len() {
            if wait_for_zero {
                state.sems[blocked_sem].zcnt = state.sems[blocked_sem].zcnt.saturating_sub(1);
            } else {
                state.sems[blocked_sem].ncnt = state.sems[blocked_sem].ncnt.saturating_sub(1);
            }
        }
    }
    result
}

fn read_ops(
    process: &UserProcess,
    sops: usize,
    nsops: usize,
) -> Result<Vec<UserSembuf>, LinuxError> {
    if nsops == 0 {
        return Err(LinuxError::EINVAL);
    }
    if nsops > SYSV_SEM_MAX_OPS {
        return Err(LinuxError::E2BIG);
    }
    if sops == 0 {
        return Err(LinuxError::EFAULT);
    }
    let mut ops = Vec::with_capacity(nsops);
    for idx in 0..nsops {
        let ptr = sops
            .checked_add(idx.saturating_mul(size_of::<UserSembuf>()))
            .ok_or(LinuxError::EFAULT)?;
        ops.push(read_user_value::<UserSembuf>(process, ptr)?);
    }
    Ok(ops)
}

pub(super) fn sys_semget(process: &UserProcess, key: usize, nsems: usize, semflg: usize) -> isize {
    match get_or_create(process, key, nsems, semflg) {
        Ok(semid) => semid as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_semctl(
    process: &UserProcess,
    semid: usize,
    semnum: usize,
    cmd: usize,
    arg: usize,
) -> isize {
    let semid = semid as i32;
    let cmd = cmd as i32;
    match cmd {
        SYSV_IPC_INFO | SYSV_SEM_INFO => {
            if arg == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let table = table().lock();
            let (max_id, used_sets, sem_count) = active_snapshot_locked(&table);
            let info = seminfo(used_sets, sem_count, cmd == SYSV_SEM_INFO);
            let ret = write_user_value(process, arg, &info);
            if ret != 0 {
                return ret;
            }
            max_id as isize
        }
        SYSV_IPC_RMID => {
            let set = {
                let mut table = table().lock();
                let Some(set) = table.get(&semid).cloned() else {
                    return neg_errno(LinuxError::EINVAL);
                };
                if !control_allowed(&set.state.lock(), process) {
                    return neg_errno(LinuxError::EPERM);
                }
                table.remove(&semid);
                set
            };
            set.removed.store(true, Ordering::Release);
            set.generation.fetch_add(1, Ordering::Release);
            set.queue.notify_all(true);
            0
        }
        SYSV_IPC_STAT => {
            let set = match lookup_set(semid) {
                Ok(set) => set,
                Err(err) => return neg_errno(err),
            };
            let state = set.state.lock();
            if !mode_allows(&state, process, true, false) {
                return neg_errno(LinuxError::EACCES);
            }
            write_semid(process, arg, &state)
        }
        SYSV_SEM_STAT | SYSV_SEM_STAT_ANY => {
            let set = match lookup_set(semid) {
                Ok(set) => set,
                Err(err) => return neg_errno(err),
            };
            let state = set.state.lock();
            if cmd == SYSV_SEM_STAT && !mode_allows(&state, process, true, false) {
                return neg_errno(LinuxError::EACCES);
            }
            let ret = write_semid(process, arg, &state);
            if ret != 0 {
                return ret;
            }
            semid as isize
        }
        SYSV_IPC_SET => {
            let requested = match read_user_value::<UserSemidDs64>(process, arg) {
                Ok(value) => value,
                Err(err) => return neg_errno(err),
            };
            let set = match lookup_set(semid) {
                Ok(set) => set,
                Err(err) => return neg_errno(err),
            };
            let mut state = set.state.lock();
            if !control_allowed(&state, process) {
                return neg_errno(LinuxError::EPERM);
            }
            state.uid = requested.sem_perm.uid;
            state.gid = requested.sem_perm.gid;
            state.mode = (state.mode & !0o777) | (requested.sem_perm.mode & 0o777);
            state.ctime = current_time_secs();
            0
        }
        SYSV_SEM_GETALL => {
            let set = match lookup_set(semid) {
                Ok(set) => set,
                Err(err) => return neg_errno(err),
            };
            let state = set.state.lock();
            if !mode_allows(&state, process, true, false) {
                return neg_errno(LinuxError::EACCES);
            }
            let bytes = state.sems.len().saturating_mul(size_of::<u16>());
            if let Err(err) = validate_user_write(process, arg, bytes) {
                return neg_errno(err);
            }
            let values: Vec<u16> = state.sems.iter().map(|sem| sem.value).collect();
            let src = unsafe { core::slice::from_raw_parts(values.as_ptr() as *const u8, bytes) };
            match write_user_bytes(process, arg, src) {
                Ok(()) => 0,
                Err(err) => neg_errno(err),
            }
        }
        SYSV_SEM_SETALL => {
            let set = match lookup_set(semid) {
                Ok(set) => set,
                Err(err) => return neg_errno(err),
            };
            let mut state = set.state.lock();
            if !mode_allows(&state, process, false, true) {
                return neg_errno(LinuxError::EACCES);
            }
            let mut values = Vec::with_capacity(state.sems.len());
            for idx in 0..state.sems.len() {
                let ptr = match arg.checked_add(idx.saturating_mul(size_of::<u16>())) {
                    Some(ptr) => ptr,
                    None => return neg_errno(LinuxError::EFAULT),
                };
                let value = match read_user_value::<u16>(process, ptr) {
                    Ok(value) => value,
                    Err(err) => return neg_errno(err),
                };
                if value as i32 > SYSV_SEM_MAX_VALUE {
                    return neg_errno(LinuxError::ERANGE);
                }
                values.push(value);
            }
            for (sem, value) in state.sems.iter_mut().zip(values) {
                sem.value = value;
                sem.last_pid = process.pid();
            }
            state.ctime = current_time_secs();
            set.generation.fetch_add(1, Ordering::Release);
            set.queue.notify_all(true);
            0
        }
        SYSV_SEM_GETVAL | SYSV_SEM_GETPID | SYSV_SEM_GETNCNT | SYSV_SEM_GETZCNT => {
            let set = match lookup_set(semid) {
                Ok(set) => set,
                Err(err) => return neg_errno(err),
            };
            let state = set.state.lock();
            if !mode_allows(&state, process, true, false) {
                return neg_errno(LinuxError::EACCES);
            }
            let semnum = match validate_semnum(&state, semnum) {
                Ok(semnum) => semnum,
                Err(err) => return neg_errno(err),
            };
            let sem = state.sems[semnum];
            match cmd {
                SYSV_SEM_GETVAL => sem.value as isize,
                SYSV_SEM_GETPID => sem.last_pid as isize,
                SYSV_SEM_GETNCNT => sem.ncnt.min(isize::MAX as usize) as isize,
                SYSV_SEM_GETZCNT => sem.zcnt.min(isize::MAX as usize) as isize,
                _ => unreachable!(),
            }
        }
        SYSV_SEM_SETVAL => {
            let value = arg as i32;
            if value < 0 || value > SYSV_SEM_MAX_VALUE {
                return neg_errno(LinuxError::ERANGE);
            }
            let set = match lookup_set(semid) {
                Ok(set) => set,
                Err(err) => return neg_errno(err),
            };
            let mut state = set.state.lock();
            if !mode_allows(&state, process, false, true) {
                return neg_errno(LinuxError::EACCES);
            }
            let semnum = match validate_semnum(&state, semnum) {
                Ok(semnum) => semnum,
                Err(err) => return neg_errno(err),
            };
            state.sems[semnum].value = value as u16;
            state.sems[semnum].last_pid = process.pid();
            state.ctime = current_time_secs();
            set.generation.fetch_add(1, Ordering::Release);
            set.queue.notify_all(true);
            0
        }
        _ => neg_errno(LinuxError::EINVAL),
    }
}

pub(super) fn sys_semop(process: &UserProcess, semid: usize, sops: usize, nsops: usize) -> isize {
    sys_semtimedop(process, semid, sops, nsops, 0)
}

pub(super) fn sys_semtimedop(
    process: &UserProcess,
    semid: usize,
    sops: usize,
    nsops: usize,
    timeout: usize,
) -> isize {
    let ops = match read_ops(process, sops, nsops) {
        Ok(ops) => ops,
        Err(err) => return neg_errno(err),
    };
    let set = match lookup_set(semid as i32) {
        Ok(set) => set,
        Err(err) => return neg_errno(err),
    };
    let timeout = match read_semop_timeout(process, timeout) {
        Ok(timeout) => timeout,
        Err(err) => return neg_errno(err),
    };
    let nowait = ops.iter().any(|op| op.sem_flg & SYSV_IPC_NOWAIT != 0);
    match try_apply_ops(&set, &ops, process) {
        Ok(()) => 0,
        Err(LinuxError::EAGAIN) if nowait => neg_errno(LinuxError::EAGAIN),
        Err(LinuxError::EAGAIN) => match blocking_semop(set, ops, process, timeout) {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        },
        Err(err) => neg_errno(err),
    }
}

pub(super) fn proc_sysvipc_sem_content() -> Vec<u8> {
    let table = table().lock();
    let mut content = String::from(
        "       key      semid perms      nsems   uid   gid  cuid  cgid      otime      ctime\n",
    );
    for (semid, set) in table.iter() {
        let state = set.state.lock();
        content.push_str(&format!(
            "{key:10} {semid:10} {mode:5o} {nsems:10} {uid:5} {gid:5} {cuid:5} {cgid:5} {otime:10} {ctime:10}\n",
            key = state.key,
            semid = semid,
            mode = state.mode,
            nsems = state.sems.len(),
            uid = state.uid,
            gid = state.gid,
            cuid = state.cuid,
            cgid = state.cgid,
            otime = state.otime,
            ctime = state.ctime,
        ));
    }
    content.into_bytes()
}

pub(super) fn proc_sys_kernel_sem_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    let normalized = super::runtime_paths::normalize_path("/", path)?;
    (normalized == PROC_SYS_KERNEL_SEM_PATH)
        .then_some((PROC_SYS_KERNEL_SEM_PATH, PROC_SYS_KERNEL_SEM_CONTENT))
}
