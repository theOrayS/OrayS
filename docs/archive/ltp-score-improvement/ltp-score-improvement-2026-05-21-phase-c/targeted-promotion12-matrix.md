# LTP promotion-candidate report

- Inputs: `docs/ltp-score-improvement-2026-05-23/rv-targeted-promotion12.log`, `docs/ltp-score-improvement-2026-05-23/la-targeted-promotion12.log`
- Required arches: la, rv
- Required libcs: glibc, musl
- Required arch/libc combos: 4
- Promotion candidates: 12
- Blocked/incomplete cases: 0

## Candidates
| Case | Clean combos | Max runtime ms | Min free-frames delta after cleanup |
| --- | --- | ---: | ---: |
| getegid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 8965 | -4110 |
| geteuid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 5761 | -21 |
| getgid03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 8052 | -21 |
| getgroups03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 6915 | -12 |
| getpgid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 6818 | -2064 |
| getppid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 7564 | -30 |
| getrlimit02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 7261 | -21 |
| getsid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 6400 | -21 |
| getuid03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 7959 | -21 |
| uname02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 6529 | -21 |
| wait01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 5637 | -21 |
| wait02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 5728 | -30 |

## Blocked or incomplete
- None
