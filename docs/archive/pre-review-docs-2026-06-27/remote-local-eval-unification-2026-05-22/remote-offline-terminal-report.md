# 远程离线编译故障修复简报（2026-05-22）

## 根因

远程评测机没有可用 DNS/网络，但原构建流程在 `scripts/make/deps.mk` 中会尝试在线 `cargo install cargo-axplat` 和 `axconfig-gen`。下载失败后 `cargo axplat` 不存在，`PLAT_CONFIG` 解析为空，最终触发 `PLAT_CONFIG= is not a valid platform configuration file`。

## 已完成修改

- `Makefile`：优先使用仓库内 `tools/bin` helper；存在 `cargo-home/config.toml` 时使用非隐藏 `cargo-home`；增加 `PLATFORM_CONFIG_DIR`。
- `scripts/make/deps.mk`：移除远程构建时的在线安装路径，缺工具时快速报清晰错误。
- `scripts/make/platform.mk`：平台配置优先从显式 `PLAT_CONFIG` 或 `configs/platforms/*.toml` 解析。
- `tools/bin/`：新增 `cargo-axplat`、`axconfig-gen`、`rust-objcopy` 源码 shim。
- `configs/platforms/`：新增本地默认平台配置离线副本。
- `cargo-home/config.toml` + `vendor/cargo/`：新增非隐藏、离线 Cargo 源码闭包。
- `AGENTS.md` 和 `docs/remote-local-eval-unification-2026-05-22/repair-plan.md`：补充远程离线构建规则和验证记录。

## 验证结果

- 离线依赖闭包：`CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH cargo metadata --locked --offline --format-version=1` 通过。
- 配置路径 dry-run：`PATH=$PWD/tools/bin:$PATH make -n all` 通过，未出现在线安装命令。
- 普通提交构建：`PATH=$PWD/tools/bin:$PATH make all` 通过。
- fresh 离线提交构建：`CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all KERNEL_BUILD_DIR=/tmp/oskernel-remote-offline-build KERNEL_RV=/tmp/oskernel-remote-offline-build/kernel-rv KERNEL_LA=/tmp/oskernel-remote-offline-build/kernel-la` 通过。
- fresh 离线产物：`/tmp/oskernel-remote-offline-build/kernel-rv` 与 `/tmp/oskernel-remote-offline-build/kernel-la` 均为 ELF。
- fresh 离线日志扫描：无 `Installing cargo-axplat`、`Installing axconfig-gen`、`Downloading crates`、`Could not resolve`、`Updating ... index`。

## 行为影响

- 不改变 syscall、errno、ABI 或 LTP 运行逻辑。
- 不伪造 PASS、不 hardcode case 结果、不隐藏真实失败。
- 远程 `make all` 继续生成远程 LA 地址映射的 `kernel-la`；本地 `./run-eval.sh` / `./run-eval.sh la` 入口不变。
