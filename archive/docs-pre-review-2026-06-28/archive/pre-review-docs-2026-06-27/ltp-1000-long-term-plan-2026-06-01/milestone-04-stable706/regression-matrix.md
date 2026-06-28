# stable706 regression matrix

## Protected existing stable baseline

- Live stable list before edit: `656 total / 656 unique / 0 duplicate`.
- Live stable list after edit: `706 total / 706 unique / 0 duplicate`.
- Final promoted-new50 gate: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=<50 promoted cases> LTP_CASE_TIMEOUT_SECS=45 ./run-eval.sh {rv,la}`.
- Adjacent already-stable regression subset: `creat08 getitimer01 getxattr01 listxattr01 open10 ppoll01 socket01 times03`.

## Regression evidence

| Set | RV evidence | LA evidence | Result |
| --- | --- | --- | --- |
| promoted-new50 | `rv-stable706-new50-final-gate-20260602T221318+0800.log` | `la-stable706-new50-final-gate-20260602T222043+0800.log` | each arch: 50 musl + 50 glibc PASS, parser-clean |
| adjacent stable8 | `rv-stable706-adjacent-regression8-20260602T222823+0800.log` | `la-stable706-adjacent-regression8-20260602T223016+0800.log` | each arch: 8 musl + 8 glibc PASS, parser-clean |
| full stable RV sweep | `rv-stable706-final-gate-20260602T205711+0800.log` | not used after RV remained dirty | disclosed blocker only; not promotion evidence |

## Lane-specific adjacent regression sets

- xattr/metadata: `setxattr01 fsetxattr01 getxattr01 listxattr01 fgetxattr01 flistxattr01 fremovexattr01 lgetxattr01 llistxattr01 lremovexattr01 removexattr01 fgetxattr03 flistxattr02 flistxattr03 fremovexattr02 lgetxattr02 listxattr02 listxattr03 llistxattr02 llistxattr03 removexattr02`.
- path/VFS/ownership: `statfs01_64 rename06 rename07 rename08 rename10 lchown01 lchown02 creat06 fsync01 llseek01`.
- fcntl/time/process/mm: `fcntl14_64 fcntl15_64 fcntl30_64 clock_adjtime01 clock_adjtime02 clock_getres01 pause03 munmap02 sigaltstack01 getcpu01`.
- socket/futex: `getsockname01 getpeername01 getsockopt01 setsockopt01 socketpair01 futex_wake03 set_robust_list01`.
- copy/syslog/proc syntheticfs: `copy_file_range01 copy_file_range03 syslog11 syslog12` plus `/proc/sys/kernel/printk` smoke through syslog tests.

## Non-regression boundaries

- Do not relax `setpriority02` protected EACCES behavior to chase `nice04`.
- Do not treat musl/glibc wrapper-specific TCONF as pass evidence.
- Do not broaden socket sockaddr validation beyond documented Linux-compatible len/user-pointer checks without rerunning socket errno/readiness subsets.
- `readahead01` remains excluded until parser-visible TCONF is removed by a real fd-family/subsystem fix.
- Full-stable blockers (`mmap10`, `mmap-corruption01`, `test_ioctl`, and any recurring long-sequence timeout/TFAIL rows) must be fixed or explicitly isolated before future final stable1000/full-suite claims.
