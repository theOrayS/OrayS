# LTP score improvement final gate - 2026-05-22

## Result

Stable LTP batch was promoted from **44** to **63** cases per libc, per architecture. The final full evaluator gate completed with exit 0 on both LA and RV, and `scripts/ltp_summary.py` reports LTP stable health as **126 PASS / 0 FAIL per architecture**.

## Stable batch promotion

新增 19 cases:

`alarm02 alarm03 clock_gettime02 gettimeofday01 time01 times01 kill03 rt_sigaction01 sigaction01 proc01 exit01 exit02 exit_group01 getpgrp01 gettid01 uname01 getrlimit01 getrusage01 sched_yield01`

为什么可以加入 stable:

- 每个新增 case 都先经过 targeted batch 筛选，再进入 stable-63 targeted validation。
- LA/RV × musl/glibc 的 stable-63 targeted logs 均为 63/0 per libc。
- Final full `./run-eval.sh la` 和 `./run-eval.sh` 均验证 stable runner 的默认可复现结果。
- 没有按 case name hardcode PASS；没有把真实失败静默转 SKIP；timeout 仍由 runner/summary 单独记录。

## Final LTP summary

| Arch | PASS LTP CASE | FAIL LTP CASE | ltp-musl | ltp-glibc | internal TFAIL | internal TBROK | internal TCONF | LTP timeout | ENOSYS | panic/trap |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| LA | 126 | 0 | 63/0 | 63/0 | 0 | 0 | 4 | 0 | 0 | 0 |
| RV | 126 | 0 | 63/0 | 63/0 | 0 | 0 | 4 | 0 | 0 | 0 |

Notes:

- `read02` remains a transparent `pass_with_tconf` case: 2 TCONF per libc group, 4 per arch total. This was not hidden or converted to PASS-only evidence.
- `timeout matches: 10` in final summaries are non-LTP benchmark group timeouts (`libctest`, `lmbench`, `cyclictest`, `iozone`, `unixbench`). LTP group timeout is 0 for both `ltp-musl` and `ltp-glibc`.

## Validation commands

- `cargo fmt --all -- --check` -> 0 (`final-cargo-fmt.status`)
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img"` -> 0 (`rv-stable-63-targeted.status`)
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img"` -> 0 (`la-stable-63-targeted.status`)
- `./run-eval.sh la 2>&1 | tee output_la.md` -> 0 (`final-gate-run-eval-la.status`)
- `./run-eval.sh 2>&1 | tee output_rv.md` -> 0 (`final-gate-run-eval-rv.status`)
- `python3 -B scripts/ltp_summary.py output_la.md` -> `final-gate-output-la-summary.txt/json`
- `python3 -B scripts/ltp_summary.py output_rv.md` -> `final-gate-output-rv-summary.txt/json`

## Files and functions changed

- `examples/shell/src/cmd.rs`
  - `LTP_STABLE_CASES`: appended 19 independently validated cases. Expected behavior: default stable runner executes 63 cases per libc instead of 44.
- `output_la.md`, `output_rv.md`
  - Refreshed final full evaluator logs for this gate.
- `docs/ltp-score-improvement-2026-05-22/*`
  - Saved targeted logs/summaries, promotion candidate reports, final summaries, hard-blocker notes, and this gate report.

No syscall implementation, errno mapping, struct layout, or POSIX ABI logic was modified in this final promotion patch.

## Promotion candidate and blocked-case evidence

- Combined promotion candidate report: `62 clean candidates`, `26 blocked/incomplete candidates`.
- Cases explicitly not promoted include: `access02, access04, clock_getres01, clock_gettime01, dup03, fstatfs01, getpgid01, getsid01, kill02, link02, lseek02, mkdir02, nanosleep01, nanosleep02, pause01, pipe02, read02, rename01, rt_sigprocmask01, sigpending02, sigprocmask01, sigsuspend01, statfs01, statvfs01, sysinfo01, unlink05`.
- These blocked cases kept their TFAIL/TBROK/TCONF/timeout/ENOSYS evidence in candidate reports instead of being converted to stable.

## Remaining risks / next batch suggestions

1. Next stable candidates should come from the clean candidate set but should still be revalidated with a fresh LA/RV × musl/glibc targeted batch before promotion.
2. Filesystem metadata/statfs/access/link/rename variants still show real failures and should be fixed as ABI work before promotion.
3. Time/signal nearby blockers (`clock_gettime01`, `nanosleep*`, `sigprocmask*`, `pause01`) need separate syscall/signal semantics work.
4. Hard blockers remain non-blocking for stable score: RV full-LTP CVE/OOM pressure and LA crash/trap lanes should continue separately.
