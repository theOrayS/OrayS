# Candidate Matrix: stable350 -> stable375

Date: 2026-05-25

Promotion rule: only RV+LA × musl+glibc clean cases can enter `LTP_STABLE_CASES`. Wrapper success alone is insufficient; internal TFAIL/TBROK/TCONF, timeout, ENOSYS, and panic/trap are blockers unless explicitly disclosed as the existing `read02` baseline TCONF.

## Promotion summary

| Stage | New cases | Count | Evidence | Decision |
| --- | --- | ---: | --- | --- |
| stable360 | `access02`, `fchmodat02`, `inode01`, `mmap06`, `ftest01`, `ftest02`, `ftest03`, `ftest04`, `mmap10`, `stream01` | 10 | targeted RV+LA evidence; covered by final RV/LA stable375 gates | accepted |
| stable368 | `ftest05`, `ftest07`, `ftest08`, `mmap09`, `mmap11`, `stream03`, `stream04`, `stream05` | 8 | targeted RV+LA evidence; covered by final RV/LA stable375 gates | accepted |
| stable375 | `abort01`, `poll01`, `fork05`, `fork10`, `kill11`, `kill12`, `mem02` | 7 | targeted RV+LA evidence; covered by final RV/LA stable375 gates | accepted |

Final live list after promotion: `375 total / 375 unique / 0 duplicates`.

## Primary + stretch pool outcomes

| Case | Pool | Subsystem | Evidence | Status | Notes |
| --- | --- | --- | --- | --- | --- |
| `access02` | primary | VFS/permission | `target-primary30-rv-002`, `target-rvclean5-la-001`, final gates | promoted stable360 | clean RV+LA × musl+glibc |
| `access04` | primary | VFS/permission | `target-primary30-rv-002` | blocked | not clean across RV targeted run |
| `chmod05` | primary | VFS/permission | `target-primary30-rv-002` | blocked | partial only; musl TBROK, not promoted |
| `chmod06` | primary | VFS/permission | `target-primary30-rv-002` | blocked | not clean |
| `chmod07` | primary | VFS/permission | `target-primary30-rv-002` | blocked | not clean |
| `fchmod02` | primary | VFS/permission | `target-primary30-rv-002` | blocked | not clean |
| `fchmod05` | primary | VFS/permission | `target-primary30-rv-002` | blocked | partial only; musl TBROK, not promoted |
| `fchmod06` | primary | VFS/permission | `target-primary30-rv-002` | blocked | not clean |
| `fchmodat02` | primary | VFS/permission | `target-primary30-rv-002`, `target-rvclean5-la-001`, final gates | promoted stable360 | clean RV+LA × musl+glibc |
| `statx01` | primary | VFS/metadata | `target-primary30-rv-002` | blocked | statx mask/semantics still not enough for clean case |
| `readlinkat02` | primary | VFS/path | `target-primary30-rv-002`, `target-rvclean5-la-001` | blocked | RV clean, LA musl TFAIL; not promoted |
| `rename01` | primary | VFS/metadata | `target-primary30-rv-002` | blocked | not clean |
| `rename03` | primary | VFS/metadata | `target-primary30-rv-002` | blocked | not clean |
| `rename04` | primary | VFS/metadata | `target-primary30-rv-002` | blocked | not clean |
| `openat02` | primary | VFS/FD | `target-primary30-rv-002` | blocked | not clean |
| `writev03` | primary | FD/iovec | `target-primary30-rv-002` | blocked | not clean |
| `pipe2_02` | primary | FD/pipe | `target-primary30-rv-002`, final gates | already stable | clean but already in stable350; not counted |
| `waitid07` | primary | process/wait | `target-primary30-rv-002` | blocked | not clean |
| `waitid08` | primary | process/wait | `target-primary30-rv-002` | blocked | not clean |
| `waitid10` | primary | process/wait | `target-primary30-rv-002` | blocked | not clean |
| `kill02` | primary | process/signal | targeted clean, `stable375-la-final-001` failed | demoted blocker | targeted clean was not enough; LA full stable aggregate produced TBROK/setup failure, so it was removed before final delivery |
| `mmap04` | primary | VM/mmap | `target-primary30-rv-002` | blocked | not clean |
| `mmap05` | primary | VM/mmap | `target-primary30-rv-002` | blocked | not clean |
| `mmap06` | primary | VM/mmap | `target-primary30-rv-002`, `target-rvclean5-la-001`, final gates | promoted stable360 | clean RV+LA × musl+glibc |
| `munmap01` | primary | VM/munmap | `target-primary30-rv-002` | blocked | not clean |
| `mprotect01` | stretch | VM/protection | `target-primary30-rv-002` | blocked | not clean |
| `mprotect02` | stretch | VM/protection | `target-primary30-rv-002` | blocked | not clean |
| `openat03` | stretch | VFS/FD | not promoted | deferred | no clean RV+LA evidence in this phase |
| `rename05` | stretch | VFS/metadata | not promoted | deferred | no clean RV+LA evidence in this phase |
| `statx03` | stretch | VFS/metadata | not promoted | deferred | no clean RV+LA evidence in this phase |

## Fallback / adjacent outcomes

| Case(s) | Evidence | Status | Notes |
| --- | --- | --- | --- |
| `inode01` | `target-fs8-rv-001`, `target-inode01-la-001`, final gates | promoted stable360 | replacement for `kill02`; clean on both arches/libcs |
| `ftest01`-`ftest04`, `mmap10`, `stream01` | `target-fallback18-rv-001`, `target-fallback6-la-001`, final gates | promoted stable360 | clean on both arches/libcs |
| `ftest05`, `ftest07`, `ftest08`, `mmap09`, `mmap11`, `stream03`-`stream05` | `target-adjacent20-rv-001`, `target-adjacent8-la-001`, final gates | promoted stable368 | clean on both arches/libcs |
| `abort01`, `fork05`, `kill11`, `mem02` | `target-scout-clean5-rv-001`, `target-stable375-la-001`, final gates | promoted stable375 | clean; `fork05` is a pass/TINFO case, no parser TCONF |
| `fork10`, `kill12` | `target-scout14-rv-001`, `target-stable375-la-001`, final gates | promoted stable375 | clean on both arches/libcs |
| `poll01` | `target-fill19-rv-001`, `target-poll01-la-001`, final gates | promoted stable375 | replacement for duplicate `alarm06`; clean on both arches/libcs |
| `alarm06` | `target-scout-clean5-rv-001`, `target-stable375-la-001` | not counted | already in stable350; duplicate was removed before final list |
| `clock_getres01`, `getrusage02` | scout/fill logs | not promoted | pass_with_tconf; not clean |
| `kill10`, `signal01`, `clock_gettime01`, `fork13`, `fork14`, `sigrelse01` | scout/fill logs | not promoted | timeout or failure |
| `pipe02` | `target-scout26-rv-001` | blocker | triggered panic during discovery; discovery log is not promotion evidence |
| `kill13`, `rt_sigaction03`, `sigaltstack*`, `mmap13`, `mmap14` | `target-scout14-rv-001` | not promoted | TFAIL/TBROK/ENOSYS or partial-only results |

## Blocker themes for follow-up

- Primary VFS metadata still has real semantic gaps (`statx01`, `readlinkat02`, `rename*`, `openat02`, `chmod*`, `fchmod*`).
- Signal/process adjacent cases include real timeout/TBROK/TFAIL blockers (`kill02`, `kill10/13`, `fork13/14`, `sigrelse01`, `rt_sigaction03`, `sigaltstack*`).
- Pipe discovery exposed a real `pipe02` panic and must not be treated as score evidence until root-caused.
- VM stretch (`mprotect*`, `mmap13/14`, `munmap01`) remains blocked by page protection/unmap semantics.
