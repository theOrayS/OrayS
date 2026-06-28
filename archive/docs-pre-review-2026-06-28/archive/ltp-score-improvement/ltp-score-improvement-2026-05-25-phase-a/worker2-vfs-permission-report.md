# Worker 2 VFS / permission static report (worker-1 reassignment)

Date: 2026-05-25
Worker: worker-1
Task: task-2, permissions/VFS/errno/metadata lane
Scope actually executed: static/root-cause report only, using existing leader/worker logs and source inspection. No QEMU/eval was started. No `.omx/ultragoal`, `LTP_STABLE_CASES`, or shared source file was edited.

## Executive outcome

`access02`, `access04`, `chmod05`, and `statx01` should stay out of the promotion batch until a serialized repair/test loop is assigned. They cluster around Linux permission edge semantics rather than simple case-list promotion:

- `access02` / `access04`: real/effective UID/GID, parent-directory execute/search permission, symlink/empty-path combinations, and setup/errno drift.
- `chmod05`: special-mode-bit clearing and `fchmodat2`/`AT_EMPTY_PATH`/`O_PATH` consistency.
- `statx01`: current evidence says `ENOSYS`/`TBROK` on RV, while this branch has a `statx` dispatch and `statx02` is stable-clean, so this is likely a specific ABI/argument/field/flag path or stale-vs-current evidence mismatch requiring targeted rerun.

I found three narrow repair surfaces worth proposing to the leader before editing shared source:

1. `examples/shell/src/uspace/metadata.rs::{chmod_effective_mode, sys_fchmodat}` — special-bit and `O_PATH`/`AT_EMPTY_PATH` behavior; risk: can regress stable `chmod01/03`, `fchmod01/03/04`, `fchmodat01`.
2. `examples/shell/src/uspace/metadata.rs::{sys_statx, stat_to_statx}` plus `examples/shell/src/uspace/syscall_dispatch.rs` statx call shape — mask/field/flag completeness; risk: can affect already clean `statx02`.
3. `examples/shell/src/uspace/fd_table.rs::{open_candidates, check_open_permission}` and `metadata.rs::sys_faccessat` — parent search and permission/errno ordering consistency; risk: can affect already clean `open01/02/03/04/08/09/13` and `faccessat*` cases.

## Existing evidence used

| Case / cluster | Current evidence | Static implication |
| --- | --- | --- |
| `access02` | `promotion-candidates-current.txt` has RV glibc+musl `TFAIL=4`; `user-priority-ae-rv-summary.txt` repeats RV fail; older phase-D extract labels it “execute-file setup/ENOENT semantics”. | Not a promotion candidate. Treat as permission + setup errno path, especially parent search and real/effective ID selection. |
| `access04` | `promotion-candidates-current.txt` has RV glibc+musl `TBROK=1`; older phase-D extract labels it “tmpfs mount EINVAL in harness”. | Likely harness/setup-sensitive; repair should separate actual permission bug from tmpfs/mount setup failure before code edits. |
| `chmod05` | `promotion-candidates-current.txt` has RV glibc `TFAIL=1`, RV musl `TBROK=1`; later `target-priority-repair-rv-summary.txt` shows RV glibc PASS but RV musl still TBROK. | Glibc/musl divergence suggests a narrow special-bit or wrapper/setup edge, not broad chmod absence. |
| `statx01` | Current docs show RV glibc+musl `TBROK=1` with `ENOSYS=1`; stable final docs show `statx02` PASS on RV+LA/glibc+musl. | Dispatch exists in this branch, so `statx01` needs a fresh serialized targeted rerun before treating ENOSYS as current branch truth. |
| Adjacent `statfs02`/`fstatfs02` | Post-285 scout shows glibc PASS, musl TFAIL for both. | Statfs core exists; musl-specific struct/errno expectations remain risky. |
| Adjacent stable guards | Stable final docs show PASS for `chmod01/03`, `fchmod01/03/04`, `fchmodat01`, `open01/02/03/04/08/09`, `truncate02/03`, `statvfs02`, `statx02`, `readlink01`, `symlink02/04`. | Any repair must rerun these guards before promotion. |

## Source surfaces inspected

### Access / permission path

- `examples/shell/src/uspace/syscall_dispatch.rs:153-156` routes `faccessat` and `faccessat2` into `metadata.rs::sys_faccessat`.
- `examples/shell/src/uspace/metadata.rs:174-239` implements `sys_faccessat` with:
  - mode validation against `ACCESS_MODE_MASK`,
  - supported flags `AT_EACCESS | AT_SYMLINK_NOFOLLOW | AT_EMPTY_PATH`,
  - real IDs by default and effective IDs under `AT_EACCESS`,
  - explicit `parent_dirs_searchable(...)` before `access_allowed(...)`.
- `examples/shell/src/uspace/credentials.rs:409-435` implements `access_allowed`: root has the Linux-like execute special case, otherwise owner/group/other mode bits decide.
- `examples/shell/src/uspace/fd_table.rs:1281-1306` implements parent directory search using `ACCESS_X_OK` on each ancestor.
- `examples/shell/src/uspace/fd_table.rs:1680-1713` implements open permission checks, but returns early for `O_PATH` and checks only the target object; it does not mirror `sys_faccessat`'s parent-search check inside `check_open_permission`.

Static read: `sys_faccessat` is more Linux-like than `open_candidates`. If failures are permission/errno-order specific, the likely inconsistency is not simply `access_allowed`; it is interaction among parent search, path resolution, symlink handling, and real/effective ID choice. `access04` also has historical tmpfs/mount `EINVAL` evidence, so it should be debugged separately from `access02`.

### chmod / chown / special bits

- `examples/shell/src/uspace/syscall_dispatch.rs:136-142` routes `fchmod`, `fchmodat`, and `fchmodat2`.
- `examples/shell/src/uspace/metadata.rs:342-452` implements `sys_fchmod`, `sys_fchmodat`, and `chmod_effective_mode`.
- `sys_fchmod` rejects `FdEntry::Path(_)` with `EBADF`; `sys_fchmodat` supports `AT_SYMLINK_NOFOLLOW | AT_EMPTY_PATH`, but also rejects `FdEntry::Path(_)` in `/proc/self/fd/N` and empty-path fd paths.
- `chmod_effective_mode` only clears `FILE_MODE_SET_GID` when the target is a directory, caller is non-root, requested setgid is present, and caller lacks the target group.
- `examples/shell/src/uspace/metadata.rs:540-576` routes `fchownat` empty-path handling into `apply_chown_metadata`; `credentials.rs` also has ID/fsuid/fsgid state used by ownership checks.

Static read: `chmod05` is the highest-value single repair candidate in this lane, but it is not safe to edit blindly because the live stable set already protects many nearby chmod/fchmod cases. The first proposed edit should be a focused special-bit semantics patch in `chmod_effective_mode`, only after confirming the exact `chmod05` subtest expectation from a serialized targeted run or LTP source lookup.

### statx / stat / statfs / statvfs

- `examples/shell/src/uspace/syscall_dispatch.rs:169-173` routes `newfstatat` and `statx`; statx currently calls `sys_statx(&process, arg0, arg1, arg2, arg4)`, skipping the syscall mask argument (`arg3`).
- `examples/shell/src/uspace/metadata.rs:774-861` implements `stat_to_statx` and `sys_statx`.
- `stat_to_statx` fills `STATX_BASIC_STATS`, block size, link count, uid/gid, mode, inode, size, blocks, dev/rdev, but leaves timestamps and richer attributes zeroed.
- `sys_statx` supports `AT_SYMLINK_NOFOLLOW | AT_EMPTY_PATH | AT_NO_AUTOMOUNT | AT_STATX_SYNC_TYPE`, null pathname only with `AT_EMPTY_PATH`, and `stat_empty_path(...)` for fd/cwd handling.
- `examples/shell/src/uspace/metadata.rs:966-997` implements `statfs`/`fstatfs`; `generic_statfs` maps magic values by path prefix and fills allocator-derived block counts.
- No independent shell-uspace `statvfs` syscall surface was found in this pass; `statvfs02` being stable-clean likely depends on libc translation or existing wrappers, not a dedicated uspace handler.

Static read: current `statx01` ENOSYS evidence conflicts with source-level presence of `__NR_statx` dispatch and stable-clean `statx02`. Before patching, leader should run a serialized `statx01` smoke/targeted batch to verify whether ENOSYS is still current. If current, inspect arch syscall number mapping first; if not, likely next fix is mask/field completeness in `sys_statx`/`stat_to_statx`.

### Adjacent namespace/open/truncate/link/rename tails

- `examples/shell/src/uspace/fd_table.rs:126-1905` owns `openat`, `ftruncate`, `renameat2`, path resolution, recorded path metadata, `O_PATH`, and synthetic/proc/dev fallback behavior.
- `sys_renameat2` accepts only `flags == 0`; nonzero flags return `EINVAL`.
- No `__NR_linkat`/`sys_linkat` dispatch was found in `syscall_dispatch.rs`, which is a clear blocker for link/linkat families that still report ENOSYS/TBROK/TFAIL in earlier docs.
- `sys_truncate`/`sys_ftruncate` already gate `RLIMIT_FSIZE`, `MAX_IN_MEMORY_FILE_SIZE`, directory rejection, write permission, and backing `File::truncate`.

Static read: link/rename/truncate/open tails are useful regression-protection batches, but they are broader than task-2's four named cases. Do not combine them with `statx01` repair unless leader assigns a separate tranche.

## Proposed leader-gated repair order

1. **Fresh serialized targeted evidence first**: run only `access02,access04,chmod05,statx01` on RV glibc+musl through leader-controlled QEMU, parse with `scripts/ltp_summary.py`, and keep `TFAIL/TBROK/ENOSYS/TCONF` visible.
2. **If `statx01` still reports ENOSYS**: inspect syscall number/arch routing before changing statx fields. Candidate files: `examples/shell/src/uspace/syscall_dispatch.rs`, `examples/shell/src/uspace/linux_abi.rs`. Risk: touches syscall dispatch and could affect `statx02`.
3. **If `statx01` reaches handler but fails fields/errno**: add mask argument handling and fuller timestamp/attribute population in `metadata.rs::{sys_statx, stat_to_statx}`. Risk: field semantics for `statx02` and hidden statx tests.
4. **For `chmod05`**: compare exact subtest against `chmod_effective_mode` and `apply_chown_metadata` special-bit behavior. Candidate file/function: `examples/shell/src/uspace/metadata.rs::chmod_effective_mode`. Risk: stable chmod/fchmod/fchmodat regressions.
5. **For `access02`**: compare failing errno/subtest against `sys_faccessat` and parent directory search. Candidate functions: `metadata.rs::sys_faccessat`, `fd_table.rs::parent_dirs_searchable`, `credentials.rs::access_allowed`. Risk: `faccessat*`, `access01/03`, and open permission behavior.
6. **For `access04`**: first split harness tmpfs/mount setup (`EINVAL`) from access permission logic. If still a kernel/VFS issue, inspect mount/tmpfs and path setup before touching permission helpers.
7. **Only after a narrow fix passes target batch**: rerun guard batch `chmod01,chmod03,fchmod01,fchmod03,fchmod04,fchmodat01,open01,open02,open03,open04,open08,open09,statx02,statvfs02,truncate02,truncate03` under the leader's serialized eval path.

## Regression / coverage gaps

- There are no shell-local Rust unit tests for these Linux permission cases under `examples/shell/src/{cmd,uspace/*}` in the inspected pass; evidence is currently integration/evaluator logs plus static source review.
- `chown*` cases are not represented in the current stable set or phase-A priority docs found in this pass, so chown special-bit behavior has little direct regression coverage.
- `chmod05` and `statx01` are not in live stable/basic lists; `access02/access04` are present in `LTP_SYSCALLS_BASIC_PLUS_CASES`, not in `LTP_STABLE_CASES`.
- `linkat` appears unimplemented at syscall-dispatch level; link/linkat cases should be a separate repair story rather than hidden inside access/chmod/statx work.

## Subagent integration

- Erdos (`019e5ad8-1c84-7b61-995f-bcd36af93432`) mapped existing case coverage and confirmed that nearby stable guards are clean while the target four remain outside stable promotion.
- Huygens (`019e5ad8-22e0-7d51-ae9e-d55643891a38`) mapped the VFS/metadata code surfaces, highlighted `statvfs` surface ambiguity, missing `linkat` dispatch, `renameat2` flag limits, `O_PATH`/`AT_EMPTY_PATH` divergence risk, and special-bit semantics risk.

Subagent spawn evidence: 2, Erdos/019e5ad8-1c84-7b61-995f-bcd36af93432 coverage probe and Huygens/019e5ad8-22e0-7d51-ae9e-d55643891a38 code-surface probe; findings integrated into coverage gaps, source-surface mapping, and leader-gated repair order.

## Verification performed for this report

- Static source inspection with `rg` and `sed` over `examples/shell/src/uspace/{metadata.rs,fd_table.rs,credentials.rs,syscall_dispatch.rs,linux_abi.rs}`, `examples/shell/src/cmd.rs`, and existing docs under `docs/ltp-score-improvement-2026-05-24-phase-a/` and `docs/ltp-score-improvement-2026-05-25-phase-a/`.
- No QEMU/eval was run, per leader instruction.
- No Rust/source file was edited, so Rust formatting/build checks are regression confidence only, not changed-code proof.
