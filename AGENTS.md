# AGENTS.md

本文件定义 OrayS 仓库的长期开发、测试、文档和审查规则，适用于仓库根目录及其全部子目录。
更深层目录中的 `AGENTS.md` 可以补充更严格的领域规则，但不得放宽本文中的测试诚信、评审合规、可追溯性和 Git 安全要求。

## 1. 项目目标与优先级

OrayS 是面向 RISC-V64 与 LoongArch64 的通用操作系统内核项目。所有工作按以下优先级决策：

1. 保持 Linux/POSIX 可见语义、ABI、errno、边界条件和安全边界正确。
2. 使用通用、可解释、可扩展的实现，不针对测例、文件名、路径、输入或运行环境特化。
3. 保持 RISC-V64 与 LoongArch64 的行为和验证强度尽可能一致。
4. 保证改动可测试、可复现、可审查、可由团队成员解释。
5. 在正确性和通用性成立后再进行性能优化。
6. 优先采用小而可逆的改动，避免无关重构和大范围格式化。

## 2. 必须先阅读的内容

开始任务前，至少阅读：

- 本文件及当前目录链中更具体的 `AGENTS.md`；
- `README.md`、根 `Cargo.toml`、`rust-toolchain.toml` 和 `Makefile`；
- `test/README.md`（存在时）；
- `docs/development-logs/README.md`；
- 与任务相关的架构、设计、测试、历史开发日志和已知限制；
- `.github/workflows/` 中与任务相关的 CI 配置。

复杂、跨模块或预计需要多个提交的任务，先在 `docs/plans/active/` 建立执行计划；计划应记录目标、非目标、阶段、风险、验证方法和重要决策，并在实施过程中持续更新。

## 3. 仓库职责边界

主要目录职责如下：

- `kernel/`：HAL、trap、内存、任务、同步、驱动、文件系统、网络和运行时。
- `api/`：公共 API、POSIX/Linux 边界和 feature 组合。
- `user/`：用户态入口、Linux ABI 兼容服务和评测执行入口。
- `ulib/`：用户库。
- `configs/`：平台及远程评测配置。
- `test/`：测试、fixture、runner、官方评测入口、结果分类和报告工具的唯一权威位置。
- `scripts/`：非测试通用构建和仓库辅助脚本；测试迁移完成后不得保留测试业务逻辑。
- `docs/development-logs/`：按 PR 保存的人工可读开发过程记录。
- `docs/plans/`：较大任务的执行计划。
- `docs/decisions/`：跨 PR、长期有效的架构决策记录。
- `docs/references/`：外部项目、论文、规范、许可证和代码借鉴记录。

不要直接修改生成物、官方镜像、构建产物或 `vendor/`，除非任务明确要求且修改理由已记录。

## 4. 开始开发前

在修改文件前：

1. 执行并记录 `git status --short --branch`、当前 `HEAD` 和基线分支/提交。
2. 检查是否存在他人未提交改动；不得覆盖、回滚或顺手整理这些改动。
3. 确认任务目标、非目标、涉及的能力域及验收条件。
4. 找到相关现有测试，并运行可执行的最小基线。
5. 在 `docs/development-logs/` 创建本 PR 的日志文件并写入基线，而不是在 PR 结束后补写。
6. 对重构或测试迁移，先盘点旧路径、引用者和行为，再使用 `git mv` 保留历史。

开发日志暂未取得 PR 编号时使用 `pr-draft`；取得编号后重命名为正式名称。

## 5. 实现规则

- 不改变与任务无关的 Linux 可见行为、ABI 布局、syscall 编号、errno 或架构配置。
- 通用代码不得根据测试程序名、二进制特征、固定路径、字符串、执行顺序或评测环境走特殊分支。
- 不得用伪造返回值、空实现、吞错、无条件成功标记或绕过真实进程、内存、文件系统、同步及权限机制来通过测试。
- 修复实现问题，不通过删除测试、弱化断言、增加无条件 skip、扩大 blacklist 或修改结果解析器把失败改成通过。
- 缺失依赖、缺失镜像、空测试集、parser 异常、panic、超时和未知状态必须显式失败或标记为阻塞，不能计为 PASS。
- 新功能和缺陷修复必须增加行为测试，优先覆盖成功路径、错误路径、边界条件、资源回收和并发交互。
- 架构专用代码放在明确的架构边界内；通用层出现架构分支时必须记录理由并验证两个目标架构。
- 不新增 `unsafe`，除非无法用现有安全抽象表达；新增或移动的 `unsafe` 必须说明不变量、调用者责任和测试依据。
- 不新增依赖、不升级工具链、不大范围更新 `Cargo.lock`，除非任务明确需要并在日志和 PR 中解释。
- 不进行与任务无关的批量格式化、重命名或目录整理。

## 6. 官方镜像和路径约定

官方镜像位于仓库父目录，且不进入 Git。典型布局：

```text
/root/OrayS/
/root/sdcard-rv.img
/root/sdcard-la.img
```

仓库代码和脚本不得写死 `/root/OrayS`。统一按以下方式解析：

```bash
export ORAYS_REPO_ROOT="$(git rev-parse --show-toplevel)"
export ORAYS_WORKSPACE_ROOT="${ORAYS_WORKSPACE_ROOT:-$(dirname "$ORAYS_REPO_ROOT")}"
export RV_TESTSUITE_IMG="${RV_TESTSUITE_IMG:-$ORAYS_WORKSPACE_ROOT/sdcard-rv.img}"
export LA_TESTSUITE_IMG="${LA_TESTSUITE_IMG:-$ORAYS_WORKSPACE_ROOT/sdcard-la.img}"
```

规则：

- runner 必须从自身位置或 Git 根解析仓库路径，不能依赖调用者当前工作目录。
- 本地特殊布局通过环境变量覆盖，不得修改已提交脚本中的默认路径。
- 官方镜像视为只读输入；运行时使用 qcow2 overlay 或临时副本，禁止原地写入。
- 镜像文件、overlay、raw console log 和大体积报告不得提交。
- `test/images/manifest.*` 只记录文件名、版本、来源日期和 SHA-256，不保存镜像本体。
- 每次官方测试记录实际镜像路径的规范化表示和 SHA-256；文档不得泄露无关的主机绝对路径。
- 找不到镜像或 QEMU 时返回环境错误/`BLOCKED`，PR 不得宣称完整测试通过。

## 7. 统一测试入口与结果语义

目标状态下，测试由 `test/run_suite.py` 和版本化 manifest 统一管理。稳定入口至少包括：

```bash
python3 test/run_suite.py --list
python3 test/run_suite.py --profile quick
python3 test/run_suite.py --profile baseline
python3 test/run_suite.py --profile official --arch rv
python3 test/run_suite.py --profile official --arch la
python3 test/run_suite.py --profile full --arch all
```

Profile 约定：

- `quick`：静态合规检查、runner/parser 单元测试和结构完整性检查。
- `baseline`：`quick` 加格式、Rust 单元测试、lint 和 RISC-V64/LoongArch64 构建。
- `official`：指定架构的完整官方镜像评测。
- `full`：`baseline` 加所选架构的官方评测。

每个 PR 在结束开发、标记 Ready for review 或请求合并前，必须运行：

```bash
python3 test/run_suite.py --profile full --arch all
```

该命令必须包含仓库 `test/` 中注册的全部适用测试及两个架构的官方测试。若统一 runner 尚未落地，必须运行并记录现有全部合规检查、Python 单元测试、Rust 单元测试、两个架构构建，以及：

```bash
RV_TESTSUITE_IMG="$RV_TESTSUITE_IMG" ./run-eval.sh rv
LA_TESTSUITE_IMG="$LA_TESTSUITE_IMG" ./run-eval.sh la
```

结果至少区分：`PASS`、`FAIL`、`ERROR`、`TIMEOUT`、`BLOCKED`、`SKIPPED`。只有所有计划执行项都得到明确 `PASS` 时，完整门禁才通过。重试可用于诊断，但不能覆盖首次失败；flake 必须作为缺陷记录和处理。

## 8. 开发日志

每个 PR 对应一个 Markdown 日志：

```text
docs/development-logs/
├── README.md
├── TEMPLATE.md
└── YYYY/
    └── MM/
        └── YYYY-MM-DD-pr-<number|draft>-<slug>.md
```

日志必须在开发过程中持续更新，并至少包含：

- PR、分支、作者/负责人、日期、基线和能力域；
- 背景、目标、非目标和验收标准；
- 基线命令、结果及已有失败；
- 设计方案、备选方案、重要决策和理由；
- 按时间或 checkpoint 记录的修改、调试现象、根因和解决方法；
- 关键代码/配置路径及对应提交；
- AI 使用披露；
- 外部参考、借鉴代码、许可证和人工改写情况；
- 每条最终验证命令、架构、退出码、结果、耗时和证据路径；
- 未解决风险、已知限制、后续工作和回滚方式；
- 最终人工复核人及“团队成员可解释”的确认。

不要把大体积原始日志提交到 `docs/`。原始输出保存到忽略的 `test/output/<run-id>/` 或 CI artifact；开发日志提交摘要、run ID、结果和必要的小段错误信息。

## 9. AI 与外部来源披露

使用 AI 辅助分析、设计、编码、测试或文档时，在本 PR 开发日志中记录：

- 工具和模型名称/版本（已知时）；
- 使用场景和交互摘要；
- 生成或显著影响的文件、函数、测试或文档范围；
- 人工修改、取舍和拒绝的建议；
- 验证方法、实际命令和负责验证的团队成员。

不得提交含凭据、隐私数据或无关内部信息的完整对话。AI 输出不能替代人工理解；PR 负责人必须能够独立解释关键算法、数据结构、并发不变量、ABI 和边界条件。

借鉴 Asterinas、ArceOS、Linux 或其他项目时，在 `docs/references/` 或对应开发日志中记录仓库/文档、版本或 commit、文件/概念、许可证、借鉴范围和 OrayS 的修改。需要保留 SPDX、版权或 NOTICE 时必须遵守。

## 10. 提交与 PR 规则

- 一个 PR 只解决一个清晰主题；发现无关问题时另开 issue/PR。
- PR 初始为 Draft；完整门禁、日志和复核完成后才可标记 Ready。
- 提交应按可审查的逻辑阶段组织，体现真实演进；不要制造无意义微提交，也不要把全部过程压成无法审查的单一巨型提交。
- 推荐提交前缀：`feat`、`fix`、`refactor`、`test`、`docs`、`ci`、`build`。
- 提交信息说明“为什么”和可见影响，不仅罗列文件。
- 未经明确授权，不 push、不 force-push、不创建/合并 PR、不发布版本、不改写主分支历史。
- 禁止 `git reset --hard`、`git clean -fdx`、破坏性 rebase、删除他人分支或覆盖远端提交。

PR 描述必须链接对应开发日志，并包含：问题与目标、设计摘要、能力域、风险、测试表、两个架构的官方结果、AI 使用、外部来源、已知限制和回滚方案。

## 11. 最终复核与完成定义

完成前必须：

1. 检查 `git diff --check`、完整 diff 和文件范围。
2. 确认没有测试特化、假成功、吞退出码、绝对机器路径、密钥或大体积产物。
3. 检查 Linux 语义、errno、ABI、资源回收、并发和两架构差异。
4. 运行受影响的定向测试，再运行 `full --arch all` 完整门禁。
5. 由未承担主要实现的人或独立只读 reviewer 检查最终 diff；所有 blocker/major 问题必须解决并重跑测试。
6. 更新开发日志、设计文档、测试 manifest、已知限制和 PR 描述。

只有以下条件全部满足才算完成：

- 验收标准逐项有代码、测试或文档证据；
- 所有适用测试均被发现并实际执行，没有空套件或漏注册；
- RISC-V64 与 LoongArch64 的完整官方测试均明确通过；
- 没有未解释的失败、超时、skip、环境阻塞或新增 `unsafe`；
- 开发日志、AI 披露和外部来源记录完整；
- 最终 diff 聚焦、可回滚、可由负责人独立解释。

若缺失官方镜像、工具、硬件或凭据，继续完成所有不依赖项，但将 PR 保持为 Draft，并把状态记为 `BLOCKED`；不得把“未运行”表述为“通过”。
