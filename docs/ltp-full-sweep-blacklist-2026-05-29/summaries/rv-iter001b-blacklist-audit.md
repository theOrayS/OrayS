# RV iter001b blacklist evidence audit

Date: 2026-05-29
Worker: `worker-3` / task 6 (`blacklist evidence auditor`)
Scope: audit proposed/necessary blacklist additions for the RV `LTP_CASES=blacklist` sweep. This report does **not** count blacklist/SKIP as PASS and does **not** promote stable cases.

## Decision

- **Accepted severe blocker candidate:** `pthserv`
- **Recommended blacklist scope:** `rv-observed`; treat as common only after LA or a second RV/LA-confirming run shows the same blocking behavior.
- **Rejected as blacklist material in this audit:** ordinary wrapper failures, `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and bounded per-case timeouts that closed with `FAIL LTP CASE ... : 137` and allowed the sweep to continue.
- **Current supplemental blacklist entries before this audit:** 0 non-comment entries in `/root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt`.
- **Mutation note:** worker-3 did not directly edit the shared blacklist file; this audit gives the leader a severe-blocker-only acceptance record with evidence and a removal condition.

## Run closure evidence

- Raw log: `/root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter001b.log`
- Log SHA-256: `617e674f5255d20ad8a5276131956d273a3fcc713da98ee2562ad70f76ad2b37`
- Status JSON: `/root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter001b.status.json`
- Runner status: `exit_code=0`, started `2026-05-29T10:55:58Z`, ended `2026-05-29T11:48:26Z`
- Closure check: `lsof <raw-log>` returned no open writer before content audit.
- Disk check: `df -h / /root` stayed at `23G used / 34G avail / 41%` before and after.

Marker/parser counts:

| Field | Count |
| --- | ---: |
| `RUN LTP CASE` | 1268 |
| wrapper pass (`FAIL ... : 0`) | 348 |
| wrapper fail (non-zero) | 919 |
| `TIMEOUT LTP CASE` | 17 |
| `[CONTEST][LTP][SKIP]` | 0 |
| incomplete `RUN` markers | 1 (`pthserv`) |
| internal `TFAIL` | 1425 |
| internal `TBROK` | 351 |
| internal `TCONF` | 487 |
| ENOSYS/not implemented matches | 246 |
| panic/trap matches | 0 |

Parser artifacts committed with this report:

- `rv-iter001b-summary.txt` — `python3 scripts/ltp_summary.py <raw-log>` output.
- `rv-iter001b-summary-compact.json` — compacted `--json` counts, timeout rows, and unknown rows.
- `rv-iter001b-marker-audit.json` — marker/incomplete/last-marker evidence.
- `rv-iter001b.log.sha256` — raw-log hash.

## Accepted blocker: `pthserv`

| Required field | Evidence |
| --- | --- |
| Case | `pthserv` |
| Reason category | QEMU/guest hang / sweep-blocking incomplete marker |
| First evidence path/location | `rv-iter001b.log` lines 25290-25295; `rv-iter001b-marker-audit.json` (`incomplete_cases=["pthserv"]`) |
| Scope | `rv-observed`; do not mark universal until confirmed on LA or a repeated RV/LA comparison |
| Removal condition | Remove from supplemental blacklist after `pthserv` completes with an explicit `PASS/FAIL/TIMEOUT LTP CASE` marker without manual QEMU termination in a full-sweep or targeted RV gate; then re-check LA separately before treating it as common/unblocked. |

Relevant closed-log tail:

```text
25290: ========== START ltp pthserv ==========
25291: RUN LTP CASE pthserv
25292: LTP MEMORY pthserv before: free_frames=98729 allocated_frames=162507
25293: WORKER_META blocker=pthserv action=terminate_stalled_qemu at=2026-05-29T11:48:21Z reason="no log growth since 2026-05-29T11:42:10Z; incomplete RUN marker without closure"
25294: qemu-system-riscv64: terminating on signal 15 from pid 635107 (<unknown process>)
25295: RUN_META exit_code=0 end=2026-05-29T11:48:26Z
```

This is acceptable blacklist evidence because the case left an incomplete `RUN LTP CASE` marker and required external QEMU termination to end the sweep. It is not an ordinary semantic failure.

## Explicit non-accepted classes

These are **not** accepted as blacklist additions in this audit:

- 919 non-zero wrapper failures as a class: they remain failure data for targeted fixing.
- 2263 internal `TFAIL/TBROK/TCONF` signals: they are quality/semantic signals, not skip evidence.
- 246 ENOSYS/not-implemented matches: keep as failures unless they block the sweep.
- 17 timeout markers with closed wrapper markers (`code=137`): `clock_gettime01`, `clock_nanosleep01`, `fork13`, `fork14`, `ftest03`, `futex_wait03`, `futex_wait05`, `futex_wake03`, `getrusage04`, `inode02`, `kill10`, `mmap3`, `oom01`, `pause01`, `pause03`, `pth_str01`, `pth_str03`. They are recorded as timeout failures; this audit does not upgrade them to blacklist entries because the sweep continued after each one.
- Panic/trap keyword total from parser: 0. Keyword false positives such as `EFAULT` TPASS lines are not severe blockers.

## Subagent review integration

- Subagents spawned: 1 (`019e7368-605d-7291-b62c-0eb189e614c1`, task name: task 6 review probe)
- Subagent model: `gpt-5.4-mini`
- Findings integrated:
  - Confirmed the report must include sweep mode, blacklist accounting, run closure, parser counts, and removal conditions.
  - Confirmed invalid blacklist recommendations include ordinary `TFAIL/TBROK/TCONF/ENOSYS/wrong errno`, pass-rate hiding, and any conclusion based on an open log.
  - Independently observed the log was still open before closure; local audit waited until `lsof` was clean and status JSON existed.
- Serial searches before spawn: 2 (inbox/task read and active log metadata check)

## Auditor conclusion

`pthserv` is the only accepted severe blocker candidate from RV iter001b. All other observed failures/timeouts stay in the failure matrix and must not be added to the blacklist merely to improve pass rate.
