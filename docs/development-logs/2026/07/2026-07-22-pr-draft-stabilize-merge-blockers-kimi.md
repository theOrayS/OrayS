# 2026-07-22 pr-draft stabilize merge blockers (kimi)

- PR: draft（本分支 `fix/stabilize-merge-blockers-kimi-20260722`，关联 PR #3 的 merge-blocker 整改）
- 分支基线：`origin/stabilize/post-integration-gates-20260716@d9891d02`
- 负责人：kimi（AI 辅助，待人工复核）
- 能力域：desktop 证据链/scope 强制、CI 工作流、测试基础设施诊断

## 背景与目标

PR #3 处于 Draft 且 mergeable_state=unstable。目标是在不真实放宽任何门禁的前提下：
(1) desktop scope 检查 fail-closed；(2) runtime 证据绑定到实际执行的 QEMU 与 guest
artifact；(3) runner 生命周期 fail-closed；(4) FAIL package 精确语义；(5) 移除未授权的
持久 self-hosted runtime CI job；(6) 诚实诊断 Build/Test/Docs CI 失败；(7) 保持全部
desktop 既有门禁。非目标：修复 `tee_device_mode`（GitHub P1 issue #2，保持显式阻塞）、
推进 PR #3 合并、改动 main 或推送任何分支。

## 基线（d9891d02，开发前实测）

- `python3 -B -m unittest discover -s test/desktop -p 'test_*.py'`：65/65 PASS。
- `git status` 干净；HEAD=d9891d02。

## 修改与决策记录

### 1. check-scope.py fail-closed（commit 90df4501）

- 旧实现用 `splitlines()`+`.strip()` 解析未加 `-z` 的 git 输出：换行名被 C 引用、
  首尾空白被剥除，可把树外路径改写成 allowlist 内路径（实测 `" user/desktop/..."`
  被 strip 后放行）。改为 `-z` NUL 分隔字节级解析（surrogateescape 解码），
  numstat 同步改 `-z --no-renames`，不可解析记录硬性报错。
- 旧 `default_base` 静默回退 `origin/develop/post-integration-next` → `HEAD`：
  在浅克隆 CI 中会退化为对空 diff 放行。改为必须显式 `--base` 或
  `.codex/state/desktop-base-sha`，且必须解析为 commit；打印 `base`/`base_source`。
- Desktop CI 改为 `fetch-depth: 0` 并显式传 `--base`。
- 回归测试：对抗性文件名（换行/制表/引号/首尾空白/Unicode）、分类不变异、
  缺省 base 硬失败、非法 base 硬失败、异常名 churn 计数。

### 2. runtime 证据绑定（commit c425d8ec，schema 3→4）

- metadata 记录：解析后 QEMU 绝对路径、二进制 SHA-256、完整有序 argv、guest
  artifact 路径/类型(ELF magic)/大小/SHA-256/架构、runner 输入（VNC display、
  timeout）、运行前后 commit/status 与 provenance_stable。
- runner 一次性解析 QEMU 路径，验证 banner 与 digest，执行同一已验证身份；
  `--record-invocation` 阶段重新 digest，身份采集与执行之间的替换硬性失败。
- 可选策略 `DESKTOP_QEMU_AUTHORIZED_SHA256`：格式错误或不匹配即硬失败
  （`qemu_authorized_digest_invalid`/`qemu_digest_mismatch`）；未设置记 `unpinned`。
- validator/packager 对上述字段做 package↔metadata 精确交叉校验；文件存在时
  重新计算 digest 比对；旧 schema 证据直接拒绝（防降级）。

### 3. runner 生命周期（同一 commit）

- 每个 `wait "$qemu_pid"` 前预置 `qemu_exit=0`：QEMU 提前 0 退出不再变成缺失
  状态或被 cleanup 的二次 wait 伪报为 127。
- cleanup 只在 `qemu_exit` 为空时填充，绝不覆盖已记录的真实退出状态。
- 行为测试：假 QEMU banner 错误/授权 digest 不匹配/格式错误 → exit 3 +
  VALID_FAIL package + 结构化 reason。

### 4. FAIL package 精确语义（同一 commit）

- 删除 `failure.split(':', 1)[0]` 类别比较；validator 要求 recorded failures 与
  重新复现的 failures 严格相等（含 runner 行的确定顺序）。
- OSError 详情改为位置无关（strerror），run dir 与迁移后 package 的同一失败
  字符串一致。
- 对抗测试：同冒号前缀不同失败被拒、删/增失败被拒、package 内 QEMU/guest
  身份篡改被拒。

### 5. 移除持久 self-hosted runtime job（commit 99067046）

- 删除 `desktop-runtime` job 与未被消费的 `FIXED_QEMU_VERSION`；不新增任何
  self-hosted 触发。本地/人工 runtime 流程与 digest 策略写入
  `docs/references/desktop-headless-development.md`；策略测试断言所有 workflow
  无 self-hosted。

### 6. Build/Test/Docs CI 诊断（本地等价命令实测）

- `cargo fmt --all -- --check`：PASS。
- `make unittest_no_fail_fast`：exit 0（全部 Rust 单元/文档测试 PASS）。
- `test/run_suite.py --profile quick`：空载 45/45 PASS；重载并行时
  `unit.suite_runner` 超 300s 预算（空载实测 228s/135 tests PASS）——对负载敏感，
  非代码缺陷，但预算余量小，需团队决策（调预算或拆分，涉及 test/ 非 desktop 路径）。
- `make clippy ARCH=x86_64|aarch64`：**真实代码缺陷**，42/39 个编译错误，全部位于
  `user/shell/src/uspace/*`：TrapFrame 无 `regs` 字段（x86_64 为 rax/...）、缺
  `SS_DISABLE`/`SI_TKILL_CODE`/`terminate_current_thread[_for_exit_group]`、
  `AUX_PLATFORM` 被 cfg 排除。即 uspace 代码未移植到 x86_64/aarch64。
  `ARCH=riscv64|loongarch64` PASS。
- `make doc_check_missing ARCH=x86_64`：**真实缺陷**，新增文件
  `api/orays_linux/src/{backend,syscall,user}.rs` 缺 missing_docs 文档注释。
- app-test：本地无 musl 工具链（CI 由 setup-musl 安装）——环境缺口；
  arceos-apps 克隆需要网络（本机有网络但未执行完整矩阵）。
- PR3 evidence profiles / pinned QEMU source baseline：依赖 CI 工件流与官方
  镜像/网络，本地不完全等价，记为外部基础设施项。

**范围冲突（需人工决策）**：上述 clippy/docs 修复必须改动 `user/shell/**` 与
`api/orays_linux/**`，二者均在 desktop scope 的 FORBIDDEN 列表；应用修复会使
目标规定的 `check-scope.py --base c776ceff...` 门禁 FAIL。按“不可在无人工决策时
发明豁免”的原则，本 PR 不越权修改这些路径，仅记录根因与最小修复方向：

- docs：为 `backend.rs`/`syscall.rs`/`user.rs` 的 public 项补 `///` 文档。
- clippy：为 x86_64/aarch64 补齐 uspace 移植（TrapFrame 字段映射、信号常量、
  线程终止 API、auxv 常量 cfg），工作量实质，应单独立 PR。

### 验证（全部在本分支最终 HEAD 实测）

- desktop Python：`python3 -B -m unittest discover -s test/desktop -p 'test_*.py'`
  100/100 PASS（65 基线 + 35 新增/更新），约 53s。
- `scripts/desktop/build.sh host-test`：14/14 PASS。
- `cargo fmt --manifest-path user/desktop/Cargo.toml -- --check`：PASS。
- desktop clippy `--offline --locked -D warnings`（host-tools）：PASS。
- `scripts/desktop/build.sh golden-check`：5/5 MATCH。
- `scripts/desktop/build.sh rv` / `la`：PASS。
- `python3 scripts/desktop/check-scope.py --base c776ceff40587de0fa0547724d0abfecbb56cc64`：
  PASS（changed_paths=116，bridge churn=74/250，base_source=cli）。
- 真实 QEMU 9.2.4 boot + review package（提交后干净树）：
  - rv：`DESKTOP_QEMU_BOOT=PASS` + `VALID_PASS failures=0`（runner exit 0）；
  - la：`DESKTOP_QEMU_BOOT=PASS` + `VALID_PASS failures=0`（runner exit 0）；
  - rv + `DESKTOP_QEMU_AUTHORIZED_SHA256=<本机 QEMU digest>`：PASS，
    policy=authorized-sha256，matches=True，provenance_stable=True；
  - la package 迁移到 `/tmp` 后重新校验：`VALID_PASS failures=0`。
- `test/run_suite.py --profile quick`：空载 45/45 PASS（ci-quick-3）。
- `make unittest_no_fail_fast`：exit 0。
- `test/run_suite.py --profile evidence-host`：PASS（ci-evidence-host）。
- `test/run_suite.py --profile evidence-runtime --arch rv|la`：两架构 build 全 PASS，
  smoke 均 `PR3_SMOKE_V1 USER_FAIL tee_device_mode`（GitHub issue #2，按政策不修不掩）。
- 固定 QEMU 源：qemu-9.2.4.tar.xz 下载实测 SHA-256 与大小（134782772）均匹配
  `test/evidence/setup_qemu.sh` 的固定策略；本机缺 libfdt-dev，未完整重编译
  （CI 安装该包；源码完整性路径无缺陷证据）。

### 未解决阻塞（诚实状态）

1. `tee_device_mode`：GitHub P1 issue #2，rv/la smoke 均复现，保持显式失败。
2. clippy x86_64/aarch64：`user/shell/src/uspace/*` 未移植两架构（42/39 个编译错误，
   根因见上），修复在 desktop scope FORBIDDEN 路径内，需人工决策后单独立 PR。
3. docs build：`api/orays_linux/src/{backend,syscall,user}.rs` 缺文档注释，同属
   FORBIDDEN 路径，需人工决策。
4. other-platform setup：`scripts/cargo-axplat.sh` offline shim 不查
   `vendor/cargo/<pkg>/axconfig.toml`，且三个 aarch64 板级配置未进
   `configs/platforms/`；修复路径同样在 scope 外。
5. `unit.suite_runner` 300s 预算余量小（空载 228s）；CI 更慢机器上存在超时风险，
   预算/拆分需团队决策。
6. app-test 需 musl 工具链（本机未装，CI 由 setup-musl 提供）——外部环境项。
7. PR #3 仍不可声明 merge-ready：上述 CI 红项、人工 review、scope 政策决策未闭合。

## AI 使用披露

- 工具：Kimi Code CLI（kimi）。场景：代码审查、修复设计、测试编写、CI 诊断。
- 生成/显著影响文件：本日志 git diff 所列全部文件。
- 人工复核：待进行；关键不变量（NUL 解析、digest 绑定、精确失败语义）均可由
  测试独立复现。

## 已知限制与回滚

- `tee_device_mode` 仍为显式 blocker（GitHub issue #2）。
- PR #3 不可声明 merge-ready（CI、review、scope 政策决策未闭合）。
- 回滚：`git revert` 本 PR 各 commit 即可，无持久外部状态。
