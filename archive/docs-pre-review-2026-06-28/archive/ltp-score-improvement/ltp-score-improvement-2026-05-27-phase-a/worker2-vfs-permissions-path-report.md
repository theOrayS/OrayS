# Worker 2 VFS / permissions / path lane report

Date: 2026-05-27
Team task: `ltp-stable413-to-stab-d9f99e59` / task 2
Scope: VFS/permissions/path lane only. No `.omx/ultragoal` edits, no final `LTP_STABLE_CASES` edits, and no concurrent default QEMU run.

## Result summary

Worker 2 added parser regression tests for promotion-gate evidence semantics and completed a static VFS/path lane audit. The current VFS/path candidate set does **not** contain a promotion-ready case from existing evidence. The best next action is a narrow parent-directory permission/sticky-bit repair before re-scouting `open06`, `creat04`, `mkdir04`, `rmdir03`, and `unlink08`.

Report paths produced by this worker:

- `scripts/test_ltp_summary.py` — added promotion-mode tests for four-way clean evidence, missing combo blocking, internal TCONF blocking, prior failure-event blocking, and CLI multi-log behavior.
- `docs/ltp-score-improvement-2026-05-27-phase-a/worker2-vfs-permissions-path-report.md` — this lane report.

## Candidate status table

| Candidate / family | Current status | Evidence | Recommended next step |
| --- | --- | --- | --- |
| `chmod05`, `fchmod05`, `fchmodat02` | Already in live stable413; not reusable for +47 | Live `LTP_STABLE_CASES` contains these cases; stable413 final gate already protects them. | Keep as regression guard only. |
| `access04`, `chmod06`, `fchmod06` | Blocked | `docs/ltp-score-improvement-2026-05-25-phase-c/raw/target-stable400-access-chmod-rv-001-summary.txt`: RV musl+glibc FAIL, TBROK=1 per row; phase report attributes setup to tmpfs mount `EINVAL`. | Do not promote. Revisit only after mount/tmpfs setup compatibility is addressed. |
| `chmod07`, `fchmod02` | Blocked | Same RV summary: RV musl+glibc FAIL, TBROK=1 per row; phase report attributes setup to `getgrnam(daemon)` breakage. | Do not promote. Revisit after user/group database semantics are fixed. |
| `readlinkat02` | Blocked by LA musl | RV musl+glibc clean in `target-stable400-readlinkat02-rv-serial-001-summary.txt`; LA glibc clean but LA musl FAIL/TFAIL=1 in `target-stable400-readlinkat02-la-serial-001-summary.txt`. | Do not promote. Diagnose LA-musl readlinkat call-boundary behavior before retry. |
| `unlink07` | Blocked | `docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker4-fd-vector-vfs-rv-001-summary.txt`: RV musl+glibc FAIL, TFAIL=1 each. | Demote from first-wave; inspect exact assertion before LA. |
| `open06`, `creat04`, `mkdir04`, `rmdir03`, `unlink08` | Source-fix-before-scout | Prior worker report classifies these as permission/sticky/parent write-search gaps. Static audit confirms create/remove paths are thin wrappers around axfs operations without an explicit parent permission/sticky gate. | Implement narrow parent write+search and sticky-bit checks, then run RV targeted parser gate first. |
| `rename01`, `rename03`-`rename05`, `openat02`, `openat03`, `link02`, `unlink05`, `mkdir02` | Inventory / no clean proof | `stable400-attempt5-inventory-statx-report.md` lists them in metadata/path inventory only; no RV+LA x musl/glibc clean matrix found. | Treat as scout candidates after parent-permission repair; not promotion evidence. |
| `statx01`, `statx03`-`statx12` | Blocked / high setup risk | `stable400-attempt5-inventory-statx-report.md` and `stable400-promotion-gate-report.md`: statx tail has TBROK/TCONF; device/mkfs/exportfs/config parsing setup blockers. | Keep out of easy-first promotion unless statx setup/device lane is explicitly chosen. |
| `open07`, `creat06`, `mkdir03`, `rmdir02` | Reserve / medium risk | Prior guardrail report notes symlink-loop/errno/EROFS coverage may be incomplete; no fresh clean proof. | Scout only after permission/symlink sanity checks. |
| `open10`, `creat08`, `creat09` | High risk | Static audit: new file owner/group currently uses process fs gid on create; parent setgid inheritance is not modeled. | Skip easy-first unless implementing setgid inheritance deliberately. |
| `open11`, `open12`, `open14`, `creat07`, `mkdir09`, `unlink09` | Skip for first batch | Device/O_TMPFILE/largefile/exec-file/ioctl/stress requirements; no clean evidence. | Keep out of stable460 first-wave budget. |

## Static implementation audit

Observed VFS/path code loci:

- `examples/shell/src/uspace/metadata.rs`
  - `sys_faccessat` already checks parent search permission before access decision.
  - `sys_fchmodat` checks parent search and chmod ownership for ordinary paths.
  - `sys_statx` supports a limited statx mask and symlink nofollow, but existing blocked statx evidence is mostly setup/device/config related.
  - `sys_readlinkat` has synthetic `/proc/self/fd`, `/proc/*/exe`, and in-memory symlink support; LA-musl failure still needs runtime assertion diagnosis.
- `examples/shell/src/uspace/fd_table.rs`
  - `mkdirat`, `mknodat`, and `unlinkat` resolve paths then call axfs create/remove directly; they do not explicitly enforce parent write+search permission or sticky-bit behavior before mutation.
  - `sys_renameat2` accepts only flags=0 and then calls `axfs::api::rename`; it does not model parent permission/sticky checks.
  - `open_fd_entry` / `open_candidates` check target open permission for existing paths, but `O_CREAT` paths do not have an explicit parent write+search check before file creation.
  - New file creation records owner as process fs uid/gid; parent setgid inheritance is not modeled, so setgid-directory cases remain high risk.
- `examples/shell/src/uspace/credentials.rs`
  - `access_allowed` already implements uid/gid/other permission-bit checks used by the proposed parent-permission helper; no sticky-bit-specific helper exists today.

Minimal repair sketch (not implemented in this worker lane because fresh targeted runtime evidence is leader-owned and default QEMU concurrency is forbidden):

1. Add a small helper in `fd_table.rs` to compute the parent path of an already-normalized absolute path.
2. Add `check_parent_write_search(process, parent_path)` that uses recorded metadata and `access_allowed(..., ACCESS_W_OK | ACCESS_X_OK, fs_uid, fs_gid)` for non-root callers.
3. Add a sticky-bit helper for unlink/rename/rmdir once `FILE_MODE_STICKY = 0o1000` is named in `linux_abi.rs`; allow mutation if caller owns the parent dir, owns the victim, or is privileged.
4. Apply the helper narrowly to `O_CREAT` creation, `mkdirat`, `mknodat`, `symlinkat`, `unlinkat`, and both sides of `renameat2`.
5. Re-scout only the source-fix-before-scout set first: `open06,creat04,mkdir04,rmdir03,unlink08`.

## Verification evidence

Commands run in worker-2 worktree:

| Check | Command | Result |
| --- | --- | --- |
| Parser regression tests | `python3 -B scripts/test_ltp_summary.py` | PASS: `Ran 10 tests in 1.602s`, `OK` |
| Python syntax/type sanity | `python3 -m py_compile scripts/ltp_summary.py scripts/test_ltp_summary.py` | PASS |
| Diff whitespace/static check | `git diff --check` | PASS |
| Test discovery | `python3 -B -m unittest discover -s scripts -p 'test_*.py'` | PASS: `Ran 10 tests in 1.702s`, `OK` |
| Promotion parser smoke | `python3 -B scripts/ltp_summary.py --promotion-candidates --json output_rv.md output_la.md` | PASS: `candidate_count 62`, `blocked_count 1`, first blocked `read02` |
| Live stable list invariant | Python parser over `examples/shell/src/cmd.rs::LTP_STABLE_CASES` | PASS: `total=413 unique=413 duplicates=0` |
| Rust format check | `cargo fmt --all -- --check` | FAIL / environment-blocked: cargo workspace mismatch for `vendor/rust-fatfs` inside OMX worktree; no Rust files changed by this worker. |

Not run:

- No `./run-eval.sh` targeted or full QEMU run, per leader instruction not to run concurrent default QEMU.
- No `LTP_STABLE_CASES` promotion edit; leader owns final promotion decisions.

## Delegation evidence

- Subagent `019e68b4-af07-7192-be00-64947a2c6b58` (`Dewey`, model `gpt-5.4-mini`) completed the required read-only test probe. Integrated findings: added promotion-mode parser tests for four-way matrix completeness, missing combo blocking, TCONF blocking, prior failure-event blocking, and CLI multi-log behavior. Rejected exact stable-count unit assertion because it would become stale during intentional stable425/stable440/stable460 promotions.
- Subagent `019e68bc-bec4-72d3-90b0-6e42a52a9859` (`Pauli`, model `gpt-5.4-mini`) completed the read-only VFS/path probe. Integrated findings: confirmed `resolve_dirfd_path` is path normalization only, `check_open_permission` covers target permissions but not parent write/search for creation, and `mkdirat`/`mknodat`/`unlinkat`/`renameat2` lack explicit sticky-bit and parent-permission gates. Pauli also highlighted setgid inheritance and dispersed `readlinkat`/`statx` errno branches as reserve risks.

## Stop condition for this lane

This worker lane should hand off to the leader with no promotion claim. The actionable next step is a leader-approved narrow source repair for parent write/search and sticky permission semantics, followed by serialized RV targeted runs for `open06,creat04,mkdir04,rmdir03,unlink08` and parser summaries before any LA or stable-list promotion.
