# stable906 ABI and behavior impact

## SysV message queues

New user-visible Linux/POSIX IPC behavior:

- Dispatches `msgget(2)`, `msgsnd(2)`, `msgrcv(2)`, and `msgctl(2)` for RV64 and LA64 userspace.
- Uses the 64-bit asm-generic SysV IPC layout: `ipc_perm` is 48 bytes and `msqid_ds` is 120 bytes.
- Exposes real `msg_stime`, `msg_rtime`, `msg_ctime`, `msg_cbytes`, `msg_qnum`, `msg_qbytes`, `msg_lspid`, `msg_lrpid`, key/uid/gid/cuid/cgid/mode fields through `IPC_STAT`.
- Implements `IPC_SET` for uid/gid/mode/qbytes updates and `IPC_RMID` removal.
- Implements message type selection for `msgtyp == 0`, positive exact type, `MSG_EXCEPT`, and negative type lookup.
- Implements visible errno boundaries used by LTP: `EEXIST`, `ENOENT`, `EACCES`, `EFAULT`, `EINVAL`, `E2BIG`, `ENOMSG`, `ENOSPC`, and `EAGAIN`.
- Honors `IPC_CREAT`, `IPC_EXCL`, mode permission bits, `IPC_NOWAIT`, `MSG_NOERROR`, and `MSG_EXCEPT`.

Resource/lifetime model:

- Queues are global in-memory kernel state for the current evaluator boot.
- Queue count, total bytes, and message size are bounded to avoid unbounded memory growth.
- The implementation does not yet model blocking wait queues, signal interruption during blocking IPC, namespace isolation, or `MSG_COPY`.
- System V persistence is intentional until `IPC_RMID`; LTP cleanup removes queues in the promoted cases.

## Memory-lock/smaps lane

Visible effects already guarded by the milestone evidence:

- `mlock2(MLOCK_ONFAULT)` marks ranges locked without immediate prefaulting; invalid flags return `EINVAL`.
- `mlock`/`mlockall` respect `RLIMIT_MEMLOCK` accounting used by promoted lock tests.
- `/proc/self/smaps` and `/proc/<pid>/smaps` expose `Rss`, `Locked`, and VMA flags from live mappings for `mlock05`.

## Signal/time/process/VFS/FD lanes

Visible effects covered by the final new50 evidence:

- `rt_sigaction` rejects invalid signal-set sizes with `EINVAL`.
- Existing real VFS/FD/time/socket/process fixes and candidate gates carry the promoted chroot, fallocate, readlink, nanosleep, setgroups, socket ioctl/setsockopt, timer, mremap, madvise, memfd, and related cases.

## No fake-pass boundary

No LTP case names, test paths, process names, or output strings were hardcoded. Parser-clean raw logs, not wrapper-only output, are the promotion source.
