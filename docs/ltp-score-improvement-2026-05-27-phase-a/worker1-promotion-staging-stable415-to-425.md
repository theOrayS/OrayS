# Worker 1 promotion staging report: stable415 -> stable425 gap

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Worker: `worker-1`
Task: `13` — report-only promotion staging.

## Scope and guardrails

- Report-only output. This worker did **not** run QEMU/evaluator.
- Did **not** edit `.omx/ultragoal` or `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- This report stages leader-owned gates from existing parser evidence only; it is not a stable-list promotion.
- Promotion-clean still means RV+LA x musl+glibc wrapper `PASS` plus zero internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS/not-implemented, panic/trap, and marker anomalies.

## Live stable baseline

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` at report time:

```text
total=413 unique=413 duplicates=0
```

Membership check for current staging rows:

| Case | Live stable membership | Staging status |
| --- | --- | --- |
| `fcntl07` | not stable | four-way clean in task-10 batch evidence; candidate for leader final gate |
| `fcntl07_64` | not stable | four-way clean in task-10 batch evidence; candidate for leader final gate |
| `readlinkat02` | not stable | blocked: LA musl `TFAIL=1` |
| `pipe02` | not stable | quarantined: batch-002 RV musl `panic/trap=1` |
| `open06` | not stable | repair-validation only after FIFO `ENXIO` patch |
| `getdents02` | not stable | repair-validation only after metadata/getdents patch |
| `statfs03`, `statfs03_64` | not stable | repair-validation only after statfs parent-search patch |

## Why only `fcntl07` / `fcntl07_64` are clean now

Task-10 integrated these raw summaries:

- `raw/batch-001-rv-inline-summary.txt`
- `raw/batch-001-la-confirm-summary.txt`
- `raw/batch-001-cross-promotion-candidates.txt`
- `raw/batch-002-rv-summary.txt`

The cross-promotion candidate report has exactly two candidates:

```text
Promotion candidates: 2
fcntl07
fcntl07_64
```

The evidence is four-way clean for both rows:

- RV musl: wrapper `PASS`, no internal blockers.
- RV glibc: wrapper `PASS`, no internal blockers.
- LA musl: wrapper `PASS`, no internal blockers.
- LA glibc: wrapper `PASS`, no internal blockers.

That is sufficient to stage a **stable415-like** confirmation gate: live stable413 + two new cases = 415 cases per libc/arch if and only if the leader edits the stable list and final RV+LA parser gate remains clean.

## Why stable425 cannot be claimed yet

Stable425 would require 12 net new cases from live stable413, or 10 more after the `fcntl07`/`fcntl07_64` pair. Current evidence does not support that:

- `readlinkat02` is still 3/4 clean only: LA musl is wrapper `FAIL` with internal `TFAIL=1` in `batch-001-la-confirm-summary.txt` and is blocked in `batch-001-cross-promotion-candidates.txt`.
- Batch 002 cannot promote anything: `batch-002-rv-summary.txt` has only `rv:musl:pipe02 UNKNOWN` with `panic/trap=1`; the panic aborts the remaining rows before classification.
- `open06` has a narrow FIFO `ENXIO` repair report, but no post-repair RV+LA parser-clean evidence yet.
- `getdents02`, `statfs03`, and `statfs03_64` have narrow repair reports, but no post-repair RV+LA parser-clean evidence yet.
- Existing matrix rows for `getpgid01`, `kill02`, `inode02`, `poll02`, `getcpu01`, `gethostid01`, `gethostname02`, mmap/waitid cases, and many VFS rows still carry prior `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic, or aggregate-risk blockers.

Therefore the honest next target is staged as:

1. **stable415-like gate:** `fcntl07,fcntl07_64` only.
2. **stable425 gap work:** find 10 additional clean cases through repair-validation and clean scout batches; do not claim stable425 until fresh cross-promotion evidence exists.

## Leader serial gate plan

All commands below are for the leader's serialized QEMU/evaluator window. Worker-1 did not run them.

### A. Stable415-like confirmation gate for the clean fcntl pair

Use this gate only for the two currently clean candidates.

```bash
mkdir -p docs/ltp-score-improvement-2026-05-27-phase-a/raw
cat > docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-cases.txt <<'CASES'
fcntl07
fcntl07_64
CASES

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh rv \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-rv.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-rv.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-rv-summary.txt

# Run LA only if RV has wrapper PASS and zero internal blockers for both libcs.
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh la \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-la.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-la.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-la-summary.txt

python3 -B scripts/ltp_summary.py \
  --promotion-candidates \
  --promotion-arches rv,la \
  --promotion-libcs musl,glibc \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-rv.log \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-la.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable415-fcntl-pair-cross-promotion-candidates.txt
```

Stop/demotion rule: if either case has wrapper `FAIL`, internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap, or marker anomaly on any arch/libc row, do not promote it.

### B. Repair-validation gates after worker patches

These are not promotion batches by themselves. They validate whether recent patches created new candidates that can later feed stable425.

#### B1. `open06` FIFO repair validation

```bash
mkdir -p docs/ltp-score-improvement-2026-05-27-phase-a/raw
cat > docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-cases.txt <<'CASES'
open06
CASES

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh rv \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-rv.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-rv.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-rv-summary.txt

# LA only if RV is clean across musl+glibc.
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh la \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-la.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-la.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-open06-la-summary.txt
```

#### B2. Metadata/statfs/getdents repair validation

This checks the repaired focus rows plus stable-ish sentinel rows that protect errno and statfs semantics.

```bash
mkdir -p docs/ltp-score-improvement-2026-05-27-phase-a/raw
cat > docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-cases.txt <<'CASES'
getdents01
getdents02
statfs03
statfs03_64
statfs02
statfs02_64
statvfs02
CASES

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh rv \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-rv.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-rv.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-rv-summary.txt

# LA only for the subset that is RV-clean and never for rows with RV internal blockers.
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh la \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-la.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-la.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/repair-metadata-statfs-getdents-la-summary.txt
```

### C. Clean scout batch excluding panic/blockers

Batch-002 proved that `pipe02` must be excluded. Task-11 also reports that pipe-heavy/SIGPIPE/blocking-peer cases should stay out of mixed scout batches until the lock-order panic path is fixed; this demotes earlier `pipe07` optimism to a separate future check.

Use a non-pipe scout remainder first:

```bash
mkdir -p docs/ltp-score-improvement-2026-05-27-phase-a/raw
cat > docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-cases.txt <<'CASES'
dup05
select01
select02
select03
select04
sendfile07
sendfile07_64
CASES

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh rv \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-rv.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-rv.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-rv-summary.txt

# LA only for RV-clean rows. If some rows fail RV, write a reduced cases file first.
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-cases.txt \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 70m ./run-eval.sh la \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-la.log 2>&1
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-la.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-la-summary.txt

python3 -B scripts/ltp_summary.py \
  --promotion-candidates \
  --promotion-arches rv,la \
  --promotion-libcs musl,glibc \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-rv.log \
  docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-la.log \
  | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/scout-nonpipe-clean-remainder-cross-promotion-candidates.txt
```

Explicit exclusions for this scout:

- `pipe02`: panic/trap blocker.
- `pipe07`: hold until pipe lock-order/SIGPIPE path is fixed or isolated, despite earlier FD-lane optimism.
- `readlinkat02`: LA musl `TFAIL=1` blocker.
- `getpgid01`: prior RV rows have `TFAIL=1/TBROK=1`; do not spend clean-scout budget until repaired.
- Batch-001 failed VFS/metadata rows: keep for repair-validation batches, not clean scout.

## Leader stop conditions

- Promote only cases appearing in a cross-promotion report with all required arch/libc combos and zero blockers.
- If the fcntl pair gate is clean, the immediate honest count is stable415, not stable425.
- Do not combine panic-prone pipe rows with otherwise useful scout rows; one kernel panic invalidates the entire mixed batch for promotion accounting.
- Keep `read02` transparent TCONF handling unchanged and do not count any new TCONF as clean.

## Verification run by worker-1

```bash
# live stable count and membership
python3 - <<'PY'
import re, pathlib
s=pathlib.Path('examples/shell/src/cmd.rs').read_text()
m=re.search(r'LTP_STABLE_CASES:\s*&\[&str\]\s*=\s*&\[(.*?)\];', s, re.S)
items=re.findall(r'"([^"\\]+)"', m.group(1))
print(f'total={len(items)} unique={len(set(items))} duplicates={len(items)-len(set(items))}')
PY
# -> total=413 unique=413 duplicates=0

# evidence grep over matrix/reports/raw summaries
rg -n "Promotion candidates: 2|fcntl07|fcntl07_64|readlinkat02|pipe02|open06|statfs03|statfs03_64|getdents02" docs/ltp-score-improvement-2026-05-27-phase-a

# report-only hygiene
git diff --check
```

No QEMU/evaluator command was run by worker-1 for this task.
