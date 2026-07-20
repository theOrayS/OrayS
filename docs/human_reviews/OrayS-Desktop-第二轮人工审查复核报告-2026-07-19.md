# OrayS Desktop 第二轮人工审查复核报告

日期：2026-07-19
审查对象：`feature/orays-desktop-environment` 第二轮人工审查整合包
建议结论：**部分通过，继续保持 Draft；当前整改内容不应原样提交并进入 Ready/合并流程**

---

## 一、结论摘要

第二轮整改质量明显高于第一轮：

- 第一轮 B-01、M-01、N-01、N-02、N-03 已得到可信修复；
- N-04 的主要目标已经实现，可关闭原问题，但建议保留输入状态恢复的后续加固项；
- 第一轮 M-03“缺少原始证据”的问题已经解决；
- 外层 `SHA256SUMS` 全部校验通过；
- 10 组场景均补齐 serial、QMP、input sequence、原始 PPM、metadata、summary 和双层哈希；
- 10 张 PNG 与对应 `frame.ppm` 的 RGB 像素逐字节一致；
- 10 个场景均来自同一验证快照 `cb42268a9d7f47fbd10f8b0a5af80026712829ca`，metadata 均记录 `source_dirty=false`。

但仍存在以下阻断项：

1. **B-02 未关闭**：canonical full 仍为 52 PASS / 4 FAIL / 3 INFRA_ERROR。
2. **新增安全 Blocker**：公开仓库的 `pull_request` 工作流直接运行持久化 self-hosted runner。
3. **M-02 尚未闭环**：CI 代码尚未 push、required check 尚未配置，也没有真实 GitHub Actions 运行结果。
4. **新增证据 Major**：打包后的 review package 无法在新目录中通过现有语义验证器。
5. **新增失败证据 Major**：runtime 失败时脚本会在打包前退出，且 RV64 失败会阻止 LA64 执行。

因此推荐选择：

> **部分通过；按备注继续修改**

而不是：

> 第二轮整改复核通过；允许直接提交/push

---

## 二、审查包完整性复核

### 2.1 外层完整性

已执行：

```bash
sha256sum -c SHA256SUMS
```

结果：所有列入清单的代码快照、patch、原始证据、截图、门禁摘要和说明文档均为 `OK`。

### 2.2 场景证据完整性

10 个场景均包含：

- `serial.log`
- `qmp-input.jsonl`
- `qmp-capture.jsonl`
- `input-sequence.json`
- `frame.ppm`
- `capture-precondition.json`
- `display-geometry.txt`
- `summary.json`
- `runtime-metadata.json`
- `hashes.sha256`
- `review-package.json`
- `package-files.sha256`
- resize 场景的 `vnc-resize.json`

独立复核结果：

- `package-files.sha256`：10/10 通过；
- `review-package.json.files` 与 `package-files.sha256`：10/10 完全一致；
- `summary.screenshot.sha256` 与实际 `frame.ppm`：10/10 一致；
- `summary.result`：10/10 为 PASS；
- `qemu_exit`：10/10 为 0；
- `summary.failures`：10/10 为空；
- source commit：10/10 相同；
- source dirty：10/10 为 false。

### 2.3 PNG 与原始 PPM

使用无损 RGB 比较核验：

| 场景 | PNG 尺寸 | PPM 尺寸 | 像素一致 |
|---|---:|---:|---|
| RV64 boot | 1024×768 | 1024×768 | 是 |
| RV64 launcher | 1024×768 | 1024×768 | 是 |
| RV64 overlap | 1024×768 | 1024×768 | 是 |
| RV64 applications | 1024×768 | 1024×768 | 是 |
| RV64 resize | 900×650 | 900×650 | 是 |
| LA64 boot | 1280×800 | 1280×800 | 是 |
| LA64 launcher | 1280×800 | 1280×800 | 是 |
| LA64 overlap | 1280×800 | 1280×800 | 是 |
| LA64 applications | 1280×800 | 1280×800 | 是 |
| LA64 resize | 900×650 | 900×650 | 是 |

视觉上未发现黑屏、坏帧、明显裁切、主控件缺失或跨架构语义不一致。

---

## 三、第一轮问题关闭建议

| 编号 | 第二轮结论 | 审查意见 |
|---|---|---|
| B-01 DMA HAL 合约 | **关闭** | 零页、乘法溢出、OOM 均 fail-fast，不再返回 dangling；成功路径保留全量清零 |
| B-02 canonical/official | **继续阻断** | full 仍有 4 FAIL、3 INFRA_ERROR |
| M-01 焦点 damage | **关闭** | 新增统一 `set_focused`，create/close/minimize/focus/Desktop/Alt-Tab/modal 路径已接入，并有 incremental/full 像素等价测试 |
| M-02 QEMU CI | **继续整改** | 代码尚未 push/实跑；并存在公开仓库 self-hosted runner 安全问题 |
| M-03 缺少原始证据 | **关闭原问题** | 原始文件和哈希链已补齐；但另开“包不可迁移验证”新 Major |
| N-01 WindowId 回绕 | **关闭** | 使用 `Option<u32>`，最终 ID 仅发放一次，随后返回 `IdExhausted` |
| N-02 P6 CRLF | **关闭** | 精确消费 CRLF，同时保留首像素为空白字节的语义 |
| N-03 Terminal 边界 | **关闭** | README、模块注释和界面均明确其为进程内 builtin interpreter |
| N-04 输入溢出 | **关闭原问题，保留加固项** | release 优先、drop 计数和增量串口诊断已实现；建议未来增加全 release 队列及状态复位测试 |

---

## 四、仍然存在的 Blocker

### B-02：完整门禁未通过

第二轮记录：

- quick：45/45 PASS；
- full：52 PASS / 4 FAIL / 3 INFRA_ERROR。

仍未通过：

- `evidence.riscv64`
- `evidence.loongarch64`
- `evidence.aggregate`
- `baseline.cargo_format`

基础设施错误：

- LoongArch clang target capability 缺失；
- `/root/sdcard-rv.img` 缺失；
- `/root/sdcard-la.img` 缺失。

特别是 LoongArch evidence 的 guest nonzero 并非单纯“没有环境”即可忽略。官方双架构 case 尚未执行时，不能批准合并。

### S-01：公开仓库 PR 直接运行 self-hosted runner

当前拟议 workflow 同时具有：

```yaml
on:
  pull_request:
```

和：

```yaml
runs-on: [self-hosted, Linux, X64, orays-desktop-qemu-9.2.4]
```

该 job 会 checkout PR 代码并执行仓库内的构建、Python 和 shell 脚本。对于公开仓库，这意味着外部 PR 中的非可信代码可能在自建机器上执行。

这应在 push workflow 之前修复。

推荐方案按优先级：

1. 使用 GitHub-hosted runner，并通过固定容器或缓存构建的方式提供 QEMU 9.2.4；
2. 使用每个 job 创建、执行后立即销毁的 ephemeral runner；
3. 如果只能使用长期 self-hosted runner，将 runtime job 限制为受信任分支的 `push` 或受控 `workflow_dispatch`，不要直接运行任意 PR head；
4. runner 不挂载宿主敏感目录，不持有长期凭据，限制网络访问，并在每次运行后销毁执行环境。

---

## 五、新发现的 Major

### M-04：Review package 复制后无法通过现有语义验证器

`qmp_screendump.py` 向 QMP 发送绝对路径，例如：

```json
{
  "execute": "screendump",
  "arguments": {
    "filename": "/root/.../original-run/frame.ppm"
  }
}
```

该绝对路径被原样写入 `qmp-capture.jsonl`。

随后 `package-review-evidence.py` 把 transcript 和 `frame.ppm` 复制到新的 review package 目录，但不调整路径。

而 `summarize-run.py` 的验证逻辑要求：

```python
Path(filename).resolve() == screenshot.resolve()
```

因此我在解压后的 10 个场景目录重新调用相同的 `validate_run` 时，10/10 均得到唯一失败：

```text
invalid capture evidence:
screendump target does not match the captured frame
```

其余 marker、QMP 顺序、input sequence、geometry、frame size、非纯色、hash 等验证均通过。

这说明：

- 原始运行目录中的验证可以通过；
- 打包文件的哈希链完整；
- 但包并不是当前脚本所宣称的“可在任意位置独立复验的 self-contained package”。

#### 修复建议

采用以下任一方案：

- metadata 中记录 `original_run_dir`，package-aware validator 同时验证原路径与包内相对路径；
- transcript 增加单独的 `evidence_relative_filename: "frame.ppm"`，保留原始 QMP command；
- 新增专用 `validate-review-package.py`，不要直接套用原始 run-dir 的绝对路径判定；
- 增加回归测试：生成 package，移动到另一个临时目录，再完整执行语义验证。

第一轮 M-03 可以关闭，但应新建 M-04，修复后才能声称 package 可独立复验。

### M-05：失败时不会生成或上传完整 runtime 证据

`run-headless-qemu.sh` 使用：

```bash
set -euo pipefail
```

结尾顺序为：

```bash
python3 summarize-run.py ...
python3 package-review-evidence.py ...
```

当 `summarize-run.py` 返回 FAIL，shell 会立即退出，第二条打包命令不会执行。更早发生 boot timeout、input timeout 或 QEMU 异常时，也不会到达打包步骤。

workflow 的 `always()` 上传路径只包括：

```text
rv-boot/review-package
la-boot/review-package
```

因此恰恰在 runtime 失败时，最需要的 serial/QMP/frame 证据可能没有 artifact。

同时 RV64 和 LA64 在同一个 shell step 中串行执行，RV64 失败会阻止 LA64 开始。

#### 修复建议

- 将 RV64、LA64 改为 matrix 或两个独立 job；
- 每个架构无论 PASS/FAIL 都执行 finalize；
- 上传经过白名单过滤的 run 目录，而不是仅上传成功后才出现的 `review-package`；
- 明确排除 `disk.img`、QMP socket、缓存和凭据；
- artifact 内必须包含失败的 summary、serial、QMP transcript 和 runtime metadata；
- 最后再根据两个架构的结果决定 job 成败。

---

## 六、Minor / 加固项

### P-01：运行元数据只记录执行前 Git 状态

`collect-runtime-metadata.py` 在 `build.sh` 之前运行，只记录：

- `source_commit`
- `source_dirty`
- `source_status`

没有在 build、QEMU 和证据生成完成后再次确认 commit/status。

本轮包有其他门禁摘要间接证明快照后续保持 clean，因此不否定本轮证据；但未来 CI artifact 最好记录：

- `source_commit_before`
- `source_commit_after`
- `source_status_before`
- `source_status_after`
- `provenance_stable`

如果运行期间 source 发生变化，package 必须 FAIL。

### T-01：B-01 的测试描述略有夸大

当前有确定性的测试覆盖：

- zero pages panic；
- size overflow panic；
- allocator failure panic。

成功路径的全量清零由代码和真实 VirtIO 场景间接覆盖，但没有一个直接验证“分配缓冲区每个字节均为零”的单元测试。

不影响关闭 B-01，但文档应避免写成“成功清零已有直接单测”，或者补一个可注入 allocator/缓冲区 helper 测试。

### N-04 后续加固

当前队列在全部槽位均为 release、又到来新的 release 时，仍必须淘汰一个旧 release。极端情况下不能从队列策略本身形式化保证所有按键和拖拽状态最终释放。

当前主循环每轮最多抓取 64 个 raw event，而队列容量为 128，并会在下一轮前排空，因此现实路径风险较低。建议后续增加：

- 按键/按钮身份感知的 release 去重；
- overflow 后的输入状态重同步；
- “拖拽开始—输入洪泛—release—最终 pointer operation 清除”的端到端测试。

---

## 七、建议填写第二轮签收表

### 整改项

| 编号 | 建议勾选 |
|---|---|
| B-01 | 关闭 |
| B-02 | 仍阻断 |
| M-01 | 关闭 |
| M-02 | 继续整改；required 未配置；CI 未实跑 |
| M-03 | 关闭第一轮原问题，同时新增 M-04 |
| N-01 | 关闭 |
| N-02 | 关闭 |
| N-03 | 关闭 |
| N-04 | 关闭原问题，记录后续加固 |

### 最终结论

建议勾选：

```text
[✓] 部分通过；按备注继续修改
```

不建议勾选：

```text
[ ] 第二轮整改复核通过；允许提交/push，继续 Draft 审查
```

### Git 决策

当前建议：

- 不要把现有 self-hosted `pull_request` workflow 原样 push；
- 先修复 S-01、M-04、M-05；
- 修复后整理为独立提交；
- push 后由仓库管理员配置 required checks；
- 获取真实 GitHub Actions 双架构结果；
- 继续保持 Draft；
- B-02 关闭前不得 Ready 或合并。

---

## 八、下一轮最低通过条件

第三轮复核至少需要：

1. self-hosted runner 信任边界得到安全处理；
2. review package 移动目录后可由专用 validator 完整复验；
3. runtime PASS/FAIL 均能上传证据；
4. RV64、LA64 独立执行，一个失败不阻止另一个；
5. 新 workflow 已 push 并真实运行；
6. required checks 已由管理员配置；
7. RV64 也使用固定 QEMU 9.2.4 重新生成场景证据；
8. canonical/official 阻塞逐项关闭或获得正式、可审计的维护者豁免；
9. 官方 RV64/LA64 镜像 case 实际执行；
10. PR base 保持 `develop/post-integration-next`，不得误对 main。
