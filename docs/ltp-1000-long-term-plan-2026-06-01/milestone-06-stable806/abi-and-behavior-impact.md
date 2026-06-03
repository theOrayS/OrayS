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

## Post-UTS blocker triage impact

The readlink/nice/statx/credential-capability triage in this documentation checkpoint made no source changes and therefore introduces no new syscall, errno, flag, FD, signal, futex, mmap, struct-layout, or user-pointer ABI behavior.

Explicit non-changes:

- `readlinkat` still returns `EINVAL` when the kernel receives `bufsiz == 0`; it does not reject legitimate `bufsiz == 1` calls just to satisfy an LA musl wrapper-specific LTP row.
- `setpriority`/`nice` priority-lowering behavior is unchanged; no wrapper- or libc-specific errno mapping was added.
- `statx`, 16-bit UID syscall compatibility, Linux capabilities, and futex behavior are unchanged by this checkpoint.

The RV VFS/FD/select scout was documentation-only. It made no syscall/errno/flag/FD/signal/futex/mmap/user-pointer changes; `select*` TCONF rows, `fcntl17*` timeouts, and VFS path errno blockers remain unchanged.


## VFS parent-symlink/rmdir errno repair impact

This source patch changes real VFS path and errno behavior; it is not a stable-list promotion.

User-visible syscall/path changes:

- `mkdirat` now resolves symlink components in the parent path before creating the final directory entry. The final new component is still created, not followed.
- `mknodat` now resolves symlink components in the parent path before creating the final node. This is a generic parent-path fix even though `mknodat02` remains blocked by environment/feature `TCONF` rows.
- `symlinkat` now resolves symlink components in the parent path of `linkpath`; the final symlink name is still not followed, preserving symlink creation semantics.
- `unlinkat(..., AT_REMOVEDIR)` / `rmdir` now resolve symlink components in the parent path before attempting directory removal.
- `rmdir(".")` and equivalent final `.` removal through `unlinkat(..., AT_REMOVEDIR)` now return `EINVAL` instead of falling through to lower-level directory removal behavior.
- Removing a path that is a process-visible mountpoint now returns `EBUSY` before attempting `directory_remove_dir`, matching the protected mountpoint boundary used by `rmdir02`.

Unchanged boundaries:

- `unlink` of non-directory final symlinks remains governed by the existing non-following final-component removal semantics.
- No FD table layout, FD_CLOEXEC, file status flag, signal, futex, mmap, user-pointer ABI, struct layout, or syscall number behavior changed.
- The patch does not hardcode LTP case names, paths, process names, or expected output. Remaining `mkdir02`, `mkdir03`, `mkdir09`, `mknod07`, `mknodat02`, `symlink03`, and `unlink09` blockers retain visible parser markers and are not counted.
