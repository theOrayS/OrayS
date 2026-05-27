# stable460 delivery report

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59` (stale hook prompts remained, but no usable live team state was available during final leader serialization)
Ultragoal story: `G006-story-stable460-final-promote-remain`

## Result

**stable460 main target reached.**

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` after the final leader-owned edit:

```text
total=460 unique=460 duplicates=0
```

Net promotion from stable452: **+8** real cases. Net promotion from stable413: **+47** real cases.

Final stable460 additions:

```text
fchown05
fchownat01
fcntl18
fcntl18_64
syscall01
mknod06
mknod02
mknod05
```

Clean definition used for promotion: RV+LA x musl+glibc wrapper PASS, zero internal `TFAIL/TBROK/TCONF` for the newly promoted rows, zero timeout, zero ENOSYS/not-implemented, zero panic/trap, and zero marker-prefix bad lines. The only internal `TCONF` still present in the aggregate stable460 gates is the previously disclosed `read02` O_DIRECT/tmpfs TCONF: 4 per arch total, 2 per libc group.

## Targeted candidate evidence

| Batch | Scope | Parser result | Promotion use |
| --- | --- | --- | --- |
| `raw/stable452-clean14-la-confirm-summary.txt` | LA confirm from the previous story for `fchown05`, `fchownat01` reserves | both rows RV+LA clean through the stable452 reserve path | promoted now |
| `raw/stable460-scout-001-rv-summary.txt` | RV ownership reserve check (`fchown05`, `fchownat01`) | RV clean, zero internal failures | used with prior LA confirm |
| `raw/stable460-scout-002-rv-summary.txt` | RV fcntl adjacency (`fcntl18`, `fcntl18_64`) | both RV clean, zero internal failures | promoted after LA confirm |
| `raw/stable460-scout-003-rv-summary.txt` | RV light syscall (`syscall01`) | RV clean, zero internal failures | promoted after LA confirm |
| `raw/stable460-scout-004-rv-summary.txt` | RV process/signal check (`kill02`) | targeted RV clean | **not promoted**; aggregate LA gate exposed a TBROK setup-timeout blocker |
| `raw/stable460-scout-005-rv-summary.txt` | RV mknod/path substitute scout | `mknod02`, `mknod05`, `mknod06`, `mknod08`, `mknodat01`, `readlinkat02`, `rename14` RV clean | promoted `mknod06`, `mknod02`, `mknod05`; retained reserves below |
| `raw/stable460-clean13-la-confirm-001-summary.txt` | LA confirm for 13 RV-clean candidates | `readlinkat02` failed LA musl with `TFAIL=1`; the other rows needed by this story were clean | confirms final mknod/fcntl/syscall subset; demotes `readlinkat02` |

Rows deliberately left outside stable after this story:

- `kill02`: targeted RV/LA scout looked clean, but the first LA stable460 aggregate gate failed both musl/glibc with `TBROK=4` per libc from child setup timeout. Targeted-only evidence is insufficient.
- `readlinkat02`: LA musl failed with `TFAIL=1` in `raw/stable460-clean13-la-confirm-001-summary.txt`.
- `mknod08`, `mknodat01`, and `rename14`: clean reserves for a future stable470 attempt; not needed for exactly stable460.

## Aggregate final gates

### RV final gate 002

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable460-rv-final-gate-002.log
```

Evidence:

- Raw log: `raw/stable460-rv-final-gate-002.log` (kept local; not intended for commit)
- Summary: `raw/stable460-rv-final-gate-002-summary.txt`
- Status: `raw/stable460-rv-final-gate-002-status.txt` -> `0`
- Marker check: `raw/stable460-rv-final-gate-002-marker-prefix.txt` -> `bad_marker_prefix_lines=0`
- Noise check: `raw/stable460-rv-final-gate-002-noise.txt`

Parser result:

```text
PASS LTP CASE: 920
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 460 passed, 0 failed
ltp-glibc: 460 passed, 0 failed
```

### LA final gate 002

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable460-la-final-gate-002.log
```

Evidence:

- Raw log: `raw/stable460-la-final-gate-002.log` (kept local; not intended for commit)
- Summary: `raw/stable460-la-final-gate-002-summary.txt`
- Status: `raw/stable460-la-final-gate-002-status.txt` -> `0`
- Marker check: `raw/stable460-la-final-gate-002-marker-prefix.txt` -> `bad_marker_prefix_lines=0`
- Noise check: `raw/stable460-la-final-gate-002-noise.txt`

Parser result:

```text
PASS LTP CASE: 920
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 460 passed, 0 failed
ltp-glibc: 460 passed, 0 failed
```

## Failed/demoted aggregate attempt

The first LA stable460 aggregate gate is preserved as negative evidence and is not used as delivery proof:

```text
raw/stable460-la-final-gate-summary.txt:
PASS LTP CASE: 918
FAIL LTP CASE: 2
Internal TFAIL/TBROK/TCONF: 12 ({'TCONF': 4, 'TBROK': 8})
ltp-musl: 459 passed, 1 failed
ltp-glibc: 459 passed, 1 failed
```

The failing rows were both `kill02`:

```text
kill02 | la | musl  | FAIL | TBROK=4
kill02 | la | glibc | FAIL | TBROK=4
kill02.c:289 / kill02.c:496: child setup failed by timing out
```

This caused `kill02` to be removed from the stable460 list and replaced by `mknod06`, followed by fresh RV/LA final gates.

## Marker/noise guardrail

Marker prefix checks:

```text
RV stable460 final 002: bad_marker_prefix_lines=0
LA stable460 final 002: bad_marker_prefix_lines=0
```

Noise counts in final 002 aggregate logs:

| Arch log | `AxError::NotADirectory` | `AxError::IsADirectory` | `AxError::AlreadyExists` | `axfs::fops:297 [AxError::NotADirectory]` | `axfs_ramfs::file:69` | Impact |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| RV stable460 final 002 | 12 | 0 | 0 | 0 | 12 | disclosed; no marker pollution and no LTP failure |
| LA stable460 final 002 | 12 | 0 | 0 | 0 | 12 | disclosed; no marker pollution and no LTP failure |

This remains below the stable413 disclosed ramfs `NotADirectory` noise (22 per arch) and does not affect LTP wrapper parsing.

## Source and behavior notes

Source change in this final story:

- `examples/shell/src/cmd.rs`: adds 8 proven stable460 cases to `LTP_STABLE_CASES` and keeps the live list at exactly 460 unique cases.

No LTP test source was modified. No fixed result was returned based on test name/path/process name. This final story makes no new syscall, errno, ABI, or POSIX-visible behavior change beyond increasing the stable evaluator case list. Earlier campaign commits added real behavior fixes for VFS parent permissions, pipe SIGPIPE teardown ordering, `flock(2)`, and `pselect6` invalid-fd-set validation; those are now covered by the stable460 aggregate gates.

## Local verification commands run for this final story

```bash
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable460-*.log
python3 - <<'PY'
# live LTP_STABLE_CASES total/unique/duplicates check
PY
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable460-rv-final-gate-002.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/stable460-la-final-gate-002.log
cargo fmt --all -- --check
git diff --check
make A=examples/shell ARCH=riscv64
df -h / /root
du -sh /root/.codex
```

Post-validation disk check: `/` 37% used with 36G available; `/root/.codex` 1.4G.

## Stretch status

Stable470 stretch was not pursued after the stable460 final gate because the main objective consumed the available final-gate budget and the remaining clean reserves are fewer than the stretch delta. See `next-session-prompt-stable460-followup.md` for the next safe continuation.
