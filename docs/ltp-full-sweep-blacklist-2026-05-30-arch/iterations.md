# RV/LA arch-specific blacklist closure run — 2026-05-30

## Goal

Use `LTP_CASES=blacklist` with separate supplemental overlays:

- common overlay: `blacklist-common.txt`
- RV overlay: `blacklist-rv.txt`
- LA overlay: `blacklist-la.txt`

The runner still applies the source default blacklist from `examples/shell/src/cmd.rs::LTP_SWEEP_DEFAULT_BLACKLIST_CASES`.  Blacklist entries are exclusions only and are never counted as PASS/stable-promotion evidence.

## Runner changes under test

- `examples/shell/src/cmd.rs` reads common guest blacklist files plus arch-specific guest files:
  - RV: `/ltp_blacklist_rv.txt`, `/tmp/ltp_blacklist_rv.txt`, `*-rv.txt`, `*_riscv64.txt`
  - LA: `/ltp_blacklist_la.txt`, `/tmp/ltp_blacklist_la.txt`, `*-la.txt`, `*_loongarch64.txt`
- Build-time overlays are supported separately:
  - common: `LTP_BLACKLIST`
  - RV: `LTP_BLACKLIST_RV` / `LTP_BLACKLIST_RISCV64`
  - LA: `LTP_BLACKLIST_LA` / `LTP_BLACKLIST_LOONGARCH64`
- `run-eval.sh` supports host-side file composition without changing default online-evaluator behavior:
  - common files: `LTP_BLACKLIST_FILE` or `LTP_BLACKLIST_COMMON_FILE`
  - RV file: `LTP_BLACKLIST_RV_FILE`
  - LA file: `LTP_BLACKLIST_LA_FILE`

## Iterations

Pending runs will append: command, raw log path, parser summary, marker audit, closure result, blacklist delta, and disk-space checks.

### la-arch001 — LA baseline with common + creat07 overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch001.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch001.monitor.log`
- Parser-backed compact summary: `summaries/la-arch001-summary.md` / `summaries/la-arch001-summary.json`
- Marker audit: `summaries/la-arch001-marker-audit.json`
- Selection: `all-minus-blacklist skipped=41 (2327 cases, timeout 15s)` for musl, then `skipped=44 (2331 cases, timeout 15s)` for glibc; this proves the LA-only `creat07` overlay was active.
- Result: **not closed**.  `ltp-musl` finished, then the run entered `ltp-glibc` while the guest was already resource-polluted.  The leader terminated QEMU/runner with `run_eval_status=143`; final incomplete RUN was `check_netem`.
- Parser summary: wrapper PASS=555, FAIL=1861, TIMEOUT=26; internal TBROK=663, TCONF=1031, TFAIL=1914; ENOSYS/not-implemented matches=601; parser panic/trap=0.
- Severe blocker evidence: first true resource failure at `la-arch001.log:36462`, current case `tcp4-multi-sameport09`, text `sh: fork: Resource temporarily unavailable`; later `la-arch001.log:38875` reports `TBROK: fork(): EAGAIN/EWOULDBLOCK (11)`.
- Blacklist delta: added LA-only network stress family (`tcp4-*`, `tcp6-*`, `udp4-*`, `udp6-*`, 364 cases from the parser case matrix) to `blacklist-la.txt`.  Scope is `arch=la` only; this is not common/RV evidence.  Reason category is resource exhaustion / fork failure / environment pollution, not ordinary TFAIL/ENOSYS.

### la-arch002 — LA with common + network-stress overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch002.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch002.monitor.log`
- Parser-backed compact summary: `summaries/la-arch002-summary.md` / `summaries/la-arch002-summary.json`
- Marker audit: `summaries/la-arch002-marker-audit.json`
- Selection: musl `skipped=405 (1963 cases, timeout 15s)`; glibc `skipped=408 (1967 cases, timeout 15s)`.
- Result: **not closed**.  `ltp-musl` finished, `ltp-glibc` later hit a kernel allocator panic during `fsync02`; `run-eval` returned 0 because QEMU powered down, so parser/marker audit is required.
- Parser summary before panic: wrapper PASS=752, FAIL=1687, TIMEOUT=33; internal TBROK=659, TCONF=1157, TFAIL=3090; ENOSYS/not-implemented matches=695; parser panic/trap=1.  `ltp-musl` suite summary was passed=605, failed=1358, timed_out=28.
- Severe blocker evidence: `la-arch002.log:50427` `RUN LTP CASE fsync02`; `la-arch002.log:50434-50436` shows `panicked at ... memory allocation of 98435072 bytes failed` then platform shutdown.  There is no normal PASS/FAIL/TIMEOUT marker for the glibc `fsync02` RUN.
- Blacklist delta: added `fsync02` to `blacklist-la.txt` with `arch=la` scope.  This skips both libc variants because the runner blacklist is case-name based; the reason is the glibc late-order LA panic, not ordinary musl behavior.

### la-arch003 — LA with `fsync02` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch003.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch003.monitor.log`
- Parser-backed compact summary: `summaries/la-arch003-summary.md` / `summaries/la-arch003-summary.json`
- Marker audit: `summaries/la-arch003-marker-audit.json`
- Selection: musl `skipped=406 (1962 cases, timeout 15s)`.  The run did not reach glibc.
- Result: **not closed**.  The run stalled in musl `pth_str01`; raw log mtime did not advance for 475s after `thread 12 started`, and QEMU was still active, so the leader terminated the run.
- Severe blocker evidence: `la-arch003.log:24872` `RUN LTP CASE pth_str01`; no corresponding PASS/FAIL/TIMEOUT marker before termination.
- Blacklist delta: added `pth_str01` to `blacklist-la.txt` with `arch=la` scope.  This is a hang/no-log-growth blocker, not a normal timeout/TFAIL promotion claim.

### la-arch004 — LA with `pth_str01` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch004.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch004.monitor.log`
- Parser-backed compact summary: `summaries/la-arch004-summary.md` / `summaries/la-arch004-summary.json`
- Marker audit: `summaries/la-arch004-marker-audit.json`
- Selection: musl `skipped=407 (1961 cases, timeout 15s)`; glibc `skipped=410 (1965 cases, timeout 15s)`.
- Result: **not closed / not clean**.  The run entered glibc, but the first true resource failure was glibc `access02` (`sh: fork: Resource temporarily unavailable`), and later cases reported `fork(): EAGAIN/EWOULDBLOCK`, so follow-on results were polluted.  The leader terminated the run instead of counting a polluted sweep.
- Severe blocker evidence: musl `fcntl16` dropped free frames from 150020 to 117244 while reporting wrapper PASS (`la-arch004.log:8921-8935`); musl `kill10` timed out and dropped free frames from 100321 to 34601 (`la-arch004.log:16587-16593`).  First follow-on resource failure: `la-arch004.log:40228`, current case `access02`, text `sh: fork: Resource temporarily unavailable`.
- Blacklist delta: added LA-only `fcntl16` and `kill10`.  Reason is cumulative environment pollution/resource exhaustion, not ordinary TFAIL or a PASS promotion claim.

### la-arch005 — LA with `fcntl16`/`kill10` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch005.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch005.monitor.log`
- Parser-backed compact summary: `summaries/la-arch005-summary.md` / `summaries/la-arch005-summary.json`
- Marker audit: `summaries/la-arch005-marker-audit.json`
- Selection: musl `skipped=409 (1959 cases, timeout 15s)`; glibc `skipped=412 (1963 cases, timeout 15s)`.
- Result: **not closed**.  The run entered glibc and then panicked during `lftest`; `run-eval` returned 0 because QEMU powered down, so this is another parser/marker-audit false-clean hazard.
- Parser summary before panic: wrapper PASS=857, FAIL=1937, TIMEOUT=37; internal TBROK=716, TCONF=1367, TFAIL=3153; ENOSYS/not-implemented matches=713; parser panic/trap=1; strict panic lines=2.
- Severe blocker evidence: `la-arch005.log:56047` `RUN LTP CASE lftest`; `la-arch005.log:56054-56055` shows `panicked at library/alloc/src/alloc.rs:437:13` and `memory allocation of 71303168 bytes failed`, followed by platform shutdown.  There is no normal lftest PASS/FAIL/TIMEOUT marker.
- Audit caveat: `summaries/la-arch005-marker-audit.json` also reports `cpuset_memory_pressure` incomplete because that case printed `usage: ...` without a newline before `FAIL LTP CASE`; the actual close marker is embedded on `la-arch005.log:43075`, so it is not a severe blocker.
- Blacklist delta: added LA-only `lftest`.  Reason is allocator panic / platform shutdown during full sweep, not ordinary TFAIL/ENOSYS.

### la-arch006 — LA with `lftest` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch006.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch006.monitor.log`
- Parser-backed compact summary: `summaries/la-arch006-summary.md` / `summaries/la-arch006-summary.json`
- Marker audit: `summaries/la-arch006-marker-audit.json`
- Selection: musl `skipped=410 (1958 cases, timeout 15s)`; glibc `skipped=413 (1962 cases, timeout 15s)`.
- Result: **not closed**.  The run entered glibc and then panicked during `mmstress`; `run-eval` returned 0 because QEMU powered down.
- Parser summary before panic: wrapper PASS=899, FAIL=2040, TIMEOUT=38; internal TBROK=752, TCONF=1397, TFAIL=3249; ENOSYS/not-implemented matches=764; parser panic/trap=1; strict panic lines=2.
- Severe blocker evidence: `la-arch006.log:58990` `RUN LTP CASE mmstress`; `la-arch006.log:58994-58995` shows `panicked at library/alloc/src/alloc.rs:437:13` and `memory allocation of 67108864 bytes failed`, followed by platform shutdown.  There is no normal mmstress PASS/FAIL/TIMEOUT marker.
- Blacklist delta: added LA-only `mmstress`.  Reason is allocator panic / platform shutdown during full sweep, not ordinary TFAIL/ENOSYS.

### la-arch007 — LA with `mmstress` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch007.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch007.monitor.log`
- Parser-backed compact summary: `summaries/la-arch007-summary.md` / `summaries/la-arch007-summary.json`
- Marker audit: `summaries/la-arch007-marker-audit.json`
- Selection: musl `skipped=411 (1957 cases, timeout 15s)`.  The run did not reach glibc.
- Result: **not closed**.  The run stalled in musl `dirtyc0w`; the leader terminated QEMU/runner with `run_eval_status=143` instead of counting a hang as a failure marker.
- Parser summary before termination: wrapper PASS=54, FAIL=139, TIMEOUT=3; internal TBROK=35, TCONF=120, TFAIL=31; ENOSYS/not-implemented matches=29; panic/trap=0.
- Severe blocker evidence: `la-arch007.log:4897` `RUN LTP CASE dirtyc0w`; `la-arch007.log:4901-4902` reports `dirtyc0w_child... tst_checkpoint_wake... ETIMEDOUT` and remaining cases broken, then no normal PASS/FAIL/TIMEOUT wrapper marker or log growth for 8+ minutes until leader termination.
- Blacklist delta: added LA-only `dirtyc0w`.  Reason is guest hang / no-log-growth closure blocker, not ordinary TBROK/TFAIL promotion evidence.

### rv-arch001 — RV current runner with common + empty RV overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_RV_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-rv.txt \
  ./run-eval.sh rv
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch001.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch001.monitor.log`
- Parser-backed compact summary: `summaries/rv-arch001-summary.md` / `summaries/rv-arch001-summary.json`
- Marker audit: `summaries/rv-arch001-marker-audit.json`
- Selection: musl `skipped=40 (2328 cases, timeout 15s)`.
- Result: **not closed**.  The run panicked during `kill10`; `run-eval` returned 0 because QEMU powered down.
- Parser summary before panic: wrapper PASS=243, FAIL=571, TIMEOUT=9; internal TBROK=183, TCONF=384, TFAIL=1192; ENOSYS/not-implemented matches=69; parser panic/trap=1; strict panic lines=2.
- Severe blocker evidence: `rv-arch001.log:16818` `RUN LTP CASE kill10`; `rv-arch001.log:16821-16822` shows `panicked at library/alloc/src/alloc.rs:437:13` and `memory allocation of 262144 bytes failed`, followed by platform shutdown.  There is no normal kill10 PASS/FAIL/TIMEOUT marker.
- Blacklist delta: added RV-only `kill10`.  LA has separate kill10 evidence in `blacklist-la.txt`; this RV entry keeps the evidence arch-scoped instead of silently broadening common blacklist.

### la-arch008 — LA with `dirtyc0w` overlay, sequential run (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch008.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch008.monitor.log`
- Parser-backed compact summary: `summaries/la-arch008-summary.md` / `summaries/la-arch008-summary.json`
- Marker audit: `summaries/la-arch008-marker-audit.json`
- Selection: musl `skipped=412 (1956 cases, timeout 15s)`; glibc `skipped=415 (1960 cases, timeout 15s)`.
- Result: **not closed**.  The run reached late glibc `write01` and then panicked; `run-eval` returned 0 because QEMU powered down.
- Parser summary before panic: wrapper PASS=1201, FAIL=2698, TIMEOUT=55; internal TBROK=1032, TCONF=1934, TFAIL=4045; ENOSYS/not-implemented matches=1279; parser panic/trap=1; strict panic lines=2.
- Severe blocker evidence: `la-arch008.log:79040` `RUN LTP CASE write01`; `la-arch008.log:79046-79047` shows `panicked at library/alloc/src/alloc.rs:437:13` and `memory allocation of 67108864 bytes failed`, followed by platform shutdown.  There is no normal write01 PASS/FAIL/TIMEOUT marker.
- Blacklist delta: added LA-only `write01`.  Reason is allocator panic / platform shutdown during full sweep, not ordinary TFAIL/ENOSYS.

### la-arch009 — LA with `write01` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch009.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch009.monitor.log`
- Parser-backed compact summary: `summaries/la-arch009-summary.md` / `summaries/la-arch009-summary.json`
- Marker audit: `summaries/la-arch009-marker-audit.json`
- Selection: musl `skipped=413 (1955 cases, timeout 15s)`; glibc `skipped=416 (1959 cases, timeout 15s)`.
- Result: **not closed**.  The run stalled in glibc `futex_wait01`; the leader terminated QEMU/runner with `run_eval_status=143`.
- Parser summary before termination: wrapper PASS=760, FAIL=1687, TIMEOUT=32; internal TBROK=660, TCONF=1158, TFAIL=3084; ENOSYS/not-implemented matches=695; panic/trap=0.
- Severe blocker evidence: `la-arch009.log:50508` `RUN LTP CASE futex_wait01`; `la-arch009.log:50513-50515` prints two TPASS lines but no normal PASS/FAIL/TIMEOUT wrapper marker, then no log growth for 7+ minutes until leader termination.
- Blacklist delta: added LA-only `futex_wait01`.  Reason is guest hang / no-log-growth closure blocker, not ordinary TPASS/TFAIL promotion evidence.

### la-arch010 — LA with `futex_wait01` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch010.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch010.monitor.log`
- Parser-backed compact summary: `summaries/la-arch010-summary.md` / `summaries/la-arch010-summary.json`
- Marker audit: `summaries/la-arch010-marker-audit.json`
- Selection: musl `skipped=414 (1954 cases, timeout 15s)`.  The run did not reach glibc.
- Result: **not closed**.  The run stalled in musl `futex_wait05`; the leader terminated QEMU/runner with `run_eval_status=143`.
- Parser summary before termination: wrapper PASS=163, FAIL=329, TIMEOUT=6; raw RUN=493, raw terminal markers=492, incomplete=1; internal TBROK=138, TCONF=183, TFAIL=1116; ENOSYS/not-implemented matches=52; panic/trap=0.
- Severe blocker evidence: `la-arch010.log:11449` `RUN LTP CASE futex_wait05`; `la-arch010.log:11461` starts the 500-iteration futex_wait timer loop, then no normal PASS/FAIL/TIMEOUT wrapper marker or log growth for 5+ minutes until leader termination at `la-arch010.log:11462`.
- Blacklist delta: added LA-only `futex_wait05`.  Reason is guest hang / no-log-growth closure blocker, not ordinary TPASS/TFAIL promotion evidence.

### la-arch011 — LA with `futex_wait05` overlay (not closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch011.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch011.monitor.log`
- Parser-backed compact summary: `summaries/la-arch011-summary.md` / `summaries/la-arch011-summary.json`
- Marker audit: `summaries/la-arch011-marker-audit.json`
- Selection: musl `skipped=415 (1953 cases, timeout 15s)`.  The run did not reach glibc.
- Result: **not closed**.  The run stalled in musl `nice05`; the leader terminated QEMU/runner with `run_eval_status=143`.
- Parser summary before termination: wrapper PASS=303, FAIL=779, TIMEOUT=10; raw RUN=1083, raw terminal markers=1082, incomplete=1; internal TBROK=290, TCONF=453, TFAIL=1330; ENOSYS/not-implemented matches=188; panic/trap=0.
- Severe blocker evidence: `la-arch011.log:21288` `RUN LTP CASE nice05`; no normal PASS/FAIL/TIMEOUT wrapper marker or log growth for 6+ minutes until leader termination at `la-arch011.log:21294`.
- Blacklist delta: added LA-only `nice05`.  Reason is guest hang / no-log-growth closure blocker, not ordinary TPASS/TFAIL promotion evidence.

### la-arch012 — LA with `nice05` overlay (closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.monitor.log`
- Parser-backed compact summary: `summaries/la-arch012-summary.md` / `summaries/la-arch012-summary.json`
- Marker audit: `summaries/la-arch012-marker-audit.json`
- Selection: musl `skipped=416 (1952 cases, timeout 15s)`; glibc `skipped=419 (1956 cases, timeout 15s)`.
- Result: **closed**.  `run-eval` returned 0; raw RUN markers=3908 and terminal markers=3908, incomplete=0.
- Parser summary: wrapper PASS=1207, FAIL=2698, TIMEOUT=53; internal TBROK=1031, TCONF=1936, TFAIL=4041; ENOSYS/not-implemented matches=1279; panic/trap=0; resource failures=0.
- Suite summaries: `ltp-musl` passed=602 failed=1350 timed_out=25; `ltp-glibc` passed=605 failed=1351 timed_out=28.
- Blacklist delta: none.  This is the first closed LA full-sweep with the LA-only overlay; skipped/blacklisted cases are not counted as PASS or stable-promotion evidence.

### rv-arch002 — RV with `kill10` overlay (closed)

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_RV_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-rv.txt \
  ./run-eval.sh rv
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log`
- Monitor log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.monitor.log`
- Parser-backed compact summary: `summaries/rv-arch002-summary.md` / `summaries/rv-arch002-summary.json`
- Marker audit: `summaries/rv-arch002-marker-audit.json`
- Selection: musl `skipped=41 (2327 cases, timeout 15s)`; glibc `skipped=44 (2331 cases, timeout 15s)`.
- Result: **closed**.  `run-eval` returned 0; raw RUN markers=4658 and terminal markers=4658, incomplete=0.
- Parser summary: wrapper PASS=1204, FAIL=3453, TIMEOUT=55; internal TBROK=1043, TCONF=2663, TFAIL=4058; ENOSYS/not-implemented matches=1280; panic/trap=0; resource failures=0.
- Suite summaries: `ltp-musl` passed=598 failed=1729 timed_out=27; `ltp-glibc` passed=606 failed=1725 timed_out=28.
- Blacklist delta: none.  This is the first closed RV full-sweep with the RV-only overlay; skipped/blacklisted cases are not counted as PASS or stable-promotion evidence.

## 2026-05-31 follow-up: remote-submission blacklist mode (superseded on 2026-06-01)

User requirement: submitting this experimental branch to online evaluation should run the blacklist full-sweep mode, not the stable list.

Note: this default was later reverted in the 2026-06-01 online-friendly stable-default follow-up below after remote score-table evidence showed the full sweep is not score-friendly as an implicit online path.

Change:

- `make` / `make all` now builds remote-submission `kernel-rv` with:
  - `LTP_CASES=blacklist`
  - `LTP_BLACKLIST=<blacklist-common.txt>`
  - `LTP_BLACKLIST_RV=<blacklist-rv.txt>`
- `make` / `make all` now builds remote-submission `kernel-la` with:
  - `LTP_CASES=blacklist`
  - `LTP_BLACKLIST=<blacklist-common.txt>`
  - `LTP_BLACKLIST_LA=<blacklist-la.txt>`
- Local `./run-eval.sh rv|la`, `make kernel-rv`, and `make kernel-la` remain opt-in for blacklist mode unless their env vars are explicitly provided.

Validation:

```bash
make -n all
make all
strings kernel-rv | rg 'kill10|pthserv|all-minus-blacklist' | head
strings kernel-la | rg 'creat07|fsync02|pthserv|all-minus-blacklist' | head
```

Result:

- `make -n all` showed `LTP_CASES="blacklist"` plus RV/LA blacklist env injection in the remote-submission build commands.
- `make all` completed and regenerated both `kernel-rv` and `kernel-la`.
- `strings kernel-rv` showed the RV-only `kill10` overlay and common entries such as `pthserv` embedded in the built submission kernel.
- `strings kernel-la` showed LA-only entries such as `creat07` / `fsync02` and common entries such as `pthserv` embedded in the built submission kernel.
- This is build-mode wiring only; the closed runtime evidence remains `rv-arch002` and `la-arch012` above.

## 2026-06-01 follow-up: online-friendly stable default

Remote score-table evidence showed that making `make` / `make all` default to the full all-minus-blacklist sweep is not score-friendly: the remote evaluator can spend time/output budget in the long sweep before preserving the known high-value stable whitelist coverage, and the LA blacklist currently excludes two stable-list cases (`fcntl16`, `write01`).  The blacklist closure remains useful experimental evidence, but it should not be the implicit online scoring path.

Change:

- `REMOTE_LTP_CASES ?= stable` is restored as the default for `make` / `make all`.
- `make all REMOTE_LTP_CASES=blacklist` remains the explicit opt-in path for the closed full-sweep blacklist experiment.
- The Makefile only requires and injects `REMOTE_LTP_BLACKLIST_*_FILE` contents when `REMOTE_LTP_CASES` is one of `blacklist`, `all-minus-blacklist`, or `sweep:blacklist`.
- Local `./run-eval.sh rv|la`, `make kernel-rv`, and `make kernel-la` behavior remains opt-in for blacklist mode unless their env vars are explicitly provided.

Validation:

```bash
make -n all
make -n all REMOTE_LTP_CASES=blacklist
make all
python3 -m json.tool docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-quality-gate.json
git diff --check -- Makefile docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-report.md docs/ltp-full-sweep-blacklist-2026-05-30-arch/iterations.md docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-quality-gate.json
```

Result:

- Default `make -n all` shows `LTP_CASES="stable"` for both RV and LA and does not inject blacklist env variables.
- Opt-in `make -n all REMOTE_LTP_CASES=blacklist` still shows `LTP_CASES="blacklist"` plus RV/LA blacklist env injection.
- `make all` completed with the stable default and regenerated `kernel-rv` / `kernel-la`; disk stayed at `/` and `/root` 59G size, 23G used, 34G available, 41% used before and after.
- This is build-mode wiring only; no new full LTP runtime claim is made in this follow-up.
