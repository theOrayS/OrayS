# Milestone 03 stable656 regression matrix

This checkpoint records the regression sets that must protect future G009 fixes. No candidate from this scout is promoted yet.

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

- current stable futex rows, if any, from `LTP_STABLE_CASES`
- timeout/EINTR-related wait and signal-mask cases
- task teardown/wakeup lifetime smoke tests

## Non-promotion rows

- `mmap10_1`: do not include until the guest LTP inventory contains the binary.
- `vma02`: do not include until libnuma-related `TCONF` is resolved and both libcs are parser-clean.
