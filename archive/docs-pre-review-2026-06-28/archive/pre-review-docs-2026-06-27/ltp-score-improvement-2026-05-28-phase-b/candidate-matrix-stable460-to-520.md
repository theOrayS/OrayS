# Candidate matrix: stable460 -> stable520

Date: 2026-05-28
Status: bootstrap skeleton; worker/leader updates append fresh parser evidence here.

## Baseline

- Live stable list: 460 total / 460 unique / 0 duplicates.
- Previous final gate: archived stable460 RV+LA final 002 both PASS 920 / FAIL 0 / musl 460/0 / glibc 460/0, only known `read02` TCONF.
- Promotion definition: RV+LA x musl+glibc wrapper PASS and zero internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap for newly promoted rows.

## Initial high-value queues

| Pool | Candidate rows | Starting status | Required next proof |
| --- | --- | --- | --- |
| Batch 0 reserves | `mknod08`, `mknodat01`, `rename14` | clean reserve in stable460 docs only; not live phase-b proof | fresh RV+LA targeted four-way clean, then aggregate gate |
| VFS/path | `mknod01`, `mknod03`, `mknod04`, `mknod07`, `mknod09`, `mknodat02`, `rename03`, `rename04`, `rename05`, `openat02`, `openat03`, plus repaired chmod/access/statx/link/unlink/symlink/mkdir/rmdir/truncate rows | scout/repair | RV first, LA only for RV-clean subset; protect parent permission/sticky/symlink/errno semantics |
| FD/fcntl/pipe/ownership | `pipe07`, `fcntl19`, `fcntl19_64`, `fcntl20`, `fcntl20_64`, `fcntl21`, `fcntl21_64`, `fcntl22`, `fcntl22_64`, `fchown04`, `fchownat02`, `chown04` | scout | prove no SIGPIPE/lock-order/timeout regressions |
| Metadata/statfs/getdents | `getdents01`, `getdents02`, `fstat02`, `fstat02_64`, `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01`, `getcwd03`, `getcwd04` | mostly blocked/diagnostic | source-level expectation + narrow fix + parser clean; never blind legacy alias |
| Process/light syscalls | `waitid07`, `waitid08`, `waitid10`, `setpriority01`, `nice04`, `clock_gettime01`, `clock_gettime04`, `sched_rr_get_interval03`, `sched_setaffinity01`, `setrlimit04`, `setrlimit05`, `signal01` | scout, aggregate-sensitive | targeted clean plus aggregate proof; `kill02` blocked until LA setup TBROK fixed |
| VM/mmap | `mmap04`, `mmap05`, `mmap06`, `munmap01`, `mprotect01`, `mprotect02`, `mmap10_1`, `mmap12`, `mmap13`, `mmap14`, `vma01`, `vma02` | reserve/repair | only after mapping/protection/VMA behavior is understood and targeted clean |
| fs-suite | `fs_perms01`-`fs_perms06`, `stream02`, `openfile01`, `writetest01`, `fs_inod01`, `inode02`, `ftest06`, `ftest09`, `rwtest01`, `rwtest02`, `iogen01` | scout/fill | RV scout then LA confirm; beware old TFAIL/TBROK/ENOSYS/timeout |

## Known no-promote rows at bootstrap

- `kill02`: targeted clean was invalidated by LA aggregate TBROK/setup-timeout in stable460 attempt.
- `readlinkat02`: LA musl TFAIL in stable460 LA confirm.
- `pipe02`: historical RV musl panic/trap/self-deadlock risk; do not first-batch.
- `select01`-`select04`, `pselect01`: pass-with-TCONF/semantic risk; not clean.
- `getcwd04`: TCONF cannot count as clean.

## Fresh evidence log

Append every targeted batch here with: cases, command, raw/summary path, RV/LA by libc result, internal failures, timeout/ENOSYS/panic/trap, promotion decision.
