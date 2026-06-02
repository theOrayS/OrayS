# Milestone 03 stable656 ABI and behavior impact

This checkpoint includes generic Linux/POSIX-visible behavior fixes and several scout-only evidence updates.

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

`/proc/<pid>/stat` now reports process state `S` when any live thread in the process has a futex wait marker, an `rt_sigsuspend` signal-wait marker, or a `poll`/`ppoll` wait marker set. This exposes common blocking waits through the synthetic procfs state field instead of reporting such a process as always runnable. The change is generic process-state reporting; it does not inspect LTP case names, paths, or outputs.


### Signal and poll wait proc-state reporting

Files: `examples/shell/src/uspace/task_context.rs`, `examples/shell/src/uspace/signal_abi.rs`, `examples/shell/src/uspace/select_fdset.rs`, `examples/shell/src/uspace/synthetic_fs.rs`

`UserTaskExt` now carries explicit wait markers for `rt_sigsuspend` and `poll`/`ppoll` loops. The signal and poll syscall paths set these markers only while the current thread is actually yielding in the blocking wait. Synthetic `/proc/<pid>/stat` consults the live thread markers and reports state `S` for those waiters.

User-visible behavior affected: `/proc/<pid>/stat` field 3 process state is more faithful for signal-waiting or poll-waiting processes, which lets generic wait-for-sleep probes observe blocked children. Syscall numbers, errno values, signal action semantics, signal masks, FD readiness results, poll return values, futex wait/wake return values, mmap ABI, and user-pointer layouts are unchanged.

Maintenance boundary: these markers are reporting-only and must not be used as a scheduling primitive. Future changes to signal or poll wait loops should preserve marker clear-on-return and rerun signal/poll/proc regression subsets on RV and LA.

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
- New visible behavior: a process with any live thread blocked in futex wait, `rt_sigsuspend`, or `poll`/`ppoll` wait reports `S` until the corresponding wait marker is cleared.
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


### `mincore` residency and `mlock` prefault behavior

Files: `examples/shell/src/uspace/memory_map.rs`, `examples/shell/src/uspace/syscall_dispatch.rs`

`mincore(2)` now separates address-range validity from residency. Pages inside an existing VMA are valid even when the lazy page fault path has not populated a PTE yet; those pages report residency byte `0` unless a PTE or shared mapping metadata exists. Pages outside every VMA still return `ENOMEM`. This preserves the Linux-visible distinction between an unmapped range and a mapped but non-resident anonymous range.

`mlock(2)` now validates that the requested range is inside mapped VMAs, rounds to page boundaries, and prefaults the range through the existing `populate_range` path. This makes subsequent `mincore` residency checks observe the locked pages as resident. `munlock`, `mlockall`, and `munlockall` remain permissive no-ops in this checkpoint; `mlock2(flags=0)` follows `mlock`, while nonzero `mlock2` flags retain the previous permissive no-op behavior.

User-visible behavior affected: valid lazy anonymous mappings are no longer misreported as `ENOMEM` by `mincore`, and successful `mlock` materializes pages in mapped ranges. Error numbers for unmapped/overflow/out-of-user-range `mincore`/`mlock` inputs remain `ENOMEM`; syscall numbers, struct layouts, FD tables, signal masks, futex values, file-backed mmap ABI, and user-pointer layouts are unchanged.

Maintenance boundary: this is a minimal residency/prefault implementation, not full Linux memory-lock accounting. Future changes to lazy fault, `mlock`, `mlock2`, or VMA bookkeeping must rerun `mincore03` plus the adjacent mincore/mlock/mmap regression subset on RV and LA.

## Stable-list impact

- Stable LTP list: unchanged at `606 total / 606 unique / 0 duplicate`.
- Candidate pool after this checkpoint: 21/50 for stable656 (`fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename05`, `sched_setaffinity01`, `signal01`, `statfs01`, `statvfs01`).

## Behavior gaps exposed but not fixed

1. `mmap05`: RV is now parser-clean, but LA musl+glibc still do not receive the expected `SIGSEGV`; an explicit TLB-flush experiment and temporary instrumentation did not close it. Treat as a LoongArch write-protect/page-modify lane, not a generic signal-queue issue.
2. `readlinkat02`: RV clean but LA musl still fails on rerun; syscall code already rejects syscall-visible `bufsiz == 0`. Source audit found musl rewrites user `bufsize == 0` into a dummy one-byte syscall, so preserving valid direct `readlinkat(..., bufsiz=1)` truncation semantics takes priority over a kernel special case.
3. `nice04`: LTP's `nice(-10)` path expects `EPERM`, while the current `setpriority` syscall-lowering path returns `EACCES`; keep stable `setpriority02` protected before changing this boundary.
4. `clone04`: RV glibc confirms the kernel/glibc path returns `EINVAL` for a NULL stack, but RV musl is killed by SIGSEGV/TBROK before a clean wrapper PASS. No code change was made; treat it as a libc-wrapper boundary until a generic clone ABI fix can be proven without regressing clone/vfork/futex/wait behavior.
5. `kill10`: severe blocker is now isolated by singleton RV evidence: musl timeout, persistent post-cleanup frame leak, and following glibc allocator panic. A temporary `poll`/`ppoll` exit-group cleanup hypothesis was rejected and removed, so this checkpoint introduces no retained syscall/errno/flag/ABI change for `kill10`.
6. `shmat1`: long/hung run was terminated manually; SysV shm/resource lifetime needs separate investigation.
7. `openat03`: real `O_TMPFILE` support remains absent; a rejected emulation/linkat attempt panicked on RV during nested-directory creation, while the retained generic gate reports unsupported semantics as `TCONF`/wrapper FAIL without panic.

## Maintenance boundary

All code changes in this checkpoint are generic behavior fixes, not LTP case-name special cases. Future fixes must stay generic and must not hardcode LTP case names, paths, processes, or outputs. Signal/futex/mmap/SysV and filesystem-capacity changes require adjacent regression sets before any stable promotion.


## `kill10` rejected cleanup hypothesis

A local temporary change made `poll`/`ppoll` wait loops observe `pending_exit_group()`, but RV singleton evidence stayed unchanged: musl still timed out, cleanup still left roughly half the free frames unreclaimed, and the following glibc group still hit allocator panic. The change was removed. Therefore the visible ABI/POSIX surface for this checkpoint is documentation-only: `kill10` remains a cleanup/resource-lifetime blocker, with no retained syscall number, errno, flag, FD, signal, futex, mmap, or user-pointer semantic change.

## `epoll_create02` documentation-only boundary

No epoll source change is retained in this checkpoint. The singleton rescout documents existing behavior only: RV musl `epoll_create(0/-1)` still reaches an ENOSYS-returning old-ABI/libc-wrapper path, while LA wrapper-PASSes but emits old `__NR_epoll_create` `TCONF` rows.

Visible ABI/POSIX impact: none from this documentation update. Syscall numbers, errno behavior, FD table semantics, epoll readiness behavior, signal/futex/mmap/user-pointer layout, and resource lifetime are unchanged. A future fix must be generic epoll compatibility work, not an LTP-name/path/output special case, and must not hide parser-visible `TCONF` rows as promotion evidence.


## G009 mm/mincore/mprotect evidence-only impact

No source change is retained for the latest clean4 confirmation. The new evidence relies on already-documented generic memory behavior:

- `mincore02` and `mincore04` are covered by the lazy-VMA validity/residency and `mlock` prefault behavior documented above.
- `mprotect02` and `mprotect04` exercise existing user protection and signal/fault behavior; this checkpoint only proves their RV + LA x musl + glibc parser-clean status.

Visible ABI/POSIX impact from this documentation/evidence update: none. Syscall numbers, errno behavior, flag constants, FD semantics, signal/futex/mmap/user-pointer layout, and resource lifetime are unchanged by this checkpoint. Future fixes for the blocked G009 mlock/mmap/mprotect rows must remain generic and prove adjacent regressions before promotion.

## `statfs01` family evidence-only impact

No source change is retained for the `statfs01,fstatfs01,fstatfs01_64,statvfs01` RV scout. The new artifact records current setup behavior only: LTP cannot acquire a free device on RV and emits parser-visible `TBROK` for both musl and glibc.

Visible ABI/POSIX impact from this documentation update: none. Syscall numbers, errno behavior, `statfs`/`statvfs` struct layout, FD semantics, block-device behavior, signal/futex/mmap/user-pointer layout, and resource lifetime are unchanged by this checkpoint.

Future work must implement or expose a generic device-acquisition model if these tests are to reach their statfs assertions. It must not hardcode LTP case names, paths, device names, or outputs, and it must rerun adjacent statfs/fstatfs/statvfs plus mount/device setup regressions before promotion.

## VFS-C mknod/rename evidence-only impact

No source change is retained for the `mknod07,mknodat02,rename03,rename04,rename05` RV scout. The new artifact records current setup behavior only: LTP cannot acquire a free device on RV and emits parser-visible `TBROK` for both musl and glibc.

Visible ABI/POSIX impact from this documentation update: none. Syscall numbers, errno behavior, `mknod`/`mknodat` mode/dev semantics, `rename` path semantics, FD behavior, block-device behavior, signal/futex/mmap/user-pointer layout, and resource lifetime are unchanged by this checkpoint.

Future work must implement or expose a generic device-acquisition model if these tests are to reach their VFS assertions. It must not hardcode LTP case names, paths, device names, or outputs, and it must rerun adjacent mknod/mknodat/rename plus mount/device setup regressions before promotion.

## LTP device and filesystem NAME_MAX impact

Files: `examples/shell/src/cmd.rs`, `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/metadata.rs`, `examples/shell/src/uspace/linux_abi.rs`.

The LTP wrapper now provides a generic `LTP_DEV=/dev/vda` for LTP tests. This exposes the evaluator's existing synthetic block-backed test device to LTP setup instead of relying on an unimplemented Linux loop-device stack. The special `chdir01` filesystem override remains `tmpfs`, but the device variable is no longer case-local.

The synthetic `/dev` directory now reports `vda`, `sda`, and `xvda` as block-device directory entries, and stat/statx-style metadata for those paths reports `S_IFBLK` with stable synthetic `st_rdev` values (`/dev/vda` as major 254 minor 0; `/dev/sda` as major 8 minor 0; `/dev/xvda` as major 202 minor 0). Opening `/dev/vda` still uses the existing synthetic block-device path; no host loop device, ext2 formatter, or hidden LTP case-name mapping is introduced.

`statfs`/`statvfs` and pathname component validation now report/enforce the real backing name capacity of `axfs_vfs::VfsDirEntry`: 63 bytes. This is POSIX-visible through `f_namelen` / `_PC_NAME_MAX`-adjacent behavior and prevents valid-looking 255-byte component names from reaching a 63-byte dirent buffer and panicking. The change intentionally reduces the exposed limit to match the current filesystem implementation rather than overstating Linux's common 255-byte value.

User-visible impact:

- A generic LTP environment variable now points device-using tests at `/dev/vda`.
- `/dev` enumeration and block-device metadata are more consistent with existing synthetic block-device opens.
- `statfs().f_namelen` / `statvfs().f_namemax` now reports 63 on this filesystem; overlength components beyond 63 are rejected before VFS dirent construction.
- Syscall numbers, struct layouts, FD allocation, signal/futex/mmap/user-pointer layouts, and ordinary non-device path behavior are otherwise unchanged.

Regression evidence: `chdir01`, `pathconf01`, and `fpathconf01` are parser-clean on RV and LA for musl+glibc after the change (`rv-ltpdev-namemax-regression-subset-20260602T041926Z.summary.txt`, `la-ltpdev-namemax-regression-subset-20260602T042012Z.summary.txt`).

Maintenance boundary: this is not full Linux loop-device or disk formatting support. `mknod07` and `mknodat02` still need a generic way to satisfy their ext2 setup (`mkfs.ext2` is absent); `rename03` and `rename04` now expose real rename semantic failures. Future device work must remain generic and rerun device/statfs/mknod/rename plus adjacent pathconf/chdir regressions before promotion.

## FD/fcntl evidence-only impact

No source change is retained for the 2026-06-02 FD/fcntl scout. The artifact records current behavior only: `fcntl15` and `fcntl11_64` are RV + LA x musl+glibc parser-clean; surrounding fcntl rows retain visible timeout/TCONF/TFAIL/TBROK blockers.

Visible ABI/POSIX impact from this documentation update: none. Syscall numbers, errno behavior, file-lock/OFD-lock semantics, FD ownership/lease behavior, signal/futex/mmap/user-pointer layout, and resource lifetime are unchanged by this checkpoint.

Future fcntl work must remain generic. In particular, `fcntl17` needs lock/wakeup timeout diagnosis; `fcntl24`/`fcntl25`/`fcntl26` need a non-tmpfs or generic lease-capable setup path before TCONF can disappear; `fcntl27`, `fcntl31`, and `fcntl34` need real lease/owner/OFD-lock semantics; `fcntl38`/`fcntl39` need a generic kconfig/capability boundary decision. None of these blocker rows is promotion evidence.

## Rename metadata/inode preservation impact

Files: `examples/shell/src/uspace/mod.rs`, `examples/shell/src/uspace/process_lifecycle.rs`, `examples/shell/src/uspace/metadata.rs`, `examples/shell/src/uspace/fd_table.rs`.

The POSIX emulation layer now records per-path inode overrides and migrates recorded metadata when `renameat2(..., flags=0)` succeeds. Before this change, stat-style `st_ino` values were derived from the pathname hash, so a successful `rename(old, new)` made the same file or directory appear to have a different inode. The new path metadata migration preserves the old object's inode at the new path and also moves recorded mode, owner, special-device, symlink, xattr, and sparse-file metadata while clearing stale target-side recorded metadata when appropriate.

User-visible behavior:

- `stat()`/`fstatat()`/statx-derived inode values for a file or directory now remain stable across successful `rename()` within the current process metadata model.
- `unlinkat()` removes recorded inode metadata for deleted paths, avoiding stale inode overrides for later unrelated creations.
- No syscall numbers, struct layouts, flag constants, FD allocation rules, signal/futex/mmap/user-pointer layouts, or block-device semantics changed.
- This is still not hard-link support: `link(2)`/`linkat(2)` remain generic blockers where the underlying implementation is absent or setup emits parser-visible TCONF/ENOSYS.

Regression evidence: `rename01` and adjacent existing candidate `rename05` are parser-clean on RV and LA for musl+glibc after the change (`rv-rename-inode-retarget-20260602T044708Z.summary.txt`, `la-rename-inode-retarget-20260602T044751Z.summary.txt`). Singleton `rename01` proof is also parser-clean on RV and LA (`rv-rename01-inode-confirm-20260602T044855Z.summary.txt`, `la-rename01-inode-confirm-20260602T044855Z.summary.txt`).

Maintenance boundary: this is a generic metadata-lifetime fix, not an LTP case/path/output special case. Future rename/link work must preserve Linux/POSIX visible errno and inode/link-count semantics and must not emulate hard links by copying data without proving shared object identity and nlink behavior.
