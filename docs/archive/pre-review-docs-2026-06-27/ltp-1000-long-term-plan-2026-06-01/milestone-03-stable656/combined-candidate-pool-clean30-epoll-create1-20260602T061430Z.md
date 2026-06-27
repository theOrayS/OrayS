# Combined candidate pool clean30: epoll_create1

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Stable baseline: `606 total / 606 unique / 0 duplicate`
Milestone target: `656 unique`

## Newly added clean cases

| Case | RV proof | LA proof | Decision |
| --- | --- | --- | --- |
| `epoll_create1_01` | `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt` | `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt` | RV + LA x musl+glibc wrapper PASS and parser-clean; keep in future pool |
| `epoll_create1_02` | `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt` | `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt` | RV + LA x musl+glibc wrapper PASS and parser-clean; keep in future pool |

Incremental promotion-candidate report: `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.txt` — 2 candidates, 0 blocked/incomplete rows.

## Parser-clean summaries

| Gate | Summary | PASS LTP CASE | FAIL LTP CASE | Internal TFAIL/TBROK/TCONF | timeout | ENOSYS | panic/trap |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: |
| RV targeted `epoll_create1_01 epoll_create1_02` | `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt` | 4 | 0 | 0 (`{}`) | 0 | 0 | 0 |
| LA targeted `epoll_create1_01 epoll_create1_02` | `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt` | 4 | 0 | 0 (`{}`) | 0 | 0 | 0 |
| RV adjacent FD/flags regression | `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.txt` | 12 | 0 | 0 (`{}`) | 0 | 0 | 0 |
| LA adjacent FD/flags regression | `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.txt` | 12 | 0 | 0 (`{}`) | 0 | 0 | 0 |

Regression subset: `close01 fcntl01 fcntl05 dup01 pipe2_01 poll01`. These rows protect ordinary FD allocation/dup/close, `FD_CLOEXEC`-adjacent fcntl behavior, pipe flags, and poll readiness after introducing an eventpoll descriptor.

## Current future promotion pool

Current four-way-clean not-yet-promoted candidate count: **30/50**.
Remaining before stable656 +50 gate: **20**.

epoll_create1_01
epoll_create1_02
fcntl11_64
fcntl15
fstatfs01
fstatfs01_64
fsync02
futex_wait01
futex_wait03
futex_wait05
mincore02
mincore03
mincore04
mmap13
mmap20
mprotect02
mprotect04
munlock02
munmap01
openat02
rename01
rename03
rename04
rename05
sched_setaffinity01
signal01
stat03
stat03_64
statfs01
statvfs01

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`; no stable promotion is allowed until 50 trustworthy unique candidates are closed.

## Non-countable epoll_create02 blocker

`epoll_create02` is still excluded. The glibc/axlibc `epoll_create(size)` path now rejects `size <= 0`, but the musl wrapper reaches the kernel as `epoll_create1(0)` for `epoll_create(size)`, so the kernel cannot observe the invalid old `size` argument without breaking valid `epoll_create1(0)` semantics. The latest RV repair-history summary `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-create1-20260602T060510Z.summary.txt` therefore keeps parser-visible musl `TFAIL`; earlier LA singleton evidence also retained old-ABI `TCONF`. This row is not blacklisted, not hidden, and not counted as PASS.
