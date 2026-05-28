# Worker 1 task 6 — batch evidence delta report

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Worker: `worker-1`
Scope: integrate leader-provided batch summaries into the stable413->460 candidate evidence. No QEMU/evaluator was run by this worker, `.omx/ultragoal` was not touched, and `examples/shell/src/cmd.rs::LTP_STABLE_CASES` was not edited.

## Evidence inputs

The leader-generated summaries were copied into this worker tree so the delta is durable with the report:

| File | Lines | SHA-256 |
| --- | ---: | --- |
| `docs/ltp-score-improvement-2026-05-27-phase-a/raw/batch-001-rv-inline-summary.txt` | 214 | `85d4e3658a064e19e3970e7f89772d84db03961ddb2d0f9ed9eb47a40be25b82` |
| `docs/ltp-score-improvement-2026-05-27-phase-a/raw/batch-001-la-confirm-summary.txt` | 54 | `48f862f900fbe9ebdafa18e761f33b39b3c0a818ae7d886abbc80dd541a62945` |
| `docs/ltp-score-improvement-2026-05-27-phase-a/raw/batch-001-cross-promotion-candidates.txt` | 59 | `f0764cabd0baf6c2e4d8a448d9484d8b5786753e2d0cbcf283fb408a93937aad` |
| `docs/ltp-score-improvement-2026-05-27-phase-a/raw/batch-002-rv-summary.txt` | 33 | `db77294924273e82fac0d67e360e452a9ee901ca47bf60c52e04edd732c8e26e` |

## Delta summary

- Batch 001 RV inline is not broadly clean: `PASS LTP CASE: 6`, `FAIL LTP CASE: 80`, internal `TFAIL/TBROK/TCONF: 365`, `ENOSYS/not implemented matches: 8`, and `panic/trap matches: 0`.
- Batch 001 LA confirmation has `PASS LTP CASE: 5`, `FAIL LTP CASE: 1`, and the sole internal failure is `la:musl:readlinkat02` with `TFAIL=1`.
- The cross-arch/libc promotion report has exactly **2 promotion candidates**: `fcntl07` and `fcntl07_64`.
- `readlinkat02` is explicitly **not** promotion-ready: RV musl/glibc and LA glibc are clean, but LA musl fails with wrapper `FAIL` and internal `TFAIL=1`.
- Batch 002 has **no promotion value**: it records `rv:musl:pipe02` as `UNKNOWN` with `panic/trap=1`; do not promote any case from that batch and do not rerun the same batch shape until `pipe02` is isolated or removed.

## Promotion/readiness table

| Case | Task-6 classification | Evidence | Action |
| --- | --- | --- | --- |
| `fcntl07` | Four-way clean promotion candidate. RV musl, RV glibc, LA musl, and LA glibc are all wrapper `PASS` with zero internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, or panic/trap. | `batch-001-rv-inline-summary.txt`; `batch-001-la-confirm-summary.txt`; `batch-001-cross-promotion-candidates.txt` | Leader can include in the next stable-list promotion only after preserving the usual final RV+LA parser gate. |
| `fcntl07_64` | Four-way clean promotion candidate. RV musl, RV glibc, LA musl, and LA glibc are all wrapper `PASS` with zero internal blockers. | `batch-001-rv-inline-summary.txt`; `batch-001-la-confirm-summary.txt`; `batch-001-cross-promotion-candidates.txt` | Same as `fcntl07`: promotion-ready evidence exists, final gate remains leader-owned. |
| `readlinkat02` | Blocked, 3/4 clean only. LA musl is wrapper `FAIL` with `TFAIL=1`. | `batch-001-rv-inline-summary.txt`; `batch-001-la-confirm-summary.txt`; `batch-001-cross-promotion-candidates.txt` | Exclude from promotion until LA musl is repaired and revalidated. |
| `pipe02` | Quarantined. Batch 002 hit `panic/trap=1` before a PASS/FAIL row. | `batch-002-rv-summary.txt` | Exclude from promotion and from mixed scout batches; isolate as a debugger task if the leader wants to pursue it. |

## Batch-001 blocked classes to keep out of promotion

The non-candidate rows in batch 001 remain blockers, not partial credit. Examples that should not be laundered into stable-list edits:

- VFS/path rows such as `access04`, `chmod06`, `chmod07`, `openat02`, `openat03`, `rename01`, `rename03`, `rename04`, `rename05`, `statx01`, `statx03` still fail RV with `TBROK` or `TFAIL`.
- Metadata rows such as `fstat02`, `fstat02_64`, `getdents01`, `getdents02`, and `link02` still include ENOSYS/not-implemented markers in RV summaries.
- `fcntl11` and `fcntl11_64` still fail with large internal `TFAIL=66` counts on both RV libcs; this confirms advisory-lock-style fcntl cases are not part of the easy promotion set.
- `writev03` is still blocked by internal `TCONF=1` on both RV libcs and must not be treated as clean.

## Recommended next leader-run batches

### Promotion gate candidate set

Use this only as a final confirmation/promotion gate, not as discovery:

```text
fcntl07,fcntl07_64
```

Stop/demotion rule: any wrapper `FAIL`, internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS/not-implemented, panic/trap, or marker-prefix anomaly demotes the row back to blocked.

### Next scout batch excluding known panic/blockers

For discovery after the promotion gate, avoid `pipe02`, avoid `readlinkat02`, and avoid the batch-001 failed rows unless a lane owner has a repair to validate. The lowest-risk scout candidates still supported by existing worker reports are:

```text
pipe07,sendfile07,sendfile07_64
```

Rationale:

- `pipe07` was identified by Worker 4 as a plausible FD-exhaustion scout and is not the `pipe02` panic case.
- `sendfile07`/`sendfile07_64` exercise socket-backed nonblocking `EAGAIN` behavior and should be diagnostic after the light FD gate, not mixed with panic/blocker rows.
- Do not include `pipe02` in mixed batches until the panic is isolated; a panic invalidates the whole batch for promotion accounting.

Optional repair-validation batches should be separate from the clean scout batch. For example, `open06` may be revalidated after Worker 4's FIFO `ENXIO` repair, and `statfs03/statfs03_64/getdents02` may be revalidated after the metadata lane repair, but these are repair checks rather than fresh promotion scouts.

## Matrix update note

`candidate-matrix-stable413-to-460.md` is updated with a task-6 delta pointer and corrected high-level rows for `fcntl07`, `fcntl07_64`, and `readlinkat02`. The matrix remains advisory; the actual stable-list edit is still leader-owned and must be gated by final RV+LA parser output.
