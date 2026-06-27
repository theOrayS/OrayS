# stable956 regression matrix

| Lane | Protected new cases | Regression/evidence boundary |
| --- | --- | --- |
| FD/fcntl | `fcntl37`, `fcntl37_64` | RV/LA final new50 gate plus earlier adjacent pipe/fcntl stable regression reports. |
| exec/process | `execve01`, `execve05`, `execve06`, `execl01`, `execle01`, `execlp01`, `execv01`, `execvp01` | RV/LA final new50 gate plus earlier exec adjacent stable regression reports. |
| SysV msg | `msgrcv08`, `msgctl06`, `msgctl12`, `msgrcv05`, `msgrcv06`, `msgrcv07`, `msgsnd05`, `msgsnd06` | RV/LA final new50 gate plus earlier SysV msg adjacent regression reports. |
| openat2/VFS | `openat201`, `openat202`, `openat203` | RV/LA final new50 gate plus earlier VFS/openat2 adjacent stable regression. |
| SysV sem | `semctl01`, `semctl02`, `semctl04`, `semctl05`, `semctl06`, `semctl07`, `semctl09`, `semget01`, `semget02`, `semget05`, `semop01`, `semop03`, `semop04` | RV/LA final new50 gate and earlier clean12/zombie proc-stat follow-ups. |
| signal wait | `sigwait01` | RV/LA final new50 gate plus earlier adjacent signal stable regression. |
| POSIX mqueue | `mq_open01`, `mq_unlink01`, `mq_timedsend01`, `mq_timedreceive01`, `mq_notify02` | RV/LA final new50 gate plus focused mqueue targeted gates; `mq_notify01/03` remain excluded. |
| pidfd | `pidfd_open01..04`, `pidfd_send_signal01/02`, `pidfd_getfd01/02` | RV/LA final new50 gate plus focused pidfd/pidfd_getfd gates; `pidfd_send_signal03` remains excluded. |
| inotify init flags | `inotify_init1_01`, `inotify_init1_02` | RV/LA final new50 gate plus focused init1 gates; event-delivery cases remain excluded. |

Final cross-lane regression signal: RV and LA new50 final gates both report PASS 100 / FAIL 0 with no parser blockers.
