# Worker 2 STABLE200 PROC/SCHED/WAIT/RLIMIT next-candidates triage

Date: 2026-05-22
Team: `phase-d-ltp-stable-sc-ae18f5c7`
Task: 7 (`STABLE200 PROC evidence triage from current artifacts`)

## Scope and guardrails

- Read-only artifact triage only: no QEMU/evaluator run, no code edit, no `LTP_STABLE_CASES` edit, no `.omx/ultragoal` checkpoint.
- Leader constraints preserved: Do not run QEMU/evaluator; Do not edit code; Do not edit `LTP_STABLE_CASES`; Do not checkpoint `.omx/ultragoal`.
- Output target: propose the next 10-20 PROC/SCHED/WAIT/RLIMIT candidates for stable200 evidence gathering.
- Current-artifact inputs:
  - Leader cwd phase-d `wave-b-priority1-rv-summary.txt` and `wave-b-priority1-new.cases`.
  - Leader cwd phase-d `wave-a-stable180-new23-rv-summary.txt`, `wave-a-stable180-new23-la-summary.txt`, `wave-a-stable180-new23.cases`, and `stable180.cases`.
  - Worktree phase-d `worker-2-proc-sched-wait-rlimit-report.md` and `worker-2-targeted-batches.cases`.
  - Phase-c blockers in `docs/ltp-score-improvement-2026-05-22-phase-c/next-session-prompt-stable115-to-150.md`.

## Artifact readout

- `wave-a-stable180-new23-{rv,la}-summary.txt` shows the +23 wave was fully clean on RV and LA: 46 PASS LTP CASE, 0 FAIL, 0 internal TFAIL/TBROK/TCONF, 0 timeout, 0 ENOSYS, 0 panic/trap on each arch. The PROC/credential subset includes `personality01/02`, `setegid01`, `setfsgid01`, `setfsuid01`, `setgid01`, `setgid03`, `setpgid01`, `setpgid02`, `setpgrp01`, `setpgrp02`, `setregid01`, `setresgid01`, and `setresuid01`.
- `candidate-matrix.md` identifies five held clean alternates for Wave B: `setgroups01`, `setgroups02`, `setreuid01`, `setuid01`, `statx02`. The PROC/credential subset is the first four.
- `wave-b-priority1-rv-summary.txt` is FD/FS-heavy (`open04` only clean; 48 wrapper FAIL) and contains no PROC/SCHED/WAIT/RLIMIT rows. It contributes a negative signal: do not borrow FD/FS failures for PROC promotion decisions.
- Worker-2 prior report keeps `getrlimit03`, `sched_getscheduler02`, `setrlimit01`, `waitpid*`, `getrusage02`, and `gettimeofday02` out of direct promotion without fresh RV+LA × glibc+musl evidence.
- Current `stable180.cases` already contains `getrlimit03`, `sched_get_priority_max02`, `sched_get_priority_min02`, `sched_rr_get_interval02`, `waitpid04`, `waitpid10`, and `wait402`; they are therefore regression-guard cases, not first-choice stable200 additions.

## Required command template

For every candidate below, run RV first, parse with `scripts/ltp_summary.py`, then run LA only if RV has wrapper FAIL=0, internal TFAIL/TBROK=0, timeout=0, ENOSYS=0, and no panic/trap. Keep `TCONF` visible.

```bash
# RV single-candidate proof
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='<candidate>' \
LTP_CASE_TIMEOUT_SECS=20 \
./run-eval.sh rv 2>&1 | tee docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-proc-<candidate>-rv.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-proc-<candidate>-rv.log \
  > docs/ltp-score-improvement-2026-05-22-phase-d/stable200-proc-<candidate>-rv-summary.txt

# LA confirmation after RV is clean
LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img \
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='<candidate>' \
LTP_CASE_TIMEOUT_SECS=20 \
./run-eval.sh la 2>&1 | tee docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-proc-<candidate>-la.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-proc-<candidate>-la.log \
  > docs/ltp-score-improvement-2026-05-22-phase-d/stable200-proc-<candidate>-la-summary.txt
```

For throughput, the same candidates can be run as one RV batch first:

```text
setgroups01 setgroups02 setreuid01 setuid01 sched_getscheduler02 getpgid01 getgroups02 getpriority03 setpriority01 setpriority02 setpriority03 getrusage02 getrusage03 gettimeofday02 gettimeofday03 setrlimit01 setrlimit03 prlimit01 prlimit02 waitpid01
```

## Next 20 PROC/SCHED/WAIT/RLIMIT candidates

| # | Candidate | Evidence status | Required fresh RV/LA command | Blocker risk | Eligible for immediate targeted validation? |
| ---: | --- | --- | --- | --- | --- |
| 1 | `setgroups01` | Held clean Wave-B alternate in `candidate-matrix.md`; not in `wave-a-stable180-new23.cases`; not in `stable180.cases`. | Use template with `LTP_CASES='setgroups01'` on RV, then LA. | Low; credential/group-list path already represented by stable `getgroups01/getgroups03`, but must prove write/set semantics. | Yes — highest priority clean alternate. |
| 2 | `setgroups02` | Held clean Wave-B alternate in `candidate-matrix.md`; not in `stable180.cases`. | `LTP_CASES='setgroups02'` RV then LA. | Low-medium; group-list edge semantics can regress, but prior matrix says cross-arch clean. | Yes — highest priority clean alternate. |
| 3 | `setreuid01` | Held clean Wave-B alternate in `candidate-matrix.md`; not in `stable180.cases`. | `LTP_CASES='setreuid01'` RV then LA. | Low; same credential family as clean `setresuid01` in stable180 new23. | Yes — highest priority clean alternate. |
| 4 | `setuid01` | Held clean Wave-B alternate in `candidate-matrix.md`; not in `stable180.cases`. | `LTP_CASES='setuid01'` RV then LA. | Low; same credential family as clean `setfsuid01`/`setresuid01`. | Yes — highest priority clean alternate. |
| 5 | `sched_getscheduler02` | Phase-c handoff says RV clean but LA history not clean; worker-2 report says no source change indicated without fresh proof. Not in `stable180.cases`. | `LTP_CASES='sched_getscheduler02'` RV then LA. | Medium; stale LA blocker/history risk. | Yes — targeted proof candidate, not promotion-ready until both arches clean. |
| 6 | `getpgid01` | Phase-c blocker / wave-a pool; worker-2 report keeps it as process-group evidence/fix candidate. Not in `stable180.cases`. | `LTP_CASES='getpgid01'` RV then LA. | High; earlier TBROK/TFAIL/ESRCH semantics. | Yes — validation is useful, but promotion requires clean internal markers. |
| 7 | `getgroups02` | Phase-c targeted pool and prior report blocker. Not in `stable180.cases`. | `LTP_CASES='getgroups02'` RV then LA. | Medium-high; group-list size/errno behavior. | Yes — validate before any credentials.rs edit. |
| 8 | `getpriority03` | Phase-c targeted pool; prior report says failed in earlier wave. Not in `stable180.cases`. | `LTP_CASES='getpriority03'` RV then LA. | High; priority errno/permission edge cases. | Yes — targeted validation only, not immediate promotion. |
| 9 | `setpriority01` | Earlier report classified as pass-with-TCONF/near-clean, not clean. Not in `stable180.cases`. | `LTP_CASES='setpriority01'` RV then LA. | Medium; TCONF must remain visible and cannot be treated as PASS. | Yes — near-clean validation candidate. |
| 10 | `setpriority02` | Phase-c targeted pool; prior report says real failure. Not in `stable180.cases`. | `LTP_CASES='setpriority02'` RV then LA. | High; permission/nice lowering semantics. | Yes — repair-target validation. |
| 11 | `setpriority03` | Phase-c targeted pool; prior report says real failure. Not in `stable180.cases`. | `LTP_CASES='setpriority03'` RV then LA. | High; invalid input/permission semantics. | Yes — repair-target validation. |
| 12 | `getrusage02` | Phase-c blocker; prior report says likely not clean / TCONF history. Not in `stable180.cases`. | `LTP_CASES='getrusage02'` RV then LA. | Medium-high; TCONF/internal marker risk. | Yes — targeted validation, not promotion-ready. |
| 13 | `getrusage03` | Phase-c wave-a pool; prior report says TBROK/failure history. Not in `stable180.cases`. | `LTP_CASES='getrusage03'` RV then LA. | High; invalid input/rusage ABI edge. | Yes — repair-target validation. |
| 14 | `gettimeofday02` | Phase-c blocker; prior report says timeout must stay visible. Not in `stable180.cases`. | `LTP_CASES='gettimeofday02'` RV then LA. | High; timeout/fault-address semantics. | Yes — targeted validation, not promotion-ready. |
| 15 | `gettimeofday03` | Phase-c wave-a pool; prior report says failure history. Not in `stable180.cases`. | `LTP_CASES='gettimeofday03'` RV then LA. | High; invalid input/time ABI edge. | Yes — repair-target validation. |
| 16 | `setrlimit01` | Explicit phase-c blocker; prior report shows RV TFAIL and musl timeout after `getrlimit03` fix. Not in `stable180.cases`. | `LTP_CASES='setrlimit01'` RV then LA. | Very high; timeout cannot count as PASS. | Yes — highest rlimit repair target. |
| 17 | `setrlimit03` | Phase-c wave-a pool and worker-2 batch recommendation. Not in `stable180.cases`. | `LTP_CASES='setrlimit03'` RV then LA. | High; rlimit write/permission semantics. | Yes — repair-target validation. |
| 18 | `prlimit01` | Phase-c wave-a pool and worker-2 rlimit batch recommendation. Not in `stable180.cases`. | `LTP_CASES='prlimit01'` RV then LA. | High; `prlimit64` target/resource/old-new pointer semantics. | Yes — repair-target validation. |
| 19 | `prlimit02` | Phase-c wave-a pool and worker-2 rlimit batch recommendation. Not in `stable180.cases`. | `LTP_CASES='prlimit02'` RV then LA. | High; `prlimit64` errno/permission edge. | Yes — repair-target validation. |
| 20 | `waitpid01` | Phase-c blocker; worker-2 report says waitpid blockers must be isolated and not inferred from clean `wait401`. Not in `stable180.cases`. | `LTP_CASES='waitpid01'` RV then LA. | Very high; wait status/child-state behavior. | Yes — first wait repair-target validation. |

## Deferred PROC/SCHED/WAIT/RLIMIT cases

- Already in `stable180.cases`; use as regression guards, not first-choice stable200 additions: `getrlimit03`, `sched_get_priority_max02`, `sched_get_priority_min02`, `sched_rr_get_interval02`, `waitpid04`, `waitpid10`, `wait402`.
- Wait family after `waitpid01`: `waitpid02`, `waitpid05`, `waitpid11`, `waitpid12`, `waitpid13` should stay in a separate wait-only repair batch after `waitpid01` establishes the current signature.
- `waitid01`-`waitid04` remain out of this next-20 list because worker-2 prior static triage found no `waitid` implementation evidence; they are likely implementation work, not stable200 promotion candidates.
- Non-PROC held clean alternate `statx02` belongs to FS/metadata ownership, not this lane.

## Stop condition

This task is complete when this report is committed and the task result records that the worker used current artifacts only. Promotion remains leader-owned and requires fresh parsed RV+LA evidence; no candidate above should be added to `LTP_STABLE_CASES` from this report alone.
