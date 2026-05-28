# LTP summary: `docs/ltp-score-improvement-2026-05-24/raw/task2-rv-candidate-batch.log`

- PASS LTP CASE: 4
- FAIL LTP CASE: 20
- Internal TFAIL/TBROK/TCONF: 41 ({'TFAIL': 19, 'TBROK': 6, 'TCONF': 16})
- timeout matches: 6
- ENOSYS/not implemented matches: 4
- panic/trap matches: 0

## Suite summaries
- ltp-musl: 2 passed, 10 failed
- ltp-glibc: 2 passed, 10 failed

## Case matrix
| Case | Arch | Libc | Group | Status | Code | Runtime ms | Free frames before | Free frames after cleanup | Free frames delta | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap |
| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| access02 | rv | glibc | ltp-glibc | FAIL | 1 | 7762 | 256390 | 256292 | -98 | 4 | 0 | 0 | 0 | 0 | 0 |
| access02 | rv | musl | ltp-musl | FAIL | 1 | 8532 | 256684 | 256586 | -98 | 4 | 0 | 0 | 0 | 0 | 0 |
| clock_getres01 | rv | glibc | ltp-glibc | PASS | 0 | 6377 | 256467 | 256432 | -35 | 0 | 0 | 8 | 0 | 0 | 0 |
| clock_getres01 | rv | musl | ltp-musl | PASS | 0 | 5662 | 256761 | 256726 | -35 | 0 | 0 | 8 | 0 | 0 | 0 |
| clock_gettime01 | rv | glibc | ltp-glibc | FAIL | 137 | 9421 | 256481 | 256467 | -14 | 0 | 0 | 0 | 1 | 0 | 0 |
| clock_gettime01 | rv | musl | ltp-musl | FAIL | 137 | 8649 | 256775 | 256761 | -14 | 0 | 0 | 0 | 1 | 0 | 0 |
| dup03 | rv | glibc | ltp-glibc | FAIL | 1 | 4598 | 256292 | 256278 | -14 | 1 | 0 | 0 | 0 | 0 | 0 |
| dup03 | rv | musl | ltp-musl | FAIL | 1 | 5305 | 256586 | 256572 | -14 | 1 | 0 | 0 | 0 | 0 | 0 |
| getpgid01 | rv | glibc | ltp-glibc | FAIL | 3 | 5794 | 256523 | 256502 | -21 | 1 | 1 | 0 | 0 | 0 | 0 |
| getpgid01 | rv | musl | ltp-musl | FAIL | 3 | 5143 | 258865 | 256796 | -2069 | 1 | 1 | 0 | 0 | 0 | 0 |
| getsid01 | rv | glibc | ltp-glibc | PASS | 0 | 5996 | 256502 | 256481 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getsid01 | rv | musl | ltp-musl | PASS | 0 | 2988 | 256796 | 256775 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| lseek02 | rv | glibc | ltp-glibc | FAIL | 2 | 7932 | 256257 | 256243 | -14 | 0 | 1 | 0 | 0 | 1 | 0 |
| lseek02 | rv | musl | ltp-musl | FAIL | 2 | 3965 | 256551 | 256537 | -14 | 0 | 1 | 0 | 0 | 1 | 0 |
| nanosleep01 | rv | glibc | ltp-glibc | FAIL | 137 | 10379 | 256432 | 256411 | -21 | 2 | 0 | 0 | 1 | 0 | 0 |
| nanosleep01 | rv | musl | ltp-musl | FAIL | 137 | 9128 | 256726 | 256705 | -21 | 1 | 0 | 0 | 1 | 0 | 0 |
| nanosleep02 | rv | glibc | ltp-glibc | FAIL | 137 | 10724 | 256411 | 256390 | -21 | 0 | 0 | 0 | 1 | 0 | 0 |
| nanosleep02 | rv | musl | ltp-musl | FAIL | 137 | 10074 | 256705 | 256684 | -21 | 0 | 0 | 0 | 1 | 0 | 0 |
| pipe02 | rv | glibc | ltp-glibc | FAIL | 1 | 7378 | 256278 | 256257 | -21 | 1 | 0 | 0 | 0 | 0 | 0 |
| pipe02 | rv | musl | ltp-musl | FAIL | 1 | 4226 | 256572 | 256551 | -21 | 1 | 0 | 0 | 0 | 0 | 0 |
| statfs01 | rv | glibc | ltp-glibc | FAIL | 6 | 5891 | 256243 | 256236 | -7 | 0 | 1 | 0 | 0 | 0 | 0 |
| statfs01 | rv | musl | ltp-musl | FAIL | 6 | 3554 | 256537 | 256530 | -7 | 0 | 1 | 0 | 0 | 0 | 0 |
| sysinfo01 | rv | glibc | ltp-glibc | FAIL | 1 | 6480 | 256236 | 256229 | -7 | 1 | 0 | 0 | 0 | 1 | 0 |
| sysinfo01 | rv | musl | ltp-musl | FAIL | 1 | 4979 | 256530 | 256523 | -7 | 1 | 0 | 0 | 0 | 1 | 0 |

## Categories
- pass_clean: 2 (rv:glibc:getsid01, rv:musl:getsid01)
- pass_with_tconf: 2 (rv:glibc:clock_getres01, rv:musl:clock_getres01)
- fail_wrapper: 20 (rv:glibc:access02, rv:musl:access02, rv:glibc:clock_gettime01, rv:musl:clock_gettime01, rv:glibc:dup03, rv:musl:dup03, rv:glibc:getpgid01, rv:musl:getpgid01, rv:glibc:lseek02, rv:musl:lseek02, rv:glibc:nanosleep01, rv:musl:nanosleep01, rv:glibc:nanosleep02, rv:musl:nanosleep02, rv:glibc:pipe02, rv:musl:pipe02, rv:glibc:statfs01, rv:musl:statfs01, rv:glibc:sysinfo01, rv:musl:sysinfo01)
- internal_tfail: 12 (rv:glibc:access02, rv:musl:access02, rv:glibc:dup03, rv:musl:dup03, rv:glibc:getpgid01, rv:musl:getpgid01, rv:glibc:nanosleep01, rv:musl:nanosleep01, rv:glibc:pipe02, rv:musl:pipe02, rv:glibc:sysinfo01, rv:musl:sysinfo01)
- internal_tbrok: 6 (rv:glibc:getpgid01, rv:musl:getpgid01, rv:glibc:lseek02, rv:musl:lseek02, rv:glibc:statfs01, rv:musl:statfs01)
- timeout: 6 (rv:glibc:clock_gettime01, rv:musl:clock_gettime01, rv:glibc:nanosleep01, rv:musl:nanosleep01, rv:glibc:nanosleep02, rv:musl:nanosleep02)
- enosys: 4 (rv:glibc:lseek02, rv:musl:lseek02, rv:glibc:sysinfo01, rv:musl:sysinfo01)
- panic_trap: 0
- unknown: 0

## FAIL LTP CASE
- ltp-musl:getpgid01
- ltp-musl:clock_gettime01
- ltp-musl:nanosleep01
- ltp-musl:nanosleep02
- ltp-musl:access02
- ltp-musl:dup03
- ltp-musl:pipe02
- ltp-musl:lseek02
- ltp-musl:statfs01
- ltp-musl:sysinfo01
- ltp-glibc:getpgid01
- ltp-glibc:clock_gettime01
- ltp-glibc:nanosleep01
- ltp-glibc:nanosleep02
- ltp-glibc:access02
- ltp-glibc:dup03
- ltp-glibc:pipe02
- ltp-glibc:lseek02
- ltp-glibc:statfs01
- ltp-glibc:sysinfo01

## Groups
### ltp-musl
- PASS: 2
- FAIL: 10
- Internal: {'TFAIL': 9, 'TBROK': 3, 'TCONF': 8}
- timeout: 3
- ENOSYS/not implemented: 2
- panic/trap: 0
- Fail cases: getpgid01, clock_gettime01, nanosleep01, nanosleep02, access02, dup03, pipe02, lseek02, statfs01, sysinfo01

### ltp-glibc
- PASS: 2
- FAIL: 10
- Internal: {'TFAIL': 10, 'TBROK': 3, 'TCONF': 8}
- timeout: 3
- ENOSYS/not implemented: 2
- panic/trap: 0
- Fail cases: getpgid01, clock_gettime01, nanosleep01, nanosleep02, access02, dup03, pipe02, lseek02, statfs01, sysinfo01
