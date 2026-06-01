# Session 3 promotion candidates

Promotion decision: **promote 4 cases** into `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.

## Promoted cases

- `fcntl11`
- `fcntl14`
- `fcntl19`
- `fcntl22`

## Evidence boundary

Promotion is based only on fresh RV/LA × musl/glibc parser-clean targeted gates:

```text
RV final combined gate: PASS LTP CASE 26, FAIL 0, internal {}, timeout 0, ENOSYS 0, panic/trap 0
LA final combined gate: PASS LTP CASE 26, FAIL 0, internal {}, timeout 0, ENOSYS 0, panic/trap 0
```

`python3 -B scripts/ltp_summary.py --promotion-candidates target/ltp-long-term-session3/session3-rv-fd-final-combined.log target/ltp-long-term-session3/session3-la-fd-final-combined.log` reported 13 four-way clean cases and 0 blocked/incomplete cases; 9 were already stable regression cases, so this session promotes only the 4 new cases listed above.

## Not promoted

- `fcntl30`: `/proc/sys/fs/pipe-max-size` synthetic/procfs dependency.
- `pipe07`: `/proc/self/fd` synthetic/procfs dependency.
- `pipe15`: `/proc/sys/fs/pipe-user-pages-soft` synthetic/procfs dependency.
- `writev03`: TCONF/unsupported device-style condition.
- `pwritev03`: TBROK/`ENOSPC` while creating `test_dev.img`.

No blacklist changes were made; no SKIP/status0 evidence was counted as PASS.
