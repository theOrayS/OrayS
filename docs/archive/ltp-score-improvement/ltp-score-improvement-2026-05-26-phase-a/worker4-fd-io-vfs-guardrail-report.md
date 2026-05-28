# Worker 4 report: FD/IO + VFS create/remove guardrail lane

Date: 2026-05-26
Team: `ltp-stable383-to-stab-2374dbd5`
Worker: `worker-4`
Task: `4` — FD/IO/sendfile/pread/pwrite/iovec plus VFS small create/remove and guardrails

## Scope and constraints honored

- Report-only task; no code change required by task JSON.
- Did **not** mutate `.omx/ultragoal`.
- Did **not** edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Did **not** run QEMU/eval or touch the default sdcard/qcow2 path.
- Did **not** edit LTP source or add case-name hardcoding/fake PASS logic.

## Live baseline and inventory

- Live stable list: `383` total / `383` unique / `0` duplicates.
- None of the 39 lane-focus cases are currently in `LTP_STABLE_CASES`.
- Phase-c RV common-not-stable inventory contains all 39 focus binaries and the relevant resource helpers:
  - `creat04/06/07/08/09` and `creat07_child`: `docs/ltp-score-improvement-2026-05-25-phase-c/raw/sdcard-rv-common-not-stable-ltp-bins.txt:194-199`
  - `mkdir03/04/09`: `.../sdcard-rv-common-not-stable-ltp-bins.txt:1061-1063`
  - `open06/07/10/11/12/open12_child/14`: `.../sdcard-rv-common-not-stable-ltp-bins.txt:1266-1272`
  - `pread02/_64`, `preadv01/02/03`: `.../sdcard-rv-common-not-stable-ltp-bins.txt:1362-1367`
  - `pwrite02/_64`, `pwrite04/_64`, `pwritev01/02/03`: `.../sdcard-rv-common-not-stable-ltp-bins.txt:1415-1422`
  - `rmdir02/03`: `.../sdcard-rv-common-not-stable-ltp-bins.txt:1481-1482`
  - `sendfile02-05/_64`: `.../sdcard-rv-common-not-stable-ltp-bins.txt:1567-1574`
  - `unlink07/08/09`: `.../sdcard-rv-common-not-stable-ltp-bins.txt:2360-2362`

## Runner and parser guardrails

- Runner case injection supports file/inline `LTP_CASES` and should be used for scout batches instead of editing stable: `examples/shell/src/cmd.rs:600-649`.
- Runner emits line-start `RUN LTP CASE` markers and wrapper status lines around every case: `examples/shell/src/cmd.rs:1483-1488`.
- Current remote-compatible success wire remains `FAIL LTP CASE <case> : 0` plus `Pass!`; non-zero and timeout paths keep real failure markers: `examples/shell/src/cmd.rs:1533-1563`.
- Parser counts internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS/not implemented, and panic/trap separately and must remain the promotion gate: `scripts/ltp_summary.py:32-39`, `scripts/ltp_summary.py:208-223`, `scripts/ltp_summary.py:305-337`.
- Timeout evidence overrides a zero-status wrapper result and must never be counted as clean PASS: `scripts/ltp_summary.py:235-246`.

## Source expectation and local-readiness matrix

Official LTP source was inspected read-only via sparse checkout at `/tmp/ltp-src-worker4`, commit `34c00cd`.  The risk column is an implementation/readiness estimate from repo-local source inspection and older parsed summaries; it is **not** promotion evidence.

| Case set | Source expectation | Repo-local readiness signal | Scout priority |
| --- | --- | --- | --- |
| `pread02`, `pread02_64` | `pread()` must fail with `EINVAL` for negative offset and `EISDIR` for directory fd (`pread02.c:31-43`). | `sys_pread64` does not reject negative offsets and maps non-file fd to `EBADF`, not `EISDIR`: `examples/shell/src/uspace/fd_table.rs:211-224`. | **Blocker-first**; expect failure until errno semantics are fixed. |
| `pwrite02`, `pwrite02_64` | `pwrite()` must fail for negative offset, invalid fd, read-only fd, and bad buffer (`pwrite02.c:39-72`). | Invalid/read-only/EFAULT likely covered, but negative offset is cast through `usize` without signed rejection in `sys_pwrite64`: `fd_table.rs:237-249`. | **Blocker-first**; narrow fix candidate for negative offset. |
| `pwrite04`, `pwrite04_64` | Linux `pwrite()` on an `O_APPEND` fd appends despite explicit offset; LTP expects size growth from 2K to 3K (`pwrite04.c:9-63`). | Normal write honors `O_APPEND`, but `write_file_at()` ignores `O_APPEND`: `fd_table.rs:1849-1861`, `fd_table.rs:853-882`. | **Blocker-first**; likely one semantic gap. |
| `preadv01`, `pwritev01` | Basic vector read/write, content checks, and file offset must not change (`preadv01.c:8-80`, `pwritev01.c:8-84`). | Non-`_64` variants are not stable, but `_64` siblings `preadv01_64`, `pwritev01_64` are stable (`cmd.rs:118-119`) and vector loops preserve fd offset by using `*_file_at`: `fd_table.rs:333-383`, `fd_table.rs:386-432`. | **High-value scout**; likely among best FD candidates. |
| `preadv02`, `pwritev02` | Negative iov length/count/offset, bad user pointer, invalid fd, wrong access mode, directory or pipe must return expected errno (`preadv02.c:8-82`, `pwritev02.c:8-78`). | iovec validation and ESPIPE/EISDIR paths exist: `user_memory.rs:166-191`, `fd_table.rs:1055-1073`, `fd_table.rs:853-870`. | **High-value scout**; use RV batch before LA. |
| `preadv03`, `pwritev03` | O_DIRECT on mounted all-filesystems path; needs block device `BLKSSZGET`, aligned buffers, and mount device (`preadv03.c:92-141`, `pwritev03.c:91-136`). | `sys_ioctl` handles `BLKGETSIZE64` but not `BLKSSZGET`; likely setup TBROK/ENOTTY: `fd_table.rs:684-708`. | **Skip for easy-first** unless leader wants device/ioctl work. |
| `sendfile02-05`, `_64` | `sendfile()` offset update/no input fd seek, EBADF, EFAULT bad offset pointer, EINVAL negative offset (`sendfile02.c:10-78`, `sendfile03.c:8-52`, `sendfile04.c:9-61`, `sendfile05.c:9-40`). | No `sendfile` syscall dispatch or helper found in `examples/shell/src/uspace`; unmatched syscalls return ENOSYS: `syscall_dispatch.rs:101-471`. | **Blocked** until real `sys_sendfile` is implemented. |
| `open06` | FIFO `open(O_NONBLOCK | O_WRONLY)` with no reader must fail `ENXIO` (`open06.c:20-25`). | FIFO metadata exists, but FIFO open currently creates endpoints and returns a write end instead of no-reader `ENXIO`: `fd_table.rs:2055-2076`. | **Blocker-first**; small FIFO semantics gap. |
| `open07` | `O_NOFOLLOW` on final symlink must fail `ELOOP`; non-symlink pass path must open (`open07.c:24-73`). | Symlink registry and `O_NOFOLLOW`/ELOOP paths exist: `metadata.rs:96-118`, `fd_table.rs:1948-1957`. Older matrix had `open07` TBROK in a prior run: `docs/ltp-score-improvement-2026-05-22-phase-d/candidate-matrix.md:124`. | **Scout after symlink sanity**; do not assume clean from code shape. |
| `open10`, `creat08`, `creat09` | New files in setgid dirs must inherit parent gid and handle setgid stripping correctly (`open10.c:70-123`, `creat08.c:72-124`, `creat09.c:47-111`). | New file owner currently uses process fs gid, not parent setgid inheritance: `fd_table.rs:2124-2139`; older `open10` evidence was mixed TFAIL/TBROK: `candidate-matrix.md:127`. | **Skip for easy-first** unless doing setgid inheritance repair. |
| `open11` | Broad open matrix: regular/dir/hardlink/symlink/device, `O_DIRECTORY`, `O_CREAT`, truncation behavior; needs `mknod` char device (`open11.c:7-40`, `open11.c:287-309`). | `mknodat` only supports regular/FIFO, so char device setup will fail; older matrix shows ENOSYS/TBROK: `fd_table.rs:1223-1256`, `candidate-matrix.md:128`. | **Blocked** by mknod/device semantics. |
| `open12` | `O_APPEND`, `O_NOATIME`, `O_CLOEXEC`, `O_LARGEFILE`; needs `open12_child`, fork/exec, strictatime mount, >4GB lseek/write (`open12.c:7-195`). | `open12_child` exists; `O_CLOEXEC` and `O_APPEND` have support, but large sparse write and strictatime are risky: `fd_table.rs:909-915`, `fd_table.rs:1849-1861`. Older matrix had TBROK: `candidate-matrix.md:129`. | **Medium/high risk**; run only after simpler FD cases. |
| `open14` | `O_TMPFILE`, link via `/proc/self/fd`, 100 tmpfile descriptors, mode checks (`open14.c:8-14`, `open14.c:35-46`, `open14.c:57-217`). | No clear O_TMPFILE support in open path; likely TBROK/TCONF depending errno; older matrix had TBROK: `candidate-matrix.md:131`. | **Skip for easy-first**. |
| `creat04` | Non-root creat in protected dir/path must fail `EACCES` (`creat04.c:32-75`). | Open-create path skips parent write permission check when creating a new file: `fd_table.rs:2104-2118`. | **Blocker-first**; likely permission gap. |
| `creat06` | Error matrix: EISDIR, ENOENT, ENOTDIR, EFAULT, EACCES, ELOOP, EROFS (`creat06.c:55-135`). | EFAULT/ELOOP/dir errors may partly work, but EACCES and EROFS are suspect; no evidence clean. | **Medium/high risk**; single-case scout after permission audit. |
| `creat07` | Creating/truncating an executing helper must fail (ETXTBSY-style behavior) or TCONF if unsupported (`creat07.c:20-64`). | `creat07_child` exists, but no obvious exec-file write exclusion in fd/open path. | **Skip for easy-first** unless process/file-exec tracking is added. |
| `mkdir03` | Negative mkdir errno matrix including EFAULT, ENAMETOOLONG, EEXIST, ENOENT, ENOTDIR, ELOOP, EROFS (`mkdir03.c:31-111`). | `mkdirat` is a thin `create_dir` wrapper with limited parent permission/rofs/symlink-loop semantics: `fd_table.rs:1206-1220`. | **Medium/high risk**; scout only after quick errno sanity. |
| `mkdir04` | Non-root mkdir under restrictive permissions must fail `EACCES` (`mkdir04.c:19-50`). | `mkdirat` does not check parent write/search permission before `create_dir`: `fd_table.rs:1206-1220`. | **Blocker-first**; likely permission gap. |
| `mkdir09` | Multi-thread create/remove stress on mounted all-filesystems path (`mkdir09.c:23-142`). | Requires pthread scheduling + mount-device/all-fs behavior; no direct clean evidence. | **Medium risk**; not first batch. |
| `rmdir02` | Error matrix for ENOENT/ENOTDIR/EFAULT/ELOOP/EROFS/EINVAL for `.` (`rmdir02.c:39-107`). | `unlinkat(AT_REMOVEDIR)` is thin `remove_dir`; symlink-loop/rofs/current-dir semantics are not clearly complete: `fd_table.rs:1261-1288`. | **Medium/high risk**. |
| `rmdir03` | Sticky/no-execute/access-denied behavior must return EACCES or EPERM for non-root (`rmdir03.c:35-90`). | `unlinkat` lacks explicit parent permission/sticky checks: `fd_table.rs:1261-1288`. | **Blocker-first**; likely permission gap. |
| `unlink07` | Simple negative path errors: missing path, empty path, bad address, component not directory (`unlink07.c:27-65`). | `read_cstr` EFAULT and empty-path ENOENT exist; likely the lightest unlink candidate: `user_memory.rs:373-419`, `fd_table.rs:1261-1276`. | **High-value scout**. |
| `unlink08` | EACCES for unwritable/unsearchable dirs and EISDIR for deleting dir as root/non-root (`unlink08.c:24-79`). | EISDIR likely through `remove_file`, but EACCES parent permission checks are missing in `unlinkat`: `fd_table.rs:1261-1288`. | **Blocker-first**. |
| `unlink09` | Immutable/append-only inode flags via `FS_IOC_GETFLAGS/SETFLAGS`; expected EPERM on unlink (`unlink09.c:24-111`). | `sys_ioctl` has no FS_IOC flag support; likely ENOTTY/TCONF/TBROK, not clean: `fd_table.rs:684-708`. | **Skip for easy-first**. |

## Recommended leader-serialized scout batches

Use file injection, not stable-list edits. Suggested RV-first sequence:

### Batch A — likely useful FD/vector candidates

`preadv01,preadv02,pwritev01,pwritev02,unlink07`

Rationale: source expectations align best with existing user-iovec and path-error support; `_64` siblings for preadv/pwritev already serve as regression sentinels in stable.

### Batch B — narrow blocker confirmation/fix candidates

`pread02,pread02_64,pwrite02,pwrite02_64,pwrite04,pwrite04_64,open06,creat04,mkdir04,rmdir03,unlink08`

Rationale: likely fail for one or two clear semantics gaps each (negative offset, `O_APPEND` with positioned write, FIFO no-reader ENXIO, parent permission checks). Good for targeted repair if leader wants FD/VFS fixes.

### Batch C — broad VFS/open risk batch after B

`open07,open10,open11,open12,open14,creat06,creat07,creat08,creat09,mkdir03,mkdir09,rmdir02,unlink09`

Rationale: device/mount/setgid/O_TMPFILE/rofs/sticky/exec-file semantics are broader. Do not spend easy-first budget here unless B unlocks supporting semantics.

### Batch D — sendfile only after syscall implementation

`sendfile02,sendfile02_64,sendfile03,sendfile03_64,sendfile04,sendfile04_64,sendfile05,sendfile05_64`

Rationale: no `sendfile` dispatch exists, so current scout would mainly prove ENOSYS. Implement real `sys_sendfile` first, then run this batch.

## Command templates for leader serial runs

```bash
cat >/tmp/ltp_cases.txt <<'CASES'
preadv01
preadv02
pwritev01
pwritev02
unlink07
CASES
OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:/tmp/ltp_cases.txt LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv
python3 -B scripts/ltp_summary.py <rv-log> > docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker4-batch-a-rv-summary.txt
```

Promotion filter for each batch: no wrapper FAIL, no internal `TFAIL/TBROK/TCONF`, no timeout, no ENOSYS/not implemented, no panic/trap.  Known `read02` TCONF is irrelevant unless stable aggregate is included; if included, keep it transparent and separate.

## Guardrail notes

- Do not treat wrapper exit alone as promotion evidence; always parse with `scripts/ltp_summary.py`.
- Do not convert `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap, or setup failure into PASS/SKIP.
- Keep marker lines at line start; line-prefix corruption can break remote scorer compatibility.
- Sendfile is a real missing-syscall blocker, not a candidate for fake PASS.
- VFS permission cases are especially vulnerable to false positives if parent write/search permission is not checked; require raw log + parser summary before any stable-list edit.
- Worker runtime evidence remains discovery-only because no isolated QEMU was used; promotion gates belong to the leader and must be serial.

## Verification

- Inbox/task protocol: read worker inbox and `task-4.json`; sent startup ACK; claimed task 4 with claim token `788f9d01-8037-4eb4-936f-9403cc56f054`.
- Stable count check: Python parsed `examples/shell/src/cmd.rs::LTP_STABLE_CASES` as `383` total / `383` unique; focus-in-stable `[]`.
- Inventory check: Python checked 39/39 focus cases present in `sdcard-rv-common-not-stable-ltp-bins.txt`; resource helpers `open12_child` and `creat07_child` present.
- Source check: official LTP sparse checkout `/tmp/ltp-src-worker4`, commit `34c00cd`, inspected syscall source files for all focus families.
- Repo implementation check: `rg`/`nl` inspected `examples/shell/src/uspace/fd_table.rs`, `syscall_dispatch.rs`, `user_memory.rs`, `metadata.rs`, plus runner/parser files.
- QEMU/eval: intentionally not run per worker lane constraint.
- Subagent spawn evidence:
  - `019e64a6-43db-7470-b9e2-e2a872230d00` (`Linnaeus`) completed read-only repo-local runner/inventory/prior-report scout.
  - `019e64a6-2293-7980-b8ca-62d52eca0e7a` (`Bernoulli`) was spawned for read-only LTP-source expectation review but timed out; closed without file changes. Local official-source inspection above substitutes for that unfinished sidecar.
