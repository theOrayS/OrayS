# Next-session prompt: stable383 -> stable413/423 easy-first with Ultragoal + Team

Created: 2026-05-26
Target repo: `/root/oskernel2026-orays`
Current HEAD when drafted: `2f16de55 Digest evaluator logs for developers` or later

Use this prompt as the first message in the next Codex/OMX session.

```text
我现在要启动下一轮 LTP stable 提分任务：从当前 stable383 开始，先不要继续硬冲 stable450；本轮目标是优先找“容易跑过”的 30~40 个新增 LTP case。请使用 Ultragoal + Team 模式推进，按仓库 AGENTS.md 执行，中文汇报。

工作目录：/root/oskernel2026-orays

当前已知基线（必须 live 复核，不要只依赖本提示词）：
- 本提示词创建时 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 为 383 total / 383 unique / 0 duplicates；下一会话必须重新计算。
- 当前 stop-state 来自 `docs/ltp-score-improvement-2026-05-25-phase-c/next-session-prompt-stable450-followup.md` 和 phase-c 报告。
- stable383 是用户停止后的最高保留 stop-state，不是 stable450 交付态。
- stable383 后已保留新增 case：`clock_settime01`, `clock_settime02`, `clone03`, `confstr01`, `chmod05`, `fchmod05`, `lseek02`, `pipe08`。
- stable383 关键证据：
  - `docs/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md`
  - `docs/ltp-score-improvement-2026-05-25-phase-c/stable400-promotion-gate-report.md`
  - `docs/ltp-score-improvement-2026-05-25-phase-c/candidate-matrix.md`
  - `docs/ltp-score-improvement-2026-05-25-phase-c/raw/stable383-la-gate-001-summary.txt`
  - `docs/ltp-score-improvement-2026-05-25-phase-c/raw/stable384-rv-gate-001-summary.txt`（只是 RV clean superset 支撑；如果需要 exact baseline 证明，要重新跑 exact stable383 RV gate）
- known transparent TCONF 仍只有 `read02`；不能把它说成 clean，也不能把新增 TCONF 算作 promotion-clean。
- phase-c 已修过远程日志噪声：原 `axfs::fops:297 [AxError::NotADirectory]` 高频噪声应保持为 0；残余 `axfs_ramfs::file:69` NotADirectory 噪声可披露但不能影响 marker prefix。

本轮目标：
- 主目标：新增 30 个 easy/low-risk clean case，从 stable383 到 stable413。
- Stretch：如果 clean subset 足够，新增 40 个 case，到 stable423。
- 中间阶段建议：stable393 -> stable403 -> stable413 -> optional stable423。
- 本轮原则是“低成本扩面”，不是继续死磕已知深水 blocker。遇到明显需要大重构、设备/挂载工具、record lock、完整 VM/signal 模型的 case，记录 blocker 后换候选。
- 只有 RV+LA × musl+glibc 全 clean 的 case 才能 promotion；不能为了数量牺牲真实性。

启动要求：
1. 先读取 `AGENTS.md` 和本提示词。
2. 磁盘 preflight：
   - `df -h / /root`
   - `du -sh /root/.codex`
   - 如果 `/` 接近满，先清理低价值临时日志/cache；不要删 memories/skills/prompts/agents/凭据/活跃 `.omx` 状态。
3. `git status --short`，确认只处理 agent-owned 变更；不要回滚用户文件或未跟踪远程输出日志。
4. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算当前 stable 数量和重复项，不能依赖记忆。
5. 复核 phase-c stop-state 报告、candidate matrix、stable383/384 相关 summary；把 stable383 当作回归保护。
6. 如果 `.omx/ultragoal` 仍有旧 active/blocked 目标，先读取 `omx ultragoal status` 和 `.omx/ultragoal/ledger.jsonl`，不要盲目覆盖；必要时根据当前 stop-state 创建本轮新 durable plan。
7. 启动 Team 模式提高吞吐；如果 tmux pane/资源受限，优先 4~5 个 worker。Leader 负责 `.omx/ultragoal`、最终 `LTP_STABLE_CASES` 修改、promotion 决策、最终 gates 和 commit；worker 只做 discovery/修复/验证/报告切片。

Ultragoal 具体用法：
1. 创建 plan：
   - `omx ultragoal create-goals --brief-file docs/ltp-score-improvement-2026-05-26-phase-a/next-session-prompt-stable383-to-423-easy30-40.md`
   - 检查 `.omx/ultragoal/goals.json` 和 `.omx/ultragoal/ledger.jsonl`。
2. 执行每个 story：
   - `omx ultragoal status`
   - `omx ultragoal complete-goals`
   - 按输出 handoff 使用 Codex goal 工具：先 `get_goal`；如无 active goal，则 `create_goal`；中间 story 完成时不要 `update_goal complete`。
3. 每个阶段完成后，leader 用新鲜 `get_goal` snapshot checkpoint：
   - `omx ultragoal checkpoint --goal-id <id> --status complete --evidence "<证据路径和摘要>" --codex-goal-json <fresh-get-goal-json-or-path>`
4. 最终 story 必须完成 final gate、marker-prefix/noise check、code review、ai-slop-cleaner audit；只有 final clean 后才 `update_goal({status: "complete"})`，再用 complete snapshot checkpoint。
5. Team workers 不拥有 `.omx/ultragoal`，不创建 worker ledger，不 checkpoint Ultragoal，不最终修改 `LTP_STABLE_CASES`。

Team 具体用法：
1. 启动前做 preflight：
   - `tmux -V`
   - `test -n "$TMUX"`
   - `command -v omx`
   - `tmux list-panes -F '#{pane_id}\t#{pane_start_command}' | rg 'hud --watch' || true`
2. 推荐启动：
   - `omx team 5:executor "LTP stable383 to stable413 easy-first promotion with Ultragoal leader-owned gates"`
   - 如果 pane/资源不足，降级为 `omx team 4:executor "LTP stable383 to stable413 easy-first promotion with Ultragoal leader-owned gates"`。
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

Easy-first 候选策略（候选不等于 promotion）：
1. 先用实际 sdcard inventory 过滤候选，避免再跑 `lseek03`-`lseek10` 这类缺 binary 的 case：
   - `docs/ltp-score-improvement-2026-05-25-phase-c/raw/sdcard-rv-common-not-stable-ltp-bins.txt`
   - 必要时重新生成 RV/LA、musl/glibc inventory。
2. 先跑 RV 小批量 scout（每批 15~25 个，timeout 45~60s），只把 RV musl+glibc clean 的 case 送 LA；LA clean 后再进入 promotion 候选。
3. 先找低风险、已存在 binary、运行短、无需外部工具/挂载设备/record lock/完整 signal-VM 模型的 case。
4. 对明显 blocker 立即跳过并记录，不要在本轮长时间硬修。目标是先捞 30~40 个 easy clean case。
5. 每次 promotion 只追加 8~12 个 clean case，跑串行 RV+LA aggregate gate 后再继续下一批。

优先 scout 池（按 low-risk/easy-first 排序；仍需 fresh proof）：
- Lightweight syscall / process / libc surface：
  - `poll02`, `times03`, `gethostname02`, `gethostid01`, `getpgid01`, `getcpu01`
  - `fork13`, `fork14`, `clone06`, `clone07`, `clone08`, `clone09`
  - `kill05`, `kill10`（不要把 `kill02` 的 targeted clean 当 promotion；它有 LA aggregate blocker）
- Metadata / directory / statfs-style：
  - `fstat02`, `fstat02_64`, `fstatfs01`, `fstatfs01_64`
  - `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01`
  - `getcwd03`, `getcwd04`, `getdents01`, `getdents02`
- Simple FD / offset / IO：
  - `pread02`, `pread02_64`, `pwrite02`, `pwrite02_64`, `pwrite04`, `pwrite04_64`
  - `preadv01`, `preadv02`, `preadv03`, `pwritev01`, `pwritev02`, `pwritev03`（遇到 iovec/TCONF 就降级为 blocker）
  - `sendfile02`, `sendfile03`, `sendfile04`, `sendfile05` 及 `_64` variants（只在 RV clean 后送 LA）
- VFS create/open/remove 小步 scout：
  - `open06`, `open07`, `open10`, `open11`, `open12`, `open14`
  - `creat04`, `creat06`, `creat07`, `creat08`, `creat09`
  - `mkdir03`, `mkdir04`, `mkdir09`, `rmdir02`, `rmdir03`, `unlink07`, `unlink08`, `unlink09`
- fs-suite substitutes（只挑轻量、短 timeout 的 clean subset）：
  - `fs_perms`, `rwtest`, `writetest`, `iogen`, `fs_inod`, `openfile`, `inode02`, `ftest06`
  - 若出现 timeout、缺工具、missing path、ENOSYS，立刻记录并跳过。

本轮不要优先投入的已知 blocker / 深水区：
- `readlinkat02`：LA-musl call-boundary split，不能用 syscall-body 特判修。
- `kill02`：targeted clean 但 LA aggregate `TBROK/setup timeout`，没有 fresh aggregate-stable 前不 promotion。
- `access04`, `chmod06`, `fchmod06`, `chmod07`, `fchmod02`：已有 RV TBROK/setup blocker。
- `waitid07`, `waitid08`, `waitid10`, `munmap01`, `mmap04`, `mmap05`, `mprotect01`, `mprotect02`, `pipe07`, `pipe15`：已有真实 TFAIL/TBROK/wrapper failure。
- `pipe02`：曾 panic/trap；不要放进 broad batch。
- `lseek03`-`lseek10`：当前 sdcard 缺 binary；`lseek11` 需要 SEEK_DATA/SEEK_HOLE。
- `statx04`-`statx12`：设备/工具/config blocker；不在 easy-first 主线。
- broad `fcntl*` record-lock、`fs_bind*`, `test_robind*`, `ksm*`, `fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*`, namespace/io_uring/perf 等低 ROI 或高重构风险族。

推荐 Team 分工：
- Worker 1: Inventory + easy candidate matrix
  - 从 live stable list、sdcard inventory、phase-c negative evidence中过滤“已 stable / 缺 binary / known blocker / deep subsystem”。
  - 输出 `candidate-matrix-easy30-40.md`：每个候选标注 subsystem、是否 present in both libc trees、是否已有 negative evidence、RV/LA/musl/glibc 状态。
- Worker 2: Lightweight syscall/process lane
  - 负责 `poll/times/get*/fork/clone/kill` 小批 scout；只做局部低风险修复。
- Worker 3: Metadata/statfs/getdents lane
  - 负责 `fstat/statfs/statvfs/getcwd/getdents` 小批 scout；注意 ABI struct、errno、目录项 copy-out。
- Worker 4: FD/IO/sendfile/pread-pwrite lane
  - 负责 `pread/pwrite/preadv/pwritev/sendfile`；注意 offset、O_APPEND、user iovec、EFAULT、EOF、partial write。
- Worker 5: VFS small-create/remove + guardrail lane
  - 负责 `open/creat/mkdir/rmdir/unlink` 和轻量 fs-suite scout；同时维护 no-fake-pass、marker-prefix、timeout/ENOSYS/panic guardrail。

执行节奏：
1. Baseline refresh：统计 current stable count/duplicates，复核 stable383 stop-state；必要时跑 exact RV stable383 aggregate 补足 baseline。
2. Discovery pass：用 inventory + old blocker reports 生成 60~100 个候选的 easy-first matrix。
3. RV scout：每批 15~25 个候选；解析 summary；只保留 RV musl+glibc clean 且无 internal TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap 的 case。
4. LA confirm：只跑 RV clean subset；LA clean 后进入 promotion candidate。
5. Promotion gates：
   - stable393：新增约 10 个 clean case，更新 `LTP_STABLE_CASES`，跑 RV+LA aggregate gate。
   - stable403：再新增约 10 个 clean case，跑 RV+LA aggregate gate。
   - stable413：再新增约 10 个 clean case，跑 RV+LA final gate。
   - optional stable423：如果 clean subset 足够，再新增约 10 个，跑完整 final gate。
6. 如果任一 arch/libc 出现 FAIL、timeout、ENOSYS、panic/trap、新 TFAIL/TBROK/TCONF，停止该 candidate 的 promotion，先修复或回退，不能硬留在 stable list。

Final gate 命令建议：
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv`
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la`
- `python3 -B scripts/ltp_summary.py <rv-log>` 和 `<la-log>`
- marker-prefix check：所有 `PASS LTP CASE`/`FAIL LTP CASE` marker 必须从行首开始。
- noise check：`axfs::fops:297 [AxError::NotADirectory]` 不得回归为高频噪声；残余 expected errno 噪声要披露。
- `cargo fmt --all -- --check`
- `git diff --check`
- 相关构建：至少 `make A=examples/shell ARCH=riscv64`；若改动影响 evaluator-kernel/remote submission，再跑 `make all` 和必要的 offline `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`。

产物要求：
- 使用本地日期目录：`docs/ltp-score-improvement-2026-05-26-phase-a/`。
- 创建并持续更新：
  - `plan-stable383-to-413-easy30-40.md`
  - `candidate-matrix-easy30-40.md`
  - `stable393-promotion-gate-report.md`
  - `stable403-promotion-gate-report.md`
  - `stable413-delivery-report.md`
  - `stable423-stretch-report.md`（如果推进 stretch）
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-and-log-noise-regression-check.md`
  - `next-session-prompt-stable413-followup.md` 或 `next-session-prompt-stable423-followup.md`
- raw logs 放在 `docs/ltp-score-improvement-2026-05-26-phase-a/raw/`，默认不要提交大 raw logs；提交摘要、报告、case lists、quality gate JSON。
- `.omx/ultragoal/goals.json` 和 `ledger.jsonl` 是 leader 维护的本地 audit trail；是否提交遵循仓库 ignore/用户要求。

交付条件：
- stable413 主目标：live `LTP_STABLE_CASES` 正好 413 个 unique case。
- stable413 RV final stable gate：PASS LTP CASE 826，FAIL 0；ltp-musl 413/0；ltp-glibc 413/0。
- stable413 LA final stable gate：PASS LTP CASE 826，FAIL 0；ltp-musl 413/0；ltp-glibc 413/0。
- 如果达成 stable423 stretch：RV+LA 均为 PASS LTP CASE 846，FAIL 0；ltp-musl 423/0；ltp-glibc 423/0。
- internal TFAIL=0、TBROK=0；除已明确披露且可接受的 known `read02` TCONF 外不得新增 TCONF。
- timeout/ENOSYS/panic/trap 均为 0。
- marker-prefix bad lines 0；日志噪声 guardrail 通过。
- 已完成 code review + ai-slop-cleaner audit。
- 已按 AGENTS.md 自动 commit agent-owned tracked 变更，并在最终回复列出 commit SHA、验证命令、未能运行的检查、用户可见行为变化、ABI/POSIX 变化。

如果在 stable413 前遇到硬阻塞：
- 不要伪造或隐藏失败。
- 保存当前最高可信 stableN 的证据和 blocker 报告。
- 明确列出失败 case、arch/libc、内部 TFAIL/TBROK/TCONF、timeout/ENOSYS/panic/trap、相关日志路径、已尝试修复和下一步建议。
```
