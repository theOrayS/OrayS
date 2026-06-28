# stable350 Delivery Report

Date: 2026-05-25
Verdict: **DELIVERED**

## Result

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` now contains exactly **350 total / 350 unique / 0 duplicates**. The final list is captured in `stable350-live.cases`.

Final aggregate gates:

| Arch | Final summary | PASS/FAIL | musl | glibc | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap | Marker prefix |
| --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| RV | `raw/stable350-rv-final-002-summary.txt` | 700/0 | 350/0 | 350/0 | 0 | 0 | 4 known `read02` only | 0 | 0 | 0 | `raw/stable350-rv-final-002-marker-prefix.txt`: bad=0 |
| LA | `raw/stable350-la-final-002-summary.txt` | 700/0 | 350/0 | 350/0 | 0 | 0 | 4 known `read02` only | 0 | 0 | 0 | `raw/stable350-la-final-002-marker-prefix.txt`: bad=0 |

`read02` remains disclosed as pass-with-TCONF and is not treated as clean. No new internal TFAIL/TBROK/TCONF was introduced beyond that known source.

## Stage summary

- stable315 additions: `alarm05, alarm07, write05, gettimeofday02, waitpid01, pipe2_02, sched_getscheduler02, fstat03, fstat03_64, statfs02, fstatfs02, fstatfs02_64, sched_getparam03, sched_setparam04, sched_setparam05`
- stable330 additions: `fchdir01, fchdir03, fcntl05, fcntl05_64, fcntl12, fcntl12_64, fcntl13, fcntl13_64, fdatasync01, fdatasync02, readlinkat01, sched_setscheduler01, sched_setscheduler02, symlinkat01, ftruncate03_64`
- stable350 additions: `chdir04, chown01, chown02, chown03, chown05, creat05, abs01, mkdir05, statfs02_64, truncate03_64, fork03, fork04, fork07, fork08, fork09, signal05, string01, memcmp01, memcpy01, memset01`

## Important demotion

`kill02` was removed from the final stable350 set. It had targeted promise, but the first LA final aggregate showed `PASS LTP CASE: 699`, `FAIL LTP CASE: 1`, `ltp-glibc 349/1`, and TBROK=4 in `raw/stable350-la-final-summary.txt`. The final list replaced it with `abs01`, which passed fresh RV+LA targeted replacement checks and the final RV+LA stable aggregate gates.

## Source changes supporting the promotion

- `examples/shell/src/cmd.rs`: stable list now has 350 unique cases; `kill02` is not present; `abs01` is present.
- `examples/shell/src/uspace/fd_table.rs` and `linux_abi.rs`: improve `O_NOFOLLOW`, symlink/O_PATH behavior, fcntl lock/lease validation, fsync/fchdir/mkdirat edge semantics.
- `examples/shell/src/uspace/metadata.rs`: support `readlinkat` empty-path/fd target behavior.
- `examples/shell/src/uspace/resource_sched.rs`: tighten scheduler permission and errno behavior for scheduler promotion/regression cases.
- `examples/shell/src/uspace/memory_map.rs`: page-fault exit now routes through SIGSEGV group exit status so wait-status tests observe signal semantics.

## Validation commands run

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-final-002.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-la-final-002.log
cargo fmt --all -- --check
make A=examples/shell ARCH=riscv64
make all
```

Marker-prefix checks were run on final RV and LA logs and both reported `TOTAL markers=700 bad=0`.

## Checks not run

- Offline remote build: `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`.
- Full `make clippy`.
- `make unittest_no_fail_fast`.

## User-visible / POSIX behavior changes

This campaign intentionally changes POSIX-visible compatibility behavior in the shell/uspace LTP environment:

- `fcntl` lock/lease commands now validate and return Linux-compatible errors for the covered cases.
- `O_NOFOLLOW` and O_PATH/symlink opening behavior is more Linux-like.
- `readlinkat` with an empty path can resolve from the supplied fd target.
- `fchdir`, `fsync`, `mkdirat`, scheduler permission checks, and scheduler errno ordering were tightened.
- User page faults now produce SIGSEGV group-exit status rather than a plain hardcoded exit code.

No fake PASS, LTP case-name special-casing, or marker-output laundering was introduced.
