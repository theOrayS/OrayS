# Worker 2 task 8 VFS parent-permission repair

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Task: 8 — `Worker 2 narrow VFS parent-permission repair`
Scope: VFS create/remove parent permission and overlong-path errno only. No `.omx/ultragoal` edit, no `examples/shell/src/cmd.rs::LTP_STABLE_CASES` edit, and no default QEMU/evaluator run.

## Result summary

Implemented a narrow source repair for the task-8 failure cluster:

- `creat04` / `open(..., O_CREAT)`: new-file creation now checks parent directory write+search permission before creating the file.
- `mkdir04`: `mkdirat` now rejects creation under unwritable/unsearchable parent directories with `EACCES` before mutating the filesystem.
- `rmdir03` / `unlink08`: `unlinkat` now checks parent write+search permission and sticky-directory ownership rules before file/dir removal.
- `unlink07`: `unlinkat`, `mkdirat`, and `mknodat` now reject Linux-overlong paths/components with `ENAMETOOLONG` before path resolution can collapse the error to `ENOENT`.

No promotion claim is made here; this is source repair plus static/build evidence for leader-owned targeted RV/LA gating.

## LTP source anchors inspected

Local copies fetched for inspection only under `/tmp/ltp-task8-sources`:

| Case | Relevant upstream expectation | Repair mapping |
| --- | --- | --- |
| `creat04` | As `nobody`, `creat("testdir/file")` and an existing file under restrictive `testdir` must fail `EACCES`. | `open_candidates` now applies parent write+search checks to actual new-file `O_CREAT` paths; existing files still use target open permission. |
| `mkdir04` | Non-owner user cannot create a subdirectory in another user's restrictive directory; expected `EACCES`. | `mkdirat` checks parent write+search before `directory_create_dir`. |
| `rmdir03` | Sticky parent can fail `EPERM`/`EACCES`; no-search parent must fail `EACCES`. | `unlinkat(..., AT_REMOVEDIR)` checks parent write+search and sticky ownership before removal. |
| `unlink08` | Unwritable parent and unsearchable path components must fail `EACCES`; unlinking a directory without `AT_REMOVEDIR` remains directory-error behavior. | `unlinkat` gates parent write+search before removal, while leaving lower remove-dir/file errno handling intact. |
| `unlink07` | Overlong pathname must fail `ENAMETOOLONG`. | `path_exceeds_linux_limits` is shared by open/create/remove entrypoints; `unlinkat` checks it before resolving the path. |

## Files changed

- `examples/shell/src/uspace/fd_table.rs`
  - Added shared Linux path/name length check and absolute-parent permission helpers.
  - Added parent write+search checks to `mkdirat`, `mknodat`, `unlinkat`, and new-file `O_CREAT` handling.
  - Added sticky parent ownership check for unlink/rmdir before removal.
  - Added parent ancestry search check for existing open permission checks.
- `examples/shell/src/uspace/linux_abi.rs`
  - Added named `FILE_MODE_STICKY = 0o1000` constant for sticky-directory checks.
- `docs/ltp-score-improvement-2026-05-27-phase-a/worker2-task8-vfs-parent-permission-repair.md`
  - This report.

## Code touchpoints

| File/lines | Change | Cases targeted |
| --- | --- | --- |
| `examples/shell/src/uspace/fd_table.rs:1436-1447` | `mkdirat` validates `ENAMETOOLONG`, preserves `EEXIST`, then checks parent write+search before `directory_create_dir`. | `mkdir04`, overlong mkdir guardrail. |
| `examples/shell/src/uspace/fd_table.rs:1460-1480` | `mknodat` uses the same path-length and parent mutation gate before creating files/FIFOs. | FIFO/file create regression guardrail. |
| `examples/shell/src/uspace/fd_table.rs:1507-1542` | `unlinkat` validates overlong paths, checks parent write+search, checks sticky ownership, then removes symlink/file/dir. | `unlink07`, `unlink08`, `rmdir03`. |
| `examples/shell/src/uspace/fd_table.rs:2139-2231` | Added `path_exceeds_linux_limits`, parent path/stat/search helpers, and sticky permission helper using recorded metadata plus `access_allowed`. | Shared create/remove permission model. |
| `examples/shell/src/uspace/fd_table.rs:2247-2274` | Existing open checks now also require searchable parent ancestry for non-root callers. | `creat04` existing-target and open permission consistency. |
| `examples/shell/src/uspace/fd_table.rs:2469-2485` | New-file `O_CREAT` paths check parent write+search instead of skipping permission checks when the target does not exist. | `creat04`, `open06`-style parent permission cases. |
| `examples/shell/src/uspace/linux_abi.rs:69` | Added `FILE_MODE_STICKY`. | `rmdir03` sticky parent case. |

## Regression risks / guardrails

| Family | Risk | Mitigation in this patch |
| --- | --- | --- |
| `access*` / `stat*` / `chmod*` | Parent-search semantics are shared conceptually with metadata code; a stricter open/create helper could expose bad recorded modes if setup metadata is wrong. | The new helper uses existing `access_allowed` and recorded path metadata instead of inventing a separate policy. No metadata syscall code was changed. |
| `open*` / `creat*` | Existing opens under non-searchable ancestors may now fail `EACCES` where axfs previously allowed host access; this is Linux-compatible but can reveal test setup assumptions. | `O_PATH` behavior was left untouched; `O_CREAT` only checks parent mutation when the target is actually missing. Existing targets still use target permission checks. |
| `unlink*` / `rmdir*` | Sticky parent checks can change errno from a lower filesystem error to `EPERM`; no-search parent can now return `EACCES` before remove. | This is the requested Linux-compatible ordering for `rmdir03`/`unlink08`; nonexistent non-dir unlink still allows lower `ENOENT` after parent permission succeeds. |
| `read*` / `write*` | File entries created after stricter parent checks keep existing read/write paths unchanged. | No read/write implementation was edited; only pre-open/create/remove permission gates changed. |
| FIFO / special-mode creation | `mknodat` now checks parent write+search before creating FIFO/special files, which can block previously over-permissive setup. | Uses the same mutation gate as `mkdirat`/`O_CREAT` to avoid case-specific behavior. |
| Root behavior | Root bypasses parent mode bits in the helper, matching the repo's existing root shortcut in search-permission helpers. | Kept narrow to avoid broad privilege model changes in this lane. |

## Verification

| Check | Command | Result |
| --- | --- | --- |
| Rust format | `cargo fmt -p arceos-shell -- --check` | PASS (`status=0`). |
| Diff whitespace/static check | `git diff --check` | PASS (`status=0`). |
| Parser regression tests | `python3 -B scripts/test_ltp_summary.py` | PASS: `Ran 10 tests in 0.523s`, `OK`. |
| Targeted Rust build, first attempt | `cargo check -p arceos-shell --target riscv64gc-unknown-none-elf --features uspace` | FAIL before task code due missing platform crate: `can't find crate for axplat_riscv64_qemu_virt`. |
| Targeted Rust build with default platform crates | `cargo check -p arceos-shell --target riscv64gc-unknown-none-elf --features 'uspace axhal/defplat'` | FAIL after reaching `arceos-shell` on unrelated existing `WaitQueue::wait_timeout_until` errors in `futex.rs` and `process_lifecycle.rs`; no `fd_table.rs`/`linux_abi.rs` compile error remained. |

Not run:

- No default QEMU, `./run-eval.sh`, or full evaluator run, per task/leader instruction.
- No stable-list promotion edit; leader owns final `LTP_STABLE_CASES` changes after targeted RV+LA evidence.

## Delegation evidence

Subagent spawned: 1, model `gpt-5.4-mini`.

- `019e68f4-591b-77d0-b7eb-46c8c0d60874` (`Sartre`) — read-only sidecar verification. Integrated finding: the sidecar reviewed only the latest split auto-checkpoint (`1dd293cd`) and correctly flagged that checkpoint alone looked like a compile-helper refactor, not the full task repair. To avoid that misleading boundary, worker-2 squashed the task-8 auto-checkpoints plus this report into one formal Lore commit before completion. The cumulative task-8 diff still contains the parent write/search, sticky, and `ENAMETOOLONG` changes recorded above.

Serial repo-search/read commands before spawn: the task was already partially implemented before the updated task-8 delegation stanza was observed; once the updated task file was re-read, one sidecar verifier was spawned for read-only patch/risk review and its boundary warning was integrated into the final commit hygiene.

## Stop condition

Worker-2 source repair and static/build evidence are complete for task 8. The next safe step is leader-owned targeted RV batch for `creat04,mkdir04,rmdir03,unlink08,unlink07` with `scripts/ltp_summary.py` summaries before any LA spend or stable-list promotion.
