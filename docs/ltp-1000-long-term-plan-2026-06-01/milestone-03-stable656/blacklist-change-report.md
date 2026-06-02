# Milestone 03 stable656 blacklist change report

No blacklist changes were made in this checkpoint.

## Severe blockers observed

| Case | Blocker type | Blacklist decision |
| --- | --- | --- |
| `kill10` | isolated RV singleton reproduces musl timeout, persistent post-cleanup frame leak, and glibc allocator panic; temporary poll/exit-group cleanup hypothesis rejected | not blacklisted here; severe blocker remains; removal requires normal RV/LA x musl/glibc completion with no timeout, panic/trap, or resource-pollution delta |
| old `futex_wait03` scout row | timeout in both libcs before procfs repair | not blacklisted here; superseded by current clean targeted evidence |
| old `futex_wait05` scout/terminated rows | slept-too-long or incomplete LA regression before precise timer/periodic-deadline repair | not blacklisted here; superseded by current clean targeted and regression evidence |
| old `signal01` scout row | timeout before poll-wait proc-state repair | not blacklisted here; superseded by current clean targeted evidence |
| `shmat1` | mixed scout had long/hung behavior and was manually terminated | not blacklisted here; evidence is scouting-only |
| `mmap05` | LA musl+glibc still report `TFAIL=1` / SIGSEGV signal not received after RV became clean | not blacklisted here; recorded as LoongArch fault-signal repair candidate |
| `munmap01` | previously failed with wrapper code 139; now four-way clean after catchable synchronous `SIGSEGV` repair | not blacklisted here; counted only in the clean candidate pool, not promoted yet |
| `mincore03` | old mixed scout reported `TBROK`/`ENOMEM`; now four-way clean after generic lazy-VMA `mincore` validity/residency handling plus `mlock` prefault | not blacklisted here; counted only in the clean candidate pool, not promoted yet |
| `mmap13` | pre-fix `TFAIL` / SIGBUS signal not received | not blacklisted here; repaired by generic file-backed mmap SIGBUS-on-EOF handling and now tracked as a clean candidate |
| `readlinkat02` | LA musl `TFAIL` from musl zero-size wrapper rewriting to a one-byte syscall | not blacklisted here; ordinary libc/test boundary and not promotion evidence |
| `clone04` | RV glibc clean but RV musl `TBROK` / killed by SIGSEGV, with LTP hint toward musl `clone.c` wrapper behavior | not blacklisted here; ordinary libc-wrapper boundary and not promotion evidence |
| `mmap10_1` | missing testcase inventory | excluded from promotion; no blacklist change |
| `vma02` | `TCONF` libnuma requirement | excluded from promotion; no blacklist change |

## Closed arch-sweep mining

Re-mining `rv-arch002.log` and `la-arch012.log` did not change the blacklist. The not-stable four-way-clean filter was empty, and remaining failures/TCONF/TBROK/TFAIL/ENOSYS/timeout rows are blocker evidence only.

## Boundary

These failures are not hidden. They are not counted as PASS, not promoted to stable, and not converted into blacklist credit. If future full-sweep lanes need temporary blacklist isolation, the report must record the severe-blocker reason, source, and removal condition separately.

## `openat03` blocker update

No blacklist entry was added for `openat03`. The rejected `O_TMPFILE`/`linkat` emulation produced RV panic/trap evidence and was removed; the retained generic unsupported gate produces visible `TCONF`/wrapper FAIL on RV/LA x musl/glibc with zero panic/trap. This is an ordinary unresolved feature/VFS robustness blocker, not blacklist credit and not promotion evidence.


## `kill10` isolated blocker update

Two RV singleton runs on 2026-06-02 confirmed that `kill10` is not merely a noisy broad-shard artifact. Both runs show musl wrapper FAIL 137 after the 120s timeout, about `-129185` free frames after cleanup, then an immediate glibc allocator panic. A temporary generic `poll`/`ppoll` pending-exit cleanup change did not alter the parser result or resource delta and was removed. No blacklist credit or promotion credit is taken from these rows.

## `epoll_create02` blocker update

No blacklist change was made for `epoll_create02`. The focused singleton evidence is an ordinary unresolved compatibility/parser-clean blocker: RV musl has `TFAIL=2` / `ENOSYS=2`, and LA still has old-ABI `TCONF` rows despite wrapper PASS. These rows stay visible, are not counted as PASS, and are not converted into blacklist credit.


## G009 mm/mlock/mmap scout blocker update

No blacklist change was made for the latest RV G009 mm/mlock/mmap scout. Four rows (`mincore02`, `mincore04`, `mprotect02`, `mprotect04`) became four-way-clean after LA confirmation and are counted only as future candidates. The remaining RV rows (`mlock02`, `mlock05`, `mlock201`, `mlock202`, `mlock203`, `mlockall02`, `mlockall03`, `munlock02`, `munlockall01`, `mprotect01`, `mprotect03`, `mmap08`, `mmap16`, `mmap18`, `mmap20`) retain visible `TFAIL/TBROK/TCONF` blocker evidence. They are not hidden, not blacklisted for credit, not LA-confirmed, and not counted as stable promotion evidence.

## `statfs01` family setup-device blocker update

No blacklist change was made for `statfs01`, `fstatfs01`, `fstatfs01_64`, or `statvfs01`. The RV scout is an ordinary visible setup blocker: both libcs fail with `TBROK` because LTP cannot acquire a free device. These rows are not hidden, not counted as PASS, and not converted into blacklist credit. Removal condition: generic device acquisition support must make the RV run parser-clean before any LA confirmation or stable promotion accounting.

## VFS-C mknod/rename setup-device blocker update

No blacklist change was made for `mknod07`, `mknodat02`, `rename03`, `rename04`, or `rename05`. The RV scout is an ordinary visible setup blocker: both libcs fail with `TBROK` because LTP cannot acquire a free device. These rows are not hidden, not counted as PASS, and not converted into blacklist credit. Removal condition: generic device acquisition support must make the RV run parser-clean before any LA confirmation or stable promotion accounting.
