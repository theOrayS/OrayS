# stable906 regression matrix

| Lane | Regression/candidate set | Evidence | Result |
| --- | --- | --- | --- |
| New stable906 candidates | 50 cases in `targeted-cases.txt` | `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/stable906-new50-rvla-final-gate-20260605T115135+0800.txt` | RV/LA x musl/glibc clean, 50 candidates, 0 blocked |
| SysV msg implementation | msgget01, msgget02, msgsnd01, msgsnd02, msgrcv01, msgrcv02, msgctl01, msgctl02, msgctl03 | `target/ltp-1000-milestone-08-stable906/rv-sysv-msg-fix-20260605T113226+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/la-sysv-msg-fix-20260605T113700+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/sysv-msg-rvla-fix-20260605T113700+0800.txt` | RV/LA x musl/glibc clean, 9 candidates |
| Adjacent stable SysV shm | shmget02, shmget03, shmget04, shmat02, shmat03, shmdt01, shmctl03, shmctl04, shmctl07, shmctl08, shmt05 | `target/ltp-1000-milestone-08-stable906/rv-sysv-ipc-adjacent-stable-regression-20260605T120125+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/la-sysv-ipc-adjacent-stable-regression-20260605T120551+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/sysv-ipc-adjacent-stable-regression-rvla-20260605T120551+0800.txt` | RV/LA x musl/glibc clean, no regression |
| Memory lock/smaps | mlock02, mlock05, mlock202, mlock203, mlockall02, mlockall03, munlockall01 | Final new50 gate plus lane reports under `target/ltp-1000-milestone-08-stable906/` | Clean in final RV/LA new50 gate |
| VFS/FD/time/socket/process | chroot*, fallocate*, ftest06, getcwd03, gethostname02, readlink*, nanosleep02, nice04, setgroups04, setsockopt04, settimeofday02, sockioctl01, timer_settime03 | Final new50 gate | Clean in final RV/LA new50 gate |

Known caveat: this matrix is targeted. It is not a full stable906 all-case sweep.
