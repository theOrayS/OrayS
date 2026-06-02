# Milestone 03 stable656 promotion candidates

This file records the current candidate pool for the next +50 stable milestone. It is **not** a stable-list update.

## Current four-way clean candidates

Clean combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean19-ltpdev-namemax-20260602T041803Z.promotion-candidates.txt`
- Required arches: `rv,la`
- Required libcs: `musl,glibc`
- Promotion candidates: 19
- Blocked/incomplete cases in this clean proof set: 20 (`mmap05`, `mknod07`, `mknodat02`, `rename03`, `rename04`, and the current RV G009 mlock/mmap/mprotect blocker rows)

| Case | Evidence | Decision |
| --- | --- | --- |
| `fstatfs01` | after generic `LTP_DEV=/dev/vda` exposure plus synthetic block-device stat/getdents and true `NAME_MAX=63` reporting, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `fstatfs01_64` | same generic device/NAME_MAX repair as `fstatfs01`; RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `statfs01` | same generic device/NAME_MAX repair as `fstatfs01`; RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `statvfs01` | after `statvfs().f_namemax` now reports the real 63-byte backing dirent capacity, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `rename05` | after generic `LTP_DEV=/dev/vda` setup reaches the actual same-filesystem rename assertion path, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `fsync02` | after the generic `statfs`/`fstatvfs` capacity clamp, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait01` | RV isolated rerun plus LA confirmation are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait03` | after synthetic `/proc/<pid>/stat` reports futex waiters as sleeping, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait05` | after generic sub-tick timer-list wakeups plus preserving the periodic tick deadline, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `mincore03` | after `mincore` distinguishes valid lazy VMA pages from unmapped pages and `mlock` prefaults mapped ranges, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `mincore02` | RV G009 mm/mlock/mmap scout and LA clean4 confirmation are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `mincore04` | RV G009 mm/mlock/mmap scout and LA clean4 confirmation are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `mprotect02` | RV G009 mm/mlock/mmap scout and LA clean4 confirmation are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `mprotect04` | RV G009 mm/mlock/mmap scout and LA clean4 confirmation are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `munmap01` | after catchable synchronous `SIGSEGV` delivery for unmapped user faults, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `mmap13` | after generic file-backed mmap beyond-EOF pages are protected and translated to catchable `SIGBUS`, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `openat02` | after generic POSIX-layer sparse logical-size/data handling for large file holes, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `sched_setaffinity01` | after generic permission fix, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `signal01` | after generic `/proc/<pid>/stat` sleeping-state reporting for `rt_sigsuspend`/`ppoll` waiters, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |

## Evidence hygiene notes

- The old RV mixed scout log contains a pre-fix `fsync02` glibc `TBROK`; it remains blocker history and is not mixed into the clean current pool.
- `rv-futex-wait01-isolated-standalone-20260601T230253Z.log` provides the clean RV futex row used by the current combined report.
- `rv-fsync02-statfs-clamp-20260601T225748Z.log` and `la-fsync02-statfs-clamp-20260601T225836Z.log` provide the current `fsync02` proof.
- `rv-futex-wait03-proc-sleep-20260601T232011Z.log` and `la-futex-wait03-proc-sleep-20260601T232052Z.log` provide the current `futex_wait03` proof; the older G009 scout timeout remains blocker history only.
- `rv-futex-wait05-periodic-fix-20260601T235234Z.log` and `la-futex-wait05-periodic-fix-20260601T235323Z.log` provide the current `futex_wait05` proof; interrupted/terminated LA regression attempts are retained as non-countable repair history only.
- `rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.log` and `la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.log` provide the current `munmap01` proof. The same LA targeted log keeps `mmap05` blocked with `TFAIL=1` on both libcs, so only `munmap01` enters the clean pool.
- `rv-mmap13-sigbus-final-20260602T012111Z.log` and `la-mmap13-sigbus-final-20260602T012141Z.log` provide the current `mmap13` proof; adjacent mmap/signal regression summaries are `rv-mmap13-sigbus-regression-20260602T011329Z.summary.txt` and `la-mmap13-sigbus-regression-20260602T011433Z.summary.txt`.
- `rv-openat02-sparse-largefile-20260602T014202Z.log` and `la-openat02-sparse-largefile-20260602T014245Z.log` provide the current `openat02` proof; adjacent VFS/FD regression summaries are `rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.summary.txt` and `la-openat02-adjacent-stable-clean-regression-20260602T014545Z.summary.txt`. The earlier `rv-openat02-post-statfs-scout-20260601T231156Z.log` remains pre-fix blocker history only.
- `rv-signal01-poll-wait-20260602T024843Z.log` and `la-signal01-poll-wait-20260602T024926Z.log` provide the current `signal01` proof; adjacent signal/poll/proc regression summaries are `rv-signal-poll-regression-20260602T025025Z.summary.txt` and `la-signal-poll-regression-20260602T025204Z.summary.txt`. The interrupted `rv-signal01-proc-sleep-20260602T024336Z.log` is obsolete non-countable repair history only.
- `rv-mincore03-mincore-mlock-20260602T032124Z.log` and `la-mincore03-mincore-mlock-20260602T032208Z.log` provide the current `mincore03` proof; adjacent mincore/mlock/mmap regression summaries are `rv-mincore03-adjacent-regression-20260602T032259Z.summary.txt` and `la-mincore03-adjacent-regression-20260602T032401Z.summary.txt`. The earlier mixed scout `mincore03` `TBROK` rows are pre-fix blocker history only.
- `rv-g009-mm-mlock-mmap-scout-20260602T034405Z.log` and `la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.log` provide the current `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` proof. The same RV scout keeps the surrounding mlock/mmap/mprotect failures visible as blocker evidence; only the four parser-clean rows enter the pool.

## Blocked / incomplete rows outside the clean pool

`readlinkat02` is RV-clean and LA-glibc-clean but LA musl still has `TFAIL`, so it is not eligible. The current root-cause audit treats it as a libc/test boundary: musl converts user `bufsize == 0` into a one-byte dummy syscall, and a kernel-side `bufsiz=1` special case would break valid Linux truncation semantics. `clone04` is RV glibc-clean but RV musl is killed by SIGSEGV/TBROK; the singleton log points to a musl `clone.c` wrapper boundary, so it stays outside the clean pool. `mmap05` remains blocked on LA musl+glibc `TFAIL` even though RV is clean. `nice05`, `shmat1`, `mlock02`, `mlock05`, `mlock201`, `mlock202`, `mlock203`, `mlockall02`, `mlockall03`, `munlock02`, `munlockall01`, `mprotect01`, `mprotect03`, `mmap08`, `mmap16`, `mmap18`, `mmap20`, `atof01`, `fptest01`, `fptest02`, `epoll_create02`, `diotest4`, `select02`, and `execve05` remain blocked or incomplete for the reasons in `validation.md` and the historical combined/scout reports. The pre-fix `fsync02` `TBROK` row is superseded by post-fix proof, but the old log remains documented as failed evidence.

## Closed arch-sweep mining result

Closed sweep artifact:

- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`

Result: the report contains 563 historical four-way-clean candidates overall, but the live-stable606 filter file is empty. No additional not-yet-stable four-way-clean case can be promoted from these closed logs.

## Stable-list decision

Do not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES` yet. The live baseline remains `606 total / 606 unique / 0 duplicate`; this milestone target is `656`, so a milestone commit that promotes stable cases requires 50 trustworthy unique candidates, not 19.

## `openat03` non-candidate note

`openat03` is explicitly outside the clean pool. A larger `O_TMPFILE`/`linkat` emulation attempt was rejected after RV targeted validation produced a supervisor page fault during the testcase's deep nested-directory phase (`rv-openat03-otmpfile-20260602T021349Z.summary.txt` and `rv-openat03-trace-20260602T022058Z.summary.txt`, both `panic/trap matches: 1`).

The retained generic change only makes unsupported `O_TMPFILE` fail honestly: `O_TMPFILE|O_RDONLY` returns `EINVAL`, and `O_TMPFILE` against an existing directory returns `EOPNOTSUPP`. Current RV and LA targeted summaries (`rv-openat03-otmpfile-enotsup-20260602T022658Z.summary.txt`, `la-openat03-otmpfile-enotsup-20260602T022748Z.summary.txt`) show zero timeout/ENOSYS/panic/trap but `TCONF=4` and wrapper FAIL, so this is blocker evidence only and must not be counted toward stable656.

## `epoll_create02` non-candidate note

`epoll_create02` was rechecked as a singleton on RV and LA. It is still outside the clean pool: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt` shows RV musl wrapper FAIL with `TFAIL=2` and `ENOSYS=2`, while `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt` wrapper-PASSes both libcs but retains parser-visible `TCONF=2`. This is blocked evidence only; no blacklist, SKIP, status0, or caveated wrapper PASS row is counted toward stable656.


## G009 mm/mincore/mprotect clean4 update

A follow-up RV scout plus LA confirmation grew the clean pool without editing the stable list:

- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.txt` — 8 wrapper PASS / 30 wrapper FAIL, with `TFAIL=50`, `TBROK=4`, `TCONF=6`, and no timeout/ENOSYS/panic/trap.
- LA confirmation summary: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.txt` — 8 wrapper PASS / 0 wrapper FAIL, zero internal markers, timeout, ENOSYS, panic/trap.
- Combined clean14 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean14-g009-mm-mprotect-20260602T034707Z.promotion-candidates.txt`.

Newly evidenced four-way-clean cases: `mincore02`, `mincore04`, `mprotect02`, and `mprotect04`. Pool at that checkpoint: 14/50 before the later clean5 update. Stable list remains `606 total / 606 unique / 0 duplicate`.

## `statfs01` family non-candidate note

The 2026-06-02 RV scout for `statfs01`, `fstatfs01`, `fstatfs01_64`, and `statvfs01` is outside the clean pool:

- Summary: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.promotion-candidates.txt`
- Result: 0 wrapper PASS / 8 wrapper FAIL, `TBROK=8`, no timeout/ENOSYS/panic/trap.

The raw log shows LTP setup failing to acquire a free device for all four cases. Because RV is parser-unclean and LA was not run, these rows are blocker evidence only and did not affect the 14-case candidate pool at that checkpoint; the later clean5 update below supersedes the setup-blocker classification for `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` only.

## VFS-C mknod/rename non-candidate note

The 2026-06-02 RV scout for `mknod07`, `mknodat02`, `rename03`, `rename04`, and `rename05` is outside the clean pool:

- Summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.promotion-candidates.txt`
- Result: 0 wrapper PASS / 10 wrapper FAIL, `TBROK=14`, no timeout/ENOSYS/panic/trap.

The raw log shows LTP setup failing to acquire a free device for all five cases. Because RV is parser-unclean and LA was not run, these rows are blocker evidence only and did not affect the 14-case candidate pool at that checkpoint; the later clean5 update below supersedes the setup-blocker classification for `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` only.

## LTP device/NAME_MAX clean5 update

A generic device setup follow-up grew the clean pool without editing the stable list:

- RV enumeration-only retest summary: `target/ltp-1000-milestone-03-stable656/rv-device-enumeration-retest-20260602T041227Z.summary.txt` — still 0 PASS / 18 FAIL, `TBROK=22`; `/dev` enumeration alone was insufficient.
- RV `LTP_DEV=/dev/vda` pre-NAME_MAX retest summary: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-vda-device-retest-20260602T041431Z.summary.txt` — 3 PASS before `statvfs01` hit a parser-visible panic/trap; this is repair history only.
- RV final device/NAME_MAX retest summary: `target/ltp-1000-milestone-03-stable656/rv-device-cases-ltpdev-namemax-retest-20260602T041654Z.summary.txt` — `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` are parser-clean for musl+glibc; `mknod07`, `mknodat02`, `rename03`, and `rename04` retain visible blockers.
- LA clean5 confirmation summary: `target/ltp-1000-milestone-03-stable656/la-device-clean5-ltpdev-namemax-retest-20260602T041803Z.summary.txt` — the five RV-clean rows are parser-clean for musl+glibc.
- Combined clean19 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean19-ltpdev-namemax-20260602T041803Z.promotion-candidates.txt`.

Newly evidenced four-way-clean cases: `fstatfs01`, `fstatfs01_64`, `rename05`, `statfs01`, and `statvfs01`. Current pool: 19/50. Stable list remains `606 total / 606 unique / 0 duplicate`.

Blocked rows from the same proof set stay outside the pool: `mknod07` and `mknodat02` are parser-visible `TCONF` because `mkfs.ext2` is missing in the guest; `rename03` and `rename04` are parser-visible `TFAIL` rename-semantic blockers. None is blacklisted or counted as PASS.
