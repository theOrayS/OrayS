# stable368 Promotion Gate Report

Date: 2026-05-25
Promotion target: stable368 (`368 total / 368 unique / 0 duplicates`)

## Added cases

| Case | Subsystem | RV targeted evidence | LA targeted evidence | Decision |
| --- | --- | --- | --- | --- |
| `ftest05` | fs/file stress | `raw/target-adjacent20-rv-001-summary.txt`: clean musl+glibc | `raw/target-adjacent8-la-001-summary.txt`: clean musl+glibc | promote |
| `ftest07` | fs/file stress | same | same | promote |
| `ftest08` | fs/file stress | same | same | promote |
| `mmap09` | mmap/ftruncate | same | same | promote |
| `mmap11` | mmap basic | same | same | promote |
| `stream03` | fs/stdio stream | same | same | promote |
| `stream04` | fs/stdio stream | same | same | promote |
| `stream05` | fs/stdio stream | same | same | promote |

Known non-promotions from the same RV scout include `ftest06` wrapper code 4/TWARN cleanup, `mmap08` TFAIL, `statfs03*` TFAIL, `statvfs01` TBROK, and `open06/open07/open10/open11/open12/read03/write04` failures. These remain blocker notes, not hidden passes.

## Gate evidence

| Gate | Evidence | Result |
| --- | --- | --- |
| RV targeted tranche | `raw/target-adjacent20-rv-001-summary.txt` | 8 promoted cases clean on RV musl+glibc; non-promotions retain explicit failures |
| LA targeted tranche | `raw/target-adjacent8-la-001-summary.txt` | 8 promoted cases clean on LA musl+glibc |
| RV aggregate coverage | `raw/stable375-rv-final-002-summary.txt` | stable375 final includes this tranche: PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0 |
| LA aggregate coverage | `raw/stable375-la-final-003-summary.txt` | stable375 final includes this tranche: PASS LTP CASE 750, FAIL 0; ltp-musl 375/0; ltp-glibc 375/0 |
| Marker prefix | `raw/stable375-final-marker-prefix.txt` | RV markers 750 bad 0; LA markers 750 bad 0 |

## Decision

stable368 tranche accepted as part of the staged stable375 promotion. It was not used to hide failed adjacent cases; only the eight RV+LA × musl+glibc clean cases were added.
