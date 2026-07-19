# OrayS Desktop 无显示开发约定

## 宿主环境

服务器不需要 X11、Wayland 或桌面会话。

启动桌面开发前清除：

```bash
unset DISPLAY WAYLAND_DISPLAY MIR_SOCKET
export ORAYS_DESKTOP_HEADLESS=1
```

## 验证路径

### Host

- 内存 framebuffer；
- PPM/PNG 输出；
- golden comparison；
- SHA-256；
- 窗口和输入状态机测试。

### QEMU

优先：

- VirtIO GPU；
- VNC backend 仅绑定 `127.0.0.1`；
- QMP Unix socket；
- serial 文件；
- `screendump`；
- 输入注入。

实际统一入口：

```bash
scripts/desktop/run-headless-qemu.sh --arch rv --scenario boot
scripts/desktop/run-headless-qemu.sh --arch rv --scenario launcher
scripts/desktop/run-headless-qemu.sh --arch rv --scenario overlap
scripts/desktop/run-headless-qemu.sh --arch rv --scenario applications
scripts/desktop/run-headless-qemu.sh --arch rv --scenario resize
scripts/desktop/run-headless-qemu.sh --arch la --scenario boot
scripts/desktop/run-headless-qemu.sh --arch la --scenario launcher
scripts/desktop/run-headless-qemu.sh --arch la --scenario overlap
scripts/desktop/run-headless-qemu.sh --arch la --scenario applications
scripts/desktop/run-headless-qemu.sh --arch la --scenario resize
```

runner 在截图前等待来宾 boot/action marker。launcher 还必须在完成 present 后到达唯一
`OPEN_STABLE`，capture 进程在实际 `screendump` 前把 action < stable 的 serial prefix
SHA-256/行号写入 sidecar；resize 必须满足
`DISPLAY_CHANGED < FRAME input < center PointerMoved`。缺 marker、sidecar、QMP transcript、
输入序列、合法 P6 截图或 QEMU exit 0 中任何一项都会失败。boot 保存只有等待步骤的显式序列
和 QMP handshake，不会把零输入记作键鼠交互。

禁止：

- `-vnc 0.0.0.0:*`；
- 公网 VNC；
- 人工依赖 QEMU GUI；
- 为验证安装完整桌面环境。

## 证据目录

```text
test/output/desktop/<run-id>/
├── serial.log
├── qmp-input.jsonl
├── qmp-capture.jsonl
├── input-sequence.json
├── capture-precondition.json  # launcher only
├── vnc-resize.json            # resize only
├── frame.ppm
├── hashes.sha256
└── summary.json
```

PNG 仅为人工查看时由本机已有工具在忽略目录生成的观察副本；PPM、transcript、serial、条件
sidecar、hash 和 summary 是 runner 的权威输出。每次运行目录由安全输出目录 helper 新建，
runner 拒绝覆盖既有目录、symlink traversal 或写出 `test/output/desktop/`。

## 视觉验收

至少保留：

- 启动桌面；
- 启动器；
- 重叠窗口；
- 文件管理器、编辑器或系统监视器交互。

若 Codex 无法直接查看图片，保留文件和哈希供人工复核；协议 PASS 不能替代视觉复核。

## 已观察限制

- 当前 FAT directory metadata 路径会在串口留下 `Is a directory` 诊断；目录枚举可继续成功，
  原始文本必须保留，不能从 summary 或日志中隐藏。
- 宿主缺少 ImageMagick `convert`；现有 ffmpeg 只用于把忽略目录中的 PPM 转成观察 PNG，
  不参与 PASS 判定。
