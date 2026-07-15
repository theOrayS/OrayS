# 集成四个 OrayS 分支并建立长期治理

状态：`active`

开始日期：2026-07-15

基线：`921171ac1ef5c85ab5a7cd1882dd40e1471b79f0`（预检时的 `origin/main`）

工作分支：`integration/four-prs-20260715`

## 目标

按 PR1、PR2、统一测试套件、PR3 的顺序保留来源历史并进行语义集成，安装长期 workflow governance，在同一最终候选 HEAD 上完成 fail-closed 的 quick、baseline、RV official、LA official 和 full 门禁，经独立只读审查后，仅在远端基线未漂移且全部条件明确通过时安全推广到 `main`。

## 非目标

- 不 squash、重写或替换四个来源分支的历史。
- 不以 testcase 名称、固定路径、输入或评测环境特化生产实现。
- 不弱化断言、parser、重复身份检查，不扩大 blacklist，不隐藏失败或超时。
- 不修改官方 backing image，不提交 overlay、QEMU 输出或大体积报告。
- 不顺带升级依赖、工具链或重排无关代码。

## 不变量

- 依赖方向保持 `orays-linux-abi -> orays-linux -> arceos-shell`，其中箭头表示被上层依赖；首阶段 `UserProcess` 仍由 shell 持有。
- Linux/POSIX 可见行为、syscall 编号、ABI 布局、errno、FD 生命周期和两架构配置不能因冲突解决而漂移。
- `test/` 是测试、fixture、runner、官方评测、parser 和报告的唯一权威位置；根 `run-eval.sh` 只能是忠实薄包装。
- 每次 canonical verdict 必须来自干净且运行前后提交稳定的工作树；首次失败证据不得被重试覆盖。
- 镜像由 `RV_TESTSUITE_IMG` / `LA_TESTSUITE_IMG` 覆盖，默认从仓库父级工作区解析；backing image 只读。

## 阶段与状态

- [x] 验证目标仓库、远端、干净基线和四个来源 tip。
- [x] 创建 source mirror、ref inventory、备份 tag、完整 bundle 和外部 journal。
- [x] 显式 merge PR1：`aa9072df32e4ced0edc70009ad456d62810ef2f3`。
- [x] 显式 merge PR2：`acc6b604eb8132bec8a26900aeb8869fea5feebc`。
- [x] 显式 merge统一测试套件：`126e21a402dc773b1057fcb83f204d11b62d3a4b`。
- [x] 显式 merge PR3：`03269960bb440e45f6e97999c20532cb3977c9be`。
- [x] 完成并提交 workflow governance 安装：`764211c5c221d7c64d57a658eac05fe7c5cee38c`。
- [x] 构建并校验双 target QEMU 9.2.4；固定源码 SHA-256 与安装戳、两个 target 版本及二进制摘要均已复核。
- [x] 在 clean/stable `05b123266fe3695bc660c2cd281a56d2ac44ccea` 上完成首次 post-repair quick：45/45 PASS；随后首次 baseline 如实得到 50 PASS、6 FAIL、1 INFRA_ERROR。
- [x] 对首次 baseline 的七个 non-pass 完成窄修复与 dirty-worktree 定向复验；host/RV64/LA64 clippy、workspace unit、三架构 semantic-evidence shard 与 aggregate 均真实退出 0，但这些结果不是 canonical verdict。
- [x] 在 clean/stable `1c0e3ba0396fcd9d8dde2ef6bb1cfc34e32647f5` 上完成 canonical quick：45/45 PASS；同提交 baseline 如实保留 56 PASS、1 FAIL。
- [x] 对该 baseline 的 cargo 结果合同完成 fail-closed 聚焦修复：精确记账 identity-bound unittest、同名 `should_panic` 和 `--no-fail-fast` 选项，同时用反例证明额外 failure/panic/TCONF/TBROK/TFAIL/ENOSYS/timeout/unknown 仍非通过；并把唯一 ignored axns doctest 改为真实 2/2 执行。最终 dirty-worktree runner 回归 133/133，但不是 canonical verdict。
- [ ] 在最终候选 HEAD 上依次通过 quick、baseline、RV official、LA official 和 full。
- [ ] 完成独立只读 reviewer 审查，清零 blocker/major finding，并在必要修复后从头重跑门禁。
- [ ] 重新 fetch 并确认 `origin/main` 未从初始基线漂移。
- [ ] 安全推广/推送，或给出不夸大的 `BLOCKED` / `FAILED` 终态。

## 已知风险

- 统一套件首次 clean quick 在 suite merge commit 上真实失败：RR skipped-task aging 守卫及其 current-tree 单测均非 PASS。
- clean `05b12326` baseline 的七个 non-pass 已由 `1c0e3ba0` clean rerun 关闭；该次 rerun 又诚实暴露 workspace cargo 合同 FAIL。最先命中的是合法 `should_panic` 的原始 stderr；逐层重分类又揭示 `--no-fail-fast` 命令回显、嵌套 identity-bound unittest 成功块和一个真实 ignored doctest。当前修复均有正反例，但仍须提交后在新 clean/stable HEAD 重跑完整 quick/baseline，不能外推 dirty 结果。
- axfs 失败来自已过期测试仍依赖早先删除的固定 `/dev/foo/bar` 假节点。曾尝试在生产 `RootDirectory` 路由前全局 canonicalize 路径，但它会改变 `..` 穿越 mount 边界的既有语义，已完整撤回；最终只让测试使用真实 `/dev/zero`，没有恢复假能力或改变生产路由。
- 当前官方镜像的 BusyBox 计划含重复身份；若最终 official parser 仍拒绝，属于镜像/计划 blocker，不能弱化去重约束。
- RV 宿主默认 QEMU 为 6.2.0；required evidence 必须使用从固定源码构建并校验的双 target QEMU 9.2.4。
- official/full 单次运行时间长；中断目录不是 verdict，必须保留并明确标记。

## 验证合同

所有命令均使用首进程隔离形式，并写入新的 ignored evidence 目录：

```bash
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --list
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile quick --output-dir test/output/<head>-quick
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile baseline --output-dir test/output/<head>-baseline
RV_TESTSUITE_IMG="$RV_TESTSUITE_IMG" python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile official --arch rv --output-dir test/output/<head>-official-rv
LA_TESTSUITE_IMG="$LA_TESTSUITE_IMG" python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile official --arch la --output-dir test/output/<head>-official-la
RV_TESTSUITE_IMG="$RV_TESTSUITE_IMG" LA_TESTSUITE_IMG="$LA_TESTSUITE_IMG" python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile full --arch all --output-dir test/output/<head>-full-all
```

每个 verdict 必须检查 shell 退出码、`summary.json` 的 `planned == executed == completed`、全部 case 状态、起止提交和 provenance stability；缺失 summary 不构成 PASS。

## 关键决策

- 使用四个显式 no-ff merge commit，逐个保留来源父提交和冲突理由。
- PR1/PR2 通过窄边界适配与 feature-invariant spin lock 合并，不移动 `UserProcess`，不改变公共 pipe mutex 类型。
- unified suite 与 PR3 收敛到一个 manifest/runner；PR3 evidence 是 canonical runner 下的专用 adapter，不建立第二套顶层框架。
- governance 单独提交，避免把策略安装混入来源 merge。
- host 默认 clippy 仅排除无法在 x86 host target 表达的 `arceos-shell`；显式 RV64/LA64 clippy 仍覆盖该 crate。LA64 使用临时 PATH 指向经能力探针确认的 clang 21，不修改仓库工具链或系统默认 clang。
- cargo 合同只消去精确的 `--no-fail-fast` 选项 token，并只记账完整 identity-bound unittest 成功块与同名、定界且正文无任何 non-pass 标记的 `should_panic` 报告；原始日志不改，ignored、失配计数、额外状态/panic 和普通 failure 仍失败。
- axns 历史 doctest 不再以 `ignore` 绕过；示例改为宏的真实三参数形式，并在 host 上实际执行 2/2。
- 只有最终所有门禁和独立审查都明确 PASS 才允许推广；任何不可修复的外部输入问题保持 Draft/BLOCKED。

## 回滚

预集成 annotated tag 为 `backup/pre-four-prs-20260715`。完整 bundle 位于任务归档中，bundle SHA-256 为 `104e4cba9c782af6717910f7ea35e26f9f2a2bdcdf369157a9183d6f5f3b76d5`。`main` 在安全推广前保持不变；不得使用破坏性 reset 或 force push。
