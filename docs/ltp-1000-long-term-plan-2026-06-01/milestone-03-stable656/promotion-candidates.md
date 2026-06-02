# Milestone 03 stable656 promotion candidates

This file records the current candidate pool for the next +50 stable milestone. It is **not** a stable-list update.

## Current four-way clean candidates

Clean combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean5-periodic-fix-20260601T235428Z.promotion-candidates.txt`
- Required arches: `rv,la`
- Required libcs: `musl,glibc`
- Promotion candidates: 5
- Blocked/incomplete cases in this clean proof set: 0

| Case | Evidence | Decision |
| --- | --- | --- |
| `fsync02` | after the generic `statfs`/`fstatvfs` capacity clamp, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait01` | RV isolated rerun plus LA confirmation are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait03` | after synthetic `/proc/<pid>/stat` reports futex waiters as sleeping, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait05` | after generic sub-tick timer-list wakeups plus preserving the periodic tick deadline, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `sched_setaffinity01` | after generic permission fix, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |

## Evidence hygiene notes

- The old RV mixed scout log contains a pre-fix `fsync02` glibc `TBROK`; it remains blocker history and is not mixed into the clean current pool.
- `rv-futex-wait01-isolated-standalone-20260601T230253Z.log` provides the clean RV futex row used by the current combined report.
- `rv-fsync02-statfs-clamp-20260601T225748Z.log` and `la-fsync02-statfs-clamp-20260601T225836Z.log` provide the current `fsync02` proof.
- `rv-futex-wait03-proc-sleep-20260601T232011Z.log` and `la-futex-wait03-proc-sleep-20260601T232052Z.log` provide the current `futex_wait03` proof; the older G009 scout timeout remains blocker history only.
- `rv-futex-wait05-periodic-fix-20260601T235234Z.log` and `la-futex-wait05-periodic-fix-20260601T235323Z.log` provide the current `futex_wait05` proof; interrupted/terminated LA regression attempts are retained as non-countable repair history only.

## Blocked / incomplete rows outside the clean pool

`readlinkat02` is RV-clean but LA musl still has `TFAIL`, so it is not eligible. `nice05`, `mincore03`, `shmat1`, `atof01`, `fptest01`, `fptest02`, `epoll_create02`, `diotest4`, `select02`, and `execve05` remain blocked or incomplete for the reasons in `validation.md` and the historical combined/scout reports. The pre-fix `fsync02` `TBROK` row is superseded by post-fix proof, but the old log remains documented as failed evidence.

## Closed arch-sweep mining result

Closed sweep artifact:

- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`

Result: the report contains 563 historical four-way-clean candidates overall, but the live-stable606 filter file is empty. No additional not-yet-stable four-way-clean case can be promoted from these closed logs.

## Stable-list decision

Do not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES` yet. The live baseline remains `606 total / 606 unique / 0 duplicate`; this milestone target is `656`, so a milestone commit that promotes stable cases requires 50 trustworthy unique candidates, not 5.
