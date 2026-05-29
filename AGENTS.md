# Agents Guidelines for ArceOS / OSKernel 2026

本文件是本仓库 agent 的高频入口和目录。它故意保持短小：先读本文件，
再按任务类型读取 `docs/agent-workflow/` 中的专题文档；不要在每次请求中
通读所有专题规则。若本文档、专题文档与当前源码/`Makefile`/脚本/实际评测
结果冲突，以当前仓库事实为准。

## 高频硬约束

- 工作根目录默认是 `/root/oskernel2026-orays`；除非任务明确跨仓库，否则不要在外层目录改动。
- 假设 worktree 可能已有他人改动；只修改、暂存、提交自己负责的文件，不要 revert 无关改动。
- 只做任务所需的最小改动；不要顺手大重构、批量格式化、机械重命名或跨子系统清理。
- 不要编辑生成物或本地证据文件，除非任务明确要求：`kernel-rv`、`kernel-la`、`sdcard-*.img`、`disk*.img`、`output*.md`、`*.log`、`.axconfig.toml`、`build/`、`target/`。
- 不允许 fake pass：不得硬编码 LTP case 名、路径、进程名或输出；不得伪造 `TPASS`/wrapper PASS；不得修改 testsuite 或 evaluator 脚本来绕过真实失败；不得隐藏 `TCONF`、timeout、`ENOSYS`、panic/trap。
- 实验分支可用 blacklist 策略探索全量 LTP，但 blacklist 只用于隔离会卡死、炸内存、破坏评测器或明显不适合当前内核模型的用例；被 blacklist 的 case 不能计为通过，也不能作为 stable/promotion 证据。
- POSIX/Linux 可见语义必须真实：syscall、errno、flag、struct layout、FD、signal、futex、mmap、用户指针 copy-in/copy-out 的变化都要显式说明。
- 新增依赖、修改 `vendor/`/`cargo-home/`/`tools/bin/`、远程提交配置或架构启动路径时，必须有明确任务理由和对应验证。
- 运行长构建、QEMU、Docker、vendoring、完整 evaluator 前后，检查 `df -h / /root`；涉及 Codex/OMX 缓存清理时再看 `du -sh /root/.codex`。
- 完成并验证对源码、文档或持久项目状态的改动后，默认自动创建 Git commit；只暂存 agent 自己的改动。提交信息规则见 `docs/agent-workflow/collaboration-and-delivery.md`。

## 按需阅读目录

| 任务类型 | 读取文件 |
| --- | --- |
| 仓库布局、生成物、工作树/磁盘卫生 | `docs/agent-workflow/repo-basics.md` |
| 构建命令、工具链、CI、选择验证范围 | `docs/agent-workflow/commands-and-validation.md` |
| 本地 QEMU 与远程提交 `kernel-rv`/`kernel-la` 差异 | `docs/agent-workflow/local-remote-eval.md` |
| 选择下一批 LTP/score 测例、估算 ROI | `docs/agent-workflow/ltp-selection.md` |
| 实验性全量 LTP / blacklist sweep | `docs/agent-workflow/commands-and-validation.md`、`docs/agent-workflow/ltp-selection.md`、`docs/agent-workflow/ltp-promotion-and-docs.md`、`docs/agent-workflow/branch-policy.md` |
| 推广 `LTP_STABLE_CASES`、LTP 报告、阶段文档命名 | `docs/agent-workflow/ltp-promotion-and-docs.md` |
| Rust/POSIX/ABI/高风险子系统修改边界 | `docs/agent-workflow/coding-boundaries.md` |
| 分支命名、`score/best`、release/fix/feat/exp 用途 | `docs/agent-workflow/branch-policy.md` |
| 多 agent 协作、冻结期、交付模板、Lore commit | `docs/agent-workflow/collaboration-and-delivery.md` |
| 更完整的开发者背景材料 | `docs/developer-guide/README.md` |

示例：如果任务是“决定下一步跑哪些测例”，只需再读
`ltp-selection.md` 和必要的当前 score/log/source；不要重读远程启动、提交模板、
Rust 风格等无关专题。如果任务是普通文档修改，不需要读取 LTP 选择规则。

## 常用事实入口

- 当前 stable LTP case 数量：实时读取 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`，不要凭记忆。
- 实验性全量 LTP 入口：当前分支可用 `LTP_CASES=blacklist` / `all-minus-blacklist` / `sweep:blacklist` 从 guest LTP bin 目录枚举全量 case，再扣除默认 blacklist、build-time `LTP_BLACKLIST`、`/ltp_blacklist.txt`、`/tmp/ltp_blacklist.txt`；报告必须列出选择模式、blacklist 来源、跳过数和未闭合 case。
- LTP 结果真相：优先用 `scripts/ltp_summary.py` 汇总，不要只凭 wrapper 输出肉眼判断。
- 构建/评测入口：`Makefile`、`run-eval.sh`、`configs/remote-eval/`。
- 主要路径：`kernel/` runtime/subsystems；`api/arceos_posix_api/` POSIX 边界；`ulib/` 用户库；`examples/shell/` evaluator 集成；`configs/` 平台配置；`scripts/`/`tools/` 构建辅助；`docs/` 进度和报告；`vendor/`/`cargo-home/` 离线依赖。

## 最小执行循环

1. 读取本文件和任务相关专题文档。
2. 从当前源码、脚本、日志或 LTP source 建立事实；不要用过期记忆替代。
3. 选择一个最小目标并保持补丁局部。
4. 运行能证明 claim 的最小验证；失败就修或说明阻塞。
5. 最终报告列出：改动文件、意图、实际验证、未验证项、用户可见行为变化、syscall/errno/ABI 影响。
6. 若改动已验证且可安全分离，按 Lore 协议提交并报告 commit SHA。
