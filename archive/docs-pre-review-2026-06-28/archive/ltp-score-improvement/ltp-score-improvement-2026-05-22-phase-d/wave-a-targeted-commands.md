# Phase-D Wave A targeted commands

Date: 2026-05-22

## Scope and guardrails

- Worker lane: command design only; do **not** edit `.omx/ultragoal` and do **not** edit `LTP_STABLE_CASES`.
- Inputs are the worker-1 Wave A files in `docs/ltp-score-improvement-2026-05-22-phase-d/`:
  - `wave-a-selected-180.cases`
  - `wave-a-batch1-stable-core.cases`
  - `wave-a-batch2-stable-mid.cases`
  - `wave-a-batch3-stable-recent.cases`
  - `wave-a-batch4-stable180-newcases.cases`
- Runner contract from `examples/shell/src/cmd.rs`: set `OSCOMP_TEST_GROUPS=ltp`, pass targeted cases through `LTP_CASES`, and control per-case timeout through `LTP_CASE_TIMEOUT_SECS`.
- Use inline `LTP_CASES="$(paste -sd, "$case_file")"` rather than `file:<host-path>` so the compile-time `option_env!("LTP_CASES")` embeds the exact case list and does not depend on guest-visible host paths.
- Every evaluator log must be parsed with `scripts/ltp_summary.py`; wrapper exit status alone is not promotion evidence.

## Shared shell setup

Run from the worker worktree root:

```bash
cd /root/oskernel2026-orays/.omx/team/phase-d-ltp-stable-sc-ae18f5c7/worktrees/worker-1
export PHASE_D_DIR="$PWD/docs/ltp-score-improvement-2026-05-22-phase-d"
export RV_TESTSUITE_IMG="${RV_TESTSUITE_IMG:-/root/oskernel2026-orays/sdcard-rv.img}"
export LA_TESTSUITE_IMG="${LA_TESTSUITE_IMG:-/root/oskernel2026-orays/sdcard-la.img}"
export LTP_CASE_TIMEOUT_SECS=20
export PHASE_D_RAW="$PHASE_D_DIR/raw"
mkdir -p "$PHASE_D_RAW"
```

`LTP_CASE_TIMEOUT_SECS=20` matches the Phase-C targeted-run convention. Increase only if the leader explicitly accepts the longer gate; never count a timeout as PASS.


## Leader-requested focused commands: selected-180 and stable180-newcases batch

These two focused probes are the fastest task-5 path. They do **not** run QEMU until a human/leader copies them into a shell.

### RV first: smaller stable180-newcases batch

```bash
case_file="$PHASE_D_DIR/wave-a-batch4-stable180-newcases.cases"
cases="$(paste -sd, "$case_file")"
count="$(wc -l < "$case_file" | tr -d ' ')"
log="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-rv.log"
status="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-rv.status"
summary="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-rv-summary.txt"
summary_json="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-rv-summary.json"
{ echo "command: RV_TESTSUITE_IMG=$RV_TESTSUITE_IMG OSCOMP_TEST_GROUPS=ltp LTP_CASES=<${count}-case-inline-from-${case_file}> LTP_CASE_TIMEOUT_SECS=$LTP_CASE_TIMEOUT_SECS ./run-eval.sh"; echo "started_at: $(date -Is)"; echo "case_count: $count"; } > "$status"
set +e
RV_TESTSUITE_IMG="$RV_TESTSUITE_IMG" OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" ./run-eval.sh 2>&1 | tee "$log"
rc=${PIPESTATUS[0]}
set -e
{ echo "exit: $rc"; echo "finished_at: $(date -Is)"; } >> "$status"
python3 -B scripts/ltp_summary.py "$log" | tee "$summary"
python3 -B scripts/ltp_summary.py --json "$log" > "$summary_json"
exit "$rc"
```

### LA follow-up: smaller stable180-newcases batch

Run only if the RV smaller batch has no disqualifying failure signal.

```bash
case_file="$PHASE_D_DIR/wave-a-batch4-stable180-newcases.cases"
cases="$(paste -sd, "$case_file")"
count="$(wc -l < "$case_file" | tr -d ' ')"
log="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-la.log"
status="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-la.status"
summary="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-la-summary.txt"
summary_json="$PHASE_D_RAW/wave-a-batch4-stable180-newcases-la-summary.json"
{ echo "command: LA_TESTSUITE_IMG=$LA_TESTSUITE_IMG OSCOMP_TEST_GROUPS=ltp LTP_CASES=<${count}-case-inline-from-${case_file}> LTP_CASE_TIMEOUT_SECS=$LTP_CASE_TIMEOUT_SECS ./run-eval.sh la"; echo "started_at: $(date -Is)"; echo "case_count: $count"; } > "$status"
set +e
LA_TESTSUITE_IMG="$LA_TESTSUITE_IMG" OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" ./run-eval.sh la 2>&1 | tee "$log"
rc=${PIPESTATUS[0]}
set -e
{ echo "exit: $rc"; echo "finished_at: $(date -Is)"; } >> "$status"
python3 -B scripts/ltp_summary.py "$log" | tee "$summary"
python3 -B scripts/ltp_summary.py --json "$log" > "$summary_json"
exit "$rc"
```

### RV first: full selected-180 cumulative probe

```bash
case_file="$PHASE_D_DIR/wave-a-selected-180.cases"
cases="$(paste -sd, "$case_file")"
count="$(wc -l < "$case_file" | tr -d ' ')"
log="$PHASE_D_RAW/wave-a-selected-180-rv.log"
status="$PHASE_D_RAW/wave-a-selected-180-rv.status"
summary="$PHASE_D_RAW/wave-a-selected-180-rv-summary.txt"
summary_json="$PHASE_D_RAW/wave-a-selected-180-rv-summary.json"
{ echo "command: RV_TESTSUITE_IMG=$RV_TESTSUITE_IMG OSCOMP_TEST_GROUPS=ltp LTP_CASES=<${count}-case-inline-from-${case_file}> LTP_CASE_TIMEOUT_SECS=$LTP_CASE_TIMEOUT_SECS ./run-eval.sh"; echo "started_at: $(date -Is)"; echo "case_count: $count"; } > "$status"
set +e
RV_TESTSUITE_IMG="$RV_TESTSUITE_IMG" OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" ./run-eval.sh 2>&1 | tee "$log"
rc=${PIPESTATUS[0]}
set -e
{ echo "exit: $rc"; echo "finished_at: $(date -Is)"; } >> "$status"
python3 -B scripts/ltp_summary.py "$log" | tee "$summary"
python3 -B scripts/ltp_summary.py --json "$log" > "$summary_json"
exit "$rc"
```

### LA follow-up: full selected-180 cumulative probe

Run only after RV selected-180 is acceptable to the leader.

```bash
case_file="$PHASE_D_DIR/wave-a-selected-180.cases"
cases="$(paste -sd, "$case_file")"
count="$(wc -l < "$case_file" | tr -d ' ')"
log="$PHASE_D_RAW/wave-a-selected-180-la.log"
status="$PHASE_D_RAW/wave-a-selected-180-la.status"
summary="$PHASE_D_RAW/wave-a-selected-180-la-summary.txt"
summary_json="$PHASE_D_RAW/wave-a-selected-180-la-summary.json"
{ echo "command: LA_TESTSUITE_IMG=$LA_TESTSUITE_IMG OSCOMP_TEST_GROUPS=ltp LTP_CASES=<${count}-case-inline-from-${case_file}> LTP_CASE_TIMEOUT_SECS=$LTP_CASE_TIMEOUT_SECS ./run-eval.sh la"; echo "started_at: $(date -Is)"; echo "case_count: $count"; } > "$status"
set +e
LA_TESTSUITE_IMG="$LA_TESTSUITE_IMG" OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" ./run-eval.sh la 2>&1 | tee "$log"
rc=${PIPESTATUS[0]}
set -e
{ echo "exit: $rc"; echo "finished_at: $(date -Is)"; } >> "$status"
python3 -B scripts/ltp_summary.py "$log" | tee "$summary"
python3 -B scripts/ltp_summary.py --json "$log" > "$summary_json"
exit "$rc"
```

## RV-first targeted batch runs

Run these before any LA follow-up. Each command writes a raw log, status file, Markdown summary, JSON summary, and promotion-candidate report.

```bash
for batch in \
  wave-a-batch1-stable-core \
  wave-a-batch2-stable-mid \
  wave-a-batch3-stable-recent \
  wave-a-batch4-stable180-newcases
 do
  case_file="$PHASE_D_DIR/${batch}.cases"
  log="$PHASE_D_RAW/${batch}-rv.log"
  status="$PHASE_D_RAW/${batch}-rv.status"
  summary="$PHASE_D_RAW/${batch}-rv-summary.txt"
  summary_json="$PHASE_D_RAW/${batch}-rv-summary.json"
  candidates="$PHASE_D_RAW/${batch}-rv-promotion-candidates.md"
  cases="$(paste -sd, "$case_file")"
  count="$(wc -l < "$case_file" | tr -d ' ')"

  {
    echo "command: RV_TESTSUITE_IMG=$RV_TESTSUITE_IMG OSCOMP_TEST_GROUPS=ltp LTP_CASES=<${count}-case-inline-from-${case_file}> LTP_CASE_TIMEOUT_SECS=$LTP_CASE_TIMEOUT_SECS ./run-eval.sh"
    echo "started_at: $(date -Is)"
    echo "case_count: $count"
  } > "$status"

  set +e
  RV_TESTSUITE_IMG="$RV_TESTSUITE_IMG" \
  OSCOMP_TEST_GROUPS=ltp \
  LTP_CASES="$cases" \
  LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" \
    ./run-eval.sh 2>&1 | tee "$log"
  rc=${PIPESTATUS[0]}
  set -e

  {
    echo "exit: $rc"
    echo "finished_at: $(date -Is)"
  } >> "$status"

  python3 -B scripts/ltp_summary.py "$log" | tee "$summary"
  python3 -B scripts/ltp_summary.py --json "$log" > "$summary_json"
  python3 -B scripts/ltp_summary.py --promotion-candidates --promotion-arches rv --promotion-libcs musl,glibc "$log" > "$candidates"

  if [ "$rc" -ne 0 ]; then
    echo "RV batch $batch exited $rc; inspect $log and $summary before continuing." >&2
    exit "$rc"
  fi
 done
```

### RV stop condition

Stop before LA and report the blocker if any RV batch summary has any of:

- `FAIL LTP CASE` greater than 0.
- Internal `TFAIL` or `TBROK` greater than 0.
- `timeout matches` greater than 0.
- `ENOSYS/not implemented matches` greater than 0.
- `panic/trap matches` greater than 0.
- New unexpected `TCONF`. The known `read02` TCONF may still appear when the stable guard includes `read02`; it must stay visible in the summary and cannot be silently treated as a clean PASS.

Quick gate helper:

```bash
python3 - <<'PY'
import json, pathlib, sys
base = pathlib.Path('docs/ltp-score-improvement-2026-05-22-phase-d/raw')
failed = []
for p in sorted(base.glob('wave-a-batch*-rv-summary.json')):
    data = json.loads(p.read_text())
    bad = []
    if data.get('fail_count', 0): bad.append(f"wrapper_fail={data['fail_count']}")
    internal = data.get('internal', {}) or {}
    if internal.get('TFAIL', 0): bad.append(f"TFAIL={internal['TFAIL']}")
    if internal.get('TBROK', 0): bad.append(f"TBROK={internal['TBROK']}")
    if data.get('timeouts', 0): bad.append(f"timeout={data['timeouts']}")
    if data.get('enosys', 0): bad.append(f"ENOSYS={data['enosys']}")
    if data.get('panic_trap', 0): bad.append(f"panic_trap={data['panic_trap']}")
    tconf = internal.get('TCONF', 0)
    if tconf:
        bad.append(f"visible_TCONF={tconf}")
    if bad:
        failed.append(f"{p.name}: " + ', '.join(bad))
if failed:
    print('RV gate needs review before LA:')
    print('\n'.join(failed))
    sys.exit(1)
print('RV gate structurally clean for LA follow-up')
PY
```

## LA follow-up targeted batch runs

Run only after the RV helper is clean or after the leader explicitly accepts a visible known caveat. Use the same batch files and timeout.

```bash
for batch in \
  wave-a-batch1-stable-core \
  wave-a-batch2-stable-mid \
  wave-a-batch3-stable-recent \
  wave-a-batch4-stable180-newcases
 do
  case_file="$PHASE_D_DIR/${batch}.cases"
  log="$PHASE_D_RAW/${batch}-la.log"
  status="$PHASE_D_RAW/${batch}-la.status"
  summary="$PHASE_D_RAW/${batch}-la-summary.txt"
  summary_json="$PHASE_D_RAW/${batch}-la-summary.json"
  candidates="$PHASE_D_RAW/${batch}-la-promotion-candidates.md"
  cases="$(paste -sd, "$case_file")"
  count="$(wc -l < "$case_file" | tr -d ' ')"

  {
    echo "command: LA_TESTSUITE_IMG=$LA_TESTSUITE_IMG OSCOMP_TEST_GROUPS=ltp LTP_CASES=<${count}-case-inline-from-${case_file}> LTP_CASE_TIMEOUT_SECS=$LTP_CASE_TIMEOUT_SECS ./run-eval.sh la"
    echo "started_at: $(date -Is)"
    echo "case_count: $count"
  } > "$status"

  set +e
  LA_TESTSUITE_IMG="$LA_TESTSUITE_IMG" \
  OSCOMP_TEST_GROUPS=ltp \
  LTP_CASES="$cases" \
  LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" \
    ./run-eval.sh la 2>&1 | tee "$log"
  rc=${PIPESTATUS[0]}
  set -e

  {
    echo "exit: $rc"
    echo "finished_at: $(date -Is)"
  } >> "$status"

  python3 -B scripts/ltp_summary.py "$log" | tee "$summary"
  python3 -B scripts/ltp_summary.py --json "$log" > "$summary_json"
  python3 -B scripts/ltp_summary.py --promotion-candidates --promotion-arches la --promotion-libcs musl,glibc "$log" > "$candidates"

  if [ "$rc" -ne 0 ]; then
    echo "LA batch $batch exited $rc; inspect $log and $summary before continuing." >&2
    exit "$rc"
  fi
 done
```

## Cross-arch promotion-candidate synthesis

After all RV and LA batch summaries exist, synthesize candidate evidence across both arches. This is still a report, not a stable-list edit.

```bash
python3 -B scripts/ltp_summary.py --promotion-candidates \
  "$PHASE_D_RAW"/wave-a-batch*-rv.log \
  "$PHASE_D_RAW"/wave-a-batch*-la.log \
  > "$PHASE_D_RAW/wave-a-rv-la-promotion-candidates.md"
```

Recommended leader-facing matrix check:

```bash
python3 - <<'PY'
import json, pathlib, sys
base = pathlib.Path('docs/ltp-score-improvement-2026-05-22-phase-d/raw')
failed = []
for p in sorted(base.glob('wave-a-batch*-*-summary.json')):
    data = json.loads(p.read_text())
    internal = data.get('internal', {}) or {}
    bad = []
    if data.get('fail_count', 0): bad.append(f"wrapper_fail={data['fail_count']}")
    for key in ('TFAIL', 'TBROK'):
        if internal.get(key, 0): bad.append(f"{key}={internal[key]}")
    for key, label in (('timeouts', 'timeout'), ('enosys', 'ENOSYS'), ('panic_trap', 'panic_trap')):
        if data.get(key, 0): bad.append(f"{label}={data[key]}")
    if bad:
        failed.append(f"{p.name}: " + ', '.join(bad))
if failed:
    print('Promotion stop: non-clean targeted evidence')
    print('\n'.join(failed))
    sys.exit(1)
print('Promotion candidate evidence is structurally clean; leader must still review visible TCONF and summaries before any LTP_STABLE_CASES edit')
PY
```

## Optional single cumulative stable180 probe

Use this only after the four split batches are clean enough for leader review. It tests the exact 180-case selected set in one run and can expose cumulative timeout/resource regressions that split batches miss.

```bash
for arch in rv la; do
  case_file="$PHASE_D_DIR/wave-a-selected-180.cases"
  cases="$(paste -sd, "$case_file")"
  log="$PHASE_D_RAW/wave-a-selected-180-${arch}.log"
  status="$PHASE_D_RAW/wave-a-selected-180-${arch}.status"
  summary="$PHASE_D_RAW/wave-a-selected-180-${arch}-summary.txt"
  summary_json="$PHASE_D_RAW/wave-a-selected-180-${arch}-summary.json"
  count="$(wc -l < "$case_file" | tr -d ' ')"
  run_arg=""
  img_env="RV_TESTSUITE_IMG=$RV_TESTSUITE_IMG"
  [ "$arch" = la ] && run_arg="la" && img_env="LA_TESTSUITE_IMG=$LA_TESTSUITE_IMG"

  {
    echo "command: $img_env OSCOMP_TEST_GROUPS=ltp LTP_CASES=<${count}-case-inline-from-${case_file}> LTP_CASE_TIMEOUT_SECS=$LTP_CASE_TIMEOUT_SECS ./run-eval.sh ${run_arg}"
    echo "started_at: $(date -Is)"
    echo "case_count: $count"
  } > "$status"

  set +e
  if [ "$arch" = rv ]; then
    RV_TESTSUITE_IMG="$RV_TESTSUITE_IMG" OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" ./run-eval.sh 2>&1 | tee "$log"
  else
    LA_TESTSUITE_IMG="$LA_TESTSUITE_IMG" OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS="$LTP_CASE_TIMEOUT_SECS" ./run-eval.sh la 2>&1 | tee "$log"
  fi
  rc=${PIPESTATUS[0]}
  set -e

  { echo "exit: $rc"; echo "finished_at: $(date -Is)"; } >> "$status"
  python3 -B scripts/ltp_summary.py "$log" | tee "$summary"
  python3 -B scripts/ltp_summary.py --json "$log" > "$summary_json"
  [ "$rc" -eq 0 ] || exit "$rc"
 done
```

## Review probe integration

The required Task 5 review probe was integrated into this runbook:

- Use worktree-safe image paths (`/root/oskernel2026-orays/sdcard-*.img`) because the worker worktree may not contain sdcard images.
- Use inline `LTP_CASES` generated from `.cases` files; avoid `file:<host-path>` because the runner reads those paths inside the guest/runtime environment.
- Capture `${PIPESTATUS[0]}` after `tee`, write explicit `.status` files, and parse every log with `scripts/ltp_summary.py` plus `--json`.
- Treat wrapper PASS, internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, and panic/trap as separate signals; never convert timeout or `TCONF` into clean promotion evidence.
- Keep `.omx/ultragoal` and `LTP_STABLE_CASES` out of scope; these commands produce evidence only.

## Promotion stop conditions

A case or batch is not promotable if any of the following is true in fresh RV/LA evidence:

- Wrapper `FAIL LTP CASE` is nonzero.
- Internal `TFAIL` or `TBROK` is nonzero.
- Timeout matches are nonzero, including host-side or in-run timeout markers.
- ENOSYS/not-implemented matches are nonzero.
- Panic/trap markers are nonzero.
- TCONF appears unexpectedly; known `read02` TCONF may be carried only as visible caveat and must not be reclassified as clean.
- Full split-batch evidence is clean but the optional cumulative stable180 probe regresses existing stable cases; then keep split-batch candidates as evidence only and do not promote until the cumulative blocker is isolated.
