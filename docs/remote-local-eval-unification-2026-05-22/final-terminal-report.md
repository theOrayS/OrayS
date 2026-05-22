# 最终简要报告（2026-05-22）

- 已修复远程最新 `axconfig-gen is unavailable`：关键离线 helper 现在默认从 `scripts/` 调用，`vendor/bin/` 仅作为依赖副本保留。
- Makefile 当前默认：`AXCONFIG_GEN=python3 scripts/axconfig-gen.py`，`RUST_OBJCOPY=sh scripts/rust-objcopy.sh`，`CARGO_AXPLAT=sh scripts/cargo-axplat.sh`。
- 已临时移走 `vendor/bin/` 和 `tools/bin/` 进行验证，仅靠 `scripts/` helper 运行远程同款 RISC-V `make test_build ARCH=riscv64 BUS=mmio ...`，实际构建通过并生成 ELF `kernel-rv`。
- 构建日志无 `axconfig-gen is unavailable`、`cargo-axplat is unavailable`、在线安装、crates 下载、DNS 失败或 registry 更新标记。
- 离线依赖仍以 `vendor/cargo-vendor.tar.gz` 源码归档提交，`scripts/ensure-cargo-vendor.sh` 构建时恢复 `vendor/cargo/`；最终工作树不保留展开的 `vendor/cargo/`。
- 远程/本地评测分流保持不变：本地 `./run-eval.sh` / `./run-eval.sh la` 使用本地配置，远程 `make all` 的 LoongArch 使用 `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`。
- 未修改 `refactor/moss_kernel_like_remote` 分支；没有引入假 PASS、硬编码用例结果或把真实失败改成 SKIP。
