# 分支命名与用途

只在需要创建、切换、重命名、合并分支，或判断某个分支是否可作为提交/冲分基线时读取本文件。

## 当前分支角色

- `main`：稳定主线；始终应可编译、可启动、可跑基础测试。
- `dev`：集成分支；可选，多人并行或多条修复线汇合时使用。
- `score/best`：当前最高分版本；可用 branch 或 tag 表示。本 checkout 的原 `refactor/moss_kernel_like` 已改名为 `score/best`。
- `release/stage-N`：每个比赛阶段或正式提交节点的冻结分支。
- `feat/<subsystem>-xxx`：功能开发分支。
- `fix/<subsystem>-xxx`：bug 修复分支。
- `exp/<name>-xxx`：高风险实验分支，默认不合进 `main`。

## 使用规则

- `score/best` 代表当前已知最高分、最值得保护的版本；不要把未验证实验直接合进去。
- `main` 以稳定为目标；从 `score/best` 或 release 分支吸收改动前，需要有构建/启动/基础测试证据。
- `dev` 只在多人并行时作为集成缓冲；单人、小修复可以直接从目标基线开 `fix/` 或 `feat/`。
- `release/stage-N` 创建后按冻结分支处理：只接受 P0 和低风险 P1，所有改动要有回滚方案。
- `exp/` 是探索隔离区；除非后续经过 targeted 验证、回归验证和人工/leader 决策，否则默认不合进 `main` 或 `score/best`。

## 命名示例

```text
feat/vfs-openat-flags
feat/mm-mprotect-basic
fix/posix-statx-errno
fix/pipe-sigpipe-teardown
exp/scheduler-timeslice-ltp
release/stage-1
score/best
```

## Agent 操作约束

- 改分支名或创建长期分支前，先看 `git branch --show-current`、`git status --short`。
- 不要删除历史分支或远程分支，除非用户明确要求。
- 不要把 untracked 生成物、镜像、日志作为分支迁移的一部分提交。
- 如果当前最高分证据来自远程平台，记录证据路径或 tag；不要只靠口头记忆更新 `score/best`。
- 合并到 `score/best` 或 `main` 前，最终报告必须列出实际验证和未验证项。
