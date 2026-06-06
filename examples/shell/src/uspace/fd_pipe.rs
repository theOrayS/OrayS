use axerrno::LinuxError;
use axsync::Mutex;
use lazyinit::LazyInit;
use linux_raw_sys::general;
use std::collections::BTreeMap;
use std::string::String;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::vec::Vec;

use super::fd_table::FdEntry;
use super::linux_abi::{fd_cloexec_flag, SIGPIPE_NUM};
use super::signal_abi::{current_unblocked_signal_pending, deliver_user_signal};
use super::task_context::{current_task_ext, current_tid};
use super::task_registry::{
    user_thread_entries_by_process_group, user_thread_entry_by_process_pid,
    user_thread_entry_by_tid,
};
use super::user_memory::{read_user_value, validate_user_write, write_user_value};
use super::{neg_errno, UserProcess};

#[derive(Clone, Copy, Eq, PartialEq)]
enum RingBufferStatus {
    Full,
    Empty,
    Normal,
}

const PIPE_BUF_SIZE: usize = 4096;
const PIPE_MAX_CAPACITY_SIZE: usize = 65536;
const PIPE_DEFAULT_CAPACITY_SIZE: usize = PIPE_MAX_CAPACITY_SIZE;
const PIPE_UNPRIVILEGED_CAPACITY_SIZE: usize = PIPE_BUF_SIZE;
const F_SETOWN_EX: u32 = 15;
const F_GETOWN_EX: u32 = 16;
const F_OWNER_TID: i32 = 0;
const F_OWNER_PID: i32 = 1;
const F_OWNER_PGRP: i32 = 2;
const SIGIO_NUM: i32 = 29;
const O_ASYNC_FLAG: u32 = 0o20000;

struct PipeRingBuffer {
    data: Vec<u8>,
    head: usize,
    tail: usize,
    capacity: usize,
    status: RingBufferStatus,
}

struct PipePeerCounts {
    readers: AtomicUsize,
    writers: AtomicUsize,
    buffered: AtomicUsize,
}

#[derive(Clone, Copy)]
struct PipeAsyncState {
    enabled: bool,
    owner_type: i32,
    owner_pid: i32,
    signal: i32,
}

impl PipeAsyncState {
    const fn new() -> Self {
        Self {
            enabled: false,
            owner_type: F_OWNER_PID,
            owner_pid: 0,
            signal: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FOwnerEx {
    owner_type: i32,
    pid: i32,
}

pub(super) struct PipeEndpoint {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
    status_flags: Arc<Mutex<u32>>,
    async_state: Arc<Mutex<PipeAsyncState>>,
    peers: Arc<PipePeerCounts>,
    fifo_path: Option<Arc<String>>,
}

impl Clone for PipeEndpoint {
    fn clone(&self) -> Self {
        if self.readable {
            self.peers.readers.fetch_add(1, Ordering::AcqRel);
        }
        if self.writable {
            self.peers.writers.fetch_add(1, Ordering::AcqRel);
        }
        Self {
            readable: self.readable,
            writable: self.writable,
            buffer: self.buffer.clone(),
            status_flags: self.status_flags.clone(),
            async_state: self.async_state.clone(),
            peers: self.peers.clone(),
            fifo_path: self.fifo_path.clone(),
        }
    }
}

impl Drop for PipeEndpoint {
    fn drop(&mut self) {
        if self.readable {
            self.peers.readers.fetch_sub(1, Ordering::AcqRel);
        }
        if self.writable {
            self.peers.writers.fetch_sub(1, Ordering::AcqRel);
        }
        if let Some(path) = &self.fifo_path {
            if self.peers.readers.load(Ordering::Acquire) == 0
                && self.peers.writers.load(Ordering::Acquire) == 0
            {
                named_fifo_table().lock().remove(path.as_str());
            }
        }
    }
}

#[derive(Clone)]
struct NamedFifoState {
    buffer: Arc<Mutex<PipeRingBuffer>>,
    async_state: Arc<Mutex<PipeAsyncState>>,
    peers: Arc<PipePeerCounts>,
    path: Arc<String>,
}

fn named_fifo_table() -> &'static Mutex<BTreeMap<String, NamedFifoState>> {
    static NAMED_FIFOS: LazyInit<Mutex<BTreeMap<String, NamedFifoState>>> = LazyInit::new();
    let _ = NAMED_FIFOS.call_once(|| Mutex::new(BTreeMap::new()));
    &NAMED_FIFOS
}

fn named_fifo_state(path: &str) -> NamedFifoState {
    let mut table = named_fifo_table().lock();
    table
        .entry(path.into())
        .or_insert_with(|| {
            let key = Arc::new(String::from(path));
            NamedFifoState {
                buffer: Arc::new(Mutex::new(PipeRingBuffer::new(PIPE_DEFAULT_CAPACITY_SIZE))),
                async_state: Arc::new(Mutex::new(PipeAsyncState::new())),
                peers: Arc::new(PipePeerCounts {
                    readers: AtomicUsize::new(0),
                    writers: AtomicUsize::new(0),
                    buffered: AtomicUsize::new(0),
                }),
                path: key,
            }
        })
        .clone()
}

impl PipeRingBuffer {
    fn new(capacity: usize) -> Self {
        let mut data = Vec::new();
        data.resize(PIPE_MAX_CAPACITY_SIZE, 0);
        Self {
            data,
            head: 0,
            tail: 0,
            capacity,
            status: RingBufferStatus::Empty,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        self.status = RingBufferStatus::Normal;
        self.data[self.tail] = byte;
        self.tail = (self.tail + 1) % self.capacity;
        if self.tail == self.head {
            self.status = RingBufferStatus::Full;
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::Normal;
        let byte = self.data[self.head];
        self.head = (self.head + 1) % self.capacity;
        if self.head == self.tail {
            self.status = RingBufferStatus::Empty;
        }
        byte
    }

    const fn available_read(&self) -> usize {
        if matches!(self.status, RingBufferStatus::Empty) {
            0
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.tail + self.capacity - self.head
        }
    }

    const fn available_write(&self) -> usize {
        if matches!(self.status, RingBufferStatus::Full) {
            0
        } else {
            self.capacity - self.available_read()
        }
    }
}

impl PipeEndpoint {
    pub(super) fn new_pair_for_process(process: &UserProcess, status_flags: u32) -> (Self, Self) {
        let capacity = if process.fs_uid() == 0 {
            PIPE_DEFAULT_CAPACITY_SIZE
        } else {
            PIPE_UNPRIVILEGED_CAPACITY_SIZE
        };
        Self::new_pair_with_capacity(status_flags, capacity)
    }

    pub(super) fn named_fifo_has_reader(path: &str) -> bool {
        named_fifo_table()
            .lock()
            .get(path)
            .is_some_and(|state| state.peers.readers.load(Ordering::Acquire) > 0)
    }

    pub(super) fn new_named_fifo(path: &str, access: u32, status_flags: u32) -> Self {
        let state = named_fifo_state(path);
        let readable = access != general::O_WRONLY;
        let writable = access != general::O_RDONLY;
        if readable {
            state.peers.readers.fetch_add(1, Ordering::AcqRel);
        }
        if writable {
            state.peers.writers.fetch_add(1, Ordering::AcqRel);
        }
        Self {
            readable,
            writable,
            buffer: state.buffer,
            status_flags: Arc::new(Mutex::new(access | (status_flags & !general::O_ACCMODE))),
            async_state: state.async_state,
            peers: state.peers,
            fifo_path: Some(state.path),
        }
    }

    fn new_pair_with_capacity(status_flags: u32, capacity: usize) -> (Self, Self) {
        let buffer = Arc::new(Mutex::new(PipeRingBuffer::new(capacity)));
        let async_state = Arc::new(Mutex::new(PipeAsyncState::new()));
        let peers = Arc::new(PipePeerCounts {
            readers: AtomicUsize::new(1),
            writers: AtomicUsize::new(1),
            buffered: AtomicUsize::new(0),
        });
        (
            Self {
                readable: true,
                writable: false,
                buffer: buffer.clone(),
                status_flags: Arc::new(Mutex::new(status_flags & !general::O_ACCMODE)),
                async_state: async_state.clone(),
                peers: peers.clone(),
                fifo_path: None,
            },
            Self {
                readable: false,
                writable: true,
                buffer,
                status_flags: Arc::new(Mutex::new(
                    general::O_WRONLY | (status_flags & !general::O_ACCMODE),
                )),
                async_state,
                peers,
                fifo_path: None,
            },
        )
    }

    fn read_peer_closed(&self) -> bool {
        self.peers.writers.load(Ordering::Acquire) == 0
    }

    fn write_peer_closed(&self) -> bool {
        self.peers.readers.load(Ordering::Acquire) == 0
    }

    fn nonblocking(&self) -> bool {
        *self.status_flags.lock() & general::O_NONBLOCK != 0
    }

    pub(super) fn status_flags(&self) -> u32 {
        *self.status_flags.lock()
    }

    pub(super) const fn readable(&self) -> bool {
        self.readable
    }

    pub(super) const fn writable(&self) -> bool {
        self.writable
    }

    pub(super) fn set_status_flags(&self, flags: u32) {
        let access = self.status_flags() & general::O_ACCMODE;
        *self.status_flags.lock() =
            access | (flags & (general::O_NONBLOCK | general::O_DIRECT | O_ASYNC_FLAG));
        self.async_state.lock().enabled = flags & O_ASYNC_FLAG != 0;
    }

    pub(super) fn fcntl_async_owner(
        &self,
        process: &UserProcess,
        cmd: u32,
        arg: usize,
    ) -> Result<Option<i32>, LinuxError> {
        match cmd {
            general::F_SETOWN => {
                let owner = arg as i32;
                let mut state = self.async_state.lock();
                if owner < 0 {
                    state.owner_type = F_OWNER_PGRP;
                    state.owner_pid = owner.saturating_neg();
                } else {
                    state.owner_type = F_OWNER_PID;
                    state.owner_pid = owner;
                }
                Ok(Some(0))
            }
            general::F_GETOWN => {
                let state = self.async_state.lock();
                let owner = if state.owner_type == F_OWNER_PGRP {
                    -state.owner_pid
                } else {
                    state.owner_pid
                };
                Ok(Some(owner))
            }
            F_SETOWN_EX => {
                let owner: FOwnerEx = read_user_value(process, arg)?;
                if !matches!(owner.owner_type, F_OWNER_TID | F_OWNER_PID | F_OWNER_PGRP)
                    || owner.pid < 0
                {
                    return Err(LinuxError::EINVAL);
                }
                let mut state = self.async_state.lock();
                state.owner_type = owner.owner_type;
                state.owner_pid = owner.pid;
                Ok(Some(0))
            }
            F_GETOWN_EX => {
                let state = self.async_state.lock();
                let owner = FOwnerEx {
                    owner_type: state.owner_type,
                    pid: state.owner_pid,
                };
                if write_user_value(process, arg, &owner) == 0 {
                    Ok(Some(0))
                } else {
                    Err(LinuxError::EFAULT)
                }
            }
            general::F_SETSIG => {
                let sig = arg as i32;
                if !(sig == 0 || (1..=64).contains(&sig)) {
                    return Err(LinuxError::EINVAL);
                }
                self.async_state.lock().signal = sig;
                Ok(Some(0))
            }
            general::F_GETSIG => Ok(Some(self.async_state.lock().signal)),
            _ => Ok(None),
        }
    }

    pub(super) fn capacity(&self) -> usize {
        self.buffer.lock().capacity
    }

    pub(super) fn set_capacity(&self, requested: usize) -> Result<usize, LinuxError> {
        if requested > (1usize << 31) {
            return Err(LinuxError::EINVAL);
        }
        if requested > PIPE_MAX_CAPACITY_SIZE {
            return Err(LinuxError::EPERM);
        }
        let requested = requested.max(PIPE_BUF_SIZE);
        let mut ring = self.buffer.lock();
        if requested == ring.capacity {
            return Ok(ring.capacity);
        }
        if !matches!(ring.status, RingBufferStatus::Empty) {
            return Err(LinuxError::EBUSY);
        }
        ring.head = 0;
        ring.tail = 0;
        ring.capacity = requested;
        Ok(ring.capacity)
    }

    pub(super) fn available_read(&self) -> usize {
        self.peers.buffered.load(Ordering::Acquire)
    }

    pub(super) fn tee_to(
        &self,
        dst: &Self,
        len: usize,
        nonblocking: bool,
    ) -> Result<usize, LinuxError> {
        if len == 0 {
            return Ok(0);
        }
        if !self.readable || !dst.writable {
            return Err(LinuxError::EBADF);
        }
        if Arc::ptr_eq(&self.buffer, &dst.buffer) {
            return Err(LinuxError::EINVAL);
        }
        loop {
            if dst.write_peer_closed() {
                Self::raise_sigpipe();
                return Err(LinuxError::EPIPE);
            }
            let mut src_ring = self.buffer.lock();
            let available_read = src_ring.available_read();
            if available_read == 0 {
                if self.read_peer_closed() {
                    return Ok(0);
                }
                drop(src_ring);
                if nonblocking || self.nonblocking() {
                    return Err(LinuxError::EAGAIN);
                }
                if Self::interrupted() {
                    return Err(LinuxError::EINTR);
                }
                Self::sleep_while_blocked();
                continue;
            }
            let mut dst_ring = dst.buffer.lock();
            let available_write = dst_ring.available_write();
            if available_write == 0 {
                drop(dst_ring);
                drop(src_ring);
                if nonblocking || dst.nonblocking() {
                    return Err(LinuxError::EAGAIN);
                }
                if Self::interrupted() {
                    return Err(LinuxError::EINTR);
                }
                Self::sleep_while_blocked();
                continue;
            }

            let to_copy = len.min(available_read).min(available_write);
            let mut cursor = src_ring.head;
            for _ in 0..to_copy {
                let byte = src_ring.data[cursor];
                cursor = (cursor + 1) % src_ring.capacity;
                dst_ring.write_byte(byte);
                dst.peers.buffered.fetch_add(1, Ordering::AcqRel);
            }
            drop(dst_ring);
            drop(src_ring);
            if to_copy > 0 {
                dst.notify_async_readable();
            }
            return Ok(to_copy);
        }
    }

    fn sleep_while_blocked() {
        if let Some(ext) = current_task_ext() {
            ext.process.set_syscall_wait_blocked(true);
            axtask::yield_now();
            ext.process.set_syscall_wait_blocked(false);
        } else {
            axtask::yield_now();
        }
    }

    fn raise_sigpipe() {
        let Some(ext) = current_task_ext() else {
            return;
        };
        if let Some(entry) = user_thread_entry_by_tid(current_tid()) {
            // `write(2)` on a pipe with no readers raises SIGPIPE and reports
            // EPIPE when the signal is ignored/handled/blocked.  The fd-table
            // syscall path still holds `process.fds` while it calls into a pipe
            // endpoint, so do not synchronously tear the process down here: the
            // normal user-return hook observes the pending default-fatal signal
            // after the fd-table lock has been released and then performs the
            // exit-group teardown.
            let _ = deliver_user_signal(&entry, SIGPIPE_NUM, ext.process.pid());
        }
    }

    fn notify_async_readable(&self) {
        let state = *self.async_state.lock();
        if !state.enabled || state.owner_pid == 0 {
            return;
        }
        let Some(ext) = current_task_ext() else {
            return;
        };
        let sig = if state.signal == 0 {
            SIGIO_NUM
        } else {
            state.signal
        };
        match state.owner_type {
            F_OWNER_TID => {
                if let Some(entry) = user_thread_entry_by_tid(state.owner_pid) {
                    let _ = deliver_user_signal(&entry, sig, ext.process.pid());
                }
            }
            F_OWNER_PID => {
                if let Some(entry) = user_thread_entry_by_process_pid(state.owner_pid) {
                    let _ = deliver_user_signal(&entry, sig, ext.process.pid());
                }
            }
            F_OWNER_PGRP => {
                for entry in user_thread_entries_by_process_group(state.owner_pid) {
                    let _ = deliver_user_signal(&entry, sig, ext.process.pid());
                }
            }
            _ => {}
        }
    }

    fn interrupted() -> bool {
        current_unblocked_signal_pending()
            || current_task_ext().is_some_and(|ext| {
                ext.process.pending_exit_group().is_some() || ext.process.eval_watchdog_expired()
            })
    }

    fn read_with_nonblocking(
        &self,
        dst: &mut [u8],
        nonblocking: bool,
    ) -> Result<usize, LinuxError> {
        if !self.readable {
            return Err(LinuxError::EBADF);
        }
        let mut read_len = 0usize;
        while read_len < dst.len() {
            let mut ring = self.buffer.lock();
            let available = ring.available_read();
            if available == 0 {
                if read_len > 0 || self.read_peer_closed() {
                    return Ok(read_len);
                }
                drop(ring);
                if nonblocking || self.nonblocking() {
                    return Err(LinuxError::EAGAIN);
                }
                if Self::interrupted() {
                    return Err(LinuxError::EINTR);
                }
                Self::sleep_while_blocked();
                continue;
            }
            for _ in 0..available {
                if read_len == dst.len() {
                    return Ok(read_len);
                }
                dst[read_len] = ring.read_byte();
                self.peers.buffered.fetch_sub(1, Ordering::AcqRel);
                read_len += 1;
            }
            if read_len > 0 {
                return Ok(read_len);
            }
        }
        Ok(read_len)
    }

    pub(super) fn read(&self, dst: &mut [u8]) -> Result<usize, LinuxError> {
        self.read_with_nonblocking(dst, false)
    }

    pub(super) fn read_partial(
        &self,
        dst: &mut [u8],
        nonblocking: bool,
    ) -> Result<usize, LinuxError> {
        self.read_with_nonblocking(dst, nonblocking)
    }

    pub(super) fn write(&self, src: &[u8]) -> Result<usize, LinuxError> {
        if !self.writable {
            return Err(LinuxError::EBADF);
        }
        let mut written = 0usize;
        while written < src.len() {
            if self.write_peer_closed() {
                Self::raise_sigpipe();
                return if written > 0 {
                    Ok(written)
                } else {
                    Err(LinuxError::EPIPE)
                };
            }
            let mut ring = self.buffer.lock();
            let available = ring.available_write();
            if available == 0 {
                drop(ring);
                if self.nonblocking() {
                    return if written > 0 {
                        Ok(written)
                    } else {
                        Err(LinuxError::EAGAIN)
                    };
                }
                if Self::interrupted() {
                    return Err(LinuxError::EINTR);
                }
                Self::sleep_while_blocked();
                continue;
            }
            for _ in 0..available {
                if written == src.len() {
                    drop(ring);
                    self.notify_async_readable();
                    return Ok(written);
                }
                ring.write_byte(src[written]);
                self.peers.buffered.fetch_add(1, Ordering::AcqRel);
                written += 1;
            }
        }
        if written > 0 {
            self.notify_async_readable();
        }
        Ok(written)
    }

    pub(super) fn write_partial(&self, src: &[u8], nonblocking: bool) -> Result<usize, LinuxError> {
        if !self.writable {
            return Err(LinuxError::EBADF);
        }
        if src.is_empty() {
            return Ok(0);
        }
        loop {
            if self.write_peer_closed() {
                Self::raise_sigpipe();
                return Err(LinuxError::EPIPE);
            }
            let mut ring = self.buffer.lock();
            let available = ring.available_write();
            if available == 0 {
                drop(ring);
                if nonblocking || self.nonblocking() {
                    return Err(LinuxError::EAGAIN);
                }
                if Self::interrupted() {
                    return Err(LinuxError::EINTR);
                }
                Self::sleep_while_blocked();
                continue;
            }
            let to_write = src.len().min(available);
            for byte in &src[..to_write] {
                ring.write_byte(*byte);
                self.peers.buffered.fetch_add(1, Ordering::AcqRel);
            }
            drop(ring);
            if to_write > 0 {
                self.notify_async_readable();
            }
            return Ok(to_write);
        }
    }

    pub(super) fn stat(&self) -> general::stat {
        let mut st: general::stat = unsafe { core::mem::zeroed() };
        st.st_ino = 1;
        st.st_mode = 0o010000 | 0o600;
        st.st_nlink = 1;
        st.st_blksize = PIPE_BUF_SIZE as _;
        st
    }

    pub(super) fn poll_readable(&self) -> bool {
        self.readable && (self.available_read() > 0 || self.read_peer_closed())
    }

    pub(super) fn poll_writable(&self) -> bool {
        let ring = self.buffer.lock();
        self.writable && (ring.available_write() >= PIPE_BUF_SIZE || self.write_peer_closed())
    }
}

pub(super) fn sys_pipe2(process: &UserProcess, pipefd: usize, flags: usize) -> isize {
    let flags = flags as u32;
    let supported_flags = general::O_CLOEXEC | general::O_NONBLOCK | general::O_DIRECT;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_write(process, pipefd, core::mem::size_of::<[i32; 2]>()) {
        return neg_errno(err);
    }
    let fd_flags = fd_cloexec_flag(flags & general::O_CLOEXEC != 0);
    let status_flags = flags & (general::O_NONBLOCK | general::O_DIRECT);
    let (read_end, write_end) = PipeEndpoint::new_pair_for_process(process, status_flags);
    let fds = {
        let mut table = process.fds.lock();
        let read_fd = match table.insert_with_flags(FdEntry::Pipe(read_end), fd_flags) {
            Ok(fd) => fd,
            Err(err) => return neg_errno(err),
        };
        let write_fd = match table.insert_with_flags(FdEntry::Pipe(write_end), fd_flags) {
            Ok(fd) => fd,
            Err(err) => {
                let _ = table.close(read_fd);
                return neg_errno(err);
            }
        };
        [read_fd, write_fd]
    };
    write_user_value(process, pipefd, &fds)
}
