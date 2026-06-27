# milestone-07 stable856 promotion candidates

These 50 unique cases are promoted into `LTP_STABLE_CASES` for milestone-07 stable856. The current pool SHA is `ae97ecb3975f7fc79fbb29b2532828b48f39011fbca30bc5f628aea634bfcd42`; the pool has `50 total / 50 unique / 0 duplicate` and the resulting stable list has `856 total / 856 unique / 0 duplicate`.

Promotion source of truth:

- RV new50 current-pool summary: `target/ltp-1000-milestone-07-stable856/rv-stable856-new50-currentpool-gate-20260604T205151+0800/rv-summary.txt` — `100 PASS / 0 FAIL / 0 internal markers`.
- LA new50 current-pool summary: `target/ltp-1000-milestone-07-stable856/la-stable856-new50-currentpool-gate-20260604T205650+0800/la-summary.txt` — `100 PASS / 0 FAIL / 0 internal markers`.
- Four-way candidate report: `target/ltp-1000-milestone-07-stable856/la-stable856-new50-currentpool-gate-20260604T205650+0800/fourway-promotion-candidates.txt` — `Promotion candidates: 50`, `Blocked/incomplete cases: 0`.

| Case | Main behavior covered | Status |
| --- | --- | --- |
| `getpgid01` | process group visibility including synthetic init process boundary | promoted stable856 |
| `tgkill03` | signal number/target validation and realtime blocked-queue `EAGAIN` boundary | promoted stable856 |
| `setsid01` | session/process-group collision errno behavior | promoted stable856 |
| `fsync03` | `fsync`/`fdatasync` special-fd errno behavior | promoted stable856 |
| `read03` | read errno/FD semantics | promoted stable856 |
| `write04` | write errno/FD semantics | promoted stable856 |
| `kill05` | kill permission/session/process-group semantics | promoted stable856 |
| `mmap05` | generic mmap validation and mapping protection behavior | promoted stable856 |
| `mmap08` | mmap bad-fd/length validation ordering | promoted stable856 |
| `mprotect01` | mprotect access validation and LA protection trapping | promoted stable856 |
| `msync03` | `MS_INVALIDATE`/locked range and file-backed mapping validation | promoted stable856 |
| `fcntl31` | pipe/F_SETPIPE_SZ/F_GETPIPE_SZ and fcntl status behavior | promoted stable856 |
| `fcntl31_64` | same as `fcntl31` for 64-bit test variant | promoted stable856 |
| `mprotect03` | shared mmap mprotect/fork protection behavior | promoted stable856 |
| `utimes01` | `utimensat` null-path `EFAULT` boundary | promoted stable856 |
| `shmt05` | SysV shm limits/attach behavior | promoted stable856 |
| `shmctl08` | SysV shm `IPC_SET` metadata update | promoted stable856 |
| `shmctl07` | SysV shm `SHM_LOCK`/`SHM_UNLOCK` state | promoted stable856 |
| `fallocate01` | fallocate baseline behavior after current FD/VFS repairs | promoted stable856 |
| `capget01` | Linux capability query ABI | promoted stable856 |
| `capget02` | capability data/version edge behavior | promoted stable856 |
| `capset01` | capability mutation ABI | promoted stable856 |
| `capset02` | capability permission/subset checks | promoted stable856 |
| `capset03` | capability validation boundary | promoted stable856 |
| `capset04` | capability version/array boundary | promoted stable856 |
| `sched_setscheduler04` | scheduler policy privilege/errno behavior | promoted stable856 |
| `setdomainname01` | domainname success path and utsname copy-out | promoted stable856 |
| `setdomainname02` | domainname permission/length validation | promoted stable856 |
| `setdomainname03` | domainname fault/errno validation | promoted stable856 |
| `sched_getattr01` | sched_attr copy-out including deadline fields | promoted stable856 |
| `sched_setattr01` | sched_attr validation/permission behavior | promoted stable856 |
| `ioprio_get01` | per-process ioprio query and default BE/4 | promoted stable856 |
| `ioprio_set01` | ioprio set/get success path | promoted stable856 |
| `ioprio_set02` | ioprio class/data validation | promoted stable856 |
| `ioprio_set03` | ioprio target/permission validation | promoted stable856 |
| `timer_delete01` | POSIX timer delete success behavior | promoted stable856 |
| `timer_delete02` | POSIX timer invalid/delete errno behavior | promoted stable856 |
| `timer_getoverrun01` | POSIX timer overrun query | promoted stable856 |
| `timer_gettime01` | POSIX timer gettime copy-out | promoted stable856 |
| `timer_settime02` | POSIX timer settime validation/armed behavior | promoted stable856 |
| `msync01` | file-backed msync baseline | promoted stable856 |
| `msync02` | msync alignment/flag behavior | promoted stable856 |
| `statx04` | statx attribute reporting | promoted stable856 |
| `statx12` | statx mount-root attribute reporting | promoted stable856 |
| `setfsgid03` | setfsgid behavior under current credentials model | promoted stable856 |
| `inode02` | inode metadata/resource stress row | promoted stable856; monitor resource deltas |
| `crash01` | signal/crash isolation without kernel panic/trap | promoted stable856 |
| `cve-2017-17052` | regression/security row, no panic/trap | promoted stable856 |
| `nptl01` | thread/runtime compatibility row | promoted stable856; monitor runtime |
| `pth_str02` | pthread/string/runtime compatibility row | promoted stable856 |

## Not promoted

- `nice04`: rejected after an intermediate setpriority errno change regressed existing stable `setpriority02`. The source change was reverted and `nice04` is not counted.
- Any row with visible `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`, blacklist/SKIP/status0/full-sweep-only local TPASS, or partial arch/libc coverage remains excluded.
