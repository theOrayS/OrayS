# Worker 4 process/wait/sched/rlimit/proc lane reconciliation report

Date: 2026-05-25
Lane owner: `worker-3` in team state (`task-4`); earlier static report from `worker-4` was superseded by worker-3 code checkpoints.
Leader reconciliation: yes, because worker-3 produced code checkpoints but did not transition the task before becoming idle.

## Scope and guardrails

- Lane scope: `waitpid01`, scheduler negative-pid, rlimit/RLIMIT_FSIZE, `prctl`, `sethostname`, and procfs-adjacent candidates.
- Did not edit `.omx/ultragoal`.
- Did not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; live list remains 300 cases.
- No worker QEMU evidence is used as promotion evidence. Any worker-side build/QEMU attempt is discovery-only unless rerun serially by the leader.
- No case was promoted to stable from this lane.

## Implemented candidate fix

Worker checkpoints integrated into the leader branch changed:

- `examples/shell/src/uspace/resource_sched.rs`
  - `prlimit_target_valid()` now accepts the current process pid as well as `pid == 0` and current tid.
  - This is intended to match Linux `prlimit64(getpid(), ...)` behavior instead of treating the process id as an unrelated target.
- `examples/shell/src/uspace/signal_abi.rs`
  - Pending default-fatal signals are processed when `rt_sigprocmask` restores/unblocks the mask.
  - Self-directed default-fatal `kill`/`tkill`/`tgkill` paths can terminate the process group synchronously.
  - This targets `waitpid01`/signal wait-status behavior where a child should be observed as killed by the signal rather than normal exit.

These are POSIX/Linux-visible semantic changes and must be verified by targeted wait/signal/rlimit regression before any promotion.

## Evidence read

| Area | Evidence | Decision |
| --- | --- | --- |
| `waitpid01` | Fresh leader blocker batch `raw/blocker-batch-rv.log` before this fix had RV glibc PASS and RV musl failure/TFAIL shape. Worker inspection focused on `raise(sig)`/self-directed signal delivery and default-fatal pending signal handling. | Needs leader-serialized targeted rerun after integrated fix. Not promotable yet. |
| Scheduler negative-pid (`sched_getparam03`, `sched_getscheduler02`, `sched_setparam04`) | Prior evidence kept RV clean but LA musl failing; batch-A RV had `sched_getscheduler02` clean on RV only. | Not promotable until LA musl/glibc is clean. Do not infer LA from RV. |
| `prlimit64` / rlimit | The process-pid target fix is narrow and plausibly correct, but no fresh RV+LA matrix was completed after the checkpoint. | Needs targeted rlimit/prlimit validation; no promotion yet. |
| `prctl`/`sethostname`/procfs adjacent | Existing previous-phase evidence suggested some cases are already clean, but this lane did not produce a fresh full promotion matrix. | Candidate pool only; leader must rerun before adding any case. |

## Verification performed for the lane

Worker-side checks reported through pane/history:

- Parsed `docs/ltp-score-improvement-2026-05-25-phase-a/raw/blocker-batch-rv.log` with `python3 -B scripts/ltp_summary.py`.
- Inspected upstream/current LTP `waitpid01` behavior and local signal/prlimit implementation anchors.
- A worker `make test_build ARCH=riscv64 ...` was attempted in the worker worktree after the code checkpoint.

Leader-side follow-up remains required before promotion:

- `cargo fmt --all -- --check` from the repository root.
- `make A=examples/shell ARCH=riscv64` or closest equivalent build.
- Serialized targeted RV then LA rerun for `waitpid01` and a small guard set including at least a signal stable regression, `getrlimit/setrlimit/prlimit` candidate, and scheduler negative-pid cases.

## Remaining risks and next step

- Signal termination changes are ABI/POSIX-visible and could regress existing stable signal cases if wait-status or pending-mask handling is wrong. Keep them out of stable promotion until targeted and aggregate gates are clean.
- `prlimit64(getpid(), ...)` support is a narrow Linux compatibility improvement, but `setrlimit01` still needs exact subtest evidence around `EFBIG`/`SIGXFSZ` ordering before promotion.
- Scheduler negative-pid cases remain blocked on LA musl until fresh leader evidence proves otherwise.


## Leader reconciliation addendum (2026-05-25)

Worker-3 auto-checkpoint `86325612` was integrated into the leader branch after this static report was created. The task lifecycle did not reach terminal state, so this addendum records the implemented candidate fix and keeps the promotion decision conservative.

Integrated code changes:

- `examples/shell/src/uspace/signal_abi.rs`
  - when `rt_sigprocmask` unblocks a pending default-fatal signal, request the process exit group and synchronously terminate the current thread if an exit group is pending;
  - for self-directed `kill`, `tkill`, and `tgkill`, synchronously honor pending default-fatal signal termination after successful delivery.
- `examples/shell/src/uspace/resource_sched.rs`
  - treat `prlimit64(pid == process.pid())` as a valid current-process target in addition to `pid == 0` and `pid == current_tid()`.

Candidate impact:

| Area | Candidate effect | Promotion decision |
| --- | --- | --- |
| `waitpid01` / signal-default termination | May address musl children that previously reported normal exit instead of signal termination after `raise()`/self-kill paths. | Not promotable until fresh leader-serialized RV and LA targeted evidence is clean for musl+glibc. |
| `prlimit64` current process pid | More Linux-compatible target acceptance for current process pid. | Needs targeted rlimit/prlimit evidence; not a standalone stable addition. |
| scheduler negative-pid LA musl | No direct fix in this addendum. | Still blocked unless fresh LA evidence proves clean. |

Verification gap:

- The implemented source edits were auto-integrated, but no final leader-owned targeted QEMU gate has yet proven them clean.
- This addendum does not convert prior `TFAIL`/`TCONF` evidence into promotion evidence.
- Required next checks are root `cargo fmt --all -- --check`, `make A=examples/shell ARCH=riscv64`, and serialized targeted RV/LA LTP runs parsed with `scripts/ltp_summary.py`.
