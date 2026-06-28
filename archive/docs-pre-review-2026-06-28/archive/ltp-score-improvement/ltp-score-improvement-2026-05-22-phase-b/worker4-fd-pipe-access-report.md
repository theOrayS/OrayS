# Worker 4 fd/pipe/access near-clean report

Date: 2026-05-22
Worker: `worker-4`
Task: `10` / `fd-pipe-access near-clean source fix lane`

## Scope

Allowed source scope was limited to:

- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/fd_pipe.rs`
- `examples/shell/src/uspace/metadata.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`
- this report

This lane did not edit `LTP_STABLE_CASES`, runner PASS/SKIP logic, `.omx/ultragoal`, or process/signal/resource files.

## Changes

### `faccessat(2)` flags and empty-path handling

File: `examples/shell/src/uspace/metadata.rs`

- `sys_faccessat` now rejects unsupported flags with `EINVAL` instead of silently ignoring them.
- Supported flags are `AT_EACCESS`, `AT_SYMLINK_NOFOLLOW`, and `AT_EMPTY_PATH`.
- Empty path now requires `AT_EMPTY_PATH`; otherwise it returns `ENOENT`.
- Empty path with an fd now checks the referenced fd via `stat_with_recorded_path` instead of trying to resolve an empty pathname through normal path lookup.

Candidate impact: `faccessat01`, `access02`, `access04`, and `openat01` adjacent errno/flag checks.

### `lseek(SEEK_SET)` negative offset

File: `examples/shell/src/uspace/fd_table.rs`

- `SEEK_SET` with a negative offset now returns `EINVAL`.
- Previous behavior clamped negative `SEEK_SET` offsets to zero, which can hide an expected errno path.

Candidate impact: `lseek02`.

### `pipe2(2)` output-pointer validation before fd allocation

File: `examples/shell/src/uspace/fd_pipe.rs`

- `sys_pipe2` validates the user output buffer for `[i32; 2]` before allocating pipe endpoints/fds.
- This preserves expected `EFAULT` behavior and avoids creating hidden fds when the output pointer is invalid.

Candidate impact: `pipe02`, `pipe04`, `pipe05`.

## Investigation notes

- `dup` / `dup3` / `fcntl(F_DUPFD*)` paths already reject invalid flags, invalid same-fd `dup3`, invalid negative minimum fd, and preserve `FD_CLOEXEC` where applicable in the audited code.
- No minimal safe change was made for `dup03`, `dup05`, `dup201`, or `dup202` without fresh LTP failure logs showing a concrete errno mismatch.
- No `syscall_dispatch.rs` change was needed; dispatch already routes `openat`, `faccessat`, `pipe2`, `lseek`, `dup`, `dup3`, and `fcntl` to the scoped implementations.

## Verification

Commands run:

```text
rustfmt --edition 2024 examples/shell/src/uspace/fd_pipe.rs examples/shell/src/uspace/fd_table.rs examples/shell/src/uspace/metadata.rs
git diff --check
cargo check -p arceos-shell --features "uspace auto-run-tests"
cargo check -p arceos-shell --target riscv64gc-unknown-none-elf --features "uspace auto-run-tests"
```

Results:

- `rustfmt ...` -> PASS, exit code 0.
- `git diff --check` -> PASS, exit code 0.
- Host `cargo check -p arceos-shell --features "uspace auto-run-tests"` -> FAIL, exit code 101, due to pre-existing host-architecture uspace build errors outside this task scope:
  - missing `AUX_PLATFORM` on host `x86_64`
  - unresolved `terminate_current_thread*` imports in `signal_abi.rs`
  - `TrapFrame.regs` field mismatches in process/task/memory code
  - `make_uspace_context` returning `()` on the host path
- Targeted RISC-V `cargo check -p arceos-shell --target riscv64gc-unknown-none-elf --features "uspace auto-run-tests"` -> FAIL, exit code 101, due to missing platform crate `axplat_riscv64_qemu_virt` from the current cargo invocation. This is a build-configuration/platform selection issue before the changed fd/pipe/access modules are checked, not a scoped source diagnostic.


## Delegation notes

Task 10 required a parallel probe unless skipped. I attempted four native subagent probe launches after the inbox reminder:

- `019e4de4-67f1-7540-8817-ef55513659ba` / root-cause probe: errored on model usage limit before findings.
- `019e4de4-6b88-79e2-b7a6-d2107409eebf` / repository map probe: completed without a final text payload, so no findings were available to integrate.
- `019e4de4-7089-7670-a34f-e6952c912db2` and replacement `019e4de4-fe5e-7ae3-b820-95180f6d1625` / change review probes: timed out while task verification was already sufficient; both were shut down to avoid leaving stale agents.

Integrated finding: no child finding changed the scoped patch. Final result relies on direct source review plus fmt/diff/check evidence above.

## Follow-up gate

These source fixes should still be proven with targeted LTP logs before any stable promotion:

```text
OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:<fd-pipe-access-cases.txt> LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh
python3 scripts/ltp_summary.py --promotion-candidates <rv-log> <la-log>
```

Do not add these cases to `LTP_STABLE_CASES` unless LA/RV x musl/glibc evidence is clean and timeout/TFAIL/TBROK/ENOSYS/panic-trap remain separated from PASS.
