# Phase 0 report: VFS metadata/path + FD/fcntl/pipe/io candidate lane

Date: 2026-06-01
Worker: `worker-2`
Task: `task-4` / report-only source diagnosis
Scope: first-milestone scouting for stable506 -> stable556.  This report does **not** promote stable cases, does **not** edit `.omx/ultragoal`, and does **not** start QEMU/evaluator runs.

## 1. Live baseline and evidence boundary

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` was re-counted in this worktree:

```text
506 total / 506 unique / 0 duplicate
```

Therefore the first 50-case milestone is `stable556`, but this lane alone should be treated as a source-diagnosis/backlog slice rather than a promotion gate.  Every ranked row below still needs fresh RV + LA x musl + glibc parser-clean evidence before it can be added to `LTP_STABLE_CASES`.

Evidence read for this report:

- Live stable source: `examples/shell/src/cmd.rs:50-271`.
- Current long-term plan: `docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md` and `startup-prompt-1000ltp-ultragoal-team.md`.
- Previous stable506 final gate summary: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md` (`PASS LTP CASE 1012`, `FAIL 0`, inherited `read02` TCONF only).
- Previous candidate matrix: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix/candidate-matrix-stable460-to-500plus.md` and `clean-candidates-not-in-stable460.txt`.
- Previous FD/fcntl/pipe lane: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-03-fd-fcntl-pipe-ownership/{session-report.md,promotion-candidates.md,targeted-cases.md,validation.md}`.
- Previous VFS/metadata/path lane: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-04-vfs-metadata-path/{session-report.md,promotion-candidates.md,targeted-cases.md,validation.md}`.
- Full-sweep closure/scouting summaries: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-report.md`, `final-quality-gate.json`, and summary paths recorded there (`rv-arch002`, `la-arch012`).

Important boundary: the old full-sweep “clean” rows are scouting hypotheses only.  They are not promotion evidence without fresh targeted gates and `scripts/ltp_summary.py` checks.  The known `read02` `TCONF` caveat must remain visible in any stable aggregate gate.

## 2. Source touchpoint map

| Area | Current source surfaces | Notes for diagnosis |
| --- | --- | --- |
| Syscall routing | `examples/shell/src/uspace/syscall_dispatch.rs:105-160`, `201-247` | Routes read/write/vector/sendfile/statfs/openat/mkdirat/mknodat/unlinkat/pipe2/truncate/fchmod/faccessat/fchownat/statx/fstat/getdents64/fcntl/readlinkat. Unmatched syscalls still return `ENOSYS`. |
| VFS permissions/path metadata | `examples/shell/src/uspace/metadata.rs:270-335`, `438-611`, `900-971`, `1190-1274`, `1291-1439`; `credentials.rs:439-457` | `faccessat`, chmod/chown metadata, statx, readlinkat, statfs/fstatfs, and owner/group metadata live here. |
| Directory/path operations | `examples/shell/src/uspace/fd_table.rs:676-825`, `1491-1625`, `2605-2679`, `3031-3057` | `renameat2`, `mkdirat`, `mknodat`, `unlinkat`, path resolution, parent-write/search permission and sticky checks. |
| FD flags/vector/sendfile | `fd_table.rs:263-330`, `490-617`, `1782-1884`, `1896-1950`, `2724-2737` | `readv/writev`, `sendfile`, `getdents64`, `fcntl` FD/status flags, pipe size, record locks, and `O_APPEND/O_NONBLOCK/O_DIRECT` filtering. |
| Pipe behavior | `examples/shell/src/uspace/fd_pipe.rs:186-220`, `280-330` | SIGPIPE is deferred to user-return hook after fd-table lock release; `pipe2` supports `O_CLOEXEC/O_NONBLOCK/O_DIRECT`, capacity is one `PIPE_BUF_SIZE`. |
| Synthetic/proc dependencies | `fd_table.rs:2740-2890`, `metadata.rs:1291-1368` | `/proc/self/fd`, synthetic proc/stat/status/comm, `/dev/null`, block device, RTC, FIFO, symlink paths. Several blocked pipe/fcntl cases depend here. |

## 3. Ranked candidate queues for stable556 planning

Rank meanings:

- **A / proof-first**: likely cheapest stable556 contribution if fresh targeted gates confirm old evidence; avoid source edit until proof fails.
- **B / narrow-fix**: source shape suggests a bounded real fix could unlock multiple cases, but targeted failing subtests must be captured first.
- **C / blocker-first**: valuable, but current evidence shows a real blocker or missing semantic model; not first promotion batch.
- **D / defer**: keep out of stable556 easy-first lane unless leader assigns a broader subsystem repair.

### 3.1 A-ranked proof-first candidates

| Rank | Cases | Why this is first | Touchpoints if proof fails | Required regressions |
| --- | --- | --- | --- | --- |
| A1 | `fs_perms`, `readdir01` | Session 1 matrix lists these as full-sweep clean but not in stable460; they remain non-stable in the live stable506 list.  This is the best “fresh targeted proof before code” VFS fill. | `fd_table.rs::open_candidates`, `getdents64`, parent permission helpers, `metadata.rs::sys_faccessat`. | `stat01/stat02/lstat01/fstatat01`, `getcwd01/getcwd02`, `statfs02/fstatfs02/statvfs02`, plus Session 4 VFS final subset. |
| A2 | `fcntl19_64`, `fcntl22_64` | Non-`_64` siblings `fcntl19` and `fcntl22` are already stable after the record-lock lane; source now has generic POSIX record-lock handling.  The `_64` variants are still non-stable and should be targeted before new design work. | `fd_table.rs::fcntl`, `fcntl_getlk`, `fcntl_setlk`, `normalize_record_lock`, `apply_record_lock`; verify whether LTP uses a different command/layout on this ABI. | Stable `fcntl11`, `fcntl14`, `fcntl19`, `fcntl22`, plus `fcntl07`, `fcntl12`, `fcntl13`, `fcntl18`, `fcntl29`, `dup05`. |
| A3 | `openat02`, `openat03` | Adjacent to stable `openat01` and many stable `open*` rows; current open path has O_PATH/O_NOFOLLOW/O_DIRECTORY, synthetic path, FIFO and permission logic.  Scout before touching path code. | `fd_table.rs::open`, `open_fd_entry`, `open_candidates`, `fcntl_status_flags`, `check_open_permission`. | Stable `open01/02/03/04/06/08/09/13`, `faccessat01/02/201/202`, `readlinkat01`, `symlink*`. |

### 3.2 B-ranked narrow-fix candidates

| Rank | Cases | Likely syscall/errno/flag surface | Why not A | Regression guard |
| --- | --- | --- | --- | --- |
| B1 | `fcntl20`, `fcntl20_64`, `fcntl21`, `fcntl21_64` | `F_GETLK/F_SETLK/F_SETLKW` record-lock behavior, `struct flock` copy-in/out, access-mode validation, blocking/yield and stale-lock cleanup. | Session 3 promoted nearby lock rows but documented `F_SETLKW` still lacks full signal-interrupt semantics; these rows need exact LTP source/subtest mapping before patching. | Same fcntl regression set as A2; add multi-process close/dup/exit lock-release checks. |
| B2 | `mknod01`, `mknod03`, `mknod04`, `mknod07`, `mknod09`, `mknodat02` | `sys_mknodat`, node-type filtering, FIFO vs regular fallback, parent write/search permission, `EEXIST/EPERM/ENAMETOOLONG/ENOENT` ordering. | Current source supports regular/FIFO only and returns `EPERM` for other node types; some LTP rows may need device-node semantics rather than another FIFO/file shim. | Stable `mknod02`, `mknod05`, `mknod06`, `mknod08`, `mknodat01`, `open06`, FIFO/open regression. |
| B3 | `rename03`, `rename04`, `rename05` | `sys_renameat2`, plain `axfs::api::rename`, path resolution, parent permissions, sticky semantics, nonzero `renameat2` flags. | `rename14` is stable, but `sys_renameat2` currently accepts only `flags == 0`; cases with `RENAME_NOREPLACE/EXCHANGE/WHITEOUT` or sticky/permission setup are broader. | Stable `rename14`, `unlink05/07/08`, `rmdir01/03`, `mkdir04/05`, `symlink*`, parent permission tests. |
| B4 | `access04`, `chmod06`, `chmod07`, `fchmod02`, `fchmod06`, `chown04`, `fchown04`, `fchownat02` | `sys_faccessat`, `sys_fchmod/sys_fchmodat`, `sys_fchown/sys_fchownat`, `apply_chown_metadata`, uid/gid/fsuid/fsgid and special-mode-bit clearing. | Prior lanes already fixed/promoted some permission rows (`chmod05`, `fchmod05`, `fchmodat02`, `chown01-03/05`, `fchown01-03/05`, `fchownat01`), so remaining rows likely hit edge subtests rather than missing dispatch. | Stable chmod/chown/fchmod/fchown/faccessat rows, parent search, O_PATH/AT_EMPTY_PATH, setuid/setgid clearing. |

### 3.3 C-ranked blocker-first diagnostics

| Rank | Cases | Current blocker evidence | Minimal next step | Do not do |
| --- | --- | --- | --- | --- |
| C1 | `readlinkat02`, `readlink03` | Session 4: `readlinkat02` was RV clean and LA glibc clean but LA musl had `TFAIL=1`; `readlink03` still had component-level `ELOOP`/TFAIL style gaps after partial fixes. | Capture exact LA musl `readlinkat02` subtest and compare `metadata.rs::sys_readlinkat` copy-out/errno/path-permission behavior. | Do not count 3/4 clean as PASS; do not hardcode `/proc/self/fd` or LTP path names. |
| C2 | `statx01`, `statx04` | Session 4 kept both out: TBROK/failure after `statx03` was fixed/promoted. | Targeted trace to see whether failure is flag/mask/pathname/field/timestamp-related; inspect `stat_to_statx` before editing. | Do not blindly widen supported mask or fake timestamp fields without Linux-visible semantics. |
| C3 | `statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01` | Prior reports marked statfs/statvfs first rows as blocked/diagnostic while `statfs02`, `statfs03`, `fstatfs02`, `statvfs02` are stable guards. | Compare LTP expected struct fields and setup path against `generic_statfs`/`statfs_type_for_path` and `statfs_path`. | Do not regress existing statfs/stable field values or convert setup TBROK/TCONF into PASS. |
| C4 | `getdents01`, `getdents02` | Session 4: `getdents64` symlink overlay improved but legacy raw `getdents` still showed `TCONF/ENOSYS`; current dispatch has `getdents64`, not a proven legacy `getdents` path. | First get a syscall-number trace from the failing binary; only then consider an ABI-appropriate handler or LTP-source-driven conclusion. | Do not add blind numeric aliases for nonexistent arch syscalls. |
| C5 | `fcntl30`, `pipe07`, `pipe15` | Session 3: blocked by `/proc/sys/fs/pipe-max-size`, `/proc/self/fd`, and `/proc/sys/fs/pipe-user-pages-soft` synthetic/procfs dependencies. | Treat as synthetic/procfs mini-project: implement real readable values/errno, then run FD/pipe targeted gates. | Do not paper over missing proc files with case-specific strings. |
| C6 | `writev03`, `pwritev03`, `pipe2_03` | Prior FD lane preserved `writev03` as TCONF/device-style and `pwritev03` as TBROK/`ENOSPC` while creating `test_dev.img`; `pipe2_03` has no current clean proof. | Re-check exact LTP source/setup and device-image requirement before source changes. | Do not treat TCONF as stable; do not hide setup/resource failure. |

## 4. Recommended stable556 lane batches

These batches are **case lists for leader-owned targeted runs**, not stable-list edits.

### Batch VFS-A: proof-first, no source edit unless proof fails

```text
fs_perms,readdir01,openat02,openat03,fcntl19_64,fcntl22_64
```

Expected value: 2-6 low-risk additions if fresh RV/LA gates are clean.  This batch intentionally mixes VFS and FD rows that are closest to existing stable semantics.

### Batch FD-B: record-lock adjacency after A2 outcome

```text
fcntl20,fcntl20_64,fcntl21,fcntl21_64
```

Run after checking whether `_64` lock rows fail due to command/layout mismatch.  If any row fails, inspect LTP subtest output before touching `fd_table.rs`.

### Batch VFS-C: create/rename/path errno cluster

```text
mknod01,mknod03,mknod04,mknod07,mknod09,mknodat02,rename03,rename04,rename05
```

This is a good stable556 repair pool only if targeted logs show ordinary errno/flag gaps.  If the failures demand real device-node, mount/rofs, sticky-dir, or `renameat2` advanced flag semantics, split into a broader VFS milestone story.

### Batch PERM-D: ownership and permission edges

```text
access04,chmod06,chmod07,fchmod02,fchmod06,chown04,fchown04,fchownat02
```

Run only after isolating setup failures from true permission bugs.  Expected source surfaces are `metadata.rs`, `credentials.rs`, and path owner/mode bookkeeping; adjacent stable permission rows must be rerun.

### Batch BLOCKER-E: diagnostics only

```text
readlinkat02,readlink03,statx01,statx04,statfs01,fstatfs01,fstatfs01_64,statvfs01,getdents01,getdents02,fcntl30,pipe07,pipe15,writev03,pwritev03
```

This batch is useful for root-cause reports and future work decomposition, not for immediate stable556 promotion.

## 5. Required regression matrix

Minimum regression sets before any promotion from this lane:

- VFS/stat/readlink/xattr/statfs: `stat01,stat02,lstat01,lstat01_64,stat01_64,stat02_64,fstatat01,fstat03,fstat03_64,statx02,statx03,statfs02,statfs02_64,statfs03,statfs03_64,fstatfs02,fstatfs02_64,statvfs02,readlink01,readlinkat01,symlink01,symlink02,symlink04,symlinkat01`.
- VFS create/remove/path: `mknod02,mknod05,mknod06,mknod08,mknodat01,rename14,open01,open02,open03,open04,open06,open08,open09,open13,openat01,unlink05,unlink07,unlink08,rmdir01,rmdir03,mkdir04,mkdir05`.
- FD/fcntl/pipe: `fcntl07,fcntl11,fcntl12,fcntl13,fcntl14,fcntl18,fcntl19,fcntl22,fcntl29,pipe02,pipe08,pipe09,pipe10,pipe11,pipe12,pipe13,pipe14,pipe2_01,pipe2_02,pipe2_04,dup05`.
- IO/vector/sendfile: `readv01,readv02,writev01,writev02,writev05,writev06,writev07,preadv01,preadv02,preadv01_64,preadv02_64,pwritev01,pwritev02,pwritev01_64,pwritev02_64,sendfile02,sendfile03,sendfile04,sendfile05,sendfile06,sendfile07,sendfile08` including `_64` variants already in stable.

Gate rule for each promoted case remains:

1. targeted RV log parsed by `python3 -B scripts/ltp_summary.py`;
2. adjacent regression subset clean;
3. targeted LA log parsed;
4. musl + glibc wrapper PASS;
5. no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` beyond the already-disclosed `read02` caveat when a stable aggregate is included;
6. stable-list edit and milestone docs only by leader-owned promotion path.

## 6. Blocker risks to report upward

- The current live stable count is 506, not the older 460/474/500 snapshots in archived reports; old candidate evidence is useful only after filtering against live `LTP_STABLE_CASES`.
- `readlinkat02` is the clearest near-clean VFS row but must not be promoted from 3/4 clean evidence.
- `getdents01/getdents02` require ABI/syscall tracing before any legacy handler work; blind syscall aliases are high-risk.
- `fcntl30`, `pipe07`, and `pipe15` are synthetic/procfs dependency work, not just FD/fcntl implementation gaps.
- `writev03` and `pwritev03` carry TCONF/TBROK/setup-resource evidence and should stay outside easy-first stable556.
- No QEMU/evaluator was run for this report, so the only verification claim here is source/doc consistency plus `git diff --check`.

## 7. Subagent note

Subagent skip reason: delegation was optional, the task was report-only/source-diagnosis with a narrow artifact path, and serial local inspection of live source plus archived parser-backed reports was sufficient while avoiding duplicate shared-file reads.
