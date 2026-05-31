# la-arch009 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch009.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **False**
- run-eval status: `143`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=413 (1955 cases, timeout 15s)`
  - line 39721: `ltp case list: all-minus-blacklist skipped=416 (1959 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 760
- Parser FAIL LTP CASE count: 1687
- TIMEOUT matches: 32
- Raw RUN markers: 2449
- Raw FAIL markers: 2448
- Raw TIMEOUT markers: 32
- Raw SKIP markers: 0
- Incomplete RUN count: 1

## Internal LTP signals

- TBROK: 660
- TCONF: 1158
- TFAIL: 3084
- ENOSYS/not implemented matches: 695
- Parser panic/trap matches: 0
- Strict panic count: 0
- First strict panic: `None`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

- ltp-musl: passed=603 failed=1352 timed_out=26
