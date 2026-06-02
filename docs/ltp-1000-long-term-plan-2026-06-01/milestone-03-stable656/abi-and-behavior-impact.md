# Milestone 03 stable656 ABI and behavior impact

This checkpoint includes three generic Linux/POSIX-visible behavior fixes and several scout-only evidence updates.

## Code changes

### `sched_setaffinity` permission path

File: `examples/shell/src/uspace/resource_sched.rs`

`sys_sched_setaffinity` now reuses the existing scheduler-target permission helper (`can_set_sched_target`) after validating that the target exists and the supplied CPU mask can run on CPU0. This aligns `sched_setaffinity(2)` with the permission behavior already used by `sched_setparam` and `sched_setscheduler`.

### Synthetic filesystem capacity reporting

File: `examples/shell/src/uspace/metadata.rs`

`generic_statfs` now clamps the synthetic free-block count to the current in-memory regular-file capacity guardrail (`MAX_IN_MEMORY_FILE_SIZE / STATFS_BLOCK_SIZE`) before computing `f_blocks`, `f_bfree`, `f_bavail`, and derived `statvfs` fields. The previous value exposed the global allocator's free pages, which could substantially overstate the amount a single temporary regular file can safely grow before the in-memory file limit returns `ENOSPC`.



### Timer-list sub-tick wakeups and periodic tick preservation

Files: `kernel/task/axtask/src/timers.rs`, `kernel/runtime/axruntime/src/lib.rs`

Timed task wakeups whose timer-list deadline falls before the next 100Hz scheduler tick now program a one-shot hardware timer for the actual deadline. The runtime timer interrupt now preserves the next periodic scheduler tick deadline when an earlier precise timer fires, so repeated sub-tick waits cannot push the scheduler tick arbitrarily far into the future. This is a generic timer/lifetime fix; it does not inspect LTP case names, process names, paths, or outputs.

### Synthetic `/proc/<pid>/stat` sleeping-state reporting

File: `examples/shell/src/uspace/synthetic_fs.rs`

`/proc/<pid>/stat` now reports process state `S` when any live thread in the process has the existing `UserTaskExt::futex_wait` marker set. This exposes futex wait blocking through the synthetic procfs state field instead of reporting such a process as always runnable. The change is generic process-state reporting; it does not inspect LTP case names, paths, or outputs.

## User-visible ABI / errno impact

### Scheduler

- Syscall affected: `sched_setaffinity`.
- New errno behavior: a non-root caller attempting to set affinity for a target whose effective test uid is different now receives `-EPERM` when the target exists and the mask is otherwise valid.
- Existing behavior preserved:
  - nonexistent / non-target PID path still returns `-ESRCH` through `is_same_sched_target`;
  - zero `cpusetsize` or null mask still returns `-EINVAL`;
  - invalid user pointer still returns the user-memory validation error;
  - masks that do not include the only supported CPU still return `-EINVAL`;
  - valid self/root-owned operations on CPU0 still return success.

### `statfs` / `fstatfs` / `statvfs`

- Syscalls / libc-visible APIs affected: `statfs`, `fstatfs`, and `statvfs` via shared `generic_statfs` data.
- Visible field behavior: reported free/available block fields are now conservative and capped by the maximum in-memory regular-file capacity rather than by total allocator free pages.
- Error numbers, syscall numbers, struct layouts, flag constants, FD semantics, signal delivery, futex behavior, mmap ABI, and user-pointer layout are unchanged by this capacity-reporting change.
- The change is intentionally generic: it does not check LTP case names or paths, and it reflects the current synthetic filesystem's per-file capacity boundary.


### `/proc/<pid>/stat`

- User-visible file affected: `/proc/<pid>/stat` field 3 process state.
- New visible behavior: a process with any live thread blocked in futex wait reports `S` until the futex wait marker is cleared.
- Existing behavior preserved: exited processes still report `Z`; child-wait/syscall-wait blocked processes still report `S`; otherwise live processes report `R`.
- Syscall numbers, errno values, FD tables, signal delivery, futex wait/wake return values, mmap ABI, and user-pointer layouts are unchanged by this reporting-only repair.


### Timed waits / sleeps / futex timeouts

- Syscall-visible behavior affected: timed futex waits, `nanosleep`, `clock_nanosleep`, and other task waits backed by `set_alarm_wakeup` can now wake at sub-10ms deadlines instead of being rounded to the next periodic scheduler tick.
- Scheduler behavior preserved: periodic 100Hz scheduler ticks remain bounded even when many precise one-shot timer interrupts occur before the next periodic tick.
- Error numbers, syscall numbers, struct layouts, FD semantics, signal masks, futex wait/wake return values, mmap ABI, and user-pointer layouts are unchanged by this timing precision repair.
- Risk boundary: timer changes can affect latency and wakeup ordering; future changes must keep timer/futex/sleep regression subsets parser-clean on RV and LA before broad promotion.

## Stable-list impact

- Stable LTP list: unchanged at `606 total / 606 unique / 0 duplicate`.
- Candidate pool after this checkpoint: 5/50 for stable656 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `sched_setaffinity01`).

## Behavior gaps exposed but not fixed

1. `mmap05` / `munmap01`: likely recoverable user page-fault signal delivery gaps.
2. `mmap13`: file-backed mapping beyond EOF does not deliver expected `SIGBUS` behavior.
3. `readlinkat02`: RV clean but LA musl still fails on rerun; syscall code already rejects syscall-visible `bufsiz == 0`. Source audit found musl rewrites user `bufsize == 0` into a dummy one-byte syscall, so preserving valid direct `readlinkat(..., bufsiz=1)` truncation semantics takes priority over a kernel special case.
4. `nice04`: LTP's `nice(-10)` path expects `EPERM`, while the current `setpriority` syscall-lowering path returns `EACCES`; keep stable `setpriority02` protected before changing this boundary.
5. `kill10`: severe panic/trap in RV scout; must be isolated before broad reruns.
6. `shmat1`: long/hung run was terminated manually; SysV shm/resource lifetime needs separate investigation.

## Maintenance boundary

All code changes in this checkpoint are generic behavior fixes, not LTP case-name special cases. Future fixes must stay generic and must not hardcode LTP case names, paths, processes, or outputs. Signal/futex/mmap/SysV and filesystem-capacity changes require adjacent regression sets before any stable promotion.
