# Full LTP evaluation attempt report

- Generated: `2026-05-19T13:51:10+08:00`
- Repository: `/root/oskernel2026-orays`
- Output directory: `eval-reports/full-ltp-20260519-132237`
- Requested scope: run full LTP, not the default `run-eval` smoke skip.
- Execution method: temporarily built `examples/shell` with the local LTP skip gate disabled and non-LTP groups filtered out; per-case wrapper used `busybox timeout 30` to prevent known hanging cases from blocking the whole sweep. Source file was restored afterward; no source diff remains from this run.

## Summary

| Arch | Command | Host/QEMU status | Started | Ended | Expected LTP files (musl+glibc) | Cases started | Last case | Group end seen | Stop reason |
| --- | --- | ---: | --- | --- | ---: | ---: | --- | --- | --- |
| riscv64 | `./run-eval` | `0` | `2026-05-19T13:22:37+08:00` | `2026-05-19T13:35:07+08:00` | 5660 | 242 | `cve-2017-17053` | no | guest memory exhaustion during `cve-2017-17052`; glibc LTP could not map ELF afterward |
| loongarch64 | `./run-eval la` | `0` | `2026-05-19T13:36:02+08:00` | `2026-05-19T13:48:20+08:00` | 5660 | 220 | `crash01` | no | kernel panic/unhandled trap in `crash01` |

## Detailed observations

### riscv64

- Expected files in image: musl `2820`, glibc `2840`.
- Started `242` LTP cases; no `ltp-musl` end marker was reached.
- The run hit `free_frames=0`, repeated frame allocation failures, and `failed to map ELF segment` when trying to enter `/glibc/ltp_testcode.sh`.
- Marker counts: TPASS `301`, TFAIL `28`, TBROK `65`, TCONF `96`, TWARN `12`, SKIP LTP CASE `45`.
- Full output Markdown: `eval-reports/full-ltp-20260519-132237/rv.full-ltp.output.md`
- Raw log: `eval-reports/full-ltp-20260519-132237/rv.full-ltp.raw.log`
- Clean log: `eval-reports/full-ltp-20260519-132237/rv.full-ltp.clean.log`

### loongarch64

- Expected files in image: musl `2820`, glibc `2840`.
- Started `220` LTP cases; no `ltp-musl` end marker was reached.
- The run stopped when `crash01` caused an unhandled `InstructionNotExist` trap and kernel panic, followed by platform shutdown.
- Marker counts: TPASS `302`, TFAIL `20`, TBROK `48`, TCONF `90`, TWARN `10`, SKIP LTP CASE `45`.
- Full output Markdown: `eval-reports/full-ltp-20260519-132237/la.full-ltp.output.md`
- Raw log: `eval-reports/full-ltp-20260519-132237/la.full-ltp.raw.log`
- Clean log: `eval-reports/full-ltp-20260519-132237/la.full-ltp.clean.log`

## Conclusion

Full LTP was attempted on both architectures. Neither architecture completed the entire LTP file set in the current kernel/evaluator environment: riscv64 exhausted guest memory during a CVE stress case, and loongarch64 panicked in `crash01`. The attached Markdown/log files are the requested evaluation outputs and include the complete captured console output for both attempts.
