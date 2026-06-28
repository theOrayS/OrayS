# 从 ArceOS 到 OSKernel 2026 参赛内核：工作量与技术亮点说明

> 面向评审的项目说明文档。本文基于 `release/20260628` 分支当前源码、构建脚本和仓库内验证工具撰写；目标不是简单罗列“修了多少测例”，而是说明我们如何把一个 ArceOS 起点工程，重构、扩展并验证成面向 OSKernel 2026 官方评测的双架构 Linux/POSIX 兼容内核。

## 0. 一句话概括

我们的工作可以概括为：**以 ArceOS 为底座，参考 Moss Kernel 的工程分层思路，重构出更接近通用内核的目录与边界；在此基础上补齐 Linux/POSIX 用户态、进程、内存、文件系统、网络、同步、IPC、信号、时间和官方评测闭环，使其从“能跑示例的模块化 unikernel”演进为“可在 RISC-V / LoongArch 双架构官方环境中持续接受 LTP、libc、BusyBox、lmbench、iozone、UnixBench 等压力测试的比赛内核”。**

这份工作的核心价值不只是“跑分”，而是把每个分数背后的语义、失败、回归风险和复现路径都工程化、显式化、可审查化。

## 1. 评委最应该看到的五个亮点

### 1.1 我们做了真正的内核工程重构，而不是在示例 shell 上打补丁

上游 ArceOS 的模块主要集中在 `modules/ax*`。本项目将这些模块按内核职责重新组织到 `kernel/` 下：

| 新目录 | 代表能力 | 说明 |
| --- | --- | --- |
| `kernel/arch/axhal` | 架构与 HAL | 保留多架构 HAL，同时服务 RISC-V / LoongArch 评测主路径。 |
| `kernel/config/axconfig` | 配置系统 | 支撑本地 QEMU、远程官方环境和不同平台地址映射。 |
| `kernel/diagnostics/axlog` | 诊断日志 | 用于运行时定位 panic、trap、资源耗尽和 evaluator 异常。 |
| `kernel/drivers/*` | 驱动层 | 块设备、网络、显示、DMA 等 ArceOS 设备层能力。 |
| `kernel/fs/axfs` | 文件系统/VFS | 支撑测试镜像、运行目录、临时目录和 POSIX 文件语义。 |
| `kernel/memory/axalloc`, `kernel/memory/axmm` | 分配器与地址空间 | 支撑用户地址空间、共享页、COW、懒加载和 mmap/fork 语义。 |
| `kernel/namespace/axns` | 命名空间 | 运行时隔离、根目录和路径解析基础。 |
| `kernel/net/axnet` | 网络栈 | 与 POSIX socket 兼容层联动。 |
| `kernel/runtime/axruntime` | 启动与运行时 | 内核启动、设备初始化、应用入口和 evaluator 运行框架。 |
| `kernel/smp/axipi` | SMP/IPI | 保留多核演进边界。 |
| `kernel/sync/axsync` | 同步原语 | mutex、wait queue、通知等同步基础。 |
| `kernel/task/axtask` | 调度与任务 | 用户进程、线程、超时、等待与调度策略的底座。 |

这一步重构让评委可以按“真实内核子系统”阅读代码，而不是在扁平模块目录中寻找比赛补丁。它也是后续 Linux ABI、LTP 和双架构评测工作的基础。

### 1.2 我们把 ArceOS 的应用运行能力扩展成了较完整的 Linux/POSIX 用户态

当前用户态兼容层主要集中在 `examples/shell/src/uspace/`。该目录不是一个简单 runner，而是一组面向 Linux ABI 的用户态内核子系统：

- `syscall_dispatch.rs`：系统调用入口；当前源码中可识别 **230 个唯一 syscall dispatch 分支**。
- `program_loader.rs`：ELF 装载、解释器、auxv、用户栈、默认环境、glibc/musl 运行时路径。
- `process_lifecycle.rs`：`clone` / `execve` / `wait4` / `waitid` / `exit` / `exit_group`，包括 fork/vfork/线程式 clone 的边界。
- `memory_map.rs`、`user_memory.rs`：`brk`、`mmap`、`mprotect`、`munmap`、`mremap`、`msync`、`mincore`、`mlock*`、用户指针 copy-in/copy-out、懒触页与 EFAULT 语义。
- `fd_table.rs`、`fd_pipe.rs`：FD 表、`openat/openat2`、`dup/dup3`、`fcntl`、`lseek`、`readv/writev`、`splice/tee/vmsplice`、`eventfd`、`epoll`、`timerfd`、`inotify` 等。
- `metadata.rs`、`mount_abi.rs`：`statx`、`xattr`、`chmod/chown`、`utimensat`、`readlinkat`、`chroot`、`mount/umount2` 等文件元数据和挂载语义。
- `fd_socket.rs`：AF_INET/AF_UNIX socket 桥接、本地 UNIX socket、socket option 真实状态、buffer size、timeout、peer credential 等。
- `futex.rs`：`FUTEX_WAIT`、`FUTEX_WAKE`、`FUTEX_REQUEUE`、`FUTEX_CMP_REQUEUE`、bitset、timeout 与等待队列。
- `signal_abi.rs`：`rt_sigaction`、`rt_sigprocmask`、`rt_sigreturn`、`sigsuspend`、`sigtimedwait`、`kill/tkill/tgkill`、`sigaltstack` 等。
- `time_abi.rs`：`clock_gettime`、`clock_nanosleep`、`gettimeofday`、`itimer`、POSIX timer、`times` 与 CPU/runtime accounting。
- `sysv_msg.rs`、`sysv_sem.rs`、`sysv_shm.rs`、`posix_mq.rs`：System V IPC 与 POSIX message queue。
- `credentials.rs`、`resource_sched.rs`、`system_info.rs`：UID/GID/capability、rlimit、nice/ioprio/scheduler、`uname/sysinfo/prctl` 等。

这些代码把比赛常见“只跑固定 case”的做法替换成了可解释、可扩展、可复审的 Linux/POSIX 语义层。

### 1.3 我们同时支持 RISC-V 与 LoongArch 官方路径，并显式处理本地/远程差异

远程官方构建入口是仓库根目录的：

```bash
make all
```

它会生成：

```text
kernel-rv
kernel-la
```

本地验证入口是：

```bash
./run-eval.sh rv
./run-eval.sh la
```

其中 LoongArch 不是简单复用本地配置。仓库提供了 `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`，`Makefile` 中的 `REMOTE_LA_PLAT_CONFIG` 专门用于官方远程 LoongArch 地址映射；本地 `run-la` 与远程 `make all` 的地址空间差异被写入规则并保持显式。这一点体现了我们不是“本地能跑就提交”，而是把官方评测机差异纳入工程模型。

### 1.4 我们建立了可审计的评测闭环，而不是只看最终输出

`examples/shell/src/cmd.rs` 不只是 shell 命令集合，也是官方 evaluator/LTP 集成点。当前源码中：

- `LTP_STABLE_CASES` 当前为 **1000 个 stable LTP case**。
- 支持 `stable`、`stable-full`、`blacklist`、`all-minus-blacklist`、`sweep:blacklist` 等选择模式。
- blacklist 只用于隔离会卡死、炸内存、破坏评测器或明显不适合当前模型的用例；**被 blacklist 的 case 不被计为通过，也不是 stable promotion 证据**。
- `run_ltp_suite()` 对 `/musl` 与 `/glibc` LTP 套件分别运行所选 case，并打印 case list、timeout、RUN/PASS/FAIL/TIMEOUT 等结构化输出。
- `maybe_run_official_tests()` 对 official group 进行分组运行、timeout 管理与显式 skip 记录。

配套脚本也已经工程化：

- `scripts/ltp_summary.py`：解析 LTP wrapper 输出，同时保留 `TFAIL/TBROK/TCONF`、timeout、`ENOSYS`、panic/trap 等内部信号，避免把 wrapper status 误读成真实通过。
- `scripts/eval_failure_report.py`：从官方 evaluator log 生成可读失败报告，覆盖 LTP、libctest、BusyBox、official group、panic/trap、ENOSYS 等。
- `scripts/check_g002_*.py` 到 `scripts/check_g013_*.py` 与对应 `test_g*.py`：把 self-check 红线、runner 诚实性、socket/time/mempolicy、musl patch、用户指针边界、syscall 热点等变成可重复的静态/单元检查。

这意味着：评委看到一个 PASS 时，可以追溯它是否只是 wrapper PASS、是否存在内部 `TFAIL/TBROK`、是否超时、是否被 skip、是否触发 kernel trap。我们选择让失败可见，而不是让日志变漂亮。

### 1.5 我们把“竞赛合规”做成了工程红线

仓库根目录 `self-check.md` 明确禁止：

- 按测试程序名称、路径、二进制特征做特殊判断；
- 对特定 syscall 参数组合或目录结构硬编码结果；
- 伪造 PASS、隐藏 `TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap`；
- 为跑分破坏 Linux syscall 语义、安全边界或必要错误处理。

这不是口号。仓库中有对应的检查脚本、单元测试、runner 输出解析和工作流规则，确保我们后续改动仍然围绕真实语义推进。即使存在当前尚未支持的 syscall 或 case，我们也倾向于返回明确 errno、记录失败、留下后续修复入口，而不是假装通过。

## 2. 量化快照

以下数字来自当前工作树的源码和本地只读统计，用于帮助评委快速感知工作规模。它们不是跑分声明，而是工程规模与验证面指标。

| 指标 | 当前值 | 证据入口 |
| --- | ---: | --- |
| 主要源码/脚本/文档文件数（排除 `vendor/`、`cargo-home/`、`build/`、`target/`、归档与评测输出，并包含本文档） | 472 个 | `git ls-files` + 本文末尾复现命令 |
| 上述文件总行数 | 约 98,179 行 | 同上 |
| `kernel/` tracked Rust/配置相关规模 | 99 文件 / 约 15,177 行 | `kernel/` |
| `examples/` tracked 规模 | 49 文件 / 约 41,851 行 | `examples/shell/` 为主 |
| `api/` tracked 规模 | 34 文件 / 约 6,289 行 | `api/arceos_posix_api/` 等 |
| `ulib/` tracked 规模 | 139 文件 / 约 14,331 行 | `ulib/axlibc`、`ulib/axstd` |
| `scripts/` tracked 规模 | 52 文件 / 约 9,620 行 | 构建、评测、回归检查、日志解析 |
| `docs/` tracked 规模 | 28 文件 / 约 2,172 行 | 工作流规则、阶段报告与本文档 |
| CodeGraph 索引规模 | 359 indexed files / 8,121 symbols / 25,670 edges | `.codegraph` 索引状态 |
| syscall dispatch 唯一分支 | 230 个 | `examples/shell/src/uspace/syscall_dispatch.rs` |
| `examples/shell/src/uspace/` 用户态 ABI 模块 | 29 个 `.rs` 文件 | `examples/shell/src/uspace/` |
| 当前 stable LTP case | 1000 个 | `examples/shell/src/cmd.rs::LTP_STABLE_CASES` |
| self-check/LTP 解析与 guard 脚本 | 26 个核心脚本 | `scripts/check_g*.py`、`scripts/test_g*.py`、`scripts/ltp_summary.py`、`scripts/test_ltp_summary.py` |

这些数字背后的重点是：我们没有把工作集中在一个测试入口里，而是在内核、ABI、用户库、runner、脚本、文档和验证层都做了系统性投入。

## 3. 与 ArceOS 基线相比，我们做了什么

### 3.1 从 `modules/` 到 `kernel/`：把教学/示例式模块改造成内核工程目录

上游 ArceOS 的模块组织方式更适合模块化 unikernel 教学和示例扩展。我们参考 Moss Kernel 的“按内核职责拆分”的工程直觉，将 ArceOS 模块映射到更直观的内核子系统目录：

```text
modules/axhal      -> kernel/arch/axhal
modules/axconfig   -> kernel/config/axconfig
modules/axlog      -> kernel/diagnostics/axlog
modules/axdriver   -> kernel/drivers/axdriver
modules/axfs       -> kernel/fs/axfs
modules/axalloc    -> kernel/memory/axalloc
modules/axmm       -> kernel/memory/axmm
modules/axns       -> kernel/namespace/axns
modules/axnet      -> kernel/net/axnet
modules/axruntime  -> kernel/runtime/axruntime
modules/axipi      -> kernel/smp/axipi
modules/axsync     -> kernel/sync/axsync
modules/axtask     -> kernel/task/axtask
```

这种重构看似“只是移动目录”，实际影响很大：Cargo workspace、crate path、Makefile、脚本、文档、远程构建、IDE/静态索引、后续 agent 协作都要同步修正。它让项目拥有了更容易被评委阅读和长期维护的内核边界。

### 3.2 从单应用/示例运行到多 libc 用户态评测环境

ArceOS 的优势是模块化、轻量、可裁剪，但 OSKernel 2026 要求的是能承受官方测试镜像的 Linux 用户态环境。我们围绕 `examples/shell/` 做了大规模扩展：

- 支持 `/musl` 与 `/glibc` 两套用户态路径；
- 支持 ELF interpreter、auxv、用户栈、envp/argv、shebang 与脚本运行；
- 支持评测中大量 shell wrapper、BusyBox applet、libctest、LTP case 的执行模式；
- 支持运行时兼容库和路径解析，使用户程序能够在测试镜像布局下找到 loader、动态库与辅助资源；
- 支持 per-case timeout、进程清理、scratch 目录清理和内存统计，避免一次失败污染后续 case。

这部分工作把内核从“能启动一个 app”扩展为“能批量加载、隔离、清理和观察数千个用户态程序”。

### 3.3 从基础 syscall 到系统调用族的成片建模

比赛里一个 case 通过并不代表 syscall 语义完整。我们更重视“成片建模”：

- FD/IO：`read/write`、`pread/pwrite`、`readv/writev`、`fcntl`、`dup`、`lseek`、`ioctl`、`sendfile`、`copy_file_range`、`splice/tee/vmsplice`。
- 文件元数据：`stat/fstat/newfstatat/statx`、`chmod/chown`、`utimensat`、xattr、hardlink/symlink、inode flag、sparse/data range。
- 进程：`clone`、`execve`、`wait4/waitid`、process group/session、pid/tid、exit group、vfork-like 路径。
- 内存：`brk`、`mmap`、`mprotect`、`mremap`、`msync`、`mincore`、`mlock`、shared mmap、COW 与懒页故障。
- 同步：futex wait/wake/requeue、robust list、timer wait queue、interruptible blocking。
- 网络：AF_INET socket 桥接、AF_UNIX local socket、socketpair、bind/listen/accept/connect/send/recv、socket option 真实状态。
- IPC：System V msg/sem/shm、POSIX MQ。
- 信号：handler、mask、pending、sigreturn、altstack、signal-interrupted wait 与 syscall restart 边界。
- 时间与资源：clock、nanosleep、itimer/POSIX timer、rusage/times、rlimit、nice/ioprio/scheduler。

我们优先把 syscall 放回 Linux 语义上下文中，而不是只针对某个测试 case 写局部返回值。

## 4. 与 Moss Kernel 的关系：参考结构，不是照搬实现

用户最初明确希望参考 Moss Kernel 的仓库结构。我们采用的是“工程结构上的启发”：Moss 给我们的最大提示是，一个面向 Linux 用户态的 Rust 内核应该让架构、运行时、驱动、文件系统、内存、任务、同步、用户态 ABI 和测试入口边界清楚，而不是把所有比赛逻辑塞进一个示例程序。

但本项目与 Moss 的实际实现路径不同：

| 维度 | Moss 给出的启发 | 我们的落地 |
| --- | --- | --- |
| 工程分层 | 内核职责清晰、面向 Linux 用户态 | 保留 ArceOS crate 生态，重排为 `kernel/arch`、`kernel/memory`、`kernel/task` 等子系统。 |
| 用户态目标 | Linux ABI 兼容是核心目标 | 以 OSKernel 官方测试镜像为目标，覆盖 musl/glibc、LTP、BusyBox、libctest 与 benchmark。 |
| 架构适配 | 架构/HAL 边界要清楚 | RISC-V 与 LoongArch 同时作为远程主路径，显式维护远程 LA 配置。 |
| 测试闭环 | 内核功能必须可验证 | 建立 `run-eval.sh`、`make all`、LTP parser、failure report、self-check guard 脚本。 |
| 语义态度 | 不能只停留在样例运行 | 明确禁止 fake pass 和硬编码；黑名单只作为实验隔离，不作为通过证据。 |

换句话说：Moss 影响了我们“如何组织一个更像 Linux 兼容内核的 ArceOS 派生项目”，但我们的代码主体、评测路径、双架构适配、LTP runner、POSIX syscall 语义和合规工具链都是围绕 OSKernel 2026 自主实现和持续迭代出来的。

参考入口：<https://github.com/hexagonal-sun/moss-kernel>

## 5. 评测与性能工作：我们优化的是真实路径，不是隐藏失败

OSKernel 评测不是单一 LTP 分数。官方路径包含 libc、BusyBox、Lua、network/iperf、lmbench、iozone、UnixBench、cyclictest 等不同负载。我们对 runner 与内核热路径做了多轮优化，但遵守两个原则：

1. **能真实修语义，就不靠 skip。** 例如睡眠、timer、futex、wait queue、epoll immediate path、exec image cache、metadata cache、read-only ext4 I/O 等优化都围绕真实运行路径展开。
2. **skip 必须显式、可清除、不可计为通过。** `REMOTE_SKIP_OFFICIAL_TEST_GROUPS ?= libctest-glibc` 是因为官方在线 scorer 不给 glibc libctest 计分，并且 Makefile 注释中保留了清除方式；LTP blacklist 只用于 full sweep 探索隔离严重 blocker。

这种做法的价值是：当分数波动时，我们可以区分是核心 LTP 语义退化、benchmark 性能瓶颈、官方 parser 格式、远程 LA 地址映射，还是具体 runner budget 配置问题。它比“看到总分低就盲修内核”更可靠。

## 6. 质量与合规工具链

评委如果担心“是否为了过测写了 hack”，建议重点看以下文件：

| 文件 | 作用 |
| --- | --- |
| `self-check.md` | 明确比赛合规红线。 |
| `AGENTS.md` | 将红线、验证、提交、LTP promotion、远程/本地区分写入协作规则。 |
| `scripts/check_g002_fake_success.py` | 防止 fake success 与隐藏失败路径。 |
| `scripts/check_g005_runner_parser.py` | 检查 runner/parser 是否绕过真实失败。 |
| `scripts/check_g007_socket_time_mempolicy.py` | 检查 socket/time/mempolicy 等易伪实现区域。 |
| `scripts/check_g012_syscall_review_hotspots.py` | 聚焦 syscall 高风险热点。 |
| `scripts/check_g013_user_copy_boundary.py` | 用户指针 copy-in/copy-out 边界检查。 |
| `scripts/ltp_summary.py` / `scripts/test_ltp_summary.py` | 保证 LTP 内部失败信号不被 wrapper 输出掩盖。 |
| `scripts/eval_failure_report.py` | 把官方评测日志转成失败报告，保留 panic/trap/timeout/ENOSYS。 |

这些工具不是“赛后文档”，而是用于持续开发的工程护栏。

## 7. 建议评委从哪里读代码

如果时间有限，建议按下面顺序阅读：

1. `README.md`：项目入口、构建和目录说明。
2. `Cargo.toml`：可以看到 `modules/` 到 `kernel/` 的 workspace 重构。
3. `Makefile`：远程 `make all`、RV/LA kernel、LTP selector、远程 LA config、offline vendor helper、官方 timeout budget。
4. `examples/shell/src/uspace/syscall_dispatch.rs`：Linux syscall 总入口。
5. `examples/shell/src/uspace/process_lifecycle.rs` + `program_loader.rs`：进程、exec、fork/clone、ELF loader。
6. `examples/shell/src/uspace/memory_map.rs` + `kernel/memory/axmm/src/aspace.rs`：用户内存、mmap、COW/shared mapping。
7. `examples/shell/src/uspace/fd_table.rs` + `metadata.rs` + `fd_socket.rs`：FD、文件元数据、socket 语义。
8. `examples/shell/src/uspace/futex.rs` + `signal_abi.rs` + `time_abi.rs`：同步、信号、时间。
9. `examples/shell/src/cmd.rs`：official runner 与 LTP runner。
10. `scripts/ltp_summary.py`、`scripts/eval_failure_report.py`、`scripts/check_g*.py`：验证与合规闭环。

## 8. 可复现的只读检查命令

这些命令不修改工作树，可用于复核本文的主要数字：

```bash
# 当前分支与提交
git branch --show-current
git log --oneline --max-count=10

# 统计 stable LTP case 数量
python3 - <<'PY'
from pathlib import Path
import re
s = Path('examples/shell/src/cmd.rs').read_text()
m = re.search(r'const\s+LTP_STABLE_CASES\s*:\s*&\[&str\]\s*=\s*&\[(.*?)\];', s, re.S)
items = re.findall(r'"([^"]+)"', m.group(1))
print(len(items))
PY

# 统计 syscall dispatch 唯一分支
python3 - <<'PY'
from pathlib import Path
import re
s = Path('examples/shell/src/uspace/syscall_dispatch.rs').read_text()
arms = re.findall(r'general::__NR_([A-Za-z0-9_]+)\s*=>', s)
print(len(dict.fromkeys(arms)))
PY

# 统计主要 tracked 文件与行数，排除 vendor / cargo-home / build / target / 归档 / 评测输出
python3 - <<'PY'
from pathlib import Path
import subprocess
skip_prefixes = ('vendor/', 'cargo-home/', 'archive/', 'eval-reports/', '.local-archive/', 'build/', 'target/')
exts = {'.rs', '.py', '.toml', '.mk', '.S', '.c', '.h', '.sh', '.md', '.yml', '.yaml'}
tracked = subprocess.check_output(['git', 'ls-files'], text=True).splitlines()
counts = {}
for f in tracked:
    if f.startswith(skip_prefixes):
        continue
    p = Path(f)
    if p.suffix not in exts and p.name not in {'Makefile', 'Cargo.lock', 'Cargo.toml', 'Dockerfile'}:
        continue
    lines = sum(1 for _ in p.open('r', encoding='utf-8', errors='ignore'))
    top = p.parts[0]
    counts.setdefault(top, [0, 0])
    counts[top][0] += 1
    counts[top][1] += lines
for k, (files, lines) in sorted(counts.items()):
    print(f'{k:18s} files={files:4d} lines={lines:8d}')
print('total', sum(v[0] for v in counts.values()), sum(v[1] for v in counts.values()))
PY

# 运行轻量 parser / guard 自测示例
python3 scripts/test_ltp_summary.py
python3 scripts/test_g005_runner_parser.py
python3 scripts/check_g012_syscall_review_hotspots.py
python3 scripts/test_g013_user_copy_boundary.py

git diff --check
```

## 9. 我们希望评委记住什么

我们不是从零写了一个玩具 kernel，也不是把 ArceOS 当成黑盒只在 shell 里“适配测例”。我们的工作是：

1. **保留 ArceOS 的模块化优势**，但重构成更像真实内核工程的目录和职责边界；
2. **把 Linux/POSIX 用户态语义作为主线**，系统性扩展 syscall、进程、内存、文件、网络、同步、IPC、信号、时间和资源模型；
3. **把官方评测当成工程环境**，而不是一次性脚本：双架构构建、本地/远程差异、timeout budget、offline vendor、parser、failure report、guard scripts 都纳入仓库；
4. **把诚实性作为核心竞争力**：不 fake PASS、不硬编码测例、不隐藏失败、不把 blacklist 当通过；
5. **用可复现证据支撑工作量**：源码结构、dispatch 表、LTP stable 列表、日志解析、合规检查和提交历史都可以被审查。

如果要用一句更有冲击力的话总结：**我们做的不是“让 ArceOS 多过几个测试”，而是把 ArceOS 改造成了一套可接受 Linux 用户态压力测试、可解释失败、可持续迭代、可被评委审查的 OSKernel 竞赛内核工程。**
