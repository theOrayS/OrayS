//! POSIX-compatible APIs for [ArceOS] modules
//!
//! [ArceOS]: https://github.com/arceos-org/arceos

#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]
#![feature(linkage)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate axlog;
extern crate axruntime;

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod utils;

mod imp;
mod signal;

/// Platform-specific constants and parameters.
pub mod config {
    pub use axconfig::*;
}

/// POSIX C types.
#[rustfmt::skip]
#[path = "./ctypes_gen.rs"]
#[allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals, clippy::upper_case_acronyms, missing_docs)]
pub mod ctypes;

pub use signal::PosixSignalIf;

pub use imp::io::{sys_read, sys_write, sys_writev};
pub use imp::resources::{sys_getrlimit, sys_setrlimit};
pub use imp::sys::sys_sysconf;
pub use imp::task::{sys_exit, sys_getpid, sys_sched_yield};
pub use imp::time::{sys_clock_gettime, sys_nanosleep};

#[cfg(feature = "fd")]
pub use imp::fd_ops::{
    fd_table_assigned_count, poll_file_like, sys_close, sys_dup, sys_dup2, sys_fcntl,
};
#[cfg(feature = "fs")]
pub use imp::fs::{
    sys_fstat, sys_getcwd, sys_lseek, sys_lstat, sys_open, sys_rename, sys_stat, sys_umask,
};
#[cfg(feature = "select")]
pub use imp::io_mpx::sys_select;
#[cfg(feature = "epoll")]
pub use imp::io_mpx::{sys_epoll_create, sys_epoll_create1, sys_epoll_ctl, sys_epoll_wait};
#[cfg(feature = "net")]
pub use imp::net::{
    force_socket_recv_buffer_size, force_socket_send_buffer_size, set_socket_recv_buffer_size,
    set_socket_recv_timeout, set_socket_reuse_addr, set_socket_send_buffer_size,
    set_socket_send_timeout, set_socket_tcp_nodelay, socket_identity, socket_recv_buffer_size,
    socket_recv_timeout, socket_reuse_addr, socket_send_buffer_size, socket_send_timeout,
    socket_tcp_max_segment_size, socket_tcp_nodelay, sys_accept, sys_bind, sys_connect,
    sys_freeaddrinfo, sys_getaddrinfo, sys_getpeername, sys_getsockname, sys_listen, sys_recv,
    sys_recvfrom, sys_send, sys_sendto, sys_shutdown, sys_socket,
};
#[cfg(feature = "pipe")]
pub use imp::pipe::sys_pipe;
#[cfg(feature = "multitask")]
pub use imp::pthread::mutex::{
    sys_pthread_mutex_init, sys_pthread_mutex_lock, sys_pthread_mutex_trylock,
    sys_pthread_mutex_unlock,
};
#[cfg(feature = "multitask")]
pub use imp::pthread::{sys_pthread_create, sys_pthread_exit, sys_pthread_join, sys_pthread_self};
