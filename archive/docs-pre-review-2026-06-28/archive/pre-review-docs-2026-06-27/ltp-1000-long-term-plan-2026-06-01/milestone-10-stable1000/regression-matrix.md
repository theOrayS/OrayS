# stable1000 regression matrix

| Lane | Protected new cases | Regression/evidence boundary |
| --- | --- | --- |
| vfork/clone/exec | `clone05`, `creat07`, `execve02`, `execve03`, `execve04`, `getrusage03`, `getrusage04` | RV/LA new44 final gates plus RV/LA regression subsets covering `clone01/03/05/06/07`, `vfork01/02`, `fork01/03/05/10`, `execve01..06`, `creat01/07`. The post-review code also protects exec failure atomicity and vfork exec wakeup. |
| FD/close/fcntl/pipe | `close_range01`, `close_range02`, `fcntl17`, `fcntl17_64`, `fcntl34`, `fcntl34_64`, `fcntl36`, `fcntl36_64`, `pipeio` | RV/LA new44 final gates plus RV/LA regression subsets covering `close01/02/close_range01/02`, `pipe01/02/pipeio`, `fcntl17`, `fcntl36`. FD unshare alias semantics protect `CLOSE_RANGE_UNSHARE` + `CLONE_FILES`. |
| mmap/resource/stress | `madvise07`, `madvise10`, `mmap18`, `mmapstress01`, `mmapstress02`, `mmapstress03`, `mmapstress05`, `sbrk01` | RV/LA new44 final gates plus RV/LA regression subsets covering `mmap18` and getrusage pressure. |
| IPC/thread namespace | `mesgq_nstest`, `sem_nstest`, `semtest_2ns` | RV/LA new44 final gates; no blacklist or namespace partial PASS counted. |
| VFS/mount/path/socket/splice | `crash02`, `dirtypipe`, `doio`, `ebizzy`, `kcmp01`, `kcmp02`, `kill10`, `mount07`, `realpath01`, `sendmsg02`, `splice06`, `tee01`, `tee02`, `vmsplice01..04` | RV/LA new44 final gates; future full-stable sweep should re-cover broad interaction order. |

Final cross-lane regression signal: RV and LA new44 gates each report PASS 88 / FAIL 0; RV regression and LA stable-order regression gates each report PASS 60 / FAIL 0; all report zero parser blockers.

Excluded diagnostic boundary: the failed `la-regression-postreview-rerun60-20260606T141808+0800` run used an artificial immediate `pipe02 -> pipeio` order that is not the live stable-list order and is not counted as promotion evidence. The same 30-case LA regression set passed in stable order at `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800`.
