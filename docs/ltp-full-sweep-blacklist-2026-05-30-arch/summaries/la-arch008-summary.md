# la-arch008 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch008.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **False**
- run-eval status: `0`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=412 (1956 cases, timeout 15s)`
  - line 39737: `ltp case list: all-minus-blacklist skipped=415 (1960 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 1201
- Parser FAIL LTP CASE count: 2698
- TIMEOUT matches: 55
- Raw RUN markers: 3903
- Raw FAIL markers: 3902
- Raw TIMEOUT markers: 55
- Raw SKIP markers: 0
- Incomplete RUN count: 1

## Internal LTP signals

- TBROK: 1032
- TCONF: 1934
- TFAIL: 4045
- ENOSYS/not implemented matches: 1279
- Parser panic/trap matches: 1
- Strict panic count: 2
- First strict panic: `{'line': 79046, 'case': 'write01', 'text': '\x1b[37m[8397.533298 0:18686 axruntime::lang_items:5] \x1b[31mpanicked at library/alloc/src/alloc.rs:437:13:'}`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

- ltp-musl: passed=604 failed=1352 timed_out=26
