# Remote marker and log noise regression check

Date: 2026-05-26
Scope: final stable413 RV/LA logs after code-review fixes.

## Logs checked

- RV: `raw/stable413-rv-final-gate-002.log`
- LA: `raw/stable413-la-final-gate-002.log`

## Marker-prefix result

| Log | `PASS LTP CASE` at column 0 | `FAIL LTP CASE` at column 0 | Bad marker-prefix lines |
| --- | ---: | ---: | ---: |
| RV stable413 | 0 | 826 | 0 |
| LA stable413 | 0 | 826 | 0 |

The current wrapper emits historical normalized success markers as `FAIL LTP CASE <case> : 0`; `scripts/ltp_summary.py` confirms these are PASS rows. The regression guard is the prefix shape: all marker lines still begin at column 0, with no ANSI/reset/color prefix pollution.

## NotADirectory noise result

| Log | `axfs::fops:297 [AxError::NotADirectory]` | `axfs_ramfs::file:69 ... NotADirectory` |
| --- | ---: | ---: |
| RV stable413 | 0 | 22 |
| LA stable413 | 0 | 22 |

The remote-scoring-sensitive `axfs::fops:297` noise remains 0. Residual `axfs_ramfs::file:69` negative-path noise is disclosed and did not affect marker parsing or promotion.
