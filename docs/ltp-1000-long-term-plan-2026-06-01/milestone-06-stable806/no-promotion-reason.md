# milestone-06 current no-promotion reason

This is an interim stable806 checkpoint. The current baseline remains stable756: `prctl08`, `prctl09`, `utsname02`, `mkdirat02`, `rmdir02`, `mkdir02`, `mkdir03`, `fcntl27`, `fcntl27_64`, `symlink03`, and `unlink09` are now four-combo clean candidates after real timerslack, shared-UTS, VFS path/errno, directory setgid, final-symlink existence, fcntl read-lease, symlink parent-permission, and FS_IOC inode-flag/unlink errno repairs, but they are only 11 unique new cases and do not satisfy the next 50-case milestone gate.

Reasons promotion is still blocked at this checkpoint:

1. The old archived 4/4 clean-not-stable seed list has already been exhausted by earlier milestones; no remaining old clean seed exists outside current stable756.
2. The broader proc/synthetic/sched scout still has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows outside `prctl08`/`prctl09`.
3. The time/fd/signal scout still has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows outside this repair lane.
4. The clean timerslack pair, UTS shared-hostname row, VFS/mkdir `mkdir02`/`mkdir03`/`mkdirat02`/`rmdir02` rows, `fcntl27`/`fcntl27_64`, `symlink03`, and `unlink09` have RV + LA × musl + glibc evidence, and the UTS, VFS/metadata, fcntl, and symlink adjacent subsets are clean; however the candidate pool is still far below the required next +50 unique stable milestone.
5. No blacklist/SKIP/status0/full-sweep partial TPASS evidence is counted.

Next safe slices:

- Keep `prctl08`, `prctl09`, `utsname02`, `mkdirat02`, `rmdir02`, `mkdir02`, `mkdir03`, `fcntl27`, `fcntl27_64`, `symlink03`, and `unlink09` in the stable806 candidate pool and batch them only with enough additional four-combo clean cases to reach the next 50-case milestone.
- Keep `nice04` out of the candidate pool unless a principled libc/ABI-compatible errno boundary is found; do not special-case the LTP wrapper.
- Avoid POSIX timer rows (`timer_create` family) as easy promotions unless the project accepts a real timer-object implementation.
- Prefer small FD/fcntl/pipe/io or narrowly scoped mmap/futex probes next; avoid readlink LA-musl, statx attribute/env-heavy rows, 16-bit UID/capability rows, and broad socket batches until their blockers have real semantic fixes.


Additional blocker/scout note:

- The RV socket-core scout `rv-socket-core-scout-20260603T184807+0800` is partial blocker evidence only. It produced visible `TCONF/TFAIL/TBROK/ENOSYS` rows and an incomplete glibc `accept02` state after a futex abort/hang, so it contributes zero promotion candidates.
- `utsname02` is now a valid candidate; `utsname03` remains blocked on real `CLONE_NEWUTS`/`unshare(CLONE_NEWUTS)` namespace semantics.


Additional blocker/scout notes from the post-UTS triage:

- `readlink03`/`readlinkat02` are near-clean but blocked on LA musl. Debug evidence shows the failing musl wrapper converts the nominal `bufsiz=0` case into a non-null one-byte buffer before the syscall reaches the kernel; returning `EINVAL` for all `bufsiz=1` reads would be a semantic regression, so no fake workaround is allowed.
- `nice04` is blocked by the musl-visible `EACCES` result from the generic `setpriority` path, while glibc maps the `nice(-10)` wrapper expectation to `EPERM`. A kernel-only special case would risk breaking stable setpriority semantics.
- The RV statx scout produced `TCONF` on `statx01`, visible FAILs on the rest, and timeouts for `statx11`; it contributes zero candidates.
- The RV credential/capability scout produced 16-bit UID/capability `TCONF` rows plus a glibc `gettid02` futex `TBROK`; it contributes zero candidates.
- The RV VFS/FD/select scout `rv-vfs-fd-select-scout-20260603T194925+0800` produced `9 PASS / 45 FAIL` with visible `TBROK/TCONF/TFAIL` markers and four `fcntl17*` timeouts; it contributes zero candidates. `select01`..`select04` are pass-with-TCONF rows, not stable evidence.


Additional no-promotion note after VFS repair:

- The parent-symlink/rmdir repair makes `mkdirat02` and `rmdir02` four-combo clean and protects a 36-case adjacent VFS stable subset on both RV and LA. This raises the candidate pool to 5 new unique cases before the later mkdir setgid/final-symlink repair, still below the +50 stable806 promotion gate. `LTP_STABLE_CASES` therefore remains unchanged at `756 total / 756 unique / 0 duplicate`.

Additional no-promotion note after mkdir setgid/final-symlink repair:

- Preserving directory `S_ISGID` across `chown` and treating final synthetic symlinks as existing makes `mkdir02` and `mkdir03` four-combo clean while keeping `mkdirat02` and `rmdir02` clean. The adjacent metadata/VFS stable subset is clean on both RV and LA. This raises the candidate pool to 7 new unique cases, still below the +50 stable806 promotion gate. `LTP_STABLE_CASES` therefore remains unchanged at `756 total / 756 unique / 0 duplicate`.

Additional no-promotion note after fcntl27 repair:

- Returning `EAGAIN` for read leases on write-open descriptors makes `fcntl27` and `fcntl27_64` four-combo clean and preserves all current stable `fcntl*` rows in RV/LA adjacent regression. This raises the candidate pool to 9 new unique cases, still below the +50 stable806 promotion gate. `LTP_STABLE_CASES` therefore remains unchanged at `756 total / 756 unique / 0 duplicate`.



Additional no-promotion note after symlink03 repair:

- Seeding standard `01777` tmpdir path modes and applying the generic parent write/search/type permission gate to `symlinkat` makes `symlink03` four-combo clean and preserves a 20-case symlink/access/readlink/link/unlink/rmdir/mkdir adjacent stable subset on RV and LA. This raises the candidate pool to 10 new unique cases, still below the +50 stable806 promotion gate. `LTP_STABLE_CASES` therefore remains unchanged at `756 total / 756 unique / 0 duplicate`.

Additional no-promotion note after unlink09 FS_IOC inode-flag repair:

- Generic `FS_IOC_GETFLAGS`/`FS_IOC_SETFLAGS` handling plus `FS_IMMUTABLE_FL`/`FS_APPEND_FL` unlink protection makes `unlink09` four-combo clean and preserves a 23-case unlink/access/symlink/readlink/link/rmdir/mkdir adjacent stable subset on RV and LA. This raises the candidate pool to 11 new unique cases, still below the +50 stable806 promotion gate. `LTP_STABLE_CASES` therefore remains unchanged at `756 total / 756 unique / 0 duplicate`.
