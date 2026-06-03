# milestone-06 regression matrix

This checkpoint changed timerslack/prctl/proc behavior and default UTS hostname sharing, but did not promote stable806. The targeted repair evidence is clean for `prctl08`, `prctl09`, and `utsname02`; the UTS adjacent stable subset is clean, while timerslack/prctl adjacent rows still need a promotion-time regression gate before any stable-list commit.

| Repair area | Covered now | Required before promotion |
| --- | --- | --- |
| timerslack / prctl | `prctl08`, `prctl09` RV + LA × musl + glibc parser-clean | Adjacent stable `prctl01`, `prctl05` and representative `PR_SET_NAME/PR_GET_NAME` rows if available |
| proc synthetic file plumbing | `/proc/self/timerslack_ns`, `/proc/<pid>/timerslack_ns` read/write/stat covered by `prctl08` | Existing `/proc` stable rows such as `proc01`, `uname01`, `uname02`, `newuname01`, `utsname01`, `utsname04` |
| UTS hostname sharing | `utsname02` RV + LA × musl + glibc parser-clean; adjacent `gethostname01,sethostname01,sethostname02,sethostname03,uname01,uname02,uname04,newuname01,utsname01,utsname04` clean on RV+LA | Keep `CLONE_NEWUTS`/`unshare(CLONE_NEWUTS)` rows blocked until a real UTS namespace implementation exists; do not count `utsname03` |
| priority/nice/rlimit | Not changed | `getpriority01`, `getpriority02`, `setpriority02`, `setrlimit01`, `setrlimit03`, `setrlimit05` if future priority fixes are batched |
| time/signal wait | Not changed | `clock_gettime04`, `nanosleep01`, `getitimer01`, `getitimer02`, `setitimer02`, `sigsuspend01`, `sigaction02`, `rt_sigprocmask01`, `sigprocmask01` if future time/signal fixes are batched |
| epoll/eventfd/timerfd | Not changed | milestone-05 promoted epoll/eventfd/timerfd/signalfd cases plus `poll01`, `pipe01`, `pipe06`, `pipe2_01`, `pipe2_02` if future fd fixes are batched |

Promotion gate remains unchanged: RV + LA × musl + glibc wrapper PASS, parser-clean, with no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` beyond explicitly disclosed inherited caveats.
