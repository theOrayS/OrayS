# stable756 ABI and behavior impact

## Summary

Milestone-05 is not a score-only stable-list change. It adds or tightens generic Linux-visible behavior in fd objects, VFS metadata, signal/process wait state, and synthetic proc/config files. No userspace struct layout is intentionally changed; the visible boundary is syscall return value/errno/flag/readiness/stat/mmap behavior.

## Syscall / errno / flag changes

- `eventfd` / `eventfd2`: adds shared eventfd counter objects with `EFD_SEMAPHORE`, `EFD_NONBLOCK`, `EFD_CLOEXEC`, read/write width checks, poll readiness, fd duplication/fork sharing, and fcntl-visible status flags. Visible errors include ordinary `EAGAIN`, `EINVAL`, and fd access errors rather than synthetic test-name handling.
- `epoll_create*`, `epoll_ctl`, `epoll_wait`, `epoll_pwait`, `epoll_pwait2`: adds epoll fd objects, control validation, duplicate/missing target handling, basic nested/cycle guard, oneshot/edge-trigger tracking, zero-timeout fast path, signal-mask-aware wait entry points, and poll wait wakeups. The behavior is generic fd/readiness semantics, not case-specific output.
- `timerfd_create`, `timerfd_settime`, `timerfd_gettime`: adds timerfd fd objects, clock/flag validation, read/poll state, and fcntl integration for the promoted timerfd subset.
- `signalfd` / `signalfd4`: adds signalfd fd objects, signal-mask copy-in, read/poll behavior, `SFD_NONBLOCK`, `SFD_CLOEXEC`, and delivery interaction with process pending signals.
- `link` / `linkat`: adds hard-link alias bookkeeping, link count metadata, symlink-following behavior, readonly/cross-mount ordering, and parent-path checks. Cross-mount hard-link attempts now report `EXDEV` before readonly fallthrough where appropriate.
- `renameat2`: adds `RENAME_NOREPLACE` and `RENAME_EXCHANGE` behavior, same-path/no-op handling, permission and sticky-bit checks, symlink parent resolution, cross-mount `EXDEV`, and sparse-file metadata move fixes.
- `fcntl` pipe size commands: pipe initial capacity and `F_GETPIPE_SZ`/`F_SETPIPE_SZ` behavior now distinguish privileged/root capacity from unprivileged defaults while preserving pipe status flags.
- FIFO `O_RDWR`: FIFO opens with `O_RDWR` now create bidirectional endpoints with reader and writer accounting, fixing readiness and EOF/SIGPIPE semantics for generic FIFO users.
- Pipe readiness: pipe endpoints now maintain an atomic buffered-byte count and expose separate readable/writable poll helpers, so zero-timeout poll/epoll paths can observe no-ready state without sleeping.
- `/dev/zero`: real `FdEntry::DevZero` behavior is added for stat/read/write/poll/fsync and mmap validation. Reads fill zero bytes, writes consume bytes, stat reports a char-device rdev, and `/dev/zero` mmap is accepted as zero-backed anonymous memory.
- `open`/`openat`: directory + `O_CREAT` and synthetic char-node handling are tightened to return Linux-like errors for open11-style cases without hardcoding case names.
- `waitid`: adds stopped/continued child-event recording for `WSTOPPED`, `WCONTINUED`, and `WNOWAIT`; `SIGCONT` ABI number is exposed through the shared signal constants.
- Regular-file physical short reads: reads from backing files now loop over physical short reads and return the actual bytes obtained instead of reporting a full zero-filled range. This changes visible copy/read behavior from silent zero-fill to correct short-read semantics and protects copied ELF helper binaries.
- Exec loader: when the executable path root falls back to `/`, PT_INTERP is used to derive `/musl` or `/glibc` for interpreter/env resolution, and exec flushes the current TLB after replacing mappings in the same page-table root. This is exec-lifetime correctness, not a new userspace struct ABI.
- `rt_sigtimedwait`: replaces the prior stub-like behavior with signal-set-aware waiting. It copies the user signal set, consumes a matching pending signal, writes Linux-shaped siginfo for the promoted paths, returns `EAGAIN` on timeout, and no longer fabricates `SIGCHLD` for unrelated waits. This is required for libc `sigwait()` users such as shared-memory handoff tests.
- `clock_getres`: reports a conservative 50 ms resolution for supported clocks. This is a visible ABI value, chosen to match the effective QEMU/scheduler timer granularity observed under full stable runs rather than claiming sub-millisecond precision that the kernel cannot reliably deliver.
- Synthetic files: adds read-only `/proc/sys/kernel/core_pattern` and synthetic kernel config paths containing `CONFIG_EVENTFD=y` for feature-discovery tests.

## FD / lifetime / resource risk

- New fd variants (`EventFd`, `Epoll`, `TimerFd`, `SignalFd`, `DevZero`) increase fd-table match surface. Each variant must be kept in fork/dup/close/poll/fcntl/stat paths when future fd operations are added.
- Epoll wait/wake and eventfd/timerfd/signalfd readiness are shared-state paths; future changes must avoid holding fd-table locks across blocking waits or signal delivery.
- Pipe capacity moved to heap-backed storage to avoid large stack frames. This reduces stack-panic risk but increases heap pressure when many large pipes exist. The atomic buffered-byte counter must remain exact across every read/write path.
- Hard-link and rename metadata add alias/lifetime coupling between path metadata and sparse-file overlays; future VFS changes must preserve link-count and alias cleanup on unlink/rename.
- `waitid` stopped/continued state is process-lifetime state. Future signal semantics must preserve WNOWAIT behavior and not consume events prematurely.

## User pointer / mmap impact

- The new fd syscalls copy masks/timespecs/event values through existing user-memory helpers; invalid pointers are expected to produce `EFAULT` rather than panic.
- `rt_sigtimedwait` copies both the signal set and optional timeout/siginfo pointers through existing checked user-memory helpers; invalid pointers are expected to produce syscall errors instead of kernel traps.
- `/dev/zero` mmap validation now accepts `FdEntry::DevZero` and routes mapping through zero-backed non-file-backed logic. This intentionally changes prior `EBADF` behavior for `/dev/zero` mappings.
- No new userspace ABI struct layout is introduced for stable-list storage. Signalfd/timerfd/eventfd data uses existing Linux-compatible userspace representations already available in the shell userspace layer.

## Maintenance boundary

- These implementations are minimal Linux-compatible semantics for the promoted LTP subset and adjacent regressions; they are not complete namespace, io_uring, fanotify, inotify, or full procfs implementations.
- Future promotion must use fresh RV + LA x musl + glibc evidence; the presence of a new fd variant in the table is not by itself proof of broad feature completeness.
