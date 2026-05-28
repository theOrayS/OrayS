# Stable379 promotion gate report

Date: 2026-05-26
Status: **accepted as highest trusted partial promotion**.

## Live list

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` is now 379 total / 379 unique / 0 duplicates.

New cases accepted from stable375:

- `clock_settime01`
- `clock_settime02`
- `clone03`
- `confstr01`

## Aggregate evidence

| Gate | Summary | Result |
| --- | --- | --- |
| RV stable379 aggregate | `raw/stable379-rv-gate-002-summary.txt` | PASS LTP CASE 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0; parser timeout/ENOSYS/panic 0 |
| LA stable379 aggregate | `raw/stable379-la-gate-001-summary.txt` | PASS LTP CASE 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0; parser timeout/ENOSYS/panic 0 |

Known transparent internal status remains only `read02` O_DIRECT-on-tmpfs `TCONF`, reported as `pass_with_tconf` for musl+glibc on each architecture. No new accepted case has internal TFAIL/TBROK/TCONF, wrapper FAIL, ENOSYS, or panic/trap.

## Flaky/blocker context

An earlier RV aggregate attempt (`raw/stable379-rv-gate-001-summary.txt`) failed on existing `ftest03` at 60s. It was not used for promotion. Follow-up targeted retries show `ftest03` clean in isolation:

- `raw/ftest03-rv-retry-60s-001-summary.txt`: PASS 2 / FAIL 0.
- `raw/ftest03-rv-retry-90s-001-summary.txt`: PASS 2 / FAIL 0.

The accepted RV aggregate retry is `raw/stable379-rv-gate-002-summary.txt`.

## Marker and noise guardrail

| Gate | Markers | Bad marker prefix | `axfs::fops` | `AxError::NotADirectory` | Residual source |
| --- | ---: | ---: | ---: | ---: | --- |
| RV stable379 aggregate | 758 | 0 | 0 | 22 | `axfs_ramfs::file:69` |
| LA stable379 aggregate | 758 | 0 | 0 | 22 | `axfs_ramfs::file:69` |

The original high-frequency `kernel/fs/axfs/src/fops.rs` warning source remains at 0 in both accepted gates. Residual `axfs_ramfs::file:69` lines are lower-frequency expected negative-path noise and are not from the original `axfs::fops:297` source.

## Decision

Promote only these four candidates. Do not claim stable400/stable425/stable450 from this evidence.
