# LTP 开发流程

本仓库的 LTP 工作以比赛得分为导向，但仍必须保持真实 Linux/POSIX 语义。目标是推广真正可靠、可回归保护的覆盖，而不是让某个 testcase 名字表面变绿。

## 事实来源

LTP runner 行为主要在 `examples/shell/src/cmd.rs`：

- `LTP_CORE_CASES`：小型 smoke 集合。
- `LTP_STABLE_CASES`：当前高置信 stable 集合。
- `LTP_CASE_BATCHES`：命名 targeted batch。
- `selected_ltp_cases()`：运行时/构建时 case 选择逻辑。
- `run_ltp_suite()`：per-case 执行、timeout、wrapper marker、清理和汇总输出。

当前 stable 数量必须从 `LTP_STABLE_CASES` 读取，不能从旧报告推断。截至本文档更新时，它包含 383 个不重复 case，并会分别在 `/musl` 和 `/glibc` 下执行。

## case 选择方式

guest 内部任一文件存在时，会覆盖选例：

```text
/ltp_cases.txt
/tmp/ltp_cases.txt
```

构建时 `LTP_CASES` 可以选择：

```text
stable
core
batch:<name>
file:<path>
case1,case2,case3
```

timeout 可以通过 guest 内的 `/ltp_case_timeout_secs` 或构建时 `LTP_CASE_TIMEOUT_SECS` 覆盖。当前默认 per-case timeout 是 15 秒。

## 必须使用 parser 解析

在声称某次 evaluator 运行健康之前，必须用 `scripts/ltp_summary.py` 解析日志：

```bash
python3 scripts/ltp_summary.py output_rv.md
python3 scripts/ltp_summary.py output_la.md
python3 scripts/ltp_summary.py --promotion-candidates rv.log la.log
```

parser 会统计 wrapper result line 和内部质量信号：

- `TFAIL`
- `TBROK`
- `TCONF`
- `TIMEOUT LTP CASE` / timeout 文本
- `ENOSYS` / not implemented
- panic/trap 信号

外层 `run-eval` 或 QEMU 退出码干净不代表 LTP 干净通过。报告里必须保留 `TCONF`、timeout、ENOSYS、panic/trap 等 caveat。

## wrapper marker 格式

runner 保留远程 scorer 使用的 completed-case wire format：

```text
FAIL LTP CASE <case> : <status>
```

其中 status `0` 表示 wrapper 层通过，非 0 表示失败。因此 summary parser 以数字 status 作为事实来源。不要随意修改 marker 格式；远程计分依赖它。

## 推荐修复循环

1. 按比赛价值和子系统相关性选择候选 case。
2. 阅读 contest testsuite 的 `runtest` entry 和对应 LTP 源码。
3. 诊断阶段先跑小 targeted batch，不要一开始跑完整 evaluator。
4. 用 `scripts/ltp_summary.py` 解析日志。
5. 修复真实 syscall / VFS / FD / process / signal / memory 语义。
6. 跑同一子系统的相邻回归 case。
7. 推广 case 前验证 RV 和 LA。
8. 通过后再考虑加入 `LTP_STABLE_CASES`。

## 红线

禁止：

- 按 testcase 名称、路径或进程名硬编码行为；
- 伪造 `TPASS` 或 wrapper PASS 输出；
- 修改 LTP testsuite 源码绕过失败；
- 把真实失败隐藏成 `SKIP` / `TCONF`；
- 把 timeout 当成通过；
- 在没有说明缺失验证的情况下，只凭单架构或单 libc 结果推广 case。
