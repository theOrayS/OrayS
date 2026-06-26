use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::ops::Deref;
use core::sync::atomic::{AtomicIsize, Ordering};

use crate::{valid_backend_priority, BaseScheduler};

/// A task wrapper for the [`RRScheduler`].
///
/// It add a time slice counter to use in round-robin scheduling.
pub struct RRTask<T, const MAX_TIME_SLICE: usize> {
    inner: T,
    time_slice: AtomicIsize,
    priority: AtomicIsize,
}

impl<T, const S: usize> RRTask<T, S> {
    /// Creates a new [`RRTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            time_slice: AtomicIsize::new(S as isize),
            priority: AtomicIsize::new(0),
        }
    }

    fn time_slice(&self) -> isize {
        self.time_slice.load(Ordering::Acquire)
    }

    fn reset_time_slice(&self) {
        self.time_slice.store(S as isize, Ordering::Release);
    }

    fn priority(&self) -> isize {
        self.priority.load(Ordering::Acquire)
    }

    fn set_priority(&self, prio: isize) {
        self.priority.store(prio, Ordering::Release);
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T, const S: usize> Deref for RRTask<T, S> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple [Round-Robin] (RR) preemptive scheduler.
///
/// It's very similar to the [`FifoScheduler`], but every task has a time slice
/// counter that is decremented each time a timer tick occurs. When the current
/// task's time slice counter reaches zero, the task is preempted and needs to
/// be rescheduled.
///
/// It internally uses a linked list as the ready queue.
///
/// [Round-Robin]: https://en.wikipedia.org/wiki/Round-robin_scheduling
/// [`FifoScheduler`]: crate::FifoScheduler
pub struct RRScheduler<T, const MAX_TIME_SLICE: usize> {
    ready_queue: VecDeque<Arc<RRTask<T, MAX_TIME_SLICE>>>,
}

impl<T, const S: usize> RRScheduler<T, S> {
    /// Creates a new empty [`RRScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "Round-robin"
    }

    fn pop_highest_priority(&mut self) -> Option<Arc<RRTask<T, S>>> {
        let mut best_priority = self.ready_queue.front()?.priority();
        for task in self.ready_queue.iter().skip(1) {
            let priority = task.priority();
            if priority < best_priority {
                best_priority = priority;
            }
        }

        let index = self
            .ready_queue
            .iter()
            .position(|task| task.priority() == best_priority)?;
        self.ready_queue.remove(index)
    }
}

impl<T, const S: usize> BaseScheduler for RRScheduler<T, S> {
    type SchedItem = Arc<RRTask<T, S>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        self.ready_queue.push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        let index = self
            .ready_queue
            .iter()
            .position(|queued| Arc::ptr_eq(queued, task))?;
        self.ready_queue.remove(index)
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        self.pop_highest_priority()
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, preempt: bool) {
        if prev.time_slice() > 0 && preempt {
            self.ready_queue.push_front(prev)
        } else {
            prev.reset_time_slice();
            self.ready_queue.push_back(prev)
        }
    }

    fn task_tick(&mut self, current: &Self::SchedItem) -> bool {
        let old_slice = current.time_slice.fetch_sub(1, Ordering::Release);
        old_slice <= 1
    }

    fn set_priority(&mut self, task: &Self::SchedItem, prio: isize) -> bool {
        if valid_backend_priority(prio) {
            task.set_priority(prio);
            true
        } else {
            false
        }
    }
}

impl<T, const S: usize> Default for RRScheduler<T, S> {
    fn default() -> Self {
        Self::new()
    }
}
