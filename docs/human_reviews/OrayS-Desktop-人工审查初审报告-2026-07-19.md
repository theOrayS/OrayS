# OrayS Desktop 分支人工审查初审报告

- 审查日期：2026-07-19
- 仓库：`theOrayS/OrayS`
- 分支：`feature/orays-desktop-environment`
- 审查目标：判断该分支是否适合创建 Draft PR、标记 Ready、合并
- 审查结论：**REQUEST CHANGES；可继续 Draft 审查，不可 Ready，不可合并**

## 1. 审查范围与证据

本轮同时检查了：

1. GitHub 当前分支相对 `main` 和基线提交 `c776ceff40587de0fa0547724d0abfecbb56cc64` 的变更范围；
2. 上传的 `Desktop人工审查包(1).zip`；
3. Desktop 高风险代码：VirtIO/DMA、显示与动态分辨率、输入注册、文件系统、窗口管理器、compositor；
4. Desktop CI、离线构建、QEMU headless 与证据汇总脚本；
5. 审查包中的 10 张 RV64/LA64 场景截图。

审查包 `SHA256SUMS` 校验通过。抽样比对的以下审查包源文件与 GitHub 当前分支 blob 一致：

- `user/desktop/src/platform/virtio.rs`
- `user/desktop/src/platform/display.rs`
- `user/desktop/src/platform/input.rs`
- `user/desktop/src/platform/filesystem.rs`
- `user/desktop/src/desktop/window_manager.rs`
- `user/desktop/src/desktop/compositor.rs`

限制：当前审查环境无法直接克隆并重新执行整个仓库；上传包也没有包含原始 `serial.log`、`qmp-input.jsonl`、`qmp-capture.jsonl` 和 `frame.ppm`。因此本轮能确认代码和摘要结构，但不能独立重放全部运行证据。

## 2. 分支范围结论

GitHub compare 显示：

- 相对 `main`：Desktop 分支领先 66 个提交；
- 相对 `c776ceff40587de0fa0547724d0abfecbb56cc64`：只领先 3 个提交；
- 上传包中的工作树上游为 `origin/develop/post-integration-next`。

因此 Desktop PR 的正确审查基线应是 `develop/post-integration-next`（或精确的 `c776...` 基线），不能直接以当前 `main` 为 base。否则 PR 会夹带大量 Linux ABI、文件对象、测试架构等非 Desktop 改动，人工审查范围会失真。

## 3. 阻断问题

### B-01：VirtIO HAL 在分配失败时返回悬空指针

**级别：Blocker / 内存安全**

位置：`user/desktop/src/platform/virtio.rs:22-35`

当前逻辑在以下情况返回：

```rust
(0, NonNull::dangling())
```

- `pages * 4096` 溢出；
- `pages == 0`；
- `global_allocator().alloc_pages(...)` 失败。

但是 `virtio_drivers::Hal::dma_alloc` 是 unsafe trait 合约，要求返回值中的 `NonNull<u8>` 必须是有效、页对齐、独占且保持有效到 `dma_dealloc` 的 DMA 分配。该接口没有 `Result`，`dangling` 不是规定的失败表示。

风险：驱动可能继续创建 virtqueue/GPU/input DMA 对象，并对物理地址 0 或悬空虚拟地址读写，把普通 OOM 转化为 DMA 非法访问、内存破坏或不可预测崩溃。

必须修改为：

- 明确拒绝 `pages == 0`；
- 溢出或 OOM 时 panic/abort，或使用能够保证成功的 DMA 池；
- 只在获得真实分配后返回；
- 保留完整清零；
- 增加针对零页、溢出、模拟分配失败策略的合约测试或设计说明。

说明：仓库已有的旧 VirtIO HAL 也存在类似返回模式，不代表新实现安全；本次分支新增了第二份 unsafe HAL，不能继续复制违反 trait 合约的行为。

### B-02：Canonical / official 完整门禁尚未通过

**级别：Merge blocker / 流程与回归风险**

上传审查包记录：

- Host Rust：53/53 PASS；
- Python：35/35 PASS；
- Desktop RV64/LA64 headless：10/10 摘要 PASS；
- 主 Quick：45/45 PASS；
- 独立 reviewer Quick：44 PASS / 1 TIMEOUT；
- canonical baseline：53 PASS / 3 FAIL / 1 INFRA_ERROR；
- RV evidence 要求 QEMU 9.2.4，环境解析到 6.2.0；
- LoongArch clang 14 不支持目标；
- `official/full --arch all` 未通过。

这些状态足以支持 Draft 审查，不足以支持 Ready 或 merge。必须把“代码缺陷”和“基础设施不满足”分别关闭或形成明确、批准过的豁免记录。

## 4. 主要问题

### M-01：焦点切换没有完整标记 damage，增量渲染会留下错误标题栏状态

**级别：Major / 可见功能错误**

相关位置：

- `user/desktop/src/desktop/window_manager.rs:275-294`：创建窗口只 damage 新窗口，直接设置新焦点；
- `window_manager.rs:297-325`：关闭后直接选择新焦点，没有 damage 新焦点窗口；
- `window_manager.rs:351-360`：最小化后同样没有 damage 新焦点窗口；
- `window_manager.rs:467-505`：点击桌面直接 `focused = None`，没有 damage 原焦点窗口；
- `user/desktop/src/desktop/compositor.rs:105-154`：焦点决定标题栏和边框颜色。

最小复现：

1. 创建窗口 A，并完成一次全量/增量 present；
2. 清空 damage；
3. 创建一个与 A 不重叠的窗口 B；
4. 逻辑焦点变为 B，但 damage 只有 B；
5. compositor 不会重绘 A，A 的标题栏仍保留 active 颜色；
6. 结果是 A、B 视觉上可能同时处于激活状态。

关闭/最小化当前焦点窗口、点击桌面清除焦点也存在同类问题。

建议集中实现：

```text
set_focused(new_focus)
  记录旧焦点 decorated/title bounds
  记录新焦点 decorated/title bounds
  更新 focused
  将旧、新区域加入 damage
```

并让 `create`、`close`、`minimize`、`focus`、点击 Desktop、modal/Alt-Tab 路径统一调用。

必须补充三类像素等价测试：

- 创建第二个不重叠窗口后，incremental == fresh full composition；
- 关闭/最小化焦点窗口后，incremental == full；
- 点击桌面清除焦点后，incremental == full。

现有测试中的 focus + drag 会因窗口移动产生大范围 damage，可能偶然覆盖标题栏，因此不能证明单独焦点切换正确。

### M-02：真实 QEMU 场景没有进入当前 Desktop CI

**级别：Major / 自动化回归风险**

`.github/workflows/desktop.yml` 当前执行：

- 资产与许可证；
- host Rust/Python tests；
- fmt/clippy；
- host golden；
- RV64/LA64 cross-build；
- scope 检查。

它没有调用 `scripts/desktop/run-headless-qemu.sh`。虽然 Makefile 提供 `desktop-rv-smoke` / `desktop-la-smoke`，但 PR CI 变绿并不代表 GPU/input/QMP/VNC/动态 resize 的真实 guest 路径通过。

建议：

- 在固定 QEMU 版本的 canonical/self-hosted runner 中建立 required runtime job；
- 至少运行两架构 boot smoke；
- launcher、overlap、applications、resize 可放入 required、nightly 或手动发布门禁，但 Ready 前必须有同一 commit 的完整记录；
- 上传原始证据为 GitHub Actions artifact。

### M-03：人工审查包缺少原始运行证据

**级别：Major for final sign-off / 证据完整性**

当前包包含：

- `summary.json`；
- `hashes.sha256`；
- display geometry；
- resize/capture precondition；
- 转换后的 PNG 截图。

但哈希表指向的原始文件没有一并提供。审查者无法独立确认：

- serial marker 是否唯一且顺序正确；
- QMP 命令、响应是否完整成功；
- 输入序列是否与 transcript 完全一致；
- PNG 是否来自被哈希的同一个 `frame.ppm`；
- summary 是否由同一 commit、同一次运行生成。

最终签收包至少应包含每个场景的：

- `serial.log`
- `qmp-input.jsonl`
- `qmp-capture.jsonl`
- `input-sequence.json`
- `frame.ppm`
- `summary.json`
- `hashes.sha256`
- resize/capture precondition 文件
- commit SHA、QEMU 版本、工具链版本和生成命令

## 5. 次要问题与建议

### N-01：WindowId 溢出后可能复用仍在使用的 ID

位置：`window_manager.rs:287-288`

`checked_add(1).unwrap_or(1)` 在 `u32::MAX` 后回到 1，可能与现存窗口冲突。实际触发概率很低，但破坏 ID 唯一性不变量。建议增加 `WindowError::IdExhausted`，或寻找未占用 ID，不能静默复用。

### N-02：P6 PPM 对 CRLF 分隔的兼容性不足

位置：`user/desktop/src/graphics/image.rs:204-210`

binary parser 在 maxval 后只消费一个 whitespace 字节。对于 `255\r\n`，可能只消费 `\r`，把 `\n` 当作第一个像素字节。建议明确支持 CRLF 作为一个行结束序列，并增加测试；同时不能泛化为无限跳过空白，因为二进制像素本身可能以 whitespace 值开头。

### N-03：Desktop Terminal 是受限内置命令解释器，不是通用进程 Shell

它真实调用文件系统，并明确显示不支持的命令，因此不属于 fake success；但文档和演示必须明确其边界，避免将其描述为完整 POSIX shell/终端进程模型。

### N-04：输入队列溢出策略应记录丢包指标

输入队列有容量上限。若按“丢弃最新事件”处理，极端输入洪泛可能丢失 button/key release，造成短暂拖拽或组合键状态异常。建议增加 dropped-event 计数和串口诊断；优先保留 release 事件，或在溢出后执行状态重同步。

## 6. 已通过的重点检查

以下设计在本轮没有发现阻断性问题：

1. **动态显示重配置**：先把 raw framebuffer 指针置空，再调用可能失败的 resolution change，避免失败路径继续解引用旧 DMA allocation；显示提交前也校验尺寸和缓冲区长度。
2. **有界文件读取**：在 metadata 后继续按预算读取，并探测超出上限的一个字节，可防止文件在检查后增长绕过上限。
3. **文件名拼接约束**：UI 创建/重命名拒绝空名、`.`、`..` 和 `/`。
4. **证据脚本 fail-closed 设计**：检查 QMP greeting、命令响应、guest markers、截图格式/尺寸、非纯色、QEMU 正常退出和 resize 顺序。
5. **系统监视器真实性**：CPU/内存不支持时明确显示 `UNSUPPORTED` 和 `NO SYNTHETIC METRICS`，没有伪造指标。
6. **CI 基础安全**：GitHub Actions 使用固定 commit SHA，权限为 `contents: read`，Cargo 构建使用 locked/offline vendor。
7. **视觉检查**：10 张 RV64/LA64 截图没有发现黑屏、空白帧、明显越界或 resize 后主要控件消失；两架构风格基本一致。

## 7. 建议的合入顺序

1. 保持 PR 为 Draft，并把 base 设置为 `develop/post-integration-next`；
2. 修复 B-01 DMA unsafe contract；
3. 修复 M-01 focus damage，并增加 full-vs-incremental 回归测试；
4. 生成同一 head commit 的完整原始 QEMU evidence artifact；
5. 把至少双架构 boot runtime smoke 纳入固定版本 CI；
6. 解决 canonical baseline / official/full 非通过项，或由维护者逐项批准并记录基础设施豁免；
7. 重新进行人工截图、窗口交互、文件操作和证据链签收；
8. 全部门禁通过后再从 Draft 转 Ready。

## 8. 当前签收意见

- [x] 允许继续代码修改和 Draft PR 人工审查
- [ ] 不同意标记 Ready
- [ ] 不同意合并
- [ ] 不同意将当前 10/10 摘要替代 canonical official/full

最终意见：**REQUEST CHANGES**。
