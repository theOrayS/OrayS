# LTP Full-Sweep Blacklist Ultragoal + Team 启动提示词

我们现在位于 `/root/oskernel2026-orays` 的实验分支 `exp/ltp-full-sweep-blacklist`。

目标：使用 `$ultragoal` + `$team` 持续运行实验性 LTP full-sweep blacklist 循环，直到达成以下完成条件：

1. RV 上至少完成一次闭合的 `LTP_CASES=blacklist` full sweep：
   - 无未闭合 `RUN LTP CASE`
   - 无未解释的 QEMU hang / kernel panic / trap / guest OOM
   - 有 parser-backed summary
2. 使用 RV 收敛后的同一 blacklist 跑一次 LA 对比：
   - 若 LA 闭合，输出 RV/LA 差异报告
   - 若 LA 遇到架构特有严重阻断，单独标注 `arch=la`，不要无证据污染通用 blacklist
3. 产出 durable artifacts：
   - `docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt`
   - `docs/ltp-full-sweep-blacklist-2026-05-29/iterations.md`
   - 每轮 summary / raw-log 路径 / 新增 blacklist 理由
   - 最终 high-yield targeted fix 候选列表

## Hard Rules

- 不允许 fake pass。
- blacklist / SKIP 不算 PASS，不得作为 stable promotion 证据。
- 普通 `TFAIL`、`TBROK`、`ENOSYS`、wrong errno 不进入 blacklist，除非它们导致 sweep 无法继续。
- 只允许 blacklist 严重阻断项：
  - kernel panic / trap
  - QEMU 或 guest hang
  - guest OOM / fork bomb / 资源耗尽
  - 破坏后续 case 的环境污染
  - 多轮复现的不可接受 timeout
  - cgroup / namespace / driver / module / cpuhotplug 等当前内核模型不支持且会阻断 sweep 的环境依赖
- 每个新增 blacklist case 必须记录：
  - case 名
  - reason category
  - 首次失败证据位置
  - 是否 RV/LA 通用或 arch-specific
  - 后续解除条件
- raw 大日志默认不提交；提交精简 summary、路径、计数和结论。
- 只暂存/提交本任务产生的文件，不触碰无关 dirty worktree。

## Suggested Ultragoal Stories

用 `$ultragoal` 创建 durable plan，建议拆成这些 stories：

1. 建立实验目录与基线记录
   - 创建 `docs/ltp-full-sweep-blacklist-2026-05-29/`
   - 写入初始 `blacklist.txt`
   - 写入 `iterations.md`
   - 记录当前 branch、HEAD、stable count、runner blacklist 入口、磁盘空间

2. RV full-sweep 第 1 轮
   - 运行前后 `df -h / /root`
   - 运行：
     ```bash
     LTP_BLACKLIST="$(cat docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
     LTP_CASES=blacklist \
     ./run-eval.sh rv 2>&1 | tee <raw-log>
     ```
   - 用 `scripts/ltp_summary.py <raw-log>` 和 marker grep 汇总
   - 定位最后未闭合 case、panic、trap、OOM、timeout

3. RV blacklist 迭代
   - 只将严重阻断项加入 `blacklist.txt`
   - 追加 `iterations.md`
   - 重复 RV sweep，直到闭合
   - 每轮都记录 RUN/PASS/FAIL/TIMEOUT/SKIP/incomplete

4. LA 对比 sweep
   - 使用 RV 收敛后的同一 blacklist 跑：
     ```bash
     LTP_BLACKLIST="$(cat docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
     LTP_CASES=blacklist \
     ./run-eval.sh la 2>&1 | tee <raw-log>
     ```
   - 汇总 LA 结果
   - 标注 LA-only blocker，不自动归入通用 blacklist

5. 最终报告与候选提取
   - 总结 RV/LA pass/fail/timeout/skip/incomplete
   - 分离 wrapper PASS/FAIL 与内部 `TPASS/TFAIL/TBROK/TCONF`
   - 输出 high-yield targeted fix 候选：
     - 高 TPASS 密度但 wrapper fail 的 case
     - fan-out 高的 syscall/mm/fs case
     - RV/LA 分歧 case
   - 明确哪些 blacklist case 可在未来真实修复后移除

## Suggested Team Staffing

启动 Team 时建议使用 4 个 executor workers：

```bash
omx team 4:executor "Run LTP full-sweep blacklist experiment on exp/ltp-full-sweep-blacklist. Leader owns Ultragoal state. Workers must not checkpoint Ultragoal. Worker lanes: RV sweep runner, log/parser analyst, blacklist evidence auditor, LA comparison/report lane. Blacklist only severe blockers; ordinary TFAIL/ENOSYS stays as failure."
```

Worker 分工：

1. RV sweep runner
   - 负责执行 RV run、保存 raw log、报告是否闭合。
2. Log/parser analyst
   - 负责 `scripts/ltp_summary.py`、marker grep、RUN/PASS/FAIL/TIMEOUT/SKIP/incomplete 计数。
3. Blacklist evidence auditor
   - 审核每个新增 blacklist 是否符合 severe blocker 标准。
   - 拒绝普通失败进入 blacklist。
4. LA comparison/report lane
   - RV 收敛后跑 LA。
   - 输出 RV/LA 差异和 arch-specific blocker。

Leader 职责：

- 创建并维护 Ultragoal。
- 读取 Team evidence 后 checkpoint Ultragoal。
- 决定每轮是否加入 blacklist。
- 确保最终报告不把 blacklist/SKIP 计为 pass。
- 最终做验证、review、提交本任务文件。

## Acceptance Criteria

- `docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt` 存在且每个新增项有理由。
- `iterations.md` 包含每轮命令、raw log 路径、summary、严重错误、blacklist 变更。
- RV 至少一次 full-sweep 闭合。
- LA 至少一次使用 RV blacklist 对比运行。
- 最终报告包含：
  - branch / HEAD
  - LTP_CASES mode
  - blacklist 来源
  - skipped count
  - RUN/PASS/FAIL/TIMEOUT/SKIP/incomplete
  - TPASS/TFAIL/TBROK/TCONF/ENOSYS/panic/trap
  - high-yield targeted fix 候选
- 已提交 durable docs；不提交大 raw log，除非用户明确要求。
