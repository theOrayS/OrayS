# PR1 Linux 边界抽取分析摘要

## 范围与不变量

本文是 PR1 的实施依据。分析覆盖仓库根清单、shell 的 `uspace` 模块、ABI/挂载/进程/时间/用户内存/dispatcher、CI 配置、Cargo feature 传播和现有静态守卫。

PR1 只建立架构边界，不改变 syscall 返回值、errno、阻塞、信号、进程、FD、VM、IPC 或调度语义；`UserProcess`、现有 handler 和 dispatcher 的控制流继续由 `arceos-shell` 所有。不得借抽取修复既有语义问题，也不得新增外部依赖或 unsafe。

## 当前依赖图

当前 Linux 兼容实现没有独立边界，依赖从 `arceos-shell::uspace` 直接扇出：

```text
arceos-shell (feature = uspace)
├── linux-raw-sys                  Linux 数字、结构和架构 cfg
├── arceos_posix_api               POSIX API 与 LinuxError
├── axhal / axmm / axtask          trap、地址空间、任务
├── axfs / axdriver / axns         文件系统、设备、命名空间
├── axalloc / axsync / kspin       内存和同步
└── user/shell/src/uspace
    ├── linux_abi.rs               A + C 混合
    ├── user_memory.rs             B 的候选接口 + C 的实际实现
    ├── syscall_dispatch.rs        数字匹配 + D + shell 控制策略
    └── 其余模块                   C/D，普遍直接依赖 UserProcess
```

`uspace` feature 同时开启 `axtask/uspace`、`axstd/fp-simd` 和所有上述可选依赖；`auto-run-tests` 另行开启 `axtask/sched-rr`。因此裸 `APP_FEATURES=uspace` 不是当前完整用户态构建配置，正式双架构构建必须同时使用 `FEATURES=alloc,paging,irq,multitask,fs,net,rtc` 与 `APP_FEATURES=auto-run-tests,uspace`。

目标依赖图为：

```text
arceos-shell
└── orays-linux (optional, activated by shell/uspace)
    └── orays-linux-abi
        └── linux-raw-sys (no_std, architecture-selected numbers)
```

两个新 crate 均不得反向依赖 shell；ABI crate 不依赖任何 OrayS kernel crate。

## 可能的 dependency cycle

当前没有新 crate，因此没有既存 cycle。以下设计会立即形成或埋下 cycle，均禁止：

- `orays-linux` 为取得 `UserProcess`、`AddrSpace` 或 handler 而依赖 `arceos-shell`，同时 shell 又依赖 `orays-linux`。
- `orays-linux-abi` 依赖 `arceos_posix_api`/`axerrno`，而这些 API 未来再使用 ABI crate；即使当前 Cargo 尚未闭环，也破坏叶子 crate 约束。
- 把 `axmm`、`axtask`、`axhal`、`axfs` 类型写入 `orays-linux` 公共 trait；这会把实现依赖向下传播，并使 shell/backend 无法保持单向。
- 让 `arceos_posix_api` 与 `orays-linux` 相互依赖。PR1 的错误类型应由 backend 关联类型承载，而不是让通用 crate 引入 `LinuxError`。

每个里程碑使用 `cargo metadata --locked --offline --no-deps` 和 manifest 检查确认方向；一旦发现 cycle，按 AGENTS.md 停止。

## A/B/C/D 分类

| 文件/符号组 | 分类 | PR1 处置 | 理由 |
|---|---|---|---|
| `linux_abi.rs`: signal 号码/flag、rlimit 资源号、FD/IOV 限制、文件 mode、statfs magic、SysV IPC flag/command、open/close-range、socket/IP/TCP option、personality、`RTC_RD_TIME`、`AUX_PLATFORM` | A | M1 迁入 `orays-linux-abi::constants`，shell 原路径显式 re-export | 纯数值或架构字符串，无 kernel 类型依赖 |
| `linux_abi.rs`: `USER_ASPACE_*`、stack/mmap/brk/load base、文件和共享内存容量、测试目录、默认 passwd/group、socket 接收 quantum、synthetic device size | C | 留在 shell | 是 OrayS 布局、资源或合成后端策略，不是 Linux ABI；G006/G009 还要求 synthetic 常量在原文件可见 |
| `linux_abi.rs`: RISC-V signal frame reserved/FP size、两架构 trampoline 指令 | C | 留在 shell | 与 shell 的 TrapFrame/signal frame 实现共同演化，不能只移常量 |
| `linux_abi.rs`: `posix_errno_from_ret`、`posix_ret_*`、`neg_errno*`、`str_err` | C | 留在 shell | 依赖 `axerrno::LinuxError` 或分配；属于 backend glue |
| `linux_abi.rs`: `fd_cloexec_flag` | C | 暂留 shell | 与 shell FD 表达和直接调用耦合，迁移收益低 |
| `time_abi.rs`: `USER_HZ`, `Tms`, `RtcTime` | A | M1 迁入 ABI crate 并 re-export | `repr(C)`/纯标量；增加 size/align/offset 守卫 |
| `time_abi.rs`: `UserTimex` | A 候选，当前留置 | PR1 不迁 | 字段多、守卫和现有语义测试密集；不扩大 M1 |
| `time_abi.rs`: clocks、discipline、timer state、转换函数、所有 `sys_*` | C/D | 留在 shell | 依赖 `UserProcess`、axtask、计时器、signal 和 LinuxError |
| `mount_abi.rs`: flag 常量、路径解析、`sys_mount`, `sys_umount2` | C/D | 全部留在 shell | flag 虽为数值但来自 `linux_raw_sys`；整个文件直接依赖 UserProcess/axfs/namespace，拆分会越过子系统边界 |
| `process_abi.rs`: synthetic PID、group/session/personality state、所有 syscall | C/D | 全部留在 shell | 直接依赖 UserProcess、task registry 和 shell 策略 |
| `user_memory.rs`: 地址/range/pointer/slice/access marker（当前尚不存在） | B | M2 在 `orays-linux` 新建 | 可用整数和 marker 表达，独立于 UserProcess |
| `user_memory.rs`: backend interface | B | M2 在 `orays-linux` 新建 trait | 只使用 typed range/slice 与关联错误类型 |
| `user_memory.rs`: validate/fault-in/copy/cstr/value helpers 的实现 | C | M2/M3 留在 shell，以 adapter 桥接 | 依赖 `UserProcess::aspace`、brk、`AddrSpace` 和 LinuxError |
| `user_memory.rs`: `sys_getrandom` | D | 留在 shell | 具体 syscall handler |
| `syscall_dispatch.rs`: syscall number identity/metadata 类型 | A/B | number 来自 ABI crate；metadata 类型 M4 放 `orays-linux` | identity 独立于 handler；元数据不拥有 dispatcher |
| `syscall_dispatch.rs`: `LOONGARCH_LEGACY_*` | A | M1 在 ABI crate 加架构数值守卫；原路由 M4 前不动 | LoongArch64 legacy ABI 特例 |
| `syscall_dispatch.rs`: `user_syscall` match、LA clone 参数交换、poll cfg、watchdog/measurement/restart accounting | D/C | 全部留在 shell | 这些是真实路由与运行策略，本 PR 不生成 dispatcher |
| 其余 handler、FD、signal、VM、IPC 模块 | C/D | 不迁移 | 超出 PR1，且多数直接依赖 UserProcess |

## 第一批可安全迁移的 ABI

M1 的 allowlist 是纯数字/字符串常量以及 `Tms`、`RtcTime`。ABI crate 通过显式模块导出，不把 `linux_abi.rs` 整文件移动。syscall number 通过 `linux-raw-sys` 的架构选择模块 re-export，并用两目标编译和关键值 const assertion 固化。

以下内容明确不能随第一批迁移：地址空间布局、synthetic 内容/容量、signal frame/trampoline、errno helper、`UserTimex`、mount/process/time handler、任何 UserProcess 字段或方法。

## 调用面与耦合结论

- `uspace/mod.rs` 是唯一的 `use linux_abi::*`；扫描未发现 uspace 内 `use super::*`。M1 保留 `linux_abi` facade，避免一次修改所有调用者。
- `UserProcess` 被 27 个 uspace 模块直接依赖；移动它会同时牵动 process、FD、signal、VM、timer、IPC，触发“多于一个 major subsystem”停止条件。
- `read_user_value`、`write_user_value`、`read_user_bytes`、`write_user_bytes`、`read_cstr` 被大量 handler 使用。M3 只让现有 facade 的实现经过 typed adapter，不批量改 handler 签名。
- dispatcher 当前匹配 231 个 `general::__NR_*` 名称；LoongArch64 另有 163/164 legacy rlimit，并在 clone 路由交换第三、第四参数。上述控制流不可由 metadata 替换。
- `runtime_compat` 的 13 个 build.rs 生成 `SYS_*` 常量是独立路径，不纳入本 PR。

## unsafe 边界与 UserPod 风险

基线扫描约有 501 个 `unsafe` block（第一方约 379、shell uspace 68、`user_memory.rs` 5）。PR1 不以总量清理为目标，也不移动现有 unsafe 出 shell。

`read_user_value<T: Copy>` 会从任意用户字节构造任意 `T`；`Copy` 不能保证所有 bit pattern 有效，也不能保证写出 padding 不泄露内核数据。直接引入 blanket `UserPod for T: Copy` 会把现有未证明假设伪装成安全契约，风险为 high。PR1 的处置是：

- typed address/range/slice 本身不解引用、不提供 `Deref`、不包含 unsafe；
- byte copy trait 只搬运 `[u8]`；
- 旧 value-copy primitive 保持原行为和调用面，并在低层模块记录安全债务；
- 不在 PR1 引入 `UserPod`。后续若引入，必须是逐类型 sealed unsafe impl、验证 bit validity/padding/layout，并单独审计所有 impl。

## 架构差异

| 项目 | riscv64 | loongarch64 | PR1 约束 |
|---|---|---|---|
| signal trampoline | `0x08b00893, ecall, ebreak` 序列 | `0x02822c0b, syscall, 0` 序列 | 留在 shell signal backend |
| signal frame | RISC-V reserved 120、FP state 528，并有布局断言 | 当前无对称的同组断言 | 不在 PR1 统一或移动 |
| alt stack | `MINSIGSTKSZ/SIGSTKSZ = 2048/8192` | `4096/16384` | 保持 linux-raw-sys cfg 结果 |
| trap PC/state | `sepc` | `era`，恢复时 `prmd = 0x7` | adapter/metadata 不引用 TrapFrame |
| clone ABI | 通用参数顺序 | dispatcher 交换 arg3/arg4 | 路由保持原样，guard 检查 |
| legacy rlimit | 通用号 | 额外 163/164 | ABI const guard + dispatcher guard |
| host clippy | 基线通过 | host libclang 不认识 `loongarch64-unknown-none` | shell/full LA build仍必须通过；clippy 失败记为环境基线，不伪报通过 |

## 风险分级

### Blocker

- 新 crate 反向依赖 shell/kernel implementation，或 Cargo 出现 cycle。
- ABI number/layout/cfg 必须改变才能编译。
- adapter 要求移动 UserProcess，或必须同时修改 process/FD/signal/VM 多个子系统。
- 需要新 crates.io 依赖、升级依赖或修改 toolchain。

### High

- 泛型 value copy 的 bit-validity/padding 风险；PR1 不扩大该边界。
- 用户 range 溢出、零长度/null、跨页 partial copy 的 errno/部分写语义发生漂移。
- facade 漏导出导致某一架构或某一 feature 才失败。
- LoongArch64 legacy syscall/clone 参数差异被通用化。

### Medium

- Cargo feature 未把新 optional crate 绑定到 `uspace`，导致默认 build 掩盖错误。
- `cargo fmt --all -- --check` 的既有格式失败淹没新增格式问题。
- bindgen 构建按最后一个架构改写 checked-in `ctypes_gen.rs`；验证后必须恢复并确认 diff。
- static guard 依赖源文件 token；兼容 facade 必须保留必要声明/名字。

### Low

- 新 crate 的公开命名将来需要 semver 整理。
- 文档与实际里程碑文件清单漂移；每次提交同步 PROGRESS/VALIDATION。

## AGENTS.md 可行性评估

现有约束可执行，且与本计划一致。双架构 shell build、静态 guard、offline/locked 检查可以覆盖主要边界；不移动 UserProcess、不批量迁 handler、不改语义与 dependency 限制可把每步控制在一个子系统。

有两点需要诚实解释而不是放宽约束：仓库基线 `cargo fmt --all -- --check` 已因四个无关文件失败，因此每个里程碑还要对本次 touched Rust 文件执行精确 rustfmt/check；host 的 LoongArch64 clippy 因 libclang target 支持失败，但 LoongArch64 official-feature shell build 可通过，不能把前者写成 PASS。二者均记录在 VALIDATION，不在 PR1 修复。

## 仍需人工决策（不阻塞 PR1）

1. PR1 后是否将 `orays-linux-abi` 作为稳定的外部 API；本 PR 暂按 workspace-internal public API 设计。
2. 后续是否引入逐类型 `UserPod`；需要单独安全评审，PR1 明确不做。
3. 后续是否把完整 dispatcher table 变成 metadata 单一真源；PR1 只建最小 model/guard，不生成路由。
4. 是否修复/升级 host LLVM/libclang 以运行 LoongArch64 clippy；这属于环境/工具链工作，不能在本 PR 通过升级依赖或 toolchain 解决。
