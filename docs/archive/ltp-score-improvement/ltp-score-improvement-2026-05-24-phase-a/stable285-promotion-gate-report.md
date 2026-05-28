# stable285 promotion gate report

## Promoted tranche

`open08, open13, pipe12, pipe13, pipe2_01, pipe2_04, dup207, getcwd02, fchdir02, fcntl23, open09, sched_getattr02, statvfs02, symlink02, symlink04`

## RV stable285 aggregate

- PASS LTP CASE: 570
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 285 passed, 0 failed
- ltp-glibc: 285 passed, 0 failed

## LA stable285 aggregate

- PASS LTP CASE: 570
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 285 passed, 0 failed
- ltp-glibc: 285 passed, 0 failed

## Gate decision

Accepted. Both RV and LA aggregates were 570/0 with ltp-musl 285/0 and ltp-glibc 285/0. Internal TCONF was limited to known `read02` pass-with-TCONF; TFAIL/TBROK/timeout/ENOSYS/panic/trap were 0.
