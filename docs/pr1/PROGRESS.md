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
| M0 | complete | 固化分析、决策、基线和白名单 | `docs(pr1): record analysis and baseline` |
| M1 | pending | 抽取纯 ABI crate，保留 shell facade | `refactor(linux): extract pure ABI crate` |
| M2 | pending | 建立通用 typed user-memory 边界和 shell adapter | `refactor(linux): add typed user memory boundary` |
| M3 | pending | 让既有 user-copy facade 经过 typed adapter | `refactor(linux): route user copy through backend adapter` |
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
cargo check --locked --offline -p <new-crate> --target loongarch64-unknown-none
python3 scripts/check_g006_synthetic_capabilities.py
python3 scripts/check_g009_post_review_semantics.py
python3 scripts/check_g012_syscall_review_hotspots.py
python3 scripts/check_g013_user_copy_boundary.py
make A=user/shell ARCH=riscv64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build
make A=user/shell ARCH=loongarch64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build
git diff --check
```

M2 起同时运行 `cargo test --locked --offline -p orays-linux`；M4 起运行 PR1 新 guard 及其测试。M5 运行 AGENTS.md 的完整矩阵，并逐项区分 PASS、BASELINE、ENVIRONMENT、REGRESSION。

## Stop 条件映射

发现 cycle、ABI value/layout 变化、需要移动 UserProcess、需要新外部依赖、需要改 syscall 语义、或必须同时修改多个 major subsystem 时立即停止并报告。本文件的白名单不是扩大授权；超出白名单必须先说明原因并更新决策。
