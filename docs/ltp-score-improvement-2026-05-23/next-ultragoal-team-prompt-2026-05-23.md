# Next Ultragoal + Team prompt: LTP stable 75 -> 90+ candidates

## 下一步计划

### 目标

继续提高 `/root/oskernel2026-orays` 的 LTP stable 测试成绩，从当前 **75 cases / libc / arch** 向 **90-100 cases / libc / arch** 推进。若一次 promotion 风险过高，先做 **80-88 cases** 的小批次 promotion。

### 当前基线

- 当前 stable：75 cases / libc / arch。
- final full gate 已完成：
  - LA：`PASS LTP CASE 150`，`FAIL LTP CASE 0`，`ltp-musl 75/0`，`ltp-glibc 75/0`。
  - RV：`PASS LTP CASE 150`，`FAIL LTP CASE 0`，`ltp-musl 75/0`，`ltp-glibc 75/0`。
  - LA/RV：internal `TFAIL=0`，`TBROK=0`，`TCONF=4`，LTP timeout `0`，ENOSYS `0`，panic/trap `0`。
- `read02` 仍是 stable 中的 pass-with-TCONF；必须继续透明记录，不能改写为 clean pass。
- `scripts/ltp_summary.py` 顶层 `timeout matches: 10` 仍来自非 LTP benchmark 组；LTP stable group timeout 是 0。
- 非 LTP `iperf-glibc ... end: fail` markers 仍存在，不属于 LTP stable promotion 成功条件；如要处理需单独开非 LTP lane。

### 已新增 stable cases

`getpgid02 getsid02 getppid02 getuid03 geteuid02 getgid03 getegid02 getgroups03 uname02 wait01 wait02 getrlimit02`

### 重点候选方向

1. **proc/session/wait/getter 近邻**
   - 优先 fresh targeted 验证：`getpgid01`, `getsid01`, 以及 sdcard 中可用的 getpid/getppid/getgroups/wait/getrlimit 近邻 case。
   - 注意：`setsid()` 当前仍是简化实现，不要声称完整 POSIX session 语义；若 case 需要 process-group-leader `EPERM`，必须真实补齐。

2. **time/signal 基础语义**
   - 候选：`clock_gettime01`, `clock_getres01`, `nanosleep01`, `nanosleep02`, `pause01`, `rt_sigprocmask01`, `sigprocmask01`, `sigpending02`, `sigsuspend01`, `kill02`。
   - 先 targeted 验证，再根据 fresh logs 修真实 signal/time 行为；`clock_getres01` 若仍 TCONF，不得作为 clean promotion。

3. **fs metadata / errno / open-link-rename variants**
   - 候选：`access02`, `access04`, `link02`, `rename01`, `unlink05`, `mkdir02`, `lseek02`, `pipe02`, `dup03`。
   - 必须修真实 errno/ABI/FD 行为，不允许把 TFAIL/TBROK/ENOSYS 静默转 SKIP。

4. **statfs/statvfs/fstatfs/sysinfo**
   - 候选：`statfs01`, `statvfs01`, `fstatfs01`, `sysinfo01`。
   - 只有实现真实 ABI 结构/字段语义后才能 promotion；不要伪造文件系统或内存信息。

5. **mmap/brk/msync 近邻**
   - 可做探索但不要让 full-LTP CVE/OOM 阻塞 stable promotion。
   - RV memory pressure 仍需独立 hard-blocker lane 观察。

### 建议 Ultragoal stories

- G001：建立 baseline 与 guardrails：75-case 基线、TCONF/timeout/ENOSYS/panic 分类、不可伪造 PASS。
- G002：Discovery lane 枚举 sdcard LA/RV musl/glibc 中未 stable 的候选，输出 30-50 个候选清单。
- G003：Stats/Report lane 生成候选 matrix：LA/RV × musl/glibc、TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap、pass_clean/pass_with_tconf。
- G004：针对最小 batch 做 fresh targeted run；每批 8-16 cases，优先 proc/wait/getter + time/signal 的低风险组合。
- G005：Syscall/ABI lane 只修 fresh targeted logs 证明的真实 ABI/errno 问题。
- G006：Promotion lane 只把 LA/RV × musl/glibc clean 的 case 加入 `LTP_STABLE_CASES`；TCONF 可透明记录但不得伪装 clean。
- G007：Targeted stable gate：LA/RV stable batch targeted run + `scripts/ltp_summary.py`。
- G008：Final full gate：fmt + LA/RV full evaluator + ltp_summary。
- G009：Final quality gate：ai-slop-cleaner、code-review、architect CLEAR、quality-gate JSON、Ultragoal checkpoint。

### Team 分工建议

- Leader：维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`，选择 promotion batch，控制 targeted -> promotion -> final gate 顺序。
- Discovery lane：枚举 sdcard/文档候选，避免 stale evidence promotion。
- Stats/Report lane：维护 matrix 和 summary，确保 timeout 不算 PASS。
- Proc/Wait/Session lane：处理 `getpgid/getsid/wait/getrlimit` 近邻真实 ABI。
- Time/Signal lane：处理 `clock_gettime/clock_getres/nanosleep/pause/sigprocmask/sigsuspend/kill` 真实语义。
- FS/Metadata lane：处理 `access/link/rename/unlink/mkdir/lseek/pipe/dup/statfs/sysinfo` 真实 errno/ABI。
- Verification/Review lane：审核 fake PASS、case-name hardcode、silent SKIP、timeout-as-pass，最终 code-review + quality gate。

### 必须验证

Promotion 前：

- LA/RV targeted candidate batch。
- LA/RV targeted promoted stable batch。
- `python3 -B scripts/ltp_summary.py <log>`，读取 internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。

最终交付前：

- `cargo fmt --all -- --check`
- `./run-eval.sh la 2>&1 | tee output_la.md`
- `./run-eval.sh 2>&1 | tee output_rv.md`
- `python3 -B scripts/ltp_summary.py output_la.md`
- `python3 -B scripts/ltp_summary.py output_rv.md`

---

## 可直接复制的新 session 提示词

```text
$ultragoal $team

目标：继续提高 `/root/oskernel2026-orays` 的 LTP stable 测试成绩。请读取并遵循仓库 AGENTS.md，继续使用 Ultragoal + Team 分阶段执行；Leader 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`，Team workers 只提供任务结果和证据，不直接 checkpoint Ultragoal。

如果这是同一个 Codex thread 延续旧 Ultragoal，请先确认旧 Codex goal 已 clear；新 session 通常可直接创建新的 Ultragoal plan。

当前已完成基线：
- 上一轮 Ultragoal 已 19/19 complete。
- stable LTP batch 已从 63 扩展到 75 cases / libc / arch。
- 最终 full evaluator gate 已完成：
  - `cargo fmt --all -- --check`: exit 0
  - `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0
  - `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_la.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_rv.md`: exit 0
  - LA: PASS LTP CASE 150, FAIL LTP CASE 0, ltp-musl 75/0, ltp-glibc 75/0, internal TFAIL=0, TBROK=0, TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
  - RV: PASS LTP CASE 150, FAIL LTP CASE 0, ltp-musl 75/0, ltp-glibc 75/0, internal TFAIL=0, TBROK=0, TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
- 已新增 stable cases：
  - `getpgid02 getsid02 getppid02 getuid03 geteuid02 getgid03 getegid02 getgroups03 uname02 wait01 wait02 getrlimit02`
- 透明风险：`read02` 仍有 TCONF，已记录为 pass_with_tconf，不要隐藏。
- final summary 中总 `timeout matches: 10` 来自非 LTP benchmark 组；LTP stable group timeout 是 0。
- full output 仍有非 LTP `iperf-glibc ... end: fail` markers；这不属于 LTP stable promotion 成功条件，但必须在最终报告中透明说明。

本轮目标：
1. 继续提升 stable LTP batch，优先从 75 扩展到 90-100 cases；如果一次扩展风险过高，先做 80-88 的小 promotion。
2. 不伪造 PASS，不 hardcode case name，不把真实失败静默转 SKIP。
3. timeout 必须单独计数，且不能算 PASS。
4. 不只看 `run-eval` exit code；必须用 `scripts/ltp_summary.py` 读取 LTP 内部 TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
5. 先 targeted batch，再 promotion，再 final full gate；不要一开始跑完整 `./run-eval.sh la` / `./run-eval.sh`。
6. 每次 promotion 必须说明：新增 case 列表、为什么可加入 stable、LA/RV × musl/glibc 证据、internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 是否为 0 或被透明记录。

请先读取这些文件：
- `AGENTS.md`
- `.omx/ultragoal/goals.json`
- `.omx/ultragoal/ledger.jsonl`
- `docs/ltp-score-improvement-2026-05-23/final-gate-report.md`
- `docs/ltp-score-improvement-2026-05-23/final-gate-quality-gate.json`
- `docs/ltp-score-improvement-2026-05-23/final-gate-output-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-23/final-gate-output-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-23/stable75-targeted-matrix.md`
- `docs/ltp-score-improvement-2026-05-23/targeted-promotion12-matrix.md`
- `docs/ltp-score-improvement-2026-05-23/final-gate-ai-slop-cleaner-report.md`
- `docs/ltp-score-improvement-2026-05-23/final-gate-code-review-report.md`
- `scripts/ltp_summary.py`
- `examples/shell/src/cmd.rs`
- `examples/shell/src/uspace/process_abi.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/resource_sched.rs`
- `examples/shell/src/uspace/synthetic_fs.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`

优先候选方向：
- proc/session/wait/getter 近邻：fresh targeted 验证 `getpgid01`, `getsid01`, 以及 sdcard 中可用的 getpid/getppid/getgroups/wait/getrlimit 近邻 cases。注意 `setsid()` 仍是简化实现，若 case 需要 process-group-leader `EPERM`，必须真实补齐。
- time/signal basic cases：优先 `clock_gettime01`, `clock_getres01`, `nanosleep01`, `nanosleep02`, `pause01`, `rt_sigprocmask01`, `sigprocmask01`, `sigpending02`, `sigsuspend01`, `kill02`。先 targeted 验证再修；`clock_getres01` 若仍 TCONF，不得作为 clean promotion。
- fs metadata/open/link/rename/statfs/access variants：重点修真实 ABI/errno，不要跳过失败。候选包括 `access02`, `access04`, `link02`, `rename01`, `unlink05`, `mkdir02`, `lseek02`, `pipe02`, `dup03`。
- statfs/statvfs/fstatfs/sysinfo：需要真实 ABI 语义；不要伪造文件系统/内存信息。
- mmap/brk/msync 近邻：注意 RV memory pressure，不要让 full-LTP CVE/OOM 阻塞 stable promotion。

上一轮明确未 promotion / blocked cases 包括：
`access02 access04 clock_getres01 clock_gettime01 dup03 fstatfs01 getpgid01 getsid01 kill02 link02 lseek02 mkdir02 nanosleep01 nanosleep02 pause01 pipe02 read02 rename01 rt_sigprocmask01 sigpending02 sigprocmask01 sigsuspend01 statfs01 statvfs01 sysinfo01 unlink05`

这些 case 需要单独判断：
- `read02` 已在 stable 中但有 TCONF，保持透明记录。
- `clock_getres01` 当前可能仍是 TCONF，不要错误 promotion 为 clean pass。
- `statfs/statvfs/fstatfs/sysinfo` 需要真实 ABI 语义；不要伪造文件系统/内存信息。
- `access02/access04/link02/rename01/unlink05/mkdir02/lseek02/pipe02/dup03` 存在真实 TFAIL/TBROK/ENOSYS/errno 问题，先修再 promotion。
- `nanosleep*`, `pause01`, `sigprocmask*`, `sigsuspend01`, `kill02` 需要真实 signal/time 行为验证。

Team 分工建议：
- Leader：
  - 创建新的 Ultragoal plan。
  - 维护 promotion gate 和 ledger。
  - 控制 targeted → promotion → final full gate 的顺序。
  - 最终运行 ai-slop-cleaner、verification、code-review，并用 quality-gate JSON 完成 Ultragoal。
- Discovery lane：
  - 从 sdcard-rv.img / sdcard-la.img 和现有 docs 中枚举下一批 30-50 个候选。
  - 优先选择低风险、高收益、两架构都可验证的 cases。
  - 先跑小 batch，保存 raw logs 和 summary。
- Stats/Report lane：
  - 使用或增强 `scripts/ltp_summary.py` 生成 promotion matrix。
  - 必须输出 LA/RV × musl/glibc、TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 分类。
- Proc/Wait/Session lane：
  - 根据 fresh targeted logs 修 `getpgid/getsid/wait/getrlimit` 近邻真实 ABI。
- Time/Signal lane：
  - 根据 fresh targeted logs 修 `clock_gettime/clock_getres/nanosleep/pause/sigprocmask/sigsuspend/kill` 真实语义。
- FS/Metadata lane：
  - 根据 fresh targeted logs 修 `access/link/rename/unlink/mkdir/lseek/pipe/dup/statfs/sysinfo` 真实 errno/ABI。
- Hard-blocker / Non-LTP observation lane：
  - 单独调查 RV full-LTP memory pressure 和非 LTP iperf/libctest benchmark markers。
  - 不作为第一批 stable promotion 阻塞项，除非影响 targeted LTP stable gate。
- Verification/Review lane：
  - 审核是否存在 fake PASS、case-name hardcode、silent SKIP、timeout 被算 PASS。
  - 最终做 code-review + quality gate。

建议执行顺序：
1. 创建 `.omx/context/ltp-score-improvement-next-*.md`，总结当前 75-case baseline、约束、候选与风险。
2. `omx ultragoal create-goals --brief-file <brief>` 创建新 plan；检查 `.omx/ultragoal/goals.json`。
3. 启动 Team：例如 `omx team 6:executor "continue LTP stable score improvement from 75 cases toward 90-100 with targeted validation first"`。
4. Discovery/Stats 先产出候选 matrix；Leader 只选低风险小 batch。
5. Targeted validation：每批建议 8-16 cases，不要过大。
6. 对失败 case 分类：真实失败留在 blocked，不 promotion；能真实修的由对应 ABI lane 修复后重测。
7. Promotion 到 stable 前必须跑 LA/RV targeted stable batch，并保存 summary。
8. 最终交付前再跑完整 final gate。

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
- 未完成风险和下一批建议
- 是否有 syscall / errno / ABI-visible 行为变化；如果没有，明确说明没有。

保存本轮新文档到：
- `docs/ltp-score-improvement-2026-05-24/`
或新 session 当天对应目录。
```
