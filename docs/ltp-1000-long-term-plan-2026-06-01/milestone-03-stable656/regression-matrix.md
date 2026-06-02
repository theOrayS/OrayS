# Milestone 03 stable656 regression matrix

This checkpoint records completed regression evidence for the `sched_setaffinity01` fix and the `generic_statfs` capacity clamp, plus required future regression sets for unresolved G009 blockers.

## Completed regression for `sched_setaffinity01`

Changed syscall: `sched_setaffinity` permission path in `examples/shell/src/uspace/resource_sched.rs`.

Targeted promotion proof:

- RV: `target/ltp-1000-milestone-03-stable656/rv-sched-setaffinity01-postfix-20260601T222738Z.summary.txt`
- LA: `target/ltp-1000-milestone-03-stable656/la-sched-setaffinity01-postfix-20260601T222823Z.summary.txt`

Adjacent stable regression subset:

- `sched_getaffinity01`
- `sched_setparam01`
- `sched_setparam02`
- `sched_setparam03`
- `sched_setparam04`
- `sched_setparam05`
- `sched_setscheduler01`
- `sched_setscheduler02`
- `sched_setscheduler03`
- `setpriority02`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-sched-affinity-regression-20260601T222920Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-sched-affinity-regression-20260601T223023Z.summary.txt`
- Result: 20/20 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

## Completed regression for `generic_statfs` capacity clamp

Changed surface: shared synthetic filesystem capacity reporting in `examples/shell/src/uspace/metadata.rs::generic_statfs`, observed through `statfs`, `fstatfs`, and `statvfs`.

Targeted promotion proof:

- RV `fsync02`: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.summary.txt`
- LA `fsync02`: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.summary.txt`

Adjacent stable regression subset:

- `statfs02`
- `fstatfs02`
- `fstatfs02_64`
- `statfs02_64`
- `statfs03`
- `statfs03_64`
- `statvfs02`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.summary.txt`
- Result: 14/14 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.


## Completed regression for precise timer-list wakeups

Changed surfaces: `kernel/task/axtask/src/timers.rs` timed wakeup programming and `kernel/runtime/axruntime/src/lib.rs` periodic timer deadline preservation.

Targeted promotion proof:

- RV `futex_wait05`: `target/ltp-1000-milestone-03-stable656/rv-futex-wait05-periodic-fix-20260601T235234Z.summary.txt`
- LA `futex_wait05`: `target/ltp-1000-milestone-03-stable656/la-futex-wait05-periodic-fix-20260601T235323Z.summary.txt`

Adjacent regression subset:

- `futex_wait01`
- `futex_wait02`
- `futex_wait03`
- `futex_wait04`
- `futex_wait05`
- `futex_wake01`
- `proc01`
- `waitpid04`
- `nanosleep01`
- `clock_nanosleep02`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-timer-futex-regression-periodic-fix-20260601T235036Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-timer-futex-regression-periodic-fix-20260601T234827Z.summary.txt`
- Result: 20/20 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Caveat: two earlier LA attempts (`la-timer-futex-regression-20260601T234109Z` and `la-timer-futex-regression-20260601T234340Z`) were terminated and are not counted; the second exposed the periodic-deadline drift fixed here.


## Completed regression for catchable synchronous `SIGSEGV` delivery

Changed surfaces: `examples/shell/src/uspace/memory_map.rs` user page-fault handling and `examples/shell/src/uspace/signal_abi.rs` synchronous signal queueing helper.

Targeted proof:

- RV `mmap05,munmap01`: `target/ltp-1000-milestone-03-stable656/rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.summary.txt`
- LA `mmap05,munmap01`: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.summary.txt`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean6-sync-sigsegv-20260602T003243Z.promotion-candidates.txt`

Result: `munmap01` is four-way parser-clean and joins the not-yet-promoted candidate pool. `mmap05` is RV-clean but remains blocked because LA musl and LA glibc both report `TFAIL=1` / SIGSEGV signal not received.

Adjacent stable regression subset:

- `mmap01`
- `mmap02`
- `mmap03`
- `mmap04`
- `mmap09`
- `mmap12`
- `signal03`
- `sigaction01`
- `rt_sigaction01`
- `rt_sigprocmask01`
- `sigprocmask01`
- `waitpid04`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-sync-sigsegv-regression-20260602T002800Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-sync-sigsegv-regression-20260602T003046Z.summary.txt`
- Result: 24/24 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

## If fixing recoverable user SIGSEGV / page-fault signal delivery

Primary retest cases:

- `mmap05`
- `munmap01` (now four-way clean; keep here only for future regressions around this signal path)

Adjacent stable/regression candidates:

- Existing stable mmap/page cases from stable606: `mmap01`, `mmap02`, `mmap03`, `mmap04`, `mmap09`, `mmap12`, `mmap16`, `mmap18`, `mmap19`, `mmap20`, `mmap-corruption01`, `dirty`
- Signal-adjacent stable cases: `sigaction01`, `signal01`, `signal03`, `sigprocmask01`, `rt_sigaction01`, `rt_sigprocmask01` if present in live stable list
- Process teardown sanity: a small stable subset that exercises faulting child exit, wait, and cleanup

## If fixing file-backed mmap SIGBUS-on-EOF

Primary retest case:

- `mmap13`

Adjacent regression candidates:

- file-backed mmap/read/write stable subset
- VFS metadata and truncation-adjacent cases
- signal delivery sanity for `SIGBUS` and `SIGSEGV` distinction

## If fixing futex wait timeout/wakeup semantics

Primary retest cases:

- `futex_wait03`
- `futex_wait05`

Closed precise-timer regression after this repair is listed above. Additional changes in this lane should rerun at least the same subset.

Adjacent regression candidates:

- `futex_wait01` (now four-way clean candidate)
- current stable futex rows, if any, from live `LTP_STABLE_CASES`
- timeout/EINTR-related wait and signal-mask cases
- task teardown/wakeup lifetime smoke tests

Closed futex/proc regression after procfs sleeping-state repair:

- Cases: `futex_wait02`, `futex_wait04`, `futex_wake01`, `proc01`, `waitpid04`.
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-futex-proc-regression-20260601T232144Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-futex-proc-regression-20260601T232232Z.summary.txt`
- Result: 10/10 wrapper PASS on each arch, zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

## If fixing SysV shm / resource lifetime

Primary retest case:

- `shmat1`

Adjacent regression candidates:

- current stable SysV shm/IPC rows from live `LTP_STABLE_CASES`
- process exit and address-space teardown cases
- resource telemetry checks on RV and LA

## If fixing `clone04` / clone wrapper NULL-stack handling

Primary retest case:

- `clone04`

Adjacent regression candidates:

- clone syscall family: `clone01`, `clone03`, `clone06`, `clone07` if present in guest inventory
- process creation/teardown: `fork01`, `vfork01`, `vfork02`, `waitpid04`, `waitid01`
- thread teardown and TLS-adjacent smoke: `set_tid_address01`, futex wait/wake cases
- signal/wait delivery sanity for children killed by invalid clone usage

Boundary: the current evidence is RV glibc-clean but RV musl TBROK/SIGSEGV, with an LTP hint toward a musl `clone.c` wrapper fix. Do not promote until RV musl is parser-clean and the adjacent clone/process/futex subset remains clean on RV and LA.

## Non-promotion rows

- `mmap10_1`: do not include until the guest LTP inventory contains the binary.
- `vma02`: do not include until libnuma-related `TCONF` is resolved and both libcs are parser-clean.
- `readlinkat02`: do not include while LA musl remains parser-unclean; current audit shows musl rewrites user `bufsize == 0` into a one-byte syscall, so any future change must preserve direct `readlinkat(..., bufsiz=1)` truncation semantics and include readlink/readlinkat plus user-pointer boundary regressions.
- `nice04`: do not include until the `nice()` wrapper errno boundary is fixed without regressing `setpriority02`; use `nice04-errno-boundary-report.md` as the handoff.
- `clone04`: do not include while RV musl is killed by SIGSEGV/TBROK; classify the musl wrapper boundary before changing kernel clone semantics.
- closed arch sweep: no extra stable606-missing four-way-clean rows remain; use the matrices only for blocker prioritization.
- `kill10`: do not include broad batches until the isolated timeout + post-cleanup frame leak + following glibc allocator panic is fixed; poll/exit cleanup was tested and rejected as insufficient.

## Completed regression for file-backed mmap `SIGBUS` beyond EOF

Changed surfaces: `examples/shell/src/uspace/memory_map.rs` file-backed mmap population/page-fault signal selection plus `examples/shell/src/uspace/process_lifecycle.rs` and `examples/shell/src/uspace/mod.rs` mmap beyond-EOF range lifetime tracking.

Targeted proof:

- RV `mmap13`: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-final-20260602T012111Z.summary.txt`
- LA `mmap13`: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-final-20260602T012141Z.summary.txt`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean7-mmap13-sigbus-final-20260602T012225Z.promotion-candidates.txt`

Result: `mmap13` is four-way parser-clean and joins the not-yet-promoted candidate pool. The older RV `mmap13` log remains pre-fix blocker evidence, and the TTY-aborted RV rerun is not counted.

Adjacent stable regression subset:

- `mmap01`
- `mmap02`
- `mmap03`
- `mmap04`
- `mmap09`
- `mmap12`
- `signal03`
- `sigaction01`
- `rt_sigaction01`
- `rt_sigprocmask01`
- `sigprocmask01`
- `waitpid04`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-regression-20260602T011329Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-regression-20260602T011433Z.summary.txt`
- Result: 24/24 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.


## Completed regression for sparse large-file regular-file handling

Changed surfaces: `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/metadata.rs`, `examples/shell/src/uspace/memory_map.rs`, `examples/shell/src/uspace/process_lifecycle.rs`, and `examples/shell/src/uspace/mod.rs` sparse logical-size/data tracking.

Targeted proof:

- RV `openat02`: `target/ltp-1000-milestone-03-stable656/rv-openat02-sparse-largefile-20260602T014202Z.summary.txt`
- LA `openat02`: `target/ltp-1000-milestone-03-stable656/la-openat02-sparse-largefile-20260602T014245Z.summary.txt`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean8-openat02-sparse-largefile-20260602T014245Z.promotion-candidates.txt`

Result: `openat02` is four-way parser-clean and joins the not-yet-promoted candidate pool. The older RV `openat02` post-statfs-clamp scout remains pre-fix blocker evidence and is not counted.

Adjacent stable clean regression subset:

- `openat01`
- `lseek01`
- `lseek02`
- `pread02`
- `pwrite02`
- `pwrite04`
- `ftruncate01`
- `truncate02`
- `read01`
- `write01`
- `write03`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-openat02-adjacent-stable-clean-regression-20260602T014545Z.summary.txt`
- Result: 22/22 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.
- Caveat: `rv-openat02-adjacent-stable-regression-20260602T014338Z.summary.txt` additionally included `read02`, which wrapper-PASSed but emitted existing O_DIRECT `TCONF`; that 12-case shard is not counted as parser-clean evidence.

## `openat03` O_TMPFILE unsupported-gate blocker

Changed surface: `examples/shell/src/uspace/fd_table.rs` `open_candidates` handling for `O_TMPFILE` flags.

Rejected implementation evidence:

- RV panic summary: `target/ltp-1000-milestone-03-stable656/rv-openat03-otmpfile-20260602T021349Z.summary.txt`
- RV trace panic summary: `target/ltp-1000-milestone-03-stable656/rv-openat03-trace-20260602T022058Z.summary.txt`
- Result: both runs have `panic/trap matches: 1`; the implementation was removed and is not promotion evidence.

Retained unsupported-gate targeted evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-openat03-otmpfile-enotsup-20260602T022658Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-openat03-otmpfile-enotsup-20260602T022748Z.summary.txt`
- Result: both arches run musl+glibc to deterministic `TCONF=4` / wrapper FAIL with zero timeout, ENOSYS, panic, or trap.

Regression decision: no stable regression subset is counted from this blocker because `openat03` remains unsupported and non-promotable. Before revisiting, require a design for real generic `O_TMPFILE`/link materialization plus adjacent VFS cases around open/openat/link/linkat/unlink/rename/deep directory handling on RV and LA.

## Completed regression for signal/poll proc-state reporting

Changed surfaces: `examples/shell/src/uspace/task_context.rs` wait markers, `examples/shell/src/uspace/signal_abi.rs::sys_rt_sigsuspend`, `examples/shell/src/uspace/select_fdset.rs::sys_poll_until`, and `examples/shell/src/uspace/synthetic_fs.rs` `/proc/<pid>/stat` state selection.

Targeted proof:

- RV `signal01`: `target/ltp-1000-milestone-03-stable656/rv-signal01-poll-wait-20260602T024843Z.summary.txt`
- LA `signal01`: `target/ltp-1000-milestone-03-stable656/la-signal01-poll-wait-20260602T024926Z.summary.txt`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean9-signal01-poll-wait-20260602T025432Z.promotion-candidates.txt`

Result: `signal01` is four-way parser-clean and joins the not-yet-promoted candidate pool. The earlier RV `rv-signal01-proc-sleep-20260602T024336Z.summary.txt` run still timed out before the poll-wait marker was added and remains repair history only.

Adjacent stable regression subset:

- `signal02`
- `signal03`
- `signal04`
- `signal05`
- `sigaction01`
- `rt_sigaction01`
- `sigprocmask01`
- `rt_sigprocmask01`
- `ppoll01`
- `pselect01`
- `poll02`
- `waitpid04`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-signal-poll-regression-20260602T025025Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-signal-poll-regression-20260602T025204Z.summary.txt`
- Result: 24/24 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.


## `kill10` resource-lifetime regression boundary

Current blocker evidence protects against treating case-local timeout cleanup as harmless. Any future `kill10` or process-group signal cleanup patch must rerun an isolated RV singleton first and prove all of the following before broad process/signal shards resume: no musl timeout, no persistent post-cleanup frame leak comparable to `-129185` frames, no allocator panic in the following glibc group, and parser-clean summaries for both libcs. Only after that should LA confirmation and adjacent process/signal regressions run.


## Completed regression for mincore lazy-VMA residency and mlock prefault

Changed surfaces: `examples/shell/src/uspace/memory_map.rs` `sys_mincore` and `sys_mlock`, plus `examples/shell/src/uspace/syscall_dispatch.rs` mlock/mlock2 dispatch.

Targeted proof:

- RV `mincore03`: `target/ltp-1000-milestone-03-stable656/rv-mincore03-mincore-mlock-20260602T032124Z.summary.txt`
- LA `mincore03`: `target/ltp-1000-milestone-03-stable656/la-mincore03-mincore-mlock-20260602T032208Z.summary.txt`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean10-mincore03-mincore-mlock-20260602T032401Z.promotion-candidates.txt`

Result: `mincore03` is four-way parser-clean and joins the not-yet-promoted candidate pool. The older mixed scout `mincore03` rows remain pre-fix `TBROK` history and are not counted.

Adjacent stable regression subset:

- `mincore01`
- `mlock01`
- `mlock03`
- `mlock04`
- `munlock01`
- `mlockall01`
- `mmap01`
- `mmap02`
- `mmap03`
- `mmap04`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mincore03-adjacent-regression-20260602T032259Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mincore03-adjacent-regression-20260602T032401Z.summary.txt`
- Result: 20/20 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

## `epoll_create02` blocker regression boundary

Current singleton evidence:

- RV: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt` — RV musl FAIL with `TFAIL=2` / `ENOSYS=2`, RV glibc PASS with `TCONF=1`.
- LA: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt` — both libcs wrapper PASS but with `TCONF=2`.

No regression subset is counted because no source change was retained and the case is not parser-clean. Any future generic epoll compatibility patch must first make `epoll_create02` parser-clean on RV and LA for musl+glibc, then run adjacent FD/epoll regressions covering at least invalid-size `epoll_create`, valid `epoll_create1` fd lifetime, `close`, `fcntl(F_GETFD/F_SETFD)`, `epoll_ctl`, and `epoll_wait` readiness/error paths.


## G009 mm/mlock/mmap scout and clean4 confirmation boundary

No new source change was retained for this evidence-only checkpoint, so no adjacent stable regression subset is counted here. The proof closes four additional candidate rows and keeps surrounding blockers visible:

- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.txt` — `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` are parser-clean for musl+glibc; the remaining mlock/mmap/mprotect rows retain `TFAIL/TBROK/TCONF` blockers.
- LA confirmation summary: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.txt` — the four clean rows are parser-clean for musl+glibc.
- Combined clean14 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean14-g009-mm-mprotect-20260602T034707Z.promotion-candidates.txt`.

Result: `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` join the not-yet-promoted candidate pool. Future fixes for the blocked mlock/mmap/mprotect rows must first make RV parser-clean, then rerun LA confirmation plus adjacent mincore/mlock/munlock/mprotect/mmap regression subsets before any promotion accounting.

## `statfs01` family setup-device regression boundary

No source change was retained for the RV `statfs01,fstatfs01,fstatfs01_64,statvfs01` scout, so no adjacent regression subset is counted here.

Current blocker evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt` — 0 wrapper PASS / 8 wrapper FAIL, `TBROK=8`, zero timeout/ENOSYS/panic/trap.
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.promotion-candidates.txt` — 0 candidates / 4 blocked.

Future generic device/free-block-device support must first make this RV setup parser-clean. Then rerun LA confirmation plus adjacent statfs/fstatfs/statvfs and device/mount setup regressions before any stable promotion accounting.

## VFS-C mknod/rename setup-device regression boundary

No source change was retained for the RV `mknod07,mknodat02,rename03,rename04,rename05` scout, so no adjacent regression subset is counted here.

Current blocker evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt` — 0 wrapper PASS / 10 wrapper FAIL, `TBROK=14`, zero timeout/ENOSYS/panic/trap.
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.promotion-candidates.txt` — 0 candidates / 5 blocked.

Future generic device/free-block-device support must first make this RV setup parser-clean. Then rerun LA confirmation plus adjacent mknod/mknodat/rename and device/mount setup regressions before any stable promotion accounting.

## Completed regression for global LTP device exposure and filesystem NAME_MAX reporting

Changed surfaces: `examples/shell/src/cmd.rs` LTP environment setup, `examples/shell/src/uspace/fd_table.rs` `/dev` enumeration/name limit and synthetic block-device stat routing, `examples/shell/src/uspace/metadata.rs` synthetic block rdev/stat metadata, and `examples/shell/src/uspace/linux_abi.rs` `statfs` name length.

Targeted proof:

- RV 9-case retest: `target/ltp-1000-milestone-03-stable656/rv-device-cases-ltpdev-namemax-retest-20260602T041654Z.summary.txt`
- LA clean5 confirmation: `target/ltp-1000-milestone-03-stable656/la-device-clean5-ltpdev-namemax-retest-20260602T041803Z.summary.txt`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean19-ltpdev-namemax-20260602T041803Z.promotion-candidates.txt`

Result: `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` are four-way parser-clean and join the not-yet-promoted candidate pool. The older enumeration-only run remains setup-blocker history, and the pre-NAME_MAX run remains panic/trap repair history.

Adjacent stable regression subset:

- `chdir01`
- `pathconf01`
- `fpathconf01`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-namemax-regression-subset-20260602T041926Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-ltpdev-namemax-regression-subset-20260602T042012Z.summary.txt`
- Result: 6/6 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Remaining regression boundary: `mknod07` and `mknodat02` are not regression-clean promotion rows because `mkfs.ext2` is absent and parser-visible `TCONF` remains; `rename03` and `rename04` are real semantic `TFAIL` blockers. Future fixes must rerun the same device/statfs/mknod/rename set plus this regression subset on RV and LA.


## FD/fcntl scout and clean2 confirmation boundary

No source change was retained for this evidence-only checkpoint, so no adjacent stable regression subset is counted here. The proof closes two candidate rows and keeps surrounding blockers visible:

- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.txt` — `fcntl15` and `fcntl11_64` are parser-clean for musl+glibc; the remaining fcntl rows retain timeout/TCONF/TFAIL/TBROK blockers.
- LA confirmation summary: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.txt` — the two clean rows are parser-clean for musl+glibc.
- Combined clean21 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`.

Result: `fcntl15` and `fcntl11_64` join the not-yet-promoted candidate pool. Future fixes for blocked fcntl rows must first make RV parser-clean, then rerun LA confirmation plus adjacent fcntl/FD/lock regression subsets before any promotion accounting.

## Completed regression for rename metadata/inode preservation

Changed surfaces: `examples/shell/src/uspace/mod.rs`, `examples/shell/src/uspace/process_lifecycle.rs`, `examples/shell/src/uspace/metadata.rs`, and `examples/shell/src/uspace/fd_table.rs` path metadata tracking plus successful `renameat2(flags=0)` metadata migration.

Targeted proof:

- RV `rename01`: `target/ltp-1000-milestone-03-stable656/rv-rename01-inode-confirm-20260602T044855Z.summary.txt`
- LA `rename01`: `target/ltp-1000-milestone-03-stable656/la-rename01-inode-confirm-20260602T044855Z.summary.txt`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean22-rename01-inode-20260602T044855Z.promotion-candidates.txt`

Result: `rename01` is four-way parser-clean and joins the not-yet-promoted candidate pool.

Adjacent regression subset:

- `rename05`

Regression evidence:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-rename-inode-retarget-20260602T044708Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-rename-inode-retarget-20260602T044751Z.summary.txt`
- Result: `rename01,rename05` produce 4/4 wrapper PASS on each arch, with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Remaining regression boundary: this does not implement `link(2)` or full hard-link/nlink semantics. Future link/rename fixes must first make RV parser-clean, then rerun LA confirmation plus adjacent `rename01`, `rename05`, link/linkat, stat/statx, and unlink regression subsets before any promotion accounting.

## Rename directory replacement regression slice

| Change | Protected cases | Evidence | Result |
| --- | --- | --- | --- |
| Generic `axfs::root::rename` source/destination type handling | `rename01`, `rename03`, `rename04`, `rename05` | `rv-rename-dir-overwrite-20260602T050256Z.summary.txt`; `la-rename-dir-overwrite-20260602T050346Z.summary.txt` | RV + LA x musl+glibc PASS, parser zero internal markers/fatal signatures |
| Combined-report hygiene after old mixed TFAIL history | `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, `rename05` | `rv-statfs-rename05-clean-retarget-20260602T050521Z.summary.txt`; old LA clean5 summary; `combined-candidate-pool-clean24-rename03-04-20260602T050630Z.promotion-candidates.txt` | clean-only parser inputs produce 24 candidates without counting old `rename03/rename04` TFAIL rows |

Remaining nearby blockers: `mknod07` and `mknodat02` still need generic ext2/device setup support; they are not promoted or blacklisted here.

## Stat/readlink path traversal regression slice

| Change | Protected cases | Evidence | Result |
| --- | --- | --- | --- |
| Component-wise symlink traversal with parent-only resolution for readlink/lstat-style syscalls | `readlink03`, `stat03`, `stat03_64` | `rv-readlink-stat-path-nonrecursive-20260602T052206Z.summary.txt`; `la-readlink-stat-path-nonrecursive-20260602T052251Z.summary.txt` | `stat03` and `stat03_64` are RV + LA x musl+glibc PASS, parser zero internal/fatal markers; `readlink03` stays blocked by LA musl `TFAIL=1` |
| Nonrecursive parent directory search-permission check for `stat_path` | `stat01`, `stat02`, `stat01_64`, `stat02_64`, `lstat01`, `lstat01_64`, `fstatat01`, `readlink01`, `readlinkat01`, `openat01`, `rename14` | `rv-stat-readlink-stable-regression-20260602T052501Z.summary.txt`; `la-stat-readlink-stable-regression-20260602T052706Z.summary.txt` | RV + LA x musl+glibc PASS, parser zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap |

Repair-history boundary: `rv-readlink-stat-path-20260602T051956Z.summary.txt` recorded a parser-visible panic/trap from recursive parent-search checking and is not promotion evidence. The retained implementation uses `stat_path_inner(..., check_parent_search=false)` while checking ancestors, and the clean regression subset proves the panic is closed for this lane.

Remaining nearby blockers: `readlink03` needs a generic LA musl zero-size-buffer boundary fix or documented classification with parser-clean evidence; hard-link/linkat/statx/getdents blockers from the broader VFS/path scout remain outside this candidate pool.

## mmap20/munlock02 mmap-range regression matrix

Targeted proof:

- RV `mmap20,munlock02`: `target/ltp-1000-milestone-03-stable656/rv-mmap20-munlock02-targeted-20260602T054424Z.summary.txt`
- LA `mmap20,munlock02`: `target/ltp-1000-milestone-03-stable656/la-mmap20-munlock02-targeted-20260602T054508Z.summary.txt`
- Incremental promotion report: `target/ltp-1000-milestone-03-stable656/mmap20-munlock02-clean2-20260602T054508Z.promotion-candidates.txt`

Result: `mmap20` and `munlock02` are RV + LA x musl+glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Adjacent regression subset:

- Cases: `mmap01`, `mmap02`, `mmap03`, `mmap04`, `mmap09`, `mmap12`, `mmap13`, `munmap01`, `munlock01`, `mincore02`, `mincore03`, `mincore04`, `mprotect02`, `mprotect04`.
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mmap-munlock-regression-20260602T054554Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mmap-munlock-regression-20260602T054705Z.summary.txt`
- Result: 28/28 wrapper PASS on each arch, zero internal markers/fatal signatures.

Repair-history boundary: `rv-mmap-munlock-errno-targeted-20260602T053636Z.summary.txt` is not promotion evidence because `mmap08` and `mlock02` still fail. The `rv-mmap08-debug-*` logs are diagnostic-only fd-lifetime evidence.


## epoll_create1 FD/flag regression matrix

Targeted proof:

- RV `epoll_create1_01,epoll_create1_02`: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt`
- LA `epoll_create1_01,epoll_create1_02`: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt`
- Incremental promotion report: `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.txt`

Result: `epoll_create1_01` and `epoll_create1_02` are RV + LA x musl+glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Adjacent regression subset:

- Cases: `close01`, `fcntl01`, `fcntl05`, `dup01`, `pipe2_01`, `poll01`.
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.txt`
- Result: 12/12 wrapper PASS on each arch, zero internal markers/fatal signatures.

Remaining nearby blockers: `epoll_create02` stays non-promotable because musl's legacy wrapper hides the invalid `size` argument by issuing valid `epoll_create1(0)`. Full `epoll_ctl`/`epoll_wait` semantics are not implemented by this descriptor-creation repair.


## clock_adjtime/sigaltstack time-signal regression matrix

Targeted proof:

- RV `adjtimex01,adjtimex03,sigaltstack02,shmt04`: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt`
- LA `adjtimex01,adjtimex03,sigaltstack02,shmt04`: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt`
- Incremental promotion report: `target/ltp-1000-milestone-03-stable656/combined-clock-sigaltstack-shmt04-20260602T143805+0800.promotion-candidates.txt`

Result: `adjtimex01`, `adjtimex03`, `shmt04`, and `sigaltstack02` are RV + LA x musl+glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap.

Adjacent regression subset:

- Cases: `clock_gettime02`, `clock_nanosleep02`, `nanosleep01`, `rt_sigaction01`, `rt_sigprocmask01`, `sigaction01`, `sigprocmask01`.
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.summary.txt`
- Result: 14/14 wrapper PASS on each arch, zero internal markers/fatal signatures.

Remaining nearby boundary: `sigaltstack` does not yet switch signal delivery to the alternate stack; this checkpoint protects syscall-visible state/errno semantics only. Future signal delivery work must rerun this stable time/signal subset plus signal-delivery-specific cases before promotion accounting.


## SysV shm IPC_STAT ABI regression matrix

Targeted proof:

- RV `shmat04,shmt04`: `target/ltp-1000-milestone-03-stable656/rv-shmat04-shmt04-ipcstat-abi-20260602T150702+0800.summary.txt`
- LA `shmat04,shmt04`: `target/ltp-1000-milestone-03-stable656/la-shmat04-shmt04-ipcstat-abi-20260602T150805+0800.summary.txt`
- Combined promotion report: `target/ltp-1000-milestone-03-stable656/combined-shmat04-shmt04-ipcstat-abi-20260602T150918+0800.promotion-candidates.txt`

Result: `shmat04` is newly RV + LA x musl+glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap. `shmt04` was revalidated in the same subset and remains clean.

Protected boundary: `shmctl(IPC_STAT)` must copy exactly the Linux 64-bit `shmid_ds` ABI struct size for RV/LA. Future SysV shm lifetime/refcount work must rerun `shmat04`, `shmt04`, and the existing stable SysV shm subset before promotion accounting.

## time/timer clean3 evidence matrix

Targeted proof:

- RV time/timer scout: `target/ltp-1000-milestone-03-stable656/rv-time-timer-scout-20260602T152018+0800.summary.txt`
- LA clean3 confirmation: `target/ltp-1000-milestone-03-stable656/la-time-timer-clean3-20260602T152722+0800.summary.txt`
- Combined promotion report: `target/ltp-1000-milestone-03-stable656/combined-time-timer-clean3-20260602T152824+0800.promotion-candidates.txt`

Result: `getitimer02`, `setitimer02`, and `times03` are RV + LA x musl+glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap on the counted rows.

No source changed in this checkpoint, so no behavior-regression subset was required beyond the RV/LA targeted proof. The surrounding RV scout rows remain a visible blocker matrix for future timer work: `clock_getres01` has TCONF, `clock_gettime01` and `setitimer01` timeout, and timerfd/POSIX timer rows retain ENOSYS/TBROK/TFAIL/TCONF blockers.

## lstat clean2 evidence matrix

Targeted proof:

- RV VFS/path scout: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-simple-scout-20260602T153210+0800.summary.txt`
- LA lstat clean2 confirmation: `target/ltp-1000-milestone-03-stable656/la-lstat-clean2-20260602T153351+0800.summary.txt`
- Combined promotion report: `target/ltp-1000-milestone-03-stable656/combined-lstat-clean2-20260602T153433+0800.promotion-candidates.txt`

Result: `lstat02` and `lstat02_64` are RV + LA x musl+glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap on the counted rows.

No source changed in this checkpoint, so no behavior-regression subset was required beyond the RV/LA targeted proof. The surrounding RV scout rows remain a visible blocker matrix for future VFS/path work.

## open clean2 evidence matrix

Targeted proof:

- RV FD/VFS/IO scout: `target/ltp-1000-milestone-03-stable656/rv-fd-vfs-io-scout-20260602T153655+0800.summary.txt`
- LA open clean2 confirmation: `target/ltp-1000-milestone-03-stable656/la-open-clean2-20260602T153756+0800.summary.txt`
- Combined promotion report: `target/ltp-1000-milestone-03-stable656/combined-open-clean2-20260602T153844+0800.promotion-candidates.txt`

Result: `open07` and `open12` are RV + LA x musl+glibc parser-clean with zero `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap on the counted rows.

No source changed in this checkpoint, so no behavior-regression subset was required beyond the RV/LA targeted proof. The surrounding RV scout rows remain a visible blocker matrix for future FD/VFS/IO work.
