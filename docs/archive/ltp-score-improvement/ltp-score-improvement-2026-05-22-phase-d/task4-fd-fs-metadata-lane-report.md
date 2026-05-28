# Task 4 FD + FS/METADATA Lane Report

Date: 2026-05-22
Worker: worker-3
Task: FD+FS/METADATA lane (`fd/pipe/open/access/fcntl` plus `statfs/statvfs/link/rename/truncate`)

## Scope and guardrails

- Implemented only a narrow real-semantics fix in the FD/pipe surface.
- Did **not** edit `LTP_STABLE_CASES` and did **not** mutate leader-owned `.omx/ultragoal` state.
- Saved evidence under `docs/ltp-score-improvement-2026-05-22-phase-d/` in this worktree.

## Code changes

### Pipe endpoint status and peer tracking

Files:
- `examples/shell/src/uspace/fd_pipe.rs`
- `examples/shell/src/uspace/fd_table.rs`

Implemented:
- `pipe2(..., O_NONBLOCK)` now accepts `O_NONBLOCK` in addition to `O_CLOEXEC` instead of returning `EINVAL` for a Linux-valid flag.
- Pipe read/write endpoints now preserve per-end status flags, including `O_NONBLOCK`, across `F_GETFL`/`F_SETFL`.
- Pipe peer-closed detection now tracks read-end and write-end reference counts separately, instead of treating any remaining endpoint clone as an open peer.
- Nonblocking empty pipe reads return `EAGAIN` instead of yielding forever.
- Nonblocking full pipe writes return `EAGAIN` if no bytes were written, or a partial byte count after progress.
- Writes with no remaining readers return `EPIPE` and queue `SIGPIPE` to the current user thread.

Why this is real semantics:
- Linux `pipe2` supports `O_CLOEXEC | O_NONBLOCK`; rejecting `O_NONBLOCK` blocks `pipe06`-style coverage.
- `fcntl(F_GETFL/F_SETFL)` on pipe file descriptors is expected to observe and mutate `O_NONBLOCK` per open file description/end.
- Broken-pipe behavior is observable by LTP pipe/fcntl cases and is not a wrapper or case-name shortcut.

## Candidate matrix

| Case/group | Current evidence | Recommendation |
| --- | --- | --- |
| `pipe01` | Phase-c clean on RV+LA glibc/musl. | Keep as baseline smoke only. |
| `pipe02` | Phase-c/subagent evidence: RV fail/TFAIL; described as child signal/pipe kill semantics. | Rerun after this fix; peer-closed/SIGPIPE semantics may help. |
| `pipe06` | Phase-c/phase-d priority candidate; likely `pipe2(O_NONBLOCK)`-adjacent. | Highest-priority targeted rerun after this fix. |
| `pipe07` | Phase-c wave candidate; no fresh clean evidence. | Rerun after `pipe06`, grouped with pipe/fcntl. |
| `fcntl01`-`fcntl03` | Phase-c clean on RV+LA glibc/musl. | Keep as baseline smoke. |
| `fcntl04+` | Phase-d broader candidate pool; no complete fresh matrix in this worktree. | Do not promote without targeted evidence; use as diagnostic only. |
| `access01`, `access03` | Phase-c clean on RV+LA glibc/musl. | Keep stable/baseline. |
| `access02`, `access04`, `faccessat02` | Phase-c/subagent evidence says failing/TBROK/TFAIL. | Needs separate access/tmpfs/setup investigation; not fixed by pipe patch. |
| `open01`-`open03` | Phase-c clean on RV+LA glibc/musl. | Keep stable/baseline. |
| `open04`-`open06`, `openat02` | Phase-c wave candidates; no clean evidence. | Rerun in FD/open/access batch; no direct promotion. |
| `link01`, `link02`, `link03`, `linkat01`, `linkat02` | Phase-c/subagent evidence shows likely fail/unknown. | Needs FS metadata/link semantics investigation. |
| `rename01`, `rename02`, `renameat01`, `renameat02` | Phase-c/subagent evidence shows likely fail/unknown. | Needs FS rename semantics investigation. |
| `statfs01`, `statfs02`, `fstatfs01`, `fstatfs02`, `statvfs01`, `fstatvfs01` | Phase-c wave candidates; subagent flags statfs family as high-risk because generic/synthetic values can mask bugs. | Group together; promote only with strict RV+LA summary evidence. |
| `truncate02` | Phase-c worker evidence says confirmed clean on RV+LA glibc/musl. | Good baseline; no new promotion by this worker. |
| `truncate01`, `ftruncate02`, `ftruncate03` | Phase-c/phase-d blockers or candidates. | Rerun with FS metadata batch; not addressed by pipe patch. |

## Targeted batch recommendation

Create a narrow pipe/fcntl diagnostic batch first:

```text
pipe01
pipe02
pipe06
pipe07
fcntl01
fcntl02
fcntl03
```

Then, if pipe batch is clean, expand to the FD/open/access neighbors without promotion:

```text
access02
access04
faccessat02
open04
open05
open06
openat02
close08
close09
lseek02
lseek03
```

Keep FS metadata separate to avoid conflating pipe/fcntl changes with path/mount/statfs behavior:

```text
link01
link02
link03
linkat01
linkat02
rename01
rename02
renameat01
renameat02
statfs01
statfs02
fstatfs01
fstatfs02
statvfs01
fstatvfs01
truncate01
truncate02
ftruncate02
ftruncate03
```

## Verification evidence

- PASS: `cargo fmt -p arceos-shell -- --check`
  - status: `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-cargo-fmt-arceos-shell-check.status`
  - log: `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-cargo-fmt-arceos-shell-check.log`
- PASS: `python3 -m unittest scripts/test_ltp_summary.py`
  - status: `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-ltp-summary-unittest.status`
  - log: `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-ltp-summary-unittest.log`
- FAIL (configuration-only): `cargo check -p arceos-shell --features 'uspace auto-run-tests' --target riscv64gc-unknown-none-elf --offline`
  - status file was not produced because `set -e` exited on cargo failure; log saved at `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-cargo-check-rv.log`.
  - failure: missing platform crate `axplat_riscv64_qemu_virt` from direct cargo invocation, matching prior phase-b notes; this command does not prove source failure.
- FAIL (timeout/config shape): `CARGO_NET_OFFLINE=true make A=examples/shell ARCH=riscv64`
  - status/log: `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-make-shell-rv.status`, `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-make-shell-rv.log`
  - result: exit 2 because the repository `all` target continued into the LA build and was terminated by the tool timeout after RV had progressed; this is not a scoped source diagnostic.
- PASS: `CARGO_NET_OFFLINE=true make kernel-rv`
  - status: `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-make-kernel-rv.status`
  - log: `docs/ltp-score-improvement-2026-05-22-phase-d/raw/task4-make-kernel-rv.log`
  - result: release `arceos-shell` RV kernel built successfully; only pre-existing vendored `smoltcp`/`axnet` warnings.

## Subagent evidence

- Subagents spawned: 3 attempts.
  - `019e5042-b09a-7652-a6c4-a762cd97c060` / Hypatia: phase-c and phase-d evidence matrix, completed.
  - `019e5042-d019-7503-9522-4ccdabe3cea4` / James: implementation-surface prompt was too large and errored with context-window failure; no findings integrated.
  - `019e5046-e475-7a73-92fb-71d69a3d5822` / Nietzsche: shortened read-only implementation-surface inspection, completed.
- Findings integrated:
  - Phase-c evidence confirms clean baseline cases (`pipe01`, `fcntl01`-`03`, `open01`-`03`, `access01/03`, `truncate02`) and failing/unknown lane candidates.
  - Implementation-surface review confirmed pipe nonblocking/broken-pipe/fcntl status as the narrow FD-pipe semantics to address, while statfs/access/truncate require separate investigations.
- Serial searches before spawn: 2.

## Notes / risks

- This patch does not claim any PASS for `pipe02`, `pipe06`, or `pipe07`; targeted LTP validation is still required before leader-owned promotion decisions.
- `cargo fmt --all -- --check` is not usable in this linked worktree because Cargo descends into excluded vendored `rust-fatfs` and reports a workspace-root mismatch with `/root/oskernel2026-orays/Cargo.toml`; package-scoped formatting was used instead.
- Direct target `cargo check` without the repo Makefile platform setup is also not a valid source diagnostic for this repo.
