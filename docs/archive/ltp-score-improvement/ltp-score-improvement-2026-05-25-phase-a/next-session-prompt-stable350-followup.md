# Next Session Prompt: stable350 Follow-up

We are on `/root/oskernel2026-orays`. The 2026-05-25 phase-a campaign delivered stable350.

## Must live-refresh first

Do not rely only on this prompt. Recompute from live tree:

```bash
git status --short
df -h / /root
du -sh /root/.codex
python3 - <<'PY'
import re
from collections import Counter
from pathlib import Path
s=Path('examples/shell/src/cmd.rs').read_text()
m=re.search(r'const LTP_STABLE_CASES: &\[&str\] = &\[(.*?)\];',s,re.S)
items=re.findall(r'"([^"]+)"', m.group(1)) if m else []
c=Counter(items)
print(len(items), len(c), [k for k,v in c.items() if v>1])
PY
```

Expected at handoff time: 350 total / 350 unique / 0 duplicates; `kill02` absent; `abs01` present.

## Stable350 evidence to recheck

- `docs/ltp-score-improvement-2026-05-25-phase-a/stable350-delivery-report.md`
- `docs/ltp-score-improvement-2026-05-25-phase-a/final-gate-quality-gate.json`
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-final-002-summary.txt`
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-la-final-002-summary.txt`
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-final-002-marker-prefix.txt`
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-la-final-002-marker-prefix.txt`

Final stable350 gate at handoff: RV and LA each PASS LTP CASE 700 / FAIL 0; musl 350/0; glibc 350/0; known read02 TCONF only; timeout/ENOSYS/panic/trap 0; marker bad=0.

## Recommended next target

Target stable380 or stable400, not by random additions but by high-yield clusters with fresh targeted evidence before promotion.

Priority blockers / candidates:

- `kill02`: demoted because LA glibc aggregate failed with TBROK in `raw/stable350-la-final-summary.txt`; inspect signal/kill setup semantics before retry.
- `readlinkat02`: near-miss around O_PATH / AT_EMPTY_PATH / symlink semantics.
- User-priority blockers not yet clean: `access02`, `access04`, `chmod05`, `statx01`, `writev03`, `mmap04`, `mmap05`, `mmap06`, `mprotect01`, `mprotect02`, `munmap01`.
- Additional clusters: signal/pause/kill tail, fcntl tail, metadata/time cases, and low-risk libc/static cases only after RV+LA × musl+glibc clean evidence.

## Guardrails

- No fake PASS, no case-name hardcoding, no test-source edits, no failure laundering.
- Wrapper success is insufficient; parse logs with `python3 -B scripts/ltp_summary.py`.
- Do not count timeout as PASS.
- Keep `read02` pass-with-TCONF visible.
- Do not let parallel workers contend on default QEMU sdcard/qcow2 paths; promotion/final gates should be leader-owned and serial unless isolated images are proven.
- Preserve marker lines at column 0.
