# milestone-07 stable856 ABI and behavior impact

Date: 2026-06-04.

This milestone makes POSIX/Linux-visible behavior changes. All changes are generic kernel/user-space semantics, not LTP-name/path/output hardcoding.

## Syscall and ABI surfaces changed

| Area | User-visible impact | Regression boundary |
| --- | --- | --- |
| `mmap`/`mprotect`/`msync` | File-backed `MAP_SHARED` mappings now record backing file/offset/valid length; `msync` writes back only valid file bytes and must not extend/write zero-filled EOF tail. `MS_INVALIDATE` over locked ranges returns `EBUSY`. Shared mapping protection changes propagate through process shared-mmap metadata. LA `PROT_NONE` user pages are encoded so user access traps instead of silently reading. | `mmap01`, `mmap05`, `mmap08`, `mprotect01`, `mprotect03`, `msync01`, `msync02`, `msync03`; final regression probe `mmap01,setpriority02`; full RV/LA stable856 gates. |
| Signals/process groups | `kill`, process-group delivery, and `tgkill` now validate signal numbers/targets and permissions more closely; same-session `SIGCONT` is allowed; blocked realtime signals with `RLIMIT_SIGPENDING=0` return `EAGAIN`. `rt_sigtimedwait` records signal-wait state while sleeping. | `kill05`, `tgkill03`, `crash01`, existing stable signal/process rows in full gates. |
| Process/session IDs | Synthetic init PID 1 is visible for `getpgid`/`getsid` and `/proc` status/stat; `setpgid`/`setsid` validate session/process-group collision semantics. | `getpgid01`, `setsid01`, `/proc` stable rows in full gates. |
| Capabilities/prctl | Added Linux capability state per process (`effective`, `permitted`, `inheritable`, `bounding`), `capget`, `capset`, and `PR_CAPBSET_READ/PR_CAPBSET_DROP` behavior. Capability mutation checks enforce effective subset of permitted and bounding-set limits. | `capget01`, `capget02`, `capset01`..`capset04`, final stable gates. |
| UTS domainname | `uname` copy-out includes domainname; `setdomainname` validates length, privilege, and user pointer fault behavior. | `setdomainname01`..`setdomainname03`, existing UTS rows in full stable gate. |
| Scheduler/ioprio | Added per-process I/O priority with `ioprio_get`/`ioprio_set` validation, plus `sched_getattr`/`sched_setattr` support for stored runtime/deadline/period and `SCHED_DEADLINE` validation. | `sched_setscheduler04`, `sched_getattr01`, `sched_setattr01`, `ioprio_get01`, `ioprio_set01`..`ioprio_set03`; `setpriority02` regression probe protects old priority errno behavior. |
| POSIX timers | Implemented process-local POSIX timer IDs, create/delete/gettime/getoverrun/settime state, generation-based cancellation, and limited signal delivery for `SIGEV_SIGNAL`/`SIGEV_THREAD_ID`; timers are cleared on exec. | `timer_delete01`, `timer_delete02`, `timer_getoverrun01`, `timer_gettime01`, `timer_settime02`; existing time/signal rows in full gates. |
| FD/fcntl/pipe/FIFO | Named FIFO instances share buffer/peer state by path; pipe `O_ASYNC`, `F_SETOWN`, `F_GETOWN`, `F_SETOWN_EX`, `F_GETOWN_EX`, `F_SETSIG`, `F_GETSIG` track owners and generate SIGIO-style notifications. `close_range` and `/dev/zero` status flag/read-write checks were added. `fsync`/`fdatasync` on pipes/sockets/special fds returns `EINVAL`. | `fcntl31`, `fcntl31_64`, `fsync03`, `read03`, `write04`, final stable FD/fcntl rows. |
| VFS/statx/time metadata | `statx` now reports inode flags and mount-root attributes with an attribute mask. `utimensat` null pathname returns `EFAULT`. | `statx04`, `statx12`, `utimes01`, full VFS/stat stable rows. |
| SysV shm | `IPC_SET` updates owner/group/permission metadata; `SHM_LOCK`/`SHM_UNLOCK` update locked state; shmmax/shmall defaults raised to 1 MiB/256 pages for current test envelope. | `shmt05`, `shmctl07`, `shmctl08`, previous SysV shm stable rows in full gates. |
| Lazy globals / lifetime | Several global tables use `call_once` initialization instead of manual `is_inited` checks to avoid race-prone double initialization. | Full RV/LA stable856 gates and thread/runtime rows `nptl01`, `pth_str02`. |
| LA trap/backend | LoongArch page privilege illegal exceptions are handled as page faults; `PROT_NONE` PTE flag mapping preserves kernel lifecycle while denying PLV3 user access. | LA mprotect/mmap rows, full LA stable856 gate. |

## No intended ABI changes

- No syscall number, struct layout, or stable-list parser format was changed.
- No testsuite/evaluator behavior was changed.
- No blacklist/SKIP/status0 semantics were used as promotion evidence.

## Known caveats and risks

- Full stable gates still contain inherited `read02` `TCONF` for `O_DIRECT not supported on tmpfs`. This milestone does not solve direct I/O; it only ensures no new promoted case relies on TCONF evidence.
- `inode02` has the largest observed runtime/free-frame delta in the current-pool four-way gate (`44680 ms`, min delta `-16646`), so future inode/resource changes should rerun it before promotion.
- POSIX timer signal delivery is intentionally limited to the semantics currently exercised by clean cases; wider timer/signal cases remain future work.
