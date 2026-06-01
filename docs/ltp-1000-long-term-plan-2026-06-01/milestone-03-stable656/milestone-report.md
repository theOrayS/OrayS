# Milestone 03 stable656 report - G009 scout and scheduler permission fix

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Baseline before this checkpoint: `606 total / 606 unique / 0 duplicate`
Target milestone: stable656
Status: **not achieved; stable list unchanged**

## Objective

Continue the post-stable606 G009 lane without weakening the promotion gate. This milestone can only advance when a +50 batch of candidates is RV + LA x musl + glibc wrapper PASS and parser-clean through `scripts/ltp_summary.py`.

## Code change performed

`examples/shell/src/uspace/resource_sched.rs::sys_sched_setaffinity` now checks scheduler target permissions with the existing `can_set_sched_target` helper before accepting an otherwise valid CPU0 affinity mask. This fixes the generic Linux behavior where an unprivileged caller should not be able to set affinity for a root/other-owned target.

## Evidence summary

### Initial RV G009 mm/futex scout

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.txt`
- JSON summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.promotion-candidates.txt`
- Derived checksums: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.derived.sha256`

Parser result: 0 PASS / 12 FAIL; `TBROK=2`, `TFAIL=2`, `TCONF=4`, timeout=2; promotion candidates=0.

### RV VFS/process scout

Artifacts:

- Summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.summary.json`

Parser result: 3 PASS / 12 FAIL; `TBROK=13`, `TFAIL=2`, `TCONF=2`, timeout=1, panic/trap=1. `kill10` caused a severe panic/trap and the run stopped before the glibc group, so no promotion evidence is taken from this shard.

### RV G009 mixed safe scout + LA futex confirmation

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mixed-safe-scout-20260602T061659Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.summary.txt`

RV mixed scout: 3 PASS / 8 FAIL; `TBROK=5`, `TFAIL=6`, timeout=1 (`shmat1`). The command was terminated with `RC=143` after `shmat1` hung/ran long, so the shard is scouting-only except for parser-clean rows already completed.

`futex_wait01` is parser-clean on RV and LA for both musl and glibc and is kept as a future promotion candidate.

### RV full-sweep divergence scout + LA readlink confirmation

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-divergence-highyield-scout-20260602T062139Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-confirm-20260602T062321Z.summary.txt`

RV scout: 8 PASS / 8 FAIL; `TFAIL=14`, `TCONF=12`, `TBROK=2`, ENOSYS=2. `readlinkat02` was RV-clean, but LA musl still has `TFAIL`, so it is blocked.

### LA `readlinkat02` rerun after code inspection

Artifacts:

- LA summary: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-rerun-20260601T223953Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-rerun-20260601T223953Z.summary.json`

Parser result: 1 PASS / 1 FAIL; `TFAIL=1`, no timeout, ENOSYS, panic, or trap. The rerun confirms the existing blocker: LA glibc is clean, but LA musl is not promotion-clean.

### RV `fsync02` isolated rerun

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.summary.json`

Parser result: 1 PASS / 1 FAIL; `TBROK=1`, no timeout, ENOSYS, panic, or trap. `fsync02` remains blocked and is not in the candidate pool.

### Closed arch full-sweep mining against live stable606

Artifacts:

- Candidate report: `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`
- RV matrix: `target/ltp-1000-milestone-03-stable656/rv-arch002-full-matrix-20260601T224223Z.json`
- LA matrix: `target/ltp-1000-milestone-03-stable656/la-arch012-full-matrix-20260601T224223Z.json`
- Not-stable filter: `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.not-stable.txt`

The closed RV/LA arch-sweep logs contain 563 four-way-clean rows overall, but filtering against the live `606/606/0` stable list yields zero not-yet-stable four-way-clean cases. This confirms the old sweep is exhausted for immediate stable656 promotion and should be used only as a blocker map for further repair lanes.

### `sched_setaffinity01` targeted fix and regression

Artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-sched-setaffinity01-postfix-20260601T222738Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-sched-setaffinity01-postfix-20260601T222823Z.summary.txt`
- RV regression summary: `target/ltp-1000-milestone-03-stable656/rv-sched-affinity-regression-20260601T222920Z.summary.txt`
- LA regression summary: `target/ltp-1000-milestone-03-stable656/la-sched-affinity-regression-20260601T223023Z.summary.txt`

Targeted result: `sched_setaffinity01` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Regression result: adjacent scheduler/priority stable subset is parser-clean on RV and LA, 20/20 wrapper PASS on each arch.

### Combined candidate pool

Combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-20260601T223023Z.promotion-candidates.txt`
- Candidates: 2 (`futex_wait01`, `sched_setaffinity01`)
- Blocked/incomplete: 13

## Conclusion

Two new unique cases are currently four-way clean, but stable656 requires 50 new unique cases from the live stable606 baseline. Therefore:

- `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.
- No milestone promotion commit is created for stable656 yet.
- The scheduler permission fix is kept as generic behavior work with closed targeted and regression evidence.
- Closed arch-sweep mining adds no further non-stable four-way-clean cases beyond the current two-case pool.

## Risks / next steps

1. Accumulate more four-way-clean candidates before editing the stable list.
2. Isolate `kill10` panic/trap before broad process/signal shards.
3. Diagnose LA musl `readlinkat02` before counting the RV-clean row; the current syscall body already rejects `bufsiz == 0`, so do not special-case the LA musl `bufsiz=1` boundary without root cause.
4. Treat `nice04` as a libc/kernel errno-boundary investigation: LTP `nice(-10)` expects `EPERM`, while current `setpriority` lowering path returns Linux `EACCES` semantics for `setpriority(2)` and is protected by stable `setpriority02` regression.
5. Continue G009 with smaller, non-hanging shards around futex/SysV/resource and avoid running multiple QEMU instances against shared images.
6. Keep all timeout/TCONF/TBROK/TFAIL/ENOSYS evidence visible; none counts toward stable promotion.
