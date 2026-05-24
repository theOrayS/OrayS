# remote-marker-regression-check

Status: report-only guardrail check completed by worker-5 on 2026-05-24.

## Scope

This check is intentionally narrow: it verifies the marker/parser guardrails that
must stay true while worker lanes search for new stable-promotion candidates. It
does **not** promote cases by itself and does **not** edit `LTP_STABLE_CASES`.

## Findings

- Parser semantics are guarded by `scripts/ltp_summary.py`: ANSI is stripped
  before matching, wrapper status is derived from the numeric status code, and
  timeout evidence can override a prior zero-status result so timed-out cases do
  not remain classified as clean PASS.
- `scripts/test_ltp_summary.py` covers the required anti-fake-pass cases:
  zero-status `PASS LTP CASE`, legacy zero-status `FAIL LTP CASE`, non-zero
  `PASS LTP CASE`, and timeout-after-zero-status.
- Current source inspection in this worktree shows successful direct LTP cases
  are emitted as legacy-compatible `FAIL LTP CASE <case> : 0`, while non-zero
  exits and timeout exits remain `FAIL LTP CASE <case> : <status>` plus
  `TIMEOUT LTP CASE ...` where applicable. This worker did not change that
  producer contract.
- `kernel/diagnostics/axlog/src/lib.rs` keeps the newline outside the ANSI reset
  fragment to avoid leaking a reset escape onto the next marker line. That is
  the source-level guard for remote parsers that require marker text at line
  start.
- The root `output_la.md` / `output_rv.md` files parsed successfully, but they
  are not fresh stable250+ promotion gates: both summarize a 63-case-per-libc
  snapshot. Treat them as parser/guardrail smoke evidence only.
- Those root outputs still contain ANSI-prefixed `PASS LTP CASE read02 : 0`
  lines. Because they are not from a fresh build/run of this worktree, they are
  recorded as stale-output risk evidence rather than proof of a current source
  regression.

## Current guardrail smoke evidence

| Check | Command | Result |
| --- | --- | --- |
| Parser anti-fake-pass tests | `python3 -m unittest scripts.test_ltp_summary -v` | PASS: 4 tests OK |
| LA root-output parser smoke | `python3 -B scripts/ltp_summary.py output_la.md > docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker5-output-la-summary.txt` | PASS: parsed; 126 wrapper PASS, 0 wrapper FAIL, TCONF=4, ENOSYS=0, panic/trap=0 |
| RV root-output parser smoke | `python3 -B scripts/ltp_summary.py output_rv.md > docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker5-output-rv-summary.txt` | PASS: parsed; 126 wrapper PASS, 0 wrapper FAIL, TCONF=4, ENOSYS=0, panic/trap=0 |
| Promotion-candidate parser smoke | `python3 -B scripts/ltp_summary.py --promotion-candidates output_rv.md output_la.md > docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker5-current-promotion-candidates.txt` | PASS: report generated; `read02` remains blocked/incomplete due TCONF |
| Source marker contract scan | Python source scan over `examples/shell/src/cmd.rs` | PASS for timeout/non-zero failure markers; NOTE: success token is current legacy `FAIL ... : 0`, not `PASS ... : 0` |

## Required reviewer interpretation

- Do not call a run clean from wrapper counts alone. Always inspect the parser
  categories, especially `pass_with_tconf`, `timeout`, `enosys`, and
  `panic_trap`.
- Do not hide `read02`: it remains a known transparent `pass_with_tconf` case
  in the parser smoke summaries and must not be counted as a clean promotion
  candidate.
- Do not count timeout as PASS: parser tests and source paths preserve timeout
  as separate failure evidence.
- Do not use stale root outputs as remote-marker proof. A final promotion gate
  still needs fresh LA/RV output generated from the current tree and checked for
  marker line-start cleanliness.

## Recommended final-gate marker check

For fresh final-gate logs, run:

```bash
python3 -B scripts/ltp_summary.py <fresh-la-log>
python3 -B scripts/ltp_summary.py <fresh-rv-log>
grep -nE '^(PASS|FAIL) LTP CASE' <fresh-log>
grep -nP '^\x1b\[[0-9;]*m(PASS|FAIL) LTP CASE' <fresh-log> && echo 'FAIL: ANSI-prefixed marker found'
```

Expected result: parser summaries show no real `TFAIL`/`TBROK`, no timeout,
no ENOSYS, no panic/trap, and any `TCONF` caveat is explicitly named rather
than folded into clean PASS.
