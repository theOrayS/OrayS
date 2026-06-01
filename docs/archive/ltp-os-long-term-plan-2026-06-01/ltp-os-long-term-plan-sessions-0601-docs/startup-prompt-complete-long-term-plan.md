# 启动提示词：完成 2026-06-01 LTP OS 长期完善计划

创建日期：2026-06-01  
目标仓库：`/root/oskernel2026-orays`  
提示词创建时分支：`dev/long-term-plan-0601`  
提示词创建时提交：`e3d43365`  
主计划文件：`docs/ltp-os-long-term-improvement-plan-2026-06-01.md`  
会话文档根目录：`docs/ltp-os-long-term-plan-sessions-0601-docs/`

用途：把下面 fenced block 作为下一次 Codex/OMX 会话的第一条消息。它要求从 Session 1 开始，连续完成主计划中的 5~10 个 sessions；每完成一个 roadmap session 就提交一次独立 Git commit，并把该 session 的文档放在会话文档根目录下的独立子文件夹中。

````text
我要执行 `/root/oskernel2026-orays` 的长期 LTP/OS 完善任务：完整完成 `docs/ltp-os-long-term-improvement-plan-2026-06-01.md` 中的全部路线图。

请按仓库 AGENTS.md 执行，中文汇报。默认自治推进：不要只停在计划讨论；能安全执行的读代码、写文档、修补、targeted 验证、总结、提交都直接做。只有破坏性、外部凭据/远程生产、或会改变长期方向的分叉决策才问我。

工作目录：`/root/oskernel2026-orays`
主计划：`docs/ltp-os-long-term-improvement-plan-2026-06-01.md`
会话文档根目录：`docs/ltp-os-long-term-plan-sessions-0601-docs/`

硬性目标：
1. 完成主计划中的 Session 1~8；如果 Session 8 后仍未达到完成定义，再执行 Session 9~10 的扩展路线。
2. 最小完成：stable 从当前 live 基线推进到约 500；至少 2 个核心 lane 有真实语义修复并通过 RV/LA × musl/glibc；LA severe-blocker 有减少或可复现阻塞报告；final report 和下一轮 prompt 写入 docs。
3. 理想完成：stable 达到 520 或更高；time/select、VFS/metadata、FD/fcntl/pipe 三条线均有可推广增量；LA-only blacklist 明显下降；新一轮 full-sweep 或 shard sweep 闭合且 panic/trap/incomplete/resource failure 为 0。
4. 每完成一个 roadmap session，必须提交一次独立 Git commit；不要把多个 roadmap sessions 混成一个 commit。
5. 每个 roadmap session 的所有相关文档必须放在 `docs/ltp-os-long-term-plan-sessions-0601-docs/session-XX-<slug>/` 子目录中。

开始前必须读取：
- `AGENTS.md`
- `docs/agent-workflow/repo-basics.md`
- `docs/agent-workflow/commands-and-validation.md`
- `docs/agent-workflow/ltp-selection.md`
- `docs/agent-workflow/ltp-promotion-and-docs.md`
- `docs/agent-workflow/coding-boundaries.md`
- `docs/agent-workflow/collaboration-and-delivery.md`
- `docs/agent-workflow/branch-policy.md`
- `docs/ltp-os-long-term-improvement-plan-2026-06-01.md`

基线事实（必须 live 复核，不要只依赖本提示词）：
- 提示词创建时 live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 为 `460 total / 460 unique / 0 duplicate`。
- 主计划记录的 stable460 归档门禁：RV/LA × musl/glibc 共 `PASS LTP CASE 920`、`FAIL 0`；已知 caveat 为 `read02 TCONF`，无 timeout/ENOSYS/panic/trap。
- full sweep 闭合入口：
  - `docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-report.md`
  - `docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/rv-arch002-summary.json`
  - `docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/la-arch012-summary.json`
- blacklist 只代表 severe-blocker 排除，不计 PASS，不作为 stable promotion 证据。

预检：
1. `pwd` 必须是 `/root/oskernel2026-orays`。
2. `git status --short`：识别既有用户/他人改动；只修改、暂存、提交自己负责的文件，绝不回滚无关改动。
3. 长跑/QEMU/Docker/evaluator 前后运行 `df -h / /root`；涉及 Codex/OMX 缓存清理时再看 `du -sh /root/.codex`。
4. live stable 计数命令：
   ```bash
   python3 - <<'PY'
   from pathlib import Path
   import re
   text = Path('examples/shell/src/cmd.rs').read_text()
   start = text.index('const LTP_STABLE_CASES')
   end = text.index('];', start)
   cases = re.findall(r'"([^"]+)"', text[start:end])
   print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
   PY
   ```
5. 所有 LTP 结果以 `python3 -B scripts/ltp_summary.py <log>` 或 JSON summary 为准；不要只看 wrapper exit code 或肉眼扫日志。

推荐 orchestration：
- 使用 Leader-owned Ultragoal + Team。
- Leader 维护 `.omx/ultragoal`、roadmap session 边界、stable list、最终 promotion gate、最终 commit。
- Team workers 只做窄 lane：discovery / source diagnosis / small fix / targeted verification / report；worker 不得自行推广 stable，不得直接改最终 stable list，除非 leader 明确分配且回收复核。
- 如果 tmux/pane/资源受限，优先 `omx team 5:executor`，失败再降到 4；不要让多个 worker 并发争用同一个默认 QEMU/sdcard/qcow2。没有隔离镜像时，promotion/final gate 由 leader 串行。

Ultragoal 建议用法：
1. 在 `docs/ltp-os-long-term-plan-sessions-0601-docs/session-00-orchestration/` 写一个短 brief：`ultragoal-brief.md`，只包含目标、session 边界、门禁、提交规则、文档目录规则。
2. 创建/刷新 goals：
   ```bash
   omx ultragoal create-goals --force --brief-file docs/ltp-os-long-term-plan-sessions-0601-docs/session-00-orchestration/ultragoal-brief.md
   omx ultragoal status
   ```
3. 每个 roadmap session 完成后，用 session 报告和 commit SHA checkpoint；不要让 worker 维护 ultragoal ledger。
4. 最终完成后，只有所有 final gate 和文档/commit 都完成，才把 Codex goal 或 Ultragoal 标记 complete。

Team 建议启动：
```bash
tmux -V
test -n "$TMUX"
command -v omx
omx team 5:executor "Complete 2026-06-01 LTP OS long-term improvement plan with leader-owned promotion gates"
# 如果 pane/资源不足：
omx team 4:executor "Complete 2026-06-01 LTP OS long-term improvement plan with leader-owned promotion gates"
```

每个 roadmap session 的文档目录规则：
- Session 0（可选 orchestration）：`docs/ltp-os-long-term-plan-sessions-0601-docs/session-00-orchestration/`
- Session 1：`session-01-baseline-candidate-matrix/`
- Session 2：`session-02-time-select-signal/`
- Session 3：`session-03-fd-fcntl-pipe-ownership/`
- Session 4：`session-04-vfs-metadata-path/`
- Session 5：`session-05-mmap-mm-resource/`
- Session 6：`session-06-futex-process-ipc/`
- Session 7：`session-07-la-severe-blockers/`
- Session 8：`session-08-integration-final-gate/`
- Optional Session 9：`session-09-network-proc-syntheticfs/`
- Optional Session 10：`session-10-full-sweep-quality-audit/`

每个 session 子目录至少包含：
- `session-report.md`：目标、改动、证据、结论、风险、下一步。
- `targeted-cases.txt` 或 `targeted-cases.md`：本 session 跑过/候选/未跑 case，明确单位是 case，不要含糊成“组”。
- `validation.md`：命令、日志/summary 路径、parser 输出摘要、未验证项。
- `promotion-candidates.md` 或 `no-promotion-reason.md`：是否可推广 stable，以及原因。
- 如涉及 blacklist：`blacklist-change-report.md`，列出 severe-blocker 理由、来源、解除条件。
- 如修改代码：`abi-and-behavior-impact.md`，列出 syscall/errno/flag/struct layout/FD/signal/futex/mmap/用户指针 copy-in/out 等用户可见变化。
- 如有大 raw log：默认不要提交大日志；提交 summary、checksum、路径说明即可，除非用户明确要求提交 raw log。

每个 roadmap session 的完成协议：
1. 完成本 session 的主计划验收条件。
2. 更新本 session 子目录文档，写清楚：已完成、未完成、证据路径、下一 session 入口。
3. 如果有 promotion：必须确认 RV + LA × musl + glibc wrapper PASS，且 `scripts/ltp_summary.py` 无新增内部 `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`；不能依赖 blacklist/SKIP/status0。
4. 如果只是 discovery/report：明确“不修改 stable list”“无 promotion”。
5. 运行最小验证：至少 `git diff --check`；代码改动还需 targeted test + 相邻回归；长跑前后 `df -h / /root`。
6. `git status --short`，只 stage 本 session 自己的源码/文档/summary；不要 stage 无关删除、无关 untracked、大 raw log、`kernel-rv`/`kernel-la`、`sdcard-*.img`、`disk*.img`、`output*.md`、`*.log`、`.axconfig.toml`、`build/`、`target/`。
7. 立刻创建一个 Git commit。提交信息必须是 Lore 协议，例如：
   ```text
   Advance long-term LTP plan session 01 with parser-backed baseline

   Constraint: Session artifacts must live under docs/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix.
   Rejected: Promoting candidates from sweep/status0 evidence | promotion requires fresh RV/LA × musl/glibc parser-clean gates.
   Confidence: high
   Scope-risk: narrow
   Directive: Keep later sessions on separate commits and do not let workers self-promote stable cases.
   Tested: git diff --check; live stable count; parser-backed summary review.
   Not-tested: No targeted runtime gate in report-only baseline session.
   ```
8. 在 session 报告顶部记录 commit SHA。若 commit 前需要先写 SHA，可提交后补一个 follow-up amend 只更新该 session 报告，或在下一 session 的 handoff 中记录；不要因此跳过每 session 一 commit。

Roadmap session 任务边界：

Session 1：冻结基线 + 建立候选矩阵
- 目标：把 rv-arch002/la-arch012 full-sweep summary 转成可执行候选矩阵。
- 产物：`candidate-matrix-stable460-to-500plus.md`、第一批 20~40 targeted cases、baseline report。
- 禁止：不修改 stable list，不修改 blacklist。

Session 2：time/select/signal 第一批
- 目标：修复/分类 `getitimer01`、`ppoll01`、`select02` 及相关 poll/pselect/clock/nanosleep 小批。
- 关注：timeout 精度、剩余时间回写、EINTR、sigmask、pending signal、POLLNVAL/POLLERR/POLLHUP。
- 验收：targeted RV 先通过或真实失败分类；LA 复核后才列 promotion candidates。

Session 3：FD/fcntl/pipe/ownership
- 目标：扩大 fcntl/pipe/chown/access 类稳定面。
- 关注：FD_CLOEXEC、file status flags、shared offset、O_APPEND、fork 后 FD 继承、pipe EOF/SIGPIPE/nonblock/EINTR、权限语义。
- 验收：promotion 必须四路 clean；改 FD 继承/flags 必须跑相关 stable 回归子集。

Session 4：VFS/metadata/path
- 目标：stat/path/symlink/xattr/getdents 低风险 stable 增量。
- 关注：statx mask/flags/errno、xattr 最小兼容、statfs 字段、getdents64 d_off/d_ino/d_type、readlink/readlinkat、rename/link/unlink 边界。
- 禁止：不能 synthetic path 或 case-name hardcode。

Session 5：mmap/mm/resource 第一批
- 目标：减少 mmap/mm 失败，并为 LA severe blockers 的资源稳定性打基础。
- 关注：file-backed shared mmap writeback、msync、mprotect VMA split/merge、mincore、SIGSEGV/exit、资源高水位与 teardown。
- 验收：不引入新的 stable mmap 回归；LA 额外记录内存/allocator 指标或替代观测。

Session 6：futex/process/IPC
- 目标：处理高频 ENOSYS 和 wait/clone/futex/IPC 缺口。
- 关注：futex wait/wake timeout/EINTR/key、waitid/zombie/reparent、clone/exec 继承、SysV shm/sem/msg 最小真实模型。
- 禁止：不能用忙等掩盖 futex timeout/调度问题。

Session 7：LA severe-blocker 专项
- 目标：降低 LA-only blacklist 规模，先处理资源/allocator/network blockers。
- 每次只挑 1~3 个 severe blockers；单 case targeted 跑到能终止；移除 blacklist 前跑小型 all-minus-blacklist shard。
- 产物重点是 blacklist-removal report，不是 stable report；普通 FAIL 照实保留，不 promotion。

Session 8：整合、推广、回归与下一轮规划
- 目标：汇总前 7 个 sessions，运行最终 RV/LA × musl/glibc stable gate，整理 blacklist diff，产出 final report 和下一轮 prompt。
- 验收：优先 stable500；clean subset 足够则冲 stable520。final gate 不得有新增 TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap。

Optional Session 9：网络/socket 与 proc/syntheticfs 语义
- 目标：socket errno、poll readiness、network stress cleanup、/proc 字段真实性；以降低 LA network blacklist 和 synthetic shim 风险为主，不优先冲 stable。

Optional Session 10：full-sweep 再闭合与质量审计
- 目标：重新跑 RV/LA all-minus-blacklist 或 shard sweep，确认 incomplete_count=0、panic/trap=0、resource failure=0，blacklist 来源/跳过数/架构差异写清楚。

全局红线：
- 不 fake PASS。
- 不 hardcode LTP case/path/process name 或输出。
- 不修改 testsuite/evaluator 绕过失败。
- 不隐藏 TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap。
- blacklist/SKIP/status0 不是 PASS，也不是 promotion evidence。
- 普通失败不能因为报告好看而加入 blacklist。
- POSIX/Linux 可见语义必须真实；所有 syscall/errno/flag/ABI/FD/signal/futex/mmap/user pointer 行为变化都要报告。

停止/回滚条件：
- 发现 stable460 回归：立即停止扩张，先回滚或修复回归。
- 候选全部变成高风险大工程：保存最高可信 stableN、blocker report、下一轮 prompt，不硬冲。
- LA severe blockers 需要架构级 allocator/runtime 改造且超出本计划：写可复现阻塞报告并停止该 lane。
- 任何日志未闭合、被截断、marker glue、parser 不可信：不能用作 final promotion 证据。

最终交付：
- `docs/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md` 或 optional Session 10 下 final report。
- 最高可信 stableN 的 live count、RV/LA × musl/glibc parser summaries、raw/summary/checksum 路径。
- blacklist diff 与 severe-blocker 证据边界。
- code-review / cleanup / marker-prefix / noise check 报告。
- 下一轮 prompt（若未达到理想完成或仍有高 ROI blocker）。
- 最后一个 session commit SHA；每个已完成 session 都有独立 commit。
````
