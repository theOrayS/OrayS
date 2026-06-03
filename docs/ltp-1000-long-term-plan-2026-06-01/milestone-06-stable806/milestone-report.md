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

## Risks and boundaries

- `CLONE_NEWUTS` and `unshare(CLONE_NEWUTS)` are still not implemented; `utsname03` remains blocked and is not counted.
- The shared hostname object models only the default shared UTS namespace. It is not a full namespace registry.
- Socket scout rows remain visibly blocked or incomplete and cannot be promoted.
- `readlink03`/`readlinkat02` remain blocked on LA musl wrapper behavior; rejecting all one-byte buffers in-kernel is not acceptable.
- `nice04` remains blocked on libc-visible errno differences around priority lowering; do not risk stable `setpriority` rows with a wrapper-specific kernel special case.
- Statx, 16-bit UID, capability, and glibc `gettid02` rows remain blocker-only until real semantics or futex/glibc robustness improve.
- The VFS/FD/select scout was split: `mkdir02`, `mkdir03`, `mkdirat02`, and `rmdir02` are repaired and candidate-clean, while the remaining select/fcntl/mknod/symlink/unlink rows still have TCONF/timeout/TFAIL/TBROK blockers.
- Timerslack/prctl adjacent stable regression still needs to be included before any eventual stable806 promotion commit.

## Conclusion

This checkpoint improves UTS, VFS path/errno, and metadata inheritance semantics and adds 5 new unique candidates (`utsname02`, `mkdirat02`, `rmdir02`, `mkdir02`, `mkdir03`), bringing the stable806 candidate pool to 7 unique cases. The blocker triage added zero candidates outside the repaired VFS/metadata rows and intentionally avoided unsafe readlink/nice workarounds. Baseline remains `756 total / 756 unique / 0 duplicate`; no stable-list milestone promotion commit is created until the next +50 unique clean cohort is available.
