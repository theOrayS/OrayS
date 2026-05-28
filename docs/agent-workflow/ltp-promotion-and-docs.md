# LTP 推广、回归保护与文档命名

只在修改 `LTP_STABLE_CASES`、推广 case、写 LTP 报告/下一轮 prompt、整理 score-improvement 文档时读取本文件。

## Stable list 真相

当前 stable 数量必须实时从这里计算：

```text
examples/shell/src/cmd.rs::LTP_STABLE_CASES
```

不要使用记忆中的 stable count。`output_rv.md` / `output_la.md` 可能只是 smoke logs，不能自动视为 promotion proof。

## 推广闸门

把 case 加入 `LTP_STABLE_CASES` 前，至少满足：

- targeted case 在相关 libc/arch 组合下通过；
- 相邻高价值回归 case 没有明显退化；
- RV 与 LA 状态已经说明；
- `scripts/ltp_summary.py` 没有隐藏 timeout、`TCONF`、`ENOSYS`、panic/trap；
- 报告包含 raw log 或 summary 路径；
- 没有 testcase-name hardcode、fake PASS、testsuite source 修改或 evaluator bypass。

## 回归保护目标

以下家族一旦通过，后续 VFS/FD/process/signal/user-memory/mmap/errno 相关改动必须考虑回归风险：

- `access01` 和 broader `access`；
- `getpid01`、`fork`、`wait`；
- `pipe11` 和 broader `pipe`；
- `chmod01`、`stat`、`statx`；
- `signal03`、`signal04` 和 broader `signal`；
- `read02`、`read`、`write`、`readv`、`writev`。

## LTP 红线

LTP 工作不得：

- 硬编码 test path、filename、case name、process name；
- 基于测试名返回固定结果；
- 修改 LTP test source 让它通过；
- 修改 evaluator scripts 绕过真实测试；
- 让测试程序 fake-print `TPASS`；
- 把真实失败伪装为 `SKIP`/`TCONF`；
- 为单个 case 破坏通用 Linux 语义；
- 不跑相邻高分回归就追一个单 case 通过。

## 阶段文档命名

LTP score-improvement campaign 的持久文档放在：

```text
docs/ltp-score-improvement-YYYY-MM-DD-phase-x/
```

使用创建文档当天的本地日历日期。当天第一组是 `phase-a`，后续同日继续 `phase-b`、`phase-c`。
不要创建未来日期目录；历史证据保留原日期/phase，从新文档引用它。移动 prompt 或 plan 到新目录时，同步更新自引用、OMX brief path 和 follow-up prompt 文本。

## 报告字段

LTP 分析或修复报告必须包含：

```text
Current gap:
- case, pass/all, remaining, subsystem, priority

Candidate evidence:
- source/runtest/log paths
- internal case/loop/variant/fork counts
- related syscalls/subsystems

Execution plan:
- targeted cases first
- likely syscall/errno/flag/boundary checks
- regression cases
- RV/LA and glibc/musl finish gate
```
