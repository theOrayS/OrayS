# Stable400 attempt 4 scout report

Date: 2026-05-26
Status: **negative scout; no promotion**.

## Context

After stable382 accepted `lseek02` via real `mknodat`/FIFO support, the next low-risk hypothesis was that adjacent lseek cases might be similarly promotable. The live stable list remained 382 total / 382 unique / 0 duplicates before this scout.

A duplicated FD/fcntl scout was accidentally launched by the tool wrapper while the first instance was still running. It was terminated, its partial log removed, and it is explicitly invalidated as evidence. No FD/fcntl result from that duplicate attempt is used for promotion or ranking.

## RV lseek-neighbor scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=lseek03,lseek04,lseek05,lseek06,lseek08,lseek09,lseek10,lseek11 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv
python3 -B scripts/ltp_summary.py \
  docs/ltp-score-improvement-2026-05-25-phase-c/raw/target-stable400-lseek-neighbors-rv-002.log
```

Summary evidence: `raw/target-stable400-lseek-neighbors-rv-002-summary.txt`.

Result:

- PASS LTP CASE: 0
- FAIL LTP CASE: 16
- `ltp-musl`: 0 passed, 8 failed
- `ltp-glibc`: 0 passed, 8 failed
- Internal: TCONF 2
- timeout: 0
- ENOSYS/not implemented: 2
- panic/trap: 0

## Per-case decision

| Case | RV result | Decision |
| --- | --- | --- |
| `lseek03` | musl+glibc wrapper FAIL `-1`; missing testcase binary | Do not promote; not present in sdcard image |
| `lseek04` | musl+glibc wrapper FAIL `-1`; missing testcase binary | Do not promote; not present in sdcard image |
| `lseek05` | musl+glibc wrapper FAIL `-1`; missing testcase binary | Do not promote; not present in sdcard image |
| `lseek06` | musl+glibc wrapper FAIL `-1`; missing testcase binary | Do not promote; not present in sdcard image |
| `lseek08` | musl+glibc wrapper FAIL `-1`; missing testcase binary | Do not promote; not present in sdcard image |
| `lseek09` | musl+glibc wrapper FAIL `-1`; missing testcase binary | Do not promote; not present in sdcard image |
| `lseek10` | musl+glibc wrapper FAIL `-1`; missing testcase binary | Do not promote; not present in sdcard image |
| `lseek11` | musl+glibc wrapper FAIL 32; TCONF on SEEK_DATA/SEEK_HOLE, ENOSYS/not implemented match | Do not promote; requires SEEK_DATA/SEEK_HOLE semantics or explicit non-promotion |

## Next action

Do not spend more promotion time on `lseek03`-`lseek10` unless the sdcard/test inventory is intentionally expanded. Do not promote `lseek11` without real SEEK_DATA/SEEK_HOLE support and fresh RV+LA x musl+glibc clean evidence.

Continue stable400 search from non-lseek families, preferably small metadata/permission or already-present FD/process cases with current test binaries and low timeout risk.
