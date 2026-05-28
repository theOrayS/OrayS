# stable270 promotion gate report

## Result

stable270 was retained as the first tranche gate in this campaign. New case promotion was accepted only after parser summaries showed zero wrapper FAIL, zero timeout, zero ENOSYS, zero panic/trap, and no internal TFAIL/TBROK/new TCONF.

## RV aggregate

- PASS LTP CASE: 540
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 270 passed, 0 failed
- ltp-glibc: 270 passed, 0 failed

## LA aggregate

- PASS LTP CASE: 540
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 270 passed, 0 failed
- ltp-glibc: 270 passed, 0 failed

## Notes

- `read02` remains the only known `pass_with_tconf` caveat in stable aggregate evidence.
- Raw logs are under `raw/` for local audit and are not intended for commit by default.
