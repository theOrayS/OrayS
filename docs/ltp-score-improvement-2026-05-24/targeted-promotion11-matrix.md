# LTP promotion-candidate report

- Inputs: `docs/ltp-score-improvement-2026-05-24/targeted-promotion11-rv.log`, `docs/ltp-score-improvement-2026-05-24/targeted-promotion11-la.log`
- Required arches: la, rv
- Required libcs: glibc, musl
- Required arch/libc combos: 4
- Promotion candidates: 10
- Blocked/incomplete cases: 1

## Candidates
| Case | Clean combos | Max runtime ms | Min free-frames delta after cleanup |
| --- | --- | ---: | ---: |
| getresgid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2198 | -12 |
| getresgid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2083 | -12 |
| getresgid03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2008 | -12 |
| getresuid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2111 | -12 |
| getresuid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 1998 | -12 |
| getresuid03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2219 | -12 |
| getsid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2140 | -2072 |
| rt_sigaction02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2140 | -4108 |
| sched_getscheduler01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2113 | -30 |
| uname04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2041 | -21 |

## Blocked or incomplete
| Case | Reason |
| --- | --- |
| sched_getscheduler02 | la:musl:ltp-musl TFAIL=1/event-failures=1/status=FAIL |
