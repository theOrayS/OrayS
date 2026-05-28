# Worker 4 process / wait / signal lane report

Date: 2026-05-25
Team: `ltp-stable375-to-stab-eae749f6`
Worker: `worker-4`
Task: `task-4`
Mode: report-only, no QEMU

## Guardrails followed

- ACKed leader and claimed `task-4` before lane work.
- Did **not** run QEMU, `run-eval.sh`, or any evaluator command.
- Did **not** edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Did **not** edit leader-owned `.omx/ultragoal` state.
- Wrote this report only under `docs/ltp-score-improvement-2026-05-25-phase-c/`.
- Subagent skip reason: optional delegation skipped because this is a bounded report-only lane; direct static inspection plus existing parsed summaries were sufficient.

Live stable-list check from `examples/shell/src/cmd.rs::LTP_STABLE_CASES`:

| Check | Result |
| --- | --- |
| total / unique | 375 / 375 |
| `kill02` | absent |
| `waitid07` | absent |
| `waitid08` | absent |
| `waitid10` | absent |
| wait family already stable | `wait01`, `wait02`, `wait401`-`wait403`, `waitid01`-`waitid06`, `waitid09`, `waitid11`, `waitpid01`, `waitpid03`-`waitpid04`, `waitpid06`-`waitpid13` |
| signal/fork adjacent already stable | `kill03`, `kill06`-`kill09`, `kill11`, `kill12`, `fork01`, `fork03`-`fork05`, `fork07`-`fork10`, `signal02`-`signal05`, `rt_sigsuspend01` |

## Primary lane findings

| Case | Current evidence | musl / glibc / LA risk | Recommendation |
| --- | --- | --- | --- |
| `waitid07` | `target-primary30-rv-002-summary.txt` has RV glibc and RV musl `FAIL`, `TFAIL=5`. Older `target-waitid-extended-rv-summary.txt` shows the same shape. | Failure is libc-independent on RV, so LA is not worth spending until the semantic gap is fixed. Upstream test requires `waitid(..., WSTOPPED | WNOWAIT)` for a child stopped by `SIGSTOP`, with `CLD_STOPPED` and `si_status=SIGSTOP`. Local `sys_waitid()` only allows `WNOHANG | WEXITED | __WNOTHREAD | __WALL` and rejects options without `WEXITED`; it has no wait-visible stopped-event model or `WNOWAIT` non-reap path. | Do not promote. Requires real child stopped-event accounting plus `WSTOPPED`/`WNOWAIT` support before any RV+LA gate. |
| `waitid08` | `target-primary30-rv-002-summary.txt` has RV glibc and RV musl `FAIL`, `TFAIL=10`. Older `target-waitid-extended-rv-summary.txt` shows the same shape. | Failure is libc-independent on RV. Upstream observes `CLD_STOPPED`, sends `SIGCONT`, then expects `waitid(..., WCONTINUED)` to return `CLD_CONTINUED` / `SIGCONT`. Local `sys_waitid()` rejects `WSTOPPED` and `WCONTINUED`; signal delivery does not persist stop/continue transitions for parent waits. | Do not promote. Same repair family as `waitid07`, with extra continued-event accounting. |
| `waitid10` | `target-primary30-rv-002-summary.txt` has RV glibc and RV musl `FAIL`, `TBROK=1`. Older `target-waitid-extended-rv-summary.txt` shows the same shape. | Failure is libc-independent on RV. Upstream setup reads `/proc/sys/kernel/core_pattern` before forking a child that raises `SIGFPE` and expecting `waitid(..., WEXITED)` to report `SIGFPE` with `CLD_DUMPED` or `CLD_KILLED`. Local signal wait-status has a `SIGFPE` core-dump bit path, so the first blocker is likely missing synthetic proc sysctl data, not the final signal status mapping. LA should wait for an RV setup fix. | Do not promote. First try a tiny leader-approved synthetic `/proc/sys/kernel/core_pattern` read path, then rerun RV+LA × glibc+musl. |
| `kill02` | Targeted RV and targeted LA rows are clean (`target-primary30-rv-002`, `target-rvclean5-la-001`), but `stable375-la-final-001-summary.txt` later fails `la:musl:kill02` with `TBROK=4`. It was removed before final stable375 gates and replaced by `inode01`. | Highest risk is aggregate/ordering sensitivity, not simple single-case syscall behavior. Upstream sends `kill(0, SIGUSR1)` and expects only processes in the caller's process group to receive it; children that called `setpgrp()` must not receive it. Local code has plausible process-group routing (`sys_kill(pid==0)` -> `deliver_process_group_signal(process.pgid(), ...)`, `sys_setpgid`, `sys_setsid`), but the LA aggregate TBROK proves targeted clean evidence is insufficient. | Do **not** re-add without aggregate proof. If revisited, run isolated LA musl/glibc raw capture first, then a full stable aggregate containing `kill02`; only promote if both parsed summaries are clean. |

## Adjacent candidate triage

| Candidate group | Evidence seen | Risk notes | Next feasible action |
| --- | --- | --- | --- |
| `kill11`, `kill12`, `fork05`, `fork10`, `signal05` | Already in live stable375; final RV/LA gates were clean after `kill02` removal. | Good regression guards for signal/fork behavior; not new score candidates. | Keep as stable regression coverage. |
| `kill10`, `signal01`, `sigrelse01`, `fork13`, `fork14` | Phase-b candidate matrix marks them not promoted; raw summaries show timeout or failure. `target-scout26-rv-001` has `kill10` and `signal01` RV musl timeouts. `target-scout14-rv-001` has `fork13`, `fork14`, and glibc `sigrelse01` timeouts. | Timeout class; likely signal wake/pause or long-running child coordination. Musl-specific timeout appears for `kill10`/`signal01`, while fork13/fork14 fail on both RV libcs. LA should not be attempted before an RV timeout root cause. | Defer from promotion. If code work is assigned, start with raw timeout logs and signal wakeup/pause semantics, not with stable-list edits. |
| `kill05`, `kill13` | `kill05` has RV TFAIL+TBROK in phase-a and phase-b raw summaries. `kill13` has RV TBROK in phase-a/phase-b scout summaries. | Setup/semantic failures on RV; no clean libc baseline. | Defer; needs individual raw subtest text before code changes. |
| `pause01` | `target-fill9-rv-001` has RV glibc and musl timeout. | Pause/signal-delivery wakeup risk; same family as signal timeout candidates. | Defer until signal wakeup model is audited. |
| `sigwait01`, `sigpending02`, `rt_sigaction03`, `sigaltstack01`, `sigaltstack02` | `target-scout14-rv-001` has TFAIL/TBROK or partial-only results. `sigaltstack02` is musl-clean but glibc fails, so not promotion evidence. | Libc divergence and signal ABI depth; high risk for accidental SKIP/pass laundering if only wrapper status is used. | Defer; require focused ABI tests and parser summaries before promotion. |
| `fork02`, `fork06`, `kill04` | `target-fill9-rv-001` has fast wrapper `FAIL -1` rows. | Likely harness/setup or unimplemented behavioral assumptions; no clean baseline. | Low priority for this promotion round. |
| `waitpid02`, `waitpid05` | Not in live stable375; older phase-d notes kept `waitpid05` in a separate isolated wait batch. No current clean four-way evidence found in this phase. | Waitpid family is partially stable, but absent cases still require isolated proof. | Treat as follow-up discovery, not direct promotion. |

## Evidence references

- Live stable count parsed from `examples/shell/src/cmd.rs`: `375 total / 375 unique`.
- `docs/ltp-score-improvement-2026-05-25-phase-b/raw/target-primary30-rv-002-summary.txt`:
  - `kill02` RV glibc+musl PASS clean;
  - `waitid07` RV glibc+musl FAIL with `TFAIL=5`;
  - `waitid08` RV glibc+musl FAIL with `TFAIL=10`;
  - `waitid10` RV glibc+musl FAIL with `TBROK=1`.
- `docs/ltp-score-improvement-2026-05-25-phase-b/raw/target-rvclean5-la-001-summary.txt`:
  - `kill02` targeted LA glibc+musl PASS clean.
- `docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-la-final-001-summary.txt`:
  - aggregate `la:musl:kill02` FAIL with `TBROK=4`, proving targeted clean evidence was insufficient.
- `docs/ltp-score-improvement-2026-05-25-phase-b/stable375-delivery-report.md`:
  - final stable375 delivered with `kill02` removed and final RV/LA gates clean.
- `docs/ltp-score-improvement-2026-05-25-phase-b/candidate-matrix.md`:
  - documents `kill02` as demoted blocker and `waitid07`/`waitid08`/`waitid10` as blocked.
- Local implementation inspection:
  - `examples/shell/src/uspace/process_lifecycle.rs::sys_waitid`, `waitid_siginfo`, `signal_wait_status`.
  - `examples/shell/src/uspace/signal_abi.rs::sys_kill`, `deliver_process_group_signal`.
  - `examples/shell/src/uspace/process_abi.rs::sys_setpgid`, `sys_setsid`.
- Existing `/tmp/ltp-src-snips/{waitid07,waitid08,waitid10,kill02}.c` snippets from the previous lane were read for expected behavior only; no repo files were copied from `/tmp`.

## Stop condition

This report-only lane is complete. No source patch or stable-list edit was made. Promotion remains blocked for `kill02`, `waitid07`, `waitid08`, and `waitid10` until fresh parsed aggregate evidence proves RV+LA × glibc+musl clean behavior.
