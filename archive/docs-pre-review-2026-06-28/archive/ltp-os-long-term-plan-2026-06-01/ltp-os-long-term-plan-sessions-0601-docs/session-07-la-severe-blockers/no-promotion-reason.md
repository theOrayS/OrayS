# Session 7 no-promotion reason

Session 7 intentionally has no stable promotion.

Reasons:

- `creat07` closes normally but fails in both LA musl/glibc with wrapper FAIL and internal `TBROK`.
- `tcp4-uni-basic01` closes normally but fails in both LA musl/glibc with wrapper FAIL and internal `TCONF`.
- No RV cross-architecture promotion gate was run for these cases.
- Blacklist removal means “not a severe full-sweep blocker anymore”; it does not mean PASS, stable, or score evidence.

Stable count remains `506 total / 506 unique / 0 duplicate` from Session 6.
