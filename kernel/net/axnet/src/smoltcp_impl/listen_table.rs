use alloc::collections::{BTreeMap, VecDeque};

use axerrno::{AxError, AxResult, ax_err};
use axsync::Mutex;
use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::tcp::{self, State};
use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint};

use super::loopback::LoopbackTcpEndpoint;
use super::{LISTEN_QUEUE_SIZE, SOCKET_SET, SocketSetWrapper};

struct ListenTableEntry {
    listen_endpoint: IpListenEndpoint,
    recv_buffer_size: usize,
    send_buffer_size: usize,
    syn_queue: VecDeque<SocketHandle>,
    loopback_queue: VecDeque<LoopbackTcpEndpoint>,
}

impl ListenTableEntry {
    pub fn new(
        listen_endpoint: IpListenEndpoint,
        recv_buffer_size: usize,
        send_buffer_size: usize,
    ) -> Self {
        Self {
            listen_endpoint,
            recv_buffer_size,
            send_buffer_size,
            syn_queue: VecDeque::with_capacity(LISTEN_QUEUE_SIZE),
            loopback_queue: VecDeque::with_capacity(LISTEN_QUEUE_SIZE),
        }
    }

    #[inline]
    fn can_accept(&self, dst: IpAddress) -> bool {
        match self.listen_endpoint.addr {
            Some(addr) => addr == dst,
            None => true,
        }
    }
}

impl Drop for ListenTableEntry {
    fn drop(&mut self) {
        for endpoint in self.loopback_queue.drain(..) {
            endpoint.shutdown();
        }
        for &handle in &self.syn_queue {
            SOCKET_SET.remove(handle);
        }
    }
}

pub struct ListenTable {
    tcp: Mutex<BTreeMap<u16, ListenTableEntry>>,
}

impl ListenTable {
    pub fn new() -> Self {
        Self {
            tcp: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn can_listen(&self, port: u16) -> bool {
        !self.tcp.lock().contains_key(&port)
    }

    pub fn listen(
        &self,
        listen_endpoint: IpListenEndpoint,
        recv_buffer_size: usize,
        send_buffer_size: usize,
    ) -> AxResult {
        let port = listen_endpoint.port;
        assert_ne!(port, 0);
        let mut table = self.tcp.lock();
        if table.contains_key(&port) {
            return Err(AxError::AddrInUse);
        }
        table.insert(
            port,
            ListenTableEntry::new(listen_endpoint, recv_buffer_size, send_buffer_size),
        );
        Ok(())
    }

    pub fn unlisten(&self, port: u16) {
        debug!("TCP socket unlisten on {}", port);
        self.tcp.lock().remove(&port);
    }

    pub fn can_accept(&self, port: u16) -> AxResult<bool> {
        let table = self.tcp.lock();
        if let Some(entry) = table.get(&port) {
            Ok(!entry.loopback_queue.is_empty()
                || entry.syn_queue.iter().any(|&handle| is_connected(handle)))
        } else {
            ax_err!(InvalidInput, "socket accept() failed: not listen")
        }
    }

    pub fn connect_loopback(&self, remote: IpEndpoint, endpoint: LoopbackTcpEndpoint) -> AxResult {
        let mut table = self.tcp.lock();
        if let Some(entry) = table.get_mut(&remote.port) {
            if !entry.can_accept(remote.addr) {
                return ax_err!(ConnectionRefused, "loopback socket connect() failed");
            }
            if entry.loopback_queue.len() >= LISTEN_QUEUE_SIZE {
                return ax_err!(ConnectionRefused, "loopback socket connect() failed");
            }
            entry.loopback_queue.push_back(endpoint);
            Ok(())
        } else {
            ax_err!(ConnectionRefused, "loopback socket connect() failed")
        }
    }

    pub fn loopback_listener_buffer_sizes(&self, remote: IpEndpoint) -> AxResult<(usize, usize)> {
        let table = self.tcp.lock();
        if let Some(entry) = table.get(&remote.port) {
            if !entry.can_accept(remote.addr) {
                return ax_err!(ConnectionRefused, "loopback socket connect() failed");
            }
            Ok((entry.recv_buffer_size, entry.send_buffer_size))
        } else {
            ax_err!(ConnectionRefused, "loopback socket connect() failed")
        }
    }

    pub fn accept_loopback(&self, port: u16) -> AxResult<Option<LoopbackTcpEndpoint>> {
        let mut table = self.tcp.lock();
        if let Some(entry) = table.get_mut(&port) {
            Ok(entry.loopback_queue.pop_front())
        } else {
            ax_err!(InvalidInput, "socket accept() failed: not listen")
        }
    }

    pub fn accept(&self, port: u16) -> AxResult<(SocketHandle, (IpEndpoint, IpEndpoint))> {
        let mut table = self.tcp.lock();
        if let Some(entry) = table.get_mut(&port) {
            let syn_queue = &mut entry.syn_queue;
            let (idx, addr_tuple) = syn_queue
                .iter()
                .enumerate()
                .find_map(|(idx, &handle)| {
                    is_connected(handle).then(|| (idx, get_addr_tuple(handle)))
                })
                .ok_or(AxError::WouldBlock)?; // wait for connection
            if idx > 0 {
                warn!(
                    "slow SYN queue enumeration: index = {}, len = {}!",
                    idx,
                    syn_queue.len()
                );
            }
            let handle = syn_queue.swap_remove_front(idx).unwrap();
            Ok((handle, addr_tuple))
        } else {
            ax_err!(InvalidInput, "socket accept() failed: not listen")
        }
    }

    pub fn incoming_tcp_packet(
        &self,
        src: IpEndpoint,
        dst: IpEndpoint,
        sockets: &mut SocketSet<'_>,
    ) {
        let mut table = self.tcp.lock();
        if let Some(entry) = table.get_mut(&dst.port) {
            if !entry.can_accept(dst.addr) {
                // not listening on this address
                return;
            }
            if entry.syn_queue.len() >= LISTEN_QUEUE_SIZE {
                // SYN queue is full, drop the packet
                warn!("SYN queue overflow!");
                return;
            }
            let mut socket = SocketSetWrapper::new_tcp_socket_with_buffer_lengths(
                entry.recv_buffer_size,
                entry.send_buffer_size,
            );
            if socket.listen(entry.listen_endpoint).is_ok() {
                let handle = sockets.add(socket);
                debug!(
                    "TCP socket {}: prepare for connection {} -> {}",
                    handle, src, entry.listen_endpoint
                );
                entry.syn_queue.push_back(handle);
            }
        }
    }
}

fn is_connected(handle: SocketHandle) -> bool {
    SOCKET_SET.with_socket::<tcp::Socket, _, _>(handle, |socket| {
        !matches!(socket.state(), State::Listen | State::SynReceived)
    })
}

fn get_addr_tuple(handle: SocketHandle) -> (IpEndpoint, IpEndpoint) {
    SOCKET_SET.with_socket::<tcp::Socket, _, _>(handle, |socket| {
        (
            socket.local_endpoint().unwrap(),
            socket.remote_endpoint().unwrap(),
        )
    })
}
