# Worker 2 log-noise / VFS metadata review report

Date: 2026-05-25
Worker: worker-2
Task: task-2, report-only log-noise + VFS metadata lane
Scope: review leader-side `kernel/fs/axfs/src/fops.rs` / `kernel/fs/axfs/src/root.rs` no-warn errno patch; statically inspect adjacent metadata candidates (`access`, `chmod`/`fchmod`, `statx`, `readlinkat`, `rename`, `openat`, `link`, `unlink`).
Guardrails: no QEMU/evaluator run; no edit to `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; no edit to `.omx/ultragoal`.

## Executive outcome

Leader's no-warn patch preserves the intended errno semantics while removing avoidable `[AxErrorKind::*]` warning noise from expected negative VFS probes:

- `Directory::_open_dir_at()` now returns `Err(AxError::NotADirectory)` for non-directory open-dir targets, preserving Linux `ENOTDIR` mapping.
- `root::create_dir()` now returns `Err(AxError::AlreadyExists)` when the target already exists, preserving Linux `EEXIST` mapping.
- `root::remove_file()` now returns `Err(AxError::IsADirectory)` when unlinking/removing a directory through the file path, preserving Linux `EISDIR` mapping.

This is a logging/noise-only semantic-preserving cleanup: the `axerrno` macros log via `warn!`, while direct `Err(AxError::...)` returns the same `AxError` constants without emitting warning lines. The remote noise baseline shows this matters: `remote-output-noise-baseline.json` records about 4.5k `AxError_NotADirectory` and 380 `AxError_IsADirectory` warning lines per remote output, with only one `AxError_AlreadyExists` line.

## Leader patch review

Observed in leader root `/root/oskernel2026-orays` as uncommitted source diffs against this worker baseline:

| File | Reviewed line(s) | Before | After | Semantics verdict |
| --- | ---: | --- | --- | --- |
| `kernel/fs/axfs/src/fops.rs` | 297 | `return ax_err!(NotADirectory);` | `return Err(AxError::NotADirectory);` | Preserves `NotADirectory -> ENOTDIR`; removes macro warning. |
| `kernel/fs/axfs/src/root.rs` | 423 | `Ok(_) => ax_err!(AlreadyExists),` | `Ok(_) => Err(AxError::AlreadyExists),` | Preserves `AlreadyExists -> EEXIST`; removes macro warning. |
| `kernel/fs/axfs/src/root.rs` | 433 | `ax_err!(IsADirectory)` | `Err(AxError::IsADirectory)` | Preserves `IsADirectory -> EISDIR`; removes macro warning. |

Why this is safe:

- `axerrno-0.2.2` defines `AxError::{NotADirectory, AlreadyExists, IsADirectory}` constants and `AxResult<T> = Result<T, AxError>`.
- `ax_err!($err)` expands through `ax_err_type!($err)`, which constructs the same `AxError` and logs `warn!("[{:?}]", err)`. Replacing only expected-control-flow errors with direct `Err(AxError::...)` avoids warning noise without changing returned error values.
- Both touched files already import `AxError`, so the direct constants are in scope.

## Adjacent metadata/VFS candidates

| Area / cases | Current static evidence | Report-only recommendation |
| --- | --- | --- |
| `access02` / `access04` | `sys_faccessat()` validates mode/flags, supports `AT_EACCESS`, `AT_SYMLINK_NOFOLLOW`, `AT_EMPTY_PATH`, resolves through fd table, checks parent search permission, and uses real IDs unless `AT_EACCESS` is set. `access02` is already in `LTP_STABLE_CASES` and remote RV/LA glibc summaries show PASS. `access04` is not in the stable list. | No local code patch from this lane. `access04` remains mount/permission-sensitive; promote only after serialized targeted proof. |
| `chmod` / `fchmod` / `fchmodat` | Existing metadata layer has owner/root permission checks, setgid-clearing logic, `ENAMETOOLONG`, parent search permission, `AT_EMPTY_PATH`, and bad fd/path errno handling. Stable list currently includes `fchmod01`, `fchmod03`, `fchmod04`, `fchmodat01`, `fchmodat02`; `chmod05`/`chmod07` are absent. | No report-only patch. Candidate expansion should prioritize serialized `chmod05`/`chmod07` proof, not broader VFS mutation. |
| `statx` | `sys_statx()` validates null output buffer, supported flags, supported mask, `AT_EMPTY_PATH`, symlink no-follow stat, and writes `statx` from recorded stat. Stable list includes `statx02`; `statx03+` are absent. | `statx03+` likely need targeted evidence for exact mask/mount-field expectations before promotion. |
| `readlinkat` | `sys_readlinkat()` handles zero buffer as `EINVAL`, proc/self fd/exe links, recorded symlinks, non-symlink `EINVAL`, and backs missing path errors through `LinuxError::from(err)`. Stable list includes `readlink01` and `readlinkat01`; `readlinkat02`/`readlink03` are absent. | Good candidate for serialized targeted run; no safe report-only source change. |
| `rename` | `sys_renameat2()` resolves both dirfd-relative paths and delegates to `axfs::api::rename`; `root::rename()` still has an explicit `warn!("dst file already exist, now remove it")` when replacing an existing target. `rename01+` are absent from stable list. | The existing-target warn is separate from `ax_err!` noise. Do not suppress unless the leader confirms it is expected noisy control flow; errno semantics for directory replacement still need runtime proof. |
| `openat` | `sys_openat()` delegates to fd table open; candidate search preserves `ENOTDIR` as final error while continuing runtime-root fallback candidates, has `EISDIR` for writable directory opens, `O_NOFOLLOW`, `O_PATH`, `O_DIRECTORY`, `O_NOATIME`, and synthetic device/proc cases. Stable list includes `openat01`; `openat02+` are absent. | Candidate-rich but broad; promote only with serialized RV/LA x libc proof. |
| `link` / `unlink` | `linkat` implementation was not found in syscall dispatch, while `unlinkat` delegates through fd table and stable list includes `unlinkat01`; `link02` and `unlink05` are absent. | `link*` is broader than log-noise cleanup; keep out of this lane. `unlink` expansion needs targeted proof around dir/file errno and symlink bookkeeping. |

## Stable-list snapshot (read-only)

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` was read only: 375 total / 375 unique. Relevant present cases: `access02`, `fchmod01`, `fchmod03`, `fchmod04`, `fchmodat01`, `fchmodat02`, `statx02`, `readlink01`, `readlinkat01`, `openat01`, `unlinkat01`. Relevant absent candidates include `access04`, `chmod05`, `chmod07`, `statx03+`, `readlinkat02`, `rename01+`, `openat02+`, `link02`, `unlink05`.

## Verification

- `omx team api send-message ... ACK` -> PASS, message `bab13920-5c05-4567-a03d-b13b4cea3cdd` delivered to `leader-fixed`.
- `omx team api claim-task --input '{"team_name":"ltp-stable375-to-stab-eae749f6","task_id":"2","worker":"worker-2"}' --json` -> PASS, claim token acquired.
- `git -C /root/oskernel2026-orays diff -- kernel/fs/axfs/src/fops.rs kernel/fs/axfs/src/root.rs` -> PASS, inspected exactly the three leader no-warn changes above.
- `nl -ba /root/oskernel2026-orays/kernel/fs/axfs/src/fops.rs | sed -n '286,302p'` and `nl -ba /root/oskernel2026-orays/kernel/fs/axfs/src/root.rs | sed -n '418,436p'` -> PASS, verified final direct `Err(AxError::...)` call sites and line numbers.
- `nl -ba ~/.cargo/registry/src/.../axerrno-0.2.2/src/lib.rs | sed -n '468,538p'` -> PASS, verified `AxResult` alias plus `ax_err_type!` warning emission.
- Static metadata inspection commands over `examples/shell/src/uspace/{metadata.rs,fd_table.rs,syscall_dispatch.rs}` -> PASS, mapped candidate surfaces without edits.
- Stable-list read-only parser over `examples/shell/src/cmd.rs` -> PASS, `375 total / 375 unique`; no write to `LTP_STABLE_CASES`.
- `git diff --name-only -- examples/shell/src/cmd.rs .omx/ultragoal` -> PASS, no scoped forbidden mutations.
- QEMU/evaluator -> intentionally not run per leader instruction; no concurrent promotion evidence claimed.

## Subagent skip reason/evidence

Subagent skip reason: Task 2 was explicitly narrowed by the leader/user to a report-only static review lane with no QEMU and no promotion edits; the required sidecar/test probe was skipped deliberately because the needed coverage evidence was obtainable by direct read-only source/diff/doc inspection, and spawning a child would add coordination overhead without improving this non-mutating review.

## Recommended leader follow-up

1. Apply/keep the three direct `Err(AxError::...)` no-warn substitutions; they preserve `ENOTDIR`/`EEXIST`/`EISDIR` and should reduce remote output noise.
2. Treat `root::rename()`'s existing-target `warn!` separately; it is not an `ax_err!` macro artifact and may represent a real behavior decision.
3. If expanding the stable list, use serialized targeted evidence for `chmod05 chmod07 statx03 readlinkat02 rename01 openat02 unlink05` before promotion; keep `link*` as a broader syscall-support lane.
