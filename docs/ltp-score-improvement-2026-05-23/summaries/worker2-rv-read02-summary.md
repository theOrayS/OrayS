# LTP summary: `docs/ltp-score-improvement-2026-05-23/raw/worker2-rv-read02.log`

- PASS LTP CASE: 2
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0

## Suite summaries
- ltp-musl: 1 passed, 0 failed
- ltp-glibc: 1 passed, 0 failed

## Case matrix
| Case | Arch | Libc | Group | Status | Code | Runtime ms | Free frames before | Free frames after cleanup | Free frames delta | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap |
| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| read02 | rv | glibc | ltp-glibc | PASS | 0 | 3210 | 256803 | 256789 | -14 | 0 | 0 | 2 | 0 | 0 | 0 |
| read02 | rv | musl | ltp-musl | PASS | 0 | 3520 | 258865 | 256803 | -2062 | 0 | 0 | 2 | 0 | 0 | 0 |

## Categories
- pass_clean: 0
- pass_with_tconf: 2 (rv:glibc:read02, rv:musl:read02)
- fail_wrapper: 0
- internal_tfail: 0
- internal_tbrok: 0
- timeout: 0
- enosys: 0
- panic_trap: 0
- unknown: 0

## Groups
### ltp-musl
- PASS: 1
- FAIL: 0
- Internal: {'TCONF': 2}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### ltp-glibc
- PASS: 1
- FAIL: 0
- Internal: {'TCONF': 2}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0
