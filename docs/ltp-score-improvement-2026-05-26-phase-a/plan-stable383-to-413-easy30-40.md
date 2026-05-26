# Plan: stable383 -> stable413/423 easy-first LTP campaign

Date: 2026-05-26
Mode: Ultragoal + Team
Leader cwd: `/root/oskernel2026-orays`

## Success criteria

- Main delivery: live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` exactly 413 total / 413 unique / 0 duplicates.
- Stretch: 423 total / 423 unique / 0 duplicates if enough clean cases remain.
- RV and LA stable aggregate gates pass serially.
- For stable413: each arch reports `PASS LTP CASE 826`, `FAIL 0`, `ltp-musl 413/0`, `ltp-glibc 413/0`.
- No internal TFAIL/TBROK; no new TCONF beyond transparent `read02`; timeout/ENOSYS/panic/trap all 0.
- Marker-prefix bad lines 0; `axfs::fops:297 [AxError::NotADirectory]` high-frequency noise remains 0 in completed logs.
- Code review and ai-slop-cleaner audit complete before final Ultragoal completion.

## Baseline refreshed

- Disk preflight: `/` 69%, 18G free; `/root/.codex` 20G; no cleanup required.
- `git status --short`: clean before phase-a edits.
- Live stable list: 383 total / 383 unique / 0 duplicates.
- Retained stable375->383 cases: `clock_settime01`, `clock_settime02`, `clone03`, `confstr01`, `chmod05`, `fchmod05`, `lseek02`, `pipe08`.
- Old stable450 Ultragoal was read and found failed/cancelled, not resumed.

## Execution stages

1. Discovery matrix: filter candidate pool by live stable list, sdcard inventory, phase-c negative evidence, and subsystem cost.
2. RV scout: run small 15-25 case batches; only RV musl+glibc clean candidates move forward.
3. LA confirm: run only RV-clean subset; only four-way clean candidates enter promotion list.
4. Promote in 8-12 case chunks: stable393, stable403, stable413, optional stable423.
5. Final quality gates: serial RV+LA stable gate, parser summaries, marker/noise check, fmt/diff/build, code review, ai-slop-cleaner, commit.

## Worker lanes

- Worker 1: inventory + easy candidate matrix; report only.
- Worker 2: lightweight syscall/process lane; narrow low-risk fixes only after reporting risk.
- Worker 3: metadata/statfs/getdents lane; ABI/copy-out caution.
- Worker 4: FD/IO/sendfile/pread/pwrite/iovec lane; offset/O_APPEND/user-iovec caution.
- Worker 5: VFS small create/remove + fs-suite guardrail; no fake-pass/marker/timeout guardrail.

Workers do not mutate `.omx/ultragoal` or final `LTP_STABLE_CASES`. Promotion gates are leader-serialized.
