# Milestone 03 stable656 candidate mining report

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Live stable baseline: `606 total / 606 unique / 0 duplicate`

## Purpose

Record the current post-stable606 candidate search state so later G009/G010 work does not re-mine exhausted evidence or mistake scout-only rows for promotion proof.

## Current four-way-clean pool

| Case | Evidence | Status |
| --- | --- | --- |
| `adjtimex01` | RV/LA targeted reruns after generic `clock_adjtime(CLOCK_REALTIME, ...)` dispatch through existing adjtimex semantics, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `adjtimex03` | RV/LA targeted reruns after generic `clock_adjtime(CLOCK_REALTIME, ...)` dispatch through existing adjtimex semantics, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `epoll_create1_01` | RV/LA targeted final reruns after generic eventpoll descriptor creation and flag validation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `epoll_create1_02` | RV/LA targeted final reruns after generic eventpoll descriptor creation and flag validation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `fcntl11_64` | RV fcntl/FD scout plus LA clean2 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `fcntl15` | RV fcntl/FD scout plus LA clean2 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `fstatfs01` | RV/LA LTP_DEV=/dev/vda and NAME_MAX retest, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `fstatfs01_64` | RV/LA LTP_DEV=/dev/vda and NAME_MAX retest, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `fsync02` | current parser-clean RV/LA x musl+glibc proof recorded in this milestone evidence set | candidate, not promoted until +50 batch |
| `futex_wait01` | current parser-clean RV/LA x musl+glibc proof recorded in this milestone evidence set | candidate, not promoted until +50 batch |
| `futex_wait03` | RV/LA targeted reruns after synthetic /proc futex-sleeping repair, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `futex_wait05` | RV/LA targeted reruns after precise timer-list wakeup plus periodic tick preservation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mincore02` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mincore03` | RV/LA targeted reruns after lazy-VMA-aware mincore plus mapped-range mlock prefaulting, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mincore04` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mmap13` | RV/LA targeted reruns after file-backed mmap beyond-EOF pages are protected and delivered as catchable SIGBUS, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mmap20` | RV/LA targeted reruns after generic mmap fd/flag validation and mapped-range munlock validation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mprotect02` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mprotect04` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `munlock02` | RV/LA targeted reruns after generic mmap fd/flag validation and mapped-range munlock validation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `munmap01` | RV/LA targeted reruns after catchable synchronous SIGSEGV delivery for unmapped user faults, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `openat02` | RV/LA targeted reruns after generic sparse logical-size/data handling for large-file holes, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `rename01` | RV/LA rename inode-preservation proof, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `rename03` | RV/LA rename directory replacement proof after generic source/destination type handling, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `rename04` | RV/LA rename directory replacement proof after generic source/destination type handling, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `rename05` | RV/LA LTP_DEV=/dev/vda and NAME_MAX retest, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `sched_setaffinity01` | current parser-clean RV/LA x musl+glibc proof recorded in this milestone evidence set | candidate, not promoted until +50 batch |
| `shmat04` | RV/LA targeted reruns after generic `shmctl(IPC_STAT)` Linux 64-bit `shmid_ds` copy-size repair, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `shmt04` | RV/LA time/signal/SysV shm targeted reruns, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `signal01` | RV/LA targeted reruns after synthetic /proc state reporting covered signal waiters, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `sigaltstack02` | RV/LA targeted reruns after per-thread sigaltstack syscall-state and errno handling, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `stat03` | RV/LA stat/readlink path traversal proof after nonrecursive parent-search repair, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `stat03_64` | RV/LA stat/readlink path traversal proof after nonrecursive parent-search repair, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `statfs01` | RV/LA LTP_DEV=/dev/vda and NAME_MAX retest, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `statvfs01` | RV/LA LTP_DEV=/dev/vda and NAME_MAX retest, both musl/glibc parser-clean | candidate, not promoted until +50 batch |

Clean current audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean35-shmat04-ipcstat-abi-20260602T150918Z.md`.
Latest incremental report: `target/ltp-1000-milestone-03-stable656/combined-shmat04-shmt04-ipcstat-abi-20260602T150918+0800.promotion-candidates.txt`.
Current pool is **35/50** for stable656; 15 more trustworthy unique cases are required before editing `LTP_STABLE_CASES`.

A stale combined report that included old mixed scouts may mark now-clean rows blocked because it contains pre-fix `TBROK/TFAIL/TCONF` rows; do not use those artifacts for current promotion accounting.

## Exhausted closed full-sweep evidence

Closed arch sweep inputs:

- `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log`
- `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log`

Derived artifacts:

- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`
- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.not-stable.txt`
- `target/ltp-1000-milestone-03-stable656/rv-arch002-full-matrix-20260601T224223Z.json`
- `target/ltp-1000-milestone-03-stable656/la-arch012-full-matrix-20260601T224223Z.json`

Result: the historical sweep report has 563 four-way-clean rows, but zero rows remain outside live stable606. Do not count this evidence toward stable656; use it only as a blocker map.

## Highest-value blocked rows from current evidence

| Case/lane | Current blocker | Next useful action |
| --- | --- | --- |
| `readlinkat02` | RV clean; LA glibc clean; LA musl `TFAIL=1`; musl turns `bufsize == 0` into a dummy one-byte `readlinkat` syscall, so the kernel only sees `bufsiz=1` | keep non-promotable as a libc/test boundary; do not special-case kernel `bufsiz=1` because direct Linux `readlinkat(..., bufsiz=1)` truncation must remain valid |
| pre-fix `openat02` row | old post-statfs-clamp isolated RV rerun had musl+glibc `TBROK` setup `ENOSPC` | superseded by post-sparse-largefile four-way proof; keep old log only as blocker history |
| `nice04` | RV musl `nice(-10)` gets `EACCES`; direct `setpriority02` source requires `EACCES` for the same unprivileged lowering class | keep blocked; see `nice04-errno-boundary-report.md`; do not flip `sys_setpriority` errno |
| `clone04` | RV glibc clean, but RV musl is killed by SIGSEGV/TBROK; raw log points to the upstream musl `clone.c` NULL-stack wrapper fix | keep blocked; classify libc-wrapper boundary first, then require RV musl closure plus clone/vfork/futex/signal/wait regressions before any LA rerun or promotion |
| `kill10` | isolated RV singleton timeout + persistent frame leak + following glibc allocator panic | keep out of broad process/signal shards until cleanup/resource lifetime is fixed; poll/exit cleanup hypothesis rejected |
| `mmap05` | RV now clean, but LA musl+glibc still report `TFAIL=1` / SIGSEGV signal not received; explicit TLB-flush experiment and temporary LA instrumentation did not produce a page fault for the write-protected access | LoongArch write-protect/page-modify lane; keep non-promotable until LA `mmap05` is parser-clean and mmap/signal regressions stay clean |
| G009 mlock/mmap/mprotect RV blockers | historical RV scout blockers are partially superseded: `mmap20` and `munlock02` are now clean candidates; remaining visible blockers include `mlock02`, `mlock05`, `mlock201`, `mlock202`, `mlock203`, `mlockall02`, `mlockall03`, `munlockall01`, `mprotect01`, `mprotect03`, `mmap08`, `mmap16`, and `mmap18` with parser-visible `TFAIL/TBROK/TCONF` | keep remaining rows visible as blocker map; do not LA-confirm or count until RV becomes parser-clean |
| `shmat1` | long/hung mixed scout | SysV shm/resource lifetime lane, isolated timeout first |

## Promotion decision

No `LTP_STABLE_CASES` edit is justified yet. The candidate pool is 35/50 for stable656, and all remaining blocker rows retain their parser-visible `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/SIGSEGV` caveats.

## `epoll_create02` focused rescout

A singleton RV/LA rescout on 2026-06-02 kept `epoll_create02` outside the clean pool:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt` — 1 PASS / 1 FAIL, `TCONF=2`, `TFAIL=2`, `ENOSYS=2`; RV musl `epoll_create(0/-1)` reports `ENOSYS` instead of `EINVAL`.
- LA summary: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt` — 2 PASS / 0 FAIL, but `TCONF=2` for the architecture-level old `__NR_epoll_create` variant.

Next useful action: treat this as an epoll/libc-wrapper compatibility boundary, not as promotion evidence. A future fix must be generic and prove RV + LA x musl + glibc parser-clean results without hiding the upstream old-ABI `TCONF` row.


## G009 mm/mlock/mmap scout update

Latest artifacts:

- RV scout: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.txt`
- LA clean4 confirmation: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.txt`
- Combined clean14 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`

Decision: add `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` to the future promotion pool. The RV scout also records 15 surrounding blocker cases with `TFAIL/TBROK/TCONF`; those rows are not eligible for LA confirmation or promotion accounting. Pool at that checkpoint was 14/50; later updates bring the current pool to 35/50, so no stable-list edit is made.

## `statfs01` family RV setup-device blocker

RV-only scout artifact: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt`.

| Case/lane | Current blocker | Next useful action |
| --- | --- | --- |
| `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01` | RV musl+glibc all fail in LTP device setup with `TBROK=8`; raw log reports `No free devices found` / `Failed to acquire device`; promotion report has 0 candidates / 4 blocked | classify guest block-device/free-device support before rerunning; do not treat these rows as statfs ABI proof, do not LA-confirm until RV setup is parser-clean |

This historical scout did not change the then-current pool (14/50); later generic repairs supersede the setup-blocker classification where noted, and the current pool is 30/50 for stable656.

## VFS-C mknod/rename RV setup-device blocker

RV-only scout artifact: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt`.

| Case/lane | Current blocker | Next useful action |
| --- | --- | --- |
| `mknod07`, `mknodat02`, `rename03`, `rename04`, `rename05` | RV musl+glibc all fail in LTP device setup with `TBROK=14`; raw log reports `No free devices found` / `Failed to acquire device`; promotion report has 0 candidates / 5 blocked | classify guest block-device/free-device support before rerunning; do not treat these rows as mknod/rename ABI proof, do not LA-confirm until RV setup is parser-clean |

This historical scout did not change the then-current pool (14/50); later generic repairs supersede the setup-blocker classification where noted, and the current pool is 30/50 for stable656.

## FD/fcntl scout update

Latest artifacts:

- RV scout: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.txt`
- LA clean2 confirmation: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.txt`
- Combined clean21 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`

Decision: add `fcntl11_64` and `fcntl15` to the future promotion pool. The RV scout also records ten surrounding blocker cases (`fcntl17`, `fcntl24`, `fcntl25`, `fcntl26`, `fcntl27`, `fcntl31`, `fcntl34`, `fcntl37`, `fcntl38`, `fcntl39`) with timeout/TCONF/TFAIL/TBROK; those rows are not eligible for LA confirmation or promotion accounting. Pool at that checkpoint was 21/50; later updates bring the current pool to 35/50, so no stable-list edit is made.

## Rename03/rename04 clean2 update

Latest artifacts:

- RV rename proof: `target/ltp-1000-milestone-03-stable656/rv-rename-dir-overwrite-20260602T050256Z.summary.txt`
- LA rename proof: `target/ltp-1000-milestone-03-stable656/la-rename-dir-overwrite-20260602T050346Z.summary.txt`
- RV clean-only statfs/rename05 retarget: `target/ltp-1000-milestone-03-stable656/rv-statfs-rename05-clean-retarget-20260602T050521Z.summary.txt`
- Combined clean24 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean24-rename03-04-20260602T050630Z.promotion-candidates.txt`

Decision: add `rename03` and `rename04` to the future promotion pool. The previous mixed `rv-device-cases-ltpdev-namemax-retest` log is retained as blocker history but intentionally excluded from the clean24 combined parser report because it contains pre-fix `rename03/rename04` TFAIL rows. Pool at that checkpoint was 24/50; later updates bring the current pool to 35/50, so no stable-list edit is made.


## epoll_create1 clean2 update

Latest artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt`
- Incremental clean2 report: `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.txt`
- RV FD regression: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.txt`
- LA FD regression: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.txt`
- Combined clean30 audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean30-epoll-create1-20260602T061430Z.md`

Decision: add `epoll_create1_01` and `epoll_create1_02` to the future promotion pool. The implementation is generic `__NR_epoll_create1` support with `EPOLL_CLOEXEC` and unknown-flag `EINVAL` handling, backed by a synthetic `anon_inode:[eventpoll]` FD entry sufficient for creation/close/dup/fcntl tests. Current pool is 35/50 after later time/signal/SysV shm updates, so no stable-list edit is made.

`epoll_create02` stays blocked and non-countable: musl's `epoll_create(size)` wrapper reaches the kernel as `epoll_create1(0)`, so the invalid old-size argument is not visible at the kernel syscall boundary without breaking valid `epoll_create1(0)`.


## clock_adjtime/sigaltstack/shmat04 clean5 update

Latest artifacts:

- RV clock/sigaltstack/shmt04 summary: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt`
- LA clock/sigaltstack/shmt04 summary: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt`
- RV shmat04/shmt04 IPC_STAT ABI summary: `target/ltp-1000-milestone-03-stable656/rv-shmat04-shmt04-ipcstat-abi-20260602T150702+0800.summary.txt`
- LA shmat04/shmt04 IPC_STAT ABI summary: `target/ltp-1000-milestone-03-stable656/la-shmat04-shmt04-ipcstat-abi-20260602T150805+0800.summary.txt`
- Current clean35 audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean35-shmat04-ipcstat-abi-20260602T150918Z.md`

Decision: add `adjtimex01`, `adjtimex03`, `sigaltstack02`, `shmt04`, and `shmat04` to the future promotion pool. `shmat04` specifically depends on the generic `shmctl(IPC_STAT)` user ABI repair that copies the 112-byte Linux 64-bit `shmid_ds` layout instead of zeroing a guessed 128-byte range. Current pool is 35/50, so no stable-list edit is made.

## time/timer scout clean3 update

Latest artifacts:

- RV scout: `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.summary.txt`
- LA clean3 confirmation: `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.summary.txt`
- Combined clean38 report: `target/ltp-1000-milestone-03-stable656/combined-time-timer-clean3-20260602T152824+0800.promotion-candidates.txt`

Decision: add `getitimer02`, `setitimer02`, and `times03` to the future promotion pool. The RV scout also records 22 surrounding blocker cases with visible TCONF/TBROK/TFAIL/ENOSYS/timeout markers; those rows are not eligible for LA confirmation or promotion accounting. The current pool is 38/50, so no stable-list edit is made.

Promising next mining direction: move back to VFS/path and small FD/IO rows for the remaining 12 candidates; timerfd/POSIX timer rows need real timerfd/POSIX timer syscall support and should not be broad-promoted from partial TPASS.
