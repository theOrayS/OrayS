# Milestone 03 stable656 no-promotion reason - superseded

This document is retained only as repair-history context. It is superseded by the final stable656 promotion gate on 2026-06-02.

Final status: **promotion achieved**. `LTP_STABLE_CASES` is `656 total / 656 unique / 0 duplicate`.

Promoted new50 unique cases:

`adjtimex01, adjtimex03, epoll_create1_01, epoll_create1_02, fcntl11_64, fcntl15, fstatfs01, fstatfs01_64, fsync02, futex_wait01, futex_wait03, futex_wait05, getitimer02, lstat02, lstat02_64, mincore02, mincore03, mincore04, mmap13, mmap20, mprotect02, mprotect04, munlock02, munmap01, open07, open12, openat02, pause01, pause02, rename01, rename03, rename04, rename05, sched_setaffinity01, setitimer02, shmat04, shmt04, signal01, sigaltstack02, stat03, stat03_64, statfs01, statvfs01, utime01, utime02, utime03, utime04, utime05, utime06, utime07`

Counting boundary:

- RV final gate: `target/ltp-1000-milestone-03-stable656/rv-stable656-new50-post-timer-safe-20260602T163655+0800.summary.txt` -> `100 PASS / 0 FAIL`, musl 50/50 and glibc 50/50, parser-clean.
- LA final gate: `target/ltp-1000-milestone-03-stable656/la-stable656-new50-final-timer-safe2-20260602T163306+0800.summary.txt` -> `100 PASS / 0 FAIL`, musl 50/50 and glibc 50/50, parser-clean.
- No blacklist/SKIP/status0/full-sweep partial rows are counted.
- Older candidate-pool-below-50 statements in this file are superseded and must not be used as current state.
