# stable300 delivery report

Status: **NOT DELIVERED**.

## Delivery outcome

- Requested target: stable300.
- Delivered, verified target in this run: **stable270**.
- Live `LTP_STABLE_CASES`: 270 entries, 270 unique, 0 duplicates.
- Stable300 final gates were **not run**, because the live stable list is not 300 and promotion evidence is insufficient.

## Why stable300 is blocked

The run produced a real stable270 promotion with RV+LA aggregate proof. Post270 discovery did not find enough clean candidates for stable285/300:

- only four additional fully clean targeted candidates were found (`sched_getattr02`, `open09`, `fcntl23`, `getegid01_16`);
- user-priority permission/pipe/mmap groups still contain real failures, timeouts, ENOSYS, TCONF, or TBROK and cannot be promoted truthfully;
- wait/fork expansion promoted the clean `waitid*` subset, but `waitpid01` and several harder waitid/kill cases still fail;
- timer/openat2/close_range/statx discovery exposed missing or incomplete syscall/device semantics, not clean candidates.

## Stable270 final evidence retained as highest trusted gate

| Arch | Summary | Result |
| --- | --- | --- |
| RV | `stable270-rv-aggregate-summary.txt` | PASS LTP CASE 540, FAIL 0; ltp-musl 270/0; ltp-glibc 270/0; TFAIL/TBROK 0; known `read02` TCONF only; timeout/ENOSYS/panic-trap 0 |
| LA | `stable270-la-aggregate-summary.txt` | PASS LTP CASE 540, FAIL 0; ltp-musl 270/0; ltp-glibc 270/0; TFAIL/TBROK 0; known `read02` TCONF only; timeout/ENOSYS/panic-trap 0 |

## Not run

- stable285 aggregate gate: not run because stable285 was not promoted.
- stable300 aggregate gate: not run because stable300 was not promoted.
- remote submission `make all`: not run as a final remote gate in this run; `run-eval.sh` builds produced fresh root `kernel-rv`/`kernel-la` artifacts but those generated files are not committed.
