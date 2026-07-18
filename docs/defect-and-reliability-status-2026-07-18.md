# 当前内核缺陷与安全/可靠性问题汇总（2026-07-18）

本文是当前稳定化分支的防御性兼容性证据快照，用于本地 OS 竞赛内核与隔离 QEMU
guest 的缺陷跟踪和合入判断。它不提供攻击载荷、武器化 PoC、外部利用步骤，也不对
未经授权的系统给出操作建议。本文不根据现有异常推断可利用性或安全严重度；文中的
优先级仅表示兼容性、可靠性和合入阻断顺序。

本文不是 Goal B 完成声明。首次失败、`FAIL`、`ERROR`、`TIMEOUT`、`TCONF`、`TBROK`、
非零退出和未归因进程终止均按原状态保留；定向测试通过不替代 quick、baseline、
official 或 full 结果。

## 1. 统计基线与证据边界

| 项目 | 精确值 | 说明 |
|---|---|---|
| 分支 | `stabilize/post-integration-gates-20260716` | 本文只描述该分支 |
| Goal B 基线 | `ac36481d6052457433b4d1ab5f2a5fd40a20df26` | Goal A 终态 `READY_FOR_SEMANTIC_FIX` |
| 证据截止 HEAD | `789f9a3dd725185a85c85c16502bf86d53dd873c` | 最新运行时证据所对应的提交；后续纯文档提交不改变该边界 |
| 最新 fresh official 候选 | `6dd0f0e0` | 晚于该提交的 vmsplice/tee 定向修复尚无 fresh official 复证 |
| Goal A official 基线 | RV 114 findings；LA 157 findings | 两边均为顶层 `FAIL`，不是通过 |
| Goal B 最新 official | RV 87 findings；LA 127 findings | 两边均为顶层 `FAIL`，不是通过 |
| 最新定向状态 | RV `/dev/null` mode test 为 outer 1/1 `FAIL` | 当前已确认未修复的 device access-mode 缺陷 |

统计口径必须区分：finding 是 parser 保留的症状记录，可能对同一 case 含多个 marker；
`group + case` identity 用于 official 前后差分；根因簇又可能覆盖多个 identity。因此
114、157、87、127 都不是“内核缺陷数量”。

Goal A fresh official 与 Goal B 最新 official 均完整计划、执行并结束 24/24/24 groups、
2544/2544/2544 cases，child return code 为 0、`error_count=0`、provenance clean/stable；
顶层仍因明确的非通过结果返回 `FAIL`。Goal B 相对 Goal A 的可核验差分为：

- RV findings 114→87，LA findings 157→127；
- 两架构、两套 libc 的 `dirtypipe`、`splice01/02/04/05/06`、`tee01` 共 14 个
  `group + case` identity 消失；
- RV 新出现 glibc/musl `epoll_wait02` 和 glibc `nanosleep01`；LA 新出现 musl
  `kill02` watchdog。它们保留为“新增且待归因”，不能据当前证据认定为修复回归或 flake；
- `tee02`、`vmsplice01` 在两架构、两套 libc 中仍非 PASS。后续定向修复尚未经过新的
  official，故本文继续保留这些最新 official failure。

## 2. 状态词汇

| 状态 | 本文含义 |
|---|---|
| 已修复（official delta） | 定向回归通过，且 fresh official 有对应 identity 消失；仍不等于完整门禁通过 |
| 已修复（targeted only） | 根因和定向 red/green 已闭合，但没有更晚的 quick/baseline/official 复证 |
| 部分修复 | 已修复一部分可见语义，但同一能力域仍有已确认缺陷或 official failure |
| 仍未修复 | 根因有直接源码/运行证据，当前 HEAD 仍可复现且没有生产修复 |
| 仅假设 | 只有症状或聚类线索，尚无最小复现和证实根因 |
| 偶发/待归因 | 一次或有限次数观察到，尚不能证明稳定性、因果关系或共同根因 |

## 3. 证据与结果可靠性问题（非内核生产缺陷）

### EVID-001：重复命令文本不能作为唯一 official 执行身份

- **受影响范围**：RV+LA official runner/parser；证据完整性域。
- **可观察症状**：BusyBox 测试计划可含相同命令文本；只按文本投影无法证明每个计划帧
  与每个执行帧一一对应，非零退出和缺帧也可能失去准确归属。
- **正确语义**：按有序 ordinal 与 payload 建立唯一 frame identity；planned、executed、
  completed 必须一一闭合，缺帧、重复帧、非零退出或解析异常 fail closed。
- **影响范围**：会影响 official 结果可追溯性和失败归属，但不是 syscall 或内核漏洞。
- **状态**：**已修复**。提交 `9ec972f4`、`1f16c889` 完成 ordered frame、兼容投影、
  非零退出解析和 fail-closed 处理；Goal A 终态 quick 45/45/45 PASS、baseline
  57/57/57 PASS。随后两架构 official 均得到完整 24 groups/2544 cases 的可信 `FAIL`。
- **证据**：Goal A 机器 summary 与 raw capture 见第 8 节；详细设计与命令见 Goal A
  开发日志及当前 Goal B 开发日志的基线部分。

### EVID-002：定向文件对象单元测试的精确计数失配

- **受影响范围**：RV+LA 共用的 host-side guard/unit 结果；测试可靠性域。
- **可观察症状**：一次 pre-guest 计数仍使用旧期望值，不能把该入口错误当作运行时 red
  或 green。
- **正确语义**：发现数、执行数与期望数必须精确一致；入口错误显式失败。
- **状态**：**已修复**。`e7fe68a1` 修正 canonical count；`6dd0f0e0` 上 quick
  45/45/45 PASS。开发日志另保留了后续一次错误的 Python module 入口
  `ModuleNotFoundError`，随后真实脚本入口 33/33 PASS，前者没有被改写成 PASS。

## 4. 已确认的内核缺陷与根因簇

### KFD-SPLICE-001：pipe endpoint 方向、offset 与 errno 顺序错误

- **架构/能力域**：RV+LA 共用实现；`splice(2)`、pipe、FD access mode、Linux errno。
- **症状**：错误方向 endpoint、pipe offset、无效 fd 和空 pipe 的预验证可能走不正确
  adapter 或返回错误 errno；早期定向 guest 在 `USER_FAIL splice_pipe` fail closed。
- **正确 Linux 语义**：输入端必须可读、输出端必须可写；pipe 携带 offset 时返回
  `ESPIPE`；无效/错误方向 fd、zero length 与对象类型检查遵循 Linux 可观察的优先级。
- **影响**：splice/pipe ABI 兼容性；错误返回会改变 libc/LTP 可见行为，并可能让调用者
  错判重试、资源处理或数据路径状态。
- **状态**：**已修复（official delta）**。
- **证据/提交**：`bfff16ea` test-only red；`b9d90a15` 首轮 endpoint 修复；
  `ad9d1ab5` 扩展 errno/边界 red；`ae446dbf` 完成修正。RV/LA 定向 outer 各 1/1 PASS、
  inner 各 3/3 PASS。该簇与 KFD-SPLICE-002 共用 official delta，不能把 14 个消失
  identity 任意拆分给某一个子缺陷。

### KFD-SPLICE-002：同 backing、原子迁移、FD 生命周期与双锁顺序

- **架构/能力域**：RV+LA 共用实现；`splice(2)`、pipe buffer、FD close/reuse、并发锁序、
  资源生命周期。
- **症状**：旧路径不能可靠识别同 backing alias；目标已满时可能在迁移前后破坏 source
  数据保持；对象获取与 fd close/reuse 存在生命周期窗口；互为 source/destination 的并发
  splice 需要稳定锁序以避免死锁。
- **正确 Linux 语义**：同一 pipe backing 的无效组合返回 `EINVAL`；迁移在 source/dest
  共同临界区内原子完成；未成功写入时不得丢失 source 数据；已取得的 open-file 对象在
  syscall 期间保持有效；双对象锁使用全局一致顺序。
- **影响**：数据完整性、并发进展与资源生命周期可靠性。本文只确认兼容性/可靠性影响，
  不据此声称可利用性。
- **状态**：**已修复（official delta）**。
- **证据/提交**：`ad9d1ab5` 的 full-destination、same-backing、close/reuse 与 reciprocal
  concurrency red；`ae446dbf` 的对象身份、有序双锁、原子迁移和固定 `OpenFileRef`；
  `e7fe68a1` 计数修正。独立复审为 0 Blocker / 0 Major / 1 文档 Minor。`6dd0f0e0`
  quick 45/45/45 PASS；baseline 首次因 clang14 不支持 LoongArch64 target 为
  `INFRA_ERROR`（56 PASS/1 infra），使用已核验 clang21 后 57/57/57 PASS；fresh official
  仍 FAIL，但两架构各有上述 14 个 splice/dirtypipe/tee01 identity 消失。

### KFD-VMSPLICE-001：跨 iovec partial progress 后自阻塞

- **架构/能力域**：RV+LA 共用实现；`vmsplice(2)`、pipe capacity、阻塞/部分完成语义。
- **症状**：blocking vmsplice 已在 iovec 边界填满 pipe 并取得进展后，继续尝试下一 iovec；
  没有 reader 时自阻塞。`44a25cff` 的真实 runtime red 触发 10 秒 guest watchdog 和
  `HARNESS_FAIL reason=guest_timeout`。
- **正确 Linux 语义**：syscall 已取得正进展后，后续 capacity probe 不应为处理下一
  vector 无限等待；应返回已完成的 partial byte count，让调用者 drain/retry。
- **影响**：单调用进展、pipe 生命周期和 watchdog 可靠性；最新 official 的
  `vmsplice01` 表现为约 30 秒 TBROK/timeout。
- **状态**：**已修复（targeted only）**；尚无更新 official 结论。
- **证据/提交**：`8e48e853` 的 run 在更早的 tee 断言退出，**不是** vmsplice runtime
  red；`44a25cff` 才是有效 watchdog red；`64af8ac4` 在已有 `total > 0` 时使用非阻塞
  capacity probe。exact-clean RV/LA targeted 均 outer 1/1 PASS、inner 3/3 PASS，cleanup
  complete。该提交之后未运行 quick、baseline 或 fresh official，故 `tee02/vmsplice01`
  仍按最新 official failure 保留。

### KFD-TEE-001：tee 参数验证顺序与 zero-length fast path

- **架构/能力域**：RV+LA 共用实现；`tee(2)`、FD access mode、pipe type、errno 顺序。
- **症状**：旧实现过早把对象分类为 non-pipe，且缺少正确的 zero-length fast path，可能
  在错误方向 fd 上返回 `EINVAL` 而不是 `EBADF`。
- **正确 Linux 语义**：当前回归约束的顺序为 flags → `len == 0` → 输入 fd → 输出 fd →
  access mode → pipe 类型 → same backing；错误方向先 `EBADF`，方向有效但非 pipe 或同
  backing 返回 `EINVAL`。
- **影响**：Linux errno/ABI 可见行为；会改变调用者的错误分类。
- **状态**：**部分修复**。主顺序在 `d75a4d8e` 上完成并有双架构 targeted green，
  但 KFD-TEE-002 使设备 fd 的 access-mode 判断仍错误，且没有更晚 official 复证。
- **证据/提交**：`5c365cc6` 建立修订后的 test-only red；`d75a4d8e` 完成一次 fd-table
  锁内的有序 snapshot。exact-clean RV/LA targeted 各 outer 1/1 PASS、inner 3/3 PASS；
  二次 review 仍为 0 Blocker / 1 Major（即 KFD-TEE-002）。
- **被否定的旧假设**：`8e48e853`/`44a25cff` 曾期待 `tee(1, pipe_write)` 返回
  `EINVAL`；fd 1 是 write-only 输入，正确结果应为 `EBADF`。旧 artifact 只作为被否定
  假设保留，不计为 tee 语义 red/green。

### KFD-TEE-002：DevNull/Rtc 丢失 open access mode

- **架构/能力域**：RV+LA 共用实现；`openat(2)`、fork/dup、`tee(2)` FD snapshot、
  文件对象资源生命周期。
- **症状**：`FdEntry::DevNull`、`FdEntry::Rtc` 为无 status-flags payload 的 unit variant；
  构造路径忽略 `fcntl_status_flags(flags)`，fork/dup 只能复制 unit variant，
  `tee_fd_snapshot` 将两者硬编码为 readable+writable。于是 O_RDONLY `/dev/null` 作为
  tee 输出端、O_WRONLY `/dev/null` 作为输入端时，当前实现进入 non-pipe 分类返回
  `EINVAL`，而不是先返回 `EBADF`。
- **正确 Linux 语义**：open file description 必须持久保存 access mode，fork/dup 和
  snapshot 继续观察同一 mode；错误方向返回 `EBADF`，方向有效的 live non-pipe 返回
  `EINVAL`。
- **影响**：设备 FD 的方向语义、复制后的状态一致性和所有依赖 snapshot access-mode
  的 syscall。共享实现意味着两架构均受影响；当前运行时 red 只在 RV 执行，LA 尚未跑
  对应 red/green。
- **状态**：**仍未修复（已确认）**。
- **证据/提交**：`324e3f4c` 加入真实 `/dev/null` O_RDONLY/O_WRONLY 四组合矩阵；
  `789f9a3d` 只细分 marker。exact-clean RV run
  `goalb-b-splice-003-red-tee-device-mode-rv-2` 为 outer 1/1 FAIL、inner
  2 PASS/1 ERROR，raw 顺序到达 write/getpid PASS 后出现 `USER_FAIL tee_device_mode`、
  guest nonzero 与正常 shutdown；无 timeout、panic、trap、ENOSYS 或残留进程。
  outer/inner/raw SHA-256 分别为
  `f476ff2286380c27439e20b180171166d4527fb6d8cb075a3900c0dbffb6b254`、
  `1465105d32df6a3315262af2f1dec56c0910ca8ef4fbd094e7ebd828a0fb9658`、
  `ae34561f938bfd9a70386a3c24f986a61a900b270da0d79f9848dc90eb072a45`。
  当前没有生产修复提交、RV green、LA red/green、后续 quick/baseline 或 official。

## 5. 最新 official 剩余失败

以下列表来自 exact-clean `6dd0f0e0` 的最新 fresh official，不用后续 targeted 结果覆盖。

### 5.1 LTP

| 架构/libc | 最新汇总 | 非 PASS case |
|---|---:|---|
| RV musl | 13 fail / 987 pass / 0 timed | `poll01`, `ppoll01`, `poll02`, `epoll_pwait03`, `epoll_wait02`, `gethostname02`, `nice04`, `mmapstress02`, `mmapstress03`, `mmapstress05`, `sbrk01`, `tee02`, `vmsplice01` |
| RV glibc | 8 fail / 992 pass | `poll01`, `ppoll01`, `poll02`, `nanosleep01`, `epoll_pwait03`, `epoll_wait02`, `tee02`, `vmsplice01` |
| LA musl | 19 fail / 981 pass / 1 timed | `poll01`, `ppoll01`, `kill02`, `clock_nanosleep02`, `poll02`, `time-schedule`, `nanosleep01`, `epoll_pwait03`, `epoll_wait02`, `sched_setscheduler04`, `gethostname02`, `readlink03`, `readlinkat02`, `mmapstress02`, `mmapstress03`, `mmapstress05`, `sbrk01`, `tee02`, `vmsplice01` |
| LA glibc | 9 fail / 991 pass | `poll01`, `ppoll01`, `clock_nanosleep02`, `poll02`, `nanosleep01`, `epoll_pwait03`, `epoll_wait02`, `tee02`, `vmsplice01` |

LA musl `kill02` 打印两条 TPASS 后未结束，达到 180 秒单例 watchdog，code 137；
`sched_setscheduler04` code 33/TCONF 仍按非 PASS 保留。`tee02` 两套 libc/两架构均有两项
期待 `EINVAL` 却得到 `EBADF`，same-pipe 子项通过后整体 code 1。`vmsplice01` 四个组合
均 code 2，约 30 秒 TBROK/timeout。

### 5.2 libctest-glibc

RV 与 LA 最新结果均为 179 pass / 38 fail，其中 `setvbuf_unget` static、dynamic 两项
均 timeout。38 个 `binary + case` identity 在两架构相同：

- static 18：`clocale_mbfuncs`, `fnmatch`, `fscanf`, `fwscanf`, `mbc`,
  `pthread_cancel_points`, `sscanf`, `strftime`, `strtol`, `swprintf`, `wcstol`,
  `daemon_failure`, `dn_expand_empty`, `dn_expand_ptr_0`, `fgetwc_buffering`,
  `regex_ere_backref`, `regex_escaped_high_byte`, `setvbuf_unget`；
- dynamic 20：上述 18 项加 `pthread_cancel`、`pthread_exit_cancel`。

两架构的个别 signal/exit code 不同，不能因 identity 列表相同而宣称单一共同根因。

### 5.3 generic、watchdog 与异常终止

- RV `cyclictest-musl` 的 P1/P8 no-stress 阶段完成，进入 hackbench 后超过 900 秒 group
  上限，code 137/`TIMEOUT`；LA 与 RV glibc 对应 group 完成。状态为**偶发/待归因**，
  不是已证实 scheduler 或 pipe 根因。
- LA musl `kill02` 的 180 秒 watchdog 为**新增且待归因**；一次观察不足以证明 flake。
- 最新 official parser 的宽泛 `panic-or-trap` finding 标签为 RV 4、LA 2，但逐条 raw
  复核没有确认 kernel panic、unhandled trap 或 kernel page fault。RV raw 中两条
  `buffer overflow detected` 是用户态进程终止文本；RV/LA libctest 另有 83、134、139
  等进程退出/信号结果。它们必须保留并继续归因，但当前证据不能把它们描述为内核内存
  破坏、可利用漏洞或已确认 panic。
- 最新 device-mode targeted red 明确无 panic、trap、ENOSYS、watchdog 或 cleanup 残留。

## 6. 仅假设或待归因的根因簇

这些条目不是已确认内核根因，不能据此直接修改生产实现。

| ID | 架构/症状 | 能力域与正确语义方向 | 状态与下一证据 |
|---|---|---|---|
| HYP-POLL-001 | RV+LA 的 `poll01`, `ppoll01`, `poll02`, `epoll_pwait03`, `epoll_wait02` | poll/epoll readiness、signal mask、等待队列；就绪、timeout 与信号交互应符合 Linux 可观察语义 | **仅假设**；先按具体 assertion 建最小 repro，不能把五项视作一个已证实根因 |
| HYP-LIBC-001 | 双架构 glibc libctest 38 fail/2 timeout | locale/mb、fnmatch/regex、stdio/wide I/O、time/numeric、pthread cancel、daemon/resolver | **仅假设**；必须按 case family 拆分，不能因计数相同合并为单缺陷 |
| HYP-CYCLIC-001 | RV musl hackbench 阶段 900 秒 group timeout | process/scheduler/pipe/FD、进展与资源回收 | **偶发/待归因**；需独立压力最小复现，并排除 splice/poll 的下游影响 |
| HYP-VM-001 | musl `mmapstress02/03/05`, `sbrk01` | VM、brk、fork/COW、映射回收；映射边界和并发生命周期应保持数据与回收一致 | **仅假设**；当前只有 case 症状 |
| HYP-LA-TIME-001 | LA 的 `clock_nanosleep02`, `nanosleep01`, musl `time-schedule`; `sched_setscheduler04` TCONF | time/scheduler/arch boundary；时间换算、唤醒与 ABI errno | **仅假设**；TCONF 不得改写为 PASS，也不能与时序 TFAIL 自动合并 |
| HYP-PATH-001 | LA musl `readlink03/readlinkat02`，双架构 musl `gethostname02`，RV musl `nice04` | path/user-memory/UTS/priority errno | **仅假设**；这些是不同 syscall，除非最小复现证明共同根因，否则分别跟踪 |
| NEW-RV-001 | 新出现两 libc `epoll_wait02`、glibc `nanosleep01` | wait/time | **新增且待归因**；保留首次 failure，不根据略超阈值认定 flake |
| NEW-LA-001 | 新出现 musl `kill02` 180 秒 watchdog | process/signal/wait/lifecycle | **新增且待归因**；保留首次 failure，不由重试覆盖 |

## 7. 定向、quick、baseline、official 证据矩阵

| 范围/截止提交 | Targeted | Quick | Baseline | Official 与 failure delta |
|---|---|---|---|---|
| Goal A evidence infra / `ac36481d` | ordered-frame/parser 单元与重放通过 | 45/45/45 PASS | 57/57/57 PASS | `1f16c889` fresh RV/LA 均 FAIL：114/157 findings；作为可信语义基线 |
| splice atomic/lifetime / `6dd0f0e0` | RV+LA outer 1/1、inner 3/3 PASS | 45/45/45 PASS | 首次 clang14 为 `INFRA_ERROR`；clang21 重跑 57/57/57 PASS | RV/LA 仍 FAIL：87/127 findings；14 个目标 identity/架构消失，但出现 3/1 个新增 identity |
| vmsplice partial progress / `64af8ac4` | RV+LA outer 1/1、inner 3/3 PASS | 未运行 | 未运行 | 未运行；最新 official 仍保留 `vmsplice01` failure |
| tee 主验证顺序 / `d75a4d8e` | RV+LA outer 1/1、inner 3/3 PASS | 未运行 | 未运行 | 未运行；二审仍有 device-mode Major，最新 official 保留 `tee02` failure |
| device access mode / `789f9a3d` | RV outer 1/1 FAIL、inner 2 PASS/1 ERROR；LA 未运行 | 未运行 | 未运行 | 未运行；无生产修复和 failure delta |

未运行表示没有该层级证据，不表示 PASS、SKIPPED 或环境阻塞。本次纯文档收尾没有启动
新的 quick、baseline、official 或 full。

## 8. 可复核证据路径与哈希

大体积原始输出位于被 Git 忽略的 `test/output/`，不作为提交内容；下面用 repo-relative
路径和 SHA-256 固定本机证据身份。若 artifact 丢失，必须 fresh 重跑，不能从本文重建
或补造原始记录。

| 证据 | 机器 summary / raw failure record | SHA-256 |
|---|---|---|
| Goal A RV official | `test/output/goala-1f16c889-remediation-official-rv-1/summary.json`; `logs/official.riscv64.stdout.log`; `logs/official.riscv64.stderr.log` | summary `651e6053bdf18d7ef4e027c1c4e7906367a8084a815b8d201ba7b6e937e6200a`; stdout `c7344b37dd55bf3c0116dc7063c6e4b9e0b5c6db5472d12b623e8be003a673cc`; stderr `d529553efb9810e6677c0dc2609897a4dcb737184cffb47b117b5a2156d940d5` |
| Goal A LA official | `test/output/goala-1f16c889-remediation-official-la-1/summary.json`; `logs/official.loongarch64.stdout.log`; `logs/official.loongarch64.stderr.log` | summary `6e233af93b90e281373683a20afe632db3af545a8f3d99bc040b5b3d1626e783`; stdout `5c8a01d89c8c4624c79e13ce4475023733c1e8f7d423cdfd5f7bd196a90e16ae`; stderr `b23d1a983b8ad1b1ed116fa148283ee467462baf27597e6dc0687e51c4fb0944` |
| Goal B RV official delta | `test/output/goalb-b-splice-001-official-rv-delta-1/summary.json`; `logs/official.riscv64.stdout.log`; `logs/official.riscv64.stderr.log` | summary `16a1dbc198f059d08201d9a7c70ff98de63fea1661dcae5a5f7a0035cb88fb24`; stdout `fed44619ca7331ceefc129b709ccb71b0c8f5796e84bcc9df230c1e56990f5c9`; stderr `5485c60156b9b66096128434b2e3ec9b7baeda458de3efd29b60f78efa52f6c0` |
| Goal B LA official delta | `test/output/goalb-b-splice-001-official-la-delta-1/summary.json`; `logs/official.loongarch64.stdout.log`; `logs/official.loongarch64.stderr.log` | summary `d57ab624cceb8a3cb61823b06fae713e47b0786617cd76db07b96851b8475a18`; stdout `07b7a18ec699a2e1e8b7b4d055d79fb4a2ad4bb875a2c640c2bcd7361b0443be`; stderr `70359007681697f46f10fe1d488127b0334814afbe173f02217935a8ec1ae103` |
| Device-mode RV red | `test/output/goalb-b-splice-003-red-tee-device-mode-rv-2/summary.json`; `semantic-rv64/semantic-evidence-v1.json`; `semantic-rv64/logs/smoke.rv64.abi--riscv64.stdout.log` | outer `f476ff2286380c27439e20b180171166d4527fb6d8cb075a3900c0dbffb6b254`; inner `1465105d32df6a3315262af2f1dec56c0910ca8ef4fbd094e7ebd828a0fb9658`; raw `ae34561f938bfd9a70386a3c24f986a61a900b270da0d79f9848dc90eb072a45` |

定向 vmsplice/tee red/green 的完整命令、运行时 marker、耗时和哈希记录在
`docs/development-logs/2026/07/2026-07-16-pr-draft-official-semantic-stabilization.md`；
活动状态与门禁记录在 `docs/plans/active/official-semantic-stabilization.md`。本文不复制
大段 raw console，也不包含主机绝对路径。

## 9. 安全/可靠性风险与优先级

此处 P0/P1/P2 是工程处理顺序，不是 CVSS、漏洞等级或可利用性结论。

### P0：已确认 kernel panic、trap 或内核内存破坏

当前证据中**没有确认项**。宽泛 parser marker、用户态 buffer-overflow termination 和
signal exit 不能替代内核根因证据。若后续出现明确 kernel panic/unhandled trap，应立即
保留 raw log、最小化本地复现并阻断合入。

### P1：当前合入阻断与进展/生命周期风险

1. **KFD-TEE-002**：已确认、当前 HEAD 可复现的 DevNull/Rtc access-mode 丢失；二次
   review 的 Major 尚未关闭。
2. **KFD-VMSPLICE-001 的 official 复证缺口**：定向修复已 green，但最新 official 仍有
   `vmsplice01` TBROK/timeout；必须用后续 fresh official 证明 identity 消失。
3. **RV cyclictest-musl 900 秒**：大量 process/pipe/FD 压力下的进展或资源回收风险，
   根因待证实。
4. **LA musl kill02 180 秒**：signal/process wait 或生命周期风险，当前仅一次观察，
   仍需优先最小化。
5. **libctest 两项 setvbuf_unget timeout 与信号退出**：不得隐藏；先区分 libc 行为、
   syscall 语义和进程终止原因。

### P2：兼容性根因簇

poll/epoll、VM/brk/COW、LA time/scheduler、readlink/hostname/nice 及其余 libctest family
仍影响 Linux/POSIX 可见语义，但当前只有 cluster 假设。应按单一行为先 red、再修生产
语义，不能按 testcase、libc、架构或路径特化。

已修复 splice 路径仍有回归关注点：source 数据保持、同 backing alias、reciprocal lock
ordering、fd close/reuse 固定对象生命周期。这些已有定向与 official delta 证据，但在
最终 full 之前仍应作为并发/资源生命周期复核项。

## 10. 下一步建议与合入阻断条件

本次收尾按用户明确指令停在文档状态，不继续 Goal B 生产修复，也不启动 official/full。
只有未来收到新的明确恢复指令后，才建议按以下顺序继续：

1. 为 `DevNull`/`Rtc` 持久保存 open access mode，覆盖构造、fork/dup、snapshot 与
   guard/mutation；先完成 RV/LA targeted green。
2. 第三次独立只读 review 必须达到 0 Blocker / 0 Major，随后才运行 clean exact-HEAD
   quick 与 baseline。
3. 有明确预期 delta 后再运行 fresh RV/LA official；验证 `tee02`、`vmsplice01` 消失，
   且没有新增 failure identity。首次 official FAIL 和新增 RV/LA identity 必须保留。
4. 将 poll/time/process/watchdog 和 libctest 按最小行为拆分，不从 case 名猜根因。

当前分支不得宣称 Ready、merge-ready 或 Goal B 完成，至少因为：

- KFD-TEE-002 仍是已确认未修复 Major，且缺 LA 定向证据；
- 最新证据截止 HEAD 没有对应的 clean quick/baseline；后续 vmsplice/tee 修改没有 fresh
  official；
- 最新 RV/LA official 均为 `FAIL`，仍有 LTP、libctest 与 watchdog 非 PASS；
- `full --arch all` 未运行；
- 第三次 0 Blocker / 0 Major 独立 review 和最终人工复核未完成；
- 任何 timeout、TCONF、TBROK、进程异常终止和新增 identity 均尚未全部归因或关闭。
