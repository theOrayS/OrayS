# milestone-09 stable956 report

## Goal

Promote stable baseline from 906 to 956 trusted unique LTP stable cases without fake pass behavior.

## Result

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: 956 total / 956 unique / 0 duplicate.
- Added 50 new cases, all RV + LA x musl + glibc wrapper PASS.
- Final RV/LA new50 gates are parser-clean: no TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.

## Code changes in this milestone

- Generic SysV message queue and SysV semaphore semantics used by the banked IPC cases.
- openat2/proc magic-link and fcntl pipe-size errno behavior used by VFS/FD cases.
- Process/exec, zombie proc stat, signal wait, pending signal, and rusage-adjacent repairs used by process/signal regression lanes.
- POSIX mqueue descriptors/syscalls and `/proc/sys/fs/mqueue` synthetic support for the five mqueue cases.
- pidfd FD type, pidfd_open, pidfd_send_signal, pidfd_getfd, KCMP_FILE comparison, pidfd poll/waitid integration, and `/proc/<pid>` directory fd support for the pidfd cases.
- Minimal inotify FD type plus `inotify_init1` flag semantics for `inotify_init1_01/02`; event-delivery semantics are deliberately not claimed.

## Evidence

See `validation.md` for exact commands, summary/log/json/checksum paths, and parser output. Final gates:

- RV: `target/ltp-1000-milestone-09-stable956/rv-stable956-new50-final-gate-20260605T222350+0800.summary.txt`.
- LA: `target/ltp-1000-milestone-09-stable956/la-stable956-new50-final-gate-20260605T222730+0800.summary.txt`.

## Risks and maintenance boundary

- No blacklist change was used for promotion.
- No LTP case names/paths/process names/output strings are hardcoded in kernel behavior.
- `inotify_init1` is intentionally minimal: it creates a Linux-like FD and exposes FD_CLOEXEC/O_NONBLOCK state; it does not claim general inotify watch/event delivery.
- pidfd PID-reuse protection (`pidfd_send_signal03`) remains unpromoted pending a generic `/proc/sys/kernel/ns_last_pid` and PID lifecycle design.
- mqueue `SIGEV_THREAD` notify variants remain unpromoted.

## Next step

Proceed to the next milestone target stable1000 with 44 additional trusted unique cases, prioritizing real semantics over partial/blacklist evidence.
