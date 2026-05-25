# Stable400 promotion gate report

Date: 2026-05-25
Status: **not achieved** in this phase-c execution slice.

## Result

The campaign found only four fresh four-way-clean promotion candidates after the log-noise repair:

- `clock_settime01`
- `clock_settime02`
- `clone03`
- `confstr01`

These four cases were temporarily tested as a stable379 candidate set, but the aggregate RV stable gate timed out on existing `ftest03` and was aborted. The live stable list is therefore kept at 375 unique cases; the four cases remain pending targeted-clean candidates, not an accepted promotion.

## Evidence accepted

| Evidence | Result |
| --- | --- |
| `raw/target-stable400-clocksettime2-rv-001-summary.txt` | RV musl+glibc PASS 4 / FAIL 0 for the two clock_settime cases; internal failures 0 |
| `raw/target-stable400-clocksettime2-la-001-summary.txt` | LA musl+glibc PASS 4 / FAIL 0 for the two clock_settime cases; internal failures 0 |
| `raw/target-stable400-cloneconf2-rv-001-summary.txt` | RV musl+glibc PASS 4 / FAIL 0 for clone03/confstr01; internal failures 0 |
| `raw/target-stable400-cloneconf2-la-001-summary.txt` | LA musl+glibc PASS 4 / FAIL 0 for clone03/confstr01; internal failures 0 |

## Evidence rejected

See `candidate-matrix.md` for rejected cases and blockers. Timeout, TFAIL/TBROK, pass_with_tconf, ENOSYS, or wrapper-failure evidence was not used for promotion.

## Gate status

The attempted stable379 aggregate gate is the blocker evidence: RV hit `FAIL LTP CASE ftest03 : 137` / `TIMEOUT LTP CASE ftest03 after 60s`, then the run was aborted before LA. Stable400 remains blocked until the aggregate gate is clean and at least 25 cases beyond stable375 have fresh RV+LA x musl+glibc clean evidence.
