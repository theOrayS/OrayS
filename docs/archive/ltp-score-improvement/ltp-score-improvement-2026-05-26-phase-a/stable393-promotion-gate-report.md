# stable393 promotion gate report

Date: 2026-05-26
Baseline: live stable383, 383 total / 383 unique / 0 duplicates.
Promotion target: stable393.

## Promoted cases

Stage 1 added 10 FD/vector/offset cases:

- `preadv01`, `preadv02`
- `pwritev01`, `pwritev02`
- `pread02`, `pread02_64`
- `pwrite02`, `pwrite02_64`
- `pwrite04`, `pwrite04_64`

## Promotion evidence

| Gate | Summary | Result |
| --- | --- | --- |
| RV stable393 | `raw/stable393-rv-gate-001-summary.txt` | PASS LTP CASE 786, FAIL 0; `ltp-musl` 393/0, `ltp-glibc` 393/0 |
| LA stable393 | `raw/stable393-la-gate-001-summary.txt` | PASS LTP CASE 786, FAIL 0; `ltp-musl` 393/0, `ltp-glibc` 393/0 |

Both parser summaries reported:

- Internal TFAIL/TBROK/TCONF: 4, all transparent known `read02` TCONF.
- Timeout matches: 0.
- ENOSYS/not implemented matches: 0.
- Panic/trap matches: 0.

## Decision

Promoted stable393. The added cases were RV+LA x musl+glibc clean under aggregate stable gates, with no新增 TFAIL/TBROK/TCONF, timeout, ENOSYS, panic, or trap beyond the known transparent `read02` TCONF.
