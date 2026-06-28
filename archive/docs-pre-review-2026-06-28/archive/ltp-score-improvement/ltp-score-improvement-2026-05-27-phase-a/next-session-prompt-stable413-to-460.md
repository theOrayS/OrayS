# Next session prompt: stable413 -> stable460 with Ultragoal + Team

Created: 2026-05-27
Target repo: `/root/oskernel2026-orays`
Suggested starting commit from this handoff: `76506626 Promote stable LTP coverage with verified low-risk FD/sendfile semantics` or later

Use this prompt as the first message in the next Codex/OMX session.

```text
我现在要启动下一轮 LTP stable 提分任务：目标是从当前 stable413 提升到 stable460，stretch goal 是 stable470。请使用 Ultragoal + Team 模式推进，按仓库 AGENTS.md 执行，中文汇报。

工作目录：/root/oskernel2026-orays

当前已知基线（必须 live 复核，不要只依赖本提示词）：
- 当前 handoff commit：76506626 Promote stable LTP coverage with verified low-risk FD/sendfile semantics
- live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 在提示词创建时为 413 total / 413 unique / 0 duplicates；下一会话必须重新计算。
- stable413 已交付，证据在 `docs/ltp-score-improvement-2026-05-26-phase-a/`：
  - `stable413-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-and-log-noise-regression-check.md`
  - `raw/stable413-rv-final-gate-002-summary.txt`
  - `raw/stable413-la-final-gate-002-summary.txt`
- stable413 local final gate at handoff:
  - RV: PASS LTP CASE 826, FAIL 0; ltp-musl 413/0; ltp-glibc 413/0
  - LA: PASS LTP CASE 826, FAIL 0; ltp-musl 413/0; ltp-glibc 413/0
  - known transparent `read02` TCONF only：每个 arch 4 个 internal TCONF，来自 `rv/la × musl/glibc read02`；不得把它说成 clean，也不得新增 TCONF。
  - timeout / ENOSYS / panic / trap: 0
  - marker-prefix bad lines: 0
  - `axfs::fops:297 [AxError::NotADirectory]`: 0；残余 `axfs_ramfs::file:69` NotADirectory: RV 22 / LA 22，披露即可，不影响 LTP marker。
- stable413 本轮已新增 30 个 case：
  - `preadv01`, `preadv02`, `pwritev01`, `pwritev02`
  - `pread02`, `pread02_64`, `pwrite02`, `pwrite02_64`, `pwrite04`, `pwrite04_64`
  - `sendfile02`, `sendfile02_64`, `sendfile03`, `sendfile03_64`, `sendfile04`, `sendfile04_64`, `sendfile05`, `sendfile05_64`, `sendfile06`, `sendfile06_64`, `sendfile08`, `sendfile08_64`
  - `preadv201`, `preadv201_64`, `preadv202`, `preadv202_64`, `pwritev201`, `pwritev201_64`, `pwritev202`, `pwritev202_64`
- 最新远程评测机输出文件可能仍在仓库根目录：`Riscv输出.txt`、`LoongArch输出.txt`；它们是用户证据，默认不要提交。

本轮目标：
1. 主目标：新增 47 个真实 clean LTP case，达到 stable460。
2. Stretch：如果 clean subset 足够且 final gate 资源允许，推进到 stable470。
3. 不为数量牺牲真实性。只有 RV+LA × musl+glibc 全 clean 的 case 才能 promotion。
4. 如果 stable460 前遇到硬 blocker，保存最高可信 stableN gate、blocker 报告和下一轮提示词，不伪造或隐藏失败。

启动要求：
1. 先读取 `AGENTS.md` 和本提示词。
2. 磁盘 preflight：
   - `df -h / /root`
   - `du -sh /root/.codex`
   - 如果 `/` 接近满（约 85%+ 或少于 10GiB 可用），先清理低价值临时日志/cache；可以清理旧 `.codex` transient logs/cache，但不要删 memories/skills/prompts/agents/凭据/活跃 `.omx` 状态。
3. `git status --short`，确认只处理 agent-owned 变更；不要回滚用户文件或未跟踪远程输出日志。
4. 从 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算当前 stable 数量和重复项，不能依赖记忆或本提示词。
5. 复核 stable413 final summaries、quality gate、marker-prefix evidence；stable413 必须作为回归保护。
6. 复核 `docs/ltp-score-improvement-2026-05-26-phase-a/` 的 worker 报告和 deferred stretch：
   - `candidate-matrix-easy30-40.md`
   - `worker1-candidate-matrix-delta-after-reports.md`
   - `worker2-light-syscall-process-scout-report.md`
   - `worker2-light-syscall-rv001-diagnosis.md`
   - `worker3-metadata-statfs-getdents-report.md`
   - `worker3-metadata-narrow-repair-feasibility.md`
   - `worker4-fd-io-vfs-guardrail-report.md`
   - `stable423-stretch-report.md`
7. 创建/恢复 Ultragoal durable plan：目标 stable460，stretch stable470；建议阶段为 baseline-refresh -> candidate-matrix -> stable425 -> stable440 -> stable452 -> stable460 -> optional stable470。
8. 启动 Team 模式提高吞吐；如果 tmux pane/资源受限，优先 5 个 worker，失败再降到 4 个 worker。Leader 负责 `.omx/ultragoal`、`LTP_STABLE_CASES` 最终修改、promotion 决策和最终验证；worker 只做被分配的 discovery/修复/验证/报告切片。

Ultragoal 具体用法：
1. 先把本提示词压缩成短 brief（建议文件名）：
   - `docs/ltp-score-improvement-2026-05-27-phase-a/ultragoal-brief-stable413-to-460.md`
   - brief 应包含：目标 stable460/stretch470、阶段、worker 分工、promotion gate、final gate、磁盘/commit/remote marker guardrails。
2. 创建 plan：
   - `omx ultragoal create-goals --force --brief-file docs/ltp-score-improvement-2026-05-27-phase-a/ultragoal-brief-stable413-to-460.md`
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
   - `omx team 5:executor "LTP stable413 to stable460 high-value promotion with Ultragoal leader-owned gates"`
   - 如果 pane/资源不足，降级为 `omx team 4:executor "LTP stable413 to stable460 high-value promotion with Ultragoal leader-owned gates"`。
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
- 远程评分修复不能回退：LTP marker 行必须保持从行首开始，不能被 ANSI reset/color prefix 污染；不要随意改变当前 remote-compatible marker wire。任何日志输出改动后都要检查 marker prefix。
- 不提交 root-level `kernel-rv`/`kernel-la`、sdcard/disk image、大 raw log、用户给的 `Riscv输出.txt`/`LoongArch输出.txt`，除非用户明确要求。
- 每次长跑前后都要做磁盘检查；如果 `.codex` 继续膨胀，只清理低价值 transient logs/cache，不删关键记忆和技能。

本轮高价值候选池（候选不等于 promotion；必须逐个用 fresh evidence 证明）：

Batch A：VFS / permission / path semantics（优先修复后找 clean subset，目标 12-18 个）
- `access04`
- `chmod05`, `chmod06`, `chmod07`
- `fchmod02`, `fchmod05`, `fchmod06`, `fchmodat02`
- `statx01`, `statx03`
- `readlinkat02`
- `rename01`, `rename03`, `rename04`, `rename05`
- `openat02`, `openat03`
- 邻近替补：`link`, `unlink`, `symlink`, `mkdir`, `rmdir`, `truncate`, `ftruncate` 中源代码/targeted evidence 显示低风险的 case。

Batch B：FD / pipe / iovec / fcntl adjacent（目标 8-12 个）
- 先从 stable413 已修好的 positioned I/O、sendfile、preadv/pwritev 语义相邻 case 做 inventory，不重复已进 stable 的 case。
- 谨慎 scout：`writev03`, `readv`/`writev` 未 stable 邻近项、`pipe2_*`, 轻量 `fcntl`。遇到 setup/TCONF/timeout 立即 demote。
- `preadv03` / `pwritev03` 仍可能涉及 O_DIRECT / block ioctl / mount-device 风险，默认不是 easy-first。

Batch C：metadata / statfs / getdents repair lane（目标 5-10 个，先修再跑）
- `getdents01`, `getdents02`：优先评估 `getdents64` d_off/d_ino/record semantics；不要盲加不存在的 legacy syscall alias。
- `fstat02`, `fstat02_64`：先捕获实际 ENOSYS syscall number；不要盲目 old-stat-family patch。
- `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01`：需真实 statfs/statvfs field/setup 修复；注意不回归 stable `statfs02`, `fstatfs02`, `statvfs02`。
- `getcwd03`, `getcwd04`：只在 path/chdir/search-permission setup 明确后处理；`getcwd04` 的 TCONF 不能算 clean。

Batch D：VFS create/open/remove repair lane（目标 8-15 个）
- Narrow repair first：`open06`, `creat04`, `mkdir04`, `rmdir03`, `unlink08`。
- Scout after sanity：`unlink07`, `open07`, `creat06`, `mkdir03`, `rmdir02`。
- Reserve / not easy-first：`open10`, `open11`, `open12`, `open14`, `creat07`, `creat08`, `creat09`, `mkdir09`, `unlink09`，因为涉及 setgid/device/O_TMPFILE/largefile/exec-file/ioctl/stress。

Batch E：process / wait / signal / light syscalls（目标 5-10 个，必须 aggregate 证明）
- `waitid07`, `waitid08`, `waitid10`
- `kill02` 必须特别谨慎：曾有 targeted near-clean 但 aggregate/setup blocker 风险；只有 RV+LA aggregate clean 后才能 promotion。
- `poll02`, `gethostid01`, `getcpu01`, `gethostname02`：上一轮 RV scout 不是 promotion-clean；只作为诊断/repair lane，不能从单 libc 或 TCONF 行推 clean。
- `times03`, `getpgid01`, `fork13`, `fork14`, `clone06`-`clone09`, `kill05`, `kill10`：已有失败/timeout/clone-flag 风险，默认不占第一批 promotion 预算。

Batch F：mmap / mprotect / fs-suite high-ROI substitutes（用于补齐 stable460）
- `mmap04`, `mmap05`, `mmap06`, `munmap01`
- `mprotect01`, `mprotect02`
- `mmap10`, `mmap10_1`, `mmap12`, `mmap13`, `mmap14`, `vma01`, `vma02`
- fs-suite 替补：`fs_perms01`-`fs_perms06`, `ftest06`, `ftest09`, `rwtest01`, `rwtest02`, `stream02`, `openfile01`, `writetest01`, `iogen01`, `fs_inod01`, `inode02`。

不要优先追：`fs_bind*`, `test_robind*`, `ksm*`, `fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*`, broad xattr/namespace/io_uring/perf 等低 ROI 或高重构风险族，除非高优先级池耗尽且有明确 clean evidence。

候选策略：
1. 本轮需要 +47，允许更大胆，但必须 targeted matrix -> source fix/diagnosis -> RV clean -> LA clean -> stable aggregate gate。
2. 每个 promotion 阶段建议新增约 12-15 个 clean case：stable425、stable440、stable452、stable460；不要一次把 47 个未知 case 直接加入 stable。
3. 可以 targeted 验证 120-200 个候选来找 clean subset；只有 RV+LA × musl+glibc 全 clean 的 case 才能加入 stable。
4. 若一个高价值 blocker 短期不 clean，记录 blocker 并转向其他 clean subset，不要让单个 case 阻塞整轮。
5. 每次 promotion 后，必须检查 `LTP_STABLE_CASES` total/unique/duplicates。
6. 涉及 syscall/errno/ABI-visible 行为的修复，最终报告必须明确列出用户可见/POSIX 行为变化；无意变化也要明确说明。

推荐 Team 分工：
- Worker 1: Discovery + promotion matrix
  - 从 live `LTP_STABLE_CASES`、phase-a evidence、sdcard/runtest inventory、`scripts/ltp_summary.py --promotion-candidates` 建立 stable413->stable460 候选池。
  - 输出 `candidate-matrix-stable413-to-460.md`：按 subsystem、价值评分、RV/LA、musl/glibc、clean/TCONF/TFAIL/TBROK/timeout/ENOSYS/panic 分类。
- Worker 2: VFS/permissions/path lane
  - 推进 `access/chmod/fchmod/statx/readlinkat/rename/openat/link/unlink/symlink/mkdir/rmdir/truncate/ftruncate`。
  - 重点检查 parent write/search permission、sticky、setgid、symlink loop、目录/文件 errno、EFAULT/ENOTDIR/EISDIR/EROFS。
- Worker 3: metadata/statfs/getdents lane
  - 先做 source-level LTP expectation + syscall trace；再尝试 `getdents64`、`fstat`、`statfs/statvfs` 的 narrow repairs。
  - 守住 stable `fstat03`, `fstatat01`, `statfs02`, `fstatfs02`, `statvfs02`, `getcwd01/02`。
- Worker 4: FD/pipe/iovec/fcntl lane
  - 找 stable413 FD/sendfile 修复的相邻 clean subset；排查 `writev03`, pipe2, light fcntl。
  - 守住 shared offset、O_APPEND、negative offset errno、pipe capacity/FIONREAD、blocking/yield、SIGPIPE 语义。
- Worker 5: mmap/process/signal + verification guardrail lane
  - 推进 mmap/mprotect/munmap/vma、waitid/kill/light syscall 的低风险 subset。
  - 同时维护 no-fake-pass/no-timeout-as-pass、marker-prefix、remote-log-size/AxError 噪声统计。

执行节奏：
1. Baseline refresh:
   - 统计当前 `LTP_STABLE_CASES` 数量和重复项。
   - 复核 stable413 final gate、code-review、ai-slop-cleaner 和 marker/noise evidence。
   - 如果 baseline 可疑，先跑一个小 stable smoke；不要一开始就烧一次完整 LA/RV final gate。
2. Candidate discovery:
   - 建立候选 matrix，先 targeted batches；不要直接把未知 case 加入 stable。
   - Targeted batches 可以一次 20-40 个候选，但必须按 parser summary 分 arch/libc、internal result、timeout、ENOSYS、panic/trap。
3. Promotion gates:
   - stable425：新增约 12 个 clean case 后更新 `LTP_STABLE_CASES`，跑 RV+LA targeted/aggregate gate，并用 `scripts/ltp_summary.py` 解析。
   - stable440：再新增约 15 个 clean case，同上。
   - stable452：再新增约 12 个 clean case，同上。
   - stable460：再新增约 8 个 clean case，同上并进入 final gate。
   - optional stable470：只有 clean subset 足够且资源允许时推进。
   - 如果任一 arch/libc 出现 FAIL、timeout、ENOSYS、panic/trap、新 TFAIL/TBROK/TCONF，停止 promotion，先修复或回退该候选。
4. Final gate:
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv`
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la`
   - `python3 -B scripts/ltp_summary.py <rv-log>` 和 `<la-log>`
   - marker-prefix check：所有 LTP wrapper marker 必须从行首开始，0 bad marker lines。
   - noise check：统计 `AxError::NotADirectory` / `AxError::IsADirectory` / `AxError::AlreadyExists`，披露是否影响远程输出体积；不得隐藏真实 LTP failures。
   - `cargo fmt --all -- --check`
   - `git diff --check`
   - 至少 `make A=examples/shell ARCH=riscv64`
   - 涉及 remote submission 或 runner/build helper 时跑 `make all`；必要时再跑离线 `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all`。

产物要求：
- 使用本地日期目录：`docs/ltp-score-improvement-2026-05-27-phase-a/`。
- 创建并持续更新：
  - `plan-stable413-to-460.md`
  - `ultragoal-brief-stable413-to-460.md`
  - `candidate-matrix-stable413-to-460.md`
  - `stable425-promotion-gate-report.md`
  - `stable440-promotion-gate-report.md`
  - `stable452-promotion-gate-report.md`
  - `stable460-delivery-report.md`
  - `final-gate-quality-gate.json`
  - `final-gate-code-review-report.md`
  - `final-gate-ai-slop-cleaner-report.md`
  - `remote-marker-and-log-noise-regression-check.md`
  - `next-session-prompt-stable460-followup.md`（如果 stable460 未达成或仍有高价值 blocker）
- raw logs 放在 `docs/ltp-score-improvement-2026-05-27-phase-a/raw/`，默认不要提交大 raw logs；提交摘要、报告、case lists、quality gate JSON。
- `.omx/ultragoal/goals.json` 和 `ledger.jsonl` 是本地 audit trail，leader 维护；是否提交遵循仓库 ignore/用户要求。

交付条件：
- stable460 主目标：live `LTP_STABLE_CASES` 正好 460 个 unique case，0 duplicates。
- stable460 RV final stable gate：PASS LTP CASE 920，FAIL 0；ltp-musl 460/0；ltp-glibc 460/0。
- stable460 LA final stable gate：PASS LTP CASE 920，FAIL 0；ltp-musl 460/0；ltp-glibc 460/0。
- 如果达成 stable470：PASS LTP CASE 940，FAIL 0；ltp-musl 470/0；ltp-glibc 470/0；RV+LA 均成立。
- internal TFAIL=0、TBROK=0；除已明确披露且可接受的 known `read02` TCONF 外不得新增 TCONF。
- timeout/ENOSYS/panic/trap 均为 0。
- marker prefix 检查通过：0 bad marker lines。
- 已完成 code review + ai-slop-cleaner audit。
- 已完成磁盘 post-check：`df -h / /root`、`du -sh /root/.codex`。
- 已按 AGENTS.md 自动 commit agent-owned tracked 变更，并在最终回复列出 commit SHA、验证命令、未能运行的检查、用户可见行为变化、ABI/POSIX 变化。

如果在 stable460 前遇到硬阻塞：
- 不要伪造或隐藏失败。
- 保存当前最高可信 stableN 的证据、case list、quality gate 和 blocker 报告。
- 明确列出失败 case、arch/libc、内部 TFAIL/TBROK/TCONF、timeout/ENOSYS/panic/trap、相关日志路径、已尝试修复和下一步建议。
- 写一份新的 `next-session-prompt-stable<N>-followup.md`，继续使用 Team + Ultragoal，并保留本轮未完成的候选 matrix。
```
