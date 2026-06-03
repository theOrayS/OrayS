# milestone-06 ABI and behavior impact

This checkpoint adds real, generic timerslack behavior. It is not a stable-list promotion.

## User-visible syscall/proc changes

- `prctl(PR_SET_TIMERSLACK, value)` now succeeds for nonzero `value` and records the current process timer slack in nanoseconds.
- `prctl(PR_SET_TIMERSLACK, 0)` now resets the current process timer slack to its per-process default timer slack.
- `prctl(PR_GET_TIMERSLACK)` now returns the current timer slack instead of `EINVAL`.
- New processes start with current/default timerslack `50000` ns.
- Forked processes inherit both current and default timerslack from the creating thread's current timerslack value, matching the LTP `prctl08` inheritance check.
- `/proc/self/timerslack_ns` and `/proc/<pid>/timerslack_ns` now exist as synthetic proc files, expose the current timer slack as decimal text plus newline, and accept decimal writes to update current timerslack. A write of `0` resets current timerslack to the target process default.

## Error/flag/FD behavior

- Unknown `prctl` options still return `EINVAL`; existing `PR_SET_NAME`, `PR_GET_NAME`, `PR_SET_PDEATHSIG`, and `PR_GET_PDEATHSIG` behavior is unchanged.
- `/proc/*/timerslack_ns` supports normal read/write open modes and reports a proc-style regular file mode `0644` through path stat.
- Invalid non-UTF8 or non-unsigned-decimal writes to `/proc/*/timerslack_ns` return `EINVAL`.
- No signal, futex, mmap, struct layout, user pointer ABI, blacklist, or evaluator behavior changed.

## Resource/lifetime risk

- State is stored in per-process atomics and cloned during fork; no heap lifetime is attached to the proc fd beyond the path/pid entry.
- Reads of `/proc/<pid>/timerslack_ns` for a vanished non-current target return `ESRCH` through the fd read/write path; path lookup requires a live pid.
- The implementation does not model scheduler timer coalescing latency; it exposes the Linux-visible control plane needed by LTP without changing actual wakeup timing.


## UTS hostname namespace behavior

This checkpoint also changes default UTS hostname state from a per-process `Mutex<String>` copy to a shared `Arc<Mutex<String>>` carried by plain `fork()` children.

User-visible effects:

- `sethostname()` in one process that shares the default UTS namespace is visible to sibling forked processes through `gethostname()`/`uname().nodename`.
- New top-level LTP test processes still start from the default hostname `arceos` because `load_program()` creates a fresh shared hostname object for each new loaded program.
- `sethostname()` errno behavior remains unchanged: non-root callers get `EPERM`; invalid length gets `EINVAL`; null user pointer gets `EFAULT`.
- `uname()` struct layout, sysname/release/version/machine/domainname strings, file descriptors, signals, futexes, mmap, and user-pointer copy semantics are unchanged.

Boundary intentionally not changed:

- `CLONE_NEWUTS`/`unshare(CLONE_NEWUTS)` are still not implemented; `utsname03` remains a namespace-engineering blocker and is not promoted.
- The shared hostname object is only a default UTS namespace model; it does not introduce a full namespace registry or per-namespace lifetime teardown beyond the existing process `Arc` lifetime.
