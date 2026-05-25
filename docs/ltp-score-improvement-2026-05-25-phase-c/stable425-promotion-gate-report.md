# Stable425 promotion gate report

Date: 2026-05-26
Status: **not achieved**.

Stable425 was not attempted as a promotion target after the campaign only produced four aggregate-clean new cases. Highest trusted live list: stable379.

Required before revisiting stable425:

1. Find at least 46 more RV+LA x musl+glibc clean cases beyond stable379.
2. Run targeted matrices first; reject any case with wrapper FAIL, internal TFAIL/TBROK, new TCONF, parser timeout, ENOSYS, panic/trap, or arch/libc split.
3. Run serial aggregate gates on RV and LA after each promotion batch.
4. Keep marker prefix bad lines at 0 and keep remote log-noise volume guarded.

Current blocker classes are documented in `candidate-matrix.md` and `next-session-prompt-stable450-followup.md`.
