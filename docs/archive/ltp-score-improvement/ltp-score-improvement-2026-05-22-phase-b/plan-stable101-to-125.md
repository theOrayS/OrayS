# Phase-b plan: LTP stable101 -> stable120/125

Date: 2026-05-22
Docs root: `docs/ltp-score-improvement-2026-05-22-phase-b/`

## Target and gates

- Baseline: stable101 cases / libc / arch, from phase-a final full LA/RV gates.
- Main target: stable120, requiring +19 newly promoted cases.
- Minimum success line: stable115, requiring +14 clean cases.
- Stretch: stable125+ if fresh targeted evidence remains clean.
- Promotion gate is strict: only LA/RV x musl/glibc clean cases may enter `LTP_STABLE_CASES`.
- Known transparent caveat remains `read02` pass_with_tconf; do not hide it or claim it as a clean pass.
- Timeout, TFAIL, TBROK, TCONF, ENOSYS, and panic/trap are counted separately and never as PASS.
- Do not use case-name hardcoding, fake PASS, silent SKIP, or timeout-as-PASS.

## Required execution order

1. Preserve historical evidence in phase-a and 2026-05-21 phase directories; create only phase-b artifacts for this run.
2. Create `.omx/context/ltp-score-improvement-stable101-to-125-*.md` with the stable101 baseline, blocker list, candidate pools, and likely touchpoints.
3. Create a fresh leader-owned Ultragoal plan. Team workers must not checkpoint `.omx/ultragoal`.
4. Launch Team with 7 executor workers if available, 6 fallback only if runtime resources block startup.
5. Discovery/matrix lane enumerates candidate availability and writes a phase-b candidate matrix, then selects Wave A 35-60 cases.
6. Wave A targeted: run RV first with `OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:<casefile> LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh`; if no panic/trap/large timeout cluster, run LA with the same file.
7. Use `scripts/ltp_summary.py` and `--promotion-candidates` to classify clean, pass_with_tconf, timeout, TFAIL, TBROK, ENOSYS, panic/trap.
8. Promote clean subsets in batches of 8-12 when available; after every promotion run LA/RV targeted stable batch and save raw logs + summaries.
9. Dispatch near-clean blockers for real ABI/errno/time/signal/FS fixes only; rerun targeted proof before promotion.
10. Before final full gate: run bounded guardrail/ai-slop review and code review against changed files and stable evidence.
11. Final required gate: `cargo fmt --all -- --check`, `make A=examples/shell ARCH=riscv64` if shell/uspace changed, `./run-eval.sh la`, `./run-eval.sh`, and `scripts/ltp_summary.py` on both outputs.
12. Sync local code to `/root/oskernel2026-orays-remote` after local validation, preserving remote-only address-mapping differences and generated/log artifacts.
13. Produce final report and quality-gate JSON, then leader checkpoints Ultragoal complete only if gates are clean.

## Candidate pool

Wave A should sample 35-60 from these groups, prioritizing cases likely to be real clean or near-clean after phase-a:

- proc/sched/wait/getter: `sched_getscheduler02`, `sched_getparam02`, `sched_get_priority_max01`, `sched_get_priority_min01`, `sched_rr_get_interval01`, `getpgid01`, `getpgid02`, `getpgrp01`, `getgroups01`, `getgroups03`, `gettid02`, `waitpid01`, `waitpid02`, `waitpid04`, `waitpid05`, `waitpid10`, `waitpid11`, `waitpid12`, `waitpid13`, `getrusage02`, `getrusage03`, `gettimeofday02`, `gettimeofday03`, `getpriority03`, `setpriority01`, `setpriority02`, `setpriority03`, `times01`, `times02`, `getrlimit03`, `setrlimit01`, `setrlimit02`.
- time/signal: `clock_gettime01`, `clock_gettime03`, `clock_gettime04`, `clock_getres01`, `clock_nanosleep01`, `clock_nanosleep02`, `nanosleep01`, `nanosleep02`, `kill02`, `kill05`, `pause01`, `sigpending02`, `rt_sigpending01`, `sigaltstack01`, `sigaltstack02`, `sigwait01`, `sigtimedwait01`, `alarm01`, `alarm02`, `setitimer01`, `getitimer01`.
- fd/pipe/open/lseek/access: `access02`, `access04`, `faccessat01`, `open01`, `open02`, `openat01`, `close01`, `close02`, `dup01`, `dup02`, `dup03`, `dup05`, `dup201`, `dup202`, `pipe01`, `pipe02`, `pipe04`, `pipe05`, `lseek01`, `lseek02`, `pread01`, `pwrite01`, `readlink01`, `readlinkat01`, `readlinkat02`.
- fs metadata/link/rename/statfs/sysinfo: `link01`, `link02`, `linkat01`, `unlink01`, `unlink05`, `unlinkat01`, `rename01`, `renameat01`, `mkdir01`, `mkdir02`, `mkdirat01`, `rmdir01`, `stat01`, `stat02`, `fstat01`, `lstat01`, `statfs01`, `statfs02`, `fstatfs01`, `fstatfs02`, `statvfs01`, `fstatvfs01`, `sysinfo01`, `chmod01`, `chmod02`, `fchmod02`, `truncate01`, `truncate02`, `ftruncate01`, `ftruncate03`.

## Known phase-a blockers that cannot be directly promoted

- `clock_nanosleep02`, `nanosleep01`: demoted after stable103 targeted RV TFAIL.
- `sched_getscheduler02`: LA musl TFAIL.
- `clock_gettime01`, `clock_nanosleep01`, `gettimeofday02`: timeout risk.
- `clock_getres01`, `getrusage02`: TCONF risk.
- `setpriority01`, `setpriority02`, `waitpid01`: real errno/status/child-state gaps.
- FS/sysinfo/statfs/access/link/rename/unlink/mkdir/lseek/pipe/dup directions require real ABI/errno fixes and fresh targeted evidence.

## Team lane ownership

- Leader: Ultragoal state, promotion decisions, final reports, remote sync.
- Discovery/Matrix: candidate matrix, Wave A selection, promotion-candidate reports.
- Proc/Sched/Wait: scheduler/getpriority/setpriority/wait/getter real semantics.
- Time/Signal: nanosleep/clock/signal blocker investigation and fixes.
- FD/Pipe/Open: dup/pipe/lseek/access/open semantics.
- FS/Metadata: access/link/rename/unlink/mkdir/statfs/statvfs/fstatfs/sysinfo semantics.
- Hard-blocker/Runtime: timeout, futex aborts, panic/trap, RV memory pressure, non-LTP markers.
- Verification/Review: fake-PASS/code-review/quality-gate evidence and stable-list audit.

## Stop/degrade criteria

Stop promotion or degrade target rather than polluting stable if stable targeted shows timeout, TFAIL/TBROK, panic/trap, RV memory pressure, ENOSYS not accounted for, or only fake/case-name/silent-SKIP approaches can pass.
