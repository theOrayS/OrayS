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

3. `examples/shell/src/uspace/task_context.rs`, `examples/shell/src/uspace/signal_abi.rs`, `examples/shell/src/uspace/select_fdset.rs`, and `examples/shell/src/uspace/synthetic_fs.rs` report `/proc/<pid>/stat` state `S` for live processes with futex, `rt_sigsuspend`, or `poll`/`ppoll` waiting threads.
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


### `signal01` signal/poll proc-state repair

Artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-signal01-poll-wait-20260602T024843Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-signal01-poll-wait-20260602T024926Z.summary.txt`
- RV adjacent regression summary: `target/ltp-1000-milestone-03-stable656/rv-signal-poll-regression-20260602T025025Z.summary.txt`
- LA adjacent regression summary: `target/ltp-1000-milestone-03-stable656/la-signal-poll-regression-20260602T025204Z.summary.txt`
- Combined clean9 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean9-signal01-poll-wait-20260602T025432Z.promotion-candidates.txt`

Targeted result: `signal01` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Regression result: `signal02`, `signal03`, `signal04`, `signal05`, `sigaction01`, `rt_sigaction01`, `sigprocmask01`, `rt_sigprocmask01`, `ppoll01`, `pselect01`, `poll02`, and `waitpid04` are parser-clean on RV and LA, 24/24 wrapper PASS on each arch.

Caveat: the intermediate `rv-signal01-proc-sleep-20260602T024336Z` run timed out before the poll-wait marker was added and is retained only as non-countable repair history.

Combined pool after this repair:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean9-signal01-poll-wait-20260602T025432Z.promotion-candidates.txt`
- Candidates: 9 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mmap13`, `munmap01`, `openat02`, `sched_setaffinity01`, `signal01`)
- Blocked/incomplete: 1 in this clean proof set (`mmap05` LA `TFAIL`)
- Stable list remained unchanged at `606/606/0`; at that point in the evidence timeline the pool was 9/50. After the later `mincore03` proof it reached 10/50, and after the G009 clean4 checkpoint below the pool was 14/50 before the later LTP device/NAME_MAX clean5 update, still below the +50 stable656 gate.

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

Fourteen new unique cases are currently four-way clean, but stable656 requires 50 new unique cases from the live stable606 baseline. Therefore:

- `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.
- No milestone promotion commit is created for stable656 yet.
- The scheduler permission fix, statfs capacity clamp, procfs futex-sleeping state repair, precise timer-list wakeup repair, catchable synchronous SIGSEGV repair, and file-backed mmap SIGBUS repair are kept as generic behavior work with closed targeted and regression evidence.
- Closed arch-sweep mining adds no further non-stable four-way-clean cases beyond the current fourteen-case pool.

## Risks / next steps

1. Accumulate 36 more four-way-clean candidates before editing the stable list for stable656.
2. Fix `kill10` cleanup/resource-lifetime blocker before broad process/signal shards; singleton RV evidence now confirms musl timeout, persistent frame leak, and following glibc allocator panic, while the poll/exit cleanup hypothesis was rejected.
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

Stable list remained unchanged at `606/606/0`; at this point in the evidence timeline the pool was 8/50, below the +50 stable656 gate.

### `openat03` O_TMPFILE unsupported-gate blocker

A broader in-memory `O_TMPFILE`/`linkat` emulation was attempted and rejected: RV `openat03` panicked during the deep nested-directory phase, so the patch was removed rather than hidden or counted. Evidence is retained at `rv-openat03-otmpfile-20260602T021349Z.summary.txt` and `rv-openat03-trace-20260602T022058Z.summary.txt`, both with `panic/trap matches: 1`.

The retained source change is only a generic unsupported-feature gate in `fd_table.rs`: `O_TMPFILE|O_RDONLY` returns `EINVAL`, and `O_TMPFILE` against an existing directory returns `EOPNOTSUPP` instead of accidentally returning a directory fd through the `O_DIRECTORY` bit. Targeted RV and LA summaries (`rv-openat03-otmpfile-enotsup-20260602T022658Z.summary.txt`, `la-openat03-otmpfile-enotsup-20260602T022748Z.summary.txt`) now show zero timeout, ENOSYS, panic, or trap, but they still contain `TCONF=4` and wrapper FAIL for musl+glibc.

Decision: this is honest blocker evidence only. `openat03` is not in the candidate pool; at this point in the evidence timeline the pool remained 8/50, stable list remained `606/606/0`, and a future attempt must first solve real generic `O_TMPFILE` semantics plus the deep-directory VFS panic.


## `kill10` isolated blocker checkpoint

A 2026-06-02 RV singleton rerun reproduced `kill10` as a severe blocker outside broad-shard noise: musl wrapper FAIL 137 after 120s, persistent free-frame delta around `-129185` after cleanup, and immediate glibc allocator panic. A temporary generic `poll`/`ppoll` exit-group cleanup hypothesis was tested and removed because it did not change the parser summary or resource delta. `kill10` remains excluded from promotion and from broad batches until cleanup/lifetime behavior is repaired.


## `mincore03` mincore/mlock residency repair

Artifacts:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-mincore03-mincore-mlock-20260602T032124Z.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-mincore03-mincore-mlock-20260602T032208Z.summary.txt`
- RV adjacent regression summary: `target/ltp-1000-milestone-03-stable656/rv-mincore03-adjacent-regression-20260602T032259Z.summary.txt`
- LA adjacent regression summary: `target/ltp-1000-milestone-03-stable656/la-mincore03-adjacent-regression-20260602T032401Z.summary.txt`
- Combined clean10 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean10-mincore03-mincore-mlock-20260602T032401Z.promotion-candidates.txt`

Targeted result: `mincore03` is RV + LA x musl + glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap after generic lazy-VMA-aware `mincore` residency handling and mapped-range `mlock` prefaulting.

Regression result: `mincore01`, `mlock01`, `mlock03`, `mlock04`, `munlock01`, `mlockall01`, `mmap01`, `mmap02`, `mmap03`, and `mmap04` are parser-clean on RV and LA, 20/20 wrapper PASS on each arch.

Combined pool after this repair:

- Candidates: 10 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore03`, `mmap13`, `munmap01`, `openat02`, `sched_setaffinity01`, `signal01`)
- Stable list remains unchanged at `606/606/0`; after this mincore repair the pool was 10/50, and after the later G009 clean4 checkpoint the pool was 14/50 before the LTP device/NAME_MAX clean5 update raised it to 19/50, still below the +50 stable656 gate.
- No `LTP_STABLE_CASES` edit or stable656 milestone promotion commit is made at this checkpoint.

## `epoll_create02` singleton blocker checkpoint

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt`
- RV/LA derived checksums: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.derived.sha256`, `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.derived.sha256`

Result: `epoll_create02` is not a stable656 candidate. RV musl still FAILs with `TFAIL=2` and `ENOSYS=2`; RV glibc and both LA libcs wrapper-PASS but include parser-visible old-ABI `TCONF`. At that checkpoint the candidate pool remained 10/50; after the G009 clean4 checkpoint below the pool was 14/50 before the later LTP device/NAME_MAX clean5 update. `LTP_STABLE_CASES` remains `606 total / 606 unique / 0 duplicate`.


## G009 mm/mlock/mmap clean4 checkpoint

Artifacts:

- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.txt`
- LA clean4 confirmation summary: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.txt`
- Combined clean14 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean14-g009-mm-mprotect-20260602T034707Z.promotion-candidates.txt`

Result: `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` are now four-way parser-clean and join the future promotion pool. The same RV scout keeps the surrounding `mlock*`, `munlock*`, `mprotect01`, `mprotect03`, and `mmap08/16/18/20` failures visible as blocker evidence; those rows are not counted.

Current conclusion after this checkpoint:

- Candidate pool at that checkpoint: 14/50 (`fsync02, futex_wait01, futex_wait03, futex_wait05, mincore02, mincore03, mincore04, mmap13, mprotect02, mprotect04, munmap01, openat02, sched_setaffinity01, signal01`), later superseded by the LTP device/NAME_MAX clean5 checkpoint below.
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 36 cases short.

## `statfs01,fstatfs01,fstatfs01_64,statvfs01` RV device-acquire blocker checkpoint

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.json`
- RV promotion-candidate report: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.promotion-candidates.txt`
- RV derived checksums: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.derived.sha256`

Result: this RV-only scout produced 0 wrapper PASS / 8 wrapper FAIL across musl+glibc. The parser reports `TBROK=8`, with zero timeout, ENOSYS, panic, or trap. The raw log shows each row failing in LTP setup with `tst_device.c:147 TINFO: No free devices found` followed by `tst_device.c:354 TBROK: Failed to acquire device`.

Decision: these rows are blocker evidence only. They are not statfs ABI proof and not stable656 candidates because RV is not parser-clean and LA was not run. Candidate pool remained 14/50 at that checkpoint; `LTP_STABLE_CASES` remained `606 total / 606 unique / 0 duplicate`.

## VFS-C `mknod07,mknodat02,rename03,rename04,rename05` RV device-acquire blocker checkpoint

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.json`
- RV promotion-candidate report: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.promotion-candidates.txt`
- RV derived checksums: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.derived.sha256`

Result: this RV-only VFS-C scout produced 0 wrapper PASS / 10 wrapper FAIL across musl+glibc. The parser reports `TBROK=14`, with zero timeout, ENOSYS, panic, or trap. The raw log shows all rows failing in LTP setup with `No free devices found` / `Failed to acquire device` from `tst_device.c`.

Decision: these rows are blocker evidence only. They are not mknod/rename ABI proof and not stable656 candidates because RV is parser-unclean and LA was not run. Candidate pool remained 14/50 at that checkpoint; `LTP_STABLE_CASES` remained `606 total / 606 unique / 0 duplicate`.

## LTP device/NAME_MAX clean5 checkpoint

A generic device-acquisition repair converted five previously blocked VFS/statfs rows into future promotion candidates without changing `LTP_STABLE_CASES`.

Code changes retained in this checkpoint:

1. `examples/shell/src/cmd.rs` now sets a generic `LTP_DEV=/dev/vda` for LTP runs, using the evaluator's existing synthetic block-backed test device instead of relying on an unimplemented Linux loop-device stack.
2. `examples/shell/src/uspace/fd_table.rs` lists synthetic block devices under `/dev`, routes synthetic block-device stat lookups through shared metadata, and reports/enforces the real 63-byte filename component capacity used by `axfs_vfs::VfsDirEntry`.
3. `examples/shell/src/uspace/metadata.rs` assigns stable synthetic `st_rdev` values for `/dev/vda`, `/dev/sda`, and `/dev/xvda`.
4. `examples/shell/src/uspace/linux_abi.rs` reports `statfs` name length as 63 instead of 255, matching the backing dirent capacity and preventing the pre-fix `statvfs01` panic.

Evidence:

- Enumeration-only RV retest: `target/ltp-1000-milestone-03-stable656/rv-device-enumeration-retest-20260602T041227Z.summary.txt` — still 0 PASS / 18 FAIL, `TBROK=22`.
- Pre-NAME_MAX RV retest: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-vda-device-retest-20260602T041431Z.summary.txt` — 3 PASS before `statvfs01` panic/trap; not countable.
- Final RV retest: `target/ltp-1000-milestone-03-stable656/rv-device-cases-ltpdev-namemax-retest-20260602T041654Z.summary.txt` — 10 PASS / 8 FAIL, no timeout/ENOSYS/panic/trap; clean cases are `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05`.
- LA clean5 confirmation: `target/ltp-1000-milestone-03-stable656/la-device-clean5-ltpdev-namemax-retest-20260602T041803Z.summary.txt` — 10 PASS / 0 FAIL, zero internal markers, timeout, ENOSYS, panic/trap.
- Regression subset: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-namemax-regression-subset-20260602T041926Z.summary.txt` and `target/ltp-1000-milestone-03-stable656/la-ltpdev-namemax-regression-subset-20260602T042012Z.summary.txt` — `chdir01`, `pathconf01`, and `fpathconf01` are parser-clean on both arches and libcs.
- Combined clean19 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean19-ltpdev-namemax-20260602T041803Z.promotion-candidates.txt`.

Current conclusion:

- Candidate pool: 19/50 (`fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename05`, `sched_setaffinity01`, `signal01`, `statfs01`, `statvfs01`).
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 31 cases short.
- Remaining blockers from this lane: `mknod07`/`mknodat02` need generic ext2/device setup (`mkfs.ext2` absent); `rename03`/`rename04` need generic rename semantics repair.


## FD/fcntl clean2 checkpoint

A documentation/evidence-only FD/fcntl scout converted two not-yet-stable rows into future promotion candidates without changing source or `LTP_STABLE_CASES`.

Evidence:

- RV fcntl scout: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.txt` — 4 PASS / 20 FAIL, with `TCONF=14`, `TFAIL=6`, `TBROK=6`, timeout=2, and no ENOSYS/panic/trap. Clean RV rows are `fcntl15` and `fcntl11_64`.
- LA clean2 confirmation: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.txt` — 4 PASS / 0 FAIL, zero internal markers, timeout, ENOSYS, panic/trap.
- Combined clean21 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`.

Current conclusion:

- Candidate pool: 21/50 (`fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename05`, `sched_setaffinity01`, `signal01`, `statfs01`, `statvfs01`).
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 29 cases short.
- Remaining blockers from this lane: `fcntl17` timeout; `fcntl24`/`fcntl25`/`fcntl26`/`fcntl37` `TCONF`; `fcntl27`/`fcntl31` `TFAIL`; `fcntl34`/`fcntl38`/`fcntl39` `TBROK`.

## Rename metadata/inode clean1 checkpoint

A generic rename metadata repair converted `rename01` into a future promotion candidate without changing `LTP_STABLE_CASES`.

Code changes retained in this checkpoint:

1. `UserProcess` now tracks per-path inode overrides as process metadata.
2. Successful `renameat2(..., flags=0)` now migrates recorded path metadata from the old path to the new path, including inode, mode, owner, special-device, symlink, xattr, and sparse-file state.
3. `unlinkat()` now clears recorded inode metadata for deleted paths.

Evidence:

- RV broad VFS/path scout: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-link-statx-scout-20260602T044314Z.summary.txt` — 4 PASS / 42 FAIL, but no parser-clean promotion candidates because wrapper-PASS rows still contain `TCONF` and the rest retain visible blockers.
- RV rename01+rename05 regression: `target/ltp-1000-milestone-03-stable656/rv-rename-inode-retarget-20260602T044708Z.summary.txt` — 4 PASS / 0 FAIL, zero internal markers, timeout, ENOSYS, panic/trap.
- LA rename01+rename05 regression: `target/ltp-1000-milestone-03-stable656/la-rename-inode-retarget-20260602T044751Z.summary.txt` — 4 PASS / 0 FAIL, zero internal markers, timeout, ENOSYS, panic/trap.
- RV rename01 singleton: `target/ltp-1000-milestone-03-stable656/rv-rename01-inode-confirm-20260602T044855Z.summary.txt` — 2 PASS / 0 FAIL, parser-clean.
- LA rename01 singleton: `target/ltp-1000-milestone-03-stable656/la-rename01-inode-confirm-20260602T044855Z.summary.txt` — 2 PASS / 0 FAIL, parser-clean.
- Combined clean22 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean22-rename01-inode-20260602T044855Z.promotion-candidates.txt`.

Current conclusion:

- Candidate pool: 22/50 (`fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename01`, `rename05`, `sched_setaffinity01`, `signal01`, `statfs01`, `statvfs01`).
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 28 cases short.
- Remaining blockers from this lane: hard-link/linkat support is still absent or setup-blocked; `stat03`/`stat03_64` and `readlink03` retain real errno/loop/path semantic blockers; `statx01`/`getdents02` wrapper-PASS rows retain parser-visible `TCONF` and are not counted.

### `rename03,rename04` directory replacement repair

Artifacts:

- RV rename targeted summary: `target/ltp-1000-milestone-03-stable656/rv-rename-dir-overwrite-20260602T050256Z.summary.txt`
- LA rename targeted summary: `target/ltp-1000-milestone-03-stable656/la-rename-dir-overwrite-20260602T050346Z.summary.txt`
- RV clean-only statfs/rename05 retarget summary: `target/ltp-1000-milestone-03-stable656/rv-statfs-rename05-clean-retarget-20260602T050521Z.summary.txt`
- Combined clean24 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean24-rename03-04-20260602T050630Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rename03-04-clean24-20260602T050630Z.derived.sha256`

Targeted result: `rename03` and `rename04` are RV + LA x musl + glibc parser-clean after generic source/destination type handling in `axfs::root::rename`. Adjacent rename rows `rename01` and `rename05` stayed clean on both arches.

Regression result: the clean-only combined report now contains 24 four-way-clean future candidates. This does not cross the 50-case stable656 gate, so `LTP_STABLE_CASES` remains unchanged at `606 total / 606 unique / 0 duplicate`.

## Stat/readlink path traversal clean2 checkpoint

A generic path/stat repair converted `stat03` and `stat03_64` into future promotion candidates without editing `LTP_STABLE_CASES`.

Code changes retained in this checkpoint:

1. `examples/shell/src/uspace/metadata.rs` now resolves symlinks component-by-component for pathname traversal, with a 40-hop `ELOOP` guard. `readlink`/`lstat`-style paths resolve parent symlinks while preserving the final symlink when required.
2. `examples/shell/src/uspace/fd_table.rs::stat_path` now checks parent directory search permission for non-root callers and returns `EACCES` when an ancestor lacks execute/search permission.
3. Parent directory traversal now returns `ENOTDIR` when a non-directory appears in a path prefix, instead of reporting a misleading permission failure.
4. `newfstatat` now returns `ENOENT` for an empty pathname unless `AT_EMPTY_PATH` is explicitly supplied.
5. `O_NOFOLLOW` open handling resolves parent symlinks before deciding whether the final component is a symlink.

Evidence:

- First RV attempt: `target/ltp-1000-milestone-03-stable656/rv-readlink-stat-path-20260602T051956Z.summary.txt` — `readlink03` passed, but `stat03` hit a parser-visible panic/trap due recursive parent-search checking. This log is blocker/repair history only and is not counted.
- Fixed RV targeted proof: `target/ltp-1000-milestone-03-stable656/rv-readlink-stat-path-nonrecursive-20260602T052206Z.summary.txt` — `readlink03`, `stat03`, and `stat03_64` are parser-clean for musl+glibc; zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA targeted proof: `target/ltp-1000-milestone-03-stable656/la-readlink-stat-path-nonrecursive-20260602T052251Z.summary.txt` — `stat03` and `stat03_64` are parser-clean for musl+glibc; `readlink03` remains blocked because LA musl has parser-visible `TFAIL=1` for the zero-size-buffer case.
- Adjacent stable regression: `target/ltp-1000-milestone-03-stable656/rv-stat-readlink-stable-regression-20260602T052501Z.summary.txt` and `target/ltp-1000-milestone-03-stable656/la-stat-readlink-stable-regression-20260602T052706Z.summary.txt` — `stat01`, `stat02`, `stat01_64`, `stat02_64`, `lstat01`, `lstat01_64`, `fstatat01`, `readlink01`, `readlinkat01`, `openat01`, and `rename14` are parser-clean on both arches and libcs.
- Combined clean26 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean26-stat03-path-20260602T052251Z.promotion-candidates.txt`.

Current conclusion:

- Newly evidenced four-way-clean cases: `stat03`, `stat03_64`.
- Candidate pool: 26/50 (`fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename01`, `rename03`, `rename04`, `rename05`, `sched_setaffinity01`, `signal01`, `stat03`, `stat03_64`, `statfs01`, `statvfs01`).
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 24 cases short.
- Remaining blocker from this step: `readlink03` remains outside the pool until LA musl zero-size-buffer behavior can be fixed or classified with parser-clean evidence; it is not blacklisted or counted as PASS.

## mmap20/munlock02 clean2 checkpoint

A generic mmap/munlock errno repair converted two previously blocked mm rows into future promotion candidates without editing `LTP_STABLE_CASES`.

Code changes retained in this checkpoint:

1. `examples/shell/src/uspace/memory_map.rs::sys_mmap` now rejects unsupported `MAP_SHARED_VALIDATE` flag bits with `EOPNOTSUPP`, instead of treating unknown validation-only bits as ordinary accepted flags.
2. Non-anonymous `mmap` now validates the fd before reserving/mapping user address space and returns generic descriptor errors (`EBADF`, `EACCES`, `EISDIR`, or `ESPIPE`) for invalid, unreadable, directory, pipe, or socket-like descriptors.
3. `munlock` is now dispatched and validates the whole page-rounded mapped range, returning `ENOMEM` for unmapped, overflow, or out-of-user-range inputs; `mlock` reuses the same range validator before prefaulting.

Evidence:

- RV targeted proof: `target/ltp-1000-milestone-03-stable656/rv-mmap20-munlock02-targeted-20260602T054424Z.summary.txt` — 4 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA targeted proof: `target/ltp-1000-milestone-03-stable656/la-mmap20-munlock02-targeted-20260602T054508Z.summary.txt` — 4 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Incremental clean2 report: `target/ltp-1000-milestone-03-stable656/mmap20-munlock02-clean2-20260602T054508Z.promotion-candidates.txt` — 2 candidates, 0 blocked rows.
- Adjacent regression: `rv-mmap-munlock-regression-20260602T054554Z.summary.txt` and `la-mmap-munlock-regression-20260602T054705Z.summary.txt` — 28 PASS / 0 FAIL on each arch, zero parser/fatal markers.

Current conclusion:

- Newly evidenced four-way-clean cases: `mmap20`, `munlock02`.
- Candidate pool: 28/50 (`fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mprotect02`, `mprotect04`, `munmap01`, `openat02`, `rename01`, `rename03`, `rename04`, `rename05`, `sched_setaffinity01`, `signal01`, `stat03`, `stat03_64`, `statfs01`, `statvfs01`, `mmap20`, `munlock02`).
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 22 cases short.
- Remaining blockers from this step: `mmap08` still fails its EBADF expectation because diagnostic-only evidence shows fd 3 is still a readable temp-file descriptor at `mmap` time; `mlock02` still needs real `RLIMIT_MEMLOCK`/capability behavior.


## epoll_create1 clean2 checkpoint

A generic eventpoll creation repair converted `epoll_create1_01` and `epoll_create1_02` into future promotion candidates without editing `LTP_STABLE_CASES`.

Code changes retained in this checkpoint:

1. `examples/shell/src/uspace/syscall_dispatch.rs` now dispatches `__NR_epoll_create1` in the shell userspace syscall bridge.
2. `examples/shell/src/uspace/fd_table.rs` now creates a synthetic `anon_inode:[eventpoll]` descriptor and validates `EPOLL_CLOEXEC`/unknown flags with `EINVAL`, preserving `FD_CLOEXEC` through the existing fd-table flag path.
3. `api/arceos_posix_api/src/imp/io_mpx/epoll.rs` now rejects `epoll_create(size <= 0)` with `EINVAL` for the axlibc/glibc-visible legacy path.

Evidence:

- RV targeted proof: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt` — 4 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA targeted proof: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt` — 4 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Incremental clean2 report: `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.txt` — 2 candidates, 0 blocked/incomplete rows.
- RV adjacent FD/flags regression: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.txt` — 12 PASS / 0 FAIL, parser-clean.
- LA adjacent FD/flags regression: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.txt` — 12 PASS / 0 FAIL, parser-clean.
- Combined clean30 audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean30-epoll-create1-20260602T061430Z.md`.

Current conclusion:

- Newly evidenced four-way-clean cases: `epoll_create1_01`, `epoll_create1_02`.
- Candidate pool: 30/50 (`epoll_create1_01`, `epoll_create1_02`, `fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mmap20`, `mprotect02`, `mprotect04`, `munlock02`, `munmap01`, `openat02`, `rename01`, `rename03`, `rename04`, `rename05`, `sched_setaffinity01`, `signal01`, `stat03`, `stat03_64`, `statfs01`, `statvfs01`).
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 20 cases short.
- Remaining blocker from this step: `epoll_create02` remains outside the pool because musl maps old `epoll_create(size)` to valid `epoll_create1(0)`, while the promotion gate requires parser-clean musl+glibc proof on both arches.


## clock_adjtime/sigaltstack/shmt04 clean4 checkpoint

Generic time/signal syscall-state repairs and a SysV shm evidence closure converted four rows into future promotion candidates without editing `LTP_STABLE_CASES`.

Code changes retained in this checkpoint:

1. `examples/shell/src/uspace/syscall_dispatch.rs` now dispatches `__NR_clock_adjtime` and `__NR_sigaltstack` in the shell userspace syscall bridge.
2. `examples/shell/src/uspace/time_abi.rs::sys_clock_adjtime` accepts `CLOCK_REALTIME` and delegates to existing `adjtimex` semantics, preserving `EINVAL` for unsupported clocks and the existing `adjtimex` permission/errno rules.
3. `examples/shell/src/uspace/task_context.rs` records per-thread alternate signal-stack state.
4. `examples/shell/src/uspace/signal_abi.rs::sys_sigaltstack` copies out old state, validates flags/size, stores new state, and reports `SS_ONSTACK` while inside an existing signal frame. This is syscall-state support, not full alternate-stack signal delivery.

Evidence:

- RV targeted proof: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt` — 8 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA targeted proof: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt` — 8 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Incremental clean4 report: `target/ltp-1000-milestone-03-stable656/combined-clock-sigaltstack-shmt04-20260602T143805+0800.promotion-candidates.txt` — 4 candidates, 0 blocked/incomplete rows.
- RV adjacent time/signal regression: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.summary.txt` — 14 PASS / 0 FAIL, parser-clean.
- LA adjacent time/signal regression: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.summary.txt` — 14 PASS / 0 FAIL, parser-clean.
- Combined clean34 audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean34-clock-sigaltstack-shmt04-20260602T143805Z.md`.

Current conclusion:

- Newly evidenced four-way-clean cases: `adjtimex01`, `adjtimex03`, `shmt04`, `sigaltstack02`.
- Candidate pool: 34/50 (`adjtimex01`, `adjtimex03`, `epoll_create1_01`, `epoll_create1_02`, `fcntl11_64`, `fcntl15`, `fstatfs01`, `fstatfs01_64`, `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mincore02`, `mincore03`, `mincore04`, `mmap13`, `mmap20`, `mprotect02`, `mprotect04`, `munlock02`, `munmap01`, `openat02`, `rename01`, `rename03`, `rename04`, `rename05`, `sched_setaffinity01`, `shmt04`, `signal01`, `sigaltstack02`, `stat03`, `stat03_64`, `statfs01`, `statvfs01`).
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 16 cases short.
- Maintenance caveat: `sigaltstack02` proves syscall-visible alternate-stack state and errno handling, not handler delivery on the alternate stack.


## shmat04 IPC_STAT ABI clean1 checkpoint

A generic SysV shm user-copy ABI repair converted `shmat04` into a future stable656 candidate without editing `LTP_STABLE_CASES`.

Code change retained in this checkpoint:

1. `examples/shell/src/uspace/sysv_shm.rs::sys_shmctl(IPC_STAT)` now writes a Linux 64-bit `shmid_ds`-compatible `#[repr(C)]` structure (112 bytes) through `write_user_value` instead of clearing a guessed 128-byte range. It records key/mode/requested-size metadata and avoids glibc stack-canary overwrite while keeping unsupported/full lifetime semantics explicit.

Evidence:

- RV targeted proof: `target/ltp-1000-milestone-03-stable656/rv-shmat04-shmt04-ipcstat-abi-20260602T150702+0800.summary.txt` — 4 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA targeted proof: `target/ltp-1000-milestone-03-stable656/la-shmat04-shmt04-ipcstat-abi-20260602T150805+0800.summary.txt` — 4 PASS / 0 FAIL, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Combined clean35 report: `target/ltp-1000-milestone-03-stable656/combined-shmat04-shmt04-ipcstat-abi-20260602T150918+0800.promotion-candidates.txt`.
- Audit table: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean35-shmat04-ipcstat-abi-20260602T150918Z.md`.

Current conclusion:

- Newly evidenced four-way-clean case: `shmat04`.
- Candidate pool: 35/50.
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 15 cases short.
- Maintenance caveat: this checkpoint fixes the visible `IPC_STAT` user ABI size/layout boundary; it does not add full SysV shm attach refcount/lifetime reclamation.

## time/timer clean3 checkpoint

A time/timer scout found three already-generic rows that are clean enough for the future stable656 pool without any source edit.

Code changes retained in this checkpoint: none.

Evidence:

- RV scout: `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.summary.txt` — 8 PASS / 42 FAIL across musl+glibc; `getitimer02`, `setitimer02`, and `times03` are the only parser-clean RV candidates.
- LA confirmation: `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.summary.txt` — 6 PASS / 0 FAIL for the three candidates, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Combined RV+LA report: `target/ltp-1000-milestone-03-stable656/combined-time-timer-clean3-20260602T152824+0800.promotion-candidates.txt` — 3 candidates and 22 blocked/incomplete rows.
- Combined clean38 audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean38-time-timer-clean3-20260602T152824Z.md`.

Current conclusion:

- Newly evidenced four-way-clean cases: `getitimer02`, `setitimer02`, `times03`.
- Candidate pool: 38/50.
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 12 cases short.
- Remaining blockers from this scout keep visible parser caveats and must not be counted until fixed and revalidated.

## lstat clean2 checkpoint

A VFS/path scout found two already-generic `lstat` rows that are clean enough for the future stable656 pool without any source edit.

Code changes retained in this checkpoint: none.

Evidence:

- RV scout: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-simple-scout-20260602T153210+0800.summary.txt` — 5 PASS / 39 FAIL across musl+glibc; `lstat02` and `lstat02_64` are the only parser-clean RV candidates.
- LA confirmation: `target/ltp-1000-milestone-03-stable656/la-lstat-clean2-20260602T153351+0800.summary.txt` — 4 PASS / 0 FAIL for the two candidates, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Combined RV+LA report: `target/ltp-1000-milestone-03-stable656/combined-lstat-clean2-20260602T153433+0800.promotion-candidates.txt` — 2 candidates and 20 blocked/incomplete rows.
- Combined clean40 audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean40-lstat-clean2-20260602T153433Z.md`.

Current conclusion:

- Newly evidenced four-way-clean cases: `lstat02`, `lstat02_64`.
- Candidate pool: 40/50.
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 10 cases short.
- Remaining VFS/path rows from this scout keep visible parser caveats and must not be counted until fixed and revalidated.

## open clean2 checkpoint

A FD/VFS/IO scout found two already-generic `open` rows that are clean enough for the future stable656 pool without any source edit.

Code changes retained in this checkpoint: none.

Evidence:

- RV scout: `target/ltp-1000-milestone-03-stable656/rv-fd-vfs-io-scout-20260602T153655+0800.summary.txt` — 4 PASS / 28 FAIL across musl+glibc; `open07` and `open12` are the only parser-clean RV candidates.
- LA confirmation: `target/ltp-1000-milestone-03-stable656/la-open-clean2-20260602T153756+0800.summary.txt` — 4 PASS / 0 FAIL for the two candidates, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Combined RV+LA report: `target/ltp-1000-milestone-03-stable656/combined-open-clean2-20260602T153844+0800.promotion-candidates.txt` — 2 candidates and 14 blocked/incomplete rows.
- Combined clean42 audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean42-open-clean2-20260602T153844Z.md`.

Current conclusion:

- Newly evidenced four-way-clean cases: `open07`, `open12`.
- Candidate pool: 42/50.
- Stable list: unchanged at `606 total / 606 unique / 0 duplicate`.
- No stable656 milestone promotion commit is made because the +50 gate is still 8 cases short.
- Remaining FD/VFS/IO rows from this scout keep visible parser caveats or wrapper-fail events and must not be counted until fixed and revalidated.
