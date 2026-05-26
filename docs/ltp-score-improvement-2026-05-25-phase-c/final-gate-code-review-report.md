# Final gate code review report

Date: 2026-05-26
Scope: `examples/shell/src/uspace/fd_socket.rs`, `examples/shell/src/cmd.rs`, phase-c reports.

## Review result

Recommendation: **COMMENT**
Architectural status: **WATCH**
Merge blocker for stable381 partial promotion: **none after follow-up fix**

## Findings and resolution

### HIGH — resolved

- File: `examples/shell/src/uspace/fd_socket.rs`
- Finding: the first AF_UNIX local socket connect repair returned `ENOTSOCK` for existing pathname targets and used direct `axfs::api::metadata(path)` lookup.
- Risk: hidden POSIX/LTP errno checks could expect `ECONNREFUSED`, and relative paths could bypass process-visible cwd/mount resolution.
- Fix applied: pathname connects now use `resolve_dirfd_path(process, ..., AT_FDCWD, path)` before metadata lookup and return `ECONNREFUSED` for existing pathname targets until a real AF_UNIX listener registry exists. Missing pathname still returns `ENOENT`; invalid user pointers/families still return their respective errno.

### WATCH — documented partial support

- The code intentionally does not implement full AF_UNIX pathname sockets. `bind/listen/accept` remain outside this patch. The new path is a bounded errno compatibility shim for valid local socket fds and known libc nscd/group lookup probes.
- This is acceptable for the stable381 partial promotion because `chmod05`/`fchmod05` have post-fix targeted RV+LA x musl+glibc clean evidence and no LTP source/harness fake PASS was introduced.

## Positive checks

- `examples/shell/src/cmd.rs` only adds `chmod05` and `fchmod05`; live stable count is 381 total / 381 unique / 0 duplicates.
- No test-name hardcoding, fake PASS marker, LTP source modification, timeout laundering, or wrapper-only promotion was found.
- Targeted and aggregate evidence are preserved in phase-c docs and raw summaries.

## Final recommendation rationale

`COMMENT` rather than `APPROVE` because the AF_UNIX support is intentionally partial and should stay on the follow-up watchlist. It is not a blocker for committing the current stable381 partial promotion because the blocking errno/path-resolution issue was fixed and revalidated with targeted RV+LA checks.
