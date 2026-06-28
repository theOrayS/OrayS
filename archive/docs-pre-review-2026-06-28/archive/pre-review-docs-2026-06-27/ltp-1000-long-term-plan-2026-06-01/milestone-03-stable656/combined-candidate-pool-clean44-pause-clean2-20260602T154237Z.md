# stable656 candidate pool clean44: pause clean2 evidence

Date: 2026-06-02 15:42 +0800
Stable baseline: 606 total / 606 unique / 0 duplicate
Promotion state: not promoted; stable656 +50 gate still needs 6 more four-way-clean unique cases.

## New four-way-clean candidates

- `pause01`
- `pause02`

## Evidence artifacts

- RV signal/wait scout raw log: `target/ltp-1000-milestone-03-stable656/rv-signal-wait-scout-20260602T154048+0800.log`
- RV scout summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-signal-wait-scout-20260602T154048+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-signal-wait-scout-20260602T154048+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-signal-wait-scout-20260602T154048+0800.derived.sha256`
- RV-only candidate report: `target/ltp-1000-milestone-03-stable656/rv-signal-wait-scout-20260602T154048+0800.promotion-candidates.txt`
- LA pause confirmation raw log: `target/ltp-1000-milestone-03-stable656/la-pause-clean2-20260602T154154+0800.log`
- LA confirmation summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-pause-clean2-20260602T154154+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-pause-clean2-20260602T154154+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/la-pause-clean2-20260602T154154+0800.derived.sha256`
- LA-only candidate report: `target/ltp-1000-milestone-03-stable656/la-pause-clean2-20260602T154154+0800.promotion-candidates.txt`
- Combined RV+LA report/checksum: `target/ltp-1000-milestone-03-stable656/combined-pause-clean2-20260602T154237+0800.promotion-candidates.txt`, `target/ltp-1000-milestone-03-stable656/combined-pause-clean2-20260602T154237+0800.derived.sha256`

## Parser-clean result

- RV scout: 4 wrapper PASS / 38 wrapper FAIL; parser-clean candidates are `pause01` and `pause02`; all other rows retain visible parser blockers or wrapper failures and are not counted.
- LA confirmation: 4 wrapper PASS / 0 wrapper FAIL for the two candidates; 0 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.
- Combined RV+LA candidate report: 2 candidates, 19 blocked/incomplete rows.

## Current not-yet-promoted candidate pool (44/50)

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
- lstat02
- lstat02_64
- mincore02
- mincore03
- mincore04
- mmap13
- mmap20
- mprotect02
- mprotect04
- munlock02
- munmap01
- open07
- open12
- openat02
- pause01
- pause02
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

The following rows remain outside the pool because RV parser output has visible caveats or wrapper-fail events. They are neither blacklisted nor counted as PASS:

- alarm01
- kill01
- kill04
- kill05
- sigaction03
- sigaction04
- sigaction05
- sigpending02
- sigprocmask02
- sigsuspend02
- sigwait01
- waitpid02
- waitpid05
- waitpid14
- waitpid15
- waitpid16
- waitpid17
- waitpid18
- waitpid19
