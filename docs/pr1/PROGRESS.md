# PR1 实施进度

## 起点

- 分支：`refactor/pr1-linux-boundary`
- 起始提交：`e7ad4862d1da1f79f30a30db41a9e635cff268fb`
- 起始 tracked worktree：干净
- 既有未跟踪用户输入（禁止修改/提交）：`.codex/CODEX_PR1_GOAL.md`、`docs/pr1-linux-boundary-survey.md`
- 起始 `Cargo.lock` SHA-256：`29d1b37d6bd6e2dbca66d0163f1049204edacfa064cab5438015a6983b59c359`
- 工具链：Rust/Cargo 1.89.0-nightly，`nightly-2025-05-20`；Python 3.10.12

## 里程碑状态

| 里程碑 | 状态 | 目标 | 计划提交 |
|---|---|---|---|
| M0 | complete | 固化分析、决策、基线和白名单 | `1b3dc605 docs(pr1): record analysis and baseline` |
| M1 | complete | 抽取纯 ABI crate，保留 shell facade | `940438f7 refactor(linux): extract pure ABI crate` |
| M2 | complete | 建立通用 typed user-memory 边界和 shell adapter | `7357d56c refactor(linux): add typed user memory boundary` |
| M3 | complete | 让既有 user-copy facade 经过 typed adapter | `refactor(linux): route user copy through backend adapter`（本里程碑提交） |
| M4 | pending | 增加最小 syscall metadata 和静态 guard | `refactor(linux): add syscall boundary metadata` |
| M5 | pending | 完整验证、独立审查、必要的后续修复与收口 | `docs(pr1): record final validation`；修复另建提交 |

## 每个里程碑的文件白名单

### M0

- `docs/pr1/ANALYSIS_SUMMARY.md`
- `docs/pr1/DECISIONS.md`
- `docs/pr1/PROGRESS.md`
- `docs/pr1/VALIDATION.md`

### M1

- `Cargo.toml`
- `Cargo.lock`
- `api/orays_linux_abi/Cargo.toml`
- `api/orays_linux_abi/src/lib.rs`
- `api/orays_linux_abi/src/constants.rs`
- `api/orays_linux_abi/src/syscall.rs`
- `api/orays_linux_abi/src/time.rs`
- `user/shell/Cargo.toml`
- `user/shell/src/uspace/linux_abi.rs`
- `user/shell/src/uspace/time_abi.rs`
- 四份 `docs/pr1/*.md`

M1 不修改 dispatcher、UserProcess、handler、signal frame 或 user-copy。

### M2

- `Cargo.toml`
- `Cargo.lock`
- `api/orays_linux/Cargo.toml`
- `api/orays_linux/src/lib.rs`
- `api/orays_linux/src/user.rs`
- `api/orays_linux/src/backend.rs`
- `user/shell/Cargo.toml`
- `user/shell/src/uspace/linux_abi.rs`
- `user/shell/src/uspace/time_abi.rs`
- `user/shell/src/uspace/user_memory.rs`
- 四份 `docs/pr1/*.md`

M2 只增加 adapter，不切换现有 handler 路径。

### M3

- `user/shell/src/uspace/user_memory.rs`
- `api/orays_linux/src/user.rs`（仅测试/边界缺陷修正，如无需则不修改）
- `api/orays_linux/src/backend.rs`（仅 contract/测试缺陷修正，如无需则不修改）
- 四份 `docs/pr1/*.md`

M3 不批量编辑 syscall handler；旧 helper 名称和签名保持。

### M4

- `api/orays_linux/src/lib.rs`
- `api/orays_linux/src/syscall.rs`
- `user/shell/src/uspace/mod.rs`
- `user/shell/src/uspace/syscall_metadata.rs`
- `scripts/check_pr1_linux_boundary.py`
- `scripts/test_pr1_linux_boundary.py`
- 四份 `docs/pr1/*.md`

M4 不修改 `syscall_dispatch.rs` 的路由。guard 读取它进行核对。

### M5

- 四份 `docs/pr1/*.md`

若独立 reviewer 发现必须修复的问题，先在本文件登记独立修复提交和精确文件白名单，再编辑；不 amend 既有里程碑提交，不扩大到 syscall 行为。

## 里程碑验收门

每个实现里程碑至少执行：

```bash
cargo fmt --all -- --check
cargo check --locked --offline -p <new-crate> --target riscv64gc-unknown-none-elf
cargo check --locked --offline -p <new-crate> --target loongarch64-unknown-none-softfloat
python3 scripts/check_g006_synthetic_capabilities.py
python3 scripts/check_g009_post_review_semantics.py
python3 scripts/check_g012_syscall_review_hotspots.py
python3 scripts/check_g013_user_copy_boundary.py
make A=user/shell ARCH=riscv64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build
make A=user/shell ARCH=loongarch64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build
git diff --check
```

M2 起同时运行 `cargo test --locked --offline -p orays-linux`；M4 起运行 PR1 新 guard 及其测试。M5 运行 AGENTS.md 的完整矩阵，并逐项区分 PASS、BASELINE、ENVIRONMENT、REGRESSION。

## M1 checkpoint

- 成功条件：纯 ABI crate 在 host、RISC-V64 与 LoongArch64 均可离线编译；两目标 `-D warnings` clippy 通过；shell facade 保留旧名字；official-feature shell 双架构构建通过；现有 guard 无回归；Cargo.lock 仅出现 path package/edge；没有 syscall、errno、handler、UserProcess、user-copy 或 unsafe 变化。
- 实际结果：满足上述条件。全局 fmt、LoongArch64 workspace clippy 与 axfs unittest 的非零结果均和 M0 基线一致，未作为 PASS；详见 `VALIDATION.md`。
- 语义审查：迁移范围只包含纯数值、`AUX_PLATFORM`、`Tms`、`RtcTime` 和 syscall number namespace。地址布局、signal frame/trampoline、synthetic policy、errno helper、`UserTimex` 与所有 handler 均留在 shell。
- 用户输入完整性：两份未跟踪输入的 SHA-256 仍分别为 `f6fb00c626dccca22ed15d1713ef4a8eb38bdb9d6a028fbd57e12ea8950efabb` 与 `b6b7911b0da05f783366baf444328400b9b93b72494f117e7e78b941a276db75`。

## M2 checkpoint

- 成功条件：`orays-linux` 是 `no_std`、无 unsafe、只依赖 `orays-linux-abi`；提供 checked `UserAddr`/`UserRange`/`UserPtr`/`UserSlice`、sealed `Read`/`Write` marker 和 byte backend trait；shell adapter 编译但现有 helper/handler 尚不改道；host 单测及两目标 `-D warnings` clippy 通过；Cargo graph 为最终单向链。
- 实际结果：满足上述条件。5 个单测覆盖零长度/null、地址溢出、长度乘法溢出、ZST、读写 marker、fake backend bounds 与 slice 长度不匹配。两架构 official-feature shell build 和 RISC-V workspace clippy 通过；LA workspace clippy、全局 fmt、axfs unittest 与 M0 基线一致。
- 行为边界：typed constructor 只验证整数算术；非零 null、映射权限、fault-in、brk、partial-copy 与 LinuxError 仍完全由 shell 原实现负责。M2 adapter 尚未被 syscall 路径构造，因而不会改变返回值或 fault 行为。
- 已处理回归：为抑制临时 dead-code warning 加入的高阶 constructor 类型证明在 LA build 触发 `E0308`；立即删除该脆弱证明，保留诚实的单个过渡期 `ProcessUserMemory is never constructed` warning。M3 实际接入 adapter 后该 warning 应自然消失，不添加 `allow`。
- 用户输入完整性：两份未跟踪输入 hash 仍与任务起点一致。

## M3 checkpoint

- 成功条件：raw `usize` 到 checked typed range 的转换集中在一个入口；旧 validate/fault/read/write facade 名称、签名和 caller 不变但实际经过 shell adapter；fault-in、brk、mapping permission、跨页、perf 计数和 `EFAULT`/`ENOMEM` 顺序保持；5 个既有 unsafe 不增删、不移动；双架构 official-feature build 通过；remaining legacy callers 有精确 inventory。
- 实际结果：满足上述条件。M3 只修改 `user_memory.rs` 和四份 PR1 文档；adapter 已真实构造，M2 的 transitional dead-code warning 自然消失。`read_user_bytes` 保持 validate → allocation → raw read 顺序；其他 byte facade 构造与 slice 等长的 `UserRange<Read/Write>` 后委托 adapter。handler、dispatcher、UserProcess、Cargo manifests 和 `Cargo.lock` 均未修改。
- caller/unsafe：`read_user_value<T: Copy>` 为 92 occurrences/17 files，`write_user_value<T: Copy>` 为 118/18，`read_cstr` 为 46/6；其余 facade 的完整计数见 `ANALYSIS_SUMMARY.md`。`user_memory.rs` 仍为 5 个 unsafe，diff 未触及其表达式或所属函数。M4 guard 将机器固化这些边界；M3 起 review policy 禁止增加调用点。
- 测试边界：`orays-linux` 的 5 个 host 测试覆盖整数 overflow、null 不由类型层擅自拒绝、零长度、typed slice 和 marker/backend contract。仓库没有可独立构造的 shell `UserProcess`/`AddrSpace` fixture；跨页、只读映射写和 partial-copy 不能在不伪造 backend 语义的情况下新增真实 host regression test，因此保留原实现并用双架构真实 shell build 验证编译路径，原因记录在分析与验证文档。
- 验证：RISC-V64/LoongArch64 official-feature shell build、RISC-V64 workspace clippy、new-crate 三目标 `-D warnings`、existing guards、workspace excluding axfs tests 均通过。全局 fmt、LA workspace clippy 与 axfs FAT unittest 分别保持 BASELINE/ENVIRONMENT/BASELINE；无未解决 PR1 regression。
- 完整性：Cargo graph 和 M2 lock hash `0f7b1d31…` 不变；host test 的 `ctypes_gen.rs` 生成副作用已精确恢复；两份未跟踪用户输入 hash 不变。

## Stop 条件映射

发现 cycle、ABI value/layout 变化、需要移动 UserProcess、需要新外部依赖、需要改 syscall 语义、或必须同时修改多个 major subsystem 时立即停止并报告。本文件的白名单不是扩大授权；超出白名单必须先说明原因并更新决策。
