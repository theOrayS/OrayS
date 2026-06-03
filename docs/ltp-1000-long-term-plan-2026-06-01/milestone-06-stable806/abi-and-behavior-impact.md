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
- The patch does not hardcode LTP case names, paths, process names, or expected output. Remaining `mkdir09`, `mknod07`, and `mknodat02` blockers retain visible parser markers and are not counted until separately repaired; `unlink09` moved to the later FS_IOC inode-flag repair lane.

## mkdir setgid and final symlink existence repair impact

This source patch changes real metadata/path behavior; it is not a stable-list promotion.

User-visible syscall/path changes:

- `chown`/`fchown` metadata updates now preserve `S_ISGID` on directories. The existing non-directory behavior still clears `S_ISGID` when group-execute is set, matching the existing special-bit safety rule for regular files and other non-directories.
- Directory creation under a parent directory with `S_ISGID` now preserves inherited group and setgid metadata through the `chown` setup path exercised by `mkdir02`.
- `mkdir`/`mkdirat` now treat a process-visible final-component synthetic symlink as an existing path and return `EEXIST` instead of creating a directory at that symlink path.
- `mknod`/`mknodat` use the same final synthetic-symlink existence check and return `EEXIST` before node creation.

Unchanged boundaries:

- Parent-path symlink resolution semantics from the prior VFS repair remain unchanged; this patch only adds final-component synthetic symlink existence checks before create.
- No syscall numbers, struct layouts, FD table layout, FD_CLOEXEC behavior, file status flags, signal delivery, futex behavior, mmap behavior, user-pointer copy semantics, blacklist, or evaluator behavior changed.
- The patch does not hardcode LTP case names, paths, process names, or expected output. Remaining non-candidate rows from earlier VFS/FD/select scouts retain visible parser markers and remain excluded.

## fcntl read-lease access repair impact

This source patch changes real `fcntl(F_SETLEASE)` errno behavior; it is not a stable-list promotion.

User-visible syscall/errno changes:

- `fcntl(fd, F_SETLEASE, F_RDLCK)` now returns `EAGAIN` when the file descriptor was opened with write access (`O_WRONLY` or `O_RDWR`).
- Read leases on read-only regular-file descriptors remain accepted, preserving existing stable `fcntl23`/`fcntl23_64` behavior.
- `F_WRLCK` and `F_UNLCK` handling is otherwise unchanged; full lease break/delivery semantics are still not modeled.

Unchanged boundaries:

- FD allocation, `FD_CLOEXEC`, `F_GETFL`/`F_SETFL`, record locks (`F_GETLK`, `F_SETLK`, `F_SETLKW`), pipe capacity commands, signals, futexes, mmap, struct layout, and user-pointer copy semantics are unchanged.
- The patch does not hardcode LTP case names, paths, process names, or expected output. It applies a generic access-mode rule before recording a read lease.

Follow-up `fcntl27_64` validation made no additional source changes. It demonstrates that the same visible errno rule also covers the 64-bit LTP variant; syscall numbers, struct layouts, FD flags, signal/futex/mmap behavior, and user-pointer copying remain unchanged beyond the generic `F_SETLEASE` access-mode rule above.



## symlink03 tmpdir and parent permission repair impact

This source patch changes real path metadata and `symlinkat` errno behavior; it is not a stable-list promotion and does not edit `LTP_STABLE_CASES`.

User-visible syscall/path changes:

- Newly loaded user programs seed per-process path mode metadata for `/tmp` and `/tmp/ltp-work` as `01777` so setuid/forked test children can create scratch subdirectories under the shared temporary root like a normal Linux tmpdir.
- `symlinkat` now checks the resolved parent path with the generic parent write/search/type permission helper before recording a synthetic symlink.
- Symlink creation under a parent directory lacking write/search permission now returns the generic permission error (`EACCES`) instead of silently creating the link.
- Symlink creation through a non-directory parent component now returns `ENOTDIR` through the same helper before synthetic link insertion.

Unchanged boundaries:

- Final symlink creation semantics remain non-following for the new link name; the change checks only the parent path before creation.
- No syscall numbers, struct layouts, FD table layout, `FD_CLOEXEC`, file status flags, signal delivery, futex behavior, mmap behavior, user-pointer copy semantics, blacklist, or evaluator behavior changed.
- The patch does not hardcode LTP case names, paths, process names, or expected output. `/tmp` and `/tmp/ltp-work` are generic harness scratch roots and receive standard Linux tmpdir permissions rather than test-result-specific behavior.

## unlink09 FS_IOC inode-flag repair impact

This source patch changes generic file `ioctl` and unlink errno behavior; it is not a stable-list promotion and does not edit `LTP_STABLE_CASES`.

User-visible syscall/errno/flag changes:

- `ioctl(fd, FS_IOC_GETFLAGS, u32*)` now succeeds for path-backed file descriptors and copies out the current in-memory inode flags, defaulting to `0` for paths without stored flags.
- `ioctl(fd, FS_IOC_SETFLAGS, u32*)` now copies in a `u32` flag word for path-backed file descriptors and records it in per-process path metadata; setting flags to `0` clears the stored metadata.
- `FS_IOC_SETFLAGS` on a path that resolves under the existing read-only mount model returns `EROFS`.
- `FS_IOC_GETFLAGS`/`FS_IOC_SETFLAGS` on a non-path-backed descriptor return `ENOTTY`; invalid descriptors still return the table error such as `EBADF`.
- `unlink`/`unlinkat` now return `EPERM` for paths carrying `FS_IMMUTABLE_FL` or `FS_APPEND_FL`, before removing regular-file or hardlink metadata.
- Successful unlink/symlink/hardlink-alias removal clears any stored inode flags for that path, and `rename` moves the stored flags with the path metadata.

Resource/lifetime and maintenance boundaries:

- Inode flags are process-local in-memory metadata cloned across `fork()`, matching the existing synthetic path metadata model; they are not persisted to the backing filesystem image and are not a full ext4 inode implementation.
- The patch does not change syscall numbers, struct layouts, FD table layout, `FD_CLOEXEC`, file status flags (`O_APPEND`, `O_NONBLOCK`, etc.), signal delivery, futex behavior, mmap behavior, or user-pointer copy semantics beyond the explicit `ioctl` copy-in/copy-out for the `u32` flags word.
- The implementation uses generic Linux inode flag constants and VFS unlink errno behavior; it does not hardcode LTP case names, paths, process names, or expected output.
