# 启动提示词：dev/1000ltp-plan 达到 1000 个可信 LTP stable cases

创建日期：2026-06-01
目标仓库：`/root/oskernel2026-orays`
目标分支：`dev/1000ltp-plan`
计划目录：`docs/ltp-1000-long-term-plan-2026-06-01/`
主计划：`docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md`
Ultragoal brief：`docs/ltp-1000-long-term-plan-2026-06-01/ultragoal-brief.md`

把下面 fenced block 作为下一次 Codex/OMX 会话的第一条消息。

````text
我要执行 `/root/oskernel2026-orays` 的 1000 LTP 长期完善任务：在 `dev/1000ltp-plan` 上把当前 stable baseline 推进到至少 1000 个可信 unique LTP stable cases，同时提升内核健壮性和可维护性，不做单纯刷分式 fake pass。

请按仓库 AGENTS.md 执行，中文汇报。默认自治推进：能安全执行的读代码、写文档、修补、targeted 验证、总结、提交都直接做。只有破坏性、外部凭据/远程生产、或会改变长期方向的分叉决策才问我。

工作目录：`/root/oskernel2026-orays`
目标分支：`dev/1000ltp-plan`
计划目录：`docs/ltp-1000-long-term-plan-2026-06-01/`
主计划：`docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md`
Ultragoal brief：`docs/ltp-1000-long-term-plan-2026-06-01/ultragoal-brief.md`

硬性目标：
1. 最终 `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 达到至少 1000 total / 1000 unique / 0 duplicate。
2. 1000 个 stable cases 必须 RV + LA × musl + glibc wrapper PASS，并由 `scripts/ltp_summary.py` 证明无新增 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap；任何已知 caveat 必须显式报告。
3. 不只冲分：每个语义 lane 都要记录 ABI/POSIX 可见影响、回归集合、资源/lifetime 风险和维护边界。
4. 每新增 50 个可信 unique LTP stable cases 必须创建一个独立 milestone Git commit；从 live baseline 动态计算目标。若 baseline 仍为 506，则 milestones 为 556、606、656、706、756、806、856、906、956、1000。
5. 每个 milestone 的文档必须放在 `docs/ltp-1000-long-term-plan-2026-06-01/milestone-XX-stableNNN/` 独立子目录。

必须先读取：
- `AGENTS.md`
- `docs/agent-workflow/repo-basics.md`
- `docs/agent-workflow/commands-and-validation.md`
- `docs/agent-workflow/ltp-selection.md`
- `docs/agent-workflow/ltp-promotion-and-docs.md`
- `docs/agent-workflow/coding-boundaries.md`
- `docs/agent-workflow/collaboration-and-delivery.md`
- `docs/agent-workflow/branch-policy.md`
- `docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md`
- `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md`

预检：
```bash
pwd
git branch --show-current
git status --short
df -h / /root
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

Ultragoal + Team 启动方式：
1. Leader 先创建/刷新 durable goals：
   ```bash
   omx ultragoal create-goals --force --brief-file docs/ltp-1000-long-term-plan-2026-06-01/ultragoal-brief.md
   omx ultragoal status
   ```
2. 确认 Team runtime 前置条件：
   ```bash
   tmux -V
   test -n "$TMUX"
   command -v omx
   ```
3. 启动 Team；资源不足时从 5 降到 4，不要让多个 worker 争用同一个默认 QEMU/sdcard/qcow2：
   ```bash
   omx team 5:executor "Complete dev/1000ltp-plan stable baseline to 1000 trustworthy LTP cases with leader-owned promotion gates"
   # fallback:
   omx team 4:executor "Complete dev/1000ltp-plan stable baseline to 1000 trustworthy LTP cases with leader-owned promotion gates"
   ```
4. Leader 维护 `.omx/ultragoal/goals.json`、`.omx/ultragoal/ledger.jsonl`、stable list、milestone gate、final report；workers 只负责窄 lane 的 discovery/source diagnosis/small fix/targeted verification/report。worker 不得自行推广 stable，不得直接 checkpoint Ultragoal。
5. 每个 milestone 完成后，leader 用 fresh `get_goal` snapshot 和 evidence 运行 `omx ultragoal checkpoint`；最终 goal 只有 code-review/cleanup/noise/final gate 都 clean 后才能 complete。

Milestone 文档要求：
- `milestone-report.md`：目标、改动、证据、结论、风险、下一步。
- `targeted-cases.txt`：本 milestone 跑过/候选/未跑 case，明确单位是 case。
- `validation.md`：命令、日志/summary/checksum 路径、parser 输出摘要、未验证项。
- `promotion-candidates.md` 或 `no-promotion-reason.md`。
- `abi-and-behavior-impact.md`：代码改动的 syscall/errno/flag/ABI/FD/signal/futex/mmap/user pointer 影响；无变化也写明。
- `blacklist-change-report.md`：如涉及 blacklist，列 severe-blocker 理由、来源和解除条件。
- `regression-matrix.md`：本 milestone 保护的相邻回归集合。

Promotion gate：
1. targeted RV 通过并由 `scripts/ltp_summary.py` 解析；
2. 相邻 stable regression 子集不退化；
3. LA 复核通过；
4. musl + glibc 均 wrapper PASS；
5. parser 无新增 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap；
6. blacklist/SKIP/status0/full-sweep 局部 TPASS 不计入 promotion；
7. 更新 stable list 和 milestone 文档；
8. 达到下一个 50-case milestone 后创建独立 Lore commit。

推荐 lane 优先级：
1. VFS/metadata/path：statx、access、chmod/chown、link/unlink/rename、readlink/readlinkat、getdents64、statfs、xattr。
2. FD/fcntl/pipe/io：FD_CLOEXEC、file status flags、shared offset、O_APPEND、pipe EOF/SIGPIPE/nonblock/EINTR、readv/writev/sendfile。
3. time/select/signal/process：pselect/ppoll/poll、clock/nanosleep/itimer、signal mask/pending/delivery、wait/waitid、fork/clone/exec、rlimit/priority。
4. mmap/mm/resource：file-backed shared mmap、msync、mprotect VMA split/merge、mincore、SIGSEGV/exit teardown、LA allocator/resource telemetry。
5. futex/thread/IPC：futex wait/wake/timeout/EINTR/key、SysV shm/sem/msg、task teardown 和 wakeup lifetime。
6. network/socket/proc/syntheticfs：socket errno/readiness/teardown、UNIX/TCP/UDP 小批、`/proc` 字段真实性、syntheticfs consistency。
7. full-sweep quality：stable-first superset scouting，即 `stable + (all guest LTP binaries - stable - active blacklist)`；不得把 blacklist/SKIP 当 PASS。

红线：
- 不 fake PASS。
- 不 hardcode LTP case/path/process name 或输出。
- 不修改 testsuite/evaluator 绕过失败。
- 不隐藏 TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap。
- 不把普通 FAIL 加 blacklist 只为报告好看。
- 不为单 case 破坏通用 Linux/POSIX 语义。
- raw log 截断、marker glue、parser 不可信、RUN_META 缺失的证据不得用于 promotion。

提交规则：
- 每新增 50 个可信 unique stable case 创建一个独立 milestone commit；不要把多个 50-case milestone 混在一个 commit。
- 只 stage 自己负责的源码、stable list、summary 和 milestone docs；不要 stage 无关脏文件、raw logs、镜像、`kernel-rv/kernel-la`、`target/`、`build/`。
- Lore commit 示例：
  ```text
  Reach stable556 with parser-clean VFS and FD promotions

  Constraint: Promotion requires RV/LA x musl/glibc parser-clean evidence and milestone docs under docs/ltp-1000-long-term-plan-2026-06-01/milestone-01-stable556.
  Rejected: Counting blacklist or status0 sweep evidence | only four-way clean stable gates count toward the 1000 target.
  Confidence: high
  Scope-risk: moderate
  Directive: Keep subsequent promotion commits at the next 50-case milestone and preserve leader-owned stable gates.
  Tested: git diff --check; targeted RV/LA summaries; stable regression subset; final stable556 gate summaries.
  Not-tested: Full all-minus-blacklist sweep beyond documented shard.
  ```

停止/降级条件：
- 已推广 stable 出现回归：停止扩张，先修复或回滚。
- 候选全部变成架构级大工程：保存 blocker report，不硬冲 50。
- LA severe blockers 需要超出计划的 allocator/runtime 重构：写可复现阻塞报告，转专题。
- 任何 evidence 不闭合：不能用于 final promotion。

最终交付：
- stable1000 final report。
- RV/LA × musl/glibc parser summaries、raw/summary/checksum 路径。
- blacklist diff 与 severe-blocker 证据边界。
- robustness-and-maintainability review。
- post-1000 roadmap。
- 每个 50-case milestone 的独立 commit SHA。
````
