use core::mem::size_of;
use core::sync::atomic::{AtomicI32, Ordering};
use core::time::Duration;

use axerrno::LinuxError;
use axsync::Mutex;
use lazyinit::LazyInit;
use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

use super::UserProcess;
use super::linux_abi::{
    SYSV_IPC_CREAT, SYSV_IPC_EXCL, SYSV_IPC_INFO, SYSV_IPC_PRIVATE, SYSV_IPC_RMID, SYSV_IPC_SET,
    SYSV_IPC_STAT, neg_errno,
};
use super::signal_abi::current_unblocked_signal_pending;
use super::task_context::current_task_ext;
use super::user_memory::{
    read_user_bytes, read_user_value, validate_user_write, write_user_bytes, write_user_value,
};

const SYSV_MSG_MAX_QUEUES: usize = 128;
const SYSV_MSG_MAX_BYTES: usize = 16 * 1024;
const SYSV_MSG_MAX_SIZE: usize = 8 * 1024;
const SYSV_IPC_NOWAIT: i32 = 0o4000;
const SYSV_MSG_NOERROR: i32 = 0o10000;
const SYSV_MSG_EXCEPT: i32 = 0o20000;
const SYSV_MSG_COPY: i32 = 0o40000;
const SYSV_MSG_STAT: i32 = 11;
const SYSV_MSG_INFO: i32 = 12;
const SYSV_MSG_STAT_ANY: i32 = 13;
const PROC_SYS_KERNEL_MSGMNI_PATH: &str = "/proc/sys/kernel/msgmni";
const PROC_SYS_KERNEL_MSGMNI_CONTENT: &[u8] = b"128\n";
const PROC_SYS_KERNEL_MSGMAX_PATH: &str = "/proc/sys/kernel/msgmax";
const PROC_SYS_KERNEL_MSGMAX_CONTENT: &[u8] = b"8192\n";
const PROC_SYS_KERNEL_MSGMNB_PATH: &str = "/proc/sys/kernel/msgmnb";
const PROC_SYS_KERNEL_MSGMNB_CONTENT: &[u8] = b"16384\n";
const SYSV_MSG_BLOCK_QUANTUM: Duration = Duration::from_millis(1);

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

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserMsginfo {
    msgpool: i32,
    msgmap: i32,
    msgmax: i32,
    msgmnb: i32,
    msgmni: i32,
    msgssz: i32,
    msgtql: i32,
    msgseg: i32,
}

// Linux 64-bit IPC user ABI: asm-generic/{ipcbuf,msgbuf}.h. RISC-V and
// LoongArch use this msqid64_ds layout for msgctl(IPC_STAT/IPC_SET).
const _: [(); 48] = [(); size_of::<UserIpcPerm64>()];
const _: [(); 120] = [(); size_of::<UserMsqidDs64>()];
const _: [(); 32] = [(); size_of::<UserMsginfo>()];

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

fn msgop_interrupted(process: &UserProcess) -> bool {
    current_unblocked_signal_pending()
        || process.pending_exit_group().is_some()
        || process.eval_watchdog_expired()
}

struct SyscallWaitBlockedGuard {
    active: bool,
}

impl SyscallWaitBlockedGuard {
    fn new() -> Self {
        let active = current_task_ext().is_some_and(|ext| {
            ext.process.set_syscall_wait_blocked(true);
            true
        });
        Self { active }
    }
}

impl Drop for SyscallWaitBlockedGuard {
    fn drop(&mut self) {
        if self.active {
            if let Some(ext) = current_task_ext() {
                ext.process.set_syscall_wait_blocked(false);
            }
        }
    }
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

fn active_msg_snapshot() -> (i32, i32, i32, i32) {
    let table = table().lock();
    let max_id = table.keys().copied().max().unwrap_or(0);
    let used_queues = table.len().min(i32::MAX as usize) as i32;
    let queued_messages = table
        .values()
        .map(|queue| queue.messages.len())
        .sum::<usize>()
        .min(i32::MAX as usize) as i32;
    let queued_bytes = table
        .values()
        .map(|queue| queue.cbytes)
        .sum::<usize>()
        .min(i32::MAX as usize) as i32;
    (max_id, used_queues, queued_messages, queued_bytes)
}

fn msginfo(
    used_queues: i32,
    queued_messages: i32,
    queued_bytes: i32,
    info_mode: bool,
) -> UserMsginfo {
    UserMsginfo {
        msgpool: if info_mode {
            used_queues
        } else {
            SYSV_MSG_MAX_QUEUES as i32
        },
        msgmap: if info_mode {
            queued_messages
        } else {
            SYSV_MSG_MAX_QUEUES as i32
        },
        msgmax: SYSV_MSG_MAX_SIZE as i32,
        msgmnb: SYSV_MSG_MAX_BYTES as i32,
        msgmni: SYSV_MSG_MAX_QUEUES as i32,
        msgssz: 16,
        msgtql: if info_mode {
            queued_bytes
        } else {
            SYSV_MSG_MAX_QUEUES as i32
        },
        msgseg: SYSV_MSG_MAX_QUEUES as i32,
    }
}

pub(super) fn proc_sysvipc_msg_content() -> Vec<u8> {
    let table = table().lock();
    let mut content = String::from(
        "       key      msqid perms      cbytes       qnum      lspid      lrpid   uid   gid  cuid  cgid      stime      rtime      ctime\n",
    );
    for (msqid, queue) in table.iter() {
        content.push_str(&format!(
            "{key:10} {msqid:10} {mode:5o} {cbytes:11} {qnum:10} {lspid:10} {lrpid:10} {uid:5} {gid:5} {cuid:5} {cgid:5} {stime:10} {rtime:10} {ctime:10}\n",
            key = queue.key,
            msqid = msqid,
            mode = queue.mode,
            cbytes = queue.cbytes,
            qnum = queue.messages.len(),
            lspid = queue.lspid,
            lrpid = queue.lrpid,
            uid = queue.uid,
            gid = queue.gid,
            cuid = queue.cuid,
            cgid = queue.cgid,
            stime = queue.stime,
            rtime = queue.rtime,
            ctime = queue.ctime,
        ));
    }
    content.into_bytes()
}

pub(super) fn proc_sys_kernel_msg_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    match path {
        PROC_SYS_KERNEL_MSGMNI_PATH => {
            Some((PROC_SYS_KERNEL_MSGMNI_PATH, PROC_SYS_KERNEL_MSGMNI_CONTENT))
        }
        PROC_SYS_KERNEL_MSGMAX_PATH => {
            Some((PROC_SYS_KERNEL_MSGMAX_PATH, PROC_SYS_KERNEL_MSGMAX_CONTENT))
        }
        PROC_SYS_KERNEL_MSGMNB_PATH => {
            Some((PROC_SYS_KERNEL_MSGMNB_PATH, PROC_SYS_KERNEL_MSGMNB_CONTENT))
        }
        _ => None,
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

    let mut queue_was_seen = false;
    let mut wait_guard = None;
    loop {
        {
            let mut table = table().lock();
            let Some(queue) = table.get_mut(&msqid) else {
                return neg_errno(if queue_was_seen {
                    LinuxError::EIDRM
                } else {
                    LinuxError::EINVAL
                });
            };
            queue_was_seen = true;
            if !queue_mode_allows(queue, process, false, true) {
                return neg_errno(LinuxError::EACCES);
            }
            let Some(next_bytes) = queue.cbytes.checked_add(payload.len()) else {
                return neg_errno(LinuxError::EAGAIN);
            };
            if next_bytes <= queue.qbytes {
                queue.messages.push(SysvMessage {
                    mtype,
                    payload: payload.clone(),
                });
                queue.cbytes = next_bytes;
                queue.lspid = process.pid();
                queue.stime = current_time_secs();
                return 0;
            }
        }

        if flags & SYSV_IPC_NOWAIT != 0 {
            return neg_errno(LinuxError::EAGAIN);
        }
        if msgop_interrupted(process) {
            return neg_errno(LinuxError::EINTR);
        }
        wait_guard.get_or_insert_with(SyscallWaitBlockedGuard::new);
        axtask::sleep(SYSV_MSG_BLOCK_QUANTUM);
    }
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
    const KNOWN_FLAGS: i32 = SYSV_IPC_NOWAIT | SYSV_MSG_NOERROR | SYSV_MSG_EXCEPT | SYSV_MSG_COPY;
    if flags & !KNOWN_FLAGS != 0
        || msgsz > SYSV_MSG_MAX_SIZE
        || (flags & SYSV_MSG_COPY != 0 && flags & SYSV_MSG_EXCEPT != 0)
        || (flags & SYSV_MSG_COPY != 0 && flags & SYSV_IPC_NOWAIT == 0)
    {
        return neg_errno(LinuxError::EINVAL);
    }

    let copy_by_ordinal = flags & SYSV_MSG_COPY != 0;
    let mut queue_was_seen = false;
    let mut wait_guard = None;
    loop {
        {
            let mut table = table().lock();
            let Some(queue) = table.get_mut(&msqid) else {
                return neg_errno(if queue_was_seen {
                    LinuxError::EIDRM
                } else {
                    LinuxError::EINVAL
                });
            };
            queue_was_seen = true;
            if !queue_mode_allows(queue, process, true, false) {
                return neg_errno(LinuxError::EACCES);
            }
            let index = if copy_by_ordinal {
                if msgtyp < 0 {
                    return neg_errno(LinuxError::EINVAL);
                }
                let index = msgtyp as usize;
                (index < queue.messages.len()).then_some(index)
            } else {
                select_message_index(&queue.messages, msgtyp, flags)
            };
            if let Some(index) = index {
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
                if !copy_by_ordinal {
                    let message = queue.messages.remove(index);
                    queue.cbytes = queue.cbytes.saturating_sub(message.payload.len());
                    queue.lrpid = process.pid();
                    queue.rtime = current_time_secs();
                }
                return copy_len as isize;
            }
        }

        if flags & SYSV_IPC_NOWAIT != 0 {
            return neg_errno(LinuxError::ENOMSG);
        }
        if msgop_interrupted(process) {
            return neg_errno(LinuxError::EINTR);
        }
        wait_guard.get_or_insert_with(SyscallWaitBlockedGuard::new);
        axtask::sleep(SYSV_MSG_BLOCK_QUANTUM);
    }
}

pub(super) fn sys_msgctl(process: &UserProcess, msqid: usize, cmd: usize, buf: usize) -> isize {
    let msqid = msqid as i32;
    let cmd = cmd as i32;
    match cmd {
        SYSV_IPC_INFO | SYSV_MSG_INFO => {
            if buf == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let (max_id, used_queues, queued_messages, queued_bytes) = active_msg_snapshot();
            let info = msginfo(
                used_queues,
                queued_messages,
                queued_bytes,
                cmd == SYSV_MSG_INFO,
            );
            let ret = write_user_value(process, buf, &info);
            if ret != 0 {
                return ret;
            }
            max_id as isize
        }
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
            if buf == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
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
        SYSV_MSG_STAT | SYSV_MSG_STAT_ANY => {
            if buf == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let table = table().lock();
            let Some(queue) = table.get(&msqid) else {
                return neg_errno(LinuxError::EINVAL);
            };
            if cmd == SYSV_MSG_STAT && !queue_mode_allows(queue, process, true, false) {
                return neg_errno(LinuxError::EACCES);
            }
            let stat = stat_from_queue(queue);
            drop(table);
            let ret = write_user_value(process, buf, &stat);
            if ret != 0 {
                return ret;
            }
            msqid as isize
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
