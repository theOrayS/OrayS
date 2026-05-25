# Next-session prompt: continue from stable379 toward stable450

工作目录：`/root/oskernel2026-orays`

请继续使用 Ultragoal + Team 模式，中文汇报，遵守 AGENTS.md。

当前最高可信状态必须 live 复核：

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 应为 379 total / 379 unique / 0 duplicates。
- 已从 stable375 真实 promotion 的 4 个 case：`clock_settime01`, `clock_settime02`, `clone03`, `confstr01`。
- RV aggregate evidence: `docs/ltp-score-improvement-2026-05-25-phase-c/raw/stable379-rv-gate-002-summary.txt`，PASS LTP CASE 758 / FAIL 0，`ltp-musl` 379/0，`ltp-glibc` 379/0。
- LA aggregate evidence: `docs/ltp-score-improvement-2026-05-25-phase-c/raw/stable379-la-gate-001-summary.txt`，PASS LTP CASE 758 / FAIL 0，`ltp-musl` 379/0，`ltp-glibc` 379/0。
- known transparent TCONF 仍只有 `read02`；不能把它说成 clean。
- original `axfs::fops:297 [AxError::NotADirectory]` 噪声在 stable379 aggregate 中为 0；残留 `axfs_ramfs::file:69` NotADirectory 每架构 22 条。

新增 G002 Attempt 3 负证据（必须重新 live 复核，不要 promotion）：

- `readlinkat02`: RV musl+glibc clean，LA glibc clean，但 LA musl TFAIL；`target-stable400-readlinkat02-serial-promotion-candidates.txt` 为 0 candidates。
- `pipe02`: RV wave2 出现 panic/trap；不要放入 broad batch，先 root-cause。
- wave2 metadata/path blockers: `access04`, `chmod06`, `chmod07`, `fchmod02`, `fchmod06`, `statx01`, `rename04`, `rename05` 仍有 TBROK/ENOSYS。
- time/signal/wait scout: `clock_gettime01`, `nanosleep01`, `nanosleep02`, `pause01`, `sigpending02`, `signal01`, `signal06`, `waitid07`, `waitid08`, `waitid10` 在 RV musl 有 TFAIL/TBROK/TCONF/timeout；`kill02` 仅 RV musl 通过，不具备 promotion 证据。
- FD/fcntl scout: `dup05`, `fcntl07`, `fcntl11`, `fcntl14`, `fcntl15`, `fcntl07_64`, `fcntl11_64`, `fcntl15_64` 在 RV musl+glibc PASS 0 / FAIL 16；涉及 `mkfifo` ENOSYS 和 record-locking TFAIL，不具备 promotion 证据。
- 任何标记为 `invalid-concurrent` 的日志都不是证据；不要用于 promotion。

目标：

1. 从 stable379 冲 stable400（先找 21 个 clean case）。
2. 再冲 stable425/stable450；stretch stable460/475 只有资源和 clean subset 足够时才做。
3. 任何新增 case 必须 RV+LA x musl+glibc 全 clean：wrapper FAIL 0，internal TFAIL/TBROK 0，无新增 TCONF，parser timeout/ENOSYS/panic/trap 0。
4. 不伪造 PASS、不 hardcode case name、不修改 LTP 源码、不把 timeout/TFAIL/TBROK/ENOSYS/panic 改成 PASS/TCONF。

启动步骤：

1. `df -h / /root` 和 `du -sh /root/.codex`。
2. `git status --short`，保护用户文件和 root-level kernel/raw logs。
3. 重新计算 live stable count/duplicates。
4. 读取 `final-gate-quality-gate.json`、`stable379-promotion-gate-report.md`、`candidate-matrix.md`。
5. Team 分工仍按 VFS/FD/process/mmap-fs/verification lanes；worker 不拥有 `.omx/ultragoal` 和最终 `LTP_STABLE_CASES` 修改。
6. 串行运行最终 promotion aggregate gate，避免并发 QEMU/sdcard 争用。

优先候选方向：

- 从 worker reports 和 `candidate-matrix.md` 继续筛选，不要重复已失败 blocker。
- 优先同子系统、低成本、高隐藏价值：access/chmod/statx/openat/link/unlink/readlinkat、pipe/pipe2/readv/writev/preadv/pwritev/fcntl、wait/waitid/kill/fork/signal、mmap/munmap/mprotect、fs_perms/ftest/rwtest/stream/openfile/writetest。
- 避免低 ROI 重构族：fs_bind/test_robind/ksm/fanotify/inotify/bpf/keyctl/ptrace/mount/quotactl/broad xattr/io_uring/perf。

交付条件：

- stable450 live list 450 unique。
- RV final stable gate PASS LTP CASE 900 / FAIL 0；LA 同样 PASS 900 / FAIL 0。
- `ltp-musl` 450/0 和 `ltp-glibc` 450/0 均成立。
- marker prefix bad lines 0。
- 原始 fops NotADirectory 噪声不回归；残留 `axfs_ramfs::file:69` 单独披露或修复。
- 自动 commit agent-owned tracked 变更，遵循 Lore commit protocol。
