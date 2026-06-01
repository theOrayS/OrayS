# Session 1 Validation

## Commands run

```bash
pwd
git status --short
df -h / /root
python3 - <<'STABLE_COUNT'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
STABLE_COUNT
python3 -B scripts/ltp_summary.py --promotion-candidates \
  target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log \
  target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log
python3 -B scripts/ltp_summary.py --json target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log
python3 -B scripts/ltp_summary.py --json target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log
git diff --check -- docs/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix
```

## Parser-backed summary

- RV source summary: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/rv-arch002-summary.json`
  - raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log`
  - sha256: `70a3f9cab0c5c7a9a2743f168cabfd2eaafb7c01565b1630a02f9604aca5f096`
  - wrapper: PASS `1204`, FAIL `3453`, TIMEOUT `55`
  - internal: `{'TBROK': 1043, 'TCONF': 2663, 'TFAIL': 4058}`, ENOSYS `1280`, panic/trap `0`
  - marker audit: incomplete `0`, resource failure `0`, panic `0`, trap-like `0`
- LA source summary: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/la-arch012-summary.json`
  - raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log`
  - sha256: `41a5fdbba4a56a4ea76a168d2c9c6aa1e86a572d09de734a43a5365ec52c84df`
  - wrapper: PASS `1207`, FAIL `2698`, TIMEOUT `53`
  - internal: `{'TBROK': 1031, 'TCONF': 1936, 'TFAIL': 4041}`, ENOSYS `1279`, panic/trap `0`
  - marker audit: incomplete `0`, resource failure `0`, panic `0`, trap-like `0`
- Cross-arch/libc promotion-candidate parser: `563` clean four-combo cases, `1768` blocked/incomplete; `106` clean cases are not in live stable460.

## Team runtime note

`omx team 5:executor` was attempted after writing `.omx/context/ltp-long-term-0601-session1-*.md`, but OMX refused startup with `leader_workspace_dirty_for_worktrees` because the leader worktree contains pre-existing unrelated dirty/untracked files. Per repo AGENTS rules, those unrelated files were not stashed, committed, or reverted. Session 1 continued as leader-owned parser-backed analysis.

## Not run

- No targeted LTP runtime gate in Session 1; this session is report-only baseline/candidate selection.
- No stable regression gate because no stable list or code was modified.
