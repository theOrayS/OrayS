# stable656 candidate pool clean40: lstat clean2 evidence

Date: 2026-06-02 15:34 +0800
Stable baseline: 606 total / 606 unique / 0 duplicate
Promotion state: not promoted; stable656 +50 gate still needs 10 more four-way-clean unique cases.

## New four-way-clean candidates

- `lstat02`
- `lstat02_64`

## Evidence artifacts

- RV VFS/path scout raw log: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-simple-scout-20260602T153210+0800.log`
- RV scout summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-simple-scout-20260602T153210+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-vfs-path-simple-scout-20260602T153210+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-vfs-path-simple-scout-20260602T153210+0800.derived.sha256`
- RV-only candidate report: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-simple-scout-20260602T153210+0800.promotion-candidates.txt`
- LA lstat confirmation raw log: `target/ltp-1000-milestone-03-stable656/la-lstat-clean2-20260602T153351+0800.log`
- LA confirmation summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-lstat-clean2-20260602T153351+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-lstat-clean2-20260602T153351+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/la-lstat-clean2-20260602T153351+0800.derived.sha256`
- LA-only candidate report: `target/ltp-1000-milestone-03-stable656/la-lstat-clean2-20260602T153351+0800.promotion-candidates.txt`
- Combined RV+LA report/checksum: `target/ltp-1000-milestone-03-stable656/combined-lstat-clean2-20260602T153433+0800.promotion-candidates.txt`, `target/ltp-1000-milestone-03-stable656/combined-lstat-clean2-20260602T153433+0800.derived.sha256`

## Parser-clean result

- RV scout: 5 wrapper PASS / 39 wrapper FAIL; parser-clean candidates are `lstat02` and `lstat02_64`; all other rows retain visible parser blockers and are not counted.
- LA confirmation: 4 wrapper PASS / 0 wrapper FAIL for the two candidates; 0 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.
- Combined RV+LA candidate report: 2 candidates, 20 blocked/incomplete rows.

## Current not-yet-promoted candidate pool (40/50)

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

The following rows remain outside the pool because RV parser output has visible caveats. They are neither blacklisted nor counted as PASS:

- fstat02
- fstat02_64
- getcwd03
- getcwd04
- mkdir02
- mkdir03
- mkdir09
- mkdirat02
- rmdir02
- symlink03
- unlink09
- utime01
- utime02
- utime03
- utime04
- utime05
- utime06
- utime07
- utimensat01
- utimes01
