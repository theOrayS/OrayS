# mmap05 LA write-protect blocker report

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Stable baseline: `606 total / 606 unique / 0 duplicate`

## Purpose

Close the immediate `mmap05` hypothesis from the `munmap01` synchronous-`SIGSEGV` repair: RV became clean, but LA still reports `SIGSEGV signal not received`. This report records why a simple TLB/mprotect refresh is not enough and why `mmap05` remains outside the stable656 candidate pool.

## Commands and artifacts

### Explicit mapping-change TLB flush experiment

Temporary local experiment: add an explicit full TLB flush after successful `mprotect` / `munmap` / `MAP_FIXED` replacement, then rerun LA `mmap05,munmap01`.

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=mmap05,munmap01 \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 35m ./run-eval.sh la
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-tlbflush-20260602T004430Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-tlbflush-20260602T004430Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-tlbflush-20260602T004430Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-tlbflush-20260602T004430Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-tlbflush-20260602T004430Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 2
FAIL LTP CASE: 2
Internal TFAIL/TBROK/TCONF: 2 ({'TFAIL': 2})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: the explicit flush experiment did **not** make `mmap05` promotable. `munmap01` stayed clean; both LA `mmap05` rows still failed with `TFAIL=1`.

### Temporary LA instrumentation rerun

Temporary instrumentation printed `mprotect` and page-fault observations. The instrumentation was removed after the run and no production code change was retained from it.

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=mmap05 \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 25m ./run-eval.sh la
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/la-mmap05-debug-20260602T004819Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/la-mmap05-debug-20260602T004819Z.summary.txt`
- Status: `target/ltp-1000-milestone-03-stable656/la-mmap05-debug-20260602T004819Z.status`

Parser summary:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 2
Internal TFAIL/TBROK/TCONF: 2 ({'TFAIL': 2})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Key observations from the temporary debug log:

- musl setup changed a page to `prot=0x1` / `READ | USER` and page-table query after `mprotect` reported `READ | USER`.
- glibc setup similarly showed `mprotect(..., PROT_READ)` rows whose post-protect page-table query no longer contained `WRITE`.
- During the later `mmap05.c:60` access, no corresponding page fault was observed for the protected page before `mmap05.c:65: TFAIL: SIGSEGV signal not received`.

## Root-cause boundary

Current source already routes LoongArch `PageModifyFault` and `PageNonReadableFault` through the `PAGE_FAULT` handler (`vendor/axcpu/src/loongarch64/trap.rs`). The generic `AddrSpace::handle_page_fault` path would return `false` for a write fault against an area whose flags are only `READ | USER`, allowing the synchronous `SIGSEGV` delivery path to run. The debug run instead shows that the local LA execution did not take a page fault for the protected access.

The LoongArch architecture model records write protection through the dirty/write-protection path: a store to a TLB entry with `V=1`, legal privilege, and `D=0` is specified to raise Page Modify Exception. The repository's LA PTE conversion clears `D` when `MappingFlags::WRITE` is absent. Local LA evidence therefore points at a LoongArch permission-enforcement/TLB-model boundary rather than the generic user signal queue itself.

## Decision

- Do not promote `mmap05`.
- Do not count the temporary flush experiment as a fix: parser evidence still has LA `TFAIL`.
- Do not keep the redundant explicit TLB flush patch from the failed experiment; current code remains at the prior committed synchronous-`SIGSEGV` repair.
- Treat `mmap05` as a LoongArch write-protect/page-modify lane. A future fix must prove LA `mmap05` clean and keep RV/LA mmap + signal regression subsets clean before it can add a new stable656 candidate.

## No fake-pass check

This report records a failed hypothesis and keeps `TFAIL` visible. It does not edit `LTP_STABLE_CASES`, does not blacklist `mmap05`, and does not modify LTP/evaluator behavior.
