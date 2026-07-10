# Agents Guidelines for ArceOS / OSKernel 2026

<a id="canonical-project-identity"></a>
## 项目定位

本项目是**基于 ArceOS、面向 Linux/POSIX 用户态兼容的模块化操作系统内核**。保留模块化、可裁剪和多架构能力；不宣称是完整 Linux 或无限范围的通用内核。决赛阶段默认质量优先、分数兜底：官方测例和分数是兼容性回归证据，不是架构目标。

<a id="quality-priority-order"></a>
## 六级质量顺序

1. 正确性与诚实语义：通用行为、错误路径、安全边界、失败可见性。
2. 鲁棒性：资源检查、异常恢复、并发/内存安全、跨条件稳定性。
3. 架构完整性与可维护性：清晰边界、可解释设计、可测试和可审查实现。
4. 性能与资源效率：必须建立在真实语义和可复现证据之上。
5. 实验创新：必须经过隔离、指标、回滚、维护性评审和晋升门。
6. 官方分数与既有测例：作为回归底线，不作为默认路线图或任务排序器。

<a id="bounded-regression-contract"></a>
## Bounded regression 合同

通用语义或架构修复若造成临时回退，进入集成线前必须记录：baseline 与采集时间、量化 delta 与影响范围、通用正确性证据、repo-visible rationale、负责人、期限、rollback/containment、完整失败可见性，以及 release 恢复旧基线或经审查采用等价新基线的闭合方式。过期、无主、无回滚或隐藏失败的例外立即成为 blocker；本合同不得豁免安全、数据完整性、权限/资源检查或真实兼容语义。

<a id="always-on-invariants"></a>
## Always-on 不变量

- 默认工作根目录是 `/root/oskernel2026-orays`；除非任务明确跨仓库，否则不在外层目录改动。
- 假设 worktree 已有他人改动；只修改、暂存、提交任务拥有的文件，不覆盖或 revert 无关改动。
- 只做任务所需的最小改动；不顺手大重构、批量格式化、机械重命名或跨子系统清理。
- 除非任务明确拥有，不编辑、删除或提交 `kernel-rv`、`kernel-la`、`sdcard-*.img`、`disk*.img`、`output*.md`、`*.log`、`.axconfig.toml`、`build/`、`target/` 等生成物或本地证据。
- 不允许 fake pass：不伪造 `TPASS`/wrapper PASS，不修改 testsuite/evaluator 绕过失败，不隐藏 `TFAIL`、`TBROK`、`TCONF`、timeout、`ENOSYS`、panic 或 trap。
- `self-check.md` 的通用语义、基本安全和竞赛合规红线始终有效；分数、性能或兼容性捷径不得弱化它们。
- 不按测试程序名或二进制特征特殊判断。
- 不对特定 syscall 参数、输入数据或目录结构硬编码结果或分支。
- 不只实现高分集合所需的局部行为而故意忽略通用 Linux/POSIX 语义。
- 不利用评测环境差异、固定路径、固定时间或固定顺序投机。
- 不为过测破坏 Linux syscall 兼容语义或内核基本安全边界。
- 不以非通用方式绕过真实的进程、内存、文件系统或同步机制。
- 不以猜测测例、适配测例或硬编码测例为实现目标。
- 不牺牲 Linux syscall 主要兼容语义换取速度。
- 不跳过必要的权限检查、资源检查或错误处理。
- 不引入在隐藏条件下可能导致内核崩溃或数据错误的实现。
- 不采用只对固定测例有效、对一般程序无效的优化。
- 不采用其他不符合正确性、通用性和可解释性的优化。
- Blacklist 只可隔离会卡死、耗尽资源、破坏评测环境或明确不适用当前内核模型的 case；skip 不算 pass，也不能作为 stable/promotion 证据。
- Linux/POSIX 可见语义必须真实；syscall、errno、flag、struct layout、FD、signal、futex、mmap、用户指针 copy-in/copy-out 的变化必须显式报告。
- 新增依赖或修改 `vendor/`、`cargo-home/`、`tools/bin/`、远程配置、平台配置或架构启动路径，必须有明确任务理由和对应验证。
- 长构建、QEMU、Docker、vendoring、完整 evaluator 前后执行磁盘门；涉及 Codex/OMX 缓存时额外检查其占用和活动状态。
- 完成且验证的源码、文档或持久项目状态改动默认按 Lore 协议提交；只 stage 自己可安全分离的改动，失败或越权时停止提交并报告。

<a id="skill-routing"></a>
## Skill 路由

- `$oskernel-kernel-engineering`：架构、子系统、Rust/unsafe、Linux/POSIX/ABI、技术债和高风险修改。
- `$oskernel-validation`：分层构建/测试/QEMU、bounded regression、证据与 claim 边界。
- `$oskernel-cross-arch-delivery`：RV/LA、本地/远程配置、offline/vendor 和提交产物。
- `$oskernel-compatibility-evaluation`：专项 LTP/score、stable promotion、blacklist 和评测报告；不替代真实内核语义。
- `$oskernel-experimental-features`：实验提案、隔离/feature gate、指标、晋升、继续或退役。
- `$oskernel-collaboration-delivery`：分支、所有权、冻结、review、交付报告、暂存和 Lore commit。
- `$oskernel-repo-hygiene`：仓库布局、dirty baseline、生成物/证据、磁盘、缓存和安全清理。

只加载当前任务需要的 skill；跨域时由主责 skill 明确 handoff，不复制相邻 runbook。Skill 与当前源码、`Makefile`、脚本、配置或实测冲突时，以当前仓库事实为准。

<a id="minimum-execution-loop"></a>
## 最小执行闭环

1. **Fact**：读取当前源码、配置、脚本、日志、测试源和 Git 状态，定义目标、证据、边界与停止条件。
2. **Edit**：选择最小 owned scope，复用现有模式，保持补丁可回滚。
3. **Verify**：运行能证明 claim 的最小新鲜验证；失败则修复并重跑，风险扩大时再加相邻回归。
4. **Report**：列出改动文件/意图、行为与 ABI 影响、实际命令和结果、未验证项、风险与回滚。
5. **Lore**：验证通过且改动可安全分离时，只暂存 owned paths，检查 cached diff，创建 Lore commit 并报告 SHA。

停止前确认：无未处理的任务内错误，失败证据未被隐藏，claim 与验证匹配，残余风险和下一负责人明确。
