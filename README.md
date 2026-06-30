## 目录
- [OrayS](#orays)
  - [文档入口](#文档入口)
  - [当前状态](#当前状态)
      - [核心功能实现](#核心功能实现)
      - [核心竞争优势](#核心竞争优势)
  - [项目结构](#项目结构)
  - [构建方式](#构建方式)
  - [运行方式](#运行方式)
  - [项目说明与模块完成情况](#项目说明与模块完成情况)
  - [开发日志](#开发日志)
  - [参考](#参考)
  - [参赛信息](#参赛信息)
# OrayS

[![Source License](https://img.shields.io/badge/Source%20License-GPL--3.0-～blue)](Cargo.toml)
[![Docs License](https://img.shields.io/badge/Docs%20License-CC--BY--SA%204.0-green)](https://creativecommons.org/licenses/by-sa/4.0/)
[![Targets](https://img.shields.io/badge/Targets-RISC--V64%20%7C%20LoongArch64-orange)](#构建方式)
[![OrayS](https://img.shields.io/badge/OrayS-black)](#orays)

基于 [ArceOS](https://github.com/arceos-org/arceos) 演进的内核，保留了ArceOS的组件化思想并加入模块化思想，同时补充了 Linux/POSIX 用户态边界、ELF 加载、进程生命周期、文件描述符、信号、futex、mmap、socket 和 LTP runner 等能力。

```text
.88888.                             .d88888b
d8'   `8b                            88.    "'
88     88 88d888b. .d8888b. dP    dP `Y88888b.
88     88 88'  `88 88'  `88 88    88       `8b
Y8.   .8P 88       88.  .88 88.  .88 d8'   .8P
 `8888P'  dP       `88888P8 `8888P88  Y88888P
                                 .88
                             d8888P
```

## 文档入口

| 内容 | 文档 |
| --- | --- |
| 项目文档 PDF | [文档.pdf](docs/orays-project-document(2).pdf) |
|项目初赛介绍ppt|[项目初赛介绍ppt](docs/orays.pptx) |
|演示视频|[演示视频](docs/演示视频.webm) |
## 当前状态

**截至 2026-06-29**：

- 完成 **531 次项目提交**
- syscall dispatcher 已注册 **231 个唯一 Linux syscall 编号**
- 建立 **12 个合规静态守卫及 12 个对应单元测试脚本**，用于检查 fake pass、
  用户指针、runner parser、FD/资源限制和 syscall 语义回归。

#### 核心功能实现

- **跨架构用户态执行环境**：在 ArceOS 组件化内核基础上打通 RISC-V64 与 LoongArch64 的 ELF 加载、trap 入口、用户上下文切换、页表根切换和远程评测启动路径。
- **Linux syscall 与 POSIX 兼容层**：`user/shell/src/uspace` 和 `api/arceos_posix_api` 共同维护 syscall dispatch、errno 转换、libc 边界和用户指针 copy-in/copy-out，目前 dispatcher 覆盖 231 个唯一 Linux syscall 编号。
- **进程、内存与信号语义**：实现 `clone`/`fork`/`vfork`/`execve`/`wait*` 生命周期，支持用户地址空间、lazy allocation、COW、`mmap`/`mprotect`/`mremap`/`munmap`、信号安装、屏蔽、等待与返回。
- **文件系统与统一 FD 模型**：在 VFS、EXT4/FAT/ramfs 基础上统一普通文件、目录、pipe/FIFO、socket、eventfd、epoll、timerfd、signalfd、memfd、pidfd 等对象的读写、poll、dup、fork 和关闭行为。
- **同步、IPC 与网络能力**：覆盖 futex wait/wake/bitset、robust list、System V SHM/msg/sem、POSIX mqueue、AF_UNIX、TCP/UDP loopback、常用 socket option 和 poll/epoll readiness。

#### 核心竞争优势

1. **系统兼容性与稳定性**：OrayS 围绕 Linux/POSIX 可见语义构建用户态兼容层，通过官方 runner、LTP runner 和静态合规检查持续验证功能路径、边界条件和异常处理。
2. **文件 I/O 与 FD 模型扎实**：内核在 VFS、EXT4/FAT/ramfs 基础上统一普通文件、目录、pipe/FIFO、socket、eventfd、epoll、timerfd、signalfd、memfd、pidfd 等对象，使 read/write、poll/epoll、dup、fork、fcntl、splice、sendfile 和 close 路径尽量复用同一套 FD 生命周期管理。文件映射、稀疏文件、msync、rename、xattr、statx 和权限语义的补强，为 iozone、coreutils 和 LTP 文件系统类负载提供基础。
3. **综合执行能力覆盖面广**：项目从最初的 RV/LA ELF 启动入口扩展到进程创建与退出、用户地址空间、lazy allocation、COW、mmap/mprotect/mremap/munmap、信号、futex、System V/POSIX IPC、TCP/UDP/AF_UNIX socket 和 procfs。关键路径按模块拆分后，syscall dispatch、用户内存 copy-in/copy-out、进程生命周期、FD、socket、signal、time 和 metadata 能独立定位问题并快速迭代。
4. **并发同步与调度路径持续优化**：任务模型继承 ArceOS 的调度、锁和上下文切换框架，并补充 futex wait/wake/bitset、robust list、信号等待、timerfd、POSIX timer、pipe/FIFO readiness 和阻塞 syscall watchdog。相关实现面向 cyclictest、hackbench、pthread、LTP futex/signal/time 类场景，重点降低永久阻塞、资源泄漏和退出后悬挂引用风险。


## 项目结构

```text
.
├── README.md
├── Cargo.toml
├── Makefile
├── rust-toolchain.toml
├── api
│   ├── arceos_api             # ArceOS 公共 API
│   ├── arceos_posix_api       # POSIX/Linux API 与 libc 边界
│   └── axfeat                 # 内核 feature 组合
├── kernel
│   ├── arch                   # HAL、trap 与架构上下文
│   ├── config                 # 编译期平台配置
│   ├── drivers                # VirtIO、块设备、网络和显示驱动
│   ├── fs                     # VFS 与文件系统
│   ├── memory                 # 分配器、地址空间与页表
│   ├── net                    # smoltcp 网络栈
│   ├── runtime                # 内核初始化与多核启动
│   ├── sync                   # 锁与同步原语
│   └── task                   # 任务与调度器
├── user
│   └── shell                  # 用户程序入口、Linux ABI、官方 runner 与 LTP runner
├── ulib
│   ├── axlibc                 # C 用户库
│   └── axstd                  # Rust 用户库
├── configs
│   ├── platforms              # 本地平台配置
│   └── remote-eval            # 远程评测配置
├── scripts                    # 构建、评测、日志汇总和合规检查
├── docs                       # 项目文档、开发日志与验证报告
├── tools                      # 仓库内置构建辅助工具
├── cargo-home                 # 离线 Cargo source replacement
└── vendor                     # 本地 crate patch 与离线依赖
```

## 构建方式

工具链由 `rust-toolchain.toml` 固定。远程提交构建会生成 RISC-V 与 LoongArch
两个内核产物：

```bash
make all
```

生成文件：

```text
kernel-rv
kernel-la
```

也可以按架构单独构建：

```bash
make kernel-rv
make kernel-la
```

直接构建用户态兼容入口：

```bash
make A=user/shell ARCH=riscv64 build
make A=user/shell ARCH=loongarch64 build
```

## 运行方式

准备对应的官方测试镜像或 SD 卡镜像后，运行本地评测：

```bash
./run-eval.sh rv
./run-eval.sh la
```

也可以使用 Make 目标直接启动 QEMU：

```bash
make run-rv ARCH=riscv64
make run-la ARCH=loongarch64
```
## 项目说明与模块完成情况

OrayS 在 ArceOS 组件化内核基础上增加 Linux 用户程序执行环境。内核侧继续使用
HAL、feature 和平台配置隔离架构差异；`user/shell` 与 `api/arceos_posix_api`
共同承担 ELF 加载、用户地址空间、syscall 分发和 POSIX/Linux 可见语义。

| 模块 | 当前实现与提升 | 状态 |
| --- | --- | --- |
| 架构与启动 | 支持 RISC-V64、LoongArch64 评测内核；维护本地/远程 LA 地址映射、trap、用户上下文、页表根切换和 RISC-V 用户寄存器恢复。 | 主要评测路径可用，持续做跨平台回归。 |
| 内存管理 | 用户地址空间、lazy allocation、缺页处理、mmap/mprotect/mremap/munmap、fork COW、vfork 共享驻留页、文件映射与 msync 写回。 | 核心语义已实现，复杂共享映射和资源压力继续完善。 |
| 进程与调度 | clone/fork/vfork、execve、wait4/waitid、进程组/会话、pidfd、资源限制、CPU affinity、FIFO/RR/CFS 调度接口。 | 主要生命周期已打通，调度细节和极端并发仍需回归。 |
| 文件系统与 FD | VFS、EXT4/FAT/ramfs、统一 FD table、pipe/FIFO、eventfd、epoll、timerfd、signalfd、xattr、statx、renameat2、splice 和 close_range。 | 官方常用路径可用，特殊文件系统和完整权限模型持续完善。 |
| 信号与同步 | sigaction、signal mask、信号发送/等待/返回、futex wait/wake/bitset、robust list、pthread 相关任务状态。 | 基本信号与线程同步可用，restart/cancel 等复杂边界继续修复。 |
| IPC | System V SHM/msg/sem、POSIX mqueue、pipe、socketpair 与 AF_UNIX；实现进程退出后的资源回收。 | 已覆盖主要对象，阻塞消息队列和 namespace 仍需扩展。 |
| 网络 | smoltcp TCP/UDP、AF_UNIX、loopback、阻塞收发、poll/epoll readiness、常用 socket option 和 netperf 兼容路径。 | IPv4 主路径可用，IPv6 和更多 socket option 尚未完整。 |


## 开发日志


| 日期 | 主要开发内容 |
| --- | --- |
| 2026-04-22 | 建立 RV/LA ELF 用户程序、syscall 入口、ext4、用户地址空间和评测构建基础。 |
| 2026-05-06 | 将内核模块按职责重组到 `kernel/`，补充 BusyBox `free`/`df` 和 ramfs rename。 |
| 2026-05-07—05-09 | 扩展 RV 网络、AF_UNIX、FD/socket、权限、procfs、System V SHM 和 pwrite。 |
| 2026-05-09—05-13 | 拆分各模块，集中用户内存 copy-in/copy-out，模块化 FD、socket、signal、futex、time 和 process。 |
| 2026-05-13—05-14 | 打通本地测试和在线评测机离线依赖恢复。 | `fad3359f`、`add36cfe` |
| 2026-05-15—05-19 | 完善 LoongArch 用户态、地址空间回收/lazy clone，并修复 RV netperf TCP_CRR。 |
| 2026-05-20—05-30 | 建立 LTP 核心兼容、远程/本地 LA 构建边界和可信评测证据机制。 |
| 2026-06-01—06-06 | 集中完善 VFS/FD、mmap、进程、信号、futex、IPC、pidfd、capability、timer 和跨架构回归。 |
| 2026-06-07—06-12 |修复长时间评测中的 OOM、引用泄漏、阻塞 syscall 和资源回收。 |
| 2026-06-19—06-23 | 完成 self-check、官方本地等价评测和剩余失败分类。 |


## 参考

- [ArceOS](https://github.com/arceos-org/arceos)：组件化内核基座。
- [smoltcp](https://github.com/smoltcp-rs/smoltcp)：无标准库 TCP/IP 协议栈，仓库中保留本地 vendor/patch。
- [rust-fatfs](https://github.com/rafalh/rust-fatfs)：FAT 文件系统实现，仓库中保留离线依赖

## 参赛信息
**参赛队名**：OrayS

**队伍编号**：T202610336999888

**参赛学校**：杭州电子科技大学

**参赛队员**：
- **王正卓**：majiaqiniubi@outlook.com
- **胡亿炜**：songyaxuannb@outlook.com
- **何漫漫**：hejunlinnb@outlook.com

**指导老师**：刘真老师、杨浩老师
