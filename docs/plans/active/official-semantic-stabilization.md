# Goal B：官方语义稳定化执行计划

状态：`IN_PROGRESS`

开始日期：2026-07-16

执行分支：`stabilize/post-integration-gates-20260716`

Goal B 基线：`ac36481d6052457433b4d1ab5f2a5fd40a20df26`

权威集成基线：`09f4076ac151e0e7800103de724d9042230738b5`

## 1. 执行合同

本计划与仓库根 `AGENTS.md`、`.codex/tasks/SESSION_GUIDANCE.md`、
`.codex/tasks/GOAL_B_SEMANTIC_STABILIZATION.md` 共同构成持续执行合同。若记录、
提示或历史证据与这些文件冲突，以更严格的 fail-closed、双架构和 Git 安全要求为准。

Goal A 的唯一终态为 `READY_FOR_SEMANTIC_FIX`。Goal B 从其 clean、已推送的精确
HEAD 开始，只修复真实内核/Linux ABI 语义；不修改官方计划、结果解析规则或完整性
门禁来改变结论。

## 2. 目标与非目标

### 目标

- 从 fresh RV/LA official 原始捕获建立可追溯、去重的失败记录和根因 cluster。
- 为每个 cluster 先构造通用的最小行为复现，再增加回归测试并修复生产语义。
- 保持 RISC-V64 与 LoongArch64 的 Linux/POSIX 可见语义、ABI、errno、资源回收和
  并发行为一致；架构差异必须有明确边界和证据。
- 每个可测修复批次在 clean HEAD 上完成相称的 quick/baseline 与双架构验证；仅在
  预期有可度量变化时运行耗时 official，并保留前后 failure identity 差分。
- 最终使 quick、baseline、RV official、LA official、`full --arch all` 在同一可追溯
  候选历史上全部明确 PASS，完成独立只读审查、文档闭环和获授权后的普通 push。

### 非目标

- 不删除、跳过、黑名单化或重排官方测例，不弱化 parser、runner、timeout 或状态语义。
- 不根据测例名、二进制、libc、架构、路径、固定参数或运行顺序走特殊实现分支。
- 不伪造返回值、吞错、无条件成功，不把 `TCONF`、`TBROK`、`TFAIL`、timeout、panic、
  trap、空套件或环境错误记为 PASS。
- 不做无关重构、依赖/工具链升级、批量格式化，不修改镜像、外部评测计划或 `main`。
- 未取得本轮明确授权前不 push；任何 push 都不得 force、rebase、squash 或改写历史。

## 3. 已核验基线

- 本地 HEAD、upstream 与远端稳定化分支均为 `ac36481d...`，worktree clean；远端
  integration 为 `09f4076a...`，`main` 为 `921171ac...`。
- 精确当前 HEAD 的 Goal A terminal quick 为 45/45/45 且 45 PASS；baseline 为
  57/57/57 且 57 PASS；二者退出码 0、起止 clean、provenance stable。
- fresh RV official：24/24/24 groups、2544/2544/2544 cases、child rc 0、
  `error_count=0`、114 条语义 finding，顶层 `FAIL`/退出码 1。
- fresh LA official：24/24/24 groups、2544/2544/2544 cases、child rc 0、
  `error_count=0`、157 条语义 finding，顶层 `FAIL`/退出码 1。
- 当前 parser 对两份原始捕获重放得到相同的 114/157 finding 与 0 integrity error。
- RV/LA 镜像 SHA-256 分别为
  `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99` 与
  `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50`；
  QEMU 9.2.4、qemu-img 9.2.4 和 clang 21 的版本及二进制哈希与 Goal A 一致。

完整命令、summary/raw-log 哈希和历史非 PASS 见对应开发日志；本节不把 official
语义 `FAIL` 表述为通过。

## 4. 根因 cluster 动态表

此表只记录当前证据支持的假设；`Minimal repro` 未落地前不得进入生产修复。一个
测试可受多个症状影响，但最终必须归入一个可解释的主要根因 cluster，避免重复计功。

| Cluster ID | Arch | Test/status | Capability | Minimal repro | Hypothesis | Fix commit | RV delta | LA delta | Status |
|---|---|---|---|---|---|---|---|---|---|
| B-SPLICE-001 | RV+LA, musl+glibc | `splice01/02/04/05/06`, `tee01/02`, `vmsplice01`, `dirtypipe`; `splice02` EBADF 后 30s timeout/TBROK | pipe/splice/FD/VM、阻塞与资源回收 | 待建立 | splice/pipe 的 FD 类型、偏移、容量或唤醒语义存在共同缺口，导致错误 errno、短传输和失去进展 | — | baseline | baseline | CLUSTERING |
| B-POLL-001 | RV+LA, musl+glibc | `poll01`, `ppoll01`, `poll02`, `epoll_pwait03`; LA 另有 `epoll_wait02` | poll/epoll readiness、signal mask、等待队列 | 待建立 | readiness 注册、边沿/电平消费或超时/信号交互不符合 Linux 语义 | — | baseline | baseline | CLUSTERING |
| B-LIBC-001 | RV+LA, glibc | libctest 179 pass / 38 fail / 2 timeout；RV 原始输出含两次用户态 buffer-overflow termination | libc-facing syscall/VM/signal/locale/time | 待按原始 identity 拆分 | 跨架构完全相同计数更像一组公共 ABI/语义缺口；须先按 case identity 再聚类，不能视作单一修复 | — | baseline | baseline | CLUSTERING |
| B-CYCLIC-001 | RV musl | `cyclictest-musl` 在 hackbench 阶段超过 900s；LA 与 RV glibc 完成 | process/scheduler/pipe/FD、资源回收 | 待建立 | 大量进程与 pipe 消息路径存在 RV 可见的进展或资源回收问题；需排除 B-SPLICE/B-POLL 的下游症状 | — | baseline | baseline | CLUSTERING |
| B-VM-001 | 主要为 RV musl，LA musl 部分重叠 | `mmapstress02/03/05`, `sbrk01` | VM、brk、fork/COW、映射回收 | 待建立 | musl 路径暴露 VM 边界或并发映射回收语义缺口 | — | baseline | baseline | CLUSTERING |
| B-LA-TIME-001 | LA, musl+glibc | `clock_nanosleep02`, `nanosleep01`; musl 另有 `time-schedule`; `sched_setscheduler04` 为 TCONF/33 | time/scheduler/arch boundary | 待建立 | LoongArch64 时间换算、睡眠唤醒或调度 ABI 与通用层契约不一致 | — | baseline | baseline | CLUSTERING |
| B-PATH-001 | LA musl + 双架构 musl + RV musl | LA `readlink03/readlinkat02`；musl `gethostname02`；RV `nice04` | path/user-memory/uts/priority errno | 待按行为拆分 | 少量独立 Linux ABI 边界错误；仅在最小复现证明共同根因时合并 | — | baseline | baseline | CLUSTERING |

## 5. 阶段与门禁

### Phase 0：可信失败记录

- [x] 完整阅读 durable contract 与相关仓库/CI/runner/parser 文档。
- [x] 实时核对 HEAD、远端、clean 状态、镜像与工具链。
- [x] 用当前 parser 重放 Goal A fresh RV/LA 捕获，确认完整性错误为 0。
- [ ] 提取所有失败 case identity、原始上下文、RV/LA 交集与差集，并锁定 cluster 所属。

### Phase 1：逐 cluster 修复循环

每个 cluster 必须依次完成：

1. 从原始日志与源代码形成可证伪假设；
2. 构造不含官方测例名/路径/固定输入特化的最小行为复现；
3. 先让回归在基线上真实失败，再修复生产实现；
4. 检查 Linux 可见语义、errno、ABI、并发、资源回收和双架构边界；
5. 运行定向测试和独立只读 diff review；
6. 形成一个可审查逻辑提交，在 clean HEAD 上运行 quick；涉及公开 ABI、VM、process、
   FD、lock 或 arch 时同时运行 baseline；
7. 仅在预期会改变官方 identity 时运行 fresh RV/LA official，记录完整前后差分。

同一根因连续三次有证据的尝试仍无可度量进展时，停止盲试并声明
`BLOCKED_TECHNICAL`，保留最小复现、原始日志、代码状态以及三条假设和反证。

### Phase 2：终态闭环

- [ ] 同一候选历史上的 RV official 明确 PASS。
- [ ] 同一候选历史上的 LA official 明确 PASS。
- [ ] clean exact HEAD quick 与 baseline 明确 PASS。
- [ ] `python3 test/run_suite.py --profile full --arch all` 全部 planned=executed=completed，
  所有状态明确 PASS，无 skip/blocked/timeout/failure/error。
- [ ] 镜像哈希不变、无 overlay 残留、runner provenance clean/stable。
- [ ] 最终完整 diff 经独立只读 reviewer 复核，0 Blocker / 0 Major。
- [ ] 开发日志、AI/外部来源披露、验证证据、风险和回滚完整。
- [ ] 获得本轮明确 push 授权后普通推送，远端精确 head/ancestry 核对通过，`main` 未变。

## 6. 风险、关键决策与回滚

| 决策 | 理由 | 风险 | 回滚方式 |
|---|---|---|---|
| 先 identity/最小复现，后生产修复 | 避免从汇总 marker 猜根因或写测例特化 | 前期分析时间较长 | 保留原始 summary/log 哈希和 cluster 表，可从任一 checkpoint 继续 |
| 公共语义优先、架构差异放在明确边界 | 保持两个目标架构行为一致 | 通用改动可能扩大影响面 | 每个逻辑提交独立可回滚，定向测试后再 baseline/official |
| official 只在有预期 delta 时运行 | 单次运行耗时高且应有明确假设 | 可能延迟发现交叉回归 | 所有公开 ABI/VM/process/FD/lock/arch 改动先跑 baseline，终态必跑完整 official/full |
| 不修改 suite、parser、blacklist 或 plan | 保持 Goal A 建立的可信证据边界 | 真实失败不会快速下降 | 只修复生产语义；若环境/合同问题则 fail closed 报告 |

回滚以逻辑提交为单位使用普通反向提交；不使用 `reset --hard`、破坏性 rebase 或
force-push，不覆盖他人改动。若某修复改变与目标无关的 ABI/errno/架构行为，立即撤出
该 cluster 并保留失败证据。

## 7. 当前下一步

从两份 fresh official 原始 stdout 构建 case identity 级失败矩阵，先处理会造成
timeout/失去进展的 `splice02`，并判断 RV cyclictest 是否是同一 pipe/process 根因的
下游表现；同时提取 glibc libctest 的 40 个非 PASS identity，避免把多种能力域错误
错误合并。
