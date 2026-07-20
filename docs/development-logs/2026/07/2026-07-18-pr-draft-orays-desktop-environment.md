# PR Draft：OrayS 图形桌面环境

日期：2026-07-18

分支：`feature/orays-desktop-environment`

状态：`BLOCKED_WITH_EVIDENCE`（B-02 与第三轮外部条件未关闭；不可标记 PR Ready、不可合并）

## 基线

- HEAD：`c776ceff40587de0fa0547724d0abfecbb56cc64`
- 基线：`.codex/state/desktop-base-sha`、`develop/post-integration-next` 与当前 HEAD 一致
- worktree：`/root/OrayS-desktop`
- 当前分支：`feature/orays-desktop-environment`
- 接管状态：11 个未跟踪文件、0 个已跟踪修改；逐项审计并保留
- 默认构建结果：逐字节匹配当前实现与测试文件集的精确干净快照中 `make kernel-rv`、
  `make kernel-la` 和 `make all` 均退出 0；更早 RV64 首轮 SIGTERM/exit 143 原样保留
- quick：同一精确 tree 的一轮为 45/45 PASS，独立 reviewer 首轮为 44 PASS / 1 TIMEOUT；
  `unit.suite_runner` 靠近 300 秒阈值，不用成功运行覆盖超时
- baseline：精确 reviewer 快照 57/57 项得到终态，53 PASS / 3 FAIL / 1 INFRA_ERROR；不是 PASS
- 已有失败：见 `docs/defect-and-reliability-status-2026-07-18.md`；本 PR 不改变其判定

## 目标

实现默认关闭、独立构建、可在无显示服务器验证的现代 OrayS 图形桌面。

## 非目标

见 `.codex/tasks/GOAL_DESKTOP_ENVIRONMENT.md`。

## Checkpoint 日志

### Checkpoint 0

2026-07-18 完成新会话接管和真实基线：

1. 完整读取根与 `user/desktop` 规则、Goal、根构建/测试入口、显示/驱动/runtime 边界、
   当前缺陷与可靠性文档、现有计划/日志、CI 和 `.codex/state`。
2. live 记录分支、HEAD、worktree；确认 base SHA 为
   `c776ceff40587de0fa0547724d0abfecbb56cc64`。
3. 逐项读取 11 个遗留未跟踪文件。保留其正确方向，但不把脚手架当成桌面实现；接管时
   `user/desktop` 只有 `AGENTS.md`。
4. `scripts/desktop/check-headless-host.sh`：退出 0，`PASS_WITH_WARNINGS`。缺少可选
   `socat`、ImageMagick `convert`，Python 3.10 无 `tomllib`；两套 QEMU 均有潜在
   headless backend，`DISPLAY`/`WAYLAND_DISPLAY` 均未设置。
5. `python3 scripts/desktop/check-scope.py`：退出 0，11 changed paths、0 bridge、
   `DESKTOP_SCOPE=PASS`。
6. 首次直接 RV64/LA64 基线均退出 2：根 Makefile 离线恢复要写只读 `vendor/`；在
   `build/desktop/` 解包隔离 Cargo 源后，构建又因现有
   `api/arceos_posix_api/build.rs` 要写只读的
   `ulib/axlibc/include/ax_pthread_mutex.h` 而退出 2。这是 sandbox/构建脚本写源树边界，
   不是架构编译结论。
7. 未扩大 `vendor/`、POSIX 或 ulib 权限。使用 `git archive HEAD` 创建
   `build/desktop/baseline/source/` 精确快照，并在可写快照执行同一构建：RV64 退出 0
   （约 1m47s），LA64 退出 0（约 1m43s）。已有 warning 保留。

产物：

- `build/desktop/baseline/snapshot-build/kernel-rv`：2,024,592 bytes，SHA-256
  `126f55442212669a6690d811da4e9f8d76d357fa722b7816b652b956f46d886f`；
- `build/desktop/baseline/snapshot-build/kernel-la`：3,078,616 bytes，SHA-256
  `f8f5f50735e9de9713b9b86776c76831f6332c110ed9f61e669ae0bc032a1e53`。

Checkpoint 0 结论：基线、隔离、headless 能力和 scope 预算已建立；进入技术选型 spike。

### Checkpoint 1

比较固定 LVGL `v9.3.0`、无新增依赖的纯 Rust/no_std renderer 和
`embedded-graphics = 0.8.1`：

- 本地没有 LVGL 源码；公开标签查询在普通及 auto-review 网络执行中都因 DNS 失败退出
  128。没有虚构 tag commit、许可证或构建结果。
- C 工具链 probe：RV64 退出 0；LA64 退出 1（clang 14 unknown target triple）。
- 纯 Rust probe：RV64/LA64 rlib 均退出 0；host stride/clipping/buffer validation 2/2
  PASS；无依赖、无 unsafe。
- embedded-graphics 离线 resolver：两架构均退出 101，固定 source 中 package missing。

决定采用纯 Rust/no_std 软件渲染和内置控件。完整比较、失败证据、许可证边界和临时产物
哈希见 `docs/decisions/0002-desktop-ui-toolkit.md` 与
`docs/references/desktop-dependencies.md`。

### Checkpoint 2

实现独立纯 Rust/no_std framebuffer 渲染基础：

1. 建立独立 `user/desktop` workspace、lockfile、host renderer、MemoryDisplay、Surface、
   clipping、alpha、文本、damage 合并与双缓冲提交。根 workspace 和默认构建入口未改变。
2. Checkpoint 2 曾在 `axdisplay` 增加 20 行回调式 framebuffer 写 bridge。后续真实 resize
   复审证明缓存 descriptor 无法表达设备配置变化；该 bridge 及全部 `axdisplay` 改动已在
   Checkpoint 8 删除。本段只保留历史演进，不描述最终架构。
3. `DisplayInfo.fb_size` 可能包含 DMA 页对齐 padding，因此使用当前唯一 VirtIO GPU 的
   `B8G8R8A8UNORM` 紧密布局合同推导 `stride = width * 4`，并验证可见大小不超过分配。
4. 新增 `scripts/desktop/build.sh`：固定 archive 解包、Cargo source、axconfig、linker、
   target、ELF/BIN 全部写在 `build/desktop/`。`Makefile.desktop` 与子目录 Makefile 均只
   显式启用桌面，不改变默认 `make`。
5. host tests 最终 8/8 PASS。1280×720 golden 三次逐字节一致；内部 checksum
   `08135c358d1a61b0`，PPM SHA-256
   `e7587a142932adbb41286cd29897daff11e63b0f0fb0fcdf0cd9c6a06c87921b`。由于本地查看器
   不支持 PPM，使用 ffmpeg 在忽略的 output 目录生成 PNG 观察副本并实际检查；此转换不
   属于产品依赖或 guest 证据。
6. 最终独立 release 构建：RV64 与 LA64 均退出 0。产物分别位于
   `build/desktop/rv/artifacts/` 和 `build/desktop/la/artifacts/`。

真实失败和修复轨迹：

- 首次离线 lockfile 因独立 workspace 未继承根 `[patch.crates-io]` 而退出 101；在桌面
  清单中显式加入四个现有本地 patch 后成功。
- golden renderer 先后因 host error conversion 与输出父目录不存在退出 101/1；补齐
  错误映射和目录创建后成功，没有吞掉退出码。
- 首次双架构直接 Cargo 构建分别因缺 `defplat`、使用错误 LA target 和缺 OrayS linker
  参数退出 101。最终 build wrapper 使用仓库规范 target、生成的 axconfig/linker 与
  `-znostart-stop-gc`，两架构均完成最终链接。
- rustfmt 检查曾两次因新文件排版退出 1；限定 `user/desktop` 格式化后复验退出 0。
- shell `install` 新增根 `Makefile.desktop` 因 mount 只读退出 1；随后使用允许的补丁通道
  新增同一审阅内容成功。根 `Makefile` 未修改。

### Checkpoint 3

实现输入 bridge、语义层和无显示注入：

1. `axdriver/virtio-input` 为默认关闭 feature，复用固定 `virtio-drivers 0.13.0` 的
   `VirtIOInput`。MMIO/PCI bus 识别 input device 后注册到 4 槽全局 registry；轮询从上次
   命中设备的下一槽开始，避免 keyboard 长队列永久饿死 pointer。
2. 每个绝对设备在初始化时查询 X/Y `ABS_INFO`。桌面按设备报告 min/max 映射分辨率，
   不写死 QEMU tablet 范围。相对 value 以有符号值处理。
3. 桌面输入层覆盖按下/释放/重复、左右 Shift/Ctrl/Alt/Super、真实 evdev 键位、相对/绝对
   指针、左中右键、滚轮、有界队列和丢弃计数。无事件时 guest sleep 8 ms。
4. `inject-input.py` 验证固定 JSON 后通过 Unix QMP 发送 `input-send-event`，QMP error
   立即失败；失败时也写部分 transcript。Python 假 QMP 测试验证 greeting、capabilities、
   command 和 response。
5. host 最终 13 项 Rust integration tests（图形 8、输入 5）和 3 项 Python tests 均
   PASS；双架构 release 最终链接退出 0。
6. RV64 QEMU 用 localhost-only VNC、Unix QMP、VirtIO block/GPU/keyboard/tablet 真实启动。
   串口会话出现 `a` 按放、指针 `(511,191)` 和左键按放 marker；QMP screendump 成功，
   PNG 已实际查看；最后 QMP `quit`，QEMU 退出 0。

真实失败和限制：

- 试图写 Git checkpoint 时 `git add` 即使通过最窄 `require_escalated` 仍退出 128：共享
  worktree Git metadata 的 `index.lock` 位于只读 mount。产品文件未丢失，但目前无法创建
  本地提交或满足 canonical suite 的 clean-tree 前置条件。
- 首次 bridge 构建 RV64 因 `VirtIoHalImpl` 可见性错误退出 101；LA64 因把不同 error
  type 的 PCI transport/input 构造直接 `and_then` 而退出 101。修复后两架构均退出 0。
- 首次真实 QMP 序列的 relative event 返回 `Input handler not found for event type rel`，
  因该 guest 配置 absolute tablet 而未配置 relative mouse；工具退出 1。改用 absolute
  event 后成功，相对语义仍由 host 测试验证。
- `-serial file:` 生成的文件为空；当前 RISC-V 平台/固件 console 实际进入 QEMU 进程
  stdout。Checkpoint 7 runner 必须捕获进程 stdout，不能把空文件当作串口证据。

### Checkpoint 4

实现可由真实输入驱动的窗口状态机与 damage compositor：

1. `Window` 保存稳定 ID、标题、当前/恢复 bounds、minimum size、normal/minimized/
   maximized 状态、modal owner 和 close/resize capability；全部位于独立桌面 workspace。
2. `WindowManager` 以 vector 的 bottom-to-top 顺序实现 focus/z-order、create/close、
   Alt-Tab、move、四边/四角 resize、minimize、maximize/restore。owner 关闭时递归回收 modal
   children；active modal 会阻止 owner 和无关窗口取得焦点或改变状态。
3. 命中测试区分 client、title bar、三个窗口控制、resize edge/corner、desktop 和 modal
   backdrop。窗口移动至少保留可抓取标题区域；resize 服从 minimum size 与 workspace。
4. compositor 对每个 damage clip 重建全局坐标一致的 wallpaper、透明阴影、窗口装饰和
   cursor；未让控件自行 flush。窗口状态变化记录 old/new decorated bounds，cursor 记录
   old/new union。
5. `WindowedDesktop` 把已翻译输入接入窗口管理器并只在 damage 非空时 present。guest 主
   入口创建 FILES 与 SYSTEM MONITOR 两个重叠窗口；左键拖拽和 Alt-Tab 产生新 frame marker，
   空闲仍 sleep。
6. host 最终 graphics 8、input 5、window/runtime 7，共 20/20 integration tests PASS；
   增量合成测试逐像素等于相同状态的 fresh full render。host clippy `-D warnings` 退出 0，
   RV64/LA64 release 均完成最终链接。

真实失败和修复轨迹：

- 首次 host build 因 cursor 宽度推导为 `i32` 而退出 101；显式区分坐标与尺寸类型后修复。
- 首次增量合成一致性测试真实 FAIL，首个差异在 `(105,372)`。根因是阴影向下偏移 3 px，
  symmetric damage margin 少覆盖最底 1 px；扩大 decorated damage bottom 后 7/7 窗口测试
  通过。失败期间未弱化逐像素断言。
- 首次 host clippy `-D warnings` 因三个 collapsible-if、输入队列缺 `is_empty` 和旧 golden
  writer 未使用 inline format args 退出 101；逐项修复后退出 0。

### Checkpoint 5

实现完整但默认关闭的桌面 Shell 与有限动画：

1. `DesktopShell` 根据实际 framebuffer 分辨率计算 top bar、workspace、Dock 和 overlay；
   1280x720 为默认布局，较小尺寸也保留可用 workspace。背景、orb、图标和文字均由内置
   矢量/位图 primitive 生成，不依赖未登记外部资产。
2. 新增 dark/light palette、半透明顶栏和 Dock、六应用启动器、真实通知队列和电源菜单。
   通知最多保存 4 条并按真实 elapsed time 过期；顶栏显示真实 `UP HH:MM`，没有虚构日期、
   CPU 或内存数据。
3. `ShadowCache` 固定缓存 16 级阴影 falloff，圆角和透明度由软件 renderer 处理。没有引入
   实时全屏高斯模糊，也没有让单个控件自行 flush。
4. launcher open/close 和 window open/close/minimize 使用 160 ms 有界状态机。每个中间
   damage frame 与相同状态的 fresh full composition 逐像素比较；结束后动画队列为空，空闲
   不继续重画。
5. shutdown 调用 `axstd::process::exit(0)`，进入现有 OrayS 退出/关机路径。当前 runtime
   没有可调用 reboot API，restart 入口保留但明确输出
   `ORAYS_DESKTOP_SYSTEM_ACTION_UNSUPPORTED restart_supported=false`。
6. host 最终 Rust integration tests 28/28 PASS，Python discovery 6/6 PASS，host clippy
   `-D warnings`、rustfmt、diff check、scope check 均通过；RV64/LA64 release 最终链接成功。

真实失败和修复轨迹：

- Shell 初次接入 compositor 时因在 `const fn` 中调用非 const constructor 而退出 101；
  把初始化放回运行时后修复。
- 首次 Shell damage/full equality 在 `(32,247)` 失败。根因是旧 damage margin 小于新的
  16 px 缓存阴影范围；扩大窗口 decorated damage 后逐像素一致，未弱化断言。
- 首张实际观察的 boot 图顶部和 Dock 为黑色。根因是初始 damage 只覆盖 workspace；构造
  runtime 时加入完整 desktop bounds damage 后修复。该问题由真实图像检查发现。
- 首次 CP5 双架构构建均退出 101：`idle_ms` 被推导成 `u64`，而动画 tick 接受 `u32`；
  显式分离 tick 毫秒和 `Duration` 转换后，两架构均退出 0。
- 首次以 package path 调用 Python unittest 因 `test.desktop` 不是 package 而 import 失败；
  使用 discovery 入口后 6/6 PASS，错误未被记为通过。

场景证据（1280x720 PPM）：

- boot：SHA-256 `6d834114499d957e60c536b90319356fa1641548330f898f957e174330c9d4e4`；
- launcher：`8e6520b431ef532e340cf3ac8c66cc519f370b810b32e06402dc6178aaa08679`，
  重复生成逐字节一致，内部 checksum `cb90a351299f8605`；
- light：`694a3cefdb8b31776e7d0de14aa1415e8ef24330bca052bc5ce4e26bbf3179cc`；
- power：`ee5dadd7e0c0deba943e58775a2a7f2141a23def3ed757285559204ff3d87283`。

最终独立构建哈希：

- RV64 ELF/BIN：`cec70be5db5ca6daeeb4439bb94e3dbb11e3fbb23c7d22413292677f3aaae2b1` /
  `bafcb7aa274c6b943f9d2056005b451bb6d167dc5df2c3ffa166effb1b2fa318`；
- LA64 ELF/BIN：`02f15d8d0a51361ea2d5706ce02a397ad09e2b9d74f0e5c3ceb593c8cdf65af0` /
  `9f348e6098b09f720aadea010387d84c4bb2f2ba7dff795c3e4b12a74af311bc`。

### Checkpoint 6

实现六个内置应用和可复用控件，应用不直接访问内核驱动：

1. `platform/filesystem.rs` 是文件能力唯一入口。OrayS 后端调用 `axstd::fs`，host-tools
   后端调用真实 host filesystem；default/no-backend 明确返回 `Unsupported`。文本读写限制
   1 MiB，文件名 join 拒绝路径分隔和 traversal component。
2. Terminal 提供输入、历史、有界输出、滚动，并执行 `pwd/ls/cd/cat/mkdir/write/echo/
   clear/help`。其中目录和文件命令作用于真实 filesystem，unknown command 与 I/O 错误原样
   呈现，不调用或伪装 `user/shell`。
3. File Manager 真实列目录，目录优先稳定排序；提供 UP/refresh/new/rename/delete、可编辑
   prompt、选择、滚动、文件打开和错误栏。PPM/PNM 打开到 Image Viewer，其他文件打开到
   Text Editor。
4. Text Editor 支持 open/edit/save、Ctrl-S、1 MiB 边界、dirty 标记和关闭拦截；dirty close
   展示 save/discard/cancel prompt，save 失败时保持窗口和错误。
5. Image Viewer 实现有边界检查的 P3/P6 PPM parser、最大 4096 单边/16M pixels、truncation/
   invalid sample 错误和 nearest-neighbor fit/25%-400% zoom。
6. System Monitor 的 desktop elapsed、window count 和 input event count 来自真实运行时状态，
   每秒更新；CPU/memory capability 在 `platform/system` 明确为 false 并显示 `UNSUPPORTED`。
   Settings 通过 app action 实时更新 shell theme 与三种程序化 wallpaper。
7. 应用注册表按稳定 WindowId 保存模型，在窗口关闭动画完成后回收；键盘送给 focused app，
   scroll/click 按 client hit target 路由。应用 damage 仍由 compositor 统一 commit。

真实失败和修复轨迹：

- 第一次 CP6 编译因补丁尾部残留重复 import 和 mutable pattern guard 退出 101；删除重复项并
  改为显式 match 后，原有 28 tests 重新通过。
- 第一次 CP6 clippy 因 `unwrap_or_else` 可替换为 `unwrap_or_default` 退出 101；修复后
  `-D warnings` 退出 0。
- 第一次加入 app tests 后 5/8 FAIL：integration-test dependency 没有 `cfg(test)`，因而选择
  fail-closed Unsupported backend，真实 host 文件操作没有执行。没有把 Unsupported 当 PASS；
  `host-test` 显式启用已有 `host-tools` feature 后 8/8 通过，最终扩展为 9/9。
- 实际查看第一版 application PNG 后发现 top-most monitor 遮挡 editor；调整仅限 host scene 的
  窗口顺序/位置后，file manager、真实 editor content 和 monitor 的真实/unsupported 指标均
  可见。最终 PPM SHA-256 为
  `8f0f9d945b1f0081b73c24bd6eb3f24374306d90d00ad662711f07ae3059ab54`。

最终验证：host Rust 37/37、Python 7/7、clippy、rustfmt、diff 和 scope 均 PASS。RV64
ELF/BIN SHA-256 为 `bb1abe54007918662f1876b2d6e483190156a5eb92f52f9c27bca05c3e57d0e8` /
`dd3a7c560fb877a37630f8866318743cb56f5bc00427e64a0b1414929046e940`；LA64 为
`a7566be1ab63cf6a318841f73afc6544d53d2676dd9445215f03d318d43be886` /
`182786b5cbeade6d549ded9b9cf1a29811063a6710fe8ab8ebfd59592255a69f`。

### Checkpoint 7

1. `run-headless-qemu.sh` 为每次运行创建不可覆盖的新证据目录，构建对应架构，创建临时 FAT32
   disk，并以 localhost-only VNC、Unix QMP、VirtIO GPU/keyboard/tablet、串口 stdout 启动
   QEMU。没有依赖 X11、Wayland、DBus session 或人工点击。
2. `inject-input.py` 保存确定性输入序列与逐消息 QMP transcript；`qmp_screendump.py` 保存
   capture transcript，截图后以 QMP `quit` 正常结束。boot 使用显式无事件等待步骤，仍保存
   input sequence 与握手 transcript，但 `input_marker_count=0`，没有伪造交互。
3. 来宾为 boot frame、launcher open、Alt-Tab、应用启动、theme change 输出稳定 action marker。
   runner 在截图前等待场景所需 marker；`summarize-run.py` 对 QEMU exit、marker、transcript、
   输入序列和 P6 截图大小/哈希统一 fail-closed。

真实失败和修复轨迹：

- 第一次在真实 elapsed tick 后运行 applications 的 `qemu-rv-applications.3AdahW` 得到 FAIL：
  QMP 输入和截图存在、QEMU exit 0，但来宾尚未消费 theme click，缺少 `THEME Light` marker。
  没有接受该截图；runner 改为等待预期 guest action marker，随后场景通过。
- 第一版 boot 对 boot 特判，不保存 input sequence/QMP input transcript。按 Goal 的“每个场景”
  合同新增仅等待的 boot fixture，并收紧 summary 对所有场景检查这两项；旧 boot 证据不再作为
  最终证据。
- 当前源码第一次 LA64 boot `qemu-la-boot.PXTxzM` 协议 PASS，但人工查看 1280x800 截图发现
  monitor footer 与 memory 行重叠。未把协议 PASS 当视觉 PASS；改为紧凑 metric layout、空间
  不足时不画 footer，并增加 410x236/过矮客户区测试。
- 串口真实保留 FAT directory metadata 的多条 `Is a directory` 诊断。目录列表仍成功显示；
  本 PR 不吞日志，也不把它表述为无错误启动。

当前源码最终证据：

- RV64 boot `qemu-rv-boot.ewCKpn`，截图 `f0400b5a52d0cf3fd3057fe9e48dcf787f819430c5583510877b1294075e41bc`；
- RV64 launcher `qemu-rv-launcher.KAog2y`，截图 `63a15c2e77404e8fc3a85dd114f784ef6c627eea2f7a92d767609d7f9732a843`；
- RV64 overlap `qemu-rv-overlap.HvnHin`，截图 `1a5676a1055809f457e821fe250c8c0ab4c542c431c98674c650a830bda0e557`；
- RV64 applications `qemu-rv-applications.OtbvDE`，截图 `c6c05689e2f47437bee9e8602f317c786826d57eb081e8010e9a3993c0233d56`；
- LA64 boot `qemu-la-boot.RLGS51`，截图 `9661a86d385d286a19d617d93fcce17c912f6e38d03b6b65b3ac062e4d0174b4`。

五份 summary 均为 `PASS`、`failures=[]`、QEMU exit 0；最终图像均实际查看。最终 RV64
ELF/BIN SHA-256 为 `3dc1c7540b1058c904617d40020c3e351cb29d4539411573be5106bdfc59167d` /
`cf9e89dad3724870a67e6bf61f48b59c75e6dec3b89f87554005847ad34cd035`；LA64 为
`575a1370f5fa4a2ce922bca0ff95857c724a28d5d336c47be1f66dc98039c654` /
`3c530d07623fd30629ed7c6bee738f3069162040023a92ebac5c5b2978825a0b`。host Rust 38/38、
Python 7/7、clippy `-D warnings`、rustfmt、diff 与 scope 复验通过。

### Checkpoint 8

1. 把 VirtIO GPU、input、transport 与 DMA ownership 收回桌面本地 `platform/`。桌面独立
   workspace 直接使用已固定的 `virtio-drivers 0.13.0`、`kspin` 和 `axdriver_pci`；通用层
   只新增默认关闭、无依赖的 `desktop-device-hook`，在正常 MMIO/PCI 总线发现及 PCI BAR
   配置后调用桌面的两个固定 C ABI 探测符号。根 workspace、根 lockfile、默认 feature、
   根 Makefile 和 ABI 不变；`axdisplay` 无最终 diff，也不在桌面依赖图中。
2. 真实运行证明初始化时机是必要语义：晚期 app 初始化、仅中断 ack 和 display-init hook
   都无法可靠发现 RV64 MMIO 输入。失败证据 `JxEf63`、`axXEWW`、`reX7Y2`、`MEzdPX`、
   `MRHHvC` 原样保留；最终方案在 runner 注入前稳定报告
   `ORAYS_DESKTOP_INPUT_READY devices=2`。
3. LA64 第二输入设备失败的根因不是 guest parser，而是 128 KiB PCI MMIO window 被 QEMU
   隐式 VirtIO NIC 占用。`f4uHuw` 保留 BAR failure，`x4dpQ7` 的实时 `query-pci` 证明分配
   次序。runner 因此只在 LA64 使用 `-nic none`；RV64 保持 MMIO topology。
4. 输入 fixture 的绝对坐标改为按实际 guest geometry 从像素映射到 tablet 0..32767，覆盖
   1024x768 与 1280x800；不再把某一架构的固定像素误用到另一架构。
5. app 初始化阶段直接重新扫描 GPU 会为已归属 input 的 MMIO range 再构造 transport；
   `qemu-rv-launcher.rT4ypp`、`nJDfzO` 因 guest 无输入 marker 超时，均未计 PASS。最终将
   GPU/input 一起放入同一 bus handoff 生命周期，`qemu-rv-launcher.E1jvXp` 恢复真实输入。
6. `DesktopVirtIoHal::dma_alloc` 对页数乘法和零页 fail-closed，并在返回前清零全部 DMA 页。
   `OraysDisplay` 在分辨率变化前验证零尺寸与 `u32 width * height * 4` 溢出，先使旧 framebuffer
   `NonNull` 失效，再调用可能释放旧 allocation 的 `change_resolution`；失败路径的
   `present` 只能返回 `DeviceFailure`。相关 `unsafe` 不变量与调用者责任写在实现旁。

2026-07-19 终审整改前的 headless 证据（已 stale）：

- RV64：`qemu-rv-boot.9mYVSe`、`qemu-rv-launcher.oomKC5`、
  `qemu-rv-overlap.wi0gXl`、`qemu-rv-applications.UTgANh`、
  `qemu-rv-resize.ErRQSB`；
- LA64：`qemu-la-boot.rts9w2`、`qemu-la-launcher.6AtklF`、
  `qemu-la-overlap.Jysffs`、`qemu-la-applications.mNI9es`、
  `qemu-la-resize.2lYk5q`。

十份 summary 均为 `PASS`、`failures=[]`、QEMU exit 0；boot 明确为 0 input marker，
launcher/overlap 为 4、applications 为 6、resize 为 1 个真实 guest input marker。普通 RV64
截图为 1024x768，普通 LA64 为 1280x800；resize 均为 900x650。十张最终截图均已实际查看，
没有发现 clipping、空白帧或文字重叠。截图 SHA-256 依次为
`b88affa91988822e58e0f52714438e4979a8f1aa0aeec2616e0d50254427140c`、
`91eda043e096f952aa813d15c29edc96a3e1edebce5b4dc48236605a6f481acc`、
`1a5676a1055809f457e821fe250c8c0ab4c542c431c98674c650a830bda0e557`、
`c6c05689e2f47437bee9e8602f317c786826d57eb081e8010e9a3993c0233d56`、
`8185dac94cfaf1c80a5262c3e20e80016328828b0d1ec4b5102179f36fa7afa1`、
`406af959897dcba1f2a770c20e5139416ed8937e6a22d98a21af0b8e58e5e7f2`、
`a1a1ee9c9e0d9045c02021018280788295db43e87d087e301734bbb651b676eb`、
`6f69810e35bf49e042c875a7606ccb0c0b2d68fe7a4b5765e979ae76f5f79587`、
`3056b5749344f40c4a247a853371fa61a4488676d4c570e5ba8f1916a83ee169`、
`d5c95b6a72771eb7e75c87658d504aeef9967c940d838073a6ee8a5eea50f5d1`。

resize 使用 localhost-only RFB `SetDesktopSize`，不是 host 模拟或固定截图。RV64 从 1024x768、
LA64 从 1280x800 变到 900x650，随后 guest 均消费按新 extent 映射的中心绝对指针
`(450,325)`。不能对 realized QEMU GPU 修改 `xres` 的旧 QMP 尝试
`qemu-rv-resize.qqj3OL` 保留为失败，不计 PASS。

终审整改前定向验证：host Rust 42/42、Python 20/20、host/RV64/LA64 clippy `-D warnings`、rustfmt、diff、
asset 和 golden comparison 均 PASS。性能记录为 7 个原始 sample，1024x768 release host
software compositor min/median/max 48.245/48.549/48.908 ms，范围不含 PPM 写入且没有设
flake 阈值。scope 为 102 paths / 3 bridge / 3 existing / 56 churn，低于 8/250 预算。
终审整改前 desktop release 的 RV64 ELF/BIN 为
`20d8794b188040d0fe4243b23672a0a70022962aed51c722f0184c42d95e50a5` /
`321d28ca75dcd54670e27b848e0465ba4bb4272e474a32eace7911854ea91ea2`，LA64 为
`af5d71c2376c649c461835a3dd1b3af847718be67e585ab1f11e98d52132a6b4` /
`b802e12179837aaa2c4b0df6f6567c1464df959104550b546d585dff77d508d5`。

较早回归在 `/tmp/orays-desktop-final-source.I527qe` 建立当时逐字节匹配的干净源码快照；
根 `Cargo.toml`、`Cargo.lock`、`Makefile` 哈希未变。其 canonical quick 为 45/45 PASS，证据为
`test/output/desktop/final-gates/quick-24dab13-retry1/summary.json`。canonical baseline
57/57 项均执行，结果 51 PASS / 3 FAIL / 3 TIMEOUT，证据为
`test/output/desktop/final-gates/baseline-24dab13-corrected/summary.json`。PASS 项包括格式、
73 个 workspace unit tests / 55 blocks、默认及双架构 clippy、双架构 fixed kernel build 和
submission build。

baseline 的首次非 PASS 原样保留：`evidence.riscv64` 因系统 QEMU 6.2.0 不满足固定 9.2.4
前置条件失败；`evidence.loongarch64` 真实输出
`PR3_SMOKE_V1 USER_FAIL tee_device_mode` 与
`PR3_SMOKE_V1 HARNESS_FAIL ... guest_nonzero_exit`；`evidence.aggregate` 继承两架构结果。
`unit.suite_runner`、`unit.synthetic_capability_integrity`、
`unit.syscall_boundary_regressions` 首次分别超时。后续 quick 45/45 证明单元项可通过，但不
覆盖首次超时，仍按 flake/既有缺陷记录。runner 已终止进程组，复核未发现残留 cargo、QEMU
或 suite 进程。

该 canonical 证据早于最终直接 GPU ownership、DMA zeroing 和 framebuffer 失败路径整改，
因此只作为历史 gate 结果。

终审整改前回归建立了逐字节匹配 102 个 task path 的隔离快照。最初 synthetic root commit
`368ab67367569fae1ace364a2d849cb69e1451bc` 不含 canonical `origin/main`，quick 在任何 case
启动前以 infrastructure error / exit 2 失败；该结果不计 PASS。保持完全相同 tree 后，以真实
当前基线 `c776ceff40587de0fa0547724d0abfecbb56cc64` 为父提交建立
`0924ee3083eb8016f9aef32edb914c00b239ba3a`，并绑定本地固定
`origin/main=921171ac1ef5c85ab5a7cd1882dd40e1471b79f0`。quick/baseline summary 均证明 runner
起止 commit 一致、dirty=false、`runner_provenance_stable=true`。

该修复前快照默认 RV64 首轮构建被执行环境 SIGTERM（exit 143），相同命令独立重跑 exit 0；
LA64 exit 0。产物分别为 2,024,584 bytes /
`c2785bd39db8da9c216bb17c3fceaeff1880977ee0f4be265b12567180aa528a` 与
3,078,616 bytes / `85f7a0ebe3a8d06a509528d5b1b4f46c4c1db8eaa1a7dff14a3f95d07ac05685`。

该修复前 canonical quick 45/45 PASS，summary
`test/output/desktop/final-gates/snapshot-0924ee30-quick/summary.json`，SHA-256
`9bf9a934235e96054bd92218eb791160bf20252f6d512e6851b9f1e746350398`。当前 baseline
57 项均有终态、56 项实际启动，结果 53 PASS / 3 FAIL / 1 INFRA_ERROR / 0 TIMEOUT /
0 CRASH，退出 2；summary
`test/output/desktop/final-gates/snapshot-0924ee30-baseline/summary.json`，SHA-256
`07395183fd31eda1c89da0ee1c8af85c79fe15088df8a42aec253ca9296c3259`。三项 FAIL 与
历史一致：RV64 要求 QEMU 9.2.4 而系统为 6.2.0；LA64 输出
`PR3_SMOKE_V1 USER_FAIL tee_device_mode` 和
`PR3_SMOKE_V1 HARNESS_FAIL ... guest_nonzero_exit`；aggregate 继承非通过。额外一项
`baseline.clippy_loongarch64` 在命令启动前因 clang target
`loongarch64-unknown-none` capability probe exit 1，被严格记录为 INFRA_ERROR。格式、
73 个 workspace tests / 55 blocks、default/RV64 clippy、双架构 kernel 与 submission build
均 PASS；没有把 infrastructure error、guest failure 或历史 timeout 改写成成功。

首轮独立只读 review 无 Blocker，发现以下整改项：

1. 最初为 `axdisplay` framebuffer bridge 增加默认关闭的 feature；后续复审证明真实 backend
   仍缓存 geometry，最终删除 bridge 和全部 `axdisplay` diff，改为桌面直接持有 GPU。
2. QEMU summary 现在要求精确 `INPUT_READY devices=2` 和唯一 display geometry，解析 QMP
   greeting/命令/成功响应，把 sequence 逐事件绑定到 input transcript，验证 screendump 目标、
   guest/frame geometry 和非单色截图；5 项新测试覆盖完整 CLI 与 readiness/QMP/sequence/
   uniform frame 篡改。
3. 三项直接依赖补齐精确版本、registry checksum、repository、许可证/版权、NOTICE 与固定
   离线 archive 路径。
4. display backend 可刷新 descriptor；运行时尺寸变化会重建 Surface、workspace 并重新约束
   normal/maximized/restore window bounds，同时 clamp cursor 和更新 input extent；host 回归
   验证 shrink 后的 normal/maximized/restore bounds。Settings 实例也同步当前 live theme/
   wallpaper。

修复后的首次 QEMU CLI 运行暴露 summary 写元数据时的 `NameError`；
`qemu-rv-boot.Ykb1TB`、`qemu-rv-launcher.TGmZUm` 保留为失败证据且未计 PASS。补充 summary
CLI 端到端测试后，从头执行最终 8 场景并全部通过严格 summary 与图像复核。

后续安全复审发现两项 Major：重配置失败路径可能保留指向已释放 DMA 的旧 framebuffer
pointer；`DesktopVirtIoHal` 未满足 DMA 页必须清零的 `Hal` 合同。上述两项均已修复并通过
host 42/42、双架构 build/clippy 和双架构真实 resize。另一次直接 GPU 晚扫描造成 MMIO
transport 所有权重叠，失败 `rT4ypp`/`nJDfzO` 后改为统一 bus handoff。以上矩阵与
`snapshot-0924ee30-*` 均早于 2026-07-19 终审整改，明确为 stale，不用于 current-source 签收。

### 2026-07-19 终审整改与 current-source 定向证据

独立 reviewer 继续发现并推动以下问题闭合：

1. 平台文本读取和图片读取原先可能无界分配；现分别以 1 MiB/64 MiB 上限流式读取，先检查
   metadata，再用额外 1 byte probe 检测读取期间增长。真实 sparse oversized 文本/图片回归
   均 fail-closed。
2. 设备 hook 原先可能在初始化失败或 registry 满槽时返回错误 claim 语义。单槽/多槽注册现在
   返回真实 bool；input registry 在 `PciTransport`/`InputDriver` 初始化前原子预留容量，失败
   取消、成功 commit，满槽测试证明初始化闭包不会执行。
3. 删除 `input::initialize` 的零设备晚期 MMIO/PCI 全总线重扫；应用只消费总线 hook registry。
   MMIO hook 只构造一个 transport 后按真实 `DeviceType` 移交 GPU/Input。
4. LA PCI hook 不再从同一 `PCI_ECAM_BASE` 构造第二个 `MmioCam/PciRoot`。axdriver 将唯一活跃
   root 和当前 device info 作为 opaque C ABI pointer 同步传入；两端写明 exact type、唯一借用、
   不得保留和调用期有效的 unsafe contract。host/RV64/LA64 clippy 均以 `-D warnings` 通过，
   reviewer 独立 RV64/LA64 boot 后关闭该 Blocker。
5. `create-run-dir.py` 通过 dirfd、`O_NOFOLLOW` 和逐层打开/创建把输出限制在真实
   `test/output/desktop`；只有 tracked `test/output/.gitignore` 的 clean layout 也可安全创建。
   asset checker 拒绝绝对路径、`.`/`..` traversal、目录/许可证/资产 symlink 和 resolved escape。
6. 编译期 `GLYPH_ATLAS`、六类 `APP_ICON_MASKS` 与已有 16 级 `ShadowCache` 共同满足字体、图标、
   阴影缓存要求；复用/非空/互异测试通过，五场景 golden hash 全部 MATCH，未改变像素输出。

当前定向验证为 host Rust 53/53、Python discovery 28/28、host/RV64/LA64 clippy
`-D warnings`、asset checker、五场景 golden comparison、rustfmt、`git diff --check` 均 PASS。
scope 为 104 paths / 3 bridge / 3 existing / 74 churn，低于 8/250 预算。当前 release 产物：

- RV64 ELF 1,664,336 bytes /
  `2482a3fb02abe09ee936315a8294d5cfb070a0019a06f7d7413341dfc4be6850`；BIN 659,648 bytes /
  `2972760dd93b6aa300e359e178def103f25465495aa1c5b3b0d5cc81b82d86b6`；
- LA64 ELF 1,229,656 bytes /
  `b7e17c3a6de4256f39db873f3bd23e8f911ffc40bcb8b79b1c10c430de071f0a`；BIN 856,256 bytes /
  `eeccc0a16402f837f19322935f2e1c99e9c25f17eb80901c93d2393125c7f572`。

当前源码十场景矩阵：

- RV64：`qemu-rv-boot.49b4a6`、`qemu-rv-launcher.2abaef`、
  `qemu-rv-overlap.7460a2`、`qemu-rv-applications.6aa0f5`、`qemu-rv-resize.f80d09`；
- LA64：`qemu-la-boot.35168e`、`qemu-la-launcher.13447e`、
  `qemu-la-overlap.aca5e1`、`qemu-la-applications.59ba55`、`qemu-la-resize.554bfd`。

十份 summary 均为 `PASS`、`failures=[]`、QEMU exit 0；十份 `hashes.sha256` 全项验证 OK。
普通 RV64 为 1024x768、普通 LA64 为 1280x800，resize 均为 900x650。截图 SHA-256 按上述
顺序为：

`538cd8c5060bcb8f644d48aa6fc9c66918dc11f65f00c50270957cd10a6eebbb`、
`01ee1cc52024eed3387ec8eabbdd97ad74eec3031f164cbd11f781c6fe69d3ab`、
`1a5676a1055809f457e821fe250c8c0ab4c542c431c98674c650a830bda0e557`、
`c6c05689e2f47437bee9e8602f317c786826d57eb081e8010e9a3993c0233d56`、
`d35787c94a68de2e990c588dbe6cfe6fefb53e731804606e144c3439ab952d80`、
`4c5544dff4570b833b5f2ff6bd107c9b8e5f1a5867c9f94436ad77d0e3308aba`、
`3f7690288afae6864aa78ca8a78eaf0c899ac2f4b5fc2cf5e5156e3621d9cf87`、
`f98b59e819d817b510851da2f5e8309401d4839d7d81c1a31b924a991a16533a`、
`3056b5749344f40c4a247a853371fa61a4488676d4c570e5ba8f1916a83ee169`、
`be8448ef6425d4d0bd0ec5085bf1560b0d527cb47ee133b1178f2ca4878be647`。

十张 PPM 转为仅供观察的 ignored PNG 后逐张实际查看；未发现空白帧、异常 clipping 或文字
重叠。空载独立性能重跑 7 次，1024x768 release renderer 为 47.814–49.512 ms，median
48.142 ms，`threshold=null`，范围不含 PPM 写入。与编译并行的受干扰 180.364 ms median
保留为一次实际但不稳定的观察，不覆盖空载样本，也不设伪阈值。

current-source 最终干净快照、默认双架构构建与 canonical quick/baseline 尚待完成；在其完成前
不把旧 snapshot 当作当前证据，也不提前声称独立 review 已无 Major。

### 2026-07-19 最终证据协议整改与当前矩阵

独立 reviewer 以 `qemu-rv-launcher.fc0e28` 复现旧 gate 的假阳性：serial 已出现
`ORAYS_DESKTOP_ACTION LAUNCHER OPEN`，但其后没有 animation frame，截图仍是 boot 状态，旧
summary 却为 PASS。该原始目录保持不变并分类为 `PROTOCOL_PASS_VISUAL_FAIL`；新 validator
只读重验明确报告 stable marker 和 capture sidecar 缺失，不覆盖旧结果。

修复后的 guest 仅在 launcher `progress == 1000`、tick 已完成 render/present 且状态发生
false-to-true 转换后输出唯一 `ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE`。runner 先等待该状态；
`qmp_screendump.py` 在 settle 后、实际发送 `screendump` 前重新读取 serial，要求唯一
action < stable，并把 serial prefix byte count、SHA-256 和精确行号写入
`capture-precondition.json`。summary 重新验证 prefix、顺序和唯一性，并将 sidecar 纳入
`hashes.sha256`。缺 stable、stable 反序、prefix 后续篡改和 capture 端反序均有 fail-closed
Python 回归。一次 LA runner `qemu-la-launcher.767134` 因 CRLF 与 shell exact-line grep 不兼容
而 TIMEOUT，marker 虽真实存在也不计 PASS；runner 改用 CRLF-safe wait，Python 仍按逻辑行精确
检查。`qemu-la-launcher.d6ecf1` 是工具会话提前交还留下的不完整目录，没有 summary，不计结果。

reviewer 随后发现 resize 中 `ORAYS_DESKTOP_INPUT` 早于 `handle_input`/present，旧协议仍可能在
cursor frame 前 capture。输入 marker 现在只在同步 handler 以及可能的 display present 完成后
输出；summary 强制 `DISPLAY_CHANGED < FRAME input < center PointerMoved`，反序负例必须 FAIL。
最终 RV64 `qemu-rv-resize.5a7d8f` 的相关行是 91/93/94，LA64
`qemu-la-resize.18f76c` 是 33/35/36；两张 900x650 截图均实际看到中心 cursor。

另一次从 `user/desktop` 调用 scope checker 得到错误的 119+ path/FAIL，暴露其 Git 子命令依赖
调用者 cwd。所有查询现绑定脚本自身解析出的 repo root；新增根目录和 `user/desktop` 调用
stdout/stderr/exit 完全一致的回归，最终两处均为 105 paths / 3 bridge / 3 existing / 74 churn /
PASS。早先误用 `cargo test --all-targets`（没有 `host-tools`，也没有指定 build target）真实得到
lib 12 PASS，随后 apps target 4 PASS / 6 FAIL 并停止，其他 integration targets 未执行；失败为
host filesystem fixture 的 Unsupported/NotFound。正确入口随后 53/53 PASS，但首错仍保留。
该错误调用生成的 511-file `user/desktop/target` 与 Python cache 没有删除或
覆盖，分别 recoverably 移至 `/tmp/orays-desktop-local-target-20260719-0041`、
`/tmp/orays-desktop-pycache-20260719-protocol` 和
`/tmp/orays-desktop-scripts-pycache-20260719-protocol`；当前工作树无对应污染。

最终 current-source 定向结果：host Rust 53/53、Python discovery 35/35、host/RV64/LA64
clippy `-D warnings`、asset checker（0 registered external/5 generated）、五场景 golden、rustfmt、
`git diff --check`、headless host check 与 scope 均完成；headless host 为 PASS_WITH_WARNINGS，缺少
可选 `socat`、ImageMagick `convert` 和 Python `tomllib`，两种 QEMU 均有 headless backend。
release 产物为：

- RV64 ELF 1,664,552 bytes / `ca56722aff051b036a8e7e0e6335c66b58b7d91183fe98bcf547d374d4fb2a35`；
  BIN 659,648 bytes / `6ceb67b256ebb72484398555ae35ee6183c9912b4bab40c9c4cf1556acf29b28`；
- LA64 ELF 1,229,656 bytes / `9b032b9d90c985cbaf804f171e3d8056c2b81daef29726332cef97dfc2b59240`；
  BIN 856,256 bytes / `bee26d1e8dfee89c846c01418ce85f1817dbb7bbdd24bbc47603ad45a938d1f4`。

最终 current-source 十场景矩阵如下；顺序均为 boot/launcher/overlap/applications/resize：

- RV64：`qemu-rv-boot.a3877b`、`qemu-rv-launcher.cd1390`、
  `qemu-rv-overlap.7c5ba5`、`qemu-rv-applications.5249f1`、`qemu-rv-resize.5a7d8f`；
- LA64：`qemu-la-boot.ccbd77`、`qemu-la-launcher.af7d8c`、
  `qemu-la-overlap.bc8062`、`qemu-la-applications.e9043a`、`qemu-la-resize.18f76c`。

10/10 summary 均为 PASS、`failures=[]`、QEMU exit 0；10 份 manifest 全项 SHA-256 OK。截图
SHA-256 依次为 `f0400b5a52d0cf3fd3057fe9e48dcf787f819430c5583510877b1294075e41bc`、
`075e7489f49882dd3557af6166d09d38ce371fc09b0fa211c633e8c7568f033f`、
`1a5676a1055809f457e821fe250c8c0ab4c542c431c98674c650a830bda0e557`、
`c6c05689e2f47437bee9e8602f317c786826d57eb081e8010e9a3993c0233d56`、
`0e7c558e714b9c1e9171cdcc4f8a3f2c1087b84f6893fcb858e18dfd1a07744a`、
`9661a86d385d286a19d617d93fcce17c912f6e38d03b6b65b3ac062e4d0174b4`、
`837be766b9fe3853691f38e968a80994705d69cf4c476c83da3b62c91ee38d11`、
`6f69810e35bf49e042c875a7606ccb0c0b2d68fe7a4b5765e979ae76f5f79587`、
`3056b5749344f40c4a247a853371fa61a4488676d4c570e5ba8f1916a83ee169`、
`327bd579044ac63d231dfb6989fd6e4a37f120cad582203c2e8a357a9417aa8d`。
十张 PPM 的 ignored PNG 观察副本已逐张实际查看；没有空白帧、异常 clipping 或文字重叠，
launcher 完全展开且六图标/文字可见，resize cursor 位于中心。所有更早矩阵均只作历史证据。

最终实现与测试文件集由两个独立干净快照复核，均得到 tree
`b943e2aecfec0ade8b757790880edf3e3305f3cf`。reviewer 快照 commit 为
`3222ac5ef19e343d98cc609bbdffe44fbceb0040`，parent 为基线 `c776ceff`；其 parent..HEAD 的
105 个变更路径与 live 工作树变更/未跟踪路径集合一致，随后对 105 个文件逐一 SHA-256 比对，
清单完全一致且整体 SHA-256 为
`72c2a5f80e32d43067fc2a118f2149ca768b5a92094bc655e9028084c495bd2e`。快照 gate 前后均 clean。

同一 tree 的 canonical quick 产生两份均保留的真实结果：

- 主快照 `b527dfb8564855d9d0b615a56e0594b00326c8e4` 为 45/45 PASS、退出 0、用时
  440.988 秒；其中 `unit.suite_runner` 为 296.708/300 秒；summary SHA-256
  `a48477dab9f7a86c8b96f4c1259606213a8917e2c68423ebc012265b384d1237`；
- 独立 reviewer 快照为 44 PASS / 1 TIMEOUT、退出 1、用时 447.773 秒；同项在
  302.012 秒被终止。summary 位于
  `test/output/desktop/final-gates/final-review-3222ac5-quick/summary.json`，SHA-256
  `717ac8956e6dfee3edbda81a49067c9106299db61fe5ce28f52127e8629fb082`。

两份 quick 均是起止 commit 不变、dirty=false、provenance stable。随后 baseline 中同一
`unit.suite_runner` 135/135 在 202.928 秒 PASS，支持资源竞争/近阈值 flake 的诊断，但不覆盖
reviewer 首轮 TIMEOUT，也不把 quick 描述成一致通过。

reviewer 精确快照的 canonical baseline 用时 1,292.431 秒、退出 2；57/57 项均有终态、
56 项实际启动，结果为 53 PASS / 3 FAIL / 1 INFRA_ERROR / 0 TIMEOUT / 0 CRASH。summary 位于
`test/output/desktop/final-gates/final-review-3222ac5-baseline/summary.json`，SHA-256
`f06fe8bdfc0fc944cc4e39994c57b15a01dbfa874fd2d8d9187975deb3265936`；runner 起止均为
`3222ac5`、dirty=false、provenance stable。非通过项原样保留：

- `evidence.riscv64` FAIL：要求 QEMU 9.2.4，主机实际 6.2.0；两个 RV64 build 子项 PASS，
  runtime smoke 因 missing prerequisite 未启动；
- `evidence.loongarch64` FAIL：两个 LA64 build 子项 PASS，guest 精确输出
  `USER_FAIL tee_device_mode` 与 `HARNESS_FAIL ... guest_nonzero_exit`；
- `evidence.aggregate` 继承双架构非通过；
- `baseline.clippy_loongarch64` 在命令执行前因 clang 14 不支持
  `loongarch64-unknown-none` target，严格记为 INFRA_ERROR。

baseline 中 `make kernel-rv`、`make kernel-la`、`make all` 均 PASS。最终默认非桌面产物为：

- RV64 2,024,576 bytes / `3a3a3f3d1a73fd58ac575004c0c92525f29f77c0ba417008f3eb74be9229a81f`；
- LA64 3,078,616 bytes / `4e9cb78e1e054c277da1cf4c66a997c5deae2d5fb86a2cbee7893d773034e0dc`。

上述 gate 完成后只更新本计划、开发日志和 Goal 状态；没有改动实现、测试、manifest 或构建
入口。文档收尾单独运行 diff/scope 检查，不把文档修改伪装成需要重新声明产品 runtime PASS。

独立 Codex reviewer 最终复读 live diff、当前精确快照证据、计划、开发日志和停止条件后，
结论为 Blocker 0 / Major 0；文档收尾的 `git diff --check`、根目录/`user/desktop` 两处 scope、
trailing whitespace 与污染检查均通过。因此本任务停止为
`READY_FOR_HUMAN_REVIEW_DRAFT`，明确不是 `MERGE_READY`。quick 首轮真实 TIMEOUT、baseline
非 PASS、official/full 未通过及人工 reviewer 待指定继续作为 Draft 已知限制。

## AI 使用

- 工具：OpenAI Codex
- 模型：工作树配置请求 `gpt-5.6-sol`；服务端实际版本未独立验证
- 推理强度：xhigh
- 用途：架构分析、实现、测试、无显示验证和文档
- 显著影响范围：`user/desktop` 渲染、桌面本地 VirtIO GPU/input/DMA HAL、窗口管理/合成、
  Shell/主题/动画、六应用/控件/平台文件系统与构建代码，`axdriver` 默认关闭的
  device bus-discovery hook，desktop host/Python/QMP/VNC tests、
  ADR 0002/0003、计划与本日志
- 人工取舍：拒绝无法离线验证的 LVGL/embedded-graphics；拒绝扩大 vendor/POSIX 写权限；
  host golden 不作为 QEMU PASS
- 人工负责人：待填写
- 独立 AI 复核：第二 Codex reviewer 逐轮检查最终 diff、双架构 guest 图像/协议、PCI/MMIO
  唯一所有权和精确快照 gate；人工复核人仍待 Draft PR 指定，不伪造人工签名

## 外部来源

所有依赖、代码参考、字体、图标、壁纸和图片来源必须记录在：

- `docs/references/desktop-dependencies.md`
- `docs/references/desktop-headless-development.md`

## 验证表

| Checkpoint | 命令 | 架构 | 退出码 | 结果 | 证据 |
|---|---|---|---:|---|---|
| 0 | `scripts/desktop/check-headless-host.sh` | host | 0 | PASS_WITH_WARNINGS | 本日志；命令真实输出 |
| 0 | `python3 scripts/desktop/check-scope.py` | 通用 | 0 | PASS | 11 paths / 0 bridge |
| 0 | `python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --list` | 通用 | 0 | 59 cases listed | 只读计划，不是 suite PASS |
| 0 | 直接最小权限 `make kernel-rv` | RV64 | 2 | BLOCKED | vendor/生成头文件写入被只读 sandbox 拒绝 |
| 0 | 直接最小权限 `make kernel-la` | LA64 | 2 | BLOCKED | 同上 |
| 0 | 精确 HEAD 隔离快照 `make kernel-rv` | RV64 | 0 | PASS | `build/desktop/baseline/snapshot-build/kernel-rv` |
| 0 | 精确 HEAD 隔离快照 `make kernel-la` | LA64 | 0 | PASS | `build/desktop/baseline/snapshot-build/kernel-la` |
| 1 | pure Rust rlib build | RV64 | 0 | PASS | `build/desktop/spikes/pure-rust/*-rv64.rlib` |
| 1 | pure Rust rlib build | LA64 | 0 | PASS | `build/desktop/spikes/pure-rust/*-la64.rlib` |
| 1 | pure Rust host tests | host | 0 | 2/2 PASS | stride/clipping/buffer validation |
| 1 | freestanding C toolchain probe | RV64 | 0 | PASS | 944-byte object；不是 LVGL build |
| 1 | freestanding C toolchain probe | LA64 | 1 | FAIL | clang 14 unknown target triple |
| 1 | embedded-graphics offline resolve | RV64/LA64 | 101 | FAIL | package absent from fixed offline source |
| 2 | `scripts/desktop/build.sh host-test` | host | 0 | 8/8 PASS | `build/desktop/target/host` |
| 2 | `scripts/desktop/build.sh golden` + `cmp` | host | 0 | PASS | deterministic PPM + fixed SHA-256 |
| 2 | `scripts/desktop/build.sh rv` | RV64 | 0 | PASS | release ELF/BIN + SHA-256 |
| 2 | `scripts/desktop/build.sh la` | LA64 | 0 | PASS | release ELF/BIN + SHA-256 |
| 2 | `git diff --check` | 通用 | 0 | PASS | 无 whitespace error |
| 2 | `python3 scripts/desktop/check-scope.py` | 通用 | 0 | PASS | 35 paths / 1 bridge / 20 churn |
| 3 | `scripts/desktop/build.sh host-test` | host | 0 | 13/13 PASS | graphics 8；input 5 |
| 3 | `python3 -m unittest test/desktop/test_input_sequences.py` | host | 0 | 3/3 PASS | fixture、fail-closed、fake QMP transcript |
| 3 | `scripts/desktop/inject-input.py --validate-only .../basic.json` | host | 0 | PASS | 3 QMP steps / 3 input steps |
| 3 | `scripts/desktop/build.sh rv` | RV64 | 0 | PASS | ELF `b1c0a59b...`；BIN `c5269d97...` |
| 3 | `scripts/desktop/build.sh la` | LA64 | 0 | PASS | ELF `b9f579a7...`；BIN `94458acf...` |
| 3 | 首次 QMP relative input sequence | RV64 guest | 1 | FAIL | tablet 无 `rel` handler；错误与 partial transcript 保留 |
| 3 | QMP absolute input sequence | RV64 guest | 0 | PASS | guest 记录 key `a`、pointer `(511,191)`、left click；transcript `21eb64aa...` |
| 3 | QMP `screendump` + 人工查看 | RV64 guest | 0 | PASS | 1024x768 PPM `dd897a3d...`；PNG `5f369de7...` |
| 3 | `python3 scripts/desktop/check-scope.py` | 通用 | 0 | PASS | 45 paths / 6 bridge / 5 existing / 61 churn |
| 3 | `git add <desktop exact allowlist>` | Git metadata | 128 | BLOCKED | shared worktree index 为只读文件系统，无法创建 `index.lock` |
| 4 | 首次 `scripts/desktop/build.sh host-test` | host | 101 | FAIL | cursor coordinate/size integer mismatch；随后修复 |
| 4 | 首次 damage/full pixel equality | host | 101 | FAIL | `(105,372)` 阴影 damage 少 1 px；随后修复 |
| 4 | `scripts/desktop/build.sh host-test` | host | 0 | 20/20 PASS | graphics 8；input 5；window/runtime 7 |
| 4 | 首次 host clippy `-D warnings` | host | 101 | FAIL | 4 条 lint，随后另有旧 writer 1 条；均逐项修复 |
| 4 | host clippy `--all-targets --features host-tools -- -D warnings` | host | 0 | PASS | `build/desktop/target/host-clippy` |
| 4 | `scripts/desktop/build.sh rv` | RV64 | 0 | PASS | ELF `650f9b1d...`；BIN `755c8bce...` |
| 4 | `scripts/desktop/build.sh la` | LA64 | 0 | PASS | ELF `771dbb78...`；BIN `957d4834...` |
| 4 | `git diff --check` + desktop rustfmt check | 通用 | 0 | PASS | 无 whitespace/format drift |
| 4 | `python3 scripts/desktop/check-scope.py` | 通用 | 0 | PASS | 50 paths / 6 bridge / 5 existing / 61 churn |
| 5 | `scripts/desktop/build.sh host-test` | host | 0 | 28/28 PASS | graphics 9；input 5；shell 5；window/runtime 9 |
| 5 | host clippy `--lib --tests --features host-tools -- -D warnings` | host | 0 | PASS | desktop host target |
| 5 | `python3 -m unittest discover -s test/desktop -p 'test_*.py'` | host | 0 | 6/6 PASS | input、renderer、window wrapper |
| 5 | `scripts/desktop/build.sh scene <name>` | host | 0 | PASS | boot/launcher/overlap/light/power PPM；四个固定 golden hash |
| 5 | `scripts/desktop/build.sh rv` | RV64 | 0 | PASS | ELF `cec70be5...`；BIN `bafcb7aa...` |
| 5 | `scripts/desktop/build.sh la` | LA64 | 0 | PASS | ELF `02f15d8d...`；BIN `9f348e60...` |
| 5 | `git diff --check` + desktop rustfmt check | 通用 | 0 | PASS | 无 whitespace/format drift |
| 5 | `python3 scripts/desktop/check-scope.py` | 通用 | 0 | PASS | 65 paths / 6 bridge / 5 existing / 61 churn |
| 6 | 首次加入 app tests 的 `scripts/desktop/build.sh host-test` | host | 101 | FAIL | 5/8 app tests 得到 fail-closed Unsupported backend；随后修复入口 |
| 6 | `scripts/desktop/build.sh host-test` | host | 0 | 37/37 PASS | apps 9；graphics 9；input 5；shell 5；window 9 |
| 6 | `python3 -m unittest discover -s test/desktop -p 'test_*.py'` | host | 0 | 7/7 PASS | 含真实 application scene 双次 golden |
| 6 | host clippy `--lib --tests --features host-tools -- -D warnings` | host | 0 | PASS | 第一次 lint failure 修复后复验 |
| 6 | `scripts/desktop/build.sh scene applications`（两次） | host | 0 | PASS | real filesystem scene `8f0f9d94...`；PNG 已查看 |
| 6 | `scripts/desktop/build.sh rv` | RV64 | 0 | PASS | ELF `bb1abe54...`；BIN `dd3a7c56...` |
| 6 | `scripts/desktop/build.sh la` | LA64 | 0 | PASS | ELF `a7566be1...`；BIN `182786b5...` |
| 6 | `git diff --check` + desktop rustfmt check | 通用 | 0 | PASS | 无 whitespace/format drift |
| 6 | `python3 scripts/desktop/check-scope.py` | 通用 | 0 | PASS | 83 paths / 6 bridge / 5 existing / 61 churn |
| 7 | `run-headless-qemu.sh --arch rv --scenario applications`（首次 real tick） | RV64 guest | 1 | FAIL | `qemu-rv-applications.3AdahW`；缺 theme marker，未接受 |
| 7 | `run-headless-qemu.sh --arch rv --scenario boot` | RV64 guest | 0 | PASS | `qemu-rv-boot.ewCKpn`；完整 sequence/transcripts/marker/frame/hash |
| 7 | 同一 runner：launcher / overlap / applications | RV64 guest | 0 | 3/3 PASS | `KAog2y` / `HvnHin` / `OtbvDE` |
| 7 | 修复前 `run-headless-qemu.sh --arch la --scenario boot` | LA64 guest | 0 | PROTOCOL_PASS_VISUAL_FAIL | `PXTxzM`；monitor 文字重叠，未接受 |
| 7 | `run-headless-qemu.sh --arch la --scenario boot` | LA64 guest | 0 | PASS | `qemu-la-boot.RLGS51`；最终视觉已检查 |
| 7 | `scripts/desktop/build.sh host-test` | host | 0 | 38/38 PASS | 含 compact monitor layout 回归 |
| 7 | Python discovery + host clippy + rustfmt + diff/scope | host/通用 | 0 | PASS | Python 7/7；89+ paths，最终 CP7 文档后复验 |
| 8 | `scripts/desktop/build.sh host-test` | host | 0 | 42/42 PASS | apps 9；graphics 9；input 6；shell 5；window 11；lib 2 |
| 8 | `python3 -B -m unittest discover -s test/desktop -p 'test_*.py'` | host | 0 | 20/20 PASS | 含 summary CLI/负向篡改、resize evidence 与双分辨率 tablet mapping |
| 8 | host clippy `--locked --lib --tests --features host-tools -- -D warnings` | host | 0 | PASS | 独立 desktop lockfile |
| 8 | 首次 RV64/LA64 cross-clippy | 双架构 | 101 | FAIL | 相对 `AX_CONFIG_PATH` 被 crate-relative 解析；错误保留 |
| 8 | RV64 parallel cross-clippy retry | RV64 | 143 | INTERRUPTED | 无 Rust 诊断；单独重跑，不计 PASS |
| 8 | RV64/LA64 cross-clippy `-D warnings` | 双架构 | 0 | 2/2 PASS | 绝对 config；MMIO/PCI cfg 均进入 lint |
| 8 | 中间 `axdisplay` feature 开/关 check | 中间方案 | 0 | HISTORICAL PASS | 后续删除全部 `axdisplay` diff，不作为最终架构证据 |
| 8 | asset check + five golden comparison | host | 0 | PASS | 0 registered external files；5 generated scenes |
| 8 | `scripts/desktop/measure-performance.py` | host | 0 | RECORDED | 7 samples；median 48.549 ms；无伪阈值 |
| 8 | 修复后首次 QEMU boot/launcher | RV64 guest | 1 | ERROR | `Ykb1TB` / `TGmZUm`；summary `NameError`，不计 PASS |
| 8 | RV64 direct GPU 晚扫描 launcher | RV64 guest | 1 | TIMEOUT | `rT4ypp` / `nJDfzO`；MMIO transport ownership 冲突，不计 PASS |
| 8 | 首次 QMP `qom-set xres/yres` resize | RV64 guest | 1 | FAIL | `qqj3OL`；realized device 拒绝属性变更，不计 PASS |
| 8 | RV64 headless boot/launcher/overlap/applications/resize | RV64 guest | 0 | 5/5 PASS | `9mYVSe` / `oomKC5` / `wi0gXl` / `UTgANh` / `ErRQSB` |
| 8 | LA64 headless boot/launcher/overlap/applications/resize | LA64 guest | 0 | 5/5 PASS | `rts9w2` / `6AtklF` / `Jysffs` / `mNI9es` / `2lYk5q` |
| 8 | 较早精确快照默认双架构 build | RV64/LA64 default | 0 | HISTORICAL PASS | 早于最终 direct GPU/DMA safety 整改 |
| 8 | 较早精确快照 canonical quick | 通用 | 0 | 45/45 HISTORICAL PASS | `final-gates/quick-24dab13-retry1/summary.json` |
| 8 | 较早精确快照 canonical baseline | 通用 | 1 | 51 PASS / 3 FAIL / 3 TIMEOUT | `final-gates/baseline-24dab13-corrected/summary.json`；首次非 PASS 保留 |
| 8 | `python3 scripts/desktop/check-scope.py` | 通用 | 0 | PASS | 102 paths / 3 bridge / 3 existing / 56 churn |
| 8 | 当前源码 synthetic 快照 canonical quick 首试 | 通用 | 2 | INFRA_ERROR | 缺 `origin/main`，case 启动前失败，不计 PASS |
| 8 | 当前源码快照默认 RV64 build 首试 | RV64 default | 143 | INTERRUPTED | SIGTERM；保留首轮结果 |
| 8 | 当前源码快照默认 RV64/LA64 build | RV64/LA64 default | 0 | 2/2 PASS | `c2785bd3…` / `85f7a0eb…` |
| 8 | 终审整改前快照 canonical quick | 通用 | 0 | 45/45 HISTORICAL PASS | `final-gates/snapshot-0924ee30-quick/summary.json`；当前已 stale |
| 8 | 终审整改前快照 canonical baseline | 通用 | 2 | 53 PASS / 3 FAIL / 1 INFRA_ERROR | `final-gates/snapshot-0924ee30-baseline/summary.json`；provenance stable，当前已 stale |
| 8 | 终审整改后 `scripts/desktop/build.sh host-test` | host | 0 | 53/53 PASS | 含有界读取、capacity reservation、字体/图标 atlas 回归 |
| 8 | 终审整改后 Python discovery | host | 0 | 28/28 PASS | 含 clean output root、traversal/symlink 负向测试 |
| 8 | 终审整改后 host/RV64/LA64 clippy `-D warnings` | 三目标 | 0 | 3/3 PASS | opaque PCI hook、MMIO/PCI cfg 均进入 lint |
| 8 | 终审整改后 asset + five golden comparison | host | 0 | PASS | asset 0 external/5 generated；golden 5/5 MATCH |
| 8 | 终审整改后 RV64/LA64 release build | 双架构 | 0 | 2/2 PASS | RV `2482a3fb…`/`2972760d…`；LA `b7e17c3a…`/`eeccc0a1…` |
| 8 | 终审整改后 RV64 五场景 | RV64 guest | 0 | 5/5 PASS | `49b4a6` / `2abaef` / `7460a2` / `6aa0f5` / `f80d09` |
| 8 | 终审整改后 LA64 五场景 | LA64 guest | 0 | 5/5 PASS | `35168e` / `13447e` / `aca5e1` / `59ba55` / `554bfd` |
| 8 | 十场景 hash manifest + 图像复核 | 双架构 | 0 | PASS | manifests 全项 OK；十张观察 PNG 已逐张查看 |
| 8 | 独立空载 `measure-performance.py` | host | 0 | RECORDED | 7 samples；median 48.142 ms；threshold=null |
| 8 | 终审整改后 `check-scope.py` | 通用 | 0 | PASS | 104 paths / 3 bridge / 3 existing / 74 churn |
| 8 | 错误 `cargo test --all-targets`（无 host-tools/target-dir） | host | 101 | lib 12 PASS；apps 4 PASS / 6 FAIL | 后续 targets 未执行；不覆盖，正确入口另跑 |
| 8 | launcher 旧 gate 反例 | RV64 guest | 0 | PROTOCOL_PASS_VISUAL_FAIL | `fc0e28`；action 后无 frame，截图为 boot 态 |
| 8 | LA stable wait 首次 CRLF 不兼容 | LA64 guest | 1 | TIMEOUT | `767134`；真实 marker 存在，runner wait 未识别 |
| 8 | 最终 `scripts/desktop/build.sh host-test` | host | 0 | 53/53 PASS | 指定 host-tools 与 `build/desktop` target |
| 8 | 最终 Python discovery | host | 0 | 35/35 PASS | 含 stable/capture、resize 顺序与 scope cwd 负例 |
| 8 | 最终 host/RV64/LA64 clippy `-D warnings` | 三目标 | 0 | 3/3 PASS | host-tools、MMIO、PCI cfg |
| 8 | 最终 asset + five golden | host | 0 | PASS | 0 external/5 generated；5/5 MATCH |
| 8 | 最终 desktop RV64/LA64 release build | 双架构 | 0 | 2/2 PASS | RV `ca56722a…/6ceb67b2…`；LA `9b032b9d…/bee26d1e…` |
| 8 | 最终 RV64 五场景 | RV64 guest | 0 | 5/5 PASS | `a3877b/cd1390/7c5ba5/5249f1/5a7d8f` |
| 8 | 最终 LA64 五场景 | LA64 guest | 0 | 5/5 PASS | `ccbd77/af7d8c/bc8062/e9043a/18f76c` |
| 8 | 最终 manifests + 十张图复核 | 双架构 | 0 | PASS | hashes 全项 OK；十图逐张查看 |
| 8 | 最终 scope 根目录/子目录 | 通用 | 0 | 2/2 PASS | 两处 105/3/3/74；输出一致 |
| 8 | 精确 tree canonical quick（主快照） | 通用 | 0 | 45/45 PASS | summary `a48477da…`；suite_runner 296.708/300s |
| 8 | 精确 tree canonical quick（独立 reviewer 首轮） | 通用 | 1 | 44 PASS / 1 TIMEOUT | `final-review-3222ac5-quick/summary.json`；`717ac895…`；不由成功轮覆盖 |
| 8 | 精确 reviewer 快照 canonical baseline | 通用 | 2 | 53 PASS / 3 FAIL / 1 INFRA_ERROR | `final-review-3222ac5-baseline/summary.json`；`f06fe8bd…`；provenance stable |
| 8 | baseline 内 `make kernel-rv` / `kernel-la` / `all` | 双架构默认路径 | 0 | 3/3 PASS | RV `3a3a3f3d…`；LA `4e9cb78e…` |

## 已知限制

- 当前工作树含本 PR 未提交文件；canonical suite 在逐路径、逐文件匹配 live 实现与测试文件集的
  精确干净快照执行。随后只更新证据文档和 Goal 状态，不改变已验证代码与测试。
- 当前精确 tree 的 quick 一轮 45/45 PASS，另一独立首轮为 44 PASS / 1 TIMEOUT；
  `unit.suite_runner` 接近 300 秒阈值，按 flake/资源竞争风险记录，不声称 quick 一致通过。
- 当前精确 tree 的 baseline 为 53 PASS / 3 FAIL / 1 INFRA_ERROR，不是完整门禁 PASS。RV64
  固定 QEMU 9.2.4 缺失、LA64 既有 `tee_device_mode`/guest nonzero、主机 clang 缺
  LoongArch64 bare-metal target，以及所有更早首次 timeout 均继续保留。
- host 缺少可选 `socat`、ImageMagick `convert`，Python 3.10 不提供 `tomllib`。
- 稳定化基线已有非桌面缺陷、失败和未完成 official/full 证据；本 PR 不隐藏或修复它们。
- 当前底层没有可调用的 reboot API；restart 菜单项显式报告 unsupported，不计为成功。

## 回滚

桌面默认关闭；删除桌面独立构建入口和 feature-gated bridge 即可回滚，不应影响默认内核路径。

## 2026-07-19 人工初审整改

- 人工初审报告：`docs/human_reviews/OrayS-Desktop-人工审查初审报告-2026-07-19.md`；结论为
  `REQUEST CHANGES`。该文件是审查输入，保持原样，不以修改报告文字代替实际修复。
- 整改接管基线：分支 `feature/orays-desktop-environment`，HEAD
  `809155269c8f77e46ba02a31e6ae8715a680cf92`，上游
  `origin/feature/orays-desktop-environment`。接管时唯一未跟踪内容为用户提供的
  `docs/human_reviews/`；没有覆盖、移动或加入该内容。
- 修改前定向基线 `scripts/desktop/build.sh host-test` 退出 0：53/53 PASS；这只证明既有测试
  通过，报告指出的缺口尚未被既有测试覆盖。
- 修改前 Python discovery 启动并输出 35 个成功点；完整退出状态在本轮复验表中以独立命令
  重新记录，不用该并发采集的截断输出宣称 PASS。
- 本轮范围：B-01 DMA 合同、M-01 焦点 damage、M-02 runtime CI、M-03 原始证据包、N-01
  WindowId 耗尽、N-02 P6 CRLF、N-03 Terminal 能力边界、N-04 输入溢出 release 保护；B-02
  canonical/official 状态只按实际门禁结果关闭或继续标记阻塞。
- AI 使用：OpenAI Codex 用于报告映射、实现、测试、CI/证据脚本与文档；所有生成修改仍需
  定向测试、双架构构建/runtime、scope/diff 和第二轮人工复核。

### Checkpoint 9：初审问题整改完成，等待第二轮人工审查

整改没有修改人工初审报告，也没有通过弱化测试、parser、QEMU marker、结果分类或扩大
blacklist 关闭问题。各项处置如下：

| 初审项 | 处置与行为证据 |
| --- | --- |
| B-01 | DMA 零页、字节数溢出和 allocator failure 均执行明确 panic 策略；只返回真实、页对齐并完整清零的分配；三条 host 合约测试 PASS。 |
| M-01 | 新增统一 `set_focused`，旧/新焦点 decoration 都加入 damage；create、close、minimize、focus、Desktop click、Alt-Tab/modal 路径统一使用；三类 incremental-vs-full 像素测试 PASS。 |
| M-02 | `.github/workflows/desktop.yml` 新增固定 QEMU 9.2.4 的 self-hosted 双架构 required boot job，并仅上传两架构 review package；branch protection 是否把该 job 设为 required 仍需 GitHub 管理员确认。 |
| M-03 | runner 记录源码 commit/dirty、QEMU/工具链版本和精确生成命令；每个场景打包 serial、QMP input/capture、输入序列、原始 PPM、summary、内外层哈希、geometry、capture precondition，resize 另含 VNC resize 证据；缺失或篡改测试 fail-closed。 |
| N-01 | `WindowId` 在 `u32::MAX` 只签发一次，随后永久返回 `IdExhausted`，不回绕复用。 |
| N-02 | P6 只把 CRLF 作为一个 header separator 消费，不继续吞掉可能是空白值的首像素；首像素 `0x20,0x0a,0xff` 测试 PASS。 |
| N-03 | Terminal 模块、启动界面、README 和测试均明确其为进程内受限内置命令解释器，不提供 POSIX process/shell、pipe、redirect、environment 或 job control。 |
| N-04 | 队列优先保留 key/button release：普通事件不驱逐 release，新 release 优先驱逐最旧普通事件；保留饱和 dropped 计数并在串口输出 total/delta 诊断。 |

精确干净快照位于忽略的
`build/desktop/review-round2-source.3FW9N9`，验证提交为
`cb42268a9d7f47fbd10f8b0a5af80026712829ca`，tree 为
`91806ae2044a934829cdf8384cc0c9986a7aef3d`；验证开始时提交中的 25 个预期路径与 live
工作树逐文件一致，未包含用户提供且保持原样的人工报告。门禁结束后只更新本日志、执行计划、
README 证据说明和忽略的 Goal 状态，产品实现、测试、CI 与证据脚本未再改变。10 组场景证据位于该快照的
`test/output/desktop/r2/{rb,rl,ro,ra,rr,lb,ll,lo,lapps,lr}/review-package/`。所有包都重新
校验外层哈希成功；normal 场景的原始 summary 哈希覆盖 8 个文件，resize 覆盖 9 个文件。
运行元数据均记录 `source_dirty=false`、相同 commit；RV 使用 QEMU 6.2.0，LA 使用 QEMU
9.2.4，所以这些 RV Desktop 场景是实际运行证据，但不能代替固定 9.2.4 的 canonical
required smoke。所有 10 个原始 PPM 均转换到 `/tmp` 后人工目视检查，没有黑屏、空白、
明显裁切、窗口异常重叠或 resize 后主要控件消失。

最终验证记录：

| 命令/证据 | 结果 | 说明 |
| --- | --- | --- |
| `python3 -I -S -B -X pycache_prefix=/dev/null -m unittest discover -s test/desktop -p 'test_*.py' -v` | 42/42 PASS，exit 0 | 包含 evidence package、runtime CI、capture binding、Unicode scope 测试。 |
| `scripts/desktop/build.sh host-test` | 63/63 PASS，exit 0 | lib 16、apps 11、graphics 9、input 8、shell 5、window manager 14。 |
| Desktop host/RV64/LA64 clippy | PASS | 首轮 host clippy 曾因 test module 位置失败；移动 test module 后三目标复验通过，首次失败保留。 |
| Desktop RV64/LA64 release build | PASS | live ELF/BIN：RV `98ec2534…` / `7b3d1550…`；LA `c41161ad…` / `2753b280…`。 |
| 10 个精确快照 headless 场景 | 10/10 PASS | 两架构各 boot、launcher、overlap、applications、resize；完整 review package 有效。 |
| `scripts/desktop/build.sh golden-check` | 5/5 PASS，exit 0 | applications、boot、launcher、light、power 与 golden 一致。 |
| asset、fmt、scope、`git diff --check` | 全部 PASS | asset registered=0/generated=5；scope 110/3/3/74。 |
| canonical quick | 45/45 PASS，exit 0 | clean snapshot；summary SHA-256 `12a17e1cd620e2a85e51d1375d29ea3489d37484d6ff5aabb617fc6680b863e1`。 |
| canonical `full --arch all` | `INFRA_ERROR`，exit 2 | planned 59、executed 56、completed 59；52 PASS / 4 FAIL / 3 INFRA_ERROR / 0 TIMEOUT / 0 CRASH；summary SHA-256 `c9d0db5278c439a784221f5dc4d693be91849cc92291dd428b52cb12f2b9c6b3`。 |

`full` 的非 PASS 逐项保留：`evidence.riscv64` 因要求 QEMU 9.2.4、实际 6.2.0 而 blocked；
`evidence.loongarch64` 因 `PR3_SMOKE_V1 USER_FAIL tee_device_mode` / guest nonzero 而 error；
`evidence.aggregate` 派生 FAIL；`baseline.cargo_format` 因验证 clone 位于 live repo 的
`build/desktop/` 下，vendor crate 被误绑定到父工作区而 FAIL，live 根目录相同 fmt 命令另行
PASS，但不覆盖 canonical FAIL；`baseline.clippy_loongarch64` 因 clang 14 不认识
`loongarch64-unknown-none` 而 INFRA_ERROR；`official.riscv64` 和 `official.loongarch64` 因
`/root/sdcard-rv.img`、`/root/sdcard-la.img` 不可用而 INFRA_ERROR。完整 summary 记录 runner
起止 commit 相同、dirty=false、status 为空、provenance stable，运行前后快照仍 clean。

过程中的首次失败同样保留：初次 scope 因 Git 对 Unicode 路径做 octal quoting 而失败，修复为
`core.quotepath=false` 后 PASS；一次包哈希命令从错误工作目录执行导致 file-not-found，切换到
包目录后 2/2 校验通过；首个长输出目录触发 Unix QMP socket 108-byte 限制，短路径重跑通过。
这些是明确的工具/调用路径问题，不被写成产品 PASS，也没有删除原始失败目录。

本 checkpoint 恢复的是 `READY_FOR_HUMAN_REVIEW_DRAFT`：代码与证据已可交付第二轮人工
审查，但 B-02 仍未关闭，PR 不可标记 Ready、不可合并。第二轮应重点复核 unsafe DMA 合同、
focus damage 像素等价测试、CI job/branch protection、10 个 raw review package 的哈希与来源；
并由维护者决定 fixed-QEMU、LoongArch clang、`tee_device_mode` 和官方镜像的关闭方式。

## 2026-07-19 第二轮人工审查整改接管

- 审查输入：`docs/human_reviews/OrayS-Desktop-第二轮人工审查复核报告-2026-07-19.md`；保持
  原文不变。结论为“部分通过；按备注继续修改”，所以撤销 Checkpoint 9 的 Draft 候选状态。
- 接管 HEAD：`809155269c8f77e46ba02a31e6ae8715a680cf92`；分支
  `feature/orays-desktop-environment`，上游 `origin/feature/orays-desktop-environment`。
- 接管时工作树已有 21 个 tracked modified 路径，以及 5 个 untracked 状态条目；其中
  `docs/human_reviews/` 是用户审查输入。全部原样保留，没有 reset、覆盖或顺手整理。
- 修改前 `scripts/desktop/check-headless-host.sh` 退出 0 / `PASS_WITH_WARNINGS`：缺少可选
  `socat`、ImageMagick `convert` 和 Python `tomllib`；两套 QEMU 均报告可用 headless backend。
- 修改前首次 `check-scope.py` 退出 1：唯一 finding 是第二轮人工报告未登记 allowlist。该失败
  原样记录；本 checkpoint 只登记这一精确审查路径，不扩大通配范围。
- 本轮本地整改范围：S-01 self-hosted 信任边界、M-04 可迁移 package 验证、M-05 失败证据与
  双架构独立执行、P-01 前后 provenance、T-01 DMA 清零直接测试，以及可安全实现的 N-04 加固。
- B-02、workflow push/实跑、required check、固定 RV QEMU 9.2.4 和官方镜像仍需真实外部证据；
  本轮不会伪造、替代或提前勾选这些条件。
- AI 使用：OpenAI Codex 用于报告映射、实现、测试、CI/证据脚本和文档；修改需由定向测试、
  双架构验证、scope/diff 和第三轮独立人工审查共同复核。

### Checkpoint 10：第二轮审查本地整改

本 checkpoint 未修改两份人工审查报告。S-01、M-04、M-05、P-01、T-01 与 N-04 可在桌面
隔离范围内完成的加固均已落地；B-02 和第三轮条件中的 push、GitHub Actions、required checks、
固定 RV QEMU、官方镜像及 PR base 仍只按真实外部状态记录。

实现与测试映射：

| 审查项 | 当前实现与直接证据 |
| --- | --- |
| S-01 / M-05 | `desktop-runtime` 不接受 `pull_request`，仅受控 dispatch/受信任 push；RV/LA 为 `fail-fast: false` matrix。每个架构在 `always()` 上传自身 filtered package。静态 workflow 回归 4/4 PASS。 |
| M-04 | package schema 2 保存原始 run/frame 绑定与包内相对 `frame.ppm`；专用 validator 验证精确文件集、双层哈希、QMP 命令、截图、summary 和 provenance。迁移、路径逃逸、迁移后篡改均有负向回归。 |
| M-05 | runner 的 exit trap 总会调用 finalizer；FAIL 包至少包含 summary、serial、QMP input/capture、input sequence 和 metadata，排除 disk/socket。cleanup 对已退出 QEMU 仍执行 wait；真实 stub 演练保留 exit 42。版本采集缺 rustc/cargo 时写 `collection_errors`，仍可得到 `VALID_FAIL`。 |
| P-01 | metadata schema 2 同时记录 commit/status/dirty before/after 与 `provenance_stable`；变化或采集不全使 semantic summary FAIL。 |
| T-01 | `zero_dma_bytes` 由生产 DMA 路径调用；直接测试把两页全部填成 `0xa5` 后逐字节验证清零。没有增加第二处裸指针写入。 |
| N-04 | 全 release 队列对相同 key/button release 身份做 coalesce，不淘汰另一个身份；满队列回归验证 dropped 不增加。跨设备全状态重同步仍明确留作后续，不伪称完成。 |

失败证据协议的实际演练：

- 固定版本前置失败：系统 RV QEMU 6.2.0 在要求 9.2.4 时 runner exit 3；干净快照包记录
  `failure_stage=runtime-prerequisites`、7 条失败、前后 commit
  `ec697a8449e00939883e44fb550913159154e051`、dirty=false、stable=true，搬迁后为
  `VALID_FAIL`。证据保存在忽略的
  `test/output/desktop/round3-fixed-rv-blocked/version-mismatch/`。
- QEMU 异常退出：`/tmp` 诊断 stub 通过 9.2.4 版本门禁后立即 exit 42；真实 runner 输出
  `QEMU exited before desktop boot marker; exit=42`，summary 保存 `qemu_exit=42`、
  `runner_exit=1`、`failure_stage=qemu-boot` 和 8 条失败，package 为 `VALID_FAIL`。证据为
  `test/output/desktop/round3-qemu-abnormal-exit-2/`。stub 不进入 Git，也不作为 guest PASS。
- 缺工具链版本的第一次演练因诊断 PATH 漏掉 rustc/cargo，使旧 collector 抛
  `FileNotFoundError` 并得到 exit 70；该目录原样保留为修复前失败。修复后回归用受控 PATH
  重现缺 rustc/cargo，metadata 明确记录两个 collection error，finalizer 不再丢包且 validator
  返回 `VALID_FAIL`。

固定 LA QEMU 9.2.4 的 current-source 中间干净快照运行了 boot、launcher、overlap、
applications、resize 五个场景，全部 runner exit 0、summary PASS、原目录及复制后的 package
均 `VALID_PASS`；metadata 前后 commit 均为 `ec697a8449e00939883e44fb550913159154e051`、
dirty=false、stable=true。复制后的证据位于忽略的
`test/output/desktop/round3-fixed-la-clean/{boot,launcher,overlap,applications,resize}/`。五个
frame SHA-256 分别为：

- boot：`406af959897dcba1f2a770c20e5139416ed8937e6a22d98a21af0b8e58e5e7f2`；
- launcher：`9316db0152d73b31acfa632edfa4d8f361bdeccbf55cb3454235eb1f7936746d`；
- overlap：`6f69810e35bf49e042c875a7606ccb0c0b2d68fe7a4b5765e979ae76f5f79587`；
- applications：`3056b5749344f40c4a247a853371fa61a4488676d4c570e5ba8f1916a83ee169`；
- resize：`4d78ab00b7533c91039645acb8673d98bc6aaca402d24c984ed1845f0e56fb5f`。

五个复制包的 dedicated validator 为 5/5 `VALID_PASS`，在各自目录执行
`sha256sum -c package-files.sha256` 也全部 OK。一次错误循环没有切换工作目录，实际重复检查
boot 五次；该调用不计 5/5，随后按五个独立 package 目录正确重跑。第一次把场景输出指定到
repo 外也被安全边界全部 exit 2 拒绝，改用快照内忽略目录后才真正启动 guest。

实现收尾前精确快照 `ec697a8449e00939883e44fb550913159154e051` 的 canonical quick 为
45/45 PASS、exit 0、294.595 秒，summary SHA-256
`b47611d63093a44480cf96e049f376ada78b1f1f01eeb93a864b11470f355e25`；`full --arch all`
为 53 PASS / 3 FAIL / 3 INFRA_ERROR、0 timeout/crash、exit 2、1,268.136 秒，summary
SHA-256 `f99096e5ba39c99b8a4ed98e20d53ae3efbd3253090e7e90422bc76dab520170`。两者均记录
runner 起止 commit 相同、dirty=false、provenance stable，证据保存在忽略的
`test/output/desktop/round3-canonical/`。

该轮 full 的三个 FAIL 仍是 `evidence.riscv64`（QEMU 6.2.0≠9.2.4）、
`evidence.loongarch64`（真实 `PR3_SMOKE_V1 USER_FAIL tee_device_mode`）和派生 aggregate；三个
INFRA_ERROR 是 clang 14 不支持 `loongarch64-unknown-none`，以及两张官方镜像缺失。与第二轮
报告的 52 PASS / 4 FAIL / 3 INFRA_ERROR 相比，`baseline.cargo_format` 已在不受父 workspace
干扰的 `/tmp` 快照中真实转为 PASS。上述快照随后促成 QEMU wait 与 metadata 缺工具两项加固，
所以只作为中间证据；最终 current-source 快照仍需在这些改动后重建并复验。

固定 QEMU 获取也进行了真实尝试：仓库 `setup_qemu.sh` 固定 9.2.4、134,782,772-byte archive
及 SHA-256 `f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a`。普通沙箱与获批的
外层重试均因 `Could not resolve host: download.qemu.org` exit 6，未留下 partial archive，未改用
未经固定 hash 的替代来源。因此 RV 9.2.4 条件仍是 BLOCKED，而不是“未运行即 PASS”。

本 checkpoint 的首次调用错误也保留：一次 `unittest` 模块路径写法导致 3 个 ImportError，
改为三个文件入口后 28/28 PASS；一次 summary 只读提取命令因 f-string 转义 SyntaxError 后改用
普通 format；QEMU abnormal-exit 测试首次期待了错误文案，实际 qemu_exit/failure_stage 断言已
通过，修正断言后 9/9 PASS。上述过程失败不覆盖最终正确命令，也不归类为产品通过。

AI 使用披露：OpenAI Codex（GPT-5 系列，具体运行时小版本未暴露）用于第二轮报告映射、CI
信任边界设计、证据 schema/validator/finalizer、DMA/input 加固、测试、运行诊断与本文档。
人工负责人仍需解释 self-hosted 信任模型、exit trap/QEMU wait、provenance 不变量、DMA slice
安全边界和 package 双层哈希；第三轮独立人工审查不能由本段 AI 自述替代。

### Checkpoint 10 最终 current-source 验证与第三轮交接

最终干净快照位于 `/tmp/orays-round3-final-source.C5AT5s/repo`，验证提交为
`7b46e94b70437490aca0e8ab1d000c35bc4bef46`。建快照时本轮 29 个变更/新增路径与 live 工作树
逐文件一致；验证前后 `git status --porcelain` 均为空。canonical summary 记录 runner
before/after commit 相同、dirty=false、status 为空、`runner_provenance_stable=true`。gate 后只
更新计划、开发日志和 Goal 状态，没有再修改实现、测试、workflow 或证据脚本。

最终验证记录：

| 命令/证据 | 退出码/结果 | 当前证据 |
| --- | --- | --- |
| Desktop Python discovery | 0；51/51 PASS | 包含 workflow 信任边界、PASS/FAIL package、迁移/篡改/路径逃逸、QEMU exit 42、缺工具 metadata 回归。 |
| `scripts/desktop/build.sh host-test` | 0；65/65 PASS | 包含两页 `0xa5` 后逐字节清零与全 release 队列 coalesce。 |
| Desktop host/RV64/LA64 clippy `-D warnings` | 0；3/3 PASS | 独立 desktop workspace 三目标 lint。 |
| Desktop rustfmt、根 rustfmt、asset、golden、scope、`git diff --check` | 0；全部 PASS | asset 0 registered/5 generated；golden 5/5；scope 113 paths/3 bridge/3 existing/74 churn。 |
| Desktop RV64/LA64 release build | 0；2/2 PASS | live RV ELF/BIN `edbd7672…` / `2513fe7a…`；LA `a8548872…` / `2a862c37…`。 |
| canonical quick | 0；45/45 PASS | 294.533 秒；summary `23c7eb5aed851aebed0b97806ae063a97c31ea0622f31d2735f24f9f50611982`。 |
| canonical `full --arch all` | 2；53 PASS / 3 FAIL / 3 INFRA_ERROR | 59 planned/completed、56 executed、0 timeout/crash、1,281.746 秒；summary `656448c74379cc330260aa849d8ce9c11f0d88a9e91645948439fba4b9184b25`。 |
| 最终快照 RV boot，要求 QEMU 9.2.4 | 3；FAIL package 有效 | 实际 QEMU 6.2.0；`runtime-prerequisites`、7 条失败；移动后 `VALID_FAIL`。 |
| 最终快照 LA boot，要求 QEMU 9.2.4 | 0；PASS package 有效 | QEMU 9.2.4；clean stable provenance；移动后 `VALID_PASS failures=0`。 |

canonical full 的非 PASS 没有被隐藏：

- `evidence.riscv64`：fixed tool contract 要求 `QEMU emulator version 9.2.4`，实际为
  `6.2.0 (Debian 1:6.2+dfsg-2ubuntu6.30)`；
- `evidence.loongarch64`：guest 真实输出
  `PR3_SMOKE_V1 USER_FAIL tee_device_mode arch=loongarch64`，随后 harness nonzero；
- `evidence.aggregate`：继承两个 required shard 的非 PASS；
- `baseline.clippy_loongarch64`：clang 14 capability probe 报
  `unknown target triple 'loongarch64-unknown-none'`，严格分类为 INFRA_ERROR；
- `official.riscv64` / `official.loongarch64`：快照 workspace 根均找不到对应官方镜像，未执行并
  分类为 INFRA_ERROR。

最终 runtime 证据已从快照迁移到 live 忽略目录并用同一最终 validator 重验。LA boot package
为 `VALID_PASS failures=0`，PPM 1280x800、SHA-256
`406af959897dcba1f2a770c20e5139416ed8937e6a22d98a21af0b8e58e5e7f2`；PPM 转为仅供观察的 PNG
后实际检查，桌面、两个窗口、任务栏和通知均可见，未见黑屏、异常裁切或文字重叠。RV package
为 `VALID_FAIL failures=7`。两包的 `review-package.json` SHA-256 分别为 RV
`119a76271c4db2acf44d6bf65a921719810b704714e1891f7b05c98b1f040c3c`、LA
`96277675e2c059e92a9807d0e82b37999e18569195db7525ce7cf14aae07d43e`。

证据路径：

- `test/output/desktop/round3-final-canonical/quick/summary.json`；
- `test/output/desktop/round3-final-canonical/full/summary.json`；
- `test/output/desktop/round3-final-canonical/pr3-evidence/`；
- `test/output/desktop/round3-final-runtime/rv-version-mismatch/`；
- `test/output/desktop/round3-final-runtime/la-boot/`。

过程错误继续保留而不冒充产品失败或 PASS：当前系统没有 `jq`，只读 JSON 提取改用 Python；
第一次 `rsync` 因 live 归档父目录不存在 exit 11，显式创建父目录后两份证据复制成功；图像查看器
不能直接处理 PPM，且 `convert` 不存在，使用本机 `ffmpeg` 生成忽略的 PNG 观察副本后才完成目视
检查。上述步骤没有修改 review package 内容，迁移后的专用 validator 结果保持有效。

第三轮最低条件中 1–4 的本地实现/证据已完成，但 5–9 未满足：workflow 尚未提交/push/实跑，
required checks 未配置，RV64 固定 QEMU 9.2.4 不可用，canonical/official 尚无维护者豁免，官方
镜像未执行。条件 10 只能证明当前分支 merge-base 是目标基线
`c776ceff40587de0fa0547724d0abfecbb56cc64`；因为尚未创建 PR，不能声称远端 PR base 已设置。
未经用户明确授权，本轮未 push、未建 PR、未改 branch protection。Goal 因而停止为
`BLOCKED_WITH_EVIDENCE`；这不是 PR Ready、merge-ready 或完整门禁通过声明。

## 2026-07-20 最新独立复审 evidence/input 整改

收到 `/tmp/codex-evidence-fix-prompt.txt` 后立即停止旧验证路径。本轮没有修改两份人工审查报告、
`tee_device_mode`、GitHub CI 远端状态或 Goal，没有在 live 分支 commit/push；修复顺序为先新增
负向/边界测试并确认当前实现真实失败，再修改实现。

### 实现与契约

- 新增共享 `runtime_evidence_contract.py`，将 canonical QEMU 固定为精确 banner
  `QEMU emulator version 9.2.4`，并统一 metadata schema 3、summary schema 2、package
  schema 3。runner 拒绝改变 policy 的环境 override；collector/summarizer 只接受精确 banner，
  不再用 prefix 判断。
- `qemu_started=false` 的合法状态必须是 null `qemu_exit`、非零 `runner_exit`、已知 pre-start
  stage 与该 stage 允许的非空 reason；合法记录永远产生 semantic FAIL。空白、非法字符、未知
  stage、stage/reason 不一致以及未启动却 runner exit 0 均在 producer/packager/validator
  fail-closed，不能输出 PASS/VALID_PASS。
- 新增 `create-qmp-runtime-dir.py`，固定在 `/tmp` 创建短 runtime dir，检查 socket 路径不超过
  AF_UNIX 107 bytes 且不含 QEMU 参数分隔符。runner 不再读取 `TMPDIR`；EXIT/INT/TERM 均经
  finalizer，signal 使用受约束 `runner-signal` reason，cleanup 回收 QEMU、socket 和 runtime dir。
- dirty provenance 负测改为 before/after status 与 dirty flag 自洽且
  `provenance_stable=false`；relocation 回归与最终 runtime 验证都在移走原 run 目录后再运行
  dedicated validator。
- 输入队列保留既有 `StateReset` 设计并补齐容量 0/1：容量 1 为 reset 腾位时同时统计无法保留的
  旧输入与新 release，pending reset 不被驱逐，重复 resync 不把内部 reset 计为 dropped input。
  `WindowedDesktop` 在 shell dispatch 前处理 reset，确保不依赖 `response.consumed` 取消 drag。

### 测试先行证据

修改实现前，新测试真实暴露以下问题：完整 evidence 配合 `qemu_started=false`、null QEMU exit、
runner exit 0 会得到 PASS；空白/未知/不一致 token 被接受；`9.2.40`、`9.2.4-rc0` 与自洽的
required/observed `6.2.0` 被接受；packager 可组装上述无效 PASS；QMP helper/固定路径不存在；
StateReset 位于 shell dispatch 后；容量 1 dropped 预期 2/3、实际均为 1。修复后相同用例全部
通过。第一次 Python 定向命令因 `test/desktop` 不是 import package 得到 ImportError，切换到该
目录使用模块名后才取得有效失败证据；一次 apps 命令漏掉既有 `host-tools` feature，属于无效调用，
权威 host-test 随后 12/12 apps PASS。

### 最终本地验证

| 命令/证据 | 退出码/结果 | 说明 |
| --- | --- | --- |
| Desktop Python discovery | 0；65/65 PASS | 含 strict version、status enum、legacy schema、forged PASS、dirty、离线 relocation、QMP 长 `TMPDIR` 负测。 |
| `scripts/desktop/build.sh host-test` | 0；73/73 PASS | lib 17、apps 12、graphics 9、input 16、shell 5、window manager 14。 |
| host clippy `--all-targets --features host-tools -- -D warnings` | 0；PASS | 当前 input/app 实现零 warning。 |
| 根/desktop rustfmt、asset、golden、scope、shell syntax、`git diff --check` | 0；全部 PASS | asset 0 registered/5 generated；golden 5/5；scope 115/3/3/74。 |
| Desktop RV64/LA64 release build | 0/0；PASS | live RV ELF/BIN `6b4b8264…` / `aa5ed3f2…`；LA `5d8d2d2…` / `07dac5ac…`。 |
| clean snapshot RV/LA 五场景 | 0；10/10 PASS | 两架构均为精确 QEMU 9.2.4；每项原地 summary/package/validator PASS。 |
| 删除原 run 后 relocated package | 0；10/10 `VALID_PASS` | 原绝对路径不存在时 dedicated validator 仍完整复现语义。 |
| canonical quick | 0；45/45 PASS | 300.404 秒；summary SHA-256 `d994e0256d4915864bd303515fc7e2d0db86bf8460ada107aab4d0a492e54426`。 |

最终隔离快照为 `/tmp/orays-evidence-fix.icXYne/repo`，仅用于验证的临时提交
`3a0fade10f542908dc689faf0a4eaf1d85a56b02`；全部 tracked diff 与 8 个新增源码/测试文件均与
live 逐文件 `cmp` 一致，快照前后 clean。canonical summary 记录 runner before/final commit 相同、
dirty=false、status=[]、`runner_provenance_stable=true`。十个搬迁包与 quick summary 已保存至
忽略目录 `test/output/desktop/evidence-fix-final/` 并再次得到 10/10 `VALID_PASS`。

首次并行启动 RV/LA 时，两个进程竞争创建共同 `evidence-fix` 父目录，LA boot 在 guest 启动前
因 `FileExistsError` 失败；该调用不计 PASS。父目录存在后单独重跑 LA boot 才得到真实
`PASS + VALID_PASS`。没有用重跑覆盖或删除首次失败事实。

当前本机两套 QEMU 都精确为 9.2.4，但 `/root/sdcard-rv.img` 与 `/root/sdcard-la.img` 仍不可读。
因此本轮没有宣称 official/full PASS；历史 LA `tee_device_mode` 非 PASS 未修改、未隐藏。GitHub
workflow/required checks 未实跑，第三轮独立人工审查尚未进行，当前状态仍不是 PR Ready 或
merge-ready。AI 使用：OpenAI Codex 用于复审映射、测试先行、evidence/input 实现、隔离 runtime
验证与本日志更新；人工负责人仍需独立解释 status enum、schema bump、QMP path/cleanup、
StateReset 和 relocated package 不变量。
