# stable756 regression matrix

## Regression focus

| Area | Protected cases | Reason |
| --- | --- | --- |
| epoll/eventfd/poll readiness | `epoll_ctl01..05`, `epoll_pwait01..05`, `epoll_wait01..07`, `eventfd01..05`, `eventfd2_01..03`, `poll01` | New fd readiness objects must not regress basic poll/pipe/event semantics. |
| pipe capacity and status flags | `pipe01`, `pipe06`, `pipe2_01`, `pipe2_02`, `fcntl35`, `fcntl35_64` | `F_GETPIPE_SZ/F_SETPIPE_SZ`, direct pipe readiness, and regular-file short-read repair must preserve already-stable pipe rows. |
| timerfd/signalfd | `timerfd_create01`, `timerfd_gettime01`, `timerfd_settime01`, `timerfd01`, `timerfd02`, `signalfd01`, `signalfd4_01`, `signalfd4_02` | New fd families expose poll/read/fcntl/clock/signal-mask behavior. |
| hard links and rename | `link02`, `link04`, `link05`, `link08`, `linkat01`, `rename09`, `rename12`, `rename13`, `renameat201`, `renameat202` | Path metadata, mount-boundary errno ordering, hard-link aliasing, and `renameat2` flags are shared VFS semantics. |
| device/mmap/open | `open11`, `mmap10` | `/dev/zero` char-device open/stat/read/write/poll and mmap validation must not break stable mmap/open cases. |
| process wait/signals | `waitid07`, `waitid08`, `waitid10` | `WSTOPPED/WCONTINUED/WNOWAIT` events and synthetic `core_pattern` support must not regress wait status accounting. |

## Required gate shape

- New 50: targeted RV + LA x musl + glibc wrapper PASS, parser-clean.
- Adjacent regression: at least the already-stable cases above are rerun on both arches.
- Full stable gate: accepted only if wrapper PASS has no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` beyond the inherited and disclosed `read02` O_DIRECT/tmpfs caveat.

## stable-gate blocker repairs

The final full-stable gate also protects previously dirty rows discovered during milestone-05 validation:

- `clock_gettime04` / `shmt06`: repaired with conservative clock resolution and signal-set-aware `rt_sigtimedwait`; targeted RV and LA summaries are parser-clean.
- `pipe2_02`: repaired on LA by returning actual regular-file short reads and preserving copied helper ELF bytes; targeted RV and LA summaries are parser-clean.
- `epoll_wait04`: repaired on RV musl by direct pipe readiness accounting plus epoll zero-timeout no-ready fast path; targeted RV summary is parser-clean.
- `read02`: still an inherited O_DIRECT/tmpfs `TCONF` wrapper-PASS caveat, disclosed in final full RV/LA summaries and not counted as a new failure.
