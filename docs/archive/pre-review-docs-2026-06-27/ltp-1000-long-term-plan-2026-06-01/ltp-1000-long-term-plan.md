# 长期计划：stable506 -> 1000 LTP cases，兼顾健壮性与可维护性

创建日期：2026-06-01
工作分支：`dev/1000ltp-plan`
文档根目录：`docs/ltp-1000-long-term-plan-2026-06-01/`
当前 live baseline：`506 total / 506 unique / 0 duplicate`

## 1. 目标定义

本计划的“通过 1000 LTP”定义为：`examples/shell/src/cmd.rs::LTP_STABLE_CASES` 中至少 1000 个 unique case，在 RV + LA × musl + glibc 四路 gate 中均 wrapper PASS，并且 `scripts/ltp_summary.py` 不报告新增 `TFAIL`、`TBROK`、`TCONF`、`ENOSYS`、timeout、panic、trap 或日志未闭合问题。

不计入 1000 的证据：

- blacklist / `[CONTEST][LTP][SKIP]` / status0；
- full sweep 中局部 `TPASS` 但 wrapper FAIL/TBROK/TCONF 的 case；
- 只在单架构、单 libc、或未解析 raw log 的 PASS；
- 任何靠 case 名、路径、进程名、输出字符串或 evaluator 绕过制造的 PASS。

成功标准分三级：

| 级别 | 标准 |
| --- | --- |
| 最低可交付 | stable >= 800，核心语义 lane 有真实修复，full-sweep severe blocker 明显下降，所有新增 case 有四路 parser-backed gate |
| 主要目标 | stable >= 1000，最终 RV/LA × musl/glibc stable gate clean；没有新增 timeout/ENOSYS/panic/trap；blacklist 只保留真实 severe blocker |
| 理想目标 | stable >= 1000 且一轮 all-minus-blacklist 或 shard sweep 闭合，`incomplete_count=0`，panic/trap/resource failure=0；补齐维护文档和回归矩阵 |

## 2. 当前事实与证据基线

必须每轮 live 复核，不能只信本文档：

```bash
pwd
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
git status --short
```

已知基线：

- 上轮 final gate：stable506，RV/LA × musl/glibc 均 `PASS LTP CASE 1012`、`FAIL 0`。
- parser-visible caveat：继承的 `read02 TCONF`；后续应优先真实修复或继续显式披露。
- active blacklist：common `5`、RV `1`、LA `374`；blacklist 只用于隔离 severe blocker，不是 promotion 证据。
- full-sweep/scouting 默认语义应保持 stable-first superset：`LTP_STABLE_CASES + (all guest LTP binaries - stable - active blacklist)`；最终 stable gate 仍使用 `LTP_CASES=stable`。

## 3. 治理原则：先稳再多

1. **真实语义优先**：syscall、errno、flag、struct layout、FD、signal、futex、mmap、用户指针 copy-in/copy-out 变化必须符合 Linux/POSIX 可见语义，并在报告中写清。
2. **小批量推广**：候选先 targeted RV，再相邻回归，再 LA 复核；只有四路 clean 才能进 stable。
3. **每 50 个新增可信 case 一个 milestone commit**：从 live baseline 动态计算 `baseline + 50*k`；当前若 baseline 为 506，则 milestone 是 `556, 606, 656, 706, 756, 806, 856, 906, 956, 1000`。最终 1000 是最后一个不足 50 的收口 milestone。
4. **不把多个 50-case milestone 混成一个提交**：每个 milestone commit 包含对应源码、stable list、summary 文档和验证证据；如果存在 P0 hotfix 或回滚需要，可额外单独提交，但不得替代 50-case promotion commit。
5. **leader-owned gate**：Leader 维护 `.omx/ultragoal`、stable list、milestone 文档、最终 promotion；Team worker 只做 discovery / source diagnosis / small fix / targeted verification / report。
6. **可维护性硬门槛**：每个 lane 必须避免 speculative abstraction；优先复用现有工具和已有模式；涉及高风险模块时补写 invariants、回归集合和 rollback notes。

## 4. 总体路线图

### Phase 0：基线冻结与候选宇宙重建（506 -> candidate backlog）

目标：建立 1000 目标的事实底座，而不是盲目扩 stable list。

产物：

- `baseline-report.md`：live stable count、分支、commit、dirty worktree、磁盘状态；
- `candidate-backlog.md`：从 archived full sweep、stable506 final gate、LTP source/runtest、near-clean FAIL 中抽取候选；
- `risk-register.md`：LA-only blocker、资源/allocator/network blockers、read02 TCONF、已知高风险子系统；
- `regression-matrix.md`：已高分家族的最小回归集合。

### Phase 1：低风险 syscall/VFS/FD 扩展（目标 +100 到 +150）

优先 case 家族：`statx`、`access`、`chmod/chown`、`link/unlink/rename`、`readlink/readlinkat`、`getdents64`、`statfs`、`xattr`、`fcntl`、`pipe`、`readv/writev`、`sendfile`。

维护目标：把 VFS/metadata/FD 行为收敛成通用语义，不为单 case 写特殊分支。建立 shared-offset、O_APPEND、FD_CLOEXEC、pipe EOF/SIGPIPE/nonblock/EINTR 的回归矩阵。

### Phase 2：time/select/signal/process 基础健壮性（目标 +100 到 +150）

优先 case 家族：`select/pselect/ppoll/poll`、`clock_gettime`、`nanosleep`、`getitimer/setitimer`、signal mask/pending/delivery、`wait/waitid`、`fork/clone/exec`、rlimit、priority/scheduler query。

维护目标：消除“刚好过 case”的 timing hack；用统一 timeout/remaining-time 回写、EINTR、sigmask 进入/退出恢复语义，补足 fork 后 FD/signal/process state 继承规则。

### Phase 3：mmap/mm/resource 与用户内存边界（目标 +100 到 +150）

优先 case 家族：`mmap/mmapstress` 的低风险子集、`mprotect`、`msync`、`mincore`、file-backed shared writeback、VMA split/merge、SIGSEGV/exit teardown、resource high-water。

维护目标：建立用户指针验证、VMA 生命周期、页权限、file-backed dirty/writeback、LA allocator/resource telemetry 的统一模型。不得用静默成功或空实现伪装 Linux 行为。

### Phase 4：futex/thread/IPC 与并发语义（目标 +100）

优先 case 家族：futex wait/wake/timeout/EINTR/key、robust list 可行子集、SysV shm/sem/msg 最小真实模型、pipe/socket 与 task teardown 交互。

维护目标：不能用忙等掩盖 futex timeout 或调度问题；所有共享对象必须有清晰 lifetime、引用计数、唤醒路径和退出清理策略。

### Phase 5：network/socket/proc/syntheticfs 与 LA severe blocker 削减（目标 +100 到 +150）

优先 case 家族：socket errno、bind/listen/connect/accept 基础、poll readiness、shutdown/close teardown、UNIX/TCP/UDP 小批、`/proc` 字段真实性、syntheticfs consistency。

维护目标：先降低 severe blocker 与资源泄漏，再推广 stable。网络和 proc/syntheticfs 不允许只写测试名 shim；必须描述模型边界和 unsupported capability 的真实 errno。

### Phase 6：长尾 hard cases、full-sweep 质量闭合与 1000 final gate

目标：从 stable956 收口到 stable1000，并完成最终质量审计。

验收：

- stable1000 四路 clean；
- all-minus-blacklist 或分片 sweep 至少覆盖新增 lane，`incomplete_count=0`，panic/trap/resource failure=0；
- blacklist diff 有 severe-blocker 证据和解除条件；
- code-review / cleanup / marker-prefix / noise check 报告闭合；
- 下一轮维护 prompt 和 post-1000 roadmap 写入 docs。

## 5. Milestone cadence 与提交规则

从每轮 live baseline 计算 milestone。以当前 `506` 为例：

| Milestone | Stable target | 新增 unique cases | 主要目标 |
| --- | ---: | ---: | --- |
| M01 | 556 | +50 | 低风险 clean/near-clean 候选；先扩 VFS/FD/time 小批 |
| M02 | 606 | +100 | VFS/metadata/path + fcntl/pipe 基础收口 |
| M03 | 656 | +150 | process/signal/time/poll 第一轮 |
| M04 | 706 | +200 | mmap/mm/resource 第一轮 |
| M05 | 756 | +250 | futex/thread/IPC 第一轮 |
| M06 | 806 | +300 | network/socket/proc/syntheticfs 低风险子集 |
| M07 | 856 | +350 | LA severe blocker 削减 + shard sweep |
| M08 | 906 | +400 | 高 fan-out syscall/mm/fs 长尾 |
| M09 | 956 | +450 | full-sweep gap driven hardening |
| M10 | 1000 | +494 | final gate、质量审计、维护文档 |

每个 milestone 子目录建议命名：

```text
docs/ltp-1000-long-term-plan-2026-06-01/milestone-XX-stableNNN/
```

每个 milestone 至少包含：

- `milestone-report.md`：目标、改动、证据、结论、风险、下一步；
- `targeted-cases.txt`：跑过/候选/未跑 case，单位必须是 case；
- `validation.md`：命令、日志/summary 路径、parser 输出摘要、未验证项；
- `promotion-candidates.md` 或 `no-promotion-reason.md`；
- `abi-and-behavior-impact.md`：如修改代码，写 syscall/errno/flag/ABI/FD/signal/futex/mmap/user pointer 行为变化；
- `blacklist-change-report.md`：如涉及 blacklist，写 severe-blocker 理由、来源、解除条件；
- `regression-matrix.md`：本 milestone 保护的已通过 case 与相邻回归集合。

## 6. 候选选择与验证闸门

候选排序公式：

```text
priority = case_count_or_score_yield
         * hidden_test_value
         * reuse_of_existing_semantics
         / implementation_cost
         / regression_risk
         / resource_risk
```

Promotion gate：

1. targeted RV 先跑通，并解析 raw log；
2. 修复真实语义，不能 case-name/path hardcode；
3. 跑相邻回归子集，尤其 access/stat/pipe/signal/read/write/mmap/process/futex；
4. LA 复核；
5. musl + glibc 均 clean；
6. `scripts/ltp_summary.py` 无新增内部失败、timeout、ENOSYS、panic/trap；
7. 更新 stable list、milestone 文档和 ABI impact；
8. 达到下一个 50-case milestone 后提交。

## 7. 工程健壮性与可维护性专项

这些专项不直接等同于冲分，但会决定 1000 是否可信：

- **用户指针与 ABI 审计**：建立 copy-in/copy-out helper 使用规范，消除 unchecked raw pointer；
- **错误码一致性**：对 common syscall 建立 errno matrix，避免 -1/ENOSYS/TCONF 混淆；
- **生命周期与资源回收**：task exit、FD close、mmap unmap、socket close、SysV IPC remove、futex waiter cleanup；
- **跨架构一致性**：RV/LA boot、地址空间、allocator、trap、timer 差异必须显式记录；
- **runner/marker 可信度**：wrapper marker prefix、RUN_META、parser summary、raw-log checksum 固化；
- **回归速度**：维护 stable smoke 子集、lane-specific regression subset、final full gate 三层验证。

## 8. 停止/降级条件

- stable506 或任一已推广 milestone 出现回归：停止扩张，先修复或回滚本轮引入风险；
- 候选变成架构级大工程：保存 blocker report，不硬冲 50；
- LA resource/allocator/network blocker 需要超出计划的重构：写可复现阻塞报告，转下一轮专题；
- raw log 截断、marker glue、parser 不可信、RUN_META 缺失：该证据不得用于 promotion；
- 出现 fake pass、case hardcode、evaluator bypass 风险：立即撤回该补丁并写审计说明。

## 9. 最终交付

- `stable1000-final-report.md`：最高可信 stable count、四路 parser summaries、raw/summary/checksum 路径；
- `blacklist-diff-final.md`：blacklist 变化与 severe-blocker 边界；
- `robustness-and-maintainability-review.md`：代码结构、ABI、资源生命周期、回归矩阵、已知风险；
- `post-1000-roadmap.md`：1000 后继续提升与内核健壮性方向；
- 最后一个 milestone commit SHA；每个 50-case milestone 都有独立 Lore commit。
