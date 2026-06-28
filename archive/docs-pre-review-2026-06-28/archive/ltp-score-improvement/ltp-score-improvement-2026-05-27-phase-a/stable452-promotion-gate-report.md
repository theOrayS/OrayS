# Stable452 promotion gate report

Date: 2026-05-27/2026-05-28 overnight
Team: `ltp-stable413-to-stab-d9f99e59` (runtime state no longer available; leader-owned serialized gates continued)
Ultragoal story: `G005-story-stable452-promote-about-12-add`

## Result

**stable452 reached as a promotion checkpoint.**

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` after the leader-owned edit:

```text
total=452 unique=452 duplicates=0
```

Net promotion from stable440: **+12** real cases. Net promotion from stable413: **+39** real cases.

Promoted cases:

```text
flock06
llseek02
llseek03
setresgid04
setresuid04
setresuid05
setreuid05
setreuid06
setreuid07
fchown01
fchown02
fchown03
```

Clean definition used here: RV+LA x musl+glibc wrapper PASS, zero internal `TFAIL/TBROK/TCONF` for each newly promoted case, zero timeout, zero ENOSYS/not-implemented, zero panic/trap, and zero marker-prefix bad lines. The only internal `TCONF` still present in the aggregate stable452 gates is the already-disclosed `read02` O_DIRECT/tmpfs TCONF: 4 per arch total, 2 per libc group.

## Targeted candidate evidence

| Batch | Scope | Parser result | Promotion use |
| --- | --- | --- | --- |
| `raw/stable452-scout-001-rv-summary.txt` | RV VFS/path scout (`flock06`, sendfile09*, open/link/unlink/fchmod/statx/ftruncate/fs_perms candidates) | RV-clean subset only: `flock06`; many path/link/statx/fs_perms rows demoted by wrapper failure, internal `TFAIL/TBROK/TCONF`, ENOSYS, missing binaries, or setup failures | Confirms `flock06` on RV only; required LA confirm below |
| `raw/stable452-scout-002-rv-summary.txt` | RV select/pipe/fcntl scout | No promotion-clean rows; `select02/03/04` are pass-with-TCONF and demoted, pipe/fcntl rows failed or had setup blockers | No promoted rows |
| `raw/stable452-scout-003-rv-summary.txt` | RV time/getrusage/llseek/fsync/utime/prctl scout | RV-clean subset: `llseek02`, `llseek03`; `getrusage02` pass-with-TCONF and demoted; several rows timed out or failed | Confirms `llseek02`, `llseek03` on RV only |
| `raw/stable452-scout-004-rv-summary.txt` | RV uid/gid credential scout | RV-clean subset: `setresuid04`, `setresuid05`, `setreuid05`, `setreuid06`, `setreuid07`; 16-bit variants demoted as TCONF | Confirms five credential rows on RV only |
| `raw/stable452-scout-005-rv-summary.txt` | RV setresgid/chown/fchown/cap/getgroups scout | RV-clean subset: `setresgid04`, `fchown01`, `fchown02`, `fchown03`, `fchown05`, `fchownat01`; `setgroups03`, `setfsgid03`, `chown04`, `fchown04`, `fchownat02`, `lchown*`, `capset*`, `getgroups*_16` demoted | Promotes first four rows from this subset; saves `fchown05`, `fchownat01` as stable460 candidates |
| `raw/stable452-clean14-la-confirm-summary.txt` | LA confirm for the 14 RV-clean rows | `PASS LTP CASE 28`, `FAIL LTP CASE 0`; internal `TFAIL/TBROK/TCONF` 0; timeout/ENOSYS/panic/trap 0 | Confirms all 14 rows on LA; leader promoted 12 now and reserved `fchown05`, `fchownat01` for stable460 |

Rows deliberately left outside stable after this story include `fchown05` and `fchownat01` only as saved clean reserve for the next story, plus demoted/blocker rows such as `sendfile09*` (large-space TCONF), `select02/03/04` (pass-with-TCONF), `getrusage02` (pass-with-TCONF), `setgroups03`, `setfsgid03`, `fchownat02`, `lchown*`, `capset*`, 16-bit credential variants, and the path/statx/fs_perms rows from scout 001.

## Aggregate promotion gates

### RV aggregate gate

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-rv-promotion-gate.log
```

Evidence:

- Raw log: `raw/stable452-rv-promotion-gate.log` (kept local; not intended for commit)
- Summary: `raw/stable452-rv-promotion-gate-summary.txt`
- Status: `raw/stable452-rv-promotion-gate-status.txt` -> `0`
- Marker check: `raw/stable452-rv-promotion-gate-marker-prefix.txt` -> `bad_marker_prefix_lines=0`
- Noise check: `raw/stable452-rv-promotion-gate-noise.txt`

Parser result:

```text
PASS LTP CASE: 904
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 452 passed, 0 failed, Internal {'TCONF': 2}
ltp-glibc: 452 passed, 0 failed, Internal {'TCONF': 2}
```

### LA aggregate gate

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-la-promotion-gate.log
```

Evidence:

- Raw log: `raw/stable452-la-promotion-gate.log` (kept local; not intended for commit)
- Summary: `raw/stable452-la-promotion-gate-summary.txt`
- Status: `raw/stable452-la-promotion-gate-status.txt` -> `0`
- Marker check: `raw/stable452-la-promotion-gate-marker-prefix.txt` -> `bad_marker_prefix_lines=0`
- Noise check: `raw/stable452-la-promotion-gate-noise.txt`

Parser result:

```text
PASS LTP CASE: 904
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 452 passed, 0 failed, Internal {'TCONF': 2}
ltp-glibc: 452 passed, 0 failed, Internal {'TCONF': 2}
```

## Marker/noise guardrail

Marker prefix checks:

```text
RV stable452: bad_marker_prefix_lines=0
LA stable452: bad_marker_prefix_lines=0
```

Noise counts in the aggregate logs:

| Arch log | `AxError::NotADirectory` | `AxError::IsADirectory` | `AxError::AlreadyExists` | Impact |
| --- | ---: | ---: | ---: | --- |
| RV stable452 | 10 | 0 | 0 | disclosed; no marker pollution and no LTP failure |
| LA stable452 | 10 | 0 | 0 | disclosed; no marker pollution and no LTP failure |

This remains below the stable413 disclosed ramfs NotADirectory noise (22 per arch) and does not affect LTP wrapper parsing.

## Source and behavior notes

Source change in this story:

- `examples/shell/src/cmd.rs`: adds 12 proven stable452 cases to `LTP_STABLE_CASES`.

No LTP test source was modified. No fixed result was returned based on test name/path/process name. This story makes no new syscall, errno, ABI, or POSIX-visible behavior change beyond increasing the stable evaluator case list. Previously delivered stable440 behavior changes (`flock(2)` support and `pselect6` invalid-fd validation) remain part of the live tree and are protected by the stable452 aggregate gates.

## Local verification commands run for this stage

```bash
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-scout-001-rv.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-scout-002-rv.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-scout-003-rv.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-scout-004-rv.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-scout-005-rv.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-clean14-la-confirm.log
python3 - <<'PY'
# live LTP_STABLE_CASES total/unique/duplicates check
PY
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-rv-promotion-gate.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable452-la-promotion-gate.log
```

Post-LA disk check remained healthy: `/` 37% used and `/root/.codex` 1.4G.

## Next stage

Proceed to `G006` stable460 with only fresh evidence. Immediate reserve candidates are `fchown05` and `fchownat01` from `raw/stable452-clean14-la-confirm-summary.txt`; at least six more clean rows are still required before editing stable to 460.
