# PR1 决策记录

## D-001：crate 身份与最终依赖方向

- 路径：`api/orays_linux_abi`
- package：`orays-linux-abi`
- Rust crate：`orays_linux_abi`
- 属性：`#![no_std]`
- feature：无默认 feature；PR1 不增加 alloc feature
- 依赖：仅复用 workspace 已锁定的 `linux-raw-sys`，关闭 default features，开启 `general` 与 `no_std`

- 路径：`api/orays_linux`
- package：`orays-linux`
- Rust crate：`orays_linux`
- 属性：`#![no_std]`
- feature：`default = []`；预留 `alloc = []`，PR1 的 core 类型/trait 不要求 alloc
- 依赖：只依赖 `orays-linux-abi`

最终 shell 只直接依赖 `orays-linux`，由后者以 `pub use orays_linux_abi as abi` 提供 ABI。M1 允许 shell 暂时直接依赖 ABI crate，以便 ABI 抽取本身可独立构建/回滚；M2 在同一提交删除这条临时直连并建立最终链。

## D-002：M1 的迁移 allowlist

迁移 `linux_abi.rs` 中无实现依赖的 Linux 数值组，以及 `time_abi.rs` 的 `USER_HZ`、`Tms`、`RtcTime`。不迁移地址布局、synthetic policy、signal frame、errno helper、`UserTimex` 或任何 handler。

ABI crate 的模块固定为：

```text
orays_linux_abi
├── constants
├── syscall
└── time
```

`syscall` re-export `linux_raw_sys::general` 的 target-selected number 定义；关键公共号和 LoongArch legacy 号由 const assertion 固化。`time` 对 `Tms`/`RtcTime` 提供 C layout assertion。

## D-003：兼容 re-export

M1 保留 `user/shell/src/uspace/linux_abi.rs` 与 `time_abi.rs`。旧的 `uspace/mod.rs` wildcard 不扩散到新 crate；facade 使用显式 allowlist：

```rust
pub use orays_linux_abi::constants::{...};
pub(super) use orays_linux_abi::time::{RtcTime, Tms, USER_HZ};
```

`linux_abi` facade 位于私有 `uspace` 模块内，因此这里的 `pub use` 不会形成新的 crate 外路径；它只让既有 `use linux_abi::*` 继续获得显式 allowlist 中的名字。采用 `pub(super)` 会对同一组兼容导出产生同等的 unused-import warning，并不能更准确地恢复原有的 wildcard 可见性。`time_abi` 的三个名字原本就是父模块可见，故继续使用 `pub(super)`。没有为兼容 facade 添加 `allow`。

原 shell-only 常量/函数仍在原文件定义。这样所有既有 public/crate-visible 路径继续可用，同时 review 能看到哪些名字越过边界。M2 后导入路径改为 `orays_linux::abi::...`，调用者不变。

`SYNTHETIC_BLOCK_DEVICE_SIZE` 必须继续在 shell `linux_abi.rs` 以原形式声明，不从 ABI crate re-export；它既是 backend policy，也受现有 guard 约束。

## D-004：用户内存最小类型

`orays-linux::user` 提供：

- `UserAddr(usize)`：纯地址值，提供 `new/get/checked_add`，不解引用。
- `UserRange<A>`：起点和字节长度，构造时检查 `start + len` 溢出；`A` 是访问 marker。
- `UserPtr<T, A>`：typed address + `PhantomData`，不检查用户映射、不实现 `Deref`。
- `UserSlice<T, A>`：typed pointer + element count，构造时检查 `len * size_of::<T>()` 与地址端点。
- sealed marker `Read`、`Write`；marker 不承载运行时状态。PR1 当前没有同时要求双向权限的通用调用，因此不提前增加 `ReadWrite`。

零长度 range 允许任意地址（包括 0），以保持现有 zero-length syscall 行为；非零 null 是否 EFAULT 由 shell backend 的既有验证决定。类型层只拒绝算术溢出，不自行发明 errno、对齐或映射语义。

## D-005：backend trait 与 shell adapter

通用 crate 定义不依赖 alloc 的接口：

```rust
pub trait UserMemoryBackend {
    type Error;

    fn validate_read(&self, range: UserRange<Read>) -> Result<(), Self::Error>;
    fn validate_write(&self, range: UserRange<Write>) -> Result<(), Self::Error>;
    fn read_bytes(&self, src: UserRange<Read>, dst: &mut [u8])
        -> Result<(), Self::Error>;
    fn write_bytes(&self, dst: UserRange<Write>, src: &[u8])
        -> Result<(), Self::Error>;
}
```

长度不匹配由 trait contract 明确要求 backend 拒绝；M2 shell adapter 防御性返回 `LinuxError::EINVAL`。该分支在 M2 不可从 syscall 到达，M3 只构造与 slice 等长的 range，因此不改变既有 errno。关联错误类型避免 `orays-linux -> axerrno/arceos_posix_api` 依赖。

shell 定义私有 `ProcessUserMemory<'a> { process: &'a UserProcess }` 并实现 trait；adapter 委托给 `user_memory.rs` 中保留的 raw backend primitive。M3 再让旧 facade 构造 typed range 并调用 adapter。`UserProcess` 不实现通用 trait 本身，避免把广泛方法面暴露为公共接口。

M2 刻意不构造 adapter，因此 shell 会新增一个明确的 transitional dead-code warning。没有用 `allow` 隐藏它；M3 将通过真实 facade 使用消除该 warning。相比为了消除 warning 而提前改动 handler 可达路径，这一提交边界更容易审查和回滚。

## D-006：用户拷贝桥接顺序

M2 只增加类型、trait、adapter 和测试，不改 handler。M3 把现有 `validate_user_*`、`read_user_bytes_into`、`read_user_bytes`、`write_user_bytes` 的内部路径桥接到 adapter；所有既有 handler 继续调用旧 shell facade，因此返回值和调用签名不变。

`read_cstr`、跨页 fault-in 和 chunk loop 继续在 shell；只在其已有 byte-read 边界使用 facade。`read_user_value<T: Copy>`/`write_user_value<T: Copy>` 不泛化为通用 safe API，也不新增调用点。

## D-007：syscall metadata 最小模型

M4 在 `orays-linux::syscall` 定义：

- `SyscallNumber(u32)`
- `SyscallArgs([usize; 6])`
- `SyscallAvailability`（all/riscv64/loongarch64）
- `SyscallMeta { number, name, argument_count, availability, alias_of }`

模型允许同号 alias，但 guard 必须区分“声明 alias”与“意外重复”。shell 只为架构敏感的 clone、poll、legacy get/setrlimit 建立边界 metadata，并由静态 guard 与现有 dispatcher 源码互相核对。PR1 不把 match 生成自 metadata、不移动 handler identity、不改 routing/accounting/watchdog/restart 逻辑。

## D-008：unsafe 与 UserPod

PR1 不新增 unsafe block。新通用类型不解引用 raw pointer；byte backend trait 本身是 safe contract，shell 实现继续使用原有经过 `AddrSpace` 验证的 primitive。

不引入 `UserPod`。现有 `T: Copy` value-copy 的 bit validity/padding 问题登记为 high 风险但保持行为。若未来实施，必须逐类型、sealed、unsafe impl，并提供 layout/validity 证据；禁止 blanket impl。

## D-009：验证和基线判定

- 所有命令离线运行；不运行 `cargo update`。
- 新 path package 导致的 `Cargo.lock` 变化只允许新增 workspace package/dependency edge；逐行审计，禁止 registry 版本变化。
- 全局 fmt 的既有失败记为 BASELINE，同时对本里程碑 touched Rust 文件单独执行 rustfmt check。
- LoongArch64 clippy 的 host libclang triple 错误记为 ENVIRONMENT；仍要求 LoongArch64 official-feature shell build 通过。
- 本机实际安装的裸机目标名是 `loongarch64-unknown-none-softfloat`；直接 crate check/clippy 使用该 target。Makefile 的 `ARCH=loongarch64` 路径仍按仓库配置运行。
- 每步必须运行现有 G006/G009/G012/G013 guard、双架构 official-feature build、`git diff --check`；M5 再跑完整矩阵。
