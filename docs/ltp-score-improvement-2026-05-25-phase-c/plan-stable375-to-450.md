# Plan: stable375 -> stable450

## Phase 0 baseline

- Disk: `/` 71% used, 17G available; `/root/.codex` 22G.
- Worktree: clean before phase-c edits.
- Live stable list: 375 total / 375 unique / 0 duplicates.
- stable375 phase-b final gate evidence reviewed: RV and LA each `PASS LTP CASE 750`, `FAIL 0`, `ltp-musl 375/0`, `ltp-glibc 375/0`, known `read02` TCONF only.
- Remote glibc-only evidence parses as 375/0/0 on both RV and LA, but contains about 4.5k `AxError::NotADirectory` lines per arch and only one libc suite, confirming output-size guardrail risk.

## Stage 1: log-noise repair

- Replace the expected `NotADirectory` return in `Directory::_open_dir_at()` with a non-logging `Err(AxError::NotADirectory)` path.
- Keep syscall-visible errno unchanged.
- Validate with formatting, diff check, riscv64 shell build, and a small LTP subset including path-negative cases.
- Record before/after AxError counts and marker-prefix checks in `log-noise-repair-report.md`.

## Stage 2: candidate discovery

- Use workers to build a candidate matrix from metadata/permission, fd/pipe/iovec, process/wait/signal, mmap/VM, and fs-suite pools.
- Targeted validation may use 20-40 cases per batch, but promotion requires RV+LA × musl+glibc clean evidence parsed with `scripts/ltp_summary.py` or equivalent matrix.

## Stage 3: promotion gates

- stable400: add about 25 clean cases, verify count/duplicates, run RV+LA targeted/aggregate gate, write report.
- stable425: repeat.
- stable450: repeat, then full final stable gates.
- If a blocker appears, demote the candidate and continue with substitutes; do not let one case block the round unless no safe substitutes remain.

## Stage 4: final quality gate

- Full `LTP_CASES=stable` RV and LA gates.
- Summary parser on both logs.
- Marker-prefix check: zero bad marker lines.
- AxError noise regression stats.
- `cargo fmt --all -- --check`, `git diff --check`, `make A=examples/shell ARCH=riscv64`; `make all` if remote-submission behavior was touched.
- Final code review and ai-slop-cleaner audit reports.
