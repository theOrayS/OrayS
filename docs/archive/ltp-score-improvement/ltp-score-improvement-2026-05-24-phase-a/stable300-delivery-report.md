# stable300 delivery report

## Delivered state

- live stable cases: total=300, unique=300, duplicates=0.
- stable300 final tranche: `nice01, nice02, prctl01, sethostname01, sethostname02, sethostname03, clock_nanosleep04, nanosleep04, nice03, fcntl23_64, setuid03, prctl05, ftruncate03, truncate03, lseek07`.
- Final RV log: `raw/stable300-rv-final-yield.log`.
- Final LA log: `raw/stable300-la-final-yield.log`.

## Final RV parser summary

- PASS LTP CASE: 600
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 300 passed, 0 failed
- ltp-glibc: 300 passed, 0 failed

## Final LA parser summary

- PASS LTP CASE: 600
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- ltp-musl: 300 passed, 0 failed
- ltp-glibc: 300 passed, 0 failed

## `read02` caveat

`read02` remains in stable as transparent `pass_with_tconf`: 2 entries per architecture aggregate (musl+glibc). It is not described as clean. No other stable case introduced TFAIL/TBROK/TCONF.

## Remote marker regression check

`remote-marker-regression-check.md` records 0 bad marker lines; every `PASS LTP CASE` / `FAIL LTP CASE` marker found in phase raw logs starts at column 0.

## Validation commands

- `python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-24-phase-a/raw/stable300-rv-final-yield.log`
- `python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-24-phase-a/raw/stable300-la-final-yield.log`
- `cargo fmt --all -- --check`
- `make A=examples/shell ARCH=riscv64`
- `make all`
- marker-prefix scanner over `docs/ltp-score-improvement-2026-05-24-phase-a/raw/*.log`
- `df -h / /root && du -sh /root/.codex`

## User-visible / ABI/POSIX behavior

This delivery intentionally changes POSIX-observable syscall behavior for LTP compatibility: iovec validation and vector I/O semantics, shared fd offset and append behavior, pipe `pipe2`/fcntl/ioctl behavior, `/proc/*/comm`, `prctl` name/PDEATHSIG support, sethostname/gethostname UTS nodename behavior, ftruncate/truncate RLIMIT/errno behavior, selected `O_PATH` metadata errno behavior, and `/proc/self/fd` readlink handling.
