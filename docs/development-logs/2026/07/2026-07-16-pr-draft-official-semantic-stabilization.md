---
title: "PR draft: official semantic stabilization"
date_started: 2026-07-16
date_completed: null
status: draft
pr: null
branch: "stabilize/post-integration-gates-20260716"
authors: ["OpenAI Codex (AI-assisted; human owner pending)"]
reviewers: ["OpenAI Codex independent read-only reviewer (Raman)"]
base_commit: "ac36481d6052457433b4d1ab5f2a5fd40a20df26"
head_commit: "789f9a3dd725185a85c85c16502bf86d53dd873c"
capability_domains: ["process", "scheduler", "memory", "filesystem", "fd", "poll", "signal", "linux-abi", "riscv64", "loongarch64"]
---

# 1. 背景与目标

## 背景

Goal A 已在精确 clean、已推送的分支 HEAD 上达到唯一终态
`READY_FOR_SEMANTIC_FIX`：统一 runner、可信官方 case plan、双流捕获、严格 parser 和
证据 provenance 已闭环。其 fresh official 运行完整执行 24 个 group、2544 个 case，
但诚实保留 RV 114 条、LA 157 条语义 finding，因此未宣称 official/full PASS。

本 Goal B 只处理这些真实内核/Linux ABI 语义失败。长期合同为根 `AGENTS.md`、
`.codex/tasks/SESSION_GUIDANCE.md`、
`.codex/tasks/GOAL_B_SEMANTIC_STABILIZATION.md` 与 `.codex/tasks/SAFETY_SCOPE.md`；
活动执行计划为
`docs/plans/active/official-semantic-stabilization.md`。

动态验证仅限授权本地仓库、只读 backing image 的 disposable overlay 与隔离 QEMU
guest；不接触外部目标、凭据、持久化、规避检测或漏洞武器化。安全范围文件是背景与
边界说明，不放宽模型、平台、仓库或测试诚信政策。

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

## 2026-07-16 — Checkpoint 2：唯一 failure identity 清单

- 从 Goal A fresh RV/LA stdout 的 official group、LTP START/RESULT/END、libctest FAIL 和
  group FAIL 边界提取 identity，而不是按 parser finding 数计数。
- RV LTP 为 glibc 13、musl 21；LA LTP 为 glibc 16、musl 39。两架构共同 33 个
  `group + case` identity，RV 独有 `ltp-musl/nice04`，LA 独有 22 个 time/epoll/
  scheduler/readlink identity。
- 双架构 glibc libctest identity 完全相同：static 18、dynamic 20，共 38；两种链接的
  `setvbuf_unget` 均 timeout。RV 另有 `cyclictest-musl` group 137/900 秒，LA 无 generic
  group failure。
- 所有 TCONF/TBROK/TFAIL、timeout、signal reason 与非零 case code 均保留。初始捕获中
  的 LTP、libctest 和 generic failure identity 已全部进入活动计划的 cluster 清单。

## 2026-07-16 — Checkpoint 3：B-SPLICE-001 回归先红与根因

- 测试提交 `bfff16ea4d0fc651245ce6145fd2a46d47a08619` 在已有双架构 freestanding
  runtime smoke 中加入通用 `pipe2 -> write -> splice -> read -> close` 行为断言，并将
  同一有序 marker 加入 RV/LA manifest 与 parser fixtures。测试不使用官方 case 名、
  镜像路径、libc 或架构专用判断；两架构只在既有 syscall ABI 汇编边界选择寄存器。
- 测试代码提交前：`cargo fmt --all -- --check`、competition semantic-evidence guard、
  33 项 competition evidence unit、75 项 semantic evidence unit 均 PASS。一次把两个
  unit 文件同时传给单文件入口的命令以 usage error/退出 2 结束，随后按入口契约拆分；
  该参数错误没有被记为测试 PASS。
- 首次在脏树调用 canonical runner 被 clean-worktree guard 以 infrastructure error/退出
  2 拒绝；这不是语义 red。建立仅含测试的 clean `bfff16ea` 后，RV
  `evidence-runtime` 外层 planned/executed/completed=1/1/1、FAIL 1、timeout/crash/infra=0；
  内层两个 build PASS，唯一 runtime instance 在 `USER_FAIL splice_pipe` fail closed。
- 为保留被后续绿色构建覆盖的内层明细，在 `/tmp` detached `bfff16ea` 工作树重放同一
  内层 manifest，并把生成证据写入 ignored red run 目录；repository before/after clean、
  revision 精确为 `bfff16ea`，2 pass / 1 error，raw log 明确含
  `USER_FAIL splice_pipe` 与 `HARNESS_FAIL ... guest_nonzero_exit`。临时工作树随后 clean
  移除，未切换或改写当前分支。
- 源码根因：`FdEntry::Pipe` 在文件对象迁移后持有 `OpenFileRef`；但
  `validate_splice_input/output` 仍用空 slice 调用只接受旧 entry 路径的
  `FdTable::read/write`，因此合法 pipe 在实际传输前固定得到 EBADF。`splice02` 随后向
  未被 drain 的 pipe 写 1 MiB，形成基线 30 秒 timeout/TBROK。tee/vmsplice 官方 case
  的准备阶段也先调用 splice，因此在当前 official delta 前不把它们另算成已修复根因。

## 2026-07-16 — Checkpoint 4：B-SPLICE-001 生产修复与双架构定向 green

- 生产提交 `b9d90a15a5b9a16421656663e196393ed88feabb` 仅修改
  `user/shell/src/uspace/fd_table.rs`：Pipe 分类保留 non-null offset 的 `ESPIPE` 优先级，
  随后直接以 `PipeEndpoint::readable/writable` 验证方向并对错误端返回 EBADF；Pipe 的
  空缓冲预验证改为 `Ok(())`，RegularFile 与 Stream 路径保持原样。
- 提交前 File object event core guard、24 项对应 unit、competition evidence guard、
  `cargo fmt --all -- --check` 与 `git diff --check` 均 PASS。
- exact clean `b9d90a15` 上，RV 与 LA `evidence-runtime` 外层均 1/1 PASS；内层固定构建、
  smoke-harness 构建、真实 QEMU runtime 各 3/3 PASS。两架构均观察到恰好一次且有序的
  `ASSERT splice_pipe PASS`、`USER_PASS`、`HARNESS_PASS status=0`、`SHUTDOWN`，无
  skip/blocked/timeout/fail/error、无残留进程，repository provenance clean/stable。
- 该定向 green 证明最小行为回归与双架构构建通过，不等于 quick/baseline、official 或
  full 门禁。独立只读 cluster review、clean quick/baseline 和 fresh official delta 仍待完成。

## 2026-07-16 — Checkpoint 5：首轮只读审查拒绝放行

- 独立只读 reviewer 对 `b9d90a15` 及其测试/证据给出 `0 Blocker / 3 Major / 0 Minor /
  0 Nit`，明确不放行进入 quick/baseline promotion 或 official delta。已有 RV/LA green
  仍是 exact-clean 真 guest 证据，但只足以证明原 happy path，不覆盖完整 cluster 合同。
- Major 1：pipe-to-pipe 仅比较 `fd_in == fd_out`，无法识别 pipe2 两端或 descriptor alias
  指向同一底层 buffer；验证和 endpoint clone 分处两次 fd-table 临界区，存在共享 table
  的 close/reuse TOCTOU；目标容量预检、source 消费和 destination 写入分离，在并发填满
  或关闭目标后可能返回 EAGAIN/EPIPE/EINTR 且丢失已消费的 source 数据。
- Major 2：当前在 `len == 0` 之前检查 flags/用户 offset，并在固定 fd 和识别 pipe 之前
  解引用 offset；因此零长度、无效 fd 加坏指针、pipe 加坏 offset 指针的 0/EBADF/ESPIPE
  优先级可能被 EINVAL/EFAULT 覆盖。
- Major 3：首轮回归只有两个独立 pipe 的单线程 success/payload/close；尚未覆盖 EBADF、
  ESPIPE、EINVAL、零长度、同 backing、close/reuse、目标已满后 source 保存和 reciprocal
  并发锁序，未满足 Goal B 对 success/errno/boundary/cleanup/concurrency 的组合要求。
- 处置：不运行 promotion quick/baseline，不运行 official。先提交审查 checkpoint，再以
  test-only commit 扩展通用行为回归并在 `b9d90a15` 生产基线上保存真实 red；随后才修改
  pipe 通用实现、双架构定向 green 和再次只读审查。parser、runner、manifest 结果语义、
  blacklist 与官方计划均不弱化。

## 2026-07-16 — Checkpoint 6：扩展先红、原子迁移与入口错误顺序

- 测试提交 `ad9d1ab5` 将 splice smoke 扩为通用组合：`len == 0` 对 flags/fd/offset 的
  优先级、无效 fd 加坏 offset 的 EBADF、pipe 加坏 offset 的 ESPIPE、错误方向 EBADF、
  同 backing EINVAL 且 source 保留、成功 payload、close/recreate/reuse、目标 pipe 满且
  `O_NONBLOCK` 时 EAGAIN 且 source 保留，以及全部 fd 清理。test-only clean RV run
  `goalb-b-splice-001-review-red-rv-1` 真实进入 guest，outer 1/1 FAIL，inner 2 pass /
  1 error，raw console 在第一条新增 zero-length 优先级断言处出现 `USER_FAIL`；这不是
  静态注册失败。
- 生产提交 `ae446dbf` 在同一 fd-table 临界区 snapshot 两个 slot identity 与
  `OpenFileRef`，以共享 ring-buffer `Arc` 身份拒绝同 backing；pipe-to-pipe 数据搬运、
  source consume、destination append 和 buffered 计数在 canonical ordered 双锁临界区
  原子完成。destination full/closed 时不会预先消费 source；endpoint `Drop` 也在同一
  ring lock 下更新最后 peer 计数，使 close 与 splice 串行化。
- syscall 入口按核对的 Linux `fs/splice.c` 路径调整为：zero length、flags、两 fd、
  pipe offset 拒绝、`off_out` copy、`off_in` copy，再做方向与同 backing 检查。非 pipe
  路径保持原有实现，并用 slot identity 复核避免 close/reuse 后访问错误对象。
- reciprocal lock-order unit 不再顺序模拟，而是用两个真实 host thread 与 Barrier 同时
  执行 A→B/B→A 迁移并 join；runtime smoke、静态 guard 和 mutation unit 将 errno、
  preservation、生命周期、双锁 helper 与 `O_NONBLOCK` 行为绑定到实现。没有新增
  `unsafe`、绝对路径、测例名分支、blacklist、skip 或 parser 弱化。
- 提交前 file-object guard、33 项 file-object unit、25 项 axfile unit、competition
  guard、33 项 competition unit、75 项 semantic-evidence unit、test-asset integrity、
  `cargo fmt --all -- --check`、`git diff --check` 以及 RV/LA kernel smoke 均通过。
  一次 `make pr2-check` 在 Python guard/unit 已通过后，cargo child 因该 standalone
  make 入口没有导出所需 `.axconfig.toml` 而以 101 退出；该次记为 `ERROR`，随后使用
  与仓库测试相同的 fallback dummy config 直接运行 `cargo test -p axfile --lib`，25/25
  PASS。未把失败的 make 调用改记为通过，也未在本 cluster 修改无关 Makefile。

## 2026-07-16 — Checkpoint 7：canonical 计数闭合、双架构 green 与复审清零

- 在 `ae446dbf` 上首次请求 exact-clean RV evidence 时，runner 在启动 guest 前 fail
  closed：`unit.file_object_event_core` 实际 canonical tests 为 33，但 runner/manifest
  仍声明 24。该次退出 2、分类 `INFRA_ERROR`，没有创建 output artifact，不能作为
  语义 red 或 green。
- `e7fe68a1` 只把 `test/run_suite.py` 与 `test/suite_manifest.json` 的精确 expected count
  从 24 同步为实际 33，没有改变 case 选择、skip、parser 或结果语义。随后 `--list`
  发现 59 个 case，file-object unit 33/33、suite-runner unit 135/135、test-asset integrity
  guard/36 项 unit 与 evaluator runner/parser integrity 24/24 均通过。
- exact-clean `e7fe68a1` 的 RV run `goalb-b-splice-001-review-green-rv-2` outer 1/1 PASS、
  inner 3/3 PASS，92.512 秒；LA run `goalb-b-splice-001-review-green-la-2` outer 1/1 PASS、
  inner 3/3 PASS，88.036 秒。两架构 raw console 都恰好一次且有序包含
  `ASSERT splice_pipe PASS`、`USER_PASS`、`HARNESS_PASS status=0`、`SHUTDOWN`；exit 0、
  child reaped、cleanup complete、无残留进程，repository before/after clean、revision
  精确且 provenance stable。
- 第二轮独立只读 reviewer 对 `b9d90a15..e7fe68a1`、red/green artifact 和三条旧 Major
  逐项复核，结论为 `0 Blocker / 0 Major / 1 Minor / 0 Nit`。旧 Major 1 的对象身份、
  生命周期、双锁原子性与 close 串行化，Major 2 的 Linux errno 优先级，Major 3 的
  errno/boundary/cleanup/concurrency 组合均已关闭。唯一 Minor 是本日志与活动计划仍停留
  在首轮状态；本 checkpoint 正在同步。该结论仅放行 clean quick，再通过后放行
  baseline，不等于 official/full 或 merge-ready。

## 2026-07-16 — Checkpoint 8：clean promotion 与 fresh official delta 未获晋级

- 文档 checkpoint `6dd0f0e0` 的 worktree 在全部 promotion 运行起止均 clean，runner
  commit/final commit 均精确为 `6dd0f0e0`、dirty/final dirty 均 false、provenance
  stable。quick 完整执行 45/45/45 且 45 PASS，summary SHA-256 为
  `4a504d8135a2ea030373e89a3295a416f843965a528e5f07b7a8b3356ca58808`。
- baseline 首次只把 QEMU 9.2.4 放入 PATH，56 项 PASS，但
  `baseline.clippy_loongarch64` 因系统 clang14 不支持 LoongArch64 target 而成为唯一
  `INFRA_ERROR`；planned/executed/completed 为 57/56/57，summary SHA-256
  `6bc0ba54a9dd003f1a07e472589fb70f31f163de6f0ca20df18abe7970e6b1e1`。该失败完整保留，
  未被后续结果覆盖。显式加入已核验 clang 21.1.8 与 `LIBCLANG_PATH` 后，第二次
  baseline 为 57/57/57 PASS，summary SHA-256
  `f2e0a3d84d87321cec19df781c9a997544d3fd229767fe8acf5884d5b8fc4dda`。
- fresh RV official 完整执行 24/24/24 groups、2544/2544/2544 cases，child rc 0、
  `error_count=0`，但按明确非通过输出返回 FAIL。finding 从 Goal A 114 降至 87；两套
  libc 的 `splice01/02/04/05/06`、`dirtypipe`、`tee01` 均转为 code 0，`tee02` 仍 code
  1、`vmsplice01` 仍 code 2/30 秒 TBROK。相对 Goal A 新增两套 libc `epoll_wait02`
  以及 glibc `nanosleep01`；原始计时只略超 LTP 阈值，但仍是不能隐藏的真实 TFAIL。
  summary/stdout/stderr SHA-256 分别为
  `16a1dbc198f059d08201d9a7c70ff98de63fea1661dcae5a5f7a0035cb88fb24`、
  `fed44619ca7331ceefc129b709ccb71b0c8f5796e84bcc9df230c1e56990f5c9`、
  `5485c60156b9b66096128434b2e3ec9b7baeda458de3efd29b60f78efa52f6c0`。
- fresh LA official 同样完整执行 24/24/24 groups、2544/2544/2544 cases，child rc 0、
  `error_count=0`，顶层 FAIL。finding 从 157 降至 127；上述 14 个 splice `group+case`
  identity 同样消失，`tee02`/`vmsplice01` 两 libc 仍失败。LA 基线本已有
  `epoll_wait02`/`nanosleep01`；本轮真正新增的是 musl `kill02`：打印两条 TPASS 后未
  结束，触发 180 秒单例 watchdog，code 137。summary/stdout/stderr SHA-256 分别为
  `d57ab624cceb8a3cb61823b06fae713e47b0786617cd76db07b96851b8475a18`、
  `07b7a18ec699a2e1e8b7b4d055d79fb4a2ad4bb875a2c640c2bcd7361b0443be`、
  `70359007681697f46f10fe1d488127b0334814afbe173f02217935a8ec1ae103`。
- 两轮 official 均清理 qcow2 overlay，镜像只读哈希不变。finding 总数下降只证明首批
  修复有可度量效果；由于目标 case 未全清零且出现新增 identity，本 cluster 状态为
  `DELTA_FAIL_NEXT_RED`，不运行 full、不宣称 official PASS 或 merge-ready。
- 上游 LTP 20240524 源确认 `tee02` 的三条通用 errno 合同：任一参数非 pipe 或两端为
  同一 pipe 均应 `EINVAL`；当前前两条误报 EBADF。`vmsplice01` 用 128 KiB iovec，循环
  poll 可写端、部分 vmsplice，再以 splice drain 到 regular file 并校验完整数据；当前
  四个架构/libc 组合均稳定在 30 秒 watchdog。下一步先把这些行为写成通用 guest red，
  不按官方 case 名、libc、架构、路径或固定评测顺序特化。

## 2026-07-17 — Checkpoint 9：首次 tee 假设、vmsplice partial progress 先红与双架构 green

- 按新增执行边界完整读取 `.codex/tasks/SAFETY_SCOPE.md` 54 行，并与根 `AGENTS.md`、
  session guidance 和 Goal B 合同共同采用。当前工作仍只涉及本地授权代码与隔离 QEMU
  guest；没有外部扫描/连接、凭据操作、持久化、规避检测或漏洞武器化。
- test-only 提交 `8e48e853` 当时把三种 tee 场景均假设为 `EINVAL`：live
  console/pipe 两种 non-pipe 组合与同 backing pipe；又用 64 KiB vector
  加一字节第二 vector 跨越 OrayS 最大 pipe capacity，循环接收 blocking vmsplice 的
  partial count、drain 并逐字节校验顺序，最后关闭全部 fd。测试不含官方 case 名、libc、
  架构或评测路径分支；RV/LA freestanding object 均编译通过，file-object guard 与
  33 项 mutation unit 通过。
- exact-clean `8e48e853` 的首次 canonical RV run 外层 1/1 FAIL，build 2 PASS、runtime
  在新增段立即打印 `USER_FAIL splice_pipe` 后以 guest nonzero fail closed。因为后续同
  架构运行会更新默认 inner 目录，另在隔离本地 clone 精确 checkout 同一 commit，以新
  run ID 重现：outer 1/1 FAIL、inner 2 pass / 1 error，cleanup/reap 完整；summary、inner
  evidence、raw SHA-256 分别为 `659da15c72b3f734c444d22810b1d5af4c297b1466ccfa7b396163cc6bc8df24`、
  `a5c1c696b84036cb762bac8d1b7892b08923b606701b6c736b661e694372fca4`、
  `b043a4f522ba9fbc1efab411a1746bd70cf6918b6f2dc51e464bfb1c17ca5ba5`。
- `44a25cff` 按上述首次假设把 tee-only helper 的 live non-pipe 结果改为 `EINVAL`；该
  假设后来被独立 review 否定，不能作为 tee 语义证明。该 exact-clean commit 的 RV run
  越过当时的 tee 断言后由 10 秒 guest watchdog
  打印 `HARNESS_FAIL reason=guest_timeout`，证实剩余 red 是 vmsplice 进展性而非 build、
  parser 或 harness。隔离 clone 重现的 outer/inner/raw SHA-256 分别为
  `7708726ec1f7f7cca8db636b97ef68e6aedc338d1442951a61ced2559d93eb4d`、
  `6e36b497a3b97d694647dd599dc98ea0a218c1af4aa4845ef9728ffe7b178fff`、
  `73138c31269deabe6a044964c5ea75658d67601267295e2af2c7b80689c0e77e`。
- 根因是 `sys_vmsplice` 在一个 iovec 恰好填满 pipe 后已有累计进展，却继续以 blocking
  模式处理下一 vector；没有 reader 能在 syscall 返回前 drain，形成自阻塞。
  `64af8ac4` 只在 `total > 0` 后把后续容量探测视为 nonblocking：有空间仍继续复制，
  已满时既有 EAGAIN 分支返回累计 count；首次字节仍可按 blocking 合同等待。guard 与
  mutation 绑定 production call site、累计返回和真实跨 64 KiB runtime 数据检查。
- exact-clean `64af8ac4` 的 RV/LA `evidence-runtime` 外层均 1/1 PASS，内层均 3/3
  required PASS。RV/LA 分别耗时 95.629/91.367 秒；所有 marker 恰好一次且有序，
  `USER_FAIL`/`HARNESS_FAIL`/panic 均为 0，process cleanup/reap 完整，runner 起止 clean、
  provenance stable。outer SHA-256 分别为
  `42d1a4604d682c36fa3f444856738122d2ca8698aa2a4e6a0d9b2c57ccee0a1f`、
  `4d236410a60e93fd1989b66117ed7a316827f7b4411544413dd798ae7e85f42b`；inner/raw 分别为
  RV `9d25d7c05f8bf42bbb98889d351317f40be1fd4044b0d4920724491ad52867ca` /
  `21c47a936fa017c00cd45c0cb3302cf240377b7b00a1edbdb68ebea81f3a37f2`，LA
  `a1540da0248d78c65643fa1b3630d1a0ec70cbd5bf5caaca767c1f416d5697e4` /
  `a1ae5efd47c1cfe461ac64aec7ff519adbcb17d1c0b04a19ecd8432c5c63f8fe`。
- 当前仅为 targeted green，正在等待独立只读 review；尚未运行本批 quick/baseline 或
  fresh official，故不宣称 `tee02`/`vmsplice01` 已从官方身份矩阵消失，也不宣称
  official/full/merge-ready。

## 2026-07-17 — Checkpoint 10：独立审查否定错误 tee 断言并完成顺序修正

- 对 `4529a255..05a421b3` 的独立只读 review 结论为
  `0 Blocker / 1 Major / 1 Minor / 0 Nit`，明确拒绝 promotion。Major 指出 Linux tee
  可见顺序应为 invalid flags→`EINVAL`、`len == 0`→0、输入 fd、输出 fd、输入可读/
  输出可写→`EBADF`，之后才按 non-pipe/same backing 返回 `EINVAL`；`44a25cff` 缺少
  zero-length fast path 且过早分类对象类型。Minor 指出本日志“后续工作”已过时。
- 因此 `8e48e853`/`44a25cff` 的原始 run 和哈希继续保留为真实执行历史，但不再支持 tee
  正确性的结论；尤其 fd 1 是 write-only，`tee(1, pipe_write)` 应先因输入方向返回
  `EBADF`。同一批 vmsplice 跨 64 KiB iovec 的 watchdog red、`64af8ac4` partial-progress
  修复及其双架构 green 不依赖该错误断言，仍然有效。
- test-only `5c365cc6` 依次覆盖 invalid flags、zero length、输入/输出 fd lookup、错误
  access mode、正确方向 non-pipe 与 same-pipe backing。exact-clean RV run
  `goalb-b-splice-002-red-tee-order-rv-1` 外层 1/1 FAIL、内层 2 PASS / 1 ERROR，耗时
  93.360 秒；raw 依次出现 `HARNESS_START`、`USER_START`、前两项 PASS、
  `USER_FAIL splice_pipe`、`HARNESS_FAIL reason=guest_nonzero_exit`、`SHUTDOWN`，无
  timeout、panic、trap 或 infra。outer/inner/raw SHA-256 分别为
  `37eff04986deecb08707a3dd2c52a316c7e0374e054a6a130870b7712fd3a7e6`、
  `4abfe2624ce1027e131fcb7122e4a95c4200f504326b58404d19a7755a6d084a`、
  `726c11b5b641fa82cd4cc88d58399fd788af861dcf8e4d1822f4f49f01667fa8`。
- `d75a4d8e` 保持 flags 最先并增加 zero-length fast path；随后在同一个 fd-table 锁内按
  输入再输出顺序固定对象快照和 access mode，先检查 source readable / destination
  writable，再检查两端是否 pipe，最后由 pipe helper 判 same backing。实现无新增
  `unsafe`、依赖、runner/parser/manifest/blacklist 或 case/libc/arch/path 特化。静态
  guard 与 mutation 绑定上述顺序，33/33 unit 及 RV/LA kernel smoke 均通过。
- exact-clean `d75a4d8e` 的 RV/LA targeted run 分别耗时 97.404/92.597 秒，外层均
  1/1 PASS、内层均 3/3 required PASS，marker 唯一有序且 cleanup/reap 完整。RV
  outer/inner/raw SHA-256 为 `f31304125796080f70fce3a11d53478342b618c1b95176be22fbf92205415190` /
  `172ef97722b278ac97acb260ee548fdba1e9c737b15b8df32bf558731e4235c5` /
  `0be3448410958a06935e989d876551cbdf665aab6d454409ac5f41cb349cb012`；LA 为
  `e1af85a332a5ddda0b1f954100adf804fc34f8ee1d156b9519eed880a7872b45` /
  `4da7d72c0498015e970c36e98bb80f81a0a3712b05e848bd2e6390ba8d54b241` /
  `caba5e859ad17796d3f0761fb0bdc8bf7bbe0da74c2d7ba839251e466331ec99`。
- 当前仍只达到 corrected targeted green，等待同一独立 reviewer 二次复核；尚未运行
  corrected exact-clean quick/baseline 或 fresh official，不宣称 cluster PASS、
  official/full PASS、Ready 或 merge-ready。

## 2026-07-17 — Checkpoint 11：二次 review 发现设备 access-mode 表示缺口

- 对 clean `04e4cb91`、修正范围 `05a421b3..04e4cb91` 及三组新 artifact 的独立只读
  review 结论为 `0 Blocker / 1 Major / 1 Minor / 0 Nit`，继续拒绝 promotion。reviewer
  重算全部 outer/inner/raw SHA-256 与 inner 10/10 artifact size/hash，确认三组均
  planned=executed=completed=1、起止 clean、revision 精确、provenance stable、cleanup/
  reap 完整；red/green 分类与 marker 真实。该证据只证明已覆盖路径，不覆盖下述 Major。
- Major：`FdEntry::DevNull` 与 `FdEntry::Rtc` 没有 status-flags payload，open 和 fork/dup
  路径会丢弃 access mode；`tee_fd_snapshot` 因而把二者硬编码为 readable+writable。
  `O_RDONLY /dev/null` 作为 tee 输出端、`O_WRONLY /dev/null` 作为输入端应先返回
  `EBADF`，当前却进入 non-pipe 分类返回 `EINVAL`。这是本批首次依赖该表示作 Linux
  `f_mode` 优先级判断，不能以既有 read/write adapter 同样宽松为由豁免。
- 其余变体逐项核验通过：stdio、DevZero/Random/CpuDmaLatency、BlockDevice、File、
  Memfd、ProcTimerSlack、POSIX MQ、ProcSys 与 pipe 保存 mode；目录类/MemoryFile/
  ProcPagemap/Inotify 固定只读；O_PATH 双向均 false；socket/eventfd/epoll/timerfd/
  signalfd/pidfd 的固定双向语义与当前对象模型一致。
- Minor：`8e48e853` 虽引入跨 iovec vmsplice 测试，但其 raw 在 tee `USER_FAIL` 后退出，
  没有执行到 vmsplice；真正的 runtime red 是 `44a25cff` 越过 tee 后的 guest watchdog。
  `64af8ac4` partial-progress fix 与双架构 green 仍有效，计划已更正证据归属。
- 下一步先增加真实 `/dev/null` O_RDONLY/O_WRONLY tee 方向矩阵，只提交测试并保存首次
  exact-clean RV red；随后让 `DevNull`/`Rtc` 持久保存 `fcntl_status_flags(flags)`，在
  fork/dup 与 tee snapshot 中保留并使用 mode，并以 guard/mutation 约束。完成 RV/LA
  targeted green 与第三次 0B/0M review 前，不运行 quick/baseline 或 official delta。

## 2026-07-17 — Checkpoint 12：真实 `/dev/null` access-mode red

- test-only `324e3f4c` 在 freestanding guest 中增加 `openat` wrapper，用真实
  `/dev/null` O_RDONLY/O_WRONLY 描述符验证四种组合：错误方向 source/destination 必须
  `EBADF`，正确方向 live non-pipe 必须 `EINVAL`。wrapper 的新增 test-only `unsafe`
  仅包裹两个架构共用的 raw syscall binding，并记录 NUL pathname 生命周期与无 O_CREAT
  时 mode 参数不使用的不变量；没有 official case/libc/arch/path 分支或生产改动。
- 预提交 `cargo fmt --all -- --check`、file-object guard、RV/LA smoke kernel build 均
  PASS。第一次误用 `python3 -m unittest test.unit.test_file_object_event_core` 因 `test/`
  非 Python package 返回 `ModuleNotFoundError`；保留为命令入口错误，随后按仓库真实入口
  `python3 test/unit/test_file_object_event_core.py` 执行 33/33 PASS，没有把前者改写成 PASS。
- exact-clean `324e3f4c` 首次 RV run `goalb-b-splice-003-red-tee-device-mode-rv-1`
  外层 1/1 FAIL、内层 2 PASS / 1 ERROR，耗时 91.365 秒，timeout/crash/infra 均为 0；
  outer/inner/raw SHA-256 为 `6cbc45dfdbb0234a3c662853ceaa047b8a198282ef5e7a6398df1b12c30fe49e` /
  `54ef1d6c456e2520076580b8a89c22fa6e3613e41cff9d9218046bd26a9c72e0` /
  `122306e1cb9d6ebf0288f23dec27180ce11769aa81ee79b7fa69b47037f622ad`。
  该 raw 的通用 `USER_FAIL splice_pipe` 未单独区分 open 与 mode 断言，故证据完整保留但
  不作为最终定位依据。
- test-only `789f9a3d` 只把设备 open/mode/close 失败拆成专用 marker，并把 marker 绑定到
  guard；格式、guard、33/33 unit 与 RV/LA build 再次 PASS。新 run
  `goalb-b-splice-003-red-tee-device-mode-rv-2` 在 clean exact commit 外层 1/1 FAIL、
  内层 2 PASS / 1 ERROR，耗时 91.737 秒；raw 明确依次到达 write/getpid PASS 后出现
  `USER_FAIL tee_device_mode`、guest nonzero、shutdown，证明 open 已成功且失败就是 mode
  errno。无 timeout/panic/trap/ENOSYS，process cleanup/reap 完整、provenance stable。
  outer/inner/raw SHA-256 为 `f476ff2286380c27439e20b180171166d4527fb6d8cb075a3900c0dbffb6b254` /
  `1465105d32df6a3315262af2f1dec56c0910ca8ef4fbd094e7ebd828a0fb9658` /
  `ae34561f938bfd9a70386a3c24f986a61a900b270da0d79f9848dc90eb072a45`。
- 当前状态为 `TARGETED_RED`；下一提交才允许修改 `FdEntry` mode 表示、构造/fork/dup、
  snapshot 与 guard/mutation。尚无修复后 green，不运行 quick/baseline/official。

## 2026-07-18 — Checkpoint 13：用户指令暂停 Goal B，完成独立缺陷文档收尾

- 用户明确要求不执行 Goal resume、不继续 Goal B 生产修复、不启动新的 official/full。
  当前 Goal B 因而保持非终态并标记为 `PAUSED_BY_EXPLICIT_USER_INSTRUCTION`；该暂停不把
  `TARGETED_RED`、official `FAIL` 或未归因 watchdog 改写为完成、阻塞或通过。
- 完整复读 `.codex/tasks/SAFETY_SCOPE.md`，并与 `SESSION_GUIDANCE.md`、
  `GOAL_B_SEMANTIC_STABILIZATION.md` 一同约束本次工作：仅限授权的本地 OS 竞赛内核与
  隔离 QEMU guest 防御性兼容性开发，不生成攻击载荷、外部利用步骤、凭据操作、持久化、
  规避检测或漏洞武器化内容。
- 核对分支、HEAD、worktree、活动计划、本日志、Goal A/B 机器 summary、raw failure
  records 与 `ac36481d..789f9a3d` 的 19 个线性提交。保留本日志和活动计划在本次开始前
  已存在的未提交 checkpoint 12 修改，没有 reset、丢弃或覆盖已有工作。
- 新增 `docs/defect-and-reliability-status-2026-07-18.md`，按“已修复（official
  delta）/已修复（targeted only）/部分修复/仍未修复/仅假设/偶发或待归因”分层汇总
  kernel 与 evidence-reliability 问题。文档明确保留最新 RV/LA official 87/127 findings
  的 `FAIL`、首次 baseline `INFRA_ERROR`、targeted watchdog/device red、TCONF/TBROK、
  libctest timeout/异常终止和所有复证缺口；不推断可利用性或安全严重度。
- 本 checkpoint 不修改 runner、parser、blacklist、官方镜像、生产实现、测试用例或测试
  判定，不启动 quick、baseline、official 或 full。纯文档验证结果在第 7 节追加。

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex（GPT-5 系列，精确子版本未知） | 合同阅读、Goal A 证据审计、failure identity 聚类、B-SPLICE-001 设计/编码/测试/文档编排 | 本计划、开发日志、缺陷/可靠性汇总、`user/shell/runtime_smoke/semantic_smoke.rs`、semantic evidence manifest/fixtures、`user/shell/src/uspace/fd_table.rs`、`fd_pipe.rs`、axfile ordered-lock tests、file-object guard/unit、runner/manifest canonical count | 采用通用 pipe 行为复现，拒绝 official case/libc/path 特化；按 pinned Linux 路径修正 errno 优先级，以既有 ordered-lock/OpenFileRef 抽象实现；保留真实 red、INFRA_ERROR 和失败的 standalone make 记录 | summary/raw-log 哈希、parser replay、静态/mutation/unit、定向 RV/LA runtime、clean quick/baseline/fresh official、两轮独立只读 review；full 未运行 | 待人工 PR 负责人确认 |

交互摘要或记录位置：本日志记录关键决定、命令、结果、反证和取舍；不提交完整对话、
凭据、隐私数据或无关主机信息。

# 6. 外部参考与许可证

| 来源 | 版本/commit | 借鉴范围 | 许可证 | OrayS 修改 | 记录/文件 |
|---|---|---|---|---|---|
| Linux `fs/splice.c` | `torvalds/linux@37e2f878a7a660a216cc7a60459995fefd150f25` | 仅核对 syscall 的 flags→zero-length→输入 fd→输出 fd→access mode→pipe type 顺序、offset 优先级、同 pipe `EINVAL` 和 pipe-to-pipe 双锁迁移语义；未复制代码 | GPL-2.0-only | 用 OrayS 既有 `OpenFileRef`、ring buffer、wait queue 和 ordered mutex helper 独立实现 | `user/shell/src/uspace/fd_table.rs`、`user/shell/src/uspace/fd_pipe.rs`、本日志；来源：<https://github.com/torvalds/linux/blob/37e2f878a7a660a216cc7a60459995fefd150f25/fs/splice.c> |
| Linux Test Project `tee02.c`、`vmsplice01.c` | tag `20240524` | 仅核对非 pipe/same-pipe errno 与 128 KiB vmsplice→splice 分段进展、数据完整性；未复制测试或实现代码 | GPL-2.0-or-later | 用既有 freestanding semantic smoke 独立构造通用行为回归 | 本日志及后续 runtime smoke；来源：<https://github.com/linux-test-project/ltp/blob/20240524/testcases/kernel/syscalls/tee/tee02.c>、<https://github.com/linux-test-project/ltp/blob/20240524/testcases/kernel/syscalls/vmsplice/vmsplice01.c> |

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
| `goalb-b-splice-001-red-rv-1` | `python3 test/run_suite.py --profile evidence-runtime --arch rv` | RISC-V64，test-only `bfff16ea` | 1 | FAIL | 94.50 s | outer 1/1 complete，summary SHA-256 `59005366a51baf2554f07dbd9738a99d0a48afa32c737e053f3dce5d3a30d6f9` |
| `goalb-b-splice-001-red-rv-1/semantic-rv64` | `semantic_evidence.py run ... --arch riscv64` | RISC-V64，detached `bfff16ea` | 1 | FAIL | 221.04 s | inner 2 pass / 1 error；evidence SHA-256 `8139dfd8d7e2a5e92586daea1e445082aa8f1098cb393b0b53a9a098efcd38a8`；runtime raw `c8c80b5d9dd5d038b292c2942d35c168d126642e6056b4ed2d402cf8200cf9a5` |
| `goalb-b-splice-001-green-rv-1` | `python3 test/run_suite.py --profile evidence-runtime --arch rv` | RISC-V64，`b9d90a15` | 0 | PASS | 92.74 s | outer 1/1、inner 3/3；summary SHA-256 `c16c3c3cf5e66111bbb668f370af42adb650cc4800e2871d8ea9eff45962b6ed`；inner evidence `61a4c675dfc042c32e5c7c038a4514ce9fea8682e7a58d4c46172abc11744b45`，runtime raw `0e77598b7920c244f8e8100c63e78cc353e8614035e97b1f2976f049e3422aae` |
| `goalb-b-splice-001-green-la-1` | `python3 test/run_suite.py --profile evidence-runtime --arch la` | LoongArch64，`b9d90a15` | 0 | PASS | 88.64 s | outer 1/1、inner 3/3；summary SHA-256 `d90c85133ab8db5c99127eed2a7426f1aaa9dbe24657f8fe9e59f53c700e72b7`；inner evidence `8aa79011b51932d3d0e8c623c8d3fa805da65c51143230481abba493ec721abd`，runtime raw `1b95c50a9c49eba13a7d5f074de1dd6c084b7f3d6288ddc69ad721032935d8a8` |
| `goalb-b-splice-001-review-red-rv-1` | `python3 test/run_suite.py --profile evidence-runtime --arch rv` | RISC-V64，test-only `ad9d1ab5` | 1 | FAIL | 93.469 s | outer 1/1 FAIL；summary `5168235f83ef9971082660af0a4a695a65fb372e69cb8bbd185c67f265a76e22`；inner 2 pass/1 error `e4eb3e6ebad7353d4be5aedc57c605976b96501b3a91ce54a84974dd4a601dba`；raw `3d58013d4a322c606864389d5ede6d88cb21d48ef995f2208e23b84305c5bcbc` |
| pre-guest canonical-count check | `python3 test/run_suite.py --profile evidence-runtime --arch rv` | exact-clean `ae446dbf` | 2 | ERROR | pre-guest | expected 24 / observed 33，分类 `INFRA_ERROR`；未创建 artifact，不计语义 red/green |
| `goalb-b-splice-001-review-green-rv-2` | `python3 test/run_suite.py --profile evidence-runtime --arch rv` | RISC-V64，exact-clean `e7fe68a1` | 0 | PASS | 92.512 s | outer 1/1 `b437714e3548008b46c265162d57230dab8f8f5f2f88b70762f8fbc7f819db52`；inner 3/3 `22ee6fbaa5fa535f44c170bb98fdfa2abcf174d0124260ac427cbcdb02805a17`；raw `081076a3bce4be77312015c5fedf169f4fe16b28b9f8a3562432d74a4d774a92` |
| `goalb-b-splice-001-review-green-la-2` | `python3 test/run_suite.py --profile evidence-runtime --arch la` | LoongArch64，exact-clean `e7fe68a1` | 0 | PASS | 88.036 s | outer 1/1 `1409dd5eb440a00056dd3abe81c25e348da0b8f1ede91c45c9f2134ef693742c`；inner 3/3 `82365414f49ed2a06e6521e36434109f34dceeac4340ae6d984fe85b1bf95393`；raw `3e56ea1ec2fe255b458dea4e7baf7fe32dd9e49d20704bd18dbb524475395592` |
| `goalb-b-splice-001-promotion-quick-1` | `python3 test/run_suite.py --profile quick` | exact-clean `6dd0f0e0` | 0 | PASS | 293.835 s | 45/45/45；summary `4a504d8135a2ea030373e89a3295a416f843965a528e5f07b7a8b3356ca58808` |
| `goalb-b-splice-001-promotion-baseline-1` | `PATH=<qemu-9.2.4> python3 test/run_suite.py --profile baseline` | exact-clean `6dd0f0e0` | 2 | ERROR | 792.210 s | 56 PASS / 1 INFRA_ERROR；系统 clang14 缺 LoongArch64 target；summary `6bc0ba54a9dd003f1a07e472589fb70f31f163de6f0ca20df18abe7970e6b1e1` |
| `goalb-b-splice-001-promotion-baseline-2` | `PATH=<qemu-9.2.4+clang-21> LIBCLANG_PATH=<llvm-21> python3 test/run_suite.py --profile baseline` | exact-clean `6dd0f0e0` | 0 | PASS | 720.220 s | 57/57/57；summary `f2e0a3d84d87321cec19df781c9a997544d3fd229767fe8acf5884d5b8fc4dda` |
| `goalb-b-splice-001-official-rv-delta-1` | `RV_TESTSUITE_IMG=<verified-rv> python3 test/run_suite.py --profile official --arch rv` | RISC-V64，exact-clean `6dd0f0e0` | 1 | FAIL | 4866.916 s | 24 groups / 2544 cases / 0 integrity error；114→87；summary `16a1dbc198f059d08201d9a7c70ff98de63fea1661dcae5a5f7a0035cb88fb24`；stdout `fed44619ca7331ceefc129b709ccb71b0c8f5796e84bcc9df230c1e56990f5c9`；stderr `5485c60156b9b66096128434b2e3ec9b7baeda458de3efd29b60f78efa52f6c0` |
| `goalb-b-splice-001-official-la-delta-1` | `LA_TESTSUITE_IMG=<verified-la> python3 test/run_suite.py --profile official --arch la` | LoongArch64，exact-clean `6dd0f0e0` | 1 | FAIL | 4644.286 s | 24 groups / 2544 cases / 0 integrity error；157→127；summary `d57ab624cceb8a3cb61823b06fae713e47b0786617cd76db07b96851b8475a18`；stdout `07b7a18ec699a2e1e8b7b4d055d79fb4a2ad4bb875a2c640c2bcd7361b0443be`；stderr `70359007681697f46f10fe1d488127b0334814afbe173f02217935a8ec1ae103` |
| `goalb-b-splice-002-red-tee-rv-1` | `PATH=<qemu-9.2.4> python3 test/run_suite.py --profile evidence-runtime --arch rv` | RISC-V64，test-only `8e48e853` 首次 run | 1 | FAIL | 97.309 s | outer 1/1、infra/timeout/crash=0；summary `7e748b14b178b70d7b59b8acc68183c04d9257df2ca001ad86b785d3ad75adcb`；后续由新 run ID 完整重现 inner/raw |
| `goalb-b-splice-002-red-tee-rv-repro-1` | 同上，隔离 local clone exact checkout | RISC-V64，test-only `8e48e853` | 1 | FAIL | 210.440 s | outer 1/1；inner 2 pass / 1 error；`USER_FAIL`/guest nonzero；outer/inner/raw `659da15c...` / `a5c1c696...` / `b043a4f5...` |
| `goalb-b-splice-002-red-vmsplice-rv-1` | 同上 | RISC-V64，tee-only fix `44a25cff` 首次 run | 1 | FAIL | 101.517 s | outer 1/1；guest 10 秒 watchdog；summary `24e78680f1c73ec76910a4a56daf9971fa02e09d1643980da02a499893593813`；后续完整重现 |
| `goalb-b-splice-002-red-vmsplice-rv-repro-1` | 同上，隔离 local clone exact checkout | RISC-V64，`44a25cff` | 1 | FAIL | 102.418 s | outer 1/1；inner 2 pass / 1 error；`HARNESS_FAIL reason=guest_timeout`；outer/inner/raw `7708726e...` / `6e36b497...` / `73138c31...` |
| `goalb-b-splice-002-green-rv-1` | 同上 | RISC-V64，exact-clean `64af8ac4` | 0 | PASS | 95.629 s | outer 1/1、inner 3/3；outer/inner/raw `42d1a460...` / `9d25d7c0...` / `21c47a93...`；marker 唯一有序、cleanup complete |
| `goalb-b-splice-002-green-la-1` | `PATH=<qemu-9.2.4> python3 test/run_suite.py --profile evidence-runtime --arch la` | LoongArch64，exact-clean `64af8ac4` | 0 | PASS | 91.367 s | outer 1/1、inner 3/3；outer/inner/raw `4d236410...` / `a1540da0...` / `a1ae5efd...`；marker 唯一有序、cleanup complete |
| `goalb-b-splice-002-red-tee-order-rv-1` | `PATH=<qemu-9.2.4> python3 test/run_suite.py --profile evidence-runtime --arch rv` | RISC-V64，test-only exact-clean `5c365cc6` | 1 | FAIL | 93.360 s | outer 1/1、inner 2 pass / 1 error；guest nonzero；outer/inner/raw `37eff049...` / `4abfe262...` / `726c11b5...`；无 timeout/crash/infra |
| `goalb-b-splice-002-green-order-rv-1` | 同上 | RISC-V64，exact-clean `d75a4d8e` | 0 | PASS | 97.404 s | outer 1/1、inner 3/3；outer/inner/raw `f3130412...` / `172ef977...` / `0be34484...`；marker 唯一有序、cleanup complete |
| `goalb-b-splice-002-green-order-la-1` | `PATH=<qemu-9.2.4> python3 test/run_suite.py --profile evidence-runtime --arch la` | LoongArch64，exact-clean `d75a4d8e` | 0 | PASS | 92.597 s | outer 1/1、inner 3/3；outer/inner/raw `e1af85a3...` / `4da7d72c...` / `caba5e85...`；marker 唯一有序、cleanup complete |
| `goalb-b-splice-003-red-tee-device-mode-rv-1` | `PATH=<qemu-9.2.4> python3 test/run_suite.py --profile evidence-runtime --arch rv` | RISC-V64，test-only exact-clean `324e3f4c` | 1 | FAIL | 91.365 s | outer 1/1、inner 2 pass / 1 error；outer/inner/raw `6cbc45df...` / `54ef1d6c...` / `122306e1...`；generic fail marker，保留但不作最终定位 |
| `goalb-b-splice-003-red-tee-device-mode-rv-2` | 同上 | RISC-V64，marker-only exact-clean `789f9a3d` | 1 | FAIL | 91.737 s | outer 1/1、inner 2 pass / 1 error；`USER_FAIL tee_device_mode`；outer/inner/raw `f476ff22...` / `1465105d...` / `ae34561f...`；cleanup/reap complete |
| docs-closeout-evidence-audit | 对本文引用的 Goal A/B summary、official stdout/stderr、device-mode inner/raw 执行 `sha256sum` | 既有本地证据 | 0 | PASS | <1 s | 15 个被引用文件均存在且哈希与文档一致；未生成或改写 artifact |
| docs-closeout-static-gate | `git diff --check`；三份文档 trailing-space/conflict-marker、必需章节、敏感模式和 >1 MiB 扫描 | 纯文档 | 0 | PASS | <1 s | 范围仅为新增缺陷汇总、既有活动计划和既有开发日志；无敏感模式、无大文件 |

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

审查人及结论：首轮独立只读 review 为 `0 Blocker / 3 Major`，不放行；第二轮对
`b9d90a15..e7fe68a1` 及证据复核为 `0 Blocker / 0 Major / 1 Minor / 0 Nit`。唯一 Minor
是计划和日志未同步，由本 checkpoint 修复；该 cluster 已获准进入 clean quick，仍未
获得 official/full 或 merge-ready 结论。第二批首次 review 对 `4529a255..05a421b3`
为 `0 Blocker / 1 Major / 1 Minor / 0 Nit`，否定错误 tee 顺序并拒绝 promotion。第二批
二次 review 对 clean `04e4cb91` 同样为 `0 Blocker / 1 Major / 1 Minor / 0 Nit`：主顺序
已正确，但 `DevNull`/`Rtc` access mode 丢失仍是 Major；vmsplice red 归属不精确是 Minor。
必须完成新 red/fix/双架构 green 并第三次取得 0B/0M 后才可 promotion。Goal B 最终完整
diff 仍须再次独立复核。

# 9. 已知限制、后续工作与回滚

## 已知限制

- Goal B 当前为非终态 `PAUSED_BY_EXPLICIT_USER_INSTRUCTION`；首轮修复后的双架构 fresh
  official 已完整运行但仍为语义 `FAIL`，full-all 未运行，不具备 Ready/merge 条件。
- B-SPLICE-001 首批扩展先红、修订实现、复审、clean quick/baseline 与 fresh official
  delta 已完成；14 个目标 identity/架构消失。第二批 vmsplice red/fix/双架构 green
  有效；首次 tee 断言被 review 否定后，已完成修订 test-only red、主顺序修复及
  双架构 targeted green，但二次 review 又发现 `DevNull`/`Rtc` mode 丢失 Major。新
  device-mode test-only red 已明确保存；生产 fix、双架构 green、第三次 review、clean
  quick/baseline 与 fresh official delta 均尚未完成，不能把 targeted green 计为
  cluster PASS。
- 上轮 official 新增的 RV `epoll_wait02`/`nanosleep01` 与 LA musl `kill02` 仍是真实
  未归因 failure identity；本批尚无官方证据证明其消失或不回归。
- 原始 official artifact 被 Git 忽略；本日志用 run ID 与 SHA-256 固定身份，但若本地
  artifact 丢失必须 fresh 重跑，不能用摘要补造原始证据。

## 后续工作

本次收尾结束后保持暂停，不自动恢复 Goal B。只有未来收到新的明确恢复指令，才让
`DevNull`/`Rtc` 保存并在 fork/dup 与 snapshot 使用 open mode，补齐 guard/mutation 与
双架构 targeted green。只有第三次独立 review 为 `0 Blocker / 0 Major` 才运行
exact-clean quick 与 baseline；其后 fresh RV/LA official delta 仍要求 `tee02`、
`vmsplice01` 消失且不新增 failure identity。新增时序/watchdog identity 分别留给
poll/time/process cluster，首次失败证据不由诊断重试覆盖。

## 回滚方式

计划/日志可用普通反向提交回滚；后续生产修复保持一个根因一个逻辑提交，必要时使用
普通 revert。禁止 `git reset --hard`、破坏性 rebase、force-push 或覆盖他人改动。

# 10. 最终摘要

尚未完成。当前状态为非终态 `PAUSED_BY_EXPLICIT_USER_INSTRUCTION`，没有宣称
official/full PASS、PR Ready 或 Goal B 终态；本次文档提交结束后不自动恢复 Goal B。
