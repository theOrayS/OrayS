# Milestone 03 stable656 blacklist change report

No blacklist changes were made in this checkpoint.

## Severe blockers observed

| Case | Blocker type | Blacklist decision |
| --- | --- | --- |
| `futex_wait03` | timeout in both libcs | not blacklisted here; recorded as repair candidate |
| `mmap05` | `TBROK` / killed by SIGSEGV | not blacklisted here; recorded as repair candidate |
| `munmap01` | wrapper FAIL code 139 | not blacklisted here; recorded as repair candidate |
| `mmap13` | `TFAIL` / SIGBUS signal not received | not blacklisted here; recorded as repair candidate |
| `mmap10_1` | missing testcase inventory | excluded from promotion; no blacklist change |
| `vma02` | `TCONF` libnuma requirement | excluded from promotion; no blacklist change |

## Boundary

These failures are not hidden. They are not counted as PASS, not promoted to stable, and not converted into blacklist credit.
