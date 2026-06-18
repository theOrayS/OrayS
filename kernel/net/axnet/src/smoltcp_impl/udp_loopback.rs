use alloc::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
    vec::Vec,
};
use core::net::SocketAddr;
use core::time::Duration;

use axsync::Mutex;
use smoltcp::wire::IpEndpoint;

use super::udp_packet_metadata_len;

struct UdpLoopbackState {
    queue: VecDeque<UdpLoopbackPacket>,
    queued_bytes: usize,
    byte_limit: usize,
    packet_limit: usize,
}

#[derive(Clone)]
pub struct UdpLoopbackQueue {
    state: Arc<Mutex<UdpLoopbackState>>,
}

impl UdpLoopbackQueue {
    pub fn new(recv_buffer_size: usize) -> Self {
        Self {
            state: Arc::new(Mutex::new(UdpLoopbackState {
                queue: VecDeque::new(),
                queued_bytes: 0,
                byte_limit: recv_buffer_size,
                packet_limit: udp_packet_metadata_len(recv_buffer_size),
            })),
        }
    }

    pub fn set_recv_buffer_size(&self, recv_buffer_size: usize) {
        let mut state = self.state.lock();
        state.byte_limit = recv_buffer_size;
        state.packet_limit = udp_packet_metadata_len(recv_buffer_size);
        while state.queued_bytes > state.byte_limit || state.queue.len() > state.packet_limit {
            if let Some(packet) = state.queue.pop_back() {
                state.queued_bytes = state.queued_bytes.saturating_sub(packet.data.len());
            } else {
                state.queued_bytes = 0;
                break;
            }
        }
    }

    pub fn push_from_slice(&self, data: &[u8], peer: IpEndpoint) -> bool {
        let mut state = self.state.lock();
        let queued_bytes = state.queued_bytes.saturating_add(data.len());
        if data.len() <= state.byte_limit
            && queued_bytes <= state.byte_limit
            && state.queue.len() < state.packet_limit
        {
            state.queue.push_back(UdpLoopbackPacket {
                data: data.to_vec(),
                peer,
            });
            state.queued_bytes = queued_bytes;
            true
        } else {
            false
        }
    }

    pub fn pop_matching(&self, remote: Option<IpEndpoint>) -> Option<UdpLoopbackPacket> {
        let mut state = self.state.lock();
        let pos = state
            .queue
            .iter()
            .position(|packet| remote.map_or(true, |remote| endpoint_matches(remote, packet.peer)));
        pos.and_then(|pos| state.queue.remove(pos))
            .inspect(|packet| {
                state.queued_bytes = state.queued_bytes.saturating_sub(packet.data.len());
            })
    }

    pub fn has_packet(&self) -> bool {
        !self.state.lock().queue.is_empty()
    }
}

pub struct UdpLoopbackPacket {
    pub data: Vec<u8>,
    pub peer: IpEndpoint,
}

#[derive(Clone)]
struct UdpLoopbackBinding {
    local: IpEndpoint,
    // Mirrors the owning UdpSocket's connected peer for loopback demux.
    // Keep updates scoped to UdpSocket::connect() so packets sent to a shared
    // local UDP port can be routed to the same peer-specific stream that
    // non-loopback UDP recv() would accept.
    peer: Option<IpEndpoint>,
    queue: UdpLoopbackQueue,
}

static UDP_LOOPBACK_TABLE: Mutex<BTreeMap<u16, Vec<UdpLoopbackBinding>>> =
    Mutex::new(BTreeMap::new());

pub fn is_loopback_endpoint(endpoint: IpEndpoint) -> bool {
    SocketAddr::from(endpoint).ip().is_loopback()
}

pub fn loopback_source_endpoint(local: IpEndpoint, remote: IpEndpoint) -> IpEndpoint {
    if local.addr.is_unspecified() && is_loopback_endpoint(remote) {
        IpEndpoint::from(SocketAddr::new(SocketAddr::from(remote).ip(), local.port))
    } else {
        local
    }
}

pub fn register_udp_loopback(local: IpEndpoint, queue: UdpLoopbackQueue) {
    let mut table = UDP_LOOPBACK_TABLE.lock();
    let bindings = table.entry(local.port).or_insert_with(Vec::new);
    bindings.push(UdpLoopbackBinding {
        local,
        peer: None,
        queue,
    });
}

pub fn unregister_udp_loopback(local: IpEndpoint, queue: &UdpLoopbackQueue) {
    let mut table = UDP_LOOPBACK_TABLE.lock();
    if let Some(bindings) = table.get_mut(&local.port) {
        bindings.retain(|binding| {
            binding.local != local || !Arc::ptr_eq(&binding.queue.state, &queue.state)
        });
        if bindings.is_empty() {
            table.remove(&local.port);
        }
    }
}

pub fn update_udp_loopback_peer(local: IpEndpoint, queue: &UdpLoopbackQueue, peer: IpEndpoint) {
    let mut table = UDP_LOOPBACK_TABLE.lock();
    if let Some(bindings) = table.get_mut(&local.port) {
        for binding in bindings {
            if binding.local == local && Arc::ptr_eq(&binding.queue.state, &queue.state) {
                binding.peer = Some(peer);
                return;
            }
        }
    }
}

pub fn send_udp_loopback(local: IpEndpoint, remote: IpEndpoint, buf: &[u8]) -> usize {
    let mut queue_full = false;
    {
        let table = UDP_LOOPBACK_TABLE.lock();
        if let Some(bindings) = table.get(&remote.port) {
            let peer = loopback_source_endpoint(local, remote);
            let has_connected_match = bindings.iter().any(|binding| {
                binding_accepts(binding.local, remote)
                    && binding.peer.is_some_and(|p| endpoint_matches(p, peer))
            });
            for binding in bindings {
                if binding_accepts(binding.local, remote)
                    && binding_peer_accepts(binding.peer, peer, has_connected_match)
                    && !binding.queue.push_from_slice(buf, peer)
                {
                    queue_full = true;
                }
            }
        }
    }
    if queue_full {
        // A blocking UDP sender may legally outpace a loopback receiver.  Dropping
        // datagrams keeps UDP semantics, but a hot sender that finds the in-kernel
        // loopback queue full must still give the receiver/server a scheduling
        // window instead of monopolising the cooperative run queue.
        axtask::sleep(Duration::from_millis(1));
    }
    buf.len()
}

fn binding_accepts(local: IpEndpoint, remote: IpEndpoint) -> bool {
    local.port == remote.port && (local.addr.is_unspecified() || local.addr == remote.addr)
}

fn binding_peer_accepts(
    expected: Option<IpEndpoint>,
    actual: IpEndpoint,
    prefer_connected: bool,
) -> bool {
    match expected {
        Some(expected) => endpoint_matches(expected, actual),
        None => !prefer_connected,
    }
}

fn endpoint_matches(expected: IpEndpoint, actual: IpEndpoint) -> bool {
    (expected.addr.is_unspecified() || expected.addr == actual.addr)
        && (expected.port == 0 || expected.port == actual.port)
}
