# milestone-01-stable556 validation

## Environment

- Working branch: `dev/1000ltp-plan`
- Clean eval worktree used for non-TTY QEMU runs: `/root/oskernel2026-orays-1000ltp-eval-20260601-1403`
- Source/evidence output directory: `target/ltp-1000-milestone-01-stable556/`
- RV image: `/root/oskernel2026-orays/sdcard-rv.img`
- LA image: `/root/oskernel2026-orays/sdcard-la.img`
- Parser: `python3 -B scripts/ltp_summary.py`

## Invalidated evidence

The following logs are explicitly not promotion evidence:

- `target/ltp-1000-milestone-01-stable556/rv-m01-proof-001-20260601T135939Z.log`: root worktree dirty/no guest output/manual termination.
- `target/ltp-1000-milestone-01-stable556/rv-sanity-clean-access01-20260601T140354Z.log`: PTY job-control stopped QEMU/manual termination.

## Sanity proof

- Log: `target/ltp-1000-milestone-01-stable556/rv-sanity-clean-access01-notty-20260601T140703Z.log`
- Summary: `target/ltp-1000-milestone-01-stable556/rv-sanity-clean-access01-notty-20260601T140703Z.summary.txt`
- Parser result: PASS LTP CASE 2, FAIL 0, internal 0, timeout 0, ENOSYS 0, panic/trap 0.

## Targeted proof batches

| Evidence | Parser summary |
| --- | --- |
| `rv-m01-proof-001-clean-notty-20260601T140810Z.log` | PASS 40, FAIL 4, Internal TBROK 8, timeout 0, ENOSYS 0, panic/trap 0; `openat02`/`openat03` not promoted. |
| `la-m01-proof-001-clean-notty-20260601T141147Z.log` | PASS 40, FAIL 0, Internal 0, timeout 0, ENOSYS 0, panic/trap 0. |
| `rv-m01-proof-002-clean-notty-20260601T141814Z.log` | PASS 50, FAIL 42, Internal TFAIL 15/TBROK 33/TCONF 4, timeout 2, ENOSYS 0, panic/trap 0; blocked rows not promoted. |
| `rv-m01-proof-003-clean-notty-20260601T142317Z.log` | PASS 58, FAIL 0, Internal 0, timeout 0, ENOSYS 0, panic/trap 0. |
| `la-m01-proof-002-clean-notty-20260601T142544Z.log` | PASS 102, FAIL 0, Internal 0, timeout 0, ENOSYS 0, panic/trap 0. |
| `m01-proof-001-003-rv-la-promotion-candidates.txt` | 71 four-way clean candidates, 26 blocked/incomplete cases. |

## Final stable gates

Final RV/LA parser summaries after `LTP_STABLE_CASES` reached 556: both arches completed full `LTP_CASES=stable` musl+glibc gates with wrapper PASS and no new promoted-case TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap. The only internal TCONF remains inherited `read02` (2 per libc).

| Arch | Log | Summary | JSON | Checksum | Parser result |
| --- | --- | --- | --- | --- | --- |
| RV | `target/ltp-1000-milestone-01-stable556/rv-stable556-final-clean-notty-20260601T143114Z.log` | `target/ltp-1000-milestone-01-stable556/rv-stable556-final-clean-notty-20260601T143114Z.summary.txt` | `target/ltp-1000-milestone-01-stable556/rv-stable556-final-clean-notty-20260601T143114Z.summary.json` | `target/ltp-1000-milestone-01-stable556/rv-stable556-final-clean-notty-20260601T143114Z.sha256` | PASS LTP CASE 1112; FAIL 0; Internal TCONF 4 (`read02` only: musl 2 + glibc 2); timeout 0; ENOSYS 0; panic/trap 0; ltp-musl 556/0, ltp-glibc 556/0. |
| LA | `target/ltp-1000-milestone-01-stable556/la-stable556-final-clean-notty-20260601T150107Z.log` | `target/ltp-1000-milestone-01-stable556/la-stable556-final-clean-notty-20260601T150107Z.summary.txt` | `target/ltp-1000-milestone-01-stable556/la-stable556-final-clean-notty-20260601T150107Z.summary.json` | `target/ltp-1000-milestone-01-stable556/la-stable556-final-clean-notty-20260601T150107Z.sha256` | PASS LTP CASE 1112; FAIL 0; Internal TCONF 4 (`read02` only: musl 2 + glibc 2); timeout 0; ENOSYS 0; panic/trap 0; ltp-musl 556/0, ltp-glibc 556/0. |

## Stable count check

Command:

```bash
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
```

Expected after edit: `556 556 0`.

## Remaining validation gaps

- Full all-minus-blacklist sweep is not part of this milestone gate.
- Deferred clean/harness-adjacent candidates are not promoted in this commit.
