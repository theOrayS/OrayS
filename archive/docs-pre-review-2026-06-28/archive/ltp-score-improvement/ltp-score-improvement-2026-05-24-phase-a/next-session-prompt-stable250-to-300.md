# Next session prompt: stable250 -> stable300 with Ultragoal + Team

Created: 2026-05-24
Target repo: `/root/oskernel2026-orays`
Suggested starting commit from this handoff: `3df426a0 Preserve delivery hygiene without filling the evaluator host`

Use this prompt as the first message in the next Codex/OMX session.

```text
我现在要启动下一轮 LTP stable 提分任务：目标是从当前 stable250 提升到 stable300。请使用 Ultragoal + Team 模式推进，按仓库 AGENTS.md 执行，中文汇报。

工作目录：/root/oskernel2026-orays
当前已知基线：
- stable250 已完成并提交：38d52376 Raise stable LTP confidence with real credential and permission semantics
- 远程评测机 LTP 计分为 0 的上一轮修复已提交：99f11921 Keep remote LTP markers at console line start
- 最终证据/审计提交：fbc0e0c9 Close stable250 delivery with post-ANSI scoring evidence
- AGENTS.md 已加入磁盘检查与自动 commit 规则：3df426a0 Preserve delivery hygiene without filling the evaluator host
- 最终 stable250 gate 证据在 docs/ltp-score-improvement-2026-05-22-phase-d/：
  - stable250-post-ansi-rv-summary.txt：PASS LTP CASE 500, FAIL 0; ltp-musl 250/0; ltp-glibc 250/0; TCONF=4 known read02 only; timeout/ENOSYS/panic-trap 0
  - stable250-post-ansi-la-summary.txt：PASS LTP CASE 500, FAIL 0; ltp-musl 250/0; ltp-glibc 250/0; TCONF=4 known read02 only; timeout/ENOSYS/panic-trap 0
  - final-gate-quality-gate.json
  - final-gate-code-review-report.md
  - final-gate-ai-slop-cleaner-report.md
- 用户提供的远程评测机输出文件可能仍在仓库根目录：Riscv输出.txt、LoongArch输出.txt；它们是用户证据，默认不要提交。

启动要求：
1. 先读取 AGENTS.md 和本提示词，确认磁盘空间：`df -h / /root`，并检查 `/root/.codex`：`du -sh /root/.codex`。如果 `/` 接近满，先清理低价值临时日志/cache，不要删 memories/skills/prompts/agents/凭据/活跃 .omx 状态。
2. `git status --short`，确认只处理 agent-owned 变更；不要回滚用户文件或未跟踪远程输出日志。
3. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算当前 stable 数量，不能依赖记忆。
4. 创建/恢复 Ultragoal durable plan：目标 stable300，分阶段至少 stable270 -> stable285 -> stable300；每阶段都必须有 RV+LA、musl+glibc 的新候选 targeted evidence 和 stable aggregate gate。
5. 启动 Team 模式提高吞吐；如果 tmux pane/资源受限，优先 5 个 worker。Leader 负责 `.omx/ultragoal` 状态、`LTP_STABLE_CASES` 最终修改、promotion 决策和最终验证；worker 只做被分配的 discovery/修复/验证/报告切片。

强约束：
- 不伪造 PASS。
- 不 hardcode case name 通过。
- 不把真实 TFAIL/TBROK/timeout/panic/trap/ENOSYS 转成 SKIP/TCONF/PASS。
- timeout 不能计为 PASS。
- wrapper 成功不等于可 promotion；所有证据必须用 `python3 -B scripts/ltp_summary.py` 或等价 case matrix 解析。
- `read02` 的 `pass_with_tconf` 必须保持透明；clean 的定义是没有新增 internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
- 远程评分修复不能回退：LTP marker 行必须保持从行首开始，不能被 ANSI reset/color prefix 污染。任何日志输出改动后都要检查 marker prefix。
- 不提交 root-level kernel、sdcard/disk image、大 raw log、用户给的 `Riscv输出.txt`/`LoongArch输出.txt`，除非用户明确要求。

推荐 Team 分工：
- Worker 1: Discovery + promotion matrix
  - 从 testsuite/LTP 输出、sdcard bin names、现有 docs/raw、`scripts/ltp_summary.py --promotion-candidates` 等来源建立 250->330 候选池。
  - 输出 candidate matrix：按 subsystem、RV/LA、musl/glibc、clean/TCONF/TFAIL/TBROK/timeout/ENOSYS/panic 分类。
- Worker 2: proc/sched/wait/rlimit/process lane
  - 优先处理接近 clean、对 stable300 贡献高的 proc/sched/wait/rlimit/process cases。
- Worker 3: fd/pipe/open/access/fcntl/fsuid/permission lane
  - 重点防止上一轮 credential/fsid/open permission 语义回退；寻找可安全 promotion 的 fd/pipe/fcntl/access cases。
- Worker 4: fs/metadata/stat/statfs/link/rename/truncate lane
  - 处理 metadata、statx/statfs/statvfs、link/rename/truncate 等候选；注意 ABI/errno 语义。
- Worker 5: time/signal/timer/memory/mmap + verification guardrail lane
  - 处理 time/signal/timer/mmap/mlock 等候选；同时维护 no-fake-pass/no-timeout-as-pass 审计和 marker-prefix 检查。

执行节奏：
1. baseline refresh：
   - 统计当前 `LTP_STABLE_CASES` 数量和重复项。
   - 跑最小 smoke，确认 stable250 没有明显回退；如果时间允许，先 RV stable aggregate，再 LA stable aggregate。
2. candidate discovery：
   - 先 targeted batches，不要直接把大批未知 case 加入 stable。
   - 每批 5-15 个候选；优先两个架构、两个 libc 都 clean 的 case。
3. promotion gate：
   - 每新增 10-20 个 clean case，可以由 leader 更新 `LTP_STABLE_CASES`。
   - 每次 promotion 后至少运行 RV+LA stable targeted/aggregate gate，并用 `scripts/ltp_summary.py` 解析。
   - 如果任一 arch/libc 出现 FAIL、timeout、ENOSYS、panic/trap、新 TFAIL/TBROK/TCONF，停止 promotion，先修复或回退该候选。
4. final stable300 gate：
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv`
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la`
   - `python3 -B scripts/ltp_summary.py <rv-log>` 和 `<la-log>`
   - marker-prefix check：所有 `PASS LTP CASE`/`FAIL LTP CASE` marker 必须从行首开始。
   - `cargo fmt --all -- --check`
   - 相关构建：至少 `make A=examples/shell ARCH=riscv64`；涉及 remote submission 时跑 `make all`，必要时再跑离线 `make all`。

产物要求：
- 使用本地日期目录：`docs/ltp-score-improvement-2026-05-24-phase-a/`。
- 创建并持续更新：
  - `plan-stable250-to-300.md`
  - `candidate-matrix.md`
  - `stable270-promotion-gate-report.md`
  - `stable285-promotion-gate-report.md`
  - `stable300-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-regression-check.md`
- raw logs 放在 `docs/ltp-score-improvement-2026-05-24-phase-a/raw/`，默认不要提交大 raw logs；提交摘要、报告、case lists、quality gate JSON。
- `.omx/ultragoal/goals.json` 和 `ledger.jsonl` 是本地 audit trail，leader 维护；是否提交遵循仓库 ignore/用户要求。

交付条件：
- live `LTP_STABLE_CASES` 正好 300 个 unique case。
- RV final stable gate：PASS LTP CASE 600，FAIL 0；ltp-musl 300/0；ltp-glibc 300/0。
- LA final stable gate：PASS LTP CASE 600，FAIL 0；ltp-musl 300/0；ltp-glibc 300/0。
- internal TFAIL=0、TBROK=0；除已明确披露且可接受的 known TCONF 外不得新增 TCONF。若 `read02` 仍在 stable，则继续披露其 pass_with_tconf，不把它说成 clean。
- timeout/ENOSYS/panic/trap 均为 0。
- remote marker prefix 检查通过：0 bad marker lines。
- 已完成 code review + ai-slop-cleaner audit。
- 已按 AGENTS.md 自动 commit agent-owned tracked 变更，并在最终回复列出 commit SHA、验证命令、未能运行的检查、用户可见行为变化、ABI/POSIX 变化。

如果在 stable300 前遇到硬阻塞：
- 不要伪造或隐藏失败。
- 保存当前最高可信 stableN 的证据和 blocker 报告。
- 明确列出失败 case、arch/libc、内部 TFAIL/TBROK/TCONF、timeout/ENOSYS/panic/trap、相关日志路径、已尝试修复和下一步建议。
```
