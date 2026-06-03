# milestone-06 promotion candidates so far

These cases are candidate-pool evidence only. They are not yet promoted into `LTP_STABLE_CASES` because milestone-06 still needs the full next 50-case cohort plus adjacent stable regression evidence. Current candidate pool: 12/50 unique cases.

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

## Explicitly excluded after blocker triage

| Case/lane | Evidence | Exclusion reason |
| --- | --- | --- |
| `readlink03` | RV clean; LA glibc clean; LA musl `TFAIL` | LA musl wrapper passes a one-byte non-null buffer for the nominal zero-size test; kernel cannot reject all `bufsiz=1` without breaking valid readlink semantics. |
| `readlinkat02` | RV clean; LA glibc clean; LA musl `TFAIL` | Same LA musl wrapper boundary as `readlink03`; combined report has 0 candidates. |
| `nice04` | RV glibc clean; RV musl `TFAIL` with `EACCES` instead of `EPERM` | Shared `setpriority` semantics would be endangered by a kernel-only wrapper special case. |
| `statx01,statx04..statx12` | RV scout has `TCONF`, wrapper FAILs, and `statx11` timeouts | Zero RV-only candidates; not safe promotion evidence. |
| `gettid02`, `*_16`, `capget*`, `capset*` | RV scout has one musl-only pass, glibc `gettid02` `TBROK`, and 16-bit UID/capability `TCONF` rows | Zero RV-only candidates; needs futex/glibc or unsupported-ABI lane work before reconsideration. |
| `unlink09`..`select04` VFS/FD/select scout | RV scout has `9 PASS / 45 FAIL`, `TBROK/TCONF/TFAIL`, and four `fcntl17*` timeouts | Zero RV-only candidates from the broad scout; later targeted repairs make `fcntl27`, same-source `fcntl27_64`, `symlink03`, and `unlink09` valid candidates. The broad scout itself and remaining `select*` pass-with-TCONF rows plus timeout/TFAIL/TBROK rows are not promotion evidence. |
| `mkdir09` isolation scout | RV isolation scout after the mkdir repair still had a glibc `mkdir09` futex abort | Superseded by the later futex bitset repair below; the isolation scout remains diagnostic blocker evidence only and is not counted by itself. Earlier `symlink03` blocker rows are superseded by the later parent-permission repair and clean RV/LA evidence above. |

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
