# LTP promotion-candidate report

- Inputs: `docs/ltp-score-improvement-2026-05-22-phase-b/wave-a2-fd-fs-proc-rv.log`, `docs/ltp-score-improvement-2026-05-22-phase-b/wave-a2-new-candidates-la.log`
- Required arches: la, rv
- Required libcs: glibc, musl
- Required arch/libc combos: 4
- Promotion candidates: 8
- Blocked/incomplete cases: 50

## Candidates
| Case | Clean combos | Max runtime ms | Min free-frames delta after cleanup |
| --- | --- | ---: | ---: |
| dup202 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 12557 | -2064 |
| mkdirat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 9019 | -12 |
| openat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 13281 | -21 |
| pipe04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 9640 | -30 |
| pipe05 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 10572 | -12 |
| pread01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 9437 | -21 |
| pwrite01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 15140 | -21 |
| sysinfo01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 10941 | -12 |

## Blocked or incomplete
| Case | Reason |
| --- | --- |
| access02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=4/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=4/event-failures=1/status=FAIL |
| access04 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| chmod01 | missing la:glibc, la:musl |
| chmod02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc event-failures=1/status=FAIL; rv:musl:ltp-musl event-failures=1/status=FAIL |
| close01 | missing la:glibc, la:musl |
| close02 | missing la:glibc, la:musl |
| dup01 | missing la:glibc, la:musl |
| dup02 | missing la:glibc, la:musl |
| dup03 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=1/event-failures=1/status=FAIL |
| dup05 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/ENOSYS=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/ENOSYS=1/event-failures=1/status=FAIL |
| dup201 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=1/event-failures=1/status=FAIL |
| faccessat01 | la:glibc:ltp-glibc TFAIL=3/ENOSYS=3/event-failures=1/status=FAIL |
| fchmod02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| fstat01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc event-failures=1/status=FAIL; rv:musl:ltp-musl event-failures=1/status=FAIL |
| fstatfs01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| fstatfs02 | missing la:glibc, la:musl; rv:musl:ltp-musl TFAIL=1/event-failures=1/status=FAIL |
| fstatvfs01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc event-failures=1/status=FAIL; rv:musl:ltp-musl event-failures=1/status=FAIL |
| ftruncate01 | missing la:glibc, la:musl |
| ftruncate03 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| getgroups03 | missing la:glibc, la:musl |
| getpgrp01 | missing la:glibc, la:musl |
| getrlimit03 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=16/ENOSYS=16/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=16/ENOSYS=16/event-failures=1/status=FAIL |
| link01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc event-failures=1/status=FAIL; rv:musl:ltp-musl event-failures=1/status=FAIL |
| link02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=1/ENOSYS=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=1/ENOSYS=1/event-failures=1/status=FAIL |
| linkat01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=2/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=2/event-failures=1/status=FAIL |
| lseek01 | missing la:glibc, la:musl |
| lseek02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/ENOSYS=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/ENOSYS=1/event-failures=1/status=FAIL |
| lstat01 | missing la:glibc, la:musl |
| mkdir01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc event-failures=1/status=FAIL; rv:musl:ltp-musl event-failures=1/status=FAIL |
| mkdir02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=2/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| open01 | missing la:glibc, la:musl |
| open02 | missing la:glibc, la:musl |
| pipe01 | missing la:glibc, la:musl |
| pipe02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=1/event-failures=1/status=FAIL |
| readlink01 | missing la:glibc, la:musl |
| readlinkat01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| readlinkat02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=2/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=2/event-failures=1/status=FAIL |
| rename01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| renameat01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=2/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=2/event-failures=1/status=FAIL |
| rmdir01 | missing la:glibc, la:musl |
| stat01 | missing la:glibc, la:musl |
| stat02 | missing la:glibc, la:musl |
| statfs01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| statfs02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=2/event-failures=1/status=FAIL |
| statvfs01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/event-failures=1/status=FAIL |
| truncate01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc event-failures=1/status=FAIL; rv:musl:ltp-musl event-failures=1/status=FAIL |
| truncate02 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TFAIL=2/ENOSYS=2/event-failures=1/status=FAIL; rv:musl:ltp-musl TFAIL=2/ENOSYS=2/event-failures=1/status=FAIL |
| unlink01 | missing la:glibc, la:musl; rv:glibc:ltp-glibc event-failures=1/status=FAIL; rv:musl:ltp-musl event-failures=1/status=FAIL |
| unlink05 | missing la:glibc, la:musl; rv:glibc:ltp-glibc TBROK=1/ENOSYS=1/event-failures=1/status=FAIL; rv:musl:ltp-musl TBROK=1/ENOSYS=1/event-failures=1/status=FAIL |
| unlinkat01 | la:glibc:ltp-glibc TBROK=1/event-failures=1/status=FAIL |
