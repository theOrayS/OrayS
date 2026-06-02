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

## If fixing recoverable user SIGSEGV / page-fault signal delivery

Primary retest cases:

- `mmap05`
- `munmap01`

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

## Non-promotion rows

- `mmap10_1`: do not include until the guest LTP inventory contains the binary.
- `vma02`: do not include until libnuma-related `TCONF` is resolved and both libcs are parser-clean.
- `readlinkat02`: do not include until LA musl is parser-clean; add readlink/readlinkat and user-pointer boundary regressions once a root-cause fix exists.
- `nice04`: do not include until the `nice()` wrapper errno boundary is fixed without regressing `setpriority02`; use `nice04-errno-boundary-report.md` as the handoff.
- closed arch sweep: no extra stable606-missing four-way-clean rows remain; use the matrices only for blocker prioritization.
- `kill10`: do not include broad batches until the panic/trap is isolated.
