# Worker 5 batch-002 misc/process scout list

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Worker lane: `worker-4` executing task 9, report-only.
Required output path: `docs/ltp-score-improvement-2026-05-27-phase-a/worker5-batch002-misc-process-scout.md`.

## Scope and guardrails

- Report-only scout. I did **not** run QEMU/evaluator.
- I did **not** edit `.omx/ultragoal` or `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- This report is not promotion proof. A case is promotion-clean only after RV+LA x musl+glibc all show wrapper PASS and zero internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS/not-implemented, and panic/trap.
- Goal: give the leader a lower-risk batch-002 queue outside the already-blocked mmap/waitid/kill subset.

## Evidence inputs

- Live stable source: `examples/shell/src/cmd.rs::LTP_STABLE_CASES` parsed as **413 total / 413 unique / 0 duplicates**. `getpgid01` is not stable; `getpgid02` is stable.
- Four-way availability: `docs/ltp-score-improvement-2026-05-27-phase-a/raw/sdcard-{rv,la}-{musl,glibc}-ltp-bins.txt` from the leader worktree; every ranked row below is available in all four sdcard inventories.
- Fresh parsed raw evidence:
  - `raw/batch-001-cross-promotion-candidates.txt`: only `fcntl07,fcntl07_64` are four-way clean; `readlinkat02` is blocked by LA musl `TFAIL=1`.
  - `raw/batch-002-rv-summary.txt`: batch-002 has `PASS LTP CASE: 0`, `FAIL LTP CASE: 0`, and one panic/trap on `rv:musl:pipe02`, so the batch is truncated/unknown rather than useful promotion proof.
- LTP sources inspected from `/tmp/ltp-src-worker4/testcases/kernel/syscalls/**`.
- Local syscall surface inspected under `examples/shell/src/uspace/**`.

## Current raw truth to preserve

- Do not infer any stable-list addition from batch-002: `pipe02` panicked before a useful RV matrix was produced.
- `fcntl07,fcntl07_64` already have batch-001 four-way clean evidence and belong to leader-owned promotion handling, not this batch-002 scout report.
- `readlinkat02` remains 3/4 clean but blocked by `la:musl TFAIL=1`; do not promote it from RV-only or 3/4 evidence.
- Existing matrix already marks `kill02,waitid07,waitid08,waitid10,poll02,getcpu01,gethostid01,gethostname02,times03,getpgid01,fork13,fork14,kill05,kill10,mmap04,mmap05,munmap01,mprotect01,mprotect02,inode02` as do-not-first / blocked.

## Inline leader-run recommendation

Conservative first leader-run batch: `select04,select02,select03,pselect03,pselect02,pselect01,clock_gettime01,clock_gettime04,setrlimit05,setrlimit04,sched_rr_get_interval03,sched_setaffinity01,setpriority01,nice04,signal01`

Optional second-wave diagnostics only after the first batch is parser-clean: `readlink03,dup05,select01,sendfile07,sendfile07_64`

Hard exclusions for this batch-002 scout: `pipe02,pipe07,poll02,getcpu01,gethostid01,gethostname02,times03,getpgid01,waitid07,waitid08,waitid10,kill02,mmap04,mmap05,munmap01,mprotect01,mprotect02,inode02`

## Ranked 20-40 case scout table

| Rank | Case | Scout status | Source-level expectation | Likely blocker / demotion rule |
| ---: | --- | --- | --- | --- |
| 1 | `select04` | first-run scout | Checks fdset clearing on full/empty pipes via `select()`. | Pipe readiness must be exact; demote on any TFAIL or timeout. |
| 2 | `select02` | first-run scout | Checks `select()` timeout behavior with a pipe. | Time/yield sensitivity; demote on timeout mismatch. |
| 3 | `select03` | first-run scout | Negative/error paths: negative nfds, EBADF, EFAULT; forks child. | User-memory fault and fork/wait paths must return exact errno. |
| 4 | `pselect03` | first-run scout | Basic regular-file `pselect()` success. | Lowest pselect surface; demote on unexpected EINVAL/EFAULT. |
| 5 | `pselect02` | first-run scout | Closed fd/negative nfds/invalid timeout errors. | Requires exact EBADF/EINVAL behavior. |
| 6 | `pselect01` | first-run scout | Timeout-only `pselect()` behavior. | Time/yield sensitivity; demote on elapsed-time mismatch. |
| 7 | `clock_gettime01` | first-run scout | Multiple clock ids plus process/thread clock checks and `/proc/self/stat` comparison. | `/proc` clock accounting may be incomplete; demote on any TFAIL/TBROK. |
| 8 | `clock_gettime04` | first-run scout | Compares vDSO/syscall/gettimeofday time consistency. | Time monotonicity/resolution drift; demote on TFAIL. |
| 9 | `setrlimit05` | first-run scout | EFAULT path for `setrlimit(RLIMIT_NOFILE, bad_addr)` in child. | User-memory validation must surface EFAULT or allowed SIGSEGV path. |
| 10 | `setrlimit04` | first-run scout | Sets small stack limit, forks, then execs `/bin/true`. | Requires `/bin/true`, fork/wait/exec and stack rlimit to be compatible. |
| 11 | `sched_rr_get_interval03` | cautious first-run | Errno tests for invalid pid / unused pid / bad interval pointer after SCHED_RR setup. | `sched_setscheduler(SCHED_RR)` privilege/RT support and libc EFAULT TCONF variants. |
| 12 | `sched_setaffinity01` | cautious first-run | EFAULT/EINVAL/ESRCH/EPERM errno matrix, uses `nobody` and a child. | Affinity mask is effectively one CPU locally; EPERM/user switching may be fragile. |
| 13 | `setpriority01` | cautious first-run | Sets priority for process/process group/user. | Prior lanes saw priority TCONF/errno gaps; demote on privilege or TCONF noise. |
| 14 | `nice04` | cautious first-run | Non-root negative nice should fail with EPERM. | Depends on uid switching and priority privilege checks. |
| 15 | `signal01` | cautious first-run | SIGKILL cannot be ignored/defaulted/caught and still terminates child. | Small signal surface; demote on wait-status mismatch. |
| 16 | `readlink03` | second-wave diagnostic | `readlink()` negative paths for access, invalid args, long names, ENOENT/ENOTDIR. | VFS permissions/path semantics; not first if leader wants process-only scout. |
| 17 | `dup05` | second-wave diagnostic | Basic `dup(2)` of a named FIFO opened `O_RDWR`. | Named FIFO + fd table path; historical batch notes include TBROK/ENOSYS-style risk. |
| 18 | `select01` | second-wave diagnostic | `select()` with regular file, system pipe, and named FIFO. | FIFO/pipe setup risk; historical guardrail marked as TBROK/ENOSYS risk. |
| 19 | `close_range02` | blocked until syscall exists | Valid close range, invalid params, CLOEXEC, CLONE_FILES/UNSHARE. | Local grep found no `close_range`/`__NR_close_range` dispatch; likely ENOSYS/TCONF. |
| 20 | `close_range01` | blocked until syscall exists | Broader close-range coverage, cloning/shared-fd behavior. | Same missing dispatch plus heavier clone/fd-table semantics. |
| 21 | `poll02` | do-not-first | Poll timeout behavior. | Explicitly blocked in matrix; previous worker summary blocks `poll02`. |
| 22 | `gethostname02` | do-not-first | Hostname length/error behavior. | No direct local `gethostname` syscall dispatch found; existing matrix blocks it. |
| 23 | `gettid02` | do-not-first | Multi-thread TID uniqueness. | Prior evidence showed libc/thread/futex fragility; keep out of first batch. |
| 24 | `getpgid01` | blocked | Uses `/proc/1/stat`, forks, and compares several process group ids. | Matrix do-not-first; historical TFAIL/TBROK; `/proc` process metadata risk. |
| 25 | `setsid01` | blocked | Session creation and process-group leader EPERM behavior. | Local `sys_setsid` always sets sid/pgid and returns pid; missing EPERM for group leader. |
| 26 | `getcpu01` | blocked | Affines process to CPU, calls `getcpu`, checks CPU/node. | No local `getcpu` syscall dispatch found; matrix blocks it. |
| 27 | `times03` | blocked | Busy-loop and child CPU time accounting via `times()`. | Matrix do-not-first; local `sys_times` writes default zero tms fields. |
| 28 | `getrusage02` | blocked/TCONF | EINVAL/EFAULT style `getrusage()` coverage. | Prior lanes marked pass-with-TCONF; not promotion-safe. |
| 29 | `getrusage03` | blocked | `ru_maxrss` behavior across allocation/fork variants. | Local rusage accounting is likely too shallow/zeroed for maxrss expectations. |
| 30 | `getrusage04` | blocked | Timing/resolution expectations for rusage. | Time/accounting sensitivity and TCONF risk. |
| 31 | `clock_gettime03` | blocked | Time namespace / `timens_offsets` behavior. | Requires `unshare(CLONE_NEWTIME)` and `/proc/self/timens_offsets`; not low-risk. |
| 32 | `leapsec01` | blocked | Leap-second/adjtimex timing behavior. | `adjtimex`/time sync semantics; not a scout candidate. |
| 33 | `setrlimit06` | blocked | RLIMIT_CPU with SIGXCPU/SIGKILL behavior. | Signal/timer/CPU accounting interaction; too high-risk for batch-002. |
| 34 | `sched_getattr01` | blocked | Root/SCHED_DEADLINE set+get scheduler attributes. | Local `sched_param_accepts_policy()` rejects SCHED_DEADLINE in `sched_setattr()`. |
| 35 | `sched_setattr01` | blocked | Positive SCHED_DEADLINE plus ESRCH/EINVAL paths. | Same SCHED_DEADLINE support gap; not first-run. |
| 36 | `sched_setscheduler04` | blocked | CAP_SYS_NICE and SCHED_RESET_ON_FORK semantics. | Capability/reset-on-fork semantics are not low-risk. |
| 37 | `sysinfo03` | blocked | Time namespace `/proc/uptime` vs `sysinfo.uptime`. | Requires timens/proc uptime semantics; local `sys_sysinfo` uses monotonic uptime only. |
| 38 | `nice05` | blocked | Thread/scheduling fairness around nice. | Fair scheduling/priority semantics are too broad for scout. |
| 39 | `signal06` | blocked | FPU/altstack/mprotect/SIGSEGV signal behavior. | Depends on mprotect/altstack/FPU paths; overlaps blocked mmap/mprotect subset. |

## Subagent integration evidence

Subagents spawned: 3 (`Wegener` `019e68f0-4b79-7c33-a321-566e76df0c31`, `Kuhn` `019e68f0-701d-72b1-9f7e-21b2599d9c38`, `Darwin` `019e68f0-96a1-70b2-813e-84cbedd0e3b5`).
Subagent model requested by task contract: `gpt-5.4-mini`.
Serial searches before spawn: 0 in this continuation context; probes had already been spawned before the current report-writing pass.

Findings integrated:
- Wegener: live stable count, raw batch-001/batch-002 evidence, four-way inventory presence, and candidate family map.
- Kuhn: LTP source expectation notes for `dup`, `close_range`, `select`, `pselect`, time, rlimit, and sched families.
- Darwin: guardrails to hard-exclude `pipe02`, `pipe07`, `dup05`, `select01`, `getpgid01`, and to keep wrapper PASS separate from internal `TFAIL/TBROK/TCONF` proof.
- Corrections applied after local verification: `getpgid01` is not in current stable; `close_range*` is not first-run because no local dispatch exists; `setsid01` is blocked because local `sys_setsid` misses EPERM-for-group-leader behavior.

## Verification commands

Final delivery commit SHA: recorded in task lifecycle completion result; a commit cannot contain its own final SHA without changing that SHA.

Fresh checks run for this report-only change:

| Check | Command | Result |
| --- | --- | --- |
| Parser unit tests | `python3 -B scripts/test_ltp_summary.py` | PASS: `Ran 10 tests ... OK` |
| Live stable count | inline Python parse of `examples/shell/src/cmd.rs::LTP_STABLE_CASES` | PASS: `413 total / 413 unique / 0 duplicates`; `getpgid01 in stable: False`; `getpgid02 in stable: True` |
| First-batch availability | inline Python over `raw/sdcard-{rv,la}-{musl,glibc}-ltp-bins.txt` plus stable set | PASS: `first_batch_cases=15`, `missing_four_way={}`, `already_stable=[]` |
| Forbidden edit check | `git diff -- .omx examples/shell/src/cmd.rs` | PASS: clean/no output under `.omx` or stable-list source |
| Whitespace/lint check | `git diff --check` then `git diff --cached --check` | PASS after removing trailing whitespace from this new report |

No Rust build/typecheck or QEMU/evaluator was run: this task is report-only and explicitly forbids evaluator runs.

## Stop condition

This lane stops at a ranked, source-checked report and leader-run recommendation. No case here should be added to `LTP_STABLE_CASES` until the leader produces fresh parser-clean RV+LA x musl+glibc evidence.
