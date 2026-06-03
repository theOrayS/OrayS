# stable756 promotion candidates

## Promoted new unique cases

The stable list advances from 706 to 756 by adding exactly 50 new unique cases. Each promoted row has targeted RV and LA evidence across both `ltp-musl` and `ltp-glibc` with wrapper PASS and no parser-visible internal failure in that targeted evidence.

See `targeted-cases.txt` for the authoritative case list.

## Evidence groups

| Group | New cases | Evidence |
| --- | ---: | --- |
| eventfd/epoll | 25 | `combined-eventfd-epoll-pipe-kconfig-pwait-clean25-promotion.md` reports 29 clean RV+LA candidates; `pipe01`, `pipe06`, `pipe2_01`, and `poll01` are already-stable regressions, leaving 25 new eventfd/epoll cases. `epoll_wait04` final repair evidence is `rv-epoll_wait04-after-pipe-poll-atomic-20260603T142956+0800.log`; final full RV/LA stable756 gates are clean. |
| timerfd/signalfd | 8 | RV logs `rv-timerfd-signalfd-readlink-after-fd-impl-20260603T020142+0800.log` plus `rv-signalfd01-timerfd-settime02-after-signalfd-return-20260603T020914+0800.log`; LA log `la-timerfd-signalfd-readlink-clean10-after-fd-impl-20260603T022002+0800.log`. Excludes `readlink03`, `readlinkat02`, and `timerfd_settime02`. |
| link/linkat | 5 | RV logs `rv-linkat-after-hardlink-overlay-20260603T023839+0800.log` and `rv-link08-after-cross-mount-order-20260603T024537+0800.log`; LA log `la-link-clean5-after-hardlink-rename-20260603T030113+0800.log`. Excludes `linkat02`. |
| rename/renameat2 | 5 | `rv-rename-clean5-after-sparse-move-fix-20260603T032354+0800.log` and `la-rename-clean5-after-sparse-move-fix-20260603T033022+0800.log`. |
| pipe size/fcntl | 2 | `rv-fcntl35-pipe-regression-after-pipe-heap-capacity-20260603T035927+0800.log` and `la-fcntl35-pipe-regression-after-pipe-heap-capacity-20260603T040720+0800.log`; `pipe01`, `pipe06`, `pipe2_01` are already-stable regressions. |
| open/creat/waitid | 5 | `rv-open-creat-waitid-clean5-scout-20260603T042820+0800.log` and `la-open-creat-waitid-clean5-20260603T043658+0800.log`. |

## Excluded observations

- `readlink03`, `readlinkat02`: RV wrapper PASS, but LA musl has parser-visible `TFAIL`; not promoted.
- `timerfd_settime02`: RV glibc has `TBROK`; not promoted.
- `linkat02`: requires mkfs/ext2 setup and emits `TCONF`; not promoted.
- `mmap05`, select/getdents/statx/O_TMPFILE/ftruncate exploratory rows: not a four-way parser-clean promotion set.
- Blacklist/SKIP/status0/full-sweep partial TPASS rows are not counted.

## Full stable gate

Final `LTP_CASES=stable` RV/LA gate results are recorded in `validation.md`: both RV and LA report `PASS LTP CASE: 1512`, `FAIL LTP CASE: 0`, `ltp-musl: 756 passed, 0 failed`, and `ltp-glibc: 756 passed, 0 failed`. The inherited `read02` O_DIRECT/tmpfs `TCONF` caveat remains disclosed and is not a new milestone-05 regression.
