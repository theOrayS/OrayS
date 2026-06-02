# Milestone 03 stable656 promotion candidates

This file records the current candidate pool for the next +50 stable milestone. It is **not** a stable-list update.

## Current four-way clean candidates

Clean evidence set:

- Previous clean28 audit table: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean28-mmap20-munlock02-20260602T054508Z.md`
- Current epoll_create1 clean2 parser report: `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.txt`
- Current clean30 audit table: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean30-epoll-create1-20260602T061430Z.md`
- Current clock/sigaltstack/shmt04 clean4 parser report: `target/ltp-1000-milestone-03-stable656/combined-clock-sigaltstack-shmt04-20260602T143805+0800.promotion-candidates.txt`
- Current clean34 audit table: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean34-clock-sigaltstack-shmt04-20260602T143805Z.md`
- Current shmat04 IPC_STAT ABI parser report: `target/ltp-1000-milestone-03-stable656/combined-shmat04-shmt04-ipcstat-abi-20260602T150918+0800.promotion-candidates.txt`
- Current clean35 audit table: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean35-shmat04-ipcstat-abi-20260602T150918Z.md`
- Required arches: `rv,la`
- Required libcs: `musl,glibc`
- Current four-way-clean not-yet-promoted candidates: 35
- Remaining before stable656 +50 gate: 15

| Case | Evidence | Decision |
| --- | --- | --- |
| `adjtimex01` | current RV/LA clock/sigaltstack/shmt04 targeted gates are parser-clean for musl+glibc after generic `clock_adjtime(CLOCK_REALTIME, ...)` dispatch | keep in candidate pool; not promoted until +50 batch is complete |
| `adjtimex03` | current RV/LA clock/sigaltstack/shmt04 targeted gates are parser-clean for musl+glibc after generic `clock_adjtime(CLOCK_REALTIME, ...)` dispatch | keep in candidate pool; not promoted until +50 batch is complete |
| `epoll_create1_01` | current RV/LA epoll_create1 targeted gates are parser-clean for musl+glibc after generic __NR_epoll_create1 support and FD_CLOEXEC flag handling | keep in candidate pool; not promoted until +50 batch is complete |
| `epoll_create1_02` | current RV/LA epoll_create1 targeted gates are parser-clean for musl+glibc after generic __NR_epoll_create1 support and FD_CLOEXEC flag handling | keep in candidate pool; not promoted until +50 batch is complete |
| `fcntl11_64` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `fcntl15` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `fstatfs01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `fstatfs01_64` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `fsync02` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait03` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `futex_wait05` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `mincore02` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `mincore03` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `mincore04` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `mmap13` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `mmap20` | current RV/LA mmap20+munlock02 targeted gates are parser-clean for musl+glibc after generic mmap flag/fd validation and munlock mapped-range validation | keep in candidate pool; not promoted until +50 batch is complete |
| `mprotect02` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `mprotect04` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `munlock02` | current RV/LA mmap20+munlock02 targeted gates are parser-clean for musl+glibc after generic mmap flag/fd validation and munlock mapped-range validation | keep in candidate pool; not promoted until +50 batch is complete |
| `munmap01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `openat02` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `rename01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `rename03` | generic rename source/destination type handling proof is parser-clean on RV and LA for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `rename04` | generic rename source/destination type handling proof is parser-clean on RV and LA for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `rename05` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `sched_setaffinity01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `shmat04` | current RV/LA shmat04/shmt04 targeted gates are parser-clean for musl+glibc after generic `shmctl(IPC_STAT)` copies the Linux 64-bit `shmid_ds` ABI struct instead of overwriting a guessed 128-byte buffer | keep in candidate pool; not promoted until +50 batch is complete |
| `shmt04` | current RV/LA clock/sigaltstack/shmt04 targeted gates are parser-clean for musl+glibc; existing SysV shm behavior now has four-way evidence | keep in candidate pool; not promoted until +50 batch is complete |
| `signal01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `sigaltstack02` | current RV/LA clock/sigaltstack/shmt04 targeted gates are parser-clean for musl+glibc after generic `sigaltstack` state/errno handling | keep in candidate pool; not promoted until +50 batch is complete |
| `stat03` | stat/readlink path traversal repair proof is parser-clean on RV and LA for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `stat03_64` | stat/readlink path traversal repair proof is parser-clean on RV and LA for musl+glibc | keep in candidate pool; not promoted until +50 batch is complete |
| `statfs01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |
| `statvfs01` | see the parser-clean evidence and historical checkpoint notes below | keep in candidate pool; not promoted until +50 batch is complete |

## Evidence hygiene notes

- The old RV mixed scout log contains a pre-fix `fsync02` glibc `TBROK`; it remains blocker history and is not mixed into the clean current pool.
- `rv-futex-wait01-isolated-standalone-20260601T230253Z.log` provides the clean RV futex row used by the current combined report.
- `rv-fsync02-statfs-clamp-20260601T225748Z.log` and `la-fsync02-statfs-clamp-20260601T225836Z.log` provide the current `fsync02` proof.
- `rv-futex-wait03-proc-sleep-20260601T232011Z.log` and `la-futex-wait03-proc-sleep-20260601T232052Z.log` provide the current `futex_wait03` proof; the older G009 scout timeout remains blocker history only.
- `rv-futex-wait05-periodic-fix-20260601T235234Z.log` and `la-futex-wait05-periodic-fix-20260601T235323Z.log` provide the current `futex_wait05` proof; interrupted/terminated LA regression attempts are retained as non-countable repair history only.
- `rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.log` and `la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.log` provide the current `munmap01` proof. The same LA targeted log keeps `mmap05` blocked with `TFAIL=1` on both libcs, so only `munmap01` enters the clean pool.
- `rv-mmap13-sigbus-final-20260602T012111Z.log` and `la-mmap13-sigbus-final-20260602T012141Z.log` provide the current `mmap13` proof; adjacent mmap/signal regression summaries are `rv-mmap13-sigbus-regression-20260602T011329Z.summary.txt` and `la-mmap13-sigbus-regression-20260602T011433Z.summary.txt`.
- `rv-openat02-sparse-largefile-20260602T014202Z.log` and `la-openat02-sparse-largefile-20260602T014245Z.log` provide the current `openat02` proof; adjacent VFS/FD regression summaries are `rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.summary.txt` and `la-openat02-adjacent-stable-clean-regression-20260602T014545Z.summary.txt`. The earlier `rv-openat02-post-statfs-scout-20260601T231156Z.log` remains pre-fix blocker history only.
- `rv-signal01-poll-wait-20260602T024843Z.log` and `la-signal01-poll-wait-20260602T024926Z.log` provide the current `signal01` proof; adjacent signal/poll/proc regression summaries are `rv-signal-poll-regression-20260602T025025Z.summary.txt` and `la-signal-poll-regression-20260602T025204Z.summary.txt`. The interrupted `rv-signal01-proc-sleep-20260602T024336Z.log` is obsolete non-countable repair history only.
- `rv-mincore03-mincore-mlock-20260602T032124Z.log` and `la-mincore03-mincore-mlock-20260602T032208Z.log` provide the current `mincore03` proof; adjacent mincore/mlock/mmap regression summaries are `rv-mincore03-adjacent-regression-20260602T032259Z.summary.txt` and `la-mincore03-adjacent-regression-20260602T032401Z.summary.txt`. The earlier mixed scout `mincore03` `TBROK` rows are pre-fix blocker history only.
- `rv-g009-mm-mlock-mmap-scout-20260602T034405Z.log` and `la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.log` provide the current `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` proof. The same RV scout keeps the surrounding mlock/mmap/mprotect failures visible as blocker evidence; only the four parser-clean rows enter the pool.

- `rv-rename01-inode-confirm-20260602T044855Z.log` and `la-rename01-inode-confirm-20260602T044855Z.log` provide the current `rename01` proof after generic rename metadata/inode migration. The two-case `rv-rename-inode-retarget-20260602T044708Z.log` / `la-rename-inode-retarget-20260602T044751Z.log` also protect existing `rename05`; the singleton logs are used for the clean22 combined report to avoid duplicate `rename05` rows.

- `rv-mmap20-munlock02-targeted-20260602T054424Z.log` and `la-mmap20-munlock02-targeted-20260602T054508Z.log` provide the current `mmap20`/`munlock02` proof. The incremental parser report `mmap20-munlock02-clean2-20260602T054508Z.promotion-candidates.txt` has 2 candidates and 0 blocked rows. The earlier mixed `rv-mmap-munlock-errno-targeted-20260602T053636Z.log` and debug-only `rv-mmap08-debug-*` logs are repair history only and are not counted.

## Blocked / incomplete rows outside the clean pool

`readlinkat02` is RV-clean and LA-glibc-clean but LA musl still has `TFAIL`, so it is not eligible. The current root-cause audit treats it as a libc/test boundary: musl converts user `bufsize == 0` into a one-byte dummy syscall, and a kernel-side `bufsiz=1` special case would break valid Linux truncation semantics. `clone04` is RV glibc-clean but RV musl is killed by SIGSEGV/TBROK; the singleton log points to a musl `clone.c` wrapper boundary, so it stays outside the clean pool. `mmap05` remains blocked on LA musl+glibc `TFAIL` even though RV is clean. `nice05`, `shmat1`, `mlock02`, `mlock05`, `mlock201`, `mlock202`, `mlock203`, `mlockall02`, `mlockall03`, `munlockall01`, `mprotect01`, `mprotect03`, `mmap08`, `mmap16`, `mmap18`, `atof01`, `fptest01`, `fptest02`, `epoll_create02`, `diotest4`, `select02`, and `execve05` remain blocked or incomplete for the reasons in `validation.md` and the historical combined/scout reports. The pre-fix `fsync02` `TBROK` row is superseded by post-fix proof, but the old log remains documented as failed evidence.

## Closed arch-sweep mining result

Closed sweep artifact:

- `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`

Result: the report contains 563 historical four-way-clean candidates overall, but the live-stable606 filter file is empty. No additional not-yet-stable four-way-clean case can be promoted from these closed logs.

## Stable-list decision

Do not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES` yet. The live baseline remains `606 total / 606 unique / 0 duplicate`; this milestone target is `656`, so a milestone commit that promotes stable cases requires 50 trustworthy unique candidates, not 35.

## `openat03` non-candidate note

`openat03` is explicitly outside the clean pool. A larger `O_TMPFILE`/`linkat` emulation attempt was rejected after RV targeted validation produced a supervisor page fault during the testcase's deep nested-directory phase (`rv-openat03-otmpfile-20260602T021349Z.summary.txt` and `rv-openat03-trace-20260602T022058Z.summary.txt`, both `panic/trap matches: 1`).

The retained generic change only makes unsupported `O_TMPFILE` fail honestly: `O_TMPFILE|O_RDONLY` returns `EINVAL`, and `O_TMPFILE` against an existing directory returns `EOPNOTSUPP`. Current RV and LA targeted summaries (`rv-openat03-otmpfile-enotsup-20260602T022658Z.summary.txt`, `la-openat03-otmpfile-enotsup-20260602T022748Z.summary.txt`) show zero timeout/ENOSYS/panic/trap but `TCONF=4` and wrapper FAIL, so this is blocker evidence only and must not be counted toward stable656.

## `epoll_create02` non-candidate note

`epoll_create02` was rechecked as a singleton on RV and LA. It is still outside the clean pool: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt` shows RV musl wrapper FAIL with `TFAIL=2` and `ENOSYS=2`, while `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt` wrapper-PASSes both libcs but retains parser-visible `TCONF=2`. This is blocked evidence only; no blacklist, SKIP, status0, or caveated wrapper PASS row is counted toward stable656.


## G009 mm/mincore/mprotect clean4 update

A follow-up RV scout plus LA confirmation grew the clean pool without editing the stable list:

- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.txt` — 8 wrapper PASS / 30 wrapper FAIL, with `TFAIL=50`, `TBROK=4`, `TCONF=6`, and no timeout/ENOSYS/panic/trap.
- LA confirmation summary: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.txt` — 8 wrapper PASS / 0 wrapper FAIL, zero internal markers, timeout, ENOSYS, panic/trap.
- Combined clean14 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean14-g009-mm-mprotect-20260602T034707Z.promotion-candidates.txt`.

Newly evidenced four-way-clean cases: `mincore02`, `mincore04`, `mprotect02`, and `mprotect04`. Pool at that checkpoint: 14/50 before the later clean5 update. Stable list remains `606 total / 606 unique / 0 duplicate`.

## `statfs01` family non-candidate note

The 2026-06-02 RV scout for `statfs01`, `fstatfs01`, `fstatfs01_64`, and `statvfs01` is outside the clean pool:

- Summary: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.promotion-candidates.txt`
- Result: 0 wrapper PASS / 8 wrapper FAIL, `TBROK=8`, no timeout/ENOSYS/panic/trap.

The raw log shows LTP setup failing to acquire a free device for all four cases. Because RV is parser-unclean and LA was not run, these rows are blocker evidence only and did not affect the 14-case candidate pool at that checkpoint; the later clean5 update below supersedes the setup-blocker classification for `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` only.

## VFS-C mknod/rename non-candidate note

The 2026-06-02 RV scout for `mknod07`, `mknodat02`, `rename03`, `rename04`, and `rename05` is outside the clean pool:

- Summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.promotion-candidates.txt`
- Result: 0 wrapper PASS / 10 wrapper FAIL, `TBROK=14`, no timeout/ENOSYS/panic/trap.

The raw log shows LTP setup failing to acquire a free device for all five cases. Because RV is parser-unclean and LA was not run, these rows are blocker evidence only and did not affect the 14-case candidate pool at that checkpoint; the later clean5 update below supersedes the setup-blocker classification for `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` only.

## LTP device/NAME_MAX clean5 update

A generic device setup follow-up grew the clean pool without editing the stable list:

- RV enumeration-only retest summary: `target/ltp-1000-milestone-03-stable656/rv-device-enumeration-retest-20260602T041227Z.summary.txt` — still 0 PASS / 18 FAIL, `TBROK=22`; `/dev` enumeration alone was insufficient.
- RV `LTP_DEV=/dev/vda` pre-NAME_MAX retest summary: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-vda-device-retest-20260602T041431Z.summary.txt` — 3 PASS before `statvfs01` hit a parser-visible panic/trap; this is repair history only.
- RV final device/NAME_MAX retest summary: `target/ltp-1000-milestone-03-stable656/rv-device-cases-ltpdev-namemax-retest-20260602T041654Z.summary.txt` — `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` are parser-clean for musl+glibc; `mknod07`, `mknodat02`, `rename03`, and `rename04` retain visible blockers.
- LA clean5 confirmation summary: `target/ltp-1000-milestone-03-stable656/la-device-clean5-ltpdev-namemax-retest-20260602T041803Z.summary.txt` — the five RV-clean rows are parser-clean for musl+glibc.
- Combined clean19 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean19-ltpdev-namemax-20260602T041803Z.promotion-candidates.txt`.

Newly evidenced four-way-clean cases: `fstatfs01`, `fstatfs01_64`, `rename05`, `statfs01`, and `statvfs01`. Pool at that checkpoint: 19/50 before the later FD/fcntl clean2 update. Stable list remains `606 total / 606 unique / 0 duplicate`.

Blocked rows from the same proof set stay outside the pool: `mknod07` and `mknodat02` are parser-visible `TCONF` because `mkfs.ext2` is missing in the guest; `rename03` and `rename04` are parser-visible `TFAIL` rename-semantic blockers. None is blacklisted or counted as PASS.

## FD/fcntl clean2 scout update

A documentation/evidence-only FD scout grew the clean pool without editing the stable list:

- RV fcntl scout summary: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.txt` — `fcntl15` and `fcntl11_64` are parser-clean for musl+glibc; the surrounding fcntl rows keep visible timeout/TCONF/TFAIL/TBROK blockers.
- LA clean2 confirmation summary: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.txt` — both RV-clean rows are parser-clean for musl+glibc.
- Combined clean21 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`.

Newly evidenced four-way-clean cases: `fcntl11_64` and `fcntl15`. Pool at that checkpoint: 21/50. Stable list remains `606 total / 606 unique / 0 duplicate`.

Blocked rows from the same RV scout stay outside the pool: `fcntl17` timed out on both libcs; `fcntl24`, `fcntl25`, `fcntl26`, and `fcntl37` retain parser-visible `TCONF`; `fcntl27` and `fcntl31` retain parser-visible `TFAIL`; `fcntl34`, `fcntl38`, and `fcntl39` retain parser-visible `TBROK`. None is blacklisted or counted as PASS.

## VFS/path scout and rename01 clean1 update

A VFS/path scout first exposed no immediately promotable clean rows but identified a generic rename metadata bug: `rename01` failed because `st_ino` was derived from the post-rename pathname rather than preserved for the renamed object.

- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-link-statx-scout-20260602T044314Z.summary.txt` — 4 wrapper PASS / 42 wrapper FAIL, with `TFAIL=53`, `TCONF=26`, `ENOSYS=34`, and no timeout/panic/trap. The PASS rows (`statx01`, `getdents02`) still contain parser-visible `TCONF`, so they are not candidates.
- RV rename inode confirmation: `target/ltp-1000-milestone-03-stable656/rv-rename01-inode-confirm-20260602T044855Z.summary.txt` — `rename01` parser-clean for musl+glibc.
- LA rename inode confirmation: `target/ltp-1000-milestone-03-stable656/la-rename01-inode-confirm-20260602T044855Z.summary.txt` — `rename01` parser-clean for musl+glibc.
- Regression proof: `rv-rename-inode-retarget-20260602T044708Z.summary.txt` and `la-rename-inode-retarget-20260602T044751Z.summary.txt` both run `rename01,rename05` and are parser-clean for musl+glibc.
- Combined clean22 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean22-rename01-inode-20260602T044855Z.promotion-candidates.txt`.

Newly evidenced four-way-clean case: `rename01`. Pool at that checkpoint: 22/50. Stable list remains `606 total / 606 unique / 0 duplicate`.

Blocked rows from the scout stay outside the pool: `link02`, `link04`, and `link05` retain generic `ENOSYS`/hard-link blockers; `linkat01`, `linkat02`, `renameat01`, `statx04`, `statx05`, `writev03`, `getdents01`, `readlink03`, `stat03`, and `stat03_64` retain visible TCONF/TFAIL/setup or semantic blockers; missing guest testcase binaries (`link01`, `link03`, `rename02`, `renameat02`, `unlink01`, `chmod02`, `readlink02`) are not evidence. None is blacklisted or counted as PASS.

## Rename03/rename04 clean2 update

A generic `axfs::root::rename` semantics repair grew the future pool without editing the stable list. The repair is not LTP-specific: destination replacement now checks source and destination object types before removal, allows directory-over-empty-directory replacement, preserves file-over-directory `EISDIR`, returns directory-over-file `ENOTDIR`, and treats an identical old/new path as success.

Evidence:

- RV rename targeted summary: `target/ltp-1000-milestone-03-stable656/rv-rename-dir-overwrite-20260602T050256Z.summary.txt` — `rename01`, `rename03`, `rename04`, and `rename05` are parser-clean for musl+glibc; zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA rename targeted summary: `target/ltp-1000-milestone-03-stable656/la-rename-dir-overwrite-20260602T050346Z.summary.txt` — the same four rows are parser-clean for musl+glibc; zero internal markers or fatal signatures.
- RV clean-only statfs/rename05 retarget summary: `target/ltp-1000-milestone-03-stable656/rv-statfs-rename05-clean-retarget-20260602T050521Z.summary.txt` — regenerates clean `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` evidence without mixing old `rename03/rename04` failures into the combined parser report.
- Combined clean24 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean24-rename03-04-20260602T050630Z.promotion-candidates.txt`.
- Checksums: `target/ltp-1000-milestone-03-stable656/rename03-04-clean24-20260602T050630Z.derived.sha256`.

Newly evidenced four-way-clean cases: `rename03` and `rename04`. Pool at that checkpoint: 24/50. Stable list remains `606 total / 606 unique / 0 duplicate` because the stable656 +50 gate has not been reached.

The earlier VFS-C setup-device/TFAIL notes are superseded only for `rename03` and `rename04`; they remain historical repair context and are not counted as promotion evidence. `mknod07` and `mknodat02` remain outside the pool because they still require a generic ext2/device setup path.

## Stat/readlink path traversal clean2 update

A generic stat/readlink path traversal repair grew the future pool without editing the stable list:

- First RV repair-history summary: `target/ltp-1000-milestone-03-stable656/rv-readlink-stat-path-20260602T051956Z.summary.txt` — not countable because `stat03` hit a parser-visible panic/trap before the nonrecursive parent-search fix.
- Fixed RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-readlink-stat-path-nonrecursive-20260602T052206Z.summary.txt` — `readlink03`, `stat03`, and `stat03_64` are parser-clean for musl+glibc.
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-readlink-stat-path-nonrecursive-20260602T052251Z.summary.txt` — `stat03` and `stat03_64` are parser-clean for musl+glibc; `readlink03` remains blocked by LA musl `TFAIL=1`.
- Regression summaries: `rv-stat-readlink-stable-regression-20260602T052501Z.summary.txt` and `la-stat-readlink-stable-regression-20260602T052706Z.summary.txt` — adjacent stable stat/lstat/fstatat/readlink/openat/rename rows are parser-clean on both arches.
- Combined clean26 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean26-stat03-path-20260602T052251Z.promotion-candidates.txt`.

Newly evidenced four-way-clean cases: `stat03` and `stat03_64`. Pool at that checkpoint: 26/50. Stable list remains `606 total / 606 unique / 0 duplicate` because the stable656 +50 gate has not been reached.

`readlink03` stays outside the pool: RV musl/glibc and LA glibc are clean, but LA musl still has parser-visible `TFAIL=1` on the zero-size-buffer case. This row is neither hidden nor blacklisted, and it is not counted as PASS.

## mmap20/munlock02 clean2 update

A generic mmap/munlock errno repair grew the future pool without editing the stable list:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-mmap20-munlock02-targeted-20260602T054424Z.summary.txt` — `mmap20` and `munlock02` are parser-clean for musl+glibc.
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-mmap20-munlock02-targeted-20260602T054508Z.summary.txt` — `mmap20` and `munlock02` are parser-clean for musl+glibc.
- Incremental clean2 parser report: `target/ltp-1000-milestone-03-stable656/mmap20-munlock02-clean2-20260602T054508Z.promotion-candidates.txt` — 2 promotion candidates, 0 blocked/incomplete rows.
- Combined milestone audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean28-mmap20-munlock02-20260602T054508Z.md`.

Newly evidenced four-way-clean cases: `mmap20` and `munlock02`. Pool at that checkpoint: 28/50. Stable list remains `606 total / 606 unique / 0 duplicate` because the stable656 +50 gate has not been reached.

`mmap08` stays outside the pool: diagnostic-only runs show the tested fd is still a readable temporary file descriptor at mmap time, so the expected EBADF path is not yet proven by the generic fd validation. `mlock02` also stays outside the pool because real `RLIMIT_MEMLOCK`/capability semantics are still absent.


## epoll_create1 clean2 update

A generic `__NR_epoll_create1` dispatch and `EPOLL_CLOEXEC`/unknown-flag validation grew the future pool without editing the stable list.

Evidence:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt` — `epoll_create1_01` and `epoll_create1_02` are parser-clean for musl+glibc; zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt` — same parser-clean result for musl+glibc.
- Incremental clean2 report: `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.txt`.
- RV FD/flags regression summary: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.txt` — 12 PASS / 0 FAIL, parser zero internal/fatal markers.
- LA FD/flags regression summary: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.txt` — 12 PASS / 0 FAIL, parser zero internal/fatal markers.
- Combined milestone audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean30-epoll-create1-20260602T061430Z.md`.

Newly evidenced four-way-clean cases: `epoll_create1_01` and `epoll_create1_02`. Current pool: 30/50. Stable list remains `606 total / 606 unique / 0 duplicate` because the stable656 +50 gate has not been reached.

`epoll_create02` remains outside the pool: glibc/axlibc `epoll_create(size)` now rejects `size <= 0`, but musl reaches the kernel as valid `epoll_create1(0)` and therefore cannot prove the invalid old-size argument at the syscall boundary. The row stays visible as blocker evidence only.

## clock_adjtime/sigaltstack/shmt04 clean4 update

Generic `clock_adjtime` dispatch, per-thread `sigaltstack` syscall-state handling, and four-way `shmt04` confirmation grew the future pool without editing the stable list.

Evidence:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt` — `adjtimex01`, `adjtimex03`, `shmt04`, and `sigaltstack02` are parser-clean for musl+glibc; zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt` — same parser-clean result for musl+glibc.
- Incremental clean4 report: `target/ltp-1000-milestone-03-stable656/combined-clock-sigaltstack-shmt04-20260602T143805+0800.promotion-candidates.txt`.
- RV time/signal regression summary: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.summary.txt` — 14 PASS / 0 FAIL, parser zero internal/fatal markers.
- LA time/signal regression summary: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.summary.txt` — 14 PASS / 0 FAIL, parser zero internal/fatal markers.
- Combined milestone audit: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean34-clock-sigaltstack-shmt04-20260602T143805Z.md`.

Newly evidenced four-way-clean cases: `adjtimex01`, `adjtimex03`, `shmt04`, and `sigaltstack02`. Current pool: 34/50. Stable list remains `606 total / 606 unique / 0 duplicate` because the stable656 +50 gate has not been reached.

`sigaltstack` support in this checkpoint is syscall-state/errno behavior only; full alternate-stack signal delivery remains a future signal-lane boundary and is not claimed by this candidate proof.


## SysV shm IPC_STAT ABI clean1 update

A generic `shmctl(IPC_STAT)` user ABI repair added one future candidate without editing the stable list:

- RV targeted summary: `target/ltp-1000-milestone-03-stable656/rv-shmat04-shmt04-ipcstat-abi-20260602T150702+0800.summary.txt` — 4 PASS / 0 FAIL, zero internal markers/fatal signatures.
- LA targeted summary: `target/ltp-1000-milestone-03-stable656/la-shmat04-shmt04-ipcstat-abi-20260602T150805+0800.summary.txt` — 4 PASS / 0 FAIL, zero internal markers/fatal signatures.
- Combined clean35 report: `target/ltp-1000-milestone-03-stable656/combined-shmat04-shmt04-ipcstat-abi-20260602T150918+0800.promotion-candidates.txt`.
- Audit table: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/combined-candidate-pool-clean35-shmat04-ipcstat-abi-20260602T150918Z.md`.

Newly evidenced four-way-clean case: `shmat04`. `shmt04` was revalidated in the same gate but was already counted in clean34. Current candidate pool: 35/50; stable list remains `606 total / 606 unique / 0 duplicate`.
