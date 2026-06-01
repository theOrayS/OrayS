# 2026-06-01 LTP OS long-term plan archive

本目录归档 2026-06-01 长期 LTP/OS 完善计划及其 Session 0~8 交付文档。该计划已完成最低交付目标，活跃工作不再继续在 `docs/` 顶层展开。

## Archived scope

- 主计划：`ltp-os-long-term-improvement-plan-2026-06-01.md`
- Session 文档：`ltp-os-long-term-plan-sessions-0601-docs/`
  - Session 0~8 的 report、targeted cases、validation、promotion/no-promotion、ABI/behavior、blacklist/final gate/next prompt 等文件均保留原子目录结构。

## Final result snapshot

- 最终 Session 8 commit：`fc13f705 Complete long-term LTP plan session 08 with stable506 final gate`
- 最高可信 stable：`506 total / 506 unique / 0 duplicate`
- Final report：`ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md`
- 下一轮 prompt：`ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/next-round-prompt.md`
- RV final gate：`PASS LTP CASE 1012`、`FAIL 0`，仅 inherited `read02` TCONF；timeout/ENOSYS/panic/trap 均为 0。
- LA final gate：`PASS LTP CASE 1012`、`FAIL 0`，仅 inherited `read02` TCONF；timeout/ENOSYS/panic/trap 均为 0。
- LA-only blacklist 在 Session 7 从 `376` 降到 `374`；移除项仍是 ordinary FAIL/TBROK/TCONF，不计 PASS。

## Boundaries

- 本归档只移动文档，不移动 raw logs、`target/` 证据、kernel/image/build 产物或 active blacklist 目录。
- `docs/ltp-full-sweep-blacklist-2026-05-30-arch/` 仍是当前 Makefile 默认 blacklist 来源，未归档。
- 若下一轮继续 stable506 -> stable520+，请从本目录中的 `next-round-prompt.md` 复制启动提示词，并 live 复核当前 stable count 与 blacklist 状态。
