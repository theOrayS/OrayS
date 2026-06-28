# Worker 3 metadata/statfs/getdents lane report

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Task: `task-3`
Mode: narrow worktree fix + documentation/report. Promotion remains leader-owned.

## Scope and guardrails

- Claimed task 3 before lane work and processed the leader lane update from mailbox message `2738274c-e066-4a51-9947-ba56944c8f44`.
- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` re-counted at this worker start: `413 total / 413 unique / 0 duplicates`.
- Focus cases remain absent from stable: `getdents01`, `getdents02`, `fstat02`, `fstat02_64`, `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01`, `getcwd03`, `getcwd04`.
- Stable sentinels remain present and leader-owned: `fstat03`, `fstatat01`, `statfs02`, `fstatfs02`, `statvfs02`, `getcwd01`, `getcwd02`.
- Did **not** edit `.omx/ultragoal` or final `LTP_STABLE_CASES`.
- Did **not** run default QEMU or claim promotion evidence. Runtime validation below is build/static/parser-test only; targeted RV/LA LTP proof must be leader-serialized.

## Source of truth chain

Promotion truth for this campaign is:

1. live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` for current stable count and duplicates;
2. `scripts/ltp_summary.py` for wrapper status plus internal `TFAIL`/`TBROK`/`TCONF`, timeout, `ENOSYS`, and panic/trap classification;
3. leader-owned RV+LA x musl+glibc runs for promotion and final gates.

Wrapper success alone is not a promotion signal. `read02` remains known transparent `pass_with_tconf` only; no new TCONF from this lane is clean.

## Source inspection

Local contest LTP C sources for the focus cases were not present in this worktree or bounded `/root` search. This report therefore uses repository syscall surfaces, `linux-raw-sys` target constants, prior parsed summaries, and the current source implementation.

Inspected surfaces:

- `examples/shell/src/uspace/fd_table.rs`
  - `FdTable::getdents64` packs `linux_dirent64` records.
  - `DirectoryEntry` wraps `axfs::fops::Directory`, which maintains a directory entry cursor internally.
  - `open_dir_entry` creates directory FDs.
- `examples/shell/src/uspace/metadata.rs`
  - `generic_statfs`, `sys_statfs`, `sys_fstatfs`, `sys_newfstatat`, and `path_inode` provide the stat/statfs surfaces.
- `examples/shell/src/uspace/syscall_dispatch.rs`
  - Dispatches `__NR_statfs`, `__NR_fstatfs`, `__NR_newfstatat`, `__NR_fstat`, and `__NR_getdents64`.
- `linux-raw-sys`/`libc` target constants
  - RV64 and LA64 expose `getdents64 = 61`; no safe target `__NR_getdents`, `__NR_stat`, or `__NR_lstat` alias was found for a blind legacy dispatch patch.

## Narrow fix applied

`examples/shell/src/uspace/fd_table.rs` now improves `getdents64` dirent metadata without changing runner markers or stable promotion state:

- adds a per-directory-FD `next_dirent_cookie`;
- rejects too-small `getdents64` buffers with `EINVAL` before advancing the directory cursor;
- emits stable nonzero inode values with `path_inode(normalize_path(dir.path, entry_name))` instead of per-buffer `idx + 1`;
- emits monotonically advancing `d_off` cookies instead of `0` for every record.

This is intentionally only a low-risk semantic repair candidate for `getdents01/02`. It is **not** evidence that either case is clean; leader must run fresh parser-backed RV and LA targeted gates.

## Candidate status table

| Candidate(s) | Current status | Rationale | Required next proof |
| --- | --- | --- | --- |
| `getdents01`, `getdents02` | Repair candidate patched; not promotion-clean | Prior rows had wrapper FAIL plus internal `TFAIL`/`TCONF` and glibc `ENOSYS`. The one-file fix improves `getdents64` `d_ino`/`d_off`/small-buffer behavior but deliberately avoids blind legacy `getdents` aliases. | Leader serial RV targeted run for both cases, then LA only if RV musl+glibc are clean; parse with `scripts/ltp_summary.py` and require zero internal blockers. |
| `fstat02`, `fstat02_64` | Blocked/diagnostic only | Prior rows showed `TBROK=1` and `ENOSYS=1`, but target ABIs already expose `__NR_fstat`; no safe old-stat alias exists without capturing the exact failing syscall. | Capture unsupported syscall number in leader-owned diagnostic run before editing. |
| `fstatfs01`, `fstatfs01_64` | Blocked semantic lane | Prior rows show wrapper FAIL/TBROK without ENOSYS, so this is not a missing-dispatch fix. | Compare LTP statfs expectations against `generic_statfs`; run RV targeted statfs batch after any narrow field/setup repair. |
| `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64` | Blocked semantic lane | `statfs01` has prior TBROK; `statfs03` has prior TFAIL. Broad statfs changes risk stable sentinels. | Guard `statfs02`/`statfs02_64`; use fresh RV then LA parser summaries after any statfs field repair. |
| `statvfs01` | Blocked semantic lane | libc `statvfs` derives from statfs-like data; prior rows show TBROK, not ENOSYS. | Only retest after statfs/fstatfs behavior is fixed. |
| `getcwd03` | Blocked setup/path lane | Prior rows show TBROK; likely path/chdir/search-permission setup rather than a tiny `getcwd` syscall issue. | Inspect LTP setup path and run leader serial targeted proof after path semantics are understood. |
| `getcwd04` | Transparent non-clean TCONF lane | Prior rows show TCONF. | Keep disclosed; never count as clean unless fresh evidence proves no TCONF. |

## Leader validation commands for this lane

Do not run these concurrently from worker panes that share default QEMU/sdcard state.

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=getdents01,getdents02 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker3-getdents-rv-afterpatch-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker3-getdents-rv-afterpatch-001.log \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker3-getdents-rv-afterpatch-001-summary.txt
```

Only if RV is clean:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=getdents01,getdents02 \
LTP_CASE_TIMEOUT_SECS=90 \
./run-eval.sh la | tee docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker3-getdents-la-afterpatch-001.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker3-getdents-la-afterpatch-001.log \
  > docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker3-getdents-la-afterpatch-001-summary.txt
```

Recommended sentinel batch after any positive getdents result:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=fstat03,fstatat01,statfs02,fstatfs02,statvfs02,getcwd01,getcwd02 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv
python3 -B scripts/ltp_summary.py <rv-sentinel-log>
```

## Verification performed

| Check | Command | Result |
| --- | --- | --- |
| Worker protocol | `omx team api send-message ... ACK`, `omx team api claim-task ... task_id=3`, `mailbox-mark-delivered ... 2738274c-e066-4a51-9947-ba56944c8f44` | PASS; task 3 in progress under `worker-3`; leader lane update acknowledged. |
| Live stable count | Python regex parse of `examples/shell/src/cmd.rs::LTP_STABLE_CASES` | PASS: `LTP_STABLE_CASES total=413 unique=413 duplicates=0`; focus cases absent; sentinels present. |
| Local LTP source search | `find /root/oskernel2026-orays ... -name getdents01.c ...`; `find /root -maxdepth 5 ...` | PASS/INFO: no local contest LTP C sources found, so no unsupported source expectation claim is made. |
| Parser tests | `python3 -m unittest scripts/test_ltp_summary.py` | PASS: `Ran 4 tests ... OK`. |
| Parser CLI smoke | `python3 -B scripts/ltp_summary.py --help` | PASS: help reports wrapper status plus `TFAIL/TBROK/TCONF`, timeout, ENOSYS semantics. |
| Whitespace | `git diff --check` | PASS. |
| Targeted Rust formatting | `rustfmt --edition 2021 --check examples/shell/src/uspace/fd_table.rs` | PASS. |
| Full cargo fmt | `cargo fmt --all -- --check` | FAIL/ENV: cargo metadata rejects the OMX worktree path for `vendor/rust-fatfs` because it believes it belongs to `/root/oskernel2026-orays/Cargo.toml`; targeted rustfmt above passed for the modified Rust file. |
| Build/typecheck | `make A=examples/shell ARCH=riscv64` | PASS: built RV `kernel-rv` and LA `kernel-la`; only pre-existing vendor warnings from `smoltcp`/`axnet` and checked-in ctypes warning for LA. |
| Runtime/QEMU | none | Not run by worker per leader instruction: no concurrent default QEMU. |

Ignored build outputs (`build/`, `kernel-rv`, `kernel-la`, caches) were left untracked/ignored and must not be committed.

## Subagent evidence

Subagents spawned: 1 (`Bacon`, `019e68b5-e6f4-7440-afcf-f743e985d8a5`) for read-only review probe.
Subagent model: `gpt-5.4-mini` requested by task delegation contract.
Serial searches before spawn: 0 repo-search/read commands after task claim; one first spawn attempt was rejected by the tool because fork-context and explicit model/role overrides conflict, then the non-fork bounded probe was started.
Findings integrated: live 413 recount, `scripts/ltp_summary.py` source-of-truth chain, no-fake-PASS/no-SKIP-laundering doc guardrail, stale-count warning for older docs, and requirement that worker reports state live `LTP_STABLE_CASES` recount.
