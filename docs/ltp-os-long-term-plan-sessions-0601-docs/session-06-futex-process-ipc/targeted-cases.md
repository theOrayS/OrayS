# Session 6 targeted cases

单位：LTP case。以下 21 个 case 是本 session fresh RV/LA targeted gate 覆盖的候选。

## Clean21 promotion gate cases

```text
futex_wait02
futex_wait04
futex_wake01
kill02
sched_tc2
sched_tc3
sched_tc4
sched_tc5
shmdt02
shmem_2nstest
shmnstest
shmt02
shmt03
shmt06
shmt07
shmt08
shmt10
tkill01
tkill02
vfork01
vfork02
```

## Known blockers not rerun/promoted in this session

```text
futex_wait03
futex_wait05
waitid07
clone02
clone04
execve01
execve05
nice04
```

这些 blocker 在 Session 1 矩阵中已有非 clean 证据；本 session 不把它们从报告状态提升为 stable。
