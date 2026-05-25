# Stable400 promotion gate report

Date: 2026-05-26
Status: **not achieved**.

## Result

The campaign accepted a smaller honest partial promotion from stable375 to stable379, not stable400. The live stable list is 379 total / 379 unique / 0 duplicates.

Accepted new cases:

- `clock_settime01`
- `clock_settime02`
- `clone03`
- `confstr01`

## Accepted gate evidence

| Evidence | Result |
| --- | --- |
| `raw/target-stable400-clocksettime2-rv-001-summary.txt` | RV targeted musl+glibc PASS 4 / FAIL 0 for `clock_settime01,clock_settime02` |
| `raw/target-stable400-clocksettime2-la-001-summary.txt` | LA targeted musl+glibc PASS 4 / FAIL 0 for `clock_settime01,clock_settime02` |
| `raw/target-stable400-cloneconf2-rv-001-summary.txt` | RV targeted musl+glibc PASS 4 / FAIL 0 for `clone03,confstr01` |
| `raw/target-stable400-cloneconf2-la-001-summary.txt` | LA targeted musl+glibc PASS 4 / FAIL 0 for `clone03,confstr01` |
| `raw/stable379-rv-gate-002-summary.txt` | RV stable379 aggregate PASS 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0 |
| `raw/stable379-la-gate-001-summary.txt` | LA stable379 aggregate PASS 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0 |

## Evidence rejected / not enough for stable400

The broader scout pools still contain real failures, setup breakage, TCONF/TBROK/TFAIL, timeout risk, or arch/libc splits. They were not promoted. Stable400 still needs at least 21 additional RV+LA x musl+glibc clean cases plus clean aggregate gates.

## Policy note

`read02` remains transparent `pass_with_tconf`. The parser reports no wrapper timeout/ENOSYS/panic in accepted stable379 aggregate gates. LA raw logs still contain two inherited LTP internal `Test timeouted, sending SIGKILL!` notices in pre-existing long-running cases; these are disclosed and are not from the four newly promoted cases.
