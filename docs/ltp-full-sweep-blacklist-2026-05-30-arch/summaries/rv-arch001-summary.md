# rv-arch001 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch001.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closed: **False**
- run-eval status: `0`
- Selection lines:
  - line 989: `ltp case list: all-minus-blacklist skipped=40 (2328 cases, timeout 15s)`

## Wrapper counts

- Parser PASS LTP CASE count: 243
- Parser FAIL LTP CASE count: 571
- TIMEOUT matches: 9
- Raw RUN markers: 815
- Raw FAIL markers: 814
- Raw TIMEOUT markers: 9
- Raw SKIP markers: 0
- Incomplete RUN count: 1

## Internal LTP signals

- TBROK: 183
- TCONF: 384
- TFAIL: 1192
- ENOSYS/not implemented matches: 69
- Parser panic/trap matches: 1
- Strict panic count: 2
- First strict panic: `{'line': 16821, 'case': 'kill10', 'text': '\x1b[37m[1519.409310 0:5309 axruntime::lang_items:5] \x1b[31mpanicked at library/alloc/src/alloc.rs:437:13:'}`
- Strict resource failure count: 0
- First strict resource failure: `None`

## Suite summaries

