# Stable382 promotion gate report

Date: 2026-05-26
Status: **accepted partial promotion**; stable400/stable425/stable450 remain undelivered.

## Result

The live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` list is now 382 total / 382 unique / 0 duplicates.

Accepted new case in this slice:

- `lseek02`

Accepted cumulative additions from stable375 in phase-c:

- `clock_settime01`
- `clock_settime02`
- `clone03`
- `confstr01`
- `chmod05`
- `fchmod05`
- `lseek02`

## Fix required for `lseek02`

Fresh targeted RV evidence showed `lseek02` was blocked by real `mkfifo()` setup failure, not by lseek semantics:

- `raw/target-stable400-lseek02-rv-001-summary.txt`: FAIL 2; musl+glibc TBROK on `mkfifo(tfifo1, 0777) failed: ENOSYS (38)`.

The implementation adds a minimal real `mknodat` path in the shell uspace layer:

- dispatches `__NR_mknodat`;
- supports regular-file creation and FIFO creation;
- records FIFO path metadata with `S_IFIFO` while preserving normal permission/owner metadata;
- opens recorded FIFO paths as pipe-backed non-seekable descriptors so `lseek()` returns ESPIPE through the existing FD path;
- removes the recorded special type on successful unlink.

This is not case-name hardcoding and does not modify LTP sources. It is intentionally minimal FIFO compatibility, not full named-FIFO peer/blocking/rename semantics.

## Targeted evidence

| Evidence | Result |
| --- | --- |
| `raw/target-stable400-lseek02-rv-002-summary.txt` | RV `lseek02` PASS 2 / FAIL 0; `ltp-musl` 1/0; `ltp-glibc` 1/0; TFAIL/TBROK/TCONF/timeout/ENOSYS/panic 0 |
| `raw/target-stable400-lseek02-la-001-summary.txt` | LA `lseek02` PASS 2 / FAIL 0; `ltp-musl` 1/0; `ltp-glibc` 1/0; TFAIL/TBROK/TCONF/timeout/ENOSYS/panic 0 |

## Aggregate stable382 gate evidence

| Arch | Summary | Marker result | Suite result | Internal status |
| --- | --- | --- | --- | --- |
| RV | `raw/stable382-rv-gate-001-summary.txt` | PASS 764 / FAIL 0 | `ltp-musl` 382/0; `ltp-glibc` 382/0 | TFAIL 0, TBROK 0, TCONF 4 known `read02`; timeout/ENOSYS/panic 0 |
| LA | `raw/stable382-la-gate-001-summary.txt` | PASS 764 / FAIL 0 | `ltp-musl` 382/0; `ltp-glibc` 382/0 | TFAIL 0, TBROK 0, TCONF 4 known `read02`; timeout/ENOSYS/panic 0 |

Marker prefix check for both aggregate logs: `bad_marker_lines=0`.

## Log-noise guardrail

The original remote-output flood source `axfs::fops:297 [AxError::NotADirectory]` remains fixed in both stable382 aggregate logs.

| Evidence | `axfs::fops:297` | `axfs_ramfs::file:69` | Any `AxError::NotADirectory` | `AxError::IsADirectory` | `AxError::AlreadyExists` |
| --- | ---: | ---: | ---: | ---: | ---: |
| RV stable382 aggregate | 0 | 22 | 22 | 0 | 0 |
| LA stable382 aggregate | 0 | 22 | 22 | 0 | 0 |

The residual NotADirectory entries are the previously disclosed `axfs_ramfs::file:69` family, not the fixed `fops.rs:297` warning flood.

## Caveats kept transparent

- `read02` remains the only known transparent `pass_with_tconf` source and is not described as clean.
- LA stable382 raw output contains two inherited LTP internal `Test timeouted, sending SIGKILL!` notices in pre-existing long paths (`waitpid10`, `pipe13` style paths). `scripts/ltp_summary.py` reports parser timeout matches 0 and wrapper FAIL 0; the notices are disclosed separately and are not caused by `lseek02`.
- Stable400 is still 18 additional clean cases away; stable450 is still 68 clean cases away.

## Decision

`lseek02` is accepted into stable382 because it has fresh targeted RV+LA x musl+glibc clean evidence and fresh RV/LA aggregate stable382 gates with wrapper FAIL 0, internal TFAIL/TBROK 0, no new TCONF, parser timeout 0, ENOSYS 0, panic/trap 0.
