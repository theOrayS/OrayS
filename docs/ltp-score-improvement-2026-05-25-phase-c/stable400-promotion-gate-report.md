# Stable400 promotion gate report

Date: 2026-05-26
Status: **not achieved**.

## Result

The campaign accepted a smaller honest partial promotion from stable375 to stable382, not stable400. The live stable list is 382 total / 382 unique / 0 duplicates.

Accepted new cases:

- `clock_settime01`
- `clock_settime02`
- `clone03`
- `confstr01`
- `chmod05`
- `fchmod05`
- `lseek02`

## Accepted gate evidence

| Evidence | Result |
| --- | --- |
| `raw/target-stable400-clocksettime2-rv-001-summary.txt` | RV targeted musl+glibc PASS 4 / FAIL 0 for `clock_settime01,clock_settime02` |
| `raw/target-stable400-clocksettime2-la-001-summary.txt` | LA targeted musl+glibc PASS 4 / FAIL 0 for `clock_settime01,clock_settime02` |
| `raw/target-stable400-cloneconf2-rv-001-summary.txt` | RV targeted musl+glibc PASS 4 / FAIL 0 for `clone03,confstr01` |
| `raw/target-stable400-cloneconf2-la-001-summary.txt` | LA targeted musl+glibc PASS 4 / FAIL 0 for `clone03,confstr01` |
| `raw/stable379-rv-gate-002-summary.txt` | RV stable379 aggregate PASS 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0 |
| `raw/stable379-la-gate-001-summary.txt` | LA stable379 aggregate PASS 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0 |
| `raw/target-stable400-chmod-fchmod-rv-001-summary.txt` | RV targeted `chmod05,fchmod05` PASS 4 / FAIL 0; `ltp-musl` 2/0; `ltp-glibc` 2/0 |
| `raw/target-stable400-chmod-fchmod-la-001-summary.txt` | LA targeted `chmod05,fchmod05` PASS 4 / FAIL 0; `ltp-musl` 2/0; `ltp-glibc` 2/0 |
| `raw/stable381-rv-gate-001-summary.txt` | RV stable381 aggregate PASS 762 / FAIL 0; `ltp-musl` 381/0; `ltp-glibc` 381/0 |
| `raw/stable381-la-gate-001-summary.txt` | LA stable381 aggregate PASS 762 / FAIL 0; `ltp-musl` 381/0; `ltp-glibc` 381/0 |
| `raw/target-stable400-lseek02-rv-002-summary.txt` | RV targeted `lseek02` PASS 2 / FAIL 0; `ltp-musl` 1/0; `ltp-glibc` 1/0 |
| `raw/target-stable400-lseek02-la-001-summary.txt` | LA targeted `lseek02` PASS 2 / FAIL 0; `ltp-musl` 1/0; `ltp-glibc` 1/0 |
| `raw/stable382-rv-gate-001-summary.txt` | RV stable382 aggregate PASS 764 / FAIL 0; `ltp-musl` 382/0; `ltp-glibc` 382/0 |
| `raw/stable382-la-gate-001-summary.txt` | LA stable382 aggregate PASS 764 / FAIL 0; `ltp-musl` 382/0; `ltp-glibc` 382/0 |

## Evidence rejected / not enough for stable400

The broader scout pools still contain real failures, setup breakage, TCONF/TBROK/TFAIL, timeout risk, or arch/libc splits. They were not promoted. Stable400 still needs at least 18 additional RV+LA x musl+glibc clean cases plus clean aggregate gates.

## Attempt 3 scout evidence (no promotion)

The G002 retry found no additional four-way-clean cases. Fresh scout summaries are preserved as negative evidence:

- `raw/target-stable400-readlinkat02-rv-serial-001-summary.txt`: RV `readlinkat02` musl+glibc clean.
- `raw/target-stable400-readlinkat02-la-serial-001-summary.txt`: LA `readlinkat02` glibc clean, LA musl TFAIL; not promotable.
- `raw/target-stable400-wave2-rv-001-summary.txt`: RV wave2 has TBROK/ENOSYS and `pipe02` panic/trap; not promotable.
- `raw/target-stable400-timesignal-rv-serial-001-summary.txt`: RV time/signal/wait scout has TFAIL/TBROK/TCONF/timeouts and was stopped after blockers; not promotable.
- `raw/target-stable400-fd-rv-serial-001-summary.txt`: RV FD/fcntl scout has PASS 0 / FAIL 16 with TBROK/TFAIL/ENOSYS; not promotable.
- `raw/target-stable400-fspath-rv-serial-001-summary.txt`: RV FS/path scout has PASS 0 / FAIL 16 with TFAIL/TBROK/ENOSYS; not promotable.

Stable400 remains undelivered; stable382 is the highest trusted partial promotion.

## Policy note

`lseek02` was only accepted after real `mknodat`/FIFO behavior was added and aggregate stable382 gates passed. `read02` remains transparent `pass_with_tconf`. The parser reports no wrapper timeout/ENOSYS/panic in accepted stable381 aggregate gates. LA stable381 raw log still contains one inherited LTP internal `Test timeouted, sending SIGKILL!` notice in a pre-existing long-running case; it is disclosed and is not from the two latest promoted chmod/fchmod cases.
