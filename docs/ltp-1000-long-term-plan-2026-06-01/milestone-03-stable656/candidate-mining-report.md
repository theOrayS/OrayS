# Milestone 03 stable656 candidate mining report

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Live stable baseline: `606 total / 606 unique / 0 duplicate`

## Purpose

Record the current post-stable606 candidate search state so later G009/G010 work does not re-mine exhausted evidence or mistake scout-only rows for promotion proof.

## Current four-way-clean pool

| Case | Evidence | Status |
| --- | --- | --- |
| `futex_wait01` | RV mixed scout + LA confirmation, both musl/glibc parser-clean | candidate, not promoted until +50 batch |
| `sched_setaffinity01` | RV/LA targeted postfix runs, both musl/glibc parser-clean | candidate, not promoted until +50 batch |

Combined report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-20260601T223023Z.promotion-candidates.txt`.

## Exhausted closed full-sweep evidence

Closed arch sweep inputs:

- `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log`
- `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log`

Derived artifacts:

- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`
- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.not-stable.txt`
- `target/ltp-1000-milestone-03-stable656/rv-arch002-full-matrix-20260601T224223Z.json`
- `target/ltp-1000-milestone-03-stable656/la-arch012-full-matrix-20260601T224223Z.json`

Result: the historical sweep report has 563 four-way-clean rows, but zero rows remain outside live stable606. Do not count this evidence toward stable656; use it only as a blocker map.

## Highest-value blocked rows from current evidence

| Case/lane | Current blocker | Next useful action |
| --- | --- | --- |
| `readlinkat02` | RV clean; LA glibc clean; LA musl `TFAIL=1` on rerun | inspect LA-musl call boundary; do not special-case syscall `bufsiz=1` |
| `fsync02` | RV musl PASS; RV glibc `TBROK=1` in isolated rerun | inspect free-space/setup behavior before any VFS fix |
| `nice04` | RV musl `nice(-10)` gets `EACCES`; direct `setpriority02` source requires `EACCES` for the same unprivileged lowering class | keep blocked; see `nice04-errno-boundary-report.md`; do not flip `sys_setpriority` errno |
| `kill10` | RV panic/trap in scout | isolate before any broad process/signal shard |
| `futex_wait03` / `futex_wait05` | timeout or slept-too-long semantics | narrow futex timeout/EINTR lane with stable futex regressions |
| `mmap05` / `munmap01` / `mmap13` | SIGSEGV/SIGBUS delivery gaps | narrow mmap fault-signal lane with signal + mmap regressions |
| `shmat1` | long/hung mixed scout | SysV shm/resource lifetime lane, isolated timeout first |

## Promotion decision

No `LTP_STABLE_CASES` edit is justified. The candidate pool is 2/50 for stable656, and all blocker rows retain their parser-visible `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic` caveats.
