# Combined candidate pool clean34: clock_adjtime, sigaltstack, shmt04

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Stable baseline: `606 total / 606 unique / 0 duplicate`
Milestone target: `656 unique`

## Newly added clean cases

| Case | RV proof | LA proof | Decision |
| --- | --- | --- | --- |
| `adjtimex01` | `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt` | `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt` | RV + LA x musl+glibc wrapper PASS and parser-clean after generic `clock_adjtime(CLOCK_REALTIME, ...)` dispatch to the existing `adjtimex` semantics |
| `adjtimex03` | `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt` | `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt` | RV + LA x musl+glibc wrapper PASS and parser-clean after generic `clock_adjtime(CLOCK_REALTIME, ...)` dispatch to the existing `adjtimex` semantics |
| `shmt04` | `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt` | `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt` | RV + LA x musl+glibc wrapper PASS and parser-clean; existing SysV shm behavior is now closed by four-way evidence |
| `sigaltstack02` | `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt` | `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt` | RV + LA x musl+glibc wrapper PASS and parser-clean after generic `sigaltstack` state/errno handling |

Incremental promotion-candidate report: `target/ltp-1000-milestone-03-stable656/combined-clock-sigaltstack-shmt04-20260602T143805+0800.promotion-candidates.txt` — 4 candidates, 0 blocked/incomplete rows.

## Parser-clean summaries

| Gate | Summary | PASS LTP CASE | FAIL LTP CASE | Internal TFAIL/TBROK/TCONF | timeout | ENOSYS | panic/trap |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: |
| RV targeted `adjtimex01 adjtimex03 sigaltstack02 shmt04` | `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt` | 8 | 0 | 0 (`{}`) | 0 | 0 | 0 |
| LA targeted `adjtimex01 adjtimex03 sigaltstack02 shmt04` | `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt` | 8 | 0 | 0 (`{}`) | 0 | 0 | 0 |
| RV adjacent time/signal regression | `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.summary.txt` | 14 | 0 | 0 (`{}`) | 0 | 0 | 0 |
| LA adjacent time/signal regression | `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.summary.txt` | 14 | 0 | 0 (`{}`) | 0 | 0 | 0 |

Regression subset: `clock_gettime02 clock_nanosleep02 nanosleep01 rt_sigaction01 rt_sigprocmask01 sigaction01 sigprocmask01`. These stable rows protect the adjacent time ABI and signal-mask/action surfaces touched by this repair.

## Current future promotion pool

Current four-way-clean not-yet-promoted candidate count: **34/50**.
Remaining before stable656 +50 gate: **16**.

adjtimex01
adjtimex03
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
shmt04
signal01
sigaltstack02
stat03
stat03_64
statfs01
statvfs01

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`; no stable promotion is allowed until 50 trustworthy unique candidates are closed.
