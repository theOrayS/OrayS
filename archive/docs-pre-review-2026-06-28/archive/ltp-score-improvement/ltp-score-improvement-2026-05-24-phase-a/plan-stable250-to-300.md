# stable250 -> stable300 Ultragoal plan

Date: 2026-05-24. Mode: Ultragoal + Team-style lane split, leader-owned final `LTP_STABLE_CASES` and promotion decisions.

## Success criteria

- live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: exactly 300 unique cases.
- RV final stable gate: PASS LTP CASE 600, FAIL 0; ltp-musl 300/0; ltp-glibc 300/0.
- LA final stable gate: PASS LTP CASE 600, FAIL 0; ltp-musl 300/0; ltp-glibc 300/0.
- Internal TFAIL=0, TBROK=0; only disclosed known `read02` TCONF remains as `pass_with_tconf`.
- timeout/ENOSYS/panic/trap: 0.
- LTP marker lines start at column 0; 0 bad marker lines.

## Phases

1. Refresh baseline from live code and current summaries; do not rely on memory.
2. Targeted batches of 5-15 cases; only promote cases that are clean across RV+LA and musl+glibc.
3. Promotion gates: stable270 -> stable285 -> stable300 with aggregate parser summaries.
4. Final gate: RV+LA stable aggregate, marker prefix check, fmt/build/make all, code review, ai-slop audit, quality JSON, auto commit.

## Evidence parser

All promotion/final decisions use `python3 -B scripts/ltp_summary.py <log>` summaries, not wrapper exit alone.
