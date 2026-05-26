# Stable381 promotion gate report

Date: 2026-05-26
Status: **accepted partial promotion**. Stable400/stable425/stable450 remain undelivered.

## Result

The campaign accepted two additional honest cases after fixing the musl group-lookup blocker: `chmod05` and `fchmod05`. Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` is now 381 total / 381 unique / 0 duplicates.

## Accepted new cases

| Case | Subsystem | Reason for acceptance |
| --- | --- | --- |
| `chmod05` | VFS permissions / group lookup / AF_UNIX nscd probe compatibility | Targeted RV+LA x musl+glibc clean; RV+LA aggregate stable381 clean |
| `fchmod05` | FD permissions / group lookup / AF_UNIX nscd probe compatibility | Targeted RV+LA x musl+glibc clean; RV+LA aggregate stable381 clean |

## Targeted evidence

| Evidence | Result |
| --- | --- |
| `raw/target-stable400-chmod-fchmod-rv-001-summary.txt` | RV targeted `chmod05,fchmod05`: PASS 4 / FAIL 0; musl 2/0; glibc 2/0; TFAIL/TBROK/TCONF 0; timeout/ENOSYS/panic 0 |
| `raw/target-stable400-chmod-fchmod-la-001-summary.txt` | LA targeted `chmod05,fchmod05`: PASS 4 / FAIL 0; musl 2/0; glibc 2/0; TFAIL/TBROK/TCONF 0; timeout/ENOSYS/panic 0 |
| `raw/target-stable400-chmod-fchmod-rv-002-summary.txt` | RV post-review errno/path-resolution retest: PASS 4 / FAIL 0; musl 2/0; glibc 2/0; internal 0 |
| `raw/target-stable400-chmod-fchmod-la-002-summary.txt` | LA post-review errno/path-resolution retest: PASS 4 / FAIL 0; musl 2/0; glibc 2/0; internal 0 |

## Aggregate promotion gate evidence

| Evidence | Result | Marker/noise guardrail |
| --- | --- | --- |
| `raw/stable381-rv-gate-001-summary.txt` | RV stable381 PASS LTP CASE 762 / FAIL 0; `ltp-musl` 381/0; `ltp-glibc` 381/0; internal TFAIL/TBROK 0; known `read02` TCONF only; timeout/ENOSYS/panic 0 | markers 762, bad prefix 0; `axfs::fops` 0; `AxError::NotADirectory` 22 |
| `raw/stable381-la-gate-001-summary.txt` | LA stable381 PASS LTP CASE 762 / FAIL 0; `ltp-musl` 381/0; `ltp-glibc` 381/0; internal TFAIL/TBROK 0; known `read02` TCONF only; timeout/ENOSYS/panic 0 | markers 762, bad prefix 0; `axfs::fops` 0; `AxError::NotADirectory` 22 |

## Visible behavior / errno note

The code change does **not** special-case `chmod05` or `fchmod05`. It repairs the local AF_UNIX `connect()` bridge used by musl's group lookup/nscd probe. A local AF_UNIX socket fd no longer falls through to the network socket-only bridge and returns blanket `ENOTSOCK`; pathname connects now return narrow Linux-compatible failure errno such as `ENOENT` for a missing pathname and `ECONNREFUSED` for existing pathname targets until a real AF_UNIX listener registry exists. This is an intended Linux/POSIX-visible compatibility improvement, not a fake PASS path.

## Stable400 status

Stable400 is still **not achieved**. The live accepted list is 381, so stable400 still needs 19 more four-way clean cases and stable450 still needs 69 more.
