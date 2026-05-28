# 阻塞项索引

本页按子系统整理最近日志里反复出现的 blocker。目的是帮助开发者选择下一步工作，而不是替代原始 summary。

## VFS / permission / metadata

| Case/family | 现象 | 证据路径 | 下一步 |
| --- | --- | --- | --- |
| `readlinkat02` | RV clean；LA glibc clean；LA musl 对 zero-size testcase 传入 syscall 的 `bufsiz` 变成 1，导致 TFAIL | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/readlinkat02-diagnostic-report.md`; `raw/readlinkat02-la-diagnostic-003-summary.txt` | 不要在 syscall body 对 `bufsiz=1` 做非 Linux special-case；需要 LA-musl call-boundary/root-cause |
| `statx01`, `statx03`, `statx04`-`statx12` | statx mask/attribute/setup 仍有 TFAIL/TBROK/TCONF | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable400-attempt5-inventory-statx-report.md`; `raw/target-stable400-statx-tail-rv-001-summary.txt` | 先解决 ENOSPC/device、`mkfs.ext4`/`exportfs`、config parsing 等 setup blocker |
| `rename*`, `openat02/03` | setup-heavy TBROK、tmpfs/device/ENOSPC 风险 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/candidate-matrix.md`; `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/candidate-matrix.md` | 先修真实 setup/path 语义，再 targeted RV+LA |
| `access04`, `chmod06`, `fchmod06` | tmpfs mount setup `EINVAL` / TBROK | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md` | 需要真实 tmpfs/mount/setup 兼容，不要转换成 skip |
| `chmod07`, `fchmod02` | `getgrnam(daemon)` setup breakage | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md` | 继续 group/user lookup 兼容性工作 |

已推广的相关回归目标：`access02`、`fchmodat02`、`chmod05`、`fchmod05`。修改 permission/group 逻辑时要把这些纳入回归。

## FD / pipe / iovec

| Case/family | 现象 | 证据路径 | 下一步 |
| --- | --- | --- | --- |
| `pipe08` | targeted RV+LA clean，当前 stable383 stop-state 保留 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md` | 若需严格证明，补 exact RV stable383 aggregate |
| `pipe02` | discovery 中出现 panic/trap | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/candidate-matrix.md`; phase-c wave2 scout | 先 root-cause panic，不能作为 promotion evidence |
| `pipe07`, `pipe15` | `/proc` pipe capacity/fd setup blocker | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md` | 需要 `/proc` pipe/fd surface 或容量语义支持 |
| `writev03` | TCONF/SMP 风格 blocker，不能用 wrapper success 推广 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/worker3-fd-pipe-iovec-report.md` | 先做语义报告/小范围复现，不要 speculative fd-layer patch |
| fcntl/record-lock/FIFO batch | PASS 0 / FAIL 16，TBROK/TFAIL/ENOSYS | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable400-promotion-gate-report.md` | 需要 FIFO/syscall 和 fcntl record-lock 语义工作 |

已推广的相关回归目标：`pipe12`、`pipe13`、`pipe2_01`、`pipe2_04`、`dup207`、`readv02`、`writev01/05/06/07`、`lseek02`。

## process / wait / signal

| Case/family | 现象 | 证据路径 | 下一步 |
| --- | --- | --- | --- |
| `kill02` | targeted clean，但 LA aggregate 出现 TBROK/setup timeout | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md`; `stable400-promotion-gate-report.md` | 不要推广；先修 LA aggregate setup timeout |
| `waitid07`, `waitid08` | 需要真实 stopped/continued-child accounting | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/worker4-process-wait-signal-report.md`; phase-c reports | 实现真实 wait 状态流转后 targeted + aggregate |
| `waitid10` | 可能先需要 synthetic `/proc/sys/kernel/core_pattern` | 同上 | 先补只读 procfs surface，再复测 |
| time/signal/wait scout | TFAIL/TBROK/TCONF/timeout，部分 scout 中止 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable400-promotion-gate-report.md` | 不要从中挑 partial pass 推广；按子系统拆小批 |

已推广的相关回归目标：`fork05`、`fork10`、`kill11`、`kill12`、`clone03`、`clock_settime01/02`。

## mmap / VM / memory

| Case/family | 现象 | 证据路径 | 下一步 |
| --- | --- | --- | --- |
| `mmap04`, `mmap05`, `munmap01`, `mprotect01`, `mprotect02` | VM permission、maps、signal 行为失败 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md`; worker5 reports | 需要 page protection/unmap/`/proc/self/maps`/SIGSEGV 语义，不要硬过 |
| `mmap13`, `mmap14` | TFAIL/TBROK/ENOSYS 或 partial-only | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/candidate-matrix.md` | 延后到基础 mmap/mprotect 行为稳定后 |
| `inode02` | RV clean，LA glibc timeout | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/candidate-matrix.md` | 先查 LA runtime/memory growth，不要推广 |

已推广的相关回归目标：`mmap06`、`mmap09`、`mmap10`、`mmap11`、`mem02`。

## 日志噪声 / marker

| 项 | 现象 | 证据路径 | 下一步 |
| --- | --- | --- | --- |
| `axfs::fops:297 [AxError::NotADirectory]` | 远程输出高频噪声，已通过 direct `Err(AxError::NotADirectory)` 消除该热点 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/log-noise-repair-report.md` | 保持 errno 不变；不要把失败变成功 |
| residual `AxError::NotADirectory` | stable383/stable384 完成日志仍有 22 条 residual，不是原 fops:297 热点 | `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/remote-marker-and-log-noise-regression-check.md` | 若远程输出仍过大，再独立 triage `axfs_ramfs::file:69` family |
| marker prefix | phase-b/phase-c 关键日志 bad marker prefix 为 0 | 同上 | runner/logging 改动后继续检查列 0 marker |
