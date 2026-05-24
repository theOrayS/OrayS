# Remote marker regression check

## Result

PASS: bad marker lines = 0.

## Check

Scanned `docs/ltp-score-improvement-2026-05-24-phase-a/raw/*.log` for any line containing `PASS LTP CASE` or `FAIL LTP CASE` that did not start with exactly that marker at column 0.

## Rationale

The previous remote scorer regression was caused by marker lines being polluted by ANSI/color/reset prefixes. This check confirms current phase logs preserve column-0 marker prefixes.

## Final gate summaries

RV:

- PASS LTP CASE: 600
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 300 passed, 0 failed
- ltp-glibc: 300 passed, 0 failed

LA:

- PASS LTP CASE: 600
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 300 passed, 0 failed
- ltp-glibc: 300 passed, 0 failed
