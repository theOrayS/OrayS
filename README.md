# OSKernel 2026 ArceOS 参赛内核

本仓库基于 ArceOS，面向 OSKernel 2026 评测环境维护。代码树包含内核运行时、RISC-V/LoongArch 平台配置、用户态 POSIX/Linux 兼容层、shell runner 以及本地和远程评测入口。

## 项目概览

- **目标架构**：RISC-V 与 LoongArch 是远程评测主路径；仓库同时保留 ArceOS 的 x86_64、AArch64 等平台支持。
- **远程构建入口**：`make all` 在仓库根目录生成 `kernel-rv` 和 `kernel-la`。
- **本地验证入口**：`./run-eval.sh rv`、`./run-eval.sh la` 分别运行 RISC-V 和 LoongArch 本地 QEMU 评测路径。
- **用户态兼容层**：`api/arceos_posix_api/`、`ulib/`、`examples/shell/` 实现 syscall 边界、C/Rust 用户库、ELF 加载、进程、文件描述符、信号、futex、mmap 和 LTP runner。
- **LTP runner**：`examples/shell/src/cmd.rs::LTP_STABLE_CASES` 维护 stable case 集合，runner 会分别对 `/musl` 和 `/glibc` 执行所选 case。

## 快速开始

### 构建远程评测内核

```bash
make all
```

生成文件：

```text
kernel-rv
kernel-la
```

也可以单独构建：

```bash
make kernel-rv
make kernel-la
```

### 运行本地评测

准备官方评测镜像/SD 卡镜像后运行：

```bash
./run-eval.sh rv
./run-eval.sh la
```

### 常用检查

```bash
python3 scripts/test_ltp_summary.py
python3 scripts/test_g008_musl_patch_stable.py
git diff --check
```

## 目录说明

| 路径 | 说明 |
| --- | --- |
| `kernel/` | ArceOS 内核运行时、HAL、内存、任务、驱动、文件系统、网络和同步模块。 |
| `api/arceos_posix_api/` | Linux/POSIX ABI 可见的 syscall 与用户态集成边界。 |
| `ulib/` | 用户库，包括 `axstd`、`axlibc` 等。 |
| `examples/shell/` | shell、官方分组 runner、LTP runner 和评测集成入口。 |
| `configs/` | 默认平台、自定义平台和远程评测平台配置。 |
| `scripts/` | 构建辅助、评测辅助、LTP 汇总和回归检查脚本。 |
| `tools/` | 仓库内置辅助工具和板级资源。 |
| `cargo-home/` | Cargo 离线 source replacement 配置。 |
| `vendor/` | 本地 crate patch 与离线 vendor 归档。 |
| `doc/` | ArceOS 平台说明和上游风格文档。 |
