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

### Catchable synchronous `SIGSEGV` for unmapped user faults

Files: `examples/shell/src/uspace/memory_map.rs`, `examples/shell/src/uspace/signal_abi.rs`

Unhandled user page faults now first check whether the current thread has an installed, unblocked user `SIGSEGV` handler and no signal frame/pending synchronous signal already in flight. If so, the kernel queues the existing user-signal delivery path and returns to user mode so the handler can run. If no catchable handler exists, or the signal is blocked/already pending, the existing fatal `SIGSEGV` exit-group behavior is preserved. The change is generic fault/signal handling; it does not inspect LTP case names, paths, process names, or outputs.

### File-backed mmap `SIGBUS` for pages wholly beyond EOF

Files: `examples/shell/src/uspace/memory_map.rs`, `examples/shell/src/uspace/process_lifecycle.rs`, `examples/shell/src/uspace/mod.rs`

File-backed `mmap` now tracks the byte count actually populated from the file. Pages wholly beyond EOF are kept in the VMA bookkeeping but protected with user-only/no-read/write/execute permissions. If a later user fault hits one of these tracked beyond-EOF mmap ranges, the generic page-fault path queues `SIGBUS`; otherwise it keeps the existing `SIGSEGV` behavior. The page containing EOF remains mapped and zero-filled, matching Linux's partial-page behavior. This is a generic file-backed mmap/signal repair; it does not inspect LTP case names, paths, process names, or outputs.

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


### User page faults / `SIGSEGV`

- Syscall-visible behavior affected: memory accesses to unmapped user addresses after `mmap`/`munmap` can now deliver a catchable `SIGSEGV` when the process installed a handler.
- Existing fatal behavior is preserved for default disposition, ignored/no handler, blocked signal, already-active signal frame, or already-pending synchronous signal cases.
- Error numbers, syscall numbers, struct layouts, FD semantics, futex values, mmap return values, and user-pointer layouts are unchanged by this delivery-path repair.
- Risk boundary: this relies on the existing `user_return_hook` signal-frame injection path; future changes must keep mmap/signal/wait regression subsets clean on RV and LA.

### File-backed `mmap` / `SIGBUS`

- Syscall-visible behavior affected: accesses to file-backed mapping pages wholly beyond the populated file length can now deliver catchable `SIGBUS` instead of silently reading/writing zero-filled anonymous pages.
- Existing partial-page behavior is preserved: the page containing EOF remains mapped and zero-filled. Anonymous mappings and file-backed bytes that were actually populated are not affected by the beyond-EOF `SIGBUS` marker.
- Error numbers, syscall numbers, struct layouts, FD semantics, futex values, and mmap return values are unchanged by this repair.
- Lifetime boundary: beyond-EOF `SIGBUS` ranges are cleared/split on `munmap`, replaced on overlapping `MAP_FIXED`, cleared on `exec`, and copied across `fork` with the rest of the user mappings.
- Risk boundary: future file-backed mmap changes must keep `mmap13` plus adjacent mmap/signal regression subsets clean on RV and LA.

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


### Sparse regular-file logical size/data for large holes

Files: `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/metadata.rs`, `examples/shell/src/uspace/memory_map.rs`, `examples/shell/src/uspace/process_lifecycle.rs`, `examples/shell/src/uspace/mod.rs`

The POSIX user-space layer now tracks per-path sparse logical size and sparse data extents when regular-file writes/truncates would otherwise require physically filling holes beyond the current in-memory file capacity. Reads from sparse holes return zeroes; writes beyond the physical capacity are stored as logical sparse extents; `stat`/`fstat`, `lseek(SEEK_END)`, `truncate`/`ftruncate`/`fallocate`, `pwrite`/`writev`/`sendfile`, and file-backed `mmap` preload consult the same logical view. `unlink`, `rename`, and `O_TRUNC` clear or move this sparse metadata.

User-visible behavior affected: generic large sparse regular files can now be created and reopened without setup-time `ENOSPC` when the write only materializes a small tail extent after a large hole. Error numbers, syscall numbers, flag constants, FD table layout, signal semantics, futex values, and user-pointer ABI are unchanged. `RLIMIT_FSIZE` still returns `EFBIG` before sparse growth beyond the configured limit.

Maintenance boundary: sparse metadata is currently maintained in the process-level POSIX emulation state and copied across fork/clone, not in the underlying filesystem. Future work that needs cross-process persistence must move this into a shared file/inode-level abstraction and rerun VFS/FD/mmap regressions. The implementation is generic and does not inspect LTP case names, paths, processes, or outputs.

### `O_TMPFILE` unsupported-gate behavior

Files: `examples/shell/src/uspace/fd_table.rs`

Syscall-visible behavior affected: `open`/`openat` flag handling for `O_TMPFILE`. Before this checkpoint, `O_TMPFILE` could be partially interpreted through its `O_DIRECTORY` bit and proceed into ordinary directory-open paths. The retained generic gate now rejects unsupported anonymous temporary-file creation explicitly: `O_TMPFILE|O_RDONLY` returns `EINVAL`, and `O_TMPFILE` against an existing directory returns `EOPNOTSUPP` (`ENOTSUP` as observed by LTP). Missing path candidates still propagate the ordinary missing-path error through the existing candidate selection logic.

No real anonymous inode, linkable unnamed file, or `linkat(AT_EMPTY_PATH)` materialization is implemented here. Error numbers for ordinary non-`O_TMPFILE` opens, FD allocation, close-on-exec handling, file status flags, signal delivery, futex behavior, mmap layout, struct layout, and user-pointer ABI are unchanged. The change is intentionally generic and does not inspect any LTP case name, path, process, or output.

Promotion boundary: current `openat03` RV/LA targeted summaries are deterministic and panic-free but contain `TCONF`/wrapper FAIL, so they are non-promotable. Real promotion requires a generic `O_TMPFILE` design plus deep-directory VFS stability evidence and adjacent open/link/unlink/rename regression gates on RV and LA.

## Stable-list impact

- Stable LTP list: unchanged at `606 total / 606 unique / 0 duplicate`.
- Candidate pool after this checkpoint: 8/50 for stable656 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mmap13`, `munmap01`, `openat02`, `sched_setaffinity01`).

## Behavior gaps exposed but not fixed

1. `mmap05`: RV is now parser-clean, but LA musl+glibc still do not receive the expected `SIGSEGV`; an explicit TLB-flush experiment and temporary instrumentation did not close it. Treat as a LoongArch write-protect/page-modify lane, not a generic signal-queue issue.
2. `readlinkat02`: RV clean but LA musl still fails on rerun; syscall code already rejects syscall-visible `bufsiz == 0`. Source audit found musl rewrites user `bufsize == 0` into a dummy one-byte syscall, so preserving valid direct `readlinkat(..., bufsiz=1)` truncation semantics takes priority over a kernel special case.
3. `nice04`: LTP's `nice(-10)` path expects `EPERM`, while the current `setpriority` syscall-lowering path returns `EACCES`; keep stable `setpriority02` protected before changing this boundary.
4. `clone04`: RV glibc confirms the kernel/glibc path returns `EINVAL` for a NULL stack, but RV musl is killed by SIGSEGV/TBROK before a clean wrapper PASS. No code change was made; treat it as a libc-wrapper boundary until a generic clone ABI fix can be proven without regressing clone/vfork/futex/wait behavior.
5. `kill10`: severe panic/trap in RV scout; must be isolated before broad reruns.
6. `shmat1`: long/hung run was terminated manually; SysV shm/resource lifetime needs separate investigation.
7. `openat03`: real `O_TMPFILE` support remains absent; a rejected emulation/linkat attempt panicked on RV during nested-directory creation, while the retained generic gate reports unsupported semantics as `TCONF`/wrapper FAIL without panic.

## Maintenance boundary

All code changes in this checkpoint are generic behavior fixes, not LTP case-name special cases. Future fixes must stay generic and must not hardcode LTP case names, paths, processes, or outputs. Signal/futex/mmap/SysV and filesystem-capacity changes require adjacent regression sets before any stable promotion.
