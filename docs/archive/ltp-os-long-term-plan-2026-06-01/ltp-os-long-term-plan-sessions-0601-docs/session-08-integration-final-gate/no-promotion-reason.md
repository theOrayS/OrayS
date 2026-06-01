# Session 8 no-promotion reason

Session 8 是最终整合和回归门禁，不是新增修复/promotion session。本 session 没有修改 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`，因此没有新的 promotion candidates。

最高可信 stableN 仍为 Session 6 推广后的 `506 total / 506 unique / 0 duplicate`。Session 8 只提供最终 RV/LA × musl/glibc parser-backed gate：

| Arch | Gate | PASS LTP CASE | FAIL LTP CASE | Internal caveat | timeout | ENOSYS | panic/trap |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: |
| RV | stable506 | 1012 | 0 | inherited `read02` TCONF only (`TCONF 4`) | 0 | 0 | 0 |
| LA | stable506 | 1012 | 0 | inherited `read02` TCONF only (`TCONF 4`) | 0 | 0 | 0 |

没有把 blacklist/SKIP/status0 或 Session 7 的 ordinary FAIL/TBROK/TCONF 当作 PASS，也没有从 final gate 外推新增 stable case。
