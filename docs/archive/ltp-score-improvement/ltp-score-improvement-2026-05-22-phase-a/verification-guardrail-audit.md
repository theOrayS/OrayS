# Verification Guardrail Audit

Scope: `scripts/ltp_summary.py`, `examples/shell/src/cmd.rs`, plus quick output scans in `output_la.md` / `output_rv.md`.

## Findings

- **Numeric status is the source of truth; legacy `FAIL ... : 0` is compatibility output, not fake PASS.**
  - `scripts/ltp_summary.py:4-8` states wrapper logs may print `FAIL LTP CASE <case> : 0` for a successful case and that numeric status is the source of truth.
  - `scripts/ltp_summary.py:108-118` makes `normalize_wrapper_status()` return PASS only for `code == 0`, otherwise FAIL.
  - `examples/shell/src/cmd.rs:1475-1485` intentionally prints `FAIL LTP CASE {case} : 0` for successful LTP cases to preserve the evaluator wire format.
  - Risk: downstream consumers that key off the literal `FAIL` token instead of the numeric status can misread healthy cases as failures.

- **Case-name hardcode guardrails are present.**
  - `examples/shell/src/cmd.rs:44-49` declares core/stable LTP case sets as explicit batch data.
  - `examples/shell/src/cmd.rs:534-548` rejects invalid case names and deduplicates accepted cases.
  - `examples/shell/src/cmd.rs:588-637` supports `stable`, `core`, `batch:<name>`, `file:<path>`, and inline lists; no single-case success hardcode path was found.
  - Risk: default stable coverage still changes through source-controlled static lists by design.

- **Silent LTP SKIP was not found in this pass.**
  - Missing LTP binaries fail visibly at `examples/shell/src/cmd.rs:1446-1456`.
  - The only explicit skip found in this runner area is for disabled non-LTP official groups at `examples/shell/src/cmd.rs:1570-1572`.

- **Timeout is not treated as PASS.**
  - `examples/shell/src/cmd.rs:1487-1504` prints a failure status plus `TIMEOUT LTP CASE` for timeout exits/errors and increments timeout/failed counters.
  - `examples/shell/src/cmd.rs:1517-1519` reports `passed`, `failed`, and `timed out` separately.
  - `scripts/ltp_summary.py:25,31-33,198-218,267-301,330-347,492-567` tracks timeout/internal/ENOSYS/panic markers separately from clean PASS.
  - Risk: consumers that ignore timeout fields and read only top-level wrapper counts can still underreport failures.

- **Promotion-candidate filtering is conservative.**
  - `scripts/ltp_summary.py:267-303` separates `pass_clean` from `pass_with_tconf`, wrapper fail, TFAIL/TBROK, timeout, ENOSYS, panic/trap, and unknown.
  - `scripts/ltp_summary.py:330-347` makes TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap, event failures, and non-PASS status blockers.
  - `scripts/ltp_summary.py:350-419` promotes a case only when every required arch/libc combo is present and all blocker lists are empty.
  - Risk: strict filtering will intentionally exclude caveated cases even when wrapper counts look green.

## Current stable85 source of truth

Use phase-d summaries as the current stable85 baseline; root `output_la.md` / `output_rv.md` in this worktree are older stable63 logs.

| Source | PASS LTP CASE | FAIL LTP CASE | ltp-musl | ltp-glibc | Internal | timeout | ENOSYS | panic/trap |
| --- | ---: | ---: | --- | --- | --- | ---: | ---: | ---: |
| `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-output-la-summary.txt:4-12` | 170 | 0 | 85/0 | 85/0 | TCONF=4 | 0 | 0 | 0 |
| `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-output-rv-summary.txt:4-12` | 170 | 0 | 85/0 | 85/0 | TCONF=4 | 0 | 0 | 0 |
| `docs/ltp-score-improvement-2026-05-21-phase-d/stable85-targeted-la-summary.txt:4-12` | 170 | 0 | 85/0 | 85/0 | TCONF=4 | 0 | 0 | 0 |
| `docs/ltp-score-improvement-2026-05-21-phase-d/stable85-targeted-rv-summary.txt:4-12` | 170 | 0 | 85/0 | 85/0 | TCONF=4 | 0 | 0 | 0 |

Phase-d final report records the key guardrail explicitly: `read02` accounts for TCONF=4 per arch and no timeout was counted as PASS (`docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-report.md:24-34`).

## Root output scan notes

`python3 -B scripts/ltp_summary.py output_la.md` and `python3 -B scripts/ltp_summary.py output_rv.md` both parse as older stable63 logs: 126/0 wrapper cases, `ltp-musl 63/0`, `ltp-glibc 63/0`, TCONF=4, total timeout matches=10, LTP-group timeout=0, ENOSYS=0, panic/trap=0.

Visible non-LTP/runtime markers in those older root outputs:

- LA libctest timeout/futex markers: `output_la.md:248-260`, `output_la.md:632-654`, and `output_la.md:6360-6390`.
- RV libctest timeout/futex markers: `output_rv.md:263-266` and `output_rv.md:5956-5987`.
- LA iperf failures: `output_la.md:6530-6560`.
- RV iperf failures: `output_rv.md:6127-6157`.
- LA non-LTP benchmark timeouts: `output_la.md:6574-7159`.
- RV non-LTP benchmark timeouts: `output_rv.md:6162-6808`.

These markers are visible follow-up runtime evidence, not stable85 LTP promotion blockers unless future targeted LTP evidence links them to LTP failure.

## Verification performed by this worker

- `python3 -B scripts/ltp_summary.py output_la.md` -> PASS parser execution; old root LA output summarized as stable63-era LTP 126/0 with LTP-group timeout 0.
- `python3 -B scripts/ltp_summary.py output_rv.md` -> PASS parser execution; old root RV output summarized as stable63-era LTP 126/0 with LTP-group timeout 0.
- `git diff --check -- docs/ltp-score-improvement-2026-05-22-phase-a/verification-guardrail-audit.md` -> PASS before commit.
- `cargo metadata --locked --offline --format-version 1 --no-deps --manifest-path ./Cargo.toml` -> PASS (manifest/dependency metadata parse).
- `cargo fmt --manifest-path ./Cargo.toml --all -- --check` -> FAIL due nested team-worktree/vendor workspace discovery, not due this docs-only change.
- `cargo check --workspace --locked --offline --manifest-path ./Cargo.toml` -> stopped after leader instructed workers to stop duplicate long build/eval and continue static analysis/reporting only.

## Subagent evidence

- Subagents spawned: 2 total in this worker session.
  - `019e4d1e-068b-7ee2-8f09-68904b020907`: read-only report-pattern lookup; integrated the concise Scope / Result / Evidence / Worker-boundary pattern into `worker-5-ultragoal-mutation-guard-report.md`.
  - `019e4d24-50c6-7aa2-a48a-948412f4f2ab`: guardrail helper; it produced the first committed audit artifact, then this worker corrected/expanded evidence line references in a follow-up commit.
- Subagent model: `gpt-5.4-mini` for both.
- Serial searches before first spawn: 2.

## `.omx/ultragoal` mutation check

- `test ! -e .omx/ultragoal` reported `PASS no .omx/ultragoal directory`.
- `git status --short -- .omx` reported no `.omx` changes.
- `git diff --name-only -- .omx/ultragoal` reported no tracked diff under `.omx/ultragoal`.
- `git ls-files .omx/ultragoal` reported no tracked `.omx/ultragoal` files in this worker worktree.

## Conclusion

The audited LTP runner/parser paths preserve the required guardrails: numeric-status truth, no silent LTP skip found, timeout remains failure/timeout evidence, and promotion candidates are blocked by internal markers, timeout, ENOSYS, panic/trap, event failures, missing arch/libc combos, or non-PASS status. Keep using `scripts/ltp_summary.py` for every targeted/promotion/full gate and keep non-LTP iperf/futex/runtime markers visible in final reporting.
