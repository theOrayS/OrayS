# Combined clean28 candidate pool audit

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Stable baseline: `606 total / 606 unique / 0 duplicate`
Stable target: `656`

This document is a milestone-local audit summary. It combines the previous parser-clean clean26 pool with the current parser-generated clean2 report for `mmap20` and `munlock02`. It is **not** a stable-list update and does not count regression-only stable rows.

## Source evidence

- Previous clean26 parser report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean26-stat03-path-20260602T052251Z.promotion-candidates.txt`
- Current clean2 parser report: `target/ltp-1000-milestone-03-stable656/mmap20-munlock02-clean2-20260602T054508Z.promotion-candidates.txt`
- Current clean2 report checksum: `target/ltp-1000-milestone-03-stable656/mmap20-munlock02-clean2-20260602T054508Z.promotion-candidates.txt.sha256`
- Current targeted RV summary: `target/ltp-1000-milestone-03-stable656/rv-mmap20-munlock02-targeted-20260602T054424Z.summary.txt`
- Current targeted LA summary: `target/ltp-1000-milestone-03-stable656/la-mmap20-munlock02-targeted-20260602T054508Z.summary.txt`

## Current candidate count

- Four-way-clean not-yet-stable candidates: 28
- Remaining cases needed before editing `LTP_STABLE_CASES` for stable656: 22
- Stable list impact: unchanged at `606/606/0`

## Candidates

- `fcntl11_64`
- `fcntl15`
- `fstatfs01`
- `fstatfs01_64`
- `fsync02`
- `futex_wait01`
- `futex_wait03`
- `futex_wait05`
- `mincore02`
- `mincore03`
- `mincore04`
- `mmap13`
- `mprotect02`
- `mprotect04`
- `munmap01`
- `openat02`
- `rename01`
- `rename03`
- `rename04`
- `rename05`
- `sched_setaffinity01`
- `signal01`
- `stat03`
- `stat03_64`
- `statfs01`
- `statvfs01`
- `mmap20`
- `munlock02`

## Non-countable evidence boundary

- `rv-mmap-munlock-errno-targeted-20260602T053636Z.log` is repair-history evidence only: `mmap20` and `munlock02` were clean, while `mmap08` and `mlock02` remained parser-visible failures.
- Temporary debug logs (`rv-mmap08-debug-*`) are diagnostic only and are not promotion evidence.
- Adjacent regression logs prove no mmap/mincore/mprotect/munlock regression but are not used to inflate the not-yet-stable candidate count with already-stable rows.
