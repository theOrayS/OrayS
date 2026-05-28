# stable403 promotion gate report

Date: 2026-05-26
Baseline: stable393 delivered in this phase.
Promotion target: stable403.

## Promoted cases

Stage 2 added 10 sendfile cases:

- `sendfile02`, `sendfile02_64`
- `sendfile03`, `sendfile03_64`
- `sendfile04`, `sendfile04_64`
- `sendfile05`, `sendfile05_64`
- `sendfile06`, `sendfile06_64`

## Promotion evidence

| Gate | Summary | Result |
| --- | --- | --- |
| RV stable403 | `raw/stable403-rv-gate-001-summary.txt` | PASS LTP CASE 806, FAIL 0; `ltp-musl` 403/0, `ltp-glibc` 403/0 |
| LA stable403 | `raw/stable403-la-gate-001-summary.txt` | PASS LTP CASE 806, FAIL 0; `ltp-musl` 403/0, `ltp-glibc` 403/0 |

Both parser summaries reported:

- Internal TFAIL/TBROK/TCONF: 4, all transparent known `read02` TCONF.
- Timeout matches: 0.
- ENOSYS/not implemented matches: 0.
- Panic/trap matches: 0.

## Decision

Promoted stable403. The sendfile subset was promoted only after parser-clean RV+LA aggregate gates; wrapper success alone was not used as promotion evidence.
