# Session 6 promotion candidates

## Promoted to stable

以下 21 个 case 已加入 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`：

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

推广依据：

- RV clean21 gate：`PASS LTP CASE 42`、`FAIL 0`、internal `{}`、timeout/ENOSYS/panic/trap 均为 0。
- LA clean21 gate：`PASS LTP CASE 42`、`FAIL 0`、internal `{}`、timeout/ENOSYS/panic/trap 均为 0。
- live stable count：`506 total / 506 unique / 0 duplicate`。
- 这些 case 的通过来自现有 futex wait/wake、signal kill/tkill、scheduler/vfork、SysV shm 真实语义的 fresh targeted gate；没有依赖 blacklist/SKIP/status0。

## Explicitly not promoted

- `futex_wait03`：Session 1 矩阵中 0/4 clean，timeout/status blocker；本 session 未修复也未重新推广。
- `futex_wait05`：Session 1 矩阵中 0/2 clean，TFAIL/timeout blocker；本 session 未修复也未重新推广。
- `waitid07`：Session 1 矩阵中 0/4 clean，TFAIL blocker；wait/zombie/reparent 仍需后续真实修复。
- `clone02`：Session 1 矩阵中 0/4 clean，ENOSYS/TFAIL blocker；未推广。
- `clone04`、`nice04`：Session 1 矩阵中 3/4 clean 但非四路 clean；未推广。
- `execve01`、`execve05`：Session 1 矩阵中 TBROK blocker；未推广。

## Stable-list boundary

Session 6 不改 blacklist，不把普通 FAIL 转成 blacklist，也不把 sweep/status0/部分架构结果当作 promotion 证据。
