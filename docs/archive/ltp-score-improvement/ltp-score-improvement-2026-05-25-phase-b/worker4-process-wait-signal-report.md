# Worker 4 process / wait / signal lane report

Date: 2026-05-25
Team: `ltp-stable350-to-stab-7c9de325`
Worker: `worker-4`
Task: `task-4`

## Guardrails followed

- ACKed leader mailbox and claimed `task-4` before lane work.
- Worked only in the worker-4 worktree.
- Did not mutate `.omx/ultragoal`.
- Did not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Did not start QEMU / `run-eval.sh`.
- Worker evidence below is static/discovery evidence only; promotion and final gates remain leader-owned.

Live stable-list check from `examples/shell/src/cmd.rs::LTP_STABLE_CASES`:

| Check | Result |
| --- | --- |
| stable count | 350 total / 350 unique |
| `waitid07` | absent |
| `waitid08` | absent |
| `waitid10` | absent |
| `kill02` | absent |

## Lane target summary

| Case | Current evidence | Static root-cause read | Patch decision in this lane |
| --- | --- | --- | --- |
| `waitid07` | Historical RV glibc+musl fail with `TFAIL=5`; later RV musl short probe still fails. | Upstream test waits for a child stopped by `SIGSTOP` using `waitid(P_PID, child, ..., WSTOPPED | WNOWAIT)` and checks `si_code=CLD_STOPPED`, `si_status=SIGSTOP`, `si_signo=SIGCHLD`. Local `sys_waitid()` currently requires `WEXITED`, does not allow `WSTOPPED`/`WNOWAIT`, and `UserProcess` has no child stopped-state event queue. | No patch: correct fix requires a process child-state model for stopped events and non-reaping `WNOWAIT`, not a safe one-line lane patch. |
| `waitid08` | Historical RV glibc+musl fail with `TFAIL=10`; later RV musl short probe still fails. | Upstream test first observes `CLD_STOPPED`, sends `SIGCONT`, then waits with `WCONTINUED` and expects `CLD_CONTINUED`/`SIGCONT`. Local `sys_waitid()` does not allow `WSTOPPED`/`WCONTINUED`, and signal delivery does not persist wait-visible stop/continue transitions. | No patch: same as `waitid07`; needs stop/continue transition accounting and wait filtering. |
| `waitid10` | Historical RV glibc+musl fail with `TBROK=1`; later RV musl short probe still `TBROK=1`. | Upstream setup reads `/proc/sys/kernel/core_pattern` before checking `RLIMIT_CORE`, then forks a child that raises `SIGFPE` and expects `waitid(..., WEXITED)` to report `SIGFPE` and `CLD_DUMPED` or `CLD_KILLED`. Local wait-status code already has `signal_wait_status(SIGFPE)` and `waitid_siginfo()` mapping for core-dump signals, so the first blocker is likely the missing proc sysctl file rather than signal status. | No code patch in this lane because the narrow likely fix is synthetic `/proc/sys/kernel/core_pattern`, which touches shared proc/VFS files owned by other lanes. Recommend a tiny leader-approved synthetic-proc patch plus serial target rerun. |
| `kill02` | RV became clean for glibc+musl in phase-a discovery, but LA glibc failed in `stable350-la-final-summary.txt` with `TBROK=4`; `kill02` was left out of the final stable350 list. | Upstream test relies on process groups: `kill(0, SIGUSR1)` should reach processes in the caller's process group and not children that called `setpgrp()`. Local `sys_kill(pid==0)` routes through `deliver_process_group_signal(process.pgid(), ...)`, and `sys_setpgid`/`setsid` update `pgid`. Existing static code shape is plausibly correct; the LA glibc `TBROK` needs raw subtest text before changing signal or process-group semantics. | No patch: mixed arch/libc evidence and no raw LA `TBROK` text in this worktree. Keep demoted until leader reruns isolated LA glibc or captures raw failure lines. |

## Evidence table

| Source | Relevant rows / signal |
| --- | --- |
| `docs/ltp-score-improvement-2026-05-24-phase-a/target-waitid-extended-rv-summary.txt` | `waitid07` RV glibc+musl `FAIL`, `TFAIL=5`; `waitid08` RV glibc+musl `FAIL`, `TFAIL=10`; `waitid10` RV glibc+musl `FAIL`, `TBROK=1`. |
| `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-discovery-process-short-001-summary.txt` | `waitid07`, `waitid08`, `waitid10` still fail on RV musl with the same TFAIL/TBROK shape. |
| `docs/ltp-score-improvement-2026-05-24-phase-a/target-kill-signal-rv-summary.txt` | Older `kill02` was RV glibc PASS but RV musl timeout/TFAIL. |
| `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-discovery-fsproc-001-summary.txt` | Later `kill02` RV glibc+musl both PASS clean. |
| `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-la-final-summary.txt` | `kill02` LA musl PASS, LA glibc FAIL with `TBROK=4`; this explains why it remained absent from stable350. |
| `/tmp/ltp-src-snips/{waitid07,waitid08,waitid10,kill02}.c` fetched from Linux Test Project upstream | Used only for static expected-behavior reading; no repository files were added from `/tmp`. |

## Implementation notes and recommended next steps

1. `waitid07` / `waitid08` need a real wait-visible child event model:
   - store child stopped and continued events separately from exit state;
   - allow `WSTOPPED`, `WCONTINUED`, and `WNOWAIT` in `sys_waitid()`;
   - preserve non-reaping behavior for `WNOWAIT`;
   - report `siginfo_t` with `CLD_STOPPED` / `CLD_CONTINUED`, `si_status=SIGSTOP` / `SIGCONT`, and `si_signo=SIGCHLD`;
   - avoid regressing existing clean `waitid01`-`waitid06`, `waitid09`, `waitid11`, `waitpid*`, and default-fatal signal wait status.

2. `waitid10` has a smaller first blocker:
   - add a read-only synthetic `/proc/sys/kernel/core_pattern` (for example `core\n` or a pipe-prefixed value if the intended default is no file dump);
   - keep the existing `SIGFPE` wait-status path visible;
   - rerun `waitid10` on RV+LA / glibc+musl before deciding whether signal status needs more work.

3. `kill02` should not be re-added from RV-only success:
   - first capture raw LA glibc `TBROK` lines from an isolated leader run;
   - if the raw failure is setup/pipe/read timing, inspect signal handler delivery and nonblocking pipe behavior;
   - if the raw failure is process-group leakage, inspect `setpgrp()`/`setpgid()` semantics and `deliver_process_group_signal()` membership.

## Verification performed

- Parsed live `LTP_STABLE_CASES` from `examples/shell/src/cmd.rs`: 350 total / 350 unique; all four lane targets absent.
- Read local implementations:
  - `examples/shell/src/uspace/process_lifecycle.rs::sys_waitid`, `waitid_pid_filter`, `waitid_siginfo`, `signal_wait_status`.
  - `examples/shell/src/uspace/signal_abi.rs::sys_kill`, `deliver_process_group_signal`, signal default handling.
  - `examples/shell/src/uspace/process_abi.rs::sys_setpgid`, `sys_setsid`.
- Read historical parser summaries listed above.
- Fetched upstream LTP source snippets to `/tmp/ltp-src-snips` for expectation mapping only.
- Ran no QEMU and no `run-eval.sh`.

## Stop condition

Lane report is complete. No source patch was made because the safe candidate (`/proc/sys/kernel/core_pattern`) is shared proc/VFS scope and the waitid stop/continue fix is larger than this lane can honestly verify without QEMU.
