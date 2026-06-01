# Session 8 report: integration final gate

Commit SHA: this Session 8 commit; exact SHA is reported in the final response and `git log -1 --oneline`.
Previous session commit: `8b00b494` (Session 7 LA severe blocker reduction).

## Goal

整合 Session 1~7 的路线图成果，live 复核 stable list，串行运行 RV/LA × musl/glibc stable gate，整理 blacklist diff，产出 final report 与下一轮 prompt。Session 8 不再追加 promotion；它只确认最高可信 stableN。

## Changes

- 写入 Session 8 文档与最终交付物：
  - `final-report.md`
  - `validation.md`
  - `targeted-cases.md` / `targeted-cases.txt`
  - `no-promotion-reason.md`
  - `blacklist-change-report.md`
  - `code-review-cleanup-noise-check.md`
  - `next-round-prompt.md`
- 未修改源码、testsuite、evaluator、stable list 或 blacklist。
- raw log / summary JSON 保留在 `target/ltp-long-term-session8/`，不提交。

## Evidence summary

- Live stable count before final gate: `506 total / 506 unique / 0 duplicate`。
- RV final gate: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh rv`
  - wrapper/parser summary: `PASS LTP CASE 1012`、`FAIL LTP CASE 0`。
  - suite: `ltp-musl 506 passed / 0 failed`，`ltp-glibc 506 passed / 0 failed`。
  - internal: only inherited known `read02` TCONF caveat (`rv:glibc:read02`, `rv:musl:read02`), total `TCONF 4`；timeout/ENOSYS/panic/trap all `0`。
- LA final gate: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh la`
  - wrapper/parser summary: `PASS LTP CASE 1012`、`FAIL LTP CASE 0`。
  - suite: `ltp-musl 506 passed / 0 failed`，`ltp-glibc 506 passed / 0 failed`。
  - internal: only inherited known `read02` TCONF caveat (`la:glibc:read02`, `la:musl:read02`), total `TCONF 4`；timeout/ENOSYS/panic/trap all `0`。
- Marker-prefix scan: both RV and LA logs have `0` non-prefix `LTP CASE` lines; no marker glue found.
- Disk pre/post gate remained `/dev/vda2 59G used 23G avail 34G use 41%`。

## Result

Session 8 is complete. 最高可信 stableN 为 `506`：从提示词创建时的 live stable460 增长到 stable506，且 RV/LA × musl/glibc final gate 均 wrapper PASS、无 FAIL、无 timeout/ENOSYS/panic/trap。唯一 parser-visible caveat 是 stable460 已知的 `read02` TCONF，不是本轮新增。

## Risks / limitations

- 未达到理想 stable520；本轮在 Session 6 达到 stable506 后未再追加可四路 clean 的新 case。
- Session 7 只降低 LA-only blacklist `2` 个条目；剩余 LA network/resource/allocator blockers 未在本 session 扩展处理。
- 未执行 Optional Session 9/10 的 network/proc/syntheticfs 或 full-sweep 再闭合；下一轮 prompt 已写入同目录。
- raw logs 未提交；只提交 summary/checksum/path 说明，符合仓库生成物约束。

## Next entry

若继续冲理想目标，应从 `next-round-prompt.md` 启动下一轮：优先 Session 9/10，聚焦 stable506→520 的高 ROI clean candidates、LA network/resource blacklist family、以及 all-minus-blacklist shard/full-sweep 质量审计。
