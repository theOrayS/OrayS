# stable315 promotion gate report

Date: 2026-05-25
Target: stable300 -> stable315
Result: **NOT PROMOTED**

## Outcome

`LTP_STABLE_CASES` remains **300 total / 300 unique / 0 duplicates**. No 15-case clean tranche was found.

## Evidence

- Baseline stable300 final summaries from `docs/ltp-score-improvement-2026-05-24-phase-a/` remain the last clean aggregate gate: RV and LA each `PASS LTP CASE 600`, `FAIL 0`, `ltp-musl 300/0`, `ltp-glibc 300/0`, with transparent known `read02` TCONF=4.
- Batch-A RV discovery (`raw/batch-a-rv-summary.txt`): `PASS LTP CASE 14`, `FAIL LTP CASE 12`, `ltp-musl 2 passed / 11 failed`, `ltp-glibc 12 passed / 1 failed`, internal `TFAIL=11`, `TBROK=2`.
- User-priority blocker RV discovery (`raw/blocker-batch-rv-summary.txt`): `PASS LTP CASE 2`, `FAIL LTP CASE 24`, internal `TFAIL=65`, `TBROK=13`, `TCONF=2`, ENOSYS=2.
- Post-Team targeted runs were aborted/untrusted due duplicate/aborted evaluator launches; status files are under `raw/post-team-candidate*.status` and are explicitly excluded from promotion evidence.

## Gate decision

Blocked. There is no RV+LA x musl+glibc clean candidate set for stable315.
