# Worker 1 discovery candidate matrix: stable375 -> stable450

Date: 2026-05-25
Team: `ltp-stable375-to-stab-eae749f6`
Worker: `worker-1`
Task: `task-1`
Scope: discovery/report only. No QEMU was started for promotion evidence, `examples/shell/src/cmd.rs::LTP_STABLE_CASES` was not edited, and `.omx/ultragoal` remained leader-owned and untouched.

## Source refresh

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: **375 total / 375 unique / 0 duplicates**.
- Stable375 final evidence from phase-b:
  - `docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-rv-final-002-summary.txt`: `PASS LTP CASE: 750`, `FAIL LTP CASE: 0`, `ltp-musl 375/0`, `ltp-glibc 375/0`, internal `TCONF=4`, timeout/ENOSYS/panic-trap 0.
  - `docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-la-final-003-summary.txt`: `PASS LTP CASE: 750`, `FAIL LTP CASE: 0`, `ltp-musl 375/0`, `ltp-glibc 375/0`, internal `TCONF=4`, timeout/ENOSYS/panic-trap 0.
  - `docs/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-final-marker-prefix.txt`: marker-prefix baseline for final stable375 gates.
- Remote phase-c baseline from `docs/ltp-score-improvement-2026-05-25-phase-c/raw/remote-output-noise-baseline.json`: remote glibc-only samples parse as 375 markers / 0 TFAIL / 0 TBROK / known `TCONF=2`, but include **4510 RV** and **4507 LA** `AxError::NotADirectory` lines.
- Promotion rule preserved from phase-b: a case is promotion-clean only after RV+LA x musl+glibc wrapper PASS and zero internal `TFAIL`/`TBROK`/new `TCONF`, timeout, ENOSYS/not-implemented, and panic/trap. Existing stable `read02` TCONF remains transparent and is not a template for new promotions.

## Executive finding

There are **no newly promotable non-stable cases proven by existing artifacts alone**. The phase-c Batch A-E pool is a repair/scout queue, not a promotion set. Highest ROI remains:

1. VFS metadata/permission/path (`openat*`, `rename*`, `statx*`, `readlinkat02`, `chmod*`, `fchmod*`) after the log-noise lane finishes.
2. FD/pipe/iovec (`writev03`, `pipe02` and adjacent pipe/readv/writev families) after preserving TCONF/TFAIL transparency.
3. Process/wait/signal (`kill02`, `waitid07/08/10`) only after raw/fresh four-way evidence resolves existing LA/TBROK and wait-state blockers.
4. VM/mmap/mprotect and fs-suite substitutes only as follow-up scouts; current evidence shows real semantic gaps.

## Matrix: clean but already stable / not a new promotion source

These cases are clean in phase-b evidence, but they are already in the live stable375 list and must not be counted again for stable400/425/450.

| Case(s) | Subsystem | Evidence | Classification | Decision |
| --- | --- | --- | --- | --- |
| `access02`, `fchmodat02`, `inode01`, `mmap06`, `ftest01`-`ftest04`, `mmap10`, `stream01` | VFS/permission/fs/VM | `docs/ltp-score-improvement-2026-05-25-phase-b/candidate-matrix.md` stable360 rows; final RV/LA stable375 summaries | clean, already stable | regression guard only |
| `ftest05`, `ftest07`, `ftest08`, `mmap09`, `mmap11`, `stream03`-`stream05` | fs/VM/stream | phase-b stable368 rows plus final RV/LA stable375 summaries | clean, already stable | regression guard only |
| `abort01`, `poll01`, `fork05`, `fork10`, `kill11`, `kill12`, `mem02` | process/signal/memory | phase-b stable375 rows plus final RV/LA stable375 summaries | clean, already stable | regression guard only |

## Matrix: known blockers from Batch A-E

| Case(s) | Batch | Subsystem | Best evidence | Current blocker classification | Recommended next action |
| --- | --- | --- | --- | --- | --- |
| `access04` | A | VFS/permission | `target-primary30-rv-002` in phase-b candidate matrix | RV targeted not clean / `TBROK` history | repair permission/setup errno, then fresh RV+LA x musl/glibc target |
| `chmod05`, `fchmod05` | A | VFS/permission | `target-primary30-rv-002`; phase-b notes RV glibc PASS but RV musl `TBROK=1` | partial clean plus musl TBROK | highest permission repair scouts after noise fix |
| `chmod06`, `chmod07`, `fchmod02`, `fchmod06` | A | VFS/permission | `target-primary30-rv-002` | RV glibc+musl `TBROK=1`/not clean | batch with chmod/fchmod setup semantics |
| `statx01` | A | VFS/statx | `target-primary30-rv-002` | `TBROK` plus ENOSYS/mask-risk history | do not scout until statx mask/syscall path is rechecked |
| `readlinkat02` | A | VFS/path | `target-primary30-rv-002`, `target-rvclean5-la-001` | conflicting: RV clean, LA musl TFAIL | fresh isolated four-way gate only after readlinkat fix |
| `rename01`, `rename03`, `rename04` | A | VFS/rename | `target-primary30-rv-002` | RV targeted not clean / TBROK family | repair rename/setup semantics as a cluster |
| `openat02` | A | VFS/openat | `target-primary30-rv-002` | not clean in phase-b target | good high-value scout after openat errno/path repair |
| `writev03` | B | FD/iovec | `target-primary30-rv-002`; phase-b discovery notes TCONF/fail-wrapper | TCONF/fail-wrapper; not promotion-clean | keep TCONF visible; rerun only after iovec/fd semantics repair |
| `pipe02` | B | pipe | `target-scout26-rv-001` | discovery panic | root-cause panic before any promotion attempt |
| `kill02` | C | process/signal | RV targeted clean, but `stable350-la-final-summary.txt` / phase-b aggregate history had LA glibc `TBROK=4`; removed before final stable375 | mixed/aggregate TBROK | capture raw LA glibc TBROK lines; no re-add from RV-only success |
| `waitid07`, `waitid08` | C | process/wait | worker4 report: RV failures `TFAIL=5`/`TFAIL=10`; static root cause requires stopped/continued child events | wait-state semantic gap | requires stopped/continued event model and `WNOWAIT` behavior |
| `waitid10` | C | process/wait/proc | worker4 report: RV `TBROK=1`; likely missing synthetic `/proc/sys/kernel/core_pattern` first | setup/proc blocker | leader-approved tiny proc sysctl scout, then four-way gate |
| `mmap04`, `mmap05`, `munmap01` | D | mmap/VM | phase-b candidate matrix plus worker5 report | `/proc/self/maps`, catchable SIGSEGV, and unmap semantics missing | VM semantic tranche; not first stable400 filler |
| `mprotect01`, `mprotect02` | D | VM/protection | phase-b candidate matrix plus worker5 report | max-permission/catchable SIGSEGV/write-permission semantics | repair protection metadata/signals first |
| `stream02` | E | fs/stream substitute | `target-stream02-rv-001-summary.txt`: RV glibc+musl FAIL, `TFAIL=4`, ENOSYS=2 | not implemented / TFAIL | not a low-risk filler until missing syscall/behavior is identified |
| `mmap13`, `mmap14`, `kill13`, `rt_sigaction03`, `sigaltstack*` | C/D adjacent | signal/VM | `target-scout14-rv-001` phase-b notes | TFAIL/TBROK/ENOSYS or partial-only | lower priority than Batch A repairs |

## Matrix: unknown / deferred candidates needing first fresh evidence

These appear in the phase-c Batch A-E plan but have no clean RV+LA x musl/glibc proof in inspected artifacts.

| Case(s) | Batch | Subsystem | Evidence state | Scout priority |
| --- | --- | --- | --- | --- |
| `openat03`, `rename05`, `statx03` | A | VFS/openat/rename/statx | phase-b candidate matrix marks deferred/no clean RV+LA evidence | medium after `openat02`/rename/statx base repairs |
| `pipe07`, `pipe08`, `pipe15`, other `pipe2_*` not already stable | B | pipe | phase-c pool only; `pipe02` panic warns this family is risky | medium, after pipe panic root-cause |
| adjacent `readv`, `writev`, `preadv`, `pwritev`, `sendfile`, `fcntl` families | B | FD/iovec/fcntl | no four-way clean proof in inspected phase-b docs | medium-high if fd/iovec lane lands real semantic fixes |
| wait/waitpid/fork/clone/signal adjacent cases beyond listed blockers | C | process/signal | no clean proof from this report; phase-b blockers remain | medium after `kill02`/`waitid*` blockers are understood |
| `mmap10_1`, `mmap12`, `vma01`, `vma02` | D | VM/VMA | phase-c pool only; worker5 notes maps/VMA metadata gap | medium-low until `/proc/self/maps` and VMA accounting improve |
| `fs_perms01`-`fs_perms06`, `rwtest01`, `rwtest02`, `openfile01`, `writetest01`, `iogen01`, `fs_inod01`, `inode02`, `ftest06`, `ftest09` | E | fs-suite substitutes | phase-c pool only; prior phase-b discovery warned some fs/rw/mmap inventory was historical or absent | scout only after current sdcard/runtest inventory is refreshed |

## Recommended stable400 first scout queue

Use this as a **test/repair ordering queue**, not a stable-list edit:

```text
chmod05,fchmod05,readlinkat02,openat02,openat03,rename01,rename03,statx03,ftest06,ftest09,openfile01,inode02
```

Rationale: it mixes the nearest known partial cases (`chmod05`, `fchmod05`, `readlinkat02`) with high-value unknown/deferred scouts. Drop any case immediately if fresh evidence shows TFAIL/TBROK/new TCONF, timeout, ENOSYS, panic/trap, or bad marker prefix.

## Verification performed in this lane

- No QEMU / `run-eval.sh` started; this is discovery evidence only.
- Verified live stable list remains 375 total / 375 unique / 0 duplicates.
- Verified `examples/shell/src/cmd.rs::LTP_STABLE_CASES` has no diff.
- Verified `.omx/ultragoal` has no diff.
- Verified `kernel/fs/axfs/src/fops.rs` has no diff after leader correction; log-noise code changes belong to another lane.
- `git diff --check` on the final report diff passed.

## Subagent findings integrated

- Subagent `019e5fb5-f0fc-7292-a244-abdb21720967` mapped stable-list mechanics, parser/promotion evidence files, and Batch A-E candidate pools.
- Subagent `019e5fb5-ee4c-78a3-898b-bc09f64cdf84` mapped the log-noise source and call paths; after leader correction, its code-change recommendation was used only to confirm that this worker report should not retain `fops.rs` edits.
