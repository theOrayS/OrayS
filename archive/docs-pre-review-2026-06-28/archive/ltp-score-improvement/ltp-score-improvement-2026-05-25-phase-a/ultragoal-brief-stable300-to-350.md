# Ultragoal brief: stable300 -> stable350

目标：使用 Ultragoal + Team 将 `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 从 live stable300 提升到 stable350。

必须 live 复核：磁盘空间、git 状态、当前 stable case 数/重复项、stable300 final summaries/candidate matrix/delivery report。不得只依赖历史提示词或记忆。

阶段门：stable315 -> stable330 -> stable350。每阶段只 promotion RV+LA × musl+glibc 全 clean 的新增 case；使用 `python3 -B scripts/ltp_summary.py` 解析 targeted 与 aggregate gate；wrapper 成功不等于可 promotion。

候选优先：access02/access04/chmod05/statx01、writev03/pipe2_02、waitpid01、mmap04/mmap05/mmap06/mprotect01/mprotect02/munmap01 及相邻高收益权限/VFS、fd/pipe/iovec、process/wait/rlimit/proc、mmap/signal/time cases。上一轮 blocker 只能作为候选/修复入口，不能当作 clean evidence。

强约束：不伪造 PASS，不按 case name hardcode，不把真实 TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap 转成 SKIP/TCONF/PASS；timeout 不计 PASS；`read02` pass_with_tconf 持续透明披露；marker 行必须保持行首 `PASS LTP CASE`/`FAIL LTP CASE`。

Leader 拥有 `.omx/ultragoal`、`LTP_STABLE_CASES` 最终修改、promotion 决策和 final gate；Team workers 只做 discovery/修复/验证/报告切片，不能 checkpoint Ultragoal 或最终修改 stable list。

最终交付：live 350 unique；RV/LA final stable gate 各 PASS LTP CASE 700, FAIL 0, ltp-musl 350/0, ltp-glibc 350/0；TFAIL/TBROK=0，除已披露 known read02 TCONF 外无新增 TCONF；timeout/ENOSYS/panic/trap=0；marker-prefix 0 bad；fmt/build/code-review/ai-slop-cleaner clean；提交 agent-owned 变更。
