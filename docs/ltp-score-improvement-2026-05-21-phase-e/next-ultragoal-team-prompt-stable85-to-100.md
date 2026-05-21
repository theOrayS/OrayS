# Next session prompt: Ultragoal + Team for LTP stable85 -> stable100/110

请在 `/root/oskernel2026-orays` 继续使用 `$ultragoal $team` 提升 LTP stable 测试成绩。本轮计划要比上一版更激进：主目标不是 stable95，而是 **stable100**；如果 evidence 允许，继续冲 **stable105-110**。

重要日期约束：今天是 5 月 22 日，但本仓库本轮相关证据目录名称需要统一使用 `2026-05-21` 日期前缀；不要新建 `2026-05-22`、`2026-05-23`、`2026-05-24` 等日期目录。若需要新目录，使用类似：

```text
docs/ltp-score-improvement-2026-05-21-phase-e/
docs/ltp-score-improvement-2026-05-21-phase-f/
```

如果这是同一个 Codex thread 延续旧 Ultragoal，请先确认旧 Codex goal 已 clear；新 session 通常可直接创建新的 Ultragoal plan。Leader 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`；Team workers 只提供任务结果和证据，不直接 checkpoint Ultragoal。

## 当前基线

- stable LTP 当前为 **85 cases / libc / arch**。
- 最近 final full gate 已通过：
  - `cargo fmt --all -- --check`: exit 0
  - `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0
  - `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_la.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_rv.md`: exit 0
- LA/RV 均：`PASS LTP CASE 170`, `FAIL LTP CASE 0`, `ltp-musl 85/0`, `ltp-glibc 85/0`, internal `TFAIL=0`, `TBROK=0`, `TCONF=4`, `LTP timeout=0`, `ENOSYS=0`, `panic/trap=0`。
- `read02` 仍是透明 `pass_with_tconf`；不得隐藏，不得宣称 clean。
- full output 仍有非 LTP `iperf ... end: fail` markers；不属于 stable LTP promotion gate，但最终报告必须透明说明。

## 必读文件

请先读取：

```text
AGENTS.md
.omx/ultragoal/goals.json
.omx/ultragoal/ledger.jsonl
.omx/plans/ltp-score-improvement-2026-05-21-stable85-to-100.md
docs/ltp-score-improvement-2026-05-21-phase-e/plan-stable85-to-100.md
docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-report.md
docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-quality-gate.json
docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-output-la-summary.txt
docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-output-rv-summary.txt
docs/ltp-score-improvement-2026-05-21-phase-d/stable85-targeted-la-summary.txt
docs/ltp-score-improvement-2026-05-21-phase-d/stable85-targeted-rv-summary.txt
docs/ltp-score-improvement-2026-05-21-phase-d/targeted-promotion11-matrix.md
docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-ai-slop-cleaner-report.md
docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-code-review-report.md
scripts/ltp_summary.py
examples/shell/src/cmd.rs
examples/shell/src/uspace/process_abi.rs
examples/shell/src/uspace/process_lifecycle.rs
examples/shell/src/uspace/resource_sched.rs
examples/shell/src/uspace/signal_abi.rs
examples/shell/src/uspace/system_info.rs
examples/shell/src/uspace/synthetic_fs.rs
examples/shell/src/uspace/syscall_dispatch.rs
examples/shell/src/uspace/task_context.rs
```

## 本轮激进目标

1. 主目标：从 stable85 提升到 **stable100**，即新增 15 个真实可验证 stable cases。
2. Stretch：如果证据足够 clean，继续到 **stable105-110**。
3. 不因为第一批没满 15 就保守收尾；应继续第二 targeted wave，直到真实 blocker 证明继续推进风险过高。
4. 不伪造 PASS，不 hardcode case name，不把真实失败静默转 SKIP。
5. timeout 必须单独计数，且不能算 PASS。
6. 不只看 `run-eval` exit code；必须用 `scripts/ltp_summary.py` 读取 LTP 内部 TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
7. 先 targeted batch，再 promotion，再 final full gate；不要一开始跑完整 `./run-eval.sh la` / `./run-eval.sh`。
8. 每次 promotion 必须说明：新增 case 列表、为什么可加入 stable、LA/RV × musl/glibc 证据、internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 是否为 0 或被透明记录。

## 更激进的候选与批次

Wave 1：proc/sched/getter/wait，建议一次 targeted **12-18 cases**，目标新增 10-15：

```text
sched_getscheduler02
sched_getparam01
getpgid01
getgroups01
gettid02
waitpid01
waitpid02
waitpid03
gettimeofday02
getrusage02
getpriority01
getpriority02
setpriority01
setpriority02
times01
```

注意：`getrusage02` 如果仍为 TCONF，不得作为 clean promotion。`gettimeofday02` 如果 timeout，不得计 PASS。`getpgid01` 可能需要真实 process group/session/ESRCH/EPERM 语义。

Wave 2：time/signal，作为 stable105/110 的第一 stretch 池：

```text
clock_gettime03 clock_gettime04 clock_getres01 clock_nanosleep01 clock_nanosleep02
nanosleep01 nanosleep02 kill05 sigaction02 pause01
sigprocmask01 rt_sigprocmask01 sigpending02 sigsuspend01
```

Wave 2：fs/metadata/statfs/sysinfo，作为 stable105/110 的第二 stretch 池：

```text
access02 access04 link02 rename01 unlink05 mkdir02 lseek02 pipe02 dup03
statfs01 statvfs01 fstatfs01 sysinfo01
```

这些必须修真实 ABI/errno，不要伪造文件系统/内存信息。

## Team 分工建议（激进版）

优先 7 lanes；如果 tmux/pane 空间不足，降为 6 lanes：

- Leader：创建新的 Ultragoal plan；维护 promotion gate 和 ledger；控制 targeted → promotion → final full gate 顺序。
- Discovery/Matrix lane：枚举 50-80 个候选，生成 candidate matrix。
- Proc/Sched/Wait lane：主攻 Wave 1，目标 stable100。
- Time/Signal lane：主攻 Wave 2 stretch。
- FS/Metadata lane：主攻 Wave 2 stretch。
- Hard-blocker/Runtime lane：专查 timeout、futex abort、panic/trap、RV memory pressure，避免一个 case 卡住整批。
- Verification/Review lane：审计 fake PASS、case-name hardcode、silent SKIP、timeout 被算 PASS；最终 code-review + quality gate。

建议启动：

```bash
omx team 7:executor "aggressively continue LTP stable score improvement from stable85 toward stable100/110 with two targeted promotion waves; Ultragoal state is leader-owned only"
```

如果 pane/资源不足：

```bash
omx team 6:executor "aggressively continue LTP stable score improvement from stable85 toward stable100/110 with targeted validation first; Ultragoal state is leader-owned only"
```

## 建议执行顺序

1. 创建 `.omx/context/ltp-score-improvement-stable85-to-110-*.md`，总结 stable85 baseline、约束、候选与风险；目录/文件引用不要使用 2026-05-22+ 日期目录名。
2. 用 `.omx/plans/ltp-score-improvement-2026-05-21-stable85-to-100.md` 或新 brief 创建 Ultragoal：

   ```bash
   omx ultragoal create-goals --brief-file .omx/plans/ltp-score-improvement-2026-05-21-stable85-to-100.md
   ```

3. 启动 Team，并把 `.omx/ultragoal` leader-owned 规则写入 worker inbox。
4. Discovery/Matrix 先产出大候选 matrix；Leader 选择 Wave 1 的 12-18 cases。
5. Wave 1 targeted validation：先 RV，若无 panic/trap/大面积 timeout，立即跑 LA；失败 case 分派修复。
6. Wave 1 promotion：clean 子集直接 promotion，目标 stable95-100。
7. 如果未达 stable100，立即进入 Wave 2 targeted，不要过早停止。
8. Wave 2 promotion：time/signal 与 fs/metadata 中取 clean 子集，目标 stable105-110。
9. 最终交付前再跑完整 final gate。

## 最终交付前必须跑

```bash
cargo fmt --all -- --check
./run-eval.sh la 2>&1 | tee output_la.md
./run-eval.sh 2>&1 | tee output_rv.md
python3 -B scripts/ltp_summary.py output_la.md
python3 -B scripts/ltp_summary.py output_rv.md
```

## 最终报告必须包括

- 修改文件
- 修改函数/常量
- 每项修复的预期行为
- 实际验证命令和 exit code
- LA/RV pass/fail 汇总
- internal TFAIL/TBROK/TCONF
- timeout / ENOSYS / panic/trap
- stable batch 新增 case
- 不纳入 stable 的 blocked cases 及原因
- 如果低于 stable100，说明为什么继续 promotion 被真实 blocker 阻止
- 未完成风险和下一批建议
- 是否同步 `/root/oskernel2026-orays-remote`，以及保留了哪些 remote-only address-mapping differences
- 是否有 syscall / errno / ABI-visible 行为变化；如果没有，明确说明没有
