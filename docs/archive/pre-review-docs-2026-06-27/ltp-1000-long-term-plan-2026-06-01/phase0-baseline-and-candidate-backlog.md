# Phase0 baseline freeze and candidate backlog

Date: 2026-06-01
Worker: `complete-dev-1000ltp-c632b4a0/worker-1`
Task: report-only source diagnosis; no QEMU, no stable promotion, no Ultragoal checkpoint.

## Scope and stop line

This document freezes the current stable506 baseline and records a candidate backlog for the long-term stable1000 plan. It is intentionally not a promotion report: no case is added to `examples/shell/src/cmd.rs::LTP_STABLE_CASES`, no blacklist entry is treated as PASS, and no final/evaluator gate was run by this worker.

## Source-of-truth files inspected

- Plan: `docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md`.
- Current stable list: `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Archived stable506 final report: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md`.
- Archived stable506 validation: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/validation.md`.
- Candidate seed matrix: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix/candidate-matrix-stable460-to-500plus.md`.
- Clean sweep seed list: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix/clean-candidates-not-in-stable460.txt`.
- Full-sweep closed-run report: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-report.md`.
- Active blacklist files: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt`, `blacklist-rv.txt`, `blacklist-la.txt`.
- Workflow rules: `docs/agent-workflow/ltp-selection.md`, `docs/agent-workflow/ltp-promotion-and-docs.md`, `docs/agent-workflow/collaboration-and-delivery.md`.

## Baseline facts

### Live stable count

Recomputed from `examples/shell/src/cmd.rs::LTP_STABLE_CASES` in worker, clean leader worktree, and root target tree:

| Tree | Stable total | Unique | Duplicates | First / last sample |
| --- | ---: | ---: | ---: | --- |
| worker worktree | 506 | 506 | 0 | first `access01`; last `vfork02` |
| clean leader worktree | 506 | 506 | 0 | first `access01`; last `vfork02` |
| root target tree | 506 | 506 | 0 | first `access01`; last `vfork02` |

### Branch, worktree, and disk facts

| Location | Branch / state | HEAD | Dirty status | Notes |
| --- | --- | --- | --- | --- |
| worker worktree `.../worktrees/worker-1` | detached | `4b4d331d` | clean before this report | only this report file is worker-owned |
| clean leader worktree `/root/oskernel2026-orays-1000ltp-leader-20260601-1336` | detached | `4b4d331d` | clean | used for clean baseline facts |
| root target `/root/oskernel2026-orays` | `dev/1000ltp-plan` | `4b4d331d` | dirty, 393 `git status --short` entries | pre-existing user/team state; not touched by this worker |

Disk preflight for `/`, `/root/oskernel2026-orays`, and the clean leader tree reported `/dev/vda2 59G`, `24G` used, `34G` available, `42%` used. No cleanup was needed.

## Archived stable506 final evidence

Stable506 is the highest trusted baseline recorded in the archived long-term plan:

| Evidence | Result |
| --- | --- |
| `session-08-integration-final-gate/final-report.md` | Session 1-8 raised stable from 460 to 506 and explicitly did not claim stable520. |
| RV final gate | `PASS LTP CASE 1012`, `FAIL 0`, `ltp-musl 506/0`, `ltp-glibc 506/0`. |
| LA final gate | `PASS LTP CASE 1012`, `FAIL 0`, `ltp-musl 506/0`, `ltp-glibc 506/0`. |
| Parser caveat | inherited `read02` remains visible as `TCONF 4` per arch aggregate; it is not internal-clean evidence. |
| Severe signals | timeout `0`, ENOSYS `0`, panic/trap `0` in final stable gate summaries. |
| Marker-prefix audit | `0` non-prefix `LTP CASE` lines for both final logs in `validation.md`. |
| Raw evidence policy | raw logs retained under `target/ltp-long-term-session8/`; committed docs contain paths, summaries, and checksums. |

## Candidate universe: current seed backlog

The Session 1 matrix over `rv-arch002` and `la-arch012` full-sweep summaries found 106 four-combo clean sweep candidates not in stable460. Comparing that seed list with the live stable506 list shows:

- 36 of those 106 are now in stable506.
- 70 remain outside stable506 and are backlog/scouting candidates only.
- The 70 remaining rows still need fresh targeted RV + LA x musl + glibc parser-clean proof before any future promotion.

Remaining seed backlog by lane:

| Lane | Count | Backlog examples |
| --- | ---: | --- |
| time/select/signal | 8 | `clock_gettime04`, `clock_nanosleep02`, `nanosleep01`, `poll02`, `pselect01`, `pselect01_64`, `settimeofday01`, `time-schedule` |
| VFS/metadata/path | 2 | `fs_perms`, `readdir01` |
| mmap/mm/resource | 8 | `data_space`, `dirty`, `mlockall01`, `mmap-corruption01`, `mmstress_dummy`, `page01`, `page02`, `stack_space` |
| network/proc/synthetic | 4 | `accept01`, `listen01`, `socket02`, `socketpair02` |
| other / low-priority / harness | 48 | math generator rows (`gen*`), harness helpers (`tst_*`), `fanotify_child`, `modify_ldt*`, `newuname01`, `ulimit01`, `utsname01`, `utsname04`, etc. |

Rows already promoted from that seed pool include `mknod08`, `mknodat01`, `rename14`, `diotest1/2/3/5/6`, `mmap001/15/17/19`, `mprotect05`, `futex_wait02/04`, `futex_wake01`, `kill02`, `tkill01/02`, `vfork01/02`, and SysV shm/scheduler rows. Do not re-queue them as new cases; use them as regression-protection anchors.

## Candidate families and likely source surfaces

These families are backlog inputs, not promotion claims.

### 1. Low-risk VFS / FD / metadata / path frontier

- Candidate families: `statx`, `access`, `chmod/chown`, `link/unlink/rename`, `readlink/readlinkat`, `getdents64`, `statfs`, `xattr`, `fcntl`, `pipe`, `readv/writev`, `sendfile`, plus remaining `fs_perms` / `readdir01` seed rows.
- Source surfaces to inspect before fixes: `examples/shell/src/uspace/metadata.rs`, `fd_table.rs`, `runtime_paths.rs`, `synthetic_fs.rs`, `syscall_dispatch.rs`, and VFS-facing helpers.
- Known blockers from archived matrix: `readlinkat02` LA musl TFAIL, legacy `getdents` TCONF/ENOSYS, `readlink03` ELOOP, `statfs/statx` blocker rows, and FD/fcntl rows such as `fcntl30`, `pipe07`, `pipe15`, `writev03`, `pwritev03`.
- Guardrail: parent permission/search, sticky-bit, symlink-loop, directory/file errno, shared offset, `O_APPEND`, negative offset, pipe capacity, SIGPIPE, and lock lifetime semantics must remain general Linux/POSIX behavior.

### 2. Time / select / signal / process frontier

- Candidate families: `select/pselect/ppoll/poll`, `clock_gettime`, `nanosleep`, `getitimer/setitimer`, signal mask/pending/delivery, `wait/waitid`, `fork/clone/exec`, rlimit, priority, and scheduler query.
- Source surfaces: `examples/shell/src/uspace/time_abi.rs`, `select_fdset.rs`, `signal_abi.rs`, `process_lifecycle.rs`, `process_abi.rs`, `resource_sched.rs`, `syscall_dispatch.rs`.
- Archived risks: `select02` TCONF plus timeout, `getitimer01` TFAIL/ENOSYS text before Session 2 work, `futex_wait03/05` timeout/EINTR, `waitid07`, `clone02`, `execve01/05`, and aggregate-sensitive `kill02` history.
- Guardrail: no timing hacks; preserve timeout accounting, remaining-time writeback, EINTR behavior, signal-mask restore, child/reparent/wait semantics, and per-libc differences.

### 3. mmap / mm / resource frontier

- Candidate families: `mmap/mprotect/msync/mincore`, VMA split/merge, user pointer copy-in/copy-out, SIGSEGV teardown, allocator/resource high-water, and remaining seed rows `data_space`, `dirty`, `mlockall01`, `mmap-corruption01`, `page01`, `page02`, `stack_space`.
- Source surfaces: `examples/shell/src/uspace/memory_map.rs`, `user_memory.rs`, `memory_policy.rs`, `program_loader.rs`, and fault/teardown integration.
- Archived risks: `diotest4` user-buffer validation, `mprotect01` errno/protection boundaries, `mprotect02` SIGSEGV handler recovery, and LA allocator/resource telemetry.
- Guardrail: broad memory/stress rows must not hide leaks, OOM, or mapping lifetime issues behind status0/SKIP.

### 4. futex / thread / IPC frontier

- Candidate families: futex wait/wake/timeout/EINTR/key, robust-list feasible subset, SysV shm/sem/msg, process/thread teardown, pipe/socket interaction.
- Source surfaces: `examples/shell/src/uspace/futex.rs`, `process_lifecycle.rs`, `process_abi.rs`, `sysv_shm.rs`, `task_registry.rs`, `fd_pipe.rs`, `fd_socket.rs`.
- Archived risks: `futex_wait03/05` remain blocked, SysV sem/msg are not covered like the promoted shm rows, and `shmt10`/resource deltas need regression tracking.
- Guardrail: do not use busy-wait or test-specific wakeups; maintain lifetime, refcount, waiter cleanup, and exit cleanup invariants.

### 5. Network / proc / syntheticfs / LA severe-blocker frontier

- Candidate families: socket errno, `bind/listen/connect/accept`, readiness/poll, shutdown/close teardown, UNIX/TCP/UDP small batches, `/proc` fields, and syntheticfs consistency.
- Source surfaces: `examples/shell/src/uspace/fd_socket.rs`, `synthetic_fs.rs`, `system_info.rs`, `process_abi.rs`, and active blacklist docs.
- Active blacklist counts: common `5`, RV `1`, LA `374` from `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-*.txt`.
- Session 7 evidence: `creat07` and `tcp4-uni-basic01` were removed from LA-only severe blacklist after degrading to ordinary closed FAIL/TBROK/TCONF; they are not PASS and not stable evidence.
- Guardrail: blacklist removal requires normal closure without severe hang/OOM/panic/resource pollution; stable promotion still requires parser-clean four-way gates.

## Backlog priorities for the next execution phase

1. Rebuild the candidate matrix from live stable506, the archived Session 1 parser-derived data, and any newer full-sweep summaries before choosing cases.
2. First wave should prefer small, local, high-ROI rows with adjacent regression anchors: time/select seed rows, remaining VFS seed rows, and mmap/mm seed rows that do not require architecture-wide redesign.
3. Treat network/proc/syntheticfs and LA blacklist families as quality/blocker reduction until targeted evidence proves ordinary closure and then parser-clean PASS.
4. Use stable506 final gate as the regression floor. Any regression in stable506 or any promoted milestone stops expansion until fixed or demoted.
5. Every future promotion milestone must be leader-owned and parser-backed; workers may produce discovery, diagnosis, small fixes, and targeted summaries, but not promote stable cases from report-only evidence.

## Evidence caveats and non-countable inputs

- `blacklist`, `[CONTEST][LTP][SKIP]`, status0, wrapper-only success, single-arch/single-libc success, raw `TPASS` density, and closed ordinary FAIL/TBROK/TCONF are not PASS.
- Full-sweep clean rows are scouting evidence only until fresh targeted RV + LA x musl + glibc gates are parser-clean.
- `scripts/ltp_summary.py` remains the gate truth; wrapper exit code or eyeballing raw logs is insufficient.
- Raw log truncation, marker glue, missing `RUN_META`, timeout, ENOSYS, panic/trap, TFAIL/TBROK/TCONF, or bad marker prefixes make the affected evidence non-promotable until resolved and re-run.
- The inherited `read02` `TCONF` must stay visible in every aggregate report; it cannot be described as internal-clean.

## Subagent spawn evidence

Task 1 required a parallel probe. Two read-only native subagents were spawned and completed without file edits:

- `Gauss` (`019e836b-d775-73f2-a0d3-4352c3d701d8`): root-cause/regression-path probe over stable506 final report, validation, candidate matrix, and source-surface families.
- `Huygens` (`019e836b-f20c-7060-96ca-7f78fc7303f7`): safe implementation slice and migration-hazard probe over the 1000 plan, stable506 archive, and live baseline facts.

Subagent spawn evidence: both agents returned concise read-only bullets; their findings are integrated above as candidate families, source surfaces, and evidence caveats. No subagent edited files.

## Verification performed for this report

- Live stable count recomputed from `examples/shell/src/cmd.rs` in worker, leader, and root target trees: `506 total / 506 unique / 0 duplicates`.
- Git/disk facts collected from worker, clean leader, and root target trees; root target dirty state was observed but not modified.
- Active blacklist counts recomputed from checked-in blacklist files: common `5`, RV `1`, LA `374`.
- Report verification is limited to source/log path citation and Markdown diff hygiene; no QEMU/build/evaluator coverage is claimed for this report-only task.
