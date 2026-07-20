# OrayS 图形桌面环境执行计划

状态：`BLOCKED_WITH_EVIDENCE`

分支：`feature/orays-desktop-environment`

基线：`c776ceff40587de0fa0547724d0abfecbb56cc64`，与
`.codex/state/desktop-base-sha`、`develop/post-integration-next` 及接管时 HEAD 一致。

## 启动与遗留接管

- 2026-07-18 在 `/root/OrayS-desktop` 重新执行完整启动协议；未恢复旧会话。
- 当前分支为 `feature/orays-desktop-environment`，上游显示为
  `origin/develop/post-integration-next`。
- 接管时只有 11 个未跟踪文件，没有已跟踪改动；这些文件已逐项读取并作为待审计工作
  保留，没有 reset、覆盖或丢弃。
- `scripts/desktop/check-headless-host.sh` 返回 0 / `PASS_WITH_WARNINGS`：缺少可选的
  `socat`、ImageMagick `convert`，系统 Python 3.10 没有 `tomllib`；RV64/LA64 QEMU
  均声明至少一个 headless display backend。
- `python3 scripts/desktop/check-scope.py` 返回 0：11 个 changed paths、0 个 bridge、
  `DESKTOP_SCOPE=PASS`。
- canonical suite `--list` 返回 0 并注册 59 个 case；由于工作树包含本 PR 的未提交文件，
  canonical quick/baseline 的 clean-worktree 前置条件尚不成立，不把未运行写成 PASS。

## 目标

在不改变默认内核和官方评测路径的前提下，通过独立 `user/desktop` workspace 实现一个现代图形桌面。

## 非目标

- Wayland/X11；
- 多进程 GUI 协议；
- 浏览器；
- 音频；
- 3D 加速；
- 修改 Linux/POSIX ABI；
- 修改官方评测。

## 隔离

- 默认内核构建不启用桌面；
- 根 workspace 不加入桌面；
- 使用 `Makefile.desktop`；
- 既有 bridge 文件最多 8 个；
- bridge 文件总变更不超过 250 行；
- 每个 checkpoint 运行 scope checker。

## Headless 验证

- memory framebuffer；
- golden image；
- QEMU localhost-only VNC；
- QMP screendump；
- QMP/HMP input injection；
- serial markers；
- screenshot hashes。

## Checkpoint

- [x] 0：基线和隔离
- [x] 1：技术选型 spike
- [x] 2：独立 workspace 和渲染
- [x] 3：输入
- [x] 4：窗口系统
- [x] 5：桌面 Shell
- [x] 6：核心应用
- [x] 7：无显示 QEMU 证据
- [x] 8：性能、回归和文档收尾

## 风险

- VirtIO input 能力可能缺失；
- LoongArch64 图形 QEMU 能力可能与 RV64 不一致；
- 大型 GUI 框架可能破坏离线构建；
- 中文字体和图片资产体积；
- 软件阴影与透明度性能；
- 当前稳定化分支已有非桌面失败。

## Checkpoint 0 证据

- 直接在最小权限工作树构建时，RV64/LA64 均在编译前后遇到只读边界：离线依赖恢复
  试图写 `vendor/.cargo.unpack.*`，随后 `arceos_posix_api/build.rs` 试图重写
  `ulib/axlibc/include/ax_pthread_mutex.h`；两次均退出 2，保留为环境边界证据。
- 未给 `vendor/`、POSIX 或 ulib 目录扩大写权限。改用 `git archive HEAD` 在
  `build/desktop/baseline/source/` 创建精确 HEAD 隔离快照，并在该可写快照运行同一默认
  构建。
- RV64 默认构建退出 0；产物
  `build/desktop/baseline/snapshot-build/kernel-rv`，大小 2,024,592 bytes，SHA-256
  `126f55442212669a6690d811da4e9f8d76d357fa722b7816b652b956f46d886f`。
- LA64 默认构建退出 0；产物
  `build/desktop/baseline/snapshot-build/kernel-la`，大小 3,078,616 bytes，SHA-256
  `f8f5f50735e9de9713b9b86776c76831f6332c110ed9f61e669ae0bc032a1e53`。
- 两套构建使用同一 SHA 的源快照；已有编译 warning 原样保留，没有当作新桌面结果。

## Checkpoint 2 证据

- 新增独立 `user/desktop/Cargo.toml` workspace 与独立 lockfile；根 workspace、lockfile、
  Makefile 和默认 feature 未修改。
- `scripts/desktop/build.sh` 将固定 Cargo archive 解包到 `build/desktop/vendor/`，生成的
  Cargo config、axconfig、linker、target 和产物均位于 desktop build root；未扩大根
  `vendor/` 或 POSIX 写权限。
- host 图形 integration tests 8/8 PASS，覆盖 clipping、alpha、stride padding、damage
  合并、局部复制、错误几何和完整帧提交。
- 1280×720 PPM 三次渲染逐字节一致；内部 checksum `08135c358d1a61b0`，文件 SHA-256
  `e7587a142932adbb41286cd29897daff11e63b0f0fb0fcdf0cd9c6a06c87921b`。本地 PNG 观察副本
  已检查，未把 host 图像当作 guest PASS。
- RV64 release 构建退出 0：ELF 1,438,320 bytes / SHA-256
  `8be59d1dbfa81f2d253808d8f3bfb612c0abc53772370c1f958c32e398000e83`；BIN 524,480 bytes /
  SHA-256 `491a73c550886cd382eb11832226b1000596437d3fd07a44c747bb473753dff2`。
- LA64 release 构建退出 0：ELF 1,021,208 bytes / SHA-256
  `cc9a1366907231d36ddc0b0f07dfd90add975b3fa039fc65b802206216277daa`；BIN 680,128 bytes /
  SHA-256 `a49d220497bdaf718cee4ac70402337c0356d012ab343e16a3b76788054c6fd1`。
- checkpoint scope：35 paths、1 个既有 bridge、20 行 churn、`DESKTOP_SCOPE=PASS`。

## Checkpoint 3 证据

- Checkpoint 3 最初新增默认关闭的 `axdriver/virtio-input` feature；复用固定的
  `virtio-drivers 0.13.0`，在 MMIO/PCI bus 发现 keyboard/tablet，最多保存 4 个设备并
  公平轮询。Checkpoint 8 的真实双架构生命周期验证证明该归属仍过宽，最终实现已改为
  桌面本地驱动加 `axdriver/desktop-device-hook` 窄发现桥；本段保留为历史证据。
- 桌面输入层支持按下/释放/重复、左右 Shift/Ctrl/Alt/Super、evdev 键位文本、相对移动、
  设备校准的绝对移动、三键、滚轮、有界队列和 drop 计数；空闲循环 sleep 8 ms。
- host tests：图形 8/8、输入 5/5 PASS；Python QMP 协议/验证测试 3/3 PASS。
- RV64/LA64 desktop release 构建均退出 0。RV64 ELF/BIN SHA-256 为
  `b1c0a59ba81b2034951cdf240d8190494c1ebf517751a90403dfb625bf616329` /
  `c5269d97ca1f107d90db1958116019df5e392746ad96ddaaa5717442b1341198`；LA64 为
  `b9f579a7c62342f18dc461f12b0bc190827af53c96a8351cc178fa3c786598ce` /
  `94458acfab31aa817db8a0057e052c75f764821e7abde37895060041a5a8a76b`。
- RV64 QEMU 使用 localhost-only VNC、Unix QMP、VirtIO GPU/keyboard/tablet 真实启动并经
  QMP 正常退出。guest marker 证明 code 30 的 `a` 按放、绝对指针移动和左键按放端到端
  可用；成功 transcript SHA-256
  `21eb64aabb4050d71ddb3cce8d2119b3d79cc01a8efb91d00e633a3c9cb495f4`。
- QMP screendump 为 1024×768，PPM SHA-256
  `dd897a3da27a678dfe7efd39acfb6301c3698f08e29701785f3577252c080610`；PNG 观察副本已实际
  检查。首轮 relative 注入因 guest 未配置相对 mouse 而退出 1，随后改为 tablet absolute；
  host 相对移动测试仍保留。
- checkpoint scope：45 paths、6 个 bridge paths，其中 5 个既有 bridge、61 行 churn、
  `DESKTOP_SCOPE=PASS`。

## Checkpoint 4 证据

- 新增独立窗口模型和管理器，覆盖 create/close、title bar、focus、bottom-to-top z-order、
  move、四边/四角 resize、minimum size、minimize、maximize/restore、Alt-Tab 和模态阻塞；
  关闭 owner 会递归关闭其 modal children。
- compositor 按 damage region 重画确定性全局渐变、阴影、窗口装饰、client 内容和 cursor；
  宿主测试把 move/focus 后的增量结果逐像素对比 fresh full composition，最终一致。
- 输入事件已接入真实窗口运行时：左键 title/edge/control 命中、pointer drag、Alt-Tab 会触发
  局部 present；无 damage 的普通键或 release 不提交空帧。guest 主入口现在启动两个真实
  重叠窗口，而非仅打印输入。
- host Rust integration tests 最终为 graphics 8、input 5、window/runtime 7，共 20/20
  PASS；host `cargo clippy --all-targets --features host-tools -- -D warnings` 退出 0。
- RV64 release 最终 ELF/BIN SHA-256 为
  `650f9b1d942f93b6bf6f5b69b85bb9f0478586a18126fb9518310f9479d20705` /
  `755c8bce50ab15c020eac8f879e687d327f2f30ba1e830430b3cdb58e8ea89cd`；LA64 为
  `771dbb788ea871a37aaab466e757442ab06f9317516e9c48a572fad5ae477dab` /
  `957d483417d0c339deadbebb9bfe6e50339bc501f50b674f1a25f95056a2293d`。
- checkpoint scope：50 paths、6 个 bridge paths，其中 5 个既有 bridge、61 行 churn、
  `DESKTOP_SCOPE=PASS`；`git diff --check` 与 desktop rustfmt check 均退出 0。

## Checkpoint 5 证据

- 完成自适应壁纸、顶栏、居中 Dock、六应用启动器、通知队列、电源菜单和 dark/light
  theme；顶栏时间为真实桌面 elapsed uptime，不伪装成未提供的 wall clock。
- 圆角与透明面板由软件 alpha composition 实现；阴影复用固定 16 级 falloff cache，未实现
  实时全屏模糊。窗口与启动器动画都是有限状态动画，空闲后不继续提交 frame。
- shutdown 通过 `axstd::process::exit(0)` 接入真实系统退出路径；当前底层没有可用 reboot
  API，因此 restart 入口明确报告 `restart_supported=false`，没有伪造成功。
- host Rust tests 28/28 PASS（graphics 9、input 5、shell 5、window/runtime 9），host clippy
  `-D warnings`、desktop rustfmt 与 `git diff --check` 均退出 0；Python discovery 6/6 PASS。
- 1280x720 的 boot/launcher/light/power 场景 SHA-256 分别为
  `6d834114499d957e60c536b90319356fa1641548330f898f957e174330c9d4e4`、
  `8e6520b431ef532e340cf3ac8c66cc519f370b810b32e06402dc6178aaa08679`、
  `694a3cefdb8b31776e7d0de14aa1415e8ef24330bca052bc5ce4e26bbf3179cc`、
  `ee5dadd7e0c0deba943e58775a2a7f2141a23def3ed757285559204ff3d87283`；
  launcher 重复生成逐字节一致。生成的 PNG 观察副本已实际检查，但不作为 guest PASS。
- RV64 release ELF/BIN SHA-256 为
  `cec70be5db5ca6daeeb4439bb94e3dbb11e3fbb23c7d22413292677f3aaae2b1` /
  `bafcb7aa274c6b943f9d2056005b451bb6d167dc5df2c3ffa166effb1b2fa318`；LA64 为
  `02f15d8d0a51361ea2d5706ce02a397ad09e2b9d74f0e5c3ceb593c8cdf65af0` /
  `9f348e6098b09f720aadea010387d84c4bb2f2ba7dff795c3e4b12a74af311bc`。
- checkpoint scope：65 paths、6 个 bridge paths，其中 5 个既有 bridge、61 行 churn、
  `DESKTOP_SCOPE=PASS`。

## Checkpoint 6 证据

- 新增唯一平台文件系统边界，host 与 OrayS 后端均执行真实 read directory、read/write、
  create directory、rename 和 remove；路径 join 拒绝空名、`.`、`..` 和包含 `/` 的不安全
  entry name，文本读写限制为 1 MiB。
- 六应用均为真实模型并接入焦点窗口输入与 damage 合成：终端执行真实文件系统 builtin；
  文件管理器读取/新建/重命名/删除/打开并显示错误；编辑器打开/编辑/保存/dirty close
  confirmation；图片查看器解析 P3/P6 PPM 并 fit/25%-400% zoom；监视器只显示 desktop
  elapsed/window/input 计数，并把 CPU/memory 标为 `UNSUPPORTED`；设置实时改变主题和三种
  程序化壁纸。
- host Rust tests 37/37 PASS，其中 9 项 app tests 使用 `/tmp/orays-desktop-*` 隔离目录验证
  真实文件 mutation、missing path、非法路径、PPM truncation、zoom clamp、monitor 更新节流、
  live settings 与 dirty close guard。Python discovery 7/7、clippy `-D warnings` 均 PASS。
- `applications` host scene 使用忽略目录中的真实文件生成 file manager/editor/monitor 画面；
  两次输出逐字节一致，PPM SHA-256
  `8f0f9d945b1f0081b73c24bd6eb3f24374306d90d00ad662711f07ae3059ab54`。PNG 观察副本已实际
  检查；仍不作为 guest PASS。
- RV64 release ELF/BIN SHA-256 为
  `bb1abe54007918662f1876b2d6e483190156a5eb92f52f9c27bca05c3e57d0e8` /
  `dd3a7c560fb877a37630f8866318743cb56f5bc00427e64a0b1414929046e940`；LA64 为
  `a7566be1ab63cf6a318841f73afc6544d53d2676dd9445215f03d318d43be886` /
  `182786b5cbeade6d549ded9b9cf1a29811063a6710fe8ab8ebfd59592255a69f`。
- checkpoint scope：83 paths、6 个 bridge paths，其中 5 个既有 bridge、61 行 churn、
  `DESKTOP_SCOPE=PASS`；diff check 与 desktop rustfmt check 均 PASS。

## Checkpoint 7 证据

- 新增统一 `run-headless-qemu.sh`：从任意 cwd 解析 Git 根，强制清除显示会话环境变量，使用
  localhost-only VNC、Unix QMP、VirtIO block/GPU/keyboard/tablet、串口 stdout 和只在
  `test/output/desktop/` 下创建的新运行目录。runner 等待 boot/action guest marker 后才截图，
  QMP 正常 quit，并由 `summarize-run.py` 对退出码、marker、PPM 几何/大小、transcript、输入
  序列与哈希 fail-closed 判定。
- boot 也保存显式的无事件等待序列和 QMP 握手 transcript；没有把零输入伪装成键鼠交互。
  RV64 launcher/overlap/applications 分别验证 Super+Space、Alt-Tab 和 pointer 启动 Settings 后
  实时切换 Light theme；串口动作 marker 证明来宾已消费输入。
- 当前源码最终 RV64 证据：`qemu-rv-boot.ewCKpn`、`qemu-rv-launcher.KAog2y`、
  `qemu-rv-overlap.HvnHin`、`qemu-rv-applications.OtbvDE`；四项均 `result=PASS`、
  `failures=[]`、QEMU exit 0。截图 SHA-256 依次为 `f0400b5a...`、`63a15c2e...`、
  `1a5676a1...`、`c6c05689...`。
- 当前源码 LA64 boot 证据 `qemu-la-boot.RLGS51` 同样 PASS，1280x800 截图 SHA-256
  `9661a86d...`。最终 RV64 ELF/BIN 为 `3dc1c754...` / `cf9e89da...`；LA64 为
  `575a1370...` / `3c530d07...`。
- 五张最终 guest 截图均实际查看。复核过程中发现并修复 LA64 较矮 monitor client 中 footer
  与 memory 行重叠；新增小客户区回归测试，最终 host Rust 38/38、Python 7/7、clippy
  `-D warnings`、rustfmt、diff/scope 均通过。
- 串口仍原样保留底层 FAT 目录 metadata 的 `Is a directory` 诊断；文件管理器成功列出目录，
  summary 未把该文本删除或改写。Checkpoint 8 将其列为已知噪声而非完整无错误声明。

## Checkpoint 8 证据

- 最终设备归属收敛到 `user/desktop/src/platform/`：`display.rs` 直接持有
  `virtio-drivers 0.13.0` 的 `VirtIOGpu`、transport 与 framebuffer，`input.rs` 持有最多 4 个
  keyboard/tablet 和公平轮询状态，`virtio.rs` 提供共用 DMA HAL。`axdisplay` 无改动且不在
  桌面依赖图中。
- `axdriver/desktop-device-hook` 不新增依赖，只在既有 MMIO/PCI 枚举及 BAR 配置生命周期中
  调用桌面提供的两个固定 C ABI 探测符号。默认 feature 关闭，根 workspace、根
  `Cargo.lock`、根 Makefile 和 Linux/POSIX ABI 均未修改。
- DMA allocation 在返回给 `virtio-drivers` 前完整清零；显示重配置先将旧 framebuffer
  `NonNull` 置空，再执行可能释放旧 DMA allocation 的 `change_resolution`，失败后
  `present` 只能返回错误。调用设备前拒绝零尺寸与 `u32 width * height * 4` 溢出，相关
  `unsafe` 不变量与调用者责任写在实现旁并有 host 回归。
- 晚于总线枚举的 app 初始化、仅中断 ack 和 display-init callback 均不能可靠取得 RV64
  MMIO 输入，真实失败保存在 `qemu-rv-applications.JxEf63`、`axXEWW`、`reX7Y2`、
  `qemu-rv-launcher.MEzdPX` 和 `MRHHvC`。最终总线发现 hook 后，guest 在注入前稳定输出精确的
  `ORAYS_DESKTOP_INPUT_READY devices=2`，没有按测试名、场景或路径分支。
- 直接在 app 初始化时重新扫描 GPU 会为已归属 input 的 MMIO range 再构造 transport，违反
  唯一所有权；`qemu-rv-launcher.rT4ypp`、`nJDfzO` 均因收不到 guest 输入 marker 失败并保留。
  改为同一总线生命周期同时 handoff GPU/input 后，`qemu-rv-launcher.E1jvXp` 恢复真实输入。
- LA64 的 PCI MMIO window 只有 128 KiB；QEMU 隐式 VirtIO NIC 会先耗尽 BAR，导致 tablet
  `NoMemory`/`BarNotAllocated(4)`。失败 `qemu-la-applications.f4uHuw` 与实时
  `query-pci` 诊断 `x4dpQ7` 均保留。runner 只对 LA64 使用 `-nic none`；RV64 保持默认 MMIO
  topology。跨架构无条件禁用 NIC 的错误尝试也未接受。
- RFB `SetDesktopSize` 替换了不能在 realized QEMU GPU 上生效的 QMP `qom-set` 尝试；失败
  `qemu-rv-resize.qqj3OL` 保留。终审修复前 RV64 `qemu-rv-resize.ErRQSB` 从 1024x768、LA64
  `qemu-la-resize.2lYk5q` 从 1280x800 真实变为 900x650，随后 guest 均消费 resize 后中心
  pointer `(450,325)`。两份 summary 为 PASS、QEMU exit 0，截图 SHA-256 分别为
  `8185dac94cfaf1c80a5262c3e20e80016328828b0d1ec4b5102179f36fa7afa1` 与
  `d5c95b6a72771eb7e75c87658d504aeef9967c940d838073a6ee8a5eea50f5d1`。
- 终审修复前同源码 headless 矩阵另外包含 RV64
  `qemu-rv-boot.9mYVSe` / `qemu-rv-launcher.oomKC5` / `qemu-rv-overlap.wi0gXl` /
  `qemu-rv-applications.UTgANh`，LA64 `qemu-la-boot.rts9w2` /
  `qemu-la-launcher.6AtklF` / `qemu-la-overlap.Jysffs` /
  `qemu-la-applications.mNI9es`。加上 resize 共十份 summary 均为 `PASS`、`failures=[]`、
  QEMU exit 0；普通 RV64 为 1024x768，普通 LA64 为 1280x800。八张普通截图 SHA-256 依次为
  `b88affa91988822e58e0f52714438e4979a8f1aa0aeec2616e0d50254427140c`、
  `91eda043e096f952aa813d15c29edc96a3e1edebce5b4dc48236605a6f481acc`、
  `1a5676a1055809f457e821fe250c8c0ab4c542c431c98674c650a830bda0e557`、
  `c6c05689e2f47437bee9e8602f317c786826d57eb081e8010e9a3993c0233d56`、
  `406af959897dcba1f2a770c20e5139416ed8937e6a22d98a21af0b8e58e5e7f2`、
  `a1a1ee9c9e0d9045c02021018280788295db43e87d087e301734bbb651b676eb`、
  `6f69810e35bf49e042c875a7606ccb0c0b2d68fe7a4b5765e979ae76f5f79587`、
  `3056b5749344f40c4a247a853371fa61a4488676d4c570e5ba8f1916a83ee169`，
  十张 PNG 观察副本均已实际检查，没有发现 clipping、空白帧或文字重叠。
- 终审修复前 desktop release 产物：RV64 ELF 1,661,432 bytes /
  `20d8794b188040d0fe4243b23672a0a70022962aed51c722f0184c42d95e50a5`，BIN 655,552 bytes /
  `321d28ca75dcd54670e27b848e0465ba4bb4272e474a32eace7911854ea91ea2`；
  LA64 ELF 1,159,632 bytes / BIN 852,160 bytes 为
  `af5d71c2376c649c461835a3dd1b3af847718be67e585ab1f11e98d52132a6b4` /
  `b802e12179837aaa2c4b0df6f6567c1464df959104550b546d585dff77d508d5`。
- 终审修复前 host Rust 42/42、Python discovery 20/20、host/RV64/LA64 clippy `-D warnings`、desktop rustfmt、
  `git diff --check` 均 PASS。资产检查为 `registered_files=0 generated=5`；五份 golden
  comparison PASS。1024x768 release host software compositor 的 7 次记录为
  48.245-48.908 ms，median 48.549 ms；范围明确不含 PPM 写入，`threshold=null`，没有伪造
  性能门槛。
- 终审修复前 scope 为 102 paths、3 个 bridge paths、3 个既有 bridge、56 行既有文件 churn，
  `DESKTOP_SCOPE=PASS`，低于 8 文件/250 行预算。
- 较早精确快照 `/tmp/orays-desktop-final-source.I527qe` 的 canonical quick 为 45/45 PASS，证据
  `test/output/desktop/final-gates/quick-24dab13-retry1/summary.json`。canonical baseline 在
  57/57 项全部执行后为 51 PASS / 3 FAIL / 3 TIMEOUT，证据
  `test/output/desktop/final-gates/baseline-24dab13-corrected/summary.json`；首次失败没有被重试
  覆盖。三项 FAIL 分别为 RV64 固定 QEMU 版本不符、LA64 既有
  `tee_device_mode`/guest nonzero，以及继承前两者的 aggregate；三项 TIMEOUT 为
  `unit.suite_runner`、`unit.synthetic_capability_integrity`、
  `unit.syscall_boundary_regressions`。后续 quick 证明这些单元项可通过，但仍按 flake/缺陷记录。
- 上述 canonical 证据早于后续终审安全整改，只保留为历史 gate 结果。终审修复前的源码验证
  建立了逐字节匹配 102 个 task path 的干净快照；首个 synthetic root
  commit `368ab67367569fae1ace364a2d849cb69e1451bc` 因没有 canonical
  `origin/main` 引用而使 quick 以 infrastructure error / exit 2 提前退出，未计作测试结果。
  随后保持完全相同 tree，以真实基线父提交 `c776ceff40587de0fa0547724d0abfecbb56cc64`
  重建快照 commit `0924ee3083eb8016f9aef32edb914c00b239ba3a`，并从本地仓库取得固定
  `origin/main=921171ac1ef5c85ab5a7cd1882dd40e1471b79f0`；runner 起止均为该 commit、
  dirty=false、`runner_provenance_stable=true`。
- 该修复前快照默认 RV64 首次构建被执行环境 SIGTERM（exit 143），同命令独立重跑 exit 0；
  LA64 exit 0。最终产物为 RV64 2,024,584 bytes /
  `c2785bd39db8da9c216bb17c3fceaeff1880977ee0f4be265b12567180aa528a`，LA64
  3,078,616 bytes / `85f7a0ebe3a8d06a509528d5b1b4f46c4c1db8eaa1a7dff14a3f95d07ac05685`。
- 该修复前快照 canonical quick 为 45/45 PASS；summary 位于
  `test/output/desktop/final-gates/snapshot-0924ee30-quick/summary.json`，SHA-256
  `9bf9a934235e96054bd92218eb791160bf20252f6d512e6851b9f1e746350398`。
  canonical baseline 的 57 项均得到终态，56 项实际启动，结果为
  53 PASS / 3 FAIL / 1 INFRA_ERROR / 0 TIMEOUT / 0 CRASH，退出 2；summary 位于
  `test/output/desktop/final-gates/snapshot-0924ee30-baseline/summary.json`，SHA-256
  `07395183fd31eda1c89da0ee1c8af85c79fe15088df8a42aec253ca9296c3259`。
  三项 FAIL 仍为 RV64 固定 QEMU 版本不符、LA64 `tee_device_mode`/guest nonzero 和
  aggregate；LA64 canonical clippy 因主机 clang 不支持
  `loongarch64-unknown-none` target 而被 fail-closed 记为 INFRA_ERROR。较早三项 timeout
  仍保留为历史首次失败；本轮对应 quick 单元项均 PASS，不覆盖旧记录。
- 首轮独立只读 review 无 Blocker、发现 4 个 Major：未 gated 的 axdisplay bridge、summary
  验证不够 fail-closed、直接依赖许可证记录不完整、缺少运行时分辨率变化路径；另有 Settings
  appearance 状态不同步 Minor。中间整改曾给 `axdisplay` 加 feature gate；后续复审指出真实
  backend 仍缓存 geometry，不能证明设备 resize，最终删除全部 `axdisplay` 改动，改为桌面
  直接持有 GPU。summary 解析并绑定 QMP/sequence/guest marker/geometry；三项直接依赖补齐
  版本、来源、许可证与离线位置；display/backend、surface、workspace、window constraints 和
  input extent 支持真实运行时重配置；Settings 同步 live shell。
  修复 summary 后首次 CLI 运行暴露 `NameError`，失败目录 `qemu-rv-boot.Ykb1TB` 与
  `qemu-rv-launcher.TGmZUm` 保留且不计 PASS；新增 CLI 端到端测试后再执行上述 8/8 最终矩阵。
- 后续安全复审又发现 framebuffer 重配置失败路径可能保留旧裸指针，以及 DMA 页未满足
  `virtio-drivers::Hal` 的 zeroed 合同；现实现已按前述方式修复，并完成双架构真实 resize、
  双架构 clippy 和 host 回归。上述 `0924ee30` snapshot 与旧十场景矩阵均早于 2026-07-19
  终审整改，明确标为 stale，不用于 current-source 签收。
- 2026-07-19 终审整改修复了有界文件读取、设备初始化失败仍 claim、registry 满槽后才初始化、
  input 零设备晚期重扫、PCI ECAM 第二 root 别名、MMIO transport 重复构造、输出目录 symlink/
  clean-checkout 创建边界和 asset license traversal/symlink。PCI hook 现在只在同步 FFI 调用期间
  借用 axdriver 唯一活跃的 `PciRoot<MmioCam<'static>>`；MMIO hook 只构造一个 transport 并按
  `DeviceType` 移交。input 在 transport/driver 初始化前预留槽位，只消费 hook registry。
- 字体、图标、阴影缓存均已有实现与测试：编译期 `GLYPH_ATLAS`、六类
  `APP_ICON_MASKS` 和 16 级 `ShadowCache`。五场景 host golden 哈希逐项 MATCH，证明 atlas
  改造没有改变既有像素输出。
- 当前定向结果：host Rust 53/53、Python 28/28、host/RV64/LA64 clippy `-D warnings`、asset
  checker、golden comparison、rustfmt、`git diff --check` 均 PASS。scope 为 104 paths、3 个
  bridge paths、3 个既有 bridge、74 行既有文件 churn，`DESKTOP_SCOPE=PASS`。
- 当前 desktop release 产物为 RV64 ELF 1,664,336 bytes /
  `2482a3fb02abe09ee936315a8294d5cfb070a0019a06f7d7413341dfc4be6850`，BIN 659,648 bytes /
  `2972760dd93b6aa300e359e178def103f25465495aa1c5b3b0d5cc81b82d86b6`；LA64 ELF
  1,229,656 bytes / `b7e17c3a6de4256f39db873f3bd23e8f911ffc40bcb8b79b1c10c430de071f0a`，
  BIN 856,256 bytes / `eeccc0a16402f837f19322935f2e1c99e9c25f17eb80901c93d2393125c7f572`。
- 当前源码 headless 矩阵为 RV64 `qemu-rv-boot.49b4a6`、`qemu-rv-launcher.2abaef`、
  `qemu-rv-overlap.7460a2`、`qemu-rv-applications.6aa0f5`、`qemu-rv-resize.f80d09`；LA64
  `qemu-la-boot.35168e`、`qemu-la-launcher.13447e`、`qemu-la-overlap.aca5e1`、
  `qemu-la-applications.59ba55`、`qemu-la-resize.554bfd`。十份 summary 均为 `PASS`、
  `failures=[]`、QEMU exit 0，十份 hash manifest 全项校验 OK。截图 SHA-256 按上述顺序为
  `538cd8c5060bcb8f644d48aa6fc9c66918dc11f65f00c50270957cd10a6eebbb`、
  `01ee1cc52024eed3387ec8eabbdd97ad74eec3031f164cbd11f781c6fe69d3ab`、
  `1a5676a1055809f457e821fe250c8c0ab4c542c431c98674c650a830bda0e557`、
  `c6c05689e2f47437bee9e8602f317c786826d57eb081e8010e9a3993c0233d56`、
  `d35787c94a68de2e990c588dbe6cfe6fefb53e731804606e144c3439ab952d80`、
  `4c5544dff4570b833b5f2ff6bd107c9b8e5f1a5867c9f94436ad77d0e3308aba`、
  `3f7690288afae6864aa78ca8a78eaf0c899ac2f4b5fc2cf5e5156e3621d9cf87`、
  `f98b59e819d817b510851da2f5e8309401d4839d7d81c1a31b924a991a16533a`、
  `3056b5749344f40c4a247a853371fa61a4488676d4c570e5ba8f1916a83ee169`、
  `be8448ef6425d4d0bd0ec5085bf1560b0d527cb47ee133b1178f2ca4878be647`。十张观察 PNG 已逐张
  查看，未发现空白帧、异常 clipping 或文字重叠。
- 独立空载性能记录为 7 个 1024x768 release sample，47.814–49.512 ms，median 48.142 ms；
  范围不含 PPM 写入，`threshold=null`，没有伪造阈值。
- reviewer 已独立复跑 RV64/LA64 boot 并关闭 PCI/MMIO hook/FFI 唯一所有权 Blocker。最终
  current-source 干净快照的默认双架构构建与 canonical quick/baseline 尚待执行；在此之前
  不把旧 snapshot 结果称为当前结果，也不提前写“无 Major”。
- 后续 reviewer 用 `qemu-rv-launcher.fc0e28` 证明旧协议会在 action 后、launcher 动画提交前
  截图且仍由 summary 误报 PASS；该轮保留为 `PROTOCOL_PASS_VISUAL_FAIL`。guest 现在只在
  launcher `progress == 1000` 且完成 present 后输出唯一
  `ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE`。QMP capture 在实际 `screendump` 前重新验证
  唯一 action/stable 顺序并记录 append-safe serial prefix hash/行号；summary 将 sidecar 纳入
  hash manifest，并有缺 marker、反序、prefix 篡改负例。旧 `fc0e28` 用新 validator 重验为 FAIL。
- resize 的中心 pointer 日志原先早于 handler/present，仍存在截图竞态。输入日志已通用地移至
  handler 和可能的 present 完成后；summary 额外要求
  `DISPLAY_CHANGED < FRAME input < center PointerMoved`。最终 RV64/LA64 resize 分别在 serial
  行 `91<93<94`、`33<35<36`，两张 900x650 截图的 cursor 均实际位于中心。
- `check-scope.py` 的 Git 查询原先依赖调用者 cwd；现全部绑定脚本解析出的 repo root，新增从
  根目录和 `user/desktop` 调用结果逐字节一致的回归。最终两处均报告 105 paths / 3 bridge /
  3 existing / 74 churn / PASS。一次错误的无 `--target-dir` Cargo 调用产生
  `user/desktop/target`，另有 Python cache；均未删除，分别可恢复地移动到新的 `/tmp` 归档，
  当前树无对应污染。
- 最终 desktop 定向验证：host Rust 53/53、Python 35/35、host/RV64/LA64 clippy
  `-D warnings`、asset 0 external/5 generated、golden 5/5、rustfmt、diff check 与 scope 均 PASS。
  release 产物为 RV64 ELF 1,664,552 bytes / `ca56722a…`、BIN 659,648 bytes / `6ceb67b2…`；
  LA64 ELF 1,229,656 bytes / `9b032b9d…`、BIN 856,256 bytes / `bee26d1e…`。
- 最终 current-source headless 矩阵为 RV64 `a3877b` / `cd1390` / `7c5ba5` / `5249f1` /
  `5a7d8f`，LA64 `ccbd77` / `af7d8c` / `bc8062` / `e9043a` / `18f76c`，顺序均为
  boot/launcher/overlap/applications/resize。10/10 summary PASS、`failures=[]`、QEMU exit 0，
  10 份 manifest 全项 OK，十张图均已实际查看。所有更早矩阵均为 stale/historical。
- 最终实现与测试文件集在两个互相独立的干净快照中均指向同一 Git tree
  `b943e2aecfec0ade8b757790880edf3e3305f3cf`；reviewer 快照 commit
  `3222ac5ef19e343d98cc609bbdffe44fbceb0040` 的 105 个变更路径与 live 工作树逐路径、逐文件
  一致，路径/内容清单整体 SHA-256 为
  `72c2a5f80e32d43067fc2a118f2149ca768b5a92094bc655e9028084c495bd2e`。
- 同一 tree 的 canonical quick 有两份均需保留的真实结果：主快照 45/45 PASS、退出 0，
  `unit.suite_runner` 用时 296.708/300 秒；独立 reviewer 快照 44 PASS / 1 TIMEOUT、退出 1，
  同项在 302.012 秒被终止。reviewer summary 位于
  `test/output/desktop/final-gates/final-review-3222ac5-quick/summary.json`，SHA-256
  `717ac8956e6dfee3edbda81a49067c9106299db61fe5ce28f52127e8629fb082`。后续 baseline 中该项
  202.928 秒 PASS；因此分类为真实近阈值 flake/资源竞争风险，而不是用成功重跑覆盖首次超时。
- 精确 reviewer 快照的 canonical baseline 57/57 项均有终态、56 项实际启动，结果为
  53 PASS / 3 FAIL / 1 INFRA_ERROR / 0 TIMEOUT / 0 CRASH，退出 2；summary 位于
  `test/output/desktop/final-gates/final-review-3222ac5-baseline/summary.json`，SHA-256
  `f06fe8bdfc0fc944cc4e39994c57b15a01dbfa874fd2d8d9187975deb3265936`。runner 起止均为
  `3222ac5`、dirty=false、provenance stable。三项 FAIL 是 RV64 固定 QEMU 9.2.4 缺失、
  LA64 `tee_device_mode`/guest nonzero 和 aggregate；LA64 clippy 在执行前因 clang 14 不支持
  `loongarch64-unknown-none` target 而严格记为 INFRA_ERROR。
- 同一 baseline 中 `make kernel-rv`、`make kernel-la` 与 `make all` 均 PASS。最终默认产物为
  RV64 2,024,576 bytes / `3a3a3f3d1a73fd58ac575004c0c92525f29f77c0ba417008f3eb74be9229a81f`，
  LA64 3,078,616 bytes / `4e9cb78e1e054c277da1cf4c66a997c5deae2d5fb86a2cbee7893d773034e0dc`；
  快照在 gate 前后均 clean。上述 gate 后只更新计划、开发日志与 Goal 状态，不改动已验证的
  实现、测试、manifest 或构建入口；文档收尾另以 diff/scope 检查验证。

## 下一阶段

2026-07-19 的人工初审报告结论为 `REQUEST CHANGES`，因此此前
`READY_FOR_HUMAN_REVIEW_DRAFT` 状态已经撤销。Checkpoint 1-8 的历史实现与失败证据继续
保留，但不能作为本轮修复后当前源码的验证结果。

本轮增加以下整改阶段：

- [x] 9A：收紧 VirtIO DMA allocation 合同并覆盖零页、长度溢出与分配失败策略；
- [x] 9B：集中管理焦点切换 damage，并用增量/全量像素一致性覆盖 create、close/minimize、
  桌面点击清焦点；
- [x] 9C：拒绝 WindowId 耗尽、修复 P6 CRLF header 边界并保护输入 release 事件；
- [x] 9D：增加固定 QEMU self-hosted 双架构 runtime CI 和可独立审计的原始证据包；
- [x] 9E：明确 Terminal 仅为受限内置命令解释器，不宣称 POSIX shell/process 语义；
- [x] 9F：运行定向验证、双架构 headless、精确干净快照 canonical gate，并记录所有非 PASS。

非目标不变：本轮不修复与桌面无关的 canonical baseline/official 缺陷，不弱化 runner、
parser、断言或结果分类，也不把环境阻塞写成 PASS。

## 完成状态

完成本轮修复后，本计划最多恢复到：

`READY_FOR_HUMAN_REVIEW_DRAFT`

2026-07-19 Checkpoint 9 已达到上述 Draft 人工复审候选状态：初审 B-01、M-01～M-03、
N-01～N-04 的代码、测试、CI 或证据整改均已落地；精确验证快照提交
`cb42268a9d7f47fbd10f8b0a5af80026712829ca`、tree
`91806ae2044a934829cdf8384cc0c9986a7aef3d` 在验证开始时与 live 预期 25 个文件逐字节一致，
运行前后均 clean。门禁结束后只更新本计划、开发日志、README 的证据说明和忽略的 Goal 状态，
实现、测试、CI 与证据脚本未再改变。该快照的 canonical quick 为 45/45 PASS，summary SHA-256 为
`12a17e1cd620e2a85e51d1375d29ea3489d37484d6ff5aabb617fc6680b863e1`；RV64/LA64 的
boot、launcher、overlap、applications、resize 共 10 个场景均生成 PASS summary、完整原始
review package，并由人工检查转换后的帧没有黑屏、空白、明显越界或 resize 后控件丢失。

这不是 PR Ready 或 merge-ready。相同快照的 `full --arch all` 计划 59、完成 59、实际启动
56，结果为 52 PASS / 4 FAIL / 3 INFRA_ERROR，退出 2，summary SHA-256 为
`c9d0db5278c439a784221f5dc4d693be91849cc92291dd428b52cb12f2b9c6b3`。非 PASS 包括：
RV64 固定 QEMU 9.2.4 缺失、LA64 既有 `tee_device_mode` guest nonzero、二者的 aggregate、
嵌套验证目录被父工作区误识别导致的 cargo fmt FAIL、clang 14 缺 LoongArch64 bare-metal target，
以及两份官方镜像不可用。live 根目录的 `cargo fmt --all -- --check` 已独立 PASS，但不改写
canonical 记录。B-02 因此继续是合并阻断，必须保持 Draft 并进入第二轮人工审查。

## Checkpoint 10：第二轮人工审查整改

第二轮人工审查报告为
`docs/human_reviews/OrayS-Desktop-第二轮人工审查复核报告-2026-07-19.md`。该报告是只读审查
输入，不以修改报告文字代替整改。当前阶段撤销此前 Draft 候选状态，并增加以下工作：

- [x] 10A：阻止公开 `pull_request` 在持久 self-hosted runner 执行不可信代码；runtime 只允许
  受信任分支 push 或受控 `workflow_dispatch`；
- [x] 10B：RV64/LA64 使用 `fail-fast: false` matrix 独立运行，单架构失败不取消另一架构；
- [x] 10C：无论 build、boot、input、capture 或 summary 在何处失败，都生成白名单 review package，
  保留明确 FAIL summary、serial、QMP transcript 与 runtime metadata，并排除磁盘镜像和 socket；
- [x] 10D：新增 package-aware 专用 validator；移动 package 后仍完整复验语义、相对 frame 绑定与
  双层哈希，篡改或路径逃逸必须 fail-closed；
- [x] 10E：记录运行前后 commit/status 与 `provenance_stable`；来源变化时 summary/package 必须 FAIL；
- [x] 10F：增加 DMA 成功清零的直接 helper 单测；评估并覆盖输入队列全 release/状态恢复加固；
- [x] 10G：运行 Python/Rust 定向测试、三目标 lint、双架构构建/headless、scope、diff 和 canonical
  gate，保留所有首次失败与外部阻塞；
- [x] 10H：更新开发日志、计划和 Goal 状态，形成第三轮人工审查材料。

外部状态不在本地源码整改权限内：workflow push、真实 GitHub Actions、required checks、固定 RV64
QEMU 9.2.4、官方 RV64/LA64 镜像以及 canonical/official 既有失败或正式豁免。未获得真实证据前
不得勾选相应第三轮条件，也不得把本地静态检查称为远端 CI PASS。

### Checkpoint 10 当前实现

- S-01：PR 仍只在 GitHub-hosted `desktop` job 执行；持久 self-hosted `desktop-runtime` 的
  job-level 条件仅接受受控 `workflow_dispatch`，或两个受信任分支的 `push`。runtime 使用
  RV64/LA64 `fail-fast: false` matrix，固定要求 QEMU 9.2.4，并按架构 `always()` 上传过滤包。
- M-04：package schema 3 同时保存原始 QMP screendump 绝对路径和包内 `frame.ppm` 相对绑定；
  `validate-review-package.py` 验证精确文件集、双层哈希、原始 transcript 绑定、截图语义和
  provenance。移动后的 PASS 包、路径逃逸和移动后篡改均有直接回归。
- M-05：runner 在创建 run 目录后安装 exit finalizer；build、boot、input、capture、QEMU 异常和
  summary 失败均保留 FAIL summary 与白名单 package。cleanup 对每个非空 QEMU PID 都执行
  `wait`，即使进程已经退出，也保留真实 exit code；诊断演练已证明 exit 42 不会被记成 0。
  工具版本采集失败写入 `collection_errors`，缺 rustc/cargo 的回归仍生成 `VALID_FAIL`，不再以
  finalizer exit 70 丢失整包。
- P-01：metadata schema 3 记录 source commit/status/dirty 的 before/after 值与
  `provenance_stable`；来源变化、采集错误或字段不一致均使 summary/package FAIL。
- T-01/N-04：生产 DMA 分配通过安全 helper 清零完整 checked slice，直接测试以 `0xa5` 填充两页后
  检查每个字节为零；全 release 队列优先按 key/button 身份合并同一 release，保留不同身份；
  无法保留所有 release 时发出不可被普通事件驱逐的 `StateReset`，translator 清除 modifier/pointer
  暂态，`WindowedDesktop` 在 shell 分发前无条件取消 drag；容量 0/1 与重复 resync 均有回归。

一次实现收尾前的精确干净快照 `ec697a8449e00939883e44fb550913159154e051` 已证明 canonical
quick 45/45 PASS；`full --arch all` 为 53 PASS / 3 FAIL / 3 INFRA_ERROR，较第二轮报告多一项
PASS：把快照放到 `/tmp` 后 `baseline.cargo_format` 已真实通过。该快照随后暴露并促成 QEMU
退出回收与缺工具 metadata 两项加固，因此只保留为中间证据，不充当最终 current-source gate。

### Checkpoint 10 最终证据与停止状态

最终实现、测试、workflow 与证据脚本在 `/tmp` 干净快照提交
`7b46e94b70437490aca0e8ab1d000c35bc4bef46` 完成验证。建快照时 29 个本轮变更/新增路径与
live 工作树逐文件一致；快照在所有 gate 和最终 runtime 前后均为 clean，runner 记录的
before/after commit 相同、dirty=false、`provenance_stable=true`。gate 后只更新本计划、开发日志
和 Goal 状态，没有修改已经验证的实现、测试、workflow 或证据脚本。

- canonical quick：45/45 PASS、exit 0、294.533 秒；summary SHA-256
  `23c7eb5aed851aebed0b97806ae063a97c31ea0622f31d2735f24f9f50611982`。
- canonical `full --arch all`：计划/完成 59、实际启动 56，53 PASS / 3 FAIL /
  3 INFRA_ERROR、0 timeout/crash、exit 2、1,281.746 秒；summary SHA-256
  `656448c74379cc330260aa849d8ce9c11f0d88a9e91645948439fba4b9184b25`。
- full 的 3 FAIL 为 RV64 semantic evidence（系统 QEMU 6.2.0 不满足固定 9.2.4）、LA64
  semantic evidence（真实 `PR3_SMOKE_V1 USER_FAIL tee_device_mode`）及派生 aggregate；
  3 INFRA_ERROR 为 clang 14 不认识 `loongarch64-unknown-none`，以及 RV64/LA64 官方镜像缺失。
  `baseline.cargo_format`、workspace tests、默认/RV64 clippy、双架构 kernel 和 submission build
  均为 PASS，没有用成功重跑覆盖任何非 PASS。
- 最终快照又独立执行 Desktop runner：LA64 QEMU 9.2.4 boot 为 PASS，移动到 live 忽略目录后
  validator 返回 `VALID_PASS failures=0`；1280x800 PPM SHA-256 为
  `406af959897dcba1f2a770c20e5139416ed8937e6a22d98a21af0b8e58e5e7f2`，转换观察副本已实际
  查看，无黑屏、异常裁切或文字重叠。RV64 在固定版本前置门禁 exit 3，生成 7 条明确失败的
  `VALID_FAIL` package；移动后仍复验成功。
- 最终 canonical 和 Desktop runtime 证据分别保存在忽略目录
  `test/output/desktop/round3-final-canonical/` 与
  `test/output/desktop/round3-final-runtime/`。前者保留 quick/full summary、日志和 PR3
  semantic evidence；后者排除了 runtime 磁盘和 socket，只保留可审计证据与 package。

第二轮报告“第三轮最低通过条件”的逐项状态如下：

| 条件 | 状态 | 当前证据 |
| --- | --- | --- |
| 1. self-hosted 信任边界 | 本地已完成，待远端复核 | PR 不进入持久 self-hosted runtime；仅受控 dispatch/受信任 push。 |
| 2. package 可迁移复验 | 已完成 | 最终 PASS/FAIL package 移动后分别 `VALID_PASS`/`VALID_FAIL`。 |
| 3. PASS/FAIL 均上传证据 | 本地 workflow/runner 已完成，待远端实跑 | `always()` 分架构上传过滤包；真实失败 finalizer 已演练。 |
| 4. 两架构独立执行 | 本地已完成，待远端实跑 | `fail-fast: false` matrix；LA 在 RV 失败后仍独立运行。 |
| 5. workflow push/真实运行 | 未满足 | live workflow 尚未提交或 push；未经授权不执行远端写操作。 |
| 6. required checks | 未满足 | 需要 GitHub 管理员配置并提供可审计状态。 |
| 7. RV64 固定 QEMU 9.2.4 | 本地已满足，待远端实跑 | 2026-07-20 本机 RV/LA 均精确为 9.2.4；最终 clean snapshot 两架构各五场景均 `VALID_PASS`。 |
| 8. canonical/official 关闭或豁免 | 未满足 | full 如实保留 3 FAIL/3 INFRA_ERROR，尚无维护者豁免。 |
| 9. 双架构 official 实跑 | 未满足 | 两张官方镜像缺失，均严格记为 INFRA_ERROR。 |
| 10. PR base | 部分可证、未满足远端条件 | 当前分支 merge-base 是目标基线 `c776ceff…`；尚未创建 PR，不能声称 PR base 已配置。 |

因此本地可修复项已完成并可进入第三轮材料审阅，但报告定义的第三轮最低条件尚未全部满足。
本轮按用户要求不设置、恢复或完成 Goal；当前状态不是 `READY_FOR_HUMAN_REVIEW_DRAFT`、PR Ready
或 merge-ready。后续需要明确的 push/建 PR 授权、GitHub 管理员配置、远端固定 QEMU 实跑、
官方镜像，以及 canonical 非 PASS 的修复或正式豁免。

### Checkpoint 11：最新独立复审 evidence/input 整改

本 checkpoint 以 `/tmp/codex-evidence-fix-prompt.txt` 为只读复审输入，停止沿用旧验证路径，
严格先补失败测试再修实现。没有修改 `tee_device_mode`、两份人工审查报告、GitHub 远端状态或
Goal，也没有在 live 分支 commit/push。

- evidence schema 明确升级为 metadata 3、summary 2、package 3；不对旧 schema 静默兼容。
- `qemu_started=false` 的合法记录必须同时具有非零 runner exit、null QEMU exit 和受约束的
  pre-start stage/reason，且语义结果必为 FAIL；非法组合在 summarizer、packager、validator
  全部 fail-closed，不能先输出 PASS/VALID_PASS。
- QEMU policy 固定为精确 `QEMU emulator version 9.2.4`。环境变量只能重复该值，不能改变
  PASS policy；`9.2.40`、`9.2.4-rc0`、`6.2.0` 与非 canonical required value 均有负测。
- QMP socket 由专用 helper 固定创建在 `/tmp/orays-qmp.*`，检查 AF_UNIX 107-byte 上限及
  QEMU 参数分隔符安全，不再继承任意 `TMPDIR`；INT/TERM 通过受约束 `runner-signal` 状态进入
  exit finalizer，cleanup 回收 QEMU/socket/runtime dir。
- dirty provenance 负测使用自洽的 before/after dirty status 与
  `provenance_stable=false`；搬迁测试先移走 package、再移走原 run，确认原路径不存在后验证。
- input 容量 0/1、pending reset、重复 resync 和 shell-dispatch-before-reset 均已补齐测试；
  容量 1 无法保留新 release 时，dropped 同时计入旧输入与新输入，内部 reset 不计为用户输入。

当前源码验证：Desktop Python 65/65、host Rust 73/73、host clippy `-D warnings`、双 rustfmt、
asset、golden 5/5、scope 115/3/3/74、`git diff --check`、RV/LA release build 均 PASS。最终
clean snapshot `3a0fade10f542908dc689faf0a4eaf1d85a56b02` 与 live 逐文件一致；RV/LA 各五场景
共 10 个 runner/package 原地 PASS，移走原 run 后 10 个 relocated package 再次全部
`VALID_PASS failures=0`。canonical quick 45/45 PASS、exit 0、300.404 秒，summary SHA-256
`d994e0256d4915864bd303515fc7e2d0db86bf8460ada107aab4d0a492e54426`，runner 前后 clean、
commit/status 相同、provenance stable。证据保存在忽略目录
`test/output/desktop/evidence-fix-final/`。

过程失败保留：第一次 Python 定向调用使用错误包路径导致 ImportError；一次 apps 定向调用漏掉
既有 `host-tools` feature，属于无效调用；RV/LA 并行首次创建共同输出父目录时 LA boot 遇到
`FileExistsError`，该次没有启动 guest，父目录就绪后单独重跑才得到真实 PASS。两张官方镜像仍
不可读，GitHub workflow/required check 未实跑，第三轮独立人工审查尚未进行，因此仍不可称
PR Ready、merge-ready 或完整 `full --arch all` PASS。
