# Remote Marker Regression Check

Date: 2026-05-25

The remote evaluator depends on marker lines beginning at column 1. Final stable375 logs were scanned for any `PASS LTP CASE` / `FAIL LTP CASE` marker not starting at the beginning of the line.

Evidence: `raw/stable375-final-marker-prefix.txt`

```text
docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-rv-final-002.log: markers=750 bad=0
docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-la-final-003.log: markers=750 bad=0
```

Result: pass. No ANSI reset/color prefix contaminated final LTP marker lines.

Note: current wrapper wire format still prints `FAIL LTP CASE <case> : 0` for completed successful cases; `scripts/ltp_summary.py` treats numeric status 0 as wrapper PASS while preserving the remote marker format.
