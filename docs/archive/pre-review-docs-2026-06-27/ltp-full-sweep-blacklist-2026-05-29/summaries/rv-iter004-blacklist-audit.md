# RV iter004 blacklist eligibility audit

Date: 2026-05-29T19:35:55Z
Worker: `worker-3` / task 19
Raw log: `/root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter004.log`
Log SHA-256: `5f60c62c916990254ba827cdba006c5818dc12dbec790d6e34ed4df8214031aa`

## Closure / marker evidence

| Field | Value |
| --- | --- |
| RUN | 2334 |
| raw FAIL markers | 2333 |
| raw FAIL status=0 | 593 |
| raw FAIL nonzero | 1740 |
| TIMEOUT markers | 31 |
| SKIP markers | 0 |
| incomplete cases | accept02 |
| last RUN | accept02 |
| last DONE | accept01 |
| RUN_META tail | RUN_META exit_code=0 end=2026-05-29T19:35:26Z |
| WORKER_META count | 0 |

## Decision

- Accepted severe blocker candidates for leader review:
  - `accept02` — incomplete RUN / sweep-blocking closure; evidence: rv-iter004.log tail and marker audit; scope: rv-observed; recommend generic only after leader review; removal condition: targeted case completes with PASS/FAIL/TIMEOUT marker without QEMU/runner stall, OOM, or external termination.

## Rejected / non-blacklist classes

- Ordinary wrapper failures, `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and closed per-case timeouts are not blacklist material.

## Severe keyword / metadata tail

- line 16786: `kill06.c:43: TPASS: receive expected signal SIGKILL(9)`
- line 16824: `kill09      1  TPASS  :  kill(5046, SIGKILL) returned 0`
- line 16857: `kill11.c:98: TPASS: signal SIGKILL         `
- line 18515: `mincore01    3  TFAIL  :  mincore01.c:200: mincore failed unexpectedly; expected: 12 - Out of memory: TEST_ERRNO=ENOSYS(38): Function not implemented`
- line 18516: `mincore01    4  TFAIL  :  mincore01.c:200: mincore failed unexpectedly; expected: 12 - Out of memory: TEST_ERRNO=ENOSYS(38): Function not implemented`
- line 19905: `test5(): sbrk(): Out of memory`
- line 19908: `test6(): sbrk(): Out of memory`
- line 21099: `msync03     6  TPASS  :  msync failed as expected: TEST_ERRNO=ENOMEM(12): Out of memory`
- line 25420: `ptrace05   10  TFAIL  :  ptrace05.c:135: Exited unexpectedly instead of dying with SIGKILL.`
- line 26827: `remap_file_pages01    2  TFAIL  :  remap_file_pages01.c:163: mmap Error, errno=12 : Out of memory`
- line 33777: `sbrk: Out of memory`
- line 33878: `signal01.c:56: TPASS: (long)signal(SIGKILL, tc->sighandler) : EINVAL (22)`
- line 33879: `signal01.c:56: TPASS: (long)signal(SIGKILL, tc->sighandler) : EINVAL (22)`
- line 33880: `signal01.c:56: TPASS: (long)signal(SIGKILL, tc->sighandler) : EINVAL (22)`
- line 43005: `waitid11.c:35: TPASS: infop->si_status == SIGKILL (9)`
- line 43056: `waitpid01.c:129: TPASS: WTERMSIG() == SIGKILL`
- line 43131: `waitpid01.c:129: TPASS: WTERMSIG() == SIGKILL`

## Raw tail excerpt

```text
RUN LTP CASE abort01
LTP MEMORY abort01 before: free_frames=75494 allocated_frames=185741
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
abort01.c:62: TPASS: abort() dumped core
abort01.c:65: TPASS: abort() raised SIGIOT

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
FAIL LTP CASE abort01 : 0
Pass!
LTP MEMORY abort01 after_run: free_frames=75473 allocated_frames=185762
LTP MEMORY abort01 after_cleanup: free_frames=75473 allocated_frames=185762
LTP CASE RUNTIME abort01: 2184 ms
========== END ltp abort01 ==========
========== START ltp abs01 ==========
RUN LTP CASE abs01
LTP MEMORY abs01 before: free_frames=75473 allocated_frames=185762
abs01       1  TPASS  :  Test passed
abs01       2  TPASS  :  Test passed
abs01       3  TPASS  :  Test passed
FAIL LTP CASE abs01 : 0
Pass!
LTP MEMORY abs01 after_run: free_frames=75466 allocated_frames=185769
LTP MEMORY abs01 after_cleanup: free_frames=75466 allocated_frames=185769
LTP CASE RUNTIME abs01: 1790 ms
========== END ltp abs01 ==========
========== START ltp accept01 ==========
RUN LTP CASE accept01
LTP MEMORY accept01 before: free_frames=75466 allocated_frames=185769
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
accept01.c:92: TPASS: bad file descriptor : EBADF (9)
[37m[5177.859927 0:28548 axnet::smoltcp_impl::tcp:284] [33m[AxError::InvalidInput] socket accept() failed: not listen[m[m
[37m[5177.872786 0:28548 arceos_posix_api::imp::net:567] [32msys_accept => Err(EINVAL)[m[m
accept01.c:92: TPASS: invalid socket buffer : EINVAL (22)
[37m[5177.875095 0:28548 axnet::smoltcp_impl::tcp:284] [33m[AxError::InvalidInput] socket accept() failed: not listen[m[m
[37m[5177.875246 0:28548 arceos_posix_api::imp::net:567] [32msys_accept => Err(EINVAL)[m[m
accept01.c:92: TPASS: invalid salen : EINVAL (22)
[37m[5177.875553 0:28548 axnet::smoltcp_impl::tcp:284] [33m[AxError::InvalidInput] socket accept() failed: not listen[m[m
[37m[5177.875686 0:28548 arceos_posix_api::imp::net:567] [32msys_accept => Err(EINVAL)[m[m
accept01.c:92: TPASS: no queued connections : EINVAL (22)
[37m[5177.876141 0:28548 arceos_posix_api::imp::net:567] [32msys_accept => Err(EOPNOTSUPP)[m[m
accept01.c:92: TPASS: UDP accept : EOPNOTSUPP (95)

Summary:
passed   5
failed   0
broken   0
skipped  0
warnings 0
FAIL LTP CASE accept01 : 0
Pass!
LTP MEMORY accept01 after_run: free_frames=75452 allocated_frames=185783
LTP MEMORY accept01 after_cleanup: free_frames=75452 allocated_frames=185783
LTP CASE RUNTIME accept01: 2193 ms
========== END ltp accept01 ==========
========== START ltp accept02 ==========
RUN LTP CASE accept02
LTP MEMORY accept02 before: free_frames=75452 allocated_frames=185783
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_buffers.c:57: TINFO: Test is using guarded buffers
accept02.c:131: TINFO: Starting listener on port: 49197
The futex facility returned an unexpected error code.
qemu-system-riscv64: terminating on signal 15 from pid 803586 (<unknown process>)
RUN_META run_eval_status=0
426s; last_run=accept02; last_done=accept01"
LEADER_META action=ensure_terminate_stalled_qemu blocker=accept02 at=2026-05-29T19:35:25Z reason="WORKER_META already recorded; qemu still alive after monitor termination"
RUN_META exit_code=0 end=2026-05-29T19:35:26Z
```
