# 命令、工具链与验证

只在需要构建、测试、CI 对齐或选择验证范围时读取本文件。

## 常用命令

从仓库根目录运行：

```bash
make                         # remote-submission kernels: kernel-rv/kernel-la
make kernel-rv && make kernel-la
./run-eval.sh rv             # local RISC-V evaluator path
./run-eval.sh la             # local LoongArch evaluator path
make run-rv ARCH=riscv64
make run-la ARCH=loongarch64
make A=examples/shell ARCH=riscv64 run
make clippy
make fmt && make fmt_c
make doc_check_missing
make unittest_no_fail_fast
```

LTP 结果优先用脚本汇总：

```bash
python3 scripts/ltp_summary.py output_rv.md
python3 scripts/ltp_summary.py output_la.md
python3 scripts/ltp_summary.py --promotion-candidates rv.log la.log
```

实验性全量 LTP / blacklist sweep 只在 `exp/` 分支或明确实验任务中运行。入口仍是
shell evaluator，但用 `LTP_CASES` 切换 case 选择：

```bash
# 枚举 guest LTP bin 目录中所有 case，再扣除默认 blacklist 和可选文件/env blacklist
LTP_CASES=blacklist ./run-eval.sh rv
LTP_CASES=blacklist ./run-eval.sh la

# 等价别名
LTP_CASES=all-minus-blacklist ./run-eval.sh rv
LTP_CASES=sweep:blacklist ./run-eval.sh rv

# 不扣 blacklist 的全量枚举仅用于小心诊断，容易卡死或打爆资源
LTP_CASES=all ./run-eval.sh rv
```

blacklist 来源按优先合并：源码默认 `LTP_SWEEP_DEFAULT_BLACKLIST_CASES`、
build-time `LTP_BLACKLIST`、guest `/ltp_blacklist.txt`、guest `/tmp/ltp_blacklist.txt`。
如需调长单 case 上限，使用 `/ltp_case_timeout_secs` 或 build-time
`LTP_CASE_TIMEOUT_SECS`，但报告必须说明原因。

## 工具链事实

- Rust pinned by `rust-toolchain.toml`: `nightly-2025-05-20`, edition 2024。
- 目标：`x86_64-unknown-none`、`riscv64gc-unknown-none-elf`、`aarch64-unknown-none-softfloat`、`loongarch64-unknown-none-softfloat`。
- Make helpers：`cargo-binutils`/`rust-objcopy`、`axconfig-gen`、`cargo-axplat`。
- C 示例需要 README 中说明的 musl cross toolchains 与 `libclang`/`clang`。
- C 格式遵循仓库 `.clang-format`；没有 repo-local `rustfmt.toml`。

## Make 变量提醒

`ARCH` 只能是 `x86_64`、`riscv64`、`aarch64`、`loongarch64`。常见变量包括：
`A`/`APP`、`FEATURES`、`APP_FEATURES`、`LOG`、`SMP`、`MODE`、`PLAT_CONFIG`、`TARGET_DIR`、`BLK`、`NET`、`GRAPHIC`、`MEM`、`DISK_IMG`。
QEMU runtime flags 不是 compile-time feature flags。

`make testsuite-sdcard` 需要 sibling checkout `../testsuits-for-oskernel`，可用 `TESTSUITE_DIR` 覆盖。

## 最小验证选择

- 文档-only：检查 Markdown 结构并运行 `git diff --check`。
- 格式-only 或大范围 Rust/C：`make fmt`、`cargo fmt --all -- --check`、`make fmt_c` 或 targeted `clang-format`。
- 库/模块：优先 `make clippy` 或 `make clippy ARCH=<arch>`；可单测的代码跑 `make unittest_no_fail_fast`。
- 示例：构建 touched example 的相关架构。
- POSIX/user-space 行为：至少 `make A=examples/shell ARCH=riscv64`；运行时语义再加 QEMU/evaluator。
- evaluator kernel 或本地分支行为：`make kernel-rv`/`make kernel-la` 后，在 QEMU 与 sdcard 可用时跑 `./run-eval.sh rv` 和 `./run-eval.sh la`。
- blacklist full sweep：运行前后检查 `df -h / /root`；保存 raw log，但不要把大 raw log 当作默认提交内容；用 `scripts/ltp_summary.py`、`rg '^RUN LTP CASE|^PASS LTP CASE|^FAIL LTP CASE|^TIMEOUT LTP CASE|^\[CONTEST\]\[LTP\]\[SKIP\]'` 和尾部闭合检查确认 started/pass/fail/timeout/skip/incomplete 数量。

## 分阶段验证

跨 boot/trap/scheduler/user-task flow 的改动，尤其涉及 `kernel/runtime/axruntime`、`kernel/arch/axhal`、`kernel/task/axtask`、`api/arceos_posix_api/src/uspace.rs` 时：

1. 先跑最小 build-only 验证；
2. build 通过后再跑行为/QEMU/evaluator 验证。

不能运行的验证必须写明具体原因，例如缺少 QEMU、sdcard、testsuite checkout、cross toolchain 或时间窗口不足。

## CI 事实

CI 覆盖格式、clippy、Rust/C examples、平台/config builds、docs check、unit tests、QEMU-backed `arceos-apps`。Pinned nightly 失败是 regression；moving-nightly lane 允许有差异但不能掩盖 pinned 失败。
