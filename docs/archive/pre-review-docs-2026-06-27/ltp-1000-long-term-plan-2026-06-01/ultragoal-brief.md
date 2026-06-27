# Ultragoal brief: stable506 -> 1000 trustworthy LTP cases

目标：在 `/root/oskernel2026-orays` 分支 `dev/1000ltp-plan` 上，把 live `LTP_STABLE_CASES` 从当前基线推进到至少 1000 个 unique case，同时提高内核 Linux/POSIX 语义健壮性和可维护性。

约束：

- 不 fake PASS；不 hardcode LTP case/path/process/output；不修改 testsuite/evaluator 绕过失败。
- blacklist/SKIP/status0/full-sweep 局部 TPASS 不算 promotion evidence。
- Promotion 必须 RV + LA × musl + glibc parser-backed clean；无新增 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap。
- Leader owns `.omx/ultragoal`、stable list、milestone gate、final report；Team workers 只做窄 lane discovery/fix/verification/report。
- 每新增 50 个可信 unique stable cases 创建一个独立 milestone commit；从 live baseline 动态计算，当前 506 对应 556/606/.../956/1000。
- 每个 milestone 在 `docs/ltp-1000-long-term-plan-2026-06-01/milestone-XX-stableNNN/` 写完整证据文档。

阶段：

1. 基线冻结、候选 backlog、风险登记、回归矩阵。
2. 低风险 VFS/metadata/FD/time/process 候选推进到首个 50-case milestone。
3. mm/mmap/resource、futex/thread/IPC、network/proc/syntheticfs 分 lane 修复和推广。
4. LA severe blocker 削减与 full-sweep/shard sweep 质量闭合。
5. stable1000 final gate、robustness review、post-1000 roadmap。
