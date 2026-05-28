# worker5 time/signal/timer/memory/mmap + guardrail lane

Status: completed as report-only candidate/guardrail lane for OMX task 24.

## Scope boundaries

- Edited only phase-a documentation/raw evidence under
  `docs/ltp-score-improvement-2026-05-24-phase-a/`.
- Did not edit leader-owned `.omx/ultragoal`.
- Did not edit final `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- No source fix was made because this lane found reportable candidate classes
  and guardrail evidence, not a single narrow near-clean source blocker that was
  safe to repair inside the worker-5 scope.

## Integrated subagent findings

- Repository map probe identified the relevant source surfaces:
  `examples/shell/src/uspace/time_abi.rs`, `signal_abi.rs`,
  `memory_map.rs`, `process_lifecycle.rs`, `task_context.rs`,
  `memory_policy.rs`, `sysv_shm.rs`, plus parser/tests in
  `scripts/ltp_summary.py` and `scripts/test_ltp_summary.py`.
- Guardrail review confirmed parser/test coverage for numeric status truth,
  legacy marker compatibility, timeout-overrides-pass, and `pass_with_tconf`.
  It also found that `remote-marker-regression-check.md` was still pending.
- Candidate probe separated safe report-only candidates from blocker-only cases
  and warned not to relabel `read02`, timeout cases, or TCONF cases as clean.

## Candidate classification

### Clean or already-stable evidence slice

These cases have prior clean matrix evidence in phase-b/final-gate summaries and
appear in the current root-output parser smoke as clean rows where present:

- Time/timer: `clock_gettime02`, `gettimeofday01`, `time01`, `times01`.
- Signal: `alarm02`, `alarm03`, `kill03`, `rt_sigaction01`,
  `rt_sigaction02`, `sigaction01`, `sigaction02`, `rt_sigprocmask01`,
  `rt_sigprocmask02`, `sigprocmask01`, `sigsuspend01`.
- Memory/mmap/resource adjacency: `brk01`, `mmap01`, `setrlimit02`.

Evidence anchors:

- `docs/ltp-score-improvement-2026-05-22-phase-b/stable115-targeted-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-output-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-output-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-b-near-clean-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-b-promotion-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/worker2-rlimit-getgroups-report.md`

### Transparent caveat, not clean promotion

- `read02` remains `pass_with_tconf` and must stay visible. The worker-5 parser
  smoke generated from root outputs again reports total `TCONF=4`, with `read02`
  excluded from clean promotion candidates.
- `clock_getres01` has historical `PASS` wrapper with `TCONF=8`; keep it out of
  clean candidate lists unless a fresh run proves clean or leadership accepts a
  transparent non-clean caveat.

### Blocker-only cases

Do not promote these without fresh fixes and LA/RV x musl/glibc clean evidence:

- Timeout or timeout-adjacent: `clock_gettime01`, `clock_nanosleep01`,
  `clock_nanosleep02`, `pause01`, `setitimer01`.
- Real TFAIL/TBROK/ENOSYS blockers: `nanosleep01`, `nanosleep02`, `getitimer01`,
  `kill02`, `kill05`, `sigpending02`, `rt_sigpending01`, `sigaltstack01`,
  `sigaltstack02`, `sigwait01`, `sigtimedwait01`.

Evidence anchors:

- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-a-targeted-rv-worker1-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-a-partial-matrix.md`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-c-signal-wait-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/stable115-promotion-gate-report.md`

## Guardrail updates made

- Updated `docs/ltp-score-improvement-2026-05-24-phase-a/remote-marker-regression-check.md`
  from pending to a completed report-only guardrail check.
- Added raw parser smoke artifacts:
  - `docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker5-output-la-summary.txt`
  - `docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker5-output-rv-summary.txt`
  - `docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker5-current-promotion-candidates.txt`

## Verification evidence

| Check | Result |
| --- | --- |
| `python3 -m unittest scripts.test_ltp_summary -v` | PASS: 4 tests OK |
| `python3 -B scripts/ltp_summary.py output_la.md` | PASS: parsed to worker5 LA summary; `TCONF=4`, no wrapper fail, no ENOSYS, no panic/trap |
| `python3 -B scripts/ltp_summary.py output_rv.md` | PASS: parsed to worker5 RV summary; `TCONF=4`, no wrapper fail, no ENOSYS, no panic/trap |
| `python3 -B scripts/ltp_summary.py --promotion-candidates output_rv.md output_la.md` | PASS: report generated; `read02` remains blocked/incomplete because of TCONF |
| Source scan for marker paths | PASS: non-zero failures and timeout emit failure/timeout markers; NOTE: current success marker source is legacy-compatible `FAIL ... : 0` |

## Stop condition

Task 24 is complete as a report-only lane: candidate classes are separated,
blockers are preserved as blockers, `read02` transparency is documented, timeout
cannot count as PASS, and the remote-marker guardrail has current evidence and
explicit stale-output caveats.
