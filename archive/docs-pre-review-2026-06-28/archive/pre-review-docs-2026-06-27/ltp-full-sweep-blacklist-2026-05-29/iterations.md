# LTP full-sweep blacklist iterations (2026-05-29)

## Baseline

- Branch: `exp/ltp-full-sweep-blacklist`
- HEAD: `9e8c6ed7`
- Live stable count: 460 total / 460 unique
- Sweep mode target: `LTP_CASES=blacklist` (runner reports `all-minus-blacklist skipped=N`)
- Supplemental blacklist file: `docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt`
- Raw log directory: `target/ltp-full-sweep-blacklist-2026-05-29/raw/`
- Summary directory: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/`
- Parser: `python3 scripts/ltp_summary.py <raw-log>`
- Marker grep: `rg '^RUN LTP CASE|^PASS LTP CASE|^FAIL LTP CASE|^TIMEOUT LTP CASE|^\[CONTEST\]\[LTP\]\[SKIP\]' <raw-log>`

### Runner blacklist sources

1. Source default: `examples/shell/src/cmd.rs::LTP_SWEEP_DEFAULT_BLACKLIST_CASES` (38 cases)
2. Build-time env: `LTP_BLACKLIST=\"$(cat docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)\"`
3. Guest files: `/ltp_blacklist.txt`, `/tmp/ltp_blacklist.txt` if present

### Source default blacklist snapshot

```text
cgroup_fj_proc
cgroup_regression_fork_processes
cpuctl_def_task01
cpuctl_def_task02
cpuctl_def_task03
cpuctl_def_task04
cpuctl_fj_cpu-hog
cpuctl_test01
cpuctl_test02
cpuctl_test03
cpuctl_test04
cpuhotplug_do_disk_write_loop
cpuhotplug_do_kcompile_loop
cpuhotplug_do_spin_loop
cpuhotplug_report_proc_interrupts
cpuset_cpu_hog
cpuset_mem_hog
cpuset_memory_test
crash01
crash02
dirtyc0w_child
dirtyc0w_shmem
doio
ebizzy
fork_exec_loop
fork_procs
fsx-linux
hackbench
mallocstress
memcg_test_2
memcg_test_4
memctl_test01
mmapstress01
mtest01
netstress
pids_task2
shm_test
timed_forkbomb
```

### Disk baseline before first run

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        59G   22G   35G  39% /
/dev/vda2        59G   22G   35G  39% /
```

### Existing tracked dirty baseline (not owned by this task)

```
-  D docs/ltp-long-term-collaboration-2026-05-28/README.md
-  D docs/ltp-long-term-collaboration-2026-05-28/three-person-long-term-ownership.md
-  D docs/ltp-score-improvement-2026-05-28-phase-a/three-person-task-allocation-stable460-to-470.md
```

## Iteration log

RV now has one clean closed full-sweep at iteration 006. Entries include command, raw log path, parser summary path, closure status, severe blockers, and blacklist delta; blacklist/SKIP are never counted as PASS.

### RV iteration 001b — first real RV full-sweep attempt (not closed)

- Command (run from clean detached worktree `/root/oskernel2026-orays-team-fullsweep-20260529` at HEAD `9e8c6ed7`):
  ```bash
  LTP_BLACKLIST="$(cat /root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
  LTP_CASES=blacklist \
  RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
  ./run-eval.sh rv 2>&1 | tee /root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter001b.log
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter001b.log` (not committed)
- Parser summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter001b-summary.txt`
- Compact summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter001b-compact.json`
- Marker/incomplete evidence:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter001b-markers.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter001b-incomplete.txt`
- Selection line: `ltp case list: all-minus-blacklist skipped=35 (2333 cases, timeout 15s)`
- Raw wrapper markers:
  - `RUN`: 1268
  - raw `PASS LTP CASE` markers: 0
  - raw `FAIL LTP CASE` markers: 1267
  - raw `FAIL ... : 0` parser-normalized pass candidates: 348
  - raw nonzero FAIL: 919
  - `TIMEOUT LTP CASE`: 17
  - `[CONTEST][LTP][SKIP]`: 0
  - incomplete unique: `pthserv`
- Parser-backed summary:
  - normalized pass count: 348
  - normalized fail count: 919
  - internal markers: `TFAIL=1425`, `TBROK=351`, `TCONF=487`
  - ENOSYS/not implemented matches: 246
  - panic/trap matches: 0
- Closure verdict: **not closed**. The runner reached `RUN LTP CASE pthserv` and then stopped making log progress; worker-1 terminated QEMU to avoid an unbounded hang. `RUN_META exit_code=0` only reflects the manual termination path and is not a clean full-sweep closure.
- Severe blocker added to supplemental blacklist:
  - case: `pthserv`
  - category: hang / incomplete `RUN LTP CASE`
  - first evidence: `rv-iter001b.log` contains `WORKER_META blocker=pthserv action=terminate_stalled_qemu at=2026-05-29T11:48:21Z reason="no log growth since 2026-05-29T11:42:10Z; incomplete RUN marker without closure"`
  - scope: generic supplemental blacklist for next RV and LA sweeps; observed on RV, not LA-specific
  - removal condition: targeted `pthserv` completes with a normal PASS/FAIL/TIMEOUT marker and no QEMU/guest stall or residual environment damage
- Non-blacklist failures: ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and closed per-case timeouts from this run remain real failures and were **not** added to blacklist.

### RV iteration 002 — second RV full-sweep attempt (not closed)

- Command (run from clean detached worktree `/root/oskernel2026-orays-team-fullsweep-20260529`; temp HEAD `b1c8090b` had docs-only Team integration commits, source unchanged from original HEAD `9e8c6ed7`):
  ```bash
  LTP_BLACKLIST="$(cat /root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
  LTP_CASES=blacklist \
  RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
  ./run-eval.sh rv 2>&1 | tee /root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter002.log
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter002.log` (not committed)
- Parser summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter002-summary.txt`
- Compact summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter002-compact.json`
- Marker/incomplete evidence:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter002-markers.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter002-incomplete.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter002-host-oom-dmesg.txt`
- Selection line: `ltp case list: all-minus-blacklist skipped=36 (2332 cases, timeout 15s)`
- Raw wrapper markers:
  - `RUN`: 1108
  - raw `PASS LTP CASE` markers: 0
  - raw `FAIL LTP CASE` markers: 1107
  - raw `FAIL ... : 0` parser-normalized pass candidates: 293
  - raw nonzero FAIL: 814
  - `TIMEOUT LTP CASE`: 16
  - `[CONTEST][LTP][SKIP]`: 0
  - incomplete unique: `oom01`
- Parser-backed summary:
  - normalized pass count: 293
  - normalized fail count: 814
  - internal markers: `TFAIL=1362`, `TBROK=296`, `TCONF=451`
  - ENOSYS/not implemented matches: 190
  - panic/trap matches: 0
- Closure verdict: **not closed**. The run reached `RUN LTP CASE oom01`; the case attempted a 3221225472-byte allocation and the host OOM killer killed `qemu-system-ris`, after which `make` reported `Killed` and `RUN_META exit_code=2`. This is a sweep-blocking resource-exhaustion/OOM event, not a test PASS.
- Severe blocker added to supplemental blacklist:
  - case: `oom01`
  - category: guest/host OOM or resource exhaustion killing QEMU/runner
  - first evidence: `rv-iter002.log` around `RUN LTP CASE oom01` shows `thread ... allocating 3221225472 bytes`, `make: *** [Makefile:373: run-rv] Killed`, and `RUN_META exit_code=2`; `rv-iter002-host-oom-dmesg.txt` records host OOM killer killing `qemu-system-ris` pid 648578.
  - scope: generic supplemental blacklist for next RV and LA sweeps; observed on RV, not LA-specific
  - removal condition: targeted `oom01` completes with a normal PASS/FAIL/TIMEOUT marker and no QEMU/runner kill, host OOM, guest OOM cascade, or residual environment damage
- Non-blacklist failures: ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and closed per-case timeouts from this run remain real failures and were **not** added to blacklist.

### RV iteration 003 — third RV full-sweep attempt (not closed)

- Command (run from clean detached worktree `/root/oskernel2026-orays-team-fullsweep-20260529`; temp HEAD `b1c8090b` had docs-only Team integration commits, source unchanged from original HEAD `9e8c6ed7`):
  ```bash
  LTP_BLACKLIST="$(cat /root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
  LTP_CASES=blacklist \
  RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
  ./run-eval.sh rv 2>&1 | tee /root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter003.log
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter003.log` (not committed)
- Parser summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter003-summary.txt`
- Compact summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter003-compact.json`
- Marker/incomplete evidence:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter003-markers.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter003-incomplete.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter003-host-oom-dmesg.txt`
- Selection line: `ltp case list: all-minus-blacklist skipped=37 (2331 cases, timeout 15s)`
- Raw wrapper markers:
  - `RUN`: 1597
  - raw `PASS LTP CASE` markers: 0
  - raw `FAIL LTP CASE` markers: 1596
  - raw `FAIL ... : 0` parser-normalized pass candidates: 477
  - raw nonzero FAIL: 1119
  - `TIMEOUT LTP CASE`: 25
  - `[CONTEST][LTP][SKIP]`: 0
  - incomplete unique: `shmat1`
- Parser-backed summary:
  - normalized pass count: 477
  - normalized fail count: 1119
  - internal markers: `TFAIL=1784`, `TBROK=431`, `TCONF=623`
  - ENOSYS/not implemented matches: 359
  - panic/trap matches: 0
- Closure verdict: **not closed**. The run reached `RUN LTP CASE shmat1`; the test repeatedly created/mapped SysV shared-memory segments and the host OOM killer killed `qemu-system-ris`, after which `make` reported `Killed` and `RUN_META exit_code=2`. This is a sweep-blocking resource-exhaustion/OOM event, not a test PASS.
- Severe blocker added to supplemental blacklist:
  - case: `shmat1`
  - category: SysV shared-memory stress / guest-host OOM or resource exhaustion killing QEMU/runner
  - first evidence: `rv-iter003.log` around `RUN LTP CASE shmat1` shows repeated `shmget()` / `Map address` / `Num iter` lines, `make: *** [Makefile:373: run-rv] Killed`, and `RUN_META exit_code=2`; `rv-iter003-host-oom-dmesg.txt` records host OOM killer killing `qemu-system-ris` pid 722284.
  - scope: generic supplemental blacklist for next RV and LA sweeps; observed on RV, not LA-specific
  - removal condition: targeted `shmat1` completes with a normal PASS/FAIL/TIMEOUT marker and no QEMU/runner kill, host OOM, guest OOM cascade, or residual environment damage
- Non-blacklist failures: ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and closed per-case timeouts from this run remain real failures and were **not** added to blacklist.

### RV iteration 004 — fourth RV full-sweep attempt (not closed)

- Command (run from clean detached worktree `/root/oskernel2026-orays-team-fullsweep-20260529`; temp HEAD `b1c8090b` had docs-only Team integration commits, source unchanged from original HEAD `9e8c6ed7`):
  ```bash
  LTP_BLACKLIST="$(cat /root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
  LTP_CASES=blacklist \
  RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
  ./run-eval.sh rv 2>&1 | tee /root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter004.log
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter004.log` (not committed)
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter004.monitor.log` (not committed)
- Parser summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter004-summary.txt`
- Compact summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter004-compact.json`
- Marker/incomplete evidence:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter004-markers.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter004-incomplete.txt`
- Selection lines:
  - musl phase: `ltp case list: all-minus-blacklist skipped=38 (2330 cases, timeout 15s)`
  - glibc phase: `ltp case list: all-minus-blacklist skipped=41 (2334 cases, timeout 15s)`
- Raw wrapper markers across both phases before blocker:
  - `RUN`: 2334
  - raw `PASS LTP CASE` markers: 0
  - raw `FAIL LTP CASE` markers: 2333
  - raw `FAIL ... : 0` parser-normalized pass candidates: 593
  - raw nonzero FAIL: 1740
  - `TIMEOUT LTP CASE`: 31
  - `[CONTEST][LTP][SKIP]`: 0
  - incomplete stack: `accept02` (glibc phase; musl `accept02` already closed earlier, so duplicate case-name accounting must be stack/order based)
- Parser-backed summary:
  - normalized pass count: 593
  - normalized fail count: 1740
  - internal markers: `TFAIL=1978`, `TBROK=521`, `TCONF=1339`
  - ENOSYS/not implemented matches: 626
  - panic/trap matches: 0
- Closure verdict: **not closed**. The musl phase reached its `ltp cases: 590 passed, 1740 failed, 31 timed out` aggregate line, then the glibc phase reached `RUN LTP CASE accept02`, printed `The futex facility returned an unexpected error code.`, and stopped making log progress. The leader/monitor terminated QEMU; `RUN_META exit_code=0` only reflects that termination path and is not a clean full-sweep closure.
- Severe blocker added to supplemental blacklist:
  - case: `accept02`
  - category: glibc-phase hang / futex error / incomplete `RUN LTP CASE`
  - first evidence: `rv-iter004.log` lines around the second `RUN LTP CASE accept02` show the futex error and no closure marker; `rv-iter004.monitor.log` records `WORKER_META blocker=accept02 action=terminate_stalled_qemu at=2026-05-29T19:34:20Z reason="no log growth for 426s; last_run=accept02; last_done=accept01"`; raw-log line interleaving truncated the `WORKER_META` prefix, so the monitor log is the canonical stall line.
  - scope: generic supplemental blacklist because runner blacklist is case-name based; observed on RV glibc phase, not LA-specific
  - removal condition: targeted `accept02` completes in both relevant variants, especially glibc, with a normal PASS/FAIL/TIMEOUT marker and no futex abort, QEMU/guest stall, or residual environment damage
- Non-blacklist failures: ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and closed per-case timeouts from this run remain real failures and were **not** added to blacklist.

### RV iteration 005 — fifth RV full-sweep attempt (not closed)

- Command (run from clean detached worktree `/root/oskernel2026-orays-team-fullsweep-20260529`; temp HEAD `b1c8090b` had docs-only Team integration commits, source unchanged from original HEAD `9e8c6ed7`):
  ```bash
  LTP_BLACKLIST="$(cat /root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
  LTP_CASES=blacklist \
  RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
  ./run-eval.sh rv 2>&1 | tee /root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter005.log
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter005.log` (not committed)
- Parser summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter005-summary.txt`
- Compact summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter005-compact.json`
- Marker/incomplete evidence:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter005-markers.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter005-incomplete.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter005-inline-marker-audit.json`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter005-host-oom-dmesg.txt`
- Selection lines:
  - musl phase: `ltp case list: all-minus-blacklist skipped=39 (2329 cases, timeout 15s)`
  - glibc phase: `ltp case list: all-minus-blacklist skipped=42 (2333 cases, timeout 15s)`
- Raw wrapper markers across both phases before blocker, using inline-marker-aware audit because `cpuset_memory_pressure` glued its wrapper marker onto stdout:
  - `RUN`: 3254
  - raw `PASS LTP CASE` markers: 0
  - raw `FAIL LTP CASE` markers: 3253
  - raw `FAIL ... : 0` parser-normalized pass candidates: 857
  - raw nonzero FAIL: 2396
  - `TIMEOUT LTP CASE`: 51
  - `[CONTEST][LTP][SKIP]`: 0
  - incomplete stack: `mincore03`
- Parser-backed summary (`scripts/ltp_summary.py`; note the parser does not recover the glued `cpuset_memory_pressure` wrapper marker, so marker closure uses the inline-aware audit above):
  - normalized pass count: 857
  - normalized fail count: 2395
  - internal markers: `TFAIL=3235`, `TBROK=742`, `TCONF=1754`
  - ENOSYS/not implemented matches: 765
  - panic/trap matches: 0
- Closure verdict: **not closed**. The run completed the first LTP phase (`ltp cases: 590 passed, 1739 failed, 31 timed out`), continued into the second phase, and then reached `RUN LTP CASE mincore03`. The host OOM killer killed `qemu-system-ris` pid 806463; the raw log ended with `make: *** [Makefile:373: run-rv] Killed`, `RUN_META run_eval_status=2`, and `RUN_META exit_code=2`. This is a sweep-blocking resource-exhaustion/OOM event, not a test PASS.
- Explicit non-blacklist caveat: `cpuset_memory_pressure` is **not** a blocker. Its terminal wrapper marker is glued to stdout (`usage: cpuset_memory_pressure mmap-size-in-kBFAIL LTP CASE cpuset_memory_pressure : 1`) and the run continued afterward. It remains an ordinary closed failure.
- Severe blocker added to supplemental blacklist:
  - case: `mincore03`
  - category: guest/host OOM or resource exhaustion killing QEMU/runner
  - first evidence: `rv-iter005.log` around `RUN LTP CASE mincore03` plus `rv-iter005-host-oom-dmesg.txt` showing host OOM killed `qemu-system-ris` pid 806463
  - scope: generic supplemental blacklist for next RV and LA sweeps; observed on RV, not LA-specific
  - removal condition: targeted `mincore03` completes with a normal PASS/FAIL/TIMEOUT marker and no QEMU/runner kill, host OOM, guest OOM cascade, or residual environment damage
- Non-blacklist failures: ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, closed per-case timeouts, and the closed `cpuset_memory_pressure` failure remain real failures and were **not** added to blacklist.

### RV iteration 006 — sixth RV full-sweep attempt (closed)

- Command (run from clean detached worktree `/root/oskernel2026-orays-team-fullsweep-20260529`; temp HEAD `b1c8090b` had docs-only Team integration commits, source unchanged from original HEAD `9e8c6ed7`):
  ```bash
  LTP_BLACKLIST="$(cat /root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
  LTP_CASES=blacklist \
  RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
  ./run-eval.sh rv 2>&1 | tee /root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter006.log
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/rv-iter006.log` (not committed)
- Parser summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-summary.txt`
- Compact summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-compact.json`
- Marker/incomplete evidence:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-markers.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-incomplete.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-marker-audit.json`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006-inline-marker-audit.json`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/rv-iter006.log.sha256`
- Selection lines:
  - `ltp case list: all-minus-blacklist skipped=40 (2328 cases, timeout 15s)`
  - `ltp case list: all-minus-blacklist skipped=43 (2332 cases, timeout 15s)`
- Raw wrapper markers across both LTP phases, using inline-marker-aware audit:
  - `RUN`: 4660
  - raw `PASS LTP CASE` markers: 0
  - raw `FAIL LTP CASE` markers: 4660
  - raw `FAIL ... : 0` parser-normalized pass candidates: 1186
  - raw nonzero FAIL: 3474
  - `TIMEOUT LTP CASE`: 68
  - `[CONTEST][LTP][SKIP]`: 0
  - incomplete stack count: 0
- Parser-backed summary (`python3 scripts/ltp_summary.py`):
  - normalized pass count: 1186
  - normalized fail count: 3473
  - status counts: `{'PASS': 1186, 'FAIL': 3473, 'UNKNOWN': 1}`
  - internal markers: `TFAIL=4119`, `TBROK=1042`, `TCONF=2663`
  - ENOSYS/not implemented matches: 1280
  - panic/trap matches: 0
  - suite summaries: `[{'failed': 1740, 'group': 'ltp-musl', 'passed': 588, 'timed_out': 33}, {'failed': 1734, 'group': 'ltp-glibc', 'passed': 598, 'timed_out': 35}]`
- Wrapper/parser caveat: official parser produced one `UNKNOWN` row for `cpuset_memory_pressure` because its terminal wrapper marker was glued to testcase stdout on the same line as `RUN`; the inline-aware audit records it as closed `FAIL : 1`. Parser wrapper total `4659` plus inline-aware delta `1` equals wrapper total `4660`.
- Closure verdict: **closed**. `RUN_META run_eval_status=0`, `RUN_META exit_code=0`, incomplete RUN stack is empty, and the audit found no `WORKER_META`/`LEADER_META` stall marker, QEMU fatal signal, make-killed line, panic, or trap. This satisfies the RV full-sweep closure gate, while the many ordinary failures remain real failures and are not converted into PASS.
- Supplemental blacklist delta: **none**. No new severe blocker was added after this closed run. `mknod06` was not added: an earlier Team audit was superseded by the final closed `rv-iter006` evidence (`open_stack=[]`, exit code 0). `cpuset_memory_pressure` also remains an ordinary closed failure, not a blocker.
- Non-blacklist failures: ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, closed per-case timeouts, and closed wrapper failures remain real failures and were **not** added to blacklist or stable-promotion evidence.

### LA iteration 001 — comparison with RV-converged blacklist (arch-specific blocker)

- Command (run from clean detached worktree `/root/oskernel2026-orays-team-fullsweep-20260529`; same RV-converged supplemental blacklist, no LA-only entries added):
  ```bash
  LTP_BLACKLIST="$(cat /root/oskernel2026-orays/docs/ltp-full-sweep-blacklist-2026-05-29/blacklist.txt)" \
  LTP_CASES=blacklist \
  LA_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-la.img \
  ./run-eval.sh la 2>&1 | tee /root/oskernel2026-orays/target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.log
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.log` (not committed)
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.monitor.log` (not committed)
- Parser summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-summary.txt`
- Compact summary: `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-compact.json`
- Marker/incomplete evidence:
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-markers.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-incomplete.txt`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001-marker-audit.json`
  - `docs/ltp-full-sweep-blacklist-2026-05-29/summaries/la-iter001.log.sha256`
- Selection lines:
  - `ltp case list: all-minus-blacklist skipped=40 (2328 cases, timeout 15s)`
- Raw wrapper markers before arch-specific blocker:
  - `RUN`: 160
  - raw `PASS LTP CASE` markers: 0
  - raw `FAIL LTP CASE` markers: 159
  - raw `FAIL ... : 0` parser-normalized pass candidates: 47
  - raw nonzero FAIL: 112
  - `TIMEOUT LTP CASE`: 2
  - `[CONTEST][LTP][SKIP]`: 0
  - incomplete stack count: 1 (`creat07`)
- Parser-backed summary (`python3 scripts/ltp_summary.py`):
  - normalized pass count: 47
  - normalized fail count: 112
  - status counts: `{'PASS': 47, 'FAIL': 112, 'UNKNOWN': 1}`
  - internal markers: `TFAIL=23`, `TBROK=28`, `TCONF=104`
  - ENOSYS/not implemented matches: 29
  - panic/trap matches: 0
- Closure verdict: **not closed on LA**. The run reached `RUN LTP CASE creat07` after `creat06`, printed `TBROK` checkpoint timeout messages, then made no log progress for 392s while `qemu-system-loongarch64` remained active. Leader terminated QEMU and recorded canonical monitor evidence. `RUN_META exit_code=0` reflects the termination path and is **not** a clean full-sweep closure.
- Arch-specific severe blocker:
  - case: `creat07`
  - category: `LA-only sweep hang / incomplete RUN after TBROK checkpoint timeout`
  - first evidence: `target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.log: last RUN LTP CASE creat07 after creat06 closure; target/ltp-full-sweep-blacklist-2026-05-29/raw/la-iter001.monitor.log: LEADER_META arch=la blocker=creat07 action=terminate_stalled_qemu at=2026-05-30T04:27:24Z reason="no log growth for 392s; last_run=creat07; last_terminal=creat06; qemu still active"`
  - scope: `arch=la` only. The generic supplemental blacklist is unchanged; do not add this to RV/common blacklist without RV evidence.
  - removal condition: `targeted creat07 on LA completes with a normal PASS/FAIL/TIMEOUT marker and no stalled QEMU/guest hang or environment damage`
- Supplemental blacklist delta: **none**. This LA-only blocker is documented separately and did not contaminate `blacklist.txt`.
- Non-blacklist failures: ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, closed per-case timeouts, and closed wrapper failures remain real failures and were **not** added to blacklist or stable-promotion evidence.
