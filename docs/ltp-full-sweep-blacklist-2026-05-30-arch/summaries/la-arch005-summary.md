# la-arch005 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch005.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **False**
- run-eval status: `0`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=409 (1959 cases, timeout 15s)`
  - line 39801: `ltp case list: all-minus-blacklist skipped=412 (1963 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 857
- Parser FAIL LTP CASE count: 1937
- TIMEOUT matches: 37
- Raw RUN markers: 2796
- Raw FAIL markers: 2795
- Raw TIMEOUT markers: 37
- Raw SKIP markers: 0
- Incomplete RUN count: 1

## Internal LTP signals

- TBROK: 716
- TCONF: 1367
- TFAIL: 3153
- ENOSYS/not implemented matches: 713
- Parser panic/trap matches: 1
- Strict panic count: 2
- First strict panic: `{'line': 56054, 'case': 'lftest', 'text': '\x1b[37m[5790.225179 0:14793 axruntime::lang_items:5] \x1b[31mpanicked at library/alloc/src/alloc.rs:437:13:'}`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

- ltp-musl: passed=604 failed=1355 timed_out=26
