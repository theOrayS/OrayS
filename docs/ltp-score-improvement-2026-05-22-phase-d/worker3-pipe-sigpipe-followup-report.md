# worker-3 task11 PIPE SIGPIPE/nonblocking follow-up report

## 结论

- 本轮只处理 worker-3 收到的 PIPE follow-up：`pipe02` / `pipe08` / `pipe11`。没有编辑 `LTP_STABLE_CASES`，没有写入 `.omx/ultragoal`，也没有做 promotion。
- `pipe02` 的 wave1 失败指向默认 SIGPIPE 没有把写端子进程杀死：musl 日志出现 `EPIPE` 后又报 `Child wasn't killed by signal`。已做窄修复：无 reader 写 pipe 时仍排队 SIGPIPE；若 SIGPIPE 是默认处理且未被阻塞，则立即请求 signal exit-group 并终止当前线程，确保父进程可观测到信号终止状态。
- `pipe11` 的 wave1 失败指向 pipe 容量过小：LTP 在启动 reader 前写入 4096 字节，原本 256 字节 ring 容易在预写阶段卡住。已把 pipe ring 提升到 4096 字节，匹配 LTP 对 `PIPE_BUF` 的最低预期。
- `pipe08` 仍是风险点：修复保留“自定义 handler/blocked SIGPIPE”走现有 pending-signal/user-return 路径，未强行把 caught signal 当默认 kill；由于 targeted RV run 未进入 QEMU/LTP，不能宣称 `pipe08` 已 PASS。

## wave1 证据

基于重新解析的 `docs/ltp-score-improvement-2026-05-22-phase-d/raw/worker3-wave1-rv-summary-recheck.txt`：

| case | wave1 RV 结果 | 关键内部证据 | 判读 |
| --- | --- | --- | --- |
| `pipe02` | glibc PASS；musl FAIL code 3，`TFAIL=1`，`TBROK=1` | `write(...) failed: EPIPE (32)`；`Child wasn't killed by signal` | 默认 SIGPIPE 没有形成 signal-death wait status。 |
| `pipe08` | glibc PASS；musl FAIL code 1，`TFAIL=1` | `sigpipe_cnt (0) != 1 (1)` | caught SIGPIPE handler 未被 musl 观测到；需要真实 runtime 再判定。 |
| `pipe11` | glibc/musl 均 FAIL code 137，`timeout=1` | `Reading 4096 per each of 1 children` 后超时 | pipe 预写 4096 字节时容量不足，reader 尚未 fork/调度。 |

LTP 上游源码参考（仅用于确认测试语义，未复制大段内容）：

- `pipe02`: https://raw.githubusercontent.com/linux-test-project/ltp/20240524/testcases/kernel/syscalls/pipe/pipe02.c
- `pipe08`: https://raw.githubusercontent.com/linux-test-project/ltp/20240524/testcases/kernel/syscalls/pipe/pipe08.c
- `pipe11`: https://raw.githubusercontent.com/linux-test-project/ltp/20240524/testcases/kernel/syscalls/pipe/pipe11.c

## 代码变更

- `examples/shell/src/uspace/fd_pipe.rs`
  - `PIPE_BUF_SIZE` 从 `256` 调整到 `4096`。
  - `raise_sigpipe()` 继续调用 `deliver_user_signal()`，但新增默认 SIGPIPE 且未阻塞时的立即 exit-group 终止路径。
  - 自定义 handler 或 blocked signal 不走 immediate kill，仍留给现有 signal delivery/user-return 路径处理，避免把 `pipe08` 的 handler 语义伪装成 PASS。

## 验证

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| 格式检查 | PASS | `cargo fmt -p arceos-shell -- --check`，状态 `0`，日志 `raw/worker3-task11-cargo-fmt-arceos-shell-check.log` |
| summary parser 单测 | PASS | `python3 -m unittest scripts/test_ltp_summary.py`，状态 `0`，日志 `raw/worker3-task11-ltp-summary-unittest.log` |
| RV kernel build | PASS | 恢复 ignored `vendor/cargo` cache 后执行 `CARGO_NET_OFFLINE=true make kernel-rv`，状态 `0`，日志 `raw/worker3-task11-make-kernel-rv.log`；尾部显示 `Finished release profile` 并生成 `kernel-rv`。 |
| wave1 summary reparse | PASS | `scripts/ltp_summary.py` 对 leader-root wave1 RV raw log 重新解析，输出 `raw/worker3-wave1-rv-summary-recheck.txt`。 |
| targeted RV pipe02/pipe08/pipe11 | NOT PROVEN | 自定义 `/ltp_cases.txt` 镜像的 targeted run 在并发队列下于 build 阶段超时/终止，未进入 QEMU/LTP；`raw/worker3-pipe-followup-rv-summary.txt` 为 PASS=0/FAIL=0，仅证明没有 case 结果，不能作为 PASS 证据。状态记录为 `raw/worker3-pipe-followup-rv.status`。 |

## 剩余风险 / handoff

- task11 在队列 API 中当前显示 owner/claim 为 `worker-4`，worker-3 申领返回 `claim_conflict`；因此本 worker 不应强行转移 task11 生命周期，只提交本地修复和证据给 leader 集成。
- `pipe08` 如果后续 wave 仍失败，建议下一步在 musl 路径插桩/核查 `sys_rt_sigaction`、signal mask、`deliver_user_signal()` 返回后 user-return frame 安装，以及 handler trampoline 是否正确返回。
- 不建议本 worker 直接 promotion；需要 leader 用真实 targeted/batch LTP 输出和 `scripts/ltp_summary.py` 再确认 `pipe02`/`pipe08`/`pipe11` 的双 libc 状态。
