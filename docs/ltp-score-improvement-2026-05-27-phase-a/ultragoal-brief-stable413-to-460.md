# Ultragoal brief: stable413 -> stable460 (stretch stable470)

Date: 2026-05-27
Mode: Ultragoal + Team, leader-owned promotion/final gates.
Working directory: `/root/oskernel2026-orays`.

## Target result

- Main goal: promote exactly 47 additional real clean LTP stable cases, from live stable413 to stable460.
- Stretch goal: promote to stable470 only if enough RV+LA x musl+glibc clean evidence exists and final-gate resources allow.
- Stop condition: stable460 delivered with final gates, reviews, marker/noise check, post-disk check, and commit; or highest trustworthy stableN plus blocker report and follow-up prompt if stable460 is blocked.

## Live baseline to protect

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: 413 total / 413 unique / 0 duplicates at campaign start.
- Handoff commit: `76506626 Promote stable LTP coverage with verified low-risk FD/sendfile semantics`.
- stable413 evidence root: `docs/ltp-score-improvement-2026-05-26-phase-a/`.
- Final summaries: RV and LA each `PASS LTP CASE 826`, `FAIL 0`, `ltp-musl 413/0`, `ltp-glibc 413/0`.
- Known transparent internal TCONF remains `read02` only: 4 internal TCONF per arch final summary; no new TCONF may be accepted as clean.
- Marker guardrail: wrapper LTP marker lines must begin at column 0; bad marker-prefix lines must remain 0.
- Log-noise guardrail: remote-sensitive `axfs::fops:297 [AxError::NotADirectory]` must remain 0; residual `axfs_ramfs::file:69` NotADirectory should be disclosed.

## Stages

1. `baseline-refresh`: re-count stable list, review stable413 summaries/quality gate/reports, check disk/git/team/OMX preconditions.
2. `candidate-matrix`: build `candidate-matrix-stable413-to-460.md` from live stable list, phase-a reports, sdcard/runtest inventory, and fresh targeted parser evidence.
3. `stable425`: add about 12 clean cases; run serialized RV+LA promotion gate and parser summaries.
4. `stable440`: add about 15 clean cases; run serialized RV+LA promotion gate and parser summaries.
5. `stable452`: add about 12 clean cases; run serialized RV+LA promotion gate and parser summaries.
6. `stable460`: add about 8 clean cases; run final stable RV+LA gate and all quality gates.
7. `optional-stable470`: only if clean surplus and resources allow; otherwise document deferred cases.

## Promotion gate

A case is promotion-clean only when all four rows pass: RV musl, RV glibc, LA musl, LA glibc. Clean means wrapper PASS plus zero internal TFAIL/TBROK/new TCONF, timeout, ENOSYS/not-implemented, panic, and trap. Wrapper success alone is never sufficient; use `python3 -B scripts/ltp_summary.py` or an equivalent matrix parser.

## Team split

- Leader: owns `.omx/ultragoal`, `LTP_STABLE_CASES`, promotion decisions, serialized QEMU gates, final validation, docs synthesis, and commit.
- Worker 1: discovery + candidate/promotion matrix.
- Worker 2: VFS/permissions/path lane (`access/chmod/fchmod/statx/readlinkat/rename/openat/link/unlink/symlink/mkdir/rmdir/truncate/ftruncate`).
- Worker 3: metadata/statfs/getdents lane (`getdents64`, `fstat`, `statfs/statvfs`, `getcwd`).
- Worker 4: FD/pipe/iovec/fcntl lane adjacent to stable413 sendfile/preadv/pwritev.
- Worker 5: mmap/process/signal + verification guardrail lane.

Workers may discover, diagnose, patch assigned worktrees, and report evidence. Workers must not mutate `.omx/ultragoal`, must not checkpoint Ultragoal, and must not make final `LTP_STABLE_CASES` promotion edits.

## High-value candidate pools

Prioritize VFS/permission/path, FD/pipe/iovec/fcntl adjacent, metadata/statfs/getdents, VFS create/open/remove, process/wait/signal/light syscall, and mmap/mprotect/fs-suite substitutes. Do not prioritize low-ROI/high-risk families such as `fs_bind*`, `ksm*`, `fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*`, broad xattr, namespace, io_uring, or perf.

## Final gate

- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv`
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la`
- `python3 -B scripts/ltp_summary.py <rv-log>` and `<la-log>`.
- marker-prefix/noise check.
- `cargo fmt --all -- --check`, `git diff --check`, and at least `make A=examples/shell ARCH=riscv64`.
- `make all`/offline `make all` only if remote submission/build helper behavior is touched.
- Final review artifacts: code review, ai-slop-cleaner audit, quality gate JSON, post-disk check.

## Commit and artifact guardrails

- Do not submit root `kernel-rv`/`kernel-la`, sdcard/disk images, large raw logs, or user evidence files `Riscv输出.txt`/`LoongArch输出.txt` unless explicitly requested.
- Raw logs go under `docs/ltp-score-improvement-2026-05-27-phase-a/raw/` and are not committed by default when large.
- Commit only agent-owned tracked/docs/source changes with Lore commit messages after verification.

## CLI story schedule

STORY baseline-refresh: prove live stable413 baseline, stable413 final gate summaries, disk/git/team/OMX preconditions, and write startup artifacts.

STORY candidate-matrix: build candidate-matrix-stable413-to-460.md from live stable list, phase-a reports, inventory, and fresh targeted parser evidence.

STORY stable425: promote about 12 RV+LA x musl/glibc clean cases, update LTP_STABLE_CASES only after proof, and write stable425-promotion-gate-report.md.

STORY stable440: promote about 15 additional clean cases and write stable440-promotion-gate-report.md.

STORY stable452: promote about 12 additional clean cases and write stable452-promotion-gate-report.md.

STORY stable460-final: promote the remaining clean cases to exactly 460 unique, run final RV+LA stable gates, marker/noise check, format/diff/build checks, code review, ai-slop-cleaner, quality gate, post-disk check, and commit.

STORY optional-stable470-or-followup: if stable470 is feasible, promote and gate it; otherwise write blocker/deferred-candidate report plus next-session prompt.
