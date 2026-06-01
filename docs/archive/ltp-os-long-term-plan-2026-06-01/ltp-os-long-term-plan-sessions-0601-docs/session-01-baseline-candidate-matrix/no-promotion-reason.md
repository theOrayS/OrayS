# No Promotion Reason — Session 1

Session 1 deliberately does not modify `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.

Reasons:

1. 本 session 是 baseline/candidate-matrix，只把 full-sweep summary/raw log 转成下一步 targeted 输入。
2. full-sweep clean/status0 只能作为 scouting evidence；promotion 仍要求 fresh targeted RV + LA × musl + glibc parser-clean gate。
3. 当前没有本 session 新跑的 targeted RV/LA 日志，也没有相邻 stable regression gate。
4. blacklist/SKIP/status0 不计 PASS，不作为 stable promotion 证据。

可后续推广的前置条件：从 `targeted-cases.txt` 选小批，串行跑 RV/LA evaluator，`python3 -B scripts/ltp_summary.py` 对 raw log parser-clean，确认无新增 `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`，再由 leader 修改 stable list。
