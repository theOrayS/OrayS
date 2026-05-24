# Candidate matrix: stable250 -> stable300

Status: discovery matrix updated by worker-1 on 2026-05-24. Worker 1 owns discovery/classification updates; leader owns promotion decisions and any `LTP_STABLE_CASES` edit.

## Guardrails

- This matrix is **not** promotion proof. It proposes targeted candidates and preserves evidence gaps.
- Do not promote a candidate until fresh RV + LA, musl + glibc evidence is parsed with `python3 -B scripts/ltp_summary.py` or an equivalent case matrix.
- `clean` means wrapper PASS plus internal `TFAIL=0`, `TBROK=0`, `TCONF=0`, `timeout=0`, `ENOSYS=0`, and `panic/trap=0` for every required arch/libc row.
- Existing stable250 full gates are clean except known transparent `read02` `pass_with_tconf`; that caveat must not be generalized to new candidates.
- Inventory presence is useful for batching, but it is not PASS evidence.

## Evidence inputs used

| Evidence | What it proves / contributes |
| --- | --- |
| `docs/ltp-score-improvement-2026-05-24-phase-a/stable250-live.cases` | Live baseline snapshot has 250 entries, 250 unique, 0 duplicates; used to exclude already-stable cases. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/stable250-post-ansi-rv-summary.txt` | RISC-V stable250 aggregate: `PASS LTP CASE: 500`, `FAIL LTP CASE: 0`, timeout/ENOSYS/panic-trap 0; only known `read02` TCONF. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/stable250-post-ansi-la-summary.txt` | LoongArch stable250 aggregate: `PASS LTP CASE: 500`, `FAIL LTP CASE: 0`, timeout/ENOSYS/panic-trap 0; only known `read02` TCONF. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/worker1-stable200-wave2-refinement.md` | Refined broad discovery pool by removing already-stable and known RV-blocked cases; provides lane splits for process/fd/fs/time/mm/system candidates. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/worker1-stable200-wave2-refined-rv.cases` | Candidate list source for many not-yet-stable cases; discovery batch only, not clean proof. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/task4-fd-fs-metadata-lane-report.md` | Identifies FD/FS/metadata targeted batches and warns that link/rename/statfs/truncate families still require strict RV+LA proof. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/worker-2-stable200-proc-next-candidates.md` | Process/sched/wait/rlimit targeted candidate guidance and known risk notes. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/candidate-matrix.md` | Earlier blocked snapshot used to avoid promoting known TFAIL/TBROK/TCONF/timeout/ENOSYS candidates without fixes. |
| `sdcard-rv.img` and `sdcard-la.img` debugfs inventories | Verified the 30 proposed candidates below exist under both `/musl/ltp/testcases/bin` and `/glibc/ltp/testcases/bin` on both evaluator images. |
| `scripts/ltp_summary.py --promotion-candidates docs/ltp-score-improvement-2026-05-22-phase-d/stable250-post-ansi-*.txt` | Current stable250 summaries yield 0 new promotion candidates outside the stable list; therefore this matrix does not claim fresh candidate PASS. |

## Recommended targeted batches

### Batch A: stable270 discovery focus (10)

These are lowest-risk discovery candidates from the refined wave2 pool: simple identity, fd/io, metadata, and memory/sysinfo smoke surfaces. Run RV first, then LA only if RV is clean.

```text
gettid02
dup05
pread02
pwrite02
pwrite04
readlink03
fstat02
ftruncate04
mmap04
sysinfo03
```

### Batch B: stable285 discovery focus (10)

These expand the same low-risk families but add more path/metadata and mmap coverage.

```text
link04
link05
rename03
rename04
utime01
utime02
utimes01
futimesat01
mmap05
mmap06
```

### Batch C: stable300 discovery focus (10)

These are still inventory-confirmed and not in stable250, but should run after Batch A/B because they cover broader VM and sync semantics.

```text
renameat201
renameat202
mmap08
munmap01
munmap02
mprotect01
mprotect02
mincore01
sync01
syncfs01
```

## Candidate classification

Legend for RV/LA libc columns: `inventory-present; targeted pending` means the testcase binary exists on both sdcard images for that libc, but no current stable250->300 targeted PASS evidence has been produced in this phase. `Known blocker: none in refined snapshot` means the case survived the phase-d refined filter that removed known RV-blocked cases; it still needs fresh proof.

| Case | Stage | Subsystem | Source | RV musl | RV glibc | LA musl | LA glibc | Current classification | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `gettid02` | stable270 | process/identity | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `dup05` | stable270 | fd/dup | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `pread02` | stable270 | fd/io | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `pwrite02` | stable270 | fd/io | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `pwrite04` | stable270 | fd/io | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `readlink03` | stable270 | fs/path | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `fstat02` | stable270 | fs/metadata | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `ftruncate04` | stable270 | fs/truncate | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `mmap04` | stable270 | memory/mmap | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `sysinfo03` | stable270 | misc/system | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `link04` | stable285 | fs/link | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; link semantics need fresh strict proof; promotion proof pending | `worker1-stable200-wave2-refinement.md`; `task4-fd-fs-metadata-lane-report.md`; debugfs inventory check |
| `link05` | stable285 | fs/link | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; link semantics need fresh strict proof; promotion proof pending | `worker1-stable200-wave2-refinement.md`; `task4-fd-fs-metadata-lane-report.md`; debugfs inventory check |
| `rename03` | stable285 | fs/rename | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; rename semantics need fresh strict proof; promotion proof pending | `worker1-stable200-wave2-refinement.md`; `task4-fd-fs-metadata-lane-report.md`; debugfs inventory check |
| `rename04` | stable285 | fs/rename | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; rename semantics need fresh strict proof; promotion proof pending | `worker1-stable200-wave2-refinement.md`; `task4-fd-fs-metadata-lane-report.md`; debugfs inventory check |
| `utime01` | stable285 | fs/time metadata | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `utime02` | stable285 | fs/time metadata | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `utimes01` | stable285 | fs/time metadata | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `futimesat01` | stable285 | fs/time metadata | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `mmap05` | stable285 | memory/mmap | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `mmap06` | stable285 | memory/mmap | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `renameat201` | stable300 | fs/renameat | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; renameat semantics need fresh strict proof; promotion proof pending | `worker1-stable200-wave2-refinement.md`; `task4-fd-fs-metadata-lane-report.md`; debugfs inventory check |
| `renameat202` | stable300 | fs/renameat | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; renameat semantics need fresh strict proof; promotion proof pending | `worker1-stable200-wave2-refinement.md`; `task4-fd-fs-metadata-lane-report.md`; debugfs inventory check |
| `mmap08` | stable300 | memory/mmap | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `munmap01` | stable300 | memory/munmap | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `munmap02` | stable300 | memory/munmap | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `mprotect01` | stable300 | memory/mprotect | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `mprotect02` | stable300 | memory/mprotect | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `mincore01` | stable300 | memory/mincore | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `sync01` | stable300 | fs/sync | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |
| `syncfs01` | stable300 | fs/sync | wave2 refined discovery | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | inventory-present; targeted pending | Candidate; known blocker: none in refined snapshot; promotion proof pending | `worker1-stable200-wave2-refinement.md`; debugfs inventory check |

## Explicitly not promoted from this matrix

| Case / source | Reason to hold |
| --- | --- |
| `read02` | Stable250 known `pass_with_tconf`; remains transparent and is not a clean-new-candidate precedent. |
| Stable250 `--promotion-candidates` output | Parser over current stable250 summaries found 0 new candidates outside the stable list. |
| Raw sdcard inventory-only names such as `alarm05`, `clock_gettime03`, `clone02`, `chmod05`, `eventfd01`, `accept01` | Inventory proves availability only; no current clean RV+LA/musl+glibc evidence. Several related names are in older blocked snapshots. |
| `sched_getscheduler02`, `getpgid01`, `waitpid01`, `getrusage02`, `gettimeofday02` | Useful repair/validation targets, but prior reports carry blocker/history risk; keep in lane-specific validation rather than the first low-risk promotion batches. |
| `link01`/`link02`/`link03`, `rename01`/`rename02`, `statfs*`, `truncate01`, `ftruncate02`/`ftruncate03` | Task4 FS metadata report marks these as unknown or needing separate strict evidence; do not infer from refined wave2 cases. |

## Verification commands run by worker-1 for this matrix

```bash
python3 - <<'PY'
from pathlib import Path
stable = Path('docs/ltp-score-improvement-2026-05-24-phase-a/stable250-live.cases').read_text().split()
print(len(stable), len(set(stable)), len(stable) - len(set(stable)))
PY

python3 -B scripts/ltp_summary.py --promotion-candidates \
  docs/ltp-score-improvement-2026-05-22-phase-d/stable250-post-ansi-rv-summary.txt \
  docs/ltp-score-improvement-2026-05-22-phase-d/stable250-post-ansi-la-summary.txt

python3 - <<'PY'
# debugfs inventory check for the 30 proposed candidates on rv/la x musl/glibc
PY
```

Observed results:

- Baseline count check: `250 250 0`.
- Stable250 promotion-candidate parser report: `Promotion candidates: 0`; `Blocked/incomplete cases: 0` for the already summarized stable250 inputs.
- Debugfs inventory check: all 30 proposed candidates are available in RV musl, RV glibc, LA musl, and LA glibc testcase bins.

## Subagent integration

- Subagents spawned: 2 (`019e578d-5d72-7b10-b47b-8e5fbf372444` / Turing, `019e578d-7f16-7af3-be74-595f4a0cb38f` / Noether).
- Subagent model: `gpt-5.4-mini`.
- Findings integrated:
  - Noether confirmed current `--promotion-candidates` evidence has no stable250-external clean candidates and warned that sdcard inventories are not PASS evidence.
  - Turing identified phase-a/phase-d report sources and older discovery candidates; those were integrated as evidence paths and hold-list caveats.
- Serial searches before spawn: 1.
