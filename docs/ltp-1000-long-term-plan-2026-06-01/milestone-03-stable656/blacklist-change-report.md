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

## LTP device/NAME_MAX clean5 blacklist update

No blacklist change was made for the generic device/NAME_MAX repair.

The earlier `statfs01` family and VFS-C setup-device blocker notes are partially superseded, not hidden: `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` are now four-way parser-clean and are tracked only as future candidates. `mknod07` and `mknodat02` still expose parser-visible `TCONF` because the guest lacks `mkfs.ext2`; `rename03` and `rename04` still expose parser-visible `TFAIL` rename-semantics failures. These rows remain visible blockers, are not blacklisted for credit, and are not counted as stable PASS.

Removal condition for the remaining blockers: implement a generic ext2/device setup path for `mknod07`/`mknodat02` and generic rename semantics for `rename03`/`rename04`, then prove RV + LA x musl+glibc parser-clean summaries plus adjacent regression evidence.


## FD/fcntl clean2 blacklist update

No blacklist change was made for the FD/fcntl scout. `fcntl15` and `fcntl11_64` are tracked only as future candidates after RV/LA confirmation. The remaining fcntl rows keep parser-visible timeout/TCONF/TFAIL/TBROK evidence and are not hidden, blacklisted for credit, or counted as stable PASS.

Removal condition for the remaining blockers: implement generic fcntl/lock/lease/owner or setup support as appropriate, then prove RV + LA x musl+glibc parser-clean summaries plus adjacent FD/fcntl regression evidence.

## Rename03/rename04 blocker closure update

No blacklist change was made for the rename repair. The previous visible `rename03`/`rename04` blockers are closed by generic `axfs::root::rename` source/destination type handling and are now tracked only as future clean candidates after RV + LA x musl+glibc parser-clean proof. The old TFAIL logs remain repair history and are not hidden or counted. `mknod07` and `mknodat02` remain visible non-promotable setup blockers; no blacklist credit is taken.

## Stat/readlink path traversal clean2 blacklist update

No blacklist change was made for the stat/readlink repair. `stat03` and `stat03_64` are tracked only as future candidates after RV + LA x musl+glibc parser-clean proof. The initial RV panic/trap log is retained as repair history and is not counted. `readlink03` remains a visible non-promotable blocker because LA musl reports `TFAIL=1`; it is not hidden, blacklisted for credit, or counted as stable PASS.

Removal condition for `readlink03`: provide a generic LA/musl-compatible zero-size-buffer behavior fix or classification that preserves valid direct `readlink/readlinkat` truncation semantics, then prove RV + LA x musl+glibc parser-clean summaries plus adjacent stat/readlink/open regression evidence.

## mmap20/munlock02 clean2 blacklist update

No blacklist change was made for the mmap/munlock repair. `mmap20` and `munlock02` are now tracked only as future clean candidates after RV + LA x musl+glibc parser-clean proof. The older RV G009 blocker note is partially superseded only for those two rows; the old failures remain repair history and are not hidden or counted.

`mmap08` and `mlock02` remain visible non-promotable blockers. `mmap08` diagnostic-only logs show fd 3 is still a readable temporary file descriptor at mmap time, so the EBADF expectation is not yet closed by generic fd validation. `mlock02` requires real `RLIMIT_MEMLOCK`/capability semantics. Neither row is blacklisted for credit or counted as PASS.


## epoll_create1 clean2 blacklist update

No blacklist change was made for the epoll_create1 repair. `epoll_create1_01` and `epoll_create1_02` are tracked only as future clean candidates after RV + LA x musl+glibc parser-clean proof. `epoll_create02` remains visible blocker evidence: musl's old wrapper maps the invalid legacy-size check onto valid `epoll_create1(0)`, and old-ABI/TCONF history is not hidden or counted. No blacklist/SKIP/status0 credit is taken.

## time/timer clean3 blacklist status

No blacklist was added, removed, or used for promotion accounting in the time/timer clean3 checkpoint. RV-blocked rows remain visible parser blockers and are not counted as PASS.

## lstat clean2 blacklist status

No blacklist was added, removed, or used for promotion accounting in the lstat clean2 checkpoint. RV-blocked VFS/path rows remain visible parser blockers and are not counted as PASS.
