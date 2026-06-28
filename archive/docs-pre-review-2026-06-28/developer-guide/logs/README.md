# 开发者日志整理

这个目录把此前分散在阶段目录、raw log、summary 和质量门禁 JSON 里的关键信息整理成开发者可读的索引文档。这里**不复制大体积 raw log**，只保留结论、风险、证据路径和继续开发时应该先看的文件。

## 阅读顺序

1. [`evaluation-log.md`](evaluation-log.md)：本地/远程评测、离线构建和 LoongArch 地址映射相关日志整理。
2. [`ltp-score-history.md`](ltp-score-history.md)：LTP stable 集合从 300 到当前 stop-state 的阶段演进和证据路径。
3. [`current-stop-state.md`](current-stop-state.md)：当前 live 状态、可信证据、未完成门禁和下一步入口。
4. [`blocker-index.md`](blocker-index.md)：按子系统整理的阻塞 case、失败形态和下一步最小行动。
5. [`log-reading-rules.md`](log-reading-rules.md)：如何读 wrapper marker、`scripts/ltp_summary.py` 输出、远程截断日志和噪声日志。

## 使用原则

- 这里是**索引和解读层**，不是原始证据本身。需要复核时回到文中列出的 summary/raw/report 路径。
- 当前数量必须从 `examples/shell/src/cmd.rs::LTP_STABLE_CASES` live 读取；历史日志里的 stableN 只是当时快照。
- LTP 结论必须以 `scripts/ltp_summary.py` 解析结果为准，不能只看 `run-eval` 或 QEMU 退出码。
- 任何包含 timeout、TFAIL、TBROK、未解释 TCONF、ENOSYS、panic/trap 的结果都不能当作 clean promotion 证据。
- 并发 QEMU 或被用户中止的 raw log 必须标注为 aborted/untrusted，不能补写成通过。

## 当前摘要

- 当前 live `LTP_STABLE_CASES`：383 total / 383 unique / 0 duplicates。
- 每架构默认 stable 执行量：383 case × `/musl` + `/glibc` = 766 wrapper case events。
- 当前 stop-state：stable383 被保留；stable400、stable450 未完成。
- 已知透明 caveat：`read02` 的 O_DIRECT-on-tmpfs `TCONF` 继续单独披露。
- 当前 runner completed-case marker：`FAIL LTP CASE <case> : <status>`，其中 status `0` 表示 wrapper pass；parser 同时兼容历史/外部 `PASS LTP CASE <case> : 0`。
