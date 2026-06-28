# Candidate Matrix: stable460 -> 500+

Session 1 只读/报告产物；本矩阵把 rv-arch002 / la-arch012 full-sweep raw logs 与 live stable460 做交叉。所有 `clean` 仅表示 sweep 中四路 wrapper PASS 且 parser 未见内部问题，仍需 fresh targeted RV/LA × musl/glibc gate 才能 promotion。

## Baseline facts

- live stable inventory: `460 total / 460 unique / 0 duplicate` from `examples/shell/src/cmd.rs`.
- promotion-candidate parser across raw rv/la logs: `563` clean four-combo cases, `1768` blocked/incomplete cases.
- clean but not currently in stable460: `106` cases.
- raw log sha256: rv `70a3f9cab0c5c7a9a2743f168cabfd2eaafb7c01565b1630a02f9604aca5f096`, la `41a5fdbba4a56a4ea76a168d2c9c6aa1e86a572d09de734a43a5365ec52c84df`.

## Clean-not-stable lane counts

| Lane | Count | Examples |
| --- | ---: | --- |
| other/low-priority or harness | 48 | `fanotify_child`, `genacos`, `genasin`, `genatan`, `genceil`, `gencos`, `gencosh`, `genexp`, `genfabs`, `genfloor`, `genfmod`, `genj0` |
| futex/process/IPC | 21 | `futex_wait02`, `futex_wait04`, `futex_wake01`, `kill02`, `sched_tc2`, `sched_tc3`, `sched_tc4`, `sched_tc5`, `shmdt02`, `shmnstest`, `shmt02`, `shmt03` |
| mmap/mm/resource | 18 | `data_space`, `diotest1`, `diotest2`, `diotest3`, `diotest5`, `diotest6`, `dirty`, `mlockall01`, `mmap-corruption01`, `mmap001`, `mmap15`, `mmap17` |
| time/select/signal | 8 | `clock_gettime04`, `clock_nanosleep02`, `nanosleep01`, `poll02`, `pselect01`, `pselect01_64`, `settimeofday01`, `time-schedule` |
| VFS/metadata/path | 7 | `fpathconf01`, `fs_perms`, `mknod08`, `mknodat01`, `pathconf01`, `readdir01`, `rename14` |
| network/proc/synthetic | 4 | `accept01`, `listen01`, `socket02`, `socketpair02` |

## First 40 targeted cases

| Case | Lane | Sweep state | RV/LA status | Max runtime ms | Min free-frame delta | Recommended action |
| --- | --- | --- | --- | ---: | ---: | --- |
| `getitimer01` | time/select/signal | 0/4 clean; blocked/failing; TFAIL×4, ENOSYS×4, event-failures×4, status×4 | rv: musl:FAIL (TFAIL=16/ENOSYS=6/event-failures=1); glibc:FAIL (TFAIL=16/ENOSYS=6/event-failures=1)<br>la: musl:FAIL (TFAIL=16/ENOSYS=6/event-failures=1); glibc:FAIL (TFAIL=16/ENOSYS=6/event-failures=1) | 1952 | -21 | Session 2 fix/classify time/select/signal; do not promote until four-combo clean. |
| `ppoll01` | time/select/signal | 0/4 clean; blocked/failing; TFAIL×4, event-failures×4, status×4 | rv: musl:FAIL (TFAIL=4/event-failures=1/status=FAIL); glibc:FAIL (TFAIL=4/event-failures=1/status=FAIL)<br>la: musl:FAIL (TFAIL=4/event-failures=1/status=FAIL); glibc:FAIL (TFAIL=4/event-failures=1/status=FAIL) | 6454 | -84 | Session 2 fix/classify time/select/signal; do not promote until four-combo clean. |
| `select02` | time/select/signal | 0/4 clean; blocked/failing; TCONF×4, timeout×4, event-failures×4, status×4 | rv: musl:FAIL (TCONF=1/timeout=1/event-failures=1); glibc:FAIL (TCONF=1/timeout=1/event-failures=1)<br>la: musl:FAIL (TCONF=1/timeout=1/event-failures=1); glibc:FAIL (TCONF=1/timeout=1/event-failures=1) | 15825 | -96 | Session 2 fix/classify time/select/signal; do not promote until four-combo clean. |
| `diotest4` | mmap/mm/resource | 0/4 clean; blocked/failing; TFAIL×4, TCONF×4, event-failures×4, status×4 | rv: musl:FAIL (TFAIL=3/TCONF=2/event-failures=1); glibc:FAIL (TFAIL=3/TCONF=2/event-failures=1)<br>la: musl:FAIL (TFAIL=3/TCONF=2/event-failures=1); glibc:FAIL (TFAIL=3/TCONF=2/event-failures=1) | 2195 | -12 | Cross-lane blocker diagnosis; keep as targeted scout before lane-specific fix. |
| `execve05` | futex/process/IPC | 0/4 clean; blocked/failing; TBROK×4, event-failures×4, status×4 | rv: musl:FAIL (TBROK=1/event-failures=1/status=FAIL); glibc:FAIL (TBROK=1/event-failures=1/status=FAIL)<br>la: musl:FAIL (TBROK=1/event-failures=1/status=FAIL); glibc:FAIL (TBROK=1/event-failures=1/status=FAIL) | 11499 | -112 | Cross-lane blocker diagnosis; keep as targeted scout before lane-specific fix. |
| `readlinkat02` | VFS/metadata/path | 3/4 clean; blocked in 1/4; TFAIL×1, event-failures×1, status×1 | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:FAIL (TFAIL=1/event-failures=1/status=FAIL); glibc:PASS clean | 1819 | -21 | Cross-lane blocker diagnosis; keep as targeted scout before lane-specific fix. |
| `epoll_create02` | time/select/signal | 0/4 clean; blocked/failing; TCONF×4, TFAIL×1, ENOSYS×1, event-failures×1, status×1 | rv: musl:FAIL (TFAIL=2/TCONF=1/ENOSYS=2); glibc:PASS (TCONF=1)<br>la: musl:PASS (TCONF=1); glibc:PASS (TCONF=1) | 1962 | -30 | Cross-lane blocker diagnosis; keep as targeted scout before lane-specific fix. |
| `nice04` | futex/process/IPC | 3/4 clean; blocked in 1/4; TFAIL×1, event-failures×1, status×1 | rv: musl:FAIL (TFAIL=1/event-failures=1/status=FAIL); glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1976 | -21 | Cross-lane blocker diagnosis; keep as targeted scout before lane-specific fix. |
| `clone04` | futex/process/IPC | 3/4 clean; blocked in 1/4; TBROK×1, event-failures×1, status×1 | rv: musl:FAIL (TBROK=1/event-failures=1/status=FAIL); glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2085 | -21 | Cross-lane blocker diagnosis; keep as targeted scout before lane-specific fix. |
| `clock_gettime04` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 3442 | -40 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `clock_nanosleep02` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 11257 | -40 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `nanosleep01` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 11076 | -40 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `poll02` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 11084 | -40 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `pselect01` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 10800 | -40 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `pselect01_64` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 11137 | -40 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `settimeofday01` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1978 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `time-schedule` | time/select/signal | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 3118 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `fpathconf01` | VFS/metadata/path | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2053 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `pathconf01` | VFS/metadata/path | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1982 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `rename14` | VFS/metadata/path | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 7146 | -30 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `mknod08` | VFS/metadata/path | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1757 | -12 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `mknodat01` | VFS/metadata/path | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1987 | -12 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `diotest1` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2033 | -12 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `diotest2` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2375 | -12 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `diotest3` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2246 | -39 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `diotest5` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2244 | -12 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `diotest6` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2852 | -39 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `mprotect05` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1945 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `mmap001` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1908 | -14 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `mmap15` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2040 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `mmap17` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2021 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `mmap19` | mmap/mm/resource | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2051 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `futex_wait02` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 3416 | -30 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `futex_wait04` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2096 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `futex_wake01` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 2020 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `kill02` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 12126 | -48 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `tkill01` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1899 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `tkill02` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1916 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `vfork01` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1953 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |
| `vfork02` | futex/process/IPC | 4/4 clean wrapper PASS (sweep evidence only); none | rv: musl:PASS clean; glibc:PASS clean<br>la: musl:PASS clean; glibc:PASS clean | 1910 | -21 | Fresh targeted RV/LA gate candidate; if clean, consider later promotion batch. |

## Blocked/failing family highlights for later sessions

| Case | Lane | Sweep state | Main blockers | Recommended session/action |
| --- | --- | --- | --- | --- |
| `fcntl11` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `fcntl14` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `fcntl19` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `fcntl22` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `fcntl30` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `pipe07` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `pipe15` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `writev03` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TCONF×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `pwritev03` | FD/fcntl/pipe/ownership | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 3 FD/fcntl/pipe/ownership: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `statx01` | VFS/metadata/path | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `statx03` | VFS/metadata/path | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `statx04` | VFS/metadata/path | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `getxattr01` | VFS/metadata/path | 0/4 clean; blocked/failing | TBROK×4, ENOSYS×4, event-failures×4, status×4 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `listxattr01` | VFS/metadata/path | 0/4 clean; blocked/failing | TBROK×4, ENOSYS×4, event-failures×4, status×4 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `statfs01` | VFS/metadata/path | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `getdents01` | VFS/metadata/path | 0/4 clean; blocked/failing | TFAIL×4, TCONF×4, event-failures×4, status×4, ENOSYS×3 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `getdents02` | VFS/metadata/path | 0/4 clean; blocked/failing | TCONF×4, ENOSYS×3 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `readlink03` | VFS/metadata/path | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 4 VFS/metadata/path: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `select01` | time/select/signal | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 2 time/select/signal: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `select03` | time/select/signal | 0/4 clean; blocked/failing | TCONF×4 | Session 2 time/select/signal: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `getitimer02` | time/select/signal | 0/4 clean; blocked/failing | TCONF×4, event-failures×4, status×4 | Session 2 time/select/signal: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `epoll_wait01` | time/select/signal | 0/4 clean; blocked/failing | TBROK×4, ENOSYS×4, event-failures×4, status×4 | Session 2 time/select/signal: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `futex_wait03` | futex/process/IPC | 0/4 clean; blocked/failing | timeout×4, event-failures×4, status×4 | Session 6 futex/process/IPC: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `futex_wait05` | futex/process/IPC | 0/2 clean; blocked/failing | TFAIL×2, timeout×2, event-failures×2, status×2 | Session 6 futex/process/IPC: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `waitid07` | futex/process/IPC | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 6 futex/process/IPC: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `clone02` | futex/process/IPC | 0/4 clean; blocked/failing | TFAIL×4, ENOSYS×4, event-failures×4, status×4 | Session 6 futex/process/IPC: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `execve01` | futex/process/IPC | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 6 futex/process/IPC: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `mincore01` | mmap/mm/resource | 0/4 clean; blocked/failing | TFAIL×4, ENOSYS×4, event-failures×4, status×4 | Session 5 mmap/mm/resource: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `mprotect01` | mmap/mm/resource | 0/4 clean; blocked/failing | TFAIL×4, event-failures×4, status×4 | Session 5 mmap/mm/resource: reproduce with targeted run; fix only real POSIX/Linux semantics. |
| `mprotect02` | mmap/mm/resource | 0/4 clean; blocked/failing | TBROK×4, event-failures×4, status×4 | Session 5 mmap/mm/resource: reproduce with targeted run; fix only real POSIX/Linux semantics. |

## Evidence boundaries

- `clean-not-stable` comes from full-sweep logs only; it is scouting evidence, not stable promotion evidence.
- Cases with `TCONF`, `TBROK`, `TFAIL`, `ENOSYS`, timeout, or missing arch/libc remain blocked even if wrapper status is 0 in some combos.
- No blacklist change and no stable list change in Session 1.
