# Worker 2 VFS / permission / metadata lane report

Date: 2026-05-25
Worker: worker-2
Task: task-2, permissions/VFS/metadata lane
Scope: `access02 access04 chmod05 chmod06 chmod07 fchmod02 fchmod05 fchmod06 fchmodat02 statx01 readlinkat02 rename01 rename03 rename04 openat02` plus stretch `openat03 rename05 statx03`.

## Executive outcome

Implemented two narrow, source-level fixes that are directly aligned with the lane's high-risk cases and do not touch `.omx/ultragoal` or `examples/shell/src/cmd.rs::LTP_STABLE_CASES`:

1. `statx(2)` now receives and validates the syscall `mask` argument instead of silently ignoring `arg3`; this targets `statx03`'s invalid-mask `EINVAL` subtest and removes one dispatch-shape risk for `statx01`.
2. `fchmod(2)` / `fchmodat(2)` now reject non-owner, non-root chmod attempts with `EPERM`, return `ENAMETOOLONG` for oversized paths, and enforce parent-directory search permission before path chmod; this targets `chmod06`, `fchmod06`, and `fchmodat02` error-order gaps while preserving the existing `chmod05`/`fchmod05` setgid-clearing behavior.

No QEMU/evaluator run was started per leader instruction. The changes are build-checked only; promotion still requires leader-serialized RV+LA x glibc+musl evidence through `scripts/ltp_summary.py`.

## Files changed

- `examples/shell/src/uspace/syscall_dispatch.rs:173-180` — passes `tf.arg3()` as the statx mask argument and keeps `tf.arg4()` as the output buffer.
- `examples/shell/src/uspace/metadata.rs:342-486` — adds chmod ownership checks, fchmodat long-path rejection, and parent search-permission enforcement for path chmod.
- `examples/shell/src/uspace/metadata.rs:830-858` — adds statx mask validation against the known `STATX_*` request bits and returns `EINVAL` for unknown/reserved masks.

## Case-by-case static assessment

| Case | Static status after this lane | Notes / remaining risk |
| --- | --- | --- |
| `chmod05` | likely improved / guarded | Existing `chmod_effective_mode()` already clears setgid for non-root directory chmod when caller lacks target group; new owner check still permits owner `nobody` in the upstream test shape. |
| `chmod06` | partially repaired | New `EPERM`, `EACCES`, and `ENAMETOOLONG` paths match upstream error subtests. `EROFS` for read-only mounts remains unproven because current mount emulation does not model read-only state explicitly. |
| `chmod07` | should remain safe | Root chmod with sticky bit still passes the new owner check. |
| `fchmod02` | should remain safe | Root fchmod with sticky bit still passes the new owner check. |
| `fchmod05` | likely improved / guarded | Existing setgid clearing is preserved for non-root directory owner; new owner check still permits the owner path. |
| `fchmod06` | partially repaired | New `EPERM` covers the non-owner fd case; `EBADF` already came from fd lookup. `EROFS` subtest remains a likely blocker without read-only mount metadata. |
| `fchmodat02` | improved | Existing bad-fd/invalid-flag/empty-path handling remains; new path-length check targets `ENAMETOOLONG`. File-dirfd relative path still routes through `resolve_path()` to `ENOTDIR`. |
| `statx03` | improved | Invalid flags were already rejected; invalid mask now returns `EINVAL` instead of being ignored. Bad fd, bad address, empty path, ENOTDIR, and long path still depend on existing path/user-memory handling. |
| `statx01` | dispatch risk reduced | Mask=0 remains valid. Remaining risks: `stx_mnt_id` support, `mknod`/device rdev setup, and exact mountinfo behavior need serialized runtime evidence. |
| `readlinkat02` | no code change | Static code already has bufsiz 0 -> `EINVAL`, non-symlink -> `EINVAL`, bad fd/ENOTDIR through `resolve_dirfd_path`, and missing path -> backing errno. Needs targeted runtime evidence. |
| `rename01`/`rename03` | no code change | Basic rename to absent/existing destination is delegated to `axfs::api::rename`; static code should handle normal cases but inode-preservation expectations need runtime proof. |
| `rename04`/`rename05` | no code change | Expected non-empty-dir and file-over-dir errno depends on `axfs::api::rename` errno mapping; no safe local patch without runtime signal. |
| `openat02` | no code change | Existing code has `O_APPEND`, `O_CLOEXEC`, `O_LARGEFILE`, `O_NOATIME`, `O_NOFOLLOW`, and `O_TRUNC` surfaces; runtime proof still required. |
| `openat03` | stretch / likely blocked | Upstream case needs `O_TMPFILE` and `linkat`; no safe patch in this lane because `linkat` support is broader than VFS metadata cleanup. |
| `access02` | no code change | `sys_faccessat()` already uses real IDs by default, effective IDs under `AT_EACCESS`, parent search permission, and symlink resolution via stat path. Runtime evidence needed. |
| `access04` | no code change / likely mount-sensitive | Invalid mode, empty path, long path, ENOTDIR, and symlink loop are represented; `EROFS` remains likely blocked by read-only mount semantics, matching phase-A notes. |

## Upstream LTP source cross-check

The local repo does not carry the LTP C sources for these cases, so I used read-only upstream LTP sources under `/tmp/ltp-src-worker2` to avoid guessing test semantics. Important expectations checked:

- `chmod06.c`: expects `EPERM` for non-owner chmod, `EACCES` for denied path-prefix search, `ENAMETOOLONG` for oversized path, plus an unresolved `EROFS` read-only-mount case.
- `fchmod06.c`: expects `EPERM`, `EBADF`, and `EROFS`; this lane only addresses `EPERM` while preserving `EBADF`.
- `fchmodat02.c`: expects `ENOTDIR`, `EBADF`, `EFAULT`, `ENAMETOOLONG`, `ENOENT`, and `EINVAL`.
- `statx03.c`: expects `EINVAL` for invalid flags and invalid mask; the invalid-mask path was missing before this lane.

## Verification performed

- `cargo fmt --check --package arceos-shell` → PASS after formatting.
- `cargo check -p arceos-shell` → PASS (`Finished dev profile ... target(s) in 1.46s`).
- `git diff --name-only -- examples/shell/src/cmd.rs` → 0 files; `LTP_STABLE_CASES` was not edited.
- `git status --short .omx/ultragoal` → 0 entries; `.omx/ultragoal` was not mutated.
- No QEMU/evaluator command was run.

## Subagent / parallel probe note

I attempted the required sidecar coverage probe, but the spawned native subagent errored with a context-window failure before returning usable findings. I closed the agent and completed the coverage/source mapping locally to avoid blocking the claimed lane.

## Recommended leader follow-up

1. Serialize a targeted non-promotion run for `chmod05 chmod06 chmod07 fchmod02 fchmod05 fchmod06 fchmodat02 statx01 statx03` and parse with `scripts/ltp_summary.py`.
2. If chmod/fchmod still fail only on `EROFS`, assign a separate read-only mount semantics repair; do not broaden this patch into fake EROFS without real mount state.
3. Keep `access04` and `fchmod06` out of promotion until read-only mount behavior is explicit.
4. Treat `openat03` as stretch only; it needs `O_TMPFILE`/`linkat` work beyond this lane's safe patch scope.
