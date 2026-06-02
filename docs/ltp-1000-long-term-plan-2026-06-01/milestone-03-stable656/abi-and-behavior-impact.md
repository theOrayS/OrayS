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
- Candidate pool after the historical FD/fcntl checkpoint was 21/50; the latest mmap/munlock checkpoint below supersedes the current count to 28/50.

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

## Rename03/rename04 directory replacement repair impact

Changed file: `kernel/fs/axfs/src/root.rs`.

User-visible behavior:

- `rename(old_file, existing_file)` still removes the destination file and renames the source file.
- `rename(old_file, existing_dir)` now reliably returns `EISDIR` instead of attempting to remove the directory as a file.
- `rename(old_dir, existing_file)` now returns `ENOTDIR` instead of deleting the destination file first or surfacing a misleading later error.
- `rename(old_dir, existing_empty_dir)` now removes the empty destination directory and renames the source directory into its place.
- `rename(old_dir, existing_non_empty_dir)` now preserves the destination and returns the underlying non-empty-directory error (`ENOTEMPTY`/`DirectoryNotEmpty` path) through `remove_dir`.
- `rename(path, path)` now succeeds without mutating the filesystem.

ABI/POSIX surface: no struct layout, syscall number, FD table, signal, futex, mmap, or user-pointer ABI changed. The visible syscall impact is limited to generic `rename`/`renameat` errno and replacement semantics for existing destinations. This intentionally aligns closer to Linux/POSIX rename behavior and is not keyed to LTP names or paths.

Lifetime/resource risk: the repair looks up the source before removing any destination, reducing destructive failure risk when the source is absent. Destination directory replacement still relies on existing `remove_dir` emptiness and permission checks, so non-empty directories and protected paths remain guarded.

## Stat/readlink path traversal and parent search-permission impact

Changed files: `examples/shell/src/uspace/metadata.rs`, `examples/shell/src/uspace/fd_table.rs`.

User-visible behavior:

- Pathname resolution now follows symlinks in intermediate path components rather than only when the complete final pathname is a recorded symlink. More than 40 symlink traversals returns `ELOOP`.
- `readlink`/`readlinkat` and `AT_SYMLINK_NOFOLLOW` stat-style calls resolve parent symlinks while preserving the final symlink when the syscall semantics require it.
- Non-root `stat`/`fstatat` path lookup now checks execute/search permission on parent directories and returns `EACCES` when an ancestor is not searchable.
- A non-directory component in a path prefix now returns `ENOTDIR` at the parent traversal boundary.
- Empty pathname `newfstatat` without `AT_EMPTY_PATH` now returns `ENOENT`.
- `O_NOFOLLOW` open handling resolves parent symlinks before applying the final-component symlink rejection.

ABI/POSIX surface: syscall numbers, struct layouts, FD allocation, signal masks, futex behavior, mmap layout, and user-pointer ABI are unchanged. The visible syscall impact is limited to generic path traversal, symlink loop, search-permission, and errno semantics for stat/readlink/open path handling. The change is not keyed to LTP case names, paths, processes, or output strings.

Lifetime/resource risk: the first local RV attempt exposed a recursion bug in parent-search checking and produced a parser-visible panic/trap. The retained implementation avoids that recursion by using an internal non-checking `stat_path_inner` while inspecting ancestors. Adjacent stat/lstat/fstatat/readlink/openat/rename regression subsets are parser-clean on RV and LA.

Maintenance boundary: future path/link/stat work must preserve parent-vs-final symlink semantics, the 40-hop `ELOOP` guard, and non-root directory search-permission checks. Any hard-link/linkat/statx/getdents fix must rerun this stat/readlink regression subset before promotion accounting.

## mmap fd/flag validation and munlock range validation impact

Changed files: `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/memory_map.rs`, `examples/shell/src/uspace/syscall_dispatch.rs`.

User-visible behavior:

- `mmap(MAP_SHARED_VALIDATE | unsupported_bits, ...)` now returns `EOPNOTSUPP` for unsupported validation bits, rather than silently accepting unknown bits. Ordinary supported mapping flags keep the previous behavior.
- Non-anonymous `mmap` validates the fd before reserving or mapping virtual address space. Invalid descriptors return `EBADF`; unreadable regular files return `EACCES`; directory/proc-fd directory descriptors return `EISDIR`; pipe/socket/local-socket descriptors return `ESPIPE`.
- `munlock(addr, len)` is now dispatched through `sys_munlock` and validates the full page-rounded mapped range. `len == 0` still succeeds; overflow, ranges beyond the user address limit, and any unmapped page return `ENOMEM`.
- `mlock(addr, len)` shares the same mapped-range validator before prefaulting pages through the existing `populate_range` path.

ABI/POSIX surface: syscall numbers, struct layouts, mmap flag numeric values, FD table layout, signal masks, futex values, and user-pointer ABI are unchanged. The visible change is limited to generic `mmap`/`mlock`/`munlock` errno behavior.

Maintenance boundary: this is still not full Linux memory-lock accounting. `mlock02` remains blocked because `RLIMIT_MEMLOCK`/capability semantics are not implemented. `mmap08` remains blocked because diagnostic evidence shows the tested fd is still a readable temp-file descriptor at mmap time, so the EBADF case is not reached. Future fd-lifetime or memory-lock accounting fixes must keep the mmap/mincore/mprotect/munlock regression subset parser-clean on RV and LA before promotion.

Regression evidence: `mmap20` and `munlock02` are parser-clean on RV and LA for musl+glibc (`rv-mmap20-munlock02-targeted-20260602T054424Z.summary.txt`, `la-mmap20-munlock02-targeted-20260602T054508Z.summary.txt`). Adjacent regression subsets are parser-clean on both arches (`rv-mmap-munlock-regression-20260602T054554Z.summary.txt`, `la-mmap-munlock-regression-20260602T054705Z.summary.txt`).

Stable-list impact: unchanged at `606 total / 606 unique / 0 duplicate`. Candidate pool after this checkpoint: 28/50 (`fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename01`, `rename03`, `rename04`, `rename05`, `sched_setaffinity01`, `signal01`, `stat03`, `stat03_64`, `statfs01`, `statvfs01`, `mmap20`, `munlock02`).


## epoll_create1 descriptor and legacy epoll_create size validation impact

Changed files: `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/syscall_dispatch.rs`, `api/arceos_posix_api/src/imp/io_mpx/epoll.rs`.

User-visible behavior:

- `epoll_create1(0)` and `epoll_create1(EPOLL_CLOEXEC)` now allocate an fd in the shell userspace syscall bridge instead of returning `ENOSYS`.
- `epoll_create1` rejects any flag bit outside `EPOLL_CLOEXEC` with `EINVAL`, including high bits beyond the low 32-bit flag space; valid `EPOLL_CLOEXEC` is recorded through the existing `FD_CLOEXEC` fd-table flag.
- The created descriptor is represented as a synthetic `anon_inode:[eventpoll]` path entry. It supports the fd-lifetime operations covered by the current proof (`close`, `dup`, `fcntl` flag queries/updates through the fd table) but does not implement full `epoll_ctl`/`epoll_wait` readiness semantics yet.
- The axlibc/glibc-visible legacy `epoll_create(size)` path now returns `EINVAL` for `size <= 0`, matching Linux's visible invalid-size behavior for callers that actually pass the legacy size to the kernel.

ABI/POSIX surface: syscall numbers, struct layouts, fd-table layout, signal/futex/mmap/user-pointer ABI, and existing pipe/socket/file descriptor semantics are unchanged. The visible syscall impact is limited to generic eventpoll fd creation and errno behavior.

Maintenance boundary: this is not a full epoll implementation. Future `epoll_ctl`/`epoll_wait` support must implement real interest-list/readiness semantics and rerun epoll, poll/select, fd/close/dup/fcntl, socket/pipe readiness, and exec/cloexec regression subsets before promotion accounting. `epoll_create02` remains blocked because musl's old `epoll_create(size)` wrapper maps to valid `epoll_create1(0)`; do not make `epoll_create1(0)` invalid to satisfy that row.

Regression evidence: `epoll_create1_01` and `epoll_create1_02` are parser-clean on RV and LA for musl+glibc (`target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt`). Adjacent FD/flag subsets are parser-clean on both arches (`target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.txt`).

Stable-list impact: unchanged at `606 total / 606 unique / 0 duplicate`. Candidate pool after this checkpoint: 30/50 (`epoll_create1_01`, `epoll_create1_02`, `fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mmap20`, `mprotect02`, `mprotect04`, `munlock02`, `munmap01`, `openat02`, `rename01`, `rename03`, `rename04`, `rename05`, `sched_setaffinity01`, `signal01`, `stat03`, `stat03_64`, `statfs01`, `statvfs01`).


## clock_adjtime and sigaltstack syscall-state impact

Changed files: `examples/shell/src/uspace/time_abi.rs`, `examples/shell/src/uspace/signal_abi.rs`, `examples/shell/src/uspace/syscall_dispatch.rs`, `examples/shell/src/uspace/task_context.rs`.

User-visible behavior:

- `clock_adjtime(CLOCK_REALTIME, tx)` is now dispatched through the userspace syscall bridge and reuses the existing `adjtimex` ABI handling. Read-only/default `timex` queries return the same conservative `TIME_OK` state as `adjtimex`; unsupported clock IDs return `EINVAL`.
- `sigaltstack(ss, old_ss)` now records per-thread `ss_sp`, `ss_flags`, and `ss_size`, returns the previous state through `old_ss`, validates unknown flag bits with `EINVAL`, rejects undersized enabled stacks with `ENOMEM`, and rejects stack changes while already in a signal frame with `EPERM`.
- `shmt04` is included as evidence-only: no SysV shm code changed in this checkpoint, but RV + LA x musl/glibc parser-clean proof now closes that candidate row.

ABI/POSIX surface: syscall numbers, struct layouts, fd-table layout, futex values, mmap layout, and user-pointer copy ABI are unchanged. The visible syscall impact is limited to removing `ENOSYS` for generic `clock_adjtime`/`sigaltstack` calls and exposing Linux-like errno/state behavior for those existing ABI structs.

Maintenance boundary: `sigaltstack` is currently syscall-state support, not full alternate-stack signal delivery. Future work that delivers handlers on the alternate stack must preserve the recorded state semantics and rerun the adjacent signal regression subset before promotion accounting. `clock_adjtime` intentionally only accepts `CLOCK_REALTIME` in this bridge; future support for dynamic or CPU clocks must not weaken the existing `adjtimex` permission/errno checks.

Regression evidence: `adjtimex01`, `adjtimex03`, `sigaltstack02`, and `shmt04` are parser-clean on RV and LA for musl+glibc (`target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt`). Adjacent time/signal stable subsets are parser-clean on both arches (`target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.summary.txt`).

Stable-list impact: unchanged at `606 total / 606 unique / 0 duplicate`. Candidate pool after this checkpoint: 34/50 (`adjtimex01`, `adjtimex03`, `epoll_create1_01`, `epoll_create1_02`, `fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mmap20`, `mprotect02`, `mprotect04`, `munlock02`, `munmap01`, `openat02`, `rename01`, `rename03`, `rename04`, `rename05`, `sched_setaffinity01`, `shmt04`, `signal01`, `sigaltstack02`, `stat03`, `stat03_64`, `statfs01`, `statvfs01`).


## SysV shm IPC_STAT user ABI impact

Changed file: `examples/shell/src/uspace/sysv_shm.rs`.

User-visible behavior:

- `shmctl(shmid, IPC_STAT, buf)` now copies a `#[repr(C)]` Linux 64-bit `shmid_ds`-compatible structure of 112 bytes instead of clearing `16 * sizeof(usize)` bytes (128 bytes on RV/LA). This prevents overwriting libc stack objects that allocate exactly `struct shmid_ds` while preserving normal successful metadata queries.
- The copied structure exposes the segment key, mode bits from `shmget(..., shmflg)`, and the originally requested segment size. Other fields remain conservative zero values in the existing lightweight SysV shm model.
- Invalid/null user buffers still return the existing user-copy `EFAULT` path through `write_user_value`; unsupported `shmctl` commands still return `EINVAL`.

ABI/POSIX surface: syscall numbers and command constants are unchanged. The intentional visible ABI change is the user-memory copy layout/size for `IPC_STAT`, aligning it with Linux 64-bit `asm-generic` `shmid64_ds` used by RISC-V and LoongArch. FD, signal, futex, mmap, path, and scheduler behavior are unchanged.

Lifetime/resource risk: this does not implement full SysV shm attach refcounting or reclamation. `shm_nattch` remains conservatively reported as 0 because the current process-local attachment map is cloned across fork without a global lifetime counter. Future SysV shm work must add a real cross-process refcount/free model before promoting stress/lifetime cases.

Regression evidence: `shmat04` and already-counted `shmt04` are parser-clean on RV and LA for musl+glibc (`target/ltp-1000-milestone-03-stable656/rv-shmat04-shmt04-ipcstat-abi-20260602T150702+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-shmat04-shmt04-ipcstat-abi-20260602T150805+0800.summary.txt`).

Stable-list impact: unchanged at `606 total / 606 unique / 0 duplicate`. Candidate pool after this checkpoint: 35/50.
