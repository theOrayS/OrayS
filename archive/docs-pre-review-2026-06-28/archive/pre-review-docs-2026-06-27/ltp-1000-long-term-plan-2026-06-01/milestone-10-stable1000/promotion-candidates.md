# stable1000 promotion candidates

Stable1000 is promoted from stable956 by adding 44 unique LTP cases. The promoted set is exactly the first list in `targeted-cases.txt`.

## Final promotion gate

- RV final new44 gate: `target/ltp-1000-milestone-10-stable1000/rv-new44-final-current-vforkshare-20260606T124648+0800/rv-summary.txt`
  - RUN_RC=0; PASS LTP CASE 88; FAIL 0; TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.
  - musl: 44 passed / 0 failed; glibc: 44 passed / 0 failed.
- LA final new44 gate: `target/ltp-1000-milestone-10-stable1000/la-new44-final-current-vforkshare-20260606T125210+0800/la-summary.txt`
  - RUN_RC=0; PASS LTP CASE 88; FAIL 0; TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.
  - musl: 44 passed / 0 failed; glibc: 44 passed / 0 failed.

## Promotion groups

- vfork/clone/exec: `clone05`, `creat07`, `execve02`, `execve03`, `execve04`, `getrusage03`, `getrusage04`.
- close/fcntl/FD: `close_range01`, `close_range02`, `fcntl17`, `fcntl17_64`, `fcntl34`, `fcntl34_64`, `fcntl36`, `fcntl36_64`, `pipeio`.
- mmap/resource/stress: `madvise07`, `madvise10`, `mmap18`, `mmapstress01`, `mmapstress02`, `mmapstress03`, `mmapstress05`, `sbrk01`.
- IPC/thread namespaces: `mesgq_nstest`, `sem_nstest`, `semtest_2ns`.
- VFS/mount/path/pipe splice family: `crash02`, `dirtypipe`, `doio`, `ebizzy`, `kcmp01`, `kcmp02`, `kill10`, `mount07`, `realpath01`, `sendmsg02`, `splice06`, `tee01`, `tee02`, `vmsplice01`, `vmsplice02`, `vmsplice03`, `vmsplice04`.

## Non-promotion evidence boundary

Rows with visible TCONF/TBROK/TFAIL/timeout/ENOSYS, missing testcase markers, partial arch/libc coverage, or diagnostic pre-fix failures were treated as blockers only. They were not counted toward stable1000.
