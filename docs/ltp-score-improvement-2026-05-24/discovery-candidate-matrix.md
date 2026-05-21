# Discovery candidate matrix for the next LTP run

## Baseline and source evidence

- Stable baseline: `docs/ltp-score-improvement-2026-05-23/stable75-targeted-matrix.md` and `docs/ltp-score-improvement-2026-05-23/rv-stable75-targeted-summary.txt` / `la-stable75-targeted-summary.txt` show 75 cases per libc per arch, all green.
- Inventory source: `docs/ltp-score-improvement-2026-05-23/raw/worker1-discovery-inventory.json` reports 2370 common executable LTP cases and 2307 outside the 63-case stable list.
- Candidate pool: 40 cases outside stable, all common to RV/LA images.

## Recommended first targeted batch

Twelve low-risk cases, chosen from the process/session/getter neighborhood first so the next run can classify whether the current scheduler/metadata ABI is already clean before touching time/signal or filesystem neighbors.

| Case | Likely touchpoint | Why this batch order |
| --- | --- | --- |
| `getpgid01` | `process_abi.rs` | Simple process getter / scheduler metadata neighbor |
| `getpgid02` | `process_abi.rs` | Simple process getter / scheduler metadata neighbor |
| `getsid01` | `process_abi.rs` | Simple process getter / scheduler metadata neighbor |
| `getsid02` | `process_abi.rs` | Simple process getter / scheduler metadata neighbor |
| `getppid02` | `process_lifecycle.rs` | Simple process getter / scheduler metadata neighbor |
| `getrlimit02` | `resource_sched.rs` | Simple process getter / scheduler metadata neighbor |
| `getrusage02` | `system_info.rs` | Simple process getter / scheduler metadata neighbor |
| `gettimeofday02` | `time_abi.rs` | Simple process getter / scheduler metadata neighbor |
| `uname02` | `system_info.rs` | Simple process getter / scheduler metadata neighbor |
| `uname04` | `system_info.rs` | Simple process getter / scheduler metadata neighbor |
| `sched_get_priority_max01` | `resource_sched.rs` | Simple process getter / scheduler metadata neighbor |
| `sched_get_priority_min01` | `resource_sched.rs` | Simple process getter / scheduler metadata neighbor |

## Full candidate pool outside stable

| # | Case | Category | Likely touchpoint | Selected batch |
| --- | --- | --- | --- | --- |
| 1 | `getpgid01` | proc/session/getter | `process_abi.rs` | yes |
| 2 | `getpgid02` | proc/session/getter | `process_abi.rs` | yes |
| 3 | `getsid01` | proc/session/getter | `process_abi.rs` | yes |
| 4 | `getsid02` | proc/session/getter | `process_abi.rs` | yes |
| 5 | `getppid02` | proc/session/getter | `process_lifecycle.rs` | yes |
| 6 | `getrlimit02` | proc/session/getter | `resource_sched.rs` | yes |
| 7 | `getrusage02` | proc/session/getter | `system_info.rs` | yes |
| 8 | `gettimeofday02` | proc/session/getter | `time_abi.rs` | yes |
| 9 | `uname02` | proc/session/getter | `system_info.rs` | yes |
| 10 | `uname04` | proc/session/getter | `system_info.rs` | yes |
| 11 | `sched_get_priority_max01` | proc/session/getter | `resource_sched.rs` | yes |
| 12 | `sched_get_priority_min01` | proc/session/getter | `resource_sched.rs` | yes |
| 13 | `sched_getscheduler01` | proc/session/getter | `resource_sched.rs` | no |
| 14 | `clock_getres01` | time/signal | `time_abi.rs` | no |
| 15 | `clock_gettime01` | time/signal | `time_abi.rs` | no |
| 16 | `nanosleep01` | time/signal | `time_abi.rs` | no |
| 17 | `nanosleep02` | time/signal | `time_abi.rs` | no |
| 18 | `pause01` | time/signal | `signal_abi.rs` | no |
| 19 | `rt_sigprocmask01` | time/signal | `signal_abi.rs` | no |
| 20 | `rt_sigprocmask02` | time/signal | `signal_abi.rs` | no |
| 21 | `sigpending02` | time/signal | `signal_abi.rs` | no |
| 22 | `sigprocmask01` | time/signal | `signal_abi.rs` | no |
| 23 | `sigsuspend01` | time/signal | `signal_abi.rs` | no |
| 24 | `kill02` | time/signal | `signal_abi.rs` | no |
| 25 | `kill05` | time/signal | `signal_abi.rs` | no |
| 26 | `sigaction02` | time/signal | `signal_abi.rs` | no |
| 27 | `rt_sigaction02` | time/signal | `signal_abi.rs` | no |
| 28 | `access02` | fs/syscall-neighbor | `fd_table.rs` | no |
| 29 | `access04` | fs/syscall-neighbor | `fd_table.rs` | no |
| 30 | `dup03` | fs/syscall-neighbor | `fd_table.rs` | no |
| 31 | `pipe02` | fs/syscall-neighbor | `fd_pipe.rs` | no |
| 32 | `lseek02` | fs/syscall-neighbor | `fd_table.rs` | no |
| 33 | `mkdir02` | fs/syscall-neighbor | `fd_table.rs` | no |
| 34 | `link02` | fs/syscall-neighbor | `metadata.rs` | no |
| 35 | `unlink05` | fs/syscall-neighbor | `fd_table.rs` | no |
| 36 | `rename01` | fs/syscall-neighbor | `fd_table.rs` | no |
| 37 | `statfs01` | fs/syscall-neighbor | `metadata.rs` | no |
| 38 | `fstatfs01` | fs/syscall-neighbor | `metadata.rs` | no |
| 39 | `statvfs01` | fs/syscall-neighbor | `metadata.rs` | no |
| 40 | `readlinkat01` | fs/syscall-neighbor | `metadata.rs` | no |

## Summary expectations for the selected batch

- If the 12-case batch is clean on both arches and both libc variants, each per-arch summary should report `PASS LTP CASE: 24` and `FAIL LTP CASE: 0`.
- Internal markers should remain zero for this batch: no `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, `timeout`, or `panic/trap`.
- The first failure surface to inspect, if any case regresses, should be the mapped touchpoint file: `process_abi.rs`, `process_lifecycle.rs`, `resource_sched.rs`, `system_info.rs`, or `time_abi.rs` for the getter batch.
- Do not reclassify `read02` into this matrix; it remains the transparent stable TCONF case from the baseline and is not part of this next candidate batch.

## Guardrails

- No `.omx/ultragoal` mutation from worker lanes.
- Timeout remains a failure signal, not PASS.
- Promote only after fresh LA/RV x musl/glibc evidence, not from stale candidate lists.

