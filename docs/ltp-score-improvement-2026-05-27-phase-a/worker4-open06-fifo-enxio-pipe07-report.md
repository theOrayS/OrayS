# Worker 4 task 7 — open06 FIFO ENXIO narrow repair and pipe07 scout

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Worker: `worker-4`
Scope: FD/FIFO/pipe follow-up only; no `.omx/ultragoal` or `LTP_STABLE_CASES` edits; no default QEMU/evaluator run.

## Outcome

Implemented the narrow Linux-compatible FIFO behavior needed by LTP `open06`: a named FIFO opened with `O_NONBLOCK | O_WRONLY` now returns `ENXIO` in the current compatibility layer, which has no named-FIFO reader rendezvous table.

The change is intentionally conservative: it affects only named FIFO opens with write-only + nonblocking flags. Regular files, anonymous pipes from `pipe()`/`pipe2()`, FIFO `O_PATH`, FIFO `O_RDONLY`, and Linux-specific FIFO `O_RDWR` paths remain on their previous paths.

## Source evidence

### `open06`

Inspected upstream LTP source at `/tmp/ltp-src-worker4/testcases/kernel/syscalls/open/open06.c`:

- Lines 8-11: verifies `open(2)` fails with `ENXIO` when `O_NONBLOCK | O_WRONLY` is used on a FIFO and no process has it open for reading.
- Line 25: `TST_EXP_FAIL2(open(TEMP_FIFO, O_NONBLOCK | O_WRONLY), ENXIO);`

### Local FIFO open path

`examples/shell/src/uspace/fd_table.rs` records FIFO special mode via `mknodat()` and handles named FIFO opens in `open_candidates()`.

Before the repair, the FIFO branch always created a private `PipeEndpoint::new_pair(status_flags)` and returned the write end for `O_WRONLY`, so `open06` saw success rather than `ENXIO`.

After the repair, the FIFO branch checks after permission validation and before path-only / endpoint creation:

```rust
if flags & general::O_ACCMODE == general::O_WRONLY && flags & general::O_NONBLOCK != 0 {
    // This compatibility layer does not keep a rendezvous table
    // for named FIFO opens.  A nonblocking writer therefore has
    // no observable reader and must fail like Linux open(2).
    return Err(LinuxError::ENXIO);
}
```

## Files changed

- `examples/shell/src/uspace/fd_table.rs`
  - Behavioral change introduced in local code commit `0b74f791`: return `LinuxError::ENXIO` for named FIFO `O_WRONLY | O_NONBLOCK` when no reader can be observed.
  - Follow-up formatting commit in this report lane also runs `rustfmt` on this file, causing import-order-only cleanup.
- `docs/ltp-score-improvement-2026-05-27-phase-a/worker4-open06-fifo-enxio-pipe07-report.md`
  - This report.

## Risk analysis

### Low-risk aspects

- The guard is restricted to named FIFO opens where `(flags & O_ACCMODE) == O_WRONLY` and `O_NONBLOCK` is present.
- It does not touch `pipe()`, `pipe2()`, anonymous pipe capacity, pipe status flags, `FIONREAD`, `SIGPIPE`, or iovec logic.
- It preserves FIFO `O_RDWR`, which is used by other LTP/fcntl-style tests to avoid blocking while still requiring non-seekable `ESPIPE` semantics.
- It runs after existing `O_CREAT|O_EXCL` and permission checks, so existing negative-path ordering for those cases is preserved.

### Known semantic limitation

The compatibility layer still does not maintain a named FIFO rendezvous table. Therefore, a future workload that opens a FIFO for reading and then opens the same named FIFO with `O_WRONLY | O_NONBLOCK` may still need a broader design to observe the reader. This task deliberately avoids that broader state model because the current implementation already creates private pipe endpoints per FIFO open and cannot accurately connect named FIFO peers.

For the current `open06` case, the absence of a reader is exactly the tested condition, so returning `ENXIO` is more Linux-compatible than creating a fake private writer endpoint.

## `pipe07` scout

Inspected `/tmp/ltp-src-worker4/testcases/kernel/syscalls/pipe/pipe07.c`:

- Lines 20-48: records currently open fds by iterating `/proc/self/fd` and skipping the directory fd.
- Lines 51-60: uses `getdtablesize()` and calculates expected fd slots available for pipes.
- Lines 67-76: repeatedly calls `pipe(fds)` until failure, expects final `errno == EMFILE`, and checks `exp_num_pipes == num_pipe_fds`.

Local support indicators:

- `examples/shell/src/uspace/fd_table.rs` has `FD_TABLE_LIMIT = DEFAULT_NOFILE_LIMIT` and `insert_min_with_flags()` returns `EMFILE` when the table is full.
- FIFO repair does not affect anonymous pipe allocation.
- `/proc/self/fd` availability/counting and `getdtablesize()`/`RLIMIT_NOFILE` consistency remain the main promotion risk for `pipe07`; it should be leader-gated with RV+LA parser output before adding to stable.

Recommendation: `pipe07` is a plausible low-risk FD candidate after this repair, but not worker-promoted here because it depends on `/proc/self/fd` enumeration and fd-limit accounting under real LTP harness conditions.

## Verification

Commands run from worker worktree:

| Check | Result |
| --- | --- |
| `python3 -B scripts/test_ltp_summary.py` | PASS — `Ran 10 tests ... OK` |
| live stable parser for `examples/shell/src/cmd.rs::LTP_STABLE_CASES` | PASS — `413 total / 413 unique / 0 duplicates` |
| `git diff --check` | PASS |
| `rustfmt --edition 2021 --check examples/shell/src/uspace/fd_table.rs` | PASS after single-file rustfmt import-order cleanup |
| `timeout 300s cargo check --manifest-path examples/shell/Cargo.toml --target riscv64gc-unknown-none-elf --features 'uspace auto-run-tests'` | BLOCKED/invalid for this target shape — failed with `can't find crate for axplat_riscv64_qemu_virt` |
| `timeout 300s cargo check --manifest-path examples/shell/Cargo.toml --target riscv64gc-unknown-none-elf --features 'uspace auto-run-tests axhal/defplat'` | PASS — `Finished dev profile ... target(s) in 3m 15s` |

No default QEMU or evaluator was run per leader instruction.
