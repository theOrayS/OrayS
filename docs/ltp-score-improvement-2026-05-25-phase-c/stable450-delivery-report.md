# Stable450 delivery report

Date: 2026-05-25
Status: **not delivered**.

## Summary

Stable450 was the main target, but this execution slice did not produce 75 clean new cases. After repairing the remote log-noise source, the campaign found four cases with fresh RV+LA x musl+glibc targeted-clean evidence, but promoted **zero** cases: the aggregate stable379 attempt hit an existing RV `ftest03` timeout and was aborted before LA, so the live stable list was reverted to and kept at 375 unique cases.

## Why stable450 was not claimed

The remaining candidate pools contain real unresolved failures:

- internal TFAIL/TBROK in VFS/path, statx, clone, timer, and setup-heavy tests;
- timeout risk in `inode02`, `clock_gettime01`, `setitimer01`, and related timer/fs cases;
- ENOSYS/not-implemented signals in some clone/time scout paths;
- wrapper or missing-test failures in fs-suite candidates;
- architecture/libc split failures such as `readlinkat02` LA musl and `inode02` LA glibc.

Claiming stable450 would violate the no-fake-PASS/no-timeout-as-PASS/no-hidden-TCONF constraints.

## Highest trusted target for this slice

Stable379 is **not delivered**. The attempted RV aggregate gate failed on existing `ftest03` timeout and was aborted before LA. A future stable379 attempt would need:

- expected RV markers: PASS LTP CASE 758, FAIL 0;
- expected LA markers: PASS LTP CASE 758, FAIL 0;
- expected suites: `ltp-musl` 379/0 and `ltp-glibc` 379/0 on both arches;
- known internal TCONF: only transparent `read02` O_DIRECT-on-tmpfs.

The final gate quality JSON records this as a failed/aborted stable379 attempt, not a stable450 delivery.
