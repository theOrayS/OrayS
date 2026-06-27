# stable906 promotion candidates

This milestone promotes the 50 cases listed in `targeted-cases.txt` from baseline stable856 to stable906.

Promotion basis:
- Fresh final new50 gate: RV + LA x musl + glibc wrapper PASS.
- Parser clean: no TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.
- Blacklist/SKIP/status0/full-sweep evidence was not counted.
- Combined evidence: `target/ltp-1000-milestone-08-stable906/stable906-new50-rvla-final-gate-20260605T115135+0800.txt`.

# LTP promotion-candidate report

- Inputs: `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.log`, `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.log`
- Required arches: la, rv
- Required libcs: glibc, musl
- Required arch/libc combos: 4
- Promotion candidates: 50
- Blocked/incomplete cases: 0

## Candidates
| Case | Clean combos | Max runtime ms | Min free-frames delta after cleanup |
| --- | --- | ---: | ---: |
| chroot01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2205 | -21 |
| chroot02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2401 | -30 |
| chroot03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2308 | -21 |
| chroot04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2229 | -21 |
| fallocate02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2230 | -12 |
| fallocate04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2386 | -21 |
| fallocate05 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 8275 | -122894 |
| ftest06 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 4111 | -81 |
| get_robust_list01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2262 | -12 |
| getcwd03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2308 | -21 |
| gethostname02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2214 | -21 |
| madvise01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2336 | -21 |
| madvise03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2380 | -21 |
| madvise05 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2287 | -21 |
| memfd_create01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2278 | -21 |
| memfd_create02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2379 | -21 |
| mlock02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2239 | -21 |
| mlock05 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2246 | -21 |
| mlock202 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2218 | -21 |
| mlock203 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2295 | -21 |
| mlockall02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2221 | -12 |
| mlockall03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2201 | -12 |
| mremap01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2321 | -15 |
| mremap02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2291 | -12 |
| mremap03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2236 | -12 |
| mremap04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2194 | -12 |
| mremap05 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2239 | -12 |
| mremap06 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2352 | -21 |
| msgctl01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2270 | -21 |
| msgctl02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2188 | -21 |
| msgctl03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2248 | -21 |
| msgget01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2335 | -21 |
| msgget02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2339 | -48 |
| msgrcv01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2299 | -21 |
| msgrcv02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2597 | -93 |
| msgsnd01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2259 | -21 |
| msgsnd02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2435 | -30 |
| munlockall01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2281 | -21 |
| munmap03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2134 | -12 |
| nanosleep02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 3255 | -30 |
| nice04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2466 | -21 |
| readlink03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2306 | -21 |
| readlinkat02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2315 | -21 |
| rt_sigaction03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2399 | -12 |
| setgroups04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 1967 | -12 |
| setsockopt04 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2247 | -21 |
| settimeofday02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2230 | -21 |
| shmat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2458 | -57 |
| sockioctl01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2206 | -12 |
| timer_settime03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2305 | -21 |

## Blocked or incomplete
- None
