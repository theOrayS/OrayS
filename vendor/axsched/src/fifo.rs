use alloc::sync::Arc;
use core::sync::atomic::{AtomicIsize, Ordering};

use linked_list_r4l::{GetLinks, Links, List};

use crate::{valid_backend_priority, BaseScheduler};

/// A task wrapper for the [`FifoScheduler`].
///
/// It adds extra states to use in [`linked_list::List`].
pub struct FifoTask<T> {
    inner: T,
    priority: AtomicIsize,
    links: Links<Self>,
}

impl<T> FifoTask<T> {
    /// Create a node.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            priority: AtomicIsize::new(0),
            links: Links::new(),
        }
    }

    fn priority(&self) -> isize {
        self.priority.load(Ordering::Acquire)
    }

    fn set_priority(&self, prio: isize) {
        self.priority.store(prio, Ordering::Release);
    }

    /// Return the reference of wrapped inner.
    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> GetLinks for FifoTask<T> {
    type EntryType = Self;

    fn get_links(t: &Self) -> &Links<Self> {
        &t.links
    }
}

impl<T> core::ops::Deref for FifoTask<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple FIFO (First-In-First-Out) cooperative scheduler.
///
/// When a task is added to the scheduler, it's placed at the end of the ready
/// queue. When picking the next task to run, the head of the ready queue is
/// taken.
///
/// As it's a cooperative scheduler, it does nothing when the timer tick occurs.
///
/// It internally uses a linked list as the ready queue.
pub struct FifoScheduler<T> {
    ready_queue: List<Arc<FifoTask<T>>>,
}

impl<T> FifoScheduler<T> {
    /// Creates a new empty [`FifoScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue: List::new(),
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "FIFO"
    }

    fn pop_highest_priority(&mut self) -> Option<Arc<FifoTask<T>>> {
        let mut cursor = self.ready_queue.cursor_front_mut();
        let mut best_priority = cursor.current()?.priority();
        cursor.move_next();
        while let Some(task) = cursor.current() {
            if task.priority() < best_priority {
                best_priority = task.priority();
            }
            cursor.move_next();
        }

        let mut cursor = self.ready_queue.cursor_front_mut();
        loop {
            match cursor.current() {
                Some(task) if task.priority() == best_priority => return cursor.remove_current(),
                Some(_) => cursor.move_next(),
                None => return None,
            }
        }
    }
}

impl<T> BaseScheduler for FifoScheduler<T> {
    type SchedItem = Arc<FifoTask<T>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        self.ready_queue.push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        unsafe { self.ready_queue.remove(task) }
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        self.pop_highest_priority()
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, _preempt: bool) {
        self.ready_queue.push_back(prev);
    }

    fn task_tick(&mut self, _current: &Self::SchedItem) -> bool {
        false // no reschedule
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

impl<T> Default for FifoScheduler<T> {
    fn default() -> Self {
        Self::new()
    }
}
