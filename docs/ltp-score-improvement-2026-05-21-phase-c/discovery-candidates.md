# Worker-1 discovery candidates for LTP stable promotion (2026-05-23)

## Scope and source evidence

Task source: `task-1.json` was updated to discovery-only: enumerate cases from `sdcard-rv.img` / `sdcard-la.img`, compare against `examples/shell/src/cmd.rs::LTP_STABLE_CASES`, and produce 20-40 plausible non-stable candidates.

Commands/evidence used:

```sh
python3 scripts/ltp_summary.py output_rv.md
python3 scripts/ltp_summary.py output_la.md
python3 scripts/ltp_summary.py --promotion-candidates output_rv.md output_la.md
debugfs -R 'ls -p /musl/ltp/testcases/bin' /root/oskernel2026-orays/sdcard-rv.img
debugfs -R 'ls -p /musl/ltp/testcases/bin' /root/oskernel2026-orays/sdcard-la.img
grep -R 'sys_getpgid\|sigsuspend\|sigprocmask\|nanosleep\|clock_getres\|statfs\|renameat\|linkat\|unlinkat\|mkdirat\|pipe2\|dup3\|lseek' -n examples/shell/src/uspace
```

Inventory snapshot saved at `docs/ltp-score-improvement-2026-05-23/raw/worker1-discovery-inventory.json`:

- Current stable list: **63** cases per libc/arch.
- RV image executable LTP cases: **2370**.
- LA image executable LTP cases: **2370**.
- Common RV/LA executable LTP cases: **2370**.
- Common executable cases outside stable: **2307**.

Current top-level logs are already stable-63 clean:

- `output_rv.md`: `PASS LTP CASE: 126`, `FAIL LTP CASE: 0`, suite summaries `ltp-musl: 63 passed, 0 failed`, `ltp-glibc: 63 passed, 0 failed`.
- `output_la.md`: `PASS LTP CASE: 126`, `FAIL LTP CASE: 0`, suite summaries `ltp-musl: 63 passed, 0 failed`, `ltp-glibc: 63 passed, 0 failed`.
- Existing transparent internal signal: `read02` has TCONF in both libcs/arches; keep it visible and do not hide it.

## Promotion rules for this candidate set

- These cases are **not promoted by this document**. They are discovery inputs for targeted runs.
- Promote only after fresh LA/RV × musl/glibc evidence shows PASS with no TFAIL/TBROK/TCONF, timeout, ENOSYS, or panic/trap.
- Prior blocked cases are included only where they are plausible repair/validation targets; they must remain blocked unless fresh evidence proves otherwise.
- Timeout stays a failure signal.

## Priority candidate table (40 cases)

| Case | Prior status | Why candidate / risk | Suggested first check |
| --- | --- | --- | --- |
| `getpgid01` | blocked/incomplete in 2026-05-22 matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getpgid01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `getpgid02` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getpgid02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `getsid01` | blocked/incomplete in 2026-05-22 matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getsid01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `getsid02` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getsid02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `getppid02` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getppid02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `getrlimit02` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getrlimit02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `getrusage02` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getrusage02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `gettimeofday02` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=gettimeofday02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `uname02` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=uname02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `uname04` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=uname04 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `sched_get_priority_max01` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_get_priority_max01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `sched_get_priority_min01` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_get_priority_min01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `sched_getscheduler01` | not in stable; needs fresh matrix | getter/scheduler metadata neighbor; likely narrow process/time ABI semantics | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_getscheduler01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `clock_getres01` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_getres01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `clock_gettime01` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_gettime01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `nanosleep01` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=nanosleep01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `nanosleep02` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=nanosleep02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `pause01` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=pause01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `rt_sigprocmask01` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=rt_sigprocmask01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `rt_sigprocmask02` | not in stable; needs fresh matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=rt_sigprocmask02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `sigpending02` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sigpending02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `sigprocmask01` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sigprocmask01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `sigsuspend01` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sigsuspend01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `kill02` | blocked/incomplete in 2026-05-22 matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=kill02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `kill05` | not in stable; needs fresh matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=kill05 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `sigaction02` | not in stable; needs fresh matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sigaction02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `rt_sigaction02` | not in stable; needs fresh matrix | time/signal semantics; current dispatch has several related syscalls but prior evidence had TCONF/timeout risks | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=rt_sigaction02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `access02` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=access02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `access04` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=access04 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `dup03` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=dup03 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `pipe02` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `lseek02` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=lseek02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `mkdir02` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=mkdir02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `link02` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=link02 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `unlink05` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=unlink05 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `rename01` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=rename01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `statfs01` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `fstatfs01` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=fstatfs01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `statvfs01` | blocked/incomplete in 2026-05-22 matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=statvfs01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |
| `readlinkat01` | not in stable; needs fresh matrix | filesystem/syscall neighbor; must verify real errno/path behavior, no fake PASS | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=readlinkat01 LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh[ la]` |

## Grouped candidate rationale

### Low-risk getter / scheduler / process metadata candidates

`getpgid01`, `getpgid02`, `getsid01`, `getsid02`, `getppid02`, `getrlimit02`, `getrusage02`, `gettimeofday02`, `uname02`, `uname04`, `sched_get_priority_max01`, `sched_get_priority_min01`, `sched_getscheduler01`

Rationale: these are closest to already stable process/getter cases such as `getpid01`, `getppid01`, `getpgrp01`, `getrlimit01`, `getrusage01`, `gettid01`, and `uname01`. Current source includes `sys_getpgid` dispatch, so `getpgid*` should be classified with fresh targeted evidence before any ABI edit.

### Time / signal candidates

`clock_getres01`, `clock_gettime01`, `nanosleep01`, `nanosleep02`, `pause01`, `rt_sigprocmask01`, `rt_sigprocmask02`, `sigpending02`, `sigprocmask01`, `sigsuspend01`, `kill02`, `kill05`, `sigaction02`, `rt_sigaction02`

Rationale: current source dispatches `clock_getres`, `nanosleep`, and `rt_sigprocmask`, while prior matrices still marked several related cases blocked or incomplete. These are good discovery candidates but not safe promotions without fresh logs because time/signal cases can expose scheduler wakeup or signal-mask edge cases.

### Filesystem / syscall-neighbor candidates

`access02`, `access04`, `dup03`, `pipe02`, `lseek02`, `mkdir02`, `link02`, `unlink05`, `rename01`, `statfs01`, `fstatfs01`, `statvfs01`, `readlinkat01`, `readlinkat02`, `symlinkat01`, `fchmod02`, `fchmod03`, `truncate02`, `ftruncate03`

Rationale: current source already has real implementations for many nearby operations (`dup3`, `pipe2`, `mkdirat`, `unlinkat`, `renameat2`, `statfs`, `fstatfs`, `readlinkat`, `symlinkat`, `lseek`). These should be run in small batches to distinguish real remaining errno/path gaps from cases that became clean after previous ABI work.

### Misc/proc candidates

`sysinfo01`, `getgroups01`, `getpriority01`, `setpgid01`, `setsid01`, `wait01`, `wait02`, `waitpid01`, `waitpid03`

Rationale: classify separately; do not mix with first promotion batch unless targeted logs are clean.

## Explicit guardrails from prior blocked evidence

The 2026-05-22 combined matrix listed these cases as blocked/incomplete and they must not be promoted from stale evidence:

`access02 access04 clock_getres01 clock_gettime01 dup03 fstatfs01 getpgid01 getsid01 kill02 link02 lseek02 mkdir02 nanosleep01 nanosleep02 pause01 pipe02 read02 rename01 rt_sigprocmask01 sigpending02 sigprocmask01 sigsuspend01 statfs01 statvfs01 sysinfo01 unlink05`

## Suggested first targeted batches

### Batch 1: time/signal/process classification

```sh
cases='getpgid01,getpgid02,getsid01,getsid02,getppid02,getrlimit02,getrusage02,gettimeofday02,uname02,uname04,sched_get_priority_max01,sched_get_priority_min01'
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh 2>&1 | tee docs/ltp-score-improvement-2026-05-23/raw/worker1-rv-batch1.log
LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh la 2>&1 | tee docs/ltp-score-improvement-2026-05-23/raw/worker1-la-batch1.log
```

### Batch 2: filesystem/syscall-neighbor classification

```sh
cases='access02,access04,dup03,pipe02,lseek02,mkdir02,link02,unlink05,rename01,statfs01,fstatfs01,statvfs01'
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh 2>&1 | tee docs/ltp-score-improvement-2026-05-23/raw/worker1-rv-batch2.log
LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh la 2>&1 | tee docs/ltp-score-improvement-2026-05-23/raw/worker1-la-batch2.log
```

## Worker note

Two native read-only subagents were spawned as required by the delegation contract, but both misinterpreted their read-only probe scope as a global prohibition on this worker task and one of them transitioned Task 1 to `failed`. This document is the continued deliverable despite that lifecycle corruption; leader may need to requeue or reconcile the task state.
