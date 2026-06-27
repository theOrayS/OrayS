# stable656 candidate pool clean38: time/timer clean3 evidence

Date: 2026-06-02 15:28 +0800
Stable baseline: 606 total / 606 unique / 0 duplicate
Promotion state: not promoted; stable656 +50 gate still needs 12 more four-way-clean unique cases.

## New four-way-clean candidates

- `getitimer02`
- `setitimer02`
- `times03`

## Evidence artifacts

- RV scout raw log: `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.log`
- RV scout summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.derived.sha256`
- RV-only candidate report: `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.promotion-candidates.txt`
- LA confirmation raw log: `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.log`
- LA confirmation summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.derived.sha256`
- LA-only candidate report: `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.promotion-candidates.txt`
- Combined RV+LA report/checksum: `target/ltp-1000-milestone-03-stable656/combined-time-timer-clean3-20260602T152824+0800.promotion-candidates.txt`, `target/ltp-1000-milestone-03-stable656/combined-time-timer-clean3-20260602T152824+0800.derived.sha256`

## Parser-clean result

- RV scout: 8 wrapper PASS / 42 wrapper FAIL; parser-clean candidates are `getitimer02`, `setitimer02`, and `times03`; visible blockers include TCONF/TBROK/TFAIL/ENOSYS/timeout rows and are not counted.
- LA confirmation: 6 wrapper PASS / 0 wrapper FAIL for the three candidates; 0 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.
- Combined RV+LA candidate report: 3 candidates, 22 blocked/incomplete rows.

## Current not-yet-promoted candidate pool (38/50)

- adjtimex01
- adjtimex03
- epoll_create1_01
- epoll_create1_02
- fcntl11_64
- fcntl15
- fstatfs01
- fstatfs01_64
- fsync02
- futex_wait01
- futex_wait03
- futex_wait05
- getitimer02
- mincore02
- mincore03
- mincore04
- mmap13
- mmap20
- mprotect02
- mprotect04
- munlock02
- munmap01
- openat02
- rename01
- rename03
- rename04
- rename05
- sched_setaffinity01
- setitimer02
- shmat04
- shmt04
- signal01
- sigaltstack02
- stat03
- stat03_64
- statfs01
- statvfs01
- times03

## Blocked rows from the same RV scout

The following rows remain outside the pool because the parser shows visible caveats. They are neither blacklisted nor counted as PASS:

- clock_getres01
- clock_gettime01
- clock_gettime03
- clock_nanosleep01
- clock_nanosleep03
- clock_settime03
- nanosleep02
- setitimer01
- timer_delete01
- timer_delete02
- timer_getoverrun01
- timer_gettime01
- timer_settime01
- timer_settime02
- timer_settime03
- timerfd01
- timerfd02
- timerfd04
- timerfd_create01
- timerfd_gettime01
- timerfd_settime01
- timerfd_settime02
