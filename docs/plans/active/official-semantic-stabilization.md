# Goal B：官方语义稳定化执行计划

状态：`IN_PROGRESS`

开始日期：2026-07-16

执行分支：`stabilize/post-integration-gates-20260716`

Goal B 基线：`ac36481d6052457433b4d1ab5f2a5fd40a20df26`

权威集成基线：`09f4076ac151e0e7800103de724d9042230738b5`

## 1. 执行合同

本计划与仓库根 `AGENTS.md`、`.codex/tasks/SESSION_GUIDANCE.md`、
`.codex/tasks/GOAL_B_SEMANTIC_STABILIZATION.md`、`.codex/tasks/SAFETY_SCOPE.md`
共同构成持续执行合同。若记录、提示或历史证据与这些文件冲突，以更严格的
fail-closed、双架构、安全范围和 Git 安全要求为准。动态验证只在授权本地仓库与隔离
QEMU guest 中进行；不扩展到外部目标、凭据、持久化、规避检测或漏洞武器化。

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
| B-SPLICE-001 | RV+LA, musl+glibc | `splice01/02/04/05/06`, `tee01/02`, `vmsplice01`, `dirtypipe`; `splice02` EBADF 后 30s timeout/TBROK | pipe/splice/FD、方向与 offset errno | `bfff16ea`/`ad9d1ab5` 覆盖首批 splice 行为；`8e48e853`/`44a25cff` 的 vmsplice red 仍有效，但其中 tee 断言经 review 判定错误；`5c365cc6` 按 flags→zero-length→输入 fd→输出 fd→access mode→pipe type 的 Linux 顺序建立修订 tee red | 已证实两个独立通用缺陷：tee 缺少 zero-length fast path，且在完成两个 fd/access-mode 校验前过早按对象类型分类；vmsplice 在已有进展且 pipe 恰于 iovec 边界填满后仍阻塞处理下一 vector | 首批 `ae446dbf`；canonical count `e7fe68a1`；vmsplice progress `64af8ac4`；修订 tee 测试 `5c365cc6`、实现/guard/mutation `d75a4d8e` | 上轮 official 114→87；exact-clean `d75a4d8e` RV targeted outer 1/1、inner 3/3 PASS，新 official delta 待二次 review/quick/baseline 后运行 | 上轮 official 157→127；exact-clean `d75a4d8e` LA targeted outer 1/1、inner 3/3 PASS，新 official delta 待运行 | TARGETED_GREEN_REVIEW_PENDING |
| B-POLL-001 | RV+LA, musl+glibc | `poll01`, `ppoll01`, `poll02`, `epoll_pwait03`; LA 另有 `epoll_wait02` | poll/epoll readiness、signal mask、等待队列 | 待建立 | readiness 注册、边沿/电平消费或超时/信号交互不符合 Linux 语义 | — | baseline | baseline | CLUSTERING |
| B-LIBC-001 | RV+LA, glibc | libctest 179 pass / 38 fail / 2 timeout；RV 原始输出含两次用户态 buffer-overflow termination | libc-facing syscall/VM/signal/locale/time | 待按原始 identity 拆分 | 跨架构完全相同计数更像一组公共 ABI/语义缺口；须先按 case identity 再聚类，不能视作单一修复 | — | baseline | baseline | CLUSTERING |
| B-CYCLIC-001 | RV musl | `cyclictest-musl` 在 hackbench 阶段超过 900s；LA 与 RV glibc 完成 | process/scheduler/pipe/FD、资源回收 | 待建立 | 大量进程与 pipe 消息路径存在 RV 可见的进展或资源回收问题；需排除 B-SPLICE/B-POLL 的下游症状 | — | baseline | baseline | CLUSTERING |
| B-VM-001 | 主要为 RV musl，LA musl 部分重叠 | `mmapstress02/03/05`, `sbrk01` | VM、brk、fork/COW、映射回收 | 待建立 | musl 路径暴露 VM 边界或并发映射回收语义缺口 | — | baseline | baseline | CLUSTERING |
| B-LA-TIME-001 | LA, musl+glibc | `clock_nanosleep02`, `nanosleep01`; musl 另有 `time-schedule`; `sched_setscheduler04` 为 TCONF/33 | time/scheduler/arch boundary | 待建立 | LoongArch64 时间换算、睡眠唤醒或调度 ABI 与通用层契约不一致 | — | baseline | baseline | CLUSTERING |
| B-PATH-001 | LA musl + 双架构 musl + RV musl | LA `readlink03/readlinkat02`；musl `gethostname02`；RV `nice04` | path/user-memory/uts/priority errno | 待按行为拆分 | 少量独立 Linux ABI 边界错误；仅在最小复现证明共同根因时合并 | — | baseline | baseline | CLUSTERING |

### 失败 identity 清单（Goal A fresh 捕获）

- RISC-V64 LTP：glibc 13 个、musl 21 个非 PASS identity；LoongArch64 LTP：glibc
  16 个、musl 39 个。按 `group + case` 去重后两架构共同 33 个，RV 独有
  `ltp-musl/nice04`，LA 独有 22 个，集中在 time/epoll/scheduler/readlink。
- 双架构共同的 LTP identity 覆盖：两 libc 的 poll/ppoll/epoll_pwait、
  splice/tee/vmsplice/dirtypipe，以及 musl 的 brk、gethostname、mmapstress、sbrk。
  `TCONF`、`TBROK`、timeout 和非零返回均保留为非 PASS，没有按 case code 0 隐藏。
- glibc libctest 在两架构有完全相同的 38 个 `binary + case` identity：static 18 个、
  dynamic 20 个；`setvbuf_unget` 两种链接方式均 timeout。case family 为 locale/mb、
  fnmatch/regex、stdio scan/wide I/O/buffering、strftime/strtol/wcstol、pthread cancel、
  daemon 和 resolver；不同架构的 signal/exit reason 差异留在原始记录中。
- group 级非 PASS 仅另有 RV `cyclictest-musl`（137，900 秒）；LA 没有对应 generic
  failure。上述清单覆盖 fresh 捕获中全部 LTP、libctest 与 generic failure identity。

## 5. 阶段与门禁

### Phase 0：可信失败记录

- [x] 完整阅读 durable contract 与相关仓库/CI/runner/parser 文档。
- [x] 实时核对 HEAD、远端、clean 状态、镜像与工具链。
- [x] 用当前 parser 重放 Goal A fresh RV/LA 捕获，确认完整性错误为 0。
- [x] 提取所有失败 case identity、原始上下文、RV/LA 交集与差集，并锁定初始 cluster 所属。

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

当前 B-SPLICE-001 checkpoint：

- [x] 通用双架构行为回归先在 `bfff16ea` 的真实 RISC-V64 guest 上 fail closed；两个
  构建实例 PASS，唯一 runtime instance 在 `USER_FAIL splice_pipe` 失败。
- [x] `b9d90a15` 直接验证 Pipe endpoint 读写方向，保留有 offset 时的 `ESPIPE`
  优先级，并停止让 Pipe 空缓冲预验证走旧 `FdTable::read/write` 适配器。
- [x] 同一 clean `b9d90a15` 上 RV/LA `evidence-runtime` 外层各 1/1 PASS、内层各
  3/3 PASS，无 skip、blocked、timeout、failure、error 或残留进程。
- [x] 独立只读首轮 review 完成：`0 Blocker / 3 Major`，因此上述 green 只证明原
  happy path，不放行 quick/baseline 或 official delta。
- [x] 先红补齐 `len == 0`、无效 fd、pipe offset、错误方向、同 backing pipe、目标已满且
  source 数据保留、close/reuse 生命周期和 reciprocal 并发锁序行为。
- [x] 生产实现按 pipe 对象身份判 self-splice，在同一有序双锁临界区迁移数据，并在取用
  endpoint 时固定 `OpenFileRef` 生命周期；修正 syscall 入口的 len/fd/offset 优先级。
- [x] 修订后的 RV/LA 定向行为测试明确 PASS，且首次失败证据完整保留。test-only red
  outer SHA-256 为 `5168235f83ef9971082660af0a4a695a65fb372e69cb8bbd185c67f265a76e22`；
  最终 RV/LA outer SHA-256 分别为
  `b437714e3548008b46c265162d57230dab8f8f5f2f88b70762f8fbc7f819db52` 与
  `1409dd5eb440a00056dd3abe81c25e348da0b8f1ede91c45c9f2134ef693742c`。
- [x] 独立只读 cluster review 为 `0 Blocker / 0 Major / 1 Minor`；唯一 Minor 是本计划与
  开发日志未同步，现由本 checkpoint 修复。复审未把 pre-guest 24/33 计数错误当作
  语义 red 或 green，也未发现 parser、blacklist、official plan 或假成功改动。
- [x] exact-clean `6dd0f0e0` quick 为 45/45/45 PASS；首次 baseline 因系统 clang14
  不支持 LoongArch64 target 而真实记录为 `INFRA_ERROR`，显式使用已核验 clang21 后
  exact-clean baseline 为 57/57/57 PASS。
- [x] fresh RV/LA official 均完整执行 24/24/24 groups、2544/2544/2544 cases，
  `error_count=0`，并保存 summary/stdout/stderr 哈希与 114→87、157→127 的身份差分。
- [x] `8e48e853`/`44a25cff` 的跨 iovec vmsplice red 仍是有效 guest nonzero/watchdog
  证据；`64af8ac4` 使 blocking vmsplice 在已有累计进展后以非阻塞容量探测返回 partial
  count，不按 case/libc/arch/path 特化，且 exact-clean RV/LA targeted 均 green。
- [x] 第一轮本批独立只读 review 为 `0 Blocker / 1 Major / 1 Minor / 0 Nit`，拒绝
  promotion：`44a25cff` 缺少 zero-length fast path，并在两个 fd 与 access mode 校验前
  过早分类 non-pipe；`8e48e853` 对 `tee(1, pipe_write)` 期待 `EINVAL` 也违反 fd 方向
  优先级。因此旧 tee red/green 不再作为语义证明，相关 artifact 只保留为错误假设历史。
- [x] test-only `5c365cc6` 重建正确顺序回归；exact-clean RV 外层 1/1 FAIL、内层
  2 PASS / 1 ERROR，在 `USER_FAIL splice_pipe` 后 guest nonzero fail closed，无 timeout、
  crash 或 infra。outer/inner/raw SHA-256 分别为
  `37eff04986deecb08707a3dd2c52a316c7e0374e054a6a130870b7712fd3a7e6`、
  `4abfe2624ce1027e131fcb7122e4a95c4200f504326b58404d19a7755a6d084a`、
  `726c11b5b641fa82cd4cc88d58399fd788af861dcf8e4d1822f4f49f01667fa8`。
- [x] `d75a4d8e` 在一次 fd-table 锁内按输入再输出顺序固定对象快照，保留 access mode，
  先返回方向 `EBADF`，再判 non-pipe/same backing `EINVAL`；flags 仍最先，`len == 0`
  紧随其后。guard/mutation 绑定该顺序，33/33 unit 与双架构 kernel smoke 通过。
- [x] exact-clean `d75a4d8e` 的 RV/LA `evidence-runtime` 外层各 1/1 PASS、内层各
  3/3 PASS；RV/LA outer SHA-256 分别为
  `f31304125796080f70fce3a11d53478342b618c1b95176be22fbf92205415190` 与
  `e1af85a332a5ddda0b1f954100adf804fc34f8ee1d156b9519eed880a7872b45`，marker 唯一
  有序，cleanup/reap 完整，无 FAIL/panic/timeout/残留进程。
- [ ] 完成本批二次独立只读 review；0 Blocker / 0 Major 后才运行 clean quick/baseline。
- [ ] fresh official delta 不得有新增 failure identity，且 `tee02`、`vmsplice01` 必须
  消失；上轮 RV 时序 identity 与 LA `kill02` watchdog 仍保留为真实待归因失败。

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

第一轮第二批 review 已正确否定 `8e48e853`/`44a25cff` 的 tee 顺序假设；其 vmsplice
red/fix/green 仍有效。test-only `5c365cc6` 已保存修订后的真实 tee red，`d75a4d8e`
完成通用顺序修复、guard/mutation，并在 exact-clean RV/LA targeted 均明确 green。
下一步由同一未参与实现的只读 reviewer 复核更正 diff 与证据。只有复审为 0 Blocker /
0 Major 后才运行 clean quick、baseline；两者通过后才运行 fresh 双架构 official delta，
要求 `tee02`、`vmsplice01` 消失且无新增 identity。RV 的 `epoll_wait02`/`nanosleep01` 与
LA musl `kill02` 继续作为真实待归因 identity，首次 FAIL 不被重试覆盖。
