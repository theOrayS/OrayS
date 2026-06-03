# milestone-06 promotion candidates so far

These cases are candidate-pool evidence only. They are not yet promoted into `LTP_STABLE_CASES` because milestone-06 still needs the full next 50-case cohort plus adjacent stable regression evidence. Current candidate pool: 20/50 unique cases.

| Case | Evidence | Status |
| --- | --- | --- |
| `prctl08` | RV + LA × musl + glibc targeted parser-clean after timerslack repair | candidate pool |
| `prctl09` | RV + LA × musl + glibc targeted parser-clean after timerslack repair | candidate pool |
| `utsname02` | RV + LA × musl + glibc targeted parser-clean after shared default UTS hostname repair | candidate pool |
| `mkdirat02` | RV + LA × musl + glibc targeted parser-clean after parent-symlink resolution repair | candidate pool |
| `rmdir02` | RV + LA × musl + glibc targeted parser-clean after `rmdir(".")`/mountpoint errno repair | candidate pool |
| `mkdir02` | RV + LA × musl + glibc targeted parser-clean after preserving directory `S_ISGID` across `chown` | candidate pool |
| `mkdir03` | RV + LA × musl + glibc targeted parser-clean after treating a final synthetic symlink as existing for `mkdir`/`mkdirat` | candidate pool |
| `fcntl27` | RV + LA × musl + glibc targeted parser-clean after returning `EAGAIN` for read leases on write-open descriptors | candidate pool |
| `fcntl27_64` | RV + LA × musl + glibc targeted parser-clean from the same generic `F_SETLEASE` read-lease access rule | candidate pool |
| `symlink03` | RV + LA × musl + glibc targeted parser-clean after Linux-like tmpdir metadata seed plus generic symlink parent write/search/type permission checks | candidate pool |
| `unlink09` | RV + LA × musl + glibc targeted parser-clean after generic `FS_IOC_GETFLAGS`/`FS_IOC_SETFLAGS` inode-flag support and immutable/append-only unlink `EPERM` checks | candidate pool |
| `mkdir09` | RV + LA × musl + glibc targeted parser-clean after generic `FUTEX_WAIT_BITSET`/`FUTEX_WAKE_BITSET` support fixed glibc pthread joins | candidate pool |
| `gettid02` | RV + LA × musl + glibc targeted parser-clean after the same futex bitset/glibc pthread repair lane removed the old glibc `TBROK` blocker | candidate pool |
| `futex_wait_bitset01` | RV + LA × musl + glibc targeted parser-clean from the generic futex bitset command surface; no additional source change after the futex bitset repair | candidate pool |
| `fstat02` | RV + LA × musl + glibc targeted parser-clean from the FD/path metadata scout and LA follow-up; no source change in this follow-up | candidate pool |
| `fstat02_64` | RV + LA × musl + glibc targeted parser-clean from the same FD/path metadata evidence; no source change in this follow-up | candidate pool |
| `setxattr03` | RV + LA × musl + glibc targeted parser-clean after generic immutable/append-only xattr mutation `EPERM` guard | candidate pool |
| `fgetxattr02` | RV + LA × musl + glibc targeted parser-clean after generic special-node xattr/read and AF_UNIX pathname bind repair | candidate pool |
| `getxattr02` | RV + LA × musl + glibc targeted parser-clean after special inode xattr mutation rejects `user.*` writes with `EPERM` while get/list stays metadata-only | candidate pool |
| `setxattr02` | RV + LA × musl + glibc targeted parser-clean after generic special inode xattr mutation `EPERM` boundary | candidate pool |

Evidence artifacts:

- RV final log: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.summary.txt`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.promotion-candidates.txt`
- LA final log: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.summary.txt`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.promotion-candidates.txt`


Additional UTS evidence artifacts:

- RV UTS targeted log: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.log`
- RV UTS summary: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.summary.txt`
- LA UTS targeted log: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.log`
- LA UTS summary: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.summary.txt`
- Combined RV+LA UTS candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-utsname-shared-hostname-20260603T190408+0800.promotion-candidates.txt`
- RV UTS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-utsname-adjacent-regression-20260603T190435+0800.summary.txt`
- LA UTS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-utsname-adjacent-regression-20260603T190701+0800.summary.txt`

Note: `utsname01` is four-combo clean in the targeted UTS run but is already present in `LTP_STABLE_CASES`, so it is counted as adjacent regression evidence, not as a new unique candidate.


Additional VFS parent-symlink/rmdir evidence artifacts:

- RV VFS targeted log: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.log`
- RV VFS summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.summary.txt`
- RV VFS candidate report: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.promotion-candidates.txt`
- LA VFS targeted log: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.log`
- LA VFS summary: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.summary.txt`
- Combined RV+LA VFS candidate report: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.combined-promotion-candidates.txt`
- RV VFS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.txt`
- LA VFS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.txt`


Additional mkdir setgid/final-symlink evidence artifacts:

- RV mkdir targeted log: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log`
- RV mkdir summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.txt`
- RV mkdir candidate report: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.promotion-candidates.txt`
- LA mkdir targeted log: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log`
- LA mkdir summary: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.txt`
- LA mkdir candidate report: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.promotion-candidates.txt`
- Combined RV+LA mkdir candidate report: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.combined-promotion-candidates.txt`
- RV metadata/VFS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.txt`
- LA metadata/VFS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.txt`


Additional fcntl27 read-lease evidence artifacts:

- RV VFS/FD isolation scout log: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-isolation-scout-20260603T211800+0800.log`
- RV VFS/FD isolation scout summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-isolation-scout-20260603T211800+0800.summary.txt`
- RV fcntl27 targeted log: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.log`
- RV fcntl27 summary: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.txt`
- RV fcntl27 candidate report: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.promotion-candidates.txt`
- LA fcntl27 targeted log: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.log`
- LA fcntl27 summary: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.txt`
- LA fcntl27 candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.promotion-candidates.txt`
- Combined RV+LA fcntl27 candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.combined-promotion-candidates.txt`
- RV fcntl adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.txt`
- LA fcntl adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.txt`
- RV fcntl27_64 targeted log: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log`
- RV fcntl27_64 summary: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.txt`
- RV fcntl27_64 candidate report: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.promotion-candidates.txt`
- LA fcntl27_64 targeted log: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log`
- LA fcntl27_64 summary: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.txt`
- LA fcntl27_64 candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.promotion-candidates.txt`
- Combined RV+LA fcntl27_64 candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.combined-promotion-candidates.txt`


Additional fstat02/fstat02_64 evidence artifacts:

- RV FD/path small scout log: `target/ltp-1000-milestone-06-stable806/rv-fd-path-small-scout-20260603T231708+0800.log`
- RV FD/path small scout summary: `target/ltp-1000-milestone-06-stable806/rv-fd-path-small-scout-20260603T231708+0800.summary.txt`
- RV FD/path small scout candidate report: `target/ltp-1000-milestone-06-stable806/rv-fd-path-small-scout-20260603T231708+0800.promotion-candidates.txt`
- LA fstat02/fstat02_64 follow-up log: `target/ltp-1000-milestone-06-stable806/la-fstat02-followup-20260603T231936+0800.log`
- LA fstat02/fstat02_64 follow-up summary: `target/ltp-1000-milestone-06-stable806/la-fstat02-followup-20260603T231936+0800.summary.txt`
- LA fstat02/fstat02_64 follow-up candidate report: `target/ltp-1000-milestone-06-stable806/la-fstat02-followup-20260603T231936+0800.promotion-candidates.txt`
- Combined RV+LA fstat candidate report: `target/ltp-1000-milestone-06-stable806/combined-fstat02-fourway-20260603T232030+0800.promotion-candidates.txt`

Additional setxattr03 immutable/append-only xattr evidence artifacts:

- RV setxattr03 targeted log: `target/ltp-1000-milestone-06-stable806/rv-setxattr03-followup-20260603T234026+0800.log`
- RV setxattr03 summary: `target/ltp-1000-milestone-06-stable806/rv-setxattr03-followup-20260603T234026+0800.summary.txt`
- RV setxattr03 candidate report: `target/ltp-1000-milestone-06-stable806/rv-setxattr03-followup-20260603T234026+0800.promotion-candidates.txt`
- LA setxattr03 targeted log: `target/ltp-1000-milestone-06-stable806/la-setxattr03-followup-20260603T234111+0800.log`
- LA setxattr03 summary: `target/ltp-1000-milestone-06-stable806/la-setxattr03-followup-20260603T234111+0800.summary.txt`
- LA setxattr03 candidate report: `target/ltp-1000-milestone-06-stable806/la-setxattr03-followup-20260603T234111+0800.promotion-candidates.txt`
- Combined RV+LA setxattr03 candidate report: `target/ltp-1000-milestone-06-stable806/combined-setxattr03-fourway-20260603T234153+0800.promotion-candidates.txt`
- RV xattr stable regression summary: `target/ltp-1000-milestone-06-stable806/rv-xattr-stable-regression-20260603T234206+0800.summary.txt`
- LA xattr stable regression summary: `target/ltp-1000-milestone-06-stable806/la-xattr-stable-regression-20260603T234337+0800.summary.txt`


Additional fgetxattr02/getxattr02/setxattr02 special-node xattr evidence artifacts:

- RV targeted log: `target/ltp-1000-milestone-06-stable806/rv-xattr-special-node-bind-fix-20260604T000534+0800.log`
- RV targeted summary: `target/ltp-1000-milestone-06-stable806/rv-xattr-special-node-bind-fix-20260604T000534+0800.summary.txt`
- RV targeted candidate report: `target/ltp-1000-milestone-06-stable806/rv-xattr-special-node-bind-fix-20260604T000534+0800.promotion-candidates.txt`
- LA targeted log: `target/ltp-1000-milestone-06-stable806/la-xattr-special-node-bind-fix-20260604T000627+0800.log`
- LA targeted summary: `target/ltp-1000-milestone-06-stable806/la-xattr-special-node-bind-fix-20260604T000627+0800.summary.txt`
- LA targeted candidate report: `target/ltp-1000-milestone-06-stable806/la-xattr-special-node-bind-fix-20260604T000627+0800.promotion-candidates.txt`
- Combined RV+LA candidate report: `target/ltp-1000-milestone-06-stable806/combined-xattr-special-node-bind-fix-20260604T000627+0800.promotion-candidates.txt`
- RV adjacent xattr/mknod/socket regression summary: `target/ltp-1000-milestone-06-stable806/rv-xattr-special-node-adjacent-regression-20260604T000750+0800.summary.txt`
- LA adjacent xattr/mknod/socket regression summary: `target/ltp-1000-milestone-06-stable806/la-xattr-special-node-adjacent-regression-20260604T001000+0800:.summary.txt`

Note: the earlier `rv-xattr-special-node-fix-20260604T000115+0800` run is diagnostic only because `fgetxattr02` still hit an AF_UNIX `bind()`/`ENOTSOCK` `TBROK`, and the `rv-xattr-special-node-bind-fix-20260604T000402+0800` run is build-fail diagnostic only. They are not counted as promotion evidence.

The same RV FD/path scout keeps `close_range01`, `close_range02`, `getcwd03`, `getcwd04`, `openat03`, `openat04`, `open14`, and `creat07` out of the pool because their rows contain visible `TCONF`, `TFAIL`, `TBROK`, or `ENOSYS` markers.

## Explicitly excluded after blocker triage

| Case/lane | Evidence | Exclusion reason |
| --- | --- | --- |
| `readlink03` | RV clean; LA glibc clean; LA musl `TFAIL` | LA musl wrapper passes a one-byte non-null buffer for the nominal zero-size test; kernel cannot reject all `bufsiz=1` without breaking valid readlink semantics. |
| `readlinkat02` | RV clean; LA glibc clean; LA musl `TFAIL` | Same LA musl wrapper boundary as `readlink03`; combined report has 0 candidates. |
| `nice04` | RV glibc clean; RV musl `TFAIL` with `EACCES` instead of `EPERM` | Shared `setpriority` semantics would be endangered by a kernel-only wrapper special case. |
| `statx01,statx04..statx12` | RV scout has `TCONF`, wrapper FAILs, and `statx11` timeouts | Zero RV-only candidates; not safe promotion evidence. |
| `gettid02` pre-futex-bitset scout | RV scout had one musl-only pass and glibc `gettid02` `TBROK` | Superseded by the later RV+LA targeted evidence below after generic futex bitset support; the earlier scout remains blocker-only. |
| `*_16`, `capget*`, `capset*` | RV scout has 16-bit UID/capability `TCONF` rows | Zero RV-only candidates; needs unsupported-ABI/capability lane work before reconsideration. |
| `unlink09`..`select04` VFS/FD/select scout | RV scout has `9 PASS / 45 FAIL`, `TBROK/TCONF/TFAIL`, and four `fcntl17*` timeouts | Zero RV-only candidates from the broad scout; later targeted repairs make `fcntl27`, same-source `fcntl27_64`, `symlink03`, and `unlink09` valid candidates. The broad scout itself and remaining `select*` pass-with-TCONF rows plus timeout/TFAIL/TBROK rows are not promotion evidence. |
| `mkdir09` isolation scout | RV isolation scout after the mkdir repair still had a glibc `mkdir09` futex abort | Superseded by the later futex bitset repair below; the isolation scout remains diagnostic blocker evidence only and is not counted by itself. Earlier `symlink03` blocker rows are superseded by the later parent-permission repair and clean RV/LA evidence above. |
| `futex_wake02`, `futex_wake04`, `futex_cmp_requeue01`, `futex_cmp_requeue02` | RV futex adjacent scout has `TBROK`/`TCONF` visible markers | Blocker-only; only `futex_wait_bitset01` was RV clean and later LA-clean. Requeue/selective wake semantics are not counted from partial scout output. |
| `clone02`, `clone04`, `clone05`, `clone08`, `clone09` | RV clone adjacent scout has `TFAIL`/`TBROK`/`ENOSYS` and only glibc-only `clone04` clean | Zero candidates; no LA follow-up because RV musl/glibc was not both clean. |
| vector IO/sendfile rows | RV FD/vector-IO scout for `writev03`, `preadv03*`, `preadv203*`, `pwritev03*`, `sendfile09*` has `TCONF` for every row | Zero candidates; no pass-with-TCONF promotion. |

Excluded evidence artifacts:

- RV readlink summary: `target/ltp-1000-milestone-06-stable806/rv-readlink03-readlinkat02-20260603T191956+0800.summary.txt`
- LA readlink summary: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.summary.txt`
- Combined readlink report: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.combined-promotion-candidates.txt`
- RV statx summary: `target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.summary.txt`
- RV credential/capability summary: `target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.summary.txt`

Additional excluded VFS/FD/select artifact:

- RV VFS/FD/select summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.summary.txt`
- RV VFS/FD/select candidate report: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.promotion-candidates.txt`



Additional symlink03 parent-permission evidence artifacts:

- RV symlink03 scratch-permission diagnostic summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-ltp-scratch-perms-fix-20260603T211855+0800.summary.txt` (blocker-only)
- RV symlink03 tmp-mode-only diagnostic summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-initial-tmp-mode-fix-20260603T212433+0800.summary.txt` (blocker-only)
- RV futex adjacent scout summary: `target/ltp-1000-milestone-06-stable806/rv-futex-adjacent-scout-20260603T225625+0800.summary.txt` (blocker-only for wake/requeue rows; `futex_wait_bitset01` separately promoted to candidate pool after LA follow-up)
- RV clone adjacent scout summary: `target/ltp-1000-milestone-06-stable806/rv-clone-adjacent-scout-20260603T225857+0800.summary.txt` (blocker-only)
- RV FD/vector-IO scout summary: `target/ltp-1000-milestone-06-stable806/rv-fd-vector-io-scout-20260603T225958+0800.summary.txt` (blocker-only)
- RV symlink03 targeted log: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.log`
- RV symlink03 summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.summary.txt`
- RV symlink03 candidate report: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.promotion-candidates.txt`
- LA symlink03 targeted log: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.log`
- LA symlink03 summary: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.summary.txt`
- LA symlink03 candidate report: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.promotion-candidates.txt`
- Combined RV+LA symlink03 candidate report: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.combined-promotion-candidates.txt`
- RV symlink/path-permission adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-adjacent-regression-20260603T213226+0800.summary.txt`
- LA symlink/path-permission adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-adjacent-regression-20260603T213538+0800.summary.txt`

Additional unlink09 FS_IOC inode-flag evidence artifacts:

- RV unlink09 pre-fix diagnostic summary: `target/ltp-1000-milestone-06-stable806/rv-unlink09-after-symlink03-perms-20260603T215126+0800.summary.txt` — `0 PASS / 2 FAIL / TBROK=2`, not promotion evidence.
- RV unlink09 targeted log: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.log`
- RV unlink09 summary: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.summary.txt`
- RV unlink09 candidate report: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.promotion-candidates.txt`
- LA unlink09 targeted log: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.log`
- LA unlink09 summary: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.summary.txt`
- LA unlink09 candidate report: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.promotion-candidates.txt`
- Combined unlink09 candidate report: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.combined-promotion-candidates.txt`
- RV unlink09 adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.txt`
- LA unlink09 adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.txt`
- Combined adjacent report: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.combined-promotion-candidates.txt`

Additional mkdir09 futex bitset evidence artifacts:

- RV mkdir09 pre-fix retest summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-current-retest-20260603T222025+0800.summary.txt` — musl PASS, glibc `TBROK` futex abort; diagnostic only.
- RV mkdir09 targeted log: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.log`
- RV mkdir09 summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.summary.txt`
- RV mkdir09 candidate report: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.promotion-candidates.txt`
- LA mkdir09 targeted log: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.log`
- LA mkdir09 summary: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.summary.txt`
- LA mkdir09 candidate report: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.promotion-candidates.txt`
- Combined mkdir09 candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-mkdir09-futex-bitset-fix-promotion-candidates.txt`
- RV futex/clone adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-futex-bitset-adjacent-regression-20260603T222822+0800.summary.txt`
- LA futex/clone adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.summary.txt`
- Combined futex/clone adjacent report: `target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.combined-promotion-candidates.txt`

Additional gettid02 futex/glibc follow-up evidence artifacts:

- RV gettid02 targeted log: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.log`
- RV gettid02 summary: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.summary.txt`
- RV gettid02 candidate report: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.promotion-candidates.txt`
- LA gettid02 targeted log: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.log`
- LA gettid02 summary: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.summary.txt`
- LA gettid02 candidate report: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.promotion-candidates.txt`
- Combined gettid02 candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-gettid02-after-futex-bitset-20260603T224549+0800.promotion-candidates.txt`

Additional futex_wait_bitset01 follow-up evidence artifacts:

- RV futex adjacent scout log: `target/ltp-1000-milestone-06-stable806/rv-futex-adjacent-scout-20260603T225625+0800.log`
- RV futex adjacent scout summary: `target/ltp-1000-milestone-06-stable806/rv-futex-adjacent-scout-20260603T225625+0800.summary.txt`
- RV futex adjacent scout candidate report: `target/ltp-1000-milestone-06-stable806/rv-futex-adjacent-scout-20260603T225625+0800.promotion-candidates.txt`
- LA futex_wait_bitset01 follow-up log: `target/ltp-1000-milestone-06-stable806/la-futex-wait-bitset01-followup-20260603T225741+0800.log`
- LA futex_wait_bitset01 follow-up summary: `target/ltp-1000-milestone-06-stable806/la-futex-wait-bitset01-followup-20260603T225741+0800.summary.txt`
- LA futex_wait_bitset01 follow-up candidate report: `target/ltp-1000-milestone-06-stable806/la-futex-wait-bitset01-followup-20260603T225741+0800.promotion-candidates.txt`
- Combined futex_wait_bitset01 candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-futex-wait-bitset01-followup-20260603T225741+0800.promotion-candidates.txt`

Additional excluded late-scout artifacts:

- RV VFS/MM small scout summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-mm-small-scout-20260603T230922+0800.summary.txt` — `mmap05` is only RV-clean; LA follow-up below fails, so no candidate is counted.
- LA mmap05 follow-up summary: `target/ltp-1000-milestone-06-stable806/la-mmap05-followup-20260603T231053+0800.summary.txt` — `0 PASS / 2 FAIL / TFAIL=2`; `mmap05` remains blocked on LA SIGSEGV/protection semantics.
- RV process/exec/signal scout summary: `target/ltp-1000-milestone-06-stable806/rv-process-exec-signal-scout-20260603T231200+0800.summary.txt` — allocator panic marker during the `kill10` batch; zero candidates.
- RV exec-only scout summary: `target/ltp-1000-milestone-06-stable806/rv-exec-small-scout-20260603T231306+0800.summary.txt` — visible `TBROK/TFAIL`; zero candidates.
- RV FD/path small scout summary: `target/ltp-1000-milestone-06-stable806/rv-fd-path-small-scout-20260603T231708+0800.summary.txt` — blocked rows remain excluded; only `fstat02`/`fstat02_64` are counted after LA confirmation.

- RV sync/fd/io scout summary: `target/ltp-1000-milestone-06-stable806/rv-sync-fd-io-scout-20260603T232921+0800.summary.txt` — `0 PASS / 20 FAIL`; no `fdatasync`, `fsync`, `sync`, `syncfs`, `sync_file_range`, FIFO `read`/`write`, or `lseek11` row is counted.
- RV xattr small scout summary: `target/ltp-1000-milestone-06-stable806/rv-xattr-small-scout-20260603T233055+0800.summary.txt` — `0 PASS / 16 FAIL`; no row is counted from that scout by itself. `setxattr03`, `fgetxattr02`, `getxattr02`, and `setxattr02` are counted only after their later generic repairs and fresh RV+LA evidence above.
