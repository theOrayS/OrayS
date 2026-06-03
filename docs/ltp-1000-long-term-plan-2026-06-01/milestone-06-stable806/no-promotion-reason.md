# milestone-06 current no-promotion reason

This is an interim stable806 checkpoint. The current baseline remains stable756: `prctl08`, `prctl09`, `utsname02`, `mkdirat02`, `rmdir02`, `mkdir02`, `mkdir03`, `fcntl27`, `fcntl27_64`, `symlink03`, `unlink09`, `mkdir09`, `gettid02`, `futex_wait_bitset01`, `fstat02`, and `fstat02_64` are now four-combo clean candidates after real timerslack, shared-UTS, VFS path/errno, directory setgid, final-symlink existence, fcntl read-lease, symlink parent-permission, FS_IOC inode-flag/unlink errno, futex bitset repairs, and late FD metadata discovery. They are only 16 unique new cases and do not satisfy the next 50-case milestone gate.

Reasons promotion is still blocked at this checkpoint:

1. The old archived 4/4 clean-not-stable seed list has already been exhausted by earlier milestones; no remaining old clean seed exists outside current stable756.
2. The broader proc/synthetic/sched scout still has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows outside `prctl08`/`prctl09`.
3. The time/fd/signal scout still has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows outside this repair lane.
4. The clean timerslack pair, UTS shared-hostname row, VFS/mkdir `mkdir02`/`mkdir03`/`mkdirat02`/`rmdir02` rows, `fcntl27`/`fcntl27_64`, `symlink03`, `unlink09`, `mkdir09`, `gettid02`, `futex_wait_bitset01`, `fstat02`, and `fstat02_64` have RV + LA × musl + glibc evidence, and the UTS, VFS/metadata, fcntl, symlink, unlink, and futex/clone adjacent subsets are clean; however the candidate pool is still far below the required next +50 unique stable milestone.
5. No blacklist/SKIP/status0/full-sweep partial TPASS evidence is counted.

Next safe slices:

- Keep `prctl08`, `prctl09`, `utsname02`, `mkdirat02`, `rmdir02`, `mkdir02`, `mkdir03`, `fcntl27`, `fcntl27_64`, `symlink03`, `unlink09`, `mkdir09`, `gettid02`, `futex_wait_bitset01`, `fstat02`, and `fstat02_64` in the stable806 candidate pool and batch them only with enough additional four-combo clean cases to reach the next 50-case milestone.
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
- The RV credential/capability scout produced 16-bit UID/capability `TCONF` rows and an earlier glibc `gettid02` futex `TBROK`; the 16-bit/capability rows still contribute zero candidates, while `gettid02` is superseded by the later futex/glibc follow-up evidence.
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

Additional no-promotion note after mkdir09 futex bitset repair:

- Generic `FUTEX_WAIT_BITSET`/`FUTEX_WAKE_BITSET` support fixes the glibc pthread join abort seen by `mkdir09` while preserving existing stable futex wait/wake rows. `mkdir09` is four-combo clean and the futex/clone adjacent subset is parser-clean on RV and LA, raising the candidate pool to 12 new unique cases. This still falls below the +50 stable806 promotion gate, so `LTP_STABLE_CASES` remains unchanged at `756 total / 756 unique / 0 duplicate`.

Additional no-promotion note after gettid02 futex/glibc follow-up:

- The same generic futex bitset/glibc pthread repair lane makes `gettid02` four-combo clean on RV and LA. No additional source change was made for this follow-up, and the earlier futex/clone adjacent subset remains the regression boundary for the underlying code change. This raises the candidate pool to 13 new unique cases, still below the +50 stable806 promotion gate, so `LTP_STABLE_CASES` remains unchanged at `756 total / 756 unique / 0 duplicate`.


Additional no-promotion note after futex_wait_bitset01 follow-up:

- The existing generic futex bitset implementation also makes `futex_wait_bitset01` four-combo clean on RV and LA. The same RV scout shows `futex_wake02`, `futex_wake04`, `futex_cmp_requeue01`, and `futex_cmp_requeue02` still have visible `TBROK`/`TCONF` blockers, and the clone plus FD/vector-IO follow-up scouts add zero candidates because their RV evidence contains visible `TFAIL`/`TBROK`/`TCONF`/`ENOSYS`. This raises the candidate pool to 14 new unique cases, still below the +50 stable806 promotion gate, so `LTP_STABLE_CASES` remains unchanged at `756 total / 756 unique / 0 duplicate`.


Additional no-promotion note after fstat02/fstat02_64 follow-up:

- The RV FD/path scout plus LA follow-up make `fstat02` and `fstat02_64` four-combo clean without any source change in this follow-up. The same scout leaves `close_range01`, `close_range02`, `getcwd03`, `getcwd04`, `openat03`, `openat04`, `open14`, and `creat07` blocked by visible parser markers, and the VFS/MM, process/exec/signal, and exec-only scouts add zero candidates. This raises the candidate pool to 16 new unique cases, still below the +50 stable806 promotion gate, so `LTP_STABLE_CASES` remains unchanged at `756 total / 756 unique / 0 duplicate`.


Additional no-promotion note after sync/fd/io and xattr scouts:

- The RV sync/fd/io scout (`fdatasync03`, `fsync03`, `fsync04`, `sync01`, `syncfs01`, `sync_file_range01`, `sync_file_range02`, `read03`, `write04`, `lseek11`) produced zero candidates: every row has visible `TCONF`, `TFAIL`, `TBROK`, or `ENOSYS` markers.
- The RV xattr scout (`fgetxattr02`, `fsetxattr02`, `getxattr02`..`getxattr05`, `setxattr02`, `setxattr03`) also produced zero candidates with visible `TBROK/TCONF/TFAIL` markers. The stable806 candidate pool remains 16/50, and `LTP_STABLE_CASES` remains unchanged at `756 total / 756 unique / 0 duplicate`.
