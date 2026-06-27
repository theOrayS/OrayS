# RV iter002 parser closure report

Date: 2026-05-29T14:35:20Z
Worker: `worker-3` / task 12 (`Parse RV iteration 002 after closure`)
Raw log: `/root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter002.log`
Log SHA-256: `1a1d2ebdc680e1bec848733d5427dea3e30477602b79915b7310659fbb4a2b3c`
Closure: **closed** (`exit_meta=RUN_META exit_code=2 end=2026-05-29T14:35:03Z`, open writers=0)

## Counts

| Field | Count |
| --- | ---: |
| `RUN LTP CASE` | 1108 |
| parser normalized PASS | 293 |
| parser normalized FAIL | 814 |
| raw `PASS LTP CASE` markers | 0 |
| raw `FAIL LTP CASE` markers | 1107 |
| raw `FAIL ... : 0` wrapper-pass candidates | 293 |
| raw non-zero wrapper failures | 814 |
| `TIMEOUT LTP CASE` markers | 16 |
| SKIP markers | 0 |
| incomplete `RUN` markers | 1 |
| internal `TFAIL` | 1362 |
| internal `TBROK` | 296 |
| internal `TCONF` | 451 |
| ENOSYS/not implemented matches | 190 |
| panic/trap matches | 0 |

## Incomplete cases

- oom01

## Artifacts

- `rv-iter002-summary.txt` — `python3 scripts/ltp_summary.py <raw-log>` output.
- `rv-iter002-summary-compact.json` — compact parser/raw marker counts.
- `rv-iter002-marker-audit.json` — marker, incomplete, metadata, and tail-marker evidence.
- `rv-iter002.log.sha256` — raw-log hash.

## Guardrails

- This parser task did not edit the shared blacklist or Ultragoal state.
- `SKIP`/blacklist lines are not counted as PASS.
- `FAIL LTP CASE <case> : 0` is treated as wrapper pass by the parser; non-zero status remains failure.
- Any open `RUN LTP CASE` at log end is reported as incomplete, not PASS.

## Subagent integration

- Subagent spawned: 1 (`019e73a9-026f-7c03-9e66-ffb6877649eb`, change-slice parser/report probe).
- Integrated findings: parser commands, expected artifact fields, and closure hazards.
