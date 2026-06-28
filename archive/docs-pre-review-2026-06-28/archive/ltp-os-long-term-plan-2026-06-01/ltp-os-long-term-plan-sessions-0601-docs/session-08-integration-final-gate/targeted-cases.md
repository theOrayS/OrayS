# Session 8 targeted cases

本 session 的 targeted unit 是当前 live `LTP_STABLE_CASES` 全量 stable gate：`506` 个 case，单位为 **case**，不是“组”。完整 case 清单见同目录 `targeted-cases.txt`（一行一个 case）。

运行范围：

- RV: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh rv`
- LA: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh la`

解释：`OSCOMP_TEST_GROUPS=ltp` 用于只跑 LTP stable list，避免把基础 libc/shell 组噪声混入 Session 8 promotion gate；最终判断仍以 `scripts/ltp_summary.py` parser summary 为准。
