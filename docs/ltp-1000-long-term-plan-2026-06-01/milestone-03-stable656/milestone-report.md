# Milestone 03 stable656 report - G009 scout, scheduler permission, and statfs capacity repair

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Baseline before this checkpoint: `606 total / 606 unique / 0 duplicate`
Target milestone: stable656
Status: **not achieved; stable list unchanged**

## Objective

Continue the post-stable606 G009 lane without weakening the promotion gate. This milestone can only advance when a +50 batch of candidates is RV + LA x musl + glibc wrapper PASS and parser-clean through `scripts/ltp_summary.py`.

## Code changes performed

1. `examples/shell/src/uspace/resource_sched.rs::sys_sched_setaffinity` checks scheduler target permissions with the existing `can_set_sched_target` helper before accepting an otherwise valid CPU0 affinity mask. This fixes the generic Linux behavior where an unprivileged caller should not be able to set affinity for a root/other-owned target.
2. `examples/shell/src/uspace/metadata.rs::generic_statfs` clamps synthetic reported free blocks to the in-memory regular-file capacity (`MAX_IN_MEMORY_FILE_SIZE / STATFS_BLOCK_SIZE`) instead of exposing the full global allocator free-page count. This keeps `statfs`/`fstatfs`/`statvfs` free-space reporting conservative relative to the current file-size guardrail and prevents tests that size writes from `fstatvfs().f_bavail` from being driven into setup-time `ENOSPC`.

Neither change hardcodes an LTP case name, path, process, or expected output.

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

### RV G009 mixed safe scout + futex isolated confirmation

Artifacts:

- RV mixed summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mixed-safe-scout-20260602T061659Z.summary.txt`
- RV isolated futex summary: `target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.summary.txt`
- LA futex summary: `target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.summary.txt`

RV mixed scout: 3 PASS / 8 FAIL; `TBROK=5`, `TFAIL=6`, timeout=1 (`shmat1`). The command was terminated with `RC=143` after `shmat1` hung/ran long, so the shard is scouting-only except for parser-clean rows already completed.

`futex_wait01` was later isolated on RV to avoid mixing its proof with pre-fix `fsync02` failures. The isolated RV and existing LA confirmations are parser-clean for both musl and glibc, so `futex_wait01` remains a future promotion candidate.

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

### `fsync02` pre-fix isolated blocker and post-fix proof

Pre-fix RV isolated artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.summary.json`

Pre-fix parser result: 1 PASS / 1 FAIL; `TBROK=1`, no timeout, ENOSYS, panic, or trap. The glibc-side setup failed with `ENOSPC`, so this old run remains blocker evidence and is not counted.

Post-fix targeted artifacts after the `generic_statfs` capacity clamp:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.summary.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.derived.sha256`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.derived.sha256`

Post-fix parser result on each arch: 2/2 wrapper PASS, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap. `fsync02` is now a four-way-clean future promotion candidate.

### Adjacent statfs/fstatfs/statvfs regression after the capacity clamp

Regression subset: `statfs02`, `fstatfs02`, `fstatfs02_64`, `statfs02_64`, `statfs03`, `statfs03_64`, `statvfs02`.

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.summary.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.derived.sha256`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.derived.sha256`

Result: 14/14 wrapper PASS on each arch, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

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


### `futex_wait03` procfs sleeping-state repair

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.summary.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.derived.sha256`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.derived.sha256`
- RV adjacent regression summary: `target/ltp-1000-milestone-03-stable656/rv-futex-proc-regression-20260601T232144Z.summary.txt`
- LA adjacent regression summary: `target/ltp-1000-milestone-03-stable656/la-futex-proc-regression-20260601T232232Z.summary.txt`

Targeted result: `futex_wait03` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Regression result: `futex_wait02`, `futex_wait04`, `futex_wake01`, `proc01`, and `waitpid04` are parser-clean on RV and LA, 10/10 wrapper PASS on each arch.

### Combined candidate pool

Clean combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean4-20260601T232334Z.promotion-candidates.txt`
- Candidates: 4 (`fsync02`, `futex_wait01`, `futex_wait03`, `sched_setaffinity01`)
- Blocked/incomplete: 0 in this clean proof set

An earlier combined report that included the old mixed scout is intentionally not used for the current pool because it mixes the pre-fix `fsync02` `TBROK` row with the post-fix `fsync02` proof.

### `openat02` post-statfs-clamp negative scout

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.derived.sha256`

Parser result: 0/2 wrapper PASS, `TBROK=4`, zero timeout, ENOSYS, panic/trap. Both RV musl and RV glibc still hit setup `ENOSPC` after the statfs clamp, so `openat02` remains a blocker and is not counted in the clean pool.

## Conclusion

Four new unique cases are currently four-way clean, but stable656 requires 50 new unique cases from the live stable606 baseline. Therefore:

- `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.
- No milestone promotion commit is created for stable656 yet.
- The scheduler permission fix, statfs capacity clamp, and procfs futex-sleeping state repair are kept as generic behavior work with closed targeted and regression evidence.
- Closed arch-sweep mining adds no further non-stable four-way-clean cases beyond the current four-case pool.

## Risks / next steps

1. Accumulate 46 more four-way-clean candidates before editing the stable list for stable656.
2. Isolate `kill10` panic/trap before broad process/signal shards.
3. Diagnose LA musl `readlinkat02` before counting the RV-clean row; the current syscall body already rejects `bufsiz == 0`, so do not special-case the LA musl `bufsiz=1` boundary without root cause.
4. Treat `nice04` as a libc/kernel errno-boundary investigation: LTP `nice(-10)` expects `EPERM`, while current `setpriority` lowering path returns Linux `EACCES` semantics for `setpriority(2)` and is protected by stable `setpriority02` regression.
5. Continue G009 with smaller, non-hanging shards around futex/SysV/resource and avoid running multiple QEMU instances against shared images.
6. Keep all timeout/TCONF/TBROK/TFAIL/ENOSYS evidence visible; none counts toward stable promotion.
