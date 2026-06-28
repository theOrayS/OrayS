# Worker 3 Time/Signal lane static report

Scope: Task 3 updated inbox assigned this worker to the Time/Signal lane only. This worker did not read or mutate `.omx/ultragoal`.

## Current lane constraints

- Candidate focus: `clock_gettime03`, `clock_gettime04`, `clock_getres01`, `clock_nanosleep01`, `clock_nanosleep02`, `nanosleep01`, `nanosleep02`, `kill05`, `sigaction02`, `pause01`, `sigprocmask01`, `rt_sigprocmask01`, `sigpending02`, `sigsuspend01`.
- Leader mailbox update at 2026-05-22T00:51Z stopped duplicate worker-side long eval/builds; this report is therefore static analysis plus existing evidence only.
- Timeout must remain a failure signal. `clock_getres01` with TCONF is not a clean promotion candidate.

## Artifacts produced in this worktree

- `docs/ltp-score-improvement-2026-05-22-phase-a/worker3-time-signal-cases.txt` — exact lane case list used for planned targeted classification.
- `docs/ltp-score-improvement-2026-05-22-phase-a/raw/worker3-rv-time-signal-targeted.status` — status `143`, stopped by termination rather than a test result.
- `docs/ltp-score-improvement-2026-05-22-phase-a/raw/worker3-rv-time-signal-targeted.stopped` — explicit stop marker.
- `docs/ltp-score-improvement-2026-05-22-phase-a/worker3-rv-time-signal-targeted-summary.txt` — preserved summary for the aborted attempt showing `PASS LTP CASE: 0`, `FAIL LTP CASE: 0`, timeout `0`, ENOSYS `0`, panic/trap `0`; do not use it as promotion evidence. The raw log is not present in this checkout, so only the status/stop marker and summary are used as evidence.

## Existing evidence incorporated

Historical phase-d and phase-b evidence still classifies several Time/Signal cases as blocked. These are not fresh phase-a promotion results, but they are useful for avoiding speculative promotion:

| Case | Existing RV evidence | Current code surface | Static conclusion |
| --- | --- | --- | --- |
| `clock_gettime03` | phase-d `targeted-batch1-rv-summary.txt`: RV glibc+musl FAIL code 2, TBROK=1 each | `examples/shell/src/uspace/time_abi.rs::sys_clock_gettime`, `clock_now_duration` | Blocked until fresh targeted run proves fixed; likely clock semantics/test expectation issue, not promotable. |
| `clock_gettime04` | phase-d: RV glibc+musl FAIL code 1, TFAIL=6 each | same clock_gettime surface | Blocked; needs semantic investigation before promotion. |
| `clock_getres01` | phase-b: RV glibc+musl PASS wrapper but TCONF=8 each | `sys_clock_getres`, `clock_getres_timespec` | Explicitly not clean; TCONF must stay visible and cannot be promoted as clean. |
| `clock_nanosleep01` | phase-d: RV glibc+musl FAIL 137; timeout=1 each, glibc TFAIL/TCONF present | `sys_clock_nanosleep`, `sys_nanosleep`, `sleep_duration` | Hard blocked by timeout; timeout cannot count as PASS. |
| `clock_nanosleep02` | phase-d: RV glibc+musl FAIL 137; timeout=1 each, TFAIL=2 each | same nanosleep surface | Hard blocked by timeout. |
| `nanosleep01` | phase-b: RV glibc+musl FAIL 137; timeout=1 each | `sys_nanosleep`, `sleep_duration` | Hard blocked by timeout. |
| `nanosleep02` | phase-b: RV glibc+musl fail with internal TFAIL evidence | same nanosleep surface | Blocked; requires fresh classification after any sleep/signal fix. |
| `kill05` | phase-d: RV glibc+musl FAIL code 3; TFAIL=1 and TBROK=1 each | `signal_abi.rs`, `syscall_dispatch.rs` kill/tkill/tgkill paths | Blocked; likely signal delivery/process-state semantics. |
| `sigaction02` | phase-d: RV glibc+musl FAIL code 1; TFAIL=4 each | `sys_rt_sigaction`, signal frame/delivery code | Blocked; do not hardcode case-specific pass. |
| `pause01` | phase-b: RV glibc+musl FAIL 137; timeout=1 each | signal sleep/wakeup path, `consume_expired_real_timer` | Hard blocked by timeout; needs signal wakeup semantics. |
| `sigprocmask01` | phase-b: RV glibc+musl FAIL with TFAIL | `sys_rt_sigprocmask` | Blocked; likely mask ABI/legacy wrapper semantics. |
| `rt_sigprocmask01` | phase-b: RV glibc+musl FAIL wrapper | `sys_rt_sigprocmask` | Blocked; needs fresh targeted details before code change. |
| `sigpending02` | phase-b: RV glibc+musl FAIL code 1; TFAIL=1 each | `sys_rt_sigpending` | Blocked; pending mask semantics currently suspect. |
| `sigsuspend01` | phase-b: RV glibc+musl FAIL code 1; TFAIL=1 and ENOSYS marker each in historical report | `sys_rt_sigsuspend` now dispatches through `syscall_dispatch.rs` | Needs fresh targeted rerun; code now has a dispatcher path, so old ENOSYS may be stale, but promotion still needs clean LA/RV x musl/glibc evidence. |

## Static code observations

- `examples/shell/src/uspace/syscall_dispatch.rs` currently dispatches the relevant clock, nanosleep, signal-mask, sigpending, sigsuspend, kill/tkill/tgkill, and rt_sigaction surfaces, so current blockers are likely semantic rather than missing-dispatch except where historical evidence is stale.
- `examples/shell/src/uspace/time_abi.rs::sleep_duration` busy-yields until wall-time deadline and only checks `pending_exit_group`; it does not provide interruptible sleep semantics for ordinary pending signals. That matches timeout risk for `nanosleep*`, `clock_nanosleep*`, and `pause01`-like behavior.
- `sys_clock_nanosleep` delegates relative sleeps to `sys_nanosleep` after already reading the request. This is semantically harmless for successful sleeps but gives the same non-interruptible behavior and zero-rem behavior as `nanosleep`.
- `sys_rt_sigpending` returns only `ext.pending_signal` as a single-bit mask. If tests create multiple pending signals or expect process/thread pending set behavior, this simplified model can explain `sigpending02` TFAIL.
- `sys_rt_sigsuspend` loops until `current_unblocked_signal_pending()` and returns `-EINTR`, but promotion needs fresh evidence because prior ENOSYS evidence predates the current dispatch implementation.

## Recommended next actions after leader Wave1 result is available

1. Do not promote any Time/Signal case from this lane based on the aborted worker run; it has no LTP executions.
2. If resources permit later, run a narrow fresh targeted batch, not the full lane first:
   - First: `clock_getres01,sigsuspend01,rt_sigprocmask01,sigprocmask01,sigpending02`
   - Then separately timeout-risk cases: `nanosleep01,nanosleep02,clock_nanosleep01,clock_nanosleep02,pause01`
3. For code fixes, prioritize general semantics:
   - interruptible sleep/rem handling for `nanosleep`/`clock_nanosleep`/`pause`, without making timeout PASS;
   - pending signal set accounting for `sigpending02`;
   - signal action/mask ABI details for `sigaction02` and `sigprocmask*`.
4. Keep `clock_getres01` outside clean promotion unless fresh summary shows zero TCONF on LA/RV x musl/glibc.

## Verification evidence for this report

- `cat docs/ltp-score-improvement-2026-05-22-phase-a/worker3-rv-time-signal-targeted-summary.txt` -> preserved summary shows PASS=0, FAIL=0, timeout=0, ENOSYS=0, panic/trap=0 because the run was stopped before LTP execution.
- Static inspected files: `examples/shell/src/uspace/time_abi.rs`, `examples/shell/src/uspace/signal_abi.rs`, `examples/shell/src/uspace/syscall_dispatch.rs`, and historical summaries under `docs/ltp-score-improvement-2026-05-21-phase-b/` and `docs/ltp-score-improvement-2026-05-21-phase-d/`.

## Worker-1 continuation update (2026-05-22)

After the new inbox assignment for Task 3, this worker continued the Time/Signal lane under the leader instruction not to launch more `run-eval`.

### Semantic fix applied

- `examples/shell/src/uspace/signal_abi.rs::sys_rt_sigaction` now rejects attempts to install handlers for uncatchable `SIGKILL` and `SIGSTOP` with `EINVAL`.
- Rationale: Linux signal semantics do not allow changing dispositions for `SIGKILL`/`SIGSTOP`; this is a real ABI fix relevant to `sigaction02`, not a case-name hardcode or fake pass.
- Existing signal-mask code already treats the same two signals as unmaskable via `unmaskable_signal_bits`, so this aligns action-disposition validation with mask semantics.

### Current blocker classification remains honest

- No Time/Signal case is promoted from this worker lane.
- `clock_getres01` remains non-clean while TCONF evidence exists.
- Timeout-risk cases (`clock_nanosleep01`, `clock_nanosleep02`, `nanosleep01`, `pause01`) still require fresh targeted LA/RV x musl/glibc evidence before any promotion decision; timeout must remain separate from PASS.
- `sigpending02`, `sigprocmask01`, `rt_sigprocmask01`, and `sigsuspend01` still require fresh targeted evidence because older failures may be partly stale after prior dispatcher work.

### Additional verification after the continuation

- `rustfmt --check examples/shell/src/uspace/signal_abi.rs` -> PASS.
- `cargo fmt --all -- --check` -> FAIL before formatting because this team worktree's vendored `rust-fatfs` package still points at the leader workspace `/root/oskernel2026-orays/Cargo.toml`; this is a worktree/workspace metadata issue, not a formatting diagnostic for the edited file.
- `git diff --check` -> PASS.

### Delegation evidence

- Attempted 2 read-only native subagents with model `gpt-5.4-mini`: Goodall (`019e4d2f-5df0-7e51-84c8-a43cf59b4cbf`) for signal surfaces failed with model-capacity error; Bohr (`019e4d2f-5fc2-7542-be20-4717dcb2b1d1`) for time surfaces was shut down after timing out because local completion evidence was sufficient.
- Integrated local findings instead of waiting on unavailable child output: dispatcher coverage exists for all task-3 syscall families; the safe local semantic fix is the uncatchable-signal `rt_sigaction` rejection, while sleep/pending-mask blockers need fresh targeted evidence before broader changes.
