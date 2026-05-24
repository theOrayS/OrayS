# remote marker regression check

Status: **PASS for fresh trusted stable270 logs**.

## Fresh marker-prefix check

Command:

```bash
python3 - <<'PY'
from pathlib import Path
logs = [
    'docs/ltp-score-improvement-2026-05-24-phase-a/raw/stable270-rv-aggregate-20260524T112750+0800.log',
    'docs/ltp-score-improvement-2026-05-24-phase-a/raw/stable270-la-aggregate-20260524T120322+0800.log',
    'docs/ltp-score-improvement-2026-05-24-phase-a/raw/target-post270-rv-clean-la-confirm-20260524T131247+0800.log',
    'docs/ltp-score-improvement-2026-05-24-phase-a/raw/target-post270-batch5-mixed-rv-20260524T131542+0800.log',
]
bad = []
total = 0
for log in logs:
    for i, line in enumerate(Path(log).read_bytes().splitlines(), 1):
        s = line.decode('utf-8', 'ignore')
        if 'PASS LTP CASE' in s or 'FAIL LTP CASE' in s:
            total += 1
            if not (s.startswith('PASS LTP CASE') or s.startswith('FAIL LTP CASE')):
                bad.append((log, i, s[:160]))
print(f'marker_lines={total} bad_marker_lines={len(bad)}')
PY
```

Result: `marker_lines=1118 bad_marker_lines=0`.

## Interpretation

- Fresh stable270 RV and LA aggregate markers remain at column 0.
- ANSI reset/color prefixes did not contaminate `PASS LTP CASE` / `FAIL LTP CASE` marker lines in the checked logs.
- This check is a parser/remote-scoring guardrail only; case promotion still depends on `scripts/ltp_summary.py` matrix categories.
