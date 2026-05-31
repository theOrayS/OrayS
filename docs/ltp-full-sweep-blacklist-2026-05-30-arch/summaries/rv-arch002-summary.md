# rv-arch002 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **True**
- run-eval status: `0`
- Selection lines:
  - line 989: `ltp case list: all-minus-blacklist skipped=41 (2327 cases, timeout 15s)`
  - line 43762: `ltp case list: all-minus-blacklist skipped=44 (2331 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 1204
- Parser FAIL LTP CASE count: 3453
- TIMEOUT matches: 55
- Raw RUN markers: 4658
- Raw FAIL markers: 4658
- Raw TIMEOUT markers: 55
- Raw SKIP markers: 0
- Incomplete RUN count: 0

## Internal LTP signals

- TBROK: 1043
- TCONF: 2663
- TFAIL: 4058
- ENOSYS/not implemented matches: 1280
- Parser panic/trap matches: 0
- Strict panic count: 0
- First strict panic: `None`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

- ltp-musl: passed=598 failed=1729 timed_out=27
- ltp-glibc: passed=606 failed=1725 timed_out=28
