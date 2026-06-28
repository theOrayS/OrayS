# Phase C plan — LTP stable115 to stable150 aggressive promotion

Date: 2026-05-22

## Target and gates

- Baseline: stable115 cases / libc / arch from phase-b.
- Main target: stable150 (+35 clean cases).
- Stage targets: stable130, stable140, stable150.
- Minimum success line: stable130 (+15 clean cases).
- Stretch: stable155+ only if fresh targeted evidence remains clean.

Promotion remains strict: a case may enter `LTP_STABLE_CASES` only with fresh LA/RV x musl/glibc clean evidence: wrapper PASS, internal TFAIL=0, TBROK=0, TCONF=0 for the candidate, timeout=0, ENOSYS=0, panic/trap=0. `read02` remains a known stable caveat and must be reported transparently as pass_with_tconf in stable/full summaries.

## Required sequence

1. Intake: read AGENTS.md, phase-b final reports, current `.omx/ultragoal` state, `scripts/ltp_summary.py`, `examples/shell/src/cmd.rs`, and syscall/uspace touchpoints.
2. Create phase-c context artifact under `.omx/context/` and leader-owned Ultragoal plan.
3. Launch Team workers. Workers provide evidence only and must not checkpoint `.omx/ultragoal`.
4. Wave A: run large targeted RV batch first (80-120 candidate cases), parse with `scripts/ltp_summary.py`; if no panic/trap or broad timeout cluster, run matching LA confirmation.
5. Wave B: fix near-clean blockers using real ABI/syscall/errno/FS/time/signal semantics only.
6. Wave C: promote in batches once 10-15 clean cases are accumulated; after every promotion run LA/RV targeted `stable` gates and save raw log + summary.
7. Stop promotion if stable targeted has timeout, TFAIL/TBROK, ENOSYS, panic/trap, RV memory pressure, or only fake/case-name/silent-skip fixes could make it green.
8. Final guardrails: ai-slop-cleaner, code-review, stable-list audit, final full LA/RV gates, quality-gate JSON, Ultragoal checkpoint.
9. If local branch code is delivered, sync source to `/root/oskernel2026-orays-remote` while preserving remote-only address-mapping differences.

## Phase-b baseline evidence

- Final stable: 115 cases / libc / arch.
- LA final stable: PASS LTP CASE 230, FAIL LTP CASE 0, ltp-musl 115/0, ltp-glibc 115/0.
- RV final stable: PASS LTP CASE 230, FAIL LTP CASE 0, ltp-musl 115/0, ltp-glibc 115/0.
- Internal: TFAIL=0, TBROK=0, TCONF=4; the TCONF=4 is known `read02` pass_with_tconf.
- timeout=0, ENOSYS=0, panic/trap=0 in stable/full LTP summaries.
- Non-LTP full-output markers remain outside stable promotion criteria and must be reported: busybox `which ls fail`, libcbench futex unexpected error code, iperf-glibc `end: fail`.

## Phase-b promoted cases now in stable

`dup202`, `mkdirat01`, `openat01`, `pipe04`, `pipe05`, `pread01`, `pwrite01`, `sysinfo01`, `faccessat01`, `getgroups01`, `setrlimit02`, `sched_get_priority_max01`, `sched_get_priority_min01`, `sched_rr_get_interval01`.

## Fresh blockers / do-not-promote without repair and re-test

`getrlimit03`, `unlinkat01`, `sched_getscheduler02`, `access02`, `access04`, `setrlimit01`, `dup03`, `pipe02`, `lseek02`, `readlinkat01`, `readlinkat02`, `chmod02`, `fchmod02`, `truncate01`, `truncate02`, `ftruncate03`, `waitpid01`, `waitpid04`, `waitpid05`, `waitpid10`, `waitpid11`, `waitpid12`, `waitpid13`, `kill02`, `kill05`, `pause01`, `sigpending02`, `rt_sigpending01`, `sigaltstack01`, `sigaltstack02`, `sigwait01`, `sigtimedwait01`, `setitimer01`, `getitimer01`, `nanosleep01`, `nanosleep02`, `clock_nanosleep02`, `clock_gettime01`, `clock_gettime03`, `clock_gettime04`, `clock_getres01`, `getrusage02`, `gettimeofday02`.

## Wave A candidate pool

Proc/sched/wait/getter/rlimit:

` sched_getscheduler02 sched_getparam02 sched_get_priority_max02 sched_get_priority_min02 sched_rr_get_interval02 getpgid01 getpgid02 getpgrp01 getgroups02 getgroups03 gettid02 waitpid01 waitpid02 waitpid04 waitpid05 waitpid10 waitpid11 waitpid12 waitpid13 wait401 wait402 getrusage02 getrusage03 gettimeofday02 gettimeofday03 getpriority03 setpriority01 setpriority02 setpriority03 times02 getrlimit03 setrlimit01 setrlimit03 prlimit01 prlimit02 `

FD/pipe/dup/lseek/open/access/readlink:

` access02 access04 faccessat02 open04 open05 open06 openat02 close08 close09 dup03 dup05 dup201 pipe02 pipe06 pipe07 lseek02 lseek03 pread02 pwrite02 readlink02 readlink03 readlinkat01 readlinkat02 readlinkat03 `

FS metadata/link/rename/statfs/sysinfo/chmod/truncate:

` link01 link02 link03 linkat01 linkat02 unlink01 unlink02 unlink05 unlinkat01 unlinkat02 rename01 rename02 renameat01 renameat02 mkdir01 mkdir02 rmdir02 stat03 stat04 fstat01 fstat02 lstat02 statfs01 statfs02 fstatfs01 fstatfs02 statvfs01 fstatvfs01 sysinfo02 chmod02 chmod03 fchmod02 fchmodat01 truncate01 truncate02 ftruncate02 ftruncate03 `

Time/signal:

` clock_gettime01 clock_gettime03 clock_gettime04 clock_getres01 clock_nanosleep01 clock_nanosleep02 nanosleep01 nanosleep02 kill02 kill04 kill05 pause01 sigpending02 rt_sigpending01 sigaltstack01 sigaltstack02 sigwait01 sigtimedwait01 alarm01 setitimer01 getitimer01 `

## Team lanes

1. Discovery/Matrix: enumerate candidate availability from images/docs/logs and generate Wave A matrix.
2. Proc/Sched/Wait/Rlimit: legacy LA rlimit wrappers, sched_getscheduler02, waitpid status/child-state.
3. FD/Pipe/Open/Access: dup03/dup201/pipe02/pipe06/lseek/access/open/close.
4. FS/Metadata: unlinkat/link/rename/mkdir/stat/fstat/lstat/statfs/statvfs/sysinfo/chmod/truncate.
5. Time/Signal: nanosleep/clock/signal blockers; timeout remains separate.
6. Hard-blocker/Runtime: timeout/futex/panic/RV memory pressure/non-LTP markers.
7. Verification/Review: no fake PASS, no case-name hardcode, no silent SKIP, timeout not PASS; final code review and quality gate.

## Final required commands

```bash
cargo fmt --all -- --check
make A=examples/shell ARCH=riscv64
./run-eval.sh la 2>&1 | tee output_la.md
./run-eval.sh 2>&1 | tee output_rv.md
python3 -B scripts/ltp_summary.py output_la.md
python3 -B scripts/ltp_summary.py output_rv.md
```
