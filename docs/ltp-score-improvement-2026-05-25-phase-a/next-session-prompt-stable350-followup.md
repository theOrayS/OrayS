# Next session prompt: stable300 retained, continue toward stable350

工作目录：`/root/oskernel2026-orays`

上一轮目标 stable300 -> stable350 未达成；不要把本轮 aborted/untrusted targeted logs 当作 promotion evidence。live `LTP_STABLE_CASES` 仍必须重新计算，当前交付报告记录为 300 total / 300 unique / 0 duplicates。

## 必须先做

1. `df -h / /root` 和 `du -sh /root/.codex`。
2. `git status --short`；只处理 agent-owned 变更。
3. live 重新统计 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`。
4. 确认没有 active evaluator/build/QEMU 进程：`ps -eo pid,ppid,pgid,stat,cmd | grep -E 'run-eval.sh|qemu-system|cargo -C examples/shell|make test_build' | grep -v grep || true`。
5. 读取本目录报告，尤其：`candidate-matrix.md`、`stable350-delivery-report.md`、`final-gate-quality-gate.json`、`worker3-fd-pipe-iovec-report.md`、`worker4-process-wait-report.md`。

## 关键约束

- 不伪造 PASS，不 hardcode case name，不把真实 TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap 转成 SKIP/TCONF/PASS。
- `post-team-candidate*.log` / `post-team-candidate*.status` 只说明 aborted/untrusted，不能作为 promotion evidence。
- 不并发运行多个 evaluator/QEMU；targeted gate 必须串行，且运行中只能轮询同一个 tool session。
- Worker 不拥有 `.omx/ultragoal` 或最终 `LTP_STABLE_CASES` 修改。

## 最小下一步

先只跑一个串行 RV targeted gate：

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe2_02,waitpid01,sched_getscheduler02,setrlimit01,signal03,signal04,prctl05,sethostname01 LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv 2>&1 | tee docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-rv-targeted-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-rv-targeted-001.log   | tee docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-rv-targeted-001-summary.txt
```

若 RV 不是 clean，不跑 LA；先修复 blocker。若 RV clean 只对一个小 subset 成立，则按同一 subset 串行跑 LA，再考虑 stable315。
