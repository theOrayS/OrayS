use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use core::time::Duration;

use axerrno::{LinuxError, LinuxResult};
use axio::PollState;
use axnet::{TcpSocket, UdpSocket};
use axsync::Mutex;

use super::fd_ops::FileLike;
use crate::ctypes;
use crate::utils::{
    char_ptr_to_str, read_user_value, readable_user_buffer, writable_user_buffer, write_user_value,
};

const SHUT_RD: c_int = 0;
const SHUT_WR: c_int = 1;
const SHUT_RDWR: c_int = 2;
const MSG_OOB: c_int = 0x1;
const MSG_ERRQUEUE: c_int = 0x2000;
const MSG_NOSIGNAL: c_int = 0x4000;
const SUPPORTED_RECV_FLAGS: c_int = MSG_OOB | MSG_ERRQUEUE;
const SUPPORTED_SEND_FLAGS: c_int = MSG_OOB | MSG_NOSIGNAL;
const UDP_MAX_PAYLOAD_LEN: usize = 65_507;
const SOCKET_STAT_DEV: ctypes::dev_t = 1;
const SOCKET_STAT_BLKSIZE: ctypes::blksize_t = 4096;
const AI_PASSIVE: c_int = 0x01;
const AI_CANONNAME: c_int = 0x02;
const AI_NUMERICHOST: c_int = 0x04;
const AI_ADDRCONFIG: c_int = 0x20;
const AI_NUMERICSERV: c_int = 0x400;
const SUPPORTED_AI_FLAGS: c_int =
    AI_PASSIVE | AI_CANONNAME | AI_NUMERICHOST | AI_ADDRCONFIG | AI_NUMERICSERV;

#[derive(Clone, Copy)]
struct ResolvedAddrInfoHints {
    flags: c_int,
    socktype: c_int,
    protocol: c_int,
}

impl ResolvedAddrInfoHints {
    fn from_ptr(hints: *const ctypes::addrinfo) -> Result<Self, c_int> {
        let raw = if hints.is_null() {
            ctypes::addrinfo::default()
        } else {
            unsafe { read_user_value(hints) }.map_err(|_| ctypes::EAI_SYSTEM)?
        };
        if raw.ai_flags & !SUPPORTED_AI_FLAGS != 0 {
            return Err(ctypes::EAI_BADFLAGS);
        }
        match raw.ai_family as u32 {
            ctypes::AF_UNSPEC | ctypes::AF_INET => {}
            _ => return Err(ctypes::EAI_FAMILY),
        }
        let (socktype, protocol) =
            normalize_getaddrinfo_socktype(raw.ai_socktype, raw.ai_protocol)?;
        Ok(Self {
            flags: raw.ai_flags,
            socktype,
            protocol,
        })
    }
}

fn normalize_getaddrinfo_socktype(
    socktype: c_int,
    protocol: c_int,
) -> Result<(c_int, c_int), c_int> {
    match (socktype as u32, protocol as u32) {
        (0, 0) => Ok((ctypes::SOCK_STREAM as c_int, ctypes::IPPROTO_TCP as c_int)),
        (0, ctypes::IPPROTO_TCP) => {
            Ok((ctypes::SOCK_STREAM as c_int, ctypes::IPPROTO_TCP as c_int))
        }
        (0, ctypes::IPPROTO_UDP) => Ok((ctypes::SOCK_DGRAM as c_int, ctypes::IPPROTO_UDP as c_int)),
        (ctypes::SOCK_STREAM, 0 | ctypes::IPPROTO_TCP) => {
            Ok((ctypes::SOCK_STREAM as c_int, ctypes::IPPROTO_TCP as c_int))
        }
        (ctypes::SOCK_DGRAM, 0 | ctypes::IPPROTO_UDP) => {
            Ok((ctypes::SOCK_DGRAM as c_int, ctypes::IPPROTO_UDP as c_int))
        }
        (ctypes::SOCK_STREAM | ctypes::SOCK_DGRAM, _) => Err(ctypes::EAI_SERVICE),
        _ => Err(ctypes::EAI_SOCKTYPE),
    }
}

fn optional_cstr(ptr: *const c_char) -> Result<Option<&'static str>, c_int> {
    if ptr.is_null() {
        return Ok(None);
    }
    char_ptr_to_str(ptr).map(Some).map_err(|_| ctypes::EAI_FAIL)
}

fn parse_getaddrinfo_service(servname: Option<&str>) -> Result<u16, c_int> {
    let Some(service) = servname else {
        return Ok(0);
    };
    if service.is_empty() {
        return Ok(0);
    }
    match service.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err(ctypes::EAI_SERVICE),
    }
}

fn copy_canonname_to_aibuf(buf: &mut ctypes::aibuf, canonname: Option<&str>) {
    let Some(canonname) = canonname else {
        return;
    };
    if buf.canonname.is_empty() {
        return;
    }
    let max_len = buf.canonname.len() - 1;
    let len = core::cmp::min(canonname.len(), max_len);
    for (dst, src) in buf.canonname[..len]
        .iter_mut()
        .zip(canonname.as_bytes().iter().copied())
    {
        *dst = src as c_char;
    }
    buf.canonname[len] = 0;
    buf.ai.ai_canonname = buf.canonname.as_mut_ptr();
}

fn check_recv_flags(flag: c_int) -> LinuxResult {
    if flag & !SUPPORTED_RECV_FLAGS != 0 {
        return Err(LinuxError::EOPNOTSUPP);
    }
    if flag & MSG_OOB != 0 {
        return Err(LinuxError::EINVAL);
    }
    if flag & MSG_ERRQUEUE != 0 {
        return Err(LinuxError::EAGAIN);
    }
    Ok(())
}

fn check_send_flags(flag: c_int) -> LinuxResult {
    if flag & !SUPPORTED_SEND_FLAGS != 0 {
        return Err(LinuxError::EOPNOTSUPP);
    }
    if flag & MSG_OOB != 0 {
        return Err(LinuxError::EOPNOTSUPP);
    }
    Ok(())
}

fn map_stream_send_error(res: LinuxResult<usize>) -> LinuxResult<usize> {
    match res {
        Err(LinuxError::ENOTCONN) => Err(LinuxError::EPIPE),
        other => other,
    }
}

pub enum Socket {
    Udp(Mutex<UdpSocket>),
    Tcp(Mutex<TcpSocket>),
}

impl Socket {
    fn add_to_fd_table(self, fd_flags: c_int) -> LinuxResult<c_int> {
        super::fd_ops::add_file_like_with_flags(Arc::new(self), fd_flags)
    }

    fn from_fd(fd: c_int) -> LinuxResult<Arc<Self>> {
        let f = super::fd_ops::get_file_like(fd)?;
        f.into_any()
            .downcast::<Self>()
            .map_err(|_| LinuxError::EINVAL)
    }

    fn send(&self, buf: &[u8]) -> LinuxResult<usize> {
        match self {
            Socket::Udp(udpsocket) => {
                if buf.len() > UDP_MAX_PAYLOAD_LEN {
                    return Err(LinuxError::EMSGSIZE);
                }
                Ok(udpsocket.lock().send(buf)?)
            }
            Socket::Tcp(tcpsocket) => {
                let res: LinuxResult<usize> = tcpsocket.lock().send(buf).map_err(Into::into);
                map_stream_send_error(res)
            }
        }
    }

    fn recv(&self, buf: &mut [u8]) -> LinuxResult<usize> {
        match self {
            Socket::Udp(udpsocket) => Ok(udpsocket.lock().recv_from(buf).map(|e| e.0)?),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().recv(buf)?),
        }
    }

    pub fn poll(&self) -> LinuxResult<PollState> {
        match self {
            Socket::Udp(udpsocket) => Ok(udpsocket.lock().poll()?),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().poll()?),
        }
    }

    fn local_addr(&self) -> LinuxResult<SocketAddr> {
        match self {
            Socket::Udp(udpsocket) => Ok(udpsocket.lock().local_addr()?),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().local_addr()?),
        }
    }

    fn peer_addr(&self) -> LinuxResult<SocketAddr> {
        match self {
            Socket::Udp(udpsocket) => Ok(udpsocket.lock().peer_addr()?),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().peer_addr()?),
        }
    }

    fn bind(&self, addr: SocketAddr) -> LinuxResult {
        match self {
            Socket::Udp(udpsocket) => Ok(udpsocket.lock().bind(addr)?),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().bind(addr)?),
        }
    }

    fn connect(&self, addr: SocketAddr) -> LinuxResult {
        match self {
            Socket::Udp(udpsocket) => Ok(udpsocket.lock().connect(addr)?),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().connect(addr)?),
        }
    }

    fn sendto(&self, buf: &[u8], addr: SocketAddr) -> LinuxResult<usize> {
        match self {
            // diff: must bind before sendto
            Socket::Udp(udpsocket) => {
                if buf.len() > UDP_MAX_PAYLOAD_LEN {
                    return Err(LinuxError::EMSGSIZE);
                }
                Ok(udpsocket.lock().send_to(buf, addr)?)
            }
            Socket::Tcp(tcpsocket) => {
                let res: LinuxResult<usize> = tcpsocket.lock().send(buf).map_err(Into::into);
                map_stream_send_error(res)
            }
        }
    }

    fn recvfrom(&self, buf: &mut [u8]) -> LinuxResult<(usize, Option<SocketAddr>)> {
        match self {
            // diff: must bind before recvfrom
            Socket::Udp(udpsocket) => Ok(udpsocket
                .lock()
                .recv_from(buf)
                .map(|res| (res.0, Some(res.1)))?),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().recv(buf).map(|res| (res, None))?),
        }
    }

    fn listen(&self) -> LinuxResult {
        match self {
            Socket::Udp(_) => Err(LinuxError::EOPNOTSUPP),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().listen()?),
        }
    }

    fn accept(&self) -> LinuxResult<TcpSocket> {
        match self {
            Socket::Udp(_) => Err(LinuxError::EOPNOTSUPP),
            Socket::Tcp(tcpsocket) => Ok(tcpsocket.lock().accept()?),
        }
    }

    fn shutdown(&self, flag: c_int) -> LinuxResult {
        if !matches!(flag, SHUT_RD | SHUT_WR | SHUT_RDWR) {
            return Err(LinuxError::EINVAL);
        }
        match self {
            Socket::Udp(udpsocket) => {
                let udpsocket = udpsocket.lock();
                udpsocket.peer_addr()?;
                udpsocket.shutdown()?;
                Ok(())
            }

            Socket::Tcp(tcpsocket) => {
                let tcpsocket = tcpsocket.lock();
                tcpsocket.peer_addr()?;
                match flag {
                    SHUT_RD => tcpsocket.shutdown_read()?,
                    SHUT_WR => tcpsocket.shutdown_write()?,
                    SHUT_RDWR => tcpsocket.shutdown()?,
                    _ => unreachable!(),
                }
                Ok(())
            }
        }
    }

    fn set_recv_timeout(&self, timeout: Option<Duration>) {
        match self {
            Socket::Udp(udpsocket) => udpsocket.lock().set_recv_timeout(timeout),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().set_recv_timeout(timeout),
        }
    }

    fn recv_timeout(&self) -> Option<Duration> {
        match self {
            Socket::Udp(udpsocket) => udpsocket.lock().recv_timeout(),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().recv_timeout(),
        }
    }

    fn set_send_timeout(&self, timeout: Option<Duration>) {
        match self {
            Socket::Udp(udpsocket) => udpsocket.lock().set_send_timeout(timeout),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().set_send_timeout(timeout),
        }
    }

    fn send_timeout(&self) -> Option<Duration> {
        match self {
            Socket::Udp(udpsocket) => udpsocket.lock().send_timeout(),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().send_timeout(),
        }
    }

    fn set_recv_buffer_size(&self, size: usize) -> LinuxResult {
        match self {
            Socket::Udp(udpsocket) => udpsocket
                .lock()
                .set_recv_buffer_size(size)
                .map_err(socket_buffer_option_error),
            Socket::Tcp(tcpsocket) => tcpsocket
                .lock()
                .set_recv_buffer_size(size)
                .map_err(socket_buffer_option_error),
        }
    }

    fn recv_buffer_size(&self) -> usize {
        match self {
            Socket::Udp(udpsocket) => udpsocket.lock().recv_buffer_size(),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().recv_buffer_size(),
        }
    }

    fn set_send_buffer_size(&self, size: usize) -> LinuxResult {
        match self {
            Socket::Udp(udpsocket) => udpsocket
                .lock()
                .set_send_buffer_size(size)
                .map_err(socket_buffer_option_error),
            Socket::Tcp(tcpsocket) => tcpsocket
                .lock()
                .set_send_buffer_size(size)
                .map_err(socket_buffer_option_error),
        }
    }

    fn send_buffer_size(&self) -> usize {
        match self {
            Socket::Udp(udpsocket) => udpsocket.lock().send_buffer_size(),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().send_buffer_size(),
        }
    }
}

pub fn set_socket_recv_timeout(sockfd: c_int, timeout: Option<Duration>) -> LinuxResult {
    Socket::from_fd(sockfd)?.set_recv_timeout(timeout);
    Ok(())
}

pub fn socket_recv_timeout(sockfd: c_int) -> LinuxResult<Option<Duration>> {
    Ok(Socket::from_fd(sockfd)?.recv_timeout())
}

pub fn set_socket_send_timeout(sockfd: c_int, timeout: Option<Duration>) -> LinuxResult {
    Socket::from_fd(sockfd)?.set_send_timeout(timeout);
    Ok(())
}

pub fn socket_send_timeout(sockfd: c_int) -> LinuxResult<Option<Duration>> {
    Ok(Socket::from_fd(sockfd)?.send_timeout())
}

fn validate_socket_buffer_size(size: c_int) -> LinuxResult<usize> {
    if size < 0 {
        Err(LinuxError::EINVAL)
    } else {
        Ok(size as usize)
    }
}

fn socket_buffer_option_error(err: impl Into<LinuxError>) -> LinuxError {
    match err.into() {
        LinuxError::EOPNOTSUPP | LinuxError::ENOSYS => LinuxError::ENOPROTOOPT,
        other => other,
    }
}

pub fn set_socket_recv_buffer_size(sockfd: c_int, size: c_int) -> LinuxResult {
    Socket::from_fd(sockfd)?.set_recv_buffer_size(validate_socket_buffer_size(size)?)
}

pub fn socket_recv_buffer_size(sockfd: c_int) -> LinuxResult<c_int> {
    Ok(Socket::from_fd(sockfd)?
        .recv_buffer_size()
        .min(c_int::MAX as usize) as c_int)
}

pub fn set_socket_send_buffer_size(sockfd: c_int, size: c_int) -> LinuxResult {
    Socket::from_fd(sockfd)?.set_send_buffer_size(validate_socket_buffer_size(size)?)
}

pub fn socket_send_buffer_size(sockfd: c_int) -> LinuxResult<c_int> {
    Ok(Socket::from_fd(sockfd)?
        .send_buffer_size()
        .min(c_int::MAX as usize) as c_int)
}

impl FileLike for Socket {
    fn read(&self, buf: &mut [u8]) -> LinuxResult<usize> {
        self.recv(buf)
    }

    fn write(&self, buf: &[u8]) -> LinuxResult<usize> {
        self.send(buf)
    }

    fn stat(&self) -> LinuxResult<ctypes::stat> {
        let st_mode = 0o140000 | 0o777u32; // S_IFSOCK | rwxrwxrwx
        let inode = (self as *const Self as usize as ctypes::ino_t).max(1);
        Ok(ctypes::stat {
            st_dev: SOCKET_STAT_DEV,
            st_ino: inode,
            st_nlink: 1,
            st_mode,
            st_uid: 0,
            st_gid: 0,
            st_blksize: SOCKET_STAT_BLKSIZE,
            ..Default::default()
        })
    }

    fn into_any(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync> {
        self
    }

    fn poll(&self) -> LinuxResult<PollState> {
        self.poll()
    }

    fn status_flags(&self) -> LinuxResult<c_int> {
        let nonblock = match self {
            Socket::Udp(udpsocket) => udpsocket.lock().is_nonblocking(),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().is_nonblocking(),
        };
        let mut flags = ctypes::O_RDWR as c_int;
        if nonblock {
            flags |= ctypes::O_NONBLOCK as c_int;
        }
        Ok(flags)
    }

    fn set_nonblocking(&self, nonblock: bool) -> LinuxResult {
        match self {
            Socket::Udp(udpsocket) => udpsocket.lock().set_nonblocking(nonblock),
            Socket::Tcp(tcpsocket) => tcpsocket.lock().set_nonblocking(nonblock),
        }
        Ok(())
    }

    fn set_status_flags(&self, flags: c_int) -> LinuxResult {
        // Linux F_SETFL preserves immutable descriptor properties such as the
        // access mode and ignores creation-only bits.  For sockets the mutable
        // status bit we model is O_NONBLOCK; rejecting unrelated bits makes
        // real applications that pass back F_GETFL|O_NONBLOCK observe a bogus
        // EOPNOTSUPP and can break readiness-driven protocols.
        self.set_nonblocking(flags & ctypes::O_NONBLOCK as c_int != 0)
    }
}

impl From<SocketAddrV4> for ctypes::sockaddr_in {
    fn from(addr: SocketAddrV4) -> ctypes::sockaddr_in {
        ctypes::sockaddr_in {
            sin_family: ctypes::AF_INET as u16,
            sin_port: addr.port().to_be(),
            sin_addr: ctypes::in_addr {
                // `s_addr` is stored as BE on all machines and the array is in BE order.
                // So the native endian conversion method is used so that it's never swapped.
                s_addr: u32::from_ne_bytes(addr.ip().octets()),
            },
            sin_zero: [0; 8],
        }
    }
}

impl From<ctypes::sockaddr_in> for SocketAddrV4 {
    fn from(addr: ctypes::sockaddr_in) -> SocketAddrV4 {
        SocketAddrV4::new(
            Ipv4Addr::from(addr.sin_addr.s_addr.to_ne_bytes()),
            u16::from_be(addr.sin_port),
        )
    }
}

fn sockaddr_from_ipv4(addr: ctypes::sockaddr_in) -> ctypes::sockaddr {
    let mut sa_data = [0 as c_char; 14];
    for (dst, src) in sa_data[..2].iter_mut().zip(addr.sin_port.to_ne_bytes()) {
        *dst = src as c_char;
    }
    for (dst, src) in sa_data[2..6]
        .iter_mut()
        .zip(addr.sin_addr.s_addr.to_ne_bytes())
    {
        *dst = src as c_char;
    }
    for (dst, src) in sa_data[6..].iter_mut().zip(addr.sin_zero) {
        *dst = src as c_char;
    }
    ctypes::sockaddr {
        sa_family: addr.sin_family,
        sa_data,
    }
}

fn into_sockaddr(addr: SocketAddr) -> LinuxResult<(ctypes::sockaddr, ctypes::socklen_t)> {
    debug!("    Sockaddr: {}", addr);
    match addr {
        SocketAddr::V4(addr) => {
            let addr = ctypes::sockaddr_in::from(addr);
            Ok((
                sockaddr_from_ipv4(addr),
                size_of::<ctypes::sockaddr>() as ctypes::socklen_t,
            ))
        }
        SocketAddr::V6(_) => Err(LinuxError::EAFNOSUPPORT),
    }
}

unsafe fn read_socklen(addrlen: *const ctypes::socklen_t) -> LinuxResult<ctypes::socklen_t> {
    unsafe { read_user_value(addrlen) }
}

unsafe fn write_sockaddr_output(
    addr: *mut ctypes::sockaddr,
    addrlen: *mut ctypes::socklen_t,
    value: SocketAddr,
) -> LinuxResult<()> {
    if addr.is_null() || addrlen.is_null() {
        return Err(LinuxError::EFAULT);
    }
    let capacity = unsafe { read_socklen(addrlen)? } as usize;
    let (sockaddr, len) = into_sockaddr(value)?;
    let mut src = [0u8; size_of::<ctypes::sockaddr>()];
    src[..size_of::<ctypes::sa_family_t>()].copy_from_slice(&sockaddr.sa_family.to_ne_bytes());
    for (dst, byte) in src[size_of::<ctypes::sa_family_t>()..]
        .iter_mut()
        .zip(sockaddr.sa_data.iter().copied())
    {
        *dst = byte as u8;
    }
    let copy_len = core::cmp::min(capacity, src.len());
    let dst = unsafe { writable_user_buffer(addr.cast::<c_void>(), copy_len)? };
    dst.copy_from_slice(&src[..copy_len]);
    unsafe { write_user_value(addrlen, len)? };
    Ok(())
}

unsafe fn maybe_write_sockaddr_output(
    addr: *mut ctypes::sockaddr,
    addrlen: *mut ctypes::socklen_t,
    value: SocketAddr,
) -> LinuxResult<()> {
    if addr.is_null() {
        return Ok(());
    }
    if addrlen.is_null() {
        return Err(LinuxError::EFAULT);
    }
    unsafe { write_sockaddr_output(addr, addrlen, value) }
}

fn from_sockaddr(
    addr: *const ctypes::sockaddr,
    addrlen: ctypes::socklen_t,
) -> LinuxResult<SocketAddr> {
    if addr.is_null() {
        return Err(LinuxError::EFAULT);
    }
    if addrlen < size_of::<ctypes::sockaddr>() as ctypes::socklen_t {
        return Err(LinuxError::EINVAL);
    }

    let mid = unsafe { read_user_value(addr as *const ctypes::sockaddr_in)? };
    if mid.sin_family != ctypes::AF_INET as u16 {
        return Err(LinuxError::EAFNOSUPPORT);
    }

    let res = SocketAddr::V4(mid.into());
    debug!("    load sockaddr:{:#x} => {:?}", addr as usize, res);
    Ok(res)
}

/// Create an socket for communication.
///
/// Return the socket file descriptor.
pub fn sys_socket(domain: c_int, socktype: c_int, protocol: c_int) -> c_int {
    debug!("sys_socket <= {} {} {}", domain, socktype, protocol);
    let (domain, raw_socktype, protocol) = (domain as u32, socktype as u32, protocol as u32);
    syscall_body!(sys_socket, {
        let supported_flags = ctypes::SOCK_CLOEXEC | ctypes::SOCK_NONBLOCK;
        let sock_flags = raw_socktype & supported_flags;
        if raw_socktype & !(supported_flags | 0xf) != 0 {
            return Err(LinuxError::EINVAL);
        }
        let socktype = raw_socktype & !supported_flags;
        let fd_flags = if sock_flags & ctypes::SOCK_CLOEXEC != 0 {
            ctypes::FD_CLOEXEC as c_int
        } else {
            0
        };
        let socket = match (domain, socktype, protocol) {
            (ctypes::AF_INET, ctypes::SOCK_STREAM, ctypes::IPPROTO_TCP)
            | (ctypes::AF_INET, ctypes::SOCK_STREAM, 0) => {
                Socket::Tcp(Mutex::new(TcpSocket::new()))
            }
            (ctypes::AF_INET, ctypes::SOCK_DGRAM, ctypes::IPPROTO_UDP)
            | (ctypes::AF_INET, ctypes::SOCK_DGRAM, 0) => Socket::Udp(Mutex::new(UdpSocket::new())),
            _ => return Err(LinuxError::EINVAL),
        };
        if sock_flags & ctypes::SOCK_NONBLOCK != 0 {
            socket.set_nonblocking(true)?;
        }
        socket.add_to_fd_table(fd_flags)
    })
}

/// Bind a address to a socket.
///
/// Return 0 if success.
pub fn sys_bind(
    socket_fd: c_int,
    socket_addr: *const ctypes::sockaddr,
    addrlen: ctypes::socklen_t,
) -> c_int {
    debug!(
        "sys_bind <= {} {:#x} {}",
        socket_fd, socket_addr as usize, addrlen
    );
    syscall_body!(sys_bind, {
        let addr = from_sockaddr(socket_addr, addrlen)?;
        if !axnet::is_local_addr(addr) {
            return Err(LinuxError::EADDRNOTAVAIL);
        }
        Socket::from_fd(socket_fd)?.bind(addr)?;
        Ok(0)
    })
}

/// Connects the socket to the address specified.
///
/// Return 0 if success.
pub fn sys_connect(
    socket_fd: c_int,
    socket_addr: *const ctypes::sockaddr,
    addrlen: ctypes::socklen_t,
) -> c_int {
    debug!(
        "sys_connect <= {} {:#x} {}",
        socket_fd, socket_addr as usize, addrlen
    );
    syscall_body!(sys_connect, {
        let addr = from_sockaddr(socket_addr, addrlen)?;
        Socket::from_fd(socket_fd)?.connect(addr)?;
        Ok(0)
    })
}

/// Send a message on a socket to the address specified.
///
/// Return the number of bytes sent if success.
///
/// # Safety
///
/// `buf_ptr` must either be null with `len == 0`, or point to a readable
/// buffer of `len` bytes. If `socket_addr` is non-null, it must point to a
/// valid socket address of `addrlen` bytes.
pub unsafe fn sys_sendto(
    socket_fd: c_int,
    buf_ptr: *const c_void,
    len: ctypes::size_t,
    flag: c_int, // currently not used
    socket_addr: *const ctypes::sockaddr,
    addrlen: ctypes::socklen_t,
) -> ctypes::ssize_t {
    debug!(
        "sys_sendto <= {} {:#x} {} {} {:#x} {}",
        socket_fd, buf_ptr as usize, len, flag, socket_addr as usize, addrlen
    );
    syscall_body!(sys_sendto, {
        check_send_flags(flag)?;
        let socket = Socket::from_fd(socket_fd)?;
        let buf = unsafe { readable_user_buffer(buf_ptr, len)? };
        if matches!(socket.as_ref(), Socket::Tcp(_)) {
            return socket.send(buf);
        }
        let addr = from_sockaddr(socket_addr, addrlen)?;
        socket.sendto(buf, addr)
    })
}

/// Send a message on a socket to the address connected.
///
/// Return the number of bytes sent if success.
///
/// # Safety
///
/// `buf_ptr` must either be null with `len == 0`, or point to a readable
/// buffer of `len` bytes.
pub unsafe fn sys_send(
    socket_fd: c_int,
    buf_ptr: *const c_void,
    len: ctypes::size_t,
    flag: c_int, // currently not used
) -> ctypes::ssize_t {
    debug!(
        "sys_sendto <= {} {:#x} {} {}",
        socket_fd, buf_ptr as usize, len, flag
    );
    syscall_body!(sys_send, {
        check_send_flags(flag)?;
        let buf = unsafe { readable_user_buffer(buf_ptr, len)? };
        Socket::from_fd(socket_fd)?.send(buf)
    })
}

/// Receive a message on a socket and get its source address.
///
/// Return the number of bytes received if success.
///
/// # Safety
///
/// `buf_ptr` must either be null with `len == 0`, or point to a writable buffer
/// of `len` bytes. If `socket_addr` is non-null, `addrlen` must point to
/// writable storage for the returned peer address.
pub unsafe fn sys_recvfrom(
    socket_fd: c_int,
    buf_ptr: *mut c_void,
    len: ctypes::size_t,
    flag: c_int, // currently not used
    socket_addr: *mut ctypes::sockaddr,
    addrlen: *mut ctypes::socklen_t,
) -> ctypes::ssize_t {
    debug!(
        "sys_recvfrom <= {} {:#x} {} {} {:#x} {:#x}",
        socket_fd, buf_ptr as usize, len, flag, socket_addr as usize, addrlen as usize
    );
    syscall_body!(sys_recvfrom, {
        check_recv_flags(flag)?;
        if !socket_addr.is_null() && addrlen.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let socket = Socket::from_fd(socket_fd)?;
        let buf = unsafe { writable_user_buffer(buf_ptr, len)? };

        let res = socket.recvfrom(buf)?;
        if let Some(addr) = res.1 {
            unsafe { maybe_write_sockaddr_output(socket_addr, addrlen, addr)? };
        }
        Ok(res.0)
    })
}

/// Receive a message on a socket.
///
/// Return the number of bytes received if success.
///
/// # Safety
///
/// `buf_ptr` must either be null with `len == 0`, or point to a writable buffer
/// of `len` bytes.
pub unsafe fn sys_recv(
    socket_fd: c_int,
    buf_ptr: *mut c_void,
    len: ctypes::size_t,
    flag: c_int, // currently not used
) -> ctypes::ssize_t {
    debug!(
        "sys_recv <= {} {:#x} {} {}",
        socket_fd, buf_ptr as usize, len, flag
    );
    syscall_body!(sys_recv, {
        check_recv_flags(flag)?;
        let buf = unsafe { writable_user_buffer(buf_ptr, len)? };
        Socket::from_fd(socket_fd)?.recv(buf)
    })
}

/// Listen for connections on a socket
///
/// Return 0 if success.
pub fn sys_listen(
    socket_fd: c_int,
    backlog: c_int, // currently not used
) -> c_int {
    debug!("sys_listen <= {} {}", socket_fd, backlog);
    syscall_body!(sys_listen, debug_errors: [LinuxError::EADDRINUSE], {
        Socket::from_fd(socket_fd)?.listen()?;
        Ok(0)
    })
}

/// Accept for connections on a socket
///
/// Return file descriptor for the accepted socket if success.
pub unsafe fn sys_accept(
    socket_fd: c_int,
    socket_addr: *mut ctypes::sockaddr,
    socket_len: *mut ctypes::socklen_t,
) -> c_int {
    debug!(
        "sys_accept <= {} {:#x} {:#x}",
        socket_fd, socket_addr as usize, socket_len as usize
    );
    syscall_body!(sys_accept, {
        if !socket_addr.is_null() && socket_len.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let socket = Socket::from_fd(socket_fd)?;
        let new_socket = socket.accept()?;
        let addr = if socket_addr.is_null() {
            None
        } else {
            Some(new_socket.peer_addr()?)
        };
        let new_fd = Socket::add_to_fd_table(Socket::Tcp(Mutex::new(new_socket)), 0)?;
        if let Some(addr) = addr {
            unsafe { write_sockaddr_output(socket_addr, socket_len, addr)? };
        }
        Ok(new_fd)
    })
}

/// Shut down a full-duplex connection.
///
/// Return 0 if success.
pub fn sys_shutdown(socket_fd: c_int, flag: c_int) -> c_int {
    debug!("sys_shutdown <= {} {}", socket_fd, flag);
    syscall_body!(sys_shutdown, {
        Socket::from_fd(socket_fd)?.shutdown(flag)?;
        Ok(0)
    })
}

/// Query addresses for a domain name.
///
/// IPv4 TCP and UDP are supported.  Unsupported families, socket types,
/// protocols, flags, and non-numeric services return standard EAI_* errors
/// instead of silently producing a TCP/IPv4 answer or panicking on IPv6.
///
/// Return address number if success, 0 for no name, or a negative EAI_* value.
///
/// # Safety
///
/// `res` must be writable for one `addrinfo` pointer when non-null.
pub unsafe fn sys_getaddrinfo(
    nodename: *const c_char,
    servname: *const c_char,
    hints: *const ctypes::addrinfo,
    res: *mut *mut ctypes::addrinfo,
) -> c_int {
    let name = optional_cstr(nodename);
    let service = optional_cstr(servname);
    debug!("sys_getaddrinfo <= {:?} {:?}", name, service);
    let result = (|| -> Result<c_int, c_int> {
        if res.is_null() {
            return Err(ctypes::EAI_SYSTEM);
        }
        unsafe { write_addrinfo_result(res, core::ptr::null_mut()) }
            .map_err(|_| ctypes::EAI_SYSTEM)?;

        let hints = ResolvedAddrInfoHints::from_ptr(hints)?;
        let name = name?;
        let service = service?;
        if name.is_none() && service.is_none() {
            return Err(ctypes::EAI_NONAME);
        }
        let port = parse_getaddrinfo_service(service)?;

        let ip_addrs = match name {
            Some(domain) if domain.is_empty() => vec![Ipv4Addr::LOCALHOST.into()],
            Some(domain) => {
                if let Ok(ip) = domain.parse::<IpAddr>() {
                    vec![ip]
                } else {
                    if hints.flags & AI_NUMERICHOST != 0 {
                        return Err(ctypes::EAI_NONAME);
                    }
                    axnet::dns_query(domain).map_err(|_| ctypes::EAI_NONAME)?
                }
            }
            None if hints.flags & AI_PASSIVE != 0 => vec![Ipv4Addr::UNSPECIFIED.into()],
            None => vec![Ipv4Addr::LOCALHOST.into()],
        };

        let canonname = if hints.flags & AI_CANONNAME != 0 {
            match name {
                Some(domain) if !domain.is_empty() => Some(String::from(domain)),
                _ => ip_addrs.iter().find_map(|ip| match ip {
                    IpAddr::V4(ip) => Some(ip.to_string()),
                    IpAddr::V6(_) => None,
                }),
            }
        } else {
            None
        };

        let max_results = core::cmp::min(ip_addrs.len(), ctypes::MAXADDRS as usize);
        let mut out: Vec<ctypes::aibuf> = Vec::with_capacity(max_results);
        let mut saw_unsupported_ip = false;
        for ip in ip_addrs.into_iter().take(max_results) {
            let IpAddr::V4(ip) = ip else {
                saw_unsupported_ip = true;
                continue;
            };
            let i = out.len();
            let buf = ctypes::aibuf {
                ai: ctypes::addrinfo {
                    ai_family: ctypes::AF_INET as _,
                    ai_socktype: hints.socktype,
                    ai_protocol: hints.protocol,
                    ai_addrlen: size_of::<ctypes::sockaddr_in>() as _,
                    ai_addr: core::ptr::null_mut(),
                    ai_canonname: core::ptr::null_mut(),
                    ai_next: core::ptr::null_mut(),
                    ai_flags: hints.flags,
                },
                sa: ctypes::aibuf_sa {
                    sin: SocketAddrV4::new(ip, port).into(),
                },
                canonname: [0; 256],
                slot: i as i16,
                lock: [0],
                ref_: 0,
            };
            out.push(buf);
            out[i].ai.ai_addr =
                unsafe { core::ptr::addr_of_mut!(out[i].sa.sin) as *mut ctypes::sockaddr };
            if i == 0 {
                copy_canonname_to_aibuf(&mut out[i], canonname.as_deref());
            }
            if i > 0 {
                out[i - 1].ai.ai_next = core::ptr::addr_of_mut!(out[i].ai);
            }
        }

        if out.is_empty() {
            return Err(if saw_unsupported_ip {
                ctypes::EAI_FAMILY
            } else {
                ctypes::EAI_NONAME
            });
        }
        let len = out.len();
        out[0].ref_ = len as i16;
        unsafe { write_addrinfo_result(res, core::ptr::addr_of_mut!(out[0].ai)) }
            .map_err(|_| ctypes::EAI_SYSTEM)?;
        core::mem::forget(out); // drop in `sys_freeaddrinfo`
        Ok(len as c_int)
    })();
    match result {
        Ok(count) => count,
        Err(err) => {
            if !res.is_null() {
                let _ = unsafe { write_addrinfo_result(res, core::ptr::null_mut()) };
            }
            err
        }
    }
}

/// Write the allocated addrinfo result head back to the caller.
///
/// # Safety
///
/// `res` must be writable for one `addrinfo` pointer.
unsafe fn write_addrinfo_result(
    res: *mut *mut ctypes::addrinfo,
    value: *mut ctypes::addrinfo,
) -> LinuxResult {
    unsafe { write_user_value(res, value) }
}

/// Rebuild the leaked `aibuf` vector returned by `sys_getaddrinfo`.
///
/// # Safety
///
/// `res` must be either null or a pointer previously returned by
/// `sys_getaddrinfo` and not already freed.
unsafe fn reclaim_addrinfo_buffer(res: *mut ctypes::addrinfo) -> Option<Vec<ctypes::aibuf>> {
    if res.is_null() {
        return None;
    }

    let aibuf_ptr = res as *mut ctypes::aibuf;
    let len = unsafe { (*aibuf_ptr).ref_ as usize };
    assert!(unsafe { (*aibuf_ptr).slot == 0 });
    assert!(len > 0);
    Some(unsafe { Vec::from_raw_parts(aibuf_ptr, len, len) }) // TODO: lock
}

/// Free queried `addrinfo` struct
///
/// # Safety
///
/// `res` must be either null or a pointer previously returned by
/// `sys_getaddrinfo` and not already freed.
pub unsafe fn sys_freeaddrinfo(res: *mut ctypes::addrinfo) {
    drop(unsafe { reclaim_addrinfo_buffer(res) });
}

/// Get current address to which the socket sockfd is bound.
pub unsafe fn sys_getsockname(
    sock_fd: c_int,
    addr: *mut ctypes::sockaddr,
    addrlen: *mut ctypes::socklen_t,
) -> c_int {
    debug!(
        "sys_getsockname <= {} {:#x} {:#x}",
        sock_fd, addr as usize, addrlen as usize
    );
    syscall_body!(sys_getsockname, {
        if addr.is_null() || addrlen.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let sockaddr = Socket::from_fd(sock_fd)?.local_addr()?;
        unsafe { write_sockaddr_output(addr, addrlen, sockaddr)? };
        Ok(0)
    })
}

/// Get peer address to which the socket sockfd is connected.
pub unsafe fn sys_getpeername(
    sock_fd: c_int,
    addr: *mut ctypes::sockaddr,
    addrlen: *mut ctypes::socklen_t,
) -> c_int {
    debug!(
        "sys_getpeername <= {} {:#x} {:#x}",
        sock_fd, addr as usize, addrlen as usize
    );
    syscall_body!(sys_getpeername, {
        if addr.is_null() || addrlen.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let sockaddr = Socket::from_fd(sock_fd)?.peer_addr()?;
        unsafe { write_sockaddr_output(addr, addrlen, sockaddr)? };
        Ok(0)
    })
}
