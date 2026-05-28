# Phase C Ultragoal Brief: stable375 -> stable450

Date: 2026-05-25
Working directory: `/root/oskernel2026-orays`

## Objective

1. First fix the high-frequency remote output noise from `axfs::fops:297 [AxError::NotADirectory]` without changing POSIX-visible errno behavior.
2. Promote the live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` from stable375 to stable450 using honest RV+LA × musl+glibc evidence.
3. Stretch to stable460/stable475 only if clean evidence is already available and final gates remain affordable.

## Baseline refreshed in this session

- `git status --short`: clean before phase-c edits.
- Disk preflight: `/` at 71%, 17G free; `/root/.codex` at 22G; no cleanup required before starting.
- Live stable count: `375 total / 375 unique / 0 duplicates` from `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- stable375 phase-b evidence reviewed under `docs/ltp-score-improvement-2026-05-25-phase-b/`.
- Remote glibc-only logs parse as 375/0 on both RV and LA but include about 4.5k `AxError::NotADirectory` warnings per arch, so output-size risk remains the primary guardrail.

## Guardrails

- No fake PASS, case-name hardcoding, or LTP source edits.
- Timeout/ENOSYS/panic/trap/TFAIL/TBROK are never promotion-clean.
- Known `read02` TCONF remains visible.
- Marker lines must keep `PASS LTP CASE` / `FAIL LTP CASE` at column 1.
- Workers do not own `.omx/ultragoal` or final `LTP_STABLE_CASES` promotion changes; leader owns final gates and checkpoints.
- Do not submit root kernels, sdcard/disk images, huge raw logs, or user-provided remote logs.

## Stories

G001 log-noise-fix: repair `kernel/fs/axfs/src/fops.rs::_open_dir_at()` expected `NotADirectory` warning noise while preserving `Err(AxError::NotADirectory)`. Validate formatting/build and a small LTP subset; write `log-noise-repair-report.md`.

G002 stable400: discover candidates from Batch A-E, run targeted evidence, promote about 25 clean cases, verify RV+LA targeted/aggregate, and write `stable400-promotion-gate-report.md`.

G003 stable425: repeat promotion for the next about 25 clean cases with fresh evidence and write `stable425-promotion-gate-report.md`.

G004 stable450-final: promote to exactly 450 unique cases, run full final RV+LA stable gates, marker/noise regression checks, code review, ai-slop-cleaner audit, quality gate JSON, and delivery report.

G005 optional-stretch: only after stable450 is clean, consider stable460/stable475 with the same evidence bar; otherwise write follow-up prompt/blocker report.
