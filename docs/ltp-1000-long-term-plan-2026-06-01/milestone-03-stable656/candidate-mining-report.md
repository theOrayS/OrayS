# Milestone 03 stable656 candidate mining report

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Live stable baseline: `606 total / 606 unique / 0 duplicate`

## Purpose

Record the current post-stable606 candidate search state so later G009/G010 work does not re-mine exhausted evidence or mistake scout-only rows for promotion proof.

## Current four-way-clean pool

| Case | Evidence | Status |
| --- | --- | --- |
| `fcntl11_64` | RV fcntl/FD scout plus LA clean2 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `fcntl15` | RV fcntl/FD scout plus LA clean2 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `fsync02` | RV/LA targeted post-`generic_statfs` clamp runs, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `futex_wait01` | RV isolated rerun + LA confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `futex_wait03` | RV/LA targeted reruns after synthetic `/proc/<pid>/stat` futex-sleeping repair, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `futex_wait05` | RV/LA targeted reruns after generic precise timer-list wakeup plus periodic tick preservation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mincore03` | RV/LA targeted reruns after lazy-VMA-aware `mincore` plus mapped-range `mlock` prefaulting, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mincore02` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mincore04` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mprotect02` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mprotect04` | RV G009 mm/mlock/mmap scout plus LA clean4 confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `munmap01` | RV/LA targeted reruns after catchable synchronous `SIGSEGV` delivery for unmapped user faults, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `mmap13` | RV/LA targeted reruns after file-backed mmap beyond-EOF pages are protected and delivered as catchable `SIGBUS`, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `openat02` | RV/LA targeted reruns after generic sparse logical-size/data handling for large-file holes, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `sched_setaffinity01` | RV/LA targeted postfix runs, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `signal01` | RV/LA targeted reruns after synthetic `/proc/<pid>/stat` sleeping-state reporting covered `rt_sigsuspend` and libc `pause()`/`ppoll` waiters, both musl/glibc parser-clean | candidate, not promoted until +50 batch |

Clean combined report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`.

A stale combined report that included the old RV mixed scout still marks `fsync02` blocked because it contains the pre-fix glibc `TBROK`; do not use that artifact for current promotion accounting.

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
| G009 mlock/mmap/mprotect RV blockers | latest RV scout keeps `mlock02`, `mlock05`, `mlock201`, `mlock202`, `mlock203`, `mlockall02`, `mlockall03`, `munlock02`, `munlockall01`, `mprotect01`, `mprotect03`, `mmap08`, `mmap16`, `mmap18`, and `mmap20` blocked by parser-visible `TFAIL/TBROK/TCONF` | keep visible as blocker map; do not LA-confirm or count until RV becomes parser-clean |
| `shmat1` | long/hung mixed scout | SysV shm/resource lifetime lane, isolated timeout first |

## Promotion decision

No `LTP_STABLE_CASES` edit is justified. The candidate pool is 21/50 for stable656, and all remaining blocker rows retain their parser-visible `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/SIGSEGV` caveats.

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

Decision: add `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` to the future promotion pool. The RV scout also records 15 surrounding blocker cases with `TFAIL/TBROK/TCONF`; those rows are not eligible for LA confirmation or promotion accounting. Current pool is 21/50, so no stable-list edit is made.

## `statfs01` family RV setup-device blocker

RV-only scout artifact: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt`.

| Case/lane | Current blocker | Next useful action |
| --- | --- | --- |
| `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01` | RV musl+glibc all fail in LTP device setup with `TBROK=8`; raw log reports `No free devices found` / `Failed to acquire device`; promotion report has 0 candidates / 4 blocked | classify guest block-device/free-device support before rerunning; do not treat these rows as statfs ABI proof, do not LA-confirm until RV setup is parser-clean |

This scout does not change the current four-way-clean pool. It remains 14/50 for stable656.

## VFS-C mknod/rename RV setup-device blocker

RV-only scout artifact: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt`.

| Case/lane | Current blocker | Next useful action |
| --- | --- | --- |
| `mknod07`, `mknodat02`, `rename03`, `rename04`, `rename05` | RV musl+glibc all fail in LTP device setup with `TBROK=14`; raw log reports `No free devices found` / `Failed to acquire device`; promotion report has 0 candidates / 5 blocked | classify guest block-device/free-device support before rerunning; do not treat these rows as mknod/rename ABI proof, do not LA-confirm until RV setup is parser-clean |

This scout does not change the current four-way-clean pool. It remains 14/50 for stable656.

## FD/fcntl scout update

Latest artifacts:

- RV scout: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.txt`
- LA clean2 confirmation: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.txt`
- Combined clean21 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`

Decision: add `fcntl11_64` and `fcntl15` to the future promotion pool. The RV scout also records ten surrounding blocker cases (`fcntl17`, `fcntl24`, `fcntl25`, `fcntl26`, `fcntl27`, `fcntl31`, `fcntl34`, `fcntl37`, `fcntl38`, `fcntl39`) with timeout/TCONF/TFAIL/TBROK; those rows are not eligible for LA confirmation or promotion accounting. Current pool is 21/50, so no stable-list edit is made.
