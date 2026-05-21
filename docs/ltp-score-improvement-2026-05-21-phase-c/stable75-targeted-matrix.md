# LTP promotion-candidate report

- Inputs: `docs/ltp-score-improvement-2026-05-23/rv-stable75-targeted.log`, `docs/ltp-score-improvement-2026-05-23/la-stable75-targeted.log`
- Required arches: la, rv
- Required libcs: glibc, musl
- Required arch/libc combos: 4
- Promotion candidates: 74
- Blocked/incomplete cases: 1

## Candidates
| Case | Clean combos | Max runtime ms | Min free-frames delta after cleanup |
| --- | --- | ---: | ---: |
| access01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 6688 | -2872 |
| access03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2308 | -57 |
| alarm02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2104 | -21 |
| alarm03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2379 | -8213 |
| brk01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2147 | -30 |
| chdir01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2123 | -21 |
| chmod01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2461 | -30 |
| clock_gettime02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2099 | -21 |
| clone01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2137 | -30 |
| close01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2283 | -21 |
| close02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2163 | -21 |
| creat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2262 | -21 |
| creat03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2144 | -21 |
| dup01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2263 | -21 |
| dup02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2063 | -21 |
| exit01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2170 | -21 |
| exit02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 1975 | -30 |
| exit_group01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2431 | -32 |
| fchmod01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2190 | -21 |
| fcntl01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2081 | -12 |
| fcntl02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2079 | -21 |
| fcntl03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2094 | -21 |
| fork01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 1972 | -30 |
| ftruncate01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2209 | -21 |
| getcwd01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2246 | -21 |
| getegid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2115 | -21 |
| getegid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2478 | -21 |
| geteuid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2233 | -21 |
| geteuid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2053 | -21 |
| getgid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2186 | -21 |
| getgid03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2338 | -21 |
| getgroups03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2177 | -12 |
| getpgid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2383 | -21 |
| getpgrp01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2311 | -21 |
| getpid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 6055 | -921 |
| getpid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2279 | -30 |
| getppid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2332 | -21 |
| getppid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2576 | -30 |
| getrlimit01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2255 | -21 |
| getrlimit02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2030 | -21 |
| getrusage01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2190 | -21 |
| getsid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2244 | -21 |
| gettid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2158 | -21 |
| gettimeofday01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2229 | -21 |
| getuid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2156 | -21 |
| getuid03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2173 | -21 |
| kill03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2411 | -21 |
| lseek01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2280 | -21 |
| lstat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2209 | -21 |
| mmap01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2211 | -21 |
| open01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2149 | -21 |
| open02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2106 | -21 |
| open03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2050 | -21 |
| pipe01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2366 | -4112 |
| proc01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2496 | -12 |
| read01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2149 | -21 |
| readlink01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2264 | -30 |
| rmdir01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2105 | -21 |
| rt_sigaction01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2496 | -12 |
| sched_yield01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2183 | -12 |
| sigaction01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2148 | -12 |
| stat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2183 | -21 |
| stat02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2021 | -21 |
| symlink01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2187 | -12 |
| time01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 1989 | -21 |
| times01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2175 | -21 |
| umask01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2314 | -21 |
| uname01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2159 | -21 |
| uname02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2216 | -21 |
| wait01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2253 | -21 |
| wait02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2355 | -30 |
| wait401 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2262 | -30 |
| write01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 3308 | -57365 |
| write02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2355 | -21 |

## Blocked or incomplete
| Case | Reason |
| --- | --- |
| read02 | la:glibc:ltp-glibc TCONF=2; la:musl:ltp-musl TCONF=2; rv:glibc:ltp-glibc TCONF=2; rv:musl:ltp-musl TCONF=2 |
