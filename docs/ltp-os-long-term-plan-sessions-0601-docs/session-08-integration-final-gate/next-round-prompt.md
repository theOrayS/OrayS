# 下一轮启动提示词：stable506 -> stable520+ 与 full-sweep 质量审计

创建日期：2026-06-01
目标仓库：`/root/oskernel2026-orays`
建议起点分支：当前 `dev/long-term-plan-0601` 或从最新 Session 8 commit 新建 `dev/ltp-stable506-to-520-plus`
当前最高可信 stable：`506 total / 506 unique / 0 duplicate`
上轮 final report：`docs/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md`

````text
我要继续 `/root/oskernel2026-orays` 的 LTP/OS 长期完善任务，从 stable506 冲 stable520+，并补 Optional Session 9/10 的质量审计。

请按仓库 AGENTS.md 执行，中文汇报。默认自治推进：能安全执行的读代码、写文档、修补、targeted 验证、总结、提交都直接做；只有破坏性、外部凭据/远程生产、或会改变长期方向的分叉决策才问我。

必须先读取：
- `AGENTS.md`
- `docs/agent-workflow/repo-basics.md`
- `docs/agent-workflow/commands-and-validation.md`
- `docs/agent-workflow/ltp-selection.md`
- `docs/agent-workflow/ltp-promotion-and-docs.md`
- `docs/agent-workflow/coding-boundaries.md`
- `docs/agent-workflow/collaboration-and-delivery.md`
- `docs/agent-workflow/branch-policy.md`
- `docs/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md`
- `docs/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/validation.md`

Live 复核，不要只信本提示词：
```bash
pwd
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
git status --short
```

当前事实：
- stable506 final gate 已闭合：RV/LA × musl/glibc 均 `PASS LTP CASE 1012`、`FAIL 0`。
- 唯一 parser-visible caveat 是 inherited `read02` TCONF：RV/LA 各 `TCONF 4`，无 timeout/ENOSYS/panic/trap。
- active blacklist counts：common `5`，RV `1`，LA `374`。
- LA Session 7 只移除了 `creat07` 与 `tcp4-uni-basic01`；它们只是 ordinary FAIL/TBROK/TCONF，不是 PASS，不可 promotion。

下一轮优先目标：
1. Stable506 -> stable520+：从 Session 1 candidate matrix 和 final gate 之外的 clean/near-clean cases 中挑 14~25 个高 ROI 候选。
2. Optional Session 9：network/socket 与 proc/syntheticfs 语义；目标是减少 LA network blacklist 和 synthetic shim 风险，不硬冲 PASS。
3. Optional Session 10：all-minus-blacklist 或 shard sweep 质量审计，至少证明 selected shard `incomplete_count=0`、panic/trap=0、resource failure=0。
4. 保持每个 roadmap session 独立文档目录与独立 Lore commit；raw logs 不提交，只提交 summary/checksum/path。

红线：
- 不 fake PASS，不 hardcode LTP case/path/process name/output。
- 不修改 testsuite/evaluator 绕过失败。
- blacklist/SKIP/status0 不是 PASS，也不是 stable promotion evidence。
- promotion 必须 RV + LA × musl + glibc wrapper PASS，并用 `scripts/ltp_summary.py` 确认无新增 TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap；已知 `read02` caveat 必须继续显式报告。
- 长跑/QEMU/Docker/evaluator 前后运行 `df -h / /root`。
````
