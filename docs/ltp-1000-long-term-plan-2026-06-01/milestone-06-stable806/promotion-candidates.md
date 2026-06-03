# milestone-06 promotion candidates so far

These cases are candidate-pool evidence only. They are not yet promoted into `LTP_STABLE_CASES` because milestone-06 still needs the full next 50-case cohort plus adjacent stable regression evidence.

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
| `unlink09`..`select04` VFS/FD/select scout | RV scout has `9 PASS / 45 FAIL`, `TBROK/TCONF/TFAIL`, and four `fcntl17*` timeouts | Zero RV-only candidates from the broad scout; later targeted repair makes `fcntl27` and same-source `fcntl27_64` valid candidates. `select*` pass-with-TCONF rows and timeout/TFAIL/TBROK rows are not promotion evidence. |
| `symlink03`, `mkdir09` isolation scout | RV isolation scout after the mkdir repair still has `symlink03` TBROK on `/tmp/ltp-work` permissions and glibc `mkdir09` futex abort | Blocker-only; not counted as stable806 candidates. |

Excluded evidence artifacts:

- RV readlink summary: `target/ltp-1000-milestone-06-stable806/rv-readlink03-readlinkat02-20260603T191956+0800.summary.txt`
- LA readlink summary: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.summary.txt`
- Combined readlink report: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.combined-promotion-candidates.txt`
- RV statx summary: `target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.summary.txt`
- RV credential/capability summary: `target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.summary.txt`

Additional excluded VFS/FD/select artifact:

- RV VFS/FD/select summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.summary.txt`
- RV VFS/FD/select candidate report: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.promotion-candidates.txt`
