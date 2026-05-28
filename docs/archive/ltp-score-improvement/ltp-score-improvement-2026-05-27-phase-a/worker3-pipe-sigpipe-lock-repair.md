# Worker 3 pipe SIGPIPE lock-order repair

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Worker executing task: `worker-1`
Task: `14` — narrow pipe SIGPIPE lock-order repair.

## Scope and guardrails

- Source patch only: no QEMU/evaluator run.
- Did **not** edit `.omx/ultragoal`.
- Did **not** edit `examples/shell/src/cmd.rs` / `LTP_STABLE_CASES`.
- No LTP case-name hardcoding.
- Fix is intentionally narrow to the pipe closed-peer/SIGPIPE path.

## Root cause being repaired

Task-11 diagnosed the `pipe02` panic as a lock-order bug:

1. `sys_write` enters `process.fds.lock()`.
2. `FdTable::write` dispatches `FdEntry::Pipe(pipe)` while that fd-table lock is still held.
3. `PipeEndpoint::write` detects that the write end has no readers.
4. The old `PipeEndpoint::raise_sigpipe` delivered SIGPIPE and synchronously called
   `terminate_current_thread_for_exit_group` for the default-unblocked SIGPIPE case.
5. Process teardown closes file descriptors and re-enters `process.fds.lock()`, causing the
   axsync self-acquire assertion.

## Patch

Changed `examples/shell/src/uspace/fd_pipe.rs` only:

- Removed the synchronous `terminate_current_thread_for_exit_group` path from
  `PipeEndpoint::raise_sigpipe`.
- Kept `deliver_user_signal(SIGPIPE)` so Linux-visible signal behavior still flows through the
  common signal machinery.
- Relied on existing `signal_abi::user_return_hook` to observe the pending default-fatal
  exit-group request after the syscall path unwinds and releases the fd-table lock.
- Kept `PipeEndpoint::write` returning `EPIPE` for ignored/handled/blocked SIGPIPE, matching
  normal `write(2)` closed-pipe behavior.

Relevant paths:

- `examples/shell/src/uspace/fd_table.rs`: `sys_write`/`FdTable::write` still hold the fd-table
  mutex while dispatching to `PipeEndpoint::write`.
- `examples/shell/src/uspace/fd_pipe.rs`: pipe write closed-peer path now only delivers SIGPIPE.
- `examples/shell/src/uspace/signal_abi.rs`: `deliver_user_signal` requests exit-group for
  default-fatal unblocked signals; `user_return_hook` performs termination before returning to
  user space.
- `examples/shell/src/uspace/process_lifecycle.rs`: teardown may close fds, so it must not run
  while the fd-table mutex is already held by `sys_write`.

## POSIX-visible behavior and risk

Expected POSIX-visible behavior remains:

- Default, unblocked SIGPIPE: process is terminated before returning to user mode.
- Ignored SIGPIPE: `write(2)` returns `-EPIPE`.
- Blocked SIGPIPE: SIGPIPE remains pending and `write(2)` returns `-EPIPE`.
- User handler for SIGPIPE: signal is delivered through the normal signal-frame path and the
  syscall result remains `-EPIPE`.

Risk: this changes the internal timing of default SIGPIPE termination from inside
`PipeEndpoint::write` to the user-return hook. That is deliberate to avoid fd-table self-deadlock,
but traces may show termination slightly later in the syscall return path. This patch does not solve
all fd-table-held blocking I/O risks; it only removes the closed-peer SIGPIPE teardown re-entry.

## Native subagent evidence

Subagent spawn evidence: 1, change-slice probe `019e691a-527a-7e22-ad11-4f3393fe9b2a`/Linnaeus; findings integrated: relevant path is `fd_pipe.rs::raise_sigpipe`, safest slice is to keep `deliver_user_signal(SIGPIPE)` and defer default-fatal termination to `signal_abi::user_return_hook`, broader pipe-only fd-table unlock was higher risk.

Serial searches before spawn: 0.
Subagent model: `gpt-5.4-mini`.

## Verification

PASS — formatting/lint target for modified Rust file:

```bash
cargo fmt -p arceos-shell -- --check
```

Result: exit 0.

PASS — target build/typecheck for affected kernel app without running QEMU:

```bash
make kernel-rv
```

Result: exit 0; finished release build and produced `kernel-rv`.

PASS — targeted clippy/linter for the affected app/target:

```bash
AX_CONFIG_PATH="$PWD/build/kernels/riscv64.axconfig.toml" \
cargo -Z unstable-options -C examples/shell clippy \
  --target riscv64gc-unknown-none-elf \
  --target-dir "$PWD/build/kernels/target/riscv64" \
  --release \
  --features "axstd/defplat axstd/log-level-info axstd/bus-mmio axstd/alloc axstd/paging axstd/irq axstd/multitask axstd/fs axstd/net auto-run-tests uspace" \
  -- -A clippy::new_without_default -A unsafe_op_in_unsafe_fn
```

Result: exit 0. The command reports pre-existing clippy warnings in unrelated files/crates; none are
from the modified `fd_pipe.rs` line slice.

PASS — patch hygiene:

```bash
git diff --check
```

Result: exit 0.

FAIL / not applicable by task constraint — end-to-end QEMU/evaluator:

- Not run because task explicitly forbids default QEMU/evaluator.
- The highest available non-QEMU evidence is successful RV kernel build plus targeted clippy.

FAIL / not applicable — focused Rust unit test suite:

- No focused unit tests exist for `examples/shell/src/uspace/fd_pipe.rs` in this repository.
- Host `cargo check -p arceos-shell --features uspace --no-default-features` was attempted as an
  additional diagnostic and failed on pre-existing host/architecture configuration errors unrelated
  to this patch (`AUX_PLATFORM` cfg, host `TrapFrame` fields, and `WaitQueue::wait_timeout_until`).
  The target build above is the valid check for this app.
