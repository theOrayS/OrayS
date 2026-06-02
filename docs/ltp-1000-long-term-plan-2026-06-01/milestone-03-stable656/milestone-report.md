# Milestone 03 stable656 report - G009 scout, scheduler/statfs/procfs/timer repairs

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

3. `examples/shell/src/uspace/synthetic_fs.rs` reports `/proc/<pid>/stat` state `S` for live processes with futex-waiting threads.
4. `kernel/task/axtask/src/timers.rs` programs precise one-shot wakeups for timer-list deadlines before the next 100Hz periodic tick, and `kernel/runtime/axruntime/src/lib.rs` preserves the periodic tick deadline when such early precise timer interrupts fire.

None of these changes hardcodes an LTP case name, path, process, or expected output.

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

Root cause audit: `sys_readlinkat` already rejects syscall-visible `bufsiz == 0`, and LA syscall argument mapping preserves `arg3`. Upstream musl `readlinkat` rewrites user `bufsize == 0` into a dummy buffer with syscall `bufsize = 1`; the kernel cannot distinguish that from a valid direct one-byte readlink syscall. `readlinkat02` is therefore excluded from promotion as a libc/test boundary, not fixed by a kernel `bufsiz=1` special case.

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


### `futex_wait05` precise timer-list repair

Artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-futex-wait05-periodic-fix-20260601T235234Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-futex-wait05-periodic-fix-20260601T235323Z.summary.txt`
- RV timer/futex regression summary: `target/ltp-1000-milestone-03-stable656/rv-timer-futex-regression-periodic-fix-20260601T235036Z.summary.txt`
- LA timer/futex regression summary: `target/ltp-1000-milestone-03-stable656/la-timer-futex-regression-periodic-fix-20260601T234827Z.summary.txt`
- Combined clean6 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean6-sync-sigsegv-20260602T003243Z.promotion-candidates.txt`

Targeted result: `futex_wait05` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Regression result: `futex_wait01`, `futex_wait02`, `futex_wait03`, `futex_wait04`, `futex_wait05`, `futex_wake01`, `proc01`, `waitpid04`, `nanosleep01`, and `clock_nanosleep02` are parser-clean on RV and LA, 20/20 wrapper PASS on each arch.

Caveat: an initial LA run launched through a TTY stopped before guest output, and a later LA regression attempt hung inside pre-fix `futex_wait05`; both terminated logs are retained only as non-countable repair history. The counted evidence is the post-periodic-deadline-fix targeted and regression set above.


### `mmap05,munmap01` catchable synchronous SIGSEGV repair

Artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.summary.txt`
- RV adjacent regression summary: `target/ltp-1000-milestone-03-stable656/rv-sync-sigsegv-regression-20260602T002800Z.summary.txt`
- LA adjacent regression summary: `target/ltp-1000-milestone-03-stable656/la-sync-sigsegv-regression-20260602T003046Z.summary.txt`
- Combined clean6 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean6-sync-sigsegv-20260602T003243Z.promotion-candidates.txt`

Targeted result: `munmap01` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap. `mmap05` is RV-clean but remains blocked because LA musl and LA glibc both report `TFAIL=1` / SIGSEGV signal not received.

Regression result: `mmap01`, `mmap02`, `mmap03`, `mmap04`, `mmap09`, `mmap12`, `signal03`, `sigaction01`, `rt_sigaction01`, `rt_sigprocmask01`, `sigprocmask01`, and `waitpid04` are parser-clean on RV and LA, 24/24 wrapper PASS on each arch.

### `mmap13` file-backed SIGBUS-on-EOF repair

Artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-final-20260602T012111Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-final-20260602T012141Z.summary.txt`
- RV adjacent regression summary: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-regression-20260602T011329Z.summary.txt`
- LA adjacent regression summary: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-regression-20260602T011433Z.summary.txt`
- Combined clean7 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean7-mmap13-sigbus-final-20260602T012225Z.promotion-candidates.txt`

Targeted result: `mmap13` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Regression result: `mmap01`, `mmap02`, `mmap03`, `mmap04`, `mmap09`, `mmap12`, `signal03`, `sigaction01`, `rt_sigaction01`, `rt_sigprocmask01`, `sigprocmask01`, and `waitpid04` are parser-clean on RV and LA, 24/24 wrapper PASS on each arch.

Caveat: the pre-fix RV log `rv-mmap13-current-20260602T005657Z.log` remains blocker history, and a TTY-launched RV rerun stopped before guest output; neither is counted as promotion evidence. The counted proof is the non-TTY RV/LA targeted pair above.

### Combined candidate pool

Clean combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean7-mmap13-sigbus-final-20260602T012225Z.promotion-candidates.txt`
- Candidates: 7 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mmap13`, `munmap01`, `sched_setaffinity01`)
- Blocked/incomplete: 1 in this clean proof set (`mmap05` LA `TFAIL`)

An earlier combined report that included the old mixed scout is intentionally not used for the current pool because it mixes the pre-fix `fsync02` `TBROK` row with the post-fix `fsync02` proof.

### `openat02` post-statfs-clamp negative scout

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.derived.sha256`

Parser result: 0/2 wrapper PASS, `TBROK=4`, zero timeout, ENOSYS, panic/trap. Both RV musl and RV glibc still hit setup `ENOSPC` after the statfs clamp, so `openat02` remains a blocker and is not counted in the clean pool.

### `clone04` RV singleton rescout

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.derived.sha256`

Parser result: 1 PASS / 1 FAIL; RV glibc is clean, but RV musl has `TBROK=1` and is killed by SIGSEGV. Promotion candidates: 0. The raw log's LTP hint points to a musl `clone.c` wrapper fix, so this row is recorded as a libc-wrapper boundary and is not LA-confirmed or promoted from the failed RV gate.

## Conclusion

Seven new unique cases are currently four-way clean, but stable656 requires 50 new unique cases from the live stable606 baseline. Therefore:

- `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.
- No milestone promotion commit is created for stable656 yet.
- The scheduler permission fix, statfs capacity clamp, procfs futex-sleeping state repair, precise timer-list wakeup repair, catchable synchronous SIGSEGV repair, and file-backed mmap SIGBUS repair are kept as generic behavior work with closed targeted and regression evidence.
- Closed arch-sweep mining adds no further non-stable four-way-clean cases beyond the current seven-case pool.

## Risks / next steps

1. Accumulate 43 more four-way-clean candidates before editing the stable list for stable656.
2. Isolate `kill10` panic/trap before broad process/signal shards.
3. Do not count LA musl `readlinkat02`: root cause is now documented as musl's zero-size wrapper rewrite into a one-byte syscall, and a kernel `bufsiz=1` special case would break valid direct Linux truncation semantics.
4. Treat `nice04` as a libc/kernel errno-boundary investigation: LTP `nice(-10)` expects `EPERM`, while current `setpriority` lowering path returns Linux `EACCES` semantics for `setpriority(2)` and is protected by stable `setpriority02` regression.
5. Treat `mmap05` as a LoongArch mmap/protection-fault signal blocker; do not count the RV-only clean row.
6. Treat `clone04` as a libc-wrapper/clone ABI boundary until RV musl no longer SIGSEGV/TBROK and clone/process/futex regressions are closed.
7. Continue G009 with smaller, non-hanging shards around futex/SysV/resource and avoid running multiple QEMU instances against shared images.
8. Keep all timeout/TCONF/TBROK/TFAIL/ENOSYS/SIGSEGV evidence visible; none counts toward stable promotion.


### `openat02` sparse large-file repair

Artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-sparse-largefile-20260602T014202Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-openat02-sparse-largefile-20260602T014245Z.summary.txt`
- RV adjacent clean regression summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.summary.txt`
- LA adjacent clean regression summary: `target/ltp-1000-milestone-03-stable656/la-openat02-adjacent-stable-clean-regression-20260602T014545Z.summary.txt`
- Combined clean8 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean8-openat02-sparse-largefile-20260602T014245Z.promotion-candidates.txt`

Targeted result: `openat02` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap after generic sparse logical-size/data handling for large regular-file holes.

Regression result: `openat01`, `lseek01`, `lseek02`, `pread02`, `pwrite02`, `pwrite04`, `ftruncate01`, `truncate02`, `read01`, `write01`, and `write03` are parser-clean on RV and LA, 22/22 wrapper PASS on each arch. A broader RV-only observation shard that also included `read02` wrapper-PASSed but emitted existing O_DIRECT `TCONF=4`, so it is retained only as non-countable caveated observation.

### Combined candidate pool after openat02

Clean combined parser report:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean8-openat02-sparse-largefile-20260602T014245Z.promotion-candidates.txt`
- Candidates: 8 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mmap13`, `munmap01`, `openat02`, `sched_setaffinity01`)
- Blocked/incomplete: 1 in this clean proof set (`mmap05` LA `TFAIL`)

Stable list remains unchanged at `606/606/0`; the current pool is 8/50, below the +50 stable656 gate.
