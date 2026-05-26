# AI-slop cleaner audit — stable382 partial promotion

Date: 2026-05-26
Scope: stable382 delta (`lseek02` and minimal `mknodat`/FIFO support).
Status: **passed with watchlist**.

## Cleanup checks

- No new dependencies.
- No broad refactor or repository-wide formatting churn.
- No case-name-specific branch or fake LTP success path.
- The new code reuses existing helpers: `resolve_dirfd_path`, metadata side maps, `OpenOptions::create_new`, `PipeEndpoint`, `FdEntry::Pipe`, and existing errno conversion.
- The stable list change is a single explicit addition after fresh targeted and aggregate evidence.

## Behavior boundary

The change intentionally adds a small POSIX-visible capability: `mknodat()` now supports regular files and FIFOs enough for `mkfifo()` setup and FIFO non-seekability checks. It does not pretend to implement complete named-FIFO semantics. Unsupported special nodes fail explicitly.

## Remaining risks

- Full named FIFO blocking/peer semantics remain unimplemented.
- FIFO side metadata is not rename-aware, matching the broader existing side-map limitation.
- LA aggregate logs still contain two inherited raw LTP internal `Test timeouted` notices even though parser timeout matches are 0; continue to disclose them.

## Evidence

- `cargo fmt --all -- --check` passed.
- `make A=examples/shell ARCH=riscv64` passed.
- RV/LA targeted `lseek02` passed after the fix.
- RV/LA stable382 aggregate gates passed by `scripts/ltp_summary.py`.
