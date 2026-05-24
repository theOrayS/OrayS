# stable270 promotion gate report

Status: **PASS / delivered as highest trusted stage in this run**.

## Live stable list

- Source: `examples/shell/src/cmd.rs::LTP_STABLE_CASES`
- Result after promotion: 270 entries, 270 unique, 0 duplicates
- Snapshot: `stable270-live.cases`
- Added cases:
  - vector/positional IO: `writev05`, `writev06`, `writev07`, `readv02`, `writev01`, `preadv01_64`, `preadv02_64`, `pwritev01_64`, `pwritev02_64`
  - wait/signal: `waitid01`, `waitid02`, `waitid03`, `waitid04`, `waitid05`, `waitid06`, `waitid09`, `waitid11`, `kill07`, `kill08`, `kill09`

## Fixes backing promotion

- `read_iovec_entries` now rejects aggregate iovec overflow / `SSIZE_MAX` excess with `EINVAL`.
- `preadv` / `pwritev` are dispatched and implemented for regular files with positional offsets.
- File status access modes are enforced for regular file read/write and positional IO.
- `O_CREAT|O_RDONLY` may create a file without turning the resulting fd into a writable fd.
- `waitid` is implemented for exited children and writes Linux-shaped `siginfo_t` fields needed by LTP.
- Default terminating signal handling now records signal wait status, including core-dump bit where applicable.

## Targeted evidence used for promotion

| Evidence | Result |
| --- | --- |
| `fix-iovec-readv-writev-rv-summary.txt` | RV `readv02,writev01`: PASS 4, FAIL 0, internal 0, timeout/ENOSYS/panic 0 |
| `fix-iovec-readv-writev-la-summary.txt` | LA `readv02,writev01`: PASS 4, FAIL 0, internal 0, timeout/ENOSYS/panic 0 |
| `user-priority-rv-clean-writev-la-summary.txt` + RV priority matrix | `writev05,writev06,writev07`: RV+LA clean for both libc variants |
| `fix-waitid-rv-summary.txt` | RV `waitid01..04`: PASS 8, FAIL 0, internal 0, timeout/ENOSYS/panic 0 |
| `fix-waitid-la-summary.txt` | LA `waitid01..04`: PASS 8, FAIL 0, internal 0, timeout/ENOSYS/panic 0 |
| `target-waitid-extended-rv-summary.txt` | RV clean subset: `waitid05,waitid06,waitid09,waitid11`; `waitid07/08/10` blocked and not promoted |
| `target-kill-signal-rv-summary.txt` | RV clean subset: `kill07,kill08,kill09`; other kill cases blocked and not promoted |
| `target-waitid-kill-la-confirm-summary.txt` | LA confirmation for `waitid05,waitid06,waitid09,waitid11,kill07,kill08,kill09`: PASS 14, FAIL 0 |
| `fix-preadv-pwritev-rv2-summary.txt` | RV `preadv01_64,preadv02_64,pwritev01_64,pwritev02_64`: PASS 8, FAIL 0 |
| `fix-preadv-pwritev-la-summary.txt` | LA same positional IO batch: PASS 8, FAIL 0 |

## Stable aggregate gates

| Gate | Raw log | Parser summary |
| --- | --- | --- |
| RV stable270 | `raw/stable270-rv-aggregate-20260524T112750+0800.log` | `stable270-rv-aggregate-summary.txt`: PASS LTP CASE 540, FAIL 0; ltp-musl 270/0; ltp-glibc 270/0; TFAIL/TBROK 0; TCONF 4 known `read02`; timeout/ENOSYS/panic-trap 0 |
| LA stable270 | `raw/stable270-la-aggregate-20260524T120322+0800.log` | `stable270-la-aggregate-summary.txt`: PASS LTP CASE 540, FAIL 0; ltp-musl 270/0; ltp-glibc 270/0; TFAIL/TBROK 0; TCONF 4 known `read02`; timeout/ENOSYS/panic-trap 0 |

## Caveat

`read02` remains `pass_with_tconf` and is disclosed here. The 20 newly promoted cases are clean in the targeted evidence used for promotion.
