# 仓库地图

本仓库保留了上游 ArceOS 的 workspace 结构，并在其上叠加 OSKernel 2026 评测所需的启动、用户态、测试运行和远程提交支持。下面这些路径是多数开发任务的第一检查点。

## 源码布局

| 路径 | 开发含义 |
| --- | --- |
| `kernel/arch/axhal/` | 架构与平台 HAL。启动、trap、页表、时钟和设备相关代码在这里。 |
| `kernel/runtime/axruntime/` | runtime 初始化和启动流程。这里的改动可能影响所有架构。 |
| `kernel/task/axtask/` | 任务、调度、进程/线程生命周期支持。 |
| `kernel/fs/axfs/` | VFS 和文件系统操作，供 POSIX syscall 与评测用例使用。 |
| `kernel/net/axnet/` | TCP/UDP 网络栈集成。 |
| `api/arceos_posix_api/` | Linux/POSIX syscall 边界、errno 映射、用户内存校验、进程/FD/signal/futex/mmap 语义。 |
| `api/arceos_api/`, `api/axfeat/` | ArceOS 公共 API 与 feature 组合。 |
| `ulib/axstd/`, `ulib/axlibc/` | 用户库和 libc 相关支持。 |
| `examples/shell/` | shell、用户程序启动器、官方评测组 runner 和 LTP runner。 |
| `configs/platforms/` | 常规本地平台配置。 |
| `configs/remote-eval/` | 远程评测专用平台配置，目前主要用于 LoongArch 提交构建。 |
| `scripts/` | 构建 shim、Makefile 片段和日志解析工具，例如 `scripts/ltp_summary.py`。 |
| `tools/bin/` | 仓库内置 helper 命令，优先于用户环境里的同名工具。 |
| `cargo-home/`, `vendor/` | 离线/vendor 源支持，用于远程提交构建。 |
| `docs/` | 兼容性记录、LTP 阶段报告、最终门禁证据和本开发者指南。 |
| `doc/` | 更接近上游 ArceOS 风格的平台与构建文档。 |

## 评测集成热路径

OSKernel 相关工作通常沿着下面这条路径展开：

```text
run-eval.sh
  -> Makefile run-rv / run-la
    -> kernel-rv / kernel-la 构建目标
      -> 带 APP_FEATURES=auto-run-tests,uspace 的 examples/shell
        -> api/arceos_posix_api syscall / 用户态边界
          -> fs / task / mm / net / hal 等 kernel 子系统
```

因此，`examples/shell` 不是普通示例程序。它负责 staging busybox/helper 资源、启动官方测试组、选择 LTP case、输出 wrapper marker、施加 per-case timeout，并在 case 之间清理用户进程。

## 生成物和本地临时产物

除非任务明确要求保留生成证据，否则不要把下面这些文件当作源码修改对象：

- `kernel-rv`, `kernel-la`
- `sdcard-*.img`, `disk*.img`, `*.qcow2`
- `output*.md`, 原始 `*.log`
- `.axconfig.toml`, `build/`, `target/`

如果需要保存评测证据，优先放入 `docs/` 下的日期目录，并让 raw log 与 `scripts/ltp_summary.py` 输出成对出现。

## 子系统定位提示

- syscall 返回值、errno、ABI 结构体和 raw user pointer 通常属于 `api/arceos_posix_api/`。
- 测例选择、helper staging、wrapper 输出和 LTP case-list 行为通常属于 `examples/shell/src/cmd.rs`。
- VFS/FD 行为可能横跨 `api/arceos_posix_api/`、`kernel/fs/axfs/` 和 shell 侧 helper/resource staging。
- process、wait、signal 和 scheduler 行为可能横跨 `api/arceos_posix_api/`、`kernel/task/axtask/`、runtime 和架构 trap 代码。
- LoongArch 启动以及本地/远程地址映射问题可能横跨 `kernel/arch/axhal/`、平台配置和 vendored platform/boot crate。
