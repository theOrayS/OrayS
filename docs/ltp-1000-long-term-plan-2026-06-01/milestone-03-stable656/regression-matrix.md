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
- `kill10`: do not include broad batches until the panic/trap is isolated.

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
