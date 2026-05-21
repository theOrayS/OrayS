# 下一轮 LTP stable 提升计划：stable85 -> stable95/100

## 背景与当前基线

- 当前 stable LTP batch：**85 cases / libc / arch**。
- 最近一轮 full gate：LA/RV 均 `PASS LTP CASE 170`、`FAIL LTP CASE 0`；`ltp-musl 85/0`、`ltp-glibc 85/0`。
- 内部信号：LA/RV 均 `TFAIL=0`、`TBROK=0`、`TCONF=4`、`LTP timeout=0`、`ENOSYS=0`、`panic/trap=0`。
- `read02` 仍是透明 `pass_with_tconf`，不得隐藏，也不得作为 clean pass 宣称。
- 非 LTP `iperf ... end: fail` markers 仍存在，但不作为 stable LTP promotion 阻塞项，除非新证据证明影响 targeted stable gate。
- 本轮历史证据目录已按用户日期要求归档为：
  - `docs/ltp-score-improvement-2026-05-21/`
  - `docs/ltp-score-improvement-2026-05-21-phase-b/`
  - `docs/ltp-score-improvement-2026-05-21-phase-c/`
  - `docs/ltp-score-improvement-2026-05-21-phase-d/`

## 目标

1. 主目标：从 stable85 提升到 **stable95**，即新增 10 个真实可验证 stable cases。
2. Stretch：如果 targeted evidence 足够干净，继续提升到 **stable100**。
3. 不接受伪 PASS、case-name hardcode、静默 SKIP、timeout 算 PASS、只看 `run-eval` exit code。
4. 每批 promotion 必须有 LA/RV × musl/glibc evidence matrix，并分别报告 internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。

## RALPLAN-DR 摘要

### Principles

1. Evidence first：只有 fresh targeted LA/RV × musl/glibc 证据全绿的 case 才能 promotion。
2. Semantic fixes only：修真实 ABI/errno/signal/time/process 语义，不为 LTP case 名称开后门。
3. Small batches：每批 6-12 个候选，先 targeted，再 promotion，再 stable targeted，最后 full gate。
4. Honest accounting：timeout/TFAIL/TBROK/TCONF/ENOSYS/panic 必须单独计数，不得混入 PASS。
5. Leader-owned durability：Leader 维护 `.omx/ultragoal/goals.json` 和 `ledger.jsonl`；Team workers 只交证据和结果。

### Decision Drivers

1. 分数收益：优先可能一次贡献 6-10 clean cases 的 proc/sched/getter/wait 邻近组。
2. 风险控制：优先低内存压力、短 runtime、无 full-LTP CVE/OOM 牵连的 cases。
3. 真实语义：修 process/session/scheduler/signal/time 的共性 ABI，而不是扩大 stable list 掩盖失败。

### Viable Options

- Option A（推荐）：先修 proc/sched/getter/wait 小组，目标 stable95。
  - Pros：与最近 blocked evidence 最接近，失败集中，预计收益高。
  - Cons：`getpgid01`/`waitpid01` 可能牵涉 process group / wait semantics，需谨慎。
- Option B：先修 time/signal 小组。
  - Pros：长期收益高，能解锁多个 clock/nanosleep/signal cases。
  - Cons：timeout 和 signal delivery 风险更高，容易拖慢 batch。
- Option C：先修 fs/statfs/sysinfo/open/link 组。
  - Pros：覆盖面大。
  - Cons：真实 errno/metadata/FS ABI 风险较分散，不适合第一批 promotion。

Decision：采用 Option A 作为第一批，Option B/C 并行 discovery/triage，但不阻塞 stable95。

## 分阶段计划

### Phase 0：日期/上下文与目标状态确认

- 确认旧 Codex goal 已 clear；如果同一 Codex thread 延续旧 Ultragoal，先清理旧 goal 状态再创建新 Ultragoal。
- 新建 context/brief，但目录名日期按用户要求使用 `2026-05-21`，不要新建 `2026-05-22+` 日期目录。
- 读取当前关键证据：`docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-report.md`、`targeted-promotion11-matrix.md`、stable85 summaries、`scripts/ltp_summary.py`、`examples/shell/src/cmd.rs`。

### Phase 1：Discovery + candidate matrix

- Team Discovery lane 从 sdcard / docs / current outputs 中枚举 30-50 个候选。
- Stats lane 生成 candidate matrix，必须区分：clean pass、pass_with_tconf、wrapper fail、inner TFAIL/TBROK、timeout、ENOSYS、panic/trap。
- 初始优先候选：
  - proc/sched/getter/wait：`sched_getscheduler02`, `sched_getparam01`, `getpgid01`, `getgroups01`, `gettid02`, `waitpid01`, `gettimeofday02`, `getrusage02`（TCONF 需单独评估，不默认 promotion）
  - time/signal：`clock_gettime03`, `clock_gettime04`, `clock_nanosleep01`, `clock_nanosleep02`, `kill05`, `sigaction02`, `pause01`, `sigprocmask01`, `rt_sigprocmask01`, `sigsuspend01`
  - fs/metadata：`access02`, `access04`, `link02`, `rename01`, `unlink05`, `mkdir02`, `lseek02`, `pipe02`, `dup03`, `statfs01`, `statvfs01`, `fstatfs01`, `sysinfo01`

### Phase 2：Targeted batch A（proc/sched/getter/wait）

- 先跑 RV 小 batch，建议 8-12 个 cases；若 RV 明显不干净，不跑 LA 全量 batch。
- 对失败分类：
  - `sched_getscheduler02`：优先查 LA musl TFAIL 根因，确认 expected scheduler policy/errno。
  - `getpgid01`：补真实 process group/session/ESRCH/EPERM 语义，不伪造。
  - `getgroups01`：查 gid/group list ABI，不能硬编码 case 输出。
  - `gettid02`：查 glibc futex abort 根因，避免只改 wrapper。
  - `waitpid01`：查 wait semantics、child state、errno。
  - `gettimeofday02`：先解决 timeout/root cause，不把 timeout 计 PASS。
- 修复后逐个 targeted 重测；只有 LA/RV × musl/glibc 全 clean 才进入 promotion list。

### Phase 3：Promotion gate

- 将 6-10 个 clean cases 加入 `LTP_STABLE_CASES`。
- 跑 LA/RV targeted stable batch（不是 full gate），保存 raw log、summary json/txt、promotion matrix。
- Promotion 说明必须列出：新增 case、为什么可加入 stable、四组合 evidence、TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。

### Phase 4：若 stable95 达成，再尝试 stretch stable100

- 从 time/signal 或 fs/metadata 选择第二小批 5 cases。
- 不为了达到 100 降低 gate；若 evidence 不干净，停止在 stable95 并给下一轮 blocked list。

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

## Team staffing guidance

建议启动 6 lanes：

1. Leader：创建 Ultragoal、维护 gate/ledger、控制 targeted→promotion→full gate。
2. Discovery/Matrix：枚举候选并生成 candidate matrix。
3. Proc/Sched/Wait：修 process/session/scheduler/wait/getter 语义。
4. Time/Signal：修 clock/nanosleep/signal blockers，作为 stretch。
5. FS/Metadata：修 access/link/rename/statfs/sysinfo 等，作为后续池。
6. Verification/Review：审计 fake PASS/hardcode/silent SKIP/timeout-as-PASS，维护 final quality evidence。

Launch hint：

```bash
omx team 6:executor "continue LTP stable score improvement from stable85 toward stable95/100 with targeted validation first; Ultragoal state is leader-owned only"
```

## Acceptance criteria

- `LTP_STABLE_CASES` 至少达到 95，或如果 blocker 真实存在，则报告 stopped-at-85/90/等实际数量与明确 blocker。
- Final full gate LA/RV 均 exit 0，且 `ltp_summary.py` 报告 stable LTP `FAIL=0`、`timeout=0`、`ENOSYS=0`、`panic/trap=0`。
- `TFAIL/TBROK=0`；任何 TCONF 必须明确 case 与组合，不能隐藏。
- 最终报告包含修改文件/函数、行为变化、验证命令 exit code、LA/RV 汇总、blocked cases、remote sync 状态、ABI-visible changes。
