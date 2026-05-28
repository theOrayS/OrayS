# Worker 5 mmap/munmap/mprotect guardrail report

Date: 2026-05-25
Team: `ltp-stable350-to-stab-7c9de325`
Worker: `worker-5`
Task: `task-5` / mmap04, mmap05, mmap06, munmap01; stretch mprotect01, mprotect02

## A. Scope and guardrails

- ACKed leader mailbox and claimed `task-5` before edits.
- Did not mutate `.omx/ultragoal`.
- Did not final-edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Did not start QEMU; all evidence below is static/source/build evidence only.
- Worker evidence remains discovery-only. No case in this lane is promoted by this report.

## B. Prior fresh failure evidence used as baseline

Phase-a targeted RV summaries showed the lane was not clean:

| Case | Prior RV status summary | Primary blocker class |
| --- | --- | --- |
| `mmap04` | FAIL/TBROK on glibc+musl | `/proc/self/maps` did not expose the newly mapped VMA permission line. |
| `mmap05` | wrapper FAIL 139 on glibc+musl | page-fault-to-application-SIGSEGV handler path is missing; process exits instead. |
| `mmap06` | FAIL with 7 internal TFAIL across glibc+musl | mmap errno validation: unreadable file fd surfaced `EBADF` instead of `EACCES`, and missing map type was not rejected early. |
| `munmap01` | wrapper FAIL 139 on glibc+musl | post-unmap child fault exits as signal status rather than delivering/catching test-level SIGSEGV semantics cleanly. |
| `mprotect01` | glibc wrapper FAIL 139; musl TFAIL=3 | mprotect alignment/permission errno gaps; file-backed max-permission tracking still missing. |
| `mprotect02` | FAIL/TBROK on glibc+musl | child SIGSEGV handler semantics and writable-after-mprotect verification are not yet complete. |

## C. Source audit

Upstream LTP test intent checked against current `linux-test-project/ltp` sources:

- `mmap04`: maps a guard pair, remaps the second page with `MAP_FIXED`, then scans `/proc/self/maps` for the remapped start address and exact permission string.
- `mmap05`: maps a file `PROT_NONE | MAP_SHARED`, installs a SIGSEGV handler, and expects a read from the mapping to invoke that handler.
- `mmap06`: expects `EACCES` for file mappings from an `O_WRONLY` fd, `EINVAL` for zero length, and `EINVAL` when no `MAP_PRIVATE`/`MAP_SHARED`/`MAP_SHARED_VALIDATE` type is present.
- `munmap01`: maps three pages, unmaps all or part of the region, forks, and expects the child access to the unmapped page to produce SIGSEGV.
- `mprotect01`: expects unaligned addresses to fail with `EINVAL`; it also expects `EACCES` when upgrading a read-only shared file mapping to writable.
- `mprotect02`: expects a write to a read-only mapping to be caught as SIGSEGV, then expects no SIGSEGV after `mprotect(..., PROT_WRITE)`.

Current repo code audit:

- `examples/shell/src/uspace/memory_map.rs` mapped file contents by reading through the generic fd read path, so an existing but write-only fd returned `EBADF`; Linux `mmap()` should report `EACCES` for this case.
- `sys_mmap()` accepted `MAP_FILE`/zero map type instead of rejecting missing map type with `EINVAL`.
- `sys_mprotect()` rounded unaligned addresses down before calling `protect()`, which could change protections on the wrong page instead of returning `EINVAL`.
- `/proc/self/maps` currently emits only text/heap/stack synthetic lines, not per-mmap VMA lines, so `mmap04` cannot be honestly marked clean yet.
- The page-fault path currently requests SIGSEGV group exit for unhandled user faults; it does not yet deliver a catchable application SIGSEGV frame for `mmap05`, `munmap01`, or `mprotect02`.

## D. Patch applied

Narrow, semantics-aligned patch only:

1. `examples/shell/src/uspace/fd_table.rs`
   - Added `FdTable::mmap_read_file_at_into_fd()`.
   - Keeps invalid/non-file fd behavior aligned with the existing path, but returns `EACCES` for an existing regular file fd that is not readable.

2. `examples/shell/src/uspace/memory_map.rs`
   - `sys_mmap()` now validates `flags & MAP_TYPE` and rejects missing/invalid map type with `EINVAL`.
   - File-backed mmap population now uses the mmap-specific fd read helper, preserving `mmap06`'s expected `EACCES` distinction.
   - `sys_mprotect()` now rejects unaligned addresses with `EINVAL` before touching page protections.

This is not a fake pass: it does not alter LTP case lists, wrapper markers, timeouts, or summary parsing. It addresses only real Linux syscall errno/argument semantics.

## E. Remaining blockers / not promoted

Do not promote these cases yet from worker-5 evidence:

- `mmap04`: needs per-mmap VMA accounting and `/proc/self/maps` rendering of anonymous/file-backed ranges with `rwx` and private/shared `p/s` permission suffixes. Current synthetic maps has no mmap records.
- `mmap05`: needs catchable user SIGSEGV delivery for protection faults, not only process/group termination status.
- `munmap01`: needs the same catchable child SIGSEGV semantics after full/partial unmap; unmap range semantics should be rechecked once signal delivery is fixed.
- `mprotect01`: one real alignment errno guard is patched, but file-backed max-permission tracking is still needed to make read-only shared mappings reject writable `mprotect()` with `EACCES`.
- `mprotect02`: still depends on catchable SIGSEGV plus write permission update semantics after `mprotect()`.

Recommended next feasible patches if leader keeps this lane open:

1. Add non-invasive mmap VMA metadata records to `UserProcess` and render them in `/proc/self/maps`; include `MAP_FIXED` replacement and `munmap` partial overlap updates.
2. Add max-permission metadata for file-backed shared mappings so `mprotect(PROT_WRITE)` can return `EACCES` when backing fd/path is not writable.
3. Route user page faults through the existing signal-delivery machinery when a handler exists; preserve signal wait-status behavior for unhandled default SIGSEGV.

## F. Verification run in this worker

No QEMU was started.

- `cargo fmt --check --manifest-path examples/shell/Cargo.toml` -> PASS after formatting.
- `cargo check --manifest-path examples/shell/Cargo.toml` -> PASS.
- `git diff -- examples/shell/src/cmd.rs` -> empty; `LTP_STABLE_CASES` untouched.
- `git diff -- .omx/ultragoal` -> empty; `.omx/ultragoal` untouched.

## G. Stop condition reached

Task-5 lane has a narrow safe mmap/mprotect errno patch plus a guardrail report. Runtime promotion remains blocked by no-QEMU policy and by unresolved `/proc/self/maps` VMA rendering and catchable SIGSEGV delivery semantics.
