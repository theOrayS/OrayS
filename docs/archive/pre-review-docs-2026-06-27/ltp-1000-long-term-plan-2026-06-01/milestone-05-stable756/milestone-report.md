# milestone-05 stable756 report

## Goal

Advance `dev/1000ltp-plan` from stable706 to stable756 by promoting exactly 50 trustworthy unique LTP stable cases, while preserving the no-fake-pass rule and closing full-stable regressions found during the milestone gate.

## Result

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` reports `756 total / 756 unique / 0 duplicate`.
- Promotion set: exactly 50 new unique cases, listed in `targeted-cases.txt` and summarized in `promotion-candidates.md`.
- Final stable gate is RV + LA × musl + glibc wrapper PASS: each architecture reports `ltp-musl: 756 passed, 0 failed` and `ltp-glibc: 756 passed, 0 failed`.
- Parser state is clean for new failures: no `TFAIL`, `TBROK`, timeout, ENOSYS, panic, or trap in the final stable756 gates. The only parser caveat is the inherited `read02` O_DIRECT/tmpfs `TCONF` row on both libcs/arches.
- Team runtime state/mailbox for `complete-dev-1000ltp-c632b4a0` was unavailable (`No team state found`, `leader-fixed.json missing`), so the leader continued in solo mode and kept the leader-owned promotion gate.

## Main changes

- Added generic fd-object support for eventfd, epoll, timerfd, signalfd, and `/dev/zero`, including poll/read/write/fcntl/fork/dup/stat/mmap surfaces needed by the promoted cases.
- Tightened epoll readiness and pipe readiness accounting, including zero-timeout no-ready behavior and atomic buffered-byte tracking for direct pipe poll checks.
- Expanded pipe/FIFO semantics: heap-backed 64 KiB capacity for privileged/default pipes, `F_GETPIPE_SZ`/`F_SETPIPE_SZ`, FIFO `O_RDWR` bidirectional endpoints, and EOF/SIGPIPE readiness separation.
- Added hard-link alias/link-count metadata and `renameat2` `RENAME_NOREPLACE`/`RENAME_EXCHANGE` semantics with cross-mount, readonly, sticky-bit, and sparse-file move handling.
- Added process/signal wait improvements for `waitid` stopped/continued/WNOWAIT events and a signal-set-aware `rt_sigtimedwait` path for libc `sigwait()` users.
- Repaired LA exec/copy stability by using PT_INTERP-derived exec roots when path roots fall back to `/`, flushing TLB after exec address-space replacement, and returning actual regular-file short reads instead of zero-filling unread physical ranges.
- Adjusted visible timer resolution to the observed stable-gate granularity and added synthetic `/proc/sys/kernel/core_pattern` plus kernel config feature files.

## Evidence summary

Key final logs are under `target/ltp-1000-milestone-05-stable756/`:

| Gate | Raw log | Parser summary | Result |
| --- | --- | --- | --- |
| RV full stable756 | `rv-stable756-final-after-pipe-poll-atomic-20260603T143606+0800.log` | `rv-stable756-final-after-pipe-poll-atomic-20260603T143606+0800-summary.txt` | `PASS LTP CASE: 1512`, `FAIL LTP CASE: 0`, internal `{'TCONF': 4}` only from `read02`, timeout/ENOSYS/panic/trap `0`. |
| LA full stable756 | `la-stable756-final-after-pipe-poll-atomic-nontty-20260603T154154+0800.log` | `la-stable756-final-after-pipe-poll-atomic-nontty-20260603T154154+0800-summary.txt` | `PASS LTP CASE: 1512`, `FAIL LTP CASE: 0`, internal `{'TCONF': 4}` only from `read02`, timeout/ENOSYS/panic/trap `0`. |

Targeted lane evidence includes eventfd/epoll, timerfd/signalfd, hard-link/rename, pipe/fcntl, open/creat/waitid, SGID, `/dev/zero`, `clock_gettime04`/`shmt06`, `pipe2_02`, and `epoll_wait04` repair logs. The detailed command/log/checksum matrix is in `validation.md` and `validation-checksums.sha256`.

## Risks and caveats

- Inherited caveat: `read02` still emits O_DIRECT/tmpfs `TCONF` in both libc runs on both architectures. It is wrapper PASS and disclosed, but not treated as a new milestone regression.
- The new fd variants increase fd-table match surface. Future fd operations must update eventfd/epoll/timerfd/signalfd/devzero paths consistently for dup/fork/close/poll/fcntl/stat/mmap.
- Epoll and pipe readiness are shared-state paths; future blocking/wakeup changes must avoid fd-table-lock waits and preserve zero-timeout semantics.
- Hard-link and rename metadata now share alias/lifetime state with sparse-file overlays; future VFS changes must preserve link counts and alias cleanup.
- `clock_getres` now reports conservative 50 ms resolution. This is intentionally visible and should be revisited only with a better timer/scheduler model and fresh LTP evidence.

## Conclusion

Stable756 promotion is accepted: exactly 50 new unique cases were added, full RV and LA stable gates pass across musl and glibc, and parser summaries show no new failure categories beyond the disclosed inherited `read02` TCONF caveat.

## Next step

Milestone-06 stable806 should prioritize another 50-case batch with low shared-state risk, likely VFS metadata/path cleanup, fd/io readiness regressions, and selected time/process rows. Keep full-stable gates mandatory before promotion and continue excluding blacklist/SKIP/status0/full-sweep partial rows.
