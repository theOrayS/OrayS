# milestone-01-stable556 regression matrix

## Protected stable baseline

The final promotion gate for this milestone is the full stable list after promotion:

- Expected stable count: 556 total / 556 unique / 0 duplicate.
- Required final gates: `LTP_CASES=stable` on RV and LA, both running musl and glibc wrappers.
- Parser truth source: `scripts/ltp_summary.py` summary and JSON outputs.

## Adjacent regression sets protected by this milestone

| Lane | Stable cases now protecting it | Adjacent non-promoted cases to keep visible |
| --- | --- | --- |
| VFS/path/metadata | fs_perms, readdir01, ioctl_ns07 | access04, chmod06, chmod07, chown04, fchmod02, fchmod06, fchown04, fchownat02, mknod*, rename*, openat02, openat03 |
| FD/fcntl/io | fcntl19_64, fcntl20, fcntl20_64, fcntl21, fcntl21_64, fcntl22_64 | later fcntl/flock/sendfile/iovec expansion must preserve shared offset, FD_CLOEXEC, and lock semantics |
| Time/select | clock_nanosleep02, poll02, pselect01, pselect01_64, settimeofday01, time-schedule | sched_rr_get_interval03 and setpriority01 remain TCONF; signal01 timeout remains blocker |
| Socket/net | accept01, listen01, socket02, socketpair02 | broader TCP/UDP/UNIX socket errno/readiness still needs separate lane evidence |
| MM/resource | data_space, dirty, mlockall01, mmap-corruption01, mmstress_dummy, page01, page02, sbrk02, stack_space, ulimit01 | file-backed shared mmap/msync/mincore/mprotect split/merge remain future targets |
| System/proc/user-libc | newuname01, utsname01, utsname04, nextafter01, gen* selected rows | proc/sys fields and harness rows should not be promoted as kernel semantics without lane-specific documentation |

## Gate decision

- Proof batches are candidate gates only.
- Full `LTP_CASES=stable` RV and LA gates are the actual regression gates for this milestone.
- Inherited stable506 caveat (`read02` TCONF in prior evidence) must stay visible if it appears; no newly added case may introduce TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.
