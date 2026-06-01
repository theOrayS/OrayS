# Milestone 03 stable656 no-promotion reason

No case from the 2026-06-02 G009 scout is eligible for stable promotion.

## Blocking evidence

Promotion candidate report:

- Required arches: RV only for this scout
- Required libcs: glibc, musl
- Promotion candidates: 0
- Blocked/incomplete cases: 6

Blocked rows:

| Case | Reason |
| --- | --- |
| `futex_wait03` | both libcs timeout and wrapper FAIL |
| `mmap05` | both libcs wrapper FAIL with `TBROK`; test killed by SIGSEGV |
| `mmap10_1` | missing testcase in both guest LTP trees; not countable |
| `mmap13` | both libcs wrapper FAIL with `TFAIL`; SIGBUS signal not received |
| `munmap01` | both libcs wrapper FAIL code 139 |
| `vma02` | both libcs wrapper FAIL with `TCONF`; libnuma requirement not satisfied |

## Decision

- Do not edit `LTP_STABLE_CASES`.
- Do not treat blacklist/SKIP/status0/timeout/TCONF/TBROK/TFAIL rows as PASS.
- Do not run LA or stable regression promotion gates for these rows until a fresh RV run has parser-clean wrapper PASS for both libcs.
