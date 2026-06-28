# Plan: LTP stable413 -> stable460 (stretch stable470)

Date: 2026-05-27
Mode: Ultragoal + Team.

## Baseline refresh

- Disk preflight passed at start: `/` and `/root` 36% used, `.codex` 1.2G; no cleanup needed.
- `git status --short` was clean at start.
- Live stable list: 413 total / 413 unique / 0 duplicates.
- stable413 final gate evidence reviewed from `docs/ltp-score-improvement-2026-05-26-phase-a/`:
  - RV: `PASS LTP CASE 826`, `FAIL 0`, `ltp-musl 413/0`, `ltp-glibc 413/0`.
  - LA: `PASS LTP CASE 826`, `FAIL 0`, `ltp-musl 413/0`, `ltp-glibc 413/0`.
  - Internal TCONF remains known transparent `read02` only; no timeout/ENOSYS/panic/trap.
  - Marker-prefix bad lines: 0; remote-sensitive `axfs::fops:297 [AxError::NotADirectory]`: 0.

## Execution strategy

1. Build a fresh candidate matrix from stable413 live state, phase-a reports, inventory, and targeted parser summaries.
2. Run candidate scouting in leader-owned serialized QEMU batches. Workers provide source/readiness reports and worktree patches only.
3. Promote in small increments: stable425, stable440, stable452, stable460; do not batch unknown cases directly into stable.
4. After each promotion, re-count `LTP_STABLE_CASES` and run RV+LA aggregate gates for the new stable list.
5. Stop and demote any candidate that introduces wrapper failure, internal TFAIL/TBROK/TCONF beyond known `read02`, timeout, ENOSYS, panic, or trap.
6. Preserve stable413 as regression baseline and produce blocker/follow-up docs if stable460 is not achieved.

## Initial lane priorities

- Batch A VFS/path: `chmod05`, `fchmod05`, `fchmodat02` already appear in stable413 final summaries as pass rows but must be checked against live stable membership before reuse; scout `access04`, `chmod06/07`, `fchmod02/06`, `statx01/03`, `readlinkat02`, `rename*`, `openat*`.
- Batch B FD/iovec: scout non-duplicated adjacent `writev03`, remaining `readv/writev`, `pipe2_*`, lightweight `fcntl`; avoid O_DIRECT-heavy `preadv03/pwritev03` unless evidence changes.
- Batch C metadata: evaluate `getdents01/02`, `fstat02/_64`, `statfs/fstatfs/statvfs` with narrow fixes only after syscall trace/source evidence.
- Batch D VFS create/remove: narrow repair candidates `open06`, `creat04`, `mkdir04`, `rmdir03`, `unlink08`; scout `unlink07`, `open07`, `creat06`, `mkdir03`, `rmdir02` after sanity.
- Batch E process/light syscall: `waitid07/08/10`, cautious `kill02`, and light syscalls only with aggregate proof.
- Batch F mmap/fs substitutes: use as fill candidates when lower-risk VFS/FD/metadata lanes under-yield.

## Required artifacts

- `ultragoal-brief-stable413-to-460.md`
- `candidate-matrix-stable413-to-460.md`
- `stable425-promotion-gate-report.md`
- `stable440-promotion-gate-report.md`
- `stable452-promotion-gate-report.md`
- `stable460-delivery-report.md`
- `final-gate-quality-gate.json`
- `final-gate-code-review-report.md`
- `final-gate-ai-slop-cleaner-report.md`
- `remote-marker-and-log-noise-regression-check.md`
- `next-session-prompt-stable460-followup.md` if stable460 is blocked or stretch remains valuable.

## Verification stop rule

The campaign is complete only when live stable count is 460 unique / 0 duplicates, RV and LA stable final gates each pass 920 LTP rows with 0 failures, only known `read02` internal TCONF is present, marker/noise checks pass, code review and ai-slop-cleaner pass, post-disk check is recorded, and agent-owned changes are committed.

## Progress checkpoints

- stable425: reached 425 total / 425 unique / 0 duplicates; RV and LA aggregate gates both `PASS LTP CASE 850`, `FAIL 0`.
- stable440: reached 440 total / 440 unique / 0 duplicates; RV and LA aggregate gates both `PASS LTP CASE 880`, `FAIL 0`.
- stable452: reached 452 total / 452 unique / 0 duplicates; RV and LA aggregate gates both `PASS LTP CASE 904`, `FAIL 0`; only known `read02` TCONF remains. Clean reserves for stable460: `fchown05`, `fchownat01`.
