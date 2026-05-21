# LTP / OS COMP 双架构评测报告

- Generated: `2026-05-19T12:43:33+08:00`
- Repository: `/root/oskernel2026-orays`
- Output directory: `eval-reports/ltp-eval-20260519-122555`
- Commands run:
  - `./run-eval` (riscv64 default)
  - `./run-eval la` (loongarch64)

## Result summary

| Arch | Command | Exit | Start | End | Groups | Success markers | Skips | Output MD |
| --- | --- | ---: | --- | --- | ---: | ---: | ---: | --- |
| riscv64 | `./run-eval` | `0` | `2026-05-19T12:25:55+08:00` | `2026-05-19T12:32:42+08:00` | 24 | 158 | 12 | `rv.output.md` |
| loongarch64 | `./run-eval la` | `0` | `2026-05-19T12:32:55+08:00` | `2026-05-19T12:42:47+08:00` | 24 | 158 | 12 | `la.output.md` |

## Important notes

- 两个命令都完成并返回 `0`。
- `run-eval.sh` 的当前启动评测中，`ltp-musl` 与 `ltp-glibc` 分组均打印 `SKIP: full LTP sweep is too large for the boot-time evaluator smoke run`；因此本次是仓库 `./run-eval` 定义的 OS COMP/LTP smoke/evaluator 启动评测输出，而不是完整 LTP sweep 全量用例展开。
- 原始日志保留 ANSI 控制字符，Markdown 输出使用 ANSI-stripped 版本，便于阅读和提交。
- 评测构建过程中出现的 warning 已保留在各架构完整输出中；未出现导致命令非零退出的错误。

## riscv64 details

- Full output Markdown: `eval-reports/ltp-eval-20260519-122555/rv.output.md`
- Raw log: `eval-reports/ltp-eval-20260519-122555/rv.raw.log`
- Clean log: `eval-reports/ltp-eval-20260519-122555/rv.clean.log`
- Groups:
  - `basic-musl`
  - `busybox-musl`
  - `cyclictest-musl`
  - `iozone-musl`
  - `iperf-musl`
  - `libcbench-musl`
  - `libctest-musl`
  - `lmbench-musl`
  - `ltp-musl`
  - `lua-musl`
  - `netperf-musl`
  - `unixbench-musl`
  - `basic-glibc`
  - `busybox-glibc`
  - `cyclictest-glibc`
  - `iozone-glibc`
  - `iperf-glibc`
  - `libcbench-glibc`
  - `libctest-glibc`
  - `lmbench-glibc`
  - `ltp-glibc`
  - `lua-glibc`
  - `netperf-glibc`
  - `unixbench-glibc`
- Skips:
  - `SKIP: iozone throughput mode currently hangs in the evaluator environment`
  - `SKIP: libcbench currently triggers an unrecovered allocator exhaustion path`
  - `SKIP: libctest still trips unresolved pthread cancellation paths`
  - `SKIP: lmbench still triggers an unresolved user-space page-fault path`
  - `SKIP: full LTP sweep is too large for the boot-time evaluator smoke run`
  - `SKIP: unixbench currently blocks on unresolved executable/runtime compatibility`
  - `SKIP: iozone throughput mode currently hangs in the evaluator environment`
  - `SKIP: libcbench currently triggers an unrecovered allocator exhaustion path`
  - `SKIP: libctest still trips unresolved pthread cancellation paths`
  - `SKIP: lmbench still triggers an unresolved user-space page-fault path`
  - `SKIP: full LTP sweep is too large for the boot-time evaluator smoke run`
  - `SKIP: unixbench currently blocks on unresolved executable/runtime compatibility`

## loongarch64 details

- Full output Markdown: `eval-reports/ltp-eval-20260519-122555/la.output.md`
- Raw log: `eval-reports/ltp-eval-20260519-122555/la.raw.log`
- Clean log: `eval-reports/ltp-eval-20260519-122555/la.clean.log`
- Groups:
  - `basic-musl`
  - `busybox-musl`
  - `cyclictest-musl`
  - `iozone-musl`
  - `iperf-musl`
  - `libcbench-musl`
  - `libctest-musl`
  - `lmbench-musl`
  - `ltp-musl`
  - `lua-musl`
  - `netperf-musl`
  - `unixbench-musl`
  - `basic-glibc`
  - `busybox-glibc`
  - `cyclictest-glibc`
  - `iozone-glibc`
  - `iperf-glibc`
  - `libcbench-glibc`
  - `libctest-glibc`
  - `lmbench-glibc`
  - `ltp-glibc`
  - `lua-glibc`
  - `netperf-glibc`
  - `unixbench-glibc`
- Skips:
  - `SKIP: iozone throughput mode currently hangs in the evaluator environment`
  - `SKIP: libcbench currently triggers an unrecovered allocator exhaustion path`
  - `SKIP: libctest still trips unresolved pthread cancellation paths`
  - `SKIP: lmbench still triggers an unresolved user-space page-fault path`
  - `SKIP: full LTP sweep is too large for the boot-time evaluator smoke run`
  - `SKIP: unixbench currently blocks on unresolved executable/runtime compatibility`
  - `SKIP: iozone throughput mode currently hangs in the evaluator environment`
  - `SKIP: libcbench currently triggers an unrecovered allocator exhaustion path`
  - `SKIP: libctest still trips unresolved pthread cancellation paths`
  - `SKIP: lmbench still triggers an unresolved user-space page-fault path`
  - `SKIP: full LTP sweep is too large for the boot-time evaluator smoke run`
  - `SKIP: unixbench currently blocks on unresolved executable/runtime compatibility`

