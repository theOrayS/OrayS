# Stable450 delivery report

Date: 2026-05-26
Status: **not delivered**.

## Summary

Stable450 remains the main target, but this execution slice honestly delivered only a partial promotion to **stable382**. The live `LTP_STABLE_CASES` list is 382 total / 382 unique / 0 duplicates.

Accepted additions from stable375:

- `clock_settime01`
- `clock_settime02`
- `clone03`
- `confstr01`
- `chmod05`
- `fchmod05`
- `lseek02`

## Highest trusted gate

| Arch | Summary | Markers | Suite result |
| --- | --- | ---: | --- |
| RV | `raw/stable382-rv-gate-001-summary.txt` | PASS 764 / FAIL 0 | `ltp-musl` 382/0; `ltp-glibc` 382/0 |
| LA | `raw/stable382-la-gate-001-summary.txt` | PASS 764 / FAIL 0 | `ltp-musl` 382/0; `ltp-glibc` 382/0 |

Known `read02` TCONF remains disclosed as `pass_with_tconf`; no new promoted case adds TFAIL/TBROK/TCONF. Parser timeout, ENOSYS, and panic/trap counts are 0 in both accepted summaries.

## Why stable450 was not claimed

The remaining candidate pools contain unresolved failures and risks:

- internal TFAIL/TBROK in VFS/path, statx, clone, timer, and setup-heavy tests;
- timeout or long-runtime risk in fs/timer/process candidates;
- ENOSYS/not-implemented signals in some clone/time scout paths;
- wrapper or missing-test failures in fs-suite candidates;
- architecture/libc split failures such as previously observed `readlinkat02` LA musl and `inode02` LA glibc.

Claiming stable450 would violate the no-fake-PASS/no-timeout-as-PASS/no-hidden-TCONF constraints.

## Follow-up

Continue from stable382. Stable400 needs 18 additional clean cases; stable450 needs 68 additional clean cases. Use `next-session-prompt-stable450-followup.md` for the next run.


## Stable381 update (2026-05-26)

Stable450 remains undelivered. A smaller truthful promotion to stable381 was accepted after `chmod05` and `fchmod05` passed targeted RV+LA x musl+glibc checks and RV/LA aggregate stable381 gates. Evidence: `stable381-promotion-gate-report.md`, `raw/stable381-rv-gate-001-summary.txt`, and `raw/stable381-la-gate-001-summary.txt`.


## Stable382 update (2026-05-26)

Stable450 remains undelivered. `lseek02` was accepted after the real `mknodat`/FIFO compatibility repair and RV/LA stable382 aggregate gates. Evidence: `stable382-promotion-gate-report.md`, `raw/stable382-rv-gate-001-summary.txt`, and `raw/stable382-la-gate-001-summary.txt`.
