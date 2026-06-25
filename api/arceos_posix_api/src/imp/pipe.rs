use alloc::sync::Arc;
use core::{
    ffi::c_int,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use axerrno::{LinuxError, LinuxResult};
use axio::PollState;
use axsync::Mutex;
#[cfg(feature = "multitask")]
use axtask::WaitQueue;

use super::fd_ops::{add_file_like, close_file_like, FileLike};
use crate::ctypes;

#[derive(Copy, Clone, PartialEq)]
enum RingBufferStatus {
    Full,
    Empty,
    Normal,
}

const RING_BUFFER_SIZE: usize = 4096;
const PIPE_STAT_DEV: ctypes::dev_t = 0x7069_7065;
const PIPE_STAT_BLKSIZE: ctypes::blksize_t = 4096;

pub struct PipeRingBuffer {
    arr: [u8; RING_BUFFER_SIZE],
    head: usize,
    tail: usize,
    status: RingBufferStatus,
}

impl PipeRingBuffer {
    pub const fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: RingBufferStatus::Empty,
        }
    }

    // Batch copies must stay equivalent to the former byte-at-a-time ring
    // operations: update head/tail/status under the buffer lock, then let
    // callers drop the lock before notifying waiters.
    pub fn read_slice(&mut self, dst: &mut [u8]) -> usize {
        let count = dst.len().min(self.available_read());
        if count == 0 {
            return 0;
        }
        self.status = RingBufferStatus::Normal;

        let first = count.min(RING_BUFFER_SIZE - self.head);
        dst[..first].copy_from_slice(&self.arr[self.head..self.head + first]);
        self.head = (self.head + first) % RING_BUFFER_SIZE;

        let second = count - first;
        if second > 0 {
            dst[first..first + second].copy_from_slice(&self.arr[..second]);
            self.head = second;
        }

        if self.head == self.tail {
            self.status = RingBufferStatus::Empty;
        }
        count
    }

    pub fn write_slice(&mut self, src: &[u8]) -> usize {
        let count = src.len().min(self.available_write());
        if count == 0 {
            return 0;
        }
        self.status = RingBufferStatus::Normal;

        let first = count.min(RING_BUFFER_SIZE - self.tail);
        self.arr[self.tail..self.tail + first].copy_from_slice(&src[..first]);
        self.tail = (self.tail + first) % RING_BUFFER_SIZE;

        let second = count - first;
        if second > 0 {
            self.arr[..second].copy_from_slice(&src[first..first + second]);
            self.tail = second;
        }

        if self.tail == self.head {
            self.status = RingBufferStatus::Full;
        }
        count
    }

    /// Get the length of remaining data in the buffer
    pub const fn available_read(&self) -> usize {
        if matches!(self.status, RingBufferStatus::Empty) {
            0
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.tail + RING_BUFFER_SIZE - self.head
        }
    }

    /// Get the length of remaining space in the buffer
    pub const fn available_write(&self) -> usize {
        if matches!(self.status, RingBufferStatus::Full) {
            0
        } else {
            RING_BUFFER_SIZE - self.available_read()
        }
    }
}

struct PipePeerCounts {
    readers: AtomicUsize,
    writers: AtomicUsize,
}

pub struct Pipe {
    readable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
    #[cfg(feature = "multitask")]
    read_wait: Arc<WaitQueue>,
    #[cfg(feature = "multitask")]
    write_wait: Arc<WaitQueue>,
    peer_counts: Arc<PipePeerCounts>,
    nonblocking: AtomicBool,
}

impl Pipe {
    pub fn new() -> (Pipe, Pipe) {
        let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
        #[cfg(feature = "multitask")]
        let read_wait = Arc::new(WaitQueue::new());
        #[cfg(feature = "multitask")]
        let write_wait = Arc::new(WaitQueue::new());
        let peer_counts = Arc::new(PipePeerCounts {
            readers: AtomicUsize::new(1),
            writers: AtomicUsize::new(1),
        });
        let read_end = Pipe {
            readable: true,
            buffer: buffer.clone(),
            #[cfg(feature = "multitask")]
            read_wait: read_wait.clone(),
            #[cfg(feature = "multitask")]
            write_wait: write_wait.clone(),
            peer_counts: peer_counts.clone(),
            nonblocking: AtomicBool::new(false),
        };
        let write_end = Pipe {
            readable: false,
            buffer,
            #[cfg(feature = "multitask")]
            read_wait,
            #[cfg(feature = "multitask")]
            write_wait,
            peer_counts,
            nonblocking: AtomicBool::new(false),
        };
        (read_end, write_end)
    }

    pub const fn readable(&self) -> bool {
        self.readable
    }

    pub const fn writable(&self) -> bool {
        !self.readable
    }

    pub fn write_end_close(&self) -> bool {
        self.peer_counts.writers.load(Ordering::Acquire) == 0
    }

    pub fn read_end_close(&self) -> bool {
        self.peer_counts.readers.load(Ordering::Acquire) == 0
    }

    fn notify_read_end_closed(&self) {
        let _ = crate::signal::raise_sigpipe();
    }

    fn notify_readable(&self) {
        // Lock order invariant: wait predicates below may inspect `buffer`
        // while the WaitQueue is evaluating readiness. Notify callers must not
        // hold `buffer.lock()` across these wakeups, or future changes could
        // introduce a waitqueue -> buffer / buffer -> waitqueue cycle.
        #[cfg(feature = "multitask")]
        self.read_wait.notify_all(false);
    }

    fn notify_writable(&self) {
        // Keep this paired with `notify_readable`: mutate pipe state first,
        // drop the ring-buffer mutex, then notify waiters.
        #[cfg(feature = "multitask")]
        self.write_wait.notify_all(false);
    }

    fn wait_for_readable(&self) {
        #[cfg(feature = "multitask")]
        self.read_wait.wait_until(|| {
            self.nonblocking.load(Ordering::Acquire)
                || self.write_end_close()
                || self.buffer.lock().available_read() > 0
        });
        #[cfg(not(feature = "multitask"))]
        crate::sys_sched_yield();
    }

    fn wait_for_writable(&self) {
        #[cfg(feature = "multitask")]
        self.write_wait.wait_until(|| {
            self.nonblocking.load(Ordering::Acquire)
                || self.read_end_close()
                || self.buffer.lock().available_write() > 0
        });
        #[cfg(not(feature = "multitask"))]
        crate::sys_sched_yield();
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        if self.readable {
            self.peer_counts.readers.fetch_sub(1, Ordering::AcqRel);
        } else {
            self.peer_counts.writers.fetch_sub(1, Ordering::AcqRel);
            self.notify_readable();
        }
        if self.readable {
            self.notify_writable();
        }
    }
}

impl FileLike for Pipe {
    fn read(&self, buf: &mut [u8]) -> LinuxResult<usize> {
        if !self.readable() {
            return Err(LinuxError::EPERM);
        }
        if buf.is_empty() {
            return Ok(0);
        }
        let mut read_size = 0usize;
        let max_len = buf.len();
        loop {
            let mut ring_buffer = self.buffer.lock();
            let loop_read = ring_buffer.available_read();
            if loop_read == 0 {
                if self.write_end_close() {
                    return Ok(read_size);
                }
                drop(ring_buffer);
                if self.nonblocking.load(Ordering::Acquire) {
                    return Err(LinuxError::EAGAIN);
                }
                // Data not ready, wait for write end or peer close.
                self.wait_for_readable();
                continue;
            }
            let to_read = (max_len - read_size).min(loop_read);
            let copied = ring_buffer.read_slice(&mut buf[read_size..read_size + to_read]);
            read_size += copied;
            if read_size > 0 {
                drop(ring_buffer);
                self.notify_writable();
                return Ok(read_size);
            }
        }
    }

    fn write(&self, buf: &[u8]) -> LinuxResult<usize> {
        if !self.writable() {
            return Err(LinuxError::EPERM);
        }
        if buf.is_empty() {
            return Ok(0);
        }
        let mut write_size = 0usize;
        let max_len = buf.len();
        loop {
            if self.read_end_close() {
                self.notify_read_end_closed();
                return if write_size == 0 {
                    Err(LinuxError::EPIPE)
                } else {
                    Ok(write_size)
                };
            }
            let mut ring_buffer = self.buffer.lock();
            let loop_write = ring_buffer.available_write();
            if loop_write == 0 {
                drop(ring_buffer);
                if self.read_end_close() {
                    self.notify_read_end_closed();
                    return if write_size == 0 {
                        Err(LinuxError::EPIPE)
                    } else {
                        Ok(write_size)
                    };
                }
                if self.nonblocking.load(Ordering::Acquire) {
                    if write_size > 0 {
                        self.notify_readable();
                    }
                    return if write_size == 0 {
                        Err(LinuxError::EAGAIN)
                    } else {
                        Ok(write_size)
                    };
                }
                // Buffer is full, wait for read end to consume or close.
                self.wait_for_writable();
                continue;
            }
            let to_write = (max_len - write_size).min(loop_write);
            let copied = ring_buffer.write_slice(&buf[write_size..write_size + to_write]);
            write_size += copied;
            if write_size == max_len {
                drop(ring_buffer);
                self.notify_readable();
                return Ok(write_size);
            }
            drop(ring_buffer);
            self.notify_readable();
        }
    }

    fn stat(&self) -> LinuxResult<ctypes::stat> {
        let st_mode = 0o10000 | 0o600u32; // S_IFIFO | rw-------
        let st_ino = (Arc::as_ptr(&self.buffer) as usize as ctypes::ino_t).max(1);
        Ok(ctypes::stat {
            st_dev: PIPE_STAT_DEV,
            st_ino,
            st_nlink: 1,
            st_mode,
            st_uid: 0,
            st_gid: 0,
            st_blksize: PIPE_STAT_BLKSIZE,
            ..Default::default()
        })
    }

    fn into_any(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync> {
        self
    }

    fn poll(&self) -> LinuxResult<PollState> {
        let buf = self.buffer.lock();
        Ok(PollState {
            readable: self.readable() && (buf.available_read() > 0 || self.write_end_close()),
            writable: self.writable() && buf.available_write() > 0,
        })
    }

    fn status_flags(&self) -> LinuxResult<c_int> {
        let mut flags = if self.readable() {
            ctypes::O_RDONLY as c_int
        } else {
            ctypes::O_WRONLY as c_int
        };
        if self.nonblocking.load(Ordering::Acquire) {
            flags |= ctypes::O_NONBLOCK as c_int;
        }
        Ok(flags)
    }

    fn set_nonblocking(&self, nonblocking: bool) -> LinuxResult {
        self.nonblocking.store(nonblocking, Ordering::Release);
        if nonblocking {
            self.notify_readable();
            self.notify_writable();
        }
        Ok(())
    }
}

/// Create a pipe
///
/// Return 0 if succeed
pub fn sys_pipe(fds: &mut [c_int; 2]) -> c_int {
    debug!("sys_pipe <= {:#x}", fds.as_ptr() as usize);
    syscall_body!(sys_pipe, {
        let (read_end, write_end) = Pipe::new();
        let read_fd = add_file_like(Arc::new(read_end))?;
        let write_fd = add_file_like(Arc::new(write_end)).inspect_err(|_| {
            close_file_like(read_fd).ok();
        })?;

        fds[0] = read_fd as c_int;
        fds[1] = write_fd as c_int;

        Ok(0)
    })
}
