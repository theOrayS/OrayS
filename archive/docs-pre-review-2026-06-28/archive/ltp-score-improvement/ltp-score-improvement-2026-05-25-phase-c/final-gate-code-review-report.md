# Final gate code-review report — stable382 partial promotion

Date: 2026-05-26
Scope reviewed: stable382 delta from stable381 (`lseek02` promotion plus minimal `mknodat`/FIFO support).
Recommendation: **COMMENT / accept stable382 partial promotion; do not claim stable400+**.

## Reviewed changes

- `examples/shell/src/cmd.rs`: adds `lseek02` to `LTP_STABLE_CASES`.
- `examples/shell/src/uspace/syscall_dispatch.rs`: dispatches `__NR_mknodat`.
- `examples/shell/src/uspace/fd_table.rs`: implements minimal `mknodat`, records FIFO metadata, and opens recorded FIFO paths as pipe-backed non-seekable descriptors.
- `examples/shell/src/uspace/metadata.rs`, `linux_abi.rs`, `mod.rs`, `process_lifecycle.rs`: add `S_IFIFO` metadata plumbing and preserve it across fork-like process cloning.

## Safety review

- No LTP case-name hardcoding was introduced.
- LTP test sources and evaluator scripts were not modified.
- The original `lseek02` blocker was a real `mkfifo()` ENOSYS setup failure; adding `mknodat` is a general syscall compatibility repair.
- FIFO descriptors reuse existing pipe `lseek` behavior, so non-seekable FIFO paths return ESPIPE through the normal FD implementation.
- Unsupported special `mknodat` node types return `EPERM` rather than fake success.
- Regular-file `mknodat` creates a real placeholder file with create-new semantics and records metadata consistently with existing chmod/chown side maps.

## Watchlist / limitations

- FIFO support is intentionally minimal: no full named-FIFO peer registry, blocking open semantics, rename propagation, or cross-process persistent endpoint sharing beyond the current recorded path metadata.
- `path_special_modes` follows the existing side-map style used for path modes/owners; it is removed on successful unlink but not moved on rename. Do not rely on full rename semantics until targeted tests are added.
- Stable450 is not delivered; this report only accepts stable382 evidence.

## Evidence reviewed

- `raw/target-stable400-lseek02-rv-001-summary.txt`: pre-fix failure due to `mkfifo` ENOSYS.
- `raw/target-stable400-lseek02-rv-002-summary.txt`: RV targeted PASS 2 / FAIL 0.
- `raw/target-stable400-lseek02-la-001-summary.txt`: LA targeted PASS 2 / FAIL 0.
- `raw/stable382-rv-gate-001-summary.txt`: RV stable382 PASS 764 / FAIL 0; `ltp-musl` 382/0; `ltp-glibc` 382/0.
- `raw/stable382-la-gate-001-summary.txt`: LA stable382 PASS 764 / FAIL 0; `ltp-musl` 382/0; `ltp-glibc` 382/0.

## Decision

No blocking code-review issue for the stable382 partial promotion. Keep the FIFO limitations explicit in future prompts and do not expand promotion based on FIFO behavior without fresh targeted plus aggregate evidence.
