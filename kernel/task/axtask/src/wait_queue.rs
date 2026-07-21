use alloc::collections::{BTreeSet, VecDeque};
use alloc::sync::Arc;

use kernel_guard::{NoOp, NoPreemptIrqSave};
use kspin::{SpinNoIrq, SpinNoIrqGuard};

use crate::{AxTaskRef, CurrentTask, current_run_queue, select_run_queue};

#[inline]
fn task_ptr_key(task: &AxTaskRef) -> usize {
    Arc::as_ptr(task) as usize
}

fn remove_waiters_matching<F>(queue: &mut VecDeque<AxTaskRef>, mut should_remove: F) -> bool
where
    F: FnMut(&AxTaskRef) -> bool,
{
    let len = queue.len();
    let mut removed = false;
    for _ in 0..len {
        let Some(task) = queue.pop_front() else {
            break;
        };
        if should_remove(&task) {
            removed = true;
        } else {
            queue.push_back(task);
        }
    }
    removed
}

fn count_same_queue_requeues<F, H>(
    source: &mut VecDeque<AxTaskRef>,
    already_selected: &BTreeSet<usize>,
    requeue_count: usize,
    predicate: &mut F,
    on_requeue: &mut H,
) -> usize
where
    F: FnMut(&AxTaskRef) -> bool,
    H: FnMut(&AxTaskRef),
{
    let source_len = source.len();
    let mut retained = BTreeSet::new();
    let mut requeued_len = 0usize;
    for _ in 0..source_len {
        let Some(task) = source.pop_front() else {
            break;
        };
        let key = task_ptr_key(&task);
        if already_selected.contains(&key) || !retained.insert(key) {
            continue;
        }
        if requeued_len < requeue_count && predicate(&task) {
            on_requeue(&task);
            requeued_len = requeued_len.saturating_add(1);
        }
        // A same-address futex requeue affects and counts the waiter without
        // moving it to a different queue. Preserve its relative queue order.
        source.push_back(task);
    }
    requeued_len
}

/// A queue to store sleeping tasks.
///
/// # Examples
///
/// ```
/// use axtask::WaitQueue;
/// use core::sync::atomic::{AtomicU32, Ordering};
///
/// static VALUE: AtomicU32 = AtomicU32::new(0);
/// static WQ: WaitQueue = WaitQueue::new();
///
/// axtask::init_scheduler();
/// // spawn a new task that updates `VALUE` and notifies the main task
/// axtask::spawn(|| {
///     assert_eq!(VALUE.load(Ordering::Acquire), 0);
///     VALUE.fetch_add(1, Ordering::Release);
///     WQ.notify_one(true); // wake up the main task
/// });
///
/// WQ.wait(); // block until `notify()` is called
/// assert_eq!(VALUE.load(Ordering::Acquire), 1);
/// ```
pub struct WaitQueue {
    queue: SpinNoIrq<VecDeque<AxTaskRef>>,
}

pub(crate) type WaitQueueGuard<'a> = SpinNoIrqGuard<'a, VecDeque<AxTaskRef>>;

impl WaitQueue {
    /// Creates an empty wait queue.
    pub const fn new() -> Self {
        Self {
            queue: SpinNoIrq::new(VecDeque::new()),
        }
    }

    /// Creates an empty wait queue with space for at least `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: SpinNoIrq::new(VecDeque::with_capacity(capacity)),
        }
    }

    /// Cancel events by removing the task from the wait queue.
    /// If `from_timer_list` is true, try to remove the task from the timer list.
    fn cancel_events(&self, curr: CurrentTask, _from_timer_list: bool) {
        // A task can be wake up only one events (timer or `notify()`), remove
        // the event from another queue.
        if curr.in_wait_queue() {
            // wake up by timer (timeout).
            let mut wq = self.queue.lock();
            remove_waiters_matching(&mut wq, |t| curr.ptr_eq(t));
            curr.set_in_wait_queue(false);
        }

        // Try to cancel a timer event from timer lists and mark the current
        // ticket as expired. Timed waits that are woken by notify/signals may
        // otherwise leave future-deadline events in the timer heap; workloads
        // with many short timed waits (for example pthread condition-variable
        // timeouts) can grow that heap until later allocations fail.
        #[cfg(feature = "irq")]
        if _from_timer_list {
            crate::timers::cancel_alarm_wakeup(curr.as_task_ref());
            curr.timer_ticket_expired();
        }
    }

    /// Blocks the current task and put it into the wait queue, until other task
    /// notifies it.
    pub fn wait(&self) {
        current_run_queue::<NoPreemptIrqSave>().blocked_resched(self.queue.lock());
        self.cancel_events(crate::current(), false);
    }

    /// Blocks the current task and put it into the wait queue, until the given
    /// `condition` becomes true.
    ///
    /// Note that even other tasks notify this task, it will not wake up until
    /// the condition becomes true.
    pub fn wait_until<F>(&self, condition: F)
    where
        F: Fn() -> bool,
    {
        let curr = crate::current();
        loop {
            let mut rq = current_run_queue::<NoPreemptIrqSave>();
            let wq = self.queue.lock();
            if condition() {
                break;
            }
            rq.blocked_resched(wq);
            // Preemption may occur here.
        }
        self.cancel_events(curr, false);
    }

    /// Blocks the current task and put it into the wait queue, until other tasks
    /// notify it, or the given duration has elapsed.
    #[cfg(feature = "irq")]
    pub fn wait_timeout(&self, dur: core::time::Duration) -> bool {
        if dur.is_zero() {
            return true;
        }
        let mut rq = current_run_queue::<NoPreemptIrqSave>();
        let curr = crate::current();
        let deadline = axhal::time::wall_time() + dur;
        debug!(
            "task wait_timeout: {} deadline={:?}",
            curr.id_name(),
            deadline
        );
        crate::timers::set_alarm_wakeup(deadline, curr.clone());

        rq.blocked_resched(self.queue.lock());

        let timeout = curr.in_wait_queue(); // still in the wait queue, must have timed out

        // Always try to remove the task from the timer list.
        self.cancel_events(curr, true);
        timeout
    }

    /// Blocks the current task and put it into the wait queue, until the given
    /// `condition` becomes true, or the given duration has elapsed.
    ///
    /// Note that even other tasks notify this task, it will not wake up until
    /// the above conditions are met.
    #[cfg(feature = "irq")]
    pub fn wait_timeout_until<F>(&self, dur: core::time::Duration, condition: F) -> bool
    where
        F: Fn() -> bool,
    {
        let curr = crate::current();
        if condition() {
            return false;
        }
        if dur.is_zero() {
            return true;
        }
        let deadline = axhal::time::wall_time() + dur;
        if axhal::time::wall_time() >= deadline {
            return true;
        }
        debug!(
            "task wait_timeout: {}, deadline={:?}",
            curr.id_name(),
            deadline
        );
        crate::timers::set_alarm_wakeup(deadline, curr.clone());

        let mut timeout = true;
        loop {
            let mut rq = current_run_queue::<NoPreemptIrqSave>();
            if axhal::time::wall_time() >= deadline {
                break;
            }
            let wq = self.queue.lock();
            if condition() {
                timeout = false;
                break;
            }

            rq.blocked_resched(wq);
            // Preemption may occur here.
        }
        // Always try to remove the task from the timer list.
        self.cancel_events(curr, true);
        timeout
    }

    /// Wakes up one task in the wait queue, usually the first one.
    ///
    /// If `resched` is true, the current task will be preempted when the
    /// preemption is enabled.
    pub fn notify_one(&self, resched: bool) -> bool {
        let mut wq = self.queue.lock();
        // Timed waits can be woken by their timer before a notifier reaches the
        // queued entry. Drain such stale entries until one task actually moves
        // from Blocked to Ready.
        while let Some(task) = wq.pop_front() {
            if unblock_one_task(task, resched) {
                return true;
            }
        }
        false
    }

    /// Wakes up to `count` distinct tasks, ignoring duplicate queue entries for
    /// tasks already selected by this call.
    ///
    /// Timed waits may leave stale duplicate entries if a task is awakened and
    /// re-checks a false condition before its final timeout/cancel cleanup.  For
    /// futex wake, the syscall return value is part of the userspace protocol, so
    /// it must count distinct waiters rather than raw queue nodes.
    pub fn notify_many_unique(&self, count: usize, resched: bool) -> usize {
        if count == 0 {
            return 0;
        }

        let mut notified = BTreeSet::new();
        let mut wq = self.queue.lock();
        let mut notified_len = 0usize;
        while notified_len < count {
            let Some(task) = wq.pop_front() else {
                break;
            };
            if !notified.insert(task_ptr_key(&task)) {
                continue;
            }
            if unblock_one_task(task, resched) {
                notified_len += 1;
            }
        }
        notified_len
    }

    /// Wakes up to `count` distinct tasks that satisfy `predicate`, invoking
    /// `on_task` only for tasks that are actually moved from blocked to ready.
    pub fn notify_many_where<F, G>(
        &self,
        count: usize,
        resched: bool,
        mut predicate: F,
        mut on_task: G,
    ) -> usize
    where
        F: FnMut(&AxTaskRef) -> bool,
        G: FnMut(&AxTaskRef),
    {
        if count == 0 {
            return 0;
        }

        let mut selected = BTreeSet::new();
        let mut notified_len = 0usize;
        let mut wq = self.queue.lock();
        let mut index = 0;
        while index < wq.len() && notified_len < count {
            let Some(task) = wq.get(index).cloned() else {
                break;
            };
            let key = task_ptr_key(&task);
            if selected.contains(&key) {
                let _ = wq.remove(index);
                continue;
            }
            if !predicate(&task) {
                index += 1;
                continue;
            }
            let Some(task) = wq.remove(index) else {
                break;
            };
            selected.insert(key);
            if unblock_one_task_with(task, resched, |task| on_task(task)) {
                notified_len += 1;
            }
        }
        notified_len
    }

    /// Wakes up to `wake_count` distinct source waiters matching `predicate`
    /// and requeues up to `requeue_count` remaining distinct source waiters to
    /// `target` while holding every involved queue lock.
    ///
    /// Futex requeue users rely on the wake and requeue phases being a single
    /// handoff: a concurrent wake on the target queue must not run between
    /// removing waiters from the source and inserting them into the target.
    pub fn notify_and_requeue_where<F, G, H>(
        &self,
        wake_count: usize,
        requeue_count: usize,
        target: &WaitQueue,
        resched: bool,
        predicate: F,
        on_wake: G,
        on_requeue: H,
    ) -> (usize, usize)
    where
        F: FnMut(&AxTaskRef) -> bool,
        G: FnMut(&AxTaskRef),
        H: FnMut(&AxTaskRef),
    {
        if wake_count == 0 && requeue_count == 0 {
            return (0, 0);
        }

        match self.notify_and_requeue_where_checked(
            wake_count,
            requeue_count,
            target,
            resched,
            predicate,
            on_wake,
            on_requeue,
            || Ok::<(), core::convert::Infallible>(()),
        ) {
            Ok(result) => result,
            Err(never) => match never {},
        }
    }

    /// Checked variant of [`Self::notify_and_requeue_where`].
    ///
    /// `check` runs after every involved queue lock has been acquired and
    /// before any waiter is woken or moved. If it fails, both queues remain
    /// unchanged. Futex compare-and-requeue uses this to serialize its value
    /// comparison with waiter enqueue, wake, and requeue operations.
    pub fn notify_and_requeue_where_checked<F, G, H, C, E>(
        &self,
        wake_count: usize,
        requeue_count: usize,
        target: &WaitQueue,
        resched: bool,
        mut predicate: F,
        mut on_wake: G,
        mut on_requeue: H,
        mut check: C,
    ) -> Result<(usize, usize), E>
    where
        F: FnMut(&AxTaskRef) -> bool,
        G: FnMut(&AxTaskRef),
        H: FnMut(&AxTaskRef),
        C: FnMut() -> Result<(), E>,
    {
        let mut operate = |source: &mut VecDeque<AxTaskRef>,
                           mut destination: Option<&mut VecDeque<AxTaskRef>>|
         -> Result<(usize, usize), E> {
            check()?;
            let mut selected = BTreeSet::new();
            let mut notified_len = 0usize;
            let mut index = 0;
            while index < source.len() && notified_len < wake_count {
                let Some(task) = source.get(index).cloned() else {
                    break;
                };
                let key = task_ptr_key(&task);
                if selected.contains(&key) {
                    let _ = source.remove(index);
                    continue;
                }
                if !predicate(&task) {
                    index += 1;
                    continue;
                }
                let Some(task) = source.remove(index) else {
                    break;
                };
                selected.insert(key);
                if unblock_one_task_with(task, resched, |task| on_wake(task)) {
                    notified_len += 1;
                }
            }

            let mut requeued = BTreeSet::new();
            let requeued_len = if let Some(destination) = destination.as_deref_mut() {
                let mut requeued_len = 0usize;
                while !source.is_empty() && requeued_len < requeue_count {
                    let Some(task) = source.pop_front() else {
                        break;
                    };
                    let key = task_ptr_key(&task);
                    if selected.contains(&key) || !requeued.insert(key) {
                        continue;
                    }
                    on_requeue(&task);
                    destination.push_back(task);
                    requeued_len += 1;
                }
                requeued_len
            } else {
                count_same_queue_requeues(
                    source,
                    &selected,
                    requeue_count,
                    &mut predicate,
                    &mut on_requeue,
                )
            };
            Ok((notified_len, requeued_len))
        };

        if core::ptr::eq(self, target) {
            let mut source = self.queue.lock();
            return operate(&mut source, None);
        }

        let self_addr = core::ptr::addr_of!(self.queue) as usize;
        let target_addr = core::ptr::addr_of!(target.queue) as usize;
        if self_addr < target_addr {
            let mut source = self.queue.lock();
            let mut destination = target.queue.lock();
            operate(&mut source, Some(&mut destination))
        } else {
            let mut destination = target.queue.lock();
            let mut source = self.queue.lock();
            operate(&mut source, Some(&mut destination))
        }
    }

    /// Wakes all tasks in the wait queue.
    ///
    /// If `resched` is true, the current task will be preempted when the
    /// preemption is enabled.
    pub fn notify_all(&self, resched: bool) {
        while self.notify_one(resched) {
            // loop until the wait queue is empty
        }
    }

    /// Wake up the given task in the wait queue.
    ///
    /// If `resched` is true, the current task will be preempted when the
    /// preemption is enabled.
    pub fn notify_task(&self, resched: bool, task: &AxTaskRef) -> bool {
        let mut wq = self.queue.lock();
        if let Some(index) = wq.iter().position(|t| Arc::ptr_eq(t, task)) {
            unblock_one_task(wq.remove(index).unwrap(), resched)
        } else {
            false
        }
    }

    /// Removes `task` from this wait queue without waking it.
    ///
    /// Higher-level wait protocols use this to clean up after a task was moved
    /// to a different queue and then timed out or was interrupted before that
    /// destination queue woke it.
    pub fn remove_task(&self, task: &AxTaskRef) -> bool {
        let mut wq = self.queue.lock();
        remove_waiters_matching(&mut wq, |queued| Arc::ptr_eq(queued, task))
    }

    /// Transfers up to `count` tasks from this wait queue to another wait queue.
    ///
    /// Note: If the current wait queue contains fewer than `count` tasks, all available tasks will be moved.
    ///
    /// ## Arguments
    /// * `count` - The maximum number of tasks to be moved.
    /// * `target` - The target wait queue to which tasks will be moved.
    ///
    /// ## Returns
    /// The number of tasks actually requeued.  
    pub fn requeue(&self, count: usize, target: &WaitQueue) -> usize {
        self.requeue_with(count, target, |_| {})
    }

    /// Transfers up to `count` tasks from this wait queue to another wait
    /// queue, invoking `on_task` while each task is still owned by the requeue
    /// operation.  This lets higher-level wait protocols update per-task wait
    /// metadata atomically with the queue transfer instead of pretending a
    /// requeue was just a wake-up.
    pub fn requeue_with<F>(&self, count: usize, target: &WaitQueue, mut on_task: F) -> usize
    where
        F: FnMut(&AxTaskRef),
    {
        if core::ptr::eq(self, target) || count == 0 {
            return 0;
        }

        let mut transfer = |source: &mut VecDeque<AxTaskRef>,
                            destination: &mut VecDeque<AxTaskRef>| {
            let limit = count.min(source.len());
            let mut moved = 0;
            for _ in 0..limit {
                let Some(task) = source.pop_front() else {
                    break;
                };
                on_task(&task);
                destination.push_back(task);
                moved += 1;
            }
            moved
        };

        // Hold both queues while moving tasks so a concurrent wake on the
        // target futex cannot miss waiters between source removal and
        // destination insertion.  Lock addresses impose a stable order for
        // simultaneous cross-requeues.
        let self_addr = core::ptr::addr_of!(self.queue) as usize;
        let target_addr = core::ptr::addr_of!(target.queue) as usize;
        if self_addr < target_addr {
            let mut source = self.queue.lock();
            let mut destination = target.queue.lock();
            transfer(&mut source, &mut destination)
        } else {
            let mut destination = target.queue.lock();
            let mut source = self.queue.lock();
            transfer(&mut source, &mut destination)
        }
    }

    /// Returns the number of tasks in the wait queue.
    pub fn len(&self) -> usize {
        self.queue.lock().len()
    }

    /// Returns true if the wait queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.lock().is_empty()
    }
}

fn unblock_one_task(task: AxTaskRef, resched: bool) -> bool {
    unblock_one_task_with(task, resched, |_| {})
}

fn unblock_one_task_with<F>(task: AxTaskRef, resched: bool, on_ready: F) -> bool
where
    F: FnOnce(&AxTaskRef),
{
    // Mark task as not in wait queue.
    task.set_in_wait_queue(false);
    // Select run queue by the CPU set of the task.
    // Use `NoOp` kernel guard here because the function is called with holding the
    // lock of wait queue, where the irq and preemption are disabled.
    select_run_queue::<NoOp>(&task).unblock_task_with(task, resched, on_ready)
}
