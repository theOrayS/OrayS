# Stable400 attempt 5 inventory and statx-tail scout report

Date: 2026-05-26
Status: **negative scout plus candidate-inventory improvement; no promotion**.

## Purpose

After `lseek03`-`lseek10` failed because the test binaries were missing, the next step was to stop blindly trying names from upstream-style families and inspect the actual RV sdcard image. The inventory was generated with `debugfs` from `sdcard-rv.img` for both libc trees.

## RV sdcard inventory

Generated artifacts:

- `raw/sdcard-rv-musl-ltp-bin-list.txt`: 2824 musl LTP binaries/scripts.
- `raw/sdcard-rv-glibc-ltp-bin-list.txt`: 2844 glibc LTP binaries/scripts.
- `raw/sdcard-rv-common-not-stable-ltp-bins.txt`: 2442 names present in both libc trees but not in live stable382.

High-ROI families confirmed present but not stable include:

- metadata/path: `access04`, `chmod06`, `chmod07`, `fchmod02`, `fchmod06`, `statx01`, `statx03`-`statx12`, `openat02`-`openat04`, `rename01`, `rename03`-`rename14`, `link02`, `unlink05`, `mkdir02`;
- FD/pipe/iovec: many `fcntl*`, `dup05`, `pipe02`, `pipe07`, `pipe08`, `pipe15`, `writev03`, `pread*`, `pwrite*`, `sendfile*`;
- process/signal: `kill02`, `waitid07`, `waitid08`, `waitid10`;
- VM/fs substitutes: `mmap04`, `mmap05`, `mmap12`-`mmap20`, `munmap01`-`munmap03`, `mprotect01`-`mprotect05`, `fs_perms`, `ftest06`, `rwtest`, `stream02`, `openfile`, `writetest`, `iogen`, `fs_inod`, `inode02`.

This inventory is a discovery aid only. Presence in the image is not promotion evidence.

## Existing RV-clean evidence scan

Parsing existing phase-c summary files found only these non-stable cases with RV musl+glibc pass-clean evidence:

- `readlinkat02`: already blocked by LA-musl call-boundary split; do not promote from RV-only evidence.
- `inode02`: already blocked by LA glibc timeout; do not promote from RV-only evidence.

No other non-stable case in existing summaries had RV musl+glibc pass-clean evidence.

## RV statx-tail scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=statx04,statx05,statx06,statx07,statx08,statx09,statx10,statx11,statx12 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-25-phase-c/raw/target-stable400-statx-tail-rv-001.log
```

Summary evidence: `raw/target-stable400-statx-tail-rv-001-summary.txt`.

Result:

- PASS LTP CASE: 0
- FAIL LTP CASE: 18
- `ltp-musl`: 0 passed, 9 failed
- `ltp-glibc`: 0 passed, 9 failed
- Internal: TBROK 14, TCONF 4
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

Blocker classes:

- `statx04`, `statx06`, `statx08`, `statx10`, `statx11`, `statx12`: `Failed to create test_dev.img: ENOSPC` / `Failed to acquire device` TBROK.
- `statx05`: TCONF, missing `mkfs.ext4`.
- `statx07`: TCONF, missing `exportfs`.
- `statx09`: TBROK, cannot parse kernel `.config`.

## Decision

No `statx04`-`statx12` case is promotable. Do not retry this tail until the device-space/tooling/config prerequisites are deliberately addressed, and do not convert those setup failures into SKIP/PASS.

Next high-yield search should use the sdcard inventory first, then target small batches that are present in both libc trees and not already blocked by device acquisition, missing host-style tooling, kernel config parsing, record locking, or known LA splits.
