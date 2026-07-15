use core::cmp;
use core::ffi::c_void;
use core::mem::size_of;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;

use arceos_posix_api::ctypes as posix_ctypes;
use axerrno::LinuxError;
use axsync::Mutex;
use axtask::WaitQueue;
use lazyinit::LazyInit;
use linux_raw_sys::general;
use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

use super::fd_pipe::PipeEndpoint;
use super::fd_table::{FdEntry, resolve_dirfd_path};
use super::linux_abi::{
    AF_UNIX_DOMAIN, INTERRUPTIBLE_SOCKET_RECV_QUANTUM, IP_MCAST_JOIN_GROUP_OPT,
    IP_MCAST_LEAVE_GROUP_OPT, IPPROTO_IP_LEVEL, LINUX_EAFNOSUPPORT, LINUX_ENOPROTOOPT,
    LINUX_EOPNOTSUPP, LINUX_EPROTONOSUPPORT, LINUX_ESOCKTNOSUPPORT, LOCAL_SOCKET_INO_BASE,
    SO_ACCEPTCONN_OPT, SO_DOMAIN_OPT, SO_ERROR_OPT, SO_PEERCRED_OPT, SO_PROTOCOL_OPT,
    SO_RCVBUF_OPT, SO_RCVBUFFORCE_OPT, SO_RCVTIMEO_OPT, SO_REUSEADDR_OPT, SO_SNDBUF_OPT,
    SO_SNDBUFFORCE_OPT, SO_SNDTIMEO_OPT, SO_TYPE_OPT, SOL_SOCKET_LEVEL, ST_MODE_SOCKET,
    TCP_INFO_COMPAT_SIZE, TCP_MAXSEG_OPT, TCP_NODELAY_OPT, fd_cloexec_flag, neg_errno_code,
    posix_errno_from_ret,
};
use super::signal_abi::current_unblocked_signal_pending;
use super::time_abi::{socket_duration_to_timeval, socket_timeval_to_duration};
use super::user_memory::{
    MAX_USER_IO_CHUNK, read_iovec_entries, read_user_bytes, read_user_bytes_into, read_user_value,
    user_io_buffer, validate_user_read, validate_user_write, write_user_bytes, write_user_value,
};
use super::{SelectMode, UserProcess, neg_errno, posix_ret_i32, posix_ret_usize};

macro_rules! socket_entry_or_return {
    ($process:expr, $fd:expr) => {
        match socket_entry($process, $fd) {
            Ok(socket) => socket,
            Err(err) => return neg_errno(err),
        }
    };
}

#[derive(Clone)]
pub(super) struct SocketEntry {
    pub(super) posix_fd: i32,
    pub(super) socktype: i32,
    read_shutdown: Arc<Mutex<bool>>,
    multicast_groups: Arc<Mutex<Vec<Vec<u8>>>>,
}

pub(super) struct LocalSocketEntry {
    id: usize,
    socktype: i32,
    nonblocking: bool,
    bound_path: Arc<Mutex<Option<String>>>,
    pair: Option<LocalSocketPairEndpoint>,
    peer_cred: Arc<Mutex<Option<LocalSocketCred>>>,
}

static NEXT_LOCAL_SOCKET_ID: AtomicUsize = AtomicUsize::new(1);
const LOCAL_SOCKET_INITIAL_BUFFER_SIZE: usize = 4096;
// AF_UNIX stream sockets have their own socket buffer, not a single PIPE_BUF
// slot.  Allow the in-kernel local socket buffer to grow enough to carry a full
// default pipe-sized splice without requiring a peer read in the middle of the
// producer's syscall.
const LOCAL_SOCKET_MAX_BUFFER_SIZE: usize = 64 * 1024;
const LOCAL_SOCKET_BLOCK_QUANTUM: Duration = Duration::from_millis(1);
const SOCKET_ADDR_STORAGE_MAX: usize = 128;
const SOCKET_OPTLEN_MAX: usize = TCP_INFO_COMPAT_SIZE;
const MSG_ERRQUEUE_FLAG: i32 = 0x2000;

#[derive(Clone, Copy, Eq, PartialEq)]
enum LocalSocketBufferStatus {
    Full,
    Empty,
    Normal,
}

struct LocalSocketBuffer {
    data: Vec<u8>,
    capacity: usize,
    head: usize,
    tail: usize,
    status: LocalSocketBufferStatus,
}

struct LocalSocketPairState {
    buffers: [Mutex<LocalSocketBuffer>; 2],
    open_ends: Mutex<[usize; 2]>,
    read_wait: [Arc<WaitQueue>; 2],
    write_wait: [Arc<WaitQueue>; 2],
}

struct LocalSocketPairEndpoint {
    side: usize,
    state: Arc<LocalSocketPairState>,
}

#[derive(Clone, Copy)]
struct LocalSocketCred {
    pid: i32,
    uid: u32,
    gid: u32,
}

struct LocalSocketPending {
    endpoint: LocalSocketPairEndpoint,
    peer_cred: LocalSocketCred,
}

struct LocalSocketListener {
    path: String,
    owner_id: usize,
    socktype: i32,
    backlog: usize,
    owner_cred: LocalSocketCred,
    pending: Vec<LocalSocketPending>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct LinuxMsghdr {
    msg_name: *mut c_void,
    msg_namelen: i32,
    msg_iov: *mut general::iovec,
    msg_iovlen: usize,
    msg_control: *mut c_void,
    msg_controllen: usize,
    msg_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct LinuxUcred {
    pid: i32,
    uid: u32,
    gid: u32,
}

impl SocketEntry {
    pub(super) fn new(posix_fd: i32, socktype: i32) -> Self {
        Self {
            posix_fd,
            socktype,
            read_shutdown: Arc::new(Mutex::new(false)),
            multicast_groups: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub(super) fn duplicate(&self) -> Result<Self, LinuxError> {
        let posix_fd = posix_ret_i32(arceos_posix_api::sys_dup(self.posix_fd))?;
        Ok(Self {
            posix_fd,
            socktype: self.socktype,
            read_shutdown: self.read_shutdown.clone(),
            multicast_groups: self.multicast_groups.clone(),
        })
    }

    fn join_multicast_group(&self, group: Vec<u8>) {
        let mut groups = self.multicast_groups.lock();
        if !groups.iter().any(|entry| entry == &group) {
            groups.push(group);
        }
    }

    fn leave_multicast_group(&self, group: &[u8]) -> Result<(), LinuxError> {
        let mut groups = self.multicast_groups.lock();
        let Some(index) = groups.iter().position(|entry| entry.as_slice() == group) else {
            return Err(LinuxError::EADDRNOTAVAIL);
        };
        groups.remove(index);
        Ok(())
    }

    pub(super) fn read(&self, dst: &mut [u8]) -> Result<usize, LinuxError> {
        posix_ret_usize(unsafe {
            arceos_posix_api::sys_recv(self.posix_fd, dst.as_mut_ptr() as *mut c_void, dst.len(), 0)
        })
    }

    pub(super) fn write(&self, src: &[u8]) -> Result<usize, LinuxError> {
        posix_ret_usize(unsafe {
            arceos_posix_api::sys_send(self.posix_fd, src.as_ptr() as *const c_void, src.len(), 0)
        })
    }

    pub(super) fn close(&self) -> Result<(), LinuxError> {
        posix_ret_i32(arceos_posix_api::sys_close(self.posix_fd)).map(|_| ())
    }

    pub(super) fn poll(&self, mode: SelectMode) -> bool {
        match arceos_posix_api::poll_file_like(self.posix_fd) {
            Ok(state) => match mode {
                SelectMode::Read => state.readable,
                SelectMode::Write => state.writable,
                SelectMode::Except => false,
            },
            Err(_) => matches!(mode, SelectMode::Except),
        }
    }

    pub(super) fn mark_shutdown(&self, how: i32) {
        if matches!(how, 0 | 2) {
            *self.read_shutdown.lock() = true;
        }
    }

    pub(super) fn poll_rdhup(&self) -> bool {
        *self.read_shutdown.lock()
    }

    pub(super) fn stat(&self) -> general::stat {
        let mut st: general::stat = unsafe { core::mem::zeroed() };
        st.st_ino = self.posix_fd as _;
        st.st_mode = ST_MODE_SOCKET | 0o666;
        st.st_nlink = 1;
        st.st_blksize = 512;
        st
    }
}

impl LocalSocketBuffer {
    fn new() -> Self {
        Self {
            data: vec![0; LOCAL_SOCKET_INITIAL_BUFFER_SIZE],
            capacity: LOCAL_SOCKET_INITIAL_BUFFER_SIZE,
            head: 0,
            tail: 0,
            status: LocalSocketBufferStatus::Empty,
        }
    }

    fn grow_for_write(&mut self, desired_write: usize) {
        if desired_write == 0 || self.capacity >= LOCAL_SOCKET_MAX_BUFFER_SIZE {
            return;
        }
        let available_read = self.available_read();
        let available_write = self.available_write();
        if available_write >= desired_write {
            return;
        }
        let needed = available_read.saturating_add(desired_write);
        let new_capacity = needed
            .next_power_of_two()
            .min(LOCAL_SOCKET_MAX_BUFFER_SIZE)
            .max(self.capacity);
        if new_capacity <= self.capacity {
            return;
        }
        let mut new_data = vec![0; new_capacity];
        if available_read != 0 {
            let first = available_read.min(self.capacity - self.head);
            new_data[..first].copy_from_slice(&self.data[self.head..self.head + first]);
            let second = available_read - first;
            if second != 0 {
                new_data[first..first + second].copy_from_slice(&self.data[..second]);
            }
        }
        self.data = new_data;
        self.capacity = new_capacity;
        self.head = 0;
        self.tail = available_read % self.capacity;
        self.status = if available_read == 0 {
            LocalSocketBufferStatus::Empty
        } else if available_read == self.capacity {
            LocalSocketBufferStatus::Full
        } else {
            LocalSocketBufferStatus::Normal
        };
    }

    fn read_into(&mut self, dst: &mut [u8]) -> usize {
        let take = self.available_read().min(dst.len());
        if take == 0 {
            return 0;
        }
        self.status = LocalSocketBufferStatus::Normal;
        let first = take.min(self.capacity - self.head);
        dst[..first].copy_from_slice(&self.data[self.head..self.head + first]);
        let second = take - first;
        if second != 0 {
            dst[first..first + second].copy_from_slice(&self.data[..second]);
        }
        self.head = (self.head + take) % self.capacity;
        if self.head == self.tail {
            self.status = LocalSocketBufferStatus::Empty;
        }
        take
    }

    fn write_from(&mut self, src: &[u8]) -> usize {
        let take = self.available_write().min(src.len());
        if take == 0 {
            return 0;
        }
        self.status = LocalSocketBufferStatus::Normal;
        let first = take.min(self.capacity - self.tail);
        self.data[self.tail..self.tail + first].copy_from_slice(&src[..first]);
        let second = take - first;
        if second != 0 {
            self.data[..second].copy_from_slice(&src[first..first + second]);
        }
        self.tail = (self.tail + take) % self.capacity;
        if self.tail == self.head {
            self.status = LocalSocketBufferStatus::Full;
        }
        take
    }

    fn available_read(&self) -> usize {
        if matches!(self.status, LocalSocketBufferStatus::Empty) {
            0
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.tail + self.capacity - self.head
        }
    }

    fn available_write(&self) -> usize {
        if matches!(self.status, LocalSocketBufferStatus::Full) {
            0
        } else {
            self.capacity - self.available_read()
        }
    }

    fn available_write_after_growth(&self) -> usize {
        LOCAL_SOCKET_MAX_BUFFER_SIZE.saturating_sub(self.available_read())
    }
}

impl LocalSocketPairState {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            buffers: [
                Mutex::new(LocalSocketBuffer::new()),
                Mutex::new(LocalSocketBuffer::new()),
            ],
            open_ends: Mutex::new([1, 1]),
            read_wait: [Arc::new(WaitQueue::new()), Arc::new(WaitQueue::new())],
            write_wait: [Arc::new(WaitQueue::new()), Arc::new(WaitQueue::new())],
        })
    }

    fn duplicate_side(&self, side: usize) {
        self.open_ends.lock()[side] += 1;
    }

    fn close_side(&self, side: usize) {
        let became_closed = {
            let mut open = self.open_ends.lock();
            if open[side] > 0 {
                open[side] -= 1;
            }
            open[side] == 0
        };
        if became_closed {
            self.notify_all_readable(side);
            self.notify_all_writable(side);
            self.notify_all_readable(1 - side);
            self.notify_all_writable(1 - side);
        }
    }

    fn peer_open(&self, side: usize) -> bool {
        self.open_ends.lock()[1 - side] > 0
    }

    fn notify_readable(&self, side: usize) {
        self.read_wait[side].notify_one(true);
    }

    fn notify_writable(&self, side: usize) {
        self.write_wait[side].notify_one(true);
    }

    fn notify_all_readable(&self, side: usize) {
        self.read_wait[side].notify_all(true);
    }

    fn notify_all_writable(&self, side: usize) {
        self.write_wait[side].notify_all(true);
    }

    fn readable_or_peer_closed_or_interrupted(&self, side: usize, process: &UserProcess) -> bool {
        let readable = self.buffers[side].lock().available_read() > 0;
        readable || !self.peer_open(side) || socket_block_interrupt_pending(process)
    }

    fn writable_or_peer_closed_or_interrupted(&self, side: usize, process: &UserProcess) -> bool {
        let peer_side = 1 - side;
        let writable = self.buffers[peer_side]
            .lock()
            .available_write_after_growth()
            > 0;
        writable || !self.peer_open(side) || socket_block_interrupt_pending(process)
    }

    fn wait_readable_or_peer_closed_or_interrupted(&self, side: usize, process: &UserProcess) {
        let wait = self.read_wait[side].clone();
        wait.wait_timeout_until(LOCAL_SOCKET_BLOCK_QUANTUM, || {
            self.readable_or_peer_closed_or_interrupted(side, process)
        });
    }

    fn wait_writable_or_peer_closed_or_interrupted(&self, side: usize, process: &UserProcess) {
        let wait = self.write_wait[side].clone();
        wait.wait_timeout_until(LOCAL_SOCKET_BLOCK_QUANTUM, || {
            self.writable_or_peer_closed_or_interrupted(side, process)
        });
    }
}

impl Clone for LocalSocketPairEndpoint {
    fn clone(&self) -> Self {
        self.state.duplicate_side(self.side);
        Self {
            side: self.side,
            state: self.state.clone(),
        }
    }
}

impl Drop for LocalSocketPairEndpoint {
    fn drop(&mut self) {
        self.state.close_side(self.side);
    }
}

fn local_socket_listeners() -> &'static Mutex<Vec<LocalSocketListener>> {
    static LISTENERS: LazyInit<Mutex<Vec<LocalSocketListener>>> = LazyInit::new();
    let _ = LISTENERS.call_once(|| Mutex::new(Vec::new()));
    &LISTENERS
}

fn local_socket_cred(process: &UserProcess) -> LocalSocketCred {
    LocalSocketCred {
        pid: process.pid(),
        uid: process.uid(),
        gid: process.gid(),
    }
}

fn remove_local_socket_listener(owner_id: usize, path: &str) {
    let mut listeners = local_socket_listeners().lock();
    listeners.retain(|listener| !(listener.owner_id == owner_id && listener.path == path));
}

fn local_socket_listener_pending(owner_id: usize) -> bool {
    local_socket_listeners()
        .lock()
        .iter()
        .any(|listener| listener.owner_id == owner_id && !listener.pending.is_empty())
}

impl Clone for LocalSocketEntry {
    fn clone(&self) -> Self {
        self.duplicate()
    }
}

impl Drop for LocalSocketEntry {
    fn drop(&mut self) {
        if Arc::strong_count(&self.bound_path) != 1 {
            return;
        }
        let Some(path) = self.bound_path.lock().clone() else {
            return;
        };
        remove_local_socket_listener(self.id, path.as_str());
    }
}

impl LocalSocketEntry {
    pub(super) fn new(socktype: i32, flags: i32) -> Self {
        Self {
            id: NEXT_LOCAL_SOCKET_ID.fetch_add(1, Ordering::Relaxed),
            socktype,
            nonblocking: flags & posix_ctypes::SOCK_NONBLOCK as i32 != 0,
            bound_path: Arc::new(Mutex::new(None)),
            pair: None,
            peer_cred: Arc::new(Mutex::new(None)),
        }
    }

    pub(super) fn new_pair(socktype: i32, flags: i32) -> (Self, Self) {
        let state = LocalSocketPairState::new();
        let nonblocking = flags & posix_ctypes::SOCK_NONBLOCK as i32 != 0;
        let first_id = NEXT_LOCAL_SOCKET_ID.fetch_add(2, Ordering::Relaxed);
        (
            Self {
                id: first_id,
                socktype,
                nonblocking,
                bound_path: Arc::new(Mutex::new(None)),
                pair: Some(LocalSocketPairEndpoint {
                    side: 0,
                    state: state.clone(),
                }),
                peer_cred: Arc::new(Mutex::new(None)),
            },
            Self {
                id: first_id + 1,
                socktype,
                nonblocking,
                bound_path: Arc::new(Mutex::new(None)),
                pair: Some(LocalSocketPairEndpoint { side: 1, state }),
                peer_cred: Arc::new(Mutex::new(None)),
            },
        )
    }

    fn new_connected(
        socktype: i32,
        flags: i32,
        pair: LocalSocketPairEndpoint,
        peer_cred: LocalSocketCred,
    ) -> Self {
        Self {
            id: NEXT_LOCAL_SOCKET_ID.fetch_add(1, Ordering::Relaxed),
            socktype,
            nonblocking: flags & posix_ctypes::SOCK_NONBLOCK as i32 != 0,
            bound_path: Arc::new(Mutex::new(None)),
            pair: Some(pair),
            peer_cred: Arc::new(Mutex::new(Some(peer_cred))),
        }
    }

    pub(super) fn duplicate(&self) -> Self {
        Self {
            id: self.id,
            socktype: self.socktype,
            nonblocking: self.nonblocking,
            bound_path: self.bound_path.clone(),
            pair: self.pair.clone(),
            peer_cred: self.peer_cred.clone(),
        }
    }

    fn set_connected(&mut self, pair: LocalSocketPairEndpoint, peer_cred: LocalSocketCred) {
        self.pair = Some(pair);
        *self.peer_cred.lock() = Some(peer_cred);
    }

    pub(super) fn read(&self, process: &UserProcess, dst: &mut [u8]) -> Result<usize, LinuxError> {
        let Some(pair) = &self.pair else {
            return Err(LinuxError::EINVAL);
        };
        let mut read_len = 0usize;
        while read_len < dst.len() {
            let mut buffer = pair.state.buffers[pair.side].lock();
            let available = buffer.available_read();
            if available == 0 {
                if read_len > 0 || !pair.state.peer_open(pair.side) {
                    return Ok(read_len);
                }
                if self.nonblocking {
                    return Err(LinuxError::EAGAIN);
                }
                if socket_block_interrupt_pending(process) {
                    return Err(LinuxError::EINTR);
                }
                drop(buffer);
                pair.state
                    .wait_readable_or_peer_closed_or_interrupted(pair.side, process);
                continue;
            }
            let take = cmp::min(available, dst.len() - read_len);
            let take = buffer.read_into(&mut dst[read_len..read_len + take]);
            read_len += take;
            if read_len > 0 {
                drop(buffer);
                pair.state.notify_writable(1 - pair.side);
                return Ok(read_len);
            }
        }
        Ok(read_len)
    }

    pub(super) fn read_partial(&self, dst: &mut [u8]) -> Result<usize, LinuxError> {
        if dst.is_empty() {
            return Ok(0);
        }
        let Some(pair) = &self.pair else {
            return Err(LinuxError::EINVAL);
        };
        let mut buffer = pair.state.buffers[pair.side].lock();
        let available = buffer.available_read();
        if available == 0 {
            return if pair.state.peer_open(pair.side) {
                Err(LinuxError::EAGAIN)
            } else {
                Ok(0)
            };
        }
        let take = cmp::min(available, dst.len());
        let take = buffer.read_into(&mut dst[..take]);
        drop(buffer);
        pair.state.notify_writable(1 - pair.side);
        Ok(take)
    }

    pub(super) fn available_read_and_peer_open(&self) -> Result<(usize, bool), LinuxError> {
        let Some(pair) = &self.pair else {
            return Err(LinuxError::EINVAL);
        };
        let available = pair.state.buffers[pair.side].lock().available_read();
        Ok((available, pair.state.peer_open(pair.side)))
    }

    pub(super) fn write(&self, process: &UserProcess, src: &[u8]) -> Result<usize, LinuxError> {
        let Some(pair) = &self.pair else {
            return Err(LinuxError::EINVAL);
        };
        if !pair.state.peer_open(pair.side) {
            return Err(LinuxError::EPIPE);
        }
        let peer_side = 1 - pair.side;
        let mut written = 0usize;
        while written < src.len() {
            if !pair.state.peer_open(pair.side) {
                return if written > 0 {
                    Ok(written)
                } else {
                    Err(LinuxError::EPIPE)
                };
            }
            let mut buffer = pair.state.buffers[peer_side].lock();
            buffer.grow_for_write(src.len() - written);
            let available = buffer.available_write();
            if available == 0 {
                if self.nonblocking {
                    return if written > 0 {
                        Ok(written)
                    } else {
                        Err(LinuxError::EAGAIN)
                    };
                }
                if socket_block_interrupt_pending(process) {
                    return if written > 0 {
                        Ok(written)
                    } else {
                        Err(LinuxError::EINTR)
                    };
                }
                drop(buffer);
                pair.state
                    .wait_writable_or_peer_closed_or_interrupted(pair.side, process);
                continue;
            }
            let take = cmp::min(available, src.len() - written);
            let take = buffer.write_from(&src[written..written + take]);
            written += take;
            drop(buffer);
            pair.state.notify_readable(peer_side);
        }
        Ok(written)
    }

    pub(super) fn write_partial(&self, src: &[u8]) -> Result<usize, LinuxError> {
        if src.is_empty() {
            return Ok(0);
        }
        let Some(pair) = &self.pair else {
            return Err(LinuxError::EINVAL);
        };
        if !pair.state.peer_open(pair.side) {
            return Err(LinuxError::EPIPE);
        }
        let peer_side = 1 - pair.side;
        let mut buffer = pair.state.buffers[peer_side].lock();
        buffer.grow_for_write(src.len());
        let available = buffer.available_write();
        if available == 0 {
            return Err(LinuxError::EAGAIN);
        }
        let take = cmp::min(available, src.len());
        let take = buffer.write_from(&src[..take]);
        drop(buffer);
        pair.state.notify_readable(peer_side);
        Ok(take)
    }

    pub(super) fn write_from_pipe_splice_reservation(
        &self,
        process: &UserProcess,
        pipe: &PipeEndpoint,
        pipe_status_flags: u32,
        requested: usize,
        nonblocking: bool,
    ) -> Result<usize, LinuxError> {
        if requested == 0 {
            return Ok(0);
        }
        let Some(pair) = &self.pair else {
            return Err(LinuxError::EINVAL);
        };
        if !pair.state.peer_open(pair.side) {
            return Err(LinuxError::EPIPE);
        }
        let peer_side = 1 - pair.side;
        loop {
            if !pipe.poll_readable() {
                if nonblocking || pipe_status_flags & general::O_NONBLOCK != 0 {
                    return Err(LinuxError::EAGAIN);
                }
                if socket_block_interrupt_pending(process) {
                    return Err(LinuxError::EINTR);
                }
                pipe.wait_for_readable()?;
                continue;
            }
            let pipe_available = pipe.available_read();
            if pipe_available == 0 {
                return Ok(0);
            }
            let requested = cmp::min(requested, pipe_available);
            if !pair.state.peer_open(pair.side) {
                return Err(LinuxError::EPIPE);
            }
            let mut buffer = pair.state.buffers[peer_side].lock();
            buffer.grow_for_write(requested);
            let available = buffer.available_write();
            if available == 0 {
                drop(buffer);
                if nonblocking || self.nonblocking {
                    return Err(LinuxError::EAGAIN);
                }
                if socket_block_interrupt_pending(process) {
                    return Err(LinuxError::EINTR);
                }
                pair.state
                    .wait_writable_or_peer_closed_or_interrupted(pair.side, process);
                continue;
            }

            let take = cmp::min(available, requested);
            let mut staging = vec![0; take];
            let read = match pipe.read_partial(pipe_status_flags, &mut staging, true) {
                Ok(read) => read,
                Err(LinuxError::EAGAIN)
                    if !nonblocking && pipe_status_flags & general::O_NONBLOCK == 0 =>
                {
                    drop(buffer);
                    if socket_block_interrupt_pending(process) {
                        return Err(LinuxError::EINTR);
                    }
                    pipe.wait_for_readable()?;
                    continue;
                }
                Err(err) => return Err(err),
            };
            if read == 0 {
                return Ok(0);
            }
            let written = buffer.write_from(&staging[..read]);
            debug_assert_eq!(written, read);
            drop(buffer);
            pair.state.notify_readable(peer_side);
            return Ok(read);
        }
    }

    pub(super) fn available_write_after_growth(&self) -> Result<usize, LinuxError> {
        let Some(pair) = &self.pair else {
            return Err(LinuxError::EINVAL);
        };
        if !pair.state.peer_open(pair.side) {
            return Err(LinuxError::EPIPE);
        }
        let peer_side = 1 - pair.side;
        let buffer = pair.state.buffers[peer_side].lock();
        Ok(buffer.available_write_after_growth())
    }

    pub(super) fn poll(&self, mode: SelectMode) -> bool {
        let Some(pair) = &self.pair else {
            return match mode {
                SelectMode::Read => local_socket_listener_pending(self.id),
                SelectMode::Write => true,
                SelectMode::Except => false,
            };
        };
        match mode {
            SelectMode::Read => {
                let buffer = pair.state.buffers[pair.side].lock();
                buffer.available_read() > 0 || !pair.state.peer_open(pair.side)
            }
            SelectMode::Write => {
                let peer_side = 1 - pair.side;
                let buffer = pair.state.buffers[peer_side].lock();
                buffer.available_write_after_growth() > 0 && pair.state.peer_open(pair.side)
            }
            SelectMode::Except => false,
        }
    }

    pub(super) fn status_flags(&self) -> i32 {
        let mut flags = self.socktype;
        if self.nonblocking {
            flags |= posix_ctypes::SOCK_NONBLOCK as i32;
        }
        flags
    }

    pub(super) fn set_status_flags(&mut self, flags: i32) {
        self.nonblocking = flags & posix_ctypes::O_NONBLOCK as i32 != 0;
    }

    pub(super) fn stat(&self) -> general::stat {
        let mut st: general::stat = unsafe { core::mem::zeroed() };
        st.st_ino = LOCAL_SOCKET_INO_BASE + self.id as u64;
        st.st_mode = ST_MODE_SOCKET | 0o666;
        st.st_nlink = 1;
        st.st_blksize = 512;
        st
    }
}

fn getsockopt_unsupported_errno_code(socket: &SocketEntry, level: i32) -> u32 {
    if level == posix_ctypes::IPPROTO_UDP as i32
        && socket.socktype as u32 != posix_ctypes::SOCK_DGRAM
    {
        LINUX_EOPNOTSUPP
    } else if level == SOL_SOCKET_LEVEL
        || level == IPPROTO_IP_LEVEL
        || level == posix_ctypes::IPPROTO_TCP as i32
        || level == posix_ctypes::IPPROTO_UDP as i32
    {
        LINUX_ENOPROTOOPT
    } else {
        LINUX_EOPNOTSUPP
    }
}

fn setsockopt_unsupported_errno_code(_level: i32) -> u32 {
    LINUX_ENOPROTOOPT
}

fn socket_protocol(socket: &SocketEntry) -> i32 {
    if socket.socktype as u32 == posix_ctypes::SOCK_DGRAM {
        posix_ctypes::IPPROTO_UDP as i32
    } else {
        posix_ctypes::IPPROTO_TCP as i32
    }
}

fn socket_level_supported(socket: &SocketEntry, level: i32) -> bool {
    level == SOL_SOCKET_LEVEL
        || level == IPPROTO_IP_LEVEL
        || (level == posix_ctypes::IPPROTO_TCP as i32
            && socket.socktype as u32 == posix_ctypes::SOCK_STREAM)
        || (level == posix_ctypes::IPPROTO_UDP as i32
            && socket.socktype as u32 == posix_ctypes::SOCK_DGRAM)
}

fn socket_recv_error_queue_empty(flags: i32) -> bool {
    flags & MSG_ERRQUEUE_FLAG != 0
}

// Deliberately expose only options backed by kernel/socket state. Mutable
// socket-option emulation used to cache arbitrary scalars and made unsupported
// options look successful; new options must wire into a real backend before
// they are accepted by setsockopt/getsockopt.
fn socket_readonly_scalar(socket: &SocketEntry, level: i32, optname: i32) -> Option<i32> {
    if !socket_level_supported(socket, level) || level != SOL_SOCKET_LEVEL {
        return None;
    }
    match optname {
        SO_ERROR_OPT => Some(0),
        SO_TYPE_OPT => Some(socket.socktype),
        SO_DOMAIN_OPT => Some(posix_ctypes::AF_INET as i32),
        SO_PROTOCOL_OPT => Some(socket_protocol(socket)),
        SO_ACCEPTCONN_OPT => Some(0),
        _ => None,
    }
}

fn read_socket_bool_option(process: &UserProcess, optval: usize) -> Result<bool, LinuxError> {
    read_user_value::<i32>(process, optval).map(|value| value != 0)
}

pub(super) fn socket_entry(process: &UserProcess, fd: usize) -> Result<SocketEntry, LinuxError> {
    let table = process.fds.lock();
    match table.entry(fd as i32)? {
        FdEntry::Socket(socket) => Ok(socket.clone()),
        FdEntry::Path(_) => Err(LinuxError::EBADF),
        _ => Err(LinuxError::ENOTSOCK),
    }
}

pub(super) fn insert_socket_entry(
    process: &UserProcess,
    posix_fd: i32,
    socktype: i32,
    flags: i32,
) -> isize {
    if flags & posix_ctypes::SOCK_NONBLOCK as i32 != 0 {
        let ret = arceos_posix_api::sys_fcntl(
            posix_fd,
            posix_ctypes::F_SETFL as i32,
            posix_ctypes::O_NONBLOCK as usize,
        );
        if ret < 0 {
            let _ = arceos_posix_api::sys_close(posix_fd);
            return neg_errno(posix_errno_from_ret(ret as isize));
        }
    }
    match process.fds.lock().insert_with_flags(
        FdEntry::Socket(SocketEntry::new(posix_fd, socktype)),
        fd_cloexec_flag(flags & posix_ctypes::SOCK_CLOEXEC as i32 != 0),
    ) {
        Ok(fd) => fd as isize,
        // FdTable owns the entry after this call starts and rolls back the raw
        // POSIX descriptor itself when no slot can be installed.
        Err(err) => neg_errno(err),
    }
}

pub(super) fn insert_local_socket_entry(process: &UserProcess, socktype: i32, flags: i32) -> isize {
    match process.fds.lock().insert_with_flags(
        FdEntry::LocalSocket(LocalSocketEntry::new(socktype, flags)),
        fd_cloexec_flag(flags & posix_ctypes::SOCK_CLOEXEC as i32 != 0),
    ) {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn insert_local_socket_pair_entries(
    process: &UserProcess,
    socktype: i32,
    flags: i32,
    sv: usize,
) -> isize {
    let fd_flags = fd_cloexec_flag(flags & posix_ctypes::SOCK_CLOEXEC as i32 != 0);
    let (first, second) = LocalSocketEntry::new_pair(socktype, flags);
    let fds = {
        let mut table = process.fds.lock();
        let first_fd = match table.insert_with_flags(FdEntry::LocalSocket(first), fd_flags) {
            Ok(fd) => fd,
            Err(err) => return neg_errno(err),
        };
        let second_fd = match table.insert_with_flags(FdEntry::LocalSocket(second), fd_flags) {
            Ok(fd) => fd,
            Err(err) => {
                let _ = table.close(first_fd);
                return neg_errno(err);
            }
        };
        [first_fd, second_fd]
    };
    let ret = write_user_value(process, sv, &fds);
    if ret != 0 {
        let mut table = process.fds.lock();
        let _ = table.close(fds[0]);
        let _ = table.close(fds[1]);
    }
    ret
}

pub(super) fn is_local_socket_fd(process: &UserProcess, fd: usize) -> Result<bool, LinuxError> {
    let table = process.fds.lock();
    Ok(matches!(table.entry(fd as i32)?, FdEntry::LocalSocket(_)))
}

pub(super) fn socket_addr_call<F>(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
    call: F,
) -> isize
where
    F: FnOnce(i32, *const posix_ctypes::sockaddr, posix_ctypes::socklen_t) -> i32,
{
    let socket = match socket_entry(process, fd) {
        Ok(socket) => socket,
        Err(err) => return neg_errno(err),
    };
    let addr_bytes = match read_socket_addr_from_user(process, addr, addrlen) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    match posix_ret_i32(call(
        socket.posix_fd,
        addr_bytes.as_ptr() as *const posix_ctypes::sockaddr,
        addrlen as posix_ctypes::socklen_t,
    )) {
        Ok(_) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn socket_name_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
    op: unsafe fn(i32, *mut posix_ctypes::sockaddr, *mut posix_ctypes::socklen_t) -> i32,
) -> isize {
    let socket = match socket_entry(process, fd) {
        Ok(socket) => socket,
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = validate_user_write(process, addrlen, size_of::<posix_ctypes::socklen_t>()) {
        return neg_errno(err);
    }
    let len = match read_user_value::<posix_ctypes::socklen_t>(process, addrlen) {
        Ok(len) => len as usize,
        Err(err) => return neg_errno(err),
    };
    if addr == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    if len > SOCKET_ADDR_STORAGE_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_write(process, addr, len) {
        return neg_errno(err);
    }
    let mut local_addr: posix_ctypes::sockaddr = unsafe { core::mem::zeroed() };
    let mut local_len = len as posix_ctypes::socklen_t;
    match posix_ret_i32(unsafe { op(socket.posix_fd, &mut local_addr, &mut local_len) }) {
        Ok(_) => write_socket_addr_to_user(process, addr, addrlen, len, &local_addr, local_len),
        Err(err) => neg_errno(err),
    }
}

pub(super) fn local_socket_name_bridge(
    process: &UserProcess,
    addr: usize,
    addrlen: usize,
) -> isize {
    if let Err(err) = validate_user_write(process, addrlen, size_of::<posix_ctypes::socklen_t>()) {
        return neg_errno(err);
    }
    let len = match read_user_value::<posix_ctypes::socklen_t>(process, addrlen) {
        Ok(len) => len as usize,
        Err(err) => return neg_errno(err),
    };
    if addr == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    if len > SOCKET_ADDR_STORAGE_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_write(process, addr, len) {
        return neg_errno(err);
    }
    let mut local_addr: posix_ctypes::sockaddr = unsafe { core::mem::zeroed() };
    local_addr.sa_family = AF_UNIX_DOMAIN as posix_ctypes::sa_family_t;
    write_socket_addr_to_user(
        process,
        addr,
        addrlen,
        len,
        &local_addr,
        size_of::<posix_ctypes::sa_family_t>() as posix_ctypes::socklen_t,
    )
}

pub(super) fn read_socket_data_from_user(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<Vec<u8>, LinuxError> {
    if ptr == 0 {
        return Err(LinuxError::EFAULT);
    }
    read_user_bytes(process, ptr, len.min(MAX_USER_IO_CHUNK))
}

pub(super) fn read_socket_addr_from_user(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<Vec<u8>, LinuxError> {
    if ptr == 0 {
        return Err(LinuxError::EFAULT);
    }
    if len < size_of::<posix_ctypes::sockaddr>() || len > SOCKET_ADDR_STORAGE_MAX {
        return Err(LinuxError::EINVAL);
    }
    validate_user_read(process, ptr, len)?;
    read_user_bytes(process, ptr, size_of::<posix_ctypes::sockaddr>())
}

fn sockaddr_bytes(addr: &posix_ctypes::sockaddr) -> [u8; size_of::<posix_ctypes::sockaddr>()] {
    let mut bytes = [0u8; size_of::<posix_ctypes::sockaddr>()];
    let family_len = size_of::<posix_ctypes::sa_family_t>();
    bytes[..family_len].copy_from_slice(&addr.sa_family.to_ne_bytes());
    for (dst, src) in bytes[family_len..]
        .iter_mut()
        .zip(addr.sa_data.iter().copied())
    {
        *dst = src as u8;
    }
    bytes
}

pub(super) fn write_socket_addr_to_user(
    process: &UserProcess,
    addr: usize,
    addrlen: usize,
    user_len: usize,
    local_addr: &posix_ctypes::sockaddr,
    local_len: posix_ctypes::socklen_t,
) -> isize {
    let copy_len = core::cmp::min(user_len, size_of::<posix_ctypes::sockaddr>());
    if copy_len > 0 {
        let local_addr_bytes = sockaddr_bytes(local_addr);
        if let Err(err) = write_user_bytes(process, addr, &local_addr_bytes[..copy_len]) {
            return neg_errno(err);
        }
    }
    write_user_value(process, addrlen, &local_len)
}

pub(super) fn recv_socket_data_to_user(
    process: &UserProcess,
    posix_fd: i32,
    buf: usize,
    len: usize,
    flags: i32,
) -> isize {
    if socket_recv_error_queue_empty(flags) {
        return neg_errno(LinuxError::EAGAIN);
    }
    let len = len.min(MAX_USER_IO_CHUNK);
    recv_socket_data_to_user_inner(process, posix_fd, buf, len, |dst| unsafe {
        arceos_posix_api::sys_recv(posix_fd, dst, len, flags)
    })
}

pub(super) fn recv_socket_data_to_user_with_addr(
    process: &UserProcess,
    posix_fd: i32,
    buf: usize,
    len: usize,
    flags: i32,
    addr: usize,
    addrlen: usize,
    user_addr_len: usize,
) -> isize {
    if socket_recv_error_queue_empty(flags) {
        return neg_errno(LinuxError::EAGAIN);
    }
    let len = len.min(MAX_USER_IO_CHUNK);
    let mut local_addr: posix_ctypes::sockaddr = unsafe { core::mem::zeroed() };
    let mut local_len = core::cmp::min(user_addr_len, size_of::<posix_ctypes::sockaddr>())
        as posix_ctypes::socklen_t;
    let ret = recv_socket_data_to_user_inner(process, posix_fd, buf, len, |dst| unsafe {
        arceos_posix_api::sys_recvfrom(posix_fd, dst, len, flags, &mut local_addr, &mut local_len)
    });
    if ret > 0 && local_len != 0 {
        let addr_ret = write_socket_addr_to_user(
            process,
            addr,
            addrlen,
            user_addr_len,
            &local_addr,
            local_len,
        );
        if addr_ret < 0 {
            return addr_ret;
        }
    }
    ret
}

fn capped_iovec_write_len(entries: &[general::iovec]) -> usize {
    let mut total = 0usize;
    for entry in entries {
        if total == MAX_USER_IO_CHUNK {
            break;
        }
        let len = entry.iov_len as usize;
        total = total.saturating_add(len.min(MAX_USER_IO_CHUNK - total));
    }
    total
}

fn validate_iovec_write(
    process: &UserProcess,
    entries: &[general::iovec],
    limit: usize,
) -> Result<(), LinuxError> {
    let mut remaining = limit;
    for entry in entries {
        if remaining == 0 {
            break;
        }
        let len = (entry.iov_len as usize).min(remaining);
        validate_user_write(process, entry.iov_base as usize, len)?;
        remaining -= len;
    }
    Ok(())
}

fn scatter_iovec_bytes_to_user(
    process: &UserProcess,
    entries: &[general::iovec],
    bytes: &[u8],
) -> Result<(), LinuxError> {
    let mut copied = 0usize;
    for entry in entries {
        if copied == bytes.len() {
            break;
        }
        let len = (entry.iov_len as usize).min(bytes.len() - copied);
        if len != 0 {
            let base = entry.iov_base as usize;
            write_user_bytes(process, base, &bytes[copied..copied + len])?;
        }
        copied += len;
    }
    Ok(())
}

fn recv_socket_data_to_buffer(
    process: &UserProcess,
    posix_fd: i32,
    bytes: &mut [u8],
    mut recv_once: impl FnMut(*mut c_void) -> isize,
) -> isize {
    if bytes.is_empty() {
        return 0;
    }
    let ret = recv_with_real_timer_interrupt(process, posix_fd, || {
        recv_once(bytes.as_mut_ptr() as *mut c_void)
    });
    if ret <= 0 {
        return ret;
    }
    if ret as usize > bytes.len() {
        return neg_errno(LinuxError::EINVAL);
    }
    ret
}

fn recv_socket_data_to_user_inner(
    process: &UserProcess,
    posix_fd: i32,
    buf: usize,
    len: usize,
    recv_once: impl FnMut(*mut c_void) -> isize,
) -> isize {
    if buf == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    if let Err(err) = validate_user_write(process, buf, len) {
        return neg_errno(err);
    }
    let mut bytes = match user_io_buffer(len) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    let ret = recv_socket_data_to_buffer(process, posix_fd, &mut bytes, recv_once);
    if ret <= 0 {
        return ret;
    }
    let received = ret as usize;
    if received > len {
        return neg_errno(LinuxError::EINVAL);
    }
    match write_user_bytes(process, buf, &bytes[..received]) {
        Ok(()) => ret,
        Err(err) => neg_errno(err),
    }
}

fn recv_with_real_timer_interrupt<F>(
    process: &UserProcess,
    posix_fd: i32,
    mut recv_once: F,
) -> isize
where
    F: FnMut() -> isize,
{
    let original_timeout = match arceos_posix_api::socket_recv_timeout(posix_fd) {
        Ok(timeout) => timeout,
        Err(err) => return neg_errno(err),
    };
    let status_flags = match posix_ret_i32(arceos_posix_api::sys_fcntl(
        posix_fd,
        posix_ctypes::F_GETFL as i32,
        0,
    )) {
        Ok(flags) => flags,
        Err(err) => return neg_errno(err),
    };
    if original_timeout.is_some()
        || status_flags & posix_ctypes::O_NONBLOCK as i32 != 0
        || !socket_block_interrupt_poll_required(process)
    {
        return match posix_ret_usize(recv_once()) {
            Ok(n) => n as isize,
            Err(err) => neg_errno(err),
        };
    }

    if let Err(err) =
        arceos_posix_api::set_socket_recv_timeout(posix_fd, Some(INTERRUPTIBLE_SOCKET_RECV_QUANTUM))
    {
        return neg_errno(err);
    }

    let result = loop {
        if socket_block_interrupt_pending(process) {
            break neg_errno(LinuxError::EINTR);
        }
        match posix_ret_usize(recv_once()) {
            Ok(n) => break n as isize,
            Err(LinuxError::EAGAIN) => {
                if socket_block_interrupt_pending(process) {
                    break neg_errno(LinuxError::EINTR);
                }
                axtask::yield_now();
            }
            Err(err) => break neg_errno(err),
        }
    };

    match arceos_posix_api::set_socket_recv_timeout(posix_fd, original_timeout) {
        Ok(()) => result,
        Err(err) if result >= 0 => neg_errno(err),
        Err(_) => result,
    }
}

pub(super) fn sys_socket_bridge(
    process: &UserProcess,
    domain: usize,
    socktype: usize,
    protocol: usize,
) -> isize {
    let domain = domain as i32;
    let raw_socktype = socktype as i32;
    let protocol = protocol as i32;
    let flag_mask = (posix_ctypes::SOCK_CLOEXEC | posix_ctypes::SOCK_NONBLOCK) as i32;
    let flags = raw_socktype & flag_mask;
    let base_socktype = raw_socktype & !flag_mask;
    if domain == AF_UNIX_DOMAIN {
        if protocol != 0 {
            return neg_errno_code(LINUX_EPROTONOSUPPORT);
        }
        if base_socktype as u32 != posix_ctypes::SOCK_STREAM
            && base_socktype as u32 != posix_ctypes::SOCK_DGRAM
        {
            return neg_errno_code(LINUX_ESOCKTNOSUPPORT);
        }
        return insert_local_socket_entry(process, base_socktype, flags);
    }
    if domain as u32 != posix_ctypes::AF_INET {
        return neg_errno_code(LINUX_EAFNOSUPPORT);
    }
    if base_socktype as u32 == posix_ctypes::SOCK_STREAM {
        if protocol != 0 && protocol as u32 != posix_ctypes::IPPROTO_TCP {
            return neg_errno_code(LINUX_EPROTONOSUPPORT);
        }
    } else if base_socktype as u32 == posix_ctypes::SOCK_DGRAM {
        if protocol != 0 && protocol as u32 != posix_ctypes::IPPROTO_UDP {
            return neg_errno_code(LINUX_EPROTONOSUPPORT);
        }
    } else if base_socktype as u32 == posix_ctypes::SOCK_RAW {
        return neg_errno_code(LINUX_EPROTONOSUPPORT);
    } else {
        return neg_errno(LinuxError::EINVAL);
    }
    let posix_fd = match posix_ret_i32(arceos_posix_api::sys_socket(
        domain,
        base_socktype,
        protocol,
    )) {
        Ok(fd) => fd,
        Err(err) => return neg_errno(err),
    };
    insert_socket_entry(process, posix_fd, base_socktype, flags)
}

pub(super) fn sys_socketpair_bridge(
    process: &UserProcess,
    domain: usize,
    socktype: usize,
    protocol: usize,
    sv: usize,
) -> isize {
    let domain = domain as i32;
    let raw_socktype = socktype as i32;
    let protocol = protocol as i32;
    let flag_mask = (posix_ctypes::SOCK_CLOEXEC | posix_ctypes::SOCK_NONBLOCK) as i32;
    let flags = raw_socktype & flag_mask;
    let base_socktype = raw_socktype & !flag_mask;

    if domain as u32 == posix_ctypes::AF_INET {
        if base_socktype as u32 == posix_ctypes::SOCK_STREAM {
            if protocol != 0 && protocol as u32 != posix_ctypes::IPPROTO_TCP {
                return neg_errno_code(LINUX_EPROTONOSUPPORT);
            }
            return neg_errno_code(LINUX_EOPNOTSUPP);
        }
        if base_socktype as u32 == posix_ctypes::SOCK_DGRAM {
            if protocol != 0 && protocol as u32 != posix_ctypes::IPPROTO_UDP {
                return neg_errno_code(LINUX_EPROTONOSUPPORT);
            }
            return neg_errno_code(LINUX_EOPNOTSUPP);
        }
        if base_socktype as u32 == posix_ctypes::SOCK_RAW {
            return neg_errno_code(LINUX_EPROTONOSUPPORT);
        }
        return neg_errno(LinuxError::EINVAL);
    } else if domain != AF_UNIX_DOMAIN {
        return neg_errno_code(LINUX_EAFNOSUPPORT);
    }
    if protocol != 0 {
        return neg_errno_code(LINUX_EPROTONOSUPPORT);
    }
    if base_socktype as u32 != posix_ctypes::SOCK_STREAM
        && base_socktype as u32 != posix_ctypes::SOCK_DGRAM
    {
        return neg_errno_code(LINUX_ESOCKTNOSUPPORT);
    }
    insert_local_socket_pair_entries(process, base_socktype, flags, sv)
}

pub(super) fn sys_bind_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return sys_bind_local_socket(process, fd, addr, addrlen),
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    let socket = match socket_entry(process, fd) {
        Ok(socket) => socket,
        Err(err) => return neg_errno(err),
    };
    let addr_bytes = match read_socket_addr_from_user(process, addr, addrlen) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    if process.uid() != 0 && inet_privileged_port(&addr_bytes).is_some() {
        return neg_errno(LinuxError::EACCES);
    }
    match posix_ret_i32(arceos_posix_api::sys_bind(
        socket.posix_fd,
        addr_bytes.as_ptr() as *const posix_ctypes::sockaddr,
        addrlen as posix_ctypes::socklen_t,
    )) {
        Ok(_) => 0,
        Err(err) => neg_errno(err),
    }
}

fn inet_privileged_port(addr_bytes: &[u8]) -> Option<u16> {
    let addr = unsafe {
        core::ptr::read_unaligned(addr_bytes.as_ptr() as *const posix_ctypes::sockaddr_in)
    };
    if addr.sin_family != posix_ctypes::AF_INET as posix_ctypes::sa_family_t {
        return None;
    }
    let port = u16::from_be(addr.sin_port);
    (1..1024).contains(&port).then_some(port)
}

fn sys_bind_local_socket(process: &UserProcess, fd: usize, addr: usize, addrlen: usize) -> isize {
    let path = match read_unix_path_socket_addr(process, addr, addrlen) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let resolved_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, general::AT_FDCWD, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };

    let bound_path = {
        let mut table = process.fds.lock();
        match table.entry_mut(fd as i32) {
            Ok(FdEntry::LocalSocket(socket)) => {
                if socket.bound_path.lock().is_some() {
                    return neg_errno(LinuxError::EINVAL);
                }
                socket.bound_path.clone()
            }
            Ok(_) => return neg_errno(LinuxError::ENOTSOCK),
            Err(err) => return neg_errno(err),
        }
    };

    match process.fds.lock().mknodat(
        process,
        general::AT_FDCWD,
        path.as_str(),
        ST_MODE_SOCKET | 0o777,
        0,
    ) {
        Ok(()) => {
            *bound_path.lock() = Some(resolved_path);
            0
        }
        Err(LinuxError::EEXIST) => neg_errno(LinuxError::EADDRINUSE),
        Err(err) => neg_errno(err),
    }
}

fn read_unix_path_socket_addr(
    process: &UserProcess,
    addr: usize,
    addrlen: usize,
) -> Result<String, LinuxError> {
    if addr == 0 {
        return Err(LinuxError::EFAULT);
    }
    let family_len = size_of::<posix_ctypes::sa_family_t>();
    if addrlen < family_len {
        return Err(LinuxError::EINVAL);
    }
    validate_user_read(process, addr, addrlen)?;
    let bytes = read_user_bytes(process, addr, addrlen.min(MAX_USER_IO_CHUNK))?;
    let family = posix_ctypes::sa_family_t::from_ne_bytes(
        bytes[..family_len]
            .try_into()
            .unwrap_or([0; size_of::<posix_ctypes::sa_family_t>()]),
    );
    if family as i32 != AF_UNIX_DOMAIN {
        return Err(LinuxError::EINVAL);
    }
    let path_bytes = &bytes[family_len..];
    let Some(&first) = path_bytes.first() else {
        return Err(LinuxError::EINVAL);
    };
    if first == 0 {
        // Abstract AF_UNIX sockets need a listener registry.  Pathname
        // sockets, which materialize as filesystem socket nodes, are handled
        // here; abstract names remain unsupported rather than being faked.
        return Err(LinuxError::EOPNOTSUPP);
    }
    let path_len = path_bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(path_bytes.len());
    let path = core::str::from_utf8(&path_bytes[..path_len]).map_err(|_| LinuxError::EINVAL)?;
    if path.is_empty() {
        return Err(LinuxError::EINVAL);
    }
    Ok(path.into())
}

pub(super) fn sys_listen_bridge(process: &UserProcess, fd: usize, backlog: usize) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return sys_listen_local_socket(process, fd, backlog),
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    let socket = socket_entry_or_return!(process, fd);
    match posix_ret_i32(arceos_posix_api::sys_listen(
        socket.posix_fd,
        backlog as i32,
    )) {
        Ok(_) => 0,
        Err(err) => neg_errno(err),
    }
}

fn sys_listen_local_socket(process: &UserProcess, fd: usize, backlog: usize) -> isize {
    let (owner_id, socktype, path) = {
        let table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::LocalSocket(socket)) => {
                if socket.socktype as u32 != posix_ctypes::SOCK_STREAM {
                    return neg_errno(LinuxError::EOPNOTSUPP);
                }
                let Some(path) = socket.bound_path.lock().clone() else {
                    return neg_errno(LinuxError::EINVAL);
                };
                (socket.id, socket.socktype, path)
            }
            Ok(_) => return neg_errno(LinuxError::ENOTSOCK),
            Err(err) => return neg_errno(err),
        }
    };
    let mut listeners = local_socket_listeners().lock();
    if let Some(listener) = listeners.iter_mut().find(|listener| listener.path == path) {
        if listener.owner_id != owner_id {
            return neg_errno(LinuxError::EADDRINUSE);
        }
        listener.backlog = backlog.max(1);
        return 0;
    }
    listeners.push(LocalSocketListener {
        path,
        owner_id,
        socktype,
        backlog: backlog.max(1),
        owner_cred: local_socket_cred(process),
        pending: Vec::new(),
    });
    0
}

pub(super) fn sys_accept_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
    flags: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return sys_accept_local_socket(process, fd, addr, addrlen, flags),
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    let socket = socket_entry_or_return!(process, fd);
    let flag_mask = (posix_ctypes::SOCK_CLOEXEC | posix_ctypes::SOCK_NONBLOCK) as usize;
    if flags & !flag_mask != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let user_addr_requested = !(addr == 0 && addrlen == 0);
    if user_addr_requested && (addr == 0 || addrlen == 0) {
        return neg_errno(LinuxError::EFAULT);
    }

    let mut local_addr: posix_ctypes::sockaddr = unsafe { core::mem::zeroed() };
    let mut local_len = size_of::<posix_ctypes::sockaddr>() as posix_ctypes::socklen_t;

    let new_posix_fd = match accept_with_real_timer_interrupt(
        process,
        socket.posix_fd,
        &mut local_addr,
        &mut local_len,
    ) {
        Ok(fd) => fd,
        Err(err) => return neg_errno(err),
    };

    if user_addr_requested {
        let cleanup = |err| {
            let _ = arceos_posix_api::sys_close(new_posix_fd);
            neg_errno(err)
        };
        if let Err(err) =
            validate_user_write(process, addrlen, size_of::<posix_ctypes::socklen_t>())
        {
            return cleanup(err);
        }
        let len = match read_user_value::<posix_ctypes::socklen_t>(process, addrlen) {
            Ok(len) => len as usize,
            Err(err) => return cleanup(err),
        };
        if let Err(err) = validate_user_write(process, addr, len) {
            return cleanup(err);
        }
        let ret = write_socket_addr_to_user(process, addr, addrlen, len, &local_addr, local_len);
        if ret < 0 {
            let _ = arceos_posix_api::sys_close(new_posix_fd);
            return ret;
        }
    }

    insert_socket_entry(process, new_posix_fd, socket.socktype, flags as i32)
}

fn sys_accept_local_socket(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
    flags: usize,
) -> isize {
    let flag_mask = (posix_ctypes::SOCK_CLOEXEC | posix_ctypes::SOCK_NONBLOCK) as usize;
    if flags & !flag_mask != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let user_addr_requested = !(addr == 0 && addrlen == 0);
    if user_addr_requested && (addr == 0 || addrlen == 0) {
        return neg_errno(LinuxError::EFAULT);
    }
    let (owner_id, listener_nonblocking) = {
        let table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::LocalSocket(socket)) => (socket.id, socket.nonblocking),
            Ok(_) => return neg_errno(LinuxError::ENOTSOCK),
            Err(err) => return neg_errno(err),
        }
    };
    let (accepted_socktype, pending) = loop {
        if socket_block_interrupt_pending(process) {
            return neg_errno(LinuxError::EINTR);
        }
        {
            let mut listeners = local_socket_listeners().lock();
            let Some(listener) = listeners
                .iter_mut()
                .find(|listener| listener.owner_id == owner_id)
            else {
                return neg_errno(LinuxError::EINVAL);
            };
            if !listener.pending.is_empty() {
                break (listener.socktype, listener.pending.remove(0));
            }
        }
        if listener_nonblocking {
            return neg_errno(LinuxError::EAGAIN);
        }
        axtask::yield_now();
    };
    if user_addr_requested {
        if let Err(err) =
            validate_user_write(process, addrlen, size_of::<posix_ctypes::socklen_t>())
        {
            return neg_errno(err);
        }
        let len = match read_user_value::<posix_ctypes::socklen_t>(process, addrlen) {
            Ok(len) => len as usize,
            Err(err) => return neg_errno(err),
        };
        if len > SOCKET_ADDR_STORAGE_MAX {
            return neg_errno(LinuxError::EINVAL);
        }
        if let Err(err) = validate_user_write(process, addr, len) {
            return neg_errno(err);
        }
        let mut local_addr: posix_ctypes::sockaddr = unsafe { core::mem::zeroed() };
        local_addr.sa_family = AF_UNIX_DOMAIN as posix_ctypes::sa_family_t;
        let ret = write_socket_addr_to_user(
            process,
            addr,
            addrlen,
            len,
            &local_addr,
            size_of::<posix_ctypes::sa_family_t>() as posix_ctypes::socklen_t,
        );
        if ret < 0 {
            return ret;
        }
    }
    let accepted = LocalSocketEntry::new_connected(
        accepted_socktype,
        flags as i32,
        pending.endpoint,
        pending.peer_cred,
    );
    match process.fds.lock().insert_with_flags(
        FdEntry::LocalSocket(accepted),
        fd_cloexec_flag(flags & posix_ctypes::SOCK_CLOEXEC as usize != 0),
    ) {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

fn accept_with_real_timer_interrupt(
    process: &UserProcess,
    posix_fd: i32,
    local_addr: &mut posix_ctypes::sockaddr,
    local_len: &mut posix_ctypes::socklen_t,
) -> Result<i32, LinuxError> {
    let original_timeout = arceos_posix_api::socket_recv_timeout(posix_fd)?;
    let status_flags = posix_ret_i32(arceos_posix_api::sys_fcntl(
        posix_fd,
        posix_ctypes::F_GETFL as i32,
        0,
    ))?;
    if original_timeout.is_some()
        || status_flags & posix_ctypes::O_NONBLOCK as i32 != 0
        || !socket_block_interrupt_poll_required(process)
    {
        return posix_ret_i32(unsafe {
            arceos_posix_api::sys_accept(posix_fd, local_addr, local_len)
        });
    }

    arceos_posix_api::set_socket_recv_timeout(posix_fd, Some(INTERRUPTIBLE_SOCKET_RECV_QUANTUM))?;

    let result = loop {
        if socket_block_interrupt_pending(process) {
            break Err(LinuxError::EINTR);
        }
        match posix_ret_i32(unsafe {
            arceos_posix_api::sys_accept(posix_fd, local_addr, local_len)
        }) {
            Ok(fd) => break Ok(fd),
            Err(LinuxError::EAGAIN) => {
                if socket_block_interrupt_pending(process) {
                    break Err(LinuxError::EINTR);
                }
                axtask::yield_now();
            }
            Err(err) => break Err(err),
        }
    };

    match (
        arceos_posix_api::set_socket_recv_timeout(posix_fd, original_timeout),
        result,
    ) {
        (Ok(()), result) => result,
        (Err(err), Ok(fd)) => {
            let _ = arceos_posix_api::sys_close(fd);
            Err(err)
        }
        (Err(_), Err(err)) => Err(err),
    }
}

fn socket_block_interrupt_poll_required(_process: &UserProcess) -> bool {
    true
}

fn socket_block_interrupt_pending(process: &UserProcess) -> bool {
    current_unblocked_signal_pending()
        || process.pending_exit_group().is_some()
        || process.eval_watchdog_expired()
        || process.consume_expired_real_timer()
}

pub(super) fn sys_connect_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return sys_connect_local_socket(process, fd, addr, addrlen),
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    socket_addr_call(process, fd, addr, addrlen, arceos_posix_api::sys_connect)
}

fn sys_connect_local_socket(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    if addr == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let family_len = size_of::<posix_ctypes::sa_family_t>();
    if addrlen < family_len {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_read(process, addr, addrlen) {
        return neg_errno(err);
    }
    let bytes = match read_user_bytes(process, addr, addrlen.min(MAX_USER_IO_CHUNK)) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    let family = posix_ctypes::sa_family_t::from_ne_bytes(
        bytes[..family_len]
            .try_into()
            .unwrap_or([0; size_of::<posix_ctypes::sa_family_t>()]),
    );
    if family as i32 != AF_UNIX_DOMAIN {
        return neg_errno_code(LINUX_EAFNOSUPPORT);
    }
    let path_bytes = &bytes[family_len..];
    let Some(&first) = path_bytes.first() else {
        return neg_errno(LinuxError::EINVAL);
    };
    if first == 0 {
        return neg_errno(LinuxError::ECONNREFUSED);
    }
    let path_len = path_bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(path_bytes.len());
    let path = match core::str::from_utf8(&path_bytes[..path_len]) {
        Ok(path) if !path.is_empty() => path,
        _ => return neg_errno(LinuxError::EINVAL),
    };
    let resolved_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, general::AT_FDCWD, path) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    if axfs::api::metadata(resolved_path.as_str()).is_err() {
        return neg_errno(LinuxError::ENOENT);
    }
    let socktype = {
        let table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::LocalSocket(socket)) => {
                if socket.pair.is_some() {
                    return neg_errno(LinuxError::EINVAL);
                }
                socket.socktype
            }
            Ok(_) => return neg_errno(LinuxError::ENOTSOCK),
            Err(err) => return neg_errno(err),
        }
    };
    let (client_side, owner_cred) = {
        let mut listeners = local_socket_listeners().lock();
        let Some(listener) = listeners
            .iter_mut()
            .find(|listener| listener.path == resolved_path)
        else {
            return neg_errno(LinuxError::ECONNREFUSED);
        };
        if socktype != listener.socktype {
            return neg_errno(LinuxError::EINVAL);
        }
        if listener.pending.len() >= listener.backlog {
            return neg_errno(LinuxError::ECONNREFUSED);
        }
        let state = LocalSocketPairState::new();
        let server_side = LocalSocketPairEndpoint {
            side: 0,
            state: state.clone(),
        };
        let client_side = LocalSocketPairEndpoint { side: 1, state };
        let owner_cred = listener.owner_cred;
        listener.pending.push(LocalSocketPending {
            endpoint: server_side,
            peer_cred: local_socket_cred(process),
        });
        (client_side, owner_cred)
    };
    let mut table = process.fds.lock();
    match table.entry_mut(fd as i32) {
        Ok(FdEntry::LocalSocket(socket)) => {
            if socket.pair.is_some() {
                return neg_errno(LinuxError::EINVAL);
            }
            socket.set_connected(client_side, owner_cred);
            0
        }
        Ok(_) => neg_errno(LinuxError::ENOTSOCK),
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_sendto_bridge(
    process: &UserProcess,
    fd: usize,
    buf: usize,
    len: usize,
    flags: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    let socket = socket_entry_or_return!(process, fd);
    let bytes = match read_socket_data_from_user(process, buf, len) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    let len = bytes.len();
    let data_ptr = bytes.as_ptr() as *const c_void;
    let ret = if addr == 0 || socket.socktype as u32 == posix_ctypes::SOCK_STREAM {
        unsafe { arceos_posix_api::sys_send(socket.posix_fd, data_ptr, len, flags as i32) }
    } else {
        let addr_bytes = match read_socket_addr_from_user(process, addr, addrlen) {
            Ok(bytes) => bytes,
            Err(err) => return neg_errno(err),
        };
        unsafe {
            arceos_posix_api::sys_sendto(
                socket.posix_fd,
                data_ptr,
                len,
                flags as i32,
                addr_bytes.as_ptr() as *const posix_ctypes::sockaddr,
                addrlen as posix_ctypes::socklen_t,
            )
        }
    };
    match posix_ret_usize(ret) {
        Ok(n) => n as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_sendmsg_bridge(
    process: &UserProcess,
    fd: usize,
    msg: usize,
    flags: usize,
) -> isize {
    if msg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let msg = match read_user_value::<LinuxMsghdr>(process, msg) {
        Ok(msg) => msg,
        Err(err) => return neg_errno(err),
    };
    if msg.msg_iovlen > 1024 {
        return neg_errno(LinuxError::EMSGSIZE);
    }
    let iov_entries = match read_iovec_entries(process, msg.msg_iov as usize, msg.msg_iovlen) {
        Ok(entries) => entries,
        Err(err) => return neg_errno(err),
    };
    let mut bytes = Vec::new();
    for entry in iov_entries {
        let base = entry.iov_base as usize;
        let len = (entry.iov_len as usize).min(MAX_USER_IO_CHUNK);
        if let Err(err) = validate_user_read(process, base, len) {
            return neg_errno(err);
        }
        if len == 0 {
            continue;
        }
        let start = bytes.len();
        if bytes.try_reserve_exact(len).is_err() {
            return neg_errno(LinuxError::ENOMEM);
        }
        bytes.resize(start + len, 0);
        if let Err(err) = read_user_bytes_into(process, base, &mut bytes[start..start + len]) {
            return neg_errno(err);
        }
    }
    if matches!(is_local_socket_fd(process, fd), Ok(true)) {
        return match process.fds.lock().write(process, fd as i32, &bytes, None) {
            Ok(n) => n as isize,
            Err(err) => neg_errno(err),
        };
    }
    let socket = socket_entry_or_return!(process, fd);
    let ret = if msg.msg_name.is_null() {
        unsafe {
            arceos_posix_api::sys_send(
                socket.posix_fd,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                flags as i32,
            )
        }
    } else {
        let addrlen = msg.msg_namelen as usize;
        let addr_bytes = match read_socket_addr_from_user(process, msg.msg_name as usize, addrlen) {
            Ok(bytes) => bytes,
            Err(err) => return neg_errno(err),
        };
        unsafe {
            arceos_posix_api::sys_sendto(
                socket.posix_fd,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                flags as i32,
                addr_bytes.as_ptr() as *const posix_ctypes::sockaddr,
                addrlen as posix_ctypes::socklen_t,
            )
        }
    };
    match posix_ret_usize(ret) {
        Ok(n) => n as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_recvfrom_bridge(
    process: &UserProcess,
    fd: usize,
    buf: usize,
    len: usize,
    flags: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    let socket = socket_entry_or_return!(process, fd);
    if addr == 0 || addrlen == 0 {
        recv_socket_data_to_user(process, socket.posix_fd, buf, len, flags as i32)
    } else {
        if let Err(err) =
            validate_user_write(process, addrlen, size_of::<posix_ctypes::socklen_t>())
        {
            return neg_errno(err);
        }
        let addr_len_value = match read_user_value::<posix_ctypes::socklen_t>(process, addrlen) {
            Ok(len) => len as usize,
            Err(err) => return neg_errno(err),
        };
        if addr_len_value > SOCKET_ADDR_STORAGE_MAX {
            return neg_errno(LinuxError::EINVAL);
        }
        if socket.socktype as u32 == posix_ctypes::SOCK_STREAM {
            return recv_socket_data_to_user(process, socket.posix_fd, buf, len, flags as i32);
        }
        if socket.socktype as u32 != posix_ctypes::SOCK_STREAM {
            if let Err(err) = validate_user_write(process, addr, addr_len_value) {
                return neg_errno(err);
            }
        }
        recv_socket_data_to_user_with_addr(
            process,
            socket.posix_fd,
            buf,
            len,
            flags as i32,
            addr,
            addrlen,
            addr_len_value,
        )
    }
}

pub(super) fn sys_recvmsg_bridge(
    process: &UserProcess,
    fd: usize,
    msg: usize,
    flags: usize,
) -> isize {
    if msg == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let mut msg_value = match read_user_value::<LinuxMsghdr>(process, msg) {
        Ok(msg) => msg,
        Err(err) => return neg_errno(err),
    };
    if msg_value.msg_namelen < 0 || msg_value.msg_namelen as usize > SOCKET_ADDR_STORAGE_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if msg_value.msg_iovlen > 1024 {
        return neg_errno(LinuxError::EMSGSIZE);
    }
    let iov_entries =
        match read_iovec_entries(process, msg_value.msg_iov as usize, msg_value.msg_iovlen) {
            Ok(entries) => entries,
            Err(LinuxError::EINVAL) if msg_value.msg_iovlen > 1024 => {
                return neg_errno(LinuxError::EMSGSIZE);
            }
            Err(err) => return neg_errno(err),
        };
    let receive_len = capped_iovec_write_len(&iov_entries);
    if receive_len == 0 {
        return 0;
    }
    if let Err(err) = validate_iovec_write(process, &iov_entries, receive_len) {
        return neg_errno(err);
    }
    if socket_recv_error_queue_empty(flags as i32) {
        return neg_errno(LinuxError::EAGAIN);
    }
    let addr_ptr = msg_value.msg_name as usize;
    let ret = if matches!(is_local_socket_fd(process, fd), Ok(true)) {
        let mut bytes = match user_io_buffer(receive_len) {
            Ok(bytes) => bytes,
            Err(err) => return neg_errno(err),
        };
        match process.fds.lock().read(process, fd as i32, &mut bytes) {
            Ok(n) => {
                if let Err(err) = scatter_iovec_bytes_to_user(process, &iov_entries, &bytes[..n]) {
                    return neg_errno(err);
                }
                if !msg_value.msg_name.is_null() {
                    msg_value.msg_namelen = 0;
                }
                n as isize
            }
            Err(err) => neg_errno(err),
        }
    } else if msg_value.msg_name.is_null() {
        let socket = socket_entry_or_return!(process, fd);
        let mut bytes = match user_io_buffer(receive_len) {
            Ok(bytes) => bytes,
            Err(err) => return neg_errno(err),
        };
        let ret = recv_socket_data_to_buffer(process, socket.posix_fd, &mut bytes, |dst| unsafe {
            arceos_posix_api::sys_recv(socket.posix_fd, dst, receive_len, flags as i32)
        });
        if ret > 0 {
            if let Err(err) =
                scatter_iovec_bytes_to_user(process, &iov_entries, &bytes[..ret as usize])
            {
                return neg_errno(err);
            }
        }
        ret
    } else {
        if let Err(err) = validate_user_write(process, addr_ptr, msg_value.msg_namelen as usize) {
            return neg_errno(err);
        }
        let user_addr_len = msg_value.msg_namelen as usize;
        let mut addr_len_value = user_addr_len as posix_ctypes::socklen_t;
        let mut local_addr: posix_ctypes::sockaddr = unsafe { core::mem::zeroed() };
        let posix_fd = socket_entry_or_return!(process, fd).posix_fd;
        let mut bytes = match user_io_buffer(receive_len) {
            Ok(bytes) => bytes,
            Err(err) => return neg_errno(err),
        };
        let ret = recv_socket_data_to_buffer(process, posix_fd, &mut bytes, |dst| unsafe {
            arceos_posix_api::sys_recvfrom(
                posix_fd,
                dst,
                receive_len,
                flags as i32,
                &mut local_addr,
                &mut addr_len_value,
            )
        });
        if ret > 0 {
            if let Err(err) =
                scatter_iovec_bytes_to_user(process, &iov_entries, &bytes[..ret as usize])
            {
                return neg_errno(err);
            }
        }
        if ret >= 0 && addr_len_value != 0 {
            let copy_len = cmp::min(
                user_addr_len,
                cmp::min(addr_len_value as usize, size_of::<posix_ctypes::sockaddr>()),
            );
            if copy_len > 0 {
                let local_addr_bytes = sockaddr_bytes(&local_addr);
                if let Err(err) = write_user_bytes(process, addr_ptr, &local_addr_bytes[..copy_len])
                {
                    return neg_errno(err);
                }
            }
            msg_value.msg_namelen = addr_len_value as i32;
        }
        ret
    };
    if ret >= 0 {
        msg_value.msg_flags = 0;
        msg_value.msg_controllen = 0;
        let write_ret = write_user_value(process, msg, &msg_value);
        if write_ret != 0 {
            return write_ret;
        }
    }
    ret
}

pub(super) fn sys_shutdown_bridge(process: &UserProcess, fd: usize, how: usize) -> isize {
    let socket = socket_entry_or_return!(process, fd);
    match posix_ret_i32(arceos_posix_api::sys_shutdown(socket.posix_fd, how as i32)) {
        Ok(_) => {
            socket.mark_shutdown(how as i32);
            0
        }
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_getsockname_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return local_socket_name_bridge(process, addr, addrlen),
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    socket_name_bridge(
        process,
        fd,
        addr,
        addrlen,
        arceos_posix_api::sys_getsockname,
    )
}

pub(super) fn sys_getpeername_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return local_socket_name_bridge(process, addr, addrlen),
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    socket_name_bridge(
        process,
        fd,
        addr,
        addrlen,
        arceos_posix_api::sys_getpeername,
    )
}

pub(super) fn sys_setsockopt_bridge(
    process: &UserProcess,
    fd: usize,
    level: usize,
    optname: usize,
    optval: usize,
    optlen: usize,
) -> isize {
    let socket = socket_entry_or_return!(process, fd);
    if optlen > 0 {
        if let Err(err) = validate_user_read(process, optval, optlen) {
            return neg_errno(err);
        }
    }
    let level_i32 = level as i32;
    let optname_i32 = optname as i32;
    if optlen > SOCKET_OPTLEN_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if optlen < size_of::<i32>()
        && (level_i32 == SOL_SOCKET_LEVEL
            || level_i32 == IPPROTO_IP_LEVEL
            || level_i32 == posix_ctypes::IPPROTO_TCP as i32)
    {
        return neg_errno(LinuxError::EINVAL);
    }
    if level_i32 == SOL_SOCKET_LEVEL && matches!(optname_i32, SO_RCVTIMEO_OPT | SO_SNDTIMEO_OPT) {
        if optlen < size_of::<general::timeval>() {
            neg_errno(LinuxError::EINVAL)
        } else {
            match read_user_value::<general::timeval>(process, optval)
                .and_then(socket_timeval_to_duration)
            {
                Ok(timeout) => {
                    let result = if optname_i32 == SO_RCVTIMEO_OPT {
                        arceos_posix_api::set_socket_recv_timeout(socket.posix_fd, timeout)
                    } else {
                        arceos_posix_api::set_socket_send_timeout(socket.posix_fd, timeout)
                    };
                    match result {
                        Ok(()) => 0,
                        Err(err) => neg_errno(err),
                    }
                }
                Err(err) => neg_errno(err),
            }
        }
    } else if level_i32 == SOL_SOCKET_LEVEL
        && (optname_i32 == SO_SNDBUFFORCE_OPT || optname_i32 == SO_RCVBUFFORCE_OPT)
    {
        if process.uid() != 0 {
            return neg_errno(LinuxError::EPERM);
        }
        let size = match read_user_value::<u32>(process, optval) {
            Ok(size) => size,
            Err(err) => return neg_errno(err),
        };
        let result = if optname_i32 == SO_SNDBUFFORCE_OPT {
            arceos_posix_api::force_socket_send_buffer_size(socket.posix_fd, size)
        } else {
            arceos_posix_api::force_socket_recv_buffer_size(socket.posix_fd, size)
        };
        match result {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        }
    } else if level_i32 == SOL_SOCKET_LEVEL && matches!(optname_i32, SO_SNDBUF_OPT | SO_RCVBUF_OPT)
    {
        let size = match read_user_value::<i32>(process, optval) {
            Ok(size) => size,
            Err(err) => return neg_errno(err),
        };
        let result = if optname_i32 == SO_SNDBUF_OPT {
            arceos_posix_api::set_socket_send_buffer_size(socket.posix_fd, size)
        } else {
            arceos_posix_api::set_socket_recv_buffer_size(socket.posix_fd, size)
        };
        match result {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        }
    } else if level_i32 == SOL_SOCKET_LEVEL && optname_i32 == SO_REUSEADDR_OPT {
        let enabled = match read_socket_bool_option(process, optval) {
            Ok(enabled) => enabled,
            Err(err) => return neg_errno(err),
        };
        match arceos_posix_api::set_socket_reuse_addr(socket.posix_fd, enabled) {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        }
    } else if level_i32 == posix_ctypes::IPPROTO_TCP as i32 && optname_i32 == TCP_NODELAY_OPT {
        let enabled = match read_socket_bool_option(process, optval) {
            Ok(enabled) => enabled,
            Err(err) => return neg_errno(err),
        };
        match arceos_posix_api::set_socket_tcp_nodelay(socket.posix_fd, enabled) {
            Ok(()) => 0,
            Err(err) => neg_errno(err),
        }
    } else if level_i32 == IPPROTO_IP_LEVEL
        && matches!(
            optname_i32,
            IP_MCAST_JOIN_GROUP_OPT | IP_MCAST_LEAVE_GROUP_OPT
        )
    {
        let group = match read_user_bytes(process, optval, optlen) {
            Ok(group) => group,
            Err(err) => return neg_errno(err),
        };
        if optname_i32 == IP_MCAST_JOIN_GROUP_OPT {
            socket.join_multicast_group(group);
            0
        } else {
            match socket.leave_multicast_group(group.as_slice()) {
                Ok(()) => 0,
                Err(err) => neg_errno(err),
            }
        }
    } else {
        neg_errno_code(setsockopt_unsupported_errno_code(level_i32))
    }
}

pub(super) fn sys_getsockopt_bridge(
    process: &UserProcess,
    fd: usize,
    level: usize,
    optname: usize,
    optval: usize,
    optlen: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => {
            return sys_getsockopt_local_socket(process, fd, level, optname, optval, optlen);
        }
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    let socket = socket_entry_or_return!(process, fd);
    if optval == 0 || optlen == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let len = match read_user_value::<posix_ctypes::socklen_t>(process, optlen) {
        Ok(len) => len as usize,
        Err(err) => return neg_errno(err),
    };
    let level = level as i32;
    let optname = optname as i32;
    if len > SOCKET_OPTLEN_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if level == SOL_SOCKET_LEVEL && matches!(optname, SO_RCVTIMEO_OPT | SO_SNDTIMEO_OPT) {
        if len < size_of::<general::timeval>() {
            return neg_errno(LinuxError::EINVAL);
        }
        if let Err(err) = validate_user_write(process, optval, size_of::<general::timeval>()) {
            return neg_errno(err);
        }
        let timeout = if optname == SO_RCVTIMEO_OPT {
            arceos_posix_api::socket_recv_timeout(socket.posix_fd)
        } else {
            arceos_posix_api::socket_send_timeout(socket.posix_fd)
        };
        let value = match timeout {
            Ok(timeout) => socket_duration_to_timeval(timeout),
            Err(err) => return neg_errno(err),
        };
        let ret = write_user_value(process, optval, &value);
        if ret != 0 {
            return ret;
        }
        let out_len = size_of::<general::timeval>() as posix_ctypes::socklen_t;
        return write_user_value(process, optlen, &out_len);
    }
    if level == SOL_SOCKET_LEVEL && matches!(optname, SO_SNDBUF_OPT | SO_RCVBUF_OPT) {
        if len < size_of::<i32>() {
            return neg_errno(LinuxError::EINVAL);
        }
        if let Err(err) = validate_user_write(process, optval, size_of::<i32>()) {
            return neg_errno(err);
        }
        let value = if optname == SO_SNDBUF_OPT {
            arceos_posix_api::socket_send_buffer_size(socket.posix_fd)
        } else {
            arceos_posix_api::socket_recv_buffer_size(socket.posix_fd)
        };
        let value = match value {
            Ok(value) => value,
            Err(err) => return neg_errno(err),
        };
        let ret = write_user_value(process, optval, &value);
        if ret != 0 {
            return ret;
        }
        let out_len = size_of::<i32>() as posix_ctypes::socklen_t;
        return write_user_value(process, optlen, &out_len);
    }
    if level == SOL_SOCKET_LEVEL && optname == SO_REUSEADDR_OPT {
        if len < size_of::<i32>() {
            return neg_errno(LinuxError::EINVAL);
        }
        if let Err(err) = validate_user_write(process, optval, size_of::<i32>()) {
            return neg_errno(err);
        }
        let value = match arceos_posix_api::socket_reuse_addr(socket.posix_fd) {
            Ok(enabled) => i32::from(enabled),
            Err(err) => return neg_errno(err),
        };
        let ret = write_user_value(process, optval, &value);
        if ret != 0 {
            return ret;
        }
        let out_len = size_of::<i32>() as posix_ctypes::socklen_t;
        return write_user_value(process, optlen, &out_len);
    }
    if level == posix_ctypes::IPPROTO_TCP as i32
        && matches!(optname, TCP_NODELAY_OPT | TCP_MAXSEG_OPT)
    {
        if len < size_of::<i32>() {
            return neg_errno(LinuxError::EINVAL);
        }
        if let Err(err) = validate_user_write(process, optval, size_of::<i32>()) {
            return neg_errno(err);
        }
        let value = if optname == TCP_NODELAY_OPT {
            match arceos_posix_api::socket_tcp_nodelay(socket.posix_fd) {
                Ok(enabled) => i32::from(enabled),
                Err(err) => return neg_errno(err),
            }
        } else {
            match arceos_posix_api::socket_tcp_max_segment_size(socket.posix_fd) {
                Ok(value) => value,
                Err(err) => return neg_errno(err),
            }
        };
        let ret = write_user_value(process, optval, &value);
        if ret != 0 {
            return ret;
        }
        let out_len = size_of::<i32>() as posix_ctypes::socklen_t;
        return write_user_value(process, optlen, &out_len);
    }
    if len < size_of::<i32>() {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_write(process, optval, size_of::<i32>()) {
        return neg_errno(err);
    }
    let value = if let Some(value) = socket_readonly_scalar(&socket, level, optname) {
        value
    } else {
        return neg_errno_code(getsockopt_unsupported_errno_code(&socket, level));
    };
    let ret = write_user_value(process, optval, &value);
    if ret != 0 {
        return ret;
    }
    let out_len = size_of::<i32>() as posix_ctypes::socklen_t;
    write_user_value(process, optlen, &out_len)
}

fn sys_getsockopt_local_socket(
    process: &UserProcess,
    fd: usize,
    level: usize,
    optname: usize,
    optval: usize,
    optlen: usize,
) -> isize {
    if optval == 0 || optlen == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let len = match read_user_value::<posix_ctypes::socklen_t>(process, optlen) {
        Ok(len) => len as usize,
        Err(err) => return neg_errno(err),
    };
    let level = level as i32;
    let optname = optname as i32;
    let socket = {
        let table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::LocalSocket(socket)) => socket.duplicate(),
            Ok(_) => return neg_errno(LinuxError::ENOTSOCK),
            Err(err) => return neg_errno(err),
        }
    };
    if level == SOL_SOCKET_LEVEL && optname == SO_PEERCRED_OPT {
        if len < size_of::<LinuxUcred>() {
            return neg_errno(LinuxError::EINVAL);
        }
        let Some(peer) = *socket.peer_cred.lock() else {
            return neg_errno(LinuxError::ENOTCONN);
        };
        if let Err(err) = validate_user_write(process, optval, size_of::<LinuxUcred>()) {
            return neg_errno(err);
        }
        let cred = LinuxUcred {
            pid: peer.pid,
            uid: peer.uid,
            gid: peer.gid,
        };
        let ret = write_user_value(process, optval, &cred);
        if ret != 0 {
            return ret;
        }
        let out_len = size_of::<LinuxUcred>() as posix_ctypes::socklen_t;
        return write_user_value(process, optlen, &out_len);
    }
    if len < size_of::<i32>() {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_write(process, optval, size_of::<i32>()) {
        return neg_errno(err);
    }
    let value = if level == SOL_SOCKET_LEVEL {
        match optname {
            SO_ERROR_OPT => 0,
            SO_TYPE_OPT => socket.socktype,
            _ => return neg_errno_code(LINUX_ENOPROTOOPT),
        }
    } else {
        return neg_errno_code(LINUX_ENOPROTOOPT);
    };
    let ret = write_user_value(process, optval, &value);
    if ret != 0 {
        return ret;
    }
    let out_len = size_of::<i32>() as posix_ctypes::socklen_t;
    write_user_value(process, optlen, &out_len)
}
