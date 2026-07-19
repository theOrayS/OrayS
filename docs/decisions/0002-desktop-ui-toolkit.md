# ADR 0002：桌面 UI 采用无新增依赖的纯 Rust 软件渲染

状态：Accepted

日期：2026-07-18

## 背景

OrayS Desktop 必须在固定 Rust 工具链、离线依赖、RV64/LA64 双架构和默认关闭的边界内
构建。技术选型不能通过修改根 workspace、根工具链、大范围 vendor 或默认 Makefile
获得便利。

## 候选

### 固定版本 LVGL

评估固定标签 `v9.3.0`。当前仓库、离线 Cargo 包、宿主 include/lib 和本地缓存均不含
LVGL 源码。两次公开 GitHub 标签查询（普通沙箱及经 auto-review 的只读网络请求）均因
DNS 解析失败退出 128，因此没有取得可校验的 tag commit、源码或许可证文件，也没有把
LVGL 构建成功。

对其必要 C 工具链做了真实最小 probe：

- `clang --target=riscv64-unknown-none ... -c probe.c`：退出 0，944-byte object；
- `clang --target=loongarch64-unknown-none ... -c probe.c`：退出 1，clang 14 报
  `unknown target triple`。

即使以后补齐源码，当前 LA64 路径仍需新增 C 交叉工具链、构建系统、FFI、安全不变量及
源码/许可证 vendoring。该成本超过本 PR 的隔离预算，因此拒绝。

### 纯 Rust/no_std 软件渲染

在 `build/desktop/spikes/pure-rust/` 创建无依赖、无 `unsafe` 的 stride-aware Surface
probe，包含溢出检查、clipping 和 padding 保持：

- RV64 `rustc --crate-type rlib`：退出 0，11,620 bytes；
- LA64 `rustc --crate-type rlib`：退出 0，11,436 bytes；
- host 行为测试：2/2 PASS，覆盖 clipping/stride 及短 buffer/窄 stride 拒绝；
- 两架构编译各约 0.62 秒，使用仓库固定的
  `rustc 1.89.0-nightly (60dabef95 2025-05-19)`；
- 不新增依赖、C/C++ 工具链、根 workspace 成员或 bridge。

此路径需要自行维护 painter、widgets、字体/图片的受限格式、窗口状态机和缓存，但这些
正是本 Goal 要求审查和测试的核心语义，且能共享同一套 RV64/LA64 源码。

### `embedded-graphics = 0.8.1`

这是明显适合 no_std 的替代方向。真实离线 Cargo probe 在 RV64 与 LA64 均退出 101：
仓库固定离线目录没有该 package，本机 registry/git cache 也没有其源码。未取得本地源码
和许可证文件，因此不接受未经固定、无法离线重建的依赖，也没有在线下载来绕过证据。

该方案理论上可减少 primitives 开发量，但完整桌面仍需另行实现布局、文本编辑、窗口、
输入和应用；为少量 primitive 引入新的 vendor/许可证面并不划算，因此本 PR 不采用。

## 比较

| 维度 | LVGL `v9.3.0` | 纯 Rust/no_std | embedded-graphics `0.8.1` |
|---|---|---|---|
| RV64 | C toolchain probe PASS；LVGL 源未构建 | rlib PASS | offline resolve FAIL |
| LA64 | C toolchain probe FAIL | rlib PASS | offline resolve FAIL |
| no_std/freestanding | 预期可配置，但未以本地源码验证 | 已验证 | 候选设计匹配，但本地源码缺失 |
| 离线重建 | FAIL：源码不在固定输入中 | PASS：无新增依赖 | FAIL：crate 不在固定输入中 |
| C/C++ 工具链 | 必需，且 LA 当前缺失 | 不需要 | 不需要 |
| 字体/图片 | 框架能力强，但未验证配置 | 实现受限、可审计格式与缓存 | primitives 为主，仍需额外实现 |
| 动画/透明度 | 框架能力强 | 由 compositor/painter 实现 | 仍需上层实现 |
| bridge 预估 | C ABI + allocator/time/display/input | 现有 Rust platform API；最小 feature bridge | Rust platform API + 新 crate vendor |
| build time/size | 未构建，不填假数据 | RV 0.62s/11,620B；LA 0.62s/11,436B | 未解析，不填假数据 |
| 许可证 | 本地未取得 LICENSE，不能签收 | 无新增第三方依赖；沿用 OrayS 许可证 | 本地未取得 crate/LICENSE，不能签收 |
| 维护成本 | C/Rust/FFI 双栈 | 单一 Rust 栈，桌面代码量较大 | 中等，但仍缺桌面系统层 |

## 决策

选择无新增第三方依赖的纯 Rust/no_std 软件渲染和内置控件方案。

实现约束：

1. 同一套架构无关 renderer、widgets 和 desktop 状态机用于 RV64/LA64。
2. `platform/` 是访问 framebuffer、input、time、filesystem 和 system info 的唯一入口。
3. 软件 renderer、控件和桌面状态机不新增 `unsafe`。Checkpoint 8 的真实 VirtIO GPU、DMA
   和 PCI/MMIO transport 边界由 [ADR 0003](0003-desktop-headless-validation.md) 取代本 ADR
   最初的“无新增 unsafe”假设；仅允许该 ADR 记录并经测试覆盖的不变量。
4. 5x7 字形在编译期生成 `GLYPH_ATLAS`，六类应用图标在编译期生成
   `APP_ICON_MASKS`；运行时只复用 atlas 并着色/绘制。阴影复用固定 16 级
   `ShadowCache`，不得在每帧重复生成字体、图标或阴影数据。
5. 字体、图标、壁纸和样例图片只在许可证、来源、哈希和离线重建均闭合后加入。
6. 不为技术选型结果修改根 Cargo workspace、根 Makefile、工具链或 vendor。

## 后果

优点是隔离边界最小、双架构工具链一致、离线可重建、Rust 状态机可直接 host 测试。
代价是本 PR 必须实现并测试更多绘制、控件和缓存逻辑；静态 atlas 只缓存可复用的字形/图标
掩码，不是预渲染整屏，也不固定应用数据。功能范围以 Goal 的基础定义为准，不得用预渲染
整屏或固定应用数据替代真实行为。
