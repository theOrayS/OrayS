# Next-session prompt — continue stable379 toward stable450

Working directory: `/root/oskernel2026-orays`
Language: Chinese reports.
Mode: Use Ultragoal + Team only if the new session can run QEMU serial promotion gates; workers may do discovery but must not race shared QEMU/sdcard images.

## Current honest state

- Phase-c log-noise repair changed expected VFS errno paths from `ax_err!` to direct `Err(AxError::...)`, preserving visible errno while suppressing high-frequency warnings.
- Stable375 baseline was live-rechecked as 375 total / 375 unique / 0 duplicates.
- Four cases have fresh RV+LA x musl+glibc clean targeted evidence but are **not currently in `LTP_STABLE_CASES`** because the stable379 aggregate gate failed on existing `ftest03` timeout: `clock_settime01`, `clock_settime02`, `clone03`, `confstr01`.
- Live stable list after blocker handling: 375 total / 375 unique / 0 duplicates.
- Do not call this stable400/425/450 unless the aggregate gate and later promotions prove it.

## Must-do first

1. Re-run `git status --short` and count live `LTP_STABLE_CASES`.
2. Parse the latest stable379 aggregate summaries if present under `docs/ltp-score-improvement-2026-05-25-phase-c/raw/`.
3. If re-attempting stable379, first address or explicitly preserve the existing `ftest03` timeout blocker without counting it as PASS; do not add pending cases until the aggregate gate is clean.
4. If stable379 is clean, checkpoint it as the highest trusted baseline.

## Best next blockers

1. `readlinkat02`: RV clean and LA glibc clean, but LA musl TFAIL. Inspect readlinkat zero-size/null-buffer errno semantics.
2. `chmod05`/`fchmod05`: RV glibc clean, RV musl TBROK likely group lookup/setup compatibility.
3. `inode02`: RV clean, LA musl clean, LA glibc timeout; investigate LA runtime/memory before promotion.
4. Time/clone neighbors: `clock_settime01/02`, `clone03`, and `confstr01` are clean; adjacent timer/clone cases have TFAIL/TBROK/timeout/ENOSYS and need real fixes.
5. fs-suite staging: `ftest09` and `openfile01` wrapper `-1` may be missing executable/path staging, but do not modify LTP tests or fake PASS.

## Promotion rule

Only add a case to `LTP_STABLE_CASES` after RV+LA x musl+glibc all pass cleanly with `scripts/ltp_summary.py`: no wrapper FAIL, no internal TFAIL/TBROK/new TCONF, no timeout, no ENOSYS, no panic/trap. Keep `read02` TCONF transparent.
