# self-check.md 合规代码审查报告（2026-06-19）

## 结论摘要

本轮审查以 `self-check.md` 的 13 条禁止行为为硬红线，覆盖：

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 中当前 **1000/1000** 个 stable LTP case；
- remote/local runner 入口（`Makefile`、`run-eval.sh`、`examples/shell/src/cmd.rs`）；
- non-LTP runner、wrapper、parser、静态 guard；
- POSIX/API/ulib 可见 syscall/libc 行为边界；
- scripts/configs 与既有 G00x 守卫。

审查产物：

- 逐 case 矩阵：`docs/self-check-compliance-review-2026-06-19/stable-ltp-case-audit-matrix.csv`
- 原始证据：`docs/self-check-compliance-review-2026-06-19/raw/`
- 本报告：`docs/self-check-compliance-review-2026-06-19/report.md`

本轮发现并已修复 6 类真实合规风险/隐患：

1. busybox runner 对固定命令 `false` 的成功豁免（固定输入 fake success 风险）。
2. `open()` flag 翻译把 `O_PATH`（含 `O_EXEC/O_SEARCH` alias）错当 `create_new`，且 `O_EXCL` 语义错误。
3. `recvfrom(NULL, NULL)` / `accept(NULL, NULL)` 错误返回 `EFAULT`。
4. `pthread_mutex_lock/unlock` 在递归锁/非 owner 解锁时 `assert!` panic。
5. libc `fcntl()` 对无第三参数命令无条件 `va_arg`。
6. libc `sysconf()` 直接返回负 errno，没有按 libc 约定设置 `errno` 并返回 `-1`。

本轮没有发现新的 stable LTP case 名称硬编码通过、伪造 TPASS、隐藏 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap 的证据。仍需持续跟踪的架构风险见“剩余风险与后续项”。

## self-check.md 规则映射

`self-check.md` 当前包含 13 条显式禁止项：

- 禁止行为 1：7 条（测试程序/二进制特征特殊判断、特定参数/输入/目录硬编码、高分集合局部实现、利用环境/固定路径/时间/顺序投机、破坏 Linux syscall 兼容或安全边界、绕过真实进程/内存/FS/同步机制、其他投机性实现）。
- 禁止行为 2：6 条（牺牲 Linux syscall 兼容换速度、特定输入硬编码、跳过权限/资源/错误处理、隐藏条件崩溃/数据错误、固定测例优化、其他不符合正确性/通用性/可解释性优化）。

本轮审查按以上 13 条逐项归类，不再沿用历史口径里的“14 条”说法。

## stable LTP 每 case 排查

### 事实源

- stable 列表源：`examples/shell/src/cmd.rs:50-619`
- 矩阵文件：`stable-ltp-case-audit-matrix.csv`
- 摘要证据：`raw/stable-ltp-case-summary.txt`

### 逐例矩阵摘要

- total：1000
- unique：1000
- duplicates：0
- 静态结论：
  - `OK_STATIC`：996
  - `REVIEW`：4
- 需人工解释的 4 个 REVIEW case：`ftest03`、`ftest07`、`dirty`、`stress`

### 4 个 REVIEW case 的处置

这 4 个 REVIEW 均不是“case 名称硬编码通过”的证据：

- `ftest03` / `ftest07`：case 名 token 同时出现在 stable/core case 列表或 guard/parser 测试语境，没有发现 runner 对它们分支返回 PASS、伪造输出或绕过真实执行。
- `dirty`：通用词，命中注释/语义上下文，不是对 LTP case `dirty` 的分支适配。
- `stress`：通用词，命中通用压力/描述语境，不是对 LTP case `stress` 的分支适配。

完整逐例处置见 CSV 的 `self_check_name_specialization_static_result` 与 `disposition` 列。

## 并行审查线索汇总

本轮使用 6 条独立子 agent 分片审查，主线负责证据整合、修复与最终验证。

| 线 | 范围 | 结论 |
| --- | --- | --- |
| explore | stable 列表、runner、non-LTP 入口 | stable count=1000；默认 `LTP_CASES=stable`；识别 runner/parser 关注点 |
| code-reviewer/LTP runner | LTP runner/parser/marker | 未发现 fake TPASS；建议强化 G005 guard，已补充固定命令成功豁免检测 |
| code-reviewer/POSIX API | `api/arceos_posix_api` | 发现 open/socket/pthread/raw pointer 等语义风险；本轮修复可局部收口项 |
| code-reviewer/ulib+shell | `ulib/axlibc`、shell runner | 发现 `line == "false"`、`fcntl()` vararg、`sysconf()` errno 问题；均已修复 |
| code-reviewer/scripts/configs | guard、scripts、configs | G002-G004 root 直跑 import 问题已修复；未发现脚本绕过 scorer |
| architect | 合规架构视角 | runner/summary/promotion 有防 fake 机制；提示 raw pointer 与脏工作区证据删除需显式报告 |

## 已修复问题

### 1. busybox 固定输入 fake success 风险

- 文件：`examples/shell/src/cmd.rs`
- 修复：删除 `Ok(status) if status == 0 || line == "false"` 中对固定命令 `false` 的成功豁免，只把真实 exit status 0 记为 success。
- 防回归：`scripts/check_g005_runner_parser.py` 新增 literal command success override 检测；`scripts/test_g005_runner_parser.py` 新增回归测试。

### 2. open flag 真实语义修复

- 文件：`api/arceos_posix_api/src/imp/fs.rs`、`api/arceos_posix_api/src/imp/fd_ops.rs`
- 修复：
  - `O_EXCL` 仅在 `O_CREAT` 同时存在时转为 `create_new(true)`；
  - `O_PATH`（包含本仓库 libc 暴露的 `O_EXEC/O_SEARCH` alias）与 `O_TMPFILE` 当前无法真实建模，显式返回 `EOPNOTSUPP`，不再误变成创建新文件；
  - `O_CLOEXEC` 写入 FD table 的 descriptor flags。

### 3. socket addr 可空参数语义修复

- 文件：`api/arceos_posix_api/src/imp/net.rs`
- 修复：
  - `recvfrom(..., NULL, NULL)` 忽略 peer address 输出，不再误报 `EFAULT`；
  - `accept(..., NULL, NULL)` 允许不取 peer address；
  - 若 `addr != NULL && addrlen == NULL`，仍按真实错误返回 `EFAULT`；
  - `socket()` 支持 `SOCK_CLOEXEC`/`SOCK_NONBLOCK` flag，不再因合法 flag 组合误拒。

### 4. pthread mutex misuse 不再 panic

- 文件：`api/arceos_posix_api/src/imp/pthread/mutex.rs`
- 修复：
  - 当前线程重复 lock 返回 `EDEADLK`；
  - 非 owner unlock 返回 `EPERM`，未加锁 unlock 返回 `EINVAL`；
  - 去掉隐藏条件下可触发内核 panic 的 `assert!`。

### 5. libc `fcntl()` vararg 修复

- 文件：`ulib/axlibc/c/fcntl.c`
- 修复：仅对 `F_DUPFD/F_SETFD/F_SETFL/F_GETLK/F_SETLK/F_SETLKW/F_SETOWN/F_SETSIG/F_DUPFD_CLOEXEC` 等需要第三参数的命令读取 vararg；`F_GETFD/F_GETFL` 不再发生 UB。

### 6. libc `sysconf()` errno 修复

- 文件：`ulib/axlibc/src/sys.rs`
- 修复：`sys_sysconf()` 返回负 errno 时，libc 层设置 `errno` 并返回 `-1`，不再把负 errno 直接暴露为配置值。

### 7. guard root 直跑修复

- 文件：`scripts/test_g002_fake_success.py`、`scripts/test_g003_stat_metadata.py`、`scripts/test_g004_rlimit_fd.py`
- 修复：补齐 `scripts/` 自身路径到 `sys.path`，使 root 下直接 `python3 -m unittest scripts/test_g00*.py` 可复现通过。

## 验证证据

已通过的命令与日志：

- `python3 scripts/check_g002_fake_success.py` → PASS
- `python3 scripts/check_g003_stat_metadata.py` → PASS
- `python3 scripts/check_g004_rlimit_fd.py` → PASS
- `python3 scripts/check_g005_runner_parser.py` → PASS
- `python3 scripts/check_g006_synthetic_capabilities.py` → PASS
- `python3 scripts/check_g007_socket_time_mempolicy.py` → PASS
- `python3 scripts/check_g008_musl_patch_stable.py` → PASS
- `python3 scripts/check_g009_post_review_semantics.py` → PASS
- `python3 scripts/check_g010_real_kernel_semantics.py` → PASS
- `python3 scripts/check_g011_empty_shells.py` → PASS
- `python3 scripts/check_g012_syscall_review_hotspots.py` → PASS
- `python3 -m unittest discover -s scripts -p 'test_g00*.py'` → 75 tests OK
- `python3 -m unittest scripts/test_g002_fake_success.py scripts/test_g003_stat_metadata.py scripts/test_g004_rlimit_fd.py` → 25 tests OK
- `python3 -m unittest scripts/test_g002_fake_success.py scripts/test_g003_stat_metadata.py scripts/test_g004_rlimit_fd.py scripts/test_g005_runner_parser.py` → 33 tests OK
- `python3 scripts/test_ltp_summary.py` → 12 tests OK
- `python3 -m py_compile ...` → OK
- `cargo check -p arceos_posix_api --offline` → PASS
- `cargo check -p arceos_posix_api --offline --features 'fd fs net pipe select epoll multitask uspace'` → PASS（仅既有 vendor/select 警告）
- `cargo check -p axlibc --offline` → PASS（仅既有 `mktime.rs` unsafe-op warnings）
- `rustfmt +nightly-2025-05-20 --check ...` → PASS
- `git diff --check -- <本轮改动文件>` → PASS
- `make -n all` → dry-run 仍显示 RV/LA 默认 `LTP_CASES="stable"`

日志文件：

- `raw/postfix-static-guards-and-tests.log`
- `raw/postfix-root-unittest-g002-g005.log`
- `raw/prefix-root-unittest-g002-g004-import-failure.log`（pre-fix 失败证据，已被 postfix root unittest 覆盖）
- `raw/postfix-cargo-check-arceos-posix-api.log`
- `raw/postfix-cargo-check-arceos-posix-api-features.log`
- `raw/postfix-cargo-check-axlibc.log`
- `raw/postfix-format-diff-make-dry-run.log`
- `raw/final-postfix-validation.log`
- `raw/final-postfix-cargo-make.log`
- `raw/current-source-diff.patch`

## AI-slop cleanup / fallback 盘点

范围限定到本轮改动文件。行为锁使用上述 G00x guard、单测、cargo check、rustfmt 与 diff-check。

扫描命令输出：`raw/ai-slop-cleaner-scope-scan.txt`

结果：

- 新增逻辑没有发现“silent fallback / just bypass / hardcoded test pass”式掩盖路径。
- `scripts/test_g00*.py` 中的 `fake`、`unimplemented`、`case == "chdir01"`、`line == "false"` 是静态 guard 的恶意样例 fixture，不是运行时实现路径。
- `ulib/axlibc/c/fcntl.c` 既有 `posix_fadvise` / `sync_file_range` 明确返回 `ENOSYS`，属于诚实 unsupported，不计为 fake pass。
- 既有 TODO：`dup2` close-newfd、`freeaddrinfo` lock 等保留为非本轮范围；未新增依赖或抽象层。


## Historical G005 independent review gate（已由 G006/G007/G008 取代）

以下是 G005 当时的历史 final-gate 结果，不是当前交付门结论：

- `code-reviewer`：`COMMENT`。源码合规可接受；唯一 LOW 是 pre-fix root unittest 失败日志仍放在 raw 目录。已处理：旧日志重命名为 `raw/prefix-root-unittest-g002-g004-import-failure.log`，并新增当前通过日志 `raw/postfix-root-unittest-g002-g005.log`（33 tests OK）。
- `architect`：`WATCH`。当时的 WATCH 覆盖 raw pointer copy-in/out、pthread mutex attr/type 简化模型、未跑 live QEMU/远程评测、以及预先存在的无关脏工作区。

取代关系：

- G006 已把源码级 WATCH 收敛为 `APPROVE/CLEAR`，详见 `g006-watch-resolution-report.md` 与 `.omx/ultragoal/quality-gate-g006.json`。
- G007 已补齐官方本地等价评测证据，详见 `g007-official-evaluation-report.md`。
- G008 只负责澄清交付口径：当前机器缺少 Docker，所以 official Docker/OJ 远程复现仍是环境后续项；它不构成 self-check 源码合规 blocker，也不得被写成已完成的远程官方分数。

## 剩余风险与后续项

1. **用户指针 copy-in/out 架构风险（需专项治理）**
   - `api/arceos_posix_api` C ABI 层仍存在多处 raw pointer deref/slice 构造，仅做 null 检查，无法在该层统一证明任意用户指针都安全。
   - `examples/shell/src/uspace` 已有 `validate_user_read/write`、`read_user_value`、`write_user_value` 等用户态执行层校验机制；但 `api/arceos_posix_api` 本身仍应后续引入统一 user-copy 边界或明确其调用域。
   - 本轮没有把该架构项伪装成已解决，也没有隐藏风险。

2. **预先存在的脏工作区**
   - 本轮开始前已有 unrelated 删除/未跟踪项（例如 `output_la.md`、`output_rv.md`、旧 docs 删除、`.codegraph/`、archive docs 等）。
   - 本轮没有 revert 或 stage 这些无关改动；提交时只应暂存本轮合规修复和报告文件。

3. **官方评测边界（G007 已更新）**
   - G007 已补跑 RV 本地官方等价评测：官方 `make all` + 官方 QEMU 参数 + 官方 judge/parser + 官方计分公式；当前总分 `1419.727675405803`（int `1419`）。
   - 当前机器缺少 `docker`，因此仍不声称官方 Docker/OJ 完整复现；本地 LA QEMU 超时也不作为远程 LA 回归证据。
   - 详见 `g007-official-evaluation-report.md`。

## 用户可见行为变化

- busybox：固定命令 `false` 不再被 wrapper 记为 success；只有真实 exit status 0 才 success。
- open：`O_EXCL` 与 `O_CREAT` 组合按排他创建处理；`O_PATH`（含 `O_EXEC/O_SEARCH` alias）/`O_TMPFILE` 暂不支持时显式失败，不再误创建文件。
- socket：合法 `SOCK_CLOEXEC/SOCK_NONBLOCK` flag 被接受并生效；`recvfrom/accept` 允许 caller 不请求 peer address。
- pthread mutex：错误用法返回 errno，不再触发 panic。
- libc：`fcntl(F_GETFD/F_GETFL)` 等无第三参数命令不再读取不存在的 vararg；`sysconf` 错误按 libc errno 语义暴露。

## syscall / errno / ABI 影响

- `open(O_PATH)`（含 `O_EXEC/O_SEARCH` alias）：从错误的 `create_new` 行为变为 `-EOPNOTSUPP`，这是诚实 unsupported 行为。
- `open(O_CREAT|O_EXCL)`：排他创建；`O_CLOEXEC` 影响 FD flags。
- `socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC|SOCK_NONBLOCK, ...)` / UDP 等：支持合法 flags。
- `recvfrom(fd, buf, len, flags, NULL, NULL)`：可成功接收并忽略地址输出。
- `accept(fd, NULL, NULL)`：可成功 accept 并忽略地址输出。
- `pthread_mutex_lock` 重入：`EDEADLK`；非 owner unlock：`EPERM`；未锁 unlock：`EINVAL`。
- `fcntl` ABI：不改变 `ax_fcntl` ABI，只修正 libc vararg 读取条件。
- `sysconf` ABI：错误返回符合 libc：`-1` + `errno`。

## G007 官方评测回归门（追加）

G007 已补跑官方本地等价评测，详见 `g007-official-evaluation-report.md`。当前环境没有 `docker`，因此不声称官方 Docker/OJ 完整复现；本轮使用官方 `make all`、官方 QEMU 参数、官方 `parse_serial_out_new()` 与官方 `postwork.py` 计分公式完成本地等价链。

关键结果：

- RV QEMU：`qemu-rv.exit=0`，完整跑完；LA 本地 QEMU：`qemu-la.exit=124`，因本地/远程 LA 地址映射差异不作为远程 LA 回归结论。
- 当前本地官方等价总分：`1419.727675405803`（int `1419`）。
- 既有本地官方基线：`1085.7787803411047`（int `1085`）；当前可比总分 `+333.9488950646984`。
- LTP raw：`ltp-musl-rv=4101`、`ltp-glibc-rv=4104`；`TCONF/TBROK/TFAIL/ENOSYS` 全部保留可见，panic/trap 为 0。
- 少数子项下降不被隐藏；`busybox false` 的 1 分下降来自删除固定命令 fake-success 风险，不允许回滚。

核心证据：`raw/g007-official-local-score-summary.json`、`raw/g007-official-local-official-judge-summary.json`、`raw/g007-official-local-ltp-summary-rv.txt`、`raw/g007-official-local-score-comparison.json`。

## G008 final-gate wording resolution（追加）

G008 处理独立 architect 对 G007 的 WATCH：

- `report.md` 中原 G005 `COMMENT/WATCH` 段落已改为历史记录，并明确由 G006/G007/G008 取代；它不再代表当前 final-gate 状态。
- G007 的“官方评测”闭环被正式限定为当前环境可执行的官方本地等价链：官方 `make all`、官方 QEMU 参数、官方 `parse_serial_out_new()`、官方 `postwork.py` 计分公式。该链已经给出 RV 完整执行和总分 `1419.727675405803`。
- 官方 Docker/OJ 远程执行仍未完成，因为本机 `docker=missing`；这是环境后续项，不是源码合规 blocker，也不能被报告成已完成。
- 本地 LA `qemu-la.exit=124` 仍不作为远程 LA 回归结论。
- `busybox false` 的 1 分下降仍被明确保留为删除 fixed-input fake-success 后的合规代价，不允许回滚。
