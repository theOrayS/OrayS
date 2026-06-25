//! I/O multiplexing:
//!
//! * [`select`](select::sys_select)
//! * [`epoll_create`](epoll::sys_epoll_create)
//! * [`epoll_ctl`](epoll::sys_epoll_ctl)
//! * [`epoll_wait`](epoll::sys_epoll_wait)

use core::time::Duration;

use axhal::time::wall_time;

#[cfg(feature = "epoll")]
mod epoll;
#[cfg(feature = "select")]
mod select;

#[cfg(feature = "epoll")]
pub use self::epoll::{sys_epoll_create, sys_epoll_create1, sys_epoll_ctl, sys_epoll_wait};
#[cfg(feature = "select")]
pub use self::select::sys_select;

const POLL_WAIT_BLOCK_QUANTUM: Duration = Duration::from_millis(1);
const POLL_DEADLINE_YIELD_WINDOW: Duration = Duration::from_millis(2);

fn wait_for_poll_retry(deadline: Option<Duration>) -> bool {
    match deadline {
        Some(deadline) => {
            let now = wall_time();
            if now >= deadline {
                return true;
            }
            let remaining = deadline - now;
            if remaining <= POLL_DEADLINE_YIELD_WINDOW {
                crate::sys_sched_yield();
                return wall_time() >= deadline;
            }
            let delay = POLL_WAIT_BLOCK_QUANTUM.min(remaining - POLL_DEADLINE_YIELD_WINDOW);
            #[cfg(feature = "multitask")]
            axtask::sleep(delay);
            #[cfg(not(feature = "multitask"))]
            crate::sys_sched_yield();
            wall_time() >= deadline
        }
        None => {
            #[cfg(feature = "multitask")]
            axtask::sleep(POLL_WAIT_BLOCK_QUANTUM);
            #[cfg(not(feature = "multitask"))]
            crate::sys_sched_yield();
            false
        }
    }
}
