# Next Session Prompt: stable375 Follow-up

工作目录：`/root/oskernel2026-orays`

当前基线（下次必须 live 复核）：

- stable375 已交付，`examples/shell/src/cmd.rs::LTP_STABLE_CASES` 应为 `375 total / 375 unique / 0 duplicates`。
- final gate evidence:
  - RV: `docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-rv-final-002-summary.txt`
  - LA: `docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-la-final-003-summary.txt`
  - marker prefix: `docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-final-marker-prefix.txt`
- 两架构 final gate 均为 PASS LTP CASE 750 / FAIL 0；ltp-musl 375/0；ltp-glibc 375/0；timeout/ENOSYS/panic/trap 0；仅保留已知 `read02` TCONF。

建议下一轮目标：stable375 -> stable390 或 stable400，先做 $ralplan / Ultragoal 规划，不要直接大批加入 stable。

优先 follow-up：

1. 修复/验证 `kill02`：它有 targeted clean 证据，但 LA full stable aggregate 曾在 `stable375-la-final-001` 暴露 TBROK/setup failure；未重新 full aggregate clean 前禁止加入 stable。
2. VFS metadata 主线：`statx01`, `readlinkat02`, `rename01/03/04/05`, `openat02/03`, `chmod*`, `fchmod*`。
3. Pipe blocker：`pipe02` discovery panic 需要 root-cause，不能当 promotion evidence。
4. VM stretch：`mprotect01/02`, `munmap01`, `mmap13/14`，需要页权限和 unmap 边界语义。
5. Code-review follow-up：`chmod_permission_allowed()` 目前按 effective uid 检查，隐藏 `setfsuid + chmod` 语义可能仍不完整。

继续遵守：不伪造 PASS、不 hardcode case name、不改 LTP 测试源码、不把真实 TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap 转成 SKIP/TCONF/PASS。promotion 必须由 `scripts/ltp_summary.py` 或等价矩阵证明 RV+LA × musl+glibc 全 clean。
