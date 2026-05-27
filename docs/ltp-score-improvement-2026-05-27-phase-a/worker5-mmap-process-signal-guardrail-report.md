# Worker 5 mmap/process/signal guardrail report

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Task: 5 — mmap/process/signal + verification guardrails lane
Status: completed as a report/guardrail lane. No QEMU was started, no source was edited, no final `examples/shell/src/cmd.rs::LTP_STABLE_CASES` edit was made, and `.omx/ultragoal` was not touched.

## Report paths

- Report: `docs/ltp-score-improvement-2026-05-27-phase-a/worker5-mmap-process-signal-guardrail-report.md`
- Evidence capture: `docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker5-mmap-process-signal-evidence.txt`

## Scope and baseline

Leader lane update requested Worker 5 to scout `mmap04/05/06`, `munmap01`, `mprotect01/02`, `mmap10*`, `vma*`, `waitid07/08/10`, cautious `kill02`, and light syscall cases while preserving no-fake-pass/no-timeout-as-pass, `read02` TCONF transparency, marker-prefix, and log-noise guardrails.

Live `LTP_STABLE_CASES` was recalculated before writing this report:

- `413 total / 413 unique / 0 duplicates`
- Already stable in this lane's pool: `mmap06`, `mmap10`
- Not stable and inventory-present in historical common non-stable lists: `mmap04`, `mmap05`, `munmap01`, `mprotect01`, `mprotect02`, `mmap12`, `mmap13`, `mmap14`, `vma01`, `vma02`, `waitid07`, `waitid08`, `waitid10`, `kill02`, `poll02`, `gethostid01`, `getcpu01`, `gethostname02`
- Not stable but inventory gap in the historical common list: `mmap10_1`

## Candidate status table

| Case | Current evidence | Status for stable413 -> stable460 | Next action |
| --- | --- | --- | --- |
| `mmap06` | Already in live stable413. | Not a new candidate. | Keep as regression guard. |
| `mmap10` | Already in live stable413. | Not a new candidate. | Keep as regression guard. |
| `mmap04` | `target-stable400-proc-vm-pipe-rv-001-summary.txt` has RV glibc+musl `FAIL code=2`, `TBROK=1`. | Blocked; not promotion-clean. | Repair `/proc/self/maps` / VMA visibility before any LA spend. |
| `mmap05` | Same summary has RV glibc+musl `FAIL code=2`, `TBROK=1`; older reports also show signal-like failures. | Blocked; not promotion-clean. | VM/page-fault diagnosis first. |
| `munmap01` | Same summary has RV glibc+musl `FAIL code=139`. | Blocked; not promotion-clean. | Repair post-unmap fault/signal/boundary semantics. |
| `mprotect01` | Same summary has RV glibc+musl `FAIL code=1`, `TFAIL=2`. | Blocked; not promotion-clean. | Dedicated `mprotect` semantic repair and regression. |
| `mprotect02` | Same summary has RV glibc+musl `FAIL code=2`, `TBROK=2`. | Blocked; not promotion-clean. | Dedicated `mprotect`/unexpected-signal repair. |
| `mmap10_1` | `target-fallback18-rv-001-summary.txt` has RV glibc+musl wrapper `FAIL code=-1`; historical common inventory did not list it. | Inventory/staging blocker. | Refresh sdcard/runtest inventory before runtime debugging. |
| `mmap12` | `target-scout26-rv-001-summary.txt` has RV musl `FAIL code=1`, `TFAIL=1`; no clean four-way evidence found. | Blocked. | Re-scout only after VM fixes or if leader needs fresh blocker proof. |
| `mmap13` | `target-scout14-rv-001-summary.txt` has RV glibc+musl `FAIL code=1`, `TFAIL=1`. | Blocked. | Defer behind `mprotect`/signal fault handling. |
| `mmap14` | Same scout has RV glibc+musl `FAIL code=1`, `TFAIL=1`; musl also had a large negative free-frame delta. | Blocked. | Defer behind VM/fault accounting repair. |
| `vma01` | `target-fallback18-rv-001-summary.txt` has RV glibc+musl `FAIL code=2`, `TBROK=4`. | Blocked. | Improve `/proc/self/maps` and VMA metadata first. |
| `vma02` | Same fallback summary has RV glibc+musl `FAIL code=32`, `TCONF=2`. | Blocked; TCONF cannot be counted clean. | Defer; no promotion without real PASS rows. |
| `waitid07` | `target-stable400-proc-vm-pipe-rv-001-summary.txt` has RV glibc+musl `FAIL`, `TFAIL=5`. Prior worker reports tie this to missing `WSTOPPED`/`WNOWAIT` child stopped-event accounting. | Blocked; not a narrow safe patch. | Needs real wait-visible stopped-event model, not a promotion-list edit. |
| `waitid08` | Same summary has RV glibc+musl `FAIL`, `TFAIL=10`; prior reports require stopped + continued event accounting. | Blocked; not a narrow safe patch. | Needs `WSTOPPED`/`WCONTINUED` state model. |
| `waitid10` | Same summary has RV glibc+musl `FAIL`, `TBROK=1`; prior reports identify missing `/proc/sys/kernel/core_pattern` as likely first setup blocker. | Blocked in this lane because a synthetic proc/VFS patch is shared-scope. | Ask leader/metadata-VFS owner before touching synthetic proc paths; then rerun RV first. |
| `kill02` | `target-stable400-proc-vm-pipe-rv-001-summary.txt` has targeted RV glibc+musl `PASS`; prior process report records targeted LA clean but later aggregate `la:musl:kill02` `TBROK=4`. | High-risk aggregate/ordering blocker; do not promote from targeted rows. | Only revisit with isolated LA raw capture plus aggregate stable gate. |
| `poll02` | `worker2-light-syscall-rv-001-summary.txt` has RV glibc+musl `FAIL`, `TFAIL=7`; diagnosis says timer precision/scheduler behavior, not missing syscall. | Blocked. | Timer/yield precision investigation outside this narrow promotion lane. |
| `gethostid01` | Same summary has RV glibc `TFAIL=4`, musl `TCONF=1`; diagnosis says glibc hits read-only hostid persistence and musl lacks `sethostid`. | Blocked; musl TCONF prevents four-way clean promotion. | Do not spend stable460 budget unless libc/filesystem policy changes. |
| `getcpu01` | Same summary has RV glibc `ENOSYS=1` and musl `TCONF=1`. A `getcpu` syscall shim could help glibc only. | Not promotion-clean; glibc-only narrow patch is insufficient for stable. | Optional low-risk code experiment only if leader wants glibc improvement without promotion. |
| `gethostname02` | Same summary has RV glibc `PASS`, RV musl `FAIL`, `TFAIL=1`; diagnosis says musl libc short-buffer behavior over `uname`. | Blocked; not kernel-missing. | Do not alter kernel hostname behavior just to hardcode a case. |

## Implementation surfaces inspected

- `examples/shell/src/uspace/memory_map.rs:127-252` — `sys_mmap` validates length/type, chooses target, populates file mappings, and records writable shared mappings.
- `examples/shell/src/uspace/memory_map.rs:255-306` — `sys_munmap` aligns ranges, defers self-stack unmap, forgets mmap ranges, and unmaps the address space.
- `examples/shell/src/uspace/memory_map.rs:343-388` — `sys_mprotect` validates page alignment and calls `aspace.protect`, with a small writable prefault path.
- `examples/shell/src/uspace/process_lifecycle.rs:619-730` — `wait_child` only returns exited children and reaps them when observed.
- `examples/shell/src/uspace/process_lifecycle.rs:1079-1116` — `sys_waitid` accepts only `WNOHANG | WEXITED | __WNOTHREAD | __WALL`; no `WSTOPPED`, `WCONTINUED`, or `WNOWAIT` event model exists.
- `examples/shell/src/uspace/syscall_dispatch.rs:421-423` and `479-488` — `kill`/`tkill`/`tgkill`, `wait4`, and `waitid` dispatch is present; the current blockers are semantics/evidence, not missing dispatch, except optional glibc-only `getcpu`.

## Guardrail status

- No-fake-pass/no-timeout-as-pass remains enforceable through `scripts/ltp_summary.py`, whose contract treats numeric wrapper status plus internal `TFAIL/TBROK/TCONF`, timeout, `ENOSYS`, and panic/trap markers as promotion blockers.
- stable413 final quality gate remains the baseline: RV and LA each have `PASS LTP CASE 826`, `FAIL 0`, `ltp-musl 413/0`, `ltp-glibc 413/0`, timeout `0`, ENOSYS `0`, panic/trap `0`.
- Known transparent internal TCONF remains `read02` only: RV internal `TCONF=4`, LA internal `TCONF=4`, accepted new TCONF `0`.
- Marker-prefix remains clean in stable413 final logs: bad marker-prefix lines `0` on RV and LA.
- Remote-sensitive `axfs::fops:297 [AxError::NotADirectory]` remains `0` on RV and LA; residual `axfs_ramfs::file:69` `NotADirectory=22` per arch is disclosed as non-marker-affecting.

## Recommended leader-serialized validation commands

Do not run these concurrently with another default QEMU/evaluator job. Use them only after the leader approves a serialized scout window.

```bash
mkdir -p docs/ltp-score-improvement-2026-05-27-phase-a/raw
cases=mmap04,mmap05,munmap01,mprotect01,mprotect02
for arch in rv la; do
  tag=worker5-vm-${arch}-001
  OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh "$arch" \
    > "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}.log" 2>&1
  printf 'status=%s\narch=%s\ncases=%s\n' "$?" "$arch" "$cases" \
    > "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}.status"
  python3 -B scripts/ltp_summary.py \
    "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}.log" \
    | tee "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}-summary.txt"
done
```

```bash
cases=waitid07,waitid08,waitid10,kill02,poll02,gethostid01,getcpu01,gethostname02
for arch in rv la; do
  tag=worker5-process-light-${arch}-001
  OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh "$arch" \
    > "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}.log" 2>&1
  printf 'status=%s\narch=%s\ncases=%s\n' "$?" "$arch" "$cases" \
    > "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}.status"
  python3 -B scripts/ltp_summary.py \
    "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}.log" \
    | tee "docs/ltp-score-improvement-2026-05-27-phase-a/raw/${tag}-summary.txt"
done
```

Only rows with RV+LA x musl+glibc wrapper PASS and zero internal `TFAIL/TBROK/new TCONF`, timeout, ENOSYS, panic/trap can feed a promotion matrix. Targeted `kill02` PASS rows are not enough; it previously failed in aggregate.

## Subagent evidence

- Subagents spawned: 2
  - `019e68b6-5e1f-7633-b973-f397dcb5a31f` (`Bohr`): high-value candidate radar. Integrated as cross-lane handoff only; most suggested VFS/create/remove candidates belong to Worker 2/4, not this lane.
  - `019e68b6-8208-7c83-9766-bfb57b59f2e8` (`Godel`): implementation surface scan. Integrated relevant `waitid` option/model risk and process-group caution; metadata/statfs/mount/symlink suggestions are out of this worker's lane.
- Subagent model: `gpt-5.4-mini`
- Findings integrated:
  - Keep VFS candidates like `open06`, `creat04`, `mkdir04`, `rmdir03`, `unlink08` out of this report except as handoffs to their owning lanes.
  - Treat `waitid07/08` as real stopped/continued-event accounting work, not a quick flag allowlist.
  - Treat `waitid10` as a likely synthetic proc setup blocker first, but shared-scope because it touches proc/VFS file surfaces.
  - Do not trust targeted `kill02` alone; aggregate gate history already disproves it.
- Serial repo-search/read commands before spawn: 3

## Verification

| Check | Command | Result |
| --- | --- | --- |
| Stable-list invariant | Python scan of `examples/shell/src/cmd.rs::LTP_STABLE_CASES` | PASS: `total=413 unique=413 duplicates=0` |
| Candidate membership/evidence capture | `python3 /tmp/worker5_collect.py` + `rg` summary-row extraction | PASS: wrote `raw/worker5-mmap-process-signal-evidence.txt`; all non-stable lane candidates are blocked, already stable, or inventory-gapped as listed above. |
| Parser regression tests | `python3 -B scripts/test_ltp_summary.py` | PASS: `Ran 4 tests in 0.005s` / `OK` |
| Correct target build | `make A=examples/shell ARCH=riscv64 build` | PASS: `Finished release profile [optimized] target(s) in 5m 29s`; produced `examples/shell/shell_riscv64-qemu-virt.bin`. |
| Host-target cargo diagnostic | `cargo check -p arceos-shell --features uspace` | NOT APPLICABLE as gate: failed on the default x86_64 host target with architecture cfg/TrapFrame mismatches; the README-specified riscv64 build above is the valid non-QEMU check. |
| Linter/static whitespace | `git diff --check --cached` | PASS: no whitespace errors after staging this docs-only report update. |
| QEMU policy | No `./run-eval.sh` command run by this worker | PASS: preserved leader-owned serialized QEMU gate. |
| Scope ownership | `git status --short --untracked-files=all` | PASS before final transition: only this tracked report is modified, then staged for the worker checkpoint; no `.omx/ultragoal` or `LTP_STABLE_CASES` edits. |

## Completion summary

This lane found no honest immediate Worker-5-owned promotion subset. The current best result is a blocker-first report: VM/VMA candidates need real mmap/mprotect/munmap or `/proc/self/maps` repairs; waitid candidates require child stopped/continued/non-reap accounting or a shared proc setup fix; light syscalls remain libc/timer/aggregate blockers. The safe next promotion work should use other workers' cleaner VFS/FD candidates while preserving this lane's guardrails.
