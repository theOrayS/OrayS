# Worker 5 mmap/mprotect/munmap + guardrail report

Status: completed as a discovery/audit lane for task 5. No source or final
`LTP_STABLE_CASES` edits were made; `.omx/ultragoal` remains leader-owned.

## Scope and current baseline

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` count checked from the
  worker worktree: 300 entries, 300 unique, 0 duplicates.
- Task-focus cases are not currently stable: `mmap04`, `mmap05`, `mmap06`,
  `mprotect01`, `mprotect02`, `munmap01` are all absent from the stable list.
- This lane treats worker/QEMU runs as discovery-only. The leader update said
  final promotion and final `LTP_STABLE_CASES` ownership stay with the leader.

## Fresh/near-fresh evidence summary

### Worker-local isolated RV attempt

Command recorded in `raw/worker5-mmap-rv-targeted.status`:

```text
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap04,mmap05,mmap06,mprotect01,mprotect02,munmap01 LTP_CASE_TIMEOUT_SECS=60 RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img RV_TESTSUITE_RUN_IMG=/tmp/arceos-worker5-rv-869990.qcow2 ./run-eval.sh rv
```

Result: `timeout(1)` exit 124 after 900s while still building. It produced no
LTP markers and is **not promotion evidence**. The attempt is still useful as a
host-capacity signal: concurrent worker/leader builds can consume the whole
worker timeout budget before QEMU starts, even with an isolated qcow path.

Artifacts:

- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/worker5-mmap-rv-targeted.log`
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/worker5-mmap-rv-targeted.status`
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/worker5-mmap-rv-targeted-summary.txt`

### Current blocker-batch RV log read-only cross-check

A concurrently available root-lane log at
`/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-25-phase-a/raw/blocker-batch-rv.log`
contains the task-5 cases. Because it was not worker-5 owned and other QEMU/build
work was active, this is **discovery-only** and not a promotion gate.

Parsed with `python3 -B scripts/ltp_summary.py`; saved as `raw/worker5-readonly-blocker-batch-rv-summary.txt`:

| Case | rv:musl | rv:glibc | Key blocker signal |
| --- | --- | --- | --- |
| `mmap04` | FAIL code 2, TBROK=1 | FAIL code 2, TBROK=1 | LTP reports `Expected 1 conversions got 0 FILE '/proc/self/maps'`; likely procfs `/proc/self/maps` visibility/format blocker before mmap semantics can be trusted. |
| `mmap05` | FAIL code 139 | FAIL code 139 | Segfault-style wrapper failure, no internal TFAIL/TBROK; investigate page-fault delivery/SIGSEGV behavior. |
| `mmap06` | FAIL code 1, TFAIL=7 | FAIL code 1, TFAIL=7 | LTP expected `EACCES` for several mmap permission cases but observed `EBADF`; likely fd access-mode vs mmap prot validation path. |
| `mprotect01` | FAIL code 1, TFAIL=3 | FAIL code 139 | musl has real TFAILs; glibc saw an address-range/page-fault style crash. |
| `mprotect02` | FAIL code 2, TBROK=2 | FAIL code 2, TBROK=2 | Unexpected-signal TBROK, likely signal/page-protection delivery semantics. |
| `munmap01` | FAIL code 139 | FAIL code 139 | Segfault-style wrapper failure; focus on unmap boundary and post-unmap fault delivery. |

Older phase-a evidence agrees with this classification:
`user-priority-refresh-rv-summary.txt` and `user-priority-ae-rv-summary.txt`
also show all six task-5 cases failing on RV musl/glibc; LA was missing for
those historical rows, so none are promotable.

## Source audit notes

Relevant source surfaces:

- `examples/shell/src/uspace/memory_map.rs:126-245` — `sys_mmap` maps anonymous
  and file-backed ranges, handles `MAP_FIXED`, eagerly copies non-anonymous file
  contents, and records writable shared mappings.
- `examples/shell/src/uspace/memory_map.rs:248-300` — `sys_munmap` aligns and
  unmaps, with a deferred self-stack-unmap special case.
- `examples/shell/src/uspace/memory_map.rs:336-378` — `sys_mprotect` delegates
  to `aspace.protect` and prefaults small writable ranges.
- `examples/shell/src/cmd.rs:1656-1715` — LTP wrapper marker emission and
  timeout/failure handling.
- `scripts/ltp_summary.py:24-26,109-119,233-255,301-331` plus
  `scripts/test_ltp_summary.py` — parser guardrails for PASS/FAIL numeric
  truth, legacy `FAIL ... : 0`, and timeout-overrides-pass semantics.

Observed likely fix targets, in priority order:

1. `/proc/self/maps` support/format for `mmap04` before treating the test as a
   pure mmap failure.
2. mmap fd permission errno mapping for `mmap06`: current behavior surfaces
   `EBADF` where LTP expects `EACCES` for access-mode/prot mismatches.
3. User page-fault-to-signal semantics for `mmap05`, `mprotect01`,
   `mprotect02`, and `munmap01`, especially SIGSEGV/SIGBUS classification,
   mprotect on edge ranges, and post-unmap accesses.
4. MAP_FIXED replacement and unmap-boundary behavior should be kept under
   regression scrutiny if any source fix is attempted.

## Guardrail audit

- Parser guardrails are present: numeric status is authoritative, non-zero PASS
  tokens are not accepted as pass, and timeout markers remove prior pass
  classification.
- Marker prefix check on the worker-local attempted log found 0 markers and 0
  bad-prefix markers; root blocker-batch markers for the observed cases are
  emitted at line start.
- Source note: `examples/shell/src/cmd.rs:1691-1698` still documents/emits the
  legacy success marker `FAIL LTP CASE <case> : 0`, while
  `scripts/ltp_summary.py` and `scripts/test_ltp_summary.py` describe current
  success markers as `PASS LTP CASE <case> : 0` and accept the legacy form for
  compatibility. This worker did not edit `cmd.rs` because final marker/stable
  policy is shared/leader-owned, but leader should decide whether to normalize
  the success marker to `PASS` in a serialized guardrail change.
- No fake-pass or timeout-as-pass source change was made in this lane.

## Integrated subagent findings

Subagents spawned: 2 (`Descartes`, `Beauvoir`) using the requested
`gpt-5.4-mini` model.

Integrated findings:

- `Descartes` confirmed stable300 count/list patterns and recommended the same
  task-5 blocker batch plus parser-based promotion-candidate checks. It also
  surfaced the exact stable-count preflight and marker-prefix commands.
- `Beauvoir` found broader RV PASS-only suggestions from older phase-a reports
  (`alarm05`, `alarm07`, `write05`, several sched/gethostname/gettid/sbrk cases)
  and repeated the concurrency warning for shared `/tmp/arceos-sdcard*.run.qcow2`
  evidence. These are not task-5 mmap evidence and should be handled by leader or
  the relevant worker lane before promotion.

## Recommended next steps

1. Do not promote `mmap04`, `mmap05`, `mmap06`, `mprotect01`, `mprotect02`, or
   `munmap01` yet.
2. If leader wants this lane to move from discovery to source repair, start with
   narrow fixes for `/proc/self/maps` and mmap fd/prot errno mapping, then rerun
   RV+LA x musl/glibc targeted cases serially with unique run images.
3. If no source repair is desired in phase-a, harvest clean candidates from other
   lanes instead; the mmap/mprotect/munmap cluster remains high hidden-test value
   but is not near-clean.
4. Serialize heavy QEMU/build runs or reuse completed kernels before asking
   worker lanes for fresh QEMU evidence; otherwise build contention can consume
   the worker timeout before QEMU starts.

## Verification

| Check | Result |
| --- | --- |
| Stable-count preflight from `examples/shell/src/cmd.rs` | PASS: 300 total, 300 unique, target six absent. |
| Worker-local isolated RV targeted run | FAIL/INCONCLUSIVE: timeout exit 124 during build, no LTP markers; not promotion evidence. |
| `python3 -B scripts/ltp_summary.py raw/worker5-mmap-rv-targeted.log` | PASS parser execution; 0 PASS/FAIL markers because build timed out before QEMU. |
| Read-only parse of root `blocker-batch-rv.log` | PASS parser execution; task-5 cases all fail on RV musl/glibc with real FAIL/TFAIL/TBROK signals. |
| Marker-prefix audit | PASS: saved in `raw/worker5-marker-prefix-check.txt`; worker-local attempted log has 0 markers/0 bad prefixes, read-only blocker-batch log has line-start markers. |
| No leader-owned state edited | PASS: no `.omx/ultragoal` or final stable-list edit. |
