# la-arch004 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch004.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **False**
- run-eval status: `None`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=407 (1961 cases, timeout 15s)`
  - line 39848: `ltp case list: all-minus-blacklist skipped=410 (1965 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 617
- Parser FAIL LTP CASE count: 1469
- TIMEOUT matches: 27
- Raw RUN markers: 2087
- Raw FAIL markers: 2086
- Raw TIMEOUT markers: 27
- Raw SKIP markers: 0
- Incomplete RUN count: 1

## Internal LTP signals

- TBROK: 586
- TCONF: 1034
- TFAIL: 1962
- ENOSYS/not implemented matches: 647
- Parser panic/trap matches: 0
- Strict panic count: 0
- First strict panic: `None`
- Strict resource failure count: 61
- First strict resource failure: `{'line': 40228, 'case': 'access02', 'text': 'sh: fork: Resource temporarily unavailable'}`

## Suite summaries

- ltp-musl: passed=604 failed=1357 timed_out=27
