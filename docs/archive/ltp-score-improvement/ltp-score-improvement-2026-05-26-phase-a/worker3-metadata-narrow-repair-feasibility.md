# Worker 3 metadata narrow-repair feasibility

Date: 2026-05-26
Team: `ltp-stable383-to-stab-2374dbd5`
Task: `task-7`
Mode: report-only follow-up. No QEMU/run-eval was started, no source was edited, no final `LTP_STABLE_CASES` edit was made, and `.omx/ultragoal` was not touched.

## Scope

Follow-up from `worker3-metadata-statfs-getdents-report.md` for three repair hypotheses:

1. `getdents01` / `getdents02`: legacy `getdents` coverage versus current `getdents64` implementation quality.
2. `fstat02` / `fstat02_64`: whether prior `ENOSYS` rows point at an old-stat-family syscall gap.
3. `statfs` / `fstatfs` / `statvfs`: synthetic `statfs` field mismatches.

Local contest LTP C sources were searched but not found in this worktree or bounded `/root` scan; this report therefore uses repository source, `linux-raw-sys` target ABI constants, sdcard inventory lists, and prior parsed LTP summaries.

## Findings summary

- There is no clear easy-first one-line patch that can honestly promote this lane today.
- `getdents01` / `getdents02` may have a **one-file semantic improvement candidate** in `fd_table.rs`, but the known failures include internal `TFAIL`, `TCONF`, and glibc `ENOSYS`, so it is not promotion-ready without source-level LTP expectation review and fresh serial runtime proof.
- The old-stat-family hypothesis for `fstat02/_64` is weak on the target arches: `linux-raw-sys` defines only `newfstatat=79` and `fstat=80` for both `riscv64` and `loongarch64`; there is no target-arch `stat`, `lstat`, or legacy `getdents` constant to wire through safely.
- `statfs` / `fstatfs` / `statvfs` failures look like real semantic/setup mismatches, not missing syscall coverage: prior rows show `TBROK` or `TFAIL` without `ENOSYS`.

## Definitely not easy-first today

| Case(s) | Prior evidence | Why not easy-first |
| --- | --- | --- |
| `fstat02`, `fstat02_64` | Prior RV rows show wrapper `FAIL`, `TBROK=1`, `ENOSYS=1` for both libc variants. | The current target ABI already has `__NR_fstat`; the `ENOSYS` source needs syscall tracing/test-source inspection before any patch. A blind dispatch alias risks hardcoding a wrong ABI path. |
| `fstatfs01`, `fstatfs01_64` | Prior RV rows show wrapper `FAIL`, `TBROK=1`, no `ENOSYS`. | This is not a missing-syscall symptom; likely setup/semantic expectations around mounted filesystem or `statfs` fields. |
| `statfs01`, `statfs01_64` | `statfs01` repeatedly failed with `TBROK=1`; `statfs01_64` has inventory presence but no fresh clean row. | `statfs01_64` is unknown, and `statfs01` has a real blocker. Do not infer clean from binary presence. |
| `statfs03`, `statfs03_64` | Prior rows show wrapper `FAIL`, `TFAIL=1`, no `ENOSYS`. | Real semantic mismatch; not an easy syscall stub. |
| `statvfs01` | Prior rows show wrapper `FAIL`, `TBROK=1`, no `ENOSYS`. | libc `statvfs` derives from statfs-like data; must clear underlying statfs expectations first. |
| `getcwd03` | Prior rows show wrapper `FAIL`, `TBROK=1`. | Likely setup/path/search semantics; not part of the requested narrow patch and not clean. |
| `getcwd04` | Prior rows show wrapper `FAIL`, `TCONF=1`. | `TCONF` is transparent and cannot count as clean. |
| `getdents01`, `getdents02` | Prior rows show wrapper `FAIL`; glibc rows include `ENOSYS=1`; both libc rows include internal `TFAIL` and `TCONF`. | Even if a directory-entry improvement lands, it must prove no internal `TFAIL/TCONF` remains. |

## Source inspection notes

### Target syscall constants

`linux-raw-sys-0.12.1` target constants for both `riscv64` and `loongarch64` include:

- `__NR_statfs = 43`
- `__NR_fstatfs = 44`
- `__NR_getdents64 = 61`
- `__NR_newfstatat = 79`
- `__NR_fstat = 80`
- `__NR_statx = 291`

The same target files did **not** define `__NR_getdents`, `__NR_stat`, or `__NR_lstat`. That makes a generic legacy `getdents` or old-stat-family dispatch patch unsafe unless a failing binary trace proves it uses a specific arch-private number or trampoline.

### Current repo implementation surfaces

- `examples/shell/src/uspace/syscall_dispatch.rs` already dispatches `statfs`, `fstatfs`, `newfstatat`, `statx`, `fstat`, and `getdents64`.
- `examples/shell/src/uspace/metadata.rs:661-681` returns synthetic `statfs` data from allocator page counts and filesystem magic inferred from path/fd kind.
- `examples/shell/src/uspace/metadata.rs:816-824` routes `fstat` through `stat_with_recorded_path`.
- `examples/shell/src/uspace/metadata.rs:1043-1074` handles `statfs` / `fstatfs` and writes `general::statfs`.
- `examples/shell/src/uspace/fd_table.rs:435-447` exposes `sys_getdents64` only.
- `examples/shell/src/uspace/fd_table.rs:1016-1052` packs `linux_dirent64` records, currently using synthetic `d_ino = idx + 1` and `d_off = 0` for every entry.

## One-file / low-risk patch candidates

### Candidate A: improve `getdents64` record offsets in `fd_table.rs`

Potential patch surface: `examples/shell/src/uspace/fd_table.rs::FdTable::getdents64`.

Possible change:

- Set `d_off` to a monotonically advancing cookie, e.g. the byte offset after the current record or another stable per-entry cookie, instead of always `0`.
- Preserve `d_reclen` alignment and existing `d_type`/name copy-out behavior.

Feasibility: **one-file, low-to-medium risk**, but not enough to call `getdents01/02` easy-first.

Why it may help:

- `d_off = 0` for all entries is suspicious for strict `getdents` tests and can explain part of the internal `TFAIL` surface.

Why it may not be enough:

- Glibc rows still showed `ENOSYS=1`; on RV64/LA64 there is no `__NR_getdents` in `linux-raw-sys`, so a legacy-getdents gap may remain outside this one-file semantic fix.
- Prior rows also include `TCONF`, which cannot be hidden or counted as clean.
- `d_ino = idx + 1` is also synthetic; strict tests may require stable nonzero inode semantics across calls.

Regression risks:

- Directory iteration state is held by the underlying `dir.read_dir`; changing cookies might affect repeated reads, seek-like expectations, or shell/test directory listings.
- If `d_off` is interpreted by libc as a seek cookie, a byte-offset cookie may still be insufficient unless the filesystem supports seekdir semantics.

### Candidate B: improve synthetic `statfs` fields in `metadata.rs`

Potential patch surface: `examples/shell/src/uspace/metadata.rs::generic_statfs` and `statfs_type_for_path`.

Possible change:

- Keep `f_bsize == f_frsize == 4096`, nonzero block/file counts, and `f_bfree >= f_bavail` invariants explicit and stable.
- Audit `f_type` and `f_fsid` for `/`, `/tmp`, `/proc`, `/sys`, `/dev`, pipes, and sockets.

Feasibility: **one-file, medium risk**.

Why it may help:

- `statfs03/_64` failures are `TFAIL=1`, consistent with a real field expectation mismatch.
- `statvfs01` likely inherits from `statfs` data through libc.

Why it is not easy-first:

- `fstatfs01` / `statfs01` / `statvfs01` currently fail with `TBROK`, so setup/mount/path assumptions may be failing before simple field checks.
- Existing stable sentinels include `statfs02`, `statfs02_64`, `fstatfs02`, `fstatfs02_64`, and `statvfs02`; a broad synthetic-field change can regress already-stable cases.

Regression risks:

- Allocator-backed counts vary with runtime memory pressure; making them more synthetic/stable could improve tests but reduce truthful resource reporting.
- Filesystem magic changes affect `/proc`, `/dev`, pipes/sockets, and any tests that inspect `f_type`.

### Not recommended: add legacy `getdents` or old `stat/lstat` aliases blindly

Potential patch surface would be `syscall_dispatch.rs`, but this is **not currently low-risk**.

Reason:

- The target `linux-raw-sys` constants for `riscv64`/`loongarch64` do not expose `__NR_getdents`, `__NR_stat`, or `__NR_lstat`.
- Adding numeric catch-alls without a failing syscall trace risks accepting the wrong ABI and hiding a real userspace/test-source issue.
- The correct next step is a leader-owned trace or instrumented serial run that captures the exact syscall number causing `ENOSYS`.

## Exact leader validation commands if patched

Run these only from the leader/serialized QEMU lane, not from concurrent workers.

### Static/build checks

```bash
git diff --check
cargo fmt --all -- --check
make A=examples/shell ARCH=riscv64
```

If the patch touches submission/evaluator packaging, add:

```bash
make all
CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all
```

### Targeted RV scout after `getdents64` patch

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=getdents01,getdents02 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv | tee docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-getdents-rv-afterpatch-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-getdents-rv-afterpatch-001.log \
  > docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-getdents-rv-afterpatch-001-summary.txt
```

Only if both RV libc rows are clean, run LA:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=getdents01,getdents02 \
LTP_CASE_TIMEOUT_SECS=90 \
./run-eval.sh la | tee docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-getdents-la-afterpatch-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-getdents-la-afterpatch-001.log \
  > docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-getdents-la-afterpatch-001-summary.txt
```

### Targeted RV scout after `statfs` patch

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=fstatfs01,fstatfs01_64,statfs01,statfs01_64,statfs03,statfs03_64,statvfs01 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv | tee docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-statfs-rv-afterpatch-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-statfs-rv-afterpatch-001.log \
  > docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-statfs-rv-afterpatch-001-summary.txt
```

Only if RV is clean, run LA with the RV-clean subset only.

### Targeted diagnostic for `fstat02/_64`

Before patching, run a diagnostic build/trace or temporary leader-only instrumentation to capture the actual unsupported syscall number. Then run:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=fstat02,fstat02_64 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv | tee docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-fstat-rv-diagnostic-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-fstat-rv-diagnostic-001.log \
  > docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker3-fstat-rv-diagnostic-001-summary.txt
```

Do not promote until RV+LA x musl+glibc are clean and the `ENOSYS` counter is zero.

## Regression risks to keep visible

- `read02` remains the known transparent `TCONF`; no new `TCONF` from this lane is acceptable.
- `statfs` changes can regress already-stable `statfs02`, `statfs02_64`, `fstatfs02`, `fstatfs02_64`, and `statvfs02`.
- `fstat` changes can regress already-stable `fstat03`, `fstat03_64`, and `fstatat01`.
- `getdents64` changes can alter libc directory iteration behavior for shell startup, resource staging, and unrelated LTP setup code.
- Any patch that merely turns `ENOSYS`, `TBROK`, `TFAIL`, `TCONF`, timeout, panic/trap, or wrapper nonzero into SKIP/PASS violates the promotion contract.

## Recommendation

For the easy-first stable413 push, keep these cases out of the promotion set until a leader-owned repair/diagnostic lane proves them clean. If one narrow source patch is worth trying, start with the `fd_table.rs` `getdents64` offset/cookie improvement because it is one-file and does not touch already-stable `statfs`/`fstat` sentinels, but treat it as a diagnostic patch until fresh parser summaries show zero internal failures on RV and LA.

## Verification

| Check | Result |
| --- | --- |
| Worker protocol | Mailbox message `09f9378b-c25e-4aae-83da-06b183ebd6bf` marked delivered; task 7 claimed by `worker-3` with claim token. |
| Source availability | `find` found no local contest LTP C sources for focus cases in the bounded worktree/root scan. |
| Source audit | Inspected `syscall_dispatch.rs`, `metadata.rs`, `fd_table.rs`, and `linux-raw-sys` RV/LA constants/ABI structs. |
| Evidence audit | Reused prior parsed summaries from task 3 and re-parsed the focus-case rows from docs; all focus candidates still have prior blocker rows or no clean proof. |
| Runtime policy | No QEMU/run-eval was started in this worker lane. |
| Scope ownership | No edits to `.omx/ultragoal` or `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; this report is the only intended artifact. |

## Subagent evidence

Subagent skip reason: task 7 was a narrow report-only follow-up with a short claim lease and no code-edit ownership; serial inspection was safer and sufficient, avoided duplicate searches, and kept all shared source files unmodified.
