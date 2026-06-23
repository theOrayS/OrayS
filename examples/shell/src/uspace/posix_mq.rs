use core::cmp;
use core::mem::size_of;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;

use axerrno::LinuxError;
use axsync::Mutex;
use lazyinit::LazyInit;
use linux_raw_sys::general;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use super::UserProcess;
use super::fd_table::{FdEntry, PathEntry};
use super::linux_abi::{RLIMIT_NOFILE_RESOURCE, neg_errno};
use super::select_fdset::yield_poll_wait;
use super::signal_abi::{
    current_unblocked_signal_pending, deliver_user_signal_with_siginfo, validate_signal_target,
};
use super::task_registry::{user_thread_entry_by_process_pid, user_thread_entry_by_tid};
use super::time_abi::timespec_to_duration;
use super::user_memory::{
    read_cstr, read_user_bytes, read_user_value, validate_user_read, validate_user_write,
    write_user_bytes, write_user_value,
};

const POSIX_MQ_NAME_MAX: usize = 255;
const POSIX_MQ_DEFAULT_MAXMSG: isize = 10;
const POSIX_MQ_DEFAULT_MSGSIZE: isize = 8192;
const POSIX_MQ_MAX_MAXMSG: isize = 64;
const POSIX_MQ_MAX_MSGSIZE: isize = 65_536;
const POSIX_MQ_DEFAULT_QUEUES_MAX: usize = 256;
const POSIX_MQ_PRIO_MAX: usize = 32_768;
const PROC_SYS_FS_MQUEUE_QUEUES_MAX_PATH: &str = "/proc/sys/fs/mqueue/queues_max";

static POSIX_MQ_QUEUES_MAX: AtomicUsize = AtomicUsize::new(POSIX_MQ_DEFAULT_QUEUES_MAX);

#[derive(Clone)]
pub(super) struct PosixMqDescriptor {
    queue: Arc<PosixMqQueue>,
    access: u32,
    status_flags: Arc<Mutex<u32>>,
}

#[derive(Clone)]
pub(super) struct ProcMqQueuesMaxEntry {
    path: String,
    offset: usize,
    status_flags: u32,
}

#[derive(Clone)]
struct PosixMqQueue {
    name: String,
    state: Arc<Mutex<PosixMqQueueState>>,
}

struct PosixMqQueueState {
    mode: u32,
    uid: u32,
    gid: u32,
    maxmsg: isize,
    msgsize: isize,
    messages: Vec<PosixMqMessage>,
    notify: Option<PosixMqNotify>,
}

#[derive(Clone)]
struct PosixMqMessage {
    prio: u32,
    seq: u64,
    data: Vec<u8>,
}

#[derive(Clone, Copy)]
enum PosixMqNotifyKind {
    None,
    Signal {
        signo: i32,
        value: i32,
        tid: Option<i32>,
    },
}

#[derive(Clone, Copy)]
struct PosixMqNotify {
    owner_pid: i32,
    owner_uid: u32,
    kind: PosixMqNotifyKind,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserMqAttr {
    mq_flags: isize,
    mq_maxmsg: isize,
    mq_msgsize: isize,
    mq_curmsgs: isize,
}

const _: [(); 32] = [(); size_of::<UserMqAttr>()];

fn namespace() -> &'static Mutex<BTreeMap<String, Arc<PosixMqQueue>>> {
    static NAMESPACE: LazyInit<Mutex<BTreeMap<String, Arc<PosixMqQueue>>>> = LazyInit::new();
    let _ = NAMESPACE.call_once(|| Mutex::new(BTreeMap::new()));
    &NAMESPACE
}

fn normalize_mq_name(name: &str) -> Result<String, LinuxError> {
    let body = name.strip_prefix('/').unwrap_or(name);
    if body.is_empty() || body.contains('/') {
        return Err(LinuxError::EINVAL);
    }
    if body.len() > POSIX_MQ_NAME_MAX {
        return Err(LinuxError::ENAMETOOLONG);
    }
    let mut normalized = String::with_capacity(body.len() + 1);
    normalized.push('/');
    normalized.push_str(body);
    Ok(normalized)
}

fn access_mode(flags: u32) -> u32 {
    flags & general::O_ACCMODE
}

fn descriptor_status_flags(flags: u32) -> u32 {
    flags & general::O_NONBLOCK
}

fn access_requires(flags: u32) -> (bool, bool) {
    match access_mode(flags) {
        general::O_WRONLY => (false, true),
        general::O_RDWR => (true, true),
        _ => (true, false),
    }
}

fn mode_allows(queue: &PosixMqQueueState, process: &UserProcess, read: bool, write: bool) -> bool {
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

fn control_allowed(queue: &PosixMqQueueState, process: &UserProcess) -> bool {
    process.uid() == 0 || process.uid() == queue.uid
}

fn read_create_attr(process: &UserProcess, attr: usize) -> Result<(isize, isize), LinuxError> {
    if attr == 0 {
        return Ok((POSIX_MQ_DEFAULT_MAXMSG, POSIX_MQ_DEFAULT_MSGSIZE));
    }
    let attr = read_user_value::<UserMqAttr>(process, attr)?;
    if attr.mq_maxmsg <= 0
        || attr.mq_msgsize <= 0
        || attr.mq_maxmsg > POSIX_MQ_MAX_MAXMSG
        || attr.mq_msgsize > POSIX_MQ_MAX_MSGSIZE
    {
        return Err(LinuxError::EINVAL);
    }
    Ok((attr.mq_maxmsg, attr.mq_msgsize))
}

fn open_fd_limit_reached(process: &UserProcess) -> bool {
    process.get_rlimit(RLIMIT_NOFILE_RESOURCE).current() == 0
}

fn new_queue(
    process: &UserProcess,
    name: String,
    mode: u32,
    maxmsg: isize,
    msgsize: isize,
) -> Arc<PosixMqQueue> {
    Arc::new(PosixMqQueue {
        name,
        state: Arc::new(Mutex::new(PosixMqQueueState {
            mode: mode & 0o777,
            uid: process.uid(),
            gid: process.gid(),
            maxmsg,
            msgsize,
            messages: Vec::new(),
            notify: None,
        })),
    })
}

impl PosixMqDescriptor {
    pub(super) fn new(queue: Arc<PosixMqQueue>, flags: u32) -> Self {
        Self {
            queue,
            access: access_mode(flags),
            status_flags: Arc::new(Mutex::new(descriptor_status_flags(flags))),
        }
    }

    pub(super) fn status_flags(&self) -> u32 {
        self.access | *self.status_flags.lock()
    }

    pub(super) fn set_status_flags(&self, flags: u32) {
        *self.status_flags.lock() = flags & general::O_NONBLOCK;
    }

    fn nonblocking(&self) -> bool {
        *self.status_flags.lock() & general::O_NONBLOCK != 0
    }

    fn readable(&self) -> bool {
        matches!(self.access, general::O_RDONLY | general::O_RDWR)
    }

    fn writable(&self) -> bool {
        matches!(self.access, general::O_WRONLY | general::O_RDWR)
    }

    pub(super) fn poll_readable(&self) -> bool {
        !self.queue.state.lock().messages.is_empty()
    }

    pub(super) fn poll_writable(&self) -> bool {
        let state = self.queue.state.lock();
        state.messages.len() < state.maxmsg as usize
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file("anon_inode:[mqueue]", 0).stat()
    }
}

impl ProcMqQueuesMaxEntry {
    fn new(path: String, status_flags: u32) -> Self {
        Self {
            path,
            offset: 0,
            status_flags,
        }
    }

    pub(super) fn status_flags(&self) -> u32 {
        self.status_flags
    }

    pub(super) fn set_status_flags(&mut self, flags: u32) {
        self.status_flags =
            (self.status_flags & general::O_ACCMODE) | (flags & general::O_NONBLOCK);
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file_with_mode(self.path.as_str(), queues_max_content().len(), 0o644)
            .stat()
    }

    pub(super) fn read(&mut self, dst: &mut [u8]) -> Result<usize, LinuxError> {
        if !file_is_readable(self.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let data = queues_max_content();
        let start = self.offset.min(data.len());
        let end = cmp::min(start + dst.len(), data.len());
        let len = end.saturating_sub(start);
        dst[..len].copy_from_slice(&data.as_bytes()[start..end]);
        self.offset = end;
        Ok(len)
    }

    pub(super) fn write(&mut self, src: &[u8]) -> Result<usize, LinuxError> {
        if !file_is_writable(self.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let text = core::str::from_utf8(src).map_err(|_| LinuxError::EINVAL)?;
        let value_text = text.split_whitespace().next().ok_or(LinuxError::EINVAL)?;
        let value = value_text
            .parse::<usize>()
            .map_err(|_| LinuxError::EINVAL)?;
        if value == 0 {
            return Err(LinuxError::EINVAL);
        }
        POSIX_MQ_QUEUES_MAX.store(value, Ordering::Release);
        self.offset = self.offset.saturating_add(src.len());
        Ok(src.len())
    }

    pub(super) fn seek(&mut self, pos: axio::SeekFrom) -> Result<u64, LinuxError> {
        let size = queues_max_content().len() as i64;
        let next = match pos {
            axio::SeekFrom::Start(offset) => offset as i64,
            axio::SeekFrom::Current(offset) => self.offset as i64 + offset,
            axio::SeekFrom::End(offset) => size + offset,
        };
        if next < 0 {
            return Err(LinuxError::EINVAL);
        }
        self.offset = next as usize;
        Ok(self.offset as u64)
    }
}

fn file_is_readable(flags: u32) -> bool {
    !matches!(flags & general::O_ACCMODE, general::O_WRONLY)
}

fn file_is_writable(flags: u32) -> bool {
    matches!(
        flags & general::O_ACCMODE,
        general::O_WRONLY | general::O_RDWR
    )
}

fn queues_max_content() -> String {
    format!("{}\n", POSIX_MQ_QUEUES_MAX.load(Ordering::Acquire))
}

pub(super) fn proc_sys_fs_mqueue_fd_entry(path: &str, status_flags: u32) -> Option<FdEntry> {
    let normalized = super::runtime_paths::normalize_path("/", path)?;
    (normalized == PROC_SYS_FS_MQUEUE_QUEUES_MAX_PATH)
        .then(|| FdEntry::ProcMqQueuesMax(ProcMqQueuesMaxEntry::new(normalized, status_flags)))
}

pub(super) fn proc_sys_fs_mqueue_path_entry(path: &str) -> Option<FdEntry> {
    let normalized = super::runtime_paths::normalize_path("/", path)?;
    (normalized == PROC_SYS_FS_MQUEUE_QUEUES_MAX_PATH).then(|| {
        FdEntry::Path(PathEntry::synthetic_file_with_mode(
            normalized.as_str(),
            queues_max_content().len(),
            0o644,
        ))
    })
}

pub(super) fn sys_mq_open(
    process: &UserProcess,
    name_ptr: usize,
    flags: usize,
    mode: usize,
    attr: usize,
) -> isize {
    let raw_name = match read_cstr(process, name_ptr) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    let name = match normalize_mq_name(raw_name.as_str()) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    let flags = flags as u32;
    let (read, write) = access_requires(flags);
    let mut ns = namespace().lock();
    if let Some(queue) = ns.get(name.as_str()).cloned() {
        if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
            return neg_errno(LinuxError::EEXIST);
        }
        if open_fd_limit_reached(process) {
            return neg_errno(LinuxError::EMFILE);
        }
        if !mode_allows(&queue.state.lock(), process, read, write) {
            return neg_errno(LinuxError::EACCES);
        }
        drop(ns);
        return insert_descriptor(process, queue, flags);
    }
    if flags & general::O_CREAT == 0 {
        return neg_errno(LinuxError::ENOENT);
    }
    if ns.len() >= POSIX_MQ_QUEUES_MAX.load(Ordering::Acquire) {
        return neg_errno(LinuxError::ENOSPC);
    }
    let (maxmsg, msgsize) = match read_create_attr(process, attr) {
        Ok(attr) => attr,
        Err(err) => return neg_errno(err),
    };
    if open_fd_limit_reached(process) {
        return neg_errno(LinuxError::EMFILE);
    }
    let queue = new_queue(process, name.clone(), mode as u32, maxmsg, msgsize);
    ns.insert(name, queue.clone());
    drop(ns);
    insert_descriptor(process, queue, flags)
}

fn insert_descriptor(process: &UserProcess, queue: Arc<PosixMqQueue>, flags: u32) -> isize {
    let fd_flags = if flags & general::O_CLOEXEC != 0 {
        general::FD_CLOEXEC
    } else {
        0
    };
    let descriptor = PosixMqDescriptor::new(queue, flags);
    match process
        .fds
        .lock()
        .insert_with_flags(FdEntry::PosixMq(descriptor), fd_flags)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_mq_unlink(process: &UserProcess, name_ptr: usize) -> isize {
    let raw_name = match read_cstr(process, name_ptr) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    let name = match normalize_mq_name(raw_name.as_str()) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    let mut ns = namespace().lock();
    let Some(queue) = ns.get(name.as_str()).cloned() else {
        return neg_errno(LinuxError::ENOENT);
    };
    if !control_allowed(&queue.state.lock(), process) {
        return neg_errno(LinuxError::EACCES);
    }
    ns.remove(name.as_str());
    0
}

fn mq_descriptor(process: &UserProcess, fd: i32) -> Result<PosixMqDescriptor, LinuxError> {
    let table = process.fds.lock();
    match table.entry(fd)? {
        FdEntry::PosixMq(desc) => Ok(desc.clone()),
        _ => Err(LinuxError::EBADF),
    }
}

fn read_abs_deadline(
    process: &UserProcess,
    abs_timeout: usize,
) -> Result<Option<Duration>, LinuxError> {
    if abs_timeout == 0 {
        return Ok(None);
    }
    let ts = read_user_value::<general::timespec>(process, abs_timeout)?;
    timespec_to_duration(ts).map(Some)
}

fn mq_interrupted(process: &UserProcess) -> bool {
    current_unblocked_signal_pending()
        || process.pending_exit_group().is_some()
        || process.eval_watchdog_expired()
}

fn deadline_expired(deadline: Option<Duration>) -> bool {
    deadline.is_some_and(|deadline| axhal::time::wall_time() >= deadline)
}

pub(super) fn sys_mq_timedsend(
    process: &UserProcess,
    fd: usize,
    msg_ptr: usize,
    msg_len: usize,
    msg_prio: usize,
    abs_timeout: usize,
) -> isize {
    let desc = match mq_descriptor(process, fd as i32) {
        Ok(desc) => desc,
        Err(err) => return neg_errno(err),
    };
    if !desc.writable() {
        return neg_errno(LinuxError::EBADF);
    }
    if msg_prio >= POSIX_MQ_PRIO_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    let msgsize = desc.queue.state.lock().msgsize as usize;
    if msg_len > msgsize {
        return neg_errno(LinuxError::EMSGSIZE);
    }
    if let Err(err) = validate_user_read(process, msg_ptr, msg_len) {
        return neg_errno(err);
    }
    let payload = match read_user_bytes(process, msg_ptr, msg_len) {
        Ok(payload) => payload,
        Err(err) => return neg_errno(err),
    };
    let mut deadline = None;
    let mut timeout_read = false;
    loop {
        let mut sent = false;
        let notify = {
            let mut state = desc.queue.state.lock();
            if state.messages.len() < state.maxmsg as usize {
                let was_empty = state.messages.is_empty();
                let seq = state
                    .messages
                    .iter()
                    .map(|msg| msg.seq)
                    .max()
                    .unwrap_or(0)
                    .saturating_add(1);
                state.messages.push(PosixMqMessage {
                    prio: msg_prio as u32,
                    seq,
                    data: payload.clone(),
                });
                state.messages.sort_by(|left, right| {
                    right.prio.cmp(&left.prio).then(left.seq.cmp(&right.seq))
                });
                sent = true;
                if was_empty { state.notify.take() } else { None }
            } else {
                None
            }
        };
        if let Some(notify) = notify {
            deliver_mq_notification(process, notify);
        }
        if sent {
            return 0;
        }
        if desc.nonblocking() {
            return neg_errno(LinuxError::EAGAIN);
        }
        if !timeout_read {
            deadline = match read_abs_deadline(process, abs_timeout) {
                Ok(deadline) => deadline,
                Err(err) => return neg_errno(err),
            };
            timeout_read = true;
        }
        if deadline_expired(deadline) {
            return neg_errno(LinuxError::ETIMEDOUT);
        }
        if mq_interrupted(process) {
            return neg_errno(LinuxError::EINTR);
        }
        yield_poll_wait();
    }
}

pub(super) fn sys_mq_timedreceive(
    process: &UserProcess,
    fd: usize,
    msg_ptr: usize,
    msg_len: usize,
    msg_prio_ptr: usize,
    abs_timeout: usize,
) -> isize {
    let desc = match mq_descriptor(process, fd as i32) {
        Ok(desc) => desc,
        Err(err) => return neg_errno(err),
    };
    if !desc.readable() {
        return neg_errno(LinuxError::EBADF);
    }
    let msgsize = desc.queue.state.lock().msgsize as usize;
    if msg_len < msgsize {
        return neg_errno(LinuxError::EMSGSIZE);
    }
    if let Err(err) = validate_user_write(process, msg_ptr, msg_len.min(msgsize)) {
        return neg_errno(err);
    }
    let mut deadline = None;
    let mut timeout_read = false;
    loop {
        if let Some(message) = {
            let mut state = desc.queue.state.lock();
            if state.messages.is_empty() {
                None
            } else {
                Some(state.messages.remove(0))
            }
        } {
            if let Err(err) = write_user_bytes(process, msg_ptr, message.data.as_slice()) {
                return neg_errno(err);
            }
            if msg_prio_ptr != 0 {
                let ret = write_user_value(process, msg_prio_ptr, &message.prio);
                if ret != 0 {
                    return ret;
                }
            }
            return message.data.len() as isize;
        }
        if desc.nonblocking() {
            return neg_errno(LinuxError::EAGAIN);
        }
        if !timeout_read {
            deadline = match read_abs_deadline(process, abs_timeout) {
                Ok(deadline) => deadline,
                Err(err) => return neg_errno(err),
            };
            timeout_read = true;
        }
        if deadline_expired(deadline) {
            return neg_errno(LinuxError::ETIMEDOUT);
        }
        if mq_interrupted(process) {
            return neg_errno(LinuxError::EINTR);
        }
        yield_poll_wait();
    }
}

pub(super) fn sys_mq_getsetattr(
    process: &UserProcess,
    fd: usize,
    newattr: usize,
    oldattr: usize,
) -> isize {
    let desc = match mq_descriptor(process, fd as i32) {
        Ok(desc) => desc,
        Err(err) => return neg_errno(err),
    };
    if oldattr != 0 {
        let state = desc.queue.state.lock();
        let attr = UserMqAttr {
            mq_flags: (*desc.status_flags.lock() & general::O_NONBLOCK) as isize,
            mq_maxmsg: state.maxmsg,
            mq_msgsize: state.msgsize,
            mq_curmsgs: state.messages.len() as isize,
        };
        let ret = write_user_value(process, oldattr, &attr);
        if ret != 0 {
            return ret;
        }
    }
    if newattr != 0 {
        let attr = match read_user_value::<UserMqAttr>(process, newattr) {
            Ok(attr) => attr,
            Err(err) => return neg_errno(err),
        };
        desc.set_status_flags(attr.mq_flags as u32);
    }
    0
}

fn parse_mq_notify(
    process: &UserProcess,
    sevp: usize,
) -> Result<Option<PosixMqNotifyKind>, LinuxError> {
    if sevp == 0 {
        return Ok(None);
    }
    let ev = read_user_value::<general::sigevent>(process, sevp)?;
    match ev.sigev_notify {
        value if value == general::SIGEV_NONE as i32 => Ok(Some(PosixMqNotifyKind::None)),
        value if value == general::SIGEV_SIGNAL as i32 => {
            validate_signal_target(ev.sigev_signo)?;
            let value = unsafe { ev.sigev_value.sival_int };
            Ok(Some(PosixMqNotifyKind::Signal {
                signo: ev.sigev_signo,
                value,
                tid: None,
            }))
        }
        value if value == general::SIGEV_THREAD_ID as i32 => {
            validate_signal_target(ev.sigev_signo)?;
            let tid = unsafe { ev._sigev_un._tid };
            if tid <= 0 {
                return Err(LinuxError::EINVAL);
            }
            let value = unsafe { ev.sigev_value.sival_int };
            Ok(Some(PosixMqNotifyKind::Signal {
                signo: ev.sigev_signo,
                value,
                tid: Some(tid),
            }))
        }
        value if value == general::SIGEV_THREAD as i32 => Err(LinuxError::EINVAL),
        _ => Err(LinuxError::EINVAL),
    }
}

pub(super) fn sys_mq_notify(process: &UserProcess, fd: usize, sevp: usize) -> isize {
    let notify_kind = match parse_mq_notify(process, sevp) {
        Ok(kind) => kind,
        Err(err) => return neg_errno(err),
    };
    let desc = match mq_descriptor(process, fd as i32) {
        Ok(desc) => desc,
        Err(err) => return neg_errno(err),
    };
    let mut state = desc.queue.state.lock();
    let Some(kind) = notify_kind else {
        state.notify = None;
        return 0;
    };
    if state.notify.is_some() {
        return neg_errno(LinuxError::EBUSY);
    }
    state.notify = Some(PosixMqNotify {
        owner_pid: process.pid(),
        owner_uid: process.uid(),
        kind,
    });
    0
}

fn deliver_mq_notification(process: &UserProcess, notify: PosixMqNotify) {
    let PosixMqNotifyKind::Signal { signo, value, tid } = notify.kind else {
        return;
    };
    let entry = tid
        .and_then(user_thread_entry_by_tid)
        .or_else(|| user_thread_entry_by_process_pid(process.pid()));
    if let Some(entry) = entry {
        let _ = deliver_user_signal_with_siginfo(
            &entry,
            signo,
            notify.owner_pid,
            general::SI_MESGQ,
            notify.owner_uid,
            value,
        );
    }
}
