# Milestone 03 stable656 no-promotion reason

This checkpoint found seven four-way-clean future candidates, but no stable promotion is performed yet.

## Why no stable list update happened

- Live stable baseline: `606 total / 606 unique / 0 duplicate`.
- Next milestone target: `656 unique`.
- Current four-way-clean new candidate pool: 7 (`fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mmap13`, `munmap01`, `sched_setaffinity01`).
- Required promotion batch size for this milestone: 50 unique cases with RV + LA x musl + glibc wrapper PASS and parser-clean summaries.

Because the candidate pool is below the +50 milestone boundary, `LTP_STABLE_CASES` remains unchanged.

## Blocking evidence kept visible

The following blockers prevent counting additional rows:

| Case / lane | Blocking reason |
| --- | --- |
| `mmap05` | RV is now clean after catchable synchronous `SIGSEGV`, but LA musl+glibc still report `TFAIL=1` / SIGSEGV signal not received; a local explicit TLB-flush experiment and temporary instrumentation left the TFAIL unchanged |
| `mmap10_1` | missing testcase in both guest LTP trees |
| `vma02` | libnuma `TCONF` |
| old `futex_wait03` scout row | superseded timeout evidence; current RV/LA targeted reruns are parser-clean after `/proc/<pid>/stat` sleeping-state repair |
| `kill10` | severe panic/trap during RV VFS/process scout; evidence cannot be promoted |
| `shmat1` | mixed scout was manually terminated after hang/long run; evidence is scouting only |
| `readlinkat02` | RV and LA glibc clean, but LA musl `TFAIL`; root-cause audit found musl converts user `bufsize == 0` to a dummy one-byte syscall, so a safe generic kernel fix is not available without breaking direct `bufsiz=1` semantics |
| pre-fix `fsync02` row | old isolated RV rerun had glibc `TBROK=1`; superseded by post-fix proof but retained as failed evidence |
| `openat02` | post-statfs-clamp isolated RV rerun still fails both musl and glibc with `TBROK` setup `ENOSPC`; no LA rerun or promotion accounting |
| `nice04` | RV musl gets `EACCES` for `nice(-10)`, but stable `setpriority02` source requires direct unprivileged `setpriority` lowering to return `EACCES`; no safe kernel errno flip |
| `clone04` | RV glibc PASSes the NULL-stack `EINVAL` check, but RV musl is killed by SIGSEGV/TBROK; raw log hints at a musl `clone.c` wrapper fix, so it is not counted or LA-confirmed from this failed RV gate |
| closed arch sweep | 563 four-way-clean historical rows, but zero not-yet-stable rows after filtering live stable606 |
| `select02`, `sched_rr_get_interval03`, `setpriority01` | wrapper PASS rows include `TCONF`; not promotion evidence |
| `nice05`, `mincore03`, `atof01`, `fptest01`, `fptest02`, `epoll_create02`, `diotest4`, `execve05` | fail, TFAIL/TBROK/TCONF/ENOSYS, or incomplete arch matrix remains |

## Decision

- Do not edit `LTP_STABLE_CASES`.
- Do not count blacklist/SKIP/status0/timeout/TCONF/TBROK/TFAIL rows as PASS.
- Keep `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, `mmap13`, `munmap01`, and `sched_setaffinity01` in `promotion-candidates.md` for the next accumulation batch.
