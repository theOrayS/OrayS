# Next session prompt: stable350 -> stable375 with Ultragoal + Team

Created: 2026-05-25
Target repo: `/root/oskernel2026-orays`
Suggested starting commit from this handoff: `b897f7e7 Keep agent guidance focused on evaluator ROI` or later

Use this prompt as the first message in the next Codex/OMX session.

```text
我现在要启动下一轮 LTP stable 提分任务：目标是从当前 stable350 提升到 stable375，stretch goal 是 stable380。请使用 Ultragoal + Team 模式推进，按仓库 AGENTS.md 执行，中文汇报。

工作目录：/root/oskernel2026-orays

当前已知基线（必须 live 复核，不要只依赖本提示词）：
- 当前 handoff commit：b897f7e7 Keep agent guidance focused on evaluator ROI
- live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 在提示词创建时为 350 total / 350 unique / 0 duplicates；下一会话必须重新计算。
- stable350 已交付，证据在 `docs/ltp-score-improvement-2026-05-25-phase-a/`：
  - `stable350-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `raw/stable350-rv-final-002-summary.txt`
  - `raw/stable350-la-final-002-summary.txt`
  - `raw/stable350-rv-final-002-marker-prefix.txt`
  - `raw/stable350-la-final-002-marker-prefix.txt`
- stable350 final gate at handoff:
  - RV: PASS LTP CASE 700, FAIL 0; ltp-musl 350/0; ltp-glibc 350/0
  - LA: PASS LTP CASE 700, FAIL 0; ltp-musl 350/0; ltp-glibc 350/0
  - known `read02` TCONF only; timeout/ENOSYS/panic/trap 0; bad marker lines 0
- AGENTS.md 已精简并加入当前 LTP 策略：
  - 源码基线优先 `oscomp/testsuits-for-oskernel@pre-2025` 的 `ltp-full-20240524`
  - 不按 raw runtest 数量盲目冲；优先 `syscalls/mm/fs` 中高 ROI 子测例
  - 大但低 ROI 的 `fs_bind*`, `test_robind*`, `ksm*`, `fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*` 不应挤占短期提分主线
- 用户提供的远程评测机输出文件可能仍在仓库根目录：`Riscv输出.txt`、`LoongArch输出.txt`；它们是用户证据，默认不要提交。

本轮目标：
- 主目标：新增约 25 个高价值 LTP case，达到 stable375。
- Stretch：如果 clean subset 足够，新增到约 30 个 case，达到 stable380。
- 不为数量牺牲真实性。只有 RV+LA × musl+glibc 全 clean 的 case 才能 promotion。
- 如果目标前遇到硬 blocker，保存最高可信 stableN gate 和 blocker 报告，不伪造或隐藏失败。

启动要求：
1. 先读取 `AGENTS.md` 和本提示词。
2. 磁盘 preflight：
   - `df -h / /root`
   - `du -sh /root/.codex`
   - 如果 `/` 接近满，先清理低价值临时日志/cache；不要删 memories/skills/prompts/agents/凭据/活跃 `.omx` 状态。
3. `git status --short`，确认只处理 agent-owned 变更；不要回滚用户文件或未跟踪远程输出日志。
4. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算当前 stable 数量和重复项，不能依赖记忆。
5. 复核 stable350 final summaries、quality gate、marker-prefix evidence；stable350 必须作为回归保护。
6. 创建/恢复 Ultragoal durable plan：目标 stable375，stretch stable380；建议阶段为 stable360 -> stable368 -> stable375 -> optional stable380。
7. 启动 Team 模式提高吞吐；如果 tmux pane/资源受限，优先 5 个 worker，失败再降到 4 个 worker。Leader 负责 `.omx/ultragoal`、`LTP_STABLE_CASES` 最终修改、promotion 决策和最终验证；worker 只做被分配的 discovery/修复/验证/报告切片。

Ultragoal 具体用法：
1. 创建 plan：
   - `omx ultragoal create-goals --brief-file docs/ltp-score-improvement-2026-05-25-phase-b/next-session-prompt-stable350-to-375.md`
   - 检查 `.omx/ultragoal/goals.json` 和 `.omx/ultragoal/ledger.jsonl`。
2. 执行每个 story：
   - `omx ultragoal status`
   - `omx ultragoal complete-goals`
   - 按输出 handoff 使用 Codex goal 工具：先 `get_goal`；如无 active goal，则 `create_goal`；中间 story 完成时不要 `update_goal complete`。
3. 每个阶段完成后，leader 用新鲜 `get_goal` snapshot checkpoint：
   - `omx ultragoal checkpoint --goal-id <id> --status complete --evidence "<证据路径和摘要>" --codex-goal-json <fresh-get-goal-json-or-path>`
4. 最终 story 必须先完成 final gate、code-review、ai-slop-cleaner、marker-prefix check；只有 final clean 后才 `update_goal({status: "complete"})`，再用 complete snapshot checkpoint。
5. Team workers 不拥有 `.omx/ultragoal`，不创建 worker ledger，不 checkpoint Ultragoal，不最终修改 `LTP_STABLE_CASES`。

Team 具体用法：
1. 启动前做 preflight：
   - `tmux -V`
   - `test -n "$TMUX"`
   - `command -v omx`
   - `tmux list-panes -F '#{pane_id}\t#{pane_start_command}' | rg 'hud --watch' || true`
2. 推荐启动：
   - `omx team 5:executor "LTP stable350 to stable375 high-value promotion with Ultragoal leader-owned gates"`
   - 如果 pane/资源不足，降级为 `omx team 4:executor "LTP stable350 to stable375 high-value promotion with Ultragoal leader-owned gates"`。
3. 启动后确认 team line、tmux target、worker panes、ACK mailbox。
4. 监控优先使用 runtime/state：`omx team status <team>`、`omx team resume <team>`、mailbox、task JSON。
5. 只有 pending=0、in_progress=0、failed=0（或失败路径已明确保存 blocker）后，才运行 `omx team shutdown <team>`。
6. 不要让多个 worker 并发争用默认 QEMU/sdcard/qcow2。若没有隔离镜像，promotion gate 和 final gate 由 leader 串行跑；worker 并发输出只能作为 discovery，不能作为 promotion 证据。

强约束：
- 不伪造 PASS。
- 不 hardcode case name 通过。
- 不修改 LTP 测试源码来通过。
- 不把真实 TFAIL/TBROK/timeout/panic/trap/ENOSYS 转成 SKIP/TCONF/PASS。
- timeout 不能计为 PASS。
- wrapper 成功不等于可 promotion；所有证据必须用 `python3 -B scripts/ltp_summary.py` 或等价 case matrix 解析。
- `read02` 的 `pass_with_tconf` 必须保持透明；clean 的定义是没有新增 internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
- 远程评分修复不能回退：LTP marker 行必须保持从行首开始，不能被 ANSI reset/color prefix 污染。任何日志输出改动后都要检查 marker prefix。
- 不提交 root-level kernel、sdcard/disk image、大 raw log、用户给的 `Riscv输出.txt`/`LoongArch输出.txt`，除非用户明确要求。

本轮高价值候选池：

Primary 25（目标 stable375；必须逐个用 fresh evidence 证明，不可直接加入 stable）：
- VFS / permission / metadata:
  - `access02`, `access04`
  - `chmod05`, `chmod06`, `chmod07`
  - `fchmod02`, `fchmod05`, `fchmod06`, `fchmodat02`
  - `statx01`
  - `readlinkat02`
  - `rename01`, `rename03`, `rename04`
  - `openat02`
- FD / pipe / iovec:
  - `writev03`
  - `pipe2_02`
- process / wait / signal:
  - `waitid07`, `waitid08`, `waitid10`
  - `kill02`
- mmap / VM boundary:
  - `mmap04`, `mmap05`, `mmap06`
  - `munmap01`

Stretch 5（用于 stable380，只有 primary 中 blocker 较少且这些 clean 时再 promotion）：
- `mprotect01`, `mprotect02`
- `openat03`
- `rename05`
- `statx03`

如果 primary 中部分 case 短期不 clean，替补池按 ROI 顺序选择：
- `fs_perms01`-`fs_perms06`
- `ftest01`-`ftest04`
- `rwtest01`, `rwtest02`
- `stream01`, `stream02`
- `mmap10`, `mmap10_1`, `vma01`, `vma02`

候选策略：
1. 本轮不是随机凑数；优先真实修复能带动一簇相邻 case 的 syscall/ABI/VFS/FD/VM 语义。
2. 每批 targeted 5-10 个候选，先建立 matrix，再 promotion；不要直接把大批未知 case 加入 stable。
3. 可以 targeted 验证 50-80 个候选来找 clean subset，但只有 RV+LA × musl+glibc 全 clean 的 case 才能加入 stable。
4. 若一个高价值 blocker 短期修不 clean，记录 blocker 并转向其他 clean subset，不要让单个 case 阻塞整轮。
5. 不要优先追 `fs_bind*`, `test_robind*`, `ksm*`, `fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*`。

推荐 Team 分工：
- Worker 1: Discovery + promotion matrix
  - 从 `LTP_STABLE_CASES`、phase-a evidence、sdcard/runtest inventory、`scripts/ltp_summary.py --promotion-candidates` 建立 stable350->stable380 候选池。
  - 输出 candidate matrix：按 subsystem、价值评分、RV/LA、musl/glibc、clean/TCONF/TFAIL/TBROK/timeout/ENOSYS/panic 分类。
- Worker 2: permissions/VFS/metadata lane
  - 优先 `access02`, `access04`, `chmod05`, `chmod06`, `chmod07`, `fchmod*`, `statx01`, `readlinkat02`, `rename*`, `openat02`。
  - 注意 errno、权限、uid/gid/fsgid、O_PATH、AT_EMPTY_PATH、setgid/sticky、目录语义。
- Worker 3: fd/pipe/iovec lane
  - 优先 `writev03`, `pipe2_02`，同时排查相邻 `readv/writev/preadv/pwritev/pipe/pipe2/dup/fcntl` 候选。
  - 守住 shared offset、O_APPEND、pipe capacity/FIONREAD、blocking/yield、SIGPIPE 语义。
- Worker 4: process/wait/signal lane
  - 优先 `waitid07`, `waitid08`, `waitid10`, `kill02`。
  - 注意 musl/glibc 差异、wait status、子进程状态、signal delivery、permission/ESRCH/EINVAL 边界。
- Worker 5: mmap/munmap/mprotect + verification guardrail lane
  - 优先 `mmap04`, `mmap05`, `mmap06`, `munmap01`，stretch `mprotect01`, `mprotect02`。
  - 同时维护 no-fake-pass/no-timeout-as-pass 审计和 marker-prefix 检查。
  - 注意 page permission、SIGSEGV/SIGBUS、MAP_FIXED/MAP_PRIVATE/MAP_SHARED、unmap 边界。

执行节奏：
1. Baseline refresh:
   - 统计当前 `LTP_STABLE_CASES` 数量和重复项。
   - 复核 stable350 final gate 和 marker-prefix evidence。
   - 如果时间允许，先跑 RV stable aggregate，再跑 LA stable aggregate；至少要确认没有明显 baseline 回退。
2. Candidate discovery:
   - 对 primary 25 + stretch 5 建立 candidate matrix。
   - 先 targeted batches，不要直接把未知 case 加入 stable。
3. Promotion gates:
   - stable360：新增约 10 个 clean case 后更新 `LTP_STABLE_CASES`，跑 RV+LA targeted/aggregate gate，并用 `scripts/ltp_summary.py` 解析。
   - stable368：同上。
   - stable375：同上并进入 final gate。
   - optional stable380：只有 stretch clean 且资源允许时推进。
   - 如果任一 arch/libc 出现 FAIL、timeout、ENOSYS、panic/trap、新 TFAIL/TBROK/TCONF，停止 promotion，先修复或回退该候选。
4. Final gate:
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv`
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la`
   - `python3 -B scripts/ltp_summary.py <rv-log>` 和 `<la-log>`
   - marker-prefix check：所有 `PASS LTP CASE`/`FAIL LTP CASE` marker 必须从行首开始。
   - `cargo fmt --all -- --check`
   - 相关构建：至少 `make A=examples/shell ARCH=riscv64`；涉及 remote submission 时跑 `make all`，必要时再跑离线 `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`。

产物要求：
- 使用本地日期目录：`docs/ltp-score-improvement-2026-05-25-phase-b/`。
- 创建并持续更新：
  - `plan-stable350-to-375.md`
  - `candidate-matrix.md`
  - `stable360-promotion-gate-report.md`
  - `stable368-promotion-gate-report.md`
  - `stable375-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-regression-check.md`
  - `next-session-prompt-stable375-followup.md`（如果 stable375 未达成或仍有高价值 blocker）
- raw logs 放在 `docs/ltp-score-improvement-2026-05-25-phase-b/raw/`，默认不要提交大 raw logs；提交摘要、报告、case lists、quality gate JSON。
- `.omx/ultragoal/goals.json` 和 `ledger.jsonl` 是本地 audit trail，leader 维护；是否提交遵循仓库 ignore/用户要求。

交付条件：
- stable375 主目标：live `LTP_STABLE_CASES` 正好 375 个 unique case。
- stable375 RV final stable gate：PASS LTP CASE 750，FAIL 0；ltp-musl 375/0；ltp-glibc 375/0。
- stable375 LA final stable gate：PASS LTP CASE 750，FAIL 0；ltp-musl 375/0；ltp-glibc 375/0。
- 如果达成 stable380 stretch：PASS LTP CASE 760，FAIL 0；ltp-musl 380/0；ltp-glibc 380/0；RV+LA 均成立。
- internal TFAIL=0、TBROK=0；除已明确披露且可接受的 known TCONF 外不得新增 TCONF。若 `read02` 仍在 stable，则继续披露其 pass_with_tconf，不把它说成 clean。
- timeout/ENOSYS/panic/trap 均为 0。
- remote marker prefix 检查通过：0 bad marker lines。
- 已完成 code review + ai-slop-cleaner audit。
- 已按 AGENTS.md 自动 commit agent-owned tracked 变更，并在最终回复列出 commit SHA、验证命令、未能运行的检查、用户可见行为变化、ABI/POSIX 变化。

如果在 stable375 前遇到硬阻塞：
- 不要伪造或隐藏失败。
- 保存当前最高可信 stableN 的证据和 blocker 报告。
- 明确列出失败 case、arch/libc、内部 TFAIL/TBROK/TCONF、timeout/ENOSYS/panic/trap、相关日志路径、已尝试修复和下一步建议。
```
