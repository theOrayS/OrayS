# Syscall / ABI candidate report (worker-4, 2026-05-22)

Task: worker-4 Syscall/ABI lane for low-risk real fixes or an evidence-gap report.  
Scope honored: no `.omx/ultragoal` edits; no fake PASS, hardcoded case success, or silent SKIP.

## Decision

No new `examples/shell/src/uspace/*` patch is included in this worker commit.

Reason: the current worker tree already includes the prior ABI expansion commit `dc8180fe` (`feat:improve ltp to 88 passes(44 each)`), which added the main low-risk symlink/readlink/path-metadata work and expanded `LTP_STABLE_CASES` to 44 cases. The available top-level logs (`output_la.md`, `output_rv.md`) still show only the previous 16-case LTP core result, not a fresh 44-case stable or expanded-batch run. Without a current failing targeted batch, another syscall patch would be speculative.

## Evidence reviewed

### Current summaries still only prove 16 core cases per arch

Commands:

```sh
python3 scripts/ltp_summary.py output_rv.md >/tmp/worker4-rv-summary.md
python3 scripts/ltp_summary.py output_la.md >/tmp/worker4-la-summary.md
```

Observed:

- `output_rv.md`: `PASS LTP CASE: 32`, `FAIL LTP CASE: 0`, internal `TCONF: 2`, ENOSYS `0`, panic/trap `0`.
- `output_la.md`: `PASS LTP CASE: 32`, `FAIL LTP CASE: 0`, internal `TCONF: 2`, ENOSYS `0`, panic/trap `0`.
- Both logs contain 16 LTP cases x 2 libc variants. They do not prove the current 44-case `stable` list.
- The only LTP-internal marker in the current summaries is `chdir01` symlink-loop `TCONF` in both libc variants on both arches.

### Current source already includes the main low-risk path/symlink fix surface

Commands:

```sh
sed -n '44,175p' examples/shell/src/cmd.rs
rg -n "symlink|readlink|path_symlink|__NR_symlinkat|__NR_readlinkat" examples/shell/src/uspace
```

Observed source state:

- `examples/shell/src/cmd.rs` now has a 44-case `LTP_STABLE_CASES` list including fs/syscall additions such as `fcntl01`, `access03`, `close02`, `dup02`, `getcwd01`, `lseek01`, `creat01`, `open02`, `stat02`, `lstat01`, `chmod01`, `fchmod01`, `rmdir01`, `symlink01`, `readlink01`, `ftruncate01`, and `umask01`.
- `examples/shell/src/uspace/syscall_dispatch.rs` dispatches `__NR_symlinkat` and `__NR_readlinkat`.
- `examples/shell/src/uspace/metadata.rs` contains `sys_symlinkat`, `sys_readlinkat`, synthetic `path_symlinks`, symlink `lstat` support, and bounded symlink resolution.
- `examples/shell/src/uspace/fd_table.rs` removes synthetic symlinks on unlink and resolves symlinks in path opening.
- `examples/shell/src/uspace/process_lifecycle.rs` clones `path_symlinks` on fork.

These are the exact P0 recommendations from the 2026-05-21 syscall triage, so the next useful step is validation, not another unproven edit.

### Prior full-LTP evidence still identifies useful next candidates

Commands:

```sh
sed -n '1,260p' docs/ltp-score-improvement-2026-05-21/discovery-candidates.md
sed -n '1,260p' docs/ltp-score-improvement-2026-05-21/syscall-hardblocker-triage.md
rg --text -n "symlink\(|Cannot parse kernel \.config|Group ID lookup|chroot\(|copy_file_range|sigsuspend\(\)|clone\(\) failed|ENOSYS|not implemented" eval-reports/full-ltp-20260519-132237/*.md
```

Relevant older failures:

- `access02`: `symlink(file_f,symlink_f) failed: ENOSYS` in the old full-LTP logs. This is likely addressed by the current symlink/readlink implementation, but needs a fresh targeted run.
- `chmod05` / `chmod07` / `creat08`: group lookup failures (`getgrnam(users)` fallback to `daemon`, then `ENOTSOCK`) on both arches. Current source exposes synthetic `/etc/group`, but the content is only `root` and `nogroup`; the failure may also involve missing NSS configuration or libc trying an nscd/socket path.
- config-gated tests: repeated `Cannot parse kernel .config`; current `synthetic_fs.rs` has `/proc/self/*`, `/proc/<pid>/*`, `/etc/passwd`, `/etc/group`, but no `/proc/config.gz` or `/proc/config` synthetic content.
- `sigsuspend()`: old cgroup/cpuctl path saw `Function not implemented`; current `time-signal-basic` batch includes `sigsuspend01`, but `syscall_dispatch.rs` does not dispatch `rt_sigsuspend` yet.
- `chroot*`, `copy_file_range*`, and clone variants remain higher-risk because real semantics affect global path resolution, fd offsets, or process hierarchy.

## Candidate ranking for next targeted runs

| Priority | Candidate | Why | Missing evidence before patch/promotion | Likely files if failure reproduces |
| --- | --- | --- | --- | --- |
| P0 validate | `stable` 44-case list | Source already includes 44 cases and symlink/path work; this is the direct score gate. | Fresh targeted LA/RV x musl/glibc run with `LTP_CASES=stable` or `/ltp_cases.txt` proving each new case. | `examples/shell/src/cmd.rs`, existing `uspace` path/metadata files only if a new case fails. |
| P0 validate | `access02 symlink01 readlink01 lstat01` | Old full-LTP symlink ENOSYS should be fixed by current source. | Per-case PASS/FAIL logs for both arches/libcs; verify no wrapper PASS hides TFAIL/TBROK/TCONF. | `metadata.rs`, `fd_table.rs`, `syscall_dispatch.rs`, `process_lifecycle.rs`. |
| P1 evidence first | `chmod05 chmod07 creat08` user/group lookup | Current `/etc/group` lacks `users` and `daemon`, matching old failure names, but old errno was `ENOTSOCK`, so content alone may not be enough. | Single-case logs showing whether libc reads `/etc/group`, tries `/etc/nsswitch.conf`, or opens an nscd socket. | `linux_abi.rs`, `synthetic_fs.rs`, possibly socket/path routing if ENOTSOCK persists. |
| P1 evidence first | config-gated cases (`acct02`, `aslr01`, `bind06`, etc.) | Old logs repeatedly broke on missing parseable kernel config; an honest disabled-feature config would convert TBROK to TCONF/skip for unsupported features. | Confirm current source still lacks `/proc/config.gz` and which exact config keys LTP requests. | `synthetic_fs.rs`, `fd_table.rs`, `metadata.rs`. |
| P2 evidence first | `sigsuspend01` / `rt_sigsuspend` | Current discovery batch includes `sigsuspend01`; dispatch appears absent. | Run `time-signal-basic` or only `sigsuspend01` and confirm `ENOSYS`/semantic failure. | `syscall_dispatch.rs`, `signal_abi.rs`. |
| P2 defer | `chroot*`, `copy_file_range*` | Real fixes can promote cases, but semantics are broader than a safe no-log patch. | Fresh targeted failure logs and narrowed expected errno/offset behavior. | `fd_table.rs`, `runtime_paths.rs`, `process_lifecycle.rs`, `syscall_dispatch.rs`. |
| P3 defer | clone variants and RV CVE/OOM | High risk to process lifecycle/memory accounting; not needed for stable promotion. | Separate hard-blocker runs with memory/process lifecycle counters. | `process_lifecycle.rs`, `task_context.rs`, `memory_map.rs`, kernel memory code. |

## Recommended next commands

Use targeted runs before any more ABI edits:

```sh
# Validate the current 44-case stable source first, not full LTP.
# Exact injection method depends on the runner wrapper used by the leader.
printf '%s\n' \
  access01 brk01 chdir01 clone01 close01 dup01 fcntl01 fcntl02 fork01 getpid01 mmap01 open01 pipe01 read01 stat01 wait401 write01 \
  access03 close02 dup02 fcntl03 getcwd01 getpid02 getppid01 getuid01 geteuid01 getgid01 getegid01 lseek01 read02 write02 \
  creat01 creat03 open02 open03 stat02 lstat01 chmod01 fchmod01 rmdir01 symlink01 readlink01 ftruncate01 umask01 \
  > /tmp/ltp-stable-44-cases.txt
```

If only one small ABI probe can be run, start with:

```text
access02 symlink01 readlink01 lstat01 chmod05 chmod07 creat08 sigsuspend01
```

This separates already-implemented symlink validation from the still-unproven userdb and signal gaps.

## Stop condition for this worker lane

A source patch should be made only after a fresh targeted run shows a concrete current failure. The present evidence supports a report and targeted validation plan, not another speculative uspace edit.
