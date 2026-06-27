# ABI and behavior impact for stable956

## Stable-list visible impact

`LTP_STABLE_CASES` increases from 906 to 956 total / 956 unique / 0 duplicate. This changes evaluator-visible stable selection only after four-way clean evidence closed.

## Syscall / ABI changes

- POSIX mqueue: adds Linux-visible `mq_open`, `mq_unlink`, `mq_timedsend`, `mq_timedreceive`, `mq_notify`, `mq_getsetattr` behavior and synthetic `/proc/sys/fs/mqueue/*` limits. User-visible effects include descriptor flags, queue lifetime, blocking/nonblocking errno, timeout errno, and `SIGEV_SIGNAL` notification siginfo. `SIGEV_THREAD` remains outside the promoted boundary.
- pidfd: adds `pidfd_open`, `pidfd_send_signal`, `pidfd_getfd`, `waitid(P_PIDFD)`, pidfd poll readability, pidfd FD_CLOEXEC/O_NONBLOCK handling, and `KCMP_FILE` comparison needed by pidfd_getfd tests. User-visible effects include EBADF/EINVAL/EPERM/ESRCH/EAGAIN boundaries, signal permission checks, duplicated FD semantics, and child-exit lifetime checks.
- inotify: adds minimal `inotify_init1` FD allocation with `IN_CLOEXEC` -> `FD_CLOEXEC` and `IN_NONBLOCK` -> `O_NONBLOCK` propagation. It does not implement watch/event delivery and therefore does not promote `inotify01..12`.
- FD/fcntl: pipe-size error paths preserve Linux-like EINVAL/EBUSY/EPERM behavior for `F_SETPIPE_SZ` tests.
- VFS/openat2: promoted cases rely on generic openat2 resolution/proc magic-link behavior, not case-specific shortcuts.
- SysV IPC: promoted message/semaphore cases rely on generic queue/set state, blocking wakeups, permission/stat fields, and `/proc/sysvipc`/`/proc/sys/kernel/sem` behavior.
- Process/exec/signal: promoted exec/libc wrapper and signal-wait cases rely on generic exec path handling, signal mask/wait delivery, and process teardown semantics.

## Resource/lifetime risk

- pidfd and mqueue descriptors carry process/queue references; teardown paths were exercised by final RV/LA gates but full stress sweeps remain future work.
- SysV IPC state can retain kernel-side objects until explicit cleanup; final gates showed no parser-level timeout/panic/trap, but long full-sweep resource pressure remains a later quality task.
- Minimal inotify FD intentionally avoids fake event queues; reads on this FD are not used as promotion evidence.

## Non-impact boundaries

- No testsuite/evaluator behavior was changed to force PASS.
- No blacklist/status0/SKIP evidence was counted.
- No new external dependency was introduced.
