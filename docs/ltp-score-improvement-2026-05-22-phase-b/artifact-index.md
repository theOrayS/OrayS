# Phase B artifact index

This directory is the current-run artifact root for the 2026-05-22 Phase B LTP stable push.

## Scope

Task 3 is documentation-only and owns only files under:

```text
docs/ltp-score-improvement-2026-05-22-phase-b/
```

It does not edit source, runner behavior, evaluator scripts, or leader-owned `.omx/ultragoal` state.

## Existing handoff files

| File | Purpose |
| --- | --- |
| `plan-stable101-to-125.md` | Phase B execution plan from stable101 toward stable120/125, including gates, candidate pools, lane ownership, and stop criteria. |
| `next-session-ultragoal-team-prompt.md` | Reusable Chinese handoff prompt for launching or resuming the Phase B Ultragoal + Team campaign. |
| `codex-goal-active-initial.json` | Initial Codex goal snapshot captured at Phase B startup; informational only, not worker-owned mutable state. |

## Worker-3 task 3 artifacts

| File | Purpose |
| --- | --- |
| `phase-b-baseline-artifact.md` | Stable101 source-of-truth baseline, guardrails, candidate waves, and known blocker categories for Phase B workers. |
| `artifact-index.md` | Directory map, artifact ownership summary, and evidence rules for this docs root. |
| `task3-worker-artifact-report.md` | Worker lifecycle, verification, and delegation compliance evidence for task 3. |

## Expected follow-up artifacts

Follow-up workers or the leader should add artifacts here as evidence is produced:

| Pattern | Expected content |
| --- | --- |
| `raw/*.status`, `raw/*.log`, `raw/*-summary.{txt,json}` | Raw targeted/full gate command exits, logs, and `scripts/ltp_summary.py` summaries. |
| `*-candidate-matrix.md` / `*-promotion-matrix.md` | LA/RV x musl/glibc candidate classification with TFAIL/TBROK/TCONF/timeout/ENOSYS/panic-trap separated. |
| `*-promotion-gate-report.md` | Newly promoted cases, evidence basis, stable count, and blocked cases after each promotion batch. |
| `final-gate-report.md` and `final-gate-quality-gate.json` | Final full-gate evidence and quality decision owned by the leader after targeted and promotion gates are clean. |
| `final-gate-ai-slop-cleaner-report.md` and `final-gate-code-review-report.md` | Guardrail/review evidence before any final success claim. |

## Artifact rules

- Keep current-run artifacts in this directory only.
- Keep historical evidence paths intact; cite them rather than moving or rewriting them.
- Store raw logs plus machine-readable summaries wherever possible.
- Do not use documentation artifacts to hide real LTP failure, timeout, ENOSYS, panic/trap, internal `TFAIL`/`TBROK`/`TCONF`, or non-LTP evaluator caveats.
- Do not checkpoint or mutate `.omx/ultragoal` from worker tasks; leader owns durable Ultragoal audit.
