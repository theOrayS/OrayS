# Next session prompt: LTP stable score improvement with Ultragoal + Team

将下面整段复制到新的 Codex/OMX session 中使用。

```text
$ultragoal $team

目标：继续提高 `/root/oskernel2026-orays` 的 LTP stable 测试成绩。请读取并遵循仓库 AGENTS.md，继续使用 Ultragoal + Team 分阶段执行；Leader 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`，Team workers 只提供任务结果和证据，不直接 checkpoint Ultragoal。

如果这是同一个 Codex thread 延续旧 Ultragoal，请先确认旧 Codex goal 已 clear；新 session 通常可直接创建新的 Ultragoal plan。

当前已完成基线：
- 上一轮 Ultragoal 已 10/10 complete。
- stable LTP batch 已从 44 扩展到 63 cases / libc / arch。
- 最终 full evaluator gate 已完成：
  - `cargo fmt --all -- --check`: exit 0
  - `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0
  - `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0
  - LA: PASS LTP CASE 126, FAIL LTP CASE 0, ltp-musl 63/0, ltp-glibc 63/0, internal TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
  - RV: PASS LTP CASE 126, FAIL LTP CASE 0, ltp-musl 63/0, ltp-glibc 63/0, internal TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
- 已新增 stable cases：
  - `alarm02 alarm03 clock_gettime02 gettimeofday01 time01 times01 kill03 rt_sigaction01 sigaction01 proc01 exit01 exit02 exit_group01 getpgrp01 gettid01 uname01 getrlimit01 getrusage01 sched_yield01`
- 透明风险：`read02` 仍有 TCONF，已记录为 pass_with_tconf，不要隐藏。
- final summary 中总 `timeout matches: 10` 来自非 LTP benchmark 组；LTP stable group timeout 是 0。

本轮目标：
1. 继续提升 stable LTP batch，优先从 63 扩展到 80-100 cases；如果一次扩展风险过高，先做 75-85 的小 promotion。
2. 不伪造 PASS，不 hardcode case name，不把真实失败静默转 SKIP。
3. timeout 必须单独计数，且不能算 PASS。
4. 不只看 `run-eval` exit code；必须用 `scripts/ltp_summary.py` 读取 LTP 内部 TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
5. 先 targeted batch，再 final full gate；不要一开始跑完整 `./run-eval.sh la` / `./run-eval.sh`。
6. 每次 promotion 必须说明：新增 case 列表、为什么可加入 stable、LA/RV × musl/glibc 证据、internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 是否为 0 或被透明记录。

请先读取这些文件：
- `AGENTS.md`
- `.omx/ultragoal/goals.json`
- `.omx/ultragoal/ledger.jsonl`
- `docs/ltp-score-improvement-2026-05-22/final-gate-report.md`
- `docs/ltp-score-improvement-2026-05-22/final-gate-quality-gate.json`
- `docs/ltp-score-improvement-2026-05-22/final-gate-output-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-22/final-gate-output-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22/combined-promotion-candidates.txt`
- `docs/ltp-score-improvement-2026-05-22/combined-promotion-candidates.json`
- `docs/ltp-score-improvement-2026-05-22/runner-harness-report.md`
- `docs/ltp-score-improvement-2026-05-22/syscall-abi-candidate-report.md`
- `docs/ltp-score-improvement-2026-05-22/hard-blocker-report.md`
- `scripts/ltp_summary.py`
- `examples/shell/src/cmd.rs`

优先候选方向：
- fs metadata/open/link/rename/statfs/access variants：重点修真实 ABI/errno，不要跳过失败。
- proc/read-only metadata cases：只加入无真实失败、无 panic/trap、无 ENOSYS 的 case。
- time/signal basic cases：优先 `nanosleep*`, `clock_gettime01`, `sigprocmask*`, `sigsuspend01`, `pause01` 这类有明确 syscall/signal 语义边界的 case；先 targeted 验证再修。
- wait/exit/reporting 近邻 cases：寻找低风险 getter/wait/exit 变体。
- mmap/brk/msync 近邻 cases：注意 RV memory pressure，不要让 full-LTP CVE/OOM 阻塞 stable promotion。

上一轮明确未 promotion / blocked cases 包括：
`access02 access04 clock_getres01 clock_gettime01 dup03 fstatfs01 getpgid01 getsid01 kill02 link02 lseek02 mkdir02 nanosleep01 nanosleep02 pause01 pipe02 read02 rename01 rt_sigprocmask01 sigpending02 sigprocmask01 sigsuspend01 statfs01 statvfs01 sysinfo01 unlink05`

这些 case 需要单独判断：
- `read02` 已在 stable 中但有 TCONF，保持透明记录。
- `clock_getres01` 当前是 TCONF，不要错误 promotion 为 clean pass。
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
  - 从 sdcard-rv.img / sdcard-la.img 和现有 docs 中枚举下一批 20-40 个候选。
  - 优先选择低风险、高收益、两架构都可验证的 cases。
  - 先跑小 batch，保存 raw logs 和 summary。
- Stats/Report lane：
  - 使用或增强 `scripts/ltp_summary.py` 生成 promotion matrix。
  - 必须输出 LA/RV × musl/glibc、TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 分类。
- Runner/Harness lane：
  - 确认 `examples/shell/src/cmd.rs` 的 stable/batch/file runner 行为仍可复现。
  - 如需要，增加更清晰的 batch/file 配置，但不要改变 timeout-as-fail 语义。
- Syscall/ABI lane：
  - 只根据 fresh targeted logs 修最确定的 ABI/errno/metadata/proc/wait/time/signal 问题。
  - 重点文件：
    - `examples/shell/src/uspace/syscall_dispatch.rs`
    - `examples/shell/src/uspace/fd_table.rs`
    - `examples/shell/src/uspace/metadata.rs`
    - `examples/shell/src/uspace/synthetic_fs.rs`
    - `examples/shell/src/uspace/memory_map.rs`
    - `examples/shell/src/uspace/process_lifecycle.rs`
    - `examples/shell/src/uspace/signal_abi.rs`
    - `examples/shell/src/uspace/time_abi.rs`
- Hard-blocker lane：
  - 单独调查 RV CVE/OOM/full-LTP memory pressure 和 LA crash/trap。
  - 不作为第一批 stable promotion 阻塞项。
- Verification/Review lane：
  - 审核是否存在 fake PASS、case-name hardcode、silent SKIP、timeout 被算 PASS。
  - 最终做 code-review + quality gate。

建议执行顺序：
1. 创建 `.omx/context/ltp-score-improvement-next-*.md`，总结当前 63-case baseline、约束、候选与风险。
2. `omx ultragoal create-goals --brief-file <brief>` 创建新 plan；检查 `.omx/ultragoal/goals.json`。
3. 启动 Team：例如 `omx team 5:executor "continue LTP stable score improvement from 63 cases toward 80-100 with targeted validation first"`。
4. Discovery/Stats 先产出候选 matrix；Leader 只选低风险小 batch。
5. Targeted validation：每批建议 8-20 cases，不要过大。
6. 对失败 case 分类：真实失败留在 blocked，不 promotion；能真实修的由 Syscall/ABI lane 修复后重测。
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
- `docs/ltp-score-improvement-2026-05-23/`
或新 session 当天对应目录。
```
