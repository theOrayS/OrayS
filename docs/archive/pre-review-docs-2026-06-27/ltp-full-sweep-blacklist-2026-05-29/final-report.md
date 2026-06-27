# LTP full-sweep blacklist final report (2026-05-29)

## Verdict

- **RV gate satisfied:** `rv-iter006` is a closed `LTP_CASES=blacklist` full sweep: no unclosed `RUN LTP CASE`, no unexplained QEMU hang/kernel panic/trap/guest OOM, and parser-backed summary plus inline-aware marker audit are recorded as durable docs in this directory.
- **LA comparison executed with the same RV-converged blacklist:** `la-iter001` hit an `arch=la` blocker at `creat07`. It is documented separately and **not** added to the generic supplemental blacklist.
- **No fake pass:** blacklist/SKIP are exclusions only; wrapper `FAIL ... : 0` is reported as parser-normalized pass evidence but not stable-promotion proof.

## Baseline and scope

- Branch: `exp/ltp-full-sweep-blacklist`
- Original HEAD: `9e8c6ed7`
- Clean detached run worktree HEAD: `b1c8090b` (docs-only Team integration commits; source unchanged for evaluator semantics)
- Live `LTP_STABLE_CASES`: 460 total / 460 unique
- Mode: `LTP_CASES=blacklist` (runner reports `all-minus-blacklist skipped=N`)
- Generic supplemental blacklist: `docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt`
- Supplemental blacklist cases (5): `pthserv, oom01, shmat1, accept02, mincore03`
- Source default blacklist: `examples/shell/src/cmd.rs::LTP_SWEEP_DEFAULT_BLACKLIST_CASES` (38 cases)
- Raw logs: `target/ltp-full-sweep-blacklist-2026-05-29/raw/` (not committed)
- Artifact manifest: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/manifest.json`
- High-yield candidate evidence: `docs/ltp-full-sweep-blacklist-2026-05-29/high-yield-candidates.json`

## RV final closed run: rv-iter006

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter006.log`
- Summary artifacts:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-summary.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-compact.json`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-marker-audit.json`
- Selection / skipped:
  - `ltp case list: all-minus-blacklist skipped=40 (2328 cases, timeout 15s)`
  - `ltp case list: all-minus-blacklist skipped=43 (2332 cases, timeout 15s)`

- Wrapper markers: RUN=4660, raw PASS markers=0, raw FAIL markers=4660, `FAIL ... : 0`=1186, nonzero FAIL=3474, TIMEOUT=68, SKIP=0, incomplete=0.
- Parser summary: normalized PASS=1186, FAIL=3473, status_counts=`{'PASS': 1186, 'FAIL': 3473, 'UNKNOWN': 1}`, case_matrix=4660.
- Internal signals: TPASS(raw)=6051, TFAIL=4119, TBROK=1042, TCONF=2663, ENOSYS/not implemented=1280, kernel panic/trap=0.
- Closure: `clean closed: RUN_META exit_code=0, run_eval_status=0, no incomplete RUN stack, no WORKER/LEADER stall meta, no qemu signal, no make-killed evidence`.
- Parser caveat: `cpuset_memory_pressure` has a glued terminal wrapper marker and appears as one `UNKNOWN` row in `scripts/ltp_summary.py`; inline-aware audit records it as closed `FAIL : 1`, not a blocker.

## LA comparison: la-iter001

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.monitor.log`
- Summary artifacts:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-summary.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-compact.json`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-marker-audit.json`
- Selection / skipped:
  - `ltp case list: all-minus-blacklist skipped=40 (2328 cases, timeout 15s)`

- Wrapper markers before blocker: RUN=160, raw PASS markers=0, raw FAIL markers=159, `FAIL ... : 0`=47, nonzero FAIL=112, TIMEOUT=2, SKIP=0, incomplete=1 (`creat07`).
- Parser summary: normalized PASS=47, FAIL=112, status_counts=`{'PASS': 47, 'FAIL': 112, 'UNKNOWN': 1}`, case_matrix=160.
- Internal signals: TPASS(raw)=506, TFAIL=23, TBROK=28, TCONF=104, ENOSYS/not implemented=29, kernel panic/trap=0.
- Closure: `not closed: incomplete_stack=1; qemu_signal=1; leader_meta_raw=0 monitor=1`.
- `arch=la` blocker: `creat07` — LA-only sweep hang / incomplete RUN after TBROK checkpoint timeout. First evidence: target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.log: last RUN LTP CASE creat07 after creat06 closure; target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.monitor.log: LEADER_META arch=la blocker=creat07 action=terminate_stalled_qemu at=2026-05-30T04:27:24Z reason="no log growth for 392s; last_run=creat07; last_terminal=creat06; qemu still active".
- Generic blacklist delta: none. The LA-only blocker is not inserted into `blacklist.txt`.

## RV/LA delta

| Field | RV iter006 | LA iter001 |
| --- | ---: | ---: |
| Closed full sweep | yes | no (`creat07`, arch=la) |
| Selection skipped | 40 / 43 | 40 before blocker |
| RUN markers | 4660 | 160 |
| raw FAIL markers | 4660 | 159 |
| `FAIL ... : 0` | 1186 | 47 |
| nonzero FAIL | 3474 | 112 |
| TIMEOUT markers | 68 | 2 |
| incomplete stack | 0 | 1 |
| parser PASS | 1186 | 47 |
| parser FAIL | 3473 | 112 |
| parser UNKNOWN | 1 | 1 |
| ENOSYS/not implemented | 1280 | 29 |
| kernel panic/trap | 0 | 0 |

## High-yield targeted fix candidates

These are **triage hypotheses**, not promotion proof. Per-case `TPASS/TFAIL/TBROK/TCONF/ENOSYS` evidence for the listed cases is normalized in `high-yield-candidates.json`; raw logs remain the source for any deeper testcase-output audit while retained locally.

1. **LA `creat07` hang (`arch=la`)**
   - Why high-yield: currently prevents LA full-sweep comparison from reaching later LTP cases.
   - Evidence: `la-iter001` incomplete stack contains `creat07`; monitor killed QEMU after 392s no log growth.
   - Removal condition: targeted `creat07` on LA must close with PASS/FAIL/TIMEOUT and no stalled QEMU/guest hang.

2. **Timer/syscall clusters with high TPASS density but wrapper FAIL**
   - `getitimer01`: both musl/glibc show 16 TPASS + 16 TFAIL and wrapper `FAIL : 1` on RV.
   - `ppoll01`: both musl/glibc show 16 TPASS + 4 TFAIL and wrapper `FAIL : 1` on RV.
   - `setitimer01` / `times03`: repeated partial success, then nonzero wrapper status or timeout; likely good ROI in time/signal semantics.

3. **Filesystem / metadata cases with partial success**
   - `diotest4`: both libc variants have 14 TPASS, 3 TFAIL, 2 TCONF, wrapper `FAIL : 33`.
   - `open10` / `creat08`: both show multiple TPASS but wrapper `FAIL : 1`; likely narrower errno/flag/permission semantics than broad subsystem rewrites.

4. **IPC / SysV fan-out cases**
   - `shmctl02`: both libc variants show 7 TPASS but 13 TFAIL and wrapper `FAIL : 5`; related fixes may improve several SysV IPC cases.
   - Existing blacklist `shmat1` should only be removed after targeted run proves no OOM/QEMU kill.

5. **Memory-mapping edge cases**
   - `remap_file_pages01`: both libc variants fail with code 139 and many TFAILs; investigate mapping compatibility and crash path.
   - Existing blacklist `mincore03` should only be removed after targeted run proves no host/guest OOM kill.

6. **Networking/socket partials**
   - `socket01`: both libc variants show 7 TPASS, 2 TFAIL, wrapper `FAIL : 1`.
   - This may have fan-out into socket errno/option behavior without needing blacklist changes.

## Blacklist removal candidates / future unblock gates

- `pthserv`: remove only after targeted `pthserv` closes normally without QEMU/guest stall.
- `oom01`: remove only after it completes without host/guest OOM or runner/QEMU kill.
- `shmat1`: remove only after shared-memory stress completes without host/guest OOM or runner/QEMU kill.
- `accept02`: remove only after both relevant variants, especially glibc, close without futex abort/stall.
- `mincore03`: remove only after it completes without host/guest OOM or runner/QEMU kill.
- `creat07` is **not** in generic blacklist; it is an `arch=la` comparison blocker only.

## Verification commands

- `python3 scripts/ltp_summary.py target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter006.log`
- `python3 scripts/ltp_summary.py --json target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter006.log`
- `python3 scripts/ltp_summary.py target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.log`
- `python3 scripts/ltp_summary.py --json target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.log`
- `python3 -m json.tool docs/ltp-full-sweep-blacklist-2026-05-29/summaries/*-compact.json`
- `git diff --check -- docs/ltp-full-sweep-blacklist-2026-05-29`

## Non-goals / honesty notes

- No stable promotion is claimed from this experiment.
- Blacklist/SKIP are not counted as PASS.
- Ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and closed timeouts remain failures.
- Raw logs are intentionally not committed; durable docs record paths, hashes, parser summaries, and blocker evidence. If `target/` is cleaned, the committed summaries/compact JSON/marker audits remain authoritative for this report, while `*.log.sha256` files only verify retained local raw logs.
