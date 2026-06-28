# OSKernel 2026 ArceOS 评审分支

本仓库是基于 ArceOS 的 OSKernel 2026 评审/提交分支。默认说明、交付说明和维护约定均使用中文；只有上游原始文档、第三方代码或工具输出保留原语言。

本分支目标是交付一个可复现、可评审、不过度携带历史日志的源码树：保留真实内核、用户态和评测入口；不追踪本地评测输出、归档日志、镜像和构建产物；所有 LTP/兼容性结论以源码、脚本和真实输出为准。

## 当前交付范围

- **架构**：RISC-V 与 LoongArch 是远程评测主路径；仓库仍保留 x86_64、AArch64 等 ArceOS 平台支持。
- **远程提交入口**：`make all` 生成仓库根目录下的 `kernel-rv` 和 `kernel-la`。
- **本地验证入口**：`./run-eval.sh rv`、`./run-eval.sh la` 分别运行本地 QEMU 评测路径。
- **用户态/系统调用边界**：`api/arceos_posix_api/`、`ulib/`、`examples/shell/` 负责 POSIX/Linux 可见行为、ELF 加载、进程、文件描述符、信号、futex、mmap 和 LTP runner。
- **LTP stable 集合**：`examples/shell/src/cmd.rs::LTP_STABLE_CASES` 当前包含 1000 个唯一 case；runner 会分别对 `/musl` 和 `/glibc` 执行所选 case。

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

请先准备官方评测镜像/SD 卡镜像，然后运行：

```bash
./run-eval.sh rv
./run-eval.sh la
```

本地 LoongArch QEMU 与远程官方评测机的地址映射可能不同；判断 LoongArch 远程结果时，应优先以远程 `make all` 提交构建和官方评测输出为准。

### 常用静态/单元检查

```bash
python3 scripts/test_ltp_summary.py
python3 scripts/test_g008_musl_patch_stable.py
git diff --check
```

完整构建、QEMU 和 evaluator 运行时间较长，运行前后建议检查磁盘空间：

```bash
df -h / /root
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
| `scripts/fixtures/` | 仍被回归测试使用的小型固定输入；这些文件需要随仓库追踪。 |
| `tools/bin/` | 仓库内置辅助工具，优先支持离线/提交环境。 |
| `cargo-home/` | 非隐藏 Cargo home，用于 vendored/offline source replacement。 |
| `vendor/` | 本地 crate patch 与离线 vendor 归档。 |
| `docs/agent-workflow/` | agent 工作流、验证、分支和协作规则。 |
| `docs/ltp-full-sweep-blacklist-2026-05-30-arch/` | Makefile 仍引用的 LTP blacklist 输入，不能随意移动。 |
| `doc/` | 上游风格 ArceOS 文档和平台说明。 |

## 不追踪的本地文件

以下内容可能在本地存在，但不属于评审源码树的 source of truth：

- `eval-reports/`
- `archive/docs-pre-review-2026-06-28/`
- `.local-archive/`
- `.codegraph/`
- `output/`
- `complete.md`
- `output*.md`
- `*.log`
- `kernel-rv`、`kernel-la`
- `sdcard-*.img`、`disk*.img`
- `build/`、`target/`

如果需要保留历史输出，请放在被忽略的本地归档目录；不要把旧日志、评测输出或大文件重新加入 Git。

## 竞赛合规红线

评审分支必须保持真实语义，禁止通过伪造输出或硬编码测例来换取分数：

- 不得硬编码 LTP case 名、路径、进程名或二进制特征。
- 不得伪造 `TPASS`、wrapper PASS 或隐藏 `TCONF`、`TBROK`、`TFAIL`、`ENOSYS`、timeout、panic、trap。
- 不得修改 testsuite 或 evaluator 脚本来制造通过。
- 不得牺牲 Linux/POSIX 主要语义、权限检查、资源检查或安全边界。
- blacklist 只能用于隔离会卡死、炸内存、破坏评测器或明显不适合当前内核模型的 case；被 blacklist 的 case 不能计为通过。

## 交付建议

评审前建议确认：

```bash
git status --short --branch
git ls-files --others --exclude-standard
make all
```

若只做文档/路径整理，可用 `git diff --check` 和相关脚本测试作为最小验证；若改动 syscall、ABI、runner 或 Makefile，必须补充对应构建、QEMU 或 parser-backed LTP 证据。
