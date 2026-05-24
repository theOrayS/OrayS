# stable315 promotion gate report

Date: 2026-05-25
Target: stable300 -> stable315
Result: **NOT PROMOTED**

## Outcome

`LTP_STABLE_CASES` remains **300 total / 300 unique / 0 duplicates**. No 15-case clean tranche was found; after the follow-up `sched_getscheduler02` and `waitpid01` fixes there are 8 clean seeds, still below +15.

## Fresh follow-up evidence

- RV targeted `raw/followup-rv-targeted-001-summary.txt` over 8 candidates:
  - `PASS LTP CASE 13`, `FAIL LTP CASE 3`.
  - `ltp-musl 6/2`, `ltp-glibc 7/1`.
  - Internal `TBROK=2`, `TFAIL=40`; timeout/ENOSYS/panic/trap 0.
  - RV-clean musl+glibc subset: `prctl05,sched_getscheduler02,sethostname01,setrlimit01,signal03,signal04`.
- LA targeted `raw/followup-la-targeted-004-summary.txt` over that RV-clean subset:
  - `PASS LTP CASE 11`, `FAIL LTP CASE 1`.
  - `ltp-musl 5/1`, `ltp-glibc 6/0`.
  - Internal `TFAIL=1` in `sched_getscheduler02` on LA/musl; timeout/ENOSYS/panic/trap 0.
  - Pre-fix four-way clean subset: `prctl05,sethostname01,setrlimit01,signal03,signal04`.
- Follow-up LA targeted `raw/followup-la-sched_getscheduler02-afterfix-001-summary.txt`: parser semantic PASS 2 / FAIL 0, `ltp-musl 1/0`, `ltp-glibc 1/0`, internal TFAIL/TBROK/TCONF=0, timeout/ENOSYS/panic/trap=0.
- Follow-up waitpid targeted gates:
  - RV `raw/followup-rv-waitpid01-maskrestore-001-summary.txt`: PASS 2 / FAIL 0, `ltp-musl 1/0`, `ltp-glibc 1/0`, internal TFAIL/TBROK/TCONF=0, timeout/ENOSYS/panic/trap=0.
  - LA `raw/followup-la-waitpid01-maskrestore-001-summary.txt`: PASS 2 / FAIL 0, `ltp-musl 1/0`, `ltp-glibc 1/0`, internal TFAIL/TBROK/TCONF=0, timeout/ENOSYS/panic/trap=0.
- Follow-up waitpid/signal guard gates:
  - RV `raw/followup-rv-waitpid-signal-guard-001-summary.txt`: PASS 16 / FAIL 0, both libc 8/0, internal TFAIL/TBROK/TCONF=0, timeout/ENOSYS/panic/trap=0.
  - LA `raw/followup-la-waitpid-signal-guard-001-summary.txt`: PASS 16 / FAIL 0, both libc 8/0, internal TFAIL/TBROK/TCONF=0, timeout/ENOSYS/panic/trap=0.
- `pipe2_02` pre-fix follow-up `raw/followup-rv-pipe2_02-resource-prestage-003-summary.txt`: PASS 0 / FAIL 2, both libc `TBROK=1` from helper copy/resource setup; superseded by the `/bin/sh` fix.
- `pipe2_02` after `/bin/sh` compatibility fix:
  - RV `raw/followup-rv-pipe2_02-binsh-001-summary.txt`: PASS 2 / FAIL 0, `ltp-musl 1/0`, `ltp-glibc 1/0`, internal TFAIL/TBROK/TCONF=0.
  - LA `raw/followup-la-pipe2_02-binsh-001-summary.txt`: PASS 2 / FAIL 0, `ltp-musl 1/0`, `ltp-glibc 1/0`, internal TFAIL/TBROK/TCONF=0.
- Marker-prefix follow-up checks: `raw/followup-marker-prefix-check.txt` and `raw/followup-waitpid-marker-prefix-check.txt` report no bad marker prefixes.
- LA attempts `followup-la-targeted-001/002/003` were aborted/untrusted due duplicated starts and are excluded from promotion evidence.

## Baseline and earlier discovery

- Baseline stable300 final summaries from `docs/ltp-score-improvement-2026-05-24-phase-a/` remain the last clean aggregate gate: RV and LA each `PASS LTP CASE 600`, `FAIL 0`, `ltp-musl 300/0`, `ltp-glibc 300/0`, with transparent known `read02` TCONF=4.
- Batch-A RV discovery (`raw/batch-a-rv-summary.txt`): `PASS LTP CASE 14`, `FAIL LTP CASE 12`, `ltp-musl 2 passed / 11 failed`, `ltp-glibc 12 passed / 1 failed`, internal `TFAIL=11`, `TBROK=2`.
- User-priority blocker RV discovery (`raw/blocker-batch-rv-summary.txt`): `PASS LTP CASE 2`, `FAIL LTP CASE 24`, internal `TFAIL=65`, `TBROK=13`, `TCONF=2`, ENOSYS=2.

## Gate decision

Blocked. There are only 8 fresh RV+LA x musl+glibc clean candidate cases (`prctl05,sched_getscheduler02,sethostname01,setrlimit01,signal03,signal04,waitpid01,pipe2_02`), below the +15 stable315 tranche threshold. No stable aggregate gate was run and no stable list entry was added.
