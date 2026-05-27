# Next-session prompt: stable413 follow-up / optional stable423+

工作目录：`/root/oskernel2026-orays`

请按仓库 `AGENTS.md` 执行，中文汇报。继续 LTP stable 提分时，以本目录作为 stop-state：`docs/ltp-score-improvement-2026-05-26-phase-a/`。

## 必须先 live 复核

1. 读取 `AGENTS.md`。
2. 磁盘 preflight：`df -h / /root`、`du -sh /root/.codex`；如果 `/` 接近满，先清理低价值临时日志/cache，不删 memories/skills/prompts/agents/凭据/活跃 `.omx`。
3. `git status --short`，只处理 agent-owned 变更，不回滚用户文件或未跟踪远程输出日志。
4. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算 stable 数量和重复项；不要依赖本提示词记忆。
5. 用 `scripts/ltp_summary.py` 复核本轮 final gate 摘要；不要用 wrapper exit status 代替 case matrix。

## 当前可信 stop-state

- 当前交付态：stable413。
- live stable list 在交付时为 413 total / 413 unique / 0 duplicates。
- 本轮新增 30 个 case：
  - stable393: `preadv01`, `preadv02`, `pwritev01`, `pwritev02`, `pread02`, `pread02_64`, `pwrite02`, `pwrite02_64`, `pwrite04`, `pwrite04_64`
  - stable403: `sendfile02`, `sendfile02_64`, `sendfile03`, `sendfile03_64`, `sendfile04`, `sendfile04_64`, `sendfile05`, `sendfile05_64`, `sendfile06`, `sendfile06_64`
  - stable413: `sendfile08`, `sendfile08_64`, `preadv201`, `preadv201_64`, `preadv202`, `preadv202_64`, `pwritev201`, `pwritev201_64`, `pwritev202`, `pwritev202_64`

## 关键证据路径

- Delivery: `stable413-delivery-report.md`
- Final quality gate: `final-gate-quality-gate.json`
- Code review: `final-gate-code-review-report.md`
- AI slop audit: `final-gate-ai-slop-cleaner-report.md`
- Marker/noise guardrail: `remote-marker-and-log-noise-regression-check.md`
- Final RV: `raw/stable413-rv-final-gate-002-summary.txt`
- Final LA: `raw/stable413-la-final-gate-002-summary.txt`

Final stable413 gate 结果：

- RV: PASS LTP CASE 826, FAIL 0；`ltp-musl` 413/0；`ltp-glibc` 413/0。
- LA: PASS LTP CASE 826, FAIL 0；`ltp-musl` 413/0；`ltp-glibc` 413/0。
- Internal TFAIL/TBROK/TCONF: 4，仍只有 known transparent `read02` TCONF；不得把它说成 clean，也不得把新增 TCONF 算 clean。
- timeout / ENOSYS / panic / trap: 0。
- marker-prefix bad lines: 0。
- `axfs::fops:297 [AxError::NotADirectory]`: 0；残余 `axfs_ramfs::file:69` NotADirectory: RV 22 / LA 22，可披露但未影响 marker。

## 已改源码行为

- `examples/shell/src/cmd.rs`: stable list 增至 413。
- `examples/shell/src/uspace/syscall_dispatch.rs`: 新增 `sendfile`、`preadv2`、`pwritev2` dispatch。
- `examples/shell/src/uspace/fd_table.rs`: 实现 general syscall 行为：负 positioned offset 返回 `EINVAL`、O_APPEND positioned write 追加、`sendfile`、`preadv2`/`pwritev2` flags/offset 语义。

这些不是 testcase-name 特判；不要引入 fake PASS、case-name hardcoding、LTP 源码修改或 failure laundering。

## 下一轮建议

目标可以是 stable423 或 stable430，但仍优先 easy/low-risk：

1. 先重新跑 small RV scout，不要直接全量硬冲 stable450。
2. 候选优先：剩余轻量 `preadv/pwritev/sendfile` 可扩展项、`poll/times/get*`、简单 metadata/statfs/getdents、轻量 open/creat/mkdir/rmdir/unlink。
3. 遇到设备/挂载工具、record lock、完整 VM/signal 模型、timeout、ENOSYS、panic/trap、真实 TFAIL/TBROK/TCONF，记录 blocker 后换候选。
4. 每次 promotion 只追加 8~12 个 clean case，leader 串行跑 RV+LA aggregate gate。
5. 如果使用 Team，worker 只做 discovery/修复/验证切片；leader 仍拥有 `.omx/ultragoal`、最终 `LTP_STABLE_CASES` 修改、promotion 决策、final gate 和 commit。

## 提醒

- 不提交 root-level `kernel-rv`/`kernel-la`、sdcard/disk image、大 raw `.log`，除非用户明确要求。
- 用 `python3 -B scripts/ltp_summary.py <log>` 或等价 case matrix 解析做证据。
- 若改动 syscall/errno/ABI-visible 行为，最终报告必须明确列出用户可见/POSIX 行为变化。
