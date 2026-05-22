# 最终简要报告（2026-05-22）

- 已修复远程 LoongArch 高半区取指异常：根因是 vendored `axplat-loongarch64-qemu-virt/src/boot.rs` 启动页表硬编码 `BOOT_PT_L0[0]`，只能覆盖本地 `0xffff_0000_8000_0000`，不能覆盖远程 `0xffff_8000_8000_0000`。
- 现在启动页表 L0 槽位由 `KERNEL_BASE_VADDR` 动态计算；本地 LA 仍入口 `0xFFFF000080000000`，远程 LA 入口 `0xFFFF800080000000`。
- 修复写入 `vendor/cargo-vendor.tar.gz` 中的源码归档，并更新 crate checksum；最终不提交展开的 `vendor/cargo/`。
- 远程离线构建链保持：`cargo-home/config.toml` + `vendor/cargo-vendor.tar.gz` + `scripts/ensure-cargo-vendor.sh`，不会在评测机 `make all` 中联网安装依赖。
- 验证已完成：
  - 远程 LA 构建配置：`make test_build ARCH=loongarch64 BUS=pci PLAT_CONFIG=configs/remote-eval/...` 通过，入口 `0xFFFF800080000000`。
  - 本地 LA 构建配置：`make test_build ARCH=loongarch64 BUS=pci` 通过，入口 `0xFFFF000080000000`。
  - `./run-eval.sh la`：通过；LTP musl/glibc 各 `157 passed, 0 failed, 0 timed out`；`scripts/ltp_summary.py` 统计 PASS 314 / FAIL 0 / timeout 0 / ENOSYS 0 / panic 0。
  - `./run-eval.sh`：通过；LTP musl/glibc 各 `157 passed, 0 failed, 0 timed out`；`scripts/ltp_summary.py` 统计 PASS 314 / FAIL 0 / timeout 0 / ENOSYS 0 / panic 0。
- 未修改 `refactor/moss_kernel_like_remote` 分支；没有引入假 PASS、硬编码用例结果或把真实失败改成 SKIP。
- 用户可见变化：远程 `make all` 生成的 LoongArch `kernel-la` 应可进入远程评测机测评；本地 `./run-eval.sh la` 继续使用本地地址映射。
- syscall / errno / ABI 可见变化：无预期变化。
