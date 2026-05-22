# Next session prompt: LTP stable score improvement with Ultragoal + Team

将下面整段复制到新的 Codex/OMX session 中使用。

```text
$ultragoal $team

目标：继续提高 `/root/oskernel2026-orays` 的 LTP stable 测试成绩。请读取并遵循仓库 AGENTS.md，继续使用 Ultragoal + Team 分阶段执行；Leader 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`，Team workers 只提供任务结果和证据，不直接 checkpoint Ultragoal。

今天是 2026-05-22，本轮新文档保存到 `docs/ltp-score-improvement-2026-05-22/`。昨天/历史证据目录保留为 `docs/ltp-score-improvement-2026-05-21-phase-*`；不要把历史证据目录改成今天，也不要新建未来日期目录。

如果这是同一个 Codex thread 延续旧 Ultragoal，请先确认旧 Codex goal 已 clear；新 session 通常可直接创建新的 Ultragoal plan。

当前已完成基线：
- 上一轮 Ultragoal 已 complete，最终 stable batch 已达到 **85 cases / libc / arch**。
- 最终 full evaluator gate 已完成：
  - `cargo fmt --all -- --check`: exit 0
  - `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0
  - `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_la.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_rv.md`: exit 0
  - LA: PASS LTP CASE 170, FAIL LTP CASE 0, ltp-musl 85/0, ltp-glibc 85/0, internal TFAIL=0, TBROK=0, TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
  - RV: PASS LTP CASE 170, FAIL LTP CASE 0, ltp-musl 85/0, ltp-glibc 85/0, internal TFAIL=0, TBROK=0, TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
- 已新增 stable cases：
  - `getresuid01 getresuid02 getresuid03 getresgid01 getresgid02 getresgid03 getsid01 rt_sigaction02 sched_getscheduler01 uname04`
- 透明风险：`read02` 仍有 TCONF，已记录为 pass_with_tconf，不要隐藏，也不要宣称 clean pass。
- final summary 中总 `timeout matches` 若出现，需区分非 LTP benchmark 组；LTP stable group timeout 必须是 0。
- full output 仍可能有非 LTP `iperf-glibc ... end: fail` markers；这不属于 LTP stable promotion 成功条件，但必须在最终报告中透明说明。

本轮目标（稍微激进）：
1. 主目标：从 stable85 提升到 **stable100**，即新增 15 个真实可验证 stable cases。
2. Stretch：如果证据足够 clean，继续冲 **stable105-110**。
3. 如果第一批 promotion 未满 15，不要保守停止；继续第二 targeted wave，直到真实 blocker 证明继续推进风险过高。
4. 不伪造 PASS，不 hardcode case name，不把真实失败静默转 SKIP。
5. timeout 必须单独计数，且不能算 PASS。
6. 不只看 `run-eval` exit code；必须用 `scripts/ltp_summary.py` 读取 LTP 内部 TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
7. 先 targeted batch，再 promotion，再 final full gate；不要一开始跑完整 `./run-eval.sh la` / `./run-eval.sh`。
8. 每次 promotion 必须说明：新增 case 列表、为什么可加入 stable、LA/RV × musl/glibc 证据、internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 是否为 0 或被透明记录。

请先读取这些文件：
- `AGENTS.md`
- `.omx/ultragoal/goals.json`
- `.omx/ultragoal/ledger.jsonl`
- `docs/ltp-score-improvement-2026-05-22/plan-stable85-to-110.md`
- `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-report.md`
- `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-quality-gate.json`
- `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-output-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-output-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-21-phase-d/stable85-targeted-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-21-phase-d/stable85-targeted-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-21-phase-d/targeted-promotion11-matrix.md`
- `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-ai-slop-cleaner-report.md`
- `docs/ltp-score-improvement-2026-05-21-phase-d/final-gate-code-review-report.md`
- `scripts/ltp_summary.py`
- `examples/shell/src/cmd.rs`
- `examples/shell/src/uspace/process_abi.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/resource_sched.rs`
- `examples/shell/src/uspace/signal_abi.rs`
- `examples/shell/src/uspace/system_info.rs`
- `examples/shell/src/uspace/synthetic_fs.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`
- `examples/shell/src/uspace/task_context.rs`

优先候选方向：
- proc/sched/getter/wait 近邻：fresh targeted 验证 `sched_getscheduler02`, `sched_getparam01`, `getpgid01`, `getgroups01`, `gettid02`, `waitpid01`, `waitpid02`, `waitpid03`, `gettimeofday02`, `getrusage02`, `getpriority01`, `getpriority02`, `setpriority01`, `setpriority02`, `times01`。注意 `getrusage02` 若仍 TCONF，不得作为 clean promotion；`gettimeofday02` 若 timeout，不得计 PASS。
- time/signal basic cases：优先 `clock_gettime03`, `clock_gettime04`, `clock_getres01`, `clock_nanosleep01`, `clock_nanosleep02`, `nanosleep01`, `nanosleep02`, `kill05`, `sigaction02`, `pause01`, `sigprocmask01`, `rt_sigprocmask01`, `sigpending02`, `sigsuspend01`。先 targeted 验证再修；`clock_getres01` 若仍 TCONF，不得作为 clean promotion。
- fs metadata/open/link/rename/statfs/access variants：重点修真实 ABI/errno，不要跳过失败。候选包括 `access02`, `access04`, `link02`, `rename01`, `unlink05`, `mkdir02`, `lseek02`, `pipe02`, `dup03`, `statfs01`, `statvfs01`, `fstatfs01`, `sysinfo01`。
- hard-blocker/runtime：单独调查 timeout、futex abort、panic/trap、RV memory pressure 和非 LTP iperf/libctest benchmark markers；不作为第一批 stable promotion 阻塞项，除非影响 targeted LTP stable gate。

上一轮明确未 promotion / blocked cases 包括：
`access02 access04 clock_getres01 clock_gettime01 dup03 fstatfs01 getpgid01 kill02 link02 lseek02 mkdir02 nanosleep01 nanosleep02 pause01 pipe02 read02 rename01 rt_sigprocmask01 sigpending02 sigprocmask01 sigsuspend01 statfs01 statvfs01 sysinfo01 unlink05`

这些 case 需要单独判断：
- `read02` 已在 stable 中但有 TCONF，保持透明记录。
- `clock_getres01` 当前可能仍是 TCONF，不要错误 promotion 为 clean pass。
- `statfs/statvfs/fstatfs/sysinfo` 需要真实 ABI 语义；不要伪造文件系统/内存信息。
- `access02/access04/link02/rename01/unlink05/mkdir02/lseek02/pipe02/dup03` 存在真实 TFAIL/TBROK/ENOSYS/errno 问题，先修再 promotion。
- `nanosleep*`, `pause01`, `sigprocmask*`, `sigsuspend01`, `kill02/kill05` 需要真实 signal/time 行为验证。

Team 分工建议（激进版）：
- Leader：
  - 创建新的 Ultragoal plan。
  - 维护 promotion gate 和 ledger。
  - 控制 targeted → promotion → final full gate 的顺序。
  - 最终运行 ai-slop-cleaner、verification、code-review，并用 quality-gate JSON 完成 Ultragoal。
- Discovery/Matrix lane：
  - 从 sdcard-rv.img / sdcard-la.img 和现有 docs 中枚举下一批 50-80 个候选。
  - 优先选择低风险、高收益、两架构都可验证的 cases。
  - 先跑 Wave 1 大小批，保存 raw logs 和 summary。
- Proc/Sched/Wait lane：
  - 根据 fresh targeted logs 修 process/session/scheduler/wait/getter 真实 ABI。
  - Wave 1 建议 12-18 cases，目标新增 10-15。
- Time/Signal lane：
  - 根据 fresh targeted logs 修 clock/nanosleep/signal blockers。
  - 作为 stable105/110 stretch 主来源之一。
- FS/Metadata lane：
  - 根据 fresh targeted logs 修 access/link/rename/unlink/mkdir/lseek/pipe/dup/statfs/sysinfo 真实 errno/ABI。
  - 不伪造文件系统/内存信息。
- Hard-blocker/Runtime lane：
  - 单独调查 timeout、futex abort、panic/trap、RV memory pressure 和非 LTP benchmark markers。
  - 不让单个 case 卡住整批 promotion。
- Verification/Review lane：
  - 审核是否存在 fake PASS、case-name hardcode、silent SKIP、timeout 被算 PASS。
  - 最终做 code-review + quality gate。

建议执行顺序：
1. 创建 `.omx/context/ltp-score-improvement-stable85-to-110-*.md`，总结当前 85-case baseline、约束、候选与风险；今天的新文档使用 `docs/ltp-score-improvement-2026-05-22/`。
2. `omx ultragoal create-goals --brief-file docs/ltp-score-improvement-2026-05-22/plan-stable85-to-110.md` 创建新 plan；检查 `.omx/ultragoal/goals.json`。
3. 启动 Team：
   - 首选：`omx team 7:executor "aggressively continue LTP stable score improvement from stable85 toward stable100/110 with two targeted promotion waves; Ultragoal state is leader-owned only"`
   - 资源不足 fallback：`omx team 6:executor "aggressively continue LTP stable score improvement from stable85 toward stable100/110 with targeted validation first; Ultragoal state is leader-owned only"`
4. Discovery/Matrix 先产出大候选 matrix；Leader 选择 Wave 1 的 12-18 cases。
5. Wave 1 targeted validation：先 RV，若无 panic/trap/大面积 timeout，立即跑 LA；失败 case 分派修复。
6. Wave 1 promotion：clean 子集直接 promotion，目标 stable95-100。
7. 如果未达 stable100，立即进入 Wave 2 targeted，不要过早停止。
8. Wave 2 promotion：time/signal 与 fs/metadata 中取 clean 子集，目标 stable105-110。
9. Promotion 到 stable 前必须跑 LA/RV targeted stable batch，并保存 summary。
10. 最终交付前再跑完整 final gate。

最终交付前必须跑：
- `cargo fmt --all -- --check`
- `./run-eval.sh la 2>&1 | tee output_la.md`
- `./run-eval.sh 2>&1 | tee output_rv.md`
- `python3 -B scripts/ltp_summary.py output_la.md`
- `python3 -B scripts/ltp_summary.py output_rv.md`

最终报告必须包括：
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

保存本轮新文档到：
- `docs/ltp-score-improvement-2026-05-22/`
```
