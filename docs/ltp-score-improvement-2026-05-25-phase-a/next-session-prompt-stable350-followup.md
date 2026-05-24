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
- Worker 不拥有 `.omx/ultragoal` 或最终 `LTP_STABLE_CASES` 修改。
- raw logs 默认不要提交；提交摘要、报告、case lists、quality gate JSON。

## 当前可信 evidence

可信 follow-up evidence：

- `raw/followup-rv-targeted-001-summary.txt`
- `raw/followup-la-targeted-004-summary.txt`
- `raw/followup-marker-prefix-check.txt`
- `raw/followup-la-sched_getscheduler02-afterfix-001-summary.txt`
- `raw/followup-rv-waitpid01-maskrestore-001-summary.txt`
- `raw/followup-la-waitpid01-maskrestore-001-summary.txt`
- `raw/followup-rv-waitpid-signal-guard-001-summary.txt`
- `raw/followup-la-waitpid-signal-guard-001-summary.txt`
- `raw/followup-rv-pipe2_02-resource-prestage-003-summary.txt`
- `raw/followup-waitpid-marker-prefix-check.txt`

Fresh RV+LA x musl+glibc clean seeds（现有 8 个，不足 stable315）：

```text
prctl05,sched_getscheduler02,sethostname01,setrlimit01,signal03,signal04,waitpid01,pipe2_02
```

## 本轮修复结论

`waitpid01` 已修复为 clean seed：

- 根因：musl/LTP fork 路径会在 fork 临界区把 maskable signals 全部临时阻塞；子进程继承该 transient mask 后，`raise()` / `kill(getpid())` 的 default-fatal signals 没有终止子进程，导致 wait status 显示正常退出。
- 修复：记录 all-application-signal mask 之前的 signal mask；fork-like process child 继承恢复后的 mask，thread clone 仍继承 live mask。
- 证据：RV/LA `waitpid01` targeted PASS 2/0；RV/LA waitpid/signal guard PASS 16/0；internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 均为 0。

Near-clean blocker：

1. `pipe2_02` 已由 `/bin/sh` exec fallback 修复并成为 clean seed；不要重复从旧 TBROK 日志判断它仍阻塞。
2. `chmod05`、`access02/access04`、`statx01`、`writev03`、mmap/mprotect/munmap 系列仍按 `candidate-matrix.md` 排队。

## 最小下一步

不要先改 `LTP_STABLE_CASES`。下一步优先从 batch-A one-combo tails 或 permissions/metadata/iovec/VM blockers 继续，先 RV targeted，只有 RV musl+glibc clean 后再串行 LA targeted。

若新修复带来更多 clean seed，必须累计至少 15 个 fresh RV+LA x musl+glibc clean case 后，再修改 stable list 并跑 stable315 aggregate gate。
