# stable315 Promotion Gate Report

Date: 2026-05-25
Verdict: **PASS**

## Added cases

- `alarm05`
- `alarm07`
- `write05`
- `gettimeofday02`
- `waitpid01`
- `pipe2_02`
- `sched_getscheduler02`
- `fstat03`
- `fstat03_64`
- `statfs02`
- `fstatfs02`
- `fstatfs02_64`
- `sched_getparam03`
- `sched_setparam04`
- `sched_setparam05`

## Gate evidence

| Arch | Command class | Summary | Marker prefix | Result |
| --- | --- | --- | --- | --- |
| RV | stable aggregate | `raw/stable315-rv-aggregate-002-summary.txt` | `raw/stable315-rv-aggregate-002-marker-prefix.txt` | PASS 630 / FAIL 0; musl 315/0; glibc 315/0 |
| LA | stable aggregate | `raw/stable315-la-aggregate-001-summary.txt` | `raw/stable315-la-aggregate-001-marker-prefix.txt` | PASS 630 / FAIL 0; musl 315/0; glibc 315/0 |

## Internal health

- TFAIL: 0
- TBROK: 0
- TCONF: 4 per architecture, known `read02` only
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0
- marker-prefix bad lines: 0

## Promotion decision

Promote stable315. No evidence was derived from worker-parallel QEMU runs with shared images; aggregate promotion evidence is leader-owned and serial.
