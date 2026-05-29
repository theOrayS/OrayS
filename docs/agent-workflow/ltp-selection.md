# LTP / Score 候选选择

只在需要决定下一步跑什么 LTP/score 测例、做候选排序或写测例选择报告时读取本文件。普通编码、文档、远程启动问题不需要读取。

## 目标

LTP 工作是比赛冲分导向，不是追求最大 upstream LTP 覆盖。目标是在保持真实 Linux/POSIX 语义的前提下，用最少开发时间提升 OS contest score。

实验性 `exp/*full-sweep*` 分支可以临时切换为“blacklist full sweep”目标：
尽量跑完整 `ltp-full-20240524` 可执行集合，借助 blacklist 避开会卡死、炸内存、
破坏评测器或明显依赖当前不支持内核模型的用例，从而发现高收益 gap 和 scorer
真实计分口径。这个目标与 stable promotion 不同：blacklist sweep 是侦察和计分实验，
不是把未跑/被跳过 case 变成通过。

## 事实来源

推荐候选前必须读取当前事实：

- 当前 score gap 或最新 evaluator/LTP summary；
- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 的实时内容；
- `scripts/ltp_summary.py` 输出；
- evaluator 的 `ltp/runtest` entry；
- 对应 `testcases/kernel/{syscalls,mem,fs}/...` 源码；
- source-level yield 信号：`tcases[]`、`ARRAY_SIZE`、`.test_variants`、loops、forks、`TST_EXP_PASS`、`TST_EXP_FAIL`、`tst_res(TPASS)`。
- blacklist sweep 事实：`LTP_CASES` 选择模式、默认 blacklist、build-time `LTP_BLACKLIST`、guest `/ltp_blacklist.txt`/`/tmp/ltp_blacklist.txt`、started/pass/fail/timeout/skip/incomplete 计数。

需要 LTP 源码证据时，优先使用 contest baseline `oscomp/testsuits-for-oskernel@pre-2025` 及其 `ltp-full-20240524`；upstream LTP master 只作辅助。

## 推荐执行模式

LTP score campaign 默认采用“Team/Ultragoal + targeted batch -> promotion -> final RV+LA gate”。如果当前环境没有 OMX runtime，就用等价的 solo/native-subagent 流程保持同样闸门：先小批验证，再推广 stable list，最后做 RV/LA 收口。

blacklist full sweep 推荐采用另一条实验流程：

```text
确认 exp 分支 -> 记录当前 score/stable 基线 -> 固定 blacklist 来源 ->
先 RV sweep -> 解析并分类 -> 修/扩 blacklist 的理由文档化 ->
再 LA sweep -> 对比 RV/LA gap -> 抽取高收益 targeted 修复候选
```

不要把 full sweep 的 `SKIP`/blacklist 结果混入 clean pass；不要为了让全量日志好看而
把真实失败加入 blacklist。新增 blacklist 项必须能说明风险类型，例如 hang、
fork-bomb、OOM、crash、长期 stress、namespace/cgroup/driver/config 依赖，或会破坏后续
评测器环境。

## 排序模型

```text
priority_score = potential_score_or_case_count
               * relevance_to_existing_work
               * hidden_test_value
               / implementation_cost
               / regression_risk
```

估分材料包括 contest score gap、runtest 数量、源码内部 case 数、variant/loop/fork fan-out。提高权重：复用已实现 `access`、`chmod`、`stat`、`pipe`、`fork`、`signal`、`read`、`write` 逻辑，或能防隐藏测试。降低权重：需要大 VM、scheduler、network、permission、filesystem redesign，或会威胁已高分回归。

## 总体优先级

1. 当前 contest score 表中 `pass < all` 的 existing gaps。
2. 已高分相邻子系统扩展，尤其 `access`、`chmod`、`stat`、`open`、`pipe`、`signal`、`wait`、`read`、`write`。
3. not-yet-run upstream/contest LTP 中内部 case、loop、variant、fork 较多的目标。
4. 隐藏测试防御：`mmap`、`mprotect`、`statx`、`openat`、`waitid` 等常见 Linux 兼容语义。
5. 复杂低 ROI 家族最后处理：`bpf`、`fanotify`、`inotify`、`keyctl`、`landlock`、`io_uring`、`perf_event_open`、`ptrace`、`mount`/`swap`、`quota`、大范围 `xattr`。

blacklist sweep 后的优先级要同时看“case 是否通过”和“内部 `TPASS`/子断言密度”。
一个高 fan-out case 可能比十几个低收益 case 更值得修；但 `TFAIL`/`TBROK`/timeout
仍按真实失败处理，不能只引用局部 `TPASS` 声称通过。

## syscalls 候选顺序

优先：`statx`、`mmap`、`fcntl`、`open`/`openat`、`rename`、`link`、`unlink`、`readlinkat`、`preadv`/`pwritev`、`writev`、`sendfile`、`waitid`、`kill`、`fork`/`clone`、`pipe`、`access`、`chmod`/`fchmod`、`chown`/`fchown`。

理由：剩余 runtest 覆盖较大，复用当前 VFS/FD/process work，或能防常见隐藏兼容测试。

## mm 候选顺序

优先：base `mm`/`page`/`mem`、`mmap10*`、`vma*`。基本 mapping 和 SysV-shm 行为清楚后再考虑 `mmapstress*` 或 `shmt*`。

低 ROI 暂缓：`ksm*`、`oom*`、`thp*`、`overcommit_memory*`、`cpuset*`、`swapping*`，除非内核已有对应 Linux VM control。

## fs 候选顺序

优先：`fs_perms*`、`ftest*`、`rwtest*`、`stream*`、`openfile01`、`writetest01`、`iogen01`、`fs_inod01`、`inode*`。

`gf*` 在基本文件语义稳定后小批量运行。`fs_bind*`、`test_robind*` 在 mount/bind mount/namespace 语义成为真实目标前暂缓。

## 不得挤占高 ROI 的大族

这些家族不能优先于相邻基础语义 case：`fs_bind*`、`test_robind*`、`ksm*`、`fanotify*`、`inotify*`、`bpf*`、`keyctl*`、`ptrace*`、`mount*`、`quotactl*`、namespace-specific `ioctl*`。

## 报告必须包含

A. 当前 gap 摘要：case、`pass`/`all`、remaining gap、subsystem、priority。

B. 值得加入 self-test 的 not-yet-run cases：source evidence、内部 case/loop/variant/fork 数、相关 syscall、理由、估计成本、回归风险。

C. 下一步最小执行计划：先跑哪些 individual cases，失败时查哪些 syscall/errno/flag/boundary，相邻回归 cases，最终 RV/LA 与 glibc/musl gate。

blacklist full sweep 报告还必须包含：

```text
Sweep mode:
- branch, arch, libc/runtime, LTP_CASES mode, timeout, raw log path

Blacklist:
- default/env/file sources
- skipped count and skipped case list or artifact path
- reason category for newly added blacklist entries

Run closure:
- RUN/PASS/FAIL/TIMEOUT/SKIP/incomplete counts
- last closed case and any unclosed current case
- parser summary plus TPASS/TFAIL/TBROK/TCONF/ENOSYS/panic/trap totals

Follow-up:
- high-yield failing cases ranked by TPASS/all or source fan-out
- candidates to remove from blacklist after a real fix
```
