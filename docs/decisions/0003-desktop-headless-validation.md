# ADR 0003：桌面 framebuffer 与无显示验证边界

状态：Accepted（Checkpoint 8 修订）

日期：2026-07-18

## 背景

Checkpoint 2 最初尝试通过 `axdisplay` 的缓存 descriptor 和 framebuffer 回调接入显示。
真实运行时 resize 复核证明，该层缓存不能表达设备配置变化；继续扩展会扩大共享显示 API，
也会让 framebuffer 所有权和 DMA 生命周期跨越不必要的层级。

最终实现由桌面平台层直接持有 `virtio-drivers 0.13.0` 的 `VirtIOGpu`、transport 和 DMA
framebuffer。开发服务器没有 X11/Wayland，会话中也没有 `DISPLAY`；运行时分辨率变化必须
通过真实设备配置中断/轮询、真实 framebuffer 重建和 QMP/VNC 证据验证，不能由 host
MemoryDisplay 或固定截图替代。

## 决定

1. 桌面内部像素统一保存为 `0xAARRGGBB`，提交到设备时显式编码为 BGRA8888。
2. 可见 stride 按 `VirtIOGpu` 当前 BGRA scanout 合同使用 `width * 4`，只要求 DMA slice
   至少容纳 `stride * height`；禁止用页对齐后的 allocation size 推导 stride。
3. `Surface` 是后备缓冲；绘制只修改后备缓冲，`present` 只复制裁剪后的 damage region，
   然后进行一次设备 flush。空 damage 不复制、不 flush。
4. 默认关闭的 `axdriver/desktop-device-hook` 仅在既有 MMIO/PCI 枚举和 BAR 配置生命周期中
   把候选设备交给桌面探测；GPU/input transport 及设备状态均保存在
   `user/desktop/src/platform/`，默认 feature graph 不启用该 hook。MMIO range 在 hook 内只
   构造一个 transport，再按真实 `DeviceType` 移交；PCI 通过 opaque C ABI pointer 在同步
   调用期间借用 axdriver 的唯一 `PciRoot<MmioCam<'static>>` 和当前设备信息，不从同一 ECAM
   构造第二个 root、不保留 pointer。input registry 在 transport/driver 初始化前预留容量，
   应用启动只报告 hook registry，不进行晚期全总线重扫。
5. `DesktopVirtIoHal` 的 DMA 页在交给设备前完整清零；MMIO/PCI 映射、DMA tuple 和
   direct-map 假设记录在 `unsafe impl Hal` 旁。framebuffer 裸视图只指向当前 GPU 实例拥有的
   DMA slice；重配置先使旧视图失效，再调用可能释放旧 allocation 的
   `change_resolution`，失败路径不得再次解引用旧地址。
6. 分辨率变更前拒绝零尺寸和 `u32 width * height * 4` 溢出；成功后重建 Surface、workspace
   和窗口约束，clamp 光标并更新绝对输入 extent。
7. host 端使用带可配置 stride 的 `MemoryDisplay` 验证 clipping、alpha、padding、damage
   与确定性；golden 使用无损 PPM，记录内部 frame checksum 和文件 SHA-256。
8. host golden 只能证明软件渲染结果，不能替代 guest 启动、VirtIO GPU flush、QMP 截图或
   输入证据。QEMU 证据必须在后续 checkpoint 单独记录。
9. 运行时 resize 通过 localhost-only RFB `SetDesktopSize` 触发；summary 必须同时绑定初始
   geometry、VNC 请求、guest `DISPLAY_CHANGED`、resize 后绝对指针、最终 900x650 截图和
   QEMU 正常退出。

## 后果

- 最终方案不修改 `axdisplay`、默认显示 API 或根构建默认值，也不引入第三方 UI 依赖。
- 当前 pixel format 判断依赖 `virtio-drivers 0.13.0` 的 GPU scanout 合同。若未来加入其他
  显示设备，必须在桌面 platform 边界显式报告 stride/format 或拒绝不支持的格式，不能猜测。
- 设备 flush 目前仍由桌面持有的 VirtIO GPU 对整个 scanout 执行；桌面侧已经避免无 damage 的
  flush 和不必要的整屏内存复制。更细粒度设备 flush 需要独立 bridge 决策。
- PPM 的观察性 PNG 转换不属于产品构建，也不改变 golden 的权威哈希。
- 最终 RV64 `qemu-rv-resize.5a7d8f` 与 LA64 `qemu-la-resize.18f76c` 分别从 1024x768、
  1280x800 变更到 900x650。summary 强制 `DISPLAY_CHANGED < FRAME input < center
  PointerMoved`；两份 serial 的对应行序分别为 `91<93<94`、`33<35<36`，证明中心指针
  `(450,325)` 的处理与 present 早于 capture。两份 summary 均为 PASS、QEMU exit 0，最终
  截图已实际检查。更早 `ErRQSB/2lYk5q` 早于该 fail-closed 协议，只作历史证据。
