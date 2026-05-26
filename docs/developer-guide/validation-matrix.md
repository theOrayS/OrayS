# Validation Matrix

Choose the smallest check that can prove the claim, then expand only when the
change affects runtime behavior, evaluator scoring, or cross-architecture code.

## Quick matrix

| Change type | Minimum validation | Stronger / release validation |
| --- | --- | --- |
| Markdown/docs only | `git diff --check` | Link/path smoke checks when docs name files or commands. |
| Rust formatting-only change | `make fmt` or targeted `cargo fmt --check` | `make clippy` if code was also changed. |
| C formatting-only change | `make fmt_c` or targeted `clang-format --dry-run` | Build touched C example if behavior changed. |
| Normal Rust module change | Relevant build or `make clippy` | `make unittest_no_fail_fast` when unit-testable. |
| `examples/shell` behavior | `make A=examples/shell ARCH=riscv64 build` or `make kernel-rv` | Targeted RV/LA evaluator batch plus parser summary. |
| POSIX/syscall/ABI behavior | Shell build for affected arch | Targeted LTP cases on RV and LA; mention errno/ABI changes. |
| VFS/FD/pipe/process/signal/mmap fix | Targeted subsystem LTP cases | Adjacent high-score regressions plus RV/LA final gate. |
| Evaluator kernel artifact path | `make kernel-rv` and/or `make kernel-la` | `./run-eval.sh rv` and `./run-eval.sh la` if images/QEMU exist. |
| Remote-submission behavior | `make all` | Offline-style build with repo `cargo-home/` and helper shims. |
| LTP promotion | targeted case list + `scripts/ltp_summary.py` | RV+LA, musl+glibc clean matrix, then stable-list update. |

## Parser-backed evaluator proof

For any LTP/evaluator claim, keep the raw log and parser output together:

```bash
python3 scripts/ltp_summary.py raw.log > raw-summary.txt
python3 scripts/ltp_summary.py raw.log --json > raw-summary.json
```

Promotion reports should name:

- selected case list or batch;
- architecture (`rv`, `la`);
- libc groups (`musl`, `glibc`);
- wrapper pass/fail counts;
- internal `TFAIL`/`TBROK`/`TCONF` counts;
- timeout, ENOSYS, and panic/trap counts;
- missing checks, if any.

## When validation cannot run

If QEMU, sdcard images, Docker, cross toolchains, or testsuite checkout are
missing, say exactly which check could not run.  Do not replace runtime evidence
with a stronger-sounding build-only claim.

## Final-report checklist

Before calling work complete, report:

- files changed and why;
- commands actually run;
- commands not run and why;
- local `./run-eval.sh rv` / `./run-eval.sh la` status when evaluator behavior
  changed;
- `make all` status when remote-submission behavior changed;
- user-visible behavior changes;
- syscall, errno, or ABI-visible changes, or explicitly state that none were
  intended.
