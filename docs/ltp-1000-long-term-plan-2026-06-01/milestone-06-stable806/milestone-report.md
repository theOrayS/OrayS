# milestone-06 stable806 interim report

Date: 2026-06-03.

## Target

Move the live baseline from stable756 toward the next stable806 milestone without fake pass behavior. This is an interim candidate-pool checkpoint, not a stable-list promotion.

## Changes in this checkpoint

- Added real timerslack behavior in the prior checkpoint and kept `prctl08`/`prctl09` as clean candidates.
- Repaired default UTS hostname sharing by making plain `fork()` children share the same hostname object.
- Documented a partial RV socket-core scout as blocker-only evidence.
- Documented blocker triage for `readlink03`/`readlinkat02`, `nice04`, RV statx rows, and RV credential/capability rows; no source workaround was made for semantically unsafe cases.
- Did not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.

## Candidate-pool status

Current new unique stable806 candidates:

1. `prctl08`
2. `prctl09`
3. `utsname02`

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

## Risks and boundaries

- `CLONE_NEWUTS` and `unshare(CLONE_NEWUTS)` are still not implemented; `utsname03` remains blocked and is not counted.
- The shared hostname object models only the default shared UTS namespace. It is not a full namespace registry.
- Socket scout rows remain visibly blocked or incomplete and cannot be promoted.
- `readlink03`/`readlinkat02` remain blocked on LA musl wrapper behavior; rejecting all one-byte buffers in-kernel is not acceptable.
- `nice04` remains blocked on libc-visible errno differences around priority lowering; do not risk stable `setpriority` rows with a wrapper-specific kernel special case.
- Statx, 16-bit UID, capability, and glibc `gettid02` rows remain blocker-only until real semantics or futex/glibc robustness improve.
- Timerslack/prctl adjacent stable regression still needs to be included before any eventual stable806 promotion commit.

## Conclusion

This checkpoint improves UTS semantics and adds 1 new unique candidate (`utsname02`), bringing the stable806 candidate pool to 3 unique cases. The later blocker triage added zero candidates and intentionally avoided unsafe readlink/nice workarounds. Baseline remains `756 total / 756 unique / 0 duplicate`; no milestone promotion commit is created until the next +50 unique clean cohort is available.
