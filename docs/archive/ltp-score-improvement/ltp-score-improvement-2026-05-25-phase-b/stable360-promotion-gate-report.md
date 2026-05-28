# stable360 Promotion Gate Report

Date: 2026-05-25
Baseline: stable350 (`350 total / 350 unique / 0 duplicates`)
Finalized promotion target: stable360 (`360 total / 360 unique / 0 duplicates`)

## Added cases

| Case | Source | RV evidence | LA evidence | Decision |
| --- | --- | --- | --- | --- |
| `access02` | primary permissions | `raw/target-primary30-rv-002-summary.txt`: RV musl+glibc clean | `raw/target-rvclean5-la-001-summary.txt`: LA musl+glibc clean | promote |
| `fchmodat02` | primary permissions | same | same | promote |
| `inode01` | fallback fs | `raw/target-fs8-rv-001-summary.txt`: RV musl+glibc clean | `raw/target-inode01-la-001-summary.txt`: LA musl+glibc clean | promote |
| `mmap06` | primary mmap | `raw/target-primary30-rv-002-summary.txt`: RV musl+glibc clean | `raw/target-rvclean5-la-001-summary.txt`: LA musl+glibc clean | promote |
| `ftest01` | fallback fs | `raw/target-fallback18-rv-001-summary.txt`: RV musl+glibc clean | `raw/target-fallback6-la-001-summary.txt`: LA musl+glibc clean | promote |
| `ftest02` | fallback fs | same | same | promote |
| `ftest03` | fallback fs | same | same | promote |
| `ftest04` | fallback fs | same | same | promote |
| `mmap10` | fallback mmap | same | same | promote |
| `stream01` | fallback fs/stdio | same | same | promote |

Notes:
- `kill02` initially had targeted evidence but was demoted after `raw/stable375-la-final-001-summary.txt` showed LA full aggregate failure/TBROK. It is not in the final stable375 list.
- `pipe2_02` was clean in primary evidence but already belonged to stable350, so it was not counted as a new promotion.
- `readlinkat02` was RV clean but LA musl failed, so it was not promoted.
- Primary blockers such as `statx01`, `rename*`, `openat02`, `mmap04/05`, `munmap01`, `mprotect*`, `chmod*`, `fchmod*`, and `waitid07/08/10` remain documented in the candidate matrix; no wrapper pass was treated as promotion without internal-clean evidence.

## Gate evidence

| Gate | Evidence | Result |
| --- | --- | --- |
| RV stable360 aggregate | `raw/stable360-rv-001.log`, `raw/stable360-rv-001-summary.txt` | Earlier tranche including `kill02`: PASS LTP CASE 720, FAIL 0; ltp-musl 360/0; ltp-glibc 360/0; internal TCONF 4 (`read02` only); timeout/ENOSYS/panic/trap 0 |
| Final RV aggregate authority | `raw/stable375-rv-final-002-summary.txt` | Final list including `inode01` instead of `kill02`: PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0 |
| Final LA aggregate authority | `raw/stable375-la-final-003-summary.txt` | Final list including `inode01` instead of `kill02`: PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0 |
| Marker prefix | `raw/stable375-final-marker-prefix.txt` | RV markers 750 bad 0; LA markers 750 bad 0 |

## Decision

stable360 tranche accepted only in its finalized form with `inode01` replacing `kill02`. The earlier RV aggregate remains useful regression evidence, but final delivery authority is the RV+LA stable375 aggregate gate over the final 375-case list.
