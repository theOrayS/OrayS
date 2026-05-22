# 下一轮 LTP stable 提升计划（激进版）：stable85 -> stable100/110

## 背景与当前基线

- 今天的新计划与提示词使用 `docs/ltp-score-improvement-2026-05-22/`。
- 昨天/历史证据目录保留为 `docs/ltp-score-improvement-2026-05-21-phase-*`；不要把这些历史证据再次改成未来日期。
- 当前 stable LTP batch：**85 cases / libc / arch**。
- 最近一轮 full gate：LA/RV 均 `PASS LTP CASE 170`、`FAIL LTP CASE 0`；`ltp-musl 85/0`、`ltp-glibc 85/0`。
- 内部信号：LA/RV 均 `TFAIL=0`、`TBROK=0`、`TCONF=4`、`LTP timeout=0`、`ENOSYS=0`、`panic/trap=0`。
- `read02` 仍是透明 `pass_with_tconf`，不得隐藏，也不得作为 clean pass 宣称。
- 非 LTP `iperf ... end: fail` markers 仍存在，但不作为 stable LTP promotion 阻塞项，除非新证据证明影响 targeted stable gate。

## 激进目标

1. 主目标：从 stable85 提升到 **stable100**，即新增 15 个真实可验证 stable cases。
2. Stretch：如果 targeted evidence 足够干净，继续提升到 **stable105-110**。
3. 若第一轮不能一次达到 100，不降级为保守结束；继续以第二 targeted wave 修复/重测，直到 evidence 表明继续推进会引入真实风险。
4. 仍不接受伪 PASS、case-name hardcode、静默 SKIP、timeout 算 PASS、只看 `run-eval` exit code。
5. 每批 promotion 必须有 LA/RV × musl/glibc evidence matrix，并分别报告 internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。

## RALPLAN-DR 摘要

### Principles

1. Aggressive but honest：批次更大、并行面更宽，但 promotion gate 不降低。
2. Evidence first：只有 fresh targeted LA/RV × musl/glibc 证据全绿的 case 才能 promotion。
3. Semantic fixes only：修真实 ABI/errno/signal/time/process/FS 语义，不为 LTP case 名称开后门。
4. Two-wave promotion：第一 wave 争取 10-15 个，第二 wave 再争取 5-10 个，避免一轮失败就停。
5. Honest accounting：timeout/TFAIL/TBROK/TCONF/ENOSYS/panic 必须单独计数，不得混入 PASS。
6. Leader-owned durability：Leader 维护 `.omx/ultragoal/goals.json` 和 `ledger.jsonl`；Team workers 只交证据和结果。

### Decision Drivers

1. 分数收益：优先组合 proc/sched/getter/wait + time/signal + fs/basic metadata，扩大候选池到 50-80 个。
2. 并行吞吐：Discovery/Matrix 与三条修复 lane 同时推进；Leader 每次只 promotion 已证实 clean 的子集。
3. 风险控制：先 targeted，再 promotion，再 stable targeted，最后 full gate；timeout 和内核异常是 hard stop。

### Viable Options

- Option A（推荐，激进双 wave）：proc/sched/wait 作为 Wave 1 主修，time/signal 与 fs/metadata 同时做 Wave 2 准备。
  - Pros：更可能一次冲到 100+，且不同子系统可并行。
  - Cons：需要 Leader 严格控制 promotion gate，防止多个 lane 的半成品混入 stable。
- Option B（中等激进）：只扩大 proc/sched/wait 到 12-15 cases，time/fs 只 discovery。
  - Pros：风险更低。
  - Cons：如果 proc/wait blocker 深，可能卡在 90-95。
- Option C（最高风险，不推荐）：一次性把所有候选放入 stable 后用 full gate 筛。
  - Rejected：违反 targeted-first，容易把 timeout/TFAIL 混进 stable。

Decision：采用 Option A。目标不再是 stable95，而是 **stable100 必达优先，105-110 stretch**；但任何 case 必须通过同样的 clean evidence gate。

## 分阶段计划

### Phase 0：日期/上下文与目标状态确认

- 确认旧 Codex goal 已 clear；如果同一 Codex thread 延续旧 Ultragoal，先清理旧 goal 状态再创建新 Ultragoal。
- 新建今天的 context/brief，保存到 `docs/ltp-score-improvement-2026-05-22/` 或 `.omx/context/`；历史证据继续引用 `2026-05-21-phase-*`。
- 读取当前关键证据：`docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-report.md`、`targeted-promotion11-matrix.md`、stable85 summaries、`scripts/ltp_summary.py`、`examples/shell/src/cmd.rs`。

### Phase 1：大候选池 Discovery + matrix

- Discovery lane 枚举 **50-80 个候选**，覆盖 proc/sched/wait、time/signal、fs/metadata/statfs/sysinfo、pipe/dup/lseek。
- Stats lane 生成 candidate matrix，必须区分：clean pass、pass_with_tconf、wrapper fail、inner TFAIL/TBROK、timeout、ENOSYS、panic/trap。
- 初始优先池：
  - Wave 1 proc/sched/getter/wait：`sched_getscheduler02`, `sched_getparam01`, `getpgid01`, `getgroups01`, `gettid02`, `waitpid01`, `gettimeofday02`, `getrusage02`, `getpriority01`, `getpriority02`, `setpriority01`, `setpriority02`, `times01`, `waitpid02`, `waitpid03`
  - Wave 2 time/signal：`clock_gettime03`, `clock_gettime04`, `clock_getres01`, `clock_nanosleep01`, `clock_nanosleep02`, `nanosleep01`, `nanosleep02`, `kill05`, `sigaction02`, `pause01`, `sigprocmask01`, `rt_sigprocmask01`, `sigpending02`, `sigsuspend01`
  - Wave 2 fs/metadata：`access02`, `access04`, `link02`, `rename01`, `unlink05`, `mkdir02`, `lseek02`, `pipe02`, `dup03`, `statfs01`, `statvfs01`, `fstatfs01`, `sysinfo01`

### Phase 2：Targeted Wave 1（12-18 cases）

- 先跑 RV targeted batch，建议 **12-18 cases**；如果只有少数失败，立即分派修复，不要缩回 6-case 小批。
- 同时准备 LA targeted；RV 无 panic/trap/大面积 timeout 后，尽快跑 LA 对照。
- Wave 1 修复重点：
  - `sched_getscheduler02`：查 LA musl TFAIL，修真实 scheduler policy/errno。
  - `sched_getparam01`：补真实 `sched_param` ABI/errno。
  - `getpgid01`：补 process group/session/ESRCH/EPERM 语义。
  - `getgroups01`：修 gid/group list ABI，不硬编码 case 输出。
  - `gettid02`：定位 glibc futex abort，修真实 thread/futex/tid 交互。
  - `waitpid01/02/03`：修 child state、WNOHANG、ECHILD、status copy-out。
  - `gettimeofday02`：解决 timeout/root cause，timeout 不计 PASS。
- 修复后按 case family 重测；不要等所有 case 都修好才 promotion，clean 子集可进入 Wave 1 promotion。

### Phase 3：Wave 1 promotion gate（目标 +10 到 +15）

- 将 **10-15 个 clean cases** 加入 `LTP_STABLE_CASES`，目标直接到 stable95-100。
- 跑 LA/RV targeted stable batch，保存 raw log、summary json/txt、promotion matrix。
- 如果 clean cases 少于 10，不结束；记录 blocker 后进入 Wave 2 targeted。

### Phase 4：Targeted Wave 2（time/signal + fs/metadata，10-16 cases）

- 同时推进 time/signal 与 fs/metadata，目标再拿 **5-10 个**。
- time/signal lane 优先修不会引入长等待的基础 case；任何 timeout 都 blocked。
- fs/metadata lane 优先低风险 errno/ABI：`lseek02`, `pipe02`, `dup03`, `mkdir02`, `rename01`, `unlink05`；`statfs/statvfs/fstatfs/sysinfo` 必须真实 ABI，不伪造内存/FS 信息。
- Wave 2 promotion 后目标 stable105-110；如果证据不 clean，停止在 stable100 附近并输出下一轮 blocker。

### Phase 5：Final full gate + review

最终交付前必须跑：

```bash
cargo fmt --all -- --check
./run-eval.sh la 2>&1 | tee output_la.md
./run-eval.sh 2>&1 | tee output_rv.md
python3 -B scripts/ltp_summary.py output_la.md
python3 -B scripts/ltp_summary.py output_rv.md
```

然后运行 changed-files-only ai-slop-cleaner、复跑必要验证、code-review，质量门必须为：

```json
{
  "aiSlopCleaner": { "status": "passed" },
  "verification": { "status": "passed" },
  "codeReview": { "recommendation": "APPROVE", "architectStatus": "CLEAR" }
}
```

## Team staffing guidance（激进版）

优先启动 **7 lanes**；如果 tmux/pane 空间不足，降为 6 lanes 并合并 Hard-blocker 与 Verification：

1. Leader：创建 Ultragoal、维护 gate/ledger、控制 targeted→promotion→full gate。
2. Discovery/Matrix：枚举 50-80 候选并维护 matrix。
3. Proc/Sched/Wait：修 process/session/scheduler/wait/getter，目标 Wave 1 主收益。
4. Time/Signal：修 clock/nanosleep/signal blockers，目标 Wave 2。
5. FS/Metadata：修 access/link/rename/statfs/sysinfo/pipe/dup/lseek，目标 Wave 2。
6. Hard-blocker/Runtime：专查 timeout、futex abort、panic/trap、RV memory pressure；不让单 case 卡住整批。
7. Verification/Review：审计 fake PASS/hardcode/silent SKIP/timeout-as-PASS，维护 final quality evidence。

Launch hint：

```bash
omx team 7:executor "aggressively continue LTP stable score improvement from stable85 toward stable100/110 with two targeted promotion waves; Ultragoal state is leader-owned only"
```

Fallback launch hint：

```bash
omx team 6:executor "aggressively continue LTP stable score improvement from stable85 toward stable100/110 with targeted validation first; Ultragoal state is leader-owned only"
```

## Acceptance criteria

- `LTP_STABLE_CASES` 达到 **100** 为主成功；达到 **105-110** 为 stretch success。
- 如果低于 100，必须证明继续 promotion 被真实 TFAIL/TBROK/timeout/ENOSYS/panic 或 ABI 风险阻塞，而不是因为过早停止。
- Final full gate LA/RV 均 exit 0，且 `ltp_summary.py` 报告 stable LTP `FAIL=0`、`timeout=0`、`ENOSYS=0`、`panic/trap=0`。
- `TFAIL/TBROK=0`；任何 TCONF 必须明确 case 与组合，不能隐藏。
- 最终报告包含修改文件/函数、行为变化、验证命令 exit code、LA/RV 汇总、blocked cases、remote sync 状态、ABI-visible changes。
