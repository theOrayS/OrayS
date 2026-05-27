# Worker 1 / task 3 metadata-statfs-getdents narrow repair report

Team: `ltp-stable413-to-stab-d9f99e59`  
Date: 2026-05-27  
Owner: `worker-1`  
Scope: task 3, metadata/statfs/getdents lane

## Guardrails and stop condition

- I did not edit `.omx/ultragoal` or `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- I did not run QEMU or `run-eval.sh`; this worker lane used only source inspection, local build/typecheck, and parser/unit checks.
- Live stable list preflight remains `413 total / 413 unique / 0 duplicates`.
- Focus cases are still absent from stable: `fstat02`, `fstat02_64`, `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01`, `getcwd03`, `getcwd04`, `getdents01`, `getdents02`.
- Stable sentinels present and protected: `fstat03`, `fstat03_64`, `fstatat01`, `statfs02`, `statfs02_64`, `fstatfs02`, `fstatfs02_64`, `statvfs02`, `getcwd01`, `getcwd02`.
- Leader follow-up `docs/ltp-score-improvement-2026-05-27-phase-a/raw/batch-001-rv-inline-summary.txt` was still absent during this lane; task 6 remains waiting on that raw file.

## Narrow code changes

`examples/shell/src/uspace/fd_table.rs`:

1. `FdTable::getdents64` now checks whether the recorded directory path still exists before reading entries. If the directory was removed after `open(O_DIRECTORY)`, it returns `ENOENT` instead of continuing on a stale directory handle. This directly targets the `getdents02` unlinked-directory errno expectation without adding any legacy `getdents` alias.
2. `FdTable::statfs_path` now opens the target with `O_PATH_FLAG` rather than read-open semantics, then enforces parent-directory search permission for non-root callers. This keeps `statfs(2)` metadata semantics independent from file read permission while allowing `statfs03` to return `EACCES` when a parent directory lacks search permission.

No dispatcher alias was added. Existing dispatch still covers `__NR_statfs`, `__NR_fstatfs`, `__NR_getcwd`, `__NR_newfstatat`, `__NR_fstat`, and `__NR_getdents64`, and the absence of a safe target-arch legacy `__NR_getdents` path remains explicit.

## LTP source expectations inspected

Source snapshot: `/tmp/ltp-src-worker4/testcases/kernel/syscalls`.

| Case | LTP expectation | Current lane conclusion |
| --- | --- | --- |
| `getdents01` | Variant matrix in `getdents.h` runs raw `__NR_getdents`, raw `__NR_getdents64`, and optional libc wrappers. `getdents01.c` expects complete `.`/`..`/dir/file/symlink listing with no duplicate or unexpected names. | Current code only implements `getdents64`; no blind legacy alias was added. Existing nonzero `d_ino`, monotonic `d_off`, aligned `d_reclen`, and `d_type` packing are now preserved, but promotion still needs runtime proof because legacy/libc variants can still TCONF/ENOSYS. |
| `getdents02` | Expects `EBADF`, `EINVAL`, `ENOTDIR`, `ENOENT` after directory unlink, and `EFAULT` for bad user buffer. | New stale-directory existence check targets the `ENOENT` row. Existing wrapper validates user write first, small buffer returns `EINVAL`, non-directory returns `ENOTDIR`, and invalid fd returns `EBADF`. Runtime proof still required. |
| `fstat02`, `fstat02_64` | `fstat()` must report current uid/gid, size `1024`, mode `0644`, and hard-link count `2` after `link()`. | Not changed. Prior rows showed `ENOSYS`; repo has no safe evidence for old-stat aliasing and no linkat syscall implementation was in this lane. Needs exact syscall trace/ABI diagnosis before repair. |
| `fstatfs01`, `fstatfs01_64` | `fstatfs()` must succeed on mounted file and pipe fds. | Not changed. Current `fstatfs(fd)` uses fd kind to synthesize statfs data; prior blockers had `TBROK` without `ENOSYS`, so this remains runtime/semantic diagnosis. |
| `statfs01`, `statfs01_64` | `statfs()` must succeed on a mounted test file across filesystems. | Read-open was replaced with `O_PATH_FLAG` in the statfs path helper, reducing false failures from data read permissions. Still not promotion-clean without RV+LA proof. |
| `statfs03`, `statfs03_64` | As unprivileged `nobody`, `statfs(testdir/subdir)` must fail with `EACCES` when parent search permission is denied. | New parent-search check targets this exact semantic while preserving stable negative-path errno sentinels. |
| `statvfs01` | libc `statvfs()` must succeed and `f_namemax` must match creat success/failure at valid/too-long lengths. | Not directly changed; it depends on statfs-like data. Keep blocked until statfs/fstatfs runtime proof is clean. |
| `getcwd03` | `getcwd()` through a symlinked directory must resolve to the same physical directory and readlink basename must match. | Not changed; likely chdir/path/symlink setup semantics, not a direct `sys_getcwd` buffer issue. |
| `getcwd04` | Fork/rename race test requires at least two CPUs and previously surfaced `TCONF`. | Not changed. `TCONF` remains transparent and must not be counted as clean. |

## Current source anchors

- `examples/shell/src/uspace/syscall_dispatch.rs:133-204` dispatches the supported metadata/statfs/getdents syscalls.
- `examples/shell/src/uspace/fd_table.rs:602-614` validates/writes the `getdents64` user buffer.
- `examples/shell/src/uspace/fd_table.rs:1188-1235` now rejects removed directory handles with `ENOENT` and preserves current `linux_dirent64` packing.
- `examples/shell/src/uspace/fd_table.rs:1661-1674` now uses `O_PATH_FLAG` and checks parent search permission before returning statfs data.
- `examples/shell/src/uspace/metadata.rs:747-825` still owns `newfstatat`/`fstat`; no blind old-stat alias was added.
- `examples/shell/src/uspace/metadata.rs:1043-1075` still owns `statfs`/`fstatfs` syscall entry points.

## Subagent findings integrated

Subagent `019e68c7-c034-7fa2-8c30-5c4b81b2d810` (`gpt-5.4-mini`) completed a read-only review probe. Integrated findings:

- Do not treat prior `TBROK`, `TFAIL`, `TCONF`, or `ENOSYS` as promotion-clean.
- Do not add legacy `getdents`, `stat`, or `lstat` aliases without exact target-arch syscall evidence.
- Treat `getdents64` directory entry semantics as a narrow improvement candidate, but not sufficient for promotion by itself.
- Treat statfs/statvfs as semantic/setup-sensitive and guard already-stable `statfs02`, `fstatfs02`, and `statvfs02` rows.
- Keep `fstat02/_64` in diagnostic status until the failing syscall number and hard-link/stat behavior are known.

## Recommended leader-run verification subset

No LA-confirmation subset is recommended yet because this lane has no fresh RV parser-clean rows.

If the leader has an isolated evaluator image, the next safe serial scout is RV-first:

```bash
LTP_CASES=getdents01,getdents02,statfs03,statfs03_64,statfs02,statfs02_64,statvfs02 \
  LTP_TIMEOUT=120 ./run-eval.sh rv | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker1-metadata-rv-afterpatch-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker1-metadata-rv-afterpatch-001.log \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker1-metadata-rv-afterpatch-001-summary.txt
```

Only if those RV rows are wrapper PASS with zero internal `TFAIL/TBROK/TCONF`, timeout, `ENOSYS`, and panic/trap should LA confirmation run for the same cases. Keep `fstat02/_64`, `fstatfs01/_64`, `statfs01/_64`, `statvfs01`, and `getcwd03/04` out of promotion until their own fresh parsed evidence is clean.

## Verification evidence

PASS/FAIL below is for local worker verification only; it is not LTP promotion evidence.

| Check | Result | Evidence |
| --- | --- | --- |
| Worker mailbox/inbox | PASS | Re-read task 3 inbox and mailbox; no new undelivered steering. `raw/batch-001-rv-inline-summary.txt` absent. |
| Stable list preflight | PASS | `stable_total=413 unique=413 duplicates=0`; focus cases absent; sentinels present. |
| Rust format | PASS | `rustfmt --edition 2024 examples/shell/src/uspace/fd_table.rs`; later `rustfmt --edition 2024 --check examples/shell/src/uspace/fd_table.rs`. |
| Typecheck/build | PASS | `cargo check -p arceos-shell --features 'uspace,auto-run-tests,axhal/defplat' --target riscv64gc-unknown-none-elf` finished successfully. A narrower check without `auto-run-tests` failed on pre-existing `WaitQueue::wait_timeout_until` feature wiring, so the successful command includes the feature used by this uspace test build shape. |
| Test suite fallback | PASS | `python3 -B scripts/test_ltp_summary.py` ran 10 tests OK. |
| Rust no-run tests | FAIL / not applicable | `cargo test -p arceos-shell --features 'uspace,auto-run-tests,axhal/defplat' --target riscv64gc-unknown-none-elf --no-run` failed with `can't find crate for test` for the bare-metal target; this is a harness limitation, not a code failure in the modified file. |
| End-to-end LTP | NOT RUN | QEMU/evaluator runs are explicitly prohibited for this worker lane without an isolated image. |

## Promotion status

- Promotion-ready cases from this lane: **none**.
- Plausible post-patch RV scout candidates: `getdents02`, `statfs03`, `statfs03_64`, plus sentinel `statfs02`, `statfs02_64`, `statvfs02`.
- Blocked/diagnostic cases: `fstat02`, `fstat02_64`, `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statvfs01`, `getcwd03`, `getcwd04`, `getdents01`.
