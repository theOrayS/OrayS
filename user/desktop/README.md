# OrayS Desktop

OrayS Desktop 是默认关闭、独立构建的单进程图形桌面。它不属于根 Cargo workspace，不改变
根 `make`/`make all`，也不修改 Linux/POSIX ABI 或官方评测路径。桌面只通过 `platform/`
边界访问 OrayS framebuffer、VirtIO 输入、单调时间、文件系统和系统能力。

## 架构

- 纯 Rust/no_std 软件渲染，不新增第三方 UI 或资产依赖；
- 单个桌面进程内包含 Shell、窗口管理器、damage compositor、控件和应用；
- framebuffer 后备缓冲支持 stride、clipping、alpha、圆角、缓存阴影和局部 present；
- VirtIO GPU、keyboard/tablet、DMA HAL 与 framebuffer 位于桌面本地 `platform/`；默认关闭的
  `axdriver/desktop-device-hook` 只在正常 MMIO/PCI 总线发现时把候选设备交给桌面探测；
- 设备分辨率变化会安全重建 framebuffer、Surface、workspace 和窗口约束，并同步绝对输入
  extent；
- RV64 使用 MMIO，LoongArch64 使用 PCI，桌面通用层不复制架构实现；
- host-tools 只提供内存 framebuffer、真实临时文件系统和确定性场景测试。

设计与依赖记录：

- `docs/decisions/0001-desktop-isolation.md`；
- `docs/decisions/0002-desktop-ui-toolkit.md`；
- `docs/decisions/0003-desktop-headless-validation.md`；
- `docs/references/desktop-dependencies.md`。

## 功能

Shell 提供自适应壁纸、顶栏、Dock、启动器、通知、dark/light 主题、有限动画和电源菜单。
窗口支持创建/关闭、焦点与 z-order、移动、八方向缩放、最小化、最大化/恢复、Alt-Tab 和模态
阻塞。

内置应用：

- Terminal：受限的进程内 builtin 命令解释器，提供输入、历史、有界输出、滚动，以及作用于
  真实文件系统的 `pwd/ls/cd/cat/mkdir/write`。它不会创建进程，不是 POSIX shell，也不支持
  管道、重定向、环境扩展、作业控制或执行任意程序；
- Files：真实列目录、新建、重命名、删除和打开文件；
- Text Editor：打开、编辑、Ctrl-S 保存、1 MiB 上限和 dirty close 确认；
- Images：带边界检查的 P3/P6 PPM 解码、fit 和 25%-400% zoom；
- System Monitor：真实桌面 uptime、窗口数和输入计数；
- Settings：实时切换主题和 Aurora/Dusk/Slate 程序化壁纸。

## 构建与 host 验证

```bash
scripts/desktop/check-headless-host.sh
python3 scripts/desktop/check-assets.py
scripts/desktop/build.sh host-test
python3 -B -m unittest discover -s test/desktop -p 'test_*.py'
scripts/desktop/build.sh golden-check
scripts/desktop/build.sh rv
scripts/desktop/build.sh la
python3 scripts/desktop/measure-performance.py
python3 scripts/desktop/check-scope.py
```

也可使用：

```bash
make -f Makefile.desktop desktop
make -f Makefile.desktop desktop-performance
```

构建脚本从固定的 `vendor/cargo-vendor.tar.gz` 恢复隔离 Cargo source 到
`build/desktop/vendor/`，不会写根 `vendor/` 或 `cargo-home/`。RV64 与 LA64 分别生成独立
配置、target、ELF 和 raw binary。生成物仅位于 `build/desktop/` 或
`test/output/desktop/`。

`golden-check` 重新生成五个 1280x720 host PPM 并逐个验证 P6 几何、字节数和版本化
SHA-256。host golden 只证明渲染确定性，不是 guest PASS。性能记录使用 release host
software compositor，写入原始 microsecond samples，不设置会产生 flake 的伪阈值。

## 无显示 QEMU

```bash
scripts/desktop/run-headless-qemu.sh --arch rv --scenario boot
scripts/desktop/run-headless-qemu.sh --arch rv --scenario launcher
scripts/desktop/run-headless-qemu.sh --arch rv --scenario overlap
scripts/desktop/run-headless-qemu.sh --arch rv --scenario applications
scripts/desktop/run-headless-qemu.sh --arch rv --scenario resize
scripts/desktop/run-headless-qemu.sh --arch la --scenario boot
scripts/desktop/run-headless-qemu.sh --arch la --scenario resize
```

runner 清除 `DISPLAY`、`WAYLAND_DISPLAY` 和 session DBus；LA runner 禁用会耗尽其 PCI
BAR 窗口的 QEMU 隐式默认 NIC。它使用 localhost-only VNC、Unix QMP、VirtIO
GPU/keyboard/tablet 和串口 stdout。来宾必须先报告
两个输入设备均已就绪，runner 才会注入事件。每次运行生成新的目录，保存 serial、QMP
input/capture transcripts、输入序列、PPM、SHA-256 与 fail-closed summary，并记录源码
执行前后的 commit/dirty 状态、QEMU/工具链版本和精确生成命令；两次源码状态不一致时
证据会失败。runner 无论 PASS/FAIL 都通过 exit finalizer 生成白名单过滤的
`review-package/`，复制已有的原始证据、display geometry、capture precondition 和独立外层
哈希；不会收录运行磁盘、QMP socket、缓存或凭据。`resize` 另含 VNC resize 记录。把 package
移动到任意目录后可用
`python3 scripts/desktop/validate-review-package.py --package <review-package>` 独立复验；
`VALID_FAIL` 只表示失败证据包结构和语义可复核，不会把 runtime 失败改成 PASS。缺 QEMU、
截图、输入 transcript、来宾 marker、原始证据文件或正常退出中的任一项都会保持失败。

GitHub workflow 中的固定 QEMU runtime job 按 RV64/LA64 独立 matrix 执行，且持久化
self-hosted runner 只接受受控 `workflow_dispatch` 或受信任分支 `push`，不会执行任意
`pull_request` head。runner 必须通过 `DESKTOP_REQUIRED_QEMU_VERSION=9.2.4` 的版本检查；
每个架构的 filtered package 使用 `always()` 独立上传，一个架构失败不会阻止另一个启动。

`resize` 场景通过 localhost-only RFB `SetDesktopSize` 请求真实设备分辨率变化，再注入按新
geometry 映射的绝对指针并使用 QMP 截取最终帧；host MemoryDisplay resize 不能替代该证据。

## 快捷键与交互

- `Super+Space`：打开/关闭启动器；
- `Alt+Tab` / `Alt+Shift+Tab`：切换窗口；
- `Super+T`：切换 dark/light 主题；
- `Ctrl+S`：在编辑器中保存；
- 鼠标：窗口焦点、拖动、缩放、控件、列表、滚动和 Dock。

## 资产与许可证

当前图标、壁纸、阴影和 ASCII glyph 都由源码在运行时生成，没有外部位图或字体。
`assets/manifest.json` 是版本化的 fail-closed 清单；加入任何文件都必须登记 SHA-256，并在
`assets/licenses/` 放置确切许可证文本，否则资产检查失败。

## 已知限制

- 只有软件 2D 合成，没有 3D、全屏 blur、Wayland/X11 或多进程 GUI 协议；
- Terminal 仅是显式 builtin 的受限命令解释器，不提供 POSIX process/shell 语义；
- 字形集目前是 ASCII，图片查看器目前只支持 P3/P6 PPM；
- CPU、memory 和 wall-clock 平台指标尚无真实 API，界面明确显示 `UNSUPPORTED`；
- 当前底层没有 reboot API，Restart 明确报告 unsupported；Shutdown 使用真实退出路径；
- monitor 每秒只在真实计数变化时重绘；主循环空闲 sleep 8 ms，有限动画结束后不继续动画帧，
  但打开的 monitor 可按 1 秒周期触发真实指标检查；
- FAT directory metadata 当前会在串口保留 `Is a directory` 诊断，目录枚举仍能完成；该日志
  不会被 summary 隐藏；
- QEMU summary 的协议 PASS 不能替代截图视觉复核，也不等价于 canonical full/official PASS。
