# Live `LTP_STABLE_CASES` evidence

Date: 2026-06-07
Scope: read-only evidence for G001 Phase 0.

## Source

- File: `examples/shell/src/cmd.rs`
- Symbol: `LTP_STABLE_CASES`
- Live source range in this checkout: `examples/shell/src/cmd.rs:50-619`

## Result

| Metric | Value |
| --- | ---: |
| Total entries | 1000 |
| Unique cases | 1000 |
| Duplicate extra entries | 0 |
| Duplicate case names | 0 |

Duplicate map: `{}`

## Command used

```bash
python3 - <<'PY'
from pathlib import Path
import re, collections, json
p=Path('examples/shell/src/cmd.rs')
text=p.read_text()
m=re.search(r'const\s+LTP_STABLE_CASES\s*:\s*&\[&str\]\s*=\s*&\[', text)
if not m:
    raise SystemExit('LTP_STABLE_CASES not found')
start=m.end()
level=1; i=start; in_str=False; esc=False; line_comment=False; block_comment=0
while i < len(text):
    c=text[i]; nxt=text[i+1] if i+1 < len(text) else ''
    if line_comment:
        if c=='\n': line_comment=False
    elif block_comment:
        if c=='/' and nxt=='*': block_comment+=1; i+=1
        elif c=='*' and nxt=='/': block_comment-=1; i+=1
    elif in_str:
        if esc: esc=False
        elif c=='\\': esc=True
        elif c=='"': in_str=False
    else:
        if c=='/' and nxt=='/': line_comment=True; i+=1
        elif c=='/' and nxt=='*': block_comment=1; i+=1
        elif c=='"': in_str=True
        elif c=='[': level+=1
        elif c==']':
            level-=1
            if level==0:
                end=i
                break
    i+=1
else:
    raise SystemExit('unterminated LTP_STABLE_CASES')
body=text[start:end]
cases=re.findall(r'"([^"\\]*(?:\\.[^"\\]*)*)"', body)
cnt=collections.Counter(cases)
dups={k:v for k,v in cnt.items() if v>1}
line_start=text[:m.start()].count('\n')+1
line_end=text[:end].count('\n')+1
print(json.dumps({
    'file':str(p),
    'line_start':line_start,
    'line_end':line_end,
    'total':len(cases),
    'unique':len(cnt),
    'duplicate_entries':sum(v-1 for v in dups.values()),
    'duplicate_case_count':len(dups),
    'duplicates':dups,
}, ensure_ascii=False, indent=2))
PY
```

## Output

```json
{
  "file": "examples/shell/src/cmd.rs",
  "line_start": 50,
  "line_end": 619,
  "total": 1000,
  "unique": 1000,
  "duplicate_entries": 0,
  "duplicate_case_count": 0,
  "duplicates": {}
}
```

## Interpretation

This is a live count, not promotion proof. It must be paired with parser-backed RV + LA × musl + glibc evidence and real behavior checks before future stable claims. Do not use remembered stable baselines, wrapper-only status0, blacklist runs, synthetic probe success, or full-sweep partial `TPASS` as a substitute.
