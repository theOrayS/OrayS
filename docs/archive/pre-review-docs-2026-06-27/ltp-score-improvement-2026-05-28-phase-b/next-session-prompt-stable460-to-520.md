# Next session prompt: stable460 -> stable520 with Ultragoal + Team

Created: 2026-05-28
Target repo: `/root/oskernel2026-orays`
Branch baseline at prompt creation: `score/best`
Suggested starting commit from this handoff: `f40332a9 Archive stale LTP evidence without deleting it` or later

Use this prompt as the first message in the next Codex/OMX session.

```text
我现在要启动下一轮 LTP stable 提分任务：目标是从当前 stable460 提升到 stable520，stretch goal 是 stable530。请使用 Ultragoal + Team 模式推进，按仓库 AGENTS.md 执行，中文汇报。

工作目录：/root/oskernel2026-orays

当前已知基线（必须 live 复核，不要只依赖本提示词）：
- 当前 handoff commit：f40332a9 Archive stale LTP evidence without deleting it
- 当前分支：score/best
- live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 在提示词创建时为 460 total / 460 unique / 0 duplicates；下一会话必须重新计算。
- stable460 已交付，可信证据在 `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-27-phase-a/`：
  - `stable460-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-and-log-noise-regression-check.md`
  - `raw/stable460-rv-final-gate-002-summary.txt`
  - `raw/stable460-la-final-gate-002-summary.txt`
- stable460 local final gate at handoff:
  - RV: PASS LTP CASE 920, FAIL 0; ltp-musl 460/0; ltp-glibc 460/0。
  - LA: PASS LTP CASE 920, FAIL 0; ltp-musl 460/0; ltp-glibc 460/0。
  - known transparent `read02` TCONF only：每个 arch 4 个 internal TCONF，来自 `rv/la × musl/glibc read02`；不得把它说成 internal-clean，也不得新增 TCONF。
  - timeout / ENOSYS / panic / trap: 0。
  - marker-prefix bad lines: 0。
  - `AxError::NotADirectory`: RV 12 / LA 12；`axfs::fops:297`: 0；披露即可，不影响 LTP marker。
- stable460 最后一轮新增 8 个 case：
  - `fchown05`, `fchownat01`, `fcntl18`, `fcntl18_64`, `syscall01`, `mknod06`, `mknod02`, `mknod05`。
- stable460 明确保留/阻塞信息：
  - clean reserves only：`mknod08`, `mknodat01`, `rename14`；它们仍需 fresh RV+LA × musl+glibc targeted gate 后才能进入 stable。
  - `kill02`：targeted clean 曾被 LA aggregate stable460 TBROK setup-timeout 推翻；不得从 targeted-only 证据推广。
  - `readlinkat02`：LA musl TFAIL；不得推广，除非先做真实修复并 fresh 四路 clean。
- 2026-05-28 phase-a 曾有 stable460 -> stable470 三人分工文档；如果工作树显示该文件被移动/删除，不要盲目恢复，先以 live git status 和当前文档归档状态为准。

本轮目标：
1. 主目标：新增 60 个真实 clean LTP case，达到 stable520。
2. Stretch：如果 clean subset 充足且 final gate 资源允许，推进到 stable530。
3. 不为数量牺牲真实性。只有 RV+LA × musl+glibc 全 clean 的 case 才能 promotion。
4. 如果 stable520 前遇到硬 blocker，保存最高可信 stableN gate、blocker 报告和下一轮提示词，不伪造、不隐藏失败。

启动要求：
1. 先读取 `AGENTS.md`、`docs/agent-workflow/ltp-promotion-and-docs.md`、`docs/agent-workflow/ltp-selection.md` 和本提示词。
2. 磁盘 preflight：
   - `df -h / /root`
   - `du -sh /root/.codex`
   - 如果 `/` 接近满（约 85%+ 或少于 10GiB 可用），先清理低价值临时日志/cache；不要删 memories/skills/prompts/agents/凭据/活跃 `.omx` 状态。
3. `git status --short`，确认只处理 agent-owned 变更；不要回滚用户文件、远程输出日志、未跟踪证据或别人删除/移动的文档。
4. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算当前 stable 数量和重复项，不能依赖记忆或本提示词。
5. 复核 stable460 final summaries、quality gate、code-review、ai-slop-cleaner、marker-prefix evidence；stable460 必须作为回归保护。
6. 读取 stable413->460 关键报告，尤其：
   - `candidate-matrix-stable413-to-460.md`
   - `stable425-promotion-gate-report.md`
   - `stable440-promotion-gate-report.md`
   - `stable452-promotion-gate-report.md`
   - `stable460-delivery-report.md`
   - `next-session-prompt-stable460-followup.md`
   - worker reports for VFS/FD/metadata/mmap/process guardrails。
7. 建立本轮目录：`docs/ltp-score-improvement-2026-05-28-phase-b/`，raw logs 放 `raw/`。
8. 创建/恢复 Ultragoal durable plan：目标 stable520，stretch stable530；建议阶段为 baseline-refresh -> candidate-matrix -> stable475 -> stable490 -> stable505 -> stable520 -> optional stable530。
9. 启动 Team 模式提高吞吐；如果 tmux pane/资源受限，优先 5 个 worker，失败再降到 4 个 worker。Leader 负责 `.omx/ultragoal`、`LTP_STABLE_CASES` 最终修改、promotion 决策和最终验证；worker 只做被分配的 discovery/修复/验证/报告切片。

Ultragoal 具体用法：
1. 先把本提示词压缩成短 brief（建议文件名）：
   - `docs/ltp-score-improvement-2026-05-28-phase-b/ultragoal-brief-stable460-to-520.md`
   - brief 应包含：目标 stable520/stretch530、阶段、worker 分工、promotion gate、final gate、磁盘/commit/remote marker guardrails。
2. 创建 plan：
   - `omx ultragoal create-goals --force --brief-file docs/ltp-score-improvement-2026-05-28-phase-b/ultragoal-brief-stable460-to-520.md`
   - 如果还没来得及写 brief，可临时用本提示词文件作为 brief source，但之后应补短 brief，避免过度碎片化 goals。
   - 检查 `.omx/ultragoal/goals.json` 和 `.omx/ultragoal/ledger.jsonl`。
3. 执行每个 story：
   - `omx ultragoal status`
   - `omx ultragoal complete-goals`
   - 按输出 handoff 使用 Codex goal 工具：先 `get_goal`；如无 active goal，则 `create_goal`；中间 story 完成时不要 `update_goal complete`。
4. 每个阶段完成后，leader 用新鲜 `get_goal` snapshot checkpoint：
   - `omx ultragoal checkpoint --goal-id <id> --status complete --evidence "<证据路径和摘要>" --codex-goal-json <fresh-get-goal-json-or-path>`
5. 最终 story 必须先完成 final gate、code-review、ai-slop-cleaner、marker-prefix/noise check、磁盘 post-check 和 commit；只有 final clean 后才 `update_goal({status: "complete"})`，再用 complete snapshot checkpoint。
6. Team workers 不拥有 `.omx/ultragoal`，不创建 worker ledger，不 checkpoint Ultragoal，不最终修改 `LTP_STABLE_CASES`。

Team 具体用法：
1. 启动前做 preflight：
   - `tmux -V`
   - `test -n "$TMUX"`
   - `command -v omx`
   - `tmux list-panes -F '#{pane_id}\t#{pane_start_command}' | rg 'hud --watch' || true`
2. 推荐启动：
   - `omx team 5:executor "LTP stable460 to stable520 high-value promotion with Ultragoal leader-owned gates"`
   - 如果 pane/资源不足，降级为 `omx team 4:executor "LTP stable460 to stable520 high-value promotion with Ultragoal leader-owned gates"`。
3. 启动后确认 team line、tmux target、worker panes、ACK mailbox。
4. 监控优先使用 runtime/state：`omx team status <team>`、`omx team resume <team>`、mailbox、task JSON。
5. 只有 pending=0、in_progress=0、failed=0（或失败路径已明确保存 blocker）后，才运行 `omx team shutdown <team>`。
6. 不要让多个 worker 并发争用默认 QEMU/sdcard/qcow2。若没有隔离镜像，promotion gate 和 final gate 由 leader 串行跑；worker 并发输出只能作为 discovery，不能作为 promotion 证据。

强约束：
- 不伪造 PASS。
- 不 hardcode case name/path/process name 通过。
- 不修改 LTP 测试源码来通过。
- 不修改 evaluator/runner 绕过真实失败。
- 不把真实 TFAIL/TBROK/timeout/panic/trap/ENOSYS 转成 SKIP/TCONF/PASS。
- timeout 不能计为 PASS。
- wrapper 成功不等于可 promotion；所有证据必须用 `python3 -B scripts/ltp_summary.py` 或等价 case matrix 解析。
- `read02` 的 `pass_with_tconf` 必须保持透明；clean 的定义是没有新增 internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
- 远程评分修复不能回退：LTP marker 行必须保持从行首开始，不能被 ANSI reset/color prefix 污染；不要随意改变当前 remote-compatible marker wire。任何日志输出改动后都要检查 marker prefix。
- 不提交 root-level `kernel-rv`/`kernel-la`、sdcard/disk image、大 raw log、用户给的远程输出 txt，除非用户明确要求。
- 每次长跑前后都要做磁盘检查；如果 `.codex` 继续膨胀，只清理低价值 transient logs/cache，不删关键记忆和技能。

本轮高价值候选池（候选不等于 promotion；必须逐个用 fresh evidence 证明）：

Batch 0：stable460 clean reserves（第一优先级，目标 +3）
- `mknod08`
- `mknodat01`
- `rename14`
- 要求：先 fresh RV+LA × musl+glibc targeted gate；如果全 clean，可作为 stable463/stable475 首批候选。

Batch A：VFS / path / permission / mknod / rename（目标 12-18 个）
- 低风险相邻 scout：`mknod01`, `mknod03`, `mknod04`, `mknod07`, `mknod09`, `mknodat02`, `rename03`, `rename04`, `rename05`, `openat02`, `openat03`。
- 需要先诊断/修复：`readlinkat02`（LA musl TFAIL）, `access04`, `chmod05`, `chmod06`, `chmod07`, `fchmod02`, `fchmod05`, `fchmod06`, `fchmodat02`, `statx01`, `statx03`。
- 邻近替补：`link`, `unlink`, `symlink`, `mkdir`, `rmdir`, `truncate`, `ftruncate` 中源代码/targeted evidence 显示低风险的 case。
- 注意 parent write/search permission、sticky bit、symlink loop、目录/文件 errno、EFAULT/ENOTDIR/EISDIR/EROFS，不要为单 case 破坏通用语义。

Batch B：FD / fcntl / pipe / ownership（目标 12-18 个）
- 第一批 scout：`pipe07`, `fcntl19`, `fcntl19_64`, `fcntl20`, `fcntl20_64`, `fcntl21`, `fcntl21_64`, `fcntl22`, `fcntl22_64`, `fchown04`, `fchownat02`, `chown04`。
- 补充 scout：剩余 `fcntl` low-risk rows、`readv`/`writev` 邻近项、`pipe2_*` 中 source-level 明确不会触发已知 SIGPIPE/lock-order/timeout blocker 的 case。
- 谨慎/默认不进第一批：`pipe02`（已有 RV musl panic/trap 史）、`select01`-`select04`/`pselect01`（TCONF/语义风险）、`close_range*`（dispatch/语义风险）。
- 守住 shared offset、O_APPEND、negative offset errno、pipe capacity/FIONREAD、blocking/yield、SIGPIPE、file-region lock 语义。

Batch C：metadata / statfs / getdents / stat-family（目标 8-12 个，先修再跑）
- `getdents01`, `getdents02`：优先评估 `getdents64` d_off/d_ino/record semantics；不要盲加不存在的 legacy syscall alias。
- `fstat02`, `fstat02_64`：先捕获实际 ENOSYS syscall number；不要盲目 old-stat-family patch。
- `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01`：需真实 field/setup 修复；注意不回归 stable `statfs02`, `fstatfs02`, `statvfs02`。
- `getcwd03`, `getcwd04`：只在 path/chdir/search-permission setup 明确后处理；`getcwd04` 的 TCONF 不能算 clean。

Batch D：process / wait / signal / light syscalls（目标 8-12 个，必须 aggregate 证明）
- 低风险 scout：`waitid07`, `waitid08`, `waitid10`, `setpriority01`, `nice04`, `clock_gettime01`, `clock_gettime04`, `sched_rr_get_interval03`, `sched_setaffinity01`, `setrlimit04`, `setrlimit05`, `signal01`。
- 谨慎 blocker：`kill02` 必须先解释 LA aggregate child setup TBROK；只有修复后 RV+LA aggregate clean 才能 promotion。
- 上轮阻塞项：`poll02`, `gethostid01`, `getcpu01`, `gethostname02` 需要 libc-specific failure 诊断；不能从单 libc 或 TCONF 行推 clean。
- 默认暂缓：`times03`, `getpgid01`, `fork13`, `fork14`, `clone06`-`clone09`, `kill05`, `kill10`，除非有明确 source fix 和四路 clean 证据。

Batch E：mmap / mprotect / VM hidden-test defense（目标 5-10 个，作为补位或专门修复 lane）
- `mmap04`, `mmap05`, `mmap06`, `munmap01`
- `mprotect01`, `mprotect02`
- `mmap10_1`, `mmap12`, `mmap13`, `mmap14`, `vma01`, `vma02`
- 只有在 basic mapping、permission、VMA split/merge、user-memory copy、page fault 行为清楚且 targeted parser clean 后才能 promotion；不要用 broad stress 掩盖真实 VM 缺口。

Batch F：fs-suite high-ROI substitutes（目标 5-12 个，先 RV scout 再 LA confirm）
- `fs_perms01`-`fs_perms06`
- `ftest06`, `ftest09`
- `rwtest01`, `rwtest02`
- `stream02`, `openfile01`, `writetest01`, `iogen01`, `fs_inod01`, `inode02`
- 注意很多历史行有 TFAIL/TBROK/ENOSYS/timeout；不要从 inventory 存在推断可 promotion。

不要优先追：`fs_bind*`, `test_robind*`, `ksm*`, `fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*`, broad xattr/namespace/io_uring/perf/landlock 等低 ROI 或高重构风险族，除非高优先级池耗尽且有明确 clean evidence。

候选策略：
1. 本轮需要 +60，允许更大胆地 scout 150-250 个候选，但 promotion 仍必须 targeted matrix -> source fix/diagnosis -> RV clean -> LA clean -> stable aggregate gate。
2. 每个 promotion 阶段建议新增约 15 个 clean case：stable475、stable490、stable505、stable520；不要一次把 60 个未知 case 直接加入 stable。
3. 可以先做 RV-first scout 找 clean subset；但只有 RV+LA × musl+glibc 全 clean 的 case 才能加入 stable。
4. 若一个高价值 blocker 短期不 clean，记录 blocker 并转向其他 clean subset，不要让单个 case 阻塞整轮。
5. 每次 promotion 后，必须检查 `LTP_STABLE_CASES` total/unique/duplicates。
6. 涉及 syscall/errno/ABI-visible 行为的修复，最终报告必须明确列出用户可见/POSIX 行为变化；无意变化也要明确说明。

推荐 Team 分工：
- Worker 1: Discovery + promotion matrix
  - 从 live `LTP_STABLE_CASES`、stable460 evidence、sdcard/runtest inventory、`scripts/ltp_summary.py --promotion-candidates` 建立 stable460->stable520 候选池。
  - 输出 `candidate-matrix-stable460-to-520.md`：按 subsystem、价值评分、RV/LA、musl/glibc、clean/TCONF/TFAIL/TBROK/timeout/ENOSYS/panic 分类。
- Worker 2: VFS/path/permission/mknod/rename lane
  - 先证明 `mknod08,mknodat01,rename14`，再推进 mknod/mknodat/rename/openat/readlinkat/statx/chmod/access/link/unlink/symlink/mkdir/rmdir/truncate。
- Worker 3: FD/fcntl/pipe/ownership lane
  - 推进 `pipe07`, `fcntl19-22` variants, `fchown04`, `fchownat02`, `chown04`，同时守住 SIGPIPE、fcntl lock、FD table、rlimit 行为。
- Worker 4: metadata/statfs/getdents/stat-family lane
  - 做 source-level LTP expectation + syscall trace；只做窄修复，保护 stable `statfs02`, `fstatfs02`, `statvfs02`, `getcwd01/02`。
- Worker 5: process/wait/signal/mmap/fs-suite + verification guardrail lane
  - 先找 low-risk process/light syscall 和 fs-suite substitute；mmap/mprotect 只在 source evidence 明确后推进。
  - 同时维护 no-fake-pass/no-timeout-as-pass、marker-prefix、remote-log-size/AxError 噪声统计。

执行节奏：
1. Baseline refresh:
   - 统计当前 `LTP_STABLE_CASES` 数量和重复项。
   - 复核 stable460 final gate、code-review、ai-slop-cleaner 和 marker/noise evidence。
   - 如果 baseline 可疑，先跑一个小 stable smoke；不要一开始就烧一次完整 LA/RV final gate。
2. Candidate discovery:
   - 建立候选 matrix，先 targeted batches；不要直接把未知 case 加入 stable。
   - Targeted batches 可以一次 20-50 个候选，但必须按 parser summary 分 arch/libc、internal result、timeout、ENOSYS、panic/trap。
3. Promotion gates:
   - stable475：新增约 15 个 clean case 后更新 `LTP_STABLE_CASES`，跑 RV+LA targeted/aggregate gate，并用 `scripts/ltp_summary.py` 解析。
   - stable490：再新增约 15 个 clean case，同上。
   - stable505：再新增约 15 个 clean case，同上。
   - stable520：再新增约 15 个 clean case，同上并进入 final gate。
   - optional stable530：只有 clean subset 足够且资源允许时推进。
   - 如果任一 arch/libc 出现 FAIL、timeout、ENOSYS、panic/trap、新 TFAIL/TBROK/TCONF，停止 promotion，先修复或回退该候选。
4. Final gate:
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv`
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la`
   - `python3 -B scripts/ltp_summary.py <rv-log>` 和 `<la-log>`
   - stable520 目标 parser 形态：RV/LA 各 `PASS LTP CASE 1040`, `FAIL 0`, `ltp-musl 520/0`, `ltp-glibc 520/0`；仍需显式披露既有 `read02` TCONF caveat。
   - marker-prefix check：所有 LTP wrapper marker 必须从行首开始，0 bad marker lines。
   - noise check：统计 `AxError::NotADirectory` / `AxError::IsADirectory` / `AxError::AlreadyExists`，披露是否影响远程输出体积；不得隐藏真实 LTP failures。
   - `cargo fmt --all -- --check`
   - `git diff --check`
   - 至少 `make A=examples/shell ARCH=riscv64`
   - 涉及 remote submission、runner/build helper、platform config 或工具链时跑 `make all`；必要时再跑离线 `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`。

产物要求：
- 使用本地日期目录：`docs/ltp-score-improvement-2026-05-28-phase-b/`。
- 创建并持续更新：
  - `plan-stable460-to-520.md`
  - `ultragoal-brief-stable460-to-520.md`
  - `candidate-matrix-stable460-to-520.md`
  - `stable475-promotion-gate-report.md`
  - `stable490-promotion-gate-report.md`
  - `stable505-promotion-gate-report.md`
  - `stable520-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-and-log-noise-regression-check.md`
  - `next-session-prompt-stable520-followup.md`（如果 stable520 未达成或仍有高价值 blocker）
- raw logs 放在 `docs/ltp-score-improvement-2026-05-28-phase-b/raw/`，默认不要提交大 raw logs；提交摘要、报告、case lists、quality gate JSON。
- `.omx/ultragoal/goals.json` 和 `ledger.jsonl` 是本地 audit trail，leader 维护；是否提交遵循仓库 ignore/用户要求。

交付条件：
- live `LTP_STABLE_CASES` 正好 520 total / 520 unique / 0 duplicates。
- RV stable aggregate：PASS LTP CASE 1040, FAIL 0; ltp-musl 520/0; ltp-glibc 520/0；除既有 `read02` caveat 外无新增 internal failure。
- LA stable aggregate：PASS LTP CASE 1040, FAIL 0; ltp-musl 520/0; ltp-glibc 520/0；除既有 `read02` caveat 外无新增 internal failure。
- timeout / ENOSYS / panic / trap: 0。
- marker-prefix bad lines: 0。
- code-review 和 ai-slop-cleaner 均通过。
- `cargo fmt --all -- --check`、`git diff --check`、`make A=examples/shell ARCH=riscv64` 通过；如改动触及 remote submission/build helper，则 `make all` 也通过。
- final report 明确列出：新增 cases、raw/summary/status/marker/noise 证据路径、用户可见行为变化、syscall/errno/ABI 影响、未运行项。
- 完成并验证后自动创建 Git commit，提交信息按 Lore 协议；只暂存 agent-owned 变更，不提交大 raw logs、root kernels、sdcard/disk images 或用户远程输出证据。

如果无法达到 stable520：
- 不要硬冲或 fake pass。
- 保留最高可信 stableN final gate（例如 stable475/stable490/stable505）。
- 写 `stable520-blocker-report.md`，列出每个 blocker 的 syscall/errno/ABI/root-cause、证据路径、下一步最小修复。
- 写 `next-session-prompt-stableN-to-520.md`，继续沿用 Team + Ultragoal + targeted batch -> promotion -> final RV+LA gate 节奏。
```
