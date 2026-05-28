# Ultragoal brief: stable350 -> stable375 / stretch stable380

Source prompt and full constraints: `docs/ltp-score-improvement-2026-05-25-phase-b/next-session-prompt-stable350-to-375.md`.

Objective: use leader-owned Ultragoal plus OMX Team to raise live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` from stable350 to stable375, with optional stable380 stretch only if clean evidence exists.

Non-negotiable constraints:
- Re-read `AGENTS.md`, run disk/Git preflight, and live-recompute stable case total/unique/duplicates.
- Stable350 final RV/LA gates and marker-prefix evidence under `docs/ltp-score-improvement-2026-05-25-phase-a/` are regression protection.
- Only promote cases that are clean across RV+LA × musl+glibc, parsed by `python3 -B scripts/ltp_summary.py` or an equivalent matrix. Wrapper success alone is not promotion evidence.
- Do not fake PASS, hardcode case names, edit LTP sources, launder failures into SKIP/TCONF/PASS, count timeouts as PASS, or hide TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Keep `read02` pass_with_tconf transparent if it remains in stable.
- Marker lines must stay at column 1: `PASS LTP CASE` / `FAIL LTP CASE`, with 0 bad marker lines.
- Leader owns `.omx/ultragoal`, final `LTP_STABLE_CASES` edits, promotion decisions, final gates, code review, ai-slop-cleaner, and commits. Workers provide discovery/fix/verification/report evidence only.
- Do not submit root-level kernels, sdcard/disk images, large raw logs, or user-provided `Riscv输出.txt` / `LoongArch输出.txt` unless explicitly requested.

Stories:
1. Baseline refresh and plan artifacts: validate disk/Git/head/stable350 evidence, create `plan-stable350-to-375.md`, context snapshot, and raw directory.
2. Team discovery and lane reports: launch 5 executor workers (fallback 4), assign discovery/VFS/FD-process/VM guardrail lanes, and collect candidate matrix without allowing worker QEMU evidence to become promotion evidence.
3. Stable360 promotion: identify about 10 clean high-ROI cases, update stable list only after leader serial proof, run RV+LA targeted/aggregate gates, parse summaries, write `stable360-promotion-gate-report.md`, checkpoint.
4. Stable368 promotion: add the next clean tranche, run leader serial gates, parse summaries, write `stable368-promotion-gate-report.md`, checkpoint.
5. Stable375 final delivery: reach exactly 375 unique live stable cases, run final RV+LA stable gates, marker-prefix check, `cargo fmt --all -- --check`, at least `make A=examples/shell ARCH=riscv64`, code-review and ai-slop-cleaner audits, quality gate JSON, and delivery report.
6. Optional stable380 stretch: only if stretch cases are already clean and resources allow; otherwise document blockers/follow-up without delaying stable375.
7. Commit and handoff: commit agent-owned tracked changes with Lore trailers if validation is clean; if blocked before stable375, save highest trustworthy stableN evidence and blocker report with exact failing cases/arch/libc/signals/log paths.
