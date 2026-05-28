# Worker 3 metadata/statfs/getdents lane report

Date: 2026-05-26
Team: `ltp-stable383-to-stab-2374dbd5`
Task: `task-3`
Mode: report/discovery lane. No QEMU/run-eval was started, no source was edited, no final `LTP_STABLE_CASES` edit was made, and `.omx/ultragoal` was not touched.

## Scope and guardrails

- Claimed task 3 through `omx team api claim-task` before lane work.
- Re-read live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: `383 total / 383 unique / 0 duplicates`.
- Focus cases are not currently stable: `fstat02`, `fstat02_64`, `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01`, `getcwd03`, `getcwd04`, `getdents01`, `getdents02`.
- Did **not** mutate leader-owned `.omx/ultragoal` or final stable promotion state.
- Did **not** start concurrent default QEMU; all runtime conclusions below are from existing parsed summaries only.
- Promotion remains leader-owned and requires fresh serial RV+LA x musl+glibc evidence with no new `TFAIL`/`TBROK`/`TCONF`, timeout, `ENOSYS`, or panic/trap.

## Inventory and current stable sentinels

The focus cases are present in the inspected RV libc inventories from phase-c:

- `docs/ltp-score-improvement-2026-05-25-phase-c/raw/sdcard-rv-glibc-ltp-bin-list.txt`
- `docs/ltp-score-improvement-2026-05-25-phase-c/raw/sdcard-rv-musl-ltp-bin-list.txt`
- `docs/ltp-score-improvement-2026-05-25-phase-c/raw/sdcard-rv-common-not-stable-ltp-bins.txt`

Older LA inventory artifacts also list the same binaries for both libc trees:

- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-la-glibc-ltp-bin-cases.txt`
- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-la-musl-ltp-bin-cases.txt`

Related already-stable sentinels in the live list: `fstatat01`, `fstat03`, `fstat03_64`, `statfs02`, `statfs02_64`, `fstatfs02`, `fstatfs02_64`, `statvfs02`, `getcwd01`, and `getcwd02`. These are useful regression sentinels only; they are not proof that the focus cases are promotion-clean.

## Current implementation surfaces

- `examples/shell/src/uspace/metadata.rs` implements `sys_fstat`, `sys_newfstatat`, `sys_statfs`, `sys_fstatfs`, `generic_statfs`, and filesystem magic/type selection.
- `examples/shell/src/uspace/fd_table.rs` implements `sys_getcwd`, `sys_getdents64`, directory entry packing, `statfs_path`, path resolution, and fd-backed metadata lookup.
- `examples/shell/src/uspace/syscall_dispatch.rs` dispatches `__NR_statfs`, `__NR_fstatfs`, `__NR_getcwd`, `__NR_newfstatat`, `__NR_fstat`, and `__NR_getdents64`.
- `examples/shell/src/uspace/linux_abi.rs` defines synthetic statfs constants such as `STATFS_BLOCK_SIZE`, `STATFS_NAME_MAX`, `TMPFS_MAGIC`, `PROC_SUPER_MAGIC`, `SYSFS_MAGIC`, `DEVFS_MAGIC`, and `PIPEFS_MAGIC`.

## Candidate assessment

| Candidate | Existing parsed evidence | Implementation risk | Decision |
| --- | --- | --- | --- |
| `fstat02`, `fstat02_64` | RV musl+glibc failed in `target-batch-a-rv-summary.txt`, `target-post285-scout2-rv-summary.txt`, `followup-rv-metadata-near-sigsegv-001-summary.txt`, and `stable350-rv-discovery-fsproc-001-summary.txt`; rows show wrapper `FAIL`, `TBROK=1`, and `ENOSYS=1`. | The current dispatch has `__NR_fstat` and `newfstatat`, but the testcase still observed `ENOSYS`; likely old/stat-family ABI coverage or testcase setup path needs exact syscall tracing before any fix. | Do not promote. Repair must be syscall/ABI-semantic, not case-name hardcoding. |
| `fstatfs01`, `fstatfs01_64` | RV failed in `target-post285-scout3-rv-summary.txt`, `followup-rv-batch-e-inventory-001-summary.txt`, `stable350-rv-discovery-static-batch-001-summary.txt`, and `stable350-rv-discovery-fsproc-001-summary.txt`; rows show wrapper `FAIL`, `TBROK=1`, no `ENOSYS`. | `generic_statfs` is synthetic and may not satisfy LTP setup expectations for filesystem identity/counts; fstatfs on directories/devices/pipes should stay POSIX-like across fd kinds. | Do not promote without a focused statfs repair plus fresh RV+LA proof. |
| `statfs01`, `statfs01_64` | `statfs01` repeatedly failed RV musl+glibc in phase-a summaries with wrapper `FAIL`, `TBROK=1`; no parsed row was found for `statfs01_64` in selected summaries. | Same synthetic statfs surface as fstatfs; path normalization and fd-backed `open_fd_entry` must preserve errno semantics for missing/non-directory paths. | Treat `statfs01_64` as unknown but not clean; scout only after statfs01 behavior is understood. |
| `statfs03`, `statfs03_64` | RV musl+glibc rows in `target-post285-scout3-rv-summary.txt` and `stable350-rv-discovery-fsproc-001-summary.txt` show wrapper `FAIL` with `TFAIL=1`, no `ENOSYS`. | The failing class is a real semantic mismatch rather than missing syscall; likely filesystem limits/free counts/type/name length checks. | Do not promote. Needs statfs semantics work and targeted proof. |
| `statvfs01` | RV rows in multiple phase-a summaries show wrapper `FAIL`, `TBROK=1`, no `ENOSYS`. | libc `statvfs` is derived from statfs-like data; fixing only wrapper status would hide true internal TBROK. | Do not promote until statfs/fstatfs repairs also clear statvfs. |
| `getcwd03` | RV rows in `target-post285-scout4-rv-summary.txt` and `stable350-rv-discovery-lowrisk-002-summary.txt` show wrapper `FAIL`, `TBROK=1`. | `sys_getcwd` itself is small, but LTP setup may depend on chdir/path metadata/search permission behavior; current code should not be changed blindly. | Do not promote; rerun only after path/chdir setup is understood. |
| `getcwd04` | RV rows show wrapper `FAIL` with `TCONF=1`. | TCONF must remain transparent and cannot count as clean. | Keep blocked/transparent; no promotion. |
| `getdents01`, `getdents02` | RV rows in `target-post285-scout4-rv-summary.txt` show wrapper `FAIL`, internal `TFAIL`, `TCONF`, and glibc `ENOSYS=1`. | Current code implements `getdents64` only; LTP/glibc may exercise legacy `getdents`. The directory packing also uses synthetic inode values and `d_off=0`, which may fail strict tests even after syscall dispatch exists. | Do not promote. Future fix likely needs legacy getdents coverage plus dirent offset/inode semantics. |

## Subagent-integrated review notes

The required read-only review probe (`Locke`, `019e64a2-2af7-70d2-8959-d667612caf59`) flagged these additional documentation guardrails, now integrated into this lane report:

- Do not overstate this phase-a docs directory as already evidence-complete. At the time of this worker report, the durable local phase-a files are plan/prompt plus this lane report; scout/promotion evidence must still come from fresh leader-owned serial runs.
- Treat `stable383` as the true stop-state, but keep the exact RV baseline caveat visible: the phase-c stop-state used a `stable384` RV clean superset where exact stable383 RV rerun was user-stopped; leader can rerun exact stable383 RV if exact-baseline proof is needed.
- Keep `read02` as transparent `pass_with_tconf`; it is not clean promotion evidence.
- Do not let targeted-only cleanliness override aggregate blockers such as the known `kill02` LA aggregate `TBROK`/setup-timeout issue.
- The metadata/statfs/getdents cases are ABI-sensitive and should not be labeled “easy” merely because binaries exist. Prior summaries show real `TBROK`, `TFAIL`, `TCONF`, and `ENOSYS` rows, so this lane is currently a blocker/repair lane, not a promotion-clean lane.

## Recommended leader scout batches

These batches are **not promotion evidence by themselves**. Use them only after any relevant repair or when leader wants a serial baseline refresh; send only RV-clean cases to LA.

### RV metadata/statfs baseline batch

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=fstat02,fstat02_64,fstatfs01,fstatfs01_64,statfs01,statfs01_64,statfs03,statfs03_64,statvfs01 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv
python3 -B scripts/ltp_summary.py <rv-log>
```

### RV cwd/dirent baseline batch

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=getcwd03,getcwd04,getdents01,getdents02 \
LTP_CASE_TIMEOUT_SECS=60 \
./run-eval.sh rv
python3 -B scripts/ltp_summary.py <rv-log>
```

### LA confirmation template

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=<rv-clean-subset-only> \
LTP_CASE_TIMEOUT_SECS=90 \
./run-eval.sh la
python3 -B scripts/ltp_summary.py <la-log>
```

## Next feasible repair hypotheses

1. For `getdents01/02`, inspect whether the binaries call legacy `getdents` rather than `getdents64`; if yes, implement the correct ABI entry point and preserve line-start wrapper markers. Then verify `d_off`, `d_ino`, `d_reclen`, `.`/`..` expectations, and end-of-directory behavior.
2. For `fstat02/_64`, capture the failing syscall number before editing. Existing summaries show `ENOSYS`; a blind change to `sys_fstat` may miss an old/stat-family syscall path.
3. For `statfs/fstatfs/statvfs`, compare LTP expectations against the synthetic `generic_statfs` fields. Prior failures are real `TBROK`/`TFAIL`, so do not launder them into `TCONF`/PASS.
4. For `getcwd03/04`, inspect LTP setup paths and chdir/search-permission setup first. `getcwd04` has `TCONF`, so it is explicitly not clean.
5. Keep this lane lower priority for the easy-first stable413 push unless a narrow repair lands; all focus cases have prior RV blocker rows or no fresh clean proof.

## Verification

| Check | Result |
| --- | --- |
| Worker protocol | Startup ACK sent; task 3 claimed by `worker-3` through claim-safe API. |
| Stable-list preflight | Python scan of `examples/shell/src/cmd.rs::LTP_STABLE_CASES` reported `383 total / 383 unique / 0 duplicates`; all focus cases are absent from stable. |
| Inventory scan | Python/`rg` scan found all focus cases in RV musl+glibc phase-c bin lists and older LA musl+glibc bin lists. |
| Source audit | Inspected `metadata.rs`, `fd_table.rs`, `syscall_dispatch.rs`, and `linux_abi.rs` surfaces listed above. |
| Prior evidence scan | Parsed phase-a/phase-c `*summary*.txt` tables for all focus cases; blocker rows are summarized in the candidate table. |
| Runtime policy | No QEMU/run-eval was started; no runtime output from this worker is being presented as promotion evidence. |
| Scope ownership | No edits to `.omx/ultragoal` or `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; only this worker report was added. |

## Subagent evidence

Subagents spawned: 1 (`Locke`, `019e64a2-2af7-70d2-8959-d667612caf59`) for a read-only review probe.
Subagent model: `gpt-5.4-mini` requested by task delegation contract.
Serial searches before spawn: 0 repo-search/read commands after task claim; the subagent was started immediately after claim.
Findings integrated: docs certainty risk, exact stable383 RV caveat, `read02`/`kill02` guardrails, and ABI-sensitive metadata/statfs/getdents blocker framing were added to this report.
