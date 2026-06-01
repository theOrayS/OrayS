# Session 2 promotion candidates / stable delta

本 session 由 leader 直接推广 2 个 case 到 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`：

- `getitimer01`
- `ppoll01`

推广依据：

- RV targeted batch：`getitimer01`、`ppoll01` 在 musl/glibc 均 `PASS LTP CASE ... : 0`，且 parser 记录 0 `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`。
- LA promotion confirmation：`getitimer01`、`ppoll01` 在 musl/glibc 均 `PASS LTP CASE ... : 0`，且 parser 记录 0 `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`。
- live stable 计数从 `460 total / 460 unique / 0 duplicate` 推进到 `462 total / 462 unique / 0 duplicate`。

不推广但作为相邻回归确认的 case：

- `poll02`
- `pselect01`
- `pselect01_64`

这些 case 在 RV batch 和 LA adjacent regression 均 parser-clean，但本 session 的稳定推广只聚焦新增真实语义修复直接解锁的 `getitimer01`/`ppoll01`，避免把相邻噪声样本一次性扩大为 stable 事实。

明确不推广：

- `select02`：RV musl/glibc 均 `FAIL 137`，parser 记录 `TCONF=1` 与 timeout=1；不是 PASS。
- `clock_gettime04`：RV musl 有内部 `TFAIL=1`；不是四路 clean。
- `nanosleep01`：RV musl 有内部 `TFAIL=2`；不是四路 clean。
