use core::sync::atomic::{AtomicU64, Ordering};

use kernel_guard::NoOp;
use lazyinit::LazyInit;
use timer_list::{TimerEvent, TimerList};

use axhal::time::{epochoffset_nanos, set_oneshot_timer, wall_time, TimeValue, NANOS_PER_SEC};

use crate::{select_run_queue, AxTaskRef};

static TIMER_TICKET_ID: AtomicU64 = AtomicU64::new(1);

percpu_static! {
    TIMER_LIST: LazyInit<TimerList<TaskWakeupEvent>> = LazyInit::new(),
}

struct TaskWakeupEvent {
    ticket_id: u64,
    task: AxTaskRef,
}

impl TimerEvent for TaskWakeupEvent {
    fn callback(self, _now: TimeValue) {
        // Ignore the timer event if timeout was set but not triggered
        // (wake up by `WaitQueue::notify()`).
        // Judge if this timer event is still valid by checking the ticket ID.
        if self.task.timer_ticket() != self.ticket_id {
            // Timer ticket ID is not matched.
            // Just ignore this timer event and return.
            return;
        }

        // Timer ticket match.
        select_run_queue::<NoOp>(&self.task).unblock_task(self.task, true)
    }
}

fn program_next_precise_deadline(timer_list: &TimerList<TaskWakeupEvent>) {
    let Some(deadline) = timer_list.next_deadline() else {
        return;
    };
    let now = wall_time();
    if deadline <= now {
        return;
    }

    // The runtime keeps a 100Hz periodic scheduler tick.  Timed waits with a
    // deadline before the next periodic tick need a one-shot interrupt at the
    // actual timer-list deadline; otherwise short POSIX waits are rounded up to
    // the next 10ms tick.  Do not reprogram far-future timers here: those would
    // unnecessarily postpone the periodic scheduler tick.
    let periodic_interval =
        core::time::Duration::from_nanos((NANOS_PER_SEC / axconfig::TICKS_PER_SEC as u64).max(1));
    if deadline > now + periodic_interval {
        return;
    }

    let monotonic_deadline = deadline
        .as_nanos()
        .saturating_sub(epochoffset_nanos() as u128)
        .min(u64::MAX as u128) as u64;
    set_oneshot_timer(monotonic_deadline);
}

pub fn set_alarm_wakeup(deadline: TimeValue, task: AxTaskRef) {
    TIMER_LIST.with_current(|timer_list| {
        let ticket_id = TIMER_TICKET_ID.fetch_add(1, Ordering::AcqRel);
        task.set_timer_ticket(ticket_id);
        timer_list.set(deadline, TaskWakeupEvent { ticket_id, task });
        program_next_precise_deadline(timer_list);
    })
}

pub fn check_events() {
    loop {
        let now = wall_time();
        let event = unsafe {
            // Safety: IRQs are disabled at this time.
            TIMER_LIST.current_ref_mut_raw()
        }
        .expire_one(now);
        if let Some((_deadline, event)) = event {
            event.callback(now);
        } else {
            break;
        }
    }
    TIMER_LIST.with_current(|timer_list| program_next_precise_deadline(timer_list));
}

pub fn init() {
    TIMER_LIST.with_current(|timer_list| {
        timer_list.init_once(TimerList::new());
    });
}
