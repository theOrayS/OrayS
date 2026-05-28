# 本地/远程评测日志整理

本页整理远程提交构建、本地 QEMU 评测、离线 vendor/helper 和远程输出诊断相关日志。目标是让开发者知道哪些结论已经被证据支持，哪些路径不能混用。

## 1. 单分支本地/远程评测策略

当前维护策略：`/root/oskernel2026-orays` 是本地 QEMU 验证和远程提交构建的同一个主工作树。

- 本地验证入口：`./run-eval.sh rv`、`./run-eval.sh la`。
- 远程提交构建入口：`make all`，生成根目录 `kernel-rv` / `kernel-la`。
- 本地 LoongArch 默认配置与远程提交 LoongArch 配置不同；远程 `kernel-la` 使用 `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`。

关键证据：

- `docs/remote-local-eval-unification-2026-05-22/final-terminal-report.md`
- `docs/remote-local-eval-unification-2026-05-22/remote-offline-terminal-report.md`
- `docs/remote-local-eval-unification-2026-05-22/final-gate-quality-gate.json`

## 2. LoongArch 远程地址映射修复

问题：远程 LoongArch 高半区入口和本地入口不同。旧 boot 页表硬编码 `BOOT_PT_L0[0]`，能覆盖本地 `0xffff_0000_8000_0000`，但不能覆盖远程 `0xffff_8000_8000_0000`。

结论：启动页表 L0 槽位必须由 `KERNEL_BASE_VADDR` 动态计算。

证据摘要：

- 远程 LA 构建配置：`make test_build ARCH=loongarch64 BUS=pci PLAT_CONFIG=configs/remote-eval/...` 通过，入口为 `0xFFFF800080000000`。
- 本地 LA 构建配置：`make test_build ARCH=loongarch64 BUS=pci` 通过，入口为 `0xFFFF000080000000`。
- `./run-eval.sh la` 和 `./run-eval.sh rv` 在该阶段均通过；当时 LTP musl/glibc 各 157 passed / 0 failed。

首选阅读：`docs/remote-local-eval-unification-2026-05-22/final-terminal-report.md`。

## 3. 远程离线构建修复

问题：远程评测机可能没有 DNS/网络；原构建流程会尝试在线安装 `cargo-axplat` 和 `axconfig-gen`，导致离线失败。

修复方向：

- `Makefile` 优先使用仓库内 helper。
- `scripts/make/deps.mk` 移除远程构建时的在线安装路径，缺工具时快速报错。
- `tools/bin/` 提供 `cargo-axplat`、`axconfig-gen`、`rust-objcopy` shim。
- `cargo-home/config.toml` 和 `vendor/cargo-vendor.tar.gz` 支撑非隐藏、离线 Cargo 源。

关键验证：

```bash
CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH cargo metadata --locked --offline --format-version=1
PATH=$PWD/tools/bin:$PATH make -n all
PATH=$PWD/tools/bin:$PATH make all
CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all KERNEL_BUILD_DIR=/tmp/oskernel-remote-offline-build KERNEL_RV=/tmp/oskernel-remote-offline-build/kernel-rv KERNEL_LA=/tmp/oskernel-remote-offline-build/kernel-la
```

首选阅读：`docs/remote-local-eval-unification-2026-05-22/remote-offline-terminal-report.md`。

## 4. 远程输出与本地输出对比

远程输出文件如 `Riscv输出.txt`、`LoongArch输出.txt` 可能被平台截断，出现类似 `...超过1MB的部分被截断...` 的标记时，只能说明“已捕获片段与本地一致”，不能声称完整等价。

开发者应先做三件事：

1. 检查截断标记。
2. 用 `scripts/ltp_summary.py` 解析可见片段。
3. 检查 marker 是否从列 0 开始，以及 `bad_marker_prefix` 是否为 0。

相关证据：

- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/raw/LoongArch-remote-summary.json`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/raw/Riscv-remote-summary.json`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/raw/remote-output-noise-baseline.json`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/remote-marker-and-log-noise-regression-check.md`

## 5. 开发者行动规则

- 改远程提交链路：至少跑 `make all`；若涉及离线依赖，补跑离线风格构建。
- 改本地 evaluator runtime：至少跑受影响架构的 `./run-eval.sh` 或 targeted LTP batch。
- 改 LoongArch boot/config：明确说明本地 LA 与远程 LA 使用的 `KERNEL_BASE_VADDR` / `PLAT_CONFIG`。
- 不要重新维护独立 remote 分支作为默认交付目标；历史 remote 分支只当只读参考。
