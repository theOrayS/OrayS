# Worker 4 report: FD / pipe / iovec / fcntl lane after stable413

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Worker: `worker-4` / task `4`
Scope from leader mailbox: FD/pipe/iovec/fcntl-adjacent candidates; assess shared offset, `O_APPEND`, negative-offset errno, pipe capacity/FIONREAD, blocking/yield, and SIGPIPE. Do not edit `.omx/ultragoal` or final `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; do not run concurrent default QEMU.

## Report paths

- This report: `docs/ltp-score-improvement-2026-05-27-phase-a/worker4-fd-pipe-iovec-fcntl-report.md`
- No raw QEMU logs were produced by this worker. Leader-owned targeted commands are listed below for serialized execution.

## Guardrails honored

- Claimed task 4 before work and delivered the leader mailbox message before acting on it.
- Did not edit `.omx/ultragoal`.
- Did not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Did not run `./run-eval.sh` or any default QEMU path.
- Did not add case-name hardcoding, fake PASS/SKIP logic, or parser/marker changes.

## Live lane baseline

Live parse of `examples/shell/src/cmd.rs::LTP_STABLE_CASES` in this worktree:

```text
total 413 unique 413 dups 0
```

Stable membership relevant to this lane:

| Family | Already stable | Still not stable / not a promoted test |
| --- | --- | --- |
| `readv` / `writev` | `readv01`, `readv02`, `writev01`, `writev02`, `writev05`, `writev06`, `writev07` | `writev03` |
| `preadv` / `pwritev` | `preadv01`, `preadv01_64`, `preadv02`, `preadv02_64`, `preadv201`, `preadv201_64`, `preadv202`, `preadv202_64`, `pwritev01`, `pwritev01_64`, `pwritev02`, `pwritev02_64`, `pwritev201`, `pwritev201_64`, `pwritev202`, `pwritev202_64` | `preadv03`, `preadv03_64`, `preadv203`, `preadv203_64`, `pwritev03`, `pwritev03_64` |
| `pipe` / `pipe2` | `pipe01`, `pipe03`, `pipe04`, `pipe05`, `pipe06`, `pipe08`, `pipe09`, `pipe10`, `pipe11`, `pipe12`, `pipe13`, `pipe14`, `pipe2_01`, `pipe2_02`, `pipe2_04` | `pipe02`, `pipe07`, `pipe15`; local LTP source has no standalone `pipe2_03` test, and inventory shows `pipe2_02_child` as a helper, not a promotion case |
| light `fcntl` | `fcntl01/02/03/04/05/08/09/10/12/13/16/23/29` plus matching `_64` variants where present | `fcntl07`, `fcntl07_64`; lock-heavy `fcntl11/14/15/17+` remain out |
| `sendfile` adjacent | `sendfile02/03/04/05/06/08` and `_64` for `02/03/04/05/06/08` | `sendfile07`, `sendfile07_64`, `sendfile09`, `sendfile09_64` |

## Source-readiness assessment

| Area requested by leader | Repo evidence | Assessment |
| --- | --- | --- |
| Shared file offsets | `FileEntry` uses a shared `offset` lock; normal read/write update `file_entry_read()` / `file_entry_write()`, and positioned vector I/O uses `read_file_at_into_fd()` / `write_file_at()` with a local offset. | Existing stable `preadv*/pwritev*` rows are plausible regression sentinels. No new patch proposed without a fresh failing case. |
| Negative-offset errno | `sys_pread64()`, `sys_pwrite64()`, `sys_preadv()`, and `sys_pwritev()` cast to signed and return `EINVAL` for negative offsets; `preadv2/pwritev2` accept `-1` as current-offset and reject `< -1`. | Previously identified blocker is already fixed/promoted in stable413; no additional action for stable460. |
| `O_APPEND` with positioned writes | `write_file_at()` now chooses EOF when the fd has `O_APPEND`; normal `file_entry_write()` also appends at EOF. | Stable413 coverage already includes `pwrite04/_64` and positioned-vector neighbors. Keep as sentinels. |
| Pipe capacity / `F_GETPIPE_SZ` | `PIPE_BUF_SIZE` is fixed at 4096; `PipeEndpoint::capacity()` returns that value; `fcntl(F_GETPIPE_SZ)` returns the pipe capacity and `F_SETPIPE_SZ` rejects growth above current capacity. | Enough for many light pipe tests; `pipe15` likely remains setup/TCONF-prone because it needs `/proc/sys/fs/pipe-user-pages-soft` and a high `RLIMIT_NOFILE`, not just this capacity hook. |
| FIONREAD / readable bytes | `PipeEndpoint::available_read()` exposes ring readable bytes; prior stable coverage includes pipe family cases and parser summaries. | Good regression signal, but no new FIONREAD-specific candidate from this lane. |
| Pipe blocking/yield | Blocking read/write paths yield with `set_syscall_wait_blocked(true)` and return `EAGAIN` for nonblocking no-space/no-data. | Good enough for a leader scout of `pipe07`; `pipe15` remains high-risk due soft-limit/procfs setup. |
| SIGPIPE/EPIPE | Pipe write checks reader count, raises SIGPIPE, and returns `EPIPE` if no reader and nothing was written. | `pipe08` is already stable, so this path is covered; no new patch. |
| `F_SETFD` / `FD_CLOEXEC` | `F_SETFD` records `FD_CLOEXEC`; `sys_execve()` calls `close_cloexec()` after loading the new image. Applies to regular fds and pipe fds; FIFO fds are represented through the same fd table. | `fcntl07` / `_64` are the best light-fcntl scout candidates because current code shape matches the test expectation; needs leader serialized proof. |
| Advisory locks | `fcntl_getlk()` always reports `F_UNLCK`; `fcntl_setlk()` validates but does not maintain lock state. | Do not promote lock-heavy `fcntl11/14/15/17+`; a real lock table would be a broader patch. |
| Socket-backed `sendfile07` | `sys_sendfile()` preflights input and output, copies through a userspace buffer, and returns partial count or the first error. `sendfile07` needs a full nonblocking Unix datagram socket and expects `EAGAIN`. | Candidate is diagnostic/medium-risk. It may work if socket write returns `EAGAIN`, but setup can be expensive and should be leader-run after `fcntl07/pipe07`. |
| Large-file `sendfile09` | LTP requires 5G free space and 1G transfer windows. | Defer for stable460 easy-first; high runtime/disk cost. |
| O_DIRECT `preadv03/pwritev03` | LTP needs mounted all-filesystems and `BLKSSZGET`; earlier lane already flagged device/ioctl setup risk. | Continue to avoid unless evidence changes. |
| `splice` / `tee` / `vmsplice` / `copy_file_range` | No syscall dispatch found in `examples/shell/src/uspace/syscall_dispatch.rs` for these names. | Not part of light FD promotion; implementing them is broader than task-4 report scope. |

## Candidate status table

| Candidate(s) | Status | Why | Leader serialized command |
| --- | --- | --- | --- |
| `fcntl07`, `fcntl07_64` | **Best light scout** | Tests only close-on-exec on regular file, pipe ends, and FIFO; repo has `F_SETFD` + `close_cloexec()` support and runner self-exec support from prior phases. | Include in first RV scout, then LA only if RV-clean. |
| `pipe07` | **Good light scout** | Verifies opening pipes until fd exhaustion returns `EMFILE`; `insert_min_with_flags()` enforces `FD_TABLE_LIMIT` and rolls back second pipe fd on failure. | Include in first RV scout; watch `/proc/self/fd` and `getdtablesize()` assumptions. |
| `sendfile07`, `sendfile07_64` | **Diagnostic after light scout** | Exercises nonblocking socket `EAGAIN`; current `sendfile()` can propagate output-write errors but socket fill/setup can be slow. | Scout after `fcntl07/pipe07`, not before. |
| `writev03` | **Blocked / diagnostic only** | LTP requires `min_cpus=2`, mount-device/all-filesystems, and long fuzzy race; prior evidence showed TCONF/fail-wrapper rather than a clean data-path bug. `sys_writev()` reads user iov data before fd write, which is the safe shape. | Do not promote unless fresh four-way evidence is clean; if rerun, use diagnostic batch only. |
| `pipe15` | **Blocked / high setup risk** | Needs `/proc/sys/fs/pipe-user-pages-soft`, many pipes, and high `RLIMIT_NOFILE`; current default nofile is 1024 and procfs soft-limit behavior is uncertain. | Do not spend first-wave stable460 budget here. |
| `fcntl11/14/15/17+` | **Blocked by missing advisory lock state** | Current `F_GETLK`/`F_SETLK` does not model inter-process lock conflicts. | Requires design/implementation task, not report-only promotion. |
| `preadv03`, `preadv03_64`, `pwritev03`, `pwritev03_64` | **Defer** | O_DIRECT/all-filesystems/`BLKSSZGET` setup remains high risk. | Avoid unless leader has fresh evidence. |
| `sendfile09`, `sendfile09_64` | **Defer** | Requires 5G tmp space and large sendfile transfers. | Avoid in stable460 easy-first path. |

## Recommended leader-run scout batches

### Batch W4-A: light fcntl + fd exhaustion

```bash
cat >/tmp/worker4-fd-light.txt <<'CASES'
fcntl07
fcntl07_64
pipe07
CASES
OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:/tmp/worker4-fd-light.txt LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv 2>&1 | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-light-rv-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-light-rv-001.log | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-light-rv-001-summary.txt
```

If and only if RV has wrapper PASS and zero internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap, run LA:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:/tmp/worker4-fd-light.txt LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la 2>&1 | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-light-la-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-light-la-001.log | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-light-la-001-summary.txt
```

### Batch W4-B: socket-backed sendfile diagnostic

```bash
cat >/tmp/worker4-sendfile-socket.txt <<'CASES'
sendfile07
sendfile07_64
CASES
OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:/tmp/worker4-sendfile-socket.txt LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv 2>&1 | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-sendfile-socket-rv-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-sendfile-socket-rv-001.log | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-sendfile-socket-rv-001-summary.txt
```

Run LA only for any RV-clean subset.

### Batch W4-C: diagnostics only, not promotion-first

```bash
cat >/tmp/worker4-fd-diagnostics.txt <<'CASES'
writev03
pipe15
CASES
OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:/tmp/worker4-fd-diagnostics.txt LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh rv 2>&1 | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-diagnostics-rv-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-diagnostics-rv-001.log | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker4-fd-diagnostics-rv-001-summary.txt
```

Do not promote from this batch unless both RV and LA are clean across both libcs with zero internal caveats.

## Exact worker verification commands and outputs

### Stable count / membership

Command:

```bash
python3 - <<'PY'
from pathlib import Path
import re
s=Path('examples/shell/src/cmd.rs').read_text()
m=re.search(r'const LTP_STABLE_CASES: &[^=]*= &\[(.*?)\];', s, re.S)
cases=re.findall(r'"([^"]+)"', m.group(1))
print('total',len(cases),'unique',len(set(cases)),'dups',len(cases)-len(set(cases)))
PY
```

Output:

```text
total 413 unique 413 dups 0
```

### Lane membership parser

Command:

```bash
python3 - <<'PY'
from pathlib import Path
import re
stable_s=Path('examples/shell/src/cmd.rs').read_text()
stable=set(re.findall(r'"([^"]+)"', re.search(r'const LTP_STABLE_CASES: &[^=]*= &\[(.*?)\];',stable_s,re.S).group(1)))
for c in ['fcntl07','fcntl07_64','pipe07','pipe15','writev03','sendfile07','sendfile07_64','preadv03','pwritev03']:
    print(c, 'stable' if c in stable else 'not')
PY
```

Output:

```text
fcntl07 not
fcntl07_64 not
pipe07 not
pipe15 not
writev03 not
sendfile07 not
sendfile07_64 not
preadv03 not
pwritev03 not
```

### Static source checks

Commands run:

```bash
rg -n "sys_(readv|writev|preadv|pwritev|pipe2|fcntl|sendfile)|F_GETPIPE_SZ|F_SETPIPE_SZ|FD_CLOEXEC|close_cloexec|PIPE_BUF_SIZE|raise_sigpipe" examples/shell/src/uspace -g '*.rs'
rg -n "splice|tee|vmsplice|copy_file_range|__NR_splice|__NR_tee|__NR_vmsplice|__NR_copy_file_range" examples/shell/src/uspace/syscall_dispatch.rs examples/shell/src/uspace -g '*.rs'
```

Output summary:

```text
PASS: fd/vector/sendfile/fcntl/pipe symbols found in fd_table.rs, fd_pipe.rs, process_lifecycle.rs.
PASS: no splice/tee/vmsplice/copy_file_range dispatch symbols found; rg exited 1 because there were no matches.
```

### Parser regression test

Command:

```bash
python3 -B scripts/test_ltp_summary.py
```

Output is recorded in the final task result after running the command.

### Formatting / diff checks

Commands:

```bash
git diff --check
```

Output is recorded in the final task result after running the command.

## Delegation evidence

- Subagents spawned: 2 read-only sidecars.
- Subagent model: `gpt-5.4-mini`.
- Serial searches before spawn: 3 state/repo/memory read phases (inbox+skill, task/claim state, memory/current-plan scan).
- Findings integrated:
  - `019e68b5-dcbe-7ea2-b741-02185e51b44a` (`Singer`) confirmed live stable413 evidence and ranked old Tier-1 candidates; after live membership refresh, its `preadv/pwritev` picks were treated as stale because those cases are now stable, while its parser-gate and scout-command recommendations were reused.
  - `019e68b6-1e38-75c1-94b1-67f6ef6a4469` (`Parfit`) highlighted FD positioned-write and VFS fixes, but live stable413 already contains the positioned-write cases and VFS belongs to worker-2; its verification-command shape and de-prioritization guidance were folded into the W4 batches above.

## Worker conclusion

Current worker-4 lane does not justify a speculative code patch. The best stable413-adjacent, low-risk subset for leader-owned serial scouting is `fcntl07`, `fcntl07_64`, and `pipe07`; `sendfile07/_64` is a secondary diagnostic; `writev03`, `pipe15`, lock-heavy `fcntl*`, O_DIRECT vector I/O, large-file sendfile, and unimplemented splice-style syscalls should not consume first-wave stable460 promotion budget without fresh parser-backed evidence.

## Post-write verification results

| Check | Result | Output / caveat |
| --- | --- | --- |
| `python3 -B scripts/test_ltp_summary.py` | PASS | `Ran 4 tests in 0.004s` / `OK` |
| `git diff --check` | PASS | no whitespace errors reported |
| `cargo fmt --all -- --check` | BLOCKED | `cargo metadata` workspace-root mismatch for `vendor/rust-fatfs`: current worktree package believes the workspace root is `/root/oskernel2026-orays/Cargo.toml`. This is the known OMX worktree workspace caveat; no Rust source was edited. |
| `rustfmt --edition 2021 --check examples/shell/src/uspace/fd_table.rs examples/shell/src/uspace/fd_pipe.rs examples/shell/src/uspace/process_lifecycle.rs examples/shell/src/uspace/user_memory.rs` | BLOCKED | pre-existing import-order formatting drift in inspected Rust files; not introduced by this report-only lane. |
| `timeout 180s cargo check --manifest-path examples/shell/Cargo.toml --target riscv64gc-unknown-none-elf --features 'uspace auto-run-tests'` | TIMEOUT | still compiling dependencies at 180s (`axconfig-gen` stage); no code changes were made in this worker lane, so this is recorded as a bounded verification gap rather than a source regression. |
| End-to-end LTP/QEMU | NOT RUN | explicitly avoided because leader instructed no concurrent default QEMU; commands above are ready for leader-owned serialized runs. |
