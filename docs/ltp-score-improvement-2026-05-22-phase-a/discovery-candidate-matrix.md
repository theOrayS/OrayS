# Phase A discovery candidate matrix: stable85 -> stable100/110

## Scope

- Lane: Discovery/Matrix only; no `.omx/ultragoal` mutation.
- Current source baseline: `examples/shell/src/cmd.rs::LTP_STABLE_CASES` has **85** cases per libc/arch.
- Image inventory was read with `debugfs -R ls -p /{musl,glibc}/ltp/testcases/bin` from `/root/oskernel2026-orays/sdcard-rv.img` and `/root/oskernel2026-orays/sdcard-la.img`.
- Common executable LTP cases across RV/LA x musl/glibc: **2822**; common not-yet-stable: **2737**.
- Promotion remains gated by fresh LA/RV x musl/glibc `scripts/ltp_summary.py --promotion-candidates` evidence with no TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap, event failure, or non-PASS status.

## Recommended wave files

- `docs/ltp-score-improvement-2026-05-22-phase-a/wave1-cases.txt`: `sched_getscheduler02,sched_getparam01,getpgid01,getgroups01,gettid02,waitpid01,gettimeofday02,getrusage02,getpriority01,getpriority02,setpriority01,setpriority02,waitpid03`
- `docs/ltp-score-improvement-2026-05-22-phase-a/wave2-cases.txt`: `clock_gettime03,clock_gettime04,clock_nanosleep01,clock_nanosleep02,nanosleep01,nanosleep02,kill05,sigaction02,pause01,sigprocmask01,rt_sigprocmask01,sigpending02,sigsuspend01,access02,access04,link02,rename01,unlink05`

## Candidate matrix

| Case | Source | Family | Rank | Promotion status | Notes |
| --- | --- | --- | --- | --- | --- |
| `sched_getscheduler02` | Wave 1 | proc/sched/wait/getter | High | targeted-required | known blocker from phase-d targeted-promotion11: LA musl TFAIL=1; rerun only after real scheduler fix |
| `sched_getparam01` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `getpgid01` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `getgroups01` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `gettid02` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `waitpid01` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `gettimeofday02` | Wave 1 | proc/sched/wait/getter | High | targeted-required | phase-a plan calls out timeout/root-cause risk; timeout cannot count as PASS |
| `getrusage02` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `getpriority01` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `getpriority02` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `setpriority01` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `setpriority02` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `waitpid03` | Wave 1 | proc/sched/wait/getter | High | targeted-required | fresh targeted matrix required; known sched_getscheduler02 LA musl TFAIL risk |
| `clock_gettime03` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `clock_gettime04` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `clock_nanosleep01` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `clock_nanosleep02` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `nanosleep01` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `nanosleep02` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `kill05` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `sigaction02` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `pause01` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `sigprocmask01` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `rt_sigprocmask01` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `sigpending02` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `sigsuspend01` | Wave 2 | time/signal | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `access02` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `access04` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `link02` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `rename01` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `unlink05` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `mkdir02` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `lseek02` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `pipe02` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `dup03` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `statfs01` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `statvfs01` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `fstatfs01` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `sysinfo01` | Wave 2 | fs/metadata/syscall | Medium | targeted-required | fresh targeted matrix required; timeouts/TCONF block promotion |
| `clock_getres01` | Backup | extended syscall family | Medium/High | targeted-required | explicit prior TCONF risk; excluded from primary wave unless fresh logs are clean |
| `clock_gettime01` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `kill02` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `rt_sigprocmask02` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `readlinkat01` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `readlinkat02` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `symlinkat01` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `fchmod02` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `fchmod03` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `truncate02` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `ftruncate03` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `getrlimit03` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `getrusage03` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid04` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid06` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid07` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid08` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid09` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid10` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid11` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid12` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `waitpid13` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `sched_get_priority_max01` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `sched_get_priority_min01` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `fstatfs02` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `statfs02` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `dup04` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `dup05` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |
| `pipe03` | Backup | extended syscall family | Medium/High | targeted-required | only after smaller waves classify blockers |

## Explicit exclusions / blocker reasons

- `read02`: already stable but pass-with-TCONF; keep transparent, never claim as clean promotion.
- `clock_getres01`: prior plan/handoff flags possible TCONF; keep out of first promotion wave until fresh targeted evidence is clean.
- `gettimeofday02`: phase-a plan flags timeout/root-cause risk; classify separately and do not promote if any timeout appears.
- `sched_getscheduler02`: phase-d targeted matrix blocked on LA musl `TFAIL=1/event-failures=1/status=FAIL`; requires real scheduler semantics fix before promotion.
- `waitpid02`: phase-a plan listed it, but current RV/LA x musl/glibc image inventory did not contain that exact binary name; use present `waitpid01`, `waitpid03`, and backup `waitpid04+` instead.
- `times01`: phase-a plan listed it in Wave 1, but it is already in stable85; no new promotion value.

## Suggested targeted validation commands

```bash
cases=$(cat docs/ltp-score-improvement-2026-05-22-phase-a/wave1-cases.txt)
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" ./run-eval.sh 2>&1 | tee docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-rv.log
LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" ./run-eval.sh la 2>&1 | tee docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-la.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-rv.log > docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-rv-summary.txt
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-la.log > docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-la-summary.txt
python3 -B scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-rv.log docs/ltp-score-improvement-2026-05-22-phase-a/wave1-targeted-la.log > docs/ltp-score-improvement-2026-05-22-phase-a/wave1-promotion-matrix.md
```

## Subagent findings integrated

- Russell confirmed `LTP_STABLE_CASES`, `selected_ltp_cases`, timeout selection, and `scripts/ltp_summary.py --promotion-candidates` as the correct local surfaces.
- Mendel confirmed stable85 baseline evidence from phase-d summaries and the hard blocker for `sched_getscheduler02`; it also pointed to current-output promotion matrix semantics.
