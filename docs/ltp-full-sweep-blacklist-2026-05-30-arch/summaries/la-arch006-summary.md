# la-arch006 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch006.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **False**
- run-eval status: `0`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=410 (1958 cases, timeout 15s)`
  - line 39780: `ltp case list: all-minus-blacklist skipped=413 (1962 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 899
- Parser FAIL LTP CASE count: 2040
- TIMEOUT matches: 38
- Raw RUN markers: 2941
- Raw FAIL markers: 2940
- Raw TIMEOUT markers: 38
- Raw SKIP markers: 0
- Incomplete RUN count: 1

## Internal LTP signals

- TBROK: 752
- TCONF: 1397
- TFAIL: 3249
- ENOSYS/not implemented matches: 764
- Parser panic/trap matches: 1
- Strict panic count: 2
- First strict panic: `{'line': 58994, 'case': 'mmstress', 'text': '\x1b[37m[6107.036454 0:15189 axruntime::lang_items:5] \x1b[31mpanicked at library/alloc/src/alloc.rs:437:13:'}`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

- ltp-musl: passed=604 failed=1354 timed_out=26
