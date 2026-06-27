# Combined candidate pool clean35: shmat04 IPC_STAT ABI

Created: 2026-06-02T15:09:18+08:00

This audit extends the previous clean34 candidate pool with one newly evidenced SysV shm case. It is not a stable-list promotion.

## Incremental evidence

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-shmat04-shmt04-ipcstat-abi-20260602T150702+0800.summary.txt`
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-shmat04-shmt04-ipcstat-abi-20260602T150805+0800.summary.txt`
- Combined RV+LA promotion report: `target/ltp-1000-milestone-03-stable656/combined-shmat04-shmt04-ipcstat-abi-20260602T150918+0800.promotion-candidates.txt`
- Combined checksum: `target/ltp-1000-milestone-03-stable656/combined-shmat04-shmt04-ipcstat-abi-20260602T150918+0800.derived.sha256`

Parser result: RV and LA each report 4 PASS / 0 FAIL for `shmat04,shmt04` across musl+glibc, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap. The combined report lists `shmat04` and already-counted `shmt04` as four-combo clean.

## Newly added candidate

| Case | Reason | Decision |
| --- | --- | --- |
| `shmat04` | Generic `shmctl(IPC_STAT)` now copies the Linux 64-bit `shmid_ds` ABI struct (112 bytes) instead of clearing a guessed 128-byte buffer, so the glibc stack object is no longer overwritten while musl remains clean. | Add to future stable656 candidate pool; do not promote until the +50 gate is complete. |

## Full clean35 pool

- `adjtimex01`
- `adjtimex03`
- `epoll_create1_01`
- `epoll_create1_02`
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
- `mmap20`
- `mprotect02`
- `mprotect04`
- `munlock02`
- `munmap01`
- `openat02`
- `rename01`
- `rename03`
- `rename04`
- `rename05`
- `sched_setaffinity01`
- `shmat04`
- `shmt04`
- `signal01`
- `sigaltstack02`
- `stat03`
- `stat03_64`
- `statfs01`
- `statvfs01`

## Stable-list decision

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains `606 total / 606 unique / 0 duplicate`. The stable656 gate still needs 15 more four-way-clean unique candidates, so no promotion commit is made for this checkpoint.
