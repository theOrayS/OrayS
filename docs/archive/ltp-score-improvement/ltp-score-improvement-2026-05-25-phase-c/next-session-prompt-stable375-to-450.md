# Next session prompt: stable375 -> stable450 with log-noise repair + Ultragoal + Team

Created: 2026-05-25
Target repo: `/root/oskernel2026-orays`
Suggested starting commit from this handoff: `9afc6a90 Preserve honest stable375 promotion evidence` or later

Use this prompt as the first message in the next Codex/OMX session.

```text
我现在要启动下一轮 LTP stable 提分任务：先修掉远程评测输出里 `axfs::fops:297 [AxError::NotADirectory]` 的高频日志噪声，然后继续从 stable375 冲到 stable450。请使用 Ultragoal + Team 模式推进，按仓库 AGENTS.md 执行，中文汇报。

工作目录：/root/oskernel2026-orays

当前已知基线（必须 live 复核，不要只依赖本提示词）：
- 当前 handoff commit：9afc6a90 Preserve honest stable375 promotion evidence
- live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 在提示词创建时为 375 total / 375 unique / 0 duplicates；下一会话必须重新计算。
- stable375 已交付，证据在 `docs/ltp-score-improvement-2026-05-25-phase-b/`：
  - `stable375-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `raw/stable375-rv-final-002-summary.txt`
  - `raw/stable375-la-final-003-summary.txt`
  - `raw/stable375-final-marker-prefix.txt`
- stable375 local final gate at handoff:
  - RV: PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0
  - LA: PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0
  - known `read02` TCONF only; timeout/ENOSYS/panic/trap 0; bad marker lines 0
- 最新远程诊断输出文件可能仍在仓库根目录：`Riscv输出.txt`、`LoongArch输出.txt`；它们是用户证据，默认不要提交。
- 最新 `for_remote_judge` 诊断思路已经证明：只跑 `ltp-glibc` 可以避开 1MB 输出截断，并且 RV/LA 的 `ltp-glibc stable375` 均为 375/0/0；问题不是 glibc 后半段隐藏失败，而是全量远程输出被高频日志和双 libc 输出挤爆。
- 远程输出里高频噪声示例：`axfs::fops:297 [AxError::NotADirectory]`。源码位置为 `kernel/fs/axfs/src/fops.rs::_open_dir_at()`，当前 `ax_err!(NotADirectory)` 会经 `axerrno` 宏打印 warning。该 errno 本身通常是 LTP 负向路径的一部分，不能改变可见 errno 语义，只能降噪或改用不打印的等价返回路径。

本轮目标：
1. 第一目标：修掉 `axfs::fops:297 [AxError::NotADirectory]` 高频日志噪声，显著降低远程输出体积，不改变 Linux/POSIX 可见 errno 语义。
2. 主目标：新增约 75 个高价值 LTP case，达到 stable450。
3. Stretch：如果 clean subset 足够，推进到 stable460 或 stable475，但不能牺牲真实性。
4. 不为数量牺牲真实性。只有 RV+LA × musl+glibc 全 clean 的 case 才能 promotion。
5. 如果 stable450 前遇到硬 blocker，保存最高可信 stableN gate 和 blocker 报告，不伪造或隐藏失败。

启动要求：
1. 先读取 `AGENTS.md` 和本提示词。
2. 磁盘 preflight：
   - `df -h / /root`
   - `du -sh /root/.codex`
   - 如果 `/` 接近满，先清理低价值临时日志/cache；不要删 memories/skills/prompts/agents/凭据/活跃 `.omx` 状态。
3. `git status --short`，确认只处理 agent-owned 变更；不要回滚用户文件或未跟踪远程输出日志。
4. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算当前 stable 数量和重复项，不能依赖记忆。
5. 复核 stable375 final summaries、quality gate、marker-prefix evidence；stable375 必须作为回归保护。
6. 解析最新 `Riscv输出.txt`、`LoongArch输出.txt`：统计 AxError 噪声、marker 数量、suite end、TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap；把远程输出截断风险作为本轮 guardrail。
7. 创建/恢复 Ultragoal durable plan：目标 stable450，stretch stable460/475；建议阶段为 log-noise-fix -> stable400 -> stable425 -> stable450 -> optional stable460/475。
8. 启动 Team 模式提高吞吐；如果 tmux pane/资源受限，优先 5 个 worker，失败再降到 4 个 worker。Leader 负责 `.omx/ultragoal`、`LTP_STABLE_CASES` 最终修改、promotion 决策和最终验证；worker 只做被分配的 discovery/修复/验证/报告切片。

Ultragoal 具体用法：
1. 创建 plan：
   - `omx ultragoal create-goals --brief-file docs/ltp-score-improvement-2026-05-25-phase-c/next-session-prompt-stable375-to-450.md`
   - 检查 `.omx/ultragoal/goals.json` 和 `.omx/ultragoal/ledger.jsonl`。
2. 执行每个 story：
   - `omx ultragoal status`
   - `omx ultragoal complete-goals`
   - 按输出 handoff 使用 Codex goal 工具：先 `get_goal`；如无 active goal，则 `create_goal`；中间 story 完成时不要 `update_goal complete`。
3. 每个阶段完成后，leader 用新鲜 `get_goal` snapshot checkpoint：
   - `omx ultragoal checkpoint --goal-id <id> --status complete --evidence "<证据路径和摘要>" --codex-goal-json <fresh-get-goal-json-or-path>`
4. 最终 story 必须先完成 final gate、code-review、ai-slop-cleaner、marker-prefix check、远程日志噪声回归检查；只有 final clean 后才 `update_goal({status: "complete"})`，再用 complete snapshot checkpoint。
5. Team workers 不拥有 `.omx/ultragoal`，不创建 worker ledger，不 checkpoint Ultragoal，不最终修改 `LTP_STABLE_CASES`。

Team 具体用法：
1. 启动前做 preflight：
   - `tmux -V`
   - `test -n "$TMUX"`
   - `command -v omx`
   - `tmux list-panes -F '#{pane_id}\t#{pane_start_command}' | rg 'hud --watch' || true`
2. 推荐启动：
   - `omx team 5:executor "LTP stable375 to stable450 log-noise repair and high-value promotion with Ultragoal leader-owned gates"`
   - 如果 pane/资源不足，降级为 `omx team 4:executor "LTP stable375 to stable450 log-noise repair and high-value promotion with Ultragoal leader-owned gates"`。
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
- 修日志噪声不能改变 syscall/POSIX 可见行为：同样的路径错误必须继续返回正确 errno，例如 ENOTDIR/EISDIR/EEXIST；不能吞掉真实错误，不能把失败转成功。
- 远程评分修复不能回退：LTP marker 行必须保持从行首开始，不能被 ANSI reset/color prefix 污染。任何日志输出改动后都要检查 marker prefix。
- 不提交 root-level kernel、sdcard/disk image、大 raw log、用户给的 `Riscv输出.txt`/`LoongArch输出.txt`，除非用户明确要求。

第一阶段：log-noise repair（必须先做）
1. 定位并修复 `kernel/fs/axfs/src/fops.rs:297` 的 `AxError::NotADirectory` 高频 warning。
2. 原则：保留 `Err(AxError::NotADirectory)` 语义，避免通过 `ax_err!` 在预期负向路径打印 warning。可考虑局部返回 `Err(AxError::NotADirectory)` 或引入小 helper 表达“expected errno without warn”；不要全局粗暴改 `axerrno` 宏，除非有充分设计和回归证据。
3. 同时评估相邻高频噪声：`root.rs:433 [AxError::IsADirectory]`、少量 `AlreadyExists`。本轮优先修 fops:297；若改动安全，可顺手做同类 expected errno 降噪，但必须保持 errno 不变。
4. 验证：
   - `cargo fmt --all -- --check`
   - `git diff --check`
   - `make A=examples/shell ARCH=riscv64`
   - targeted remote/log local check：跑一个小 LTP subset（例如 `access01,read02,ftest07,mem02`）确认 marker 正常、case 仍 PASS/透明 TCONF、`AxError::NotADirectory` 噪声显著下降。
   - 用脚本统计日志中 `AxError::NotADirectory`、`AxError::IsADirectory` 数量，并写入报告。

本轮高价值候选池（候选不等于 promotion；必须逐个用 fresh evidence 证明）：

Batch A：metadata / permissions / path semantics（优先 20-30 个）
- `access04`
- `chmod05`, `chmod06`, `chmod07`
- `fchmod02`, `fchmod05`, `fchmod06`
- `statx01`, `statx03`
- `readlinkat02`
- `rename01`, `rename03`, `rename04`, `rename05`
- `openat02`, `openat03`
- 继续从 `link`, `unlink`, `symlink`, `mkdir`, `rmdir`, `truncate`, `ftruncate` 邻近家族筛选高 ROI clean subset。

Batch B：FD / pipe / iovec / fcntl（优先 15-25 个）
- `writev03` 仍需谨慎：此前有 blocker 记忆，必须 fresh 复核 internal TCONF/TFAIL，不 clean 不 promotion。
- `pipe02`, `pipe07`, `pipe08`, `pipe15`, `pipe2_*` 邻近候选。
- `readv`, `writev`, `preadv`, `pwritev`, `sendfile`, `fcntl` 邻近候选，优先已有语义覆盖的 shared offset、O_APPEND、F_GETFL/F_SETFL、FD_CLOEXEC、pipe capacity/FIONREAD。

Batch C：process / wait / signal（优先 15-25 个）
- `kill02`：曾有 targeted clean 但 LA full aggregate 早期暴露 TBROK/setup 风险，必须重新 full/aggregate clean 才能 promotion。
- `waitid07`, `waitid08`, `waitid10`
- wait/waitpid/fork/clone/signal 邻近候选，注意 wait status、child lifecycle、signal delivery、ESRCH/EINVAL/EPERM 边界和 musl/glibc 差异。

Batch D：mmap / VM boundary（优先 15-25 个）
- `mmap04`, `mmap05`, `munmap01`
- `mprotect01`, `mprotect02`
- `mmap10_1`, `mmap12`, `mmap13`, `mmap14`, `vma01`, `vma02`
- 注意 page permission、SIGSEGV/SIGBUS、MAP_FIXED/MAP_PRIVATE/MAP_SHARED、unmap 边界、user-memory copying。

Batch E：fs suite high-ROI substitutes（用于补齐 stable450）
- `fs_perms01`-`fs_perms06`
- `ftest06`, `ftest09` 及相邻 ftest clean subset（注意 timeout 风险）
- `rwtest01`, `rwtest02`
- `stream02` 及相邻 stream clean subset
- `openfile01`, `writetest01`, `iogen01`, `fs_inod01`, `inode02` 等轻量 VFS/FD/IO 行为测试。

不要优先追：`fs_bind*`, `test_robind*`, `ksm*`, `fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*`, broad xattr/namespace/io_uring/perf 等低 ROI 或高重构风险族，除非高优先级池耗尽且有明确 clean evidence。

候选策略：
1. 本轮允许更大胆：每个 promotion 小阶段可以新增约 25 个 clean case，而不是 5-10 个；但必须先 targeted matrix，再 promotion。
2. 建议 targeted 验证 100-160 个候选来找 clean subset；只有 RV+LA × musl+glibc 全 clean 的 case 才能加入 stable。
3. 阶段建议：
   - stable400：新增约 25 个 clean case。
   - stable425：再新增约 25 个 clean case。
   - stable450：再新增约 25 个 clean case并进入 final gate。
   - optional stable460/475：只有 clean subset 足够且 final gate 资源允许才推进。
4. 若一个高价值 blocker 短期不 clean，记录 blocker 并转向其他 clean subset，不要让单个 case 阻塞整轮。
5. 每次 promotion 后，必须检查 `LTP_STABLE_CASES` total/unique/duplicates。

推荐 Team 分工：
- Worker 1: Discovery + promotion matrix
  - 从 live `LTP_STABLE_CASES`、phase-b evidence、sdcard/runtest inventory、`scripts/ltp_summary.py --promotion-candidates` 建立 stable375->stable450 候选池。
  - 输出 candidate matrix：按 subsystem、价值评分、RV/LA、musl/glibc、clean/TCONF/TFAIL/TBROK/timeout/ENOSYS/panic 分类。
- Worker 2: log-noise + VFS/metadata lane
  - 先修/审 `axfs::fops:297 [AxError::NotADirectory]` 降噪方案；验证 errno 不变。
  - 之后推进 `access/chmod/fchmod/statx/readlinkat/rename/openat/link/unlink`。
- Worker 3: fd/pipe/iovec lane
  - 排查 `writev03`, pipe/pipe2/readv/writev/preadv/pwritev/sendfile/fcntl 邻近候选。
  - 守住 shared offset、O_APPEND、pipe capacity/FIONREAD、blocking/yield、SIGPIPE 语义。
- Worker 4: process/wait/signal lane
  - 优先 `kill02`, `waitid07/08/10` 和 wait/fork/signal 邻近候选。
  - 注意 musl/glibc 差异、wait status、子进程状态、signal delivery、permission/ESRCH/EINVAL 边界。
- Worker 5: mmap/fs-suite + verification guardrail lane
  - 优先 mmap/munmap/mprotect/vma，以及 fs_perms/ftest/rwtest/stream/openfile/writetest/iogen/fs_inod/inode 替补池。
  - 同时维护 no-fake-pass/no-timeout-as-pass、marker-prefix、remote-log-size/AxError 噪声统计。

执行节奏：
1. Baseline refresh:
   - 统计当前 `LTP_STABLE_CASES` 数量和重复项。
   - 复核 stable375 final gate 和 marker-prefix evidence。
   - 解析最新远程输出，确认 `ltp-glibc` 375/0/0 和 AxError 噪声现状。
2. Log-noise repair gate:
   - 修复 fops:297 噪声。
   - 跑小 subset 证明 errno/marker/LTP 结果不变且噪声下降。
   - 写 `log-noise-repair-report.md`。
3. Candidate discovery:
   - 对 Batch A-E 建立 candidate matrix。
   - Targeted batches 可以一次 20-40 个候选，但不要直接把未知 case 加入 stable。
4. Promotion gates:
   - stable400：新增约 25 个 clean case 后更新 `LTP_STABLE_CASES`，跑 RV+LA targeted/aggregate gate，并用 `scripts/ltp_summary.py` 解析。
   - stable425：同上。
   - stable450：同上并进入 final gate。
   - optional stable460/475：只有 clean subset 足够且资源允许时推进。
   - 如果任一 arch/libc 出现 FAIL、timeout、ENOSYS、panic/trap、新 TFAIL/TBROK/TCONF，停止 promotion，先修复或回退该候选。
5. Final gate:
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv`
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la`
   - `python3 -B scripts/ltp_summary.py <rv-log>` 和 `<la-log>`
   - marker-prefix check：所有 `PASS LTP CASE`/`FAIL LTP CASE` marker 必须从行首开始。
   - AxError 噪声 check：统计 `AxError::NotADirectory`、`AxError::IsADirectory`、`AxError::AlreadyExists`，报告是否显著下降且未隐藏真实 LTP failures。
   - `cargo fmt --all -- --check`
   - 相关构建：至少 `make A=examples/shell ARCH=riscv64`；涉及 remote submission 时跑 `make all`，必要时再跑离线 `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`。

产物要求：
- 使用本地日期目录：`docs/ltp-score-improvement-2026-05-25-phase-c/`。
- 创建并持续更新：
  - `plan-stable375-to-450.md`
  - `log-noise-repair-report.md`
  - `candidate-matrix.md`
  - `stable400-promotion-gate-report.md`
  - `stable425-promotion-gate-report.md`
  - `stable450-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-and-log-noise-regression-check.md`
  - `next-session-prompt-stable450-followup.md`（如果 stable450 未达成或仍有高价值 blocker）
- raw logs 放在 `docs/ltp-score-improvement-2026-05-25-phase-c/raw/`，默认不要提交大 raw logs；提交摘要、报告、case lists、quality gate JSON。
- `.omx/ultragoal/goals.json` 和 `ledger.jsonl` 是本地 audit trail，leader 维护；是否提交遵循仓库 ignore/用户要求。

交付条件：
- log-noise repair：`axfs::fops:297 [AxError::NotADirectory]` 高频噪声明显降低，且 errno/POSIX 行为不变；marker prefix 仍 0 bad lines。
- stable450 主目标：live `LTP_STABLE_CASES` 正好 450 个 unique case。
- stable450 RV final stable gate：PASS LTP CASE 900，FAIL 0；ltp-musl 450/0；ltp-glibc 450/0。
- stable450 LA final stable gate：PASS LTP CASE 900，FAIL 0；ltp-musl 450/0；ltp-glibc 450/0。
- 如果达成 stable460：PASS LTP CASE 920，FAIL 0；ltp-musl 460/0；ltp-glibc 460/0；RV+LA 均成立。
- 如果达成 stable475：PASS LTP CASE 950，FAIL 0；ltp-musl 475/0；ltp-glibc 475/0；RV+LA 均成立。
- internal TFAIL=0、TBROK=0；除已明确披露且可接受的 known TCONF 外不得新增 TCONF。若 `read02` 仍在 stable，则继续披露其 pass_with_tconf，不把它说成 clean。
- timeout/ENOSYS/panic/trap 均为 0。
- 已完成 code review + ai-slop-cleaner audit。
- 已按 AGENTS.md 自动 commit agent-owned tracked 变更，并在最终回复列出 commit SHA、验证命令、未能运行的检查、用户可见行为变化、ABI/POSIX 变化。

如果在 stable450 前遇到硬阻塞：
- 不要伪造或隐藏失败。
- 保存当前最高可信 stableN 的证据和 blocker 报告。
- 明确列出失败 case、arch/libc、内部 TFAIL/TBROK/TCONF、timeout/ENOSYS/panic/trap、相关日志路径、已尝试修复和下一步建议。
```
