# Final evidence: LTP core fixes 2026-05-20

- LA full run: `./run-eval.sh la 2>&1 | tee output_la.md`, exit 0.
  - `ltp-musl`: 16 passed, 0 failed.
  - `ltp-glibc`: 16 passed, 0 failed.
  - `PASS LTP CASE=32`, `FAIL LTP CASE=0`, `TFAIL/TBROK/TCONF=2 ({TCONF: 2})`, `ENOSYS/not implemented=0`.
- RV full run: `./run-eval.sh 2>&1 | tee output_rv.md`, exit 0.
  - `ltp-musl`: 16 passed, 0 failed.
  - `ltp-glibc`: 16 passed, 0 failed.
  - `PASS LTP CASE=32`, `FAIL LTP CASE=0`, `TFAIL/TBROK/TCONF=2 ({TCONF: 2})`, `ENOSYS/not implemented=0`.
- Temporary staging-only LTP filtering was removed before final full runs; `examples/shell/src/cmd.rs` contains 16 `LTP_CORE_CASES` and no `if group != "ltp"` gate.
