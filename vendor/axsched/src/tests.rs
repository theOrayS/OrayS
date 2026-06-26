macro_rules! def_test_sched {
    ($name: ident, $scheduler: ty, $task: ty) => {
        mod $name {
            use crate::*;
            use alloc::sync::Arc;

            #[test]
            fn test_sched() {
                const NUM_TASKS: usize = 11;

                let mut scheduler = <$scheduler>::new();
                for i in 0..NUM_TASKS {
                    scheduler.add_task(Arc::new(<$task>::new(i)));
                }

                for i in 0..NUM_TASKS * 10 - 1 {
                    let next = scheduler.pick_next_task().unwrap();
                    assert_eq!(*next.inner(), i % NUM_TASKS);
                    // pass a tick to ensure the order of tasks
                    scheduler.task_tick(&next);
                    scheduler.put_prev_task(next, false);
                }

                let mut n = 0;
                while scheduler.pick_next_task().is_some() {
                    n += 1;
                }
                assert_eq!(n, NUM_TASKS);
            }

            #[test]
            fn bench_yield() {
                const NUM_TASKS: usize = 1_000_000;
                const COUNT: usize = NUM_TASKS * 3;

                let mut scheduler = <$scheduler>::new();
                for i in 0..NUM_TASKS {
                    scheduler.add_task(Arc::new(<$task>::new(i)));
                }

                let t0 = std::time::Instant::now();
                for _ in 0..COUNT {
                    let next = scheduler.pick_next_task().unwrap();
                    scheduler.put_prev_task(next, false);
                }
                let t1 = std::time::Instant::now();
                println!(
                    "  {}: task yield speed: {:?}/task",
                    stringify!($scheduler),
                    (t1 - t0) / (COUNT as u32)
                );
            }

            #[test]
            fn bench_remove() {
                const NUM_TASKS: usize = 10_000;

                let mut scheduler = <$scheduler>::new();
                let mut tasks = Vec::new();
                for i in 0..NUM_TASKS {
                    let t = Arc::new(<$task>::new(i));
                    tasks.push(t.clone());
                    scheduler.add_task(t);
                }

                let t0 = std::time::Instant::now();
                for i in (0..NUM_TASKS).rev() {
                    let t = scheduler.remove_task(&tasks[i]).unwrap();
                    assert_eq!(*t.inner(), i);
                }
                let t1 = std::time::Instant::now();
                println!(
                    "  {}: task remove speed: {:?}/task",
                    stringify!($scheduler),
                    (t1 - t0) / (NUM_TASKS as u32)
                );
            }
        }
    };
}

def_test_sched!(fifo, FifoScheduler::<usize>, FifoTask::<usize>);
def_test_sched!(rr, RRScheduler::<usize, 5>, RRTask::<usize, 5>);
def_test_sched!(cfs, CFScheduler::<usize>, CFSTask::<usize>);

#[test]
fn rr_preempted_task_keeps_remaining_slice_at_front() {
    use crate::{BaseScheduler, RRScheduler, RRTask};
    use alloc::sync::Arc;

    let mut scheduler = RRScheduler::<usize, 5>::new();
    let first = Arc::new(RRTask::<usize, 5>::new(0));
    let second = Arc::new(RRTask::<usize, 5>::new(1));
    scheduler.add_task(first);
    scheduler.add_task(second);

    let next = scheduler.pick_next_task().unwrap();
    assert_eq!(*next.inner(), 0);
    scheduler.task_tick(&next);
    scheduler.put_prev_task(next, true);

    let next = scheduler.pick_next_task().unwrap();
    assert_eq!(
        *next.inner(),
        0,
        "preempted RR task with remaining slice should stay ahead"
    );
}

#[test]
fn rr_realtime_priority_preempts_normal_tasks() {
    use crate::{BaseScheduler, RRScheduler, RRTask};
    use alloc::sync::Arc;

    let mut scheduler = RRScheduler::<usize, 5>::new();
    let realtime = Arc::new(RRTask::<usize, 5>::new(0));
    let normal = Arc::new(RRTask::<usize, 5>::new(1));
    assert!(scheduler.set_priority(&realtime, -120));
    assert!(scheduler.set_priority(&normal, -20));
    scheduler.add_task(normal);
    scheduler.add_task(realtime);

    for _ in 0..64 {
        let next = scheduler.pick_next_task().unwrap();
        assert_eq!(
            *next.inner(),
            0,
            "runnable RT/deadline task should outrank normal tasks"
        );
        scheduler.put_prev_task(next, false);
    }
}
