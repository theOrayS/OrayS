# Remote LTP score-zero ANSI marker fix

Date: 2026-05-24

## Problem

The user-provided remote evaluator logs `Riscv输出.txt` and `LoongArch输出.txt` showed real LTP execution with passing internal `TPASS` output, but the remote scorer still reported 0 LTP points after the earlier `FAIL LTP CASE <case> : 0` compatibility change.

Key evidence from the submitted logs:

- `PASS LTP CASE`: 0 raw lines.
- `FAIL LTP CASE .* : 0`: 440 raw lines in each architecture log.
- Each successful wrapper marker was prefixed by an ANSI reset, e.g. `\x1b[mFAIL LTP CASE access01 : 0`.
- `scripts/ltp_summary.py` still parsed both logs as 440 semantic PASS / 0 FAIL, so the local parser was tolerant but the remote scorer likely anchored result markers at line start.

The reference branch `refactor/moss_kernel_like_remote` emits clean line-start markers (`FAIL LTP CASE access01 : 0`) because its axlog implementation keeps the newline outside the colored log fragment.

## Fix

Restore the axlog colored-line helper so the ANSI reset terminator is emitted before the newline, not at the beginning of the next console line. This keeps evaluator result markers at the physical line start without changing LTP pass/fail semantics.

This does **not** fake PASS:

- wrapper exit status 0 still emits the compatibility success marker `FAIL LTP CASE <case> : 0`;
- non-zero exits and timeouts still emit real `FAIL LTP CASE`/`TIMEOUT LTP CASE` markers;
- internal `TFAIL`/`TBROK`/`TCONF` output remains visible and parsable.

## Validation

- `python3 -B scripts/test_ltp_summary.py`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- RV smoke: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=access01 LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh rv`
  - `scripts/ltp_summary.py`: 2 PASS / 0 FAIL.
  - marker check: both `FAIL LTP CASE access01 : 0` lines start at column 1 with no ANSI prefix.
- LA smoke: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=access01 LTP_CASE_TIMEOUT_SECS=25 ./run-eval.sh la`
  - `scripts/ltp_summary.py`: 2 PASS / 0 FAIL.
  - marker check: both `FAIL LTP CASE access01 : 0` lines start at column 1 with no ANSI prefix.

Remote scorer result still requires a real remote submission check.
