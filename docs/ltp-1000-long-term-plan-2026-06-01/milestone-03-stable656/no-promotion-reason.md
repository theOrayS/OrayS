# Milestone 03 stable656 no-promotion reason

This checkpoint found fourteen four-way-clean future candidates, but no stable promotion is performed yet.

## Why no stable list update happened

- Live stable baseline: `606 total / 606 unique / 0 duplicate`.
- Next milestone target: `656 unique`.
- Current four-way-clean new candidate pool: 14 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `sched_setaffinity01`, `signal01`).
- Required promotion batch size for this milestone: 50 unique cases with RV + LA x musl + glibc wrapper PASS and parser-clean summaries.

Because the 14-case candidate pool is below the +50 milestone boundary, `LTP_STABLE_CASES` remains unchanged.

## Blocking evidence kept visible

The following blockers prevent counting additional rows:

| Case / lane | Blocking reason |
| --- | --- |
| `mmap05` | RV is now clean after catchable synchronous `SIGSEGV`, but LA musl+glibc still report `TFAIL=1` / SIGSEGV signal not received; a local explicit TLB-flush experiment and temporary instrumentation left the TFAIL unchanged |
| `mmap10_1` | missing testcase in both guest LTP trees |
| `vma02` | libnuma `TCONF` |
| old `futex_wait03` scout row | superseded timeout evidence; current RV/LA targeted reruns are parser-clean after `/proc/<pid>/stat` sleeping-state repair |
| `kill10` | isolated RV singleton still times out in musl, leaves a persistent post-cleanup frame leak, and panics in the following glibc group; temporary poll/exit-group cleanup hypothesis rejected |
| `shmat1` | mixed scout was manually terminated after hang/long run; evidence is scouting only |
| `readlinkat02` | RV and LA glibc clean, but LA musl `TFAIL`; root-cause audit found musl converts user `bufsize == 0` to a dummy one-byte syscall, so a safe generic kernel fix is not available without breaking direct `bufsiz=1` semantics |
| pre-fix `fsync02` row | old isolated RV rerun had glibc `TBROK=1`; superseded by post-fix proof but retained as failed evidence |
| pre-fix `openat02` row | old post-statfs-clamp isolated RV rerun failed both musl and glibc with `TBROK` setup `ENOSPC`; superseded by post-sparse-largefile proof but retained as failed evidence |
| `nice04` | RV musl gets `EACCES` for `nice(-10)`, but stable `setpriority02` source requires direct unprivileged `setpriority` lowering to return `EACCES`; no safe kernel errno flip |
| `clone04` | RV glibc PASSes the NULL-stack `EINVAL` check, but RV musl is killed by SIGSEGV/TBROK; raw log hints at a musl `clone.c` wrapper fix, so it is not counted or LA-confirmed from this failed RV gate |
| `openat03` | real `O_TMPFILE` remains unsupported: the rejected emulation/linkat patch produced RV panic/trap in the deep nested-directory phase; the retained generic gate returns `EOPNOTSUPP`/`EINVAL`, yielding honest `TCONF` and wrapper FAIL on RV/LA x musl/glibc |
| closed arch sweep | 563 four-way-clean historical rows, but zero not-yet-stable rows after filtering live stable606 |
| `select02`, `sched_rr_get_interval03`, `setpriority01` | wrapper PASS rows include `TCONF`; not promotion evidence |
| `nice05`, `mlock02`, `mlock05`, `mlock201`, `mlock202`, `mlock203`, `mlockall02`, `mlockall03`, `munlock02`, `munlockall01`, `mprotect01`, `mprotect03`, `mmap08`, `mmap16`, `mmap18`, `mmap20`, `atof01`, `fptest01`, `fptest02`, `epoll_create02`, `diotest4`, `execve05` | fail, TFAIL/TBROK/TCONF/ENOSYS, or incomplete arch matrix remains |

## Decision

- Do not edit `LTP_STABLE_CASES`.
- Do not count blacklist/SKIP/status0/timeout/TCONF/TBROK/TFAIL rows as PASS.
- Keep `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `sched_setaffinity01`, and `signal01` in `promotion-candidates.md` for the next accumulation batch.

## `epoll_create02` blocker update

The 2026-06-02 singleton rescout confirms `epoll_create02` remains non-promotable:

- RV: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt` reports RV musl wrapper FAIL 33 with `TFAIL=2` and `ENOSYS=2`; RV glibc wrapper PASSes but still has `TCONF=1`.
- LA: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt` has wrapper PASS for both libcs but `TCONF=2` remains.

Because the promotion gate requires wrapper PASS **and** parser-clean summaries on RV + LA x musl + glibc, this row is excluded from the candidate pool; after the later G009 clean4 update the current pool is 14/50.


## G009 clean4 no-promotion update

The latest G009 mm/mlock/mmap scout found four additional RV-clean rows and the LA confirmation proved them parser-clean on both libcs:

- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.txt`
- LA clean4 summary: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.txt`
- Combined clean14 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean14-g009-mm-mprotect-20260602T034707Z.promotion-candidates.txt`

These four rows (`mincore02`, `mincore04`, `mprotect02`, `mprotect04`) increase the pool to 14/50. They do not cross the stable656 +50 gate, so `LTP_STABLE_CASES` remains unchanged.

## `statfs01` family no-promotion update

The RV scout for `statfs01`, `fstatfs01`, `fstatfs01_64`, and `statvfs01` did not add candidates:

- Summary: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt`
- Parser result: 0 PASS / 8 FAIL, with `TBROK=8` and zero timeout/ENOSYS/panic/trap.
- Raw-log diagnosis: LTP setup reports `No free devices found` / `Failed to acquire device`.

These four rows are excluded because the promotion gate requires RV + LA x musl + glibc wrapper PASS and parser-clean summaries. The stable list remains unchanged and the pool remains 14/50.
