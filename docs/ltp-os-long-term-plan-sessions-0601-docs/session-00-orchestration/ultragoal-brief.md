# Ultragoal Brief：2026-06-01 LTP/OS 长期完善计划

目标：按 `docs/ltp-os-long-term-improvement-plan-2026-06-01.md` 连续推进 Session 1~8；若 Session 8 后未满足完成定义，再进入 Session 9~10。

Session 边界：
- Session 1：冻结 stable460 基线，基于 rv-arch002 / la-arch012 full-sweep summary 生成候选矩阵与第一批 20~40 个 targeted cases；不修改 stable list，不修改 blacklist。
- Session 2：time/select/signal 第一批，优先 `getitimer01`、`ppoll01`、`select02` 及相邻 poll/pselect/clock/nanosleep case。
- Session 3：FD/fcntl/pipe/ownership，修复或分类 fcntl/pipe/chown/access 类目标。
- Session 4：VFS/metadata/path，处理 stat/path/symlink/xattr/getdents 类目标。
- Session 5：mmap/mm/resource 第一批，减少 mmap/mm 失败并记录 LA 资源观测。
- Session 6：futex/process/IPC，处理 futex/wait/clone/SysV IPC 高频缺口。
- Session 7：LA severe-blocker 专项，降低 LA-only blacklist 或留下可复现阻塞报告。
- Session 8：整合、推广、最终 RV/LA × musl/glibc gate、final report 与下一轮 prompt。
- Optional Session 9~10：网络/proc/syntheticfs 或 full-sweep/shard sweep 质量审计。

门禁：
- promotion 只由 leader 修改 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`，并且必须有 RV + LA × musl + glibc parser-clean 证据；blacklist/SKIP/status0 不计 PASS。
- 每个 roadmap session 完成后独立 Git commit；只 stage 本 session 自己的源码、文档、summary，不 stage 既有无关 dirty/untracked 文件或大 raw log。
- 每个 session 文档必须放在 `docs/ltp-os-long-term-plan-sessions-0601-docs/session-XX-<slug>/`，至少包含 report、targeted cases、validation、promotion/no-promotion，代码改动另写 ABI/behavior impact，blacklist 改动另写 blacklist-change report。
- 长跑/QEMU/evaluator 前后运行 `df -h / /root`；所有 LTP 结果以 `scripts/ltp_summary.py` 或 JSON summary 为准。
- 若发现 stable460 回归，停止扩张并先修复或回滚本计划引入的回归。

提交规则：
- 每个 session 一个独立 Lore commit。
- Session 1 禁止 stable/blacklist 修改；后续 session 如无四路 clean 证据，只能报告候选或阻塞，不能推广。
