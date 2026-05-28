# LTP 75-to-90 Discovery+Stats candidate matrix (2026-05-24)
## Baseline
- Current source `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: **75** cases per libc/arch.
- Existing latest top-level `output_rv.md` / `output_la.md` summaries in this worktree still show stable-63 (`PASS LTP CASE: 126`, `FAIL LTP CASE: 0` each); therefore this lane treats the source stable-75 list plus `docs/ltp-score-improvement-2026-05-23/stable75-targeted-matrix.md` as the 75-case handoff evidence.
- `docs/ltp-score-improvement-2026-05-23/stable75-targeted-matrix.md` reports 74 clean promotion candidates and 1 transparent blocked/incomplete case (`read02` TCONF), matching the existing policy that `read02` stays visible rather than hidden.
- No `.omx/ultragoal` files were read or modified by this worker.

## Promotion rule

Promote a new case only after fresh targeted RV+LA x musl+glibc evidence shows wrapper PASS with **zero** internal TFAIL/TBROK/TCONF, timeout, ENOSYS, and panic/trap. Timeout remains a failure signal.

## Candidate matrix

| Case | Priority | Group | Inventory | Prior blocked | Source/support evidence | Gate |
| --- | --- | --- | --- | --- | --- | --- |
| `getpgid01` | P1-first-batch | process/credentials/scheduler | yes | yes | sys_getpgid dispatch (syscall_dispatch.rs:350, process_abi.rs:76) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getsid01` | P1-first-batch | process/credentials/scheduler | yes | yes | sys_getsid dispatch (syscall_dispatch.rs:351, process_abi.rs:83) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getrusage02` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getrusage dispatch (syscall_dispatch.rs:249, system_info.rs:112) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `gettimeofday02` | P1-first-batch | process/credentials/scheduler | yes | no | sys_gettimeofday dispatch (syscall_dispatch.rs:244, time_abi.rs:532) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `gettid02` | P1-first-batch | process/credentials/scheduler | yes | no | direct __NR_gettid dispatch (syscall_dispatch.rs:275) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getgroups01` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getgroups dispatch (syscall_dispatch.rs:344, credentials.rs:282) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getresuid01` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getresuid dispatch (syscall_dispatch.rs:339, credentials.rs:252) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getresuid02` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getresuid dispatch (syscall_dispatch.rs:339, credentials.rs:252) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getresuid03` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getresuid dispatch (syscall_dispatch.rs:339, credentials.rs:252) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getresgid01` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getresgid dispatch (syscall_dispatch.rs:341, credentials.rs:260) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getresgid02` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getresgid dispatch (syscall_dispatch.rs:341, credentials.rs:260) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getresgid03` | P1-first-batch | process/credentials/scheduler | yes | no | sys_getresgid dispatch (syscall_dispatch.rs:341, credentials.rs:260) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sched_getparam01` | P1-first-batch | process/credentials/scheduler | yes | no | sys_sched_getparam dispatch (syscall_dispatch.rs:257, resource_sched.rs:162) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sched_getscheduler01` | P1-first-batch | process/credentials/scheduler | yes | no | sys_sched_getscheduler dispatch (syscall_dispatch.rs:261, resource_sched.rs:197) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sched_getscheduler02` | P1-first-batch | process/credentials/scheduler | yes | no | sys_sched_getscheduler dispatch (syscall_dispatch.rs:261, resource_sched.rs:197) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `waitpid01` | P1-first-batch | process/credentials/scheduler | yes | no | wait-family via sys_wait4 dispatch (syscall_dispatch.rs:401, process_lifecycle.rs:855) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sched_getaffinity01` | P2-backup | process/credentials/scheduler | yes | no | sys_sched_getaffinity dispatch (syscall_dispatch.rs:271, resource_sched.rs:322) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sched_getattr01` | P2-backup | process/credentials/scheduler | yes | no | sys_sched_getattr dispatch (syscall_dispatch.rs:265, resource_sched.rs:230) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sched_getattr02` | P2-backup | process/credentials/scheduler | yes | no | sys_sched_getattr dispatch (syscall_dispatch.rs:265, resource_sched.rs:230) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `setpgid01` | P2-backup | process/credentials/scheduler | yes | no | sys_setpgid dispatch (syscall_dispatch.rs:349, process_abi.rs:20) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `setsid01` | P2-backup | process/credentials/scheduler | yes | no | sys_setsid dispatch (syscall_dispatch.rs:352, process_abi.rs:90) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `waitpid03` | P2-backup | process/credentials/scheduler | yes | no | wait-family via sys_wait4 dispatch (syscall_dispatch.rs:401, process_lifecycle.rs:855) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getpriority01` | P2-backup | process/credentials/scheduler | yes | no | not yet confirmed in dispatch; classify before any promotion | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getpriority02` | P2-backup | process/credentials/scheduler | yes | no | not yet confirmed in dispatch; classify before any promotion | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getrandom01` | P2-backup | process/credentials/scheduler | yes | no | sys_getrandom dispatch (syscall_dispatch.rs:246, user_memory.rs:33) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `getrandom02` | P2-backup | process/credentials/scheduler | yes | no | sys_getrandom dispatch (syscall_dispatch.rs:246, user_memory.rs:33) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `get_robust_list01` | P2-backup | process/credentials/scheduler | yes | no | sys_get_robust_list dispatch (syscall_dispatch.rs:317, task_context.rs:159) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `clock_getres01` | P3-risk-gated | time/signal | yes | yes | sys_clock_getres dispatch (syscall_dispatch.rs:243, time_abi.rs:521) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `clock_gettime01` | P3-risk-gated | time/signal | yes | yes | sys_clock_gettime dispatch (syscall_dispatch.rs:242, time_abi) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `clock_nanosleep01` | P3-risk-gated | time/signal | yes | no | sys_clock_nanosleep dispatch (syscall_dispatch.rs:252, time_abi.rs:596) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `nanosleep01` | P3-risk-gated | time/signal | yes | yes | sys_nanosleep dispatch (time_abi) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `nanosleep02` | P3-risk-gated | time/signal | yes | yes | sys_nanosleep dispatch (time_abi) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `rt_sigprocmask01` | P3-risk-gated | time/signal | yes | yes | sys_rt_sigprocmask dispatch (syscall_dispatch.rs:368, signal_abi.rs:498) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `rt_sigprocmask02` | P3-risk-gated | time/signal | yes | no | sys_rt_sigprocmask dispatch (syscall_dispatch.rs:368, signal_abi.rs:498) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sigpending02` | P3-risk-gated | time/signal | yes | yes | signal-mask surface; prior blocked needs classification | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `sigprocmask01` | P3-risk-gated | time/signal | yes | yes | libc wrapper over signal mask surface; prior blocked needs classification | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `pause01` | P3-risk-gated | time/signal | yes | yes | signal sleep/wakeup surface; prior blocked needs classification | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `kill02` | P3-risk-gated | time/signal | yes | yes | sys_kill dispatch exists; prior blocked needs classification | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `dup03` | P3-risk-gated | fs/fd/syscall-neighbor | yes | yes | sys_dup3/dup surface exists; prior blocked needs errno classification | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `pipe02` | P3-risk-gated | fs/fd/syscall-neighbor | yes | yes | sys_pipe2/pipe surface exists; prior blocked needs flag/errno classification | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `statfs01` | P3-risk-gated | fs/fd/syscall-neighbor | yes | yes | sys_statfs dispatch (syscall_dispatch.rs:99, metadata.rs:759) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `fstatfs01` | P3-risk-gated | fs/fd/syscall-neighbor | yes | yes | sys_fstatfs dispatch (syscall_dispatch.rs:100, metadata.rs:782) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `readlinkat01` | P3-risk-gated | fs/fd/syscall-neighbor | yes | no | sys_readlinkat dispatch (syscall_dispatch.rs imports metadata readlinkat) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `symlinkat01` | P3-risk-gated | fs/fd/syscall-neighbor | yes | no | sys_symlinkat dispatch (syscall_dispatch.rs imports metadata symlinkat) | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `fchmod02` | P3-risk-gated | fs/fd/syscall-neighbor | yes | no | sys_fchmod/fchmodat surface exists; needs errno/mode validation | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |
| `ftruncate03` | P3-risk-gated | fs/fd/syscall-neighbor | yes | no | ftruncate surface exists; needs size/errno validation | RV+LA x musl+glibc PASS with zero TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap |

## First targeted batch recommendation

Batch goal: classify 16 process/credential/scheduler neighbors that are outside the current stable-75 source list, present in the common RV/LA LTP inventory, and mostly map to existing syscall dispatch surfaces. If all 16 pass cleanly, the stable list can move from 75 toward 91 cases per libc/arch with one follow-up promotion patch.

```text
getpgid01,getsid01,getrusage02,gettimeofday02,gettid02,getgroups01,getresuid01,getresuid02,getresuid03,getresgid01,getresgid02,getresgid03,sched_getparam01,sched_getscheduler01,sched_getscheduler02,waitpid01
```

Recommended commands (save raw logs before summarizing):

```sh
cases="$(cat docs/ltp-score-improvement-2026-05-24/raw/first-targeted-batch-cases.txt)"
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh 2>&1 | tee docs/ltp-score-improvement-2026-05-24/raw/rv-first-targeted-batch.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-24/raw/rv-first-targeted-batch.log > docs/ltp-score-improvement-2026-05-24/rv-first-targeted-batch-summary.txt
LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=8 ./run-eval.sh la 2>&1 | tee docs/ltp-score-improvement-2026-05-24/raw/la-first-targeted-batch.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-24/raw/la-first-targeted-batch.log > docs/ltp-score-improvement-2026-05-24/la-first-targeted-batch-summary.txt
python3 scripts/ltp_summary.py --promotion-candidates docs/ltp-score-improvement-2026-05-24/raw/rv-first-targeted-batch.log docs/ltp-score-improvement-2026-05-24/raw/la-first-targeted-batch.log > docs/ltp-score-improvement-2026-05-24/first-targeted-batch-promotion-matrix.md
```

## Blocked carry-forward

Known prior blocked/incomplete cases remain non-promoted until fresh targeted evidence proves them clean: `access02 access04 clock_getres01 clock_gettime01 dup03 fstatfs01 getpgid01 getsid01 kill02 link02 lseek02 mkdir02 nanosleep01 nanosleep02 pause01 pipe02 read02 rename01 rt_sigprocmask01 sigpending02 sigprocmask01 sigsuspend01 statfs01 statvfs01 sysinfo01 unlink05`.
