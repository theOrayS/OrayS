# Milestone 03 stable656 report - G009 scout blocker record

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Baseline before this scout: `606 total / 606 unique / 0 duplicate`
Target milestone: stable656
Status: **not achieved; no promotion performed**

## Objective

Scout the first G009 `mm/mmap/resource` + `futex/thread` candidates after stable606 without weakening the promotion gate. This milestone can only advance when candidates are RV + LA x musl + glibc wrapper PASS and parser-clean through `scripts/ltp_summary.py`.

## What was run

RV targeted scout over six candidates:

- `mmap05`
- `munmap01`
- `mmap10_1`
- `mmap13`
- `vma02`
- `futex_wait03`

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.log`
- Run meta: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.log.meta`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.txt`
- JSON summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.json`
- Promotion candidate report: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.promotion-candidates.txt`
- Derived checksums: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.derived.sha256`

## Evidence summary

Parser result:

- PASS LTP CASE: 0
- FAIL LTP CASE: 12
- Internal TFAIL/TBROK/TCONF: 8 (`TBROK=2`, `TFAIL=2`, `TCONF=4`)
- timeout matches: 2
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- Promotion candidates: 0
- Blocked/incomplete cases: 6

Per-case classification:

| Case | Result | Promotion decision |
| --- | --- | --- |
| `mmap05` | both libcs FAIL with `TBROK=1`; test killed by SIGSEGV | blocked; likely user-fault signal delivery / `PROT_NONE` semantics lane |
| `munmap01` | both libcs FAIL code 139 | blocked; likely recoverable user SIGSEGV handler delivery lane |
| `mmap10_1` | missing testcase in both guest LTP trees | not countable; inventory mismatch, not a kernel PASS |
| `mmap13` | both libcs FAIL with `TFAIL=1`; SIGBUS signal not received | blocked; file-backed mmap SIGBUS-on-EOF semantics lane |
| `vma02` | both libcs TCONF due libnuma requirement | not promotion evidence; environment/configuration TCONF |
| `futex_wait03` | both libcs timeout and killed after 90s | blocked; futex wait timeout/wakeup semantics lane |

## Conclusion

No case from this scout is eligible for stable656. The stable list remains at `606 total / 606 unique / 0 duplicate`; no stable promotion, blacklist change, or Ultragoal completion checkpoint is warranted from this evidence.

## Risks / next steps

1. Repair or deeply diagnose recoverable user page-fault signal delivery before retesting `mmap05` and `munmap01`.
2. Treat `mmap13` as a separate file-backed mmap/SIGBUS-on-EOF semantics lane.
3. Treat `futex_wait03` as a futex timeout/wakeup lifetime lane; do not rerun broad futex shards concurrently with default QEMU images.
4. Exclude `mmap10_1` from promotion candidates unless the guest LTP inventory genuinely contains it.
5. Keep `vma02` out of stable promotion unless its libnuma TCONF condition is resolved and parser-clean evidence exists.
