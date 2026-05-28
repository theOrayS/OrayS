# Stable440 promotion gate report

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59` (runtime state already shut down; leader-owned gates continue)
Ultragoal story: `G004-story-stable440-promote-about-15-add`

## Result

**stable440 reached as a promotion checkpoint.**

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` after the leader-owned edit:

```text
total=440 unique=440 duplicates=0
```

Net promotion from stable425: **+15** real cases. Net promotion from stable413: **+27** real cases.

Promoted cases:

```text
unlink05
pipe02
dup05
sendfile07
sendfile07_64
stream02
flock01
flock02
flock03
flock04
clone06
clone07
pselect03_64
pselect02
pselect02_64
```

Clean definition used here: RV+LA x musl+glibc wrapper PASS, zero internal `TFAIL/TBROK/TCONF` for each newly promoted case, zero timeout, zero ENOSYS/not-implemented, zero panic/trap, and zero marker-prefix bad lines. The only internal `TCONF` still present in the aggregate stable440 gates is the already-disclosed `read02` O_DIRECT/tmpfs TCONF: 4 per arch total, 2 per libc group.

## Targeted candidate evidence

| Batch | Scope | Parser result | Promotion use |
| --- | --- | --- | --- |
| `raw/stable440-vfs-scout-001-rv-inline-summary.txt` | RV VFS/path scout | `PASS LTP CASE 4`, `FAIL LTP CASE 54`; internal `TFAIL/TBROK`, ENOSYS present on demoted rows | RV-clean subset only; `unlink05` selected, `readlinkat02` later demoted on LA |
| `raw/stable440-vfs-clean2-la-confirm-summary.txt` | LA confirm for VFS clean subset | `unlink05` clean; `readlinkat02` not promotion-clean because LA musl hit wrapper/zero-size setup mismatch | Confirms `unlink05` only |
| `raw/stable440-pipe02-rv-postfix-summary.txt` | RV pipe narrow repair check | `PASS LTP CASE 2`, `FAIL LTP CASE 0`; timeout/ENOSYS/panic 0 | Confirms `pipe02` after inherited pipe/SIGPIPE repair path |
| `raw/stable440-fd-select-sendfile-rv-scout-summary.txt` | RV FD/select/sendfile scout | RV-clean rows: `dup05`, `sendfile07`, `sendfile07_64`; `select04` is pass-with-TCONF and demoted | Confirms FD/sendfile subset on RV |
| `raw/stable440-fs-substitute-rv-scout-summary.txt` | RV fs substitute scout | `stream02` clean; other fs-suite rows demoted | Confirms `stream02` on RV |
| `raw/stable440-clean6-la-confirm-summary.txt` | LA confirm for `unlink05,pipe02,dup05,sendfile07,sendfile07_64,stream02` | `PASS LTP CASE 12`, `FAIL LTP CASE 0`; internal 0; timeout/ENOSYS/panic 0 | Confirms first six new rows |
| `raw/stable440-flock-rv-postfix-summary.txt` | RV `flock(2)` after narrow implementation | `PASS LTP CASE 8`, `FAIL LTP CASE 0`; internal 0; timeout/ENOSYS/panic 0 | Confirms `flock01`-`flock04` on RV |
| `raw/stable440-flock-la-postfix-summary.txt` | LA `flock(2)` after narrow implementation | `PASS LTP CASE 8`, `FAIL LTP CASE 0`; internal 0; timeout/ENOSYS/panic 0 | Confirms `flock01`-`flock04` on LA |
| `raw/stable440-mixed-scout-002-rv-summary.txt` | RV process/select/misc scout | RV-clean rows: `clone06`, `clone07`, `pselect03_64`; demoted rows include statfs/statvfs/open/readlink blockers | Confirms process/select subset on RV |
| `raw/stable440-pselect02-rv-postfix-summary.txt` | RV `pselect02*` after invalid-fd validation | `PASS LTP CASE 4`, `FAIL LTP CASE 0`; internal 0; timeout/ENOSYS/panic 0 | Confirms `pselect02`, `pselect02_64` on RV |
| `raw/stable440-clean5-la-confirm-summary.txt` | LA confirm for `clone06,clone07,pselect03_64,pselect02,pselect02_64` | `PASS LTP CASE 10`, `FAIL LTP CASE 0`; internal 0; timeout/ENOSYS/panic 0 | Confirms final five new rows |

Demoted rows remain outside stable because at least one arch/libc path still has wrapper failure, internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap, or split-risk evidence. Examples from this story: `readlinkat02` (LA musl setup/wrapper mismatch), `select04` (pass-with-TCONF), `pipe07`, `select01`-`select03`, `pselect01`, `pselect01_64`, `getpgid01`, `readlink03`, `open15`, `statfs01`, `statfs01_64`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, `clone08`, and `clone09`.

The earlier `raw/stable440-vfs-scout-001-rv-summary.txt` used a non-effective `LTP_CASES=file:...` invocation and produced a 0/0 scout; it is explicitly superseded by `raw/stable440-vfs-scout-001-rv-inline-summary.txt` and is not promotion evidence.

## Aggregate promotion gates

### RV aggregate gate

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-rv-promotion-gate.log
```

Evidence:

- Raw log: `raw/stable440-rv-promotion-gate.log` (kept local; not intended for commit)
- Summary: `raw/stable440-rv-promotion-gate-summary.txt`
- Status: `raw/stable440-rv-promotion-gate-status.txt` -> `run_status=0`
- Marker check: `raw/stable440-rv-promotion-gate-marker-prefix.txt` -> `bad_marker_prefix_lines=0`
- Noise check: `raw/stable440-rv-promotion-gate-noise.txt`

Parser result:

```text
PASS LTP CASE: 880
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 440 passed, 0 failed, Internal {'TCONF': 2}
ltp-glibc: 440 passed, 0 failed, Internal {'TCONF': 2}
```

### LA aggregate gate

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-la-promotion-gate.log
```

Evidence:

- Raw log: `raw/stable440-la-promotion-gate.log` (kept local; not intended for commit)
- Summary: `raw/stable440-la-promotion-gate-summary.txt`
- Status: `raw/stable440-la-promotion-gate-status.txt` -> `run_status=0`
- Marker check: `raw/stable440-la-promotion-gate-marker-prefix.txt` -> `bad_marker_prefix_lines=0`
- Noise check: `raw/stable440-la-promotion-gate-noise.txt`

Parser result:

```text
PASS LTP CASE: 880
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 440 passed, 0 failed, Internal {'TCONF': 2}
ltp-glibc: 440 passed, 0 failed, Internal {'TCONF': 2}
```

## Marker/noise guardrail

Marker prefix checks:

```text
RV stable440: bad_marker_prefix_lines=0
LA stable440: bad_marker_prefix_lines=0
```

Noise counts in the aggregate logs:

| Arch log | `AxError::NotADirectory` | `AxError::IsADirectory` | `AxError::AlreadyExists` | Impact |
| --- | ---: | ---: | ---: | --- |
| RV stable440 | 10 | 0 | 0 | disclosed; no marker pollution and no LTP failure |
| LA stable440 | 10 | 0 | 0 | disclosed; no marker pollution and no LTP failure |

This remains below the stable413 disclosed ramfs NotADirectory noise (22 per arch) and does not affect LTP wrapper parsing.

## Source and behavior notes

Source changes behind this promotion are real subsystem changes, not case-name hardcoding:

- `examples/shell/src/cmd.rs`: adds the 15 proven stable440 cases to `LTP_STABLE_CASES`.
- `examples/shell/src/uspace/fd_table.rs`: implements a narrow but real `flock(2)` table for path-backed files, including shared/exclusive/unlock/nonblocking conflict behavior and close-time cleanup.
- `examples/shell/src/uspace/syscall_dispatch.rs`: wires `general::__NR_flock` to the FD table implementation.
- `examples/shell/src/uspace/select_fdset.rs`: validates selected fd-set entries before `pselect6` waiting so invalid selected descriptors return `EBADF` instead of timing out.

Visible POSIX/Linux behavior changes:

- `flock(2)` is no longer ENOSYS for supported operations on path-backed files. It supports `LOCK_SH`, `LOCK_EX`, `LOCK_UN`, and `LOCK_NB`; invalid operations return `EINVAL`; nonblocking conflicts return `EAGAIN`/`EWOULDBLOCK`; locks are released on explicit unlock and final close of the open file description.
- Known limitation: blocking `flock(2)` conflicts currently return `EAGAIN` rather than sleeping until unlock. This is sufficient for the promoted `flock01`-`flock04` subset but remains a hidden-test risk.
- `pselect6` now reports `EBADF` for invalid descriptors that are actually selected in read/write/except fdsets, matching the LTP `pselect02*` expectation and avoiding timeout-as-pass.

No LTP test source was modified. No fixed result was returned based on test name/path/process name.

## Local verification commands run for this stage

```bash
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-vfs-scout-001-rv-inline.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-vfs-clean2-la-confirm.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-pipe02-rv-postfix.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-fd-select-sendfile-rv-scout.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-fs-substitute-rv-scout.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-clean6-la-confirm.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-flock-rv-postfix.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-flock-la-postfix.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-mixed-scout-002-rv.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-pselect02-rv-postfix.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-clean5-la-confirm.log
cargo fmt -p arceos-shell -- --check
make A=examples/shell ARCH=riscv64
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-rv-promotion-gate.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable440-la-promotion-gate.log
python3 - <<'PY'
# live LTP_STABLE_CASES total/unique/duplicates check
PY
git diff --check
```

`cargo fmt -p arceos-shell -- --check`, the live stable count, and `git diff --check` passed after the stable440 edits/report updates. Disk post-check after the LA gate remained healthy: `/` 37% used and `/root/.codex` 1.4G.

## Next stage

Proceed to `G005` stable452 only with fresh candidate evidence. Highest immediate candidates should come from the newly discovered clean-adjacent rows that already appeared inside the aggregate stable440 run but still require explicit targeted RV+LA x musl/glibc promotion proof before any stable list edit.
