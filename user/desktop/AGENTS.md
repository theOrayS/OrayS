# OrayS Desktop 子系统开发规则

本文件适用于 `user/desktop/` 及其全部子目录。

根 `AGENTS.md` 中的 Git 安全、测试诚信、双架构、AI 披露和可追溯性要求仍然有效。

## 1. 子系统定位

OrayS Desktop 是默认关闭、独立构建的图形桌面应用。

本 PR 使用：

- 单桌面进程；
- 内置窗口管理器；
- 内置桌面应用；
- 独立 Cargo workspace；
- 独立 lockfile；
- 独立 Makefile 和 CI。

本 PR 不建立通用多进程 GUI ABI。

## 2. 架构边界

必须分层：

1. `platform/`
   - display；
   - input；
   - time；
   - filesystem；
   - system information。
2. `graphics/`
   - Surface；
   - geometry；
   - painter；
   - text；
   - image；
   - shadow；
   - damage。
3. `desktop/`
   - compositor；
   - window manager；
   - shell；
   - Dock；
   - top bar；
   - launcher；
   - notifications；
   - shortcuts；
   - theme。
4. `widgets/`
   - 可复用控件。
5. `apps/`
   - 终端；
   - 文件管理器；
   - 编辑器；
   - 图片查看器；
   - 系统监视器；
   - 设置。

上层模块不得直接调用内核驱动。

所有 OrayS 能力访问必须经过 `platform/`。

## 3. 内核隔离

优先使用现有：

- `axstd`；
- `arceos_api`；
- `axdisplay`；
- 已有任务、时间和文件系统接口。

修改桥接文件前必须：

1. 在 ADR 中写明缺失能力；
2. 证明桌面子树无法自行实现；
3. 保持 feature-gated；
4. 保持默认 feature graph 不变；
5. 添加 feature 开启和关闭测试；
6. 运行 scope checker；
7. 检查双架构。

禁止修改：

- `user/shell/**`；
- POSIX/Linux ABI；
- 官方评测；
- 根 workspace；
- 根默认 Makefile；
- 根工具链。

## 4. 无显示服务器

不得依赖实时 GUI。

视觉和交互验证使用：

- 内存 framebuffer；
- PPM/PNG；
- localhost-only VNC；
- QMP；
- screendump；
- 输入注入；
- serial marker；
- image hash。

所有 QEMU、截图、输入和日志脚本必须能在：

```text
DISPLAY 未设置
WAYLAND_DISPLAY 未设置
```

的环境中运行。

不得要求安装完整 Linux 桌面。

## 5. 图形实现规则

必须：

- 处理 stride；
- 处理 pixel format；
- clipping；
- 防止整数溢出；
- 双缓冲；
- dirty rectangle；
- 统一 frame commit；
- 缓存字体、图标和阴影；
- 空闲时等待或睡眠；
- 支持分辨率变化。

禁止：

- 控件自行 flush；
- 无条件整屏持续重绘；
- 未裁剪的像素访问；
- 仅靠截图伪装实际交互；
- 实时全屏高斯模糊。

## 6. 功能真实性

所有应用必须读取或修改真实 OrayS 状态。

禁止：

- 固定文件列表；
- 假进程；
- 随机 CPU/内存数据；
- 固定成功返回；
- 为录屏写死操作路径；
- 用预渲染整屏图片代替桌面。

没有对应内核指标时，显示“不支持”，并记录限制。

## 7. 依赖与资产

所有依赖必须：

- 固定版本或 commit；
- 能离线重建；
- 记录来源；
- 记录许可证；
- 记录必要的版权和 NOTICE；
- 不更新根 Rust 工具链；
- 不要求宿主机 GUI 库。

字体、图标、壁纸和图片分别记录许可证。

不得提交不明来源资产。

## 8. 测试

至少覆盖：

- geometry；
- clipping；
- alpha blend；
- stride；
- damage merge；
- 窗口状态机；
- z-order；
- focus；
- move/resize；
- 输入序列；
- 文件错误；
- 图片错误；
- asset/license；
- golden frame；
- RV64 build；
- LoongArch64 build；
- headless QEMU smoke；
- 默认非桌面回归。

输出统一放在：

```text
build/desktop/
test/output/desktop/<run-id>/
```

## 9. 提交

每个 checkpoint 可创建本地 commit。

禁止：

- force-push；
- 改写基线；
- 无关重构；
- 批量格式化；
- 修复桌面范围外 Bug；
- 把全部实现压成一个不可审查的 commit。

每次提交前：

```bash
git diff --check
python3 scripts/desktop/check-scope.py
```
