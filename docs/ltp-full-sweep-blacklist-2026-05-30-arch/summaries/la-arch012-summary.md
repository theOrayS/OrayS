# la-arch012 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **True**
- run-eval status: `0`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=416 (1952 cases, timeout 15s)`
  - line 39534: `ltp case list: all-minus-blacklist skipped=419 (1956 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 1207
- Parser FAIL LTP CASE count: 2698
- TIMEOUT matches: 53
- Raw RUN markers: 3908
- Raw FAIL markers: 3908
- Raw TIMEOUT markers: 53
- Raw SKIP markers: 0
- Incomplete RUN count: 0

## Internal LTP signals

- TBROK: 1031
- TCONF: 1936
- TFAIL: 4041
- ENOSYS/not implemented matches: 1279
- Parser panic/trap matches: 0
- Strict panic count: 0
- First strict panic: `None`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

- ltp-musl: passed=602 failed=1350 timed_out=25
- ltp-glibc: passed=605 failed=1351 timed_out=28
