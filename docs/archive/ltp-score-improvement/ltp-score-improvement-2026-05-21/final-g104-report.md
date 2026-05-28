# G104 final report - LTP stable batch 44

Date: 2026-05-21

## Result

Promoted the default stable LTP runner from 31 to 44 cases per libc/arch. The final full evaluator gate completed on both architectures without LTP failures:

| Arch | run-eval status | LTP musl | LTP glibc | PASS LTP CASE | FAIL LTP CASE | LTP internal | LTP timeout | ENOSYS | panic/trap |
| --- | ---: | --- | --- | ---: | ---: | --- | ---: | ---: | ---: |
| LA | 0 | 44 passed, 0 failed | 44 passed, 0 failed | 88 | 0 | TCONF=4 | 0 | 0 | 0 |
| RV | 0 | 44 passed, 0 failed | 44 passed, 0 failed | 88 | 0 | TCONF=4 | 0 | 0 | 0 |

Notes:
- The summary tool reports 10 timeout matches in each full output from non-LTP benchmark groups (`libctest`, `lmbench`, `cyclictest`, `iozone`, `unixbench`); LTP group timeout remains 0.
- `read02` contributes the 4 LTP `TCONF` markers, from `O_DIRECT not supported on tmpfs`, while the wrapper case still passes.

## Promoted stable LTP cases

The 13 cases added beyond the prior 31-case stable list are:

`creat01`, `creat03`, `open02`, `open03`, `stat02`, `lstat01`, `chmod01`, `fchmod01`, `rmdir01`, `symlink01`, `readlink01`, `ftruncate01`, `umask01`.

## Implementation summary

- `examples/shell/src/cmd.rs`
  - Added named LTP batches: `stable`, `core`, `syscalls-basic-plus`, `fs-basic`, `proc-basic`, `time-signal-basic`.
  - Added `LTP_CASES` / `/ltp_cases.txt` case selection and `OSCOMP_TEST_GROUPS` / `/test_groups.txt` group filtering.
  - Added configurable per-case timeout (`LTP_CASE_TIMEOUT_SECS` / `/ltp_case_timeout_secs`), per-case runtime lines, and separate timeout accounting.
- `scripts/ltp_summary.py`
  - Emits per-case matrix and classification for TFAIL/TBROK/TCONF, timeout, ENOSYS, panic/trap.
- `examples/shell/src/uspace/fd_table.rs`
  - Tracks file status flags for `fcntl(F_GETFL/F_SETFL)`.
  - Applies umask to `open(O_CREAT)` and `mkdirat` recorded metadata.
  - Enforces `O_NOATIME` permission checks using effective uid and recorded/default owner.
  - Resolves/removes recorded symlinks and exposes `/proc/<pid>/status` synthetic entries.
- `examples/shell/src/uspace/metadata.rs`
  - Added syscall-compatible `umask` state.
  - Added recorded symlink metadata and `symlinkat`/`readlinkat`/`newfstatat`/`statx(AT_SYMLINK_NOFOLLOW)` behavior.
  - Added `ENAMETOOLONG` handling for symlink target/linkpath length >= 4096.
- `examples/shell/src/uspace/mod.rs`, `process_lifecycle.rs`
  - Added/cloned `path_symlinks` and `umask` per user process/fork.
- `examples/shell/src/uspace/linux_abi.rs`
  - Added Linux symlink file type mode constant.
- `examples/shell/src/uspace/memory_map.rs`
  - Preserves shared anonymous mappings for msync/shared-map behavior.
- `examples/shell/src/uspace/synthetic_fs.rs`
  - Added `/proc/self/status` and `/proc/<pid>/status` synthetic content.
- `examples/shell/src/uspace/syscall_dispatch.rs`
  - Routed `symlinkat` and `umask` to real handlers.

## Verification commands

```bash
cargo fmt --all -- --check
./run-eval.sh la 2>&1 | tee output_la.md
./run-eval.sh 2>&1 | tee output_rv.md
python3 scripts/ltp_summary.py output_la.md
python3 scripts/ltp_summary.py output_rv.md
```

Artifacts:
- `output_la.md`
- `output_rv.md`
- `docs/ltp-score-improvement-2026-05-21/final-output-la-g104-summary.txt`
- `docs/ltp-score-improvement-2026-05-21/final-output-rv-g104-summary.txt`
- `docs/ltp-score-improvement-2026-05-21/phase-1-2-evidence.md`

## Remaining risks / next batch

- `link02` still needs hard-link syscall semantics.
- `unlink05` still exposes FIFO/mkfifo gaps after ordinary unlink passes.
- `mkdir02` still has group/SGID inheritance and musl group lookup blockers.
- `rename01` remains blocked by evaluator disk/device setup returning ENOSPC/TBROK.
- Recorded symlinks are process/fork-local metadata, not a complete persistent global symlink filesystem.
- Umask and mode fixes update recorded metadata used by the POSIX stat path; underlying axfs permission storage is still limited.
- Hard-blocker lane remains: LA crash01/user trap to signal, RV full-LTP CVE/OOM/free_frames.

## POSIX / ABI visible behavior changes

- `umask(2)` now stores and returns previous mask and affects recorded mode metadata for `open(O_CREAT)` and `mkdirat`.
- `symlinkat(2)` no longer returns ENOSYS for recorded shell-uspace symlinks; `readlinkat`, `newfstatat`, and `statx` observe those symlinks, including `AT_SYMLINK_NOFOLLOW`.
- `fcntl(F_GETFL/F_SETFL)` now preserves visible status flags for regular file descriptors.
- `open(O_NOATIME)` now returns `EPERM` for non-root users that do not own the target.
- `/proc/<pid>/status` and `/proc/self/status` now exist as synthetic read-only files.
