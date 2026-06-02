# Milestone 03 stable656 promotion candidates

This file records the current candidate pool for the next +50 stable milestone. It is **not** a stable-list update.

## Current four-way clean candidates

Clean combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean8-openat02-sparse-largefile-20260602T014245Z.promotion-candidates.txt`
- Required arches: `rv,la`
- Required libcs: `musl,glibc`
- Promotion candidates: 8
- Blocked/incomplete cases in this clean proof set: 1 (`mmap05`, LA `TFAIL`)

| Case | Evidence | Decision |
| --- | --- | --- |
| `fsync02` | after the generic `statfs`/`fstatvfs` capacity clamp, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait01` | RV isolated rerun plus LA confirmation are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait03` | after synthetic `/proc/<pid>/stat` reports futex waiters as sleeping, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait05` | after generic sub-tick timer-list wakeups plus preserving the periodic tick deadline, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `munmap01` | after catchable synchronous `SIGSEGV` delivery for unmapped user faults, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `mmap13` | after generic file-backed mmap beyond-EOF pages are protected and translated to catchable `SIGBUS`, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `openat02` | after generic POSIX-layer sparse logical-size/data handling for large file holes, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `sched_setaffinity01` | after generic permission fix, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |

## Evidence hygiene notes

- The old RV mixed scout log contains a pre-fix `fsync02` glibc `TBROK`; it remains blocker history and is not mixed into the clean current pool.
- `rv-futex-wait01-isolated-standalone-20260601T230253Z.log` provides the clean RV futex row used by the current combined report.
- `rv-fsync02-statfs-clamp-20260601T225748Z.log` and `la-fsync02-statfs-clamp-20260601T225836Z.log` provide the current `fsync02` proof.
- `rv-futex-wait03-proc-sleep-20260601T232011Z.log` and `la-futex-wait03-proc-sleep-20260601T232052Z.log` provide the current `futex_wait03` proof; the older G009 scout timeout remains blocker history only.
- `rv-futex-wait05-periodic-fix-20260601T235234Z.log` and `la-futex-wait05-periodic-fix-20260601T235323Z.log` provide the current `futex_wait05` proof; interrupted/terminated LA regression attempts are retained as non-countable repair history only.
- `rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.log` and `la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.log` provide the current `munmap01` proof. The same LA targeted log keeps `mmap05` blocked with `TFAIL=1` on both libcs, so only `munmap01` enters the clean pool.
- `rv-mmap13-sigbus-final-20260602T012111Z.log` and `la-mmap13-sigbus-final-20260602T012141Z.log` provide the current `mmap13` proof; adjacent mmap/signal regression summaries are `rv-mmap13-sigbus-regression-20260602T011329Z.summary.txt` and `la-mmap13-sigbus-regression-20260602T011433Z.summary.txt`.
- `rv-openat02-sparse-largefile-20260602T014202Z.log` and `la-openat02-sparse-largefile-20260602T014245Z.log` provide the current `openat02` proof; adjacent VFS/FD regression summaries are `rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.summary.txt` and `la-openat02-adjacent-stable-clean-regression-20260602T014545Z.summary.txt`. The earlier `rv-openat02-post-statfs-scout-20260601T231156Z.log` remains pre-fix blocker history only.

## Blocked / incomplete rows outside the clean pool

`readlinkat02` is RV-clean and LA-glibc-clean but LA musl still has `TFAIL`, so it is not eligible. The current root-cause audit treats it as a libc/test boundary: musl converts user `bufsize == 0` into a one-byte dummy syscall, and a kernel-side `bufsiz=1` special case would break valid Linux truncation semantics. `clone04` is RV glibc-clean but RV musl is killed by SIGSEGV/TBROK; the singleton log points to a musl `clone.c` wrapper boundary, so it stays outside the clean pool. `mmap05` remains blocked on LA musl+glibc `TFAIL` even though RV is clean. `nice05`, `mincore03`, `shmat1`, `atof01`, `fptest01`, `fptest02`, `epoll_create02`, `diotest4`, `select02`, and `execve05` remain blocked or incomplete for the reasons in `validation.md` and the historical combined/scout reports. The pre-fix `fsync02` `TBROK` row is superseded by post-fix proof, but the old log remains documented as failed evidence.

## Closed arch-sweep mining result

Closed sweep artifact:

- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`

Result: the report contains 563 historical four-way-clean candidates overall, but the live-stable606 filter file is empty. No additional not-yet-stable four-way-clean case can be promoted from these closed logs.

## Stable-list decision

Do not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES` yet. The live baseline remains `606 total / 606 unique / 0 duplicate`; this milestone target is `656`, so a milestone commit that promotes stable cases requires 50 trustworthy unique candidates, not 8.
