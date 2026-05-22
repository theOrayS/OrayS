# Worker 2 PROC/SCHED/WAIT/RLIMIT lane report

Date: 2026-05-22
Team: `phase-d-ltp-stable-sc-ae18f5c7`
Task: 3 (`PROC/SCHED/WAIT/RLIMIT lane`)

## Scope and guardrails

- Lane candidates: `getpgrp`, `getgroups`, `getpgid`, `waitpid`, `waitid`, `getrusage`, `gettimeofday`, `priority`, `rlimit`, `sched_getscheduler02`.
- This is an evidence/candidate-matrix delivery. It does **not** edit `LTP_STABLE_CASES`, does **not** checkpoint `.omx/ultragoal`, and does **not** claim promotion.
- Source of truth used here is local phase-c evidence available in this worktree:
  - `docs/ltp-score-improvement-2026-05-22-phase-c/task8-output-rv-summary.txt`
  - `docs/ltp-score-improvement-2026-05-22-phase-c/task8-output-la-summary.txt`
  - `docs/ltp-score-improvement-2026-05-22-phase-c/task8-current-final-promotion-candidates.txt`
  - `docs/ltp-score-improvement-2026-05-22-phase-c/next-session-prompt-stable115-to-150.md`
  - `docs/ltp-score-improvement-2026-05-22-phase-c/wave-a-targeted.cases`
  - `docs/ltp-score-improvement-2026-05-22-phase-c/wave-a-targeted-rv.status`
  - Supplemental phase-b targeted evidence under `docs/ltp-score-improvement-2026-05-22-phase-b/` for rlimit/getgroups/wait/priority history.

## Evidence summary

### Already clean in available final promotion evidence

The phase-c task8 summaries show clean RV and LA glibc/musl rows for these lane cases:

| Case | Evidence | Recommendation |
| --- | --- | --- |
| `getpgrp01` | RV PASS rows in `task8-output-rv-summary.txt:73-74`; LA PASS rows in `task8-output-la-summary.txt:73-74`; promotion candidate row in `task8-current-final-promotion-candidates.txt:41` | Safe regression-watch candidate; no syscall fix indicated. |
| `getrlimit01` | RV PASS rows in `task8-output-rv-summary.txt:81-82`; LA PASS rows in `task8-output-la-summary.txt:81-82`; promotion candidate row in `task8-current-final-promotion-candidates.txt:45` | Safe regression-watch candidate; no syscall fix indicated. |
| `getrusage01` | RV PASS rows in `task8-output-rv-summary.txt:83-84`; LA PASS rows in `task8-output-la-summary.txt:83-84`; promotion candidate row in `task8-current-final-promotion-candidates.txt:46` | Safe regression-watch candidate; no syscall fix indicated. |
| `gettimeofday01` | RV PASS rows in `task8-output-rv-summary.txt:87-88`; LA PASS rows in `task8-output-la-summary.txt:87-88`; promotion candidate row in `task8-current-final-promotion-candidates.txt:48` | Safe regression-watch candidate; no syscall fix indicated. |
| `wait401` | RV PASS rows in `task8-output-rv-summary.txt:137-138`; LA PASS rows in `task8-output-la-summary.txt:137-138`; promotion candidate row in `task8-current-final-promotion-candidates.txt:72` | Safe regression-watch candidate; no syscall fix indicated. |

The same candidate report says there are 62 promotion candidates and only one blocked/incomplete case (`read02`) in that final-candidate matrix (`task8-current-final-promotion-candidates.txt:7-8,76-79`). This supports the rows above but does not prove the wider proc/sched/wait/rlimit wave-a pool.


### Supplemental phase-b targeted history

Phase-b artifacts provide earlier failure signatures and one already-applied rlimit/getgroups repair trail:

| Evidence | Finding | Phase-d implication |
| --- | --- | --- |
| `worker2-rlimit-getgroups-rv.summary.txt:17-24,27-34` | Before the rlimit fix, `getgroups01` and `setrlimit02` passed, while `getrlimit03` failed on both RV libcs with 16 ENOSYS matches each and `setrlimit01` failed. | Confirms the original `getrlimit03` issue was a real missing-syscall/ABI gap, not a parser artifact. |
| `worker2-rlimit-getgroups-rv-after-getrlimit.summary.txt:17-24,27-34` | After the rlimit fix, `getrlimit03` passed on both RV libcs; `setrlimit01` still failed and musl timed out with code 137. | Treat `getrlimit03` as a regression/proof candidate, while keeping `setrlimit01` as a real blocker. |
| `wave-a-targeted-rv-partial-summary.txt:19-47,49-58` | Broad RV musl wave shows failures/timeouts across `getpgid01`, `getpriority03`, `getrusage03`, `gettimeofday02/03`, `setpriority02/03`, `setrlimit01`, and waitpid blockers; `sched_getscheduler02` passed clean in the same wave. | Supports isolated blocker batches and keeping timeout/TBROK/TFAIL visible. |
| `final-gate-report.md:79-93` | Final phase-b gate explicitly refused stable120 promotion due to `getrlimit03`, `sched_getscheduler02`, `setrlimit01`, waitpid and other real blockers. | Reinforces leader-owned promotion: do not add these to stable from stale or partial evidence. |

### Explicit blockers or evidence gaps from phase-c handoff

The phase-c handoff warns not to promote these lane cases without fresh targeted evidence (`next-session-prompt-stable115-to-150.md:115-166`):

| Case / family | Handoff evidence | Status for phase-d |
| --- | --- | --- |
| `getrlimit03` | Listed as RV clean but LA legacy-wrapper/ENOSYS gap, and duplicated in blocker list (`next-session-prompt...:120,125`). | Needs fresh RV+LA × glibc+musl targeted proof before leader promotion; current source has legacy LoongArch wrapper dispatch in `syscall_dispatch.rs`, so report-only unless new run shows a real remaining ABI gap. |
| `sched_getscheduler02` | Listed as RV clean but LA history not clean (`next-session-prompt...:122`). | Needs fresh LA/RV targeted proof; no local code change indicated from static dispatch alone. |
| `setrlimit01` | Listed as RV TFAIL/timeout under 20s targeted gate (`next-session-prompt...:126`). | Highest-priority rlimit write-path rerun/fix candidate. |
| `waitpid01`, `waitpid04`, `waitpid05`, `waitpid10`, `waitpid11`, `waitpid12`, `waitpid13` | Listed as blockers (`next-session-prompt...:137-143`). | Keep as isolated wait batch; do not infer pass from stable `wait401`. |
| `getrusage02` | Listed as blocker (`next-session-prompt...:162`). | Needs targeted evidence; likely not clean enough for promotion. |
| `gettimeofday02` | Listed as blocker (`next-session-prompt...:163`). | Needs targeted evidence; timeout must remain visible and cannot be counted as PASS. |

The phase-c wave-a pool already groups the lane cases for targeted execution (`wave-a-targeted.cases:1-35`) and records a 117-case RV targeted command start with 20s per-case timeout (`wave-a-targeted-rv.status:1-3`), but no completed parsed wave-a result artifact exists in this worktree. Therefore this report should drive fresh targeted batches rather than direct promotion.

## Static implementation map

Read-only source inspection found existing syscall coverage for the lane:

| Area | Current implementation anchor | Notes |
| --- | --- | --- |
| Process groups | `examples/shell/src/uspace/process_abi.rs` (`sys_getpgid`, `sys_setpgid`, `sys_getsid`) and dispatch in `syscall_dispatch.rs` | `getpgrp01` is already clean; `getpgid01` remains a targeted evidence/fix candidate from phase-c pool. |
| Groups | `examples/shell/src/uspace/credentials.rs` (`sys_getgroups`, `sys_setgroups`) and dispatch in `syscall_dispatch.rs` | `getgroups03` already appears stable; `getgroups02` is in the targeted pool and should be rerun before any fix. |
| Wait | `examples/shell/src/uspace/process_lifecycle.rs` (`sys_wait4`) and dispatch in `syscall_dispatch.rs` | `wait401` is clean; waitpid blockers should be isolated because wait status/child-state failures can be masked by broad batches. No `waitid` implementation was found in `examples/shell/src/uspace`, so `waitid*` remains report-only until a concrete failing case is assigned. |
| Resource limits / priority / scheduler | `examples/shell/src/uspace/resource_sched.rs` and dispatch in `syscall_dispatch.rs` | Legacy LoongArch `getrlimit`/`setrlimit` dispatch is present; run targeted evidence before changing ABI. |
| Time / usage | `examples/shell/src/uspace/time_abi.rs` (`sys_gettimeofday`) and `examples/shell/src/uspace/system_info.rs` (`sys_getrusage`) | `gettimeofday01`/`getrusage01` are clean; `02`/`03` variants need fresh targeted evidence. |

## Recommended targeted batches

Run these in small batches so timeout/TFAIL/TBROK remains attributable. Suggested RV first; if no panic/trap or widespread timeout, run the same batch on LA.

### Batch 1 — low-risk regression/proof candidates

```text
getpgrp01
getpgid02
getgroups03
getrlimit01
getrusage01
gettimeofday01
wait401
sched_getscheduler02
getrlimit03
```

Purpose: confirm clean candidates and resolve stale `getrlimit03` / `sched_getscheduler02` concerns with fresh dual-arch evidence. Leader decides promotion after RV+LA × glibc+musl clean rows.

### Batch 2 — proc / priority blockers

```text
getpgid01
getgroups02
getpriority03
setpriority01
setpriority02
setpriority03
```

Purpose: isolate process-group, group-list, and nice/priority semantics before touching `credentials.rs`, `process_abi.rs`, or `resource_sched.rs`.

### Batch 3 — wait blockers

```text
waitpid01
waitpid02
waitpid04
waitpid05
waitpid10
waitpid11
waitpid12
waitpid13
wait402
```

Purpose: isolate wait status and child-state behavior. Do not promote waitpid subcases from wrapper status alone; require internal marker summary.

### Batch 4 — resource/time write and invalid-input blockers

```text
getrusage02
getrusage03
gettimeofday02
gettimeofday03
setrlimit01
setrlimit03
prlimit01
prlimit02
```

Purpose: capture timeout/TCONF/TFAIL signatures and distinguish real ABI errors from harness timing.

## Suggested command shape

Replace `<cases>` with each batch as comma- or space-separated input matching the runner behavior used by this repo.

```bash
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='<cases>' \
LTP_CASE_TIMEOUT_SECS=20 \
./run-eval.sh rv 2>&1 | tee docs/ltp-score-improvement-2026-05-22-phase-d/worker-2-<batch>-rv.log

python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-d/worker-2-<batch>-rv.log \
  > docs/ltp-score-improvement-2026-05-22-phase-d/worker-2-<batch>-rv-summary.txt
```

Then repeat with `LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img ./run-eval.sh la` only after RV is not panic/trap dominated.

## Decision

No source-code patch is recommended from the currently available evidence. The existing tree already contains the rlimit wrapper/dispatch shape that phase-b showed was needed for `getrlimit03`; remaining red cases need fresh failure logs before a safe ABI patch can be scoped. The safest next action is targeted evidence collection by the leader or a dedicated runner lane, then a narrow syscall/ABI patch only for cases that still show a real failure signature.
