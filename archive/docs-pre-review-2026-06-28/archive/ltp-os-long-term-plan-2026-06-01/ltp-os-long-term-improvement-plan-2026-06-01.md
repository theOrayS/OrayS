# OS 长期完善计划：full-LTP 后 5~10 sessions 路线图

日期：2026-06-01  
适用仓库：`/root/oskernel2026-orays`  
建议执行方式：Leader 持有 Ultragoal/门禁台账，必要时用 Team 并行拆 lane；每个 session 都必须留下可复查的报告、case 清单、parser 摘要或阻塞说明。

## 0. 背景与当前事实

这份计划基于 full-LTP blacklist sweep 后的最新画像，目标不是继续盲目扩大 sweep，而是在 5~10 个 sessions 内把系统从“稳定 460 + full-sweep 可闭合”推进到“stable 稳步扩张、严重 blocker 下降、核心 Linux/POSIX 语义更可信”。

### 已确认基线

- 当前 `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 实时重算：`460 total / 460 unique / 0 duplicate`。
- stable460 归档门禁：RV/LA × musl/glibc 共 `PASS LTP CASE 920`，`FAIL 0`；已知 caveat 为 `read02 TCONF`，无 timeout/ENOSYS/panic/trap。
- full sweep 最新闭合结果：
  - RV `rv-arch002`：`RUN 4658`，`PASS 1204`，`FAIL 3453`，`TIMEOUT 55`，`TBROK 1043`，`TCONF 2663`，`TFAIL 4058`，`ENOSYS 1280`，panic/trap 为 0。
  - LA `la-arch012`：`RUN 3908`，`PASS 1207`，`FAIL 2698`，`TIMEOUT 53`，`TBROK 1031`，`TCONF 1936`，`TFAIL 4041`，`ENOSYS 1279`，panic/trap 为 0。
- blacklist 只代表 severe-blocker 排除，不计 PASS，不作为 stable promotion 证据。
- LA-only blacklist 显著多于 RV，LA 低 FAIL 数主要来自跳过更多 severe blockers，不代表语义更好。

主要证据入口：

- `docs/ltp-score-improvement-2026-05-28-phase-b/baseline-refresh-report.md`
- `docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-report.md`
- `docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/rv-arch002-summary.json`
- `docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/la-arch012-summary.json`
- `examples/shell/src/cmd.rs`
- `scripts/ltp_summary.py`

## 1. 总目标

### 1.1 5~10 sessions 的结果目标

优先级从高到低：

1. **stable 从 460 稳定推进到 500~520 区间**，只接受 RV/LA × musl/glibc 均干净的 case。
2. **降低 full-sweep severe blockers**，尤其 LA allocator/resource/network 类 blocker；blacklist 减少本身不计分，但降低后续评测风险。
3. **补齐核心语义族群**：time/select/signal、FD/fcntl/pipe、VFS/metadata、mmap/mm、futex/process/IPC。
4. **保持证据诚实**：不隐藏 `TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap`，不把 SKIP/status0/blacklist 当 PASS。
5. **留下下一阶段可接续资产**：每轮有 case 清单、parser 摘要、结论、未闭合项、后续候选。

### 1.2 不追求的目标

- 不在一个 session 内追求“全量 LTP 大幅通过”。
- 不为了分数把普通失败塞进 blacklist。
- 不做跨子系统大重构，除非已有 targeted 证据证明局部修补不可持续。
- 不修改 testsuite/evaluator 绕过失败。

## 2. 全局门禁规则

### 2.1 stable promotion 门禁

一个 case 只有同时满足以下条件才允许加入 `LTP_STABLE_CASES`：

1. RV + LA 均运行。
2. musl + glibc 均 wrapper PASS。
3. `scripts/ltp_summary.py` 未发现新增内部 `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`。
4. 不依赖 blacklist/SKIP/status0。
5. 失败重跑后结论一致，或报告明确说明非确定性风险并暂不推广。

### 2.2 blacklist removal 门禁

一个 severe-blocker 从 blacklist 移除前必须满足：

1. targeted run 不再卡死、不爆内存、不破坏后续 case。
2. marker audit 无 incomplete/panic/trap/resource failure。
3. 移除后 full-sweep 仍能闭合。
4. 移除 blacklist 只表示“可以真实失败/通过”，不表示 stable PASS。

### 2.3 每个 session 的固定产物

每个 session 至少留下：

- `docs/ltp-score-improvement-YYYY-MM-DD-*/` 或对应长期计划子文档中的 brief/report。
- targeted case list。
- RV summary + LA summary，优先 JSON，其次 txt/status。
- promotion candidates 或 no-promotion reason。
- 若修改代码：列出 syscall/errno/flag/ABI/用户可见语义变化。
- 若新增 blacklist 或移除 blacklist：列出 severe-blocker 理由和验证。

## 3. 推荐组织方式

### 3.1 默认执行模式

- **Leader/Ultragoal owner**：维护目标、stable 列表、最终门禁、文档与提交。
- **Team workers**：只负责窄 lane，先 report-only 再小补丁；不得自行推广 stable。
- **Verifier**：对 promotion claim、blacklist claim、parser 输出做独立复核。

### 3.2 推荐 lanes

1. time/select/signal lane
2. FD/fcntl/pipe/ownership lane
3. VFS/metadata/path lane
4. mmap/mm/resource lane
5. futex/process/IPC lane
6. LA severe-blocker lane

## 4. 5~10 sessions 路线图

下面按 8 个 sessions 设计；如果某轮收益很高可合并到 5~6 个 sessions，如果 blocker 较重则扩展到 9~10 个 sessions。

### Session 1：冻结基线 + 建立候选矩阵

目标：把 full-sweep 画像转成可执行候选清单，避免后续凭印象选 case。

任务：

1. 实时重算 `LTP_STABLE_CASES` 数量和重复项。
2. 复读 `rv-arch002` / `la-arch012` JSON，抽取：
   - clean-ish candidates
   - RV/LA divergence
   - timeout/resource/panic-like blocker
   - `ENOSYS` 高频族群
3. 生成 `candidate-matrix-stable460-to-500plus.md`：按 lane 分组，每行包含 case、失败类型、疑似子系统、RV/LA 状态、推荐动作。
4. 选第一批 20~40 个 targeted candidates。

优先候选：

- `getitimer01`
- `ppoll01`
- `select02`
- `diotest4`
- `execve05`
- `readlinkat02`
- `epoll_create02`
- `nice04`
- `clone04`
- 小批 `fcntl*`
- 小批 `statx/getxattr/statfs/getdents`

验收：

- 候选矩阵写入 docs。
- 明确第一批 targeted case list。
- 不修改 stable list。

### Session 2：time/select/signal 第一批

目标：修复高 ROI 的 timer/select/ppoll/poll/signal 交界问题。

重点文件：

- `examples/shell/src/uspace/time_abi.rs`
- `examples/shell/src/uspace/select_fdset.rs`
- `examples/shell/src/uspace/signal_abi.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`

重点语义：

- realtime vs monotonic deadline。
- timeout 精度与剩余时间回写。
- `EINTR`、sigmask、pending signal 处理。
- `POLLNVAL/POLLERR/POLLHUP`。
- `getitimer/setitimer` 最小真实行为。

候选 case：

- `getitimer01`
- `ppoll01`
- `select02`
- 相关 `poll/pselect/clock/nanosleep` 小批 case。

验收：

- targeted RV 先通过或给出真实失败分类。
- LA 复核通过后才列 promotion candidates。
- 不允许因 timeout 消失但内部 `TFAIL/TBROK/TCONF` 仍存在就推广。

### Session 3：FD/fcntl/pipe/ownership

目标：扩大 FD 与权限相关稳定面，减少 fcntl/pipe/chown 类真实失败。

重点文件：

- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/fd_pipe.rs`
- `examples/shell/src/uspace/metadata.rs`
- `examples/shell/src/uspace/credentials.rs`

重点语义：

- `fcntl` flags、dup、FD_CLOEXEC、file status flags。
- shared offset、`O_APPEND`、fork 后 FD 继承。
- pipe EOF/SIGPIPE/nonblock/EINTR。
- `fchown/fchmod/umask/access` 权限细节。

候选 case：

- 小批 `fcntl*`
- `pipe*`
- `writev*`
- `fchown*`
- `access*`

验收：

- promotion candidates 必须 RV/LA × musl/glibc 全干净。
- 如修改 FD 继承或 flags，必须跑当前 stable 中相关回归子集，防止破坏已有 460。

### Session 4：VFS/metadata/path

目标：补齐 stat/path/symlink/xattr/getdents 类语义，争取一批低风险 stable 增量。

重点文件：

- `examples/shell/src/uspace/metadata.rs`
- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/synthetic_fs.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`

重点语义：

- `statx` mask/flags/errno。
- `getxattr/listxattr` 最小兼容。
- `statfs` 字段一致性。
- `getdents64` 的 `d_off/d_ino/d_type`。
- symlink/readlink/readlinkat。
- sticky bit、目录权限、rename/link/unlink 边界。

候选 case：

- `readlinkat02`
- `statx*`
- `getxattr*`
- `statfs*`
- `getdents*`
- 可控 `rename/link/unlink` 小批。

验收：

- 不用 synthetic path 硬编码 case 名。
- errno 与 Linux 预期一致；若无法完整支持，宁可保持真实 FAIL，不伪造 PASS。

### Session 5：mmap/mm/resource 第一批

目标：减少 mmap/mm 类失败，并为 LA severe blockers 的资源稳定性打基础。

重点文件：

- `examples/shell/src/uspace/memory_map.rs`
- `examples/shell/src/uspace/user_memory.rs`
- `kernel/memory/axmm/src/aspace.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`

重点语义：

- file-backed shared mmap writeback。
- `msync` 行为。
- `mprotect` 最大权限、VMA 分裂/合并。
- `mincore` 最小实现。
- page fault 后 SIGSEGV/exit 行为一致性。
- 资源高水位与 teardown 后泄漏观测。

候选 case：

- `mmap*`
- `mprotect*`
- `mincore*`
- `move_pages*` 中低风险子集。
- `diotest4` 相关 IO/mm 交界 case。

验收：

- 不引入新的 stable mmap 回归。
- 对 LA 额外记录内存占用、frame/allocator 高水位或可用替代指标。

### Session 6：futex/process/IPC

目标：处理高频 `ENOSYS` 和 wait/clone/futex/IPC 类语义缺口。

重点文件：

- `examples/shell/src/uspace/futex.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/process_ops.rs`
- `examples/shell/src/uspace/sysv_shm.rs`
- `examples/shell/src/uspace/signal_abi.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`

重点语义：

- futex wait/wake timeout、EINTR、地址 key。
- wait/waitid/zombie/reparent。
- clone/exec FD 与 signal 状态继承。
- SysV shm/sem/msg 的最小真实模型。

候选 case：

- `futex_*` 非 severe-blocker 子集。
- `wait*`/`waitid*` 小批。
- `clone*` 小批。
- `shm*` 可控子集。

验收：

- 所有 process/signal 改动必须跑相关 stable 回归。
- futex 不允许通过忙等掩盖 timeout/调度问题。

### Session 7：LA severe-blocker 专项

目标：降低 LA-only blacklist 规模，先解决会破坏 sweep 闭合的资源/allocator/network blocker。

优先 blocker：

- `creat07`
- `write01`
- `fsync02`
- `lftest`
- `mmstress`
- network stress 类 case
- `futex_wait01/05`
- `nice05`
- `dirtyc0w`
- `pth_str01`

任务：

1. 每次只挑 1~3 个 severe blockers。
2. 单 case targeted 跑到能终止。
3. 修资源泄漏、teardown、allocator pressure、socket cleanup 或 process cleanup。
4. 移除 blacklist 前跑小型 all-minus-blacklist shard。

验收：

- 可从 blacklist 移除的 case 必须不再破坏后续 case。
- 移除后若普通 FAIL，照实保留为 FAIL，不 promotion。
- 产物是 blacklist-removal report，不是 stable report。

### Session 8：整合、推广、回归与下一轮规划

目标：把前 7 个 sessions 的成果合并为稳定可交付状态。

任务：

1. 汇总所有 promotion candidates。
2. 运行 RV/LA × musl/glibc stable final gate。
3. 检查 stable list：数量、重复项、顺序、注释。
4. 运行 targeted regression：time/select、FD/pipe、VFS、mmap、process/futex。
5. 整理 blacklist diff：新增/移除原因和证据。
6. 产出 final report 与 next-session prompt。

验收：

- stable 数量目标：优先 500；如果低风险候选足够，冲 520。
- final gate 不能有新增 `TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap`。
- 所有未推广 case 有明确原因：普通 FAIL、架构差异、flaky、severe blocker、低 ROI。

## 5. 如果扩展到 9~10 sessions

如果 Session 2~6 中某个 lane 产出不足，可追加：

### Session 9：网络/socket 与 proc/syntheticfs 语义

重点：socket errno、poll readiness、network stress cleanup、`/proc` 字段真实性。该 session 以降低 LA network blacklist 和减少 synthetic shim 风险为主，不优先冲 stable。

### Session 10：full-sweep 再闭合与质量审计

重点：重新跑 RV/LA all-minus-blacklist 或 shard sweep，确认：

- `incomplete_count = 0`
- panic/trap = 0
- resource failure = 0
- blacklist 来源、跳过数、架构差异写清楚
- 普通失败仍照实保留

## 6. 推荐 case 优先级表

| 优先级 | 方向 | 代表 case/族群 | 主要收益 | 风险 |
| --- | --- | --- | --- | --- |
| P0 | time/select | `getitimer01`, `ppoll01`, `select02` | 高 ROI，可能直接增加 stable | timeout/EINTR 语义易回归 |
| P0 | VFS/path | `readlinkat02`, `statx*`, `getdents*` | errno/字段修正收益高 | path shim 不可硬编码 |
| P1 | FD/fcntl/pipe | `fcntl*`, `pipe*`, `writev*` | 可批量扩稳定面 | FD 继承/O_APPEND 回归风险 |
| P1 | metadata/ownership | `fchown*`, `access*`, `chmod*` | 与 stable 现有族群接近 | credentials 语义需谨慎 |
| P1 | mmap/mm | `mmap*`, `mprotect*`, `mincore*` | 降低 full-sweep 失败面 | VMA/allocator 风险较高 |
| P2 | futex/process | `futex_*`, `wait*`, `clone*` | 减少 ENOSYS/timeout | 调度/信号 race 难 |
| P2 | IPC | `shm*`, `sem*`, `msg*` | 减少大族群 ENOSYS | 真实模型成本高 |
| P0-blocker | LA severe | `creat07`, `fsync02`, `lftest`, `mmstress`, network stress | 降低 sweep 风险 | 不一定增加 stable |

## 7. 验证命令模板

按实际 case 替换 `LTP_CASES`，不要把模板输出当证明。

```bash
# 1. 实时 stable 计数
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY

# 2. targeted RV smoke/gate 示例
LTP_CASES=getitimer01,ppoll01,select02 make A=examples/shell ARCH=riscv64 run

# 3. summary truth，具体参数按最新脚本帮助调整
python3 scripts/ltp_summary.py <log-path>
python3 scripts/ltp_summary.py --json <log-path> > <summary>.json

# 4. 基础静态校验
git diff --check
```

长跑/QEMU/remote evaluator 前后必须按仓库规则检查磁盘：

```bash
df -h / /root
```

## 8. 风险与缓解

| 风险 | 表现 | 缓解 |
| --- | --- | --- |
| 冲分污染 | blacklist/SKIP/status0 被误当 PASS | Leader-only promotion gate，parser-backed final report |
| 语义回归 | 修一个 case 破坏 stable460 | 每个 lane 跑相关 stable 回归子集 |
| LA 资源不稳 | allocator panic、hang、网络污染 | LA severe-blocker lane 单独处理，先 closure 后 promotion |
| 大重构失控 | 多子系统同时改动难回滚 | session 内只修一个 smell/族群，补丁局部化 |
| full-sweep 证据误读 | open log、truncated log、marker glue | 只在 closed log 上做最终解析，保留 marker audit |
| worker 重复/越权 | 多 lane 同改 stable list | stable list 只由 Leader 修改和提交 |

## 9. 完成定义

本长期计划完成时，应满足至少一组：

### 最小完成

- stable 从 460 提升到 500 左右。
- 至少 2 个核心 lane 有真实语义修复并通过 RV/LA × musl/glibc。
- LA severe-blocker 有明确减少或留下可复现阻塞报告。
- final report 和 next-session prompt 已写入 docs。

### 理想完成

- stable 达到 520 或更高。
- time/select、VFS/metadata、FD/fcntl/pipe 三条线均有可推广增量。
- LA-only blacklist 明显下降，至少一批 allocator/resource blocker 可安全移除。
- 新一轮 full-sweep 或 shard sweep 闭合，panic/trap/incomplete/resource failure 仍为 0。

### 停止条件

- 候选 case 全部变成高风险大语义工程，短期不适合继续冲 stable。
- LA severe blockers 需要架构级 allocator/runtime 改造，超出当前 5~10 sessions 范围。
- 任何门禁发现 stable460 回归，必须先回滚或修复回归，再继续扩张。

## 10. 下一次 session 启动提示

建议下一次直接以 Session 1 开始：

```text
请从 docs/ltp-os-long-term-improvement-plan-2026-06-01.md 的 Session 1 开始执行。
目标：基于 rv-arch002/la-arch012 full-sweep summary 和当前 stable460，生成 candidate-matrix-stable460-to-500plus.md，并选出第一批 20~40 个 targeted cases。保持只读/报告优先，不推广 stable，不修改 blacklist。所有结论必须 parser-backed，blacklist/SKIP/status0 不计 PASS。
```
