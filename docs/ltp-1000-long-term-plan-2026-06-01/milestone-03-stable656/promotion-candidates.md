# Milestone 03 stable656 promotion candidates

This file records the current candidate pool for the next +50 stable milestone. It is **not** a stable-list update.

## Current four-way clean candidates

Combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-20260601T223023Z.promotion-candidates.txt`
- Required arches: `rv,la`
- Required libcs: `musl,glibc`
- Promotion candidates: 2
- Blocked/incomplete cases: 13

| Case | Evidence | Decision |
| --- | --- | --- |
| `futex_wait01` | RV mixed scout plus LA confirmation are both parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `sched_setaffinity01` | after generic permission fix, RV and LA targeted gates are parser-clean for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |

## Blocked / incomplete rows from the same combined candidate report

`readlinkat02` is RV-clean but LA musl still has `TFAIL`, so it is not eligible. `fsync02`, `nice05`, `mincore03`, `shmat1`, `futex_wait05`, `atof01`, `fptest01`, `fptest02`, `epoll_create02`, `diotest4`, `select02`, and `execve05` remain blocked or incomplete for the reasons in `validation.md` and the combined parser report.

## Closed arch-sweep mining result

Closed sweep artifact:

- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`

Result: the report contains 563 historical four-way-clean candidates overall, but the live-stable606 filter file is empty. No additional not-yet-stable four-way-clean case can be promoted from these closed logs.

## Stable-list decision

Do not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES` yet. The live baseline remains `606 total / 606 unique / 0 duplicate`; this milestone target is `656`, so a milestone commit that promotes stable cases requires 50 trustworthy unique candidates, not 2.
