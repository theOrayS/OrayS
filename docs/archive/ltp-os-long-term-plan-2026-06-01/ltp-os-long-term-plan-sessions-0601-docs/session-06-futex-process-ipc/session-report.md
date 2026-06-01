# Session 6 report: futex/process/IPC

Commit SHA: to be recorded after this session commit is created.
Previous session commit: `1578a684` (Session 5 mmap/mm/resource).

## Goal

把 Session 1 中 futex/process/IPC lane 的 full-sweep clean 候选转成 fresh RV/LA × musl/glibc targeted promotion 证据，同时不掩盖仍失败的 futex/wait/clone blocker。

## Changes

- 未修改 futex/process/IPC 运行时代码；本 session 先用现有真实语义做 promotion gate。
- 基于当前 `sys_futex` 的 WAIT/WAKE、signal kill/tkill、vfork/clone 基础语义和 SysV shm 最小真实模型，fresh 复核 21 个候选。
- 将 21 个四路 clean case 加入 `LTP_STABLE_CASES`：`futex_wait02`、`futex_wait04`、`futex_wake01`、`kill02`、`sched_tc2`、`sched_tc3`、`sched_tc4`、`sched_tc5`、`shmdt02`、`shmem_2nstest`、`shmnstest`、`shmt02`、`shmt03`、`shmt06`、`shmt07`、`shmt08`、`shmt10`、`tkill01`、`tkill02`、`vfork01`、`vfork02`。
- 未修改 blacklist、testsuite 或 evaluator。

## Evidence summary

- live stable count after promotion: `506 total / 506 unique / 0 duplicate`。
- RV clean21 gate：`PASS LTP CASE 42`、`FAIL 0`、internal `{}`、timeout `0`、ENOSYS `0`、panic/trap `0`。
- LA clean21 gate：`PASS LTP CASE 42`、`FAIL 0`、internal `{}`、timeout `0`、ENOSYS `0`、panic/trap `0`。
- Build、parser summary 和 checksum 见 `validation.md`。

## Result

Session 6 is complete. Stable 从 485 推进到 506，达到主计划最低 stable500 目标；本 session 的新增项全部来自 fresh targeted RV/LA × musl/glibc parser-clean 证据，不依赖 blacklist/SKIP/status0。

## Risks / limitations

- 本 session 不是 futex/wait/clone 新代码修复；`futex_wait03`、`futex_wait05`、`waitid07`、`clone02`、`execve01` 等仍按 Session 1 blocker 分类保留，未推广。
- `shmt10` 的 free-frame delta 较大但 RV/LA case 均闭合，需在 Session 8 final stable gate 再观察资源趋势。
- 本 session 没有跑完整 stable506 四路 gate；完整门禁保留给 Session 8。

## Next session entry

Session 7 进入 LA severe-blocker 专项。建议优先从 LA-only blacklist 中选择 1~3 个资源/allocator/network blocker 做单 case 可终止复现；如不安全移除 blacklist，则输出可复现阻塞报告而不是 promotion。
