# Ultragoal brief: LTP stable score improvement next phase

Continue improving `/root/oskernel2026-orays` LTP stable score from the completed stable-63 baseline toward 80-100 cases per libc/arch, using Team lanes for parallel evidence gathering and leader-owned Ultragoal checkpointing.

## Non-negotiable constraints
- No fake PASS, no case-name hardcoded PASS, no silent SKIP of real failures.
- Timeout is a failure signal, counted separately, never counted as PASS.
- Do not rely on `run-eval` exit code alone; use `scripts/ltp_summary.py` for internal TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap.
- Targeted batches first; final full `./run-eval.sh la` and `./run-eval.sh` only after promotion is justified.
- Every promoted case needs LA/RV × musl/glibc evidence and a promotion rationale.
- Workers do not mutate `.omx/ultragoal`; the leader owns `goals.json` / `ledger.jsonl` checkpoints.
- Preserve transparent `read02` TCONF record; do not hide it.

## Baseline
- Previous Ultragoal 10/10 complete; current Codex goal was clear before this plan.
- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` currently has 63 cases per libc/arch.
- `docs/ltp-score-improvement-2026-05-22/final-gate-report.md` reports final LA/RV gates exit 0 and 126 PASS / 0 FAIL per arch.
- 2026-05-22 combined candidate report has no obvious clean not-yet-stable cases; this phase must discover fresh candidates and/or make real ABI fixes before promotion.

## Executable goals
1. Establish and document the baseline/context for the new run, including old Ultragoal clear status, current 63-case stable list, prior final-gate evidence, and blocked-case guardrails.
2. Launch and supervise an OMX Team with discovery, stats/report, runner/harness, syscall/ABI, and verification/review lanes; collect terminal evidence before shutdown.
3. Enumerate available LTP cases from sdcard images and prior docs, select 20-40 plausible candidates outside stable, and run small targeted RV/LA batches to classify them.
4. Generate a promotion matrix with LA/RV × musl/glibc rows and internal TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap classification; save raw logs and JSON/TXT summaries under `docs/ltp-score-improvement-2026-05-23/`.
5. If targeted logs reveal low-risk real ABI/errno/metadata/time/signal fixes, implement only those fixes and rerun the affected targeted cases; otherwise document blockers without promotion.
6. Promote only freshly clean cases to `LTP_STABLE_CASES` (prefer 75-85 first) and explain each addition; blocked cases remain outside stable with evidence.
7. Run promoted stable targeted gates on LA and RV, then final full gates: `cargo fmt --all -- --check`, `./run-eval.sh la 2>&1 | tee output_la.md`, `./run-eval.sh 2>&1 | tee output_rv.md`, and `scripts/ltp_summary.py` on both logs.
8. Complete final cleanup/review/quality gate: changed-file anti-slop scan, post-clean verification, code review approval, final report, quality-gate JSON, and Ultragoal final checkpoint.
