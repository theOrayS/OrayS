# ADR 0001：桌面子系统与默认内核基础设施隔离

状态：Accepted

日期：2026-07-18

## 背景

OrayS 当前稳定化分支仍有已知缺陷。桌面开发不能扩大这些问题，也不能改变默认提交和官方评测路径。

## 决策

1. 桌面放在独立 `user/desktop` workspace。
2. 不加入根 Cargo workspace。
3. 使用独立 `Cargo.lock`。
4. 使用 `Makefile.desktop`，不修改根默认 Makefile。
5. 桌面默认关闭。
6. 所有已有代码桥接 feature-gated。
7. 既有 bridge 文件最多 8 个、总变更最多 250 行。
8. 使用自动 scope checker。
9. 本 PR 使用单进程桌面和内置应用。
10. 多进程 GUI 协议放到后续 PR。
11. VirtIO GPU 与 input 驱动都归属桌面本地平台层；默认关闭的
    `axdriver/desktop-device-hook` 只复用既有总线发现生命周期，通过两个固定 C ABI 探测
    符号传递 MMIO range，或在同步调用期间独占借用当前 PCI root 与设备信息；不在通用
    驱动层保存桌面设备、transport 或 framebuffer 状态。MMIO hook 只构造一次 transport
    并按设备类型移交；PCI hook 不重建 ECAM root。input 只消费总线 hook 已登记的设备，不在
    应用启动后重新扫描总线；固定容量 registry 必须在 `InputDriver`/PCI transport 初始化前
    预留槽位，初始化失败时撤销预留。

## 后果

优点：

- 默认内核路径稳定；
- 桌面可独立回滚；
- 便于 headless CI；
- 降低与稳定化修复冲突。

代价：

- 部分能力需要适配层；
- 通用 GUI ABI 延后；
- 应用暂时与桌面进程同生命周期。
- 独立桌面链接必须提供两个 device hook 符号；默认非桌面构建不启用 feature，因此没有
  链接或运行时影响。
