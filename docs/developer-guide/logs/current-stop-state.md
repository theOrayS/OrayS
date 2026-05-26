# 当前 stop-state 整理

本页是继续开发前最应该先读的状态页。它把 live 源码、已完成日志、被拒绝证据和下一步行动压缩到一个开发者视图。

## Live 状态

从 `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 重新计算：

- total：383
- unique：383
- duplicates：0
- 每架构默认 stable wrapper events：766
- 当前新增于 stable382 的保留 case：`pipe08`
- stable400 缺口：17 个 clean case
- stable450 缺口：67 个 clean case

## 当前可信证据

| 证据 | 结论 | 用途 |
| --- | --- | --- |
| `docs/ltp-score-improvement-2026-05-25-phase-c/raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | RV targeted 中 `pipe08` musl+glibc clean | `pipe08` RV targeted proof |
| `docs/ltp-score-improvement-2026-05-25-phase-c/raw/target-stable400-kill02-pipe08-la-001-summary.txt` | LA targeted 中 `pipe08` musl+glibc clean | `pipe08` LA targeted proof |
| `docs/ltp-score-improvement-2026-05-25-phase-c/raw/stable383-la-gate-001-summary.txt` | LA exact stable383 aggregate PASS 766 / FAIL 0；musl/glibc 383/0；known `read02` TCONF only | exact LA aggregate proof |
| `docs/ltp-score-improvement-2026-05-25-phase-c/raw/stable384-rv-gate-001-summary.txt` | RV stable384 superset aggregate PASS 768 / FAIL 0；musl/glibc 384/0；known `read02` TCONF only | completed RV support containing `pipe08` |
| `docs/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md` | stable383 stop-state 报告 | 当前状态的首选总报告 |

## 未完成 / 不应夸大的证据

- exact RV stable383 aggregate 曾启动，但用户要求停止前未完成；该 raw log 不作为 promotion 证据。
- RV 侧当前使用 completed stable384 superset 支撑 `pipe08`，不是 exact stable383 证明。
- stable400、stable425、stable450 均未交付；不能从 candidate scout 或 partial targeted result 推断已达成。
- `kill02` targeted clean 不足以推广；LA aggregate 曾暴露 TBROK/setup timeout。

## 当前拒绝项

| Case/family | 主要证据 | 拒绝原因 |
| --- | --- | --- |
| `kill02` | `raw/target-stable400-kill02-pipe08-la-001-summary.txt`; `raw/stable384-la-gate-001-summary.txt` | targeted clean 但 LA aggregate 出现 TBROK/setup timeout |
| `access04`, `chmod06`, `fchmod06` | `raw/target-stable400-access-chmod-rv-001-summary.txt` | tmpfs mount setup `EINVAL` / TBROK |
| `chmod07`, `fchmod02` | `raw/target-stable400-access-chmod-rv-001-summary.txt` | `getgrnam(daemon)` setup breakage |
| `waitid07/08/10` | `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | wait status 和 `/proc/sys/kernel/core_pattern` blocker |
| `munmap01`, `mmap04/05`, `mprotect01/02` | `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | VM permission / maps / signal 行为失败 |
| `pipe07/15` | `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | `/proc` pipe capacity / fd setup blocker |

## 下一步最小行动

1. 如果要严格补齐当前 stop-state，先串行补跑 exact RV stable383 aggregate，并用 `scripts/ltp_summary.py` 解析。
2. 不要重新加入 `kill02`，除非 LA aggregate 不再出现 setup timeout/TBROK。
3. 优先选择 VFS/path/permission、FD/pipe、process/wait、mmap 小范围 targeted batch；每次推广前要求 RV+LA × musl+glibc clean。
4. 每次日志/runner 相关改动都要保留 marker-prefix 检查和 `read02` TCONF 披露。
5. 对并发 QEMU、用户中止、截断输出、partial scout 的日志统一标注为非 promotion 证据。
