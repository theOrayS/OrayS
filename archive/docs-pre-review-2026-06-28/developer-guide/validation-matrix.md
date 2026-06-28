# 验证矩阵

先选择能证明当前 claim 的最小检查；只有当改动影响运行时行为、评测计分或跨架构代码时，再扩大验证范围。

## 快速矩阵

| 修改类型 | 最小验证 | 更强 / 发布级验证 |
| --- | --- | --- |
| 仅 Markdown / docs | `git diff --check` | 文档中写了路径或命令时，增加链接/路径 smoke check。 |
| Rust 格式化改动 | `make fmt` 或 targeted `cargo fmt --check` | 如果同时改了逻辑，再跑 `make clippy`。 |
| C 格式化改动 | `make fmt_c` 或 targeted `clang-format --dry-run` | 如果行为改变，构建受影响 C example。 |
| 普通 Rust module 改动 | 相关 build 或 `make clippy` | 可单测代码再跑 `make unittest_no_fail_fast`。 |
| `examples/shell` 行为 | `make A=examples/shell ARCH=riscv64 build` 或 `make kernel-rv` | targeted RV/LA evaluator batch 加 parser summary。 |
| POSIX / syscall / ABI 行为 | 受影响架构的 shell build | RV 和 LA 上跑 targeted LTP；报告 errno/ABI 变化。 |
| VFS / FD / pipe / process / signal / mmap 修复 | targeted 子系统 LTP case | 相邻高分回归 case 加 RV/LA 最终门禁。 |
| evaluator kernel 产物路径 | `make kernel-rv` 和/或 `make kernel-la` | 如果镜像和 QEMU 可用，跑 `./run-eval.sh rv` 和 `./run-eval.sh la`。 |
| 远程提交行为 | `make all` | 使用仓库 `cargo-home/` 和 helper shim 的离线风格构建。 |
| LTP 推广 | targeted case list 加 `scripts/ltp_summary.py` | RV+LA、musl+glibc 干净矩阵，再更新 stable list。 |

## parser 支撑的 evaluator 证据

任何 LTP/evaluator claim 都应该让 raw log 和 parser 输出成对保存：

```bash
python3 scripts/ltp_summary.py raw.log > raw-summary.txt
python3 scripts/ltp_summary.py raw.log --json > raw-summary.json
```

推广报告应该写清楚：

- 选中的 case list 或 batch；
- 架构：`rv`、`la`；
- libc 分组：`musl`、`glibc`；
- wrapper pass/fail 数；
- 内部 `TFAIL` / `TBROK` / `TCONF` 数；
- timeout、ENOSYS、panic/trap 数；
- 如有缺失检查，明确列出。

## 无法运行验证时

如果缺少 QEMU、sdcard 镜像、Docker、交叉工具链或 testsuite checkout，要明确说明哪个检查不能运行。不要用“看起来更强”的 build-only 结论替代运行时证据。

## 最终报告 checklist

声称完成前，报告中应包含：

- 修改了哪些文件，以及为什么修改；
- 实际运行过哪些命令；
- 没运行哪些命令，以及原因；
- 如果改变 evaluator 行为，说明 `./run-eval.sh rv` / `./run-eval.sh la` 状态；
- 如果改变远程提交行为，说明 `make all` 状态；
- 用户可见行为变化；
- syscall、errno 或 ABI 可见变化；如果没有预期变化，也要明确说明。
