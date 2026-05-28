# LTP stable 分数演进日志整理

本页按阶段整理最近几轮 LTP stable 推广的可信结论和证据路径。stable 数量是历史快照；当前数量仍以 `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 为准。

## 阶段总览

| 阶段 | 目标 / 结果 | 关键结果 | 首选证据 |
| --- | --- | --- | --- |
| 2026-05-22 remote/local 统一 | stable157 级别验证 | RV/LA 各 314 wrapper PASS / 0 FAIL；musl/glibc 各 157/0 | `docs/remote-local-eval-unification-2026-05-22/final-terminal-report.md` |
| 2026-05-24 phase-a | stable250 -> stable300 | RV/LA 各 600 PASS / 0 FAIL；musl/glibc 300/0；`read02` TCONF 透明披露 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-24-phase-a/final-gate-quality-gate.json` |
| 2026-05-25 phase-a | stable300 -> stable350 | RV/LA 各 700 PASS / 0 FAIL；musl/glibc 350/0；`kill02` demote，`abs01` replacement | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-a/final-gate-quality-gate.json` |
| 2026-05-25 phase-b | stable350 -> stable375 | RV/LA 各 750 PASS / 0 FAIL；musl/glibc 375/0；`kill02` 不交付 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/stable375-delivery-report.md` |
| 2026-05-25 phase-c | stable375 -> stable450 尝试 | stable450 未完成；先后接受 stable379、381、382，最终用户 stop-state 保留 stable383 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md` |

## stable300 交付要点

结果：`LTP_STABLE_CASES` 达到 300 total / 300 unique / 0 duplicates。

最终门禁：

- RV：PASS LTP CASE 600 / FAIL 0；`ltp-musl` 300/0；`ltp-glibc` 300/0。
- LA：PASS LTP CASE 600 / FAIL 0；`ltp-musl` 300/0；`ltp-glibc` 300/0。
- 内部 TFAIL/TBROK 为 0；仅保留已披露的 `read02` TCONF。
- timeout、ENOSYS、panic/trap 为 0。
- marker-prefix bad lines 为 0。

首选证据：

- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-24-phase-a/final-gate-quality-gate.json`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-24-phase-a/candidate-matrix.md`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-24-phase-a/final-gate-code-review-report.md`

开发者关注点：`access02`、`access04`、`chmod05`、`statx01`、`writev03`、`pipe2_02`、`waitpid01`、mmap/mprotect/munmap 等在该阶段仍不是 clean promotion。

## stable350 交付要点

结果：`LTP_STABLE_CASES` 达到 350 total / 350 unique / 0 duplicates。

最终门禁：

- RV：PASS LTP CASE 700 / FAIL 0；`ltp-musl` 350/0；`ltp-glibc` 350/0。
- LA：PASS LTP CASE 700 / FAIL 0；`ltp-musl` 350/0；`ltp-glibc` 350/0。
- `read02` TCONF 继续透明披露；timeout、ENOSYS、panic/trap 为 0。
- marker-prefix bad lines 为 0。
- `kill02` 不在 stable350 中；`abs01` 在交付集合中。

首选证据：

- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-a/final-gate-quality-gate.json`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-final-002-summary.txt`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-la-final-002-summary.txt`

开发者关注点：targeted clean 不能替代 aggregate clean；`kill02` 在后续阶段也因 LA aggregate 风险被反复拒绝。

## stable375 交付要点

结果：`LTP_STABLE_CASES` 达到 375 total / 375 unique / 0 duplicates。

交付新增 case：

```text
access02, fchmodat02, inode01, mmap06,
ftest01, ftest02, ftest03, ftest04, mmap10, stream01,
ftest05, ftest07, ftest08, mmap09, mmap11, stream03, stream04, stream05,
abort01, poll01, fork05, fork10, kill11, kill12, mem02
```

最终门禁：

- RV：`OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv`，summary `raw/stable375-rv-final-002-summary.txt`，PASS 750 / FAIL 0。
- LA：`OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la`，summary `raw/stable375-la-final-003-summary.txt`，PASS 750 / FAIL 0。
- 两边都是 `ltp-musl` 375/0、`ltp-glibc` 375/0；TFAIL/TBROK 0；`read02` TCONF 4；timeout/ENOSYS/panic/trap 0；bad marker 0。

首选证据：

- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/stable375-delivery-report.md`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/candidate-matrix.md`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-rv-final-002-summary.txt`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-la-final-003-summary.txt`

## stable400/stable450 尝试与 stable383 stop-state

stable450 未交付。phase-c 只诚实接受了小步 partial promotion：stable379、stable381、stable382，最终用户要求停止后保留 stable383。

已接受新增：

- stable379：`clock_settime01`、`clock_settime02`、`clone03`、`confstr01`。
- stable381：`chmod05`、`fchmod05`。
- stable382：`lseek02`。
- stable383 stop-state：保留 `pipe08`。

当前 live：383 total / 383 unique / 0 duplicates。

可信证据：

- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable400-promotion-gate-report.md`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable450-delivery-report.md`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/raw/stable383-la-gate-001-summary.txt`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/raw/stable384-rv-gate-001-summary.txt`

重要 caveat：exact RV stable383 aggregate 被用户 stop 请求中止，未作为证据提交；当前 RV 支撑来自已完成的 stable384 superset summary。若下一轮要求严格 exact 双架构证明，应先补跑 exact RV stable383 aggregate。
