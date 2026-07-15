# PR1 验证记录

## 结果分类

- `PASS`：命令成功，且证据覆盖本里程碑。
- `BASELINE`：失败在起始提交可复现，且与 PR1 修改无关。
- `ENVIRONMENT`：host 工具/target 支持阻止命令执行；不能记为 PASS。
- `INVOCATION`：调用参数或架构切换状态不构成有效验收；随后必须用正确调用重跑。
- `BOUNDED`：外层 timeout 按设计终止，但已观察到声明的 smoke marker；不能外推为完整测试 PASS。
- `REGRESSION`：由 PR1 引入；提交前必须修复或按 stop 条件停止。

日志存于 `/tmp/orays-pr1-baseline/` 与 `/tmp/orays-pr1-final/`，不属于仓库交付物。

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

## M1 — pure ABI crate

### Targeted ABI checks

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `cargo check --offline -p orays-linux-abi` | host | 0 | PASS | `no_std` crate 可离线解析和编译；首次执行生成所需 path-package lock 条目 |
| `cargo check --locked --offline -p orays-linux-abi --target riscv64gc-unknown-none-elf` | RISC-V64 | 0 | PASS | ABI constants、number namespace 和 layout assertions 编译通过 |
| `cargo check --locked --offline -p orays-linux-abi --target loongarch64-unknown-none` | 错误 target 名 | 101 | INVOCATION | 本机未安装该 target；随后使用实际安装的 softfloat target 重跑 |
| `cargo check --locked --offline -p orays-linux-abi --target loongarch64-unknown-none-softfloat` | LoongArch64 | 0 | PASS | ABI constants、legacy rlimit numbers 和 layout assertions 编译通过 |
| `cargo clippy --locked --offline -p orays-linux-abi --target riscv64gc-unknown-none-elf -- -D warnings` | RISC-V64 | 0 | PASS | 新 crate 零 clippy warning |
| `cargo clippy --locked --offline -p orays-linux-abi --target loongarch64-unknown-none-softfloat -- -D warnings` | LoongArch64 | 0 | PASS | 新 crate 零 clippy warning |
| `cargo test --locked --offline -p orays-linux-abi` | host | 0 | PASS | crate test harness 与所有 const assertions 编译通过；0 runtime tests |

### Milestone matrix

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `cargo fmt --all -- --check` | workspace | 1 | BASELINE | 仍只报告 M0 的 4 个无关文件；未格式化它们 |
| `rustfmt --edition 2024 --check api/orays_linux_abi/src/lib.rs api/orays_linux_abi/src/constants.rs api/orays_linux_abi/src/syscall.rs api/orays_linux_abi/src/time.rs user/shell/src/uspace/linux_abi.rs user/shell/src/uspace/time_abi.rs` | touched Rust files | 0 | PASS | M1 Rust diff 格式正确 |
| `python3 scripts/check_g006_synthetic_capabilities.py` | static | 0 | PASS | synthetic backend policy 仍留在 shell |
| `python3 scripts/check_g009_post_review_semantics.py` | static | 0 | PASS | semantic guard 无新增 finding |
| `python3 scripts/check_g012_syscall_review_hotspots.py` | static | 0 | PASS | dispatcher 未修改 |
| `python3 scripts/check_g013_user_copy_boundary.py` | static | 0 | PASS | user-copy 未修改 |
| `make A=user/shell ARCH=riscv64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build` | RISC-V64 | 0 | PASS | shell 的真实 uspace feature 路径解析 ABI facade |
| `make A=user/shell ARCH=loongarch64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build` | LoongArch64 | 0 | PASS | 同一 feature 路径与 LA cfg 通过 |
| `make clippy ARCH=riscv64` | RISC-V64 workspace | 0 | PASS | 最终 M1 源码重跑通过；仅既有 warning |
| `make clippy ARCH=loongarch64` | LoongArch64 workspace | 2 | ENVIRONMENT | 与 M0 相同的 host/target 工具链路径失败；不能记为 PASS |
| `cargo clippy --locked --offline -p axlibc --target loongarch64-unknown-none-softfloat -- -A clippy::new_without_default -A unsafe_op_in_unsafe_fn` | LoongArch64 ad-hoc probe | 101 | INVOCATION | 缺少 Makefile 注入的 `axplat_loongarch64_qemu_virt` feature；该窄调用不构成验收，重复确认同一错误后停止；由完整 shell build 和新 crate clippy 提供有效 LA 证据 |
| `make unittest_no_fail_fast` | host | 2 | BASELINE | 与 M0 相同：axfs FAT `test_devfs_ramfs() failed: NotFound` |
| `cargo test --locked --offline -p axfs --no-fail-fast --features myfs -- --nocapture` | host | 101 | BASELINE | 与 M0 相同：RAMFS 的 `test_devfs_ramfs() failed: NotFound` |
| `cargo test --locked --offline --workspace --exclude axfs --no-fail-fast -- --nocapture` | host | 0 | PASS | 排除唯一已证明的基线失败后，workspace tests 全部通过 |
| `cargo metadata --locked --offline --format-version 1 --no-deps` | dependency graph | 0 | PASS | 新 package 可解析，无 Cargo cycle |
| `cargo tree --locked --offline -p arceos-shell -e normal --features uspace` | dependency graph | 0 | PASS | M1 临时方向仅为 `arceos-shell -> orays-linux-abi -> linux-raw-sys`；无反向 shell 边 |
| `git diff --check` | worktree | 0 | PASS | 无 whitespace error |
| `git diff --exit-code -- api/arceos_posix_api/src/ctypes_gen.rs` | generated source | 0 | PASS | 双架构/测试命令产生的 target-specific 改写已用精确补丁恢复，M1 不包含该文件 |
| `sha256sum .codex/CODEX_PR1_GOAL.md docs/pr1-linux-boundary-survey.md` | user inputs | 0 | PASS | hash 与任务起点完全一致 |

### M1 audit

- changed files：`Cargo.toml`、`Cargo.lock`、`api/orays_linux_abi/Cargo.toml`、`api/orays_linux_abi/src/{lib,constants,syscall,time}.rs`、`user/shell/Cargo.toml`、`user/shell/src/uspace/{linux_abi,time_abi}.rs`、本目录的进度/决策/验证文档。
- syscall/errno/handler/user-copy：零修改；`syscall_dispatch.rs` 未触碰。关键公共 syscall number 及 LA legacy rlimit number 由目标相关 const assertion 固化。
- ABI/layout：所有迁移值逐项保留；`Tms`/`RtcTime` 维持字段顺序和标量类型，并增加 size、alignment、offset assertions。
- unsafe delta：新增 0、删除 0、移动 0；新 crate 使用 `#![forbid(unsafe_code)]`。
- dependency：ABI crate 不依赖任何 OrayS implementation crate；M1 的 shell 直连是 D-001 记录的可回滚临时边，M2 将替换为最终 `shell -> orays-linux -> orays-linux-abi`。
- `Cargo.lock`：仅给 `arceos-shell` 增加 `orays-linux-abi` edge，并新增 `orays-linux-abi 0.2.0` path package stanza；没有 registry package、版本、checksum 变化。M1 当前 SHA-256 为 `a624e5cdf24e13202a4fb3fec70da957973f557a361d9d5b43335da1b5ab45b3`。
- known failures：仅 M0 已证明的全局 fmt、LoongArch workspace clippy 环境问题和 axfs unittest；没有 PR1 regression。

## M2 — typed user-memory boundary and shell adapter

### Focused crate checks

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `cargo check --offline -p orays-linux` | host | 0 | PASS | 生成新 path-package lock stanza；crate 为 `no_std` 且只解析 ABI 依赖 |
| `cargo check --locked --offline -p orays-linux --all-features --target riscv64gc-unknown-none-elf` | RISC-V64 | 0 | PASS | typed types、trait 和 `alloc=[]` feature 编译通过 |
| `cargo check --locked --offline -p orays-linux --all-features --target loongarch64-unknown-none-softfloat` | LoongArch64 | 0 | PASS | 同一 API 在 LA target 编译通过 |
| `cargo clippy --locked --offline -p orays-linux --all-features -- -D warnings` | host | 0 | PASS | 新 crate 零 clippy warning |
| `cargo clippy --locked --offline -p orays-linux --all-features --target riscv64gc-unknown-none-elf -- -D warnings` | RISC-V64 | 0 | PASS | 新 crate 零 clippy warning |
| `cargo clippy --locked --offline -p orays-linux --all-features --target loongarch64-unknown-none-softfloat -- -D warnings` | LoongArch64 | 0 | PASS | 新 crate 零 clippy warning |
| `cargo test --locked --offline -p orays-linux` | host | 0 | PASS | 5 passed：range/null/overflow/ZST、typed slice、fake backend read/write/bounds/mismatch |

### Integration matrix

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `cargo fmt --all -- --check` | workspace | 1 | BASELINE | 仍只报告 M0 的 4 个无关文件 |
| `rustfmt --edition 2024 --check api/orays_linux/src/lib.rs api/orays_linux/src/user.rs api/orays_linux/src/backend.rs user/shell/src/uspace/linux_abi.rs user/shell/src/uspace/time_abi.rs user/shell/src/uspace/user_memory.rs` | touched Rust files | 0 | PASS | M2 Rust diff 格式正确 |
| `python3 scripts/check_g006_synthetic_capabilities.py` | static | 0 | PASS | synthetic policy 未迁移 |
| `python3 scripts/check_g009_post_review_semantics.py` | static | 0 | PASS | semantic guard 无 finding |
| `python3 scripts/check_g012_syscall_review_hotspots.py` | static | 0 | PASS | dispatcher/handler 未修改 |
| `python3 scripts/check_g013_user_copy_boundary.py` | static | 0 | PASS | 既有 user-copy 调用纪律未回归 |
| `make A=user/shell ARCH=riscv64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build` | RISC-V64 | 0 | PASS | 最终 M2 源码 official-feature build 通过 |
| `make A=user/shell ARCH=loongarch64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build`（未先 defconfig） | LoongArch64 | 2 | INVOCATION | Makefile 检测到上一条 RV `.axconfig`；运行 `make A=user/shell ARCH=loongarch64 defconfig` 后重跑 |
| 同一 LA build（首次代码重跑） | LoongArch64 | 2 | REGRESSION | compile-only constructor HRTB 证明触发 `E0308`；删除证明后同命令通过 |
| `make A=user/shell ARCH=loongarch64 defconfig` 后同一 LA build（最终源码） | LoongArch64 | 0 | PASS | facade、adapter 与 LA cfg 完整编译/链接/objcopy |
| `make A=user/shell ARCH=riscv64 defconfig` 后同一 RV build（最终源码） | RISC-V64 | 0 | PASS | facade、adapter 与 RV cfg 完整编译/链接/objcopy |
| `make clippy ARCH=riscv64` | RISC-V64 workspace | 0 | PASS | 最终 M2 源码通过；保留既有 warnings，并新增一个诚实的未构造 adapter 过渡 warning |
| `make A=user/shell ARCH=loongarch64 defconfig` 后 `make clippy ARCH=loongarch64` | LoongArch64 workspace | 2 | ENVIRONMENT | 与 M0/M1 相同：axlibc bindgen 的 host libclang 不识别 `loongarch64-unknown-none` |
| `make unittest_no_fail_fast` | host | 2 | BASELINE | 与 M0/M1 相同：axfs FAT `test_devfs_ramfs() failed: NotFound` |
| `cargo test --locked --offline --workspace --exclude axfs --no-fail-fast -- --nocapture` | host | 0 | PASS | 其余 workspace 测试通过，并实际运行 `orays-linux` 的 5 个测试 |
| `cargo metadata --locked --offline --format-version 1 --no-deps` | dependency graph | 0 | PASS | 两个新 package 与 feature 均可离线解析，无 cycle |
| `cargo tree --locked --offline -p orays-linux -e normal` | dependency graph | 0 | PASS | `orays-linux -> orays-linux-abi -> linux-raw-sys` |
| `cargo tree --locked --offline -p arceos-shell -e normal --features uspace` | dependency graph | 0 | PASS | 精确显示 `arceos-shell -> orays-linux -> orays-linux-abi` |
| `rg -n 'orays-linux-abi\|orays_linux_abi' user/shell/Cargo.toml user/shell/src` | shell direct edge | 1 | PASS | 无匹配；shell 已移除 ABI 直连。命令包装 `|| true` 时 shell exit 为 0，`rg` 自身的无匹配语义为 1 |
| `git diff --check` | worktree | 0 | PASS | 无 whitespace error |
| `git diff --exit-code -- api/arceos_posix_api/src/ctypes_gen.rs` | generated source | 0 | PASS | host tests 的 packed epoll 改写已精确恢复，M2 不包含该文件 |
| `sha256sum .codex/CODEX_PR1_GOAL.md docs/pr1-linux-boundary-survey.md` | user inputs | 0 | PASS | 两个 hash 与任务起点一致 |

### M2 audit

- changed files：`Cargo.toml`、`Cargo.lock`、`api/orays_linux/Cargo.toml`、`api/orays_linux/src/{lib,user,backend}.rs`、`user/shell/Cargo.toml`、`user/shell/src/uspace/{linux_abi,time_abi,user_memory}.rs`、本目录的进度/决策/验证文档。
- dependency：最终链已建立；`orays-linux` 只依赖 ABI crate，ABI crate 只依赖 `linux-raw-sys`，两者均不依赖 shell、UserProcess 或任何 OrayS implementation crate。
- handler/semantics：adapter 在 M2 未构造，所有现有 helper body 与 handler 调用点保持不变；UserProcess 仍由 shell 所有。
- typed boundary：range 是 half-open，检查 `start + len`；slice 还检查 `len * size_of::<T>()`。零长度在任意整数地址有效，非零 null 留给 backend 判定，和现有 zero-length 行为一致。
- unsafe delta：新增 0、删除 0、移动 0；新 crate 使用 `#![forbid(unsafe_code)]`。既有 `T: Copy` value-copy unsafe 未修改。
- `Cargo.lock`：shell edge 从 `orays-linux-abi` 替换为 `orays-linux`，新增 `orays-linux 0.2.0` path package stanza，后者只列 `orays-linux-abi`；没有 registry package、版本或 checksum 变化。M2 SHA-256 为 `0f7b1d3135d88d007eca51ab853007a182b5a0d8291808e67d582723cd72c4c8`。
- known failures：全局 fmt、LA workspace clippy 与 axfs unittest 均为已证明基线；中途 `E0308` 是 M2 regression，已修复并由双架构最终 build 证明。无未解决 PR1 regression。

## M3 — route legacy user-copy facades through the typed adapter

### Focused and integration checks

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `rustfmt --edition 2024 user/shell/src/uspace/user_memory.rs` | touched Rust file | 0 | PASS | 只格式化 M3 Rust 文件；未运行 repo-wide formatter 写入 |
| `rustfmt --edition 2024 --check user/shell/src/uspace/user_memory.rs` | touched Rust file | 0 | PASS | typed bridge diff 格式正确；最终文档注释后再次检查 |
| `cargo test --locked --offline -p orays-linux` | host | 0 | PASS | 5 passed：typed range/slice、overflow/null/zero-length、fake byte backend contract |
| `cargo clippy --locked --offline -p orays-linux --all-features -- -D warnings` | host | 0 | PASS | 通用 crate 零 warning |
| `cargo clippy --locked --offline -p orays-linux --all-features --target riscv64gc-unknown-none-elf -- -D warnings` | RISC-V64 | 0 | PASS | typed API/trait 编译通过 |
| `cargo clippy --locked --offline -p orays-linux --all-features --target loongarch64-unknown-none-softfloat -- -D warnings` | LoongArch64 | 0 | PASS | typed API/trait 编译通过 |
| `python3 scripts/check_g006_synthetic_capabilities.py` | static | 0 | PASS | 0 findings |
| `python3 scripts/check_g009_post_review_semantics.py` | static | 0 | PASS | 0 findings |
| `python3 scripts/check_g012_syscall_review_hotspots.py` | static | 0 | PASS | 0 findings；dispatcher 未修改 |
| `python3 scripts/check_g013_user_copy_boundary.py` | static | 0 | PASS | 0 findings |
| `make A=user/shell ARCH=riscv64 defconfig` | RISC-V64 | 0 | PASS | 切换到 RV config；每次跨架构前显式执行 |
| `make A=user/shell ARCH=riscv64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build` | RISC-V64 | 0 | PASS | shell adapter 完整编译/链接/objcopy；M2 adapter dead-code warning 消失 |
| `make A=user/shell ARCH=loongarch64 defconfig` | LoongArch64 | 0 | PASS | 切换到 LA config |
| `make A=user/shell ARCH=loongarch64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build` | LoongArch64 | 0 | PASS | shell adapter 完整编译/链接/objcopy |
| `cargo fmt --all -- --check` | workspace | 1 | BASELINE | 仍且只报告 M0 的 4 个无关文件：POSIX pipe、axfs dev/root、axtask wait_queue |
| `make clippy ARCH=riscv64`（先 RV defconfig） | RISC-V64 workspace | 0 | PASS | workspace clippy 通过；既有 warnings 未提升为错误 |
| `make clippy ARCH=loongarch64`（先 LA defconfig） | LoongArch64 workspace | 2 | ENVIRONMENT | 与 M0–M2 相同：axlibc bindgen 的 host libclang 不识别 `loongarch64-unknown-none`；在进入 shell M3 代码前失败 |
| `make unittest_no_fail_fast` | host | 2 | BASELINE | 与 M0–M2 相同：axfs FAT `test_devfs_ramfs() failed: NotFound`；其余 axfs targets 继续运行 |
| `cargo test --locked --offline --workspace --exclude axfs --no-fail-fast -- --nocapture` | host | 0 | PASS | 其余 workspace 全部通过，含 `orays-linux` 5 tests |
| `cargo metadata --locked --offline --format-version 1 --no-deps` | dependency graph | 0 | PASS | offline/locked 解析通过，无新 edge/cycle |
| `cargo tree --locked --offline -p arceos-shell -e normal --features uspace` | dependency graph | 0 | PASS | `arceos-shell -> orays-linux -> orays-linux-abi -> linux-raw-sys` |
| combined integrity wrapper with `rg 'unsafe\\s*\\{'` | invocation | 1 | INVOCATION | shell 收到重复转义的 regex，`rg` 报 repetition quantifier；没有代码/仓库副作用，立即用下一条精确 pattern 重跑 |
| `test "$(rg -n 'unsafe\s*\{' user/shell/src/uspace/user_memory.rs \| wc -l)" -eq 5` | unsafe audit | 0 | PASS | M3 前后均为 5；后续 integrity 子命令也全部成功 |
| `git diff --check` | worktree | 0 | PASS | 无 whitespace error |
| `git diff --exit-code -- Cargo.lock api/arceos_posix_api/src/ctypes_gen.rs` | lock/generated source | 0 | PASS | M3 不改 lock；host test 的 packed epoll 生成副作用已用精确 apply patch 恢复 |
| `sha256sum Cargo.lock .codex/CODEX_PR1_GOAL.md docs/pr1-linux-boundary-survey.md` | integrity | 0 | PASS | lock 为 M2 hash `0f7b1d31…`；两份用户输入仍为 `f6fb00c6…`/`b6b7911b…` |

### M3 semantic audit

- changed files：`user/shell/src/uspace/user_memory.rs` 与四份 `docs/pr1/*.md`；未修改 handler、dispatcher、manifest、lock 或新 crate API。
- raw/typed boundary：所有新 raw `usize` → typed range 转换集中在 `typed_user_range`；非零 overflow 仍为 `EFAULT`，zero-length 仍提前成功，null/mapping/brk/fault-in 留给原 backend。
- error/perf ordering：`read_user_bytes` 保持 fault/permission → allocation → address-space read → read counter；没有 double fault 或 double counter。write/into facade 使用等长 range，因此 defensive mismatch `EINVAL` 不可达。
- legacy inventory：完整 occurrences/file counts 写入 `ANALYSIS_SUMMARY.md`。M3 没有 handler call-site migration，因为 facade 内部桥接已覆盖所有既有调用；M4 guard 将冻结计数和 raw visibility。
- regression-test boundary：现有 host fixture 不能构造真实 `UserProcess`/`AddrSpace` permission、跨页与 partial-copy 状态；未添加会把 fake backend 误报为 shell 行为的测试。保留原 fault implementation，并由双架构 official-feature integration build、existing guard 和 generic boundary tests 覆盖当前可验证范围。
- unsafe delta：`user_memory.rs` 仍为 5 个 unsafe block；新增 0、删除 0、移动 0，diff 未触及这些表达式。generic `T: Copy` 风险未包装成 `UserPod`。
- Cargo.lock：0 byte diff，SHA-256 保持 `0f7b1d3135d88d007eca51ab853007a182b5a0d8291808e67d582723cd72c4c8`。
- known failures：仅三项已证明非 PR1 失败（全局 fmt BASELINE、LA workspace clippy ENVIRONMENT、axfs unittest BASELINE）；无 M3 regression。

## M4 — syscall audit metadata and PR1 static guard

### Focused checks and implementation iterations

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `rg --files vendor/linux-raw-sys/src` | source lookup | 2 | INVOCATION | 依赖不是 vendored 目录；只读查找无副作用，随后从 Cargo registry 实际路径核对 target-selected number 模块 |
| `cargo test --locked --offline -p orays-linux` | host | 0 | PASS | 9 passed；原 5 个 user/backend 测试及新增 4 个 syscall identity/availability/alias/六参数上限测试 |
| `cargo clippy --locked --offline -p orays-linux --all-features -- -D warnings` | host | 0 | PASS | metadata model 零 warning |
| `cargo clippy --locked --offline -p orays-linux --all-features --target riscv64gc-unknown-none-elf -- -D warnings` | RISC-V64 | 0 | PASS | target-selected ABI number namespace与 metadata model 编译通过 |
| `cargo clippy --locked --offline -p orays-linux --all-features --target loongarch64-unknown-none-softfloat -- -D warnings` | LoongArch64 | 0 | PASS | legacy number import 和通用 model 编译通过 |
| `rustfmt --edition 2024 --check api/orays_linux/src/lib.rs api/orays_linux/src/syscall.rs user/shell/src/uspace/mod.rs user/shell/src/uspace/syscall_metadata.rs` | touched Rust files | 0 | PASS | M4 Rust diff 格式正确 |
| `python3 scripts/check_pr1_linux_boundary.py` | static | 0 | PASS | 当前树 0 findings；最终源码重复执行仍通过 |
| `python3 scripts/test_pr1_linux_boundary.py`（首次） | mutation tests | 1 | REGRESSION | 13/14 通过；反向依赖 test 期待字符串与 guard 的准确诊断不一致，属于新测试断言缺陷，不是产品代码失败 |
| 同一 mutation test（修正断言后） | mutation tests | 0 | PASS | 14/14 通过；最终 `#[used] static` table 后再次重复执行仍为 14/14 |
| `make A=user/shell ARCH=riscv64 defconfig` 后 official-feature build（首次） | RISC-V64 | 0 | REGRESSION | 编译成功，但私有 const metadata table 新增 6 个 dead-code warning；未用 `allow` 掩盖，改为有意保留的 `#[used] static` table |
| 同一 RISC-V64 official-feature build（最终源码） | RISC-V64 | 0 | PASS | 完整编译/链接/objcopy；恢复为原有 12 个 shell warning，无 metadata warning |
| `make A=user/shell ARCH=loongarch64 defconfig` 后 official-feature build（最终源码） | LoongArch64 | 0 | PASS | 完整编译/链接/objcopy；原有 12 个 shell warning，无 metadata warning |

### Milestone matrix

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `cargo fmt --all -- --check` | workspace | 1 | BASELINE | 仍且只报告 M0 的 4 个无关文件：POSIX pipe、axfs dev/root、axtask wait_queue |
| `python3 scripts/check_g006_synthetic_capabilities.py` | static | 0 | PASS | 0 findings |
| `python3 scripts/check_g009_post_review_semantics.py` | static | 0 | PASS | 0 findings |
| `python3 scripts/check_g012_syscall_review_hotspots.py` | static | 0 | PASS | 0 findings；dispatcher 为零 diff |
| `python3 scripts/check_g013_user_copy_boundary.py` | static | 0 | PASS | 0 findings |
| `make clippy ARCH=riscv64`（先 RV defconfig） | RISC-V64 workspace | 0 | PASS | workspace clippy 通过；无新增 PR1 warning/error |
| `make clippy ARCH=loongarch64`（先 LA defconfig） | LoongArch64 workspace | 2 | ENVIRONMENT | 与 M0–M3 相同：`axlibc` bindgen 的 host libclang 报 unknown target triple `loongarch64-unknown-none` |
| `make unittest_no_fail_fast` | host | 2 | BASELINE | 与 M0–M3 相同：axfs FAT `test_devfs_ramfs() failed: NotFound`；其余 axfs targets 继续运行 |
| `cargo test --locked --offline --workspace --exclude axfs --no-fail-fast -- --nocapture` | host | 0 | PASS | 其余 workspace 全部通过，含 `orays-linux` 9 tests |
| `cargo metadata --locked --offline --format-version 1 --no-deps` | dependency graph | 0 | PASS | M4 无 manifest/lock 变化，offline/locked 解析通过 |
| `cargo tree --locked --offline -p arceos-shell -e normal --features uspace` | dependency graph | 0 | PASS | `arceos-shell -> orays-linux -> orays-linux-abi -> linux-raw-sys` |
| combined integrity wrapper with over-escaped `rg` unsafe pattern | invocation | 1 | INVOCATION | wrapper 把反斜杠重复传给 `rg`，报 repetition quantifier；无仓库副作用，随即用下一条 POSIX character class pattern 重跑 |
| `test "$(rg -n 'unsafe[[:space:]]*\{' user/shell/src/uspace/user_memory.rs \| wc -l)" -eq 5` | unsafe audit | 0 | PASS | 精确确认 M4 后仍为同 5 个 unsafe block |
| `git diff --check` | worktree | 0 | PASS | 无 whitespace error |
| `git diff --exit-code -- Cargo.lock user/shell/src/uspace/syscall_dispatch.rs api/arceos_posix_api/src/ctypes_gen.rs` | protected sources | 0 | PASS | M4 不改 lock/dispatcher；host test 的 packed epoll 生成副作用已精确恢复 |
| `sha256sum Cargo.lock .codex/CODEX_PR1_GOAL.md docs/pr1-linux-boundary-survey.md` | integrity | 0 | PASS | lock 为 M2 hash `0f7b1d31…`；两份用户输入仍为 `f6fb00c6…`/`b6b7911b…` |

### M4 audit

- changed files：`api/orays_linux/src/{lib,syscall}.rs`、`user/shell/src/uspace/{mod,syscall_metadata}.rs`、`scripts/{check,test}_pr1_linux_boundary.py` 与四份 `docs/pr1/*.md`。
- executable behavior：`syscall_dispatch.rs`、所有 handler、TrapFrame 参数读取、errno/return、watchdog/accounting/restart 路径均为零 diff。metadata 只保存 number/name/count/availability/handler-name/alias/audit-id，不含可调用 handler。
- target facts：RV 与 LA 都登记 clone/fsync/fdatasync；RV rlimit 使用 general 163/164，LA 使用已抽取 legacy 163/164；poll 只在现有 non-RV/non-AArch64/non-LA cfg 登记。guard 对 LA clone 的 arg3/arg4 交换做源码核对。
- dependency/ABI：manifest 和 lock 为零 diff；最终链不变，无 cycle。ABI number 与 `Tms`/`RtcTime` layout assertions 由 guard 再冻结。
- unsafe delta：新增 0、删除 0、移动 0；两个边界 crate 继续 `forbid(unsafe_code)`；`user_memory.rs` 仍为同 5 个 audited unsafe fingerprint。
- test honesty：static mutation tests 证明 guard 对预设漂移有检测能力，但不宣称 syscall runtime PASS。中途 1 个测试断言 regression 与 6 个 warning 均在提交前修复；无未解决 M4 regression。
- known failures：仅全局 fmt BASELINE、LA workspace clippy ENVIRONMENT 和 axfs unittest BASELINE。

## M5-R1 — independent review minor follow-up

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `python3 scripts/check_pr1_linux_boundary.py` | static | 0 | PASS | 当前树 0 findings；依赖扫描含普通与 target-specific table |
| `python3 scripts/test_pr1_linux_boundary.py` | mutation tests | 0 | PASS | 15/15；新增 target-specific 反向依赖用例通过 |
| `python3 -m py_compile scripts/check_pr1_linux_boundary.py scripts/test_pr1_linux_boundary.py` | Python syntax | 0 | PASS | 两个脚本可编译 |
| `cargo fmt -p orays-linux -- --check` | touched Rust crate | 0 | PASS | `SyscallArgs` 文档变更格式正确 |
| `cargo test --locked --offline -p orays-linux` | host | 0 | PASS | 9/9 |
| `cargo clippy --locked --offline -p orays-linux --all-targets -- -D warnings` | host | 0 | PASS | host 全 target 零 warning |
| 同一命令加 `--target riscv64gc-unknown-none-elf` | RISC-V64 | 101 | INVOCATION | 裸机 target 没有 `libtest`；`--all-targets` 不适用，随后用 library-only 调用重跑 |
| 同一命令加 `--target loongarch64-unknown-none-softfloat` | LoongArch64 | 101 | INVOCATION | 同上；不是源码错误 |
| `cargo clippy --locked --offline -p orays-linux --target riscv64gc-unknown-none-elf -- -D warnings` | RISC-V64 | 0 | PASS | library target 零 warning |
| `cargo clippy --locked --offline -p orays-linux --target loongarch64-unknown-none-softfloat -- -D warnings` | LoongArch64 | 0 | PASS | library target 零 warning |
| `cargo fmt --all -- --check` | workspace | 1 | BASELINE | 仍只命中 M0 的四个无关文件 |
| `git diff --check` | worktree | 0 | PASS | 无 whitespace error |

审查修复白名单为 `api/orays_linux/src/syscall.rs`、两个 PR1 guard 脚本及四份 PR1 文档。
没有 manifest/lock/dispatcher/handler 变化；unsafe 新增、删除、移动均为 0。

## M5 — final integration validation

### Boundary crates and static tests

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `cargo check --locked --offline -p orays-linux-abi` 及 RV/LA `--target` 变体 | host/RISC-V64/LoongArch64 | 0/0/0 | PASS | `m5-check-abi-{host,rv,la}.log` |
| `cargo check --locked --offline -p orays-linux` 及 RV/LA `--target` 变体 | host/RISC-V64/LoongArch64 | 0/0/0 | PASS | `m5-check-linux-{host,rv,la}.log` |
| `cargo clippy --locked --offline -p orays-linux-abi ... -- -D warnings` | host/RISC-V64/LoongArch64 | 0/0/0 | PASS | 三目标零 warning；对应 `m5-clippy-abi-*.log` |
| `cargo clippy --locked --offline -p orays-linux ... -- -D warnings` | host/RISC-V64/LoongArch64 | 0/0/0 | PASS | 三目标零 warning；正确 cross-target 调用不含 `--all-targets`；对应 `m5-clippy-linux-*.log` |
| `cargo test --locked --offline -p orays-linux-abi` | host | 0 | PASS | 0 runtime tests，const number/layout assertion 编译；`m5-test-abi.log` |
| `cargo test --locked --offline -p orays-linux` | host | 0 | PASS | 9/9；`m5-test-linux.log` |
| `python3 scripts/check_g006_synthetic_capabilities.py` / mutation tests | static | 0/0 | PASS | guard PASS，5/5 tests |
| `python3 scripts/check_g009_post_review_semantics.py` / mutation tests | static | 0/0 | PASS | guard PASS，15/15 tests |
| `python3 scripts/check_g012_syscall_review_hotspots.py` | static | 0 | PASS | guard PASS；dispatcher 未修改 |
| `python3 scripts/test_g012_syscall_review_hotspots.py` | mutation tests | 1 | BASELINE | 25/26；`test_detects_empty_central_user_trace` 在 `git archive e7ad4862` 的精确临时树同样失败；`m5-test-g012-baseline.log` |
| `python3 scripts/check_g013_user_copy_boundary.py` / mutation tests | static | 0/0 | PASS | guard PASS，13/13 tests |
| `python3 scripts/check_pr1_linux_boundary.py` / mutation tests | static | 0/0 | PASS | 最终 guard PASS，15/15 tests；target-specific reverse edge 已覆盖 |

### Workspace and architecture integration

| 命令 | 架构/target | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| `cargo fmt --all -- --check` | workspace | 1 | BASELINE | 仍只命中 M0 的 POSIX pipe、axfs dev/root、axtask wait_queue 四个文件；`m5-fmt-workspace.log` |
| 对 PR1 touched Rust 文件执行 `rustfmt --edition 2024 --check` | touched files | 0 | PASS | `m5-fmt-touched.log`；M5-R1 后 `cargo fmt -p orays-linux -- --check` 亦为 0 |
| `make A=user/shell ARCH=riscv64 FEATURES=alloc,paging,irq,multitask,fs,net,rtc APP_FEATURES=auto-run-tests,uspace build` | RISC-V64 | 0 | PASS | 完整编译/链接/objcopy；既有 12 shell warnings；`m5-build-riscv64.log` |
| 同一 official-feature build，`ARCH=loongarch64` | LoongArch64 | 0 | PASS | 完整编译/链接/objcopy；既有 12 shell warnings；`m5-build-loongarch64.log` |
| `make A=user/shell ARCH=riscv64 defconfig && make ARCH=riscv64 A=user/shell` | canonical Make default | 0 | PASS | 根 Makefile 默认 `all` 实际依次构建 RV/LA remote-style kernel；`m5-make-shell-riscv64-default.log` |
| `make A=user/shell ARCH=loongarch64 defconfig && make ARCH=loongarch64 A=user/shell` | canonical Make default | 0 | PASS | 同一默认 `all` 路径再次从 LA defconfig 验证；`m5-make-shell-loongarch64-default.log` |
| `make clippy ARCH=riscv64` | RISC-V64 workspace | 0 | PASS | `m5-clippy-riscv64-workspace.log` |
| `make clippy ARCH=loongarch64` | LoongArch64 workspace | 2 | ENVIRONMENT | axlibc bindgen 的 host libclang 不识别 `loongarch64-unknown-none`，与 M0 相同；`m5-clippy-loongarch64-workspace.log` |
| `make unittest_no_fail_fast` | host | 2 | BASELINE | axfs FAT `test_devfs_ramfs() failed: NotFound`，与 M0 相同；`m5-unittest-no-fail-fast.log` |
| `cargo test --locked --offline --workspace --exclude axfs --no-fail-fast -- --nocapture` | host | 0 | PASS | 排除唯一已证明基线失败后 workspace tests 通过；`m5-workspace-exclude-axfs.log` |
| `make kernel-rv` / `make kernel-la` | local QEMU RV/LA configs | 0/0 | PASS | 本地 QEMU 默认配置的 kernel target 构建通过；`m5-kernel-{rv,la}-official.log` |

### QEMU smoke with official images

官方镜像只作为 qcow2 backing file；运行写入 `/tmp/orays-pr1-final/*.qcow2`，不写源镜像。

| 命令 | 架构 | exit | 分类 | 证据/说明 |
|---|---|---:|---|---|
| 无 block device 的默认 QEMU probe | RISC-V64 | 0 | INVOCATION | guest panic `No block device found!`；不是 PASS；`m5-qemu-riscv64-boot.log` |
| 加临时 FAT block、未加 NIC 的 probe | RISC-V64 | 0 | INVOCATION | guest panic `No NIC device found!`；不是 PASS；`m5-qemu-riscv64-smoke.log` |
| `timeout` + 临时 FAT + VirtIO net 的 shell smoke | RISC-V64 | 124 | BOUNDED | 到达 `OrayS:/$` 后由 timeout 终止；`m5-qemu-riscv64-smoke-net.log` |
| 同一 shell smoke | LoongArch64 | 124 | BOUNDED | 到达 `OrayS:/$` 后由 timeout 终止；`m5-qemu-loongarch64-smoke.log` |
| `timeout -k 3s 60s make run-rv RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img RV_TESTSUITE_RUN_IMG=/tmp/orays-pr1-final/m5-official-rv.qcow2` | RISC-V64 | 124 | BOUNDED | 进入 Ext4/net/`ltp-musl`；27 个 case wrapper END，第 28 个 `getegid01` 开始后截断；一条 brk01 libc TCONF；无 TFAIL/TBROK/ENOSYS/非零 summary/panic/trap；`m5-run-rv-official-smoke.log` |
| `timeout -k 3s 60s make run-la LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img LA_TESTSUITE_RUN_IMG=/tmp/orays-pr1-final/m5-official-la.qcow2` | LoongArch64 | 124 | BOUNDED | 23 个 case wrapper END，第 24 个 `getppid01` 开始后截断；同一 brk01 libc TCONF；其余不利 marker 同样未见；`m5-official-la.log` |
| `make kernel-rv`（起始提交 `e7ad4862` 的 `git archive` 临时树） | RISC-V64 baseline | 0 | PASS | 精确基线构建；`m5-baseline-kernel-rv.log` |
| `qemu-img create ...` 后 `timeout -k 3s 25s qemu-system-riscv64 ...`（同一官方 RV backing image） | RISC-V64 baseline | 124 | BOUNDED | 99 个 case wrapper END，第 100 个开始后截断；复现同一 brk01 libc TCONF，syscall variant TPASS；无 TFAIL/TBROK/ENOSYS/非零 summary；`m5-baseline-official-rv.log` |

两次 60 秒命令的时限包含约 43–46 秒增量构建，所以 case 数不能用于跨架构性能比较；也不能把
截断窗口外的 994-case 列表、glibc 或其他官方组记为通过。

### Dependency, unsafe, lock, and review audit

| 命令/审计 | exit | 分类 | 证据/说明 |
|---|---:|---|---|
| `cargo metadata --locked --offline --format-version 1 --no-deps` | 0 | PASS | workspace 含两个 path crate，无解析 cycle |
| `cargo tree --locked --offline -p orays-linux-abi -e normal` | 0 | PASS | ABI → `linux-raw-sys` |
| `cargo tree --locked --offline -p orays-linux -e normal` | 0 | PASS | Linux → ABI → `linux-raw-sys` |
| `cargo tree --locked --offline -p arceos-shell -e normal --features uspace` | 0 | PASS | 同时显示 shell → Linux → ABI 链及 shell 的既存 `linux-raw-sys` 直连；无反向边 |
| `git diff --check e7ad4862..HEAD` / `git diff --check` | 0/0 | PASS | commit range 与最终 worktree 均无 whitespace error |
| tracked Rust `unsafe {` inventory | 0 | PASS | 起点 501、最终 501；`user_memory.rs` 5→5；两个 boundary crate 为 0；忽略的 bindgen 输出不计 tracked source |
| `git diff e7ad4862 -- Cargo.lock` 与 SHA-256 | 0 | PASS | 仅 shell→Linux、Linux→ABI、ABI→`linux-raw-sys` path edge/stanza；15 insertions，无 registry version/checksum 变化；最终 `0f7b1d3135d88d007eca51ab853007a182b5a0d8291808e67d582723cd72c4c8` |
| `git diff --exit-code e7ad4862..HEAD -- user/shell/src/uspace/syscall_dispatch.rs` | 0 | PASS | dispatcher 零 diff |
| 两轮独立只读 review | 0 | PASS | 首轮 0 blocker/0 major/3 minor；`9de38988` 后复核 0/0/0 |
| `sha256sum .codex/CODEX_PR1_GOAL.md docs/pr1-linux-boundary-survey.md` | 0 | PASS | 分别保持 `f6fb00c6…` 与 `b6b7911b…`；未跟踪、未提交 |

### M5 final audit

- PR1 range：24 个 tracked files；2865 insertions/203 deletions。实现提交没有
  syscall behavior、errno、blocking、signal、process、FD、VM、IPC 或 scheduling 语义修改。
- commits：`1b3dc605`、`940438f7`、`7357d56c`、`f7d0a5a5`、`a5703bfd`、`9de38988`，再加本次
  final validation 文档提交；均为本地提交，未 push、未创建远端 PR。
- generated side effect：host tests 曾改写 `api/arceos_posix_api/src/ctypes_gen.rs`，已用精确补丁恢复；
  最终 tracked worktree 只允许本次四份 PR1 文档 diff。
- unresolved PR1 regressions：0。已知非零只有已证明的 BASELINE、ENVIRONMENT、INVOCATION，或按
  设计 timeout 的 BOUNDED smoke；没有把它们写成完整 PASS。

## 后续记录模板

每个里程碑追加：commit SHA、逐条命令/exit/classification、changed files、已知失败、unsafe block 增减/移动、Cargo.lock diff 原因、双架构证据和工作树状态。不得用一次默认 build 代替 uspace official-feature build。
