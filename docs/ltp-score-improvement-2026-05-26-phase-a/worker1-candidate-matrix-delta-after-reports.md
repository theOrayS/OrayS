# Worker 1 candidate-matrix delta after worker reports

Date: 2026-05-26
Team: `ltp-stable383-to-stab-2374dbd5`
Worker: `worker-1`
Task: `task-5` / consolidate candidate matrix after worker reports

## Scope and guardrails

- Report-only follow-up. No QEMU/evaluator command was run in this lane.
- No `.omx/ultragoal` mutation and no `examples/shell/src/cmd.rs::LTP_STABLE_CASES` mutation.
- This file does **not** edit the leader-owned final matrix; it records replacement/delta rows for the leader to apply.
- Promotion remains leader-owned: a candidate is promotion-clean only with RV+LA x musl+glibc parser-clean evidence, no internal `TFAIL`/`TBROK`/new `TCONF`, no timeout, no ENOSYS/not-implemented, and no panic/trap. Existing `read02` TCONF stays transparent.

## Inputs read

- Current worker-1 matrix: `docs/ltp-score-improvement-2026-05-26-phase-a/candidate-matrix-easy30-40.md`.
- Leader/root current phase-a docs and raw summaries under `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-26-phase-a/`.
- Worker reports:
  - `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-26-phase-a/worker2-light-syscall-process-scout-report.md`.
  - `/root/oskernel2026-orays/.omx/team/ltp-stable383-to-stab-2374dbd5/worktrees/worker-2/docs/ltp-score-improvement-2026-05-26-phase-a/worker2-light-syscall-rv001-diagnosis.md`.
  - `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-26-phase-a/worker3-metadata-statfs-getdents-report.md`.
  - `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-26-phase-a/worker4-fd-io-vfs-guardrail-report.md`.
- Live task status: worker3 follow-up task 7 was still `in_progress` when this report was written, so only worker3 task-3 report is integrated here.
- Leader-root current uncommitted source diff was inspected read-only: `fd_table.rs` / `syscall_dispatch.rs` contain FD negative-offset/O_APPEND and `sendfile` work that explains the later RV-clean sendfile/pread/pwrite summaries.

## Leader raw-scout outcome recap

| Evidence | Parser result | Matrix delta |
| --- | --- | --- |
| `raw/worker2-light-syscall-rv-001.log` | RV promotion candidates: 0; `poll02`, `gethostid01`, `getcpu01`, `gethostname02` all blocked/incomplete | Demote all four out of first-wave; only `getcpu01` has a narrow glibc-only syscall-hole experiment, but musl TCONF still blocks promotion |
| `raw/worker4-fd-vector-vfs-rv-001.log` + `raw/worker4-fd-vector-vfs-la-001.log` | RV+LA promotion candidates: `preadv01`, `preadv02`, `pwritev01`, `pwritev02`; `unlink07` blocked on RV TFAIL | Promote-ready candidate set for leader consideration after aggregate gate: 4 vector cases; demote `unlink07` |
| `raw/patched-fd-sendfile-rv-001.log` | RV-only promotion candidates: 12 scalar FD/sendfile cases; `sendfile03`, `sendfile03_64` failed in that run | Keep 12 as LA-confirm queue, not four-way clean yet |
| `raw/patched-sendfile03-rv-002.log` | RV-only promotion candidates: `sendfile03`, `sendfile03_64` | Add `sendfile03`, `sendfile03_64` to LA-confirm queue after leader sendfile patch |
| Worker3 metadata/statfs report | All 13 metadata/statfs/getcwd/getdents focus cases have prior RV `FAIL`/`TBROK`/`TFAIL`/`TCONF`/`ENOSYS` evidence or no clean proof | Demote the whole metadata/statfs/getcwd/getdents first-wave block until task7 repair feasibility lands |

## Exact recommended replacement rows

Use these as replacement status rows for the current candidate matrix. They are deltas from the original first-wave/reserve ranking, not stable-list edits.

| Case/family | Current replacement status | Evidence | Recommended matrix row/action |
| --- | --- | --- | --- |
| `preadv01`, `preadv02`, `pwritev01`, `pwritev02` | Four-way targeted-clean candidates | `worker4-fd-vector-vfs-{rv,la}-001`: RV+LA x musl/glibc PASS, internal 0 | Mark `four-way clean; leader aggregate-gate candidate`. These are the only currently complete RV+LA candidates in this delta. |
| `pread02`, `pread02_64`, `pwrite02`, `pwrite02_64`, `pwrite04`, `pwrite04_64` | RV clean after leader FD patch; LA pending | `patched-fd-sendfile-rv-001`: RV musl+glibc PASS, internal 0 | Move from generic first-wave to `LA-confirm queue; do not promote until LA clean + aggregate stable gate`. Preserve negative-offset/O_APPEND semantics patch before rerun. |
| `sendfile02`, `sendfile02_64`, `sendfile04`, `sendfile04_64`, `sendfile05`, `sendfile05_64` | RV clean after leader `sendfile` patch; LA pending | `patched-fd-sendfile-rv-001`: RV musl+glibc PASS, internal 0 | Move to `LA-confirm queue; source-fix dependency: real sys_sendfile dispatch/helper must remain`. |
| `sendfile03`, `sendfile03_64` | RV clean only after second leader patch/run; LA pending | first RV run TFAIL; `patched-sendfile03-rv-002`: RV musl+glibc PASS, internal 0 | Replace older blocked row with `post-patch RV clean; LA-confirm queue`. Do not use the earlier failed run as current status after the patch. |
| `poll02` | Blocked | `worker2-light-syscall-rv-001`: both libcs FAIL, `TFAIL=7` each | Demote from first-wave to `timer precision blocker`; no low-risk source patch before scout. |
| `gethostid01` | Blocked / non-promotable | RV glibc TFAIL, RV musl TCONF | Demote from first-wave; filesystem/libc policy, not a narrow kernel syscall fix. |
| `getcpu01` | Blocked, with glibc-only low-risk source experiment | RV glibc ENOSYS/TFAIL; RV musl TCONF | Demote from promotion. Optional source fix: add `__NR_getcpu` route/helper for glibc only, but do not scout for promotion until musl no longer TCONF-blocks. |
| `gethostname02` | Blocked by musl only | RV glibc PASS, RV musl TFAIL short-buffer semantics | Demote from first-wave; no safe kernel patch recommended from current evidence. |
| `times03`, `getpgid01`, `fork13`, `fork14`, `kill05`, `kill10` | Known process/signal/timing blockers | Worker2 report: repeated prior RV FAIL/TBROK/TFAIL/timeout evidence | Keep out of easy-first scout batches. |
| `clone06`, `clone07`, `clone08`, `clone09` | Reserve diagnosis only | Adjacent clone-family flags/process semantics risk; no fresh clean proof | Keep out of promotion-first batches; run only as isolated RV clone diagnosis if leader wants raw blockers. |
| `fstat02`, `fstat02_64` | Blocked; possible syscall/ABI repair lane | Worker3 report: repeated RV FAIL/TBROK/ENOSYS history | Demote from first-wave. Wait for task7; require syscall-number/raw trace before any old-stat/fstat patch. |
| `fstatfs01`, `fstatfs01_64`, `statfs01`, `statfs01_64`, `statfs03`, `statfs03_64`, `statvfs01` | Blocked statfs/statvfs semantic lane | Worker3 report: RV FAIL/TBROK/TFAIL; synthetic statfs field risk | Demote as not easy-first. Scout only after statfs field/magic/free-count repair hypothesis. |
| `getcwd03`, `getcwd04` | Blocked | Worker3 report: `getcwd03` TBROK; `getcwd04` TCONF | Demote from first-wave; keep TCONF transparent. |
| `getdents01`, `getdents02` | Blocked, with potential low-risk legacy syscall coverage | Worker3 report: RV FAIL with TFAIL/TCONF and glibc ENOSYS; code has `getdents64` only | Demote from first-wave. Candidate low-risk source fix is legacy `getdents` dispatch/ABI plus dirent offset/inode validation, pending task7 feasibility. |
| `unlink07` | Blocked | `worker4-fd-vector-vfs-rv-001`: RV musl+glibc FAIL, TFAIL=1 each | Demote from high-value scout to blocked; do not send LA. |
| `open06` | Source-fix-before-scout | Worker4 report: FIFO write-only nonblocking no-reader should return `ENXIO`; local FIFO open likely returns write end | Candidate narrow VFS/FIFO fix before RV scout. |
| `creat04`, `mkdir04`, `rmdir03`, `unlink08` | Source-fix-before-scout | Worker4 report: parent write/search/sticky/permission gaps likely | Do not batch before permission semantics patch; good low-risk VFS repair candidates if leader wants source work. |
| `open07`, `creat08`, `creat09`, `mkdir03`, `rmdir02` | Medium-risk reserve | Worker4 report: no current clean proof; symlink/setgid/errno semantics need sanity | Keep reserve after FD/sendfile LA confirmation and after task7. |
| `open10`, `open11`, `open12`, `open14`, `creat06`, `creat07`, `mkdir09`, `unlink09`, `preadv03`, `pwritev03`, fs-suite substitutes | High-risk / skip-now | Worker4 report: setgid/device/O_TMPFILE/largefile/exec-file/O_DIRECT/ioctl/fs-suite risks | Keep out of easy-first mainline. |

## Recommended next leader-serial execution order

### Immediate non-RV step: LA confirmation for RV-clean FD/sendfile cases

The highest-yield next action is not another broad RV scout. It is a leader-serial LA confirm for the 14 RV-clean scalar FD/sendfile cases, while preserving the current leader source diff that made them RV-clean.

```bash
cases=pread02,pread02_64,pwrite02,pwrite02_64,pwrite04,pwrite04_64,sendfile02,sendfile02_64,sendfile03,sendfile03_64,sendfile04,sendfile04_64,sendfile05,sendfile05_64
tag=patched-fd-sendfile-la-001
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$cases" LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" 2>&1
printf 'status=%s\narch=la\ncases=%s\n' "$?" "$cases" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.status"
python3 -B scripts/ltp_summary.py "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  | tee "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}-summary.txt"
python3 -B scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc \
  docs/ltp-score-improvement-2026-05-26-phase-a/raw/patched-fd-sendfile-rv-001.log \
  docs/ltp-score-improvement-2026-05-26-phase-a/raw/patched-sendfile03-rv-002.log \
  "docs/ltp-score-improvement-2026-05-26-phase-a/raw/${tag}.log" \
  > "docs/ltp-score-improvement-2026-05-26-phase-a/raw/patched-fd-sendfile-rvla-promotion-candidates.txt"
```

### RV Batch 1: FD/sendfile refresh only if source changes again

If the leader edits `fd_table.rs` or `syscall_dispatch.rs` further before LA, first re-run this RV refresh. If no additional FD/sendfile source changes occur, this is redundant because the current RV logs are already clean for these cases.

```text
pread02,pread02_64,pwrite02,pwrite02_64,pwrite04,pwrite04_64,sendfile02,sendfile02_64,sendfile03,sendfile03_64,sendfile04,sendfile04_64,sendfile05,sendfile05_64
```

### RV Batch 2: VFS/FIFO/permission repair validation after source fixes only

Run only after leader applies or decides to test narrow source fixes for FIFO no-reader and parent permission/sticky behavior.

```text
open06,creat04,mkdir04,rmdir03,unlink08
```

Expected source-fix prerequisites:

- `open06`: FIFO `O_NONBLOCK|O_WRONLY` without a reader should fail `ENXIO`.
- `creat04` / `mkdir04` / `rmdir03` / `unlink08`: parent directory write/search and sticky/permission checks must be modeled honestly.

### RV Batch 3: metadata repair validation after worker3 task7 lands

Run only if task7 identifies and/or leader applies a narrow repair. Do not include the entire statfs family in an easy-first batch before that.

```text
getdents01,getdents02,fstat02,fstat02_64
```

Expected source-fix prerequisites:

- `getdents01/getdents02`: legacy `getdents` syscall/ABI coverage plus dirent `d_off`/`d_ino`/record-length semantics.
- `fstat02/_64`: confirm actual old-stat/fstat syscall number before adding dispatch; existing summaries show ENOSYS but a blind `sys_fstat` edit may miss the ABI path.

## Promotion candidate ledger after this delta

| Bucket | Cases | Count | Required next gate |
| --- | --- | ---: | --- |
| Already four-way targeted-clean | `preadv01`, `preadv02`, `pwritev01`, `pwritev02` | 4 | Leader aggregate stable gate before promotion |
| RV-clean, LA pending | `pread02`, `pread02_64`, `pwrite02`, `pwrite02_64`, `pwrite04`, `pwrite04_64`, `sendfile02`, `sendfile02_64`, `sendfile03`, `sendfile03_64`, `sendfile04`, `sendfile04_64`, `sendfile05`, `sendfile05_64` | 14 | LA targeted confirm, then aggregate stable gate |
| Explicitly demoted from first-wave | worker2 four light cases + worker3 metadata/statfs/getcwd/getdents block + `unlink07` | 18 | Repair or fresh diagnosis before any LA/promotion |
| Source-fix-before-scout candidates | `getcpu01` (glibc-only/non-promotable), `getdents01/02`, `fstat02/_64`, `open06`, `creat04`, `mkdir04`, `rmdir03`, `unlink08` | 9 | Narrow source fix + RV parser-clean proof |

If the 14 LA-pending FD/sendfile cases pass LA, the leader could have up to 18 targeted-clean cases from the FD/iovec/sendfile slice before aggregate stable gates. That is still short of +30, so the next honest pool should come from VFS/FIFO/permission and metadata repair-validation batches, not from the now-demoted light syscall/statfs blockers.

## Verification performed in this lane

- Mailbox message `4bb11ebd-0539-46fe-b486-5eed391330ab` marked delivered.
- Claimed task 5 through `omx team api claim-task` before editing.
- Read worker inbox, task JSON, worker2/3/4 reports, task6 result, and current task7 status.
- Parsed leader raw logs with `python3 -B scripts/ltp_summary.py --promotion-candidates`:
  - worker2 light syscall RV: 0 candidates / 4 blocked.
  - worker4 vector RV+LA: 4 four-way candidates / 1 blocked.
  - patched FD/sendfile RV: 12 RV candidates / 2 blocked.
  - patched sendfile03 RV: 2 RV candidates / 0 blocked.
- Rechecked no code/stable-list/.omx mutation from this lane; this file is the only intended worker-1 delta artifact.

Subagent spawn/skip evidence: skipped. Task 5 was a bounded report-only consolidation over already-produced worker reports and leader raw parser outputs; serial read/parse was safer than spawning overlapping sidecars and avoided duplicating active worker3 task7.
