# Agent 优先的比赛开发规范

本文档是 `AGENTS.md` 的比赛执行层补充，面向会直接读仓库、改代码、跑验证、提交 PR 的 AI agent。目标是在 OSKernel 2026 时间紧迫的条件下，把大部分开发、诊断、验证和文档工作交给 agent，人类只保留必要的方向判断和最终闸门。

当本文档与仓库事实冲突时，优先级为：

1. 当前源码、`Makefile`、脚本、配置和实际评测结果；
2. 根目录 `AGENTS.md`；
3. `README.md` 与 `docs/developer-guide/`；
4. 本文档。

## 0. 核心目标

本仓库不是普通长期维护项目，而是比赛冲分项目。agent 的工作目标按顺序是：

1. 保住当前可用分数和可提交状态；
2. 用最小改动修复真实 Linux/POSIX 语义缺口；
3. 推广可回归保护的高价值 LTP case；
4. 保持远程提交产物 `kernel-rv` / `kernel-la` 可构建；
5. 让人类可以用最少时间判断是否合并。

任何 agent 都必须遵守：

- 不硬编码 testcase 名称、路径、进程名或输出制造通过；
- 不伪造 `TPASS`、wrapper PASS 或隐藏 `TCONF` / timeout / ENOSYS / panic；
- 不修改 testsuite 源码绕过失败；
- 不用大重构替代局部修复；
- 不声称运行过没有实际运行的验证。

## 1. 人类与 agent 的职责边界

### 1.1 人类只做这些事

人类默认只负责：

- 指定比赛方向：例如“优先提升 LTP syscall 分组”或“先保远程 LA 启动”；
- 提供 agent 无法访问的材料：官方分数截图、远程评测日志、sdcard 镜像路径、账号权限；
- 对高风险 PR 做最终合并决定；
- 在两个 agent 结论冲突时选择保守方案；
- 比赛截止前决定冻结窗口和最终提交版本。

人类不应该手动做常规代码搬运、格式化、日志汇总、case 统计、PR 模板填写、回归命令挑选。这些都应交给 agent。

### 1.2 agent 默认必须做这些事

每个开发 agent 接到任务后，默认要自行完成：

- 读取相关仓库文档和代码；
- 定位修改点；
- 阅读相关 LTP / evaluator 日志或源码证据；
- 制定最小补丁；
- 运行能证明当前 claim 的最小验证；
- 总结未运行验证和原因；
- 提交 commit 或创建 PR，除非用户明确禁止或 worktree 有不可分离的他人改动；
- 给出清晰的回滚点和风险说明。

## 2. Agent 启动检查

任何 agent 修改代码前都必须完成下面的仓库自检。不要要求人类替 agent 查这些信息。

### 2.1 必读文件

优先阅读：

```text
AGENTS.md
README.md
docs/developer-guide/README.md
docs/developer-guide/repository-map.md
docs/developer-guide/build-and-eval.md
docs/developer-guide/validation-matrix.md
docs/developer-guide/ltp-workflow.md
```

涉及具体子系统时，再读对应源码和已有阶段报告。

### 2.2 本地状态检查

在真实 checkout 中工作的 agent，应先检查：

```bash
git status --short
df -h / /root || true
```

长时间构建、QEMU、Docker、vendor、完整 evaluator 前后都要检查磁盘。发现 worktree 有无关改动时，agent 只能改、暂存、提交自己负责的文件，不能 revert 或覆盖他人改动。

### 2.3 事实来源

不要凭记忆判断仓库状态。必须从当前文件读取：

- 当前 stable LTP case 数量：读 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`；
- 构建入口：读 `Makefile`；
- 本地评测入口：读 `run-eval.sh`；
- 远程 LA 平台配置：读 `configs/remote-eval/` 和 Makefile 中的 `REMOTE_LA_PLAT_CONFIG`；
- 评测总结方式：读 `scripts/ltp_summary.py`。

## 3. 任务分级和 agent 处理策略

### P0：保分 / 修提交阻塞

定义：`make all` 失败，`kernel-rv` / `kernel-la` 缺失，RV/LA 启动失败，已知高分 case 大面积回退，远程提交明显不可用。

agent 行为：

- 立即缩小到最小可疑范围；
- 优先 revert 自己最近引入的高风险改动或做局部 hotfix；
- 必须报告哪个提交、文件或语义路径最可能导致回退；
- 验证至少覆盖失败入口；
- 人类只需判断是否立即合并或回滚。

### P1：明确加分修复

定义：已有日志或分数表显示某些 LTP / syscall / fs / process / signal / mmap case 未通过，修复点较明确。

agent 行为：

- 先读对应 LTP source / runtest entry / raw log；
- 只修真实 Linux/POSIX 语义；
- 跑 targeted case，再跑相邻回归 case；
- 如果 RV 与 LA 均受影响，最终至少说明两边验证状态；
- 通过后才建议加入 stable list 或推广到更大 batch。

### P2：高价值探索

定义：没有直接失败日志，但根据 score gap、runtest 数量、源码内部 case 数、相邻已通过 case 推测 ROI 高。

agent 行为：

- 先产出候选排序和预期收益，不直接大改；
- 选择一个最小 case 或最小 syscall 语义切口；
- 放在窄分支或独立 PR；
- 没有 targeted 证据前不得修改 stable list。

### P3：清理 / 重构 / 风格

比赛阶段默认不做。除非它直接解除 P0/P1 阻塞，否则 agent 应拒绝扩大范围，并把清理任务记录为赛后 TODO。

## 4. 推荐的 agent 工作循环

对每个任务，agent 使用下面循环：

```text
读取事实 -> 选择最小目标 -> 写补丁 -> targeted 验证 -> 相邻回归 -> 总结证据 -> PR/commit -> reviewer agent 复核 -> 人类合并
```

### 4.1 读取事实

agent 必须在报告里说明自己依据了哪些事实：

- failing case / score gap / raw log；
- 相关 LTP source 或 runtest entry；
- 相关 syscall、VFS、FD、task、signal、mmap、HAL 或 shell runner 代码路径；
- 当前验证命令和结果。

### 4.2 选择最小目标

一个任务只允许一个主目标。例如：

- 好：修复 `openat()` 的某个 flag / errno 行为；
- 好：修复 `waitid()` 的一个可见语义差异；
- 坏：同时重写 FD table、VFS path lookup 和 signal 处理；
- 坏：为了一个 case 改整个 shell runner 输出格式。

### 4.3 写补丁

补丁必须满足：

- 优先改真实语义所在层，而不是在 evaluator/shell 层包一层假象；
- 优先局部 helper，不做跨 `kernel/`、`api/`、`ulib/`、`examples/` 的无关重构；
- 新增 `unsafe` 时尽量缩小作用域，并在不明显处写 `// SAFETY:`；
- syscall / ABI / errno 变化必须在 PR 描述和最终报告中显式说明；
- 运行时路径避免无根据的 `unwrap()` / `expect()`。

### 4.4 验证

agent 按 `docs/developer-guide/validation-matrix.md` 选择最小但足够的验证。常用命令：

```bash
# 格式 / 静态检查
make fmt
make fmt_c
make clippy
make doc_check_missing
make unittest_no_fail_fast

# 远程提交产物
make kernel-rv
make kernel-la
make all

# 本地评测
./run-eval.sh rv
./run-eval.sh la

# LTP 日志总结
python3 scripts/ltp_summary.py output_rv.md
python3 scripts/ltp_summary.py output_la.md
python3 scripts/ltp_summary.py --promotion-candidates rv.log la.log
```

不能运行的验证必须如实写明原因，例如缺少 QEMU、sdcard 镜像、testsuite checkout、交叉工具链或时间窗口不足。

### 4.5 证据总结

所有 agent 完成报告必须包含：

```text
Files changed:
- path: reason

Behavior change:
- user-visible / syscall / errno / ABI change, or explicitly none

Validation run:
- command -> result

Validation not run:
- command -> reason

Risk:
- regression area
- rollback plan

Score relevance:
- affected LTP/evaluator cases or submission path
```

## 5. 多 agent 协作流程

为了让人类少做审查，默认采用“两 agent + 人类闸门”的协作模式。

### 5.1 Implementer agent

实现 agent 负责：

1. 诊断；
2. 修改；
3. 运行 targeted 验证；
4. 填写 PR 描述；
5. 标记风险和未运行验证。

PR 标题格式：

```text
<subsystem>: <short scoring-oriented change>
```

示例：

```text
posix: fix openat errno for invalid flags
fs: preserve fd offset semantics across dup
ltp: promote verified access regression cases
la: keep remote boot address map in submission build
```

### 5.2 Reviewer agent

复核 agent 不应该重写实现，除非发现明确问题。它负责：

- 检查 diff 是否越界；
- 检查有没有 fake pass / testcase hardcode；
- 检查 errno、ABI、用户指针、锁、生命周期、跨架构 cfg；
- 检查验证是否足以支撑 claim；
- 必要时补跑最小验证；
- 给出 `APPROVE` / `REQUEST_CHANGES` / `COMMENT`。

Reviewer agent 的结论模板：

```text
Review result: APPROVE | REQUEST_CHANGES | COMMENT

Blocking issues:
- ...

Non-blocking notes:
- ...

Validation inspected or run:
- ...

Human decision needed:
- yes/no, reason
```

### 5.3 人类闸门

人类合并前只检查 5 件事：

1. 这个 PR 是否对应 P0/P1/P2，而不是无关清理；
2. diff 是否只改了相关源码/文档，没有生成物、磁盘镜像、日志污染；
3. agent 是否明确列出实际验证和未验证项；
4. 是否没有违反 LTP 红线；
5. 是否有可接受的回滚方案。

通过则合并；有疑问则让 reviewer agent 继续追问或让 implementer agent 补证据。

## 6. 分支、提交与合并规则

### 6.1 分支命名

推荐：

```text
agent/p0-<area>-<short-name>
agent/p1-<area>-<short-name>
agent/p2-<area>-<short-name>
agent/docs-<short-name>
```

示例：

```text
agent/p1-posix-openat-errno
agent/p1-fs-dup-offset
agent/p0-la-remote-boot
agent/docs-workflow
```

### 6.2 提交信息

格式：

```text
<subsystem>: <imperative summary>
```

正文写清：

```text
Problem:
- what failed and where the evidence came from

Change:
- what was changed

Validation:
- commands actually run

Risk:
- known regression area and rollback plan
```

推荐 trailer：

```text
Agent-Implemented-by: <agent name or session>
Agent-Reviewed-by: <agent name or session>
Human-Decision: pending
```

如果工具链或仓库已有 `Signed-off-by` 习惯，可以同时保留。

### 6.3 合并策略

- 小 bugfix：允许 squash merge；
- 中等功能：保留少量逻辑 commit；
- 大范围行为变化：必须拆 PR；
- 冻结期只合并 P0 和低风险 P1；
- P2 探索不能直接进最终提交，除非已有 targeted + 回归证据。

## 7. LTP / score 工作规范

### 7.1 候选选择

agent 选择 LTP 候选时按 ROI 排序：

```text
priority_score = potential_score_or_case_count
               * relevance_to_existing_work
               * hidden_test_value
               / implementation_cost
               / regression_risk
```

优先：

- 已有 score gap 中 `pass < all` 的 case；
- 与已实现子系统相邻的 syscall / fs / process / signal / pipe / mmap case；
- 一个 LTP case 内部包含多个子 case、循环、variant 或 fork fan-out 的高收益目标；
- 能防隐藏测试的真实 Linux/POSIX 语义。

暂缓：

- 需要大规模 VM、scheduler、network、namespace、mount、ptrace、bpf、fanotify、inotify、quota、landlock、io_uring 等基础设施的低 ROI 目标；
- 只能通过 fake / special-case 变绿的目标。

### 7.2 修复前必须读的证据

修一个 LTP case 前，agent 应尽量读取：

- evaluator raw log；
- `scripts/ltp_summary.py` 输出；
- contest testsuite 的 runtest entry；
- 对应 `testcases/kernel/...` 源码；
- 仓库内 syscall / VFS / task / signal / mmap 实现。

如果 contest testsuite 不在当前环境，agent 必须说明缺失，不得假装读过。

### 7.3 推广 stable list 的条件

把 case 加入 `LTP_STABLE_CASES` 前，需要满足：

- targeted case 在相关 libc 分组下通过；
- 相关相邻回归 case 没有明显退化；
- RV / LA 状态已经说明；
- parser 没有隐藏 timeout、ENOSYS、panic/trap 等严重信号；
- PR 描述包含 raw log 或 summary 路径。

## 8. 高风险区域特别规则

### 8.1 `api/arceos_posix_api/`

这里是 Linux/POSIX ABI 边界。agent 修改时必须报告：

- syscall 号、返回值、errno、flag 或 struct layout 是否变化；
- raw user pointer 是否在 copy-in/copy-out 前被验证；
- 是否影响 FD、process、signal、futex、mmap、ELF loading；
- 是否可能同时影响 musl 与 glibc。

### 8.2 `examples/shell/`

这里是评测集成面，不是普通 demo。agent 修改时必须确认：

- 没有破坏 wrapper marker 格式；
- 没有隐藏内部 LTP 输出；
- case selection、timeout、cleanup 逻辑有明确理由；
- 修改不是为了让某个 case 名字假通过。

### 8.3 `kernel/arch/axhal/`、`kernel/runtime/axruntime/`、`kernel/task/axtask/`

这里影响启动、trap、调度、用户任务流。agent 修改时必须：

- 保留跨架构 cfg 和平台差异；
- 特别说明 RV 与 LA 影响；
- 先 build，再 QEMU / evaluator；
- 不把本地 LA 地址映射假设误用于远程提交配置。

### 8.4 `vendor/`、`cargo-home/`、`tools/bin/`

除非任务明确是离线构建或依赖闭包，否则不要改。必须改时，agent 需要验证：

```bash
make all
CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all
```

如果不能运行离线风格验证，必须说明。

## 9. 冻结期规则

比赛截止前进入冻结期后，agent 默认只能处理：

- P0：编译、启动、远程提交、明显回退修复；
- 低风险 P1：小范围真实语义修复，并且已有 targeted 证据；
- 文档化最终提交证据。

冻结期禁止：

- 大重构；
- 批量格式化；
- 新增低置信 stable case；
- 改 vendor / toolchain / 平台配置但没有完整验证；
- 改 shell runner 输出格式；
- 跨多个子系统同时改动。

人类可以用一句话启用冻结期：

```text
进入冻结期：只接受 P0 和低风险 P1，所有 agent PR 必须写回滚方案。
```

## 10. 人类给 agent 的最小任务模板

人类发任务时只需填这几项，不需要写实现方案：

```markdown
## Goal
提升/修复：<case、分组、远程提交路径或错误现象>

## Evidence
日志/分数/截图/失败命令：<路径或粘贴关键片段>

## Priority
P0 / P1 / P2

## Constraints
冻结期：是/否
必须保留：<不可动文件、不可回退提交等>

## Expected output
PR / commit / 诊断报告 / 验证报告
```

示例：

```markdown
## Goal
修复 statx 相关 LTP gap，优先真实 errno 和 struct 行为。

## Evidence
远程 RV 分数里 statx pass < all；本地日志见 docs/.../rv.log。

## Priority
P1

## Constraints
不要改 testsuite；不要大改 VFS path lookup。

## Expected output
一个 PR，包含 targeted LTP 证据和相邻 stat/open 回归说明。
```

## 11. Agent 交付模板

agent 最终回复或 PR 描述必须能让人类快速合并：

```markdown
## Summary
一句话说明修了什么。

## Score relevance
- Affected cases/groups:
- Expected score impact:

## Files changed
- `path`: why

## Behavior / ABI impact
- syscall / errno / ABI visible change:
- user-visible behavior change:

## Validation
- [x] command: result
- [ ] not run: reason

## Risk and rollback
- Risk:
- Rollback:

## Red-line check
- [x] no testcase-name hardcode
- [x] no fake TPASS / wrapper PASS
- [x] no testsuite source modification
- [x] no hidden timeout / TCONF / ENOSYS / panic
```

## 12. 最短可执行规则

如果时间极端紧张，所有 agent 至少遵守下面 8 条：

1. 先读 `AGENTS.md` 和相关开发者指南；
2. 每个任务只改一个真实语义问题；
3. 不碰无关生成物、镜像、日志和批量格式化；
4. 不硬编码 testcase，不伪造通过；
5. 修改 syscall / errno / ABI 必须明说；
6. 跑最小能证明 claim 的验证，不能跑就说明原因；
7. PR 里写清风险、回滚、未验证项；
8. 人类只看方向、红线、证据和是否合并。
