# Phase 0 time/select/signal/process candidate lane

- Worker: `worker-3`
- Date: 2026-06-01
- Task: report-only/source-diagnosis for stable556/stable606 planning
- Scope: mine current source/docs/archives; **no stable promotion, no Ultragoal checkpoint, no QEMU**.

## Source-of-truth baseline

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` re-read in this worktree: `506 total / 506 unique / 0 duplicate`.
- Current long-term plan defines success as RV + LA × musl + glibc wrapper PASS with `scripts/ltp_summary.py` showing no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` or unclosed logs (`docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md:10`).
- Stable506 final gate is the last trusted baseline: RV and LA each `PASS LTP CASE 1012`, `FAIL 0`, with only inherited `read02` `TCONF 4` per arch (`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/validation.md:56-62`).
- Milestones from the current 506 baseline are stable556 and stable606 first; worker lanes must provide candidate evidence only, while leader owns promotion gates and stable-list edits (`docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md:56-58`).

## Compact citation map

For table readability, short evidence filenames below refer to these exact committed paths:

- Session 1 matrix: `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix/candidate-matrix-stable460-to-500plus.md`.
- Session 2 RV summary: `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-02-time-select-signal/summary-rv-time-select.txt`.
- Session 2 LA poll/pselect summary: `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-02-time-select-signal/summary-la-poll-pselect-regression.txt`.
- Session 2 promotion notes and validation: `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-02-time-select-signal/promotion-candidates.md`, `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-02-time-select-signal/validation.md`.
- Referenced raw-log paths from committed validation, not committed here: `target/ltp-long-term-session2/session2-rv-time-select.log`, `target/ltp-long-term-session2/session2-la-poll-pselect-regression.log`, `target/ltp-long-term-session8/session8-rv-stable506.log`, `target/ltp-long-term-session8/session8-la-stable506.log`.

## Already stable / not new candidates

These rows should be used as regressions, not counted as phase0 candidates:

- Session 2 already promoted `getitimer01` and `ppoll01` after RV targeted and LA confirmation were parser-clean (`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-02-time-select-signal/promotion-candidates.md:5-12`).
- Session 6 already promoted 21 futex/process/IPC rows, including `kill02`, `tkill01`, `tkill02`, `vfork01`, and `vfork02`, with RV/LA clean21 gates both `PASS LTP CASE 42`, `FAIL 0` (`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-06-futex-process-ipc/promotion-candidates.md:33-35`).
- Live stable list also already contains `pselect02`, `pselect02_64`, `pselect03`, `pselect03_64`, `waitpid01/04/10/11/12/13`, `sigaction02`, `sigprocmask01`, `sigsuspend01`, `setrlimit05`, `sched_getscheduler02`, and many adjacent scheduler/signal/fork guards.

## Ranked candidate queue

Priority meaning:

- **M01**: realistic to contribute to stable556 after fresh targeted RV/LA × musl/glibc proof.
- **M02**: realistic for stable606 only after a small source diagnosis/fix or more targeted data.
- **Deferred**: useful backlog, but current evidence says not first-two-milestone material.

| Rank | Case(s) | Milestone fit | Evidence state | Likely touchpoints | Required next proof | Risks / reason for rank |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | `poll02` | M01 candidate, but rerun first | Session 2 RV targeted had `poll02` clean on glibc+musl (`summary-rv-time-select.txt:25-26`); LA adjacent regression had glibc+musl clean (`summary-la-poll-pselect-regression.txt:17-18`). It was intentionally not promoted then (`promotion-candidates.md:16-20`). | `examples/shell/src/uspace/select_fdset.rs:330-368` (`sys_poll`/`sys_poll_until`), `current_unblocked_signal_pending`, deadline handling. | Fresh targeted RV + LA for `poll02`, then adjacent regression with `poll01`, `ppoll01`, `pselect02/03`, pipe/readiness cases. | Older 2026-05-26 scout diagnosed timer precision failures for `poll02`; Session 2 evidence is newer and cleaner, but this conflict makes a fresh gate mandatory. Runtime is ~10s per libc, so keep batch small. |
| 2 | `pselect01`, `pselect01_64` | M01 candidate | Session 1 full-sweep matrix marked both 4/4 clean sweep candidates (`candidate-matrix-stable460-to-500plus.md:40-41`). Session 2 RV targeted and LA adjacent regression were parser-clean (`summary-rv-time-select.txt:29-32`, `summary-la-poll-pselect-regression.txt:19-22`). | `select_fdset.rs:134-189` (`sys_pselect6`, temporary sigmask, timeout), `signal_abi.rs:142` (`install_temporary_signal_mask`). | Fresh targeted RV + LA for both cases, plus regression with stable `pselect02`, `pselect02_64`, `pselect03`, `pselect03_64`, `rt_sigprocmask01/02`, `rt_sigsuspend01`. | Signal-mask enter/restore bugs can regress stable signal cases; do not mix with broad signal rewrites. |
| 3 | `clock_nanosleep02` | M01/M02 candidate | Session 1 sweep said 4/4 clean (`candidate-matrix-stable460-to-500plus.md:37`); Session 2 RV targeted had both libcs clean (`summary-rv-time-select.txt:19-20`). No Session 2 LA targeted row was committed for this case. | `time_abi.rs:619-659` (`sys_nanosleep`, `sys_clock_nanosleep`, `sleep_duration`). | Fresh RV + LA targeted; include stable `clock_nanosleep04`, `nanosleep04`, `clock_gettime02`, `clock_settime01/02`. | `sleep_duration` busy-yields and does not return remaining time on ordinary signal interruption; avoid promoting if any signal/EINTR subtest appears. |
| 4 | `settimeofday01`, `time-schedule` | M01/M02 candidate | Session 1 sweep said 4/4 clean (`candidate-matrix-stable460-to-500plus.md:42-43`); Session 2 RV targeted had both libcs clean (`summary-rv-time-select.txt:35-38`). No Session 2 LA targeted confirmation was committed. | `time_abi.rs:537-557` (`sys_clock_gettime`/`sys_clock_settime`), `time_abi.rs:571-617` (`gettimeofday`, `times`). | Fresh RV + LA targeted; pair with `gettimeofday01/02`, `time01`, `times01`, `clock_settime01/02`. | `settimeofday01` affects global realtime offset; `time-schedule` may expose scheduler/timer drift. Keep isolated from process/wait batches. |
| 5 | `nice04` | M02 near-clean | Session 1 matrix: 3/4 clean; RV musl failed with `TFAIL=1` while RV glibc and LA musl/glibc were clean (`candidate-matrix-stable460-to-500plus.md:34`). Session 6 explicitly did not promote it (`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-06-futex-process-ipc/promotion-candidates.md:44`). | `resource_sched.rs:236-266` (`sys_getpriority`, `sys_setpriority`), uid/priority privilege model. | Reproduce RV musl first; if fixed, run RV+LA × musl/glibc and regression `nice01/02/03`, `getpriority01/02`, `setpriority02`, scheduler stable rows. | Could be libc errno/privilege interpretation rather than kernel logic; do not adjust uid/permission semantics only for this case. |
| 6 | `clone04` | M02 near-clean | Session 1 matrix: 3/4 clean; RV musl failed with `TBROK=1` while other combos were clean (`candidate-matrix-stable460-to-500plus.md:35`). Session 6 left `clone04` unpromoted (`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-06-futex-process-ipc/promotion-candidates.md:44`). | `process_lifecycle.rs:807-948` (`sys_clone`, fork-like/thread flags, `ENOSYS` for unsupported flags), signal-mask inheritance. | Reproduce RV musl, inspect raw subtest, then gate with stable `clone01/03/06/07`, `fork01/03-10`, `vfork01/02`, `waitpid*`. | `sys_clone` deliberately rejects unsupported exit signals/flags with `ENOSYS`; changing accepted flags can affect fork/thread semantics. |
| 7 | `setrlimit04`, `sched_rr_get_interval03`, `sched_setaffinity01`, `setpriority01`, `signal01` | M02 scout set | 2026-05-27 misc-process scout listed these as cautious first-run candidates with specific errno/privilege/signal risks (`worker5-batch002-misc-process-scout.md:52-58`). `setrlimit05` is now stable, but these are not in live stable506. | `resource_sched.rs:451-460`, `resource_sched.rs:562-655`, `signal_abi.rs:749`, `process_lifecycle.rs:1009-1128`. | RV first, one family at a time; only LA-confirm rows that are RV-clean. Required regression: `setrlimit01/02/03/05`, `getrlimit01/02/03`, stable scheduler rows, `signal02-05`, `kill02/03/06-09/11/12`. | Mostly errno/privilege/one-CPU affinity and wait-status edge cases. Good for stable606 if a small raw diagnosis finds a real narrow fix; too broad for blind M01 promotion. |
| 8 | `clock_gettime04`, `nanosleep01` | M02 only after repair | Session 1 sweep once marked them 4/4 clean (`candidate-matrix-stable460-to-500plus.md:36,38`), but Session 2 targeted RV found RV musl internal `TFAIL` for both (`summary-rv-time-select.txt:17-24`) and explicitly did not promote them (`promotion-candidates.md:24-26`). | `time_abi.rs:522-659`, monotonic/realtime conversion and sleep/rem handling. | Start with RV musl raw subtest capture; after source fix, run both architectures and libc variants. | Conflicting evidence means targeted failure wins. Treat as repair/blocker, not a clean queue. |
| 9 | `waitid07`, `waitid08`, `waitid10` | Deferred until wait-state model exists | Session 1/older process reports mark `waitid07` 0/4 clean and Session 6 left it unpromoted (`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-06-futex-process-ipc/promotion-candidates.md:42`). Later guardrail reports keep `waitid08/10` blocked by stopped/continued/core-pattern semantics. Current source only accepts `WNOHANG | WEXITED | __WNOTHREAD | __WALL` for `waitid` (`process_lifecycle.rs:1099-1104`). | `process_lifecycle.rs:1009-1128`, child state accounting, `/proc/sys/kernel/core_pattern` for `waitid10` setup. | Do not include in stable556. Before stable606, write a wait-state design note and add stopped/continued/WNOWAIT/core dump regression tests; then RV-only targeted. | Requires persistent stopped/continued child events, non-reap `WNOWAIT`, and synthetic proc data; high regression risk to stable `waitpid*` and `waitid01-06/09/11`. |
| 10 | `select01`, `select02`, `select03`, `getitimer02`, `epoll_wait01` | Deferred | Session 1 matrix marks them blocked: `select02` TCONF+timeout; `select01` TBROK; `select03` TCONF; `getitimer02` TCONF; `epoll_wait01` TBROK+ENOSYS (`candidate-matrix-stable460-to-500plus.md:29,90-93`). Session 2 confirmed `select02` RV timeout+TCONF (`validation.md:31-33`). | `select_fdset.rs`, epoll syscall dispatch gap, timer accounting. | Separate blocker batch; no stable list work until targeted raw logs show a true small source fix. | TCONF/timeout/ENOSYS make these invalid promotion candidates; epoll likely belongs to a broader fd/eventpoll model. |
| 11 | `clone02`, `execve01`, `execve05` | Deferred / process hardening | Session 1 matrix: `clone02` TFAIL+ENOSYS, `execve01/05` TBROK; Session 6 explicitly did not promote them (`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-06-futex-process-ipc/promotion-candidates.md:43-45`). | `process_lifecycle.rs:761-805` (`sys_execve`), `process_lifecycle.rs:807-948` (`sys_clone`). | Raw subtest diagnosis first; verify executable path/errno/CLOEXEC/fork-wait behavior before any patch. | Exec and clone changes are broad and can regress shell runner, fork/vfork, FD_CLOEXEC, and wait semantics. |
| 12 | `getcpu01`, `gethostname02`, `gethostid01`, `getpgid01`, `times03`, `kill05`, `kill10` | Deferred / diagnosis only | 2026-05-26 and 2026-05-27 reports classify these as blocked: musl TCONF/short-buffer, glibc ENOSYS, timer/accounting failures, process group TBROK, or signal timeout (`worker2-light-syscall-rv001-diagnosis.md:49-52`, `worker1-candidate-matrix-delta-after-reports.md:47-51`, `candidate-matrix-stable413-to-460.md:128-136`). | `process_abi.rs` for pgid/session, `system_info.rs` for hostname, `time_abi.rs:610-617` for `times`, signal delivery/wait status for `kill*`. | Keep as source-diagnosis backlog. `getcpu01` may accept a glibc-only `__NR_getcpu` shim, but musl TCONF prevents four-way promotion. | Not suitable for first two milestones unless leader wants non-promotion compatibility fixes. |

## Suggested stable556/stable606 use

### stable556 (M01) contribution from this lane

Use this lane to provide a small, low-risk slice rather than all +50 cases:

1. Fresh-targeted batch A: `poll02,pselect01,pselect01_64`.
2. If clean, targeted batch B: `clock_nanosleep02,settimeofday01,time-schedule`.
3. Only after both targeted batches are clean, run adjacent regression including current stable rows: `ppoll01,poll01,pselect02,pselect02_64,pselect03,pselect03_64,getitimer01,clock_nanosleep04,nanosleep04,gettimeofday01,gettimeofday02,time01,times01,rt_sigprocmask01,rt_sigsuspend01`.
4. Promotion still requires leader-owned RV + LA × musl + glibc parser-clean aggregate; this report is not promotion evidence.

### stable606 (M02) contribution from this lane

After M01 gates, use this lane for source-diagnosis/fix-first process work:

1. Near-clean process: `nice04`, `clone04`.
2. Cautious first-run source/errno cases: `setrlimit04`, `sched_rr_get_interval03`, `sched_setaffinity01`, `setpriority01`, `signal01`.
3. Design-first blockers: `waitid07/08/10`, `clone02`, `execve01/05`, `select01/02/03`, `getitimer02`, `epoll_wait01`.

## Regression matrix required before any promotion

- **Existing final baseline**: do not disturb stable506 final gate shape (`PASS LTP CASE 1012`, `FAIL 0`, inherited `read02` TCONF only).
- **Poll/select**: `poll01`, `ppoll01`, `pselect02`, `pselect02_64`, `pselect03`, `pselect03_64`, pipe/readiness cases, and signal-mask rows `rt_sigprocmask01/02`, `rt_sigsuspend01`.
- **Time**: `clock_gettime02`, `clock_settime01/02`, `clock_nanosleep04`, `nanosleep04`, `gettimeofday01/02`, `time01`, `times01`, alarm/itimer rows already stable.
- **Process/wait/signal**: `waitpid01/04/10/11/12/13`, `waitid01-06/09/11`, `fork01/03-10`, `clone01/03/06/07`, `vfork01/02`, `kill02/03/06-09/11/12`, `tkill01/02`, `sigaction02`, `signal02-05`.
- **Rlimit/scheduler/priority**: `getrlimit01/02/03`, `setrlimit01/02/03/05`, `nice01/02/03`, `getpriority01/02`, `setpriority02`, `sched_get*`, `sched_setparam*`, `sched_setscheduler*`, `sched_yield01`, `sched_tc2-5`.
- **Parser gate**: `scripts/ltp_summary.py` must show zero new internal failure categories, zero timeout, zero ENOSYS, zero panic/trap, and no marker-prefix/log-closure caveat beyond explicitly disclosed inherited `read02` TCONF.

## Source touchpoint summary

- `examples/shell/src/uspace/select_fdset.rs:134-189`, `303-322`, `330-368` — `pselect6`, `ppoll`, `poll`, deadline/EINTR/signal-mask behavior.
- `examples/shell/src/uspace/time_abi.rs:445-503`, `522-659` — itimer state, `getitimer`, `nanosleep`, `clock_nanosleep`, realtime/monotonic behavior.
- `examples/shell/src/uspace/signal_abi.rs:610-720`, `749-826` — signal mask/pending/sigsuspend/kill/tkill/tgkill behavior.
- `examples/shell/src/uspace/process_lifecycle.rs:761-948`, `1009-1128` — `execve`, `clone`, `wait4`, `waitid`; current `waitid` support is WEXITED-only.
- `examples/shell/src/uspace/resource_sched.rs:236-266`, `451-460`, `562-655` — priority, scheduler interval/affinity, rlimit/prlimit.
- `examples/shell/src/uspace/process_abi.rs:20-90`, `examples/shell/src/uspace/system_info.rs:134` — process group/session and hostname surfaces for deferred light-process cases.

## Risks and caveats

- Some 2026-05-26/05-27 reports conflict with later Session 2 evidence for `poll02`; newer Session 2 parser-clean evidence is more relevant, but the conflict is enough to require a fresh targeted gate before any promotion.
- Full-sweep and `high-yield-candidates.json` rows from 2026-05-29 are useful backlog evidence but stale relative to the live stable506 list: `getitimer01`, `ppoll01`, `kill02`, `tkill01/02`, `vfork01/02`, and many scheduler rows are already stable.
- `waitid*`, `clone*`, `execve*`, `select*`, and epoll blockers have shared semantics and should not be patched by a report-only worker without leader-approved scope.
- Blacklist/SKIP/status0/partial-arch PASS is not promotion evidence.

## Verification performed for this report

- Re-read live `LTP_STABLE_CASES`: `506 total / 506 unique / 0 duplicate`.
- Inspected committed docs/summaries under:
  - `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix/`
  - `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-02-time-select-signal/`
  - `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-06-futex-process-ipc/`
  - `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/`
  - older guardrail docs under `archive/docs-pre-review-2026-06-28/archive/ltp-score-improvement/ltp-score-improvement-2026-05-26-phase-a/` and `archive/docs-pre-review-2026-06-28/archive/ltp-score-improvement/ltp-score-improvement-2026-05-27-phase-a/`.
- Inspected source files listed in the touchpoint summary.
- Subagent skip reason: this was a narrow report-only/source-diagnosis task with enough committed evidence; no independent subagent was needed. CodeGraph was unavailable because this worker worktree has no `.codegraph` index, so source inspection used normal read-only repo commands.
