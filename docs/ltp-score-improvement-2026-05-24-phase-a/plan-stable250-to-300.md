# LTP stable250 -> stable300 plan (2026-05-24 phase-a)

## Target

Promote `examples/shell/src/cmd.rs::LTP_STABLE_CASES` from the live stable250 baseline to exactly 300 unique LTP cases.

## Live baseline refreshed

- Source: `examples/shell/src/cmd.rs::LTP_STABLE_CASES`
- Refreshed at: 2026-05-24 Asia/Shanghai
- Entries: 250
- Unique cases: 250
- Duplicates: 0
- Baseline case snapshot: `stable250-live.cases`
- Previous final gate evidence: `docs/ltp-score-improvement-2026-05-22-phase-d/stable250-post-ansi-*-summary.txt`

## Hard constraints

- No fake PASS, no case-name hardcoding, no hiding real failures as SKIP/TCONF/PASS.
- Timeout, ENOSYS, panic/trap, TFAIL, and TBROK block promotion.
- Wrapper success is not enough; every gate must be parsed with `python3 -B scripts/ltp_summary.py` or an equivalent case matrix.
- `read02` remains transparent as known `pass_with_tconf`; new promoted cases must be clean.
- Remote marker regression must not return: `PASS LTP CASE` / `FAIL LTP CASE` lines must start at column 0.
- Root-level kernels, sdcard/disk images, large raw logs, and user `Riscv输出.txt` / `LoongArch输出.txt` are not committed by default.

## Promotion stages

### Stage 1: stable270

1. Discover and classify at least 20 clean candidates.
2. Run targeted batches of 5-15 cases, first RV when risk is unknown, then LA.
3. Require RV + LA, musl + glibc clean targeted evidence for each promoted case.
4. Leader updates `LTP_STABLE_CASES` only after evidence is clean.
5. Run stable aggregate gate and write `stable270-promotion-gate-report.md`.

### Stage 2: stable285

Same gate as stable270 for +15 additional cases. Write `stable285-promotion-gate-report.md`.

### Stage 3: stable300

Same gate as stable270 for +15 additional cases. Write `stable300-delivery-report.md`, quality gate JSON, code review report, ai-slop-cleaner report, and remote marker regression report.

## Team lanes

1. Discovery + promotion matrix: candidate pool 250->330 and classification.
2. proc/sched/wait/rlimit/process lane.
3. fd/pipe/open/access/fcntl/fsuid/permission lane.
4. fs/metadata/stat/statfs/link/rename/truncate lane.
5. time/signal/timer/memory/mmap + verification guardrail lane.

## Final gate commands

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la
python3 -B scripts/ltp_summary.py <rv-log>
python3 -B scripts/ltp_summary.py <la-log>
cargo fmt --all -- --check
make A=examples/shell ARCH=riscv64
```

Run `make all` if remote-submission behavior or build wrappers are affected.
