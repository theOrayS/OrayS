# milestone-06 current no-promotion reason

This is an interim stable806 checkpoint. The current baseline remains stable756: `prctl08`, `prctl09`, and `utsname02` are now four-combo clean candidates after real timerslack and shared-UTS repairs, but they are only 3 unique new cases and do not satisfy the next 50-case milestone gate.

Reasons promotion is still blocked at this checkpoint:

1. The old archived 4/4 clean-not-stable seed list has already been exhausted by earlier milestones; no remaining old clean seed exists outside current stable756.
2. The broader proc/synthetic/sched scout still has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows outside `prctl08`/`prctl09`.
3. The time/fd/signal scout still has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows outside this repair lane.
4. The clean timerslack pair and UTS shared-hostname row have RV + LA × musl + glibc evidence, and the UTS adjacent subset is clean; however the candidate pool is still far below the required next +50 unique stable milestone.
5. No blacklist/SKIP/status0/full-sweep partial TPASS evidence is counted.

Next safe slices:

- Keep `prctl08` and `prctl09` in the stable806 candidate pool and batch them only with enough additional four-combo clean cases to reach the next 50-case milestone.
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
