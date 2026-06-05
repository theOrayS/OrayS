use core::mem::size_of;
use core::sync::atomic::{AtomicI32, Ordering};

use axerrno::LinuxError;
use axsync::Mutex;
use lazyinit::LazyInit;
use std::collections::BTreeMap;
use std::vec::Vec;

use super::linux_abi::{
    neg_errno, SYSV_IPC_CREAT, SYSV_IPC_EXCL, SYSV_IPC_PRIVATE, SYSV_IPC_RMID, SYSV_IPC_SET,
    SYSV_IPC_STAT,
};
use super::user_memory::{
    read_user_bytes, read_user_value, validate_user_write, write_user_bytes, write_user_value,
};
use super::UserProcess;

const SYSV_MSG_MAX_QUEUES: usize = 128;
const SYSV_MSG_MAX_BYTES: usize = 16 * 1024;
const SYSV_MSG_MAX_SIZE: usize = 8 * 1024;
const SYSV_IPC_NOWAIT: i32 = 0o4000;
const SYSV_MSG_NOERROR: i32 = 0o10000;
const SYSV_MSG_EXCEPT: i32 = 0o20000;

#[derive(Clone)]
struct SysvMessage {
    mtype: isize,
    payload: Vec<u8>,
}

struct SysvMsgQueue {
    key: i32,
    mode: u32,
    uid: u32,
    gid: u32,
    cuid: u32,
    cgid: u32,
    qbytes: usize,
    messages: Vec<SysvMessage>,
    cbytes: usize,
    stime: isize,
    rtime: isize,
    ctime: isize,
    lspid: i32,
    lrpid: i32,
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
struct UserMsqidDs64 {
    msg_perm: UserIpcPerm64,
    msg_stime: isize,
    msg_rtime: isize,
    msg_ctime: isize,
    msg_cbytes: usize,
    msg_qnum: usize,
    msg_qbytes: usize,
    msg_lspid: i32,
    msg_lrpid: i32,
    unused4: usize,
    unused5: usize,
}

// Linux 64-bit IPC user ABI: asm-generic/{ipcbuf,msgbuf}.h. RISC-V and
// LoongArch use this msqid64_ds layout for msgctl(IPC_STAT/IPC_SET).
const _: [(); 48] = [(); size_of::<UserIpcPerm64>()];
const _: [(); 120] = [(); size_of::<UserMsqidDs64>()];

static NEXT_SYSV_MSG_ID: AtomicI32 = AtomicI32::new(1);

fn table() -> &'static Mutex<BTreeMap<i32, SysvMsgQueue>> {
    static SYSV_MSG: LazyInit<Mutex<BTreeMap<i32, SysvMsgQueue>>> = LazyInit::new();
    let _ = SYSV_MSG.call_once(|| Mutex::new(BTreeMap::new()));
    &SYSV_MSG
}

fn current_time_secs() -> isize {
    axhal::time::wall_time().as_secs().min(isize::MAX as u64) as isize
}

fn requested_access(flags: i32) -> (bool, bool) {
    let mode = flags as u32 & 0o777;
    (mode & 0o444 != 0, mode & 0o222 != 0)
}

fn queue_mode_allows(queue: &SysvMsgQueue, process: &UserProcess, read: bool, write: bool) -> bool {
    if process.uid() == 0 {
        return true;
    }
    let shift = if process.uid() == queue.uid {
        6
    } else if process.gid() == queue.gid {
        3
    } else {
        0
    };
    let perms = (queue.mode >> shift) & 0o7;
    (!read || perms & 0o4 != 0) && (!write || perms & 0o2 != 0)
}

fn queue_control_allowed(queue: &SysvMsgQueue, process: &UserProcess) -> bool {
    process.uid() == 0 || process.uid() == queue.uid || process.uid() == queue.cuid
}

fn create_queue(process: &UserProcess, key: i32, flags: i32) -> Result<i32, LinuxError> {
    let mut table = table().lock();
    if table.len() >= SYSV_MSG_MAX_QUEUES {
        return Err(LinuxError::ENOSPC);
    }
    let msqid = NEXT_SYSV_MSG_ID.fetch_add(1, Ordering::Relaxed);
    table.insert(
        msqid,
        SysvMsgQueue {
            key,
            mode: (flags as u32) & 0o777,
            uid: process.uid(),
            gid: process.gid(),
            cuid: process.uid(),
            cgid: process.gid(),
            qbytes: SYSV_MSG_MAX_BYTES,
            messages: Vec::new(),
            cbytes: 0,
            stime: 0,
            rtime: 0,
            ctime: current_time_secs(),
            lspid: 0,
            lrpid: 0,
        },
    );
    Ok(msqid)
}

fn get_or_create(process: &UserProcess, key: usize, msgflg: usize) -> Result<i32, LinuxError> {
    let key = key as i32;
    let flags = msgflg as i32;
    if key != SYSV_IPC_PRIVATE {
        let table = table().lock();
        if let Some((msqid, queue)) = table.iter().find(|(_, queue)| queue.key == key) {
            if flags & SYSV_IPC_CREAT != 0 && flags & SYSV_IPC_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            let (read, write) = requested_access(flags);
            if (read || write) && !queue_mode_allows(queue, process, read, write) {
                return Err(LinuxError::EACCES);
            }
            return Ok(*msqid);
        }
        if flags & SYSV_IPC_CREAT == 0 {
            return Err(LinuxError::ENOENT);
        }
    }
    create_queue(process, key, flags)
}

fn stat_from_queue(queue: &SysvMsgQueue) -> UserMsqidDs64 {
    UserMsqidDs64 {
        msg_perm: UserIpcPerm64 {
            key: queue.key,
            uid: queue.uid,
            gid: queue.gid,
            cuid: queue.cuid,
            cgid: queue.cgid,
            mode: queue.mode,
            ..UserIpcPerm64::default()
        },
        msg_stime: queue.stime,
        msg_rtime: queue.rtime,
        msg_ctime: queue.ctime,
        msg_cbytes: queue.cbytes,
        msg_qnum: queue.messages.len(),
        msg_qbytes: queue.qbytes,
        msg_lspid: queue.lspid,
        msg_lrpid: queue.lrpid,
        ..UserMsqidDs64::default()
    }
}

fn select_message_index(messages: &[SysvMessage], msgtyp: isize, flags: i32) -> Option<usize> {
    if msgtyp == 0 {
        return (!messages.is_empty()).then_some(0);
    }
    if msgtyp > 0 {
        if flags & SYSV_MSG_EXCEPT != 0 {
            return messages.iter().position(|message| message.mtype != msgtyp);
        }
        return messages.iter().position(|message| message.mtype == msgtyp);
    }

    let max_type = msgtyp.checked_abs()?;
    let mut best: Option<(usize, isize)> = None;
    for (index, message) in messages.iter().enumerate() {
        if message.mtype > 0 && message.mtype <= max_type {
            match best {
                Some((_, best_type)) if message.mtype >= best_type => {}
                _ => best = Some((index, message.mtype)),
            }
        }
    }
    best.map(|(index, _)| index)
}

fn copy_out_message(
    process: &UserProcess,
    msgp: usize,
    mtype: isize,
    payload: &[u8],
) -> Result<(), LinuxError> {
    validate_user_write(process, msgp, size_of::<isize>() + payload.len())?;
    let status = write_user_value(process, msgp, &mtype);
    if status < 0 {
        return Err(LinuxError::EFAULT);
    }
    let payload_ptr = msgp
        .checked_add(size_of::<isize>())
        .ok_or(LinuxError::EFAULT)?;
    write_user_bytes(process, payload_ptr, payload)
}

fn set_queue_metadata(
    process: &UserProcess,
    msqid: i32,
    requested: UserMsqidDs64,
) -> Result<(), LinuxError> {
    let mut table = table().lock();
    let queue = table.get_mut(&msqid).ok_or(LinuxError::EINVAL)?;
    if !queue_control_allowed(queue, process) {
        return Err(LinuxError::EPERM);
    }
    queue.uid = requested.msg_perm.uid;
    queue.gid = requested.msg_perm.gid;
    queue.mode = (queue.mode & !0o777) | (requested.msg_perm.mode & 0o777);
    queue.qbytes = requested
        .msg_qbytes
        .max(queue.cbytes)
        .min(SYSV_MSG_MAX_BYTES);
    queue.ctime = current_time_secs();
    Ok(())
}

pub(super) fn sys_msgget(process: &UserProcess, key: usize, msgflg: usize) -> isize {
    match get_or_create(process, key, msgflg) {
        Ok(msqid) => msqid as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_msgsnd(
    process: &UserProcess,
    msqid: usize,
    msgp: usize,
    msgsz: usize,
    msgflg: usize,
) -> isize {
    let msqid = msqid as i32;
    let flags = msgflg as i32;
    if flags & !SYSV_IPC_NOWAIT != 0 || msgsz > SYSV_MSG_MAX_SIZE {
        return neg_errno(LinuxError::EINVAL);
    }

    let mtype = match read_user_value::<isize>(process, msgp) {
        Ok(mtype) => mtype,
        Err(err) => return neg_errno(err),
    };
    if mtype <= 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let payload_ptr = match msgp.checked_add(size_of::<isize>()) {
        Some(ptr) => ptr,
        None => return neg_errno(LinuxError::EFAULT),
    };
    let payload = match read_user_bytes(process, payload_ptr, msgsz) {
        Ok(payload) => payload,
        Err(err) => return neg_errno(err),
    };

    let mut table = table().lock();
    let Some(queue) = table.get_mut(&msqid) else {
        return neg_errno(LinuxError::EINVAL);
    };
    if !queue_mode_allows(queue, process, false, true) {
        return neg_errno(LinuxError::EACCES);
    }
    let Some(next_bytes) = queue.cbytes.checked_add(payload.len()) else {
        return neg_errno(LinuxError::EAGAIN);
    };
    if next_bytes > queue.qbytes {
        return neg_errno(LinuxError::EAGAIN);
    }
    queue.messages.push(SysvMessage { mtype, payload });
    queue.cbytes = next_bytes;
    queue.lspid = process.pid();
    queue.stime = current_time_secs();
    0
}

pub(super) fn sys_msgrcv(
    process: &UserProcess,
    msqid: usize,
    msgp: usize,
    msgsz: usize,
    msgtyp: isize,
    msgflg: usize,
) -> isize {
    let msqid = msqid as i32;
    let flags = msgflg as i32;
    const KNOWN_FLAGS: i32 = SYSV_IPC_NOWAIT | SYSV_MSG_NOERROR | SYSV_MSG_EXCEPT;
    if flags & !KNOWN_FLAGS != 0 || msgsz > SYSV_MSG_MAX_SIZE {
        return neg_errno(LinuxError::EINVAL);
    }

    let mut table = table().lock();
    let Some(queue) = table.get_mut(&msqid) else {
        return neg_errno(LinuxError::EINVAL);
    };
    if !queue_mode_allows(queue, process, true, false) {
        return neg_errno(LinuxError::EACCES);
    }
    let Some(index) = select_message_index(&queue.messages, msgtyp, flags) else {
        return neg_errno(LinuxError::ENOMSG);
    };
    let message_len = queue.messages[index].payload.len();
    if message_len > msgsz && flags & SYSV_MSG_NOERROR == 0 {
        return neg_errno(LinuxError::E2BIG);
    }
    let copy_len = message_len.min(msgsz);
    let mtype = queue.messages[index].mtype;
    let payload = queue.messages[index].payload[..copy_len].to_vec();
    if let Err(err) = copy_out_message(process, msgp, mtype, &payload) {
        return neg_errno(err);
    }
    let message = queue.messages.remove(index);
    queue.cbytes = queue.cbytes.saturating_sub(message.payload.len());
    queue.lrpid = process.pid();
    queue.rtime = current_time_secs();
    copy_len as isize
}

pub(super) fn sys_msgctl(process: &UserProcess, msqid: usize, cmd: usize, buf: usize) -> isize {
    let msqid = msqid as i32;
    let cmd = cmd as i32;
    match cmd {
        SYSV_IPC_RMID => {
            let mut table = table().lock();
            let Some(queue) = table.get(&msqid) else {
                return neg_errno(LinuxError::EINVAL);
            };
            if !queue_control_allowed(queue, process) {
                return neg_errno(LinuxError::EPERM);
            }
            table.remove(&msqid);
            0
        }
        SYSV_IPC_STAT => {
            let table = table().lock();
            let Some(queue) = table.get(&msqid) else {
                return neg_errno(LinuxError::EINVAL);
            };
            if !queue_mode_allows(queue, process, true, false) {
                return neg_errno(LinuxError::EACCES);
            }
            let stat = stat_from_queue(queue);
            drop(table);
            let status = write_user_value(process, buf, &stat);
            if status < 0 {
                neg_errno(LinuxError::EFAULT)
            } else {
                0
            }
        }
        SYSV_IPC_SET => {
            let requested = match read_user_value::<UserMsqidDs64>(process, buf) {
                Ok(value) => value,
                Err(err) => return neg_errno(err),
            };
            match set_queue_metadata(process, msqid, requested) {
                Ok(()) => 0,
                Err(err) => neg_errno(err),
            }
        }
        _ => neg_errno(LinuxError::EINVAL),
    }
}
