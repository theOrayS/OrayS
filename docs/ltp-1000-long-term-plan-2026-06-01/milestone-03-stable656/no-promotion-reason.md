# Milestone 03 stable656 no-promotion reason

This checkpoint found twenty-one four-way-clean future candidates, but no stable promotion is performed yet.

## Why no stable list update happened

- Live stable baseline: `606 total / 606 unique / 0 duplicate`.
- Next milestone target: `656 unique`.
- Current four-way-clean new candidate pool: 21 (`fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename05`, `sched_setaffinity01`, `signal01`, `statfs01`, `statvfs01`).
- Required promotion batch size for this milestone: 50 unique cases with RV + LA x musl + glibc wrapper PASS and parser-clean summaries.

Because the 21-case candidate pool is below the +50 milestone boundary, `LTP_STABLE_CASES` remains unchanged.

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
- Keep `fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename05`, `sched_setaffinity01`, `signal01`, `statfs01`, `statvfs01` in `promotion-candidates.md` for the next accumulation batch.

## `epoll_create02` blocker update

The 2026-06-02 singleton rescout confirms `epoll_create02` remains non-promotable:

- RV: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt` reports RV musl wrapper FAIL 33 with `TFAIL=2` and `ENOSYS=2`; RV glibc wrapper PASSes but still has `TCONF=1`.
- LA: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt` has wrapper PASS for both libcs but `TCONF=2` remains.

Because the promotion gate requires wrapper PASS **and** parser-clean summaries on RV + LA x musl + glibc, this row is excluded from the candidate pool; after the later G009 clean4 update the pool was 14/50 before the LTP device/NAME_MAX clean5 update below.


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

These four rows are excluded because the promotion gate requires RV + LA x musl + glibc wrapper PASS and parser-clean summaries. The stable list remained unchanged and the pool remained 14/50 at that checkpoint; the later clean5 update below supersedes this setup-blocker classification for the five now-clean rows.

## VFS-C mknod/rename no-promotion update

The RV scout for `mknod07`, `mknodat02`, `rename03`, `rename04`, and `rename05` did not add candidates:

- Summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt`
- Parser result: 0 PASS / 10 FAIL, with `TBROK=14` and zero timeout/ENOSYS/panic/trap.
- Raw-log diagnosis: LTP setup reports `No free devices found` / `Failed to acquire device`.

These five rows are excluded because the promotion gate requires RV + LA x musl + glibc wrapper PASS and parser-clean summaries. The stable list remained unchanged and the pool remained 14/50 at that checkpoint; the later clean5 update below supersedes this setup-blocker classification for the five now-clean rows.

## LTP device/NAME_MAX clean5 no-promotion update

The generic `LTP_DEV=/dev/vda` plus synthetic block-device exposure and true `NAME_MAX=63` repair moved five rows from setup-blocker history into the clean candidate pool:

- RV final retest summary: `target/ltp-1000-milestone-03-stable656/rv-device-cases-ltpdev-namemax-retest-20260602T041654Z.summary.txt`
- LA clean5 summary: `target/ltp-1000-milestone-03-stable656/la-device-clean5-ltpdev-namemax-retest-20260602T041803Z.summary.txt`
- Combined clean19 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean19-ltpdev-namemax-20260602T041803Z.promotion-candidates.txt`

The newly clean cases are `fstatfs01`, `fstatfs01_64`, `rename05`, `statfs01`, and `statvfs01`, increasing the pool from 14/50 to 19/50. This still does not cross the stable656 +50 gate, so `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.

The same evidence keeps remaining rows visible and non-promotable: `mknod07` and `mknodat02` need guest `mkfs.ext2` support or another generic ext2 setup path; `rename03` and `rename04` now reach real assertions but still report parser-visible `TFAIL`. No blacklist/SKIP/status0/TCONF/TBROK/TFAIL row is counted.

## FD/fcntl clean2 no-promotion update

The 2026-06-02 RV fcntl scout plus LA confirmation added two future candidates without crossing the stable656 gate:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.txt`
- Combined clean21 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`

The newly clean cases are `fcntl11_64` and `fcntl15`, increasing the pool from 19/50 to 21/50. This still does not cross the stable656 +50 gate, so `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.

The same RV scout keeps remaining fcntl rows visible and non-promotable: `fcntl17` timeout; `fcntl24`, `fcntl25`, `fcntl26`, and `fcntl37` TCONF; `fcntl27` and `fcntl31` TFAIL; `fcntl34`, `fcntl38`, and `fcntl39` TBROK. No blacklist/SKIP/status0/TCONF/TBROK/TFAIL/timeout row is counted.

## Rename01 clean1 no-promotion update

The 2026-06-02 rename metadata/inode fix added one future candidate without crossing the stable656 gate:

- RV rename01 singleton summary: `target/ltp-1000-milestone-03-stable656/rv-rename01-inode-confirm-20260602T044855Z.summary.txt`
- LA rename01 singleton summary: `target/ltp-1000-milestone-03-stable656/la-rename01-inode-confirm-20260602T044855Z.summary.txt`
- Combined clean22 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean22-rename01-inode-20260602T044855Z.promotion-candidates.txt`

The newly clean case is `rename01`, increasing the pool from 21/50 to 22/50. This still does not cross the stable656 +50 gate, so `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.

The broad RV VFS/path scout remains blocker evidence only: wrapper-PASS `statx01`/`getdents02` rows include parser-visible `TCONF`; hard-link/linkat rows expose `ENOSYS`/TCONF/setup blockers; `stat03`, `stat03_64`, `getdents01`, and `readlink03` retain TFAIL semantics; missing guest binaries are not evidence. No blacklist/SKIP/status0/TCONF/TBROK/TFAIL/timeout row is counted.
