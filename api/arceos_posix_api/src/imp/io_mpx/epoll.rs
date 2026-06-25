//! `epoll` implementation.
//!
//! Unsupported edge-triggered/one-shot/exclusive modes are rejected visibly.

use alloc::collections::BTreeMap;
use alloc::collections::btree_map::Entry;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::{ffi::c_int, time::Duration};

use axerrno::{LinuxError, LinuxResult};
use axhal::time::wall_time;
use axsync::Mutex;

use crate::ctypes;
use crate::imp::fd_ops::{FileLike, add_file_like, add_file_like_with_flags, get_file_like};
use crate::utils::{read_user_value, writable_user_slice};

const EPOLL_STAT_DEV: ctypes::dev_t = 0x6570_6f6c_6c;
const EPOLL_STAT_BLKSIZE: ctypes::blksize_t = 4096;

pub struct EpollInstance {
    events: Mutex<BTreeMap<usize, ctypes::epoll_event>>,
}

unsafe impl Send for ctypes::epoll_event {}
unsafe impl Sync for ctypes::epoll_event {}

fn validate_epoll_event_flags(events: u32) -> LinuxResult {
    let unsupported = events & (ctypes::EPOLLET | ctypes::EPOLLONESHOT | ctypes::EPOLLEXCLUSIVE);
    if unsupported != 0 {
        return Err(LinuxError::EOPNOTSUPP);
    }
    let supported = ctypes::EPOLLIN
        | ctypes::EPOLLPRI
        | ctypes::EPOLLOUT
        | ctypes::EPOLLRDNORM
        | ctypes::EPOLLRDBAND
        | ctypes::EPOLLWRNORM
        | ctypes::EPOLLWRBAND
        | ctypes::EPOLLERR
        | ctypes::EPOLLHUP
        | ctypes::EPOLLRDHUP;
    if events & !supported != 0 {
        return Err(LinuxError::EOPNOTSUPP);
    }
    Ok(())
}

fn epoll_create_fd_flags(flags: c_int) -> LinuxResult<c_int> {
    let flags = flags as u32;
    if flags & !(ctypes::EPOLL_CLOEXEC | ctypes::EPOLL_NONBLOCK) != 0 {
        return Err(LinuxError::EINVAL);
    }
    if flags & ctypes::EPOLL_NONBLOCK != 0 {
        return Err(LinuxError::EOPNOTSUPP);
    }
    Ok(if flags & ctypes::EPOLL_CLOEXEC != 0 {
        ctypes::FD_CLOEXEC as c_int
    } else {
        0
    })
}

impl EpollInstance {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(BTreeMap::new()),
        }
    }

    fn from_fd(fd: c_int) -> LinuxResult<Arc<Self>> {
        get_file_like(fd)?
            .into_any()
            .downcast::<EpollInstance>()
            .map_err(|_| LinuxError::EINVAL)
    }

    fn control(
        &self,
        op: usize,
        fd: usize,
        event: Option<&ctypes::epoll_event>,
    ) -> LinuxResult<usize> {
        match get_file_like(fd as c_int) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match op as u32 {
            ctypes::EPOLL_CTL_ADD => {
                let event = event.ok_or(LinuxError::EFAULT)?;
                validate_epoll_event_flags(event.events)?;
                if let Entry::Vacant(e) = self.events.lock().entry(fd) {
                    e.insert(*event);
                } else {
                    return Err(LinuxError::EEXIST);
                }
            }
            ctypes::EPOLL_CTL_MOD => {
                let event = event.ok_or(LinuxError::EFAULT)?;
                validate_epoll_event_flags(event.events)?;
                let mut events = self.events.lock();
                if let Entry::Occupied(mut ocp) = events.entry(fd) {
                    ocp.insert(*event);
                } else {
                    return Err(LinuxError::ENOENT);
                }
            }
            ctypes::EPOLL_CTL_DEL => {
                let mut events = self.events.lock();
                if let Entry::Occupied(ocp) = events.entry(fd) {
                    ocp.remove_entry();
                } else {
                    return Err(LinuxError::ENOENT);
                }
            }
            _ => {
                return Err(LinuxError::EINVAL);
            }
        }
        Ok(0)
    }

    fn poll_all(&self, events: &mut [ctypes::epoll_event]) -> LinuxResult<usize> {
        let ready_list = self.events.lock();
        let mut events_num = 0;

        for (infd, ev) in ready_list.iter() {
            match get_file_like(*infd as c_int)?.poll() {
                Err(_) => {
                    if !push_ready_event(
                        events,
                        &mut events_num,
                        ctypes::EPOLLERR | ctypes::EPOLLHUP,
                        ev.data,
                    ) {
                        return Ok(events_num);
                    }
                }
                Ok(state) => {
                    if state.readable
                        && (ev.events
                            & (ctypes::EPOLLIN
                                | ctypes::EPOLLPRI
                                | ctypes::EPOLLRDNORM
                                | ctypes::EPOLLRDBAND)
                            != 0)
                    {
                        if !push_ready_event(
                            events,
                            &mut events_num,
                            ev.events
                                & (ctypes::EPOLLIN
                                    | ctypes::EPOLLPRI
                                    | ctypes::EPOLLRDNORM
                                    | ctypes::EPOLLRDBAND),
                            ev.data,
                        ) {
                            return Ok(events_num);
                        }
                    }

                    if state.writable
                        && (ev.events
                            & (ctypes::EPOLLOUT | ctypes::EPOLLWRNORM | ctypes::EPOLLWRBAND)
                            != 0)
                    {
                        if !push_ready_event(
                            events,
                            &mut events_num,
                            ev.events
                                & (ctypes::EPOLLOUT | ctypes::EPOLLWRNORM | ctypes::EPOLLWRBAND),
                            ev.data,
                        ) {
                            return Ok(events_num);
                        }
                    }
                }
            }
        }
        Ok(events_num)
    }
}

impl FileLike for EpollInstance {
    fn read(&self, _buf: &mut [u8]) -> LinuxResult<usize> {
        Err(LinuxError::ENOSYS)
    }

    fn write(&self, _buf: &[u8]) -> LinuxResult<usize> {
        Err(LinuxError::ENOSYS)
    }

    fn stat(&self) -> LinuxResult<ctypes::stat> {
        let st_mode = 0o600u32; // rw-------
        let st_ino = (self as *const Self as usize as ctypes::ino_t).max(1);
        Ok(ctypes::stat {
            st_dev: EPOLL_STAT_DEV,
            st_ino,
            st_nlink: 1,
            st_mode,
            st_uid: 0,
            st_gid: 0,
            st_blksize: EPOLL_STAT_BLKSIZE,
            ..Default::default()
        })
    }

    fn into_any(self: Arc<Self>) -> alloc::sync::Arc<dyn core::any::Any + Send + Sync> {
        self
    }

    fn poll(&self) -> LinuxResult<axio::PollState> {
        Err(LinuxError::ENOSYS)
    }

    fn status_flags(&self) -> LinuxResult<c_int> {
        Ok(ctypes::O_RDWR as c_int)
    }

    fn set_nonblocking(&self, nonblocking: bool) -> LinuxResult {
        if nonblocking {
            Err(LinuxError::EOPNOTSUPP)
        } else {
            Ok(())
        }
    }
}

fn push_ready_event(
    events: &mut [ctypes::epoll_event],
    events_num: &mut usize,
    event_mask: u32,
    data: ctypes::epoll_data_t,
) -> bool {
    if *events_num >= events.len() {
        return false;
    }
    events[*events_num].events = event_mask;
    events[*events_num].data = data;
    *events_num += 1;
    true
}

/// Creates a new epoll instance.
///
/// It returns a file descriptor referring to the new epoll instance.
pub fn sys_epoll_create(size: c_int) -> c_int {
    debug!("sys_epoll_create <= {}", size);
    syscall_body!(sys_epoll_create, {
        if size <= 0 {
            return Err(LinuxError::EINVAL);
        }
        let epoll_instance = EpollInstance::new();
        add_file_like(Arc::new(epoll_instance))
    })
}

/// Creates a new epoll instance with Linux `epoll_create1` flags.
pub fn sys_epoll_create1(flags: c_int) -> c_int {
    debug!("sys_epoll_create1 <= {:#x}", flags);
    syscall_body!(sys_epoll_create1, {
        let fd_flags = epoll_create_fd_flags(flags)?;
        let epoll_instance = EpollInstance::new();
        add_file_like_with_flags(Arc::new(epoll_instance), fd_flags)
    })
}

/// Control interface for an epoll file descriptor
///
/// # Safety
///
/// For `EPOLL_CTL_ADD` and `EPOLL_CTL_MOD`, `event` must be valid for reads of
/// one `epoll_event`. `EPOLL_CTL_DEL` ignores `event` and may receive null.
pub unsafe fn sys_epoll_ctl(
    epfd: c_int,
    op: c_int,
    fd: c_int,
    event: *mut ctypes::epoll_event,
) -> c_int {
    debug!("sys_epoll_ctl <= epfd: {} op: {} fd: {}", epfd, op, fd);
    syscall_body!(sys_epoll_ctl, {
        let event_value = match op as u32 {
            ctypes::EPOLL_CTL_ADD | ctypes::EPOLL_CTL_MOD => {
                Some(unsafe { read_user_value(event as *const ctypes::epoll_event)? })
            }
            _ => None,
        };
        let event = event_value.as_ref();
        let ret = EpollInstance::from_fd(epfd)?.control(op as usize, fd as usize, event)? as c_int;
        Ok(ret)
    })
}

/// Waits for events on the epoll instance referred to by the file descriptor epfd.
///
/// # Safety
///
/// `events` must be valid for writes of `maxevents` `epoll_event` entries.
pub unsafe fn sys_epoll_wait(
    epfd: c_int,
    events: *mut ctypes::epoll_event,
    maxevents: c_int,
    timeout: c_int,
) -> c_int {
    debug!(
        "sys_epoll_wait <= epfd: {}, maxevents: {}, timeout: {}",
        epfd, maxevents, timeout
    );

    syscall_body!(sys_epoll_wait, {
        if maxevents <= 0 {
            return Err(LinuxError::EINVAL);
        }
        let maxevents = maxevents as usize;
        let events = unsafe { writable_user_slice(events, maxevents)? };
        let mut ready_events = Vec::new();
        ready_events
            .try_reserve_exact(maxevents)
            .map_err(|_| LinuxError::ENOMEM)?;
        ready_events.resize(maxevents, ctypes::epoll_event::default());
        let deadline =
            (!timeout.is_negative()).then(|| wall_time() + Duration::from_millis(timeout as u64));
        let epoll_instance = EpollInstance::from_fd(epfd)?;
        if timeout == 0 {
            let events_num = epoll_instance.poll_all(&mut ready_events)?;
            if events_num > 0 {
                events[..events_num].copy_from_slice(&ready_events[..events_num]);
            }
            return Ok(events_num as c_int);
        }
        loop {
            #[cfg(feature = "net")]
            axnet::poll_interfaces();
            let events_num = epoll_instance.poll_all(&mut ready_events)?;
            if events_num > 0 {
                events[..events_num].copy_from_slice(&ready_events[..events_num]);
                return Ok(events_num as c_int);
            }

            if super::wait_for_poll_retry(deadline) {
                debug!("    timeout!");
                return Ok(0);
            }
        }
    })
}
