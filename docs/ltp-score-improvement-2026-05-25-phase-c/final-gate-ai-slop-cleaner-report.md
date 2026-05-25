# Final gate AI-slop-cleaner report

Date: 2026-05-25
Status: **PASS for cleanup discipline; delivery blocker preserved honestly**.

## Cleanup plan used

1. Preserve behavior first: do not alter visible errno, syscall success/failure, marker format, or stable case acceptance rules.
2. Prefer deletion/no-op simplification over new abstractions: replace noisy `ax_err!` call sites with direct `Err(AxError::...)` only where the same error is expected and intentionally returned.
3. Keep promotion truth separate from discovery: targeted-clean candidates may be recorded, but `LTP_STABLE_CASES` is leader-owned and only updated after aggregate gates are clean.
4. Run lightweight formatting/diff/build checks and parse LTP evidence with `scripts/ltp_summary.py`.

## Slop risks checked

| Risk | Result |
| --- | --- |
| Fake PASS or case-name hardcoding | Not introduced. No LTP source or evaluator bypass was changed. |
| Timeout counted as PASS | Not introduced. RV `ftest03` timeout blocked the promotion and caused abort before LA aggregate. |
| Hidden TCONF | Not introduced. Known `read02` TCONF remains disclosed; targeted-clean pending cases had no internal TCONF in their four-way targeted summaries. |
| Broad refactor or dependency churn | Not introduced. Changes are local to expected-error logging paths plus reports. |
| Unnecessary abstraction | Not introduced. Direct `Err(AxError::...)` keeps the existing API surface and avoids a new helper. |
| Stale promotion docs | Fixed. Reports now state zero accepted promotions and stable375 as the live baseline. |

## Evidence summary

- Original remote baseline: `axfs::fops` NotADirectory warning count RV 4432 / LA 4433 in user remote outputs.
- Post-fix RV subset: exact `axfs::fops` NotADirectory count 0, marker bad-prefix count 0, PASS 6 / FAIL 0 with only known `read02` TCONF.
- Aborted RV stable379 sample: `axfs::fops=0`, `AxError::IsADirectory=0`, `AxError::AlreadyExists=0`, marker bad-prefix count 0; not promotion evidence because `ftest03` timed out.
- Live stable list after blocker handling: 375 total / 375 unique / 0 duplicates.

## Remaining risks

- Stable promotion is blocked by an existing `ftest03` RV timeout in the aggregate run. It must be fixed or isolated honestly before re-adding pending targeted-clean cases.
- Residual lower-frequency `AxError::NotADirectory` noise from `axfs_ramfs::file:69` remains for a later logging-noise pass. It is not the original `axfs::fops:297` hot path.
- Stable400/425/450 final gates were not run because stable379 aggregate failed first.

## Decision

PASS for the scoped cleanup/log-noise repair and report hygiene. Do not claim stable379, stable400, stable425, or stable450 from this phase-c slice.
