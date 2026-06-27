# RV iter003 blacklist eligibility audit

Date: 2026-05-29T17:57:18Z
Worker: `worker-3` / task 16
Raw log: `/root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter003.log`
Log SHA-256: `bd702ce941c5a5b43d9162f1168bf9b08d04c59c6503b989e74e0abd654f1e6d`

## Closure / marker evidence

| Field | Value |
| --- | --- |
| RUN | 1597 |
| raw FAIL markers | 1596 |
| raw FAIL status=0 | 477 |
| raw FAIL nonzero | 1119 |
| TIMEOUT markers | 25 |
| SKIP markers | 0 |
| incomplete cases | shmat1 |
| last RUN | shmat1 |
| last DONE | shmat04 |
| RUN_META tail | RUN_META exit_code=2 end=2026-05-29T17:57:07Z |
| WORKER_META count | 0 |

## Decision

- Accepted severe blocker candidates for leader review:
  - `shmat1` — incomplete RUN / sweep-blocking closure; evidence: rv-iter003.log tail and marker audit; scope: rv-observed; recommend generic only after leader review; removal condition: targeted case completes with PASS/FAIL/TIMEOUT marker without QEMU/runner stall, OOM, or external termination.

## Rejected / non-blacklist classes

- Ordinary wrapper failures, `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and closed per-case timeouts are not blacklist material.

## Severe keyword / metadata tail

- line 16809: `kill06.c:43: TPASS: receive expected signal SIGKILL(9)`
- line 16847: `kill09      1  TPASS  :  kill(4565, SIGKILL) returned 0`
- line 16880: `kill11.c:98: TPASS: signal SIGKILL         `
- line 18538: `mincore01    3  TFAIL  :  mincore01.c:200: mincore failed unexpectedly; expected: 12 - Out of memory: TEST_ERRNO=ENOSYS(38): Function not implemented`
- line 18539: `mincore01    4  TFAIL  :  mincore01.c:200: mincore failed unexpectedly; expected: 12 - Out of memory: TEST_ERRNO=ENOSYS(38): Function not implemented`
- line 19833: `test5(): sbrk(): Out of memory`
- line 19836: `test6(): sbrk(): Out of memory`
- line 21027: `msync03     6  TPASS  :  msync failed as expected: TEST_ERRNO=ENOMEM(12): Out of memory`
- line 25625: `ptrace05   10  TFAIL  :  ptrace05.c:135: Exited unexpectedly instead of dying with SIGKILL.`
- line 27032: `remap_file_pages01    2  TFAIL  :  remap_file_pages01.c:163: mmap Error, errno=12 : Out of memory`
- line 34105: `make: *** [Makefile:373: run-rv] Killed`

## Raw tail excerpt

```text
e 1171456
  [0x10000cbb08]: Map address = 0x100a306000
  [0x10000cbb08]: Num iter: [82] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100a306000]
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100a425000
  [0x10000cbb08]: Num iter: [83] Total Num Iter: [1000]
      [0x100030bb08]: read_from_mem():  memory address: [0x100a425000]
      [0x100030bb08]: read_mem(): content of memory: X
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100a544000
  [0x10000cbb08]: Num iter: [84] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100a544000]
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100a663000
  [0x10000cbb08]: Num iter: [85] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100a663000]
      [0x100030bb08]: read_from_mem():  memory address: [0x100a663000]
      [0x100030bb08]: read_mem(): content of memory: Y
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100a782000
  [0x10000cbb08]: Num iter: [86] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100a782000]
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100a8a1000
  [0x10000cbb08]: Num iter: [87] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100a8a1000]
      [0x100030bb08]: read_from_mem():  memory address: [0x100a8a1000]
      [0x100030bb08]: read_mem(): content of memory: Y
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100a9c0000
  [0x10000cbb08]: Num iter: [88] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100a9c0000]
      [0x100030bb08]: read_from_mem():  memory address: [0x100a9c0000]
      [0x100030bb08]: read_mem(): content of memory: Y
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100aadf000
  [0x10000cbb08]: Num iter: [89] Total Num Iter: [1000]
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100abfe000
  [0x10000cbb08]: Num iter: [90] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100abfe000]
      [0x100030bb08]: read_from_mem():  memory address: [0x100abfe000]
      [0x100030bb08]: read_mem(): content of memory: Y
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100ad1d000
  [0x10000cbb08]: Num iter: [91] Total Num Iter: [1000]
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100ae3c000
  [0x10000cbb08]: Num iter: [92] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100ae3c000]
      [0x100030bb08]: read_from_mem():  memory address: [0x100ae3c000]
      [0x100030bb08]: read_mem(): content of memory: Y
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100af5b000
  [0x10000cbb08]: Num iter: [93] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100af5b000]
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100b07a000
  [0x10000cbb08]: Num iter: [94] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100b07a000]
      [0x100030bb08]: read_from_mem():  memory address: [0x100b07a000]
      [0x100030bb08]: read_mem(): content of memory: Y
  [0x10000cbb08]: shmget(): success, got segment of size 1171456
  [0x10000cbb08]: Map address = 0x100b199000
  [0x10000cbb08]: Num iter: [95] Total Num Iter: [1000]
    [0x10001ebb08]: write_to_mem(): memory address: [0x100b199000]
make: *** [Makefile:373: run-rv] Killed
RUN_META run_eval_status=2
RUN_META exit_code=2 end=2026-05-29T17:57:07Z
```
