use axerrno::LinuxError;
use axsync::Mutex;
use linux_raw_sys::general;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

use super::fd_table::FdEntry;
use super::linux_abi::{SIGPIPE_NUM, fd_cloexec_flag};
use super::signal_abi::{current_unblocked_signal_pending, deliver_user_signal};
use super::task_context::{current_task_ext, current_tid};
use super::task_registry::user_thread_entry_by_tid;
use super::user_memory::{validate_user_write, write_user_value};
use super::{UserProcess, neg_errno};

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

pub(super) struct PipeEndpoint {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
    status_flags: Arc<Mutex<u32>>,
    peers: Arc<PipePeerCounts>,
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
            peers: self.peers.clone(),
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
    }
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
    pub(super) fn new_pair(status_flags: u32) -> (Self, Self) {
        Self::new_pair_with_capacity(status_flags, PIPE_DEFAULT_CAPACITY_SIZE)
    }

    pub(super) fn new_pair_for_process(process: &UserProcess, status_flags: u32) -> (Self, Self) {
        let capacity = if process.fs_uid() == 0 {
            PIPE_DEFAULT_CAPACITY_SIZE
        } else {
            PIPE_UNPRIVILEGED_CAPACITY_SIZE
        };
        Self::new_pair_with_capacity(status_flags, capacity)
    }

    pub(super) fn new_fifo_readwrite(status_flags: u32) -> Self {
        let buffer = Arc::new(Mutex::new(PipeRingBuffer::new(PIPE_DEFAULT_CAPACITY_SIZE)));
        let peers = Arc::new(PipePeerCounts {
            readers: AtomicUsize::new(1),
            writers: AtomicUsize::new(1),
            buffered: AtomicUsize::new(0),
        });
        Self {
            readable: true,
            writable: true,
            buffer,
            status_flags: Arc::new(Mutex::new(
                general::O_RDWR | (status_flags & !general::O_ACCMODE),
            )),
            peers,
        }
    }

    fn new_pair_with_capacity(status_flags: u32, capacity: usize) -> (Self, Self) {
        let buffer = Arc::new(Mutex::new(PipeRingBuffer::new(capacity)));
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
                peers: peers.clone(),
            },
            Self {
                readable: false,
                writable: true,
                buffer,
                status_flags: Arc::new(Mutex::new(
                    general::O_WRONLY | (status_flags & !general::O_ACCMODE),
                )),
                peers,
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

    pub(super) fn set_status_flags(&self, flags: u32) {
        let access = self.status_flags() & general::O_ACCMODE;
        *self.status_flags.lock() = access | (flags & (general::O_NONBLOCK | general::O_DIRECT));
    }

    pub(super) fn capacity(&self) -> usize {
        self.buffer.lock().capacity
    }

    pub(super) fn set_capacity(&self, requested: usize) -> Result<usize, LinuxError> {
        let requested = requested.clamp(PIPE_BUF_SIZE, PIPE_MAX_CAPACITY_SIZE);
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

    fn interrupted() -> bool {
        current_unblocked_signal_pending()
            || current_task_ext().is_some_and(|ext| {
                ext.process.pending_exit_group().is_some() || ext.process.eval_watchdog_expired()
            })
    }

    pub(super) fn read(&self, dst: &mut [u8]) -> Result<usize, LinuxError> {
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
                if self.nonblocking() {
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
                    return Ok(written);
                }
                ring.write_byte(src[written]);
                self.peers.buffered.fetch_add(1, Ordering::AcqRel);
                written += 1;
            }
        }
        Ok(written)
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
