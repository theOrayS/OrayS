# stable413 delivery report

Date: 2026-05-26
Mode: Ultragoal + Team, leader-owned final gates.
Stop state: stable413 main goal delivered; stretch stable423 deferred.
Final evidence set: `stable413-*-final-gate-002`, re-run after code-review fixes.

## Live stable count

Fresh live count from `examples/shell/src/cmd.rs::LTP_STABLE_CASES`:

- Total: 413
- Unique: 413
- Duplicates: 0

## Cases promoted this round

Stage 1, stable393:

- `preadv01`, `preadv02`, `pwritev01`, `pwritev02`
- `pread02`, `pread02_64`, `pwrite02`, `pwrite02_64`
- `pwrite04`, `pwrite04_64`

Stage 2, stable403:

- `sendfile02`, `sendfile02_64`, `sendfile03`, `sendfile03_64`
- `sendfile04`, `sendfile04_64`, `sendfile05`, `sendfile05_64`
- `sendfile06`, `sendfile06_64`

Stage 3, stable413:

- `sendfile08`, `sendfile08_64`
- `preadv201`, `preadv201_64`, `preadv202`, `preadv202_64`
- `pwritev201`, `pwritev201_64`, `pwritev202`, `pwritev202_64`

## Final stable413 gates

| Arch | Command | Summary | Result |
| --- | --- | --- | --- |
| RV | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 110m ./run-eval.sh rv` | `raw/stable413-rv-final-gate-002-summary.txt` | PASS LTP CASE 826, FAIL 0; `ltp-musl` 413/0; `ltp-glibc` 413/0 |
| LA | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 120m ./run-eval.sh la` | `raw/stable413-la-final-gate-002-summary.txt` | PASS LTP CASE 826, FAIL 0; `ltp-musl` 413/0; `ltp-glibc` 413/0 |

Both final summaries report:

- Internal TFAIL/TBROK/TCONF: 4, from known transparent `read02` TCONF only.
- Timeout matches: 0.
- ENOSYS/not implemented matches: 0.
- Panic/trap matches: 0.

The earlier `stable413-*-final-gate-001` logs are superseded by the `002` evidence because code review found two `sendfile` side-effect/offset issues that were fixed before final delivery.

## Code-review fixes before final evidence

The blocking review issues were fixed before running `stable413-*-final-gate-002`:

1. `sendfile(offset_ptr != NULL)` now validates the user offset pointer for write before performing FD I/O, preventing side effects before discovering an invalid copy-out destination.
2. `sendfile(offset_ptr == NULL)` now reads from the input fd without committing the offset first, then advances the input fd only by the actual output bytes written, preserving partial-write semantics.

## Implementation summary

Changed source files:

- `examples/shell/src/cmd.rs`: appended 30 four-way clean cases to `LTP_STABLE_CASES`.
- `examples/shell/src/uspace/fd_table.rs`: added general syscall behavior for negative positioned offsets, O_APPEND positioned writes, `sendfile`, and `preadv2`/`pwritev2` flag/offset handling.
- `examples/shell/src/uspace/syscall_dispatch.rs`: routed `__NR_sendfile`, `__NR_preadv2`, and `__NR_pwritev2` through real syscall dispatch.

This is not a testcase-name special case: no promoted LTP case names appear in `fd_table.rs` or `syscall_dispatch.rs`.

## User-visible / ABI-visible behavior changes

- `pread64`/`pwrite64` now return `EINVAL` for negative signed offsets instead of treating them as large unsigned offsets.
- Positioned writes now honor `O_APPEND` by writing at the current file size.
- `sendfile` is newly implemented for the promoted regular-file paths, including optional offset pointer copy-in/copy-out and RLIMIT_FSIZE-aware output writes through the existing FD writer.
- `preadv2`/`pwritev2` are newly dispatched; non-zero flags return `EOPNOTSUPP`, offset `-1` uses and advances the file descriptor offset, offsets below `-1` return `EINVAL`, and non-negative offsets use positioned vector I/O.

Known follow-up risks from architecture review are documented in `final-gate-code-review-report.md`; they do not invalidate the stable413 aggregate evidence but should guide the next hidden-test hardening round.

## Guardrails

- No fake PASS.
- No case-name hardcoding.
- No LTP source edits.
- No conversion of real failure/timeout/ENOSYS/panic/trap into SKIP/TCONF/PASS.
- Promotion relied on `scripts/ltp_summary.py` summaries, not wrapper exit status alone.
