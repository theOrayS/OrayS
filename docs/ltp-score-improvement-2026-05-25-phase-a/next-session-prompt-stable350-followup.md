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
- Worker 不拥有 `.omx/ultragoal` 或最终 `LTP_STABLE_CASES` 修改。

## 当前可信候选状态

Fresh RV+LA x musl+glibc clean seeds（只有 5 个，不足 stable315）：

```text
prctl05,sethostname01,setrlimit01,signal03,signal04
```

Near-clean blocker：

1. `sched_getscheduler02`
   - RV musl+glibc clean；LA glibc clean；LA musl `TFAIL=1`。
   - 优先检查 LA/musl libc wrapper vs syscall path 的 ESRCH/errno 语义。
2. `pipe2_02`
   - Fresh RV both libc `TBROK=1`，helper copy/resource setup 仍失败。
   - 先修 helper/resource cwd/LTPROOT/PATH，再 RV targeted。
3. `waitpid01`
   - Fresh RV glibc PASS，musl `TFAIL=40`，`WIFSIGNALED()` 不符合预期。
   - 先查 musl wait-status / default-fatal signal delivery。

## 最小下一步

不要先改 `LTP_STABLE_CASES`。优先修一个 near-clean blocker，推荐从 `sched_getscheduler02` 开始，因为只差 LA/musl：

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_getscheduler02 LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la 2>&1 | tee docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-la-sched_getscheduler02-afterfix-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-la-sched_getscheduler02-afterfix-001.log | tee docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-la-sched_getscheduler02-afterfix-001-summary.txt
```

若修复后 `sched_getscheduler02` 四路 clean，可加入 5 个 clean seeds，但仍需继续找满至少 15 个 clean case 后再跑 stable315 aggregate gate。
