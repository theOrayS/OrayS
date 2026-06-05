# stable956 promotion candidates

Stable956 is promoted from stable906 by adding 50 unique LTP cases. The promoted set is exactly the list in `targeted-cases.txt`.

## Final promotion gate

- RV final new50 gate: `target/ltp-1000-milestone-09-stable956/rv-stable956-new50-final-gate-20260605T222350+0800.summary.txt`
  - RUN_RC=0; PASS LTP CASE 100; FAIL 0; TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.
  - musl: 50 passed / 0 failed; glibc: 50 passed / 0 failed.
- LA final new50 gate: `target/ltp-1000-milestone-09-stable956/la-stable956-new50-final-gate-20260605T222730+0800.summary.txt`
  - RUN_RC=0; PASS LTP CASE 100; FAIL 0; TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.
  - musl: 50 passed / 0 failed; glibc: 50 passed / 0 failed.

## Promotion groups

- FD/fcntl: `fcntl37`, `fcntl37_64`.
- Process/exec: `execve01`, `execve05`, `execve06`, `execl01`, `execle01`, `execlp01`, `execv01`, `execvp01`.
- SysV message queues: `msgrcv08`, `msgctl06`, `msgctl12`, `msgrcv05`, `msgrcv06`, `msgrcv07`, `msgsnd05`, `msgsnd06`.
- VFS/openat2: `openat201`, `openat202`, `openat203`.
- SysV semaphores: `semctl01`, `semctl02`, `semctl04`, `semctl05`, `semctl06`, `semctl07`, `semctl09`, `semget01`, `semget02`, `semget05`, `semop01`, `semop03`, `semop04`.
- Signal wait: `sigwait01`.
- POSIX mqueue: `mq_open01`, `mq_unlink01`, `mq_timedsend01`, `mq_timedreceive01`, `mq_notify02`.
- pidfd: `pidfd_open01`, `pidfd_open02`, `pidfd_open03`, `pidfd_open04`, `pidfd_send_signal01`, `pidfd_send_signal02`, `pidfd_getfd01`, `pidfd_getfd02`.
- inotify init flags: `inotify_init1_01`, `inotify_init1_02`.

## Non-promotion evidence boundary

Rows with visible TCONF/TBROK/TFAIL/timeout/ENOSYS, missing testcase markers, partial arch/libc coverage, or outer RUN_RC=124 diagnostic logs were treated as blockers only. They were not counted toward stable956.
