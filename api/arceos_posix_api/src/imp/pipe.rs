use alloc::sync::Arc;
use core::{
    ffi::c_int,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use axerrno::{LinuxError, LinuxResult};
use axio::PollState;
use axsync::Mutex;

use super::fd_ops::{add_file_like, close_file_like, FileLike};
use crate::ctypes;

#[derive(Copy, Clone, PartialEq)]
enum RingBufferStatus {
    Full,
    Empty,
    Normal,
}

const RING_BUFFER_SIZE: usize = 256;
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

    pub fn write_byte(&mut self, byte: u8) {
        self.status = RingBufferStatus::Normal;
        self.arr[self.tail] = byte;
        self.tail = (self.tail + 1) % RING_BUFFER_SIZE;
        if self.tail == self.head {
            self.status = RingBufferStatus::Full;
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::Normal;
        let c = self.arr[self.head];
        self.head = (self.head + 1) % RING_BUFFER_SIZE;
        if self.head == self.tail {
            self.status = RingBufferStatus::Empty;
        }
        c
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
    peer_counts: Arc<PipePeerCounts>,
    nonblocking: AtomicBool,
}

impl Pipe {
    pub fn new() -> (Pipe, Pipe) {
        let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
        let peer_counts = Arc::new(PipePeerCounts {
            readers: AtomicUsize::new(1),
            writers: AtomicUsize::new(1),
        });
        let read_end = Pipe {
            readable: true,
            buffer: buffer.clone(),
            peer_counts: peer_counts.clone(),
            nonblocking: AtomicBool::new(false),
        };
        let write_end = Pipe {
            readable: false,
            buffer,
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
}

impl Drop for Pipe {
    fn drop(&mut self) {
        if self.readable {
            self.peer_counts.readers.fetch_sub(1, Ordering::AcqRel);
        } else {
            self.peer_counts.writers.fetch_sub(1, Ordering::AcqRel);
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
                // Data not ready, wait for write end
                crate::sys_sched_yield(); // TODO: use synconize primitive
                continue;
            }
            for _ in 0..loop_read {
                if read_size == max_len {
                    return Ok(read_size);
                }
                buf[read_size] = ring_buffer.read_byte();
                read_size += 1;
            }
            if read_size > 0 {
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
                    return if write_size == 0 {
                        Err(LinuxError::EAGAIN)
                    } else {
                        Ok(write_size)
                    };
                }
                // Buffer is full, wait for read end to consume
                crate::sys_sched_yield(); // TODO: use synconize primitive
                continue;
            }
            for _ in 0..loop_write {
                if write_size == max_len {
                    return Ok(write_size);
                }
                ring_buffer.write_byte(buf[write_size]);
                write_size += 1;
            }
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
