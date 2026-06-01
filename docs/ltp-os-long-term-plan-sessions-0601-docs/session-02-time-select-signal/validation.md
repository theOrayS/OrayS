# Validation - Session 2

## Pre/build checks

- `rustfmt examples/shell/src/cmd.rs examples/shell/src/uspace/{mod.rs,process_lifecycle.rs,time_abi.rs,signal_abi.rs,select_fdset.rs,syscall_dispatch.rs}`：通过。
- `make A=examples/shell ARCH=riscv64`：通过；命令会串行构建 RV 与 LA evaluator kernels。已知 vendor warning 保持原样。
- 长跑/QEMU 前后 `df -h / /root`：均显示 `/` 与 `/root` 约 41% 使用，约 34G 可用。

## Live stable count

```text
before Session 2 promotion: 460 total / 460 unique / 0 duplicate
post Session 2 promotion:   462 total / 462 unique / 0 duplicate
```

## Parser-backed runtime evidence

### RV time/select/signal batch

- 命令：

```bash
cases='getitimer01,ppoll01,select02,clock_gettime04,clock_nanosleep02,nanosleep01,poll02,pselect01,pselect01_64,settimeofday01,time-schedule'
LTP_CASES="$cases" ./run-eval.sh rv > target/ltp-long-term-session2/session2-rv-time-select.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session2/session2-rv-time-select.log > target/ltp-long-term-session2/session2-rv-time-select-summary.txt
```

- Summary artifact committed: `summary-rv-time-select.txt`
- Raw log path（未提交）: `target/ltp-long-term-session2/session2-rv-time-select.log`
- Raw log sha256: `3011f43b4839084a6933743030054d66e8f493e76f52facd32bd5c21927c955a`
- Parser headline: `PASS LTP CASE 18`, `FAIL LTP CASE 4`, internal `{'TCONF': 2, 'TFAIL': 3}`, timeout `2`, ENOSYS `0`, panic/trap `0`。
- Clean/pass cases include `getitimer01` and `ppoll01` for both RV musl/glibc.
- Known failures: `select02` timeout+TCONF, `clock_gettime04` RV musl TFAIL, `nanosleep01` RV musl TFAIL。

### LA promotion confirmation

- 命令：

```bash
cases='getitimer01,ppoll01'
LTP_CASES="$cases" ./run-eval.sh la > target/ltp-long-term-session2/session2-la-getitimer-ppoll.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session2/session2-la-getitimer-ppoll.log > target/ltp-long-term-session2/session2-la-getitimer-ppoll-summary.txt
```

- Summary artifact committed: `summary-la-getitimer-ppoll.txt`
- Raw log path（未提交）: `target/ltp-long-term-session2/session2-la-getitimer-ppoll.log`
- Raw log sha256: `d2991154b76be0c23e1ddacc9abbdb6b33b6c2a3efd576daf02e4a9d0099d893`
- Parser headline: `PASS LTP CASE 4`, `FAIL LTP CASE 0`, internal `{}`, timeout `0`, ENOSYS `0`, panic/trap `0`。

### LA adjacent regression

- 命令：

```bash
cases='poll02,pselect01,pselect01_64'
LTP_CASES="$cases" ./run-eval.sh la > target/ltp-long-term-session2/session2-la-poll-pselect-regression.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session2/session2-la-poll-pselect-regression.log > target/ltp-long-term-session2/session2-la-poll-pselect-regression-summary.txt
```

- Summary artifact committed: `summary-la-poll-pselect-regression.txt`
- Raw log path（未提交）: `target/ltp-long-term-session2/session2-la-poll-pselect-regression.log`
- Raw log sha256: `c30ff6ac1104df8d36db8b62ee99d095a154d67e7e92d26ce0670ab6e4859434`
- Parser headline: `PASS LTP CASE 6`, `FAIL LTP CASE 0`, internal `{}`, timeout `0`, ENOSYS `0`, panic/trap `0`。

## 未验证项

- 未运行完整 stable462 四路 gate；按主计划保留到 Session 8 integration/final gate。
- 未对 `select02` 调整 wrapper timeout 或 legacy `__NR_select` TCONF；本 session 只分类，不推广。
- 未实现 `ITIMER_VIRTUAL`/`ITIMER_PROF` 的 CPU accounting 与 SIGVTALRM/SIGPROF delivery。
