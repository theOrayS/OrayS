# Milestone 03 stable656 regression matrix

This checkpoint records both completed regression evidence for the `sched_setaffinity01` fix and required future regression sets for unresolved G009 blockers.

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

Primary retest case:

- `futex_wait03`

Adjacent regression candidates:

- `futex_wait01` (now four-way clean candidate)
- current stable futex rows, if any, from live `LTP_STABLE_CASES`
- timeout/EINTR-related wait and signal-mask cases
- task teardown/wakeup lifetime smoke tests

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
- `readlinkat02`: do not include until LA musl is parser-clean.
- `kill10`: do not include broad batches until the panic/trap is isolated.
