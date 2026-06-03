# milestone-06 regression matrix

This checkpoint changed timerslack/prctl/proc behavior and default UTS hostname sharing, but did not promote stable806. The targeted repair evidence is clean for `prctl08`, `prctl09`, and `utsname02`; the UTS adjacent stable subset is clean, while timerslack/prctl adjacent rows still need a promotion-time regression gate before any stable-list commit.

| Repair area | Covered now | Required before promotion |
| --- | --- | --- |
| timerslack / prctl | `prctl08`, `prctl09` RV + LA × musl + glibc parser-clean | Adjacent stable `prctl01`, `prctl05` and representative `PR_SET_NAME/PR_GET_NAME` rows if available |
| proc synthetic file plumbing | `/proc/self/timerslack_ns`, `/proc/<pid>/timerslack_ns` read/write/stat covered by `prctl08` | Existing `/proc` stable rows such as `proc01`, `uname01`, `uname02`, `newuname01`, `utsname01`, `utsname04` |
| UTS hostname sharing | `utsname02` RV + LA × musl + glibc parser-clean; adjacent `gethostname01,sethostname01,sethostname02,sethostname03,uname01,uname02,uname04,newuname01,utsname01,utsname04` clean on RV+LA | Keep `CLONE_NEWUTS`/`unshare(CLONE_NEWUTS)` rows blocked until a real UTS namespace implementation exists; do not count `utsname03` |
| VFS parent symlink / rmdir errno | `mkdirat02`, `rmdir02` RV + LA × musl + glibc parser-clean; adjacent stable `mkdir/rmdir/unlink/symlink/mknod/rename` subset clean on RV+LA | Keep remaining `mkdir02`, `mkdir03`, `mkdir09`, `mknod07`, `mknodat02`, `symlink03`, `unlink09` rows blocked until their visible `TFAIL/TBROK/TCONF` causes are fixed. |
| priority/nice/rlimit | Not changed | `getpriority01`, `getpriority02`, `setpriority02`, `setrlimit01`, `setrlimit03`, `setrlimit05` if future priority fixes are batched |
| time/signal wait | Not changed | `clock_gettime04`, `nanosleep01`, `getitimer01`, `getitimer02`, `setitimer02`, `sigsuspend01`, `sigaction02`, `rt_sigprocmask01`, `sigprocmask01` if future time/signal fixes are batched |
| epoll/eventfd/timerfd | Not changed | milestone-05 promoted epoll/eventfd/timerfd/signalfd cases plus `poll01`, `pipe01`, `pipe06`, `pipe2_01`, `pipe2_02` if future fd fixes are batched |

Promotion gate remains unchanged: RV + LA × musl + glibc wrapper PASS, parser-clean, with no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` beyond explicitly disclosed inherited caveats.

## Additional blocker matrix from post-UTS triage

| Lane | Current evidence | Promotion boundary |
| --- | --- | --- |
| readlink/readlinkat | RV clean, LA glibc clean, LA musl `TFAIL` for zero-size wrapper case | Do not reject valid `bufsiz=1` kernel calls; revisit only if libc/test ABI boundary has a real semantic fix. |
| nice/setpriority | RV glibc `nice04` clean, RV musl `nice04` `TFAIL`; shared `setpriority` code unchanged | Protect existing stable `setpriority` semantics before any errno-boundary change. |
| statx | RV statx scout has `TCONF`, wrapper FAILs, and `statx11` timeouts | Needs a real statx attribute/env semantics lane; no `pass_with_tconf` promotion. |
| credentials/capabilities | RV 16-bit UID/cap rows `TCONF`; glibc `gettid02` `TBROK` futex abort | Needs unsupported-ABI policy or real capability/futex work; no partial musl-only promotion. |
| VFS/FD/select scout | RV scout has select pass-with-TCONF rows, fcntl17/fcntl17_64 timeouts, VFS path errno TFAIL/TBROK, and zero candidates | Split into isolated fixes; no broad VFS/FD/select promotion from this evidence. |


## Additional VFS regression boundary

The parent-symlink/rmdir repair is protected by a 36-case adjacent stable subset on both architectures (`72 PASS / 0 FAIL` for RV and `72 PASS / 0 FAIL` for LA). This covers existing stable mkdir/mkdirat, rmdir, unlink/unlinkat, symlink/symlinkat, mknod/mknodat, rename, and renameat rows. Future edits in `FdTable::mkdirat`, `FdTable::mknodat`, `FdTable::unlinkat`, `sys_symlinkat`, or process mountpoint checks should rerun this subset before promotion.

## mkdir setgid/final-symlink regression boundary

The `mkdir02`/`mkdir03` repair is protected by targeted RV + LA × musl + glibc evidence for `mkdir02`, `mkdir03`, `mkdirat02`, and `rmdir02` (`16 PASS / 0 FAIL` across the two architecture logs) plus a 35-case adjacent stable subset on both architectures (`70 PASS / 0 FAIL` for RV and `70 PASS / 0 FAIL` for LA).

Covered stable adjacency includes chmod/fchmod/fchmodat, chown/fchown/fchownat, `open10`, `creat08`, `creat09`, mkdir/mkdirat, mknod/mknodat, symlink/symlinkat, and rmdir rows. Future edits in `clear_path_chown_special_bits`, `record_created_path_metadata`, `FdTable::mkdirat`, or `FdTable::mknodat` should rerun this subset before promotion.

## fcntl read-lease regression boundary

The `fcntl27`/`fcntl27_64` repair is protected by targeted RV + LA × musl + glibc evidence for `fcntl27` and `fcntl27_64` (`8 PASS / 0 FAIL` across the two targeted candidate pairs) plus all current stable `fcntl*` rows and `fcntl27` on both architectures (`98 PASS / 0 FAIL` for RV and `98 PASS / 0 FAIL` for LA). `fcntl27_64` required no extra source change beyond the same generic lease access rule.

Covered stable adjacency includes `F_DUPFD`, `F_GETFD`/`F_SETFD`, `F_GETFL`/`F_SETFL`, record locking (`F_GETLK`, `F_SETLK`, `F_SETLKW`), lease read-only success (`fcntl23`, `fcntl23_64`), and OFD/lock stress rows. Future edits in `FdTable::fcntl`, `fcntl_getlease`, or `fcntl_setlease` should rerun this subset before promotion.
