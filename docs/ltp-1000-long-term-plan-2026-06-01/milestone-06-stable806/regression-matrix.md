# milestone-06 regression matrix draft

No source change or stable promotion was made in this interim checkpoint. If the next slices repair the observed blockers, protect at least these adjacent stable rows before promotion:

| Future repair area | Adjacent stable regression rows |
| --- | --- |
| timerslack / prctl / proc synthetic | `prctl01`, `prctl05`, `uname01`, `uname02`, `newuname01`, `utsname01`, `utsname04`, `/proc` stable rows such as `proc01` |
| priority/nice/rlimit | `getpriority01`, `getpriority02`, `setpriority02`, `setrlimit01`, `setrlimit03`, `setrlimit05`, scheduler stable rows |
| time/signal wait | `clock_gettime04`, `nanosleep01`, `getitimer01`, `getitimer02`, `setitimer02`, `sigsuspend01`, `sigaction02`, `rt_sigprocmask01`, `sigprocmask01` |
| epoll/eventfd/timerfd | milestone-05 promoted epoll/eventfd/timerfd/signalfd cases plus `poll01`, `pipe01`, `pipe06`, `pipe2_01`, `pipe2_02` |
| network/socket scout | stable `accept01`, `listen01`, `socket02`, `socketpair02`, plus pipe/poll readiness rows |

Promotion gate remains unchanged: RV + LA × musl + glibc wrapper PASS, parser-clean, with no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` beyond explicitly disclosed inherited caveats.
