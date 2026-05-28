# Next session prompt: stable300 -> stable350 with Ultragoal + Team

Created: 2026-05-25
Target repo: `/root/oskernel2026-orays`
Suggested starting commit from this handoff: `59e4c70d Prepare the next high-value LTP scoring push` or later

Use this prompt as the first message in the next Codex/OMX session.

```text
我现在要启动下一轮 LTP stable 提分任务：目标是从当前 stable300 提升到 stable350。请使用 Ultragoal + Team 模式推进，按仓库 AGENTS.md 执行，中文汇报。

工作目录：/root/oskernel2026-orays
当前已知基线（必须 live 复核，不要只依赖本提示词）：
- stable300 已完成并提交：f4dfa42d Raise stable LTP coverage to stable300 with verified syscall semantics
- Team shutdown 后当前分支可能还有无树差异 merge commit：df342f5a Merge commit '5f8cded0ac7726dc28284288cec78d99019cf67b' into refactor/moss_kernel_like
- 下一轮提示词提交：59e4c70d Prepare the next high-value LTP scoring push
- live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 在提示词创建时为 300 total / 300 unique / 0 duplicates；下一会话仍必须重新计算。
- 最终 stable300 gate 证据在 docs/ltp-score-improvement-2026-05-24-phase-a/：
  - stable300-rv-final-summary.txt：PASS LTP CASE 600, FAIL 0; ltp-musl 300/0; ltp-glibc 300/0; TCONF=4 known read02 only; timeout/ENOSYS/panic-trap 0
  - stable300-la-final-summary.txt：PASS LTP CASE 600, FAIL 0; ltp-musl 300/0; ltp-glibc 300/0; TCONF=4 known read02 only; timeout/ENOSYS/panic-trap 0
  - candidate-matrix.md
  - stable300-delivery-report.md
  - final-gate-quality-gate.json
  - final-gate-code-review-report.md
  - final-gate-ai-slop-cleaner-report.md
  - remote-marker-regression-check.md
- 上一轮用户优先但未 promotion 的高价值 blocker：access02, access04, chmod05, statx01, writev03, pipe2_02, waitpid01, mmap04, mmap05, mmap06, mprotect01, mprotect02, munmap01。它们可以优先修复/重测，但不能直接加入 stable。
- 用户提供的远程评测机输出文件可能仍在仓库根目录：Riscv输出.txt、LoongArch输出.txt；它们是用户证据，默认不要提交。

启动要求：
1. 先读取 AGENTS.md 和本提示词，确认磁盘空间：`df -h / /root`，并检查 `/root/.codex`：`du -sh /root/.codex`。如果 `/` 接近满，先清理低价值临时日志/cache，不要删 memories/skills/prompts/agents/凭据/活跃 .omx 状态。
2. `git status --short`，确认只处理 agent-owned 变更；不要回滚用户文件或未跟踪远程输出日志。
3. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算当前 stable 数量和重复项，不能依赖记忆。
4. 复核 stable300 final summaries、candidate-matrix 和 delivery report；上一轮 blocker 只能作为候选/修复入口，不能当作 clean evidence。
5. 创建/恢复 Ultragoal durable plan：目标 stable350，分阶段至少 stable315 -> stable330 -> stable350；每阶段都必须有 RV+LA、musl+glibc 的新候选 targeted evidence 和 stable aggregate gate。
6. 启动 Team 模式提高吞吐；如果 tmux pane/资源受限，优先 5 个 worker，失败再降到 4 个 worker。Leader 负责 `.omx/ultragoal` 状态、`LTP_STABLE_CASES` 最终修改、promotion 决策和最终验证；worker 只做被分配的 discovery/修复/验证/报告切片。

Ultragoal 具体用法：
1. 创建 plan：
   - `omx ultragoal create-goals --brief-file docs/ltp-score-improvement-2026-05-25-phase-a/next-session-prompt-stable300-to-350.md`
   - 然后检查 `.omx/ultragoal/goals.json` 和 `.omx/ultragoal/ledger.jsonl`。
2. 执行每个 story：
   - `omx ultragoal status`
   - `omx ultragoal complete-goals`
   - 按输出 handoff 使用 Codex goal 工具：先 `get_goal`；如无 active goal，则 `create_goal`；中间 story 完成时不要 `update_goal complete`。
3. 每个阶段完成后，leader 用新鲜 `get_goal` snapshot checkpoint：
   - `omx ultragoal checkpoint --goal-id <id> --status complete --evidence "<证据路径和摘要>" --codex-goal-json <fresh-get-goal-json-or-path>`
4. 最终 story 必须先完成 verification、ai-slop-cleaner、code-review clean gate；只有 final clean 后才 `update_goal({status: "complete"})`，再用 complete snapshot checkpoint。
5. Team workers 不拥有 `.omx/ultragoal`，不创建 worker ledger，不 checkpoint Ultragoal，不最终修改 `LTP_STABLE_CASES`。

Team 具体用法：
1. 启动前先创建/复用 context snapshot，并做 preflight：
   - `tmux -V`
   - `test -n "$TMUX"`
   - `command -v omx`
   - `tmux list-panes -F '#{pane_id}\t#{pane_start_command}' | rg 'hud --watch' || true`
2. 推荐启动：
   - `omx team 5:executor "LTP stable300 to stable350 high-value promotion with Ultragoal leader-owned gates"`
   - 如果 pane/资源不足，降级为 `omx team 4:executor "LTP stable300 to stable350 high-value promotion with Ultragoal leader-owned gates"`。
3. 启动后确认 team line、tmux target、worker panes、ACK mailbox。
4. 监控优先使用 runtime/state：`omx team status <team>`、`omx team resume <team>`、mailbox、task JSON。
5. 只有 pending=0、in_progress=0、failed=0（或失败路径已明确保存 blocker）后，才运行 `omx team shutdown <team>`。
6. 不要让多个 worker 并发争用默认 QEMU/sdcard/qcow2。若没有隔离镜像，promotion gate 和 final gate 由 leader 串行跑；worker 并发输出只能作为 discovery，不能作为 promotion 证据。

强约束：
- 不伪造 PASS。
- 不 hardcode case name 通过。
- 不把真实 TFAIL/TBROK/timeout/panic/trap/ENOSYS 转成 SKIP/TCONF/PASS。
- timeout 不能计为 PASS。
- wrapper 成功不等于可 promotion；所有证据必须用 `python3 -B scripts/ltp_summary.py` 或等价 case matrix 解析。
- `read02` 的 `pass_with_tconf` 必须保持透明；clean 的定义是没有新增 internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
- 远程评分修复不能回退：LTP marker 行必须保持从行首开始，不能被 ANSI reset/color prefix 污染。任何日志输出改动后都要检查 marker prefix。
- 不提交 root-level kernel、sdcard/disk image、大 raw log、用户给的 `Riscv输出.txt`/`LoongArch输出.txt`，除非用户明确要求。

高价值刷分策略：
1. 本轮目标不是随便凑 50 个；优先 score-per-development-time 高、能带动一簇相邻 case 的真实 syscall/ABI 语义修复。
2. 候选优先级：
   - 已在某一 arch/libc clean、另一侧只差小 errno/边界语义的 case。
   - 同 subsystem 共用一个真实修复即可带动多个 case 的 batch。
   - 对隐藏测试防守价值高的权限/VFS/errno、fd/pipe/iovec、process/wait/rlimit/proc、mmap/mprotect/signal/time case。
   - 运行时间短、稳定、不会在大 stable aggregate 中制造 timeout 的 case。
3. 每批 targeted 5-15 个候选，先建立 matrix，再 promotion；不要直接把大批未知 case 加入 stable。
4. 可以 targeted 验证 80-150 个候选来找 clean subset，但只有 RV+LA × musl+glibc 全 clean 的 case 才能加入 stable。
5. 若一个高价值 blocker 短期修不 clean，先记录 blocker 并转向其他 clean subset，不要让单个 case 阻塞整轮。

推荐 Team 分工：
- Worker 1: Discovery + high-value promotion matrix
  - 从 testsuite/LTP 输出、sdcard bin names、现有 docs/raw、`scripts/ltp_summary.py --promotion-candidates`、当前 stable list 建立 300->380 候选池。
  - 输出 candidate matrix：按 subsystem、价值评分、RV/LA、musl/glibc、clean/TCONF/TFAIL/TBROK/timeout/ENOSYS/panic 分类。
- Worker 2: permissions/VFS/errno/metadata lane
  - 优先 access02、access04、chmod05、statx01 及相邻 chmod/chown/stat/statfs/statvfs/link/rename/truncate/open 候选。
  - 注意真实 errno、权限、uid/gid/fsgid、O_PATH、AT_EMPTY_PATH、setgid/sticky/目录语义。
- Worker 3: fd/pipe/iovec/fcntl lane
  - 优先 writev03、pipe2_02 及相邻 readv/writev/preadv/pwritev/pipe/pipe2/dup/fcntl/poll/select 候选。
  - 守住上一轮 shared offset、O_APPEND、pipe capacity/FIONREAD、blocking/yield、SIGPIPE 语义，避免回退。
- Worker 4: process/wait/sched/rlimit/proc lane
  - 优先 waitpid01 与 scheduler negative-pid、rlimit、prctl、sethostname、procfs 相邻候选。
  - 注意 musl/glibc 差异、wait status、子进程状态、RLIMIT_FSIZE、proc synthetic fs 可见语义。
- Worker 5: mmap/mprotect/munmap/signal/time + verification guardrail lane
  - 优先 mmap04、mmap05、mmap06、mprotect01、mprotect02、munmap01，同时维护 no-fake-pass/no-timeout-as-pass 审计和 marker-prefix 检查。
  - 注意 page permission、SIGSEGV/SIGBUS、MAP_FIXED/MAP_PRIVATE/MAP_SHARED、unmap 边界；这些是隐藏测试防守高价值区，但必须真实 clean 才能 promotion。

执行节奏：
1. baseline refresh：
   - 统计当前 `LTP_STABLE_CASES` 数量和重复项。
   - 跑最小 smoke，确认 stable300 没有明显回退；如果时间允许，先 RV stable aggregate，再 LA stable aggregate。
2. candidate discovery：
   - 建立 300->380 候选池与价值评分；候选不足时说明原因。
   - 先 targeted batches，不要直接把大批未知 case 加入 stable。
3. promotion gate：
   - stable315：新增约 15 个 clean case 后更新 `LTP_STABLE_CASES`，跑 RV+LA targeted/aggregate gate，并用 `scripts/ltp_summary.py` 解析。
   - stable330：同上。
   - stable350：同上并进入 final full gate。
   - 如果任一 arch/libc 出现 FAIL、timeout、ENOSYS、panic/trap、新 TFAIL/TBROK/TCONF，停止 promotion，先修复或回退该候选。
4. final stable350 gate：
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv`
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la`
   - `python3 -B scripts/ltp_summary.py <rv-log>` 和 `<la-log>`
   - marker-prefix check：所有 `PASS LTP CASE`/`FAIL LTP CASE` marker 必须从行首开始。
   - `cargo fmt --all -- --check`
   - 相关构建：至少 `make A=examples/shell ARCH=riscv64`；涉及 remote submission 时跑 `make all`，必要时再跑离线 `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`。

产物要求：
- 使用本地日期目录：`docs/ltp-score-improvement-2026-05-25-phase-a/`。
- 创建并持续更新：
  - `plan-stable300-to-350.md`
  - `candidate-matrix.md`
  - `stable315-promotion-gate-report.md`
  - `stable330-promotion-gate-report.md`
  - `stable350-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-regression-check.md`
  - `next-session-prompt-stable350-followup.md`（如果 stable350 未达成或仍有高价值 blocker）
- raw logs 放在 `docs/ltp-score-improvement-2026-05-25-phase-a/raw/`，默认不要提交大 raw logs；提交摘要、报告、case lists、quality gate JSON。
- `.omx/ultragoal/goals.json` 和 `ledger.jsonl` 是本地 audit trail，leader 维护；是否提交遵循仓库 ignore/用户要求。

交付条件：
- live `LTP_STABLE_CASES` 正好 350 个 unique case。
- RV final stable gate：PASS LTP CASE 700，FAIL 0；ltp-musl 350/0；ltp-glibc 350/0。
- LA final stable gate：PASS LTP CASE 700，FAIL 0；ltp-musl 350/0；ltp-glibc 350/0。
- internal TFAIL=0、TBROK=0；除已明确披露且可接受的 known TCONF 外不得新增 TCONF。若 `read02` 仍在 stable，则继续披露其 pass_with_tconf，不把它说成 clean。
- timeout/ENOSYS/panic/trap 均为 0。
- remote marker prefix 检查通过：0 bad marker lines。
- 已完成 code review + ai-slop-cleaner audit。
- 已按 AGENTS.md 自动 commit agent-owned tracked 变更，并在最终回复列出 commit SHA、验证命令、未能运行的检查、用户可见行为变化、ABI/POSIX 变化。

如果在 stable350 前遇到硬阻塞：
- 不要伪造或隐藏失败。
- 保存当前最高可信 stableN 的证据和 blocker 报告。
- 明确列出失败 case、arch/libc、内部 TFAIL/TBROK/TCONF、timeout/ENOSYS/panic/trap、相关日志路径、已尝试修复和下一步建议。
```
