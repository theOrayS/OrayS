# LTP real-semantics repair baseline (G001)

Date: 2026-06-07
Ultragoal story: `G001-g001-phase-0-quarantine`
Scope: documentation/baseline only; no source-code edits or stable-list edits in this phase.

## Source of truth

This baseline follows:

- `.omx/plans/ltp-real-semantics-repair-plan-2026-06-07.md`
- `.omx/context/ltp-real-semantics-repair-20260607T151429Z.md`
- live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`
- `scripts/ltp_summary.py` and `scripts/test_ltp_summary.py` parser semantics

G001 records the current truthfulness boundary before implementation work starts.  It does **not** promote or demote any case, does **not** edit `.omx/ultragoal`, and does **not** claim runtime LTP evidence.

## Deliverables

- [`fake-implementation-inventory.md`](fake-implementation-inventory.md) — initial inventory of fake-success, weak-success, runner-specialization, synthetic-façade, and runtime-patch risks.
- [`promotion-quarantine.md`](promotion-quarantine.md) — quarantine and promotion rules for evidence that must not be used to expand `LTP_STABLE_CASES`.
- [`ltp-stable-cases-evidence.md`](ltp-stable-cases-evidence.md) — machine-readable live stable-count command/output evidence for total/unique/duplicate checks.
- This README — live stable-count summary, verification notes, and integration boundaries.

## Live `LTP_STABLE_CASES` evidence

The stable list was re-read from the current worktree, not from prior reports or memory.

Command:

```bash
python3 - <<'PY'
import re
from pathlib import Path
from collections import Counter
p = Path('examples/shell/src/cmd.rs')
text = p.read_text()
m = re.search(r'(?:const|static)\s+LTP_STABLE_CASES\s*:[^=]+=', text)
if not m:
    raise SystemExit('assignment not found')
start_idx = text.find('&[', m.end())
if start_idx == -1:
    raise SystemExit('initializer not found')
br = text.find('[', start_idx)
depth = 0
in_str = False
esc = False
end_idx = None
for i, ch in enumerate(text[br:], br):
    if in_str:
        if esc:
            esc = False
        elif ch == '\\':
            esc = True
        elif ch == '"':
            in_str = False
    else:
        if ch == '"':
            in_str = True
        elif ch == '[':
            depth += 1
        elif ch == ']':
            depth -= 1
            if depth == 0:
                end_idx = i + 1
                break
body = text[br:end_idx]
items = re.findall(r'"([^"\\]*(?:\\.[^"\\]*)*)"', body)
c = Counter(items)
dups = {k: v for k, v in c.items() if v > 1}
print(f'const_line_range={text.count(chr(10), 0, m.start()) + 1}-{text.count(chr(10), 0, end_idx) + 1}')
print(f'total={len(items)}')
print(f'unique={len(c)}')
print(f'duplicate_extra_entries={len(items) - len(c)}')
print('duplicate_names_with_counts=' + ('<none>' if not dups else ', '.join(f'{k}:{v}' for k, v in sorted(dups.items()))))
PY
```

Observed output:

```text
const_line_range=50-619
total=1000
unique=1000
duplicate_extra_entries=0
duplicate_names_with_counts=<none>
```

Interpretation:

- Current stable whitelist has **1000 total entries**.
- Current stable whitelist has **1000 unique entries**.
- Current duplicate count is **0 extra duplicate entries** and **no duplicate names**.
- These numbers are a baseline only.  They are not promotion evidence and do not prove real Linux/POSIX semantics for each listed case.

## G001 quarantine summary

The plan identifies multiple places where a passing wrapper status can be decoupled from real behavior:

1. **Unimplemented interfaces returning success** — e.g. TODO paths returning `0`/`Ok(0)` in POSIX API and libc wrappers.
2. **Weak success semantics** — calls accept or store inputs without observable Linux/POSIX behavior, such as non-delivering timers or mempolicy validation without policy effect.
3. **Runner selection/specialization risk** — `stable-plus-blacklist` default, guest case override files, and `chdir01`-specific environment injection can confuse audit evidence with promotion evidence.
4. **Synthetic façade risk** — `/proc`, `/dev`, `/etc`, kernel config, and synthetic block devices can help compatibility but must not advertise unsupported kernel capabilities.
5. **Runtime libc patch risk** — musl byte patches are temporary shims and cannot be the sole basis for stable promotion.

Short-term stable-count decreases caused by removing fake success are acceptable truthfulness repairs.  A lower honest count is preferable to a higher count based on fake PASS, status0-only, hidden TCONF/TBROK/TFAIL/ENOSYS, timeout masking, panic/trap masking, or synthetic probe-only evidence.

## Verification performed for this baseline

G001 is docs-only, so verification focuses on parser semantics, live source re-read, and file/diff hygiene:

- `python3 scripts/test_ltp_summary.py` — parser regression suite for numeric status, timeout, TCONF/TBROK/TFAIL/ENOSYS promotion blocking.
- `python3 scripts/ltp_summary.py --help` — confirms numeric wrapper status is parser truth and quality signals are tracked.
- Live Python extraction of `examples/shell/src/cmd.rs::LTP_STABLE_CASES` — total/unique/duplicate evidence above.
- `test -f` checks for all three generated docs.
- `git diff --check -- docs/ltp-real-semantics-repair-2026-06-07` — whitespace/diff hygiene.

Not run in G001: QEMU, remote evaluator, full LTP runtime, `make all`, `cargo check`, or source-code builds.  Those are required in later implementation/promotion phases, not for this documentation baseline.

## Coordination and ownership boundaries

- Leader owns `.omx/ultragoal` checkpointing for `G001-g001-phase-0-quarantine`.
- Worker output is limited to G001 docs and team task evidence.
- This baseline intentionally preserves unrelated dirty files and does not touch source code.
- Boundary checked: sibling team tasks were in progress while this worker wrote an independent docs baseline in its own worker worktree; leader integration remains the source of truth for merging task outputs.
