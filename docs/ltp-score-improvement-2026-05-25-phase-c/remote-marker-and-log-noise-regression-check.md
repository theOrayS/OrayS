# Remote marker and log-noise regression check

Date: 2026-05-25

## Marker guardrail

- Stable375 phase-b final marker-prefix evidence remains clean: `raw/stable375-final-marker-prefix.txt` has RV `markers=750 bad=0` and LA `markers=750 bad=0`.
- Phase-c remote glibc-only diagnostic outputs have `bad_marker_prefix=0` for both `Riscv输出.txt` and `LoongArch输出.txt` in `raw/remote-output-noise-baseline.json`.
- Post-fix local RV subset `raw/log-noise-rv-subset-002-noise-counts.json` has `markers=6 bad_marker_prefix=0`.

## Noise guardrail

| Evidence | fops NotADirectory | any NotADirectory | any IsADirectory | AlreadyExists | LTP semantic status |
| --- | ---: | ---: | ---: | ---: | --- |
| Remote RV baseline | 4432 | 4510 | 380 | 1 | parser-clean glibc-only stable375, known TCONF only |
| Remote LA baseline | 4433 | 4507 | 380 | 1 | parser-clean glibc-only stable375, known TCONF only |
| RV subset after fops-only patch | 0 | 0 | 16 | 0 | ftest07 timed out at 60s, not promotion evidence |
| RV subset after fops+root patch | 0 | 0 | 0 | 0 | PASS 6 / FAIL 0; known `read02` TCONF only |
| Aborted RV stable379 aggregate sample | 0 | 11 | 0 | 0 | not promotion evidence: existing `ftest03` timeout, PASS 360 / FAIL 1 before abort |

## Follow-up guardrail

After G002 scouts revealed adjacent expected `NotADirectory`, `IsADirectory`, and `AlreadyExists` warning paths, those paths were converted to direct `Err(AxError::...)` returns as well. This keeps syscall-visible errno unchanged and only removes the `ax_err!` warning side effect.

The attempted stable379 RV aggregate gate was aborted after `FAIL LTP CASE ftest03 : 137` / `TIMEOUT LTP CASE ftest03 after 60s`; LA aggregate was not run. It is therefore a blocker sample, not promotion evidence. The sample still shows `bad_marker_prefix=0`, `axfs::fops=0`, `AxError::IsADirectory=0`, and `AxError::AlreadyExists=0`; the remaining 11 `AxError::NotADirectory` entries are from `axfs_ramfs::file:69`, a separate lower-frequency source to triage later. Stable379/stable400/stable450 must not be claimed from this run.

## Decision

The log-noise repair is accepted as a first-stage output-volume fix. It does not claim stable450 promotion evidence. Promotion still requires fresh RV+LA x musl+glibc clean gates parsed by `scripts/ltp_summary.py`.
