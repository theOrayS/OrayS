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

## 后续记录模板

每个里程碑追加：commit SHA、逐条命令/exit/classification、changed files、已知失败、unsafe block 增减/移动、Cargo.lock diff 原因、双架构证据和工作树状态。不得用一次默认 build 代替 uspace official-feature build。
