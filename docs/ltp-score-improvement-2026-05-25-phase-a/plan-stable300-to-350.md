# Plan: LTP stable300 -> stable350

## Baseline refresh

- Disk preflight: `df -h / /root`; `/root/.codex` size via `du -sh /root/.codex`.
- Git preflight: `git status --short`; preserve user evidence logs such as `Riscv输出.txt` and `LoongArch输出.txt`.
- Live stable count: parse `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; initial snapshot saved as `stable300-live.cases`.
- Prior evidence: review `docs/ltp-score-improvement-2026-05-24-phase-a/` final summaries, candidate matrix, delivery report, and quality gate.

## Stage gates

1. `stable315`: choose about 15 RV+LA × musl+glibc clean cases from targeted batches, update stable list only after clean evidence, run stable aggregate gate, and write `stable315-promotion-gate-report.md`.
2. `stable330`: repeat with the next clean tranche and write `stable330-promotion-gate-report.md`.
3. `stable350`: repeat to 350 unique cases, then run final stable gate and write `stable350-delivery-report.md` plus quality/review artifacts.

## Candidate strategy

- First-pass candidates come from stable300 deferred blockers and neighboring high-value syscall families.
- Batches should be 5-15 cases for promotion evidence; broader 80-150 case discovery is allowed only to find clean subsets.
- Promotion requires clean parser evidence, not wrapper success alone.
- If a high-value blocker remains non-clean, record it and move to a different clean subset.

## Team lanes

- Worker 1: discovery + candidate matrix.
- Worker 2: permissions/VFS/errno/metadata.
- Worker 3: fd/pipe/iovec/fcntl.
- Worker 4: process/wait/sched/rlimit/proc.
- Worker 5: mmap/mprotect/munmap/signal/time + marker/no-fake-pass guardrails.

## Final gate

- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv`
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la`
- `python3 -B scripts/ltp_summary.py <rv-log>` and `<la-log>`
- marker-prefix scanner over phase raw logs
- `cargo fmt --all -- --check`
- `make A=examples/shell ARCH=riscv64`
- `make all` only when remote submission build is touched or final policy requires it.
