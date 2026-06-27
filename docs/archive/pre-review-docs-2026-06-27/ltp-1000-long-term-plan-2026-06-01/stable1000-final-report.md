# stable1000 final report

## Outcome

The long-term plan reaches `stable1000`: `examples/shell/src/cmd.rs::LTP_STABLE_CASES` is now 1000 total / 1000 unique / 0 duplicate.

## Evidence summary

- Cumulative milestone evidence covers stable556, stable606, stable656, stable706, stable756, stable806, stable856, stable906, and stable956 as committed earlier under this plan.
- Milestone-10 adds 44 cases and closes post-review RV + LA x musl + glibc parser-clean evidence:
  - RV new44: `target/ltp-1000-milestone-10-stable1000/rv-new44-postreview-rerun60-20260606T135933+0800/rv-summary.txt` — PASS 88 / FAIL 0 / parser blockers 0.
  - LA new44: `target/ltp-1000-milestone-10-stable1000/la-new44-postreview-rerun60-20260606T140605+0800/la-summary.txt` — PASS 88 / FAIL 0 / parser blockers 0.
- Current regression gates protect the changed vfork/clone/exec/close_range/pipe/fcntl/mmap lanes:
  - RV subset: `target/ltp-1000-milestone-10-stable1000/rv-regression-postreview-rerun60-20260606T141353+0800/rv-summary.txt` — PASS 60 / FAIL 0 / parser blockers 0.
  - LA stable-order subset: `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800/la-summary.txt` — PASS 60 / FAIL 0 / parser blockers 0.
- Supporting smoke: `target/ltp-1000-milestone-10-stable1000/rv-postreview-exec-fd-vfork-smoke-20260606T133920+0800/rv-summary.txt` — PASS 16 / FAIL 0 for exec/FD/vfork-focused cases.
- Final quality gate JSON: `docs/ltp-1000-long-term-plan-2026-06-01/stable1000-final-quality-gate.json`.

## Post-review fixes included

- `execve()` failure atomicity: new images are built in a scratch address space and swapped into the live process only after loader success; task page-table root is updated after the swap.
- `vfork()` parent wake: child exec success now marks `vfork_exec_done` and wakes the parent wait path; child exit still wakes as before.
- FD sharing after `CLOSE_RANGE_UNSHARE` plus `CLONE_FILES`: FD-table alias ownership avoids mutating the old base table used by existing sharers.
- Noisy `execve-load-failure` debug print was removed.

## Caveat

A full stable1000 all-case RV/LA sweep was not rerun in this milestone. This report therefore treats stable1000 as cumulative milestone-backed evidence plus current final-44 and regression gates, not as a hidden full-sweep PASS claim.

## Guardrails

- No fake PASS, case/path/process/output hardcoding, testsuite/evaluator bypass, or blacklist/SKIP/status0 promotion evidence was used.
- All caveats and excluded diagnostics are explicit in `milestone-10-stable1000/validation.md` and `stable1000-final-quality-gate.json`.

## Final static/cleanup/review gate

- Static gate log: `target/ltp-1000-milestone-10-stable1000/final-static-postreview-20260606T143910+0800/final-static-checks.log`.
- Count/rustfmt/cargo-check/diff-check/debug-marker/new44-hardcode scans passed in the final static gate.
- AI slop cleaner final gate report: `milestone-10-stable1000/final-gate-ai-slop-cleaner-report.md`.
- Independent code-reviewer `RECOMMENDATION: APPROVE` and architect `Architectural Status: CLEAR` are recorded in `stable1000-final-quality-gate.json` and the milestone final-gate review reports.
