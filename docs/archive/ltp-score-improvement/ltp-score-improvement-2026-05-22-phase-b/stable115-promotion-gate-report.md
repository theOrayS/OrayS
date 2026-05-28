# Stable115 promotion gate report — phase-b

Date: 2026-05-22

## Result

- Baseline: stable101 cases / libc / arch.
- Promoted result: **stable115 cases / libc / arch** (+14).
- Minimum success line stable115: **met**.
- Main target stable120: **not promoted** because fresh Wave B/C evidence left only 14 LA/RV x musl/glibc clean cases; remaining candidates had real TFAIL/TBROK/timeout/ENOSYS or architecture-specific blockers.
- Strict gate retained: no timeout counted as PASS; no case-name hardcode; no fake PASS; no silent SKIP.

## Newly promoted cases

- `dup202`
- `mkdirat01`
- `openat01`
- `pipe04`
- `pipe05`
- `pread01`
- `pwrite01`
- `sysinfo01`
- `faccessat01`
- `getgroups01`
- `setrlimit02`
- `sched_get_priority_max01`
- `sched_get_priority_min01`
- `sched_rr_get_interval01`

## Promotion evidence

The promoted cases were selected from Wave A2/B scheduler/near-clean results and then revalidated by stable115 targeted gates on both architectures.

| Gate | LA | RV |
| --- | --- | --- |
| PASS LTP CASE | 230 | 230 |
| FAIL LTP CASE | 0 | 0 |
| ltp-musl | 115/0 | 115/0 |
| ltp-glibc | 115/0 | 115/0 |
| internal TFAIL/TBROK/TCONF | TFAIL=0, TBROK=0, TCONF=4 | TFAIL=0, TBROK=0, TCONF=4 |
| timeout | 0 | 0 |
| ENOSYS/not implemented | 0 | 0 |
| panic/trap | 0 | 0 |


Evidence files:

- `wave-a2-fd-fs-proc-rv-summary.txt`
- `wave-a2-new-candidates-la-summary.txt`
- `wave-b-near-clean-rv-summary.txt`
- `wave-b-promotion-la-summary.txt`
- `wave-b-sched-v2-rv-summary.txt`
- `wave-b-sched-v2-la-summary.txt`
- `stable115-targeted-la-summary.txt`
- `stable115-targeted-rv-summary.txt`
- `final-gate-output-la-summary.txt`
- `final-gate-output-rv-summary.txt`

## Known TCONF caveat

`read02` remains pass_with_tconf on both libc variants and both architectures. Final stable gates report `TCONF=4` total, all from `read02`; this is transparent and is not claimed as a clean pass.

## Not promoted / blockers

- `getrlimit03`: RV clean but LA musl/glibc ENOSYS/missing legacy getrlimit syscall wrapper; not promoted.
- `unlinkat01`: RV clean but LA glibc wrapper/TBROK/order pollution; not promoted.
- `sched_getscheduler02`: RV clean but prior/current LA evidence not clean enough; historical LA musl TFAIL.
- `access02`: TFAIL on execute-file setup/ENOENT semantics.
- `access04`: TBROK, tmpfs mount returns EINVAL in harness environment.
- `dup03`: TFAIL, dup unexpectedly succeeded in negative case.
- `pipe02`: TFAIL, child signal/pipe kill semantics not correct.
- `lseek02`: mkfifo/fixture path hits ENOSYS.
- `readlinkat01/readlinkat02`: invalid input / ENOTDIR semantics still failing.
- `setrlimit01`: RV TFAIL/timeout under 20s targeted gate; not clean.
- `waitpid04/kill02/kill05/pause01/sigpending02/sigwait01/sigtimedwait01/setitimer01/getitimer01/nanosleep02`: Wave C had real TFAIL/TBROK/timeout/ENOSYS; not promoted.

## Aborted diagnostic gate

`stable109-targeted-la` with `LTP_CASE_TIMEOUT_SECS=20` was stopped after real timeout/TFAIL evidence. It is retained as diagnostic evidence only and was not used as a promotion gate.
