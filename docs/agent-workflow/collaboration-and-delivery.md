# 协作、冻结期与交付

只在需要多 agent 协作、任务分级、冻结期、最终报告或 Git commit 时读取本文件。

## 任务分级

- P0：保分/提交阻塞。`make all` 失败、`kernel-rv`/`kernel-la` 缺失、RV/LA 启动失败、高分 case 大面积回退、远程明显不可用。立即缩小范围，优先 hotfix 或回滚自己引入的风险。
- P1：明确加分修复。有日志或分数表显示 LTP/syscall/fs/process/signal/mmap case 未通过且修复点较明确。先读 source/runtest/log，只修真实语义，targeted 后跑相邻回归。
- P2：高价值探索。没有直接失败日志但 ROI 高。先产出候选排序和预期收益；没有 targeted 证据前不得改 stable list。
- P3：清理/重构/风格。比赛阶段默认不做，除非直接解除 P0/P1 阻塞或用户明确要求。

## 推荐工作循环

```text
读取事实 -> 选择最小目标 -> 写补丁 -> targeted 验证 -> 相邻回归 -> 总结证据 -> commit/PR -> reviewer 复核 -> 人类合并
```

报告里说明依据：failing case/score gap/raw log、LTP source/runtest、相关 syscall/VFS/FD/task/signal/mmap/HAL/shell path、验证命令和结果。

## 多 agent 协作

Implementer agent：诊断、修改、targeted 验证、填写 PR/报告、列风险和未验证项。

Reviewer agent：不重写实现，除非发现明确问题；检查 diff 越界、fake pass、errno/ABI/user pointer/lock/lifetime/cfg、验证是否支撑 claim，并给出 `APPROVE` / `REQUEST_CHANGES` / `COMMENT`。

人类闸门只需看：任务是否 P0/P1/P2、diff 是否无污染、验证是否真实、是否违反 LTP 红线、是否有可接受回滚方案。

## 冻结期

用户可用一句话启用：

```text
进入冻结期：只接受 P0 和低风险 P1，所有 agent PR 必须写回滚方案。
```

冻结期禁止大重构、批量格式化、新增低置信 stable case、无完整验证的 vendor/toolchain/platform config 改动、shell runner 输出格式改动、跨多个子系统同时改动。

## 最终报告模板

```markdown
## Summary
一句话说明结果。

## Files changed
- `path`: why

## Behavior / ABI impact
- syscall / errno / ABI visible change:
- user-visible behavior change:

## Validation
- [x] command: result
- [ ] not run: reason

## Risk and rollback
- Risk:
- Rollback:
```

Evaluator-mode 改动还要说明 `./run-eval.sh rv`、`./run-eval.sh la`、`make all` 的状态。

## Lore commit 协议

完成并验证对 tracked source/docs/durable state 的改动后，默认自动 commit，除非用户明确禁止、验证仍失败，或 worktree 有无法安全分离的无关改动。只 stage agent-owned changes。

提交信息格式：

```text
<intent line: why the change was made, not what changed>

<optional concise body: constraints and rationale>

Constraint: <external constraint that shaped the decision>
Rejected: <alternative considered> | <reason>
Confidence: <low|medium|high>
Scope-risk: <narrow|moderate|broad>
Directive: <future warning>
Tested: <what was verified>
Not-tested: <known gaps>
```

规则：intent line 写 why；trailers 只在提供决策上下文时使用；`Rejected:` 记录未来不该重复探索的替代方案；`Directive:` 写前向警告；`Not-tested:` 写已知验证缺口。

## PR 标题建议

```text
<subsystem>: <short scoring-oriented change>
```

示例：`posix: fix openat errno for invalid flags`、`fs: preserve fd offset semantics across dup`、`ltp: promote verified access regression cases`、`la: keep remote boot address map in submission build`。
