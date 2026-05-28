# stable330 Promotion Gate Report

Date: 2026-05-25
Verdict: **PASS**

## Added cases

- `fchdir01`
- `fchdir03`
- `fcntl05`
- `fcntl05_64`
- `fcntl12`
- `fcntl12_64`
- `fcntl13`
- `fcntl13_64`
- `fdatasync01`
- `fdatasync02`
- `readlinkat01`
- `sched_setscheduler01`
- `sched_setscheduler02`
- `symlinkat01`
- `ftruncate03_64`

## Gate evidence

| Arch | Command class | Summary | Marker prefix | Result |
| --- | --- | --- | --- | --- |
| RV | stable aggregate | `raw/stable330-rv-aggregate-002-summary.txt` | `raw/stable330-rv-aggregate-002-marker-prefix.txt` | PASS 660 / FAIL 0; musl 330/0; glibc 330/0 |
| LA | stable aggregate | `raw/stable330-la-aggregate-001-summary.txt` | `raw/stable330-la-aggregate-001-marker-prefix.txt` | PASS 660 / FAIL 0; musl 330/0; glibc 330/0 |

## Internal health

- TFAIL: 0
- TBROK: 0
- TCONF: 4 per architecture, known `read02` only
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0
- marker-prefix bad lines: 0

## Promotion decision

Promote stable330. No evidence was derived from worker-parallel QEMU runs with shared images; aggregate promotion evidence is leader-owned and serial.
