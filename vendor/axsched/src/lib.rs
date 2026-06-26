#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod cfs;
mod fifo;
mod round_robin;

#[cfg(test)]
mod tests;

extern crate alloc;

pub use cfs::{CFSTask, CFScheduler};
pub use fifo::{FifoScheduler, FifoTask};
pub use round_robin::{RRScheduler, RRTask};

/// Backend priority range used by simple priority-aware schedulers.
///
/// Values lower than Linux nice priorities are reserved for real-time/deadline
/// classes supplied by the POSIX layer.  Keeping those values in the common
/// scheduler range lets `sched_setscheduler(SCHED_FIFO/RR, ...)` have a real
/// scheduling effect instead of being reduced to a small nice adjustment.
pub(crate) const MIN_BACKEND_PRIORITY: isize = -140;
pub(crate) const MAX_BACKEND_PRIORITY: isize = 20;

pub(crate) fn valid_backend_priority(prio: isize) -> bool {
    (MIN_BACKEND_PRIORITY..=MAX_BACKEND_PRIORITY).contains(&prio)
}

/// The base scheduler trait that all schedulers should implement.
///
/// All tasks in the scheduler are considered runnable. If a task is go to
/// sleep, it should be removed from the scheduler.
pub trait BaseScheduler {
    /// Type of scheduled entities. Often a task struct.
    type SchedItem;

    /// Initializes the scheduler.
    fn init(&mut self);

    /// Adds a task to the scheduler.
    fn add_task(&mut self, task: Self::SchedItem);

    /// Removes a task by its reference from the scheduler. Returns the owned
    /// removed task with ownership if it exists.
    ///
    /// # Safety
    ///
    /// The caller should ensure that the task is in the scheduler, otherwise
    /// the behavior is undefined.
    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem>;

    /// Picks the next task to run, it will be removed from the scheduler.
    /// Returns [`None`] if there is not runnable task.
    fn pick_next_task(&mut self) -> Option<Self::SchedItem>;

    /// Puts the previous task back to the scheduler. The previous task is
    /// usually placed at the end of the ready queue, making it less likely
    /// to be re-scheduled.
    ///
    /// `preempt` indicates whether the previous task is preempted by the next
    /// task. In this case, the previous task may be placed at the front of the
    /// ready queue.
    fn put_prev_task(&mut self, prev: Self::SchedItem, preempt: bool);

    /// Advances the scheduler state at each timer tick. Returns `true` if
    /// re-scheduling is required.
    ///
    /// `current` is the current running task.
    fn task_tick(&mut self, current: &Self::SchedItem) -> bool;

    /// set priority for a task
    fn set_priority(&mut self, task: &Self::SchedItem, prio: isize) -> bool;
}
