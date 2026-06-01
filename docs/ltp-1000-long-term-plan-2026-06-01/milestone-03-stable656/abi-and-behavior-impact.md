# Milestone 03 stable656 ABI and behavior impact

This checkpoint includes one generic Linux/POSIX-visible behavior fix and several scout-only evidence updates.

## Code change

File: `examples/shell/src/uspace/resource_sched.rs`

`sys_sched_setaffinity` now reuses the existing scheduler-target permission helper (`can_set_sched_target`) after validating that the target exists and the supplied CPU mask can run on CPU0. This aligns `sched_setaffinity(2)` with the permission behavior already used by `sched_setparam` and `sched_setscheduler`.

## User-visible ABI / errno impact

- Syscall affected: `sched_setaffinity`.
- New errno behavior: a non-root caller attempting to set affinity for a target whose effective test uid is different now receives `-EPERM` when the target exists and the mask is otherwise valid.
- Existing behavior preserved:
  - nonexistent / non-target PID path still returns `-ESRCH` through `is_same_sched_target`;
  - zero `cpusetsize` or null mask still returns `-EINVAL`;
  - invalid user pointer still returns the user-memory validation error;
  - masks that do not include the only supported CPU still return `-EINVAL`;
  - valid self/root-owned operations on CPU0 still return success.
- Struct layout / syscall numbers / flag constants: unchanged.
- FD, signal, futex, mmap, user-pointer layout: unchanged by the code fix.
- Stable LTP list: unchanged at `606 total / 606 unique / 0 duplicate`.

## Behavior gaps exposed but not fixed

1. `mmap05` / `munmap01`: likely recoverable user page-fault signal delivery gaps.
2. `mmap13`: file-backed mapping beyond EOF does not deliver expected `SIGBUS` behavior.
3. `futex_wait03`: futex wait timeout path does not complete within the case timeout.
4. `readlinkat02`: RV clean but LA musl still fails; likely path/readlink semantics or arch/libc interaction still needs diagnosis.
5. `kill10`: severe panic/trap in RV scout; must be isolated before broad reruns.
6. `shmat1`: long/hung run was terminated manually; SysV shm/resource lifetime needs separate investigation.

## Maintenance boundary

The `sched_setaffinity` change is a generic permission check, not an LTP case-name special case. Future fixes must stay generic and must not hardcode LTP case names, paths, processes, or outputs. Signal/futex/mmap/SysV changes require adjacent regression sets before any stable promotion.
