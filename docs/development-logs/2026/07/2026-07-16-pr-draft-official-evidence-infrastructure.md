---
title: "PR draft: stabilize official evidence infrastructure"
date_started: 2026-07-16
date_completed: 2026-07-16
status: ready-for-semantic-fix
pr: null
branch: "stabilize/post-integration-gates-20260716"
authors:
  - "Codex (OpenAI)"
reviewers:
  - "Codex independent read-only reviewer (automated; not human PR owner)"
base_commit: "09f4076ac151e0e7800103de724d9042230738b5"
head_commit: "1a320a9f0b016dc6a861da364a3b7af6ba8e0d1d"
evidence_commit: "9ec972f4eb06e7f50dcdec023d494b7e67c9a990"
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

- [x] 重复文本在不同稳定序号下可合法完成；相同显式 ID 必须拒绝。
- [x] 重放、缺失、额外、乱序、畸形身份、未知组和不完整执行全部拒绝。
- [x] 有真实测试失败但身份完整的官方结果为 `FAIL`，不是 `ERROR` 或 `PASS`。
- [x] quick、baseline、RV official、LA official 满足 Goal A 的干净验证合同。
- [x] 两个官方镜像哈希不变，临时 overlay 全部清理。
- [x] 独立 reviewer 的 blocker/major finding 为零。
- [x] 分支保持可追溯祖先关系并仅以普通 push 发布。

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

## 2026-07-16 — Checkpoint 2：有序身份协议与 fail-closed 回归

- 修改：把可信 BusyBox 计划从字符串数组迁移为 55 个结构化
  `{ordinal, command}` case；原命令、顺序、source metadata 与 libctest 计划均未改变。
- 修改：生产者对每个实际执行的非空行发出同一 ordinal 的 START、唯一终态 RESULT
  和 END；成功、普通非零、timeout-like 退出与执行错误都保留真实状态。
- 修改：解析器验证 ordinal、command payload、计划顺序、frame 完成性、可选显式 ID
  唯一性以及 planned/executed/completed 计数；reporter 保留失败 ordinal 与命令。
- 完整性边界：旧文本记录仍可用于不可变日志回放，但产生
  `busybox-legacy-identity`，不能成为 canonical PASS；混合协议、重放、缺失、额外、
  乱序、孤立记录和不完整 frame 均为结构错误。
- 语义边界：即使同一流同时存在结构错误，合法或畸形流中可辨认的 BusyBox `fail`
  仍保留为 `busybox-failure`，不由 `ERROR` 隐藏。身份完整且有语义失败的流严格为
  `FAIL`，`error_count == 0`。
- 计划一致性复核：55 个命令和旧 schema 的顺序逐项相同，54 个不同文本；重复命令
  仍只位于 ordinal 37 与 41。source snapshot 和 217 个 libctest case 未改变。
- 旧日志重新回放：RV 与 LA 仍各有 24/24/24 个 planned/executed/completed group、
  2544/2544/2544 个 planned/executed/completed case；旧协议各产生 2 个
  `busybox-legacy-identity`，同时分别保留 119 与 161 条语义发现。
- 诊断投影：第一次内存投影未先规范化 ANSI 清屏序列，留下旧协议残片，解析器正确
  返回 `ERROR`；修正诊断脚本为先调用产品同一 `normalize_output_text` 后，完整投影的
  RV/LA 分别返回 `FAIL`（119/161 semantic findings）、`error_count == 0`，且两者均为
  24/24/24 group 与 2544/2544/2544 case。投影未修改或复制原始 evidence 文件，且
  不是最终运行证据。
- 测试：官方解析 111/111、失败报告 9/9、静态 guard mutation 24/24、LTP 汇总
  20/20、测试资产完整性 36/36 均 PASS；直接 integrity guard 为 0 findings；
  `test/run_suite.py --list` 发现 59 个注册 case；runner 集成重跑 134/134 PASS
  （196.053 s）。
- 调试记录：首次 runner 集成测试因 manifest 的精确方法数尚未同步而出现 10 个
  assertion failure；这是预期的 fail-closed inventory 检测。将 4 个受影响 suite 的
  exact count 从 8/23/106/133 更新为 9/24/111/134 后，先前重跑为 134/134 PASS；
  本 checkpoint 再次重跑最终候选。
- 代码范围：`user/shell/src/cmd.rs`、`test/evaluation/`、对应 checks/unit tests、
  canonical manifest/count inventory 与 `test/README.md`；未修改 ABI、syscall、errno、
  架构配置、依赖、工具链、镜像或外部测试源。
- 对应提交：计划/日志基线 `8aa57fd8`；实现、回归与合同文档
  `9ec972f4eb06e7f50dcdec023d494b7e67c9a990`。

## 2026-07-16 — Checkpoint 3：干净实现提交上的 quick 与 baseline

- provenance：候选实现提交为
  `9ec972f4eb06e7f50dcdec023d494b7e67c9a990`；quick 与 baseline 的
  `runner_commit`/`runner_commit_final` 均精确等于该提交，运行前后
  `runner_dirty == false`，且 `runner_provenance_stable == true`。
- quick：45/45/45 planned/executed/completed，45 PASS，其他状态均为 0；退出码
  0，用时 290.899 s。
- baseline：57/57/57 planned/executed/completed，57 PASS，其他状态均为 0；退出码
  0，用时 1287.946 s。其内包含格式检查、静态合规、host/RV/LA evidence、Rust
  workspace 单元测试、clippy、RV/LA kernel build 与 submission build，均为 PASS。
- 两次运行的 summary、逐项 stdout/stderr 和 runner provenance 留在忽略的
  `test/output/` 下；未把原始日志或构建产物提交到 Git。

## 2026-07-16 — Checkpoint 4：新鲜双架构官方证据

- 工具：两个架构均使用同一临时 prefix 中由 QEMU 9.2.4 源码构建的必需
  `qemu-system-*` 与 `qemu-img`。源码归档 SHA-256 为
  `f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a`；
  RV QEMU、LA QEMU、`qemu-img` SHA-256 分别为
  `194d645ab5063833b35512c2d15364070401f63a4f97baf4b7da2244d44efeee`、
  `668da3b54ae3ec6eaf3ce58f37a1ca3a89b881ac3b22bff0b2872f087c1b9f32`、
  `ad01688fda982d710780c06ad3277119a6d110723f0ccb6f9f48535e85d8c8f5`。
  构建所用 clang 21 SHA-256 为
  `82481792aef943c1750ae5fd71e5a5737212741337debd0fe5d28bd82dd018e9`。
- RV official：24/24/24 groups、2544/2544/2544 cases，guest runner 退出码 0，
  `error_count == 0`；严格汇总为 `FAIL`，包含 115 条未隐藏语义发现。failure kind
  为 forbidden-status 66、ltp-internal-summary-failure 35、timeout 4、
  panic-or-trap 4、ltp-summary-failure 2、ltp-nonzero-result 2、
  libctest-failure 1、official-group-failure 1。
- LA official：24/24/24 groups、2544/2544/2544 cases，guest runner 退出码 0，
  `error_count == 0`；严格汇总为 `FAIL`，包含 159 条未隐藏语义发现。failure kind
  为 forbidden-status 95、ltp-internal-summary-failure 54、timeout 3、
  panic-or-trap 2、ltp-summary-failure 2、ltp-nonzero-result 2、
  libctest-failure 1。
- 两个 libc 的 BusyBox 组在两个架构均为 55 START / 55 RESULT / 55 END，全部 55
  个命令真实成功；相同文本的 ordinal 37 与 41 分别完整成功，没有合并或丢失。
- RV 的 cyclictest-musl 真实达到 900 s timeout 并保留为语义 failure；LA 的两个
  cyclictest 组均真实完成。此架构差异未被统一、过滤或改写。
- 两次 official 的 suite 顶层退出码均为 1，因为唯一计划项状态是语义 `FAIL`；
  顶层 `INFRA_ERROR == 0`。这不是 official PASS，也不据此宣称 full PASS。
- 每次运行前后官方 backing image SHA-256 均与基线一致；两次运行结束后均确认
  `sdcard-*.run.qcow2` 不存在。运行证据来自同一干净稳定实现提交 `9ec972f4…`。

## 2026-07-16 — Checkpoint 5：独立只读审查与发布边界

- 独立自动化只读 reviewer 审查
  `09f4076ac151e0e7800103de724d9042230738b5..8301898911f34e847e7070f04988c704beaa751d`、
  四份 canonical summary、必要 raw log、QEMU/image/overlay provenance；审查过程未修改
  工作树、镜像、证据或 Git 状态。
- 结论：0 Blocker、0 Major、1 Minor、0 Nit，Goal A 的独立 review 门禁满足。reviewer
  确认没有命令/测例/路径/架构/libc 特化，没有弱化 duplicate/missing/unknown/
  malformed/incomplete 检测，没有把 TCONF/TBROK/TFAIL、timeout、panic/trap 或普通
  失败映射为 PASS；双架构 summary 与原始日志支持开发日志中的全部关键计数和分类。
- 唯一 Minor：front matter 的 `head_commit` 仍指向实现 evidence commit，而受审候选
  已为 `83018989…`。本 checkpoint 将 `head_commit` 更新为已审候选，并新增
  `evidence_commit` 保留 official 证据与实现提交的精确关联。包含本行的文档提交无法
  自引用其未来 SHA；最终精确 Git HEAD、final-head gate 与远端 head 由忽略的 canonical
  summary 和本 session 的终态报告共同记录。
- reviewer 明确声明其为自动化只读审查，不冒充根 `AGENTS.md` 要求的真实人工 PR
  负责人理解/复核声明。
- 在远端再次确认权威基线仍为 `09f4076a…` 且同名稳定化分支不存在后，主 agent 请求
  执行首次普通 push；执行环境的外部写入审批拒绝该动作，因此 push 命令没有执行、
  远端状态没有变化。未尝试 force-push、旁路执行或修改 `main`；等待明确授权时继续
  完成所有本地 final-head 验证。

## 2026-07-16 — Checkpoint 6：final-head 门禁、首次失败与根因重跑

- review/closure 文档提交后的受测 HEAD 为
  `cbb3baf64ad0af5b4d6bb35b8e4d24f483abc314`。所有下述 runner 都在开始与结束时
  精确报告该 commit、`runner_dirty == false` 和 stable provenance。
- final quick：45/45/45 planned/executed/completed，45 PASS、其余状态为 0，退出码 0，
  用时 289.222 s；summary SHA-256 为
  `b225634a64562c1f36033984074b5a753fef3e06844106241af1606d29948f4b`。
- final baseline 首次运行没有使用先前验证过的工具 prefix，因而诚实结束为
  `INFRA_ERROR`：57 planned、56 executed、57 completed，54 PASS、2 FAIL、
  1 INFRA_ERROR，退出码 2，用时 714.663 s；summary SHA-256 为
  `696d1646d0a4f24c71b5f967eb63d1a559ebc562f2b84d1203de6588dfa732d9`。
- 首次失败明细：`evidence.riscv64` 解析到系统 QEMU 6.2，而 manifest 要求 QEMU
  9.2.4，因此 required smoke 为 BLOCKED、该 case 为 FAIL；`evidence.aggregate` 仅派生
  FAIL。`baseline.clippy_loongarch64` 解析到 clang 14，其 LoongArch target capability
  probe 返回 1，故主体命令未执行并严格记为 INFRA_ERROR。LA evidence、两个 kernel
  build、submission build、workspace 73 tests 及其余已执行项均保留真实结果。
- 诊断先独立复核既有 QEMU 9.2.4 与 clang 21：两架构 QEMU、`qemu-img`、clang 的
  SHA-256 均与 Checkpoint 4 一致；两架构 QEMU 的版本首行均为 9.2.4，clang 21 的空
  LoongArch translation-unit probe 返回 0。该根因属于 invocation prerequisite 选择，
  没有修改产品、runner、manifest、工具二进制或失败分类代码。
- 随后以显式
  `PATH=<qemu-9.2.4-prefix>/bin:<llvm-21-prefix>/bin:$PATH` 写入独立输出目录重跑完整
  baseline。该次为 57/57/57 planned/executed/completed，57 PASS、其余状态为 0，
  退出码 0，用时 708.662 s；summary SHA-256 为
  `886c87df1bc39e8f50c057936a06939b75eea9e1f87c70cbe56725a97bd7ae4b`。
  summary 的 required-command provenance 精确指向 QEMU 9.2.4 和 clang 21。
- 第二次 PASS 不覆盖首次 `INFRA_ERROR`；两份 summary 与逐项日志位于不同忽略目录，
  本日志同时保留首次失败、根因和重跑结果。此事件是可解释的错误工具选择，不记为
  产品 PASS，也不记作未解释 flake。
- 门禁后再次确认两官方镜像哈希仍精确等于合同值，未发现 `sdcard-*.run.qcow2`；
  worktree clean，完整 `base..HEAD` diff check 通过，远端权威基线仍为 `09f4076a…`，
  同名稳定化分支仍不存在。
- 独立 follow-up reviewer 在本 checkpoint 写入前正确报告 1 Major：上述 final-head
  三份 summary 尚未进入 tracked 开发日志。当前 checkpoint 专门补齐命令、退出码、
  计数、哈希、首次失败和根因；修复后的只读复核结论为 0 Blocker / 0 Major /
  0 Minor / 0 Nit，不能沿用修复前的结论代替本次复核。

## 2026-07-16 — Checkpoint 7：terminal HEAD、普通 push 与 Goal A 终态

- 记录首次失败与根因的 documentation-only 提交后，精确 terminal HEAD 为
  `1a320a9f0b016dc6a861da364a3b7af6ba8e0d1d`；worktree clean，完整
  `base..HEAD` diff check 通过，权威基线仍为 `09f4076a…`。
- terminal quick：45/45/45 planned/executed/completed，45 PASS、其余状态为 0，
  退出码 0，用时 283.746 s；summary SHA-256 为
  `d65720c1fb0394078152c295879651ff026670ec4ac8429b7f3303fb5d6666db`。
- terminal baseline：显式使用与 official/final rerun 相同的已验证 QEMU 9.2.4 与
  clang 21 PATH，57/57/57 planned/executed/completed，57 PASS、其余状态为 0，
  退出码 0，用时 702.813 s；summary SHA-256 为
  `935a86a2a994c1b491ca238b47fc62356a197da0ff7baa3cbd09fa7d68f63355`。
- 两份 terminal summary 的 `runner_commit` 与 `runner_commit_final` 均精确为
  `1a320a9f…`，运行前后 `runner_dirty == false`，stable provenance 为 true。
  baseline required-command provenance 精确指向同一 QEMU 9.2.4 双架构二进制和
  clang 21；没有复用系统 QEMU 6.2 或 clang 14。
- terminal 独立只读 reviewer 复核最终 diff、两份 exact-HEAD summary、实现 official
  evidence、首次 baseline `INFRA_ERROR` 记录、镜像/overlay 与推送前远端 freshness，结论为
  0 Blocker / 0 Major / 0 Minor / 0 Nit。该自动化审查不替代人工 PR owner 声明。
- terminal gate 后再次计算两官方镜像 SHA-256，均仍精确等于合同值；未发现
  `sdcard-*.run.qcow2`。未修改镜像、外部计划、`main` 或 Goal B 范围。
- 用户明确授权后，以普通 `git push -u origin stabilize/post-integration-gates-20260716`
  创建远端分支；命令退出码 0，未 force-push。随后本地 HEAD、upstream 与远端 head
  均精确为 `1a320a9f…`，远端权威基线仍为 `09f4076a…`。
- 至此 Goal A 的成功终止条件全部成立，唯一声明的终态为
  `READY_FOR_SEMANTIC_FIX`。本任务停止，不自动开始 Goal B，也不把该交接终态表述为
  official/full PASS 或 PR Ready。

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex（GPT-5 系列，精确子版本未知） | 合同阅读、证据回放、根因分析、设计、实现、测试、官方运行、文档、独立只读复核与分支发布编排 | 本 Goal A 分支的计划、开发日志、BusyBox 有序证据协议、解析/报告、回归与 manifest inventory | 严格限制在 Goal A；拒绝修改外部计划、弱化解析、掩盖 official failure 或提前处理语义失败；外部写入审批拒绝后未绕过，取得用户明确授权后才普通 push | 聚焦与 mutation tests、干净 quick/baseline、新鲜双架构 official、镜像/overlay 复核、独立只读审查、远端 head/ancestry 验证 | 待人工 PR 负责人确认 |

交互摘要或记录位置：本开发日志记录决定、实际命令、结果和取舍；不提交完整对话或
主机隐私信息。

# 6. 外部参考与许可证

本 checkpoint 未复制或改写任何外部项目代码。任务依据仅为仓库内合同、源码、测试和
既有运行证据；如后续使用外部资料，将在此追加来源、版本、许可证与改写范围。

# 7. 最终验证

镜像信息：

| 架构 | 文件名 | SHA-256 | 来源/版本 |
|---|---|---|---|
| RISC-V64 | `sdcard-rv.img` | `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99` | Goal A 指定官方只读输入；运行前后相同 |
| LoongArch64 | `sdcard-la.img` | `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50` | Goal A 指定官方只读输入；运行前后相同 |

实现证据提交：`9ec972f4eb06e7f50dcdec023d494b7e67c9a990`。以下四次 canonical
运行的 runner 在开始与结束时均报告该精确提交、clean worktree 和稳定 provenance。

| UTC 时间 | 命令 | 退出码 | 结果 | 计数与耗时 | 忽略的证据目录 / summary SHA-256 |
|---|---|---:|---|---|---|
| 04:59:21–05:04:12 | `python3 test/run_suite.py --profile quick` | 0 | PASS | 45/45/45；45 PASS；290.899 s | `test/output/20260716T045921Z-quick-none-2/` / `16d3ba472cee764941d4e4ea949a2c0cb76ba903a3ee0d8852a5a83f7c00b87c` |
| 05:05:28–05:26:56 | `python3 test/run_suite.py --profile baseline --output-dir test/output/goala-9ec972f4-baseline-1` | 0 | PASS | 57/57/57；57 PASS；1287.946 s | `test/output/goala-9ec972f4-baseline-1/` / `9898e723b30b5b1ebf652393e3df8e604f247bc2ccc3164f0656b30805d7bbed` |
| 05:29:33–06:49:17 | `python3 test/run_suite.py --profile official --arch rv --output-dir test/output/goala-9ec972f4-official-rv-1` | 1 | FAIL（仅语义） | 24/24/24 groups；2544/2544/2544 cases；0 error、115 failure；4784.534 s | `test/output/goala-9ec972f4-official-rv-1/` / `ace51a9e6ec217d55276c1f98caec2722eb94fa63b2d24a07dc020e55b35933b` |
| 06:51:14–08:07:46 | `python3 test/run_suite.py --profile official --arch la --output-dir test/output/goala-9ec972f4-official-la-1` | 1 | FAIL（仅语义） | 24/24/24 groups；2544/2544/2544 cases；0 error、159 failure；4591.489 s | `test/output/goala-9ec972f4-official-la-1/` / `090d3fc3a6127da0937d2b08ee7ba6f4a39d1887e054de88c8d6c607a7ef6658` |

final-head closure 验证如下。首次 baseline 的非 PASS 被完整保留；第二次运行使用经哈希
和 capability probe 验证的必需工具 prefix，不修改任何产品或测试文件。

| UTC 时间 | 命令 | 退出码 | 结果 | 计数与耗时 | 忽略的证据目录 / summary SHA-256 |
|---|---|---:|---|---|---|
| 08:28:41–08:33:31 | `python3 test/run_suite.py --profile quick --output-dir test/output/goala-cbb3baf6-final-quick-1` | 0 | PASS | 45/45/45；45 PASS；289.222 s | `test/output/goala-cbb3baf6-final-quick-1/` / `b225634a64562c1f36033984074b5a753fef3e06844106241af1606d29948f4b` |
| 08:33:48–08:45:42 | `python3 test/run_suite.py --profile baseline --output-dir test/output/goala-cbb3baf6-final-baseline-1` | 2 | INFRA_ERROR | 57/56/57；54 PASS、2 FAIL、1 INFRA_ERROR；714.663 s | `test/output/goala-cbb3baf6-final-baseline-1/` / `696d1646d0a4f24c71b5f967eb63d1a559ebc562f2b84d1203de6588dfa732d9` |
| 08:47:15–08:59:04 | `PATH=<qemu-9.2.4-prefix>/bin:<llvm-21-prefix>/bin:$PATH python3 test/run_suite.py --profile baseline --output-dir test/output/goala-cbb3baf6-final-baseline-2` | 0 | PASS | 57/57/57；57 PASS；708.662 s | `test/output/goala-cbb3baf6-final-baseline-2/` / `886c87df1bc39e8f50c057936a06939b75eea9e1f87c70cbe56725a97bd7ae4b` |

terminal documentation-only HEAD 的不可自引用运行证据如下；该证据由 terminal reviewer
独立核对，并由本 session 终态报告与远端精确 head 共同绑定。

| UTC 时间 | 命令 | 退出码 | 结果 | 计数与耗时 | 忽略的证据目录 / summary SHA-256 |
|---|---|---:|---|---|---|
| 09:15:14–09:19:58 | `PATH=<qemu-9.2.4-prefix>/bin:<llvm-21-prefix>/bin:$PATH python3 test/run_suite.py --profile quick --output-dir test/output/goala-1a320a9f-terminal-quick-1` | 0 | PASS | 45/45/45；45 PASS；283.746 s | `test/output/goala-1a320a9f-terminal-quick-1/` / `d65720c1fb0394078152c295879651ff026670ec4ac8429b7f3303fb5d6666db` |
| 09:20:06–09:31:48 | `PATH=<qemu-9.2.4-prefix>/bin:<llvm-21-prefix>/bin:$PATH python3 test/run_suite.py --profile baseline --output-dir test/output/goala-1a320a9f-terminal-baseline-1` | 0 | PASS | 57/57/57；57 PASS；702.813 s | `test/output/goala-1a320a9f-terminal-baseline-1/` / `935a86a2a994c1b491ca238b47fc62356a197da0ff7baa3cbd09fa7d68f63355` |

official 运行显式设置 `$RV_TESTSUITE_IMG` / `$LA_TESTSUITE_IMG` 为 workspace 父目录的
对应只读镜像，并将 `QEMU_PREFIX` 指向临时 QEMU 9.2.4 prefix。RV 原始 stdout/stderr
SHA-256 分别为
`d83bb29434a93f65d00815b7dbc0addd7d35628f90674eebfefe7a2ef57fbb5f` /
`f94ef4bf5914dd35034e6ba024041f40930a26fcb84f286e221dbaed5db913c2`；LA 分别为
`208a99196e6abeed1216c50948e4ef406a6aacc4745c24eeb9e4eaf026213962` /
`18e80d507fc59f857abb565175c36023e05ac9836b3bdf8d6a5e4aad210fd305`。

镜像在两次 official 前后分别以 `sha256sum` 复核，值与上表一致；在每次运行后以
overlay 路径检查确认无 `sdcard-*.run.qcow2`。因此两个 `FAIL` 明确来自完整保留的
guest 语义失败，不是结构、identity、parser、runner、镜像或 QEMU 基础设施错误，
也不宣称 official/full PASS。

# 8. 最终审查

- [x] `git diff --check` 通过。
- [x] 无测例特化、假成功或吞退出码。
- [x] 无凭据、无机器相关绝对路径、无大体积生成物。
- [x] Linux/ABI/errno/并发/资源回收已检查；本改动只改变测试证据协议与解析，不改变这些可见语义。
- [x] Goal A 要求的 clean quick、baseline 与双架构 official 已完成。
- [x] AI 使用披露已更新到实现、验证与文档范围。
- [x] 当前无外部代码来源；若变化则追加披露。
- [x] 独立 reviewer 的 blocker/major finding 已清零。
- [ ] 人工 PR 负责人能够不依赖 AI 解释和调试本 PR。

审查人及结论：独立 Codex 只读 reviewer（自动化、非人工 PR 负责人）初审候选
`8301898911f34e847e7070f04988c704beaa751d` 为 0 Blocker / 0 Major / 1 Minor / 0 Nit；
Minor 已通过区分 `head_commit` 与 `evidence_commit` 处理。follow-up reviewer 对
`cbb3baf6…` 的代码和三份 final-head summary 未发现实现问题，但因这些结果尚未写入
tracked 日志而给出 1 Major；Checkpoint 6 补齐记录后，reviewer 复核为 0 Blocker /
0 Major / 0 Minor / 0 Nit。terminal reviewer 随后对精确 `1a320a9f…`、最终两份 clean
summary、镜像与推送前远端 freshness 再次给出 0 Blocker / 0 Major / 0 Minor / 0 Nit，Goal A 独立
review 门禁满足。

# 9. 已知限制、后续工作与回滚

## 已知限制

- 旧 capture 的文本协议不具备无歧义身份，只能证明根因，不能替代新鲜最终证据。
- 新鲜 official 中 RV 的 115 条与 LA 的 159 条语义 findings 均仍存在；它们属于
  Goal B 或后续语义修复，不在本任务处理，且未被本任务隐藏或改写。
- 本日志记录的是 Goal A 代码交接门禁；根 `AGENTS.md` 要求的真实队员理解/复核声明
  仍须在 PR 标记 Ready 或合并前由人工负责人完成，本自动化任务不冒充该声明。

## 后续工作

Goal A 已停止在 `READY_FOR_SEMANTIC_FIX`。语义 failure 的修复必须作为独立 Goal B 或
后续任务显式启动；本任务不自动进入语义修复。

## 回滚方式

按提交逆序普通 revert Goal A 稳定化提交；权威基线与官方镜像保持不变，无需改写历史。

# 10. 最终摘要

Goal A 唯一终态：`READY_FOR_SEMANTIC_FIX`。

实现、final 与 terminal quick/baseline 均有 clean/stable provenance；RV/LA official
均完整执行 24/24 groups、2544/2544 cases、0 infrastructure error，并保留为真实语义
`FAIL`（115/159 findings），没有宣称 official/full PASS。两镜像哈希未变、overlay 无
残留，独立 terminal review 为 0 Blocker / 0 Major / 0 Minor / 0 Nit，稳定化分支已普通
push 且远端 head/权威基线精确验证。未修改或推进 `main`，未开始 Goal B；PR Ready 前
仍须真实人工负责人完成理解与复核声明。
