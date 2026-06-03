# milestone-06 stable806 interim report

Date: 2026-06-03.

## Target

Move the live baseline from stable756 toward the next stable806 milestone without fake pass behavior. This is an interim candidate-pool checkpoint, not a stable-list promotion.

## Changes in this checkpoint

- Added real timerslack behavior in the prior checkpoint and kept `prctl08`/`prctl09` as clean candidates.
- Repaired default UTS hostname sharing by making plain `fork()` children share the same hostname object.
- Documented a partial RV socket-core scout as blocker-only evidence.
- Documented blocker triage for `readlink03`/`readlinkat02`, `nice04`, RV statx rows, and RV credential/capability rows; no source workaround was made for semantically unsafe cases.
- Documented RV VFS/FD/select scout as blocker-only evidence with zero candidates.
- Repaired generic VFS parent-symlink resolution and `rmdir` errno boundaries, making `mkdirat02` and `rmdir02` four-combo clean candidates.
- Repaired directory `chown`/setgid preservation and final-component symlink existence checks for `mkdirat`/`mknodat`, making `mkdir02` and `mkdir03` four-combo clean candidates.
- Repaired `F_SETLEASE` read-lease access semantics so write-open file descriptors fail with `EAGAIN`, making `fcntl27` and same-source `fcntl27_64` four-combo clean candidates while preserving existing read-only lease rows.
- Seeded Linux-like `01777` metadata for `/tmp` and `/tmp/ltp-work`, then made `symlinkat` use the generic parent write/search/type permission gate before recording synthetic symlinks; this makes `symlink03` four-combo clean without hardcoding the case or path.
- Added generic `FS_IOC_GETFLAGS`/`FS_IOC_SETFLAGS` handling plus immutable/append-only unlink `EPERM` checks; this makes `unlink09` four-combo clean without hardcoding the case or output.
- Added generic `FUTEX_WAIT_BITSET`/`FUTEX_WAKE_BITSET` support; this makes glibc `mkdir09` four-combo clean without hardcoding the case or output.
- Confirmed the same futex bitset/glibc pthread repair lane also makes `gettid02` and `futex_wait_bitset01` four-combo clean; no extra source change was needed for either follow-up.
- Documented RV futex wake/requeue, clone, and FD/vector-IO scouts as blocker-only evidence; no partial PASS, `TCONF`, or glibc-only row was counted.
- Documented late RV VFS/MM, process/exec/signal, exec-only, and FD/path scouts. Only `fstat02` and `fstat02_64` reached RV + LA × musl + glibc parser-clean; `mmap05`, close-range/O_TMPFILE/getcwd/creat, kill/process, and exec rows remain blocker-only.
- Documented RV sync/fd/io and xattr small scouts, then repaired the generic immutable/append-only xattr mutation gap so `setxattr03` is now four-combo clean. Remaining sync/fd/io and xattr scout rows stay blocker-only with visible parser markers.
- Did not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.

## Candidate-pool status

Current new unique stable806 candidates:

1. `prctl08`
2. `prctl09`
3. `utsname02`
4. `mkdirat02`
5. `rmdir02`
6. `mkdir02`
7. `mkdir03`
8. `fcntl27`
9. `fcntl27_64`
10. `symlink03`
11. `unlink09`
12. `mkdir09`
13. `gettid02`
14. `futex_wait_bitset01`
15. `fstat02`
16. `fstat02_64`
17. `setxattr03`

`utsname01` is clean in the UTS targeted run but is already stable, so it is only adjacent regression evidence.

## Evidence

- UTS targeted RV: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.summary.txt` — `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- UTS targeted LA: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.summary.txt` — same clean result.
- Combined UTS candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-utsname-shared-hostname-20260603T190408+0800.promotion-candidates.txt` — four-combo candidates `utsname01`, `utsname02`; only `utsname02` is new unique.
- Adjacent RV UTS/hostname/uname regression: `target/ltp-1000-milestone-06-stable806/rv-utsname-adjacent-regression-20260603T190435+0800.summary.txt` — `20 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA UTS/hostname/uname regression: `target/ltp-1000-milestone-06-stable806/la-utsname-adjacent-regression-20260603T190701+0800.summary.txt` — `20 PASS / 0 FAIL / 0 internal markers`.
- Readlink near-clean triage: RV summary clean, LA summary `2 PASS / 2 FAIL / 2 TFAIL`; combined report has 0 candidates.
- RV statx scout: `2 PASS / 18 FAIL / 32 TCONF / 2 timeout`; 0 candidates.
- RV credential/capability scout: `1 PASS / 23 FAIL / 22 TCONF / 1 TBROK`; 0 candidates.
- RV VFS/FD/select scout: `9 PASS / 45 FAIL / 112 TCONF / 26 TFAIL / 7 TBROK / 4 timeout`; 0 candidates.

- VFS parent-symlink/rmdir targeted RV: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.summary.txt` — `5 PASS / 13 FAIL`, with new clean RV candidates `mkdirat02` and `rmdir02`; remaining rows still have visible `TFAIL/TBROK/TCONF` and are excluded.
- VFS parent-symlink/rmdir targeted LA: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.summary.txt` — `4 PASS / 0 FAIL / 0 internal markers` for `mkdirat02` and `rmdir02`.
- Combined VFS candidate report: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.combined-promotion-candidates.txt` — four-combo candidates `mkdirat02`, `rmdir02`.
- Adjacent RV VFS regression: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.txt` — `72 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA VFS regression: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.txt` — `72 PASS / 0 FAIL / 0 internal markers`.

- mkdir setgid/final-symlink targeted RV: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.txt` — `8 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- mkdir setgid/final-symlink targeted LA: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.txt` — same clean `8 PASS / 0 FAIL / 0 internal markers` result.
- Combined mkdir candidate report: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.combined-promotion-candidates.txt` — four-combo candidates `mkdir02`, `mkdir03`, `mkdirat02`, and `rmdir02`; only `mkdir02` and `mkdir03` are new unique candidates in this checkpoint.
- Adjacent RV metadata/VFS regression: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.txt` — `70 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA metadata/VFS regression: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.txt` — `70 PASS / 0 FAIL / 0 internal markers`.

- RV VFS/FD isolation scout: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-isolation-scout-20260603T211800+0800.summary.txt` — `1 PASS / 5 FAIL`, initially confirming `symlink03` and glibc `mkdir09` as blocker rows while `fcntl27` was a repairable lease-access row. The later `symlink03` repair below supersedes the `symlink03` blocker evidence; `mkdir09` is superseded by the later futex bitset repair evidence below.
- fcntl27 targeted RV: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- fcntl27 targeted LA: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.txt` — same clean `2 PASS / 0 FAIL / 0 internal markers` result.
- Combined fcntl27 candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.combined-promotion-candidates.txt` — four-combo candidate `fcntl27`.
- fcntl27_64 targeted RV: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- fcntl27_64 targeted LA: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.txt` — same clean `2 PASS / 0 FAIL / 0 internal markers` result.
- Combined fcntl27_64 candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.combined-promotion-candidates.txt` — four-combo candidate `fcntl27_64`.
- Adjacent RV fcntl regression: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.txt` — `98 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA fcntl regression: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.txt` — `98 PASS / 0 FAIL / 0 internal markers`.
- symlink03 diagnostic after tmp-mode seed only: `target/ltp-1000-milestone-06-stable806/rv-symlink03-initial-tmp-mode-fix-20260603T212433+0800.summary.txt` — `0 PASS / 2 FAIL / TFAIL=4`; this is blocker/root-cause evidence only and is not promotion evidence.
- symlink03 targeted RV after parent-permission repair: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- symlink03 targeted LA after parent-permission repair: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.summary.txt` — same clean `2 PASS / 0 FAIL / 0 internal markers` result.
- Combined symlink03 candidate report: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.combined-promotion-candidates.txt` — four-combo candidate `symlink03`.
- Adjacent RV symlink/path-permission regression: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-adjacent-regression-20260603T213226+0800.summary.txt` — `40 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA symlink/path-permission regression: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-adjacent-regression-20260603T213538+0800.summary.txt` — `40 PASS / 0 FAIL / 0 internal markers`.


- unlink09 pre-fix diagnostic: `target/ltp-1000-milestone-06-stable806/rv-unlink09-after-symlink03-perms-20260603T215126+0800.summary.txt` — `0 PASS / 2 FAIL / TBROK=2`, not promotion evidence.
- unlink09 targeted RV after FS_IOC inode-flag repair: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- unlink09 targeted LA after FS_IOC inode-flag repair: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.summary.txt` — same clean `2 PASS / 0 FAIL / 0 internal markers` result.
- Combined unlink09 candidate report: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.combined-promotion-candidates.txt` — four-combo candidate `unlink09`.
- Adjacent RV unlink/path-permission regression: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.txt` — `46 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA unlink/path-permission regression: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.txt` — `46 PASS / 0 FAIL / 0 internal markers`.

- mkdir09 pre-fix diagnostic: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-current-retest-20260603T222025+0800.summary.txt` — `1 PASS / 1 FAIL / TBROK=1`, not promotion evidence.
- mkdir09 targeted RV after futex bitset repair: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- mkdir09 targeted LA after futex bitset repair: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.summary.txt` — same clean `2 PASS / 0 FAIL / 0 internal markers` result.
- Combined mkdir09 candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-mkdir09-futex-bitset-fix-promotion-candidates.txt` — four-combo candidate `mkdir09`.
- Adjacent RV futex/clone regression: `target/ltp-1000-milestone-06-stable806/rv-futex-bitset-adjacent-regression-20260603T222822+0800.summary.txt` — `22 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA futex/clone regression: `target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.summary.txt` — `22 PASS / 0 FAIL / 0 internal markers`.
- gettid02 targeted RV after futex bitset repair: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- gettid02 targeted LA after futex bitset repair: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.summary.txt` — same clean `2 PASS / 0 FAIL / 0 internal markers` result.
- Combined gettid02 candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-gettid02-after-futex-bitset-20260603T224549+0800.promotion-candidates.txt` — four-combo candidate `gettid02`.
- RV futex adjacent scout: `target/ltp-1000-milestone-06-stable806/rv-futex-adjacent-scout-20260603T225625+0800.summary.txt` — `2 PASS / 8 FAIL`, `TBROK=2`, `TCONF=6`; only `futex_wait_bitset01` is RV-clean, while wake/requeue rows remain blocker-only.
- LA futex_wait_bitset01 follow-up: `target/ltp-1000-milestone-06-stable806/la-futex-wait-bitset01-followup-20260603T225741+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined futex_wait_bitset01 candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-futex-wait-bitset01-followup-20260603T225741+0800.promotion-candidates.txt` — four-combo candidate `futex_wait_bitset01`.
- RV clone adjacent scout: `target/ltp-1000-milestone-06-stable806/rv-clone-adjacent-scout-20260603T225857+0800.summary.txt` — `1 PASS / 9 FAIL`, with visible `TFAIL/TBROK/ENOSYS`; zero candidates.
- RV FD/vector-IO scout: `target/ltp-1000-milestone-06-stable806/rv-fd-vector-io-scout-20260603T225958+0800.summary.txt` — `0 PASS / 18 FAIL`, `TCONF=18`; zero candidates.
- RV VFS/MM small scout: `target/ltp-1000-milestone-06-stable806/rv-vfs-mm-small-scout-20260603T230922+0800.summary.txt` — `4 PASS / 26 FAIL`, `TCONF=22`, `TFAIL=21`, `ENOSYS=2`; only `mmap05` was RV-clean, but LA follow-up failed.
- LA mmap05 follow-up: `target/ltp-1000-milestone-06-stable806/la-mmap05-followup-20260603T231053+0800.summary.txt` — `0 PASS / 2 FAIL`, `TFAIL=2`; `mmap05` is excluded.
- RV process/exec/signal scout: `target/ltp-1000-milestone-06-stable806/rv-process-exec-signal-scout-20260603T231200+0800.summary.txt` — `1 PASS / 3 FAIL` before an allocator panic marker; zero candidates and `kill10` must be isolated before reuse.
- RV exec-only scout: `target/ltp-1000-milestone-06-stable806/rv-exec-small-scout-20260603T231306+0800.summary.txt` — `2 PASS / 18 FAIL`, with visible `TBROK/TFAIL`; zero candidates.
- RV FD/path small scout: `target/ltp-1000-milestone-06-stable806/rv-fd-path-small-scout-20260603T231708+0800.summary.txt` — `4 PASS / 16 FAIL`, with clean RV rows for `fstat02` and `fstat02_64`; close_range, getcwd, O_TMPFILE/openat/open14, and `creat07` remain blocked.
- LA fstat02 follow-up: `target/ltp-1000-milestone-06-stable806/la-fstat02-followup-20260603T231936+0800.summary.txt` — `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined fstat02/fstat02_64 candidate report: `target/ltp-1000-milestone-06-stable806/combined-fstat02-fourway-20260603T232030+0800.promotion-candidates.txt` — four-combo candidates `fstat02` and `fstat02_64`.
- RV sync/fd/io scout: `target/ltp-1000-milestone-06-stable806/rv-sync-fd-io-scout-20260603T232921+0800.summary.txt` — `0 PASS / 20 FAIL`, `TCONF=14`, `TFAIL=6`, `TBROK=4`, `ENOSYS=2`; zero candidates.
- RV xattr small scout: `target/ltp-1000-milestone-06-stable806/rv-xattr-small-scout-20260603T233055+0800.summary.txt` — `0 PASS / 16 FAIL`, `TBROK=6`, `TCONF=8`, `TFAIL=4`; zero candidates from the scout by itself.
- setxattr03 targeted RV after immutable/append-only xattr mutation repair: `target/ltp-1000-milestone-06-stable806/rv-setxattr03-followup-20260603T234026+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- setxattr03 targeted LA after immutable/append-only xattr mutation repair: `target/ltp-1000-milestone-06-stable806/la-setxattr03-followup-20260603T234111+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined setxattr03 candidate report: `target/ltp-1000-milestone-06-stable806/combined-setxattr03-fourway-20260603T234153+0800.promotion-candidates.txt` — four-combo candidate `setxattr03`.
- Adjacent RV xattr stable regression: `target/ltp-1000-milestone-06-stable806/rv-xattr-stable-regression-20260603T234206+0800.summary.txt` — `42 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA xattr stable regression: `target/ltp-1000-milestone-06-stable806/la-xattr-stable-regression-20260603T234337+0800.summary.txt` — `42 PASS / 0 FAIL / 0 internal markers`.

## Risks and boundaries

- `CLONE_NEWUTS` and `unshare(CLONE_NEWUTS)` are still not implemented; `utsname03` remains blocked and is not counted.
- The shared hostname object models only the default shared UTS namespace. It is not a full namespace registry.
- Socket scout rows remain visibly blocked or incomplete and cannot be promoted.
- `readlink03`/`readlinkat02` remain blocked on LA musl wrapper behavior; rejecting all one-byte buffers in-kernel is not acceptable.
- `nice04` remains blocked on libc-visible errno differences around priority lowering; do not risk stable `setpriority` rows with a wrapper-specific kernel special case.
- Statx, 16-bit UID, capability, futex wake/requeue, clone, FD/vector-IO, sync/fd/io, remaining xattr rows beyond `setxattr03`, mmap protection, O_TMPFILE/openat, close_range/capability, getcwd deleted-directory setup, creat checkpoint, process/kill, and exec rows remain blocker-only until real semantics improve; glibc `gettid02` is superseded by the futex/glibc follow-up evidence above.
- The VFS/FD/select scout was split: `mkdir02`, `mkdir03`, `mkdirat02`, `rmdir02`, `fcntl27`, `fcntl27_64`, `symlink03`, and `unlink09` are repaired and candidate-clean, while the remaining select/mknod rows still have TCONF/timeout/TFAIL/TBROK blockers.
- Timerslack/prctl adjacent stable regression still needs to be included before any eventual stable806 promotion commit.
- The inode-flag implementation is in-memory process metadata, not persistent filesystem inode state; future broader FS_IOC work must preserve generic Linux errno/flag semantics and avoid LTP-specific branches.

## Conclusion

This checkpoint improves UTS, VFS path/errno, metadata inheritance, fcntl lease, symlink parent-permission, FS_IOC inode-flag/unlink errno, futex bitset, and immutable/append-only xattr mutation semantics, then adds evidence-only FD metadata discovery for `fstat02`/`fstat02_64`. It brings the stable806 candidate pool to 17 unique cases (`prctl08`, `prctl09`, `utsname02`, `mkdirat02`, `rmdir02`, `mkdir02`, `mkdir03`, `fcntl27`, `fcntl27_64`, `symlink03`, `unlink09`, `mkdir09`, `gettid02`, `futex_wait_bitset01`, `fstat02`, `fstat02_64`, `setxattr03`). The blocker triage intentionally avoids unsafe mmap, readlink, nice, O_TMPFILE, close_range, sync/fd/io, remaining xattr, kill/process, and exec workarounds. Baseline remains `756 total / 756 unique / 0 duplicate`; no stable-list milestone promotion commit is created until the next +50 unique clean cohort is available.
