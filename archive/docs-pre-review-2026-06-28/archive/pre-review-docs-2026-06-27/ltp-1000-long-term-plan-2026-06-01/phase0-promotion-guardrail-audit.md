# Phase0 promotion guardrail and evidence policy audit

日期：2026-06-01
Worker：`complete-dev-1000ltp-c632b4a0/worker-1`
任务边界：report-only；不修改 stable list，不运行 QEMU，不 checkpoint Ultragoal。

## 1. 当前 promotion 基线

- live stable list source：`examples/shell/src/cmd.rs::LTP_STABLE_CASES`。
- 本轮复核结果：`506 total / 506 unique / 0 duplicates`。
- 长期计划入口：`docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md`。
- 最高可信历史交付：`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md`。
- stable506 final gate 形态：RV/LA 均 `PASS LTP CASE 1012`、`FAIL 0`、`ltp-musl 506/0`、`ltp-glibc 506/0`；timeout/ENOSYS/panic/trap 为 0；继承 caveat 是 `read02` 的 `TCONF 4`，必须持续显式披露。

## 2. Promotion gate：什么才允许进 `LTP_STABLE_CASES`

未来每个 case 进入 stable list 前，至少满足以下全部条件：

1. **live baseline 先行**：推广前重新统计 `LTP_STABLE_CASES` total/unique/duplicates，确认没有重复和意外回退。
2. **targeted proof 先行**：候选先用 targeted case batch 验证，不从 full-sweep、blacklist 或 smoke-only 输出直接 promotion。
3. **RV + LA x musl + glibc 四路 clean**：每个候选在 RV/LA 两架构、musl/glibc 两套 LTP runtime 下均 wrapper PASS。
4. **parser-backed clean**：`python3 -B scripts/ltp_summary.py <log>` 或同等 case matrix 解析必须证明没有新增 internal `TFAIL`、`TBROK`、`TCONF`、ENOSYS/not-implemented、timeout、panic、trap、日志未闭合或 bad marker-prefix。
5. **相邻回归保护**：至少覆盖相关 subsystem 的相邻高价值回归，例如 access/stat/pipe/signal/read/write/mmap/process/futex/FD/path/errno 组合。
6. **源码语义真实**：syscall、errno、flag、struct layout、FD、signal、futex、mmap、用户指针 copy-in/out 等变化必须是通用 Linux/POSIX 可见语义，而不是 case-name/path/process-name 特判。
7. **Leader-owned promotion**：Team worker 可以提交 discovery、source diagnosis、小修复、targeted summary；最终 stable list 编辑、milestone gate、Ultragoal checkpoint 和 promotion commit 必须由 leader 串行决策。

## 3. 不可计数证据

以下证据只能作为 scouting / backlog / blocker 分类，不能计为 stable PASS：

| 输入 | 不能计数的原因 |
| --- | --- |
| blacklist / `[CONTEST][LTP][SKIP]` | 只是隔离或跳过，未执行 PASS。 |
| status0 / wrapper exit 0 | wrapper 成功不等于 case 内部 clean。 |
| full-sweep 局部 `TPASS` 密度 | `TFAIL/TBROK/TCONF`、timeout 或 wrapper FAIL 仍是真失败。 |
| 单架构或单 libc PASS | 不满足 RV + LA x musl + glibc 四路门禁。 |
| `pass_with_tconf` / `read02` TCONF | 可作为已知 caveat 披露，不能描述为 internal-clean。 |
| timeout / ENOSYS / panic / trap | 均为 blocker 或 severe signal，不得转成 PASS/SKIP/TCONF。 |
| raw log 被截断、marker glue、缺少 `RUN_META` | parser 可信度不足，需重跑或做 marker audit。 |
| smoke-only `output_rv.md` / `output_la.md` | 可能只覆盖小集合；必须用当前目标 gate 解析确认。 |
| remote output 超过 1MB 被截断 | 只能说明远端日志不完整，不能证明 PASS 等价。 |

## 4. No-fake-pass 红线

从 `AGENTS.md` 与 `docs/agent-workflow/ltp-promotion-and-docs.md` 汇总，本计划必须继续执行以下红线：

- 不 hardcode LTP case name、path、process name、输出文本或 scorer marker。
- 不修改 LTP testsuite source 来让测试通过。
- 不修改 evaluator/runner 来绕过真实失败或隐藏失败。
- 不 fake-print `TPASS` / wrapper PASS。
- 不把真实 `TFAIL`、`TBROK`、`TCONF`、timeout、panic/trap、ENOSYS 包装成 SKIP/PASS。
- 不为了单个 case 破坏通用 Linux/POSIX 语义。
- 不让并发 worker 争用默认 QEMU/sdcard/qcow2 产物；没有隔离 artifact 时，leader 串行跑 promotion/final gate。

## 5. Milestone commit cadence

长期计划要求每新增 50 个可信 unique stable cases 创建一个独立 milestone commit。以当前 live baseline `506` 为例：

| Milestone | Target | 说明 |
| --- | ---: | --- |
| M01 | 556 | 首个 +50；低风险 VFS/FD/time 小批优先。 |
| M02 | 606 | 第二个 +50；继续 parser-clean 小批。 |
| M03 | 656 | process/signal/time/poll 第一轮。 |
| M04 | 706 | mmap/resource 与 user-memory 边界。 |
| M05 | 756 | futex/thread/IPC 稳定子集。 |
| M06 | 806 | network/proc/syntheticfs blocker 缩小后的小批。 |
| M07 | 856 | 跨 subsystem 回归加固。 |
| M08 | 906 | full-sweep severe blocker 继续下降。 |
| M09 | 956 | 长尾 hard cases 前的稳定收束。 |
| M10 | 1000 | final gate、质量审计、维护文档。 |

规则：

- 不把多个 50-case milestone 混成一个提交。
- 每个 milestone commit 必须包含源码/稳定列表变更、milestone 文档、parser summaries、raw/summary/checksum 路径、ABI/behavior impact、not-tested caveat。
- 如果有 P0 hotfix 或回滚，可以单独提交，但不能替代 milestone commit。
- Commit message 使用 Lore 协议，至少写清 `Constraint`、`Rejected`、`Confidence`、`Scope-risk`、`Directive`、`Tested`、`Not-tested`。

## 6. Worker evidence reporting contract

Worker 提交报告或修复时必须写清：

- task id、claim 状态、是否 report-only。
- changed files，仅限 worker-owned scope。
- live stable count：total/unique/duplicates。
- 候选 case 列表，单位必须是 LTP case，不是 syscall 名或日志片段。
- 运行命令、环境变量、arch/libc、timeout、raw log path、summary path、checksum/path retention policy。
- `scripts/ltp_summary.py` 输出摘要：PASS/FAIL、suite summaries、internal `TFAIL/TBROK/TCONF`、timeout、ENOSYS、panic/trap、incomplete、marker-prefix audit。
- 非 promotion 原因：如果证据是 blacklist/full-sweep/smoke/单架构/含 caveat，必须明确写 `not promotion proof`。
- subagent evidence：若 task 要求 parallel probe，completion result 必须单独一行写 `Subagent spawn evidence:` 或 `Subagent skip reason:`，否则 lifecycle gate 会拒绝完成。
- not-tested：report-only 任务不得声称 build/QEMU/evaluator coverage。

## 7. 本 audit 的验证

- 复用 task-1 的 live stable count 复核：`506 total / 506 unique / 0 duplicates`。
- 读取并引用 `AGENTS.md`、`docs/agent-workflow/ltp-selection.md`、`docs/agent-workflow/ltp-promotion-and-docs.md`、`docs/agent-workflow/collaboration-and-delivery.md`。
- 未运行 QEMU、build、evaluator；本任务无代码改动、无 stable promotion。
- Subagent skip reason: task-2 delegation is optional and the audit is a narrow policy synthesis from already-read local docs; serial execution avoided redundant sidecar work.
