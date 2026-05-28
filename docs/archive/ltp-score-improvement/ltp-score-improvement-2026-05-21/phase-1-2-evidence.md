# LTP score improvement phase 1-2 evidence (2026-05-21)

## Scope and guardrails

Goal: raise the stable LTP runner score without hardcoded PASS, fake success, or silent skip of real failures.
Timeouts remain visible as timeout counts and are not counted as PASS. The default final runner is documented and reproducible: `stable`.

Team/Ultragoal execution:
- Team: `ltp-score-improvement-40e54c49`
- Team status before shutdown: 8 tasks completed, 0 pending, 0 failed.
- Team shutdown completed after integration. Worker-1 had a shutdown merge conflict against already integrated local `examples/shell/src/cmd.rs`; no additional worker diff was applied.

## Stable batch promoted

Default LTP batch is now `stable`, 31 cases per libc/arch:

```text
access01 brk01 chdir01 clone01 close01 dup01 fcntl01 fcntl02 fork01 getpid01 mmap01 open01 pipe01 read01 stat01 wait401 write01
access03 close02 dup02 fcntl03 getcwd01 getpid02 getppid01 getuid01 geteuid01 getgid01 getegid01 lseek01 read02 write02
```

Newly promoted over the previous 16-case core:

```text
fcntl01 access03 close02 dup02 fcntl03 getcwd01 getpid02 getppid01 getuid01 geteuid01 getgid01 getegid01 lseek01 read02 write02
```

Promotion condition used: both LA and RV, both musl and glibc, wrapper PASS, no LTP timeout, no ENOSYS/not implemented, no TFAIL/TBROK. `chdir01` and `read02` still report expected TCONF-only skips inside passing cases.

## Modified files and functions

### `examples/shell/src/cmd.rs`

Functions/constants changed:
- `LTP_STABLE_CASES`, `LTP_SYSCALLS_BASIC_PLUS_CASES`, `LTP_FS_BASIC_CASES`, `LTP_PROC_BASIC_CASES`, `LTP_TIME_SIGNAL_BASIC_CASES`, `LTP_CASE_BATCHES`
- `valid_ltp_case_name`
- `push_ltp_case`
- `split_ltp_case_list`
- `ltp_cases_from_slice`
- `ltp_static_case_list`
- `selected_ltp_cases`
- `ltp_case_timeout_secs`
- `selected_official_test_groups`
- `ltp_case_env`
- `rewrite_ltp_case_line`
- `run_ltp_suite`
- `maybe_run_official_tests`
- per-case runtime emission using `Instant` (`LTP CASE RUNTIME <case>: <ms> ms`)

Expected behavior:
- Default full evaluator LTP runner uses the reproducible `stable` 31-case batch.
- `core` remains available explicitly for the old 16-case baseline.
- Configurable exploration paths are available through `/ltp_cases.txt`, `/tmp/ltp_cases.txt`, compile-time `LTP_CASES`, `batch:<name>`, `file:<path>`, or inline lists.
- Per-case timeout defaults to 10 seconds and can be configured by `/ltp_case_timeout_secs` or compile-time `LTP_CASE_TIMEOUT_SECS`.
- Case script timeout prints `TIMEOUT LTP SCRIPT ...`; runner treats timeout exit codes 137/143 as timeout/failure, not PASS.
- Each case now emits `LTP CASE RUNTIME <case>: <ms> ms`, including missing-testcase failure paths, so the summary tool can report per-case runtime without affecting PASS/FAIL semantics.
- `OSCOMP_TEST_GROUPS` or `/test_groups.txt` can restrict stage runs to `ltp` only; default remains all evaluator groups.

### `scripts/ltp_summary.py`

Expected behavior:
- Emits LA/RV + musl/glibc case matrix.
- Separates wrapper PASS/FAIL, TFAIL/TBROK/TCONF, timeout, ENOSYS/not implemented, and panic/trap classifications.
- Parses suite summaries with optional `timed out` field and avoids attributing suite summary timeout text to the previous case.
- Parses `LTP CASE RUNTIME` lines, adds a `Runtime ms` matrix column, and includes `runtime_ms` in JSON output.

### `examples/shell/src/uspace/synthetic_fs.rs`

Functions changed/added:
- `proc_pid_status_content`
- `proc_pid_status_fd_entry`
- `proc_pid_status_path_entry`

Expected behavior:
- `/proc/self/status` and `/proc/<pid>/status` expose minimal Linux-like process identity metadata.
- `Name`, `State`, `Tgid`, `Pid`, `PPid`, `Uid`, `Gid`, and `Groups` are available for proc-reading LTP cases.
- `Gid:` layout uses real/effective/saved/effective values to match the existing single-group model.

### `examples/shell/src/uspace/fd_table.rs`

Functions/fields changed:
- `FileEntry::status_flags`
- `FdTable::fcntl`
- `fcntl_status_flags`
- `fcntl_setfl_flags`
- `open_candidates`

Expected behavior:
- Routes `/proc/*/status` opens to the synthetic proc file while preserving read-only and directory error behavior.
- Normal files now retain POSIX status flags from `open(2)` for `F_GETFL` and update allowed status bits for `F_SETFL`, preserving access mode bits.
- `F_GETFL` returns stored file status flags including access mode and supported status bits.
- `F_SETFL` updates supported settable status bits while preserving the access mode, fixing `fcntl01` without hardcoding a case result or converting failures to skips.

### `examples/shell/src/uspace/memory_map.rs`

Function changed:
- `MemoryMap::mmap`

Expected behavior:
- Writable `MAP_SHARED | MAP_ANONYMOUS` mappings are populated and registered as shared memory, so forked parent/child views share the same physical page.
- This fixes real POSIX-visible shared anonymous mmap semantics needed by `getpid02`.

## Discovery and remaining classified failures

`syscalls-basic-plus` discovery identified low-risk passes and remaining blockers. Before promotion, failures clustered as:
- `access02` / `lseek02`: symlink and readlink/mkfifo-related metadata gaps.
- `access04`: tmpfs mount/metadata behavior gap.
- `dup03`: closed-fd/EBADF handling details need deeper syscall triage.
- `fcntl01`: fixed by normal-file `F_GETFL`/`F_SETFL` status flag tracking, then promoted.
- `pipe02`: signal/pipe wait behavior.
- `getpid02`: fixed by shared anonymous mmap population/registration, then promoted.

Hard blockers intentionally not on the first promotion path:
- LA full-LTP `crash01` / `InstructionNotExist` / user trap-to-signal path.
- RV full-LTP OOM/free-frame exhaustion around CVE cases.

## Stage validation evidence

Targeted LTP-only stable promotion:
- RV: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-ltp-only-stable-30.log`
- LA: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-ltp-only-stable-30.log`

Stage result:
- RV stable-30: PASS LTP CASE 60, FAIL LTP CASE 0, LTP timeout 0, ENOSYS 0, panic/trap 0, internal TCONF 6.
- LA stable-30: PASS LTP CASE 60, FAIL LTP CASE 0, LTP timeout 0, ENOSYS 0, panic/trap 0, internal TCONF 6.

Targeted `getpid02` after shared anonymous mmap fix:
- RV: `docs/ltp-score-improvement-2026-05-21/rv-ltp-only-getpid02-after-shared-anon-mmap.log`
- LA: `docs/ltp-score-improvement-2026-05-21/la-ltp-only-getpid02-after-shared-anon-mmap.log`
- Result: both musl/glibc passed with `child getpid() == parent fork()`.

Targeted `fcntl01` after normal-file `F_GETFL`/`F_SETFL` status flag tracking:
- RV: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=fcntl01 LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-ltp-only-fcntl01-after-status-flags.log`
- LA: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=fcntl01 LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-ltp-only-fcntl01-after-status-flags.log`
- Result: both musl/glibc passed on RV and LA, with timeout 0, ENOSYS 0, panic/trap 0. Summaries: `rv-fcntl01-status-flags-summary.txt`, `la-fcntl01-status-flags-summary.txt`.

Targeted LTP-only stable-31 promotion after adding `fcntl01`:
- RV: `flock -n /tmp/oskernel-ltp-run.lock /tmp/run-rv-stable31.sh` equivalent command captured in `docs/ltp-score-improvement-2026-05-21/rv-ltp-only-stable-31.log` with status file `rv-ltp-only-stable-31.status` = 0.
- LA: `flock -n /tmp/oskernel-ltp-run.lock /tmp/run-la-stable31.sh` equivalent command captured in `docs/ltp-score-improvement-2026-05-21/la-ltp-only-stable-31.log` with status file `la-ltp-only-stable-31.status` = 0.
- RV stable-31 summary: PASS LTP CASE 62, FAIL LTP CASE 0, LTP timeout 0, ENOSYS 0, panic/trap 0, internal TCONF 6.
- LA stable-31 summary: PASS LTP CASE 62, FAIL LTP CASE 0, LTP timeout 0, ENOSYS 0, panic/trap 0, internal TCONF 6.
- Summary files: `rv-stable-31-summary.txt`, `la-stable-31-summary.txt`.
- Operator note: earlier duplicate/aborted RV stable-31 attempts were discarded and the final evidence above comes from a clean single-instance rerun.

Targeted runtime column validation after adding `LTP CASE RUNTIME` emission:
- RV: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=getpid02 LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-ltp-only-getpid02-runtime.log`
- Summary: `python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/rv-ltp-only-getpid02-runtime.log | tee docs/ltp-score-improvement-2026-05-21/rv-getpid02-runtime-summary.txt`
- Result: PASS LTP CASE 2, FAIL LTP CASE 0, LTP timeout 0, ENOSYS 0, panic/trap 0; runtime matrix records `rv:musl:getpid02` = 701 ms and `rv:glibc:getpid02` = 863 ms.
- Note: the final full-output summaries below also include runtime columns from `LTP CASE RUNTIME` emission.

## Final validation evidence

Final full evaluator validation was rerun after the stable-31 promotion and after the runtime/matrix reporting updates. Duplicate or aborted LA attempts from earlier shell/session issues were excluded; see `discarded-duplicate-la-runs.note`. The final accepted evidence is from clean single-instance LA/RV runs with no residual QEMU/eval processes.

Commands run:

```sh
cargo fmt --all -- --check
./run-eval.sh la 2>&1 | tee output_la.md
./run-eval.sh 2>&1 | tee output_rv.md
python3 scripts/ltp_summary.py output_la.md | tee docs/ltp-score-improvement-2026-05-21/final-output-la-summary.txt
python3 scripts/ltp_summary.py output_rv.md | tee docs/ltp-score-improvement-2026-05-21/final-output-rv-summary.txt
```

Exit status:
- `cargo fmt --all -- --check`: 0 (`final-cargo-fmt.status`)
- `./run-eval.sh la`: 0 (`final-run-eval-la.status`)
- `./run-eval.sh`: 0 (`final-run-eval-rv.status`)
- `python3 scripts/ltp_summary.py output_la.md`: 0
- `python3 scripts/ltp_summary.py output_rv.md`: 0

Final LTP summary after stable-31 promotion:

| Output | Suite | Passed | Failed | LTP internal TFAIL | LTP internal TBROK | LTP internal TCONF | LTP timeout | ENOSYS | panic/trap |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `output_la.md` | ltp-musl | 31 | 0 | 0 | 0 | 3 | 0 | 0 | 0 |
| `output_la.md` | ltp-glibc | 31 | 0 | 0 | 0 | 3 | 0 | 0 | 0 |
| `output_rv.md` | ltp-musl | 31 | 0 | 0 | 0 | 3 | 0 | 0 | 0 |
| `output_rv.md` | ltp-glibc | 31 | 0 | 0 | 0 | 3 | 0 | 0 | 0 |

Final per-output totals from `scripts/ltp_summary.py`:
- LA: PASS LTP CASE 62, FAIL LTP CASE 0, internal TFAIL/TBROK/TCONF 6 (`TCONF`: 6), LTP case timeout category 0, ENOSYS 0, panic/trap 0.
- RV: PASS LTP CASE 62, FAIL LTP CASE 0, internal TFAIL/TBROK/TCONF 6 (`TCONF`: 6), LTP case timeout category 0, ENOSYS 0, panic/trap 0.

Final category summary:
- LA: `pass_clean` 58, `pass_with_tconf` 4 (`chdir01`, `read02` on musl/glibc), `fail_wrapper` 0, `internal_tfail` 0, `internal_tbrok` 0, `timeout` 0, `enosys` 0, `panic_trap` 0, `unknown` 0.
- RV: `pass_clean` 58, `pass_with_tconf` 4 (`chdir01`, `read02` on musl/glibc), `fail_wrapper` 0, `internal_tfail` 0, `internal_tbrok` 0, `timeout` 0, `enosys` 0, `panic_trap` 0, `unknown` 0.

Full-output note: the summary tool also reports 10 timeout matches per full evaluator output from non-LTP groups such as libctest/lmbench/cyclictest/iozone/unixbench. These are not LTP case timeouts and were not counted as LTP PASS.

## User-visible / ABI-visible behavior changes

- The default evaluator LTP batch changed from `core` 16 cases to documented `stable` 31 cases.
- New runner configuration sources exist for LTP case selection, per-case timeout, and test-group selection.
- `/proc/self/status` and `/proc/<pid>/status` are now readable synthetic proc files.
- Writable `MAP_SHARED | MAP_ANONYMOUS` mappings now behave as shared across fork in the shell uspace runtime.
- No syscall number or ABI struct layout changes were introduced.

## Remaining risks and next batch recommendations

Recommended next fixes:
1. Implement symlink/readlink/mkfifo semantics enough to unlock `access02` and `lseek02`.
2. Investigate `dup03` closed-fd/error-path expectations.
3. Improve remaining fcntl status-flag semantics beyond the `fcntl01` surface, especially full append/nonblock side effects if later cases require them.
4. Improve pipe + signal/wait behavior for `pipe02`.
5. Improve tmpfs mount/metadata behavior for `access04` and the `fs-basic` lane.
6. Keep LA `crash01` and RV full-LTP OOM as separate hard-blocker tracks; do not block stable-batch promotion on them.

## Post-promotion G102 TCONF follow-up: chdir01 symlink-loop semantics

After the stable-31 promotion, the remaining internal `TCONF` total was 6 per arch: `chdir01` symlink-loop coverage plus `read02` O_DIRECT-on-tmpfs coverage. This follow-up removed the real `chdir01` semantic gap by adding non-case-specific `symlinkat`/`readlinkat`/nofollow-stat/open-path symlink handling in the shell uspace boundary.

Changed behavior:
- `symlinkat(target, newdirfd, linkpath)` records a symlink at the resolved link path and returns `EEXIST` for an existing file/link.
- path open/chdir resolution follows recorded symlinks and returns `ELOOP` after a bounded loop depth.
- `readlinkat` returns the recorded target for recorded symlinks.
- `newfstatat(..., AT_SYMLINK_NOFOLLOW)` reports `S_IFLNK | 0777` and target length for recorded symlinks.
- `unlinkat` removes recorded symlinks before falling back to normal file/directory removal.

Targeted `chdir01` evidence:

```sh
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chdir01 LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-ltp-only-chdir01-symlink.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chdir01 LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-ltp-only-chdir01-symlink.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/rv-ltp-only-chdir01-symlink.log | tee docs/ltp-score-improvement-2026-05-21/rv-chdir01-symlink-summary.txt
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/la-ltp-only-chdir01-symlink.log | tee docs/ltp-score-improvement-2026-05-21/la-chdir01-symlink-summary.txt
```

Results:
- RV: PASS LTP CASE 2, FAIL 0, internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
- LA: PASS LTP CASE 2, FAIL 0, internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
- `chdir01` now reports `TPASS: ... chdir("symloop") returned correct value: ELOOP (40)` for root and nobody on both musl/glibc.

Stable-31 rerun after the symlink fix:

```sh
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-ltp-only-stable-31-after-symlink.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-ltp-only-stable-31-after-symlink.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/rv-ltp-only-stable-31-after-symlink.log | tee docs/ltp-score-improvement-2026-05-21/rv-stable-31-after-symlink-summary.txt
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/la-ltp-only-stable-31-after-symlink.log | tee docs/ltp-score-improvement-2026-05-21/la-stable-31-after-symlink-summary.txt
```

Results:
- RV stable-31: PASS LTP CASE 62, FAIL 0, internal `TCONF` 4, timeout 0, ENOSYS 0, panic/trap 0.
- LA stable-31: PASS LTP CASE 62, FAIL 0, internal `TCONF` 4, timeout 0, ENOSYS 0, panic/trap 0.
- Remaining internal `TCONF` is only `read02` on musl/glibc for both architectures.

`read02` TCONF classification:
- This is not currently treated as a real semantic gap for this stable batch. The LTP 20240524 `read02.c` source explicitly does not open `O_DIRECT` when the test filesystem is tmpfs and emits `TCONF: O_DIRECT not supported on tmpfs filesystem` for those two subcases.
- This runner intentionally forces LTP to tmpfs with `LTP_FORCE_SINGLE_FS_TYPE=tmpfs` and `LTP_DEV_FS_TYPE=tmpfs`, so the remaining `read02` TCONF is the expected test-configuration result rather than a hidden failure.
- Host Linux spot-check: opening a file on `/dev/shm` with `O_DIRECT` returned `EINVAL`, matching tmpfs not supporting the O_DIRECT subcase in this environment.
- Source reference used for classification: `https://raw.githubusercontent.com/linux-test-project/ltp/20240524/testcases/kernel/syscalls/read/read02.c`.

Follow-up validation still required before the next final delivery because code changed after the earlier full final gate:
- `cargo fmt --all -- --check`
- full `./run-eval.sh la` and `./run-eval.sh` with refreshed `output_la.md` / `output_rv.md`
- final `scripts/ltp_summary.py` summaries for both refreshed outputs

## G104 filesystem metadata batch promotion: stable-44

This pass expanded the stable LTP batch from 31 to 44 cases by promoting only fs-basic cases that passed on both LA and RV, musl and glibc, without timeout, ENOSYS, panic/trap, TFAIL, or TBROK. No failing fs-basic cases were converted to PASS or SKIP.

Promoted fs-basic cases:

- `creat01`
- `creat03`
- `open02`
- `open03`
- `stat02`
- `lstat01`
- `chmod01`
- `fchmod01`
- `rmdir01`
- `symlink01`
- `readlink01`
- `ftruncate01`
- `umask01`

Implementation behavior added for this promotion:

- `UserProcess::umask` now tracks a real per-process umask, inherited across fork.
- `sys_umask()` returns the previous mask and installs the new permission mask.
- `UserProcess::apply_umask()` applies the current mask to recorded file/directory creation metadata.
- `open(..., O_CREAT, mode)` and `mkdirat(..., mode)` record modes after umask application.
- `open(..., O_NOATIME)` now returns `EPERM` for an unprivileged non-owner instead of silently succeeding.
- `sys_symlinkat()` now rejects target/link path lengths at Linux `PATH_MAX` with `ENAMETOOLONG`.
- `sys_statx(..., AT_SYMLINK_NOFOLLOW)` reports recorded symlink metadata instead of following the link.

Targeted G104 fixes were verified with:

```sh
OSCOMP_TEST_GROUPS=ltp LTP_CASES=umask01,open02,symlink01 LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-g104-umask-open-symlink.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=umask01,open02,symlink01 LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-g104-umask-open-symlink.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=symlink01,lstat01 LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-g104-statx-symlink-lstat.log
```

Targeted results:

- RV `umask01,open02,symlink01`: PASS LTP CASE 6, FAIL 0, internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
- LA `umask01,open02`: passed in the first targeted run; `symlink01` exposed the LA/glibc `statx(..., AT_SYMLINK_NOFOLLOW)` gap and was not counted as fixed until the follow-up.
- LA `symlink01,lstat01` after the `statx` fix: PASS LTP CASE 4, FAIL 0, internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.

Full fs-basic discovery after G104 fixes:

```sh
OSCOMP_TEST_GROUPS=ltp LTP_CASES=batch:fs-basic LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-fs-basic-g104-after-fixes.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=batch:fs-basic LTP_CASE_TIMEOUT_SECS=8 make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-fs-basic-g104-after-fixes.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/rv-fs-basic-g104-after-fixes.log | tee docs/ltp-score-improvement-2026-05-21/rv-fs-basic-g104-after-fixes-summary.txt
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/la-fs-basic-g104-after-fixes.log | tee docs/ltp-score-improvement-2026-05-21/la-fs-basic-g104-after-fixes-summary.txt
```

Results on both RV and LA:

- PASS LTP CASE 26, FAIL LTP CASE 8.
- Suite summaries: `ltp-musl: 13 passed, 4 failed`; `ltp-glibc: 13 passed, 4 failed`.
- Clean promoted cases: 13 per libc.
- Not promoted: `mkdir02`, `link02`, `unlink05`, `rename01`.
- Remaining failure classes:
  - `mkdir02`: musl group lookup TBROK; glibc SGID/GID inheritance TFAIL.
  - `link02`: hard-link syscall path still returns ENOSYS.
  - `unlink05`: `mkfifo`/FIFO creation path still returns ENOSYS after the basic unlink(file) subcase passes.
  - `rename01`: test-device acquisition fails with ENOSPC/TBROK in the current evaluator disk setup.
- Timeout: 0.
- Panic/trap: 0.

Stable-44 promotion gate:

```sh
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=8 make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/rv-stable-44-g104.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$PWD/sdcard-la.img" 2>&1 | tee docs/ltp-score-improvement-2026-05-21/la-stable-44-g104-default-timeout.log
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/rv-stable-44-g104.log | tee docs/ltp-score-improvement-2026-05-21/rv-stable-44-g104-summary.txt
python3 scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-21/la-stable-44-g104-default-timeout.log | tee docs/ltp-score-improvement-2026-05-21/la-stable-44-g104-default-timeout-summary.txt
```

Promotion-gate results:

| Arch | Timeout setting | PASS LTP CASE | FAIL LTP CASE | musl | glibc | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap |
| --- | --- | ---: | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| RV | 8s explicit | 88 | 0 | 44/0 | 44/0 | 0 | 0 | 4 | 0 | 0 | 0 |
| LA | default 10s | 88 | 0 | 44/0 | 44/0 | 0 | 0 | 4 | 0 | 0 | 0 |

A stricter exploratory LA run with `LTP_CASE_TIMEOUT_SECS=8` produced one timeout in `ltp-glibc:access01` at 8563 ms and was not used as a PASS artifact. The default runner timeout is 10 seconds (`LTP_CASE_TIMEOUT_SECS` default), and the default-timeout LA promotion run completed `access01` in 4810 ms with no timeout. Timeout remains counted as fail/timeout, not PASS.

Stable batch delta:

- Previous stable: 31 cases, 62 case executions per architecture.
- New stable: 44 cases, 88 case executions per architecture.
- Net promoted cases: +13 stable LTP cases (+26 musl/glibc executions per architecture).
- Remaining stable-batch internal `TCONF`: only `read02` O_DIRECT-on-tmpfs on musl/glibc for each architecture.

