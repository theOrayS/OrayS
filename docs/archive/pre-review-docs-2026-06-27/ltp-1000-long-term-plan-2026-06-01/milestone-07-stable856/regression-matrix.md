# milestone-07 stable856 regression matrix

Promotion gate remains RV + LA × musl + glibc wrapper PASS, parser-clean for new cases, with no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`. Full stable856 gates disclose only the inherited `read02` `TCONF` caveat.

| Lane | New cases protected | Evidence | Future rerun trigger |
| --- | --- | --- | --- |
| mmap/mprotect/msync | `mmap05`, `mmap08`, `mprotect01`, `mprotect03`, `msync01`, `msync02`, `msync03` | New50 RV/LA gates; targeted `mmap01,setpriority02` regression probe; full stable856 gates | Changes to file-backed mmap, MAP_SHARED writeback, VMA split/merge, mprotect, LA PTE/protection flags, or msync invalidation/writeback. |
| process/session/signal | `getpgid01`, `tgkill03`, `setsid01`, `kill05`, `crash01`, `cve-2017-17052` | New50 RV/LA gates and full stable856 gates | Changes to process groups, session IDs, synthetic init PID, signal permission, signal target validation, crash/trap delivery, or realtime signal queueing. |
| FD/fcntl/pipe/io | `fsync03`, `read03`, `write04`, `fcntl31`, `fcntl31_64`, `fallocate01` | New50 RV/LA gates and full stable856 gates | Changes to fd status flags, pipe capacity, named FIFO, `O_ASYNC`, fd close/unshare, special-fd fsync errno, or read/write EBADF/flag handling. |
| credentials/capabilities/domainname | `capget01`, `capget02`, `capset01`..`capset04`, `setdomainname01`..`setdomainname03`, `setfsgid03` | New50 RV/LA gates and full stable856 gates | Changes to UID/GID state, saved IDs, capability masks/bounding set, prctl capability commands, UTS/domainname copy-in/out, or permission checks. |
| scheduler/ioprio | `sched_setscheduler04`, `sched_getattr01`, `sched_setattr01`, `ioprio_get01`, `ioprio_set01`..`ioprio_set03` | New50 RV/LA gates; `setpriority02` regression probe | Changes to scheduler policy/priority validation, `SCHED_DEADLINE`, ioprio target selection, or setpriority errno behavior. |
| POSIX timers/time | `timer_delete01`, `timer_delete02`, `timer_getoverrun01`, `timer_gettime01`, `timer_settime02` | New50 RV/LA gates and full stable856 gates | Changes to timer id allocation, generation cancellation, signal delivery, exec teardown, timespec copy-in/out, or time accounting. |
| VFS/statx/metadata | `utimes01`, `statx04`, `statx12`, `inode02` | New50 RV/LA gates and full stable856 gates | Changes to statx attributes/mount-root, inode flags, utimensat null pathname, path metadata movement, or inode/resource allocation. |
| SysV shm | `shmt05`, `shmctl07`, `shmctl08` | New50 RV/LA gates and full stable856 gates | Changes to shm limits, attach accounting, IPC_SET, LOCK/UNLOCK, permission checks, or `/proc/sys/kernel/shm*` defaults. |
| thread/runtime/lifetime | `nptl01`, `pth_str02` | New50 RV/LA gates and full stable856 gates | Changes to thread teardown, futex wakeups, LazyInit global tables, signal wait state, timer teardown, or allocator/backend sharing. |

## Resource watchpoints

| Case | Observation | Required follow-up |
| --- | --- | --- |
| `inode02` | Current-pool four-way max runtime `44680 ms`, min free-frame delta `-16646` | Rerun on any inode/VFS/resource allocator change; compare both runtime and free-frame delta. |
| `nptl01` | Current-pool four-way max runtime `21951 ms` | Rerun after thread/futex/signal/timer lifetime changes. |
| `capget02` / `capset02` | Current-pool min free-frame deltas around `-4112`/`-4110` | Rerun after capability or process-credential structure changes. |
| `cve-2017-17052` | Current-pool max runtime `8701 ms`, min delta `-661` | Keep in stable regression subset after signal/crash/trap edits. |

## Explicit non-regression decision

`nice04` remains outside stable856 because the attempted errno change regressed existing stable `setpriority02`. Future nice/setpriority work must first preserve `setpriority02` and then produce fresh RV + LA × musl + glibc evidence for `nice04`; this milestone does not count it.
