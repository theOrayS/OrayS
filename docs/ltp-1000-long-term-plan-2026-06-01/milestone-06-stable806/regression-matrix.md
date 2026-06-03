# milestone-06 regression matrix

This checkpoint changed timerslack/prctl/proc behavior and default UTS hostname sharing, but did not promote stable806. The targeted repair evidence is clean for `prctl08`, `prctl09`, and `utsname02`; the UTS adjacent stable subset is clean, while timerslack/prctl adjacent rows still need a promotion-time regression gate before any stable-list commit.

| Repair area | Covered now | Required before promotion |
| --- | --- | --- |
| timerslack / prctl | `prctl08`, `prctl09` RV + LA × musl + glibc parser-clean | Adjacent stable `prctl01`, `prctl05` and representative `PR_SET_NAME/PR_GET_NAME` rows if available |
| proc synthetic file plumbing | `/proc/self/timerslack_ns`, `/proc/<pid>/timerslack_ns` read/write/stat covered by `prctl08` | Existing `/proc` stable rows such as `proc01`, `uname01`, `uname02`, `newuname01`, `utsname01`, `utsname04` |
| UTS hostname sharing | `utsname02` RV + LA × musl + glibc parser-clean; adjacent `gethostname01,sethostname01,sethostname02,sethostname03,uname01,uname02,uname04,newuname01,utsname01,utsname04` clean on RV+LA | Keep `CLONE_NEWUTS`/`unshare(CLONE_NEWUTS)` rows blocked until a real UTS namespace implementation exists; do not count `utsname03` |
| VFS parent symlink / rmdir errno | `mkdirat02`, `rmdir02` RV + LA × musl + glibc parser-clean; adjacent stable `mkdir/rmdir/unlink/symlink/mknod/rename` subset clean on RV+LA | `mkdir09` moved to the futex bitset candidate lane below; keep remaining `mknod07` and `mknodat02` rows blocked until their visible `TFAIL/TBROK/TCONF` causes are fixed. `unlink09` moved to the FS_IOC inode-flag candidate lane below. |
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
| credentials/capabilities | RV 16-bit UID/cap rows `TCONF`; earlier glibc `gettid02` `TBROK` futex abort | 16-bit/cap rows still need unsupported-ABI policy or real capability work; `gettid02` is superseded by the later futex/glibc follow-up evidence below. |
| VFS/FD/select scout | RV scout has select pass-with-TCONF rows, fcntl17/fcntl17_64 timeouts, VFS path errno TFAIL/TBROK, and zero candidates | Split into isolated fixes; no broad VFS/FD/select promotion from this evidence. |


## Additional VFS regression boundary

The parent-symlink/rmdir repair is protected by a 36-case adjacent stable subset on both architectures (`72 PASS / 0 FAIL` for RV and `72 PASS / 0 FAIL` for LA). This covers existing stable mkdir/mkdirat, rmdir, unlink/unlinkat, symlink/symlinkat, mknod/mknodat, rename, and renameat rows. Future edits in `FdTable::mkdirat`, `FdTable::mknodat`, `FdTable::unlinkat`, `sys_symlinkat`, or process mountpoint checks should rerun this subset before promotion.

## mkdir setgid/final-symlink regression boundary

The `mkdir02`/`mkdir03` repair is protected by targeted RV + LA × musl + glibc evidence for `mkdir02`, `mkdir03`, `mkdirat02`, and `rmdir02` (`16 PASS / 0 FAIL` across the two architecture logs) plus a 35-case adjacent stable subset on both architectures (`70 PASS / 0 FAIL` for RV and `70 PASS / 0 FAIL` for LA).

Covered stable adjacency includes chmod/fchmod/fchmodat, chown/fchown/fchownat, `open10`, `creat08`, `creat09`, mkdir/mkdirat, mknod/mknodat, symlink/symlinkat, and rmdir rows. Future edits in `clear_path_chown_special_bits`, `record_created_path_metadata`, `FdTable::mkdirat`, or `FdTable::mknodat` should rerun this subset before promotion.

## fcntl read-lease regression boundary

The `fcntl27`/`fcntl27_64` repair is protected by targeted RV + LA × musl + glibc evidence for `fcntl27` and `fcntl27_64` (`8 PASS / 0 FAIL` across the two targeted candidate pairs) plus all current stable `fcntl*` rows and `fcntl27` on both architectures (`98 PASS / 0 FAIL` for RV and `98 PASS / 0 FAIL` for LA). `fcntl27_64` required no extra source change beyond the same generic lease access rule.

Covered stable adjacency includes `F_DUPFD`, `F_GETFD`/`F_SETFD`, `F_GETFL`/`F_SETFL`, record locking (`F_GETLK`, `F_SETLK`, `F_SETLKW`), lease read-only success (`fcntl23`, `fcntl23_64`), and OFD/lock stress rows. Future edits in `FdTable::fcntl`, `fcntl_getlease`, or `fcntl_setlease` should rerun this subset before promotion.



## symlink03 tmpdir/parent-permission regression boundary

The `symlink03` repair is protected by targeted RV + LA × musl + glibc evidence (`4 PASS / 0 FAIL` across the two targeted architecture logs) plus a 20-case adjacent stable symlink/access/readlink/link/unlink/rmdir/mkdir subset on both architectures (`40 PASS / 0 FAIL` for RV and `40 PASS / 0 FAIL` for LA).

Covered stable adjacency includes `access*`, `faccessat*`, representative `chmod*`, `symlink*`, `symlinkat01`, `readlink*`, `link*`, `unlinkat01`, `rmdir01`, and `mkdir04`. Future edits in `initial_path_modes`, `check_parent_write_search_permission`, `sys_symlinkat`, or path-mode/chmod permission handling should rerun this subset before promotion.

## unlink09 FS_IOC inode-flag regression boundary

The `unlink09` repair is protected by targeted RV + LA × musl + glibc evidence (`4 PASS / 0 FAIL` across the two targeted architecture logs) plus a 23-case adjacent stable unlink/access/symlink/readlink/link/rmdir/mkdir subset on both architectures (`46 PASS / 0 FAIL` for RV and `46 PASS / 0 FAIL` for LA).

Covered stable adjacency includes `access*`, `faccessat*`, representative `chmod*`, `symlink*`, `symlinkat01`, `readlink*`, `link*`, `unlink05`, `unlink07`, `unlinkat01`, `rmdir01`, and `mkdir04`, with `unlink09` included as the newly clean candidate row. Future edits in `sys_ioctl`, `path_inode_flags` metadata, `move_path_metadata`, or `FdTable::unlinkat` should rerun this subset before promotion.

## mkdir09 futex bitset regression boundary

The `mkdir09` repair is protected by targeted RV + LA × musl + glibc evidence (`4 PASS / 0 FAIL` across the two targeted architecture logs), the later `futex_wait_bitset01` four-combo follow-up evidence, plus an 11-case futex/clone adjacent stable subset on both architectures (`22 PASS / 0 FAIL` for RV and `22 PASS / 0 FAIL` for LA).

Covered stable adjacency includes `futex_wait01` through `futex_wait05`, `futex_wake01`, `futex_wake03`, and representative `clone01`, `clone03`, `clone06`, and `clone07` process/thread boundaries. Future edits in `sys_futex`, futex timeout conversion, futex keying/wake behavior, or process teardown wakeups should rerun this subset before promotion.

## gettid02 futex/glibc follow-up regression boundary

`gettid02` is protected by targeted RV + LA × musl + glibc evidence (`4 PASS / 0 FAIL` across the two targeted architecture logs). No source change was added after the futex bitset patch, so the code-regression boundary remains the futex/clone adjacent subset already run for the `mkdir09` repair (`22 PASS / 0 FAIL` on RV and `22 PASS / 0 FAIL` on LA).

Future changes to `sys_futex`, thread teardown, pthread join compatibility, or `gettid`/TID allocation must rerun `gettid02` plus the futex/clone adjacent subset before counting this candidate toward a stable milestone.


## futex_wait_bitset01 follow-up regression boundary

`futex_wait_bitset01` is protected by RV + LA × musl + glibc targeted evidence (`4 PASS / 0 FAIL` across the RV futex scout row and the LA follow-up log). No additional source changed after the generic futex bitset patch, so the code-regression boundary remains `sys_futex`, futex timeout conversion, futex keying/wake behavior, and process/thread teardown wakeups.

The same RV scout keeps `futex_wake02`, `futex_wake04`, `futex_cmp_requeue01`, and `futex_cmp_requeue02` out of the candidate pool because their evidence has visible `TBROK`/`TCONF` markers. The RV clone and FD/vector-IO scouts are also blocker-only. Future work on selective wake, requeue, clone flags, or vector I/O must rerun their targeted rows plus the stable futex/clone or FD adjacent subset before promotion.


## fstat02/fstat02_64 evidence-only regression boundary

`fstat02` and `fstat02_64` are protected by RV + LA × musl + glibc targeted evidence (`8 PASS / 0 FAIL` across the RV FD/path scout rows and the LA follow-up log). No source changed in this follow-up, so the code-regression boundary remains the existing `fstat(2)` metadata path, path-backed file descriptor lookup, and stat struct copy-out semantics.

The same RV FD/path scout keeps `close_range01`, `close_range02`, `getcwd03`, `getcwd04`, `openat03`, `openat04`, `open14`, and `creat07` out of the candidate pool because their evidence has visible `TCONF`, `TFAIL`, `TBROK`, or `ENOSYS` markers. The RV VFS/MM, LA `mmap05`, process/exec/signal, and exec-only scouts are also blocker-only. Future edits in `stat`/`fstat` metadata, FD lookup, or user-pointer stat copy-out should rerun `fstat02`, `fstat02_64`, and a representative adjacent stable stat/fstat subset before promotion.


## sync/fd/io and xattr blocker boundary

The RV sync/fd/io scout remains blocker-only and adds no regression-protected candidates. The RV xattr scout was blocker-only by itself; only `setxattr03` became a candidate after the later generic immutable/append-only mutation guard and fresh four-combo evidence. Future work on filesystem sync support, `sync_file_range`, `SEEK_DATA`/`SEEK_HOLE`, FIFO nonblocking open, device/special-file creation, or remaining xattr rows must first remove the visible `TCONF/TFAIL/TBROK/ENOSYS` markers, then rerun the targeted rows on RV + LA × musl + glibc plus adjacent stable sync/xattr subsets before promotion.

## setxattr03 immutable/append-only xattr regression boundary

The `setxattr03` repair is protected by targeted RV + LA × musl + glibc evidence (`4 PASS / 0 FAIL` across the two targeted architecture logs) plus a 21-case adjacent stable xattr subset on both architectures (`42 PASS / 0 FAIL` for RV and `42 PASS / 0 FAIL` for LA).

Covered stable adjacency includes existing `fgetxattr*`, `flistxattr*`, `fremovexattr*`, `fsetxattr01`, `getxattr01`, `lgetxattr*`, `listxattr*`, `llistxattr*`, `lremovexattr01`, `removexattr*`, and `setxattr01` rows. Future edits in `sys_setxattr_for_path`, `sys_removexattr_for_path`, `set_path_xattr`, `remove_path_xattr`, or `path_inode_flags` should rerun `setxattr03` plus this stable xattr subset before promotion.


## xattr special-node / AF_UNIX pathname socket regression boundary

The `fgetxattr02`/`getxattr02`/`setxattr02` repair is protected by targeted RV + LA × musl + glibc evidence (`12 PASS / 0 FAIL` across the two targeted architecture logs) plus a 37-case adjacent xattr/mknod/socket stable subset on both architectures (`74 PASS / 0 FAIL` for RV and `74 PASS / 0 FAIL` for LA).

Covered stable adjacency includes existing `fgetxattr*`, `flistxattr*`, `fremovexattr*`, `fsetxattr01`, `getxattr01`, `lgetxattr*`, `listxattr*`, `llistxattr*`, `lremovexattr01`, `removexattr*`, `setxattr01`, representative `mknod*`/`mknodat01`, and socket creation/option/socketpair rows. Future edits in special inode metadata, `FdTable::mknodat`, `FdTable::openat`, xattr mutation guards, or AF_UNIX pathname `bind()` should rerun these three targeted cases plus this adjacent subset before promotion.

## 2026-06-04 late actual-bin blocker reprobes regression boundary

These reprobes add no new candidate-regression boundary because they were blocker-only. Future work should treat them as focused blocker maps:

| Area | Current blocker evidence | Required before reconsidering promotion |
| --- | --- | --- |
| FD/VFS/IO late reprobe | `0 PASS / 26 FAIL`, missing-current-bin/status `-1` rows, O_TMPFILE `TCONF`, FIFO `TBROK` | Use current guest-bin names only; fix generic semantics, then rerun RV + LA × musl + glibc before counting. |
| fcntl uncovered rows | `0 PASS / 44 FAIL`, `TCONF/TFAIL/TBROK` in lease/OFD/dnotify/cap rows | Implement real feature/errno behavior or document unsupported scope; rerun targeted fcntl rows plus existing stable fcntl subset. |
| process/time/signal late reprobe | `10 PASS / 38 FAIL`, pass-with-internal-marker rows, 4 timeouts | Fix signal/process/session/rusage/prctl semantics and timeout roots first; wrapper PASS with internal markers remains disallowed. |

## 2026-06-04 epoll/eventfd/poll/pselect RV scout regression boundary

The scout adds adjacent RV regression evidence for existing eventfd/poll/pselect stable rows, but no new candidate-regression boundary.

| Area | Current evidence | Promotion boundary |
| --- | --- | --- |
| eventfd/poll/pselect stable adjacency | Existing stable rows are RV parser-clean in the mixed scout | Useful as RV adjacency only; any new promotion still needs fresh RV + LA × musl + glibc clean evidence for new unique rows. |
| epoll_create01/02 | pass-with-TCONF and musl TFAIL remain visible | Implement real raw `epoll_create`/invalid-size semantics or document unsupported scope before reconsidering; no wrapper-specific workaround. |
| eventfd06 | `libaio` unavailable `TCONF` | Requires real AIO/libaio test support or upstream environment change; TCONF is not promotion evidence. |

## splice01..splice05 generic splice(2) regression boundary

`splice01`, `splice02`, `splice03`, `splice04`, and `splice05` are protected by fresh RV + LA × musl + glibc targeted evidence (`20 PASS / 0 FAIL / 0 internal markers` across the two architecture logs). The regression boundary covers `sys_splice`, `FdTable::read`, `FdTable::write`, regular-file current-offset advancement, pipe read/write availability, AF_UNIX `LocalSocketEntry` read/write behavior, optional user offset copy-in/copy-out, and invalid-fd errno ordering.

Future edits to `sys_splice`, pipe semantics, AF_UNIX local sockets, regular-file offset/write paths, `O_APPEND` handling, or syscall dispatch should rerun `splice01`..`splice05` on RV + LA × musl + glibc before counting these cases in a stable milestone. `splice06`/`splice07`/`splice08`/`splice09` remain blocker-only and must not be promoted until their visible `TCONF/ENOSYS` or version-gate markers are removed by real semantics.
