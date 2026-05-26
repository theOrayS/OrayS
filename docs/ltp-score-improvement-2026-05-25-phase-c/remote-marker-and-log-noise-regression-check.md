# Remote marker and log-noise regression check

Date: 2026-05-26

## Marker guardrail

- Stable375 phase-b final marker-prefix evidence remains clean: `raw/stable375-final-marker-prefix.txt` had RV `markers=750 bad=0` and LA `markers=750 bad=0`.
- Phase-c remote glibc-only diagnostic outputs had `bad_marker_prefix=0` for both `Riscv输出.txt` and `LoongArch输出.txt` in `raw/remote-output-noise-baseline.json`.
- Post-fix local RV subset `raw/log-noise-rv-subset-002-noise-counts.json` had `markers=6 bad_marker_prefix=0`.
- Accepted stable379 aggregate gates have marker prefix clean:
  - RV `raw/stable379-rv-gate-002.log`: `markers=758 bad=0`.
  - LA `raw/stable379-la-gate-001.log`: `markers=758 bad=0`.

## Noise guardrail

| Evidence | fops NotADirectory | any NotADirectory | any IsADirectory | AlreadyExists | LTP semantic status |
| --- | ---: | ---: | ---: | ---: | --- |
| Remote RV baseline | 4432 | 4510 | 380 | 1 | parser-clean glibc-only stable375, known TCONF only |
| Remote LA baseline | 4433 | 4507 | 380 | 1 | parser-clean glibc-only stable375, known TCONF only |
| RV subset after fops-only patch | 0 | 0 | 16 | 0 | ftest07 timed out at 60s, not promotion evidence |
| RV subset after fops+root patch | 0 | 0 | 0 | 0 | PASS 6 / FAIL 0; known `read02` TCONF only |
| RV accepted stable379 aggregate | 0 | 22 | 0 | 0 | PASS 758 / FAIL 0; parser timeout/ENOSYS/panic 0; known `read02` TCONF only |
| LA accepted stable379 aggregate | 0 | 22 | 0 | 0 | PASS 758 / FAIL 0; parser timeout/ENOSYS/panic 0; known `read02` TCONF only |

## Follow-up guardrail

After G002 scouts revealed adjacent expected `NotADirectory`, `IsADirectory`, and `AlreadyExists` warning paths, those paths were converted to direct `Err(AxError::...)` returns. This keeps syscall-visible errno unchanged and only removes the `ax_err!` warning side effect.

The original high-frequency `axfs::fops:297 [AxError::NotADirectory]` source is 0 in both accepted stable379 aggregate logs. Remaining `AxError::NotADirectory` entries are from `axfs_ramfs::file:69`, most visibly statfs/statvfs/truncate negative paths; triage later if remote output volume still needs more reduction.

LA stable379 raw log contains two inherited LTP internal `Test timeouted, sending SIGKILL!` notices in pre-existing long-running cases. `scripts/ltp_summary.py` reports wrapper timeout matches 0 and both cases passed, but the notices are disclosed here so they are not hidden as clean new promotion evidence.

## Decision

The log-noise repair is accepted as a first-stage output-volume fix and stable381 partial promotion evidence is accepted. Stable400/stable425/stable450 still require more fresh clean candidates and serial aggregate gates.


## Stable381 update

| Evidence | fops NotADirectory | any NotADirectory | any IsADirectory | AlreadyExists | marker bad prefix | LTP semantic status |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| RV accepted stable381 aggregate | 0 | 22 | 0 | 0 | 0 | PASS 762 / FAIL 0; `ltp-musl` 381/0; `ltp-glibc` 381/0; known `read02` TCONF only |
| LA accepted stable381 aggregate | 0 | 22 | 0 | 0 | 0 | PASS 762 / FAIL 0; `ltp-musl` 381/0; `ltp-glibc` 381/0; known `read02` TCONF only |

Stable381 preserves the original `axfs::fops` high-frequency warning fix: `axfs::fops` warning count remains 0 on both architectures. The remaining 22 `AxError::NotADirectory` entries per architecture are the already-disclosed `axfs_ramfs::file:69` family, not the fixed `fops.rs:297` remote-output flood.
