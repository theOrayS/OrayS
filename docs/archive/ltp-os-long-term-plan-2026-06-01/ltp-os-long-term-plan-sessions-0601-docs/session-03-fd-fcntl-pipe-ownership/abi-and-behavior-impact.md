# Session 3 ABI and behavior impact

## User-visible behavior changes

Implemented a real, generic POSIX `fcntl` advisory record-lock model for regular files:

- `F_GETLK` now reports the first conflicting live owner lock instead of always writing `F_UNLCK`.
- `F_SETLK` now validates read/write lock access, records per-process byte-range locks, detects conflicts from other processes, returns `EAGAIN` for non-blocking conflicts, and removes/splits locks on `F_UNLCK`.
- `F_SETLKW` now waits by yielding until the conflicting lock is released, then installs the requested lock.
- Record locks are keyed by file identity (`path_inode`) and owner PID, not by LTP case names or paths.
- Locks are released when the owning process closes a file descriptor for the locked file, when `dup3` replaces an existing file descriptor, and during process teardown.
- Stale locks from exited owners are pruned before conflict checks.

## Syscall / errno impact

- `fcntl(fd, F_GETLK, struct flock *)`: may now write `l_type`, `l_whence=SEEK_SET`, `l_start`, `l_len`, and `l_pid` for a real conflicting lock.
- `fcntl(fd, F_SETLK, struct flock *)`: may now return `-EAGAIN` on conflicting locks and `-EBADF` when a read lock is requested on a non-readable file or a write lock on a non-writable file.
- `fcntl(fd, F_SETLKW, struct flock *)`: can yield/block until the conflict clears; signal-interrupt semantics are not fully modeled yet.
- `close(fd)` / process exit: now releases this process's POSIX record locks for the corresponding file/process.
- `dup3(oldfd, newfd, flags)`: replacing `newfd` now uses process-aware close semantics so replaced file locks are released for that file/process.

## ABI / layout impact

- No public struct layout changes.
- `struct flock` copy-in/copy-out uses the existing `linux_raw_sys::general::flock` layout.
- No change to FD table layout visible to user programs.
- No changes to signal, futex, mmap, or user pointer validation APIs outside the `fcntl` copy-in/copy-out paths.

## Known limitations

- Blocking `F_SETLKW` is implemented with cooperative yielding, not a dedicated wait queue, and does not yet return `EINTR` for pending signals.
- OFD locks (`F_OFD_*`) remain out of scope for this session.
- Mandatory locking is not a separate kernel-enforced I/O permission model; the LTP `fcntl14` mandatory-locking checks pass because advisory conflict reporting/blocking semantics are now real.
