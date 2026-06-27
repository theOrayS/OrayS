# stable656 candidate pool clean42: open clean2 evidence

Date: 2026-06-02 15:38 +0800
Stable baseline: 606 total / 606 unique / 0 duplicate
Promotion state: not promoted; stable656 +50 gate still needs 8 more four-way-clean unique cases.

## New four-way-clean candidates

- `open07`
- `open12`

## Evidence artifacts

- RV FD/VFS/IO scout raw log: `target/ltp-1000-milestone-03-stable656/rv-fd-vfs-io-scout-20260602T153655+0800.log`
- RV scout summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-fd-vfs-io-scout-20260602T153655+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-fd-vfs-io-scout-20260602T153655+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-fd-vfs-io-scout-20260602T153655+0800.derived.sha256`
- RV-only candidate report: `target/ltp-1000-milestone-03-stable656/rv-fd-vfs-io-scout-20260602T153655+0800.promotion-candidates.txt`
- LA open confirmation raw log: `target/ltp-1000-milestone-03-stable656/la-open-clean2-20260602T153756+0800.log`
- LA confirmation summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-open-clean2-20260602T153756+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-open-clean2-20260602T153756+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/la-open-clean2-20260602T153756+0800.derived.sha256`
- LA-only candidate report: `target/ltp-1000-milestone-03-stable656/la-open-clean2-20260602T153756+0800.promotion-candidates.txt`
- Combined RV+LA report/checksum: `target/ltp-1000-milestone-03-stable656/combined-open-clean2-20260602T153844+0800.promotion-candidates.txt`, `target/ltp-1000-milestone-03-stable656/combined-open-clean2-20260602T153844+0800.derived.sha256`

## Parser-clean result

- RV scout: 4 wrapper PASS / 28 wrapper FAIL; parser-clean candidates are `open07` and `open12`; all other rows retain visible parser blockers or wrapper failures and are not counted.
- LA confirmation: 4 wrapper PASS / 0 wrapper FAIL for the two candidates; 0 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.
- Combined RV+LA candidate report: 2 candidates, 14 blocked/incomplete rows.

## Current not-yet-promoted candidate pool (42/50)

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

- chmod04
- chdir02
- getcwd05
- open05
- open11
- open14
- open15
- open16
- close08
- read03
- write04
- write07
- write08
- readv03
