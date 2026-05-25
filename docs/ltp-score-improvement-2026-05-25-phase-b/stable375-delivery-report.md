# stable375 Delivery Report

Date: 2026-05-25
Target: stable350 -> stable375
Stretch: stable380 deferred

## Result

Delivered stable375 with `examples/shell/src/cmd.rs::LTP_STABLE_CASES` at `375 total / 375 unique / 0 duplicates`.

Final added cases:

```text
access02, fchmodat02, inode01, mmap06,
ftest01, ftest02, ftest03, ftest04, mmap10, stream01,
ftest05, ftest07, ftest08, mmap09, mmap11, stream03, stream04, stream05,
abort01, poll01, fork05, fork10, kill11, kill12, mem02
```

`kill02` was not delivered: it had targeted clean evidence, but LA full stable aggregate later produced TBROK/setup failure. It was removed and replaced with `inode01` before final gates.

## Final gates

| Arch | Command | Summary | Result |
| --- | --- | --- | --- |
| RV | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv` | `raw/stable375-rv-final-002-summary.txt` | PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0 |
| LA | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la` | `raw/stable375-la-final-003-summary.txt` | PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0 |

Quality signals in both final gates:

- Internal TFAIL: 0
- Internal TBROK: 0
- Internal TCONF: 4, matching the known `read02` baseline TCONF only
- Timeout: 0
- ENOSYS/not implemented: 0
- Panic/trap: 0
- Marker prefix: 0 bad marker lines in both RV and LA final logs

## Source changes

- `examples/shell/src/cmd.rs`: extends `LTP_STABLE_CASES` from 350 to 375 unique cases.
- `examples/shell/src/uspace/metadata.rs`: improves `fchmod`/`fchmodat` errno/permission handling and validates `statx` mask bits.
- `examples/shell/src/uspace/syscall_dispatch.rs`: passes the real `statx` mask argument into `sys_statx`.
- `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/memory_map.rs`: makes mmap file reads reject unreadable descriptors with `EACCES`, rejects missing/invalid mapping type, and returns `EINVAL` for unaligned `mprotect` addresses.
- `scripts/ltp_summary.py`: documents the current wrapper marker wire format without changing parser semantics.

## Not delivered / follow-up blockers

- Stretch stable380 deferred: no additional five cases met the final promotion bar within this round.
- `kill02`: LA full aggregate TBROK/setup failure; do not re-add without full aggregate proof.
- `readlinkat02`: RV clean, LA musl TFAIL.
- Primary VFS/metadata cases (`statx01`, `rename*`, `openat02`, `chmod*`, `fchmod*`) still need semantic work.
- `pipe02`: discovery panic; requires root-cause before any promotion attempt.
- VM stretch (`mprotect*`, `munmap01`, `mmap13/14`) remains blocked by page-protection/unmap semantics.
