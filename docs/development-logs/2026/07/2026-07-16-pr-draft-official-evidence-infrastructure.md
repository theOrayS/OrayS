---
title: "PR draft: stabilize official evidence infrastructure"
date_started: 2026-07-16
date_completed: null
status: draft
pr: null
branch: "stabilize/post-integration-gates-20260716"
authors:
  - "Codex (OpenAI)"
reviewers: []
base_commit: "09f4076ac151e0e7800103de724d9042230738b5"
head_commit: "09f4076ac151e0e7800103de724d9042230738b5"
capability_domains:
  - "official-evidence"
  - "test-runner"
  - "result-parser"
  - "riscv64"
  - "loongarch64"
---

# 1. 背景与目标

## 背景

四分支集成候选的完整双架构运行已经实际完成全部 24 个官方组，QEMU
进程正常退出，也保留了真实语义失败；但 BusyBox 计划中的两个有意重复
命令仅以文本充当身份，导致解析器把正常的有序步骤误报为基础设施重复。
本任务只稳定证据基础设施，不处理这些官方运行揭示的内核语义失败。

## 目标

- 让计划、生产者、解析器和 runner 使用稳定、可追溯的有序 case 身份。
- 保证 `planned == executed == completed`，并继续对缺失、乱序、重放、未知组、
  畸形协议和不完整证据 fail-closed。
- 严格区分语义 `FAIL` 与基础设施 `ERROR`，绝不把失败改写成通过。
- 在干净提交上完成 quick、baseline 和新鲜 RV/LA official 验证，复核镜像与
  overlay 生命周期，经独立只读审查后只正常推送稳定化分支。

## 非目标

- 不开始 Goal B，不修复或掩盖官方测试揭示的语义失败。
- 不修改外部测试计划、官方镜像、`main`、依赖或工具链。
- 不按命令文本、测例名、路径、架构、libc 或评测环境特化。
- 不 rebase、squash、force-push 或改写既有集成历史。

## 验收标准

- [ ] 重复文本在不同稳定序号下可合法完成；相同显式 ID 必须拒绝。
- [ ] 重放、缺失、额外、乱序、畸形身份、未知组和不完整执行全部拒绝。
- [ ] 有真实测试失败但身份完整的官方结果为 `FAIL`，不是 `ERROR` 或 `PASS`。
- [ ] quick、baseline、RV official、LA official 满足 Goal A 的干净验证合同。
- [ ] 两个官方镜像哈希不变，临时 overlay 全部清理。
- [ ] 独立 reviewer 的 blocker/major finding 为零。
- [ ] 分支保持可追溯祖先关系并仅以普通 push 发布。

# 2. 基线

初始 Git 状态：

- 分支：`stabilize/post-integration-gates-20260716`。
- 初始 HEAD：`09f4076ac151e0e7800103de724d9042230738b5`。
- 本地基线、merge-base 与远端 `origin/integration/four-prs-20260715`：
  `09f4076ac151e0e7800103de724d9042230738b5`。
- 初始 worktree：clean。
- 远端尚无同名稳定化分支。

| 日期 | 命令 | 架构/目标 | 退出码 | 结果 | 备注/证据 |
|---|---|---|---:|---|---|
| 2026-07-16 | `git status --short --branch`、`git rev-parse HEAD`、merge-base 与远端 heads 检查 | Git provenance | 0 | PASS | 三个基线提交一致；工作树 clean |
| 2026-07-16 | `sha256sum` 两个官方镜像 | RV/LA image provenance | 0 | PASS | 哈希与 Goal A 合同一致，见下表 |
| 2026-07-16 | `python3 -B -m unittest test/unit/test_official_result_validation.py` | host unit baseline | 1 | ERROR | 调用形式错误；`test/` 不是 Python package，未执行产品测试 |
| 2026-07-16 | `python3 -B test/unit/test_official_result_validation.py` | host unit baseline | 0 | PASS | 106 tests，0.921 s |
| 2026-07-16 | 严格解析旧集成 RV stdout/stderr capture，process exit 0 | RISC-V64 replay | 2 | ERROR | 24 groups；119 semantic findings；仅 2 个 integrity findings，均为 `busybox-duplicate-case` |
| 2026-07-16 | 严格解析旧集成 LA stdout/stderr capture，process exit 0 | LoongArch64 replay | 2 | ERROR | 24 groups；161 semantic findings；仅 2 个 integrity findings，均为 `busybox-duplicate-case` |

已有失败与环境约束：

- 旧捕获来自不可变的既有集成 run；本任务只读取并重新解析，没有修改原始日志。
- RV capture SHA-256：stdout
  `74dd190ba339a3e1729aa1aa703b62b3091615923a234e48f346096cfbc201cf`，stderr
  `f47a03c045fcc1da02fb437f7b1feac93a82253739d0dd1fca3bd9522451db81`。
- LA capture SHA-256：stdout
  `7e1df41b1c08984157ac1a702a52053f232bb69ee94b2cb8cd4eef50e2bd26da`，stderr
  `510e961d5dab1fb0fe6f4531b8e6fb2bffd3e693238fc32c4ea541b896805864`。
- 旧 run 的规范化证据位置记为
  `<workspace>/oskernel2026-orays/test/output/integration-74f55223-review-full-all-1/`；
  原始大日志不复制、不提交。
- 初始计划包含 55 个 BusyBox 有序行、54 个不同文本；相同命令位于两个不同
  序位且参与同一个状态构造序列。此处的重复文本不是外部计划缺陷。

# 3. 设计与决策

## 方案

生产者为每个实际执行的 BusyBox 非空行发出一基的执行序号，并保留原命令文本
作为载荷。可信计划改为结构化有序 case；解析器同时验证序号、文本、计划顺序、
完成性和可选显式 ID 的唯一性。旧文本协议因无法区分合法重复与 frame replay，
只作为诊断输入，不静默升级为可信身份。

## 备选方案

- 继续以文本为身份并放宽重复检查：拒绝，因为会隐藏重放和缺失。
- 删除计划中的第二条重复命令：拒绝，因为会改变权威执行计划和有状态行为。
- 按架构/libc/具体命令特判：拒绝，因为不通用且违反测试诚信要求。
- 把结构错误映射成 `FAIL` 或 `PASS`：拒绝，因为会破坏结果语义。

## 关键决策

| 决策 | 理由 | 风险 | 回滚方式 |
|---|---|---|---|
| 使用一基执行 ordinal 作为稳定来源身份 | 当前外部源按行顺序执行，且旧快照没有可靠物理空行信息；执行 ordinal 可从生产者与快照一致导出 | 源行插入会改变后续 ordinal | 计划哈希/顺序审查会显式暴露 drift；回滚协议提交 |
| 命令文本是证据载荷而非唯一身份 | 相同文本可在不同状态位置合法出现 | 文本与 identity 可能错配 | 同时严格校验 ordinal、文本和顺序 |
| 显式 ID 若出现必须独立全局唯一 | ordinal 不应掩盖人工 ID 冲突 | 计划迁移错误会阻断运行 | 修正计划，不能放宽解析 |
| 旧文本记录保持 fail-closed | 无法可靠区分重复步骤与重放 | 旧日志不能成为最终通过证据 | 用新 producer 生成双架构新鲜证据 |
| 语义发现与结构发现分别累计 | 保持 `FAIL`/`ERROR` 边界并保留全部诊断 | 错误优先级需要测试 | runner/parser contract 单测覆盖 |

# 4. 开发与调试记录

> 在开发过程中按 checkpoint 持续追加，不在任务结束后补造过程。

## 2026-07-16 — Checkpoint 1：合同、provenance 与旧证据回放

- 修改：在任何产品代码修改前创建本计划与开发日志。
- 观察：两架构旧 run 均完整执行 24 组并保留大量真实语义发现；结构错误各只有
  两项，种类全部为 `busybox-duplicate-case`。
- 问题：生产者只输出命令文本，解析器也只用文本做身份；计划本身有意重复文本。
- 根因：把可重复的显示文本误当作唯一 case identity，而不是 producer/parser
  协议缺少稳定序位。
- 解决：拟增加通用 ordered identity，并为完整性、重放、显式 ID 和结果分类补齐测试。
- 对应文件/提交：本日志、活动计划；尚无代码提交。
- 下一步：完整阅读相关 producer/parser/runner 测试，实施最小协议与可信计划迁移。

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex（GPT-5 系列，精确子版本未知） | 合同阅读、证据回放、设计、实现、测试与文档 | 本 Goal A 分支的计划、开发日志及后续证据基础设施改动 | 严格限制在 Goal A；拒绝修改外部计划、弱化解析或提前处理语义失败 | 聚焦测试、mutation tests、quick、baseline、双架构 official、独立只读审查 | 待人工 PR 负责人确认 |

交互摘要或记录位置：本开发日志记录决定、实际命令、结果和取舍；不提交完整对话或
主机隐私信息。

# 6. 外部参考与许可证

本 checkpoint 未复制或改写任何外部项目代码。任务依据仅为仓库内合同、源码、测试和
既有运行证据；如后续使用外部资料，将在此追加来源、版本、许可证与改写范围。

# 7. 最终验证

镜像信息：

| 架构 | 文件名 | SHA-256 | 来源/版本 |
|---|---|---|---|
| RISC-V64 | `sdcard-rv.img` | `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99` | Goal A 指定官方只读输入；待最终复核 |
| LoongArch64 | `sdcard-la.img` | `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50` | Goal A 指定官方只读输入；待最终复核 |

测试结果：尚未开始最终候选验证；后续逐条追加，未运行不计为通过。

# 8. 最终审查

- [ ] `git diff --check` 通过。
- [ ] 无测例特化、假成功或吞退出码。
- [ ] 无凭据、无机器相关绝对路径、无大体积生成物。
- [ ] Linux/ABI/errno/并发/资源回收已检查。
- [ ] Goal A 要求的 clean quick、baseline 与双架构 official 已完成。
- [x] AI 初始使用披露已建立；任务结束前继续更新。
- [x] 当前无外部代码来源；若变化则追加披露。
- [ ] 独立 reviewer 的 blocker/major finding 已清零。
- [ ] 人工 PR 负责人能够不依赖 AI 解释和调试本 PR。

审查人及结论：待最终独立只读审查。

# 9. 已知限制、后续工作与回滚

## 已知限制

- 旧 capture 的文本协议不具备无歧义身份，只能证明根因，不能替代新鲜最终证据。
- 旧 capture 中的真实语义失败属于 Goal B 或后续语义修复，不在本任务处理。

## 后续工作

仅完成 Goal A 合同中的协议、回归、clean gates、新鲜双架构证据、只读审查和普通分支
push。到达终态后停止，不自动进入语义修复。

## 回滚方式

按提交逆序普通 revert Goal A 稳定化提交；权威基线与官方镜像保持不变，无需改写历史。

# 10. 最终摘要

进行中。尚未达到任何 Goal A 终态，也未开始 Goal B。
