# PR1 验证记录

## 结果分类

- `PASS`：命令成功，且证据覆盖本里程碑。
- `BASELINE`：失败在起始提交可复现，且与 PR1 修改无关。
- `ENVIRONMENT`：host 工具/target 支持阻止命令执行；不能记为 PASS。
- `INVOCATION`：调用参数或架构切换状态不构成有效验收；随后必须用正确调用重跑。
- `REGRESSION`：由 PR1 引入；提交前必须修复或按 stop 条件停止。

日志存于 `/tmp/orays-pr1-baseline/`，不属于仓库交付物。

## M0 基线

| 命令 | exit | 分类 | 证据/说明 |
|---|---:|---|---|
| `cargo fmt --all -- --check` | 1 | BASELINE | 起始提交已有 4 个格式差异：`api/arceos_posix_api/src/imp/pipe.rs`、`kernel/fs/axfs/src/dev.rs`、`kernel/fs/axfs/src/root.rs`、`kernel/task/axtask/src/wait_queue.rs` |
| `python3 scripts/check_g006_synthetic_capabilities.py` | 0 | PASS | synthetic capability guard |
| `python3 scripts/check_g009_post_review_semantics.py` | 0 | PASS | post-review semantics guard |
| `python3 scripts/check_g012_syscall_review_hotspots.py` | 0 | PASS | syscall hotspot guard |
| `python3 scripts/check_g013_user_copy_boundary.py` | 0 | PASS | user-copy boundary guard |
| `make clippy ARCH=riscv64` | 0 | PASS | `m0-clippy-riscv64.log` |
| `make clippy ARCH=loongarch64` | 2 | ENVIRONMENT | axlibc bindgen 使用 host libclang，报 `unknown target triple 'loongarch64-unknown-none'`；`m0-clippy-loongarch64.log` |
| `make A=user/shell ARCH=riscv64 build` | 0 | PASS | 默认 feature，不编译 uspace；`m0-build-riscv64-default.log` |
| `make A=user/shell ARCH=loongarch64 build` | 2 | INVOCATION | 前一条命令留下 RISC-V `.axconfig`；`m0-build-loongarch64-default.log` |
| `make A=user/shell ARCH=loongarch64 defconfig` 后重跑默认 build | 0 | PASS | `m0-build-loongarch64-default-retry.log` |
| `make A=user/shell ARCH=riscv64 APP_FEATURES=uspace build` | 2 | BASELINE | 裸 uspace 未开启 `axtask/sched-rr`，15 处缺少 `WaitQueue::wait_timeout_until`；`m0-build-riscv64-uspace.log` |
| `make A=user/shell ARCH=riscv64 APP_FEATURES=auto-run-tests,uspace build` | 2 | INVOCATION | 未同时开启 kernel irq 等 feature，`set_oneshot_timer` 不可用；`m0-build-riscv64-full.log` |
| `make A=user/shell ARCH=riscv64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build` | 0 | PASS | official feature 等价路径；`m0-build-riscv64-official-features.log` |
| `make A=user/shell ARCH=loongarch64 defconfig` 后用相同 official features build | 0 | PASS | `m0-build-loongarch64-official-features.log` |
| `make unittest_no_fail_fast` | 2 | BASELINE | axfs FAT `test_devfs_ramfs() failed: NotFound`；`m0-unittest-no-fail-fast.log` |
| `cargo test -p axfs --features myfs --no-fail-fast -- --nocapture` | 101 | BASELINE | RAMFS 同一 `test_devfs_ramfs() failed: NotFound`；`m0-unittest-axfs-myfs.log` |
| `cargo test --workspace --exclude axfs --no-fail-fast -- --nocapture` | 0 | PASS | `m0-unittest-workspace-exclude-axfs.log` |
| `cargo metadata --locked --offline --format-version 1 --no-deps` | 0 | PASS | 当前 workspace 图可离线解析 |

## 基线完整性

- 构建期间 bindgen 曾把 `api/arceos_posix_api/src/ctypes_gen.rs` 改为目标相关的 packed epoll layout；已用精确补丁恢复起始提交内容，未将其视为源码修改。
- M0 基线后 `Cargo.lock` SHA-256 仍为 `29d1b37d6bd6e2dbca66d0163f1049204edacfa064cab5438015a6983b59c359`。
- M0 文档提交前应只有四份 `docs/pr1/*.md` tracked diff；两个既有未跟踪输入保持不动。

## 后续记录模板

每个里程碑追加：commit SHA、逐条命令/exit/classification、changed files、已知失败、unsafe block 增减/移动、Cargo.lock diff 原因、双架构证据和工作树状态。不得用一次默认 build 代替 uspace official-feature build。
