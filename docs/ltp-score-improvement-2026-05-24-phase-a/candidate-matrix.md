# Candidate matrix: stable250 -> stable300

## Final promoted set overview

- stable count: total=300, unique=300, duplicates=0.
- Final promoted tranche to stable300: `nice01, nice02, prctl01, sethostname01, sethostname02, sethostname03, clock_nanosleep04, nanosleep04, nice03, fcntl23_64, setuid03, prctl05, ftruncate03, truncate03, lseek07`.
- stable285 tranche: `open08, open13, pipe12, pipe13, pipe2_01, pipe2_04, dup207, getcwd02, fchdir02, fcntl23, open09, sched_getattr02, statvfs02, symlink02, symlink04`.
- stable270 tranche used earlier in this phase is documented in `stable270-promotion-gate-report.md`.

## User-priority cases

| Case | Final status | Rationale |
| --- | --- | --- |
| `access02` | not promoted | deferred: targeted RV evidence still had real TFAIL; not promoted |
| `access04` | not promoted | deferred: targeted evidence TBROK; not promoted |
| `open08` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `open13` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `chmod05` | not promoted | deferred: glibc cleaned after chmod mode work but musl still TBROK; not promoted |
| `statx01` | not promoted | deferred: ENOSYS/TBROK evidence; not promoted |
| `readv02` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `writev01` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `writev03` | not promoted | deferred: expected SIGPIPE/TCONF path still not clean; not promoted |
| `writev05` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `writev06` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `writev07` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `pipe12` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `pipe2_01` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `pipe2_02` | not promoted | deferred: TBROK evidence; not promoted |
| `pipe2_04` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `pipe13` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `waitpid01` | not promoted | deferred: glibc cleaned but musl still TFAIL; not promoted |
| `waitid01` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `waitid02` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `waitid03` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `waitid04` | promoted/stable | Clean in final stable300 RV+LA aggregate. |
| `mmap05` | not promoted | deferred: signal/page-permission failures; not promoted |
| `mprotect01` | not promoted | deferred: signal/page-permission failures; not promoted |
| `mprotect02` | not promoted | deferred: TBROK; not promoted |
| `munmap01` | not promoted | deferred: signal/page-permission failures; not promoted |
| `mmap04` | not promoted | deferred: TBROK; not promoted |
| `mmap06` | not promoted | deferred: TFAIL; not promoted |


## Subsystem classification

| Subsystem | Promoted examples | Blocked/deferred examples |
| --- | --- | --- |
| permissions/VFS/errno | `open08`, `open13`, `ftruncate03`, `truncate03`, `lseek07` | `access02`, `access04`, `chmod05`, `statx01` |
| iovec/read-write | `readv02`, `writev01`, `writev05`, `writev06`, `writev07`, preadv/pwritev stable cases | `writev03` |
| pipe/fd lifecycle | `pipe12`, `pipe13`, `pipe2_01`, `pipe2_04`, `dup207`, `fcntl23_64` | `pipe2_02` |
| process/wait/sched/rlimit | `waitid01`..`waitid04`, `nice01`..`nice03`, `setuid03`, `prctl01`, `prctl05`, `sethostname01`..`03` | scheduler negative-pid candidates blocked on LA libc evidence |
| time/mmap/signal | `clock_nanosleep04`, `nanosleep04` | `mmap04`, `mmap05`, `mmap06`, `mprotect01`, `mprotect02`, `munmap01` |

## Guardrail

No case was promoted from wrapper success alone. Cases with TFAIL/TBROK/TCONF (except disclosed `read02`), timeout, ENOSYS, panic/trap were excluded.
