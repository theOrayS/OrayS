# Worker 2 light syscall/process scout report

Date: 2026-05-26
Team: `ltp-stable383-to-stab-2374dbd5`
Task: 2 / light syscall-process-libc candidates

## Scope and guardrails

This lane is discovery/report-only for the current team task. I did not run QEMU,
did not edit `.omx/ultragoal`, and did not edit
`examples/shell/src/cmd.rs::LTP_STABLE_CASES`. Any runtime commands below are
leader-serial scout commands; if run by a worker they are discovery-only and not
promotion evidence.

Live stable list was recalculated from `examples/shell/src/cmd.rs`:

- total: 383
- unique: 383
- duplicates: 0
- all 14 task-2 focus cases are outside stable383 and present in the RV common
  non-stable inventory from `docs/ltp-score-improvement-2026-05-25-phase-c/raw/sdcard-rv-common-not-stable-ltp-bins.txt`.

Subagent spawned: 1 (`019e64a1-d9e8-7ef2-b570-0acd5e1a0066`, test-coverage probe,
model `gpt-5.4-mini`). Findings integrated: existing parser tests cover wrapper
PASS/FAIL and timeout semantics, but `scripts/ltp_summary.py --promotion-candidates`
and stable-list count/unique/duplicate invariants do not yet have automated tests.
Those gaps are recorded here as verification suggestions; this task's live JSON is
`requires_code_change=false`, so no parser/source tests were edited in this lane.

## Source/risk findings

Relevant syscall surfaces inspected:

- `examples/shell/src/uspace/select_fdset.rs`: `sys_ppoll()` and `sys_poll_until()`
  exist; direct `sys_poll()` is compiled out for `riscv64` and `loongarch64`, so
  `poll02` depends on libc using `ppoll` on the evaluator arches.
- `examples/shell/src/uspace/time_abi.rs`: `sys_times()` writes a default `tms` and
  returns monotonic ticks; prior `times03` failures show this is not enough for the
  test's accounting expectations.
- `examples/shell/src/uspace/system_info.rs`: `sys_uname()` and `sys_sethostname()`
  are implemented; no direct `gethostname`, `gethostid`, or `getcpu` dispatch was
  found in `syscall_dispatch.rs`.
- `examples/shell/src/uspace/process_abi.rs`: `sys_getpgid()` and `sys_setpgid()`
  are present, but prior `getpgid01` runs still report `TFAIL`/`TBROK`, so the
  missing semantics are likely process-group/session edge cases rather than a
  trivial syscall hole.
- `examples/shell/src/uspace/process_lifecycle.rs`: `sys_clone()` handles a limited
  fork/vfork-like subset; unsupported clone flag shapes return `ENOSYS`. This makes
  `clone06`-`clone09` risky until raw subtest expectations are isolated.
- `examples/shell/src/uspace/signal_abi.rs`: `sys_kill()`/`tkill`/`tgkill` exist,
  but prior `kill05`/`kill10` evidence is real failure/timeout, not missing binary
  noise.

## Candidate status matrix

| Case | Inventory | Prior evidence | Current assessment | Next action |
| --- | --- | --- | --- | --- |
| `poll02` | RV common non-stable present | No direct prior summary found in inspected phase-b/c reports | Best low-risk scout candidate; source has `ppoll` path, direct `poll` is arch-gated out | Include in first RV small scout |
| `gethostid01` | present | No direct prior summary found | Possibly libc/file fallback; no direct kernel syscall found | Include in first RV small scout |
| `getcpu01` | present | No direct prior summary found | Likely direct syscall gap if test calls `getcpu`; potential narrow fix after raw proof | Include in first RV small scout; if ENOSYS, report `__NR_getcpu` patch risk before editing |
| `gethostname02` | present | Repeated RV glibc PASS but RV musl FAIL/TFAIL in `target-near-clean-rv-summary.txt`, `target-extra-simple-rv-summary.txt`, `target-fill16-rv-001-summary.txt` | Small UTS/libc edge, but not clean today | Scout only if leader wants a narrow diagnosis batch; do not promote from glibc-only pass |
| `times03` | present | RV glibc/musl FAIL in `target-post285-scout3-rv-summary.txt` and `target-fill16-rv-001-summary.txt`; older musl timeout also seen | Not easy-first; needs real CPU/user/system time accounting semantics | Defer from promotion batch; run only in blocker batch |
| `getpgid01` | present | Repeated RV glibc+musl FAIL with TFAIL/TBROK across phase-b/phase-a summaries | Not easy-first; process-group/session semantics still incomplete | Defer; isolate raw subtests before code change |
| `fork13` | present | `target-scout14-rv-001-summary.txt`: RV glibc+musl timeout | Timeout class; not safe for easy-first | Defer from mixed scout; run only in timeout blocker batch |
| `fork14` | present | `target-scout14-rv-001-summary.txt`: RV glibc+musl timeout | Timeout class; not safe for easy-first | Defer from mixed scout; run only in timeout blocker batch |
| `clone06` | present | No fresh phase-c direct clean evidence found; old hardblocker notes mention clone-family flag issues | Likely unsupported clone-flag semantics | Separate RV clone batch before any LA |
| `clone07` | present | Old hardblocker notes mention clone07 wait/return-value symptoms | Risky clone semantics | Separate RV clone batch before any LA |
| `clone08` | present | Old hardblocker notes mention `CLONE_PARENT` failure | Likely unsupported clone semantics | Separate RV clone batch before any LA |
| `clone09` | present | No fresh direct clean evidence found | Unknown clone semantics | Separate RV clone batch before any LA |
| `kill05` | present | Repeated RV glibc+musl FAIL with TFAIL/TBROK (`target-kill-signal-rv-summary.txt`, phase-b/fill summaries) | Not easy-first; real signal semantic/setup failure | Defer; needs raw subtest diagnosis |
| `kill10` | present | RV timeout in `target-kill-signal-rv-summary.txt` and `target-scout26-rv-001-summary.txt` | Timeout class; not safe for easy-first | Defer from promotion; do not mix with short clean scouts |

## Recommended leader-serial scout commands

Use serial execution only. Do not run these concurrently with another QEMU/evaluator
job that shares the default sdcard/qcow2 names.

### Batch A: lowest-risk unknown/light candidates

```bash
mkdir -p docs/ltp-score-improvement-2026-05-26-phase-a/raw
cases=poll02,gethostid01,getcpu01,gethostname02
tag=worker2-light-syscall-rv-001
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=45 ./run-eval.sh rv \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" 2>&1
printf 'status=%s\narch=rv\ncases=%s\n' "$?" "$cases" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.status"
python3 -B scripts/ltp_summary.py \
  "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  | tee "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-summary.txt"
python3 -B scripts/ltp_summary.py --json \
  "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-summary.json"
python3 -B scripts/ltp_summary.py --promotion-candidates \
  --promotion-arches rv --promotion-libcs musl,glibc \
  "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-promotion-candidates.txt"
```

Only cases with RV musl+glibc `PASS`, zero internal TFAIL/TBROK/TCONF, zero
timeout, zero ENOSYS, and zero panic/trap should be sent to LA. If `getcpu01`
shows only ENOSYS, the likely narrow source location is `syscall_dispatch.rs` plus
a small `sys_getcpu` helper, but report that patch/risk before editing shared
source.

### Batch B: clone-family diagnosis, not promotion-first

```bash
cases=clone06,clone07,clone08,clone09
tag=worker2-clone-rv-001
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" 2>&1
printf 'status=%s\narch=rv\ncases=%s\n' "$?" "$cases" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.status"
python3 -B scripts/ltp_summary.py "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  | tee "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-summary.txt"
python3 -B scripts/ltp_summary.py --json "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-summary.json"
```

Do not send clone cases to LA unless RV is fully clean. Based on current source,
unsupported flag combinations are expected to surface as real failures/ENOSYS.

### Batch C: known blocker/timeout confirmations only

```bash
cases=times03,getpgid01,fork13,fork14,kill05,kill10
tag=worker2-known-blockers-rv-001
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" 2>&1
printf 'status=%s\narch=rv\ncases=%s\n' "$?" "$cases" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.status"
python3 -B scripts/ltp_summary.py "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  | tee "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-summary.txt"
```

Run Batch C only if the leader wants fresh blocker proof. It should not be mixed
with Batch A because `fork13`/`fork14`/`kill10` have known timeout history and can
waste an easy-first scout window.

### LA confirm template for RV-clean subset only

```bash
cases=<comma-separated RV-clean cases only>
tag=worker2-light-syscall-la-001
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh la \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" 2>&1
printf 'status=%s\narch=la\ncases=%s\n' "$?" "$cases" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.status"
python3 -B scripts/ltp_summary.py "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  | tee "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-summary.txt"
python3 -B scripts/ltp_summary.py --promotion-candidates \
  --promotion-arches rv,la --promotion-libcs musl,glibc \
  docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker2-light-syscall-rv-001.log \
  "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker2-light-syscall-rv-la-promotion-candidates.txt"
```

## Verification notes for this report

- The parser/unit-test sidecar found that `--promotion-candidates` lacks a direct
  unit test. Recommended future test: synthetic RV+LA logs with one clean case,
  one missing arch/libc case, one `TCONF`, and one wrapper failure.
- Recommended stable-list invariant before any leader promotion edit:

```bash
python3 - <<'PY'
from pathlib import Path
from collections import Counter
import re
text = Path('examples/shell/src/cmd.rs').read_text()
match = re.search(r'const LTP_STABLE_CASES: &\[&str\] = &\[(.*?)\];', text, re.S)
items = re.findall(r'"([^"]+)"', match.group(1))
dups = [name for name, count in Counter(items).items() if count > 1]
print(f'total={len(items)} unique={len(set(items))} duplicates={len(dups)}')
if dups:
    raise SystemExit(f'duplicate LTP_STABLE_CASES: {dups}')
PY
```

## Completion summary

This lane found no immediately promotable case from prior evidence alone. The
best first serial scout is `poll02,gethostid01,getcpu01,gethostname02`; all other
focus cases either have known RV failures/timeouts or clone/process semantics risk
that should be isolated before LA or promotion.
