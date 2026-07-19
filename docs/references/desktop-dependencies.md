# OrayS Desktop 依赖、资产与许可证记录

日期：2026-07-18

## 已选择的生产依赖

Checkpoint 1 决定桌面 UI 不新增第三方 crate 或 C/C++ 库。`user/desktop` 使用 OrayS
仓库内的 `axstd`、`axdriver`、`axalloc`、`axconfig`、`axhal` 及固定 Rust 工具链；这些是
仓库内部路径依赖，不复制外部源码。最终实现不依赖或修改 `axdisplay`：VirtIO GPU、input、
transport 与 framebuffer 均由桌面 `platform/` 层持有。

最终 VirtIO input/GPU 实现位于桌面本地 `user/desktop/src/platform/input.rs`、
`display.rs` 与 `virtio.rs`，直接复用下表三项固定依赖。它们在桌面加入前已经存在于 OrayS
的固定依赖输入，没有下载、升级或新增 vendor 内容；桌面独立 `Cargo.lock` 固定精确版本、
registry source、checksum 和完整依赖图。

| crate | 精确版本与 Cargo checksum | 来源 | 许可证与版权/NOTICE | 离线位置 |
|---|---|---|---|---|
| `axdriver_pci` | `0.2.0`；`7c7f9b680051dd1a872378a27c3dfb115559448e0a662d11f13a82caca536e75` | crates.io registry；manifest repository `https://github.com/arceos-org/axdriver_crates` | `GPL-3.0-or-later OR Apache-2.0 OR MulanPSL-2.0`，由 package `Cargo.toml` 声明；作者 Yuekai Jia；crate archive 不含独立 LICENSE/NOTICE | 固定输入 `vendor/cargo-vendor.tar.gz` 解包后的 `build/desktop/vendor/cargo/axdriver_pci/` |
| `kspin` | `0.2.0`；`6aeccb0b2a2babd189e98c42a8836dfc4996fabed4cdcdfdf296090c34fa75b4` | crates.io registry；manifest repository `https://github.com/arceos-org/kspin` | `GPL-3.0-or-later OR Apache-2.0 OR MulanPSL-2.0`，由 package `Cargo.toml` 声明；作者 Yuekai Jia；crate archive 不含独立 LICENSE/NOTICE | 固定输入 `vendor/cargo-vendor.tar.gz` 解包后的 `build/desktop/vendor/cargo/kspin/` |
| `virtio-drivers` | `0.13.0`；`cfdc1c628cdd8ce7c3b9e65a8ed550d0338e9ef9f911e729666f1cce097de2f7` | crates.io registry；manifest repository `https://github.com/rcore-os/virtio-drivers` | MIT；随包 `LICENSE`，copyright 2019-2020 rCore Developers；无单独 NOTICE | 固定输入 `vendor/cargo-vendor.tar.gz` 解包后的 `build/desktop/vendor/cargo/virtio-drivers/` |

固定离线 archive 的 SHA-256 为
`f52a589885afbc5369a7b9c83dc4bc2fe75da45e23da88eddb7a0b8286e3dd24`；它是只读输入，
`scripts/desktop/build.sh` 只把它解包到忽略的 `build/desktop/`。上述版本和 checksum 同时可在
`user/desktop/Cargo.lock` 审计；未提交解包副本。

`axdriver/desktop-device-hook` 是默认关闭、无新增依赖的窄生命周期桥：只有桌面独立构建会
启用它；它在正常 MMIO/PCI 总线枚举和 PCI BAR 配置完成后调用桌面提供的 C ABI 探测符号。
根默认 feature、根 `Cargo.lock` 和默认非桌面依赖图均不因此启用或引入 GPU/input 实现。

桌面独立 workspace 应声明与 OrayS 根 workspace 一致的许可证表达式：

```text
GPL-3.0-or-later OR Apache-2.0 OR MulanPSL-2.0
```

后续任何新增依赖必须在本文件记录固定版本或 commit、来源、许可证文件、离线存储位置、
使用范围和人工改写；未完成这些字段不得进入生产依赖。

## 已评估但未采用

### LVGL `v9.3.0`

- 目的：比较成熟 C UI toolkit 的字体、图片、动画、透明度和控件能力。
- 本地来源：无；仓库、离线包、系统头文件和缓存均未找到。
- 网络获取：公开 GitHub 标签查询两次均因 DNS 失败，未取得 tag commit 或源码。
- 工具链证据：RV64 freestanding C probe PASS；LA64 clang 14 不识别 target，FAIL。
- 许可证：未取得候选版本的本地 LICENSE，未把记忆或网页印象当作审计证据。
- 借鉴/复制：无代码、资产或设计实现被复制。
- 结论：因离线来源、LA64 C 工具链、FFI 和隔离成本拒绝。

### `embedded-graphics = 0.8.1`

- 目的：比较 Rust/no_std primitives 生态。
- 本地来源：仓库固定离线 Cargo source、本机 registry 和 git cache均无该 crate。
- 构建证据：RV64/LA64 的 `cargo check --offline` 均退出 101，package not found。
- 许可证：未取得候选版本的本地 crate/LICENSE，因此没有签收或提交许可证声明。
- 借鉴/复制：仅编写调用接口的临时 probe，位于忽略的 `build/desktop/spikes/`；生产代码
  未复制其实现。
- 结论：因无法由当前固定输入离线重建且仍需自行实现桌面系统层而拒绝。

## Spike 证据

临时证据位于忽略的 `build/desktop/spikes/`：

| 文件 | 大小 | SHA-256 |
|---|---:|---|
| `pure-rust/libdesktop_pure_rust_spike-rv64.rlib` | 11,620 | `f698e3a1a05bd271f76007790f6b1a811cf80bc77e351f699f9b52b18bc26c43` |
| `pure-rust/libdesktop_pure_rust_spike-la64.rlib` | 11,436 | `07410ea5f4f6002ceddf05e310d57c66ead9cde7feacbb3855477640924dcbf3` |
| `lvgl-toolchain/probe-rv64.o` | 944 | `0d224c9b216c9868c299e77992c46b8418614233bca787bafb53e3cb95d34745` |

这些临时文件不是生产资产，也不提交 Git。LA64 C probe 和两套 embedded-graphics probe
均失败，因此不存在对应成功产物或哈希。

## 资产登记

Checkpoint 5 的图标、壁纸、阴影、圆角和 ASCII glyph 均由本 PR 的纯 Rust renderer 在
运行时程序化生成，没有复制或嵌入外部位图、字体或图标文件。其许可证随
`user/desktop` 源码的仓库许可证表达式，不存在需要单独分发的第三方资产许可证。

后续如为图片查看器测试加入样例图片，必须逐项填写：

| 资产 | 来源/版本 | 许可证 | SHA-256 | 修改 | 用途 |
|---|---|---|---|---|---|
| CP5 程序化 Shell 资产 | OrayS 本 PR 源码 | GPL-3.0-or-later OR Apache-2.0 OR MulanPSL-2.0 | 不适用（运行时生成） | 原创程序化 primitive | 壁纸、图标、glyph、阴影 |
| CP6 PPM 测试像素 | 测试代码内 2x1 原创字节 | 同本 PR 源码 | 运行时临时文件，不提交 | 红/绿色测试像素 | parser、viewer 和 host application scene |
