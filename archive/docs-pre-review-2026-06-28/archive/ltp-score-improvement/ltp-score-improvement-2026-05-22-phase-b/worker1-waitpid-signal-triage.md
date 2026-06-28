# Worker 1 waitpid / signal-status triage

Date: 2026-05-22
Worker: worker-1
Task: 7 (`waitpid and signal-status root cause triage`)

## Scope and guardrails

- Inspected RV Wave A evidence for `waitpid01`, `waitpid04`, `waitpid10`, `waitpid11`, `waitpid12`, and `waitpid13`.
- Touched only real process/signal ABI semantics before the lane was stopped as not promotion-ready.
- Did **not** touch `.omx/ultragoal` or `LTP_STABLE_CASES`.
- The current patch is **not promotion-ready** because latest RV evidence still has `rv:musl:waitpid01` failing and earlier focused evidence still has `waitpid10` / glibc `waitpid11` timeout blockers.

## Evidence summary

### Baseline RV Wave A evidence

Raw log: `docs/ltp-score-improvement-2026-05-22-phase-b/wave-a-targeted-rv.log`

Parsed with `python3 -B scripts/ltp_summary.py`:

- `waitpid01` / musl: FAIL, 40 internal `TFAIL`, all non-`SIGKILL` signal deaths reported as normal exit status `0`.
- `waitpid04` / musl: FAIL, expected `ESRCH` for `waitpid(INT_MIN, NULL, 0)` but saw the wrong wait error before patching.
- `waitpid10`, `waitpid11`, `waitpid12`, `waitpid13` / musl: all failed in the broad Wave A run; later focused evidence showed some of these were improved by process-group wait selection.

### Focused RV evidence before final wait-status fixes

Raw log: `docs/ltp-score-improvement-2026-05-22-phase-b/worker1-waitpid-focused-rv.log`

Parsed summary:

- PASS LTP CASE: 5
- FAIL LTP CASE: 7
- timeout matches: 3
- `waitpid11`, `waitpid12`, `waitpid13` improved to clean PASS in at least one libc/variant set.
- Remaining failures included:
  - `rv:musl:waitpid01` and `rv:glibc:waitpid01` internal `TFAIL`.
  - `rv:musl:waitpid04` and `rv:glibc:waitpid04` internal `TFAIL`.
  - `rv:musl:waitpid10`, `rv:glibc:waitpid10`, and `rv:glibc:waitpid11` timeouts.

### Latest narrow RV proof after final small patch

Command:

```sh
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=waitpid01,waitpid04 \
LTP_CASE_TIMEOUT_SECS=20 \
./run-eval.sh
```

Status file: `docs/ltp-score-improvement-2026-05-22-phase-b/worker1-waitpid01-04-rv-after-core-v2.status`

- Wrapper exit code: `0`
- Parsed summary: `docs/ltp-score-improvement-2026-05-22-phase-b/worker1-waitpid01-04-rv-after-core-v2-summary.txt`
- PASS LTP CASE: 3
- FAIL LTP CASE: 1
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0

Latest case matrix:

| Case | Arch | Libc | Status | TFAIL | timeout | Promotion-safe? |
| --- | --- | --- | --- | ---: | ---: | --- |
| `waitpid01` | rv | glibc | PASS | 0 | 0 | RV glibc only; still needs LA and musl clean |
| `waitpid01` | rv | musl | FAIL | 40 | 0 | No |
| `waitpid04` | rv | glibc | PASS | 0 | 0 | RV only; needs LA confirmation |
| `waitpid04` | rv | musl | PASS | 0 | 0 | RV only; needs LA confirmation |

## Root-cause mapping

### 1. Signal deaths were not always converted to wait-visible signaled status

Relevant source:

- `examples/shell/src/uspace/signal_abi.rs:78-86` — `default_signal_terminates(sig)` table.
- `examples/shell/src/uspace/signal_abi.rs:110-124` — `deliver_user_signal()` only forces immediate process exit for `SIGKILL`; other signals are queued for user-return handling.
- `examples/shell/src/uspace/signal_abi.rs:272-280` and `347-356` — pending default-terminating signals are converted into `request_signal_exit_group(sig)` plus thread termination when injected on user return.

Baseline symptom:

- `waitpid01` reported child PIDs correctly, but non-`SIGKILL` signal deaths had `WIFSIGNALED()` false and looked like normal exit `0`.

Patch direction taken:

- Broadened the default-terminating traditional signal table to cover the Linux default-terminate set in the 1..=31 range, excluding ignored/job-control signals.

Remaining blocker:

- RV glibc `waitpid01` is now clean, but RV musl `waitpid01` still reports 40 `WIFSIGNALED()` failures. That indicates musl's raise/kill path still bypasses or out-races the queued-signal user-return termination path for non-`SIGKILL`; do not promote until the musl path is root-caused and clean.

### 2. Core-dumping signal status bit was missing

Relevant source:

- `examples/shell/src/uspace/process_lifecycle.rs:190-199` — `signal_core_dump_bit(sig)`.
- `examples/shell/src/uspace/process_lifecycle.rs:642-648` — `wait_status()` now ORs bit `0x80` for Linux core-default signals.

Baseline/focused symptom:

- After default signal termination improved glibc `WIFSIGNALED`, glibc still failed checks such as “Child did not dump core when expected”.

Patch direction taken:

- Encode Linux wait-status core bit `0x80` for default core signals (`SIGQUIT`, `SIGILL`, `SIGTRAP`, `SIGABRT`, `SIGBUS`, `SIGFPE`, `SIGSEGV`, `SIGXCPU`, `SIGXFSZ`, `SIGSYS`).

Latest evidence:

- RV glibc `waitpid01` passes after this change.

### 3. `waitpid()` pid selector semantics were incomplete

Relevant source:

- `examples/shell/src/uspace/process_lifecycle.rs:565-584` — pid selector matching for `pid == -1`, `pid == 0`, `pid > 0`, and `pid < -1`.
- `examples/shell/src/uspace/process_lifecycle.rs:589-600` — `ECHILD` vs `Ok(None)` behavior for no matching child vs matching non-exited child under `WNOHANG`.

Baseline symptoms:

- `waitpid04` expected `ESRCH` for `waitpid(INT_MIN, NULL, 0)` but did not receive it.
- `waitpid10`/`waitpid11`/`waitpid12`/`waitpid13` failures were consistent with missing `pid == 0` / `pid < -1` process-group selection and `WNOHANG` mismatch handling.

Patch direction taken:

- Implemented Linux-compatible pid matching for current process group (`pid == 0`) and target process group (`pid < -1`).
- Used `checked_neg()` so `INT_MIN` produces `ESRCH` without overflow.

Latest evidence:

- RV musl and glibc `waitpid04` now both pass in the narrow proof.
- Earlier focused evidence showed `waitpid12`/`waitpid13` clean and `waitpid11` partially clean, but `waitpid10` and glibc `waitpid11` still had timeout blockers; they remain not promotion-safe.

## Exact changed files / functions

Committed by team auto-checkpoint commits before this report:

- `examples/shell/src/uspace/signal_abi.rs`
  - `default_signal_terminates()`
  - `inject_pending_signal()` paths for default signal handling on RV and LA
  - import list cleanup around signal constants
- `examples/shell/src/uspace/process_lifecycle.rs`
  - new `signal_core_dump_bit()` helper
  - `UserProcess::wait_child()` pid selection / `WNOHANG` behavior
  - `UserProcess::wait_status()` wait-status encoding
- `examples/shell/src/uspace/linux_abi.rs`
  - removed unused signal constants while keeping `SIGALRM_NUM`, which is still used by timer code

## Verification

- `rustfmt --check examples/shell/src/uspace/signal_abi.rs examples/shell/src/uspace/process_lifecycle.rs examples/shell/src/uspace/linux_abi.rs` → PASS.
- `make A=examples/shell ARCH=riscv64` → PASS; this repo make target built RV and LA kernels successfully, with log at `docs/ltp-score-improvement-2026-05-22-phase-b/worker1-rv-build-after-waitpid-patch.log`.
- `python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-b/worker1-waitpid01-04-rv-after-core-v2.log` → parsed 3 PASS / 1 FAIL / 0 timeout / 0 ENOSYS / 0 panic-trap.

## Promotion-safe / blocked cases

Promotion-safe from this lane: **none**.

Blocked / not promotion-safe:

- `waitpid01`: blocked by RV musl 40 `TFAIL` in latest proof.
- `waitpid04`: clean on RV musl/glibc latest proof, but still needs LA musl/glibc confirmation before any promotion discussion.
- `waitpid10`: blocked by earlier focused RV musl/glibc timeouts.
- `waitpid11`: blocked by earlier focused RV glibc timeout; musl passed in that run.
- `waitpid12`: RV musl/glibc passed in the focused run, but needs LA confirmation and should not be promoted independently while adjacent waitpid process-group/timeouts remain unresolved.
- `waitpid13`: RV musl/glibc passed in the focused run, but needs LA confirmation and should not be promoted independently while adjacent waitpid process-group/timeouts remain unresolved.

## Recommended next step if this lane is resumed

Do not broaden the waitpid patch until scheduler/resource lanes finish. If resumed, isolate RV musl `waitpid01` first: compare musl's `raise()`/`kill(getpid(), sig)` syscall path against glibc using signal-delivery tracing or a tiny non-LTP reproducer, then prove `waitpid01` on RV musl/glibc before touching LA or adjacent waitpid timeout cases.
