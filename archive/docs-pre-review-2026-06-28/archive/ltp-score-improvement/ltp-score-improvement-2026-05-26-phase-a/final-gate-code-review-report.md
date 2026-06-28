# Final gate code review report

Date: 2026-05-26
Scope: source diff for stable413 promotion and final gate evidence.
Verdict: APPROVE after blocker fixes.

## Review inputs

- Source diff: `examples/shell/src/cmd.rs`, `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/syscall_dispatch.rs`.
- Final gate evidence: `raw/stable413-rv-final-gate-002-summary.txt` and `raw/stable413-la-final-gate-002-summary.txt`.
- Quality gate: `final-gate-quality-gate.json`.

## Initial blocking findings and fixes

1. `sendfile(offset_ptr != NULL)` could perform FD I/O before proving the output offset pointer was writable.
   - Fix: validate `offset_ptr` with `validate_user_write(..., size_of::<i64>())` before any FD I/O, then still use `write_user_value` defensively at copy-out.
2. `sendfile(offset_ptr == NULL)` could over-advance the input file offset when the output write was partial.
   - Fix: read the current input fd offset without committing it, then advance the input fd only by the actual bytes written.

Incremental review after these changes returned `APPROVE` and found no new commit-blocking issues.

## Guardrail review

- No promoted LTP case name appears in the new syscall implementation paths.
- No LTP runner marker or parser semantics were changed.
- No LTP test source was changed.
- The stable list edit contains exactly the 30 promoted cases documented in `stable413-delivery-report.md`.
- The implementation is general syscall behavior: negative offset errno handling, O_APPEND positioned writes, `sendfile`, and `preadv2`/`pwritev2` dispatch.

## Evidence after fixes

- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- `make A=examples/shell ARCH=riscv64`: PASS.
- `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`: PASS.
- RV final stable413 gate: PASS LTP CASE 826, FAIL 0; `ltp-musl` 413/0; `ltp-glibc` 413/0.
- LA final stable413 gate: PASS LTP CASE 826, FAIL 0; `ltp-musl` 413/0; `ltp-glibc` 413/0.
- Known transparent internal TCONF remains `read02` only; timeout/ENOSYS/panic/trap remain 0.

## Non-blocking follow-up risks

- Hidden `sendfile` paths may still need more explicit FD capability errno mapping beyond current regular-file evidence.
- Positioned write limit behavior beyond the promoted cases should be hardened before broader `pwrite*`/`sendfile*` expansion.
- Non-v2 `preadv`/`pwritev` high-offset split semantics should be revisited before using them as hidden-test defense.

## Verdict

No remaining blocker for stable413 delivery. The code changes are narrow, behavior-backed, and final gates were re-run after the review fixes.
