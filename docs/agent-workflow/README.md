# Agent Workflow 索引

本目录保存根 `AGENTS.md` 下沉出来的低频规则。目标是节省上下文：
agent 每次只读取与当前任务相关的短文档，而不是把所有比赛、LTP、远程构建、
提交模板和协作规则一次性塞进上下文。

优先级：当前源码/`Makefile`/脚本/实际评测结果 > 根 `AGENTS.md` > 本目录专题文档 > 其他说明文档。

## 如何使用

1. 先读根 `AGENTS.md`。
2. 根据任务类型只读下表中的必要文件。
3. 如果任务跨多个领域，再追加读取对应专题。
4. 若文档与当前仓库事实冲突，以当前文件和实际命令输出为准，并在报告中说明。

## 专题目录

| 文件 | 何时读取 | 内容 |
| --- | --- | --- |
| `repo-basics.md` | 开始修改仓库、清理、提交前 | 仓库布局、生成物、磁盘和 worktree 卫生 |
| `commands-and-validation.md` | 需要构建、测试、CI 或选择验证范围 | 常用命令、工具链、最小验证矩阵 |
| `local-remote-eval.md` | 涉及本地/远程 evaluator、LA/RV 启动、`make all` | 本地 QEMU 与远程提交配置差异 |
| `ltp-selection.md` | 选择下一批 LTP/score 测例或做 ROI 排序 | 候选选择、证据要求、优先级模型 |
| `ltp-promotion-and-docs.md` | 修改 `LTP_STABLE_CASES` 或写 LTP 阶段文档 | 推广闸门、报告字段、文档命名 |
| `coding-boundaries.md` | 修改 Rust、POSIX ABI、高风险子系统 | 最小补丁、unsafe、用户指针、子系统边界 |
| `branch-policy.md` | 创建、切换、重命名或合并分支 | `main`/`dev`/`score/best`/`release`/`feat`/`fix`/`exp` 规则 |
| `collaboration-and-delivery.md` | 多 agent、冻结期、交付、自动提交 | 工作循环、review、模板、Lore commit |

## 最短规则

- 不 fake pass，不硬编码 testcase，不隐藏失败证据。
- 不碰无关生成物、镜像、日志、批量格式化。
- 修改 syscall/errno/ABI/POSIX 可见行为必须明说。
- 选择测例时实时读取当前 score/log/source，不凭记忆。
- 交付时只声称实际验证过的内容。
