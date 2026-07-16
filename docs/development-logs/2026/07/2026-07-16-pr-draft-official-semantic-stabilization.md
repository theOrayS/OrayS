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
head_commit: "e7fe68a1ebe9a5a942dfc48fcb7ea1e4b6860102"
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

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex（GPT-5 系列，精确子版本未知） | 合同阅读、Goal A 证据审计、failure identity 聚类、B-SPLICE-001 设计/编码/测试/文档编排 | 本计划、开发日志、`user/shell/runtime_smoke/semantic_smoke.rs`、semantic evidence manifest/fixtures、`user/shell/src/uspace/fd_table.rs`、`fd_pipe.rs`、axfile ordered-lock tests、file-object guard/unit、runner/manifest canonical count | 采用通用 pipe 行为复现，拒绝 official case/libc/path 特化；按 pinned Linux 路径修正 errno 优先级，以既有 ordered-lock/OpenFileRef 抽象实现；保留真实 red、INFRA_ERROR 和失败的 standalone make 记录 | summary/raw-log 哈希、parser replay、静态/mutation/unit、定向 RV/LA runtime、clean quick/baseline/fresh official/full、两轮独立只读 review | 待人工 PR 负责人确认 |

交互摘要或记录位置：本日志记录关键决定、命令、结果、反证和取舍；不提交完整对话、
凭据、隐私数据或无关主机信息。

# 6. 外部参考与许可证

| 来源 | 版本/commit | 借鉴范围 | 许可证 | OrayS 修改 | 记录/文件 |
|---|---|---|---|---|---|
| Linux `fs/splice.c` | `torvalds/linux@37e2f878a7a660a216cc7a60459995fefd150f25` | 仅核对 syscall 的 `len`/flags/fd/offset 优先级、同 pipe `EINVAL` 和 pipe-to-pipe 双锁迁移语义；未复制代码 | GPL-2.0-only | 用 OrayS 既有 `OpenFileRef`、ring buffer、wait queue 和 ordered mutex helper 独立实现 | `user/shell/src/uspace/fd_table.rs`、`user/shell/src/uspace/fd_pipe.rs`、本日志；来源：<https://github.com/torvalds/linux/blob/37e2f878a7a660a216cc7a60459995fefd150f25/fs/splice.c> |
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
获得 official/full 或 merge-ready 结论。Goal B 最终完整 diff 仍须再次独立复核。

# 9. 已知限制、后续工作与回滚

## 已知限制

- 当前状态为 `IN_PROGRESS`；首轮修复后的双架构 fresh official 已完整运行但仍为语义
  `FAIL`，full-all 未运行，不具备 Ready/merge 条件。
- B-SPLICE-001 的扩展先红、errno/边界/生命周期/并发补测、修订实现、双架构定向
  green、cluster 复审、clean quick/baseline 与 fresh official delta 已完成；14 个目标
  identity/架构消失，但 `tee02`/`vmsplice01` 仍失败，且 RV/LA 均有新增 failure
  identity，故必须进入下一 test-only red，不能把本轮计为 cluster PASS。
- 原始 official artifact 被 Git 忽略；本日志用 run ID 与 SHA-256 固定身份，但若本地
  artifact 丢失必须 fresh 重跑，不能用摘要补造原始证据。

## 后续工作

先在 B-SPLICE-001 内完成 `tee` 非 pipe EINVAL 与大于 pipe 容量的 vmsplice→splice
进展/数据完整性 test-only red，再做最小生产修复、双架构定向 green、复审和新的 clean
promotion。新增时序/watchdog identity 分别留给 poll/time/process cluster，首次失败
证据不由诊断重试覆盖。

## 回滚方式

计划/日志可用普通反向提交回滚；后续生产修复保持一个根因一个逻辑提交，必要时使用
普通 revert。禁止 `git reset --hard`、破坏性 rebase、force-push 或覆盖他人改动。

# 10. 最终摘要

尚未完成。当前唯一状态为 `IN_PROGRESS`，没有宣称 official/full PASS、PR Ready 或
Goal B 终态。
