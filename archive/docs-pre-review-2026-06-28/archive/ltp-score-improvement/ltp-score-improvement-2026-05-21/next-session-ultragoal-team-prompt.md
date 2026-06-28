# Next session prompt: LTP score improvement with Ultragoal + Team

```text
$ultragoal $team

目标：继续提高 /root/oskernel2026-orays 的 LTP 测试成绩。请读取并遵循仓库 AGENTS.md，继续使用 Team + Ultragoal 分阶段执行。

当前状态：
- 上一轮已完成 Team + Ultragoal，ultragoal 117/117 complete。
- LTP stable batch 已从原 16 项扩展到 44 项/每 libc/每架构。
- 最终验证：
  - cargo fmt --all -- --check: exit 0
  - ./run-eval.sh la 2>&1 | tee output_la.md: exit 0
  - ./run-eval.sh 2>&1 | tee output_rv.md: exit 0
  - LA: PASS LTP CASE 88, FAIL LTP CASE 0, ltp-musl 44/0, ltp-glibc 44/0, internal TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
  - RV: PASS LTP CASE 88, FAIL LTP CASE 0, ltp-musl 44/0, ltp-glibc 44/0, internal TCONF=4, LTP timeout=0, ENOSYS=0, panic/trap=0
- 关键证据：
  - output_la.md
  - output_rv.md
  - docs/ltp-score-improvement-2026-05-21/final-gate-output-la-summary.txt
  - docs/ltp-score-improvement-2026-05-21/final-gate-output-rv-summary.txt
  - docs/ltp-score-improvement-2026-05-21/final-gate-code-review-report.md
  - docs/ltp-score-improvement-2026-05-21/final-gate-quality-gate.json
- 当前重要实现：
  - examples/shell/src/cmd.rs: stable/batch LTP runner、per-case timeout、memory stats
  - scripts/ltp_summary.py: LA/RV + musl/glibc case matrix、TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 分类
  - examples/shell/src/uspace/*: fd flags、metadata/symlink/statx/umask、/proc/status、mmap VA hole search 等真实 ABI 行为增强

本轮目标：
1. 继续提升 LTP 分数，不伪造 PASS、不 hardcode case name、不把真实失败转 SKIP。
2. 在 44 stable cases 基础上，目标扩展到 60-80 stable cases。
3. 继续使用小 batch / targeted validation，不要一开始跑完整 ./run-eval.sh la 和 ./run-eval.sh。
4. 优先找“低风险、高收益、两架构都可验证”的 case：
   - fs metadata/open/link/rename/statfs/access variants
   - proc/read-only metadata cases
   - time/signal basic cases
   - wait/exit/reporting cases
   - mmap/brk/msync 近邻 cases
5. 对仍存在的 hard blockers 单独 lane 调查，但不要阻塞 stable batch 提分：
   - RV CVE/OOM/full-LTP memory pressure
   - cve-2017-17053 非 x86 TCONF，不要错误纳入 stable
   - LA 用户态 crash/trap 相关 cases
6. 每次 promotion 必须说明：
   - 新增 case 列表
   - 为什么可以加入 stable
   - LA/RV × musl/glibc 证据
   - internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 是否为 0 或被透明记录
7. 保存新文档到：
   - docs/ltp-score-improvement-2026-05-22/
   或当前日期对应目录。

团队分工建议：
- Leader:
  - 维护 Ultragoal ledger
  - 控制 promotion gate
  - 最终集成和验收
- Discovery lane:
  - 从 sdcard-rv.img / sdcard-la.img 枚举更多 LTP cases
  - 基于上一轮 stable 44 生成下一批候选 batch
  - 先跑 LA/RV 小批量验证并保存 raw logs
- Runner/Harness lane:
  - 检查 examples/shell/src/cmd.rs runner 是否需要更好的 batch 文件/inline 配置
  - 保持 timeout 计数为 FAIL/TIMEOUT，绝不 PASS
- Stats/Report lane:
  - 继续增强 scripts/ltp_summary.py，如需要输出 promotion candidate report
  - 汇总新增 case matrix
- Syscall/ABI lane:
  - 根据 discovery 结果修最确定的 ENOSYS/wrong errno/metadata/proc/wait/time/signal 问题
  - 重点文件：
    - examples/shell/src/uspace/syscall_dispatch.rs
    - examples/shell/src/uspace/fd_table.rs
    - examples/shell/src/uspace/metadata.rs
    - examples/shell/src/uspace/synthetic_fs.rs
    - examples/shell/src/uspace/memory_map.rs
    - examples/shell/src/uspace/process_lifecycle.rs
    - examples/shell/src/uspace/signal_abi.rs
    - examples/shell/src/uspace/time_abi.rs
- Hard-blocker lane:
  - 单独调查 RV full-LTP CVE/OOM 和 LA crash/trap，不作为第一批 stable promotion 阻塞项

硬性要求：
- 不允许通过 hardcode case name 返回 PASS。
- 不允许把真实失败静默转 SKIP。
- timeout 必须单独计数，不能算 PASS。
- 不要只看 run-eval exit code；必须用 scripts/ltp_summary.py 看 LTP 内部健康。
- 默认 stable runner 必须明确、可复现、文档化。
- 每轮修改后先跑 targeted batch，再考虑最终完整验证。
- 最终交付前必须跑：
  - cargo fmt --all -- --check
  - ./run-eval.sh la 2>&1 | tee output_la.md
  - ./run-eval.sh 2>&1 | tee output_rv.md
  - python3 scripts/ltp_summary.py output_la.md
  - python3 scripts/ltp_summary.py output_rv.md
- 最终报告必须包括：
  - 修改文件
  - 修改函数
  - 每项修复的预期行为
  - 实际验证命令
  - LA/RV pass/fail 汇总
  - internal TFAIL/TBROK/TCONF
  - timeout / ENOSYS / panic/trap
  - stable batch 新增 case
  - 未完成风险和下一批建议

请先读取：
- .omx/ultragoal/goals.json
- .omx/ultragoal/ledger.jsonl
- docs/ltp-score-improvement-2026-05-21/final-gate-output-la-summary.txt
- docs/ltp-score-improvement-2026-05-21/final-gate-output-rv-summary.txt
- docs/ltp-score-improvement-2026-05-21/final-gate-code-review-report.md
- scripts/ltp_summary.py
- examples/shell/src/cmd.rs

然后创建新的 Ultragoal plan，启动 Team，多 lane 并行推进下一批 LTP 提分。
```
