# Stable425 promotion gate report

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Ultragoal story: `G003-story-stable425-promote-about-12-rv`

## Result

**stable425 reached as a promotion checkpoint.**

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` after the leader-owned edit:

```text
total=425 unique=425 duplicates=0
```

Net promotion from stable413: **+12** real cases.

Promoted cases:

```text
fcntl07
fcntl07_64
open06
creat04
mkdir04
rmdir03
unlink08
unlink07
statfs03
statfs03_64
pselect03
setrlimit05
```

Clean definition used here: RV+LA x musl+glibc wrapper PASS, zero internal `TFAIL/TBROK/TCONF` for the promoted cases, zero timeout, zero ENOSYS/not-implemented, zero panic/trap, and zero marker-prefix bad lines. The only internal `TCONF` still present in aggregate stable425 is the already-disclosed `read02` O_DIRECT/tmpfs TCONF: 4 per arch total, 2 per libc group.

## Targeted candidate evidence

| Batch | Scope | Parser result | Promotion use |
| --- | --- | --- | --- |
| `raw/postrepair-scout-001-rv-summary.txt` | RV VFS/metadata repair scout | `PASS LTP CASE 22`, `FAIL 16`; timeout 0, panic/trap 0 | RV-clean subset selected only |
| `raw/postrepair-scout-001-la-confirm-summary.txt` | LA confirmation for RV-clean subset | `PASS LTP CASE 20`, `FAIL 0`; timeout/ENOSYS/panic/trap 0 | Confirms `open06,creat04,mkdir04,rmdir03,unlink08,unlink07,statfs03,statfs03_64` |
| `raw/scout-misc-process-001-rv-summary.txt` | RV misc/process scout | `PASS LTP CASE 11`, `FAIL 19`; timeout 6 on blocked rows | Only `pselect03,setrlimit05` selected |
| `raw/scout-misc-process-001-la-confirm-summary.txt` | LA confirmation for `pselect03,setrlimit05` | `PASS LTP CASE 4`, `FAIL 0`; timeout/ENOSYS/panic/trap 0 | Confirms both misc/process rows |

Demoted rows from the same targeted evidence remain outside stable: `getdents01`, `getdents02`, `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statvfs01`, `getcwd03`, `getcwd04`, `select01`/`select02`/`select03`/`select04`, `pselect01`/`pselect02`, `clock_gettime01`/`clock_gettime04`, `setrlimit04`, `sched_rr_get_interval03`, `sched_setaffinity01`, `setpriority01`, `nice04`, and `signal01` because at least one RV/LA/libc row had wrapper failure, internal blocker, timeout, ENOSYS, or split-risk evidence.

## Aggregate promotion gates

### RV current-source aggregate gate

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable425-rv-current-source-gate.log
```

Evidence:

- Raw log: `raw/stable425-rv-current-source-gate.log` (kept local; not intended for commit)
- Summary: `raw/stable425-rv-current-source-gate-summary.txt`
- Status: `raw/stable425-rv-current-source-gate-status.txt` -> `run_status=0`
- Marker check: `raw/stable425-rv-current-source-gate-marker-prefix.txt` -> `bad_marker_prefix_lines=0`

Parser result:

```text
PASS LTP CASE: 850
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 425 passed, 0 failed, Internal {'TCONF': 2}
ltp-glibc: 425 passed, 0 failed, Internal {'TCONF': 2}
```

### LA aggregate gate

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable425-la-promotion-gate.log
```

Evidence:

- Raw log: `raw/stable425-la-promotion-gate.log` (kept local; not intended for commit)
- Summary: `raw/stable425-la-promotion-gate-summary.txt`
- Status: `raw/stable425-la-promotion-gate-status.txt` -> `run_status=0`
- Marker check: `raw/stable425-la-promotion-gate-marker-prefix.txt` -> `bad_marker_prefix_lines=0`

Parser result:

```text
PASS LTP CASE: 850
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 425 passed, 0 failed, Internal {'TCONF': 2}
ltp-glibc: 425 passed, 0 failed, Internal {'TCONF': 2}
```

### Superseded RV precheck

A first RV stable425 gate also passed at `raw/stable425-rv-promotion-gate-summary.txt`, but it was treated as a precheck only because the final pipe SIGPIPE source repair was merged after that kernel build. The promotion evidence above uses the later `stable425-rv-current-source-gate` built from final current source.

## Marker/noise guardrail

Marker prefix checks:

```text
RV current-source: bad_marker_prefix_lines=0
LA: bad_marker_prefix_lines=0
```

Noise counts in the aggregate logs:

| Arch log | `AxError::NotADirectory` | `AxError::IsADirectory` | `AxError::AlreadyExists` | Impact |
| --- | ---: | ---: | ---: | --- |
| RV current-source stable425 | 10 | 0 | 0 | disclosed; no marker pollution and no LTP failure |
| LA stable425 | 10 | 0 | 0 | disclosed; no marker pollution and no LTP failure |

This is lower than the stable413 disclosed ramfs NotADirectory noise (22 per arch) and does not affect LTP wrapper parsing.

## Source and behavior notes

Source changes behind this promotion are real subsystem changes, not case-name hardcoding:

- `examples/shell/src/cmd.rs`: adds the 12 proven stable cases to `LTP_STABLE_CASES`.
- VFS/path permission repairs landed from Team workers in `examples/shell/src/uspace/fd_table.rs` and `linux_abi.rs`; visible effect is more Linux-compatible parent write/search and long-path errno behavior for negative-path file operations.
- Pipe SIGPIPE teardown ordering was repaired in `examples/shell/src/uspace/fd_pipe.rs`; visible POSIX intent remains default SIGPIPE termination before user return, while ignored/blocked/handled SIGPIPE still reports `EPIPE`/pending/handler behavior through the common signal path.

No LTP test source was modified. No fixed result was returned based on test name/path/process name.

## Local verification commands run for this stage

```bash
python3 -B scripts/ltp_summary.py raw/postrepair-scout-001-rv.log
python3 -B scripts/ltp_summary.py raw/postrepair-scout-001-la-confirm.log
python3 -B scripts/ltp_summary.py raw/scout-misc-process-001-rv.log
python3 -B scripts/ltp_summary.py raw/scout-misc-process-001-la-confirm.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py raw/stable425-rv-current-source-gate.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py raw/stable425-la-promotion-gate.log
git diff --check
```

`git diff --check` passed after the stable425 edits/report updates.

## Next stage

Proceed to `G004` stable440 only with fresh candidate evidence. Highest immediate candidates should come from low-risk repair/scout lanes and must still pass RV+LA x musl+glibc clean before promotion.
