# OSKernel 2026 ArceOS 分支开发者指南

这组文档面向维护 OSKernel 2026 评测分支的开发者，目标是说明：代码应该从哪里改、改完应该如何验证、哪些捷径在这个比赛导向的仓库里是禁止的。

请把它和仓库根目录的 `README.md`、`AGENTS.md` 一起使用：

- `README.md`：仓库快速入口，说明如何构建和运行。
- `AGENTS.md`：面向 agent 和维护者的操作约束。
- `docs/agent-workflow/`：面向比赛冲分的 agent 优先工作规范，把诊断、实现、验证和 PR 交付尽量交给 agent。
- `docs/developer-guide/`：面向日常开发的代码地图、评测流程和验证规范。

## 推荐阅读顺序

1. [`../../AGENTS.md`](../../AGENTS.md)：理解 agent 修改仓库时必须遵守的硬约束。
2. [`../agent-workflow/README.md`](../agent-workflow/README.md)：理解比赛阶段的人机分工、agent 工作循环、PR/验证/冻结期规则。
3. [`repository-map.md`](repository-map.md)：理解主要目录、评测相关代码边界和常见修改入口。
4. [`build-and-eval.md`](build-and-eval.md)：在修改评测相关代码前，先理解本地 QEMU 评测和远程提交构建的区别。
5. [`ltp-workflow.md`](ltp-workflow.md)：按照诚实的 LTP 选例、targeted 运行、日志解析和推广流程开展工作。
6. [`validation-matrix.md`](validation-matrix.md)：根据修改类型选择最小但足够的验证命令。
7. [`logs/README.md`](logs/README.md)：阅读此前评测/LTP 日志的开发者索引和当前 stop-state。

## 核心开发原则

- 明确区分本地验证和远程提交构建。本地 RV/LA 评测使用 `./run-eval.sh`；远程提交产物来自 `make all`。
- 把 `examples/shell` 当作评测集成面，而不是普通 demo。这里的改动会影响官方测试组、LTP 执行、wrapper marker 和远程计分。
- 保持真实 Linux/POSIX 语义。禁止按 testcase 名称硬编码、伪造成功输出、隐藏 `TCONF`/timeout，或修改 testsuite 源码来制造通过。
- 优先用 targeted LTP batch 和 `scripts/ltp_summary.py` 解析结果，再进入完整门禁。外层 QEMU 或 `run-eval` 退出码为 0 不等于 LTP 干净通过。
- 保持补丁窄而可审查。修一个子系统时，避免无关重构、编辑生成物或批量格式化无关文件。

## 当前 live 评测事实

截至本文档更新时，`examples/shell/src/cmd.rs::LTP_STABLE_CASES` 包含 383 个不重复 case。runner 会对 `/musl` 和 `/glibc` 各执行一遍选中的列表，因此默认 stable 集合在每个架构上会产生 766 次 LTP case 执行。

当数量本身影响结论时，必须重新从源码读取；不要依赖历史阶段报告里的旧快照。
