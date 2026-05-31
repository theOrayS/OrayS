# la-arch002 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch002.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **False**
- run-eval status: `0`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=405 (1963 cases, timeout 15s)`
  - line 39885: `ltp case list: all-minus-blacklist skipped=408 (1967 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 752
- Parser FAIL LTP CASE count: 1687
- TIMEOUT matches: 33
- Raw RUN markers: 2441
- Raw FAIL markers: 2439
- Raw TIMEOUT markers: 33
- Raw SKIP markers: 0
- Incomplete RUN count: 2

## Internal LTP signals

- TBROK: 659
- TCONF: 1157
- TFAIL: 3090
- ENOSYS/not implemented matches: 695
- Parser panic/trap matches: 1
- Strict panic count: 2
- First strict panic: `{'line': 50434, 'case': 'fsync02', 'text': '\x1b[37m[4806.347772 0:21677 axruntime::lang_items:5] \x1b[31mpanicked at library/alloc/src/alloc.rs:437:13:'}`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

- ltp-musl: passed=605 failed=1358 timed_out=28
