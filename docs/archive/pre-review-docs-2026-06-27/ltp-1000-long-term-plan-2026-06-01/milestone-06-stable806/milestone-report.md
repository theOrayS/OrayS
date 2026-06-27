# milestone-06 stable806 promotion report

Date: 2026-06-04.

## Target

Move the live baseline from stable756 to stable806 without fake pass behavior. This is the milestone-06 stable-list promotion record.

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
- Repaired generic special-inode xattr mutation errno, special-device fd opening for synthetic char/block nodes, and AF_UNIX pathname `bind()` filesystem socket-node creation; `fgetxattr02`, `getxattr02`, and `setxattr02` are now four-combo clean candidates with xattr/mknod/socket adjacent regression evidence.
- Added generic `splice(2)` dispatch and conservative pipe/file/AF_UNIX stream transfer semantics; `splice01`..`splice05` are now four-combo clean candidates. `splice06` remains blocked by writable proc-sysfile semantics, and `splice07` is wrapper-PASS only with `TCONF/ENOSYS` from unsupported optional fd fixtures, so neither is counted.
- Added generic `fadvise64` dispatch/errno handling and `FALLOC_FL_KEEP_SIZE` support, making `posix_fadvise02`, `posix_fadvise02_64`, `posix_fadvise04`, `posix_fadvise04_64`, and `fallocate03` four-combo clean candidates.
- Added generic SysV SHM metadata/control coverage (`IPC_INFO`, `SHM_INFO`, `SHM_STAT(_ANY)`, `SHM_RND`/`SHM_REMAP`, attach-count teardown/fork retention, dynamic `/proc/sysvipc/shm`, and `/proc/sys/kernel/shmmax`/`shmall` defaults), making eight new SysV SHM rows four-combo clean while preserving adjacent `shmat04`/`shmdt02`.
- Updated `examples/shell/src/cmd.rs::LTP_STABLE_CASES` from `756 total / 756 unique / 0 duplicate` to `806 total / 806 unique / 0 duplicate`.

## Candidate-pool status

Current new unique stable806 candidates: **50/50**. `examples/shell/src/cmd.rs::LTP_STABLE_CASES` is now `806 total / 806 unique / 0 duplicate`; milestone-06 is promoted to stable806.

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
18. `fgetxattr02`
19. `getxattr02`
20. `setxattr02`
21. `splice01`
22. `splice02`
23. `splice03`
24. `splice04`
25. `splice05`
26. `lseek11`
27. `accept02`
28. `bind01`
29. `bind02`
30. `connect01`
31. `recv01`
32. `recvfrom01`
33. `send01`
34. `sendto01`
35. `bind03`
36. `getsockopt02`
37. `recvmsg01`
38. `posix_fadvise02`
39. `posix_fadvise02_64`
40. `posix_fadvise04`
41. `posix_fadvise04_64`
42. `fallocate03`
43. `shmget02`
44. `shmget03`
45. `shmget04`
46. `shmat02`
47. `shmat03`
48. `shmdt01`
49. `shmctl03`
50. `shmctl04`

`utsname01` and the eventfd/poll/pselect follow-up rows are clean in targeted runs but are already stable, so they are adjacent regression evidence only. Earlier in this file some historical subsections mention smaller pool sizes (20/25/26); those statements are preserved as checkpoint chronology and are superseded by this current 42/50 pool.

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

This checkpoint improves UTS, VFS path/errno, metadata inheritance, fcntl lease, symlink parent-permission, FS_IOC inode-flag/unlink errno, futex bitset, immutable/append-only xattr mutation semantics, and special-inode xattr/AF_UNIX pathname socket handling, then adds evidence-only FD metadata discovery for `fstat02`/`fstat02_64`. It brings the stable806 candidate pool to 25 unique cases (`prctl08`, `prctl09`, `utsname02`, `mkdirat02`, `rmdir02`, `mkdir02`, `mkdir03`, `fcntl27`, `fcntl27_64`, `symlink03`, `unlink09`, `mkdir09`, `gettid02`, `futex_wait_bitset01`, `fstat02`, `fstat02_64`, `setxattr03`, `fgetxattr02`, `getxattr02`, `setxattr02`, `splice01`, `splice02`, `splice03`, `splice04`, `splice05`). The blocker triage intentionally avoids unsafe mmap, readlink, nice, O_TMPFILE, close_range, sync/fd/io, remaining xattr, kill/process, and exec workarounds. Baseline remains `756 total / 756 unique / 0 duplicate`; no stable-list milestone promotion commit is created until the next +50 unique clean cohort is available.


## 2026-06-04 xattr special-node / AF_UNIX pathname socket follow-up

This follow-up converts three previously blocker-only xattr rows into candidate-pool evidence after generic semantics work, not by counting the earlier RV scout.

- Targeted RV: `target/ltp-1000-milestone-06-stable806/rv-xattr-special-node-bind-fix-20260604T000534+0800.summary.txt` — `6 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap` for `fgetxattr02`, `getxattr02`, `setxattr02` across musl + glibc.
- Targeted LA: `target/ltp-1000-milestone-06-stable806/la-xattr-special-node-bind-fix-20260604T000627+0800.summary.txt` — same clean `6 PASS / 0 FAIL / 0 internal markers` result.
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/combined-xattr-special-node-bind-fix-20260604T000627+0800.promotion-candidates.txt` — four-combo candidates `fgetxattr02`, `getxattr02`, and `setxattr02`; blocked/incomplete cases `0`.
- Adjacent RV xattr/mknod/socket regression: `target/ltp-1000-milestone-06-stable806/rv-xattr-special-node-adjacent-regression-20260604T000750+0800.summary.txt` — `74 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA xattr/mknod/socket regression: `target/ltp-1000-milestone-06-stable806/la-xattr-special-node-adjacent-regression-20260604T001000+0800:.summary.txt` — `74 PASS / 0 FAIL / 0 internal markers`.

The candidate pool was 20/50 unique cases at that point; the later generic `splice(2)` follow-up raised the pool to 25/50, and the later `lseek11` follow-up raises the current pool to 26/50. `examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains `756 total / 756 unique / 0 duplicate`; no stable806 promotion commit is made before the next +50 gate.

## 2026-06-04 late actual-bin blocker reprobes

After the remaining xattr blocker retest, three more RV-only scouts were run to avoid relying on stale case names and to map the next blocker surface. They produced no promotion candidates and did not change code or the stable list.

- RV FD/VFS/IO reprobe: `target/ltp-1000-milestone-06-stable806/rv-fd-vfs-io-reprobe-20260604T002533+0800.summary.txt` — `0 PASS / 26 FAIL / TCONF=4 / TBROK=4 / 0 timeout / 0 ENOSYS / 0 panic/trap`; candidate report has `0` candidates.
- RV fcntl actual-bin reprobe: `target/ltp-1000-milestone-06-stable806/rv-fcntl-uncovered-reprobe-20260604T002658+0800.summary.txt` — `0 PASS / 44 FAIL / TCONF=48 / TFAIL=4 / TBROK=8 / 0 timeout / 0 ENOSYS / 0 panic/trap`; candidate report has `0` candidates.
- RV process/time/signal reprobe: `target/ltp-1000-milestone-06-stable806/rv-process-time-signal-reprobe-20260604T002910+0800.summary.txt` — `10 PASS / 38 FAIL / TFAIL=321 / TBROK=12 / TCONF=26 / timeout=4 / 0 ENOSYS / 0 panic/trap`; candidate report has `0` candidates.

Conclusion: these scouts are blocker-only. No LA follow-up was run, no `LTP_STABLE_CASES` update is allowed, and stable806 remained `20/50` candidate-pool cases at that point, baseline `756 total / 756 unique / 0 duplicate`.

## 2026-06-04 epoll/eventfd/poll/pselect RV scout

A small RV scout checked current guest-bin epoll/eventfd/poll/pselect rows after the stale Team cleanup. It produced useful adjacent regression evidence but no new unique stable806 candidates.

- RV summary: `target/ltp-1000-milestone-06-stable806/rv-epoll-eventfd-poll-pselect-scout-20260604T013000+0800.summary.txt` — `37 PASS / 3 FAIL / TCONF=6 / TFAIL=2 / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-epoll-eventfd-poll-pselect-scout-20260604T013000+0800.promotion-candidates.txt` — 17 RV candidates are all existing stable rows; the three new rows (`epoll_create01`, `epoll_create02`, `eventfd06`) are blocked.

Conclusion: no LA follow-up and no stable-list change. stable806 remained `20/50` candidate-pool cases at that point, baseline `756 total / 756 unique / 0 duplicate`.


## 2026-06-04 generic splice(2) follow-up

This follow-up converts the first five current guest-bin `splice*` rows into candidate-pool evidence after a generic syscall implementation, not by counting the earlier `ENOSYS` scout.

- Targeted RV current-code gate: `target/ltp-1000-milestone-06-stable806/rv-splice01-05-gate-20260604T011100+0800.summary.txt` — `10 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap` for `splice01`..`splice05` across musl + glibc.
- Targeted LA current-code gate: `target/ltp-1000-milestone-06-stable806/la-splice01-05-gate-20260604T011154+0800.summary.txt` — same clean `10 PASS / 0 FAIL / 0 internal markers` result.
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/la-splice01-05-gate-20260604T011154+0800.promotion-candidates.txt` — five four-combo candidates: `splice01`, `splice02`, `splice03`, `splice04`, `splice05`; blocked/incomplete cases `0`.
- `splice07` RV retest after conservative invalid-fd errno cleanup: `target/ltp-1000-milestone-06-stable806/rv-splice07-fix-20260604T011013+0800.summary.txt` — wrapper `PASS` on RV but `TCONF=336` and `ENOSYS=336` across optional fd-fixture setup. It is explicitly not promotion evidence.

The candidate pool was 25/50 unique cases at the splice checkpoint; the later `lseek11` follow-up raises the current pool to 26/50. `examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains `756 total / 756 unique / 0 duplicate`; no stable806 promotion commit is made before the next +50 gate.


## 2026-06-04 lseek11 SEEK_DATA/SEEK_HOLE follow-up

This follow-up adds a generic regular-file data/hole map so `lseek(fd, off, SEEK_DATA)` and `lseek(fd, off, SEEK_HOLE)` distinguish written data ranges from sparse holes instead of returning `EINVAL`/`ENOSYS` or treating every zero-filled gap as data.

- Targeted RV: `target/ltp-1000-milestone-06-stable806/rv-lseek11-seek-data-hole-20260604T013358+0800.summary.txt` — `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap` for `lseek11` across musl + glibc.
- Targeted LA: `target/ltp-1000-milestone-06-stable806/la-lseek11-seek-data-hole-20260604T013443+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers` for the same case/libc matrix.
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/la-lseek11-seek-data-hole-20260604T013443+0800.promotion-candidates.txt` — four-combo candidate `lseek11`; blocked/incomplete cases `0`.
- Adjacent RV stable lseek subset: `target/ltp-1000-milestone-06-stable806/rv-lseek-adjacent-regression-20260604T013535+0800.summary.txt` — `8 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA stable lseek subset: `target/ltp-1000-milestone-06-stable806/la-lseek-adjacent-regression-20260604T013626+0800.summary.txt` — `8 PASS / 0 FAIL / 0 internal markers`.

The candidate pool is now 26/50 unique cases (`lseek11` added after `splice01`..`splice05`). The stable list remains `756 total / 756 unique / 0 duplicate`; no stable806 promotion commit is made until the full +50 cohort is available.


## 2026-06-04 socket errno/address candidate follow-up

This follow-up converts nine socket rows into candidate-pool evidence after generic socket errno/address-boundary fixes. It does not edit `LTP_STABLE_CASES` and does not count blocked socket namespace/socketcall or pass-with-`TCONF` rows.

New four-combo candidates from this follow-up:

- `accept02` (LA follow-up after RV was already parser-clean).
- `bind01`, `bind02`, `connect01` (generic AF_INET local-address and privileged-port errno boundaries).
- `recv01`, `recvfrom01` (generic receive flag errno handling).
- `send01`, `sendto01` (generic send flag, UDP size, TCP stream destination/error behavior).
- `bind03` (generic AF_UNIX pathname bind node/existing-bind behavior).

Evidence:

- RV socket basic scout: `target/ltp-1000-milestone-06-stable806/rv-socket-basic-scout-20260604T015858+0800.summary.txt` — `accept02` parser-clean on RV; other socket rows still blocker-only in that scout.
- LA `accept02` follow-up: `target/ltp-1000-milestone-06-stable806/la-accept02-followup-20260604T020823+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers`.
- Combined `accept02` report: `target/ltp-1000-milestone-06-stable806/la-accept02-followup-20260604T020823+0800.combined-promotion-candidates.txt` — four-combo candidate `accept02`.
- RV `bind01`/`bind02`/`connect01`: `target/ltp-1000-milestone-06-stable806/rv-bind-privileged-port-fix-20260604T022349+0800.summary.txt` — `6 PASS / 0 FAIL / 0 internal markers`.
- LA `bind01`/`bind02`/`connect01`: `target/ltp-1000-milestone-06-stable806/la-bind-privileged-port-fix-20260604T022457+0800.summary.txt` — `6 PASS / 0 FAIL / 0 internal markers`.
- Combined report: `target/ltp-1000-milestone-06-stable806/la-bind-privileged-port-fix-20260604T022457+0800.combined-promotion-candidates.txt` — four-combo candidates `bind01`, `bind02`, `connect01`.
- RV `recv01`/`recvfrom01`: `target/ltp-1000-milestone-06-stable806/rv-recv-flags-fix-20260604T022734+0800.summary.txt` — `4 PASS / 0 FAIL / 0 internal markers`.
- LA `recv01`/`recvfrom01`: `target/ltp-1000-milestone-06-stable806/la-recv-flags-fix-20260604T022833+0800.summary.txt` — `4 PASS / 0 FAIL / 0 internal markers`.
- Combined report: `target/ltp-1000-milestone-06-stable806/la-recv-flags-fix-20260604T022833+0800.combined-promotion-candidates.txt` — four-combo candidates `recv01`, `recvfrom01`.
- RV `send01`: `target/ltp-1000-milestone-06-stable806/rv-send01-flags-size-fix-20260604T023249+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers`.
- LA `send01`: `target/ltp-1000-milestone-06-stable806/la-send01-flags-size-fix-20260604T023335+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers`.
- Combined report: `target/ltp-1000-milestone-06-stable806/la-send01-flags-size-fix-20260604T023335+0800.combined-promotion-candidates.txt` — four-combo candidate `send01`.
- RV `sendto01`: `target/ltp-1000-milestone-06-stable806/rv-sendto01-tcp-ignore-dest-20260604T024113+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers`.
- LA `sendto01`: `target/ltp-1000-milestone-06-stable806/la-sendto01-tcp-ignore-dest-20260604T024159+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers`.
- Combined report: `target/ltp-1000-milestone-06-stable806/la-sendto01-tcp-ignore-dest-20260604T024159+0800.combined-promotion-candidates.txt` — four-combo candidate `sendto01`.
- RV `bind03`: `target/ltp-1000-milestone-06-stable806/rv-bind03-unix-bound-path-20260604T024400+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers`.
- LA `bind03`: `target/ltp-1000-milestone-06-stable806/la-bind03-unix-bound-path-20260604T024448+0800.summary.txt` — `2 PASS / 0 FAIL / 0 internal markers`.
- Combined report: `target/ltp-1000-milestone-06-stable806/la-bind03-unix-bound-path-20260604T024448+0800.combined-promotion-candidates.txt` — four-combo candidate `bind03`.

Negative follow-up evidence kept out of promotion:

- LA readlink refresh: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-refresh-20260604T025514+0800.summary.txt` — glibc clean but musl `TFAIL=2`; zero candidates.
- RV socket/epoll low-risk scout: `target/ltp-1000-milestone-06-stable806/rv-socket-epoll-lowrisk-scout-20260604T025727+0800.summary.txt` — `5 PASS / 41 FAIL`, `TCONF=34`, `TBROK=12`, `TFAIL=6`; zero clean candidates. `epoll_create01`, `epoll_create02`, and `setsockopt03` are pass-with-internal markers only.
- RV 16-bit credential scout: `target/ltp-1000-milestone-06-stable806/rv-cred16-scout-20260604T025923+0800.summary.txt` — `0 PASS / 58 FAIL`, all blocker-only `TCONF`; zero candidates.
- RV VFS/time/proc low-risk scout: `target/ltp-1000-milestone-06-stable806/rv-vfs-time-proc-lowrisk-scout-20260604T030139+0800.summary.txt` — `6 PASS / 46 FAIL`, `TFAIL=24`, `TBROK=10`, `TCONF=45`, `timeout=2`, `ENOSYS=2`; zero clean candidates.

At that socket errno/address checkpoint the candidate pool was **35/50**. The later AF_UNIX `SO_PEERCRED`/`recvmsg` follow-up raised it to **37/50**, and the later fadvise64/fallocate follow-up raises the current pool to **42/50**; stable806 remains blocked until at least 8 additional unique four-combo clean cases are found and the full milestone gate is rerun.

## 2026-06-04 AF_UNIX SO_PEERCRED/recvmsg candidate follow-up

This follow-up converts `getsockopt02` and `recvmsg01` from earlier socket blocker/scout rows into current candidate-pool evidence. It does not edit `LTP_STABLE_CASES`; it only records parser-clean evidence for the next stable806 cohort.

New four-combo candidates from this follow-up:

- `getsockopt02` — AF_UNIX pathname stream `listen`/`accept` plus `SO_PEERCRED` peer-credential copy-out.
- `recvmsg01` — AF_UNIX pathname stream connection setup plus minimal `sendmsg`/`recvmsg` bridge sufficient for the generic return/errno checks in this LTP row.

Evidence:

- RV targeted summary: `target/ltp-1000-milestone-06-stable806/rv-afunix-getsockopt02-recvmsg01-20260604T033322+0800-summary.txt` — `4 PASS / 0 FAIL / 0 internal markers`.
- LA targeted summary: `target/ltp-1000-milestone-06-stable806/la-afunix-getsockopt02-recvmsg01-20260604T033757+0800-summary.txt` — `4 PASS / 0 FAIL / 0 internal markers`.
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/afunix-getsockopt02-recvmsg01-promotion-candidates-20260604T034432+0800.txt` — two four-combo candidates; blocked/incomplete `0`.
- RV adjacent socket regression: `target/ltp-1000-milestone-06-stable806/rv-afunix-socket-adjacent-regression-20260604T034559+0800-summary.txt` — `36 PASS / 0 FAIL / 0 internal markers`.
- LA adjacent socket regression: `target/ltp-1000-milestone-06-stable806/la-afunix-socket-adjacent-regression-20260604T035259+0800-summary.txt` — `36 PASS / 0 FAIL / 0 internal markers`.

The candidate pool was **37/50** at the AF_UNIX checkpoint. The later fadvise64/fallocate follow-up raises the current pool to **42/50**; stable806 remains blocked until at least 8 additional unique four-combo clean cases are found and the full milestone gate is rerun. Stable count remains `756 total / 756 unique / 0 duplicate`.

## 2026-06-04 fadvise64/fallocate KEEP_SIZE candidate follow-up

This follow-up adds generic `fadvise64` dispatch/errno behavior and `FALLOC_FL_KEEP_SIZE` handling, then records five new candidate-pool cases. It does not update `LTP_STABLE_CASES` and does not create a stable806 milestone commit because the +50 cohort is still incomplete.

New four-combo candidates:

- `posix_fadvise02`
- `posix_fadvise02_64`
- `posix_fadvise04`
- `posix_fadvise04_64`
- `fallocate03`

Evidence:

- Pre-fix fadvise/fallocate RV scout: `target/ltp-1000-milestone-06-stable806/rv-fadvise-fallocate-scout-20260604T042346+08:00.summary.txt` — zero candidates because all wrapper PASS rows had visible `TCONF` and the other rows had `TFAIL/TBROK/ENOSYS` blockers.
- RV targeted gate: `target/ltp-1000-milestone-06-stable806/rv-fadvise02-04-fallocate03-fix-20260604T043416+0800.summary.txt` — `10 PASS / 0 FAIL / 0 internal markers`.
- LA targeted gate: `target/ltp-1000-milestone-06-stable806/la-fadvise02-04-fallocate03-fix-20260604T043828+0800.summary.txt` — `10 PASS / 0 FAIL / 0 internal markers`.
- Combined RV+LA candidate report: `target/ltp-1000-milestone-06-stable806/fadvise02-04-fallocate03-rv-la-fourway.promotion-candidates.txt` — five candidates; blocked/incomplete `0`.
- Adjacent RV FD/storage regression: `target/ltp-1000-milestone-06-stable806/rv-adjacent-fd-storage-regression-after-fadvise-fallocate-20260604T044511+0800.summary.txt` — `20 PASS / 0 FAIL / 0 internal markers`.
- Adjacent LA FD/storage regression: `target/ltp-1000-milestone-06-stable806/la-adjacent-fd-storage-regression-after-fadvise-fallocate-20260604T044915+0800.summary.txt` — `20 PASS / 0 FAIL / 0 internal markers`.
- SysV shm scout remains blocker-only: `target/ltp-1000-milestone-06-stable806/rv-sysv-shm-small-scout-20260604T041600+0800.summary.txt` — `0 PASS / 26 FAIL`; no LA follow-up.

Current milestone-06 state: candidate pool **42/50**, short by 8; stable list unchanged at `756 total / 756 unique / 0 duplicate`.

## Final stable806 promotion gate (2026-06-04)

Current-code final gate ran the full 50-case cohort on RV and LA, each with musl and glibc wrappers. Parser result is clean on both architectures:

- RV final gate: `target/ltp-1000-milestone-06-stable806/rv-stable806-candidate50-final-gate-20260604T062225+0800.summary.txt` — `100 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA final gate: `target/ltp-1000-milestone-06-stable806/la-stable806-candidate50-final-gate-20260604T062526+0800.summary.txt` — `100 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined report: `target/ltp-1000-milestone-06-stable806/stable806-candidate50-final-gate-rv-la-fourway.promotion-candidates.txt` — `Promotion candidates: 50`, `Blocked/incomplete cases: 0`.
- Stable list check after edit: `806 total / 806 unique / 0 duplicate`.

The final SysV SHM slice contributes the last eight new unique cases: `shmget02`, `shmget03`, `shmget04`, `shmat02`, `shmat03`, `shmdt01`, `shmctl03`, and `shmctl04`. `shmat04` and `shmdt02` were included in the SysV final8+adjacent gate as already-stable regression coverage, not as new candidates.
