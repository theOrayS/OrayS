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
