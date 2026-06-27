# G007 官方评测回归门报告（2026-06-19）

## 结论

G007 在 G001-G006 self-check 源码合规修复之后，补跑了用户要求的官方评测路径。当前环境没有 `docker`，因此**没有声称已完成官方 Docker/OJ 远程环境**；本轮采用的是最大可复现的本地官方等价链：

1. 使用官方 `prework.py` 同一入口 `make all` 构建 `kernel-rv` / `kernel-la`。
2. 使用 `/root/autotest-for-oskernel/kernel/run_qemu.py` 对应 QEMU 参数运行 RV/LA，其中 RV 完整跑完，LA 因本地/远程地址映射差异只作为本地非权威 smoke，300s 后超时停止。
3. 使用官方 `run.py::parse_serial_out_new()` 读取串口输出。
4. 使用官方 `postwork.py` 公式计算 rank：LTP group 走 `500 * log10(1 + 9 * raw / 10000)`，非 LTP group 累加 raw score。

当前本地官方等价评测总分：**1419.727675405803（int 1419）**。已有本地官方基线 `/root/oskernel-eval-runs/official-20260618-085217/artifacts/score-summary.json` 为 **1085.7787803411047（int 1085）**，当前可比总分 **+333.9488950646984**。

合规结论：G007 没有发现新的 self-check 违规实现，也没有为了追回分数恢复固定输入/固定 case 的成功豁免。少数子项分数下降被保留为真实评测结果；其中 busybox `false` 从旧基线 54 降到 53 是 G001-G006 已删除固定命令成功豁免后的预期合规代价，不允许以 fake success 方式恢复。

## 官方路径与本地等价边界

官方 harness 事实：

- 官方构建入口：`/root/autotest-for-oskernel/kernel/prework.py:49-51` 执行 `make all`。
- 官方 RV/LA QEMU 入口：`/root/autotest-for-oskernel/kernel/run.py:189-202` 启动 `run_qemu(... kernel-rv ...)` 与 `run_qemu_loong(... kernel-la ...)`，然后调用 `parse_serial_out_new()`。
- 官方 LTP 计分公式：`/root/autotest-for-oskernel/kernel/postwork.py:238-242` 对 group 名含 `ltp` 的 raw score 使用对数映射，其他 group 直接累加。

本轮边界：

- `docker=missing`，见 `raw/g007-official-local-metadata.txt`；因此不是官方 Docker/OJ 完整复现。
- 本地 LA QEMU 串口只有 77 bytes，`qemu-la.exit=124`；根据仓库既有约束，本地 LA 无输出/超时不作为远程 LA 回归结论。
- RV QEMU 完整执行结束，`qemu-rv.exit=0`，当前分数主要来自 RV 侧官方 judge 输出。

## 运行证据

核心证据文件：

- 运行根目录：`/root/oskernel-eval-runs/g007-official-local-20260619-110459`
- 运行脚本快照：`raw/g007-official-local-driver.sh.txt`
- 元数据：`raw/g007-official-local-metadata.txt`
- `make all` 输出：`raw/g007-official-local-make-all.txt`
- kernel 产物：`raw/g007-official-local-kernel-artifacts.txt`（`kernel-rv` 1.8M，`kernel-la` 2.7M）
- QEMU 命令：`raw/g007-official-local-qemu-rv.cmd`、`raw/g007-official-local-qemu-la.cmd`
- QEMU exit：`raw/g007-official-local-qemu-rv.exit`=`0`，`raw/g007-official-local-qemu-la.exit`=`124`
- QEMU 进度：`raw/g007-official-local-qemu-monitor.txt`
- 官方 judge 汇总：`raw/g007-official-local-official-judge-summary.json`
- 简化分数：`raw/g007-official-local-score-summary.json`
- 与既有基线比较：`raw/g007-official-local-score-comparison.json`
- LTP parser 明细：`raw/g007-official-local-ltp-summary-rv.txt`、`raw/g007-official-local-ltp-summary-la.txt`
- 磁盘/大镜像清理证据：`raw/g007-disk-prep.txt`、`raw/g007-official-local-df-after-cleanup.txt`

## 分数摘要

当前非零 rank entries：

| group | score |
| --- | ---: |
| basic-glibc-rv | 101.0 |
| basic-musl-rv | 101.0 |
| busybox-glibc-rv | 53.0 |
| busybox-musl-rv | 53.0 |
| cyclictest-glibc-rv | 4.0 |
| cyclictest-musl-rv | 3.0 |
| iozone-glibc-rv | 25.7867376869493 |
| iozone-musl-rv | 24.885347763216288 |
| libcbench-glibc-rv | 29.54333592599454 |
| libcbench-musl-rv | 27.461324087097864 |
| libctest-musl-rv | 215.0 |
| lmbench-glibc-rv | 41.2986509584364 |
| lmbench-musl-rv | 41.37115397387201 |
| ltp-glibc-rv | 4104.0 |
| ltp-musl-rv | 4101.0 |
| lua-glibc-rv | 9.0 |
| lua-musl-rv | 9.0 |
| netperf-glibc-rv | 5.0 |
| netperf-musl-rv | 5.0 |
| rank | 1419.727675405803 |

与既有官方本地基线比较：

- 总分：1085.7787803411047 → 1419.727675405803（+333.9488950646984）。
- int score：1085 → 1419（+334）。
- LTP raw：旧基线 `ltp-musl-rv=4109`、`ltp-glibc-rv=0`；当前 `ltp-musl-rv=4101`、`ltp-glibc-rv=4104`，合计 raw 8205，对数贡献约 461.7386。
- `busybox-*-rv`：54 → 53，原因是固定命令 `false` 不再被 wrapper 人为算作 success。这是 self-check 合规修复，不应回滚。
- `iperf-*-rv`：旧基线非零，当前为 0；G007 没有引入或修复该项，保留真实 judge 输出，不用任何固定输出/路径投机补分。
- `libctest-musl-rv`：217 → 215；`ltp-musl-rv`：4109 → 4101；均作为真实回归信号保留，不隐藏。

因此，本轮满足“在没有违规行为前提下，总体跑分能力不下降”的可复现证据；但不声称每个子 benchmark 都单调不降。

## LTP 可见失败/异常未隐藏

RV 侧 stable LTP 执行：

- `ltp-musl`：992 pass / 8 fail；失败 case 为 `accept02`、`gethostname02`、`nice04`、`setsockopt04`、`mmapstress02`、`mmapstress03`、`mmapstress05`、`sbrk01`。
- `ltp-glibc`：998 pass / 2 fail；失败 case 为 `accept02`、`setsockopt04`。
- 内部状态：`TCONF=2`、`TBROK=4`、`TFAIL=7`。
- `ENOSYS/not implemented`：2。
- LTP timeout：0。
- panic/trap：0。

注意：runner 文本中仍存在 `FAIL LTP CASE <case> : 0` 这类容易误读的 wrapper 行；G007 不把 wrapper 文本当成 PASS 证据，官方分数以官方 judge 的 inner LTP 输出解析为准。所有 `TCONF/TBROK/TFAIL/ENOSYS` 均保留在日志与汇总中。

## self-check 合规判定

G007 只增加评测证据和报告，没有改动内核源码或 evaluator/testsuite 来补分。对 self-check 风险的判定：

- 没有新增按测试程序名、路径、固定输入、固定目录、固定顺序的通过分支。
- 没有伪造 `TPASS` / wrapper PASS。
- 没有把 `TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap` 过滤掉或改写为成功。
- 没有恢复 `busybox false` 固定命令成功豁免。
- 没有修改 `/root/autotest-for-oskernel` 官方 harness 或 testsuite；仅只读调用 judge 逻辑并保存结果。


## G008 交付口径澄清

G008 对 G007 的 final-review WATCH 做出如下收束：

- 本报告中的“官方评测路径”指当前机器可执行的**官方本地等价链**，不是官方 Docker/OJ 远程复现；`docker=missing` 仍保留为环境限制。
- 该等价链使用官方 `make all`、官方 QEMU 参数、官方 `parse_serial_out_new()`、官方 `postwork.py` 计分公式，并已产生 RV 完整执行结果和总分 `1419.727675405803`。
- 官方 Docker/OJ 远程执行在本机不可运行，后续若具备 Docker/OJ 环境应重新验证；但当前缺失不表示存在 self-check 违规实现，也不能作为恢复 fake-success 补丁的理由。
- 本地 LA 超时仍只记录为本地非权威现象，不推导远程 LA 结论。

## 未验证项/风险

- 官方 Docker/OJ 远程执行未完成：当前机器没有 `docker`，不能声称远程完全等价。
- 本地 LA 不权威：`qemu-la.exit=124` 仅记录本地现象，不推导远程 LA 分数。
- 子项波动仍需后续专项优化：`iperf`、`libctest-musl`、`ltp-musl` 有局部下降；本轮保持真实可见，不用违规方式补偿。
- 既有脏工作区仍未触碰：本轮只准备 stage G007 相关报告/证据，不回滚或混入无关删除/修改。
