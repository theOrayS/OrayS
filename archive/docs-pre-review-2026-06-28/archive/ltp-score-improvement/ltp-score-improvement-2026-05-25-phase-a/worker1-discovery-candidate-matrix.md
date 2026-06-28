# Worker 1 discovery candidate matrix: stable300 -> stable350/380

Date: 2026-05-25
Worker: `worker-1`
Scope: discovery only. No `.omx/ultragoal` mutation, no `examples/shell/src/cmd.rs::LTP_STABLE_CASES` edit, and no QEMU/eval promotion gate was run from this worker lane.

## Source-of-truth refresh

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: **300 total / 300 unique / 0 duplicates**.
- Existing stable300 final gate: `docs/ltp-score-improvement-2026-05-24-phase-a/stable300-delivery-report.md` records RV and LA final stable gates at **PASS LTP CASE 600 / FAIL 0** with known `read02` `TCONF=4` caveat and no timeout/ENOSYS/panic-trap.
- Prior clean promotion-candidate reports from top-level `output_rv.md`/`output_la.md` are now guardrail/baseline evidence only: their 62 candidates are already stable300 except the transparent `read02` TCONF case.
- Static LTP inventory exists through historical sdcard/bin lists, not a live `testsuite/` directory in this worktree. Candidate presence is therefore discovery evidence, not promotion evidence.

## Promotion rule used for this matrix

A case is **promotion-clean** only after leader-owned serialized RV+LA x musl+glibc targeted evidence shows wrapper PASS and zero internal `TFAIL`/`TBROK`/new `TCONF`, timeout, ENOSYS, and panic/trap. This worker matrix ranks what to test or repair next; it does not add cases to stable.

Status legend: `P` = clean PASS in inspected summary; `F(...)` = wrapper/internal failure class observed; `P+TCONF` = wrapper PASS but not clean; `?` = no known direct combo evidence in inspected phase-a docs.

## Highest-value candidate matrix

| Case / batch | Subsystem | Priority score | RV glibc | RV musl | LA glibc | LA musl | Blocker class | Recommended action |
| --- | --- | ---: | --- | --- | --- | --- | --- | --- |
| `sched_getscheduler02` | process/sched | 91 | P | P | P | F(TFAIL) | LA musl TFAIL tail | First stable315 scout; one combo remains, likely narrow scheduler/negative-pid semantic. |
| `gettid02` | process/thread id | 86 | F(TBROK) | P | ? | ? | RV glibc TBROK | Pair with thread/proc identity repair; cheap single-combo tail before broad wait work. |
| `gethostname02` | UTS/proc | 84 | P | F(TFAIL) | ? | ? | RV musl TFAIL | Retest after UTS hostname semantics; likely small musl-visible edge. |
| `alarm05`, `alarm07` | time/signal | 82 | P | F(TFAIL) | ? | ? | RV musl TFAIL | Time-signal light batch; avoid deeper timeout-heavy clock cases until these are classified. |
| `write05` | write/I/O | 82 | P | F(TFAIL) | ? | ? | RV musl TFAIL | Small write semantics tail; pair with fd/iovec lane if worker capacity exists. |
| `fchmod05` | permissions/VFS | 78 | P | F(TBROK) | ? | ? | RV musl TBROK | Permission/metadata tail; classify with `fchmod02` but do not mix with statx ENOSYS batch. |
| `fstat03`, `fstat03_64` | metadata/stat | 78 | P | F(TFAIL) | ? | ? | RV musl TFAIL | Metadata mode/stat-size tail; good after permission errno fixes. |
| `statfs02`, `fstatfs02` | statfs/statvfs | 76 | P | F(TFAIL) | ? | ? | RV musl TFAIL | Statfs follow-up; likely shared statfs struct/errno surface. |
| `nice04`, `sbrk01` | process / memory brk | 72 | P | F(TFAIL) | ? | ? | RV musl TFAIL | Useful scouts but lower priority than clustered permission/proc tails. |
| `kill11`, `kill12` | signal | 70 | P | F(timeout) | ? | ? | RV musl timeout | Keep out of promotion until timeout cause is understood; signal wait/death semantics risk. |
| `waitid07`, `waitid08`, `waitid10` | wait/process | 69 | F(TFAIL/TBROK) | F(TFAIL/TBROK) | ? | ? | RV waitid residual failures | Residual waitid tranche after `waitid05/06/09/11` promotion; repair first, no direct promotion. |
| `access02`, `access04`, `chmod05` | permissions/VFS/errno | 68 | F(TFAIL/TBROK) | F(TFAIL/TBROK) | ? | ? | real RV permission failures | User-priority cluster; high hidden-test value, but not clean. Needs errno/permission repair before retest. |
| `pipe2_02` | pipe/fd flags | 66 | F(TBROK) | F(TBROK) | ? | ? | RV TBROK | Pair with fd/pipe lane; stable300 already promoted `pipe2_01`/`pipe2_04`, so this is the remaining pipe2 tail. |
| `waitpid01` | wait/process | 65 | F(TFAIL) | F(TFAIL) | ? | ? | RV TFAIL, musl worse | High value but not near-clean; needs wait4/status semantics repair. |
| `writev03` | iovec/write SIGPIPE | 64 | F(TCONF) | F(TCONF) | ? | ? | TCONF, not clean | Keep transparent; do not count as clean even if wrapper shape improves. |
| `statx01`, `statx03`..`statx12` | statx/metadata | 62 | F(TFAIL/TBROK/TCONF/ENOSYS) | F(TFAIL/TBROK/TCONF/ENOSYS) | ? | ? | statx ENOSYS/TBROK/TCONF spread | Isolate as repair story; valuable if implemented, but too risky for first promotion batch. |
| `mmap04`, `mmap05`, `mmap06`, `mprotect01`, `mprotect02`, `munmap01` | mmap/mprotect | 58 | F(TFAIL/TBROK/event-fail) | F(TFAIL/TBROK/event-fail) | ? | ? | memory protection/signal failures | High hidden-test value, but not stable315 material without real VM/signal repair. |
| `link02`, `link04`, `link05`, `link08`, `rename01`, `rename03`..`rename06`, `readlink03`, `readlinkat01`, `readlinkat02`, `lseek02`, `write04`, `lseek11` | fs namespace/link/readlink | 55 | F(TFAIL/TBROK/TCONF/ENOSYS) | F(TFAIL/TBROK/TCONF/ENOSYS) | ? | ? | legacy fs ENOSYS/TBROK/TFAIL | Treat as a broad VFS repair tranche, not a quick promotion tranche. |
| `clock_gettime01`, `clock_nanosleep01`..`03`, `nanosleep01`, `nanosleep02`, `pause01`, `kill02` | time/signal | 52 | F(TFAIL/TBROK/timeout) | F(TFAIL/TBROK/timeout) | ? | ? | timeout and timing semantics | Keep behind lighter `alarm05/07` and `write05`; timeout cannot be promoted. |
| Static inventory-only cases: `clock_gettime03`, `clock_gettime04`, `clock_getres01`, `sigpending02`, `getrusage02`, `getrusage03`, `setpriority01`, `fstatfs01`, `statvfs01`, `symlinkat01`, `dup05`, `mkdir02`, `unlink05`, `pipe02` | mixed | 45 | ? | ? | ? | ? | unknown/currently unproven | Good broad discovery batch only. Must be leader-serialized before any promotion decision. |

## Recommended targeted batches

### Batch A: first near-clean stable315 scout

Goal: find 5-10 clean additions quickly from one-combo tails, while keeping blockers visible.

```text
sched_getscheduler02,gettid02,gethostname02,alarm05,alarm07,write05,fchmod05,fstat03,fstat03_64,statfs02,fstatfs02,nice04,sbrk01
```

Expected outcome: many will still need one musl/glibc fix; if any become RV+LA x musl+glibc clean, leader can carve a smaller promotion tranche. Do **not** include `kill11/kill12` in the first clean batch because they already show RV musl timeout.

### Batch B: user-priority blocker repair/regression batch

```text
access02,access04,chmod05,statx01,writev03,pipe2_02,waitpid01,mmap04,mmap05,mmap06,mprotect01,mprotect02,munmap01
```

Expected outcome: repair guidance and regression protection, not immediate promotion. `statx01` and mmap/mprotect cases should be split if early logs show ENOSYS/TBROK/timeouts dominating.

### Batch C: process/sched/wait residuals

```text
waitid07,waitid08,waitid10,sched_getparam03,sched_rr_get_interval03,sched_setparam04,sched_setparam05,sched_setscheduler01,sched_setscheduler02,sched_setscheduler04,setpgid03,setpriority03,sched_getscheduler02
```

Expected outcome: identify whether a scheduler negative-pid/param fix can unlock several cases. `sched_getscheduler02` is the only near-clean case in this batch from prior evidence.

### Batch D: VFS/stat/link/readlink repair tranche

```text
statx03,statx04,statx05,statx06,statx07,statx08,statx09,statx10,statx11,statx12,readlink03,readlinkat01,readlinkat02,lseek02,link02,link04,link05,link08,rename01,rename03,rename04,rename05,rename06,write04,lseek11
```

Expected outcome: no direct promotion until ENOSYS/TBROK/TFAIL classes are repaired; useful for Worker 2 VFS/metadata prioritization.

### Batch E: static inventory-only broad discovery

```text
clock_gettime03,clock_gettime04,clock_getres01,sigpending02,getrusage02,getrusage03,setpriority01,fstatfs01,statvfs01,symlinkat01,dup05,mkdir02,unlink05,pipe02
```

Expected outcome: classify availability/status only. This list came from historical inventory and should not be used as clean evidence.

## Evidence map

- `docs/ltp-score-improvement-2026-05-24-phase-a/candidate-matrix.md`: stable300 promoted/deferred overview and user-priority blocker status.
- `docs/ltp-score-improvement-2026-05-24-phase-a/stable300-delivery-report.md`: final stable300 RV/LA aggregate summary and `read02` caveat.
- `docs/ltp-score-improvement-2026-05-24-phase-a/target-near-clean-rv-summary.txt`: near-clean RV tails (`gethostname02`, `gettid02`, `alarm05`, `alarm07`, `write05`).
- `docs/ltp-score-improvement-2026-05-24-phase-a/target-stable300-scout-rv-a-summary.txt`: time/signal/process blockers and `nice04` clean-on-RV-glibc only.
- `docs/ltp-score-improvement-2026-05-24-phase-a/target-stable300-scout-rv-b-summary.txt`: fd/metadata/fcntl blockers plus `fchmod05`, `fstat03`, `fstat03_64` RV glibc-only clean observations.
- `docs/ltp-score-improvement-2026-05-24-phase-a/target-post285-scout3-rv-summary.txt`: `statfs02`/`fstatfs02` RV glibc-only clean observations.
- `docs/ltp-score-improvement-2026-05-24-phase-a/target-waitid-extended-rv-summary.txt`: residual waitid blockers.
- `docs/ltp-score-improvement-2026-05-24-phase-a/target-priority-repair-rv-summary.txt` and `user-priority-ae-rv-summary.txt`: user-priority blocker evidence.
- `docs/ltp-score-improvement-2026-05-22-phase-a/raw/discovery-inventory.json`: static candidate-pool source; useful only as inventory/discovery evidence.
- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-*-ltp-bin-cases.txt`: historical LTP binary inventory; no live `testsuite/` directory was present in this worktree.

## Subagent integration

- Subagents spawned: 2 (`Pascal` = phase-a docs/candidate extraction, `Mill` = stable300/static inventory diff).
- Integrated findings:
  - `Mill` found 38 non-stable candidates in the historical `candidate_pool`, grouped into process/cred/sched, time/signal, and fs/vfs/fd discovery batches.
  - `Pascal` highlighted high-value residuals: waitid blockers, near-clean proc/UTS/thread tails, alarm/write one-combo tails, and user-priority permission/statx blockers.
- Serial searches before spawn: 1 (`omx explore` was attempted for path mapping and later timed out; normal repo inspection continued).

## Guardrails / gaps

- No QEMU discovery or promotion gate was run by this worker.
- Existing worker/QEMU logs from prior phases are treated as discovery unless leader serializes isolated promotion gates.
- `read02` remains transparent `pass_with_tconf`; this matrix does not recommend laundering it into clean promotion status.
- LA status is unknown for most post-stable300 candidates because inspected phase-a scout logs are primarily RV targeted summaries; leader should require LA confirmation before any stable315/330/350 edit.
