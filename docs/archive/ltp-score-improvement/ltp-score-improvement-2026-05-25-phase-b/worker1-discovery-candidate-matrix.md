# Worker 1 discovery candidate matrix: stable350 -> stable375 / stretch stable380

Date: 2026-05-25
Worker: `worker-1`
Scope: discovery-only. This lane did **not** run QEMU, did **not** mutate `.omx/ultragoal`, and did **not** edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.

## Source refresh

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: **350 total / 350 unique / 0 duplicates**.
- Stable350 final evidence from `docs/ltp-score-improvement-2026-05-25-phase-a/stable350-delivery-report.md`: RV and LA final stable gates each reported **PASS LTP CASE 700 / FAIL 0**, `ltp-musl 350/0`, `ltp-glibc 350/0`, known `read02` `TCONF=4` only, timeout/ENOSYS/panic-trap 0, marker-prefix bad 0.
- `scripts/ltp_summary.py --promotion-candidates output_rv.md output_la.md` is usable, but its current top-level promotion set contributes **0 non-stable cases** after filtering against live stable350. The phase-a saved remote promotion report likewise contributes **0 non-stable cases** after filtering; those clean cases are already in stable350.
- Historical sdcard binary inventory under `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-*-ltp-bin-cases.txt` confirms every phase-b primary/stretch candidate is present in all four RV/LA x musl/glibc images except none of the primary/stretch set. Fallback inventory gaps: `fs_perms01`-`fs_perms06`, `rwtest01`, `rwtest02`, and `mmap10_1` are absent from those historical bin lists.
- Current worktree has no live `testsuite/` tree and only one phase-a raw `.log`; most phase-a evidence is already parsed `*-summary.txt`. Therefore this report is a promotion **input**, not a promotion gate.

Promotion-clean rule used below: a non-stable case is clean only after serialized leader-owned RV+LA x musl+glibc evidence shows wrapper PASS and zero internal `TFAIL`/`TBROK`/new `TCONF`, timeout, ENOSYS/not-implemented, and panic/trap. Wrapper PASS alone and worker/QEMU discovery evidence are insufficient.

## Executive finding

There are **no immediately promotable non-stable cases** proven by the inspected artifacts. The best ROI path is still the planned 25-primary + 5-stretch pool, but it should be treated as a repair/test ordering queue:

1. `kill02`, `chmod05`, `fchmod05`, and `readlinkat02` have the strongest near-clean/targeted evidence, but each has a concrete caveat or conflict that requires a fresh isolated four-way gate.
2. Permission/rename/statx cases have high hidden-test value but current RV summaries show real `TFAIL`/`TBROK`/ENOSYS blockers.
3. `waitid*` and VM/mprotect cases remain deeper repair tranches; several show `TFAIL`, `TBROK`, or signal/segv-style exit (`code=139`) and should not be used to fill stable360 unless the owning worker lands a real semantic fix.
4. `pipe2_02` appears in the phase-b primary list but is already in live stable350 and should be treated only as a regression guard, not a new promotion candidate.

## Primary 25 matrix

Legend: `4/4 inv` = present in historical RV/LA x musl/glibc sdcard bin lists. `Unknown` means no phase-a row was found in inspected summaries, not that the case is clean.

| Case | Subsystem | Inventory | Best observed status | Classification | ROI / recommendation |
| --- | --- | --- | --- | --- | --- |
| `access02` | VFS/permission | 4/4 inv | RV glibc+musl FAIL `TFAIL=4` | TFAIL | High hidden-test value; Worker 2 repair before retest. |
| `access04` | VFS/permission | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Pair with `access02`; likely setup/permission errno mismatch. |
| `chmod05` | VFS/permission | 4/4 inv | RV glibc PASS, RV musl FAIL `TBROK=1` | mixed clean/TBROK | Nearer than other chmod cases; stable360 scout after musl/setup fix. |
| `chmod06` | VFS/permission | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Good stable368 candidate only after common chmod setup repair. |
| `chmod07` | VFS/permission | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Same tranche as `chmod06`; not stable360 material yet. |
| `fchmod02` | VFS/fd permission | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | High ROI if fchmod setup semantics are fixed. |
| `fchmod05` | VFS/fd permission | 4/4 inv | RV glibc PASS, RV musl FAIL `TBROK=1` | mixed clean/TBROK | Near-clean scout; do not promote without LA and RV musl clean. |
| `fchmod06` | VFS/fd permission | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Batch with fchmod repair. |
| `fchmodat02` | VFS/*at permission | 4/4 inv | RV glibc+musl FAIL `TFAIL=1` | TFAIL | Valuable *at flag/path test; repair before promotion. |
| `statx01` | VFS/metadata/statx | 4/4 inv | RV glibc+musl FAIL `TBROK=1`, ENOSYS=1 | TBROK+ENOSYS | Do not use for stable360; statx implementation/errno story first. |
| `readlinkat02` | VFS/readlinkat | 4/4 inv | LA glibc PASS, LA musl TFAIL; RV has both clean targeted rows and later TFAIL rows | conflicting/TFAIL | High ROI but conflicting evidence; fresh isolated RV+LA gate required after readlinkat fix. |
| `rename01` | VFS/rename | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Rename cluster candidate after setup/namespace repair. |
| `rename03` | VFS/rename | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Keep with rename cluster, not first tranche. |
| `rename04` | VFS/rename | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Keep with rename cluster, not first tranche. |
| `openat02` | VFS/openat | 4/4 inv | No phase-a row found | unknown | Good stable360 scout because inventory-present and likely low-runtime, but needs first evidence. |
| `writev03` | FD/iovec | 4/4 inv | RV glibc+musl FAIL with `TCONF=1` | TCONF/fail-wrapper | Keep transparent; do not launder TCONF into clean promotion. |
| `pipe2_02` | FD/pipe | 4/4 inv | Live stable350 already includes it; final stable gates clean | already stable | Regression guard only; remove from new-candidate accounting. |
| `waitid07` | process/wait | 4/4 inv | RV musl FAIL `TFAIL=5` | TFAIL | Worker 4 repair tranche; not stable360. |
| `waitid08` | process/wait | 4/4 inv | RV musl FAIL `TFAIL=10` | TFAIL | Same waitid repair tranche. |
| `waitid10` | process/wait | 4/4 inv | RV musl FAIL `TBROK=1` | TBROK | Same waitid repair tranche. |
| `kill02` | signal/process | 4/4 inv | RV clean; LA has a prior final glibc `TBROK=4` and later LA targeted clean rows | conflicting/TBROK history | Highest ROI signal candidate, but only after fresh isolated LA glibc + full four-way gate. |
| `mmap04` | VM/mmap | 4/4 inv | RV glibc+musl FAIL `TBROK=1` | TBROK | Worker 5 repair/guardrail; not promotion-ready. |
| `mmap05` | VM/mmap | 4/4 inv | RV code 139 in blocker batch; later RV `TBROK=1` | signal/segv + TBROK | Treat code 139 as segv-style test failure, not parser panic_trap; repair first. |
| `mmap06` | VM/mmap | 4/4 inv | RV glibc+musl FAIL `TFAIL=7` | TFAIL | VM semantic repair before retest. |
| `munmap01` | VM/munmap | 4/4 inv | RV glibc+musl code 139 | signal/segv | Deep VM/exit-status blocker; keep out of stable360. |

## Stretch 5 matrix

| Case | Subsystem | Inventory | Best observed status | Classification | Recommendation |
| --- | --- | --- | --- | --- | --- |
| `mprotect01` | VM/protection | 4/4 inv | RV glibc code 139; RV musl `TFAIL=3` | signal/segv + TFAIL | Stretch only after Worker 5 protection semantics repair. |
| `mprotect02` | VM/protection | 4/4 inv | RV glibc+musl `TBROK=2` | TBROK | Stretch only; useful regression once mprotect setup works. |
| `openat03` | VFS/openat | 4/4 inv | No phase-a row found | unknown | Lightweight scout candidate after `openat02`. |
| `rename05` | VFS/rename | 4/4 inv | RV glibc+musl `TBROK=1` | TBROK | Rename stretch; pair with `rename03/04`. |
| `statx03` | VFS/statx | 4/4 inv | No phase-a row found; statx family already shows ENOSYS/TBROK around `statx01` | unknown/statx-risk | Stretch only after statx base semantics are addressed. |

## Fallback pool

| Pool | Inventory finding | Recommendation |
| --- | --- | --- |
| `ftest01`-`ftest04` | 4/4 historical bin presence | Best fallback for low-risk file I/O scouting if primary fixes stall; no phase-a rows found. |
| `stream01`, `stream02` | 4/4 historical bin presence | Good libc/I/O smoke fallback; verify runtime and memory pressure first. |
| `mmap10`, `vma01`, `vma02` | 4/4 historical bin presence | VM fallback after Worker 5 guardrails; not first promotion batch. |
| `fs_perms01`-`fs_perms06` | 0/4 historical bin presence | Do not schedule until current sdcard/runtest inventory proves they exist. |
| `rwtest01`, `rwtest02` | 0/4 historical bin presence | Do not schedule until current inventory proves they exist. |
| `mmap10_1` | 0/4 historical bin presence | Inventory gap; leave out unless leader refreshes sdcard contents. |

## Recommended tranches

These are **test/repair target tranches**, not stable-list edits. The leader should only promote the subset that becomes clean under serialized RV+LA x musl/glibc gates.

### stable360 target queue (highest chance to yield 10 after small fixes/scouts)

```text
kill02,chmod05,fchmod05,readlinkat02,openat02,openat03,ftest01,ftest02,ftest03,ftest04
```

Rationale: `kill02` has high ROI but a prior LA glibc TBROK final-gate regression; `chmod05`/`fchmod05` are closer than the rest of the chmod/fchmod cluster; `readlinkat02` is valuable but conflicting; `openat*` and `ftest*` are inventory-present scouts that may be cheaper than known TBROK-heavy cases. If any of these show `TCONF`, timeout, ENOSYS, code 139, or panic/trap, drop them from stable360 immediately.

### stable368 target queue (permission/rename repair tranche)

```text
access02,access04,chmod06,chmod07,fchmod02,fchmod06,fchmodat02,rename01
```

Rationale: these are high-value VFS/permission tests with current `TFAIL`/`TBROK` evidence. They need Worker 2 repairs before they can become promotion candidates.

### stable375 target queue (second VFS + process/FD tranche)

```text
rename03,rename04,rename05,statx01,statx03,writev03,waitid07
```

Rationale: this tranche starts with rename/statx/writev/waitid blockers after the earlier VFS fixes. `writev03` currently has TCONF and must stay transparent; `statx*` should be dropped if ENOSYS remains; `waitid07` can be replaced by `waitid08`/`waitid10` only after Worker 4 repairs prove clean status.

### stretch stable380 / fallback queue

```text
waitid08,waitid10,mmap04,mmap05,mmap06,munmap01,mprotect01,mprotect02,stream01,stream02,mmap10,vma01,vma02
```

Rationale: VM/wait cases are valuable but deeper; they should follow semantic fixes and guardrail evidence, not fill a score tranche by hope. `stream*`/`vma*` are inventory-present fallback scouts if VM guardrails improve.

## Evidence map

- `examples/shell/src/cmd.rs`: live stable list = 350 total / 350 unique; `pipe2_02` is already stable; `kill02` is not stable.
- `docs/ltp-score-improvement-2026-05-25-phase-a/stable350-delivery-report.md`: final stable350 RV/LA aggregate gate summary and honesty notes.
- `docs/ltp-score-improvement-2026-05-25-phase-a/candidate-matrix.md`: stable315/330/350 promoted stages and demoted/blocked candidate notes.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/blocker-batch-rv-summary.txt` and `worker5-readonly-blocker-batch-rv-summary.txt`: access/statx/writev/mmap/mprotect blocker classifications.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-discovery-fsproc-001-summary.txt`: chmod/fchmod/rename/statfs/prctl RV classifications.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-rv-lowrisk-tail-001-summary.txt`: fchmod/readlinkat/rename low-risk tail evidence.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-discovery-process-short-001-summary.txt`: `waitid07/08/10` and signal/process residuals.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-la-final-summary.txt` and `stable350-la-discovery-fsproc-clean-001-summary.txt`: `kill02` conflicting LA evidence.
- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-*-ltp-bin-cases.txt`: historical sdcard binary presence matrix for primary/stretch/fallback pools.
- `python3 -B scripts/ltp_summary.py --promotion-candidates output_rv.md output_la.md`: current parser usable; no non-stable clean candidates after live stable350 filtering.

## Guardrails / gaps

- No QEMU was started in this lane; all runtime classifications are from existing parsed artifacts.
- Worker discovery evidence must not be used as final promotion proof.
- `TCONF` remains visible (`writev03`, known stable `read02` elsewhere); this report does not convert `TCONF` to clean PASS.
- Code `139` VM rows are treated as signal/segv-style failures; the inspected summaries did not mark them as parser `panic_trap`, but they are still blockers.
- The fallback inventory is historical. Before scheduling `fs_perms*`, `rwtest*`, or `mmap10_1`, refresh sdcard/runtest inventory in a leader-owned safe path.
