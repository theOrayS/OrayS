# Worker 4 guardrail audit: Phase B promotion safety

Date: 2026-05-22
Worker: `worker-4`
Task: `5` / phase-b guardrail audit for promotion safety

## Scope

Audit current worker worktree source and Phase B setup for these promotion-safety risks:

- fake PASS or wrapper-output tricks that hide real failure
- case-name hardcoded success
- silent SKIP of real LTP cases
- timeout counted as PASS
- `LTP_STABLE_CASES` pollution without clean evidence
- accidental worker mutation of leader-owned `.omx/ultragoal`

This report is source/setup audit first. No Wave A raw logs or generated `*-candidate-matrix.md` / `*-promotion-matrix.md` artifacts are present in this worktree yet, so log-dependent promotion checks remain pending.

## Evidence commands

```text
rg -n "PASS LTP CASE|FAIL LTP CASE|TCONF|TBROK|TFAIL|SKIP|timeout|timed out|LTP_STABLE_CASES|LTP_CORE_CASES|hardcode|case_name|case name" examples/shell/src scripts docs/ltp-score-improvement-2026-05-22-phase-b -S
nl -ba examples/shell/src/cmd.rs | sed -n '40,80p;540,665p;1435,1520p'
nl -ba scripts/ltp_summary.py | sed -n '1,45p;190,215p;270,305p;490,548p'
python3 - <<'PY'
import re, pathlib
s = pathlib.Path('examples/shell/src/cmd.rs').read_text()
m = re.search(r'const LTP_STABLE_CASES: &\[&str\] = &\[(.*?)\];', s, re.S)
items = re.findall(r'"([^"]+)"', m.group(1)) if m else []
from collections import Counter
c = Counter(items)
print('stable_count', len(items))
print('unique_count', len(c))
print('duplicates', [k for k,v in c.items() if v > 1])
print('last', items[-10:])
PY
find docs/ltp-score-improvement-2026-05-22-phase-b -type f \( -name '*log*' -o -name '*summary*' -o -name '*matrix*' -o -name '*wave*' \) -print | sort
git status --short -- .omx/ultragoal docs/ltp-score-improvement-2026-05-22-phase-b
```

## Findings

### 1. Fake PASS / wrapper-output risk

Status: **guardrail present, but wording remains intentionally parser-compatible and should be treated carefully.**

- `examples/shell/src/cmd.rs:1491-1500` prints `FAIL LTP CASE {case} : 0` plus `Pass!` only when the user program exits with `Ok(0)`.
- The in-source comment at `examples/shell/src/cmd.rs:1492-1497` says this is the remote score parser-compatible wire format, not an unconditional PASS.
- `scripts/ltp_summary.py:108-118` normalizes wrapper status from the numeric code: only code `0` is semantic PASS; every non-zero code remains FAIL even if a misleading token appears.

Risk: downstream humans may misread `FAIL LTP CASE ... : 0` without `scripts/ltp_summary.py`, but current source does not fake a PASS for non-zero exits.

### 2. Case-name hardcoded success risk

Status: **no case-name hardcoded success found in audited LTP runner path.**

- The only direct case-name branch found in the LTP execution path is `ltp_case_env()` for `chdir01` at `examples/shell/src/cmd.rs:1200-1214`.
- That branch injects environment/device settings for the real `chdir01` body; it does not bypass execution or force status `0`.
- No `match case` or `case == ...` branch was found that converts a specific LTP case to PASS.

Risk: future near-clean fixes must remain semantic fixes, not per-case success overrides.

### 3. Silent SKIP risk

Status: **no silent LTP case SKIP found in `run_ltp_suite`; missing tests fail loudly.**

- Missing LTP testcase binaries emit `FAIL LTP CASE {case} : -1`, a missing-path message, runtime, END marker, and increment `failed` at `examples/shell/src/cmd.rs:1461-1471`.
- Disabled official test groups are printed as `autorun: skip disabled test group ...` at `examples/shell/src/cmd.rs:1585-1587`, but this is group-level outside the selected LTP stable case loop and is visible, not silent.

Risk: log summaries should continue to include non-LTP evaluator caveats separately from stable LTP promotion evidence.

### 4. Timeout-as-PASS risk

Status: **guardrail present.**

- LTP timeout configuration is explicit via `LTP_CASE_TIMEOUT_SECS`, defaulting to 15 seconds at `examples/shell/src/cmd.rs:546` and `examples/shell/src/cmd.rs:656-663`.
- Exit statuses `137` and `143` print `FAIL LTP CASE {case} : {status}`, print `TIMEOUT LTP CASE`, increment both `failed` and `timed_out`, and do not increment `passed` at `examples/shell/src/cmd.rs:1502-1507`.
- Error strings containing timeout also print `TIMEOUT LTP CASE`, increment `timed_out`, and increment `failed` at `examples/shell/src/cmd.rs:1512-1520`.
- The suite summary prints separate `passed`, `failed`, and `timed out` totals at `examples/shell/src/cmd.rs:1532-1534`.
- `scripts/ltp_summary.py:31-34` detects timeout markers; `scripts/ltp_summary.py:204-208` increments timeout counters; `scripts/ltp_summary.py:296-297` categorizes timeout rows outside clean promotion.

Risk: none found in current source; promotion reports must still require timeout count `0` for newly stable cases.

### 5. `LTP_STABLE_CASES` pollution risk

Status: **current stable list shape matches the documented stable101 baseline; no duplicate pollution found.**

- `examples/shell/src/cmd.rs:49-150` defines checked-in `LTP_STABLE_CASES`.
- Local parse result: `stable_count 101`, `unique_count 101`, `duplicates []`.
- Last ten current stable cases are `sigaction02`, `sigprocmask01`, `sigsuspend01`, `dup04`, `fchmod03`, `pipe03`, `waitpid06`, `waitpid07`, `waitpid08`, `waitpid09`, matching the Phase B baseline artifact's stable101 framing.
- `docs/ltp-score-improvement-2026-05-22-phase-b/phase-b-baseline-artifact.md:17-28` records stable101 with LA/RV stable LTP `202` wrapper passes, `0` wrapper fails, internal `TFAIL=0`, `TBROK=0`, `TCONF=4`, and timeout/ENOSYS/panic-trap `0`.
- `docs/ltp-score-improvement-2026-05-22-phase-b/plan-stable101-to-125.md:12-16` requires only LA/RV x musl/glibc clean cases enter `LTP_STABLE_CASES` and explicitly forbids fake PASS, case-name hardcoding, silent SKIP, and timeout-as-PASS.

Risk: no current source pollution detected. Wave A promotion must not edit `LTP_STABLE_CASES` until logs/matrices prove clean LA/RV x musl/glibc behavior.

### 6. Log-dependent checks still pending

Status: **pending because no Wave A logs/matrices are present in this worker worktree.**

- `find docs/ltp-score-improvement-2026-05-22-phase-b -type f \( -name '*log*' -o -name '*summary*' -o -name '*matrix*' -o -name '*wave*' \)` returned no files.
- Therefore this audit cannot yet confirm whether future Wave A candidates are clean across LA/RV x musl/glibc.

Required follow-up once logs exist:

```text
python3 scripts/ltp_summary.py --promotion-candidates <rv-log> <la-log>
```

Reject promotion if any candidate row has wrapper FAIL, internal `TFAIL`/`TBROK`, unaccepted `TCONF`, timeout, ENOSYS, panic/trap, or event failures.

### 7. `.omx/ultragoal` ownership

Status: **not mutated by this task.**

- `git status --short -- .omx/ultragoal` produced no output during the audit.
- The only intended repository change from task 5 is this markdown report under `docs/ltp-score-improvement-2026-05-22-phase-b/`.

## Recommendation

Do not promote new cases in Phase B until a promotion matrix exists for both LA and RV logs and shows clean rows for each libc variant. Current runner and summary code have the required guardrails for honest accounting, but the next risk point is human/process error: editing `LTP_STABLE_CASES` before saving and parsing Wave A evidence.
