# Next session prompt: stable300 retained, continue toward stable350

工作目录：`/root/oskernel2026-orays`

上一轮目标 stable300 -> stable350 未达成。live `LTP_STABLE_CASES` 仍必须重新计算；当前交付报告记录为 300 total / 300 unique / 0 duplicates。

## 必须先做

1. `df -h / /root` 和 `du -sh /root/.codex`。
2. `git status --short`；只处理 agent-owned 变更。
3. live 重新统计 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`。
4. 确认没有 active evaluator/build/QEMU 进程；targeted gate 运行中只能轮询同一个 tool session，不要开第二个 evaluator。
5. 读取本目录报告，尤其：`candidate-matrix.md`、`stable315-promotion-gate-report.md`、`stable350-delivery-report.md`、`final-gate-quality-gate.json`、`worker3-fd-pipe-iovec-report.md`、`worker4-process-wait-report.md`。

## 关键约束

- 不伪造 PASS，不 hardcode case name，不把真实 TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap 转成 SKIP/TCONF/PASS。
- `raw/post-team-candidate*.log/status` 以及 `raw/followup-la-targeted-001/002/003-aborted-untrusted.log` 只说明 aborted/untrusted，不能作为 promotion evidence。
- 可信 follow-up evidence 只有：
  - `raw/followup-rv-targeted-001-summary.txt`
  - `raw/followup-la-targeted-004-summary.txt`
  - `raw/followup-marker-prefix-check.txt`
  - `raw/followup-la-sched_getscheduler02-afterfix-001-summary.txt`
- Worker 不拥有 `.omx/ultragoal` 或最终 `LTP_STABLE_CASES` 修改。

## 当前可信候选状态

Fresh RV+LA x musl+glibc clean seeds（现有 6 个，不足 stable315）：

```text
prctl05,sched_getscheduler02,sethostname01,setrlimit01,signal03,signal04
```

Near-clean blocker：

1. `pipe2_02`
   - Fresh RV both libc `TBROK=1`，helper copy/resource setup 仍失败。
   - 先修 helper/resource cwd/LTPROOT/PATH，再 RV targeted。
2. `waitpid01`
   - Fresh RV glibc PASS，musl `TFAIL=40`，`WIFSIGNALED()` 不符合预期。
   - 先查 musl wait-status / default-fatal signal delivery。

## 最小下一步

不要先改 `LTP_STABLE_CASES`。`sched_getscheduler02` 已经通过 follow-up LA afterfix gate，可作为第 6 个 clean seed。下一步优先从 `pipe2_02` 或 `waitpid01` 继续，先 RV targeted，只有 RV musl+glibc clean 后再串行 LA targeted。

若新修复带来更多 clean seed，必须累计至少 15 个 fresh RV+LA x musl+glibc clean case 后，再修改 stable list 并跑 stable315 aggregate gate。
