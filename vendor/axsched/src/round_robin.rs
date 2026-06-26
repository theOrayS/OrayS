use alloc::sync::Arc;
use core::ops::Deref;
use core::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

use linked_list_r4l::{GetLinks, Links, List};

use crate::{valid_backend_priority, BaseScheduler};

/// A task wrapper for the [`RRScheduler`].
///
/// It add a time slice counter to use in round-robin scheduling.
pub struct RRTask<T, const MAX_TIME_SLICE: usize> {
    inner: T,
    time_slice: AtomicIsize,
    priority: AtomicIsize,
    skipped_rounds: AtomicUsize,
    links: Links<Self>,
}

impl<T, const S: usize> RRTask<T, S> {
    /// Creates a new [`RRTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            time_slice: AtomicIsize::new(S as isize),
            priority: AtomicIsize::new(0),
            // A freshly spawned normal task should get a prompt first slice
            // even when a long-running benchmark has already filled the ready
            // queue.  This boost is still confined to the normal scheduling
            // class: it never outranks runnable RT/deadline tasks because class
            // ordering is compared before effective priority.
            skipped_rounds: AtomicUsize::new(NEW_TASK_READY_BOOST),
            links: Links::new(),
        }
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

    fn scheduling_class(&self) -> isize {
        scheduling_class(self.priority())
    }

    fn effective_priority(&self) -> isize {
        let priority = self.priority();
        if self.scheduling_class() != NORMAL_SCHEDULING_CLASS {
            return priority;
        }
        let boost = self
            .skipped_rounds
            .load(Ordering::Acquire)
            .min(NEW_TASK_READY_BOOST) as isize;
        priority.saturating_sub(boost)
    }

    fn scheduling_key(&self) -> (isize, isize) {
        (self.scheduling_class(), self.effective_priority())
    }

    fn note_skipped_round(&self) {
        let mut current = self.skipped_rounds.load(Ordering::Acquire);
        loop {
            if current >= MAX_AGING_BOOST {
                return;
            }
            match self.skipped_rounds.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return,
                Err(next) => current = next,
            }
        }
    }

    fn reset_skipped_rounds(&self) {
        self.skipped_rounds.store(0, Ordering::Release);
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T, const MAX_TIME_SLICE: usize> GetLinks for RRTask<T, MAX_TIME_SLICE> {
    type EntryType = Self;

    fn get_links(data: &Self::EntryType) -> &Links<Self::EntryType> {
        &data.links
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
    ready_queue: List<Arc<RRTask<T, MAX_TIME_SLICE>>>,
}

const MAX_AGING_BOOST: usize = 39;
const NEW_TASK_READY_BOOST: usize = MAX_AGING_BOOST * 2;
const REALTIME_BACKEND_PRIORITY_MAX: isize = -21;
const IDLE_BACKEND_PRIORITY_MIN: isize = 20;
const REALTIME_SCHEDULING_CLASS: isize = 0;
const NORMAL_SCHEDULING_CLASS: isize = 1;
const IDLE_SCHEDULING_CLASS: isize = 2;

fn scheduling_class(priority: isize) -> isize {
    if priority <= REALTIME_BACKEND_PRIORITY_MAX {
        REALTIME_SCHEDULING_CLASS
    } else if priority >= IDLE_BACKEND_PRIORITY_MIN {
        IDLE_SCHEDULING_CLASS
    } else {
        NORMAL_SCHEDULING_CLASS
    }
}

impl<T, const S: usize> RRScheduler<T, S> {
    /// Creates a new empty [`RRScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue: List::new(),
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "Round-robin"
    }

    fn pop_highest_priority(&mut self) -> Option<Arc<RRTask<T, S>>> {
        let mut cursor = self.ready_queue.cursor_front_mut();
        let mut best_key = cursor.current()?.scheduling_key();
        cursor.move_next();
        while let Some(task) = cursor.current() {
            let key = task.scheduling_key();
            if key < best_key {
                best_key = key;
            }
            cursor.move_next();
        }

        let mut cursor = self.ready_queue.cursor_front_mut();
        let mut selected = None;
        loop {
            match cursor.current() {
                Some(task) if task.scheduling_key() == best_key => {
                    selected = cursor.remove_current();
                    break;
                }
                Some(_) => cursor.move_next(),
                None => break,
            }
        }
        let selected = selected?;
        selected.reset_skipped_rounds();
        let mut cursor = self.ready_queue.cursor_front_mut();
        while let Some(task) = cursor.current() {
            task.note_skipped_round();
            cursor.move_next();
        }
        Some(selected)
    }
}

impl<T, const S: usize> BaseScheduler for RRScheduler<T, S> {
    type SchedItem = Arc<RRTask<T, S>>;

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
        // Keep priority selection in `pick_next_task()` and round-robin fairness
        // in the queue order.  Timer wakeups pass `preempt=true`; inserting them
        // at the front lets the shortest-period RT thread immediately outrun
        // equal-priority peers under stress.  Pushing runnable tasks to the back
        // still lets higher-priority tasks win by key while preserving same-class
        // rotation.
        prev.reset_time_slice();
        self.ready_queue.push_back(prev)
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
