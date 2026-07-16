---
title: "PR draft: official semantic stabilization"
date_started: 2026-07-16
date_completed: null
status: draft
pr: null
branch: "stabilize/post-integration-gates-20260716"
authors: ["OpenAI Codex (AI-assisted; human owner pending)"]
reviewers: []
base_commit: "ac36481d6052457433b4d1ab5f2a5fd40a20df26"
head_commit: "ac36481d6052457433b4d1ab5f2a5fd40a20df26"
capability_domains: ["process", "scheduler", "memory", "filesystem", "fd", "poll", "signal", "linux-abi", "riscv64", "loongarch64"]
---

# 1. 背景与目标

## 背景

Goal A 已在精确 clean、已推送的分支 HEAD 上达到唯一终态
`READY_FOR_SEMANTIC_FIX`：统一 runner、可信官方 case plan、双流捕获、严格 parser 和
证据 provenance 已闭环。其 fresh official 运行完整执行 24 个 group、2544 个 case，
但诚实保留 RV 114 条、LA 157 条语义 finding，因此未宣称 official/full PASS。

本 Goal B 只处理这些真实内核/Linux ABI 语义失败。长期合同为根 `AGENTS.md`、
`.codex/tasks/SESSION_GUIDANCE.md` 与
`.codex/tasks/GOAL_B_SEMANTIC_STABILIZATION.md`；活动执行计划为
`docs/plans/active/official-semantic-stabilization.md`。

## 目标

- 以 fresh 原始日志和当前源码为依据建立唯一失败矩阵与根因 cluster。
- 对每个 cluster 建立通用最小复现和行为回归，修复生产语义而非测试结论。
- 在 RISC-V64 与 LoongArch64 上保持 Linux/POSIX ABI、errno、边界、并发和资源回收
  一致，并以 clean、可复现证据验证。
- 最终满足 Goal B 合同中的双架构 official、quick、baseline、full-all、独立审查、
  文档和普通 push 条件后，只声明一个终态。

## 非目标

- 不修改官方 case plan、suite 内容、parser、结果状态、blacklist、timeout 来减少失败。
- 不按测例名、路径、libc、架构或固定参数特化；不伪造成功、吞错或隐藏非 PASS。
- 不修改官方镜像、外部评测计划、`main`，不升级依赖/工具链，不做无关重构。
- 未获本轮明确授权前不 push；不 force-push、rebase、squash 或改写历史。

## 验收标准

- [ ] 每个基线 failure identity 都归入有原始证据、最小复现和可证伪假设的 cluster。
- [ ] 所有生产修复均有行为回归，且不是测例/路径/环境特化。
- [ ] RV/LA official 在可信计划下均 planned=executed=completed 且全部 PASS。
- [ ] exact clean HEAD quick、baseline 与 `full --arch all` 均全部 PASS。
- [ ] 镜像哈希不变，无 overlay 残留，无新增未解释 `unsafe`、skip、timeout 或环境阻塞。
- [ ] 最终独立只读 review 为 0 Blocker / 0 Major，文档与披露完整。
- [ ] 获明确授权后普通 push，远端精确 head/ancestry 正确且 `main` 未变。

# 2. 基线

| 时间 | 命令 | 架构/目标 | 退出码 | 结果 | 备注/证据 |
|---|---|---|---:|---|---|
| 2026-07-16 | `git status --short --branch`; `git rev-parse HEAD` | Git | 0 | PASS | worktree clean；HEAD `ac36481d...`，跟踪同名远端分支 |
| 2026-07-16 | `git ls-remote origin <stabilize> <integration> <main>` | 远端引用 | 0 | PASS | stabilize `ac36481d...`；integration `09f4076a...`；main `921171ac...` |
| 2026-07-16 | 读取 Goal A exact-HEAD quick summary | quick | 0 | PASS | 45/45/45，45 PASS；summary SHA-256 `ec1492f48f77aaed7e4e2b246530e2135a353f135daa149e3617407e73dd9904` |
| 2026-07-16 | 读取 Goal A exact-HEAD baseline summary | baseline | 0 | PASS | 57/57/57，57 PASS；summary SHA-256 `2ec6174195a80aeb210fc8b3889ba18521336b6fcef684836137337b9f97e0a9` |
| 2026-07-16 | 当前 parser 重放 Goal A fresh RV stdout/stderr，process rc=0 | RISC-V64 official | 1 | FAIL | 24/24/24 groups，2544/2544/2544 cases，0 integrity error，114 semantic findings；这是可信语义基线 |
| 2026-07-16 | 当前 parser 重放 Goal A fresh LA stdout/stderr，process rc=0 | LoongArch64 official | 1 | FAIL | 24/24/24 groups，2544/2544/2544 cases，0 integrity error，157 semantic findings；这是可信语义基线 |
| 2026-07-16 | `sha256sum` 两张只读官方镜像 | RV+LA image | 0 | PASS | 哈希与 Goal A 合同一致 |
| 2026-07-16 | QEMU/qemu-img/clang 版本、能力和 SHA-256 探针 | toolchain | 0 | PASS | QEMU/qemu-img 9.2.4；clang 21.1.8；二进制哈希与 Goal A 一致 |

两份 official summary 的 SHA-256：

- RV：`651e6053bdf18d7ef4e027c1c4e7906367a8084a815b8d201ba7b6e937e6200a`；
- LA：`6e233af93b90e281373683a20afe632db3af545a8f3d99bc040b5b3d1626e783`。

本次重放所读原始捕获 SHA-256：

- RV stdout：`c7344b37dd55bf3c0116dc7063c6e4b9e0b5c6db5472d12b623e8be003a673cc`；
  stderr：`d529553efb9810e6677c0dc2609897a4dcb737184cffb47b117b5a2156d940d5`；
- LA stdout：`5c8a01d89c8c4624c79e13ce4475023733c1e8f7d423cdfd5f7bd196a90e16ae`；
  stderr：`b23d1a983b8ad1b1ed116fa148283ee467462baf27597e6dc0687e51c4fb0944`。

原始输出保存在已忽略的 `test/output/goala-1f16c889-remediation-official-<arch>-1/`；
不提交大体积日志。本轮哈希指向该 remediation 捕获，Goal A 日志较早 checkpoint 中的
另一组 stdout/stderr 哈希属于旧 `9ec972f4...` official 运行，不能混用。

已有失败与环境约束：

- RV：LTP musl 19、glibc 13 个非零 case；glibc libctest 38 fail / 2 timeout；
  `cyclictest-musl` 在 hackbench 阶段达到 900s group 上限。
- LA：LTP musl 25、glibc 16 个非零 case；glibc libctest 同为 38 fail / 2 timeout；
  cyclictest 两组均完成。
- 两架构共同失败集中在 poll/epoll 与 splice/tee/vmsplice/pipe；LA 另有 time/scheduler/
  readlink 差异，RV 另有 `nice04` 与 cyclictest timeout。详细 identity 将在 Checkpoint 2
  从原始 case 边界提取，不用 parser marker 数量代替根因数量。
- 两张镜像只读输入，SHA-256 分别为
  `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99` 和
  `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50`。

# 3. 设计与决策

## 方案

执行单位是根因 cluster，不是单个官方 case。每个循环固定为：原始证据和源码假设、
通用最小复现、基线真实失败、行为回归、生产修复、双架构定向验证、只读审查、逻辑
提交、clean quick/baseline；只有预期产生可测 official delta 时才运行 fresh official。

优先级依次为：panic/trap/内存与资源安全；timeout/死锁/失去唤醒；双架构 divergence/
ABI；公共 libc/LTP 根因；复合文件/process/VM/FD/signal；孤立兼容；性能。当前首个
候选是 `splice02` 的 EBADF + 30s timeout，因为它在两架构、两 libc 重现并直接体现
错误 errno 与失去进展。

## 备选方案

- 直接按 114/157 parser finding 逐条修：拒绝。多个 marker 来自同一 case，且会造成
  重复修复和错误计功。
- 先跑更多完整 official 获取样本：暂不采用。现有捕获结构完整，先建立最小复现更能
  区分确定性语义缺口和下游症状。
- 通过调整 timeout、blacklist、case plan 或 parser 降低失败：合同明确禁止。

## 关键决策

| 决策 | 理由 | 风险 | 回滚方式 |
|---|---|---|---|
| Goal A fresh remediation 捕获是本轮唯一初始 official 基线 | 当前 parser 可完整重放且 0 integrity error | 原始 artifact 已忽略，需靠路径和哈希维持身份 | 开发日志固定 summary/raw 哈希；丢失时重新 fresh 运行，不用记忆重建 |
| 先按 case boundary 聚类，再写测试和代码 | 防止把 TFAIL/TBROK/汇总 marker 当作独立根因 | 分析阶段较长 | 持续更新 cluster 表和原始上下文，不做产品改动前可无损重来 |
| 公开 ABI、VM、process、FD、lock、arch 改动至少跑 baseline | 这些领域交叉风险高 | 验证耗时 | 每个逻辑提交独立，失败时普通 revert 并保留证据 |
| official 只由明确预期 delta 触发 | 保持运行可解释且避免无假设消耗 | 可能延后发现组合问题 | 终态仍强制双架构 official 与 full-all，不减门禁 |

# 4. 开发与调试记录

> 在开发过程中按 checkpoint 持续追加；中间失败、反证和首次非 PASS 不覆盖。

## 2026-07-16 — Checkpoint 1：合同采纳与 Goal A 前置审计

- 修改：创建 Goal B 活动计划和本开发日志；尚未修改生产代码、测试业务逻辑或 manifest。
- 观察：当前本地/远端稳定化 HEAD 精确一致且 clean；integration、main 未变化。Goal A
  exact-HEAD quick/baseline 全 PASS，fresh 双架构 official 完整执行但分别保留 114/157
  语义 finding。当前 parser 重放得到相同结论和 0 integrity error。
- 问题：parser finding 数并非唯一根因数；例如 `splice02` 同时产生 TFAIL、timeout、
  TBROK、summary 和非零 case code，必须在 case boundary 内解释。RV 的两个
  `panic-or-trap` 来自 LTP timeout 文本，另两个来自 glibc 用户态 fortify termination，
  现阶段均不能宣称为内核 panic。
- 根因：待逐 cluster 证明。首批证据显示 `splice02` 在 RV/LA、musl/glibc 均先返回
  EBADF，随后写入 1 MiB 时失去进展并被 LTP 30s watchdog 杀死；RV cyclictest 在
  hackbench 大量 task/pipe 阶段达到 900s 上限。
- 解决：尚未实施。下一步从原始 START/RUN/RESULT/END 边界提取所有非 PASS identity
  和上下文，补齐 glibc libctest 的 38 fail / 2 timeout 身份，再选择首个最小复现。
- 对应文件/提交：本 checkpoint 仅新增计划和日志，尚未提交。
- 下一步：完成失败矩阵；检查 pipe/splice、FD 类型/offset、阻塞唤醒和相关现有行为测试。

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex（GPT-5 系列，精确子版本未知） | 合同阅读、Goal A 证据审计、失败聚类、后续设计/编码/测试/文档编排 | 本计划、开发日志；后续受影响文件将逐 checkpoint 列出 | 坚持 fail-closed、无测例特化、双架构和 clean provenance；任何 AI 建议须经源码、回归与真实 guest 证据验证 | summary/raw-log 哈希、当前 parser replay、定向回归、quick/baseline、fresh official/full、独立只读 review | 待人工 PR 负责人确认 |

交互摘要或记录位置：本日志记录关键决定、命令、结果、反证和取舍；不提交完整对话、
凭据、隐私数据或无关主机信息。

# 6. 外部参考与许可证

| 来源 | 版本/commit | 借鉴范围 | 许可证 | OrayS 修改 | 记录/文件 |
|---|---|---|---|---|---|
| 暂无 | — | 当前仅使用仓库源码、测试协议与已有官方镜像行为证据 | — | — | 若后续查阅 Linux/Asterinas/ArceOS 等实现或规范，在借鉴前追加精确来源、版本和许可证 |

# 7. 最终验证

镜像信息：

| 架构 | 文件名 | SHA-256 | 来源/版本 |
|---|---|---|---|
| RISC-V64 | `sdcard-rv.img` | `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99` | workspace 父目录只读官方镜像；版本沿用 Goal A 记录 |
| LoongArch64 | `sdcard-la.img` | `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50` | workspace 父目录只读官方镜像；版本沿用 Goal A 记录 |

测试结果：

| Run ID | 命令 | 架构/目标 | 退出码 | 结果 | 耗时 | 原始证据 |
|---|---|---|---:|---|---:|---|
| Goal A exact-HEAD terminal quick | `python3 test/run_suite.py --profile quick` | quick | 0 | PASS | 见 Goal A 日志 | `test/output/goala-ac36481d-terminal-declaration-quick-1/summary.json` |
| Goal A exact-HEAD terminal baseline | `PATH=<verified-tools> python3 test/run_suite.py --profile baseline` | baseline | 0 | PASS | 见 Goal A 日志 | `test/output/goala-ac36481d-terminal-declaration-baseline-1/summary.json` |
| Goal B initial replay RV | `parse_official_results.py --stdout <rv-capture> --stderr <rv-capture> --process-exit-code 0` | RISC-V64 | 1 | FAIL | 5.06 s | Goal A fresh RV raw capture；114 semantic / 0 integrity |
| Goal B initial replay LA | `parse_official_results.py --stdout <la-capture> --stderr <la-capture> --process-exit-code 0` | LoongArch64 | 1 | FAIL | 4.88 s | Goal A fresh LA raw capture；157 semantic / 0 integrity |

结果状态只使用：`PASS`、`FAIL`、`ERROR`、`TIMEOUT`、`BLOCKED`、`SKIPPED`。上表两个
official replay 是可信失败基线，不是完成门禁；Goal B 实现后的 fresh 结果将在此追加。

# 8. 最终审查

- [ ] `git diff --check` 通过。
- [ ] 无测例特化、假成功或吞退出码。
- [ ] 无凭据、无机器相关绝对路径、无大体积生成物。
- [ ] Linux/ABI/errno/并发/资源回收已检查。
- [ ] RISC-V64 与 LoongArch64 完整门禁通过。
- [ ] AI 和外部来源披露完整。
- [ ] 独立 reviewer 的 blocker/major finding 已清零。
- [ ] 负责人能够不依赖 AI 解释和调试本 PR。

审查人及结论：待最终独立只读 review。

# 9. 已知限制、后续工作与回滚

## 已知限制

- 当前状态为 `IN_PROGRESS`；双架构 official 均为语义 `FAIL`，full-all 未运行，不具备
  Ready/merge 条件。
- 初始 cluster 仍是假设；在最小复现证明前不得把架构/测例相关性写入生产分支。
- 原始 official artifact 被 Git 忽略；本日志用 run ID 与 SHA-256 固定身份，但若本地
  artifact 丢失必须 fresh 重跑，不能用摘要补造原始证据。

## 后续工作

完成 identity 矩阵后按活动计划逐 cluster 推进；每个 checkpoint 更新尝试、反证、
回归、提交、双架构 delta 和剩余风险。

## 回滚方式

计划/日志可用普通反向提交回滚；后续生产修复保持一个根因一个逻辑提交，必要时使用
普通 revert。禁止 `git reset --hard`、破坏性 rebase、force-push 或覆盖他人改动。

# 10. 最终摘要

尚未完成。当前唯一状态为 `IN_PROGRESS`，没有宣称 official/full PASS、PR Ready 或
Goal B 终态。
