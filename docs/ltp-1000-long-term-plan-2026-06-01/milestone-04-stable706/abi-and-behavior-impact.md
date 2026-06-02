# stable706 ABI and behavior impact

## Syscall and ABI changes

- `getcpu(2)`: added `__NR_getcpu` dispatch. Optional `cpu` and `node` user pointers are individually copy-out checked and receive `0`; bad writable pointers return `EFAULT`; NULL pointers are allowed. This is a topology-minimal single-CPU model, not a case-name shim.
- `syslog(2)`: expanded action semantics for Linux-compatible empty log behavior: type 2 read with user-buffer validation, type 3/4 empty reads, type 10 size-buffer, privileged no-op actions 0/1/5/6/7, and console-level action 8. Negative lengths and NULL read buffers return `EINVAL`; non-root privileged actions return `EPERM`.
- `clock_getres(2)`: `CLOCK_REALTIME_ALARM` and `CLOCK_BOOTTIME_ALARM` are now accepted for clock validation/resolution. They map to realtime and monotonic/boottime sources respectively; no alarm delivery support is implied. The reported resolution is now `20ms`, matching the current effective timer/scheduler granularity instead of advertising unsupported 1ns precision.
- `copy_file_range(2)`: added generic file-to-file copy with flag validation, optional in/out offset pointer copy-in/copy-out, chunked transfer, current-offset advancement when offset pointers are NULL, file-size-limit honoring, and write timestamp update through recorded metadata. Nonzero flags return `EINVAL`; negative offset pointer values return `EINVAL`; bad pointers return `EFAULT`.
- `readahead(2)`: added a minimal regular-file fd validator/no-op prefetch path while preserving fd/readability/ESPIPE errors. It was not promoted because `readahead01` still has parser-visible TCONF from unsupported auxiliary fd families.

## VFS/metadata/path effects

- xattr name and value boundaries were tightened to Linux-visible limits (`name >255 -> ERANGE`, `value >65536 -> E2BIG`) instead of accepting values that later fail opaquely.
- path translation gained Linux path-length guards at dirfd and checked-path boundaries to avoid later internal overflow/panic behavior.
- regular file writes now record mtime/ctime updates in the per-process metadata overlay. This affects `stat/newfstatat/statx` visibility for files written through `write`, `pwrite`, `sendfile`, and `copy_file_range` paths; atime is preserved. The timestamp helper uses the current wall clock when it is monotonic relative to the stored timestamp and otherwise advances from the stored timestamp, avoiding long-sequence time-reset regressions without per-case hardcoding.
- `/proc/sys/kernel/printk` synthetic file now exposes `4\t4\t1\t7\n`, matching the syslog console-level tests' expected procfs contract.

## FD/socket/futex/resource effects

- Socket errno/optlen handling now returns Linux-compatible values for basic getsockname/getpeername/get/setsockopt/socketpair cases, including `ENOPROTOOPT=92` and `EOPNOTSUPP=95` constants.
- Robust-list `set_robust_list` length validation now requires the Linux ABI size (`size_of::<usize>() * 3`) instead of accepting arbitrary lengths.
- No new dependency, vendor, evaluator, runner, or testsuite changes were made.

## Risk and maintenance boundary

- `copy_file_range(2)` is a regular-file copy implementation, not reflink/dedupe/offload. It intentionally stays within existing read/write helpers and sparse-size metadata.
- `CLOCK_*_ALARM` acceptance is limited to query/getres/gettime-style validation; alarm timers remain unsupported unless implemented by a future timer lane.
- `getcpu(2)` reports a stable single-node/single-CPU view suitable for the current single-hart evaluator model; SMP topology expansion must revisit this ABI.
- The 20ms clock-resolution contract should be revisited if the scheduler/timer implementation is upgraded to reliably provide finer sleep/wakeup precision.
