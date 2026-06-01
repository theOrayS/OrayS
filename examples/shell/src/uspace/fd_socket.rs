use core::cmp;
use core::ffi::c_void;
use core::mem::size_of;
use core::sync::atomic::{AtomicUsize, Ordering};

use arceos_posix_api::ctypes as posix_ctypes;
use axerrno::LinuxError;
use axsync::Mutex;
use linux_raw_sys::general;
use std::sync::Arc;
use std::vec::Vec;

use super::fd_table::{FdEntry, resolve_dirfd_path};
use super::linux_abi::{
    AF_UNIX_DOMAIN, DEFAULT_SOCKET_BUFFER_SIZE, DEFAULT_TCP_MAXSEG,
    INTERRUPTIBLE_SOCKET_RECV_QUANTUM, IP_RECVERR_OPT, IPPROTO_IP_LEVEL, LINUX_EAFNOSUPPORT,
    LINUX_EPROTONOSUPPORT, LINUX_ESOCKTNOSUPPORT, LOCAL_SOCKET_INO_BASE, MCAST_JOIN_GROUP_OPT,
    MCAST_LEAVE_GROUP_OPT, SO_BROADCAST_OPT, SO_DONTROUTE_OPT, SO_ERROR_OPT, SO_KEEPALIVE_OPT,
    SO_RCVBUF_OPT, SO_RCVTIMEO_OPT, SO_REUSEADDR_OPT, SO_REUSEPORT_OPT, SO_SNDBUF_OPT,
    SO_SNDTIMEO_OPT, SO_TYPE_OPT, SOL_SOCKET_LEVEL, ST_MODE_SOCKET, TCP_INFO_COMPAT_SIZE,
    TCP_INFO_OPT, TCP_MAXSEG_OPT, TCP_NODELAY_OPT, fd_cloexec_flag, neg_errno_code,
    posix_errno_from_ret,
};
use super::signal_abi::current_unblocked_signal_pending;
use super::time_abi::{socket_duration_to_timeval, socket_timeval_to_duration};
use super::user_memory::{
    MAX_USER_IO_CHUNK, clear_user_bytes, read_user_bytes, read_user_value, user_io_buffer,
    validate_user_read, validate_user_write, write_user_bytes, write_user_value,
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

#[derive(Default)]
pub(super) struct SocketOptions {
    pub(super) ip_mcast_joined: bool,
}

#[derive(Clone)]
pub(super) struct SocketEntry {
    pub(super) posix_fd: i32,
    pub(super) socktype: i32,
    pub(super) options: Arc<Mutex<SocketOptions>>,
}

pub(super) struct LocalSocketEntry {
    id: usize,
    socktype: i32,
    nonblocking: bool,
    pair: Option<LocalSocketPairEndpoint>,
    options: Arc<Mutex<SocketOptions>>,
}

static NEXT_LOCAL_SOCKET_ID: AtomicUsize = AtomicUsize::new(1);
const LOCAL_SOCKET_BUFFER_SIZE: usize = 4096;

#[derive(Clone, Copy, Eq, PartialEq)]
enum LocalSocketBufferStatus {
    Full,
    Empty,
    Normal,
}

struct LocalSocketBuffer {
    data: [u8; LOCAL_SOCKET_BUFFER_SIZE],
    head: usize,
    tail: usize,
    status: LocalSocketBufferStatus,
}

struct LocalSocketPairState {
    buffers: [Mutex<LocalSocketBuffer>; 2],
    open_ends: Mutex<[usize; 2]>,
}

struct LocalSocketPairEndpoint {
    side: usize,
    state: Arc<LocalSocketPairState>,
}

impl SocketEntry {
    pub(super) fn new(posix_fd: i32, socktype: i32) -> Self {
        Self {
            posix_fd,
            socktype,
            options: Arc::new(Mutex::new(SocketOptions::default())),
        }
    }

    pub(super) fn duplicate(&self) -> Result<Self, LinuxError> {
        let posix_fd = posix_ret_i32(arceos_posix_api::sys_dup(self.posix_fd))?;
        Ok(Self {
            posix_fd,
            socktype: self.socktype,
            options: self.options.clone(),
        })
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
    const fn new() -> Self {
        Self {
            data: [0; LOCAL_SOCKET_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: LocalSocketBufferStatus::Empty,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        self.status = LocalSocketBufferStatus::Normal;
        self.data[self.tail] = byte;
        self.tail = (self.tail + 1) % LOCAL_SOCKET_BUFFER_SIZE;
        if self.tail == self.head {
            self.status = LocalSocketBufferStatus::Full;
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.status = LocalSocketBufferStatus::Normal;
        let byte = self.data[self.head];
        self.head = (self.head + 1) % LOCAL_SOCKET_BUFFER_SIZE;
        if self.head == self.tail {
            self.status = LocalSocketBufferStatus::Empty;
        }
        byte
    }

    const fn available_read(&self) -> usize {
        if matches!(self.status, LocalSocketBufferStatus::Empty) {
            0
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.tail + LOCAL_SOCKET_BUFFER_SIZE - self.head
        }
    }

    const fn available_write(&self) -> usize {
        if matches!(self.status, LocalSocketBufferStatus::Full) {
            0
        } else {
            LOCAL_SOCKET_BUFFER_SIZE - self.available_read()
        }
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
        })
    }

    fn duplicate_side(&self, side: usize) {
        self.open_ends.lock()[side] += 1;
    }

    fn close_side(&self, side: usize) {
        let mut open = self.open_ends.lock();
        if open[side] > 0 {
            open[side] -= 1;
        }
    }

    fn peer_open(&self, side: usize) -> bool {
        self.open_ends.lock()[1 - side] > 0
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

impl Clone for LocalSocketEntry {
    fn clone(&self) -> Self {
        self.duplicate()
    }
}

impl LocalSocketEntry {
    pub(super) fn new(socktype: i32, flags: i32) -> Self {
        Self {
            id: NEXT_LOCAL_SOCKET_ID.fetch_add(1, Ordering::Relaxed),
            socktype,
            nonblocking: flags & posix_ctypes::SOCK_NONBLOCK as i32 != 0,
            pair: None,
            options: Arc::new(Mutex::new(SocketOptions::default())),
        }
    }

    pub(super) fn new_pair(socktype: i32, flags: i32) -> (Self, Self) {
        let state = LocalSocketPairState::new();
        let options = Arc::new(Mutex::new(SocketOptions::default()));
        let nonblocking = flags & posix_ctypes::SOCK_NONBLOCK as i32 != 0;
        let first_id = NEXT_LOCAL_SOCKET_ID.fetch_add(2, Ordering::Relaxed);
        (
            Self {
                id: first_id,
                socktype,
                nonblocking,
                pair: Some(LocalSocketPairEndpoint {
                    side: 0,
                    state: state.clone(),
                }),
                options: options.clone(),
            },
            Self {
                id: first_id + 1,
                socktype,
                nonblocking,
                pair: Some(LocalSocketPairEndpoint { side: 1, state }),
                options,
            },
        )
    }

    pub(super) fn duplicate(&self) -> Self {
        Self {
            id: self.id,
            socktype: self.socktype,
            nonblocking: self.nonblocking,
            pair: self.pair.clone(),
            options: self.options.clone(),
        }
    }

    pub(super) fn read(&self, dst: &mut [u8]) -> Result<usize, LinuxError> {
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
                drop(buffer);
                axtask::yield_now();
                continue;
            }
            let take = cmp::min(available, dst.len() - read_len);
            for byte in &mut dst[read_len..read_len + take] {
                *byte = buffer.read_byte();
            }
            read_len += take;
            if read_len > 0 {
                return Ok(read_len);
            }
        }
        Ok(read_len)
    }

    pub(super) fn write(&self, src: &[u8]) -> Result<usize, LinuxError> {
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
            let available = buffer.available_write();
            if available == 0 {
                if self.nonblocking {
                    return if written > 0 {
                        Ok(written)
                    } else {
                        Err(LinuxError::EAGAIN)
                    };
                }
                drop(buffer);
                axtask::yield_now();
                continue;
            }
            let take = cmp::min(available, src.len() - written);
            for byte in &src[written..written + take] {
                buffer.write_byte(*byte);
            }
            written += take;
        }
        Ok(written)
    }

    pub(super) fn poll(&self, mode: SelectMode) -> bool {
        let Some(pair) = &self.pair else {
            return matches!(mode, SelectMode::Write);
        };
        match mode {
            SelectMode::Read => {
                let buffer = pair.state.buffers[pair.side].lock();
                buffer.available_read() > 0 || !pair.state.peer_open(pair.side)
            }
            SelectMode::Write => {
                let peer_side = 1 - pair.side;
                let buffer = pair.state.buffers[peer_side].lock();
                buffer.available_write() > 0 && pair.state.peer_open(pair.side)
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

pub(super) fn socket_option_supported(level: i32, optname: i32) -> bool {
    if level == SOL_SOCKET_LEVEL {
        matches!(
            optname,
            SO_REUSEADDR_OPT
                | SO_REUSEPORT_OPT
                | SO_DONTROUTE_OPT
                | SO_BROADCAST_OPT
                | SO_KEEPALIVE_OPT
                | SO_SNDBUF_OPT
                | SO_RCVBUF_OPT
                | SO_RCVTIMEO_OPT
                | SO_SNDTIMEO_OPT
                | SO_ERROR_OPT
                | SO_TYPE_OPT
        )
    } else if level == IPPROTO_IP_LEVEL {
        matches!(
            optname,
            IP_RECVERR_OPT | MCAST_JOIN_GROUP_OPT | MCAST_LEAVE_GROUP_OPT
        )
    } else if level == posix_ctypes::IPPROTO_TCP as i32 {
        matches!(optname, TCP_NODELAY_OPT | TCP_MAXSEG_OPT)
    } else {
        false
    }
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
        Err(err) => {
            let _ = arceos_posix_api::sys_close(posix_fd);
            neg_errno(err)
        }
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
    validate_user_read(process, ptr, len)?;
    if ptr == 0 {
        return Err(LinuxError::EFAULT);
    }
    if len != size_of::<posix_ctypes::sockaddr>() {
        return Err(LinuxError::EINVAL);
    }
    read_user_bytes(process, ptr, len)
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
    let len = len.min(MAX_USER_IO_CHUNK);
    let mut local_addr: posix_ctypes::sockaddr = unsafe { core::mem::zeroed() };
    let mut local_len = 0 as posix_ctypes::socklen_t;
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

fn recv_socket_data_to_user_inner(
    process: &UserProcess,
    posix_fd: i32,
    buf: usize,
    len: usize,
    mut recv_once: impl FnMut(*mut c_void) -> isize,
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
    let ret = recv_with_real_timer_interrupt(process, posix_fd, || {
        recv_once(bytes.as_mut_ptr() as *mut c_void)
    });
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
        || !process.real_timer_armed()
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
        if current_unblocked_signal_pending() || process.consume_expired_real_timer() {
            break neg_errno(LinuxError::EINTR);
        }
        match posix_ret_usize(recv_once()) {
            Ok(n) => break n as isize,
            Err(LinuxError::EAGAIN) => {
                if current_unblocked_signal_pending() {
                    break neg_errno(LinuxError::EINTR);
                }
                if process.consume_expired_real_timer() {
                    break neg_errno(LinuxError::EINTR);
                }
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

    if domain != AF_UNIX_DOMAIN {
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
    socket_addr_call(process, fd, addr, addrlen, arceos_posix_api::sys_bind)
}

pub(super) fn sys_listen_bridge(process: &UserProcess, fd: usize, backlog: usize) -> isize {
    let socket = socket_entry_or_return!(process, fd);
    match posix_ret_i32(arceos_posix_api::sys_listen(
        socket.posix_fd,
        backlog as i32,
    )) {
        Ok(_) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_accept_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
    flags: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return neg_errno(LinuxError::EINVAL),
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
        || !process.real_timer_armed()
    {
        return posix_ret_i32(unsafe {
            arceos_posix_api::sys_accept(posix_fd, local_addr, local_len)
        });
    }

    arceos_posix_api::set_socket_recv_timeout(posix_fd, Some(INTERRUPTIBLE_SOCKET_RECV_QUANTUM))?;

    let result = loop {
        if current_unblocked_signal_pending() || process.consume_expired_real_timer() {
            break Err(LinuxError::EINTR);
        }
        match posix_ret_i32(unsafe {
            arceos_posix_api::sys_accept(posix_fd, local_addr, local_len)
        }) {
            Ok(fd) => break Ok(fd),
            Err(LinuxError::EAGAIN) => {
                if current_unblocked_signal_pending() || process.consume_expired_real_timer() {
                    break Err(LinuxError::EINTR);
                }
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

pub(super) fn sys_connect_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
    match is_local_socket_fd(process, fd) {
        Ok(true) => return sys_connect_local_socket(process, addr, addrlen),
        Ok(false) => {}
        Err(err) => return neg_errno(err),
    }
    socket_addr_call(process, fd, addr, addrlen, arceos_posix_api::sys_connect)
}

fn sys_connect_local_socket(process: &UserProcess, addr: usize, addrlen: usize) -> isize {
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
    match axfs::api::metadata(resolved_path.as_str()) {
        // No pathname AF_UNIX listener registry exists yet. Linux reports
        // ECONNREFUSED when the socket fd is valid but no peer is listening.
        Ok(_) => neg_errno(LinuxError::ECONNREFUSED),
        Err(_) => neg_errno(LinuxError::ENOENT),
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
    let ret = if addr == 0 {
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
        if let Err(err) = validate_user_write(process, addr, addr_len_value) {
            return neg_errno(err);
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

pub(super) fn sys_shutdown_bridge(process: &UserProcess, fd: usize, how: usize) -> isize {
    let socket = socket_entry_or_return!(process, fd);
    match posix_ret_i32(arceos_posix_api::sys_shutdown(socket.posix_fd, how as i32)) {
        Ok(_) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_getsockname_bridge(
    process: &UserProcess,
    fd: usize,
    addr: usize,
    addrlen: usize,
) -> isize {
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
    if level_i32 == IPPROTO_IP_LEVEL
        && matches!(optname_i32, MCAST_JOIN_GROUP_OPT | MCAST_LEAVE_GROUP_OPT)
    {
        if optval == 0 || optlen < size_of::<u32>() {
            neg_errno(LinuxError::EINVAL)
        } else {
            let mut table = process.fds.lock();
            match table.entry_mut(fd as i32) {
                Ok(FdEntry::Socket(socket)) => {
                    let mut options = socket.options.lock();
                    if optname_i32 == MCAST_JOIN_GROUP_OPT {
                        options.ip_mcast_joined = true;
                        0
                    } else if options.ip_mcast_joined {
                        options.ip_mcast_joined = false;
                        0
                    } else {
                        neg_errno(LinuxError::EADDRNOTAVAIL)
                    }
                }
                Ok(_) => neg_errno(LinuxError::ENOTSOCK),
                Err(err) => neg_errno(err),
            }
        }
    } else if !socket_option_supported(level_i32, optname_i32) {
        neg_errno(LinuxError::EINVAL)
    } else if level_i32 == SOL_SOCKET_LEVEL
        && matches!(optname_i32, SO_RCVTIMEO_OPT | SO_SNDTIMEO_OPT)
    {
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
    } else {
        0
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
    if level == posix_ctypes::IPPROTO_TCP as i32 && optname == TCP_INFO_OPT {
        if len == 0 {
            return neg_errno(LinuxError::EINVAL);
        }
        let out_len = len.min(TCP_INFO_COMPAT_SIZE);
        if let Err(err) = clear_user_bytes(process, optval, out_len) {
            return neg_errno(err);
        }
        let out_len = out_len as posix_ctypes::socklen_t;
        return write_user_value(process, optlen, &out_len);
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
            SO_SNDBUF_OPT | SO_RCVBUF_OPT => DEFAULT_SOCKET_BUFFER_SIZE,
            _ if socket_option_supported(level, optname) => 0,
            _ => return neg_errno(LinuxError::EINVAL),
        }
    } else if level == posix_ctypes::IPPROTO_TCP as i32 && socket_option_supported(level, optname) {
        match optname {
            TCP_MAXSEG_OPT => DEFAULT_TCP_MAXSEG,
            _ => 0,
        }
    } else if level == IPPROTO_IP_LEVEL && socket_option_supported(level, optname) {
        0
    } else {
        return neg_errno(LinuxError::EINVAL);
    };
    let ret = write_user_value(process, optval, &value);
    if ret != 0 {
        return ret;
    }
    let out_len = size_of::<i32>() as posix_ctypes::socklen_t;
    write_user_value(process, optlen, &out_len)
}
