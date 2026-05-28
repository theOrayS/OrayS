# Team + Ultragoal Prompt: LTP Score Improvement

```text
$ultragoal $team

目标：继续提高 /root/oskernel2026-orays 的 LTP 测试成绩，采用 Team + Ultragoal 模式执行。请先读取并遵循仓库 AGENTS.md，再基于已有计划继续推进：

计划文件：
.omx/plans/ltp-score-improvement-2026-05-21.md

当前基线：
- 2026-05-20 已完成 LTP core 修复。
- LA/RV 的当前 LTP core runner 均为 16 项。
- 最终验证结果：
  - ./run-eval.sh la exit 0
  - ./run-eval.sh exit 0
  - ltp-musl: 16 passed, 0 failed
  - ltp-glibc: 16 passed, 0 failed
  - PASS LTP CASE: 32
  - FAIL LTP CASE: 0
  - ENOSYS/not implemented: 0
- 证据文件：
  - output_la.md
  - output_rv.md
  - docs/ltp-core-fixes-2026-05-20/final-evidence.md
  - scripts/ltp_summary.py

本轮目标：
1. 在不伪造结果、不硬编码 PASS、不屏蔽真实失败的前提下，提高 LTP 得分。
2. 使用 Team + Ultragoal 做分阶段目标管理和多 lane 并行。
3. 优先实现计划中的 Phase 1 和 Phase 2：
   - 增强 LTP batch runner。
   - 增加 per-case timeout。
   - 增强 scripts/ltp_summary.py，输出 LA/RV + musl/glibc case matrix。
   - 建立第一批扩展 LTP batch，例如：
     - syscalls-basic-plus
     - fs-basic
     - proc-basic
     - time-signal-basic
   - 跑小批量 LA/RV 验证，按 ENOSYS / wrong errno / missing proc-metadata / wait-exit-reporting / timeout-panic 分类。
   - 先修最确定、低风险、高收益的问题。
4. 中期目标：把稳定 LTP batch 从当前 16 项扩到 30-50 项。
5. 不要每次都跑完整 ./run-eval.sh la 和 ./run-eval.sh；阶段验证用小批量/定向测试即可。
6. 最终交付前必须跑完整：
   - cargo fmt --all -- --check
   - ./run-eval.sh la 2>&1 | tee output_la.md
   - ./run-eval.sh 2>&1 | tee output_rv.md
   - python3 scripts/ltp_summary.py output_la.md
   - python3 scripts/ltp_summary.py output_rv.md

团队分工建议：
- Leader：
  - 维护 Ultragoal ledger。
  - 控制 promotion gate。
  - 负责最终集成和验收。
- Runner/Harness lane：
  - 修改 examples/shell/src/cmd.rs。
  - 实现 batch runner、per-case timeout、case-list 配置。
- Stats/Report lane：
  - 修改 scripts/ltp_summary.py。
  - 输出 per-case matrix、timeout、ENOSYS、TFAIL/TBROK/TCONF、panic/trap 分类。
- Discovery lane：
  - 从 sdcard-rv.img / sdcard-la.img 中枚举 LTP case。
  - 生成第一批候选 batch。
  - 跑小批量测试并保存原始日志。
- Syscall/ABI lane：
  - 根据 discovery 结果修 ENOSYS、wrong errno、基础 syscall/ABI 缺口。
  - 重点目录：
    - examples/shell/src/uspace/syscall_dispatch.rs
    - examples/shell/src/uspace/fd_table.rs
    - examples/shell/src/uspace/memory_map.rs
    - examples/shell/src/uspace/process_abi.rs
    - examples/shell/src/uspace/process_lifecycle.rs
    - examples/shell/src/uspace/signal_abi.rs
    - examples/shell/src/uspace/time_abi.rs
    - examples/shell/src/uspace/synthetic_fs.rs
- Hard-blocker lane：
  - 单独调查但不要阻塞第一批提分：
    - LA crash01 / InstructionNotExist / 用户态 trap 转 signal
    - RV full-LTP CVE 附近 free_frames=0 / OOM / 资源释放

硬性要求：
- 不允许通过 hardcode case name 返回 PASS。
- 不允许把真实失败静默转成 SKIP。
- timeout 必须单独计数，不能算 PASS。
- 阶段性修改 runner 可以用于探索，但最终默认 runner 必须明确、可复现、文档化。
- 每次提升 stable batch 时，都要说明新增 case、通过条件、LA/RV 证据。
- 保存文档到一个带日期和任务描述的目录，例如：
  docs/ltp-score-improvement-2026-05-21/
- 最终报告必须包括：
  - 修改文件
  - 修改函数
  - 每项修复的预期行为
  - 实际验证命令
  - LA/RV pass/fail 汇总
  - 内部 TFAIL/TBROK/TCONF
  - timeout / ENOSYS
  - 未完成风险和下一批建议

请先读取计划文件和现有证据，然后启动 Team + Ultragoal，拆分目标并执行第一阶段。
```
