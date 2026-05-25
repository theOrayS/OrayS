# Worker 5 mmap/fs guardrail report

Date: 2026-05-25
Task: 5 — Mmap fs-suite verification guardrail lane
Status: completed as a report-only lane. No QEMU was started, no source was
edited, no final `LTP_STABLE_CASES` edit was made, and `.omx/ultragoal` was
not touched.

## Scope and baseline

- Live stable list checked from `examples/shell/src/cmd.rs::LTP_STABLE_CASES`:
  `375 total / 375 unique / 0 duplicates`.
- Already stable in the requested pool: `mmap06`, `mmap09`, `mmap10`,
  `mmap11`, `ftest01`, `ftest02`, `ftest03`, `ftest04`, `ftest05`,
  `ftest07`, `ftest08`, `stream01`, `stream03`, `stream04`, `stream05`,
  `inode01`.
- Not stable and still requiring clean serial evidence before promotion:
  `mmap04`, `mmap05`, `munmap01`, `mprotect01`, `mprotect02`, `mmap10_1`,
  `mmap12`, `mmap13`, `mmap14`, `vma*` cases, `fs_perms01`, `ftest06`,
  `rwtest*`, `stream02`, `openfile01`, `writetest01`, `iogen01`,
  `fs_inod01`, and `inode02`.
- Evidence below is from existing phase-a/phase-b/phase-c artifacts only. The
  leader explicitly did not serialize QEMU for this worker lane.

## Guardrail status

- Runner marker semantics remain auditable in `examples/shell/src/cmd.rs`: each
  case emits a line-start wrapper marker, status `0` is the only wrapper-pass
  value, non-zero exits emit `FAIL LTP CASE`, and timeout exits also emit
  `TIMEOUT LTP CASE` (`cmd.rs:1483-1559`).
- Parser guardrails are present in `scripts/ltp_summary.py`: wrapper status is
  normalized from the numeric code, internal `TFAIL/TBROK/TCONF`, timeout,
  `ENOSYS`, and panic/trap markers are counted, and timeout evidence removes a
  prior zero-status pass classification (`ltp_summary.py:1-10`, `110-121`,
  `201-223`, `235-255`, `303-335`).
- Marker-prefix evidence from the stable375 phase-b final gate remains clean:
  `raw/stable375-final-marker-prefix.txt` reports RV `markers=750 bad=0` and
  LA `markers=750 bad=0`.
- Phase-c remote glibc-only baseline remains parser-clean but noisy:
  `Riscv输出` has `4510` `AxError::NotADirectory` lines and `LoongArch输出`
  has `4507`; both have `375` markers, `bad_marker_prefix=0`, `TFAIL=0`,
  `TBROK=0`, `TCONF=2`, `TIMEOUT_LTP_CASE=0`, `ENOSYS=0`, and remote summaries
  parse as `pass_count=375`, `fail_count=0`, `timeouts=0`. Treat this as output
  size/noise risk, not a candidate-promotion blocker.

## mmap / mprotect / munmap candidates

| Case | Existing evidence | Decision |
| --- | --- | --- |
| `mmap04` | Phase-a/blocker and phase-b primary RV summaries repeatedly fail on RV musl+glibc with code `2`, `TBROK=1`; earlier note points at `/proc/self/maps` parsing/visibility. | Do not promote; repair/prove `/proc/self/maps` first. |
| `mmap05` | RV musl+glibc fails: either code `139` or code `2` with `TBROK=1` across phase-a/phase-b summaries. | Do not promote; page-fault/signal classification remains suspect. |
| `munmap01` | RV musl+glibc fails code `139` across phase-a blocker/follow-up and phase-b primary summaries. | Do not promote; post-unmap fault delivery/boundary semantics need repair. |
| `mprotect01` | RV glibc failed code `139`/`1`; RV musl failed code `1` with `TFAIL=3` in phase-a and code `1` with `TFAIL=2` in phase-b primary retry. | Do not promote; real `mprotect` behavior failures remain visible. |
| `mprotect02` | RV musl+glibc fail code `2`, `TBROK=2` across phase-a and phase-b primary summaries. | Do not promote; unexpected-signal/TBROK blocker. |
| `mmap10_1` | `target-fallback18-rv-001-summary.txt` shows RV musl+glibc wrapper `FAIL code=-1`, consistent with missing/unstaged testcase or run-dir failure. | Do not promote; first verify asset/staging availability. |
| `mmap12` | `target-scout26-rv-001-summary.txt` shows RV musl `FAIL code=1`, `TFAIL=1`; no four-way clean evidence found. | Do not promote. |
| `mmap13` | `target-scout14-rv-001-summary.txt` shows RV musl+glibc `FAIL code=1`, `TFAIL=1`. | Do not promote. |
| `mmap14` | `target-scout14-rv-001-summary.txt` shows RV musl+glibc `FAIL code=1`, `TFAIL=1`; musl also had a large negative free-frame delta in that scout. | Do not promote. |
| `vma01` | `target-fallback18-rv-001-summary.txt` shows RV musl+glibc `FAIL code=2`, `TBROK=4`. | Do not promote. |
| `vma02` | `target-fallback18-rv-001-summary.txt` shows RV musl+glibc `FAIL code=32`, `TCONF=2`; timeout keyword was not set in the summary row, but wrapper failure is enough to reject. | Do not promote. |
| `vma03+` | No phase-a/phase-b/phase-c summary rows found in this lane's read-only scan. | Unknown; needs targeted serial evidence before consideration. |

Relevant implementation surfaces if leader assigns a future repair lane:
`examples/shell/src/uspace/memory_map.rs:127-252` (`sys_mmap`, fixed mapping,
file-backed copy, shared mapping record), `255-306` (`sys_munmap`, alignment,
self-stack deferral, unmap), and `343-388` (`sys_mprotect`, protection update
and small writable prefault).

## fs-suite substitutes and blockers

| Candidate group | Existing evidence | Decision |
| --- | --- | --- |
| `ftest01`-`ftest04` | Already stable375 and covered by phase-b final RV+LA summaries. | Already promoted; not new phase-c candidates. |
| `ftest05`, `ftest07`, `ftest08` | Already stable375; promoted in phase-b with clean RV `target-adjacent20-rv-001-summary.txt` and LA `target-adjacent8-la-001-summary.txt`, then covered by final stable375 gates. | Already promoted; keep as regression sentinels. |
| `ftest06` | RV musl+glibc `FAIL code=4` in `target-adjacent20-rv-001-summary.txt`; phase-b report identifies TWARN/cleanup behavior. | Do not promote until cleanup/wrapper status is understood. |
| `stream01`, `stream03`, `stream04`, `stream05` | Already stable375 and final-gate clean. | Already promoted. |
| `stream02` | RV musl+glibc `FAIL code=1`, `TFAIL=2`, `ENOSYS=1` in fallback/stream02 scouts. | Do not promote; likely missing syscall/path in stream workload. |
| `rwtest01`, `rwtest02`, `openfile01`, `writetest01`, `iogen01`, `fs_inod01` | RV musl+glibc `FAIL code=-1` in fallback/fs8 scouts, indicating missing/staging/run-dir failures rather than clean runtime behavior. | Do not promote; first audit testcase asset/helper staging. |
| `rwtest03+`, `fs_perms*` beyond `fs_perms01`, `inode` beyond `inode01/02` | No clean four-way evidence found in this lane's phase-a/b/c summary scan. | Unknown; targeted serial evidence required. |
| `inode01` | Already stable375 and covered by final RV+LA gates. | Already promoted. |
| `inode02` | RV musl+glibc `FAIL code=137`, `timeout=1` in `target-fs8-rv-001-summary.txt`. | Do not promote; timeout is a hard guardrail rejection. |

## Recommended next feasible work

1. For stable400/425 promotions, prefer other lanes' RV+LA × musl+glibc clean
   candidates rather than forcing the mmap/vma/fs-stress blockers above.
2. If this lane becomes a repair lane, start with low-risk staging/asset checks
   for `mmap10_1`, `rwtest*`, `openfile01`, `writetest01`, `iogen01`, and
   `fs_inod01` because their `code=-1` rows suggest missing testcase/run-dir
   issues, not necessarily kernel semantics.
3. Keep the mmap/mprotect/munmap cluster as high hidden-test value but require a
   real VM/signal repair followed by serialized RV+LA × musl+glibc targeted
   proof. Never convert `TFAIL/TBROK/TCONF`, timeout, `ENOSYS`, or wrapper
   non-zero rows into promotion-clean evidence.
4. Preserve the current marker-prefix/no-fake-pass/no-timeout-as-pass audit path
   for every promotion tranche, and keep `AxError::NotADirectory` counts in the
   final report because the remote glibc-only logs still carry about 4.5k lines
   per arch in the baseline artifact.

## Verification

| Check | Result |
| --- | --- |
| Worker protocol | ACK sent to `leader-fixed`; task 5 claimed by `worker-5` with claim token recorded in task state. |
| QEMU policy | No QEMU/run-eval command was started in this lane; report uses existing artifacts only. |
| Stable-list preflight | `python3` scan of `examples/shell/src/cmd.rs` found `375 total / 375 unique / 0 duplicates`. |
| Candidate evidence scan | Python table scan over phase-a/phase-b/phase-c `*summary*.txt` artifacts found blocker rows listed above; no clean new four-way candidate in this lane's assigned pool. |
| Guardrail source audit | `nl -ba examples/shell/src/cmd.rs` and `scripts/ltp_summary.py` inspected marker/timeout/parser lines cited above. |
| Marker-prefix/AxError audit | `raw/stable375-final-marker-prefix.txt` and `raw/remote-output-noise-baseline.json` inspected; counts summarized above. |
| Scope ownership | No edits to `examples/shell/src/cmd.rs::LTP_STABLE_CASES` or `.omx/ultragoal`; only this report file was added. |
| Subagent skip/evidence | Subagent spawn skipped: report-only lane had bounded read-only artifact scanning and no independent parallel subtask worth the overhead; evidence is the command outputs and report citations above. |
