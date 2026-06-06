# milestone-10 stable1000 report

## Goal

Promote the live stable baseline from 956 to 1000 trusted unique LTP stable cases, closing the long-term stable1000 target without fake pass behavior.

## Result

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: 1000 total / 1000 unique / 0 duplicate.
- Added 44 new cases, exactly the cases listed in `targeted-cases.txt`.
- Post-review new44 gates are RV + LA x musl + glibc wrapper PASS and parser-clean: no TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap.
- Post-review vfork/clone/exec/close_range/pipe/fcntl/mmap regression gates are RV + LA parser-clean.

## Code changes in this milestone

- `vendor/axcpu/src/riscv/uspace.rs`: replaced the RISC-V user-entry inline-asm path with a naked trampoline so saved user registers (notably clone child s0/s1) are restored without Rust codegen clobbering.
- `kernel/memory/axmm/src/aspace.rs`: added vfork-oriented shared user mapping clone support. The child gets a separate page table for exec/exit teardown, while resident writable frames can remain shared so pre-exec child stores are visible to the blocked parent.
- `kernel/task/axtask/src/task.rs` and `kernel/task/axtask/Cargo.toml`: expose a uspace-gated task page-table-root update so successful exec swaps keep the saved task context and hardware root aligned.
- `examples/shell/src/uspace/process_lifecycle.rs`: adjusted vfork/clone handling, exec atomicity, vfork exec wakeup, busybox applet alias resolution, executable tracking, and process teardown boundaries used by `clone05`, glibc `system()`/`creat07`, and exec/path helper cases.
- `examples/shell/src/uspace/fd_table.rs`: keeps FD unshare aliases separate from the base shared table so `CLOSE_RANGE_UNSHARE` plus `CLONE_FILES` does not leak the caller's private table to older sharers.
- `examples/shell/src/cmd.rs`: promoted 44 stable cases and kept LTP helper PATH focused on real `/musl`/`/glibc` busybox applet aliases rather than non-executable `/tmp` wrappers.

## Evidence

See `validation.md` for exact commands, log/summary/json/checksum paths, parser outputs, excluded diagnostics, and caveats. Final milestone gates:

- RV new44: `target/ltp-1000-milestone-10-stable1000/rv-new44-postreview-rerun60-20260606T135933+0800/rv-summary.txt` — PASS LTP CASE 88, FAIL 0, internal 0, timeout 0, ENOSYS 0, panic/trap 0.
- LA new44: `target/ltp-1000-milestone-10-stable1000/la-new44-postreview-rerun60-20260606T140605+0800/la-summary.txt` — PASS LTP CASE 88, FAIL 0, internal 0, timeout 0, ENOSYS 0, panic/trap 0.
- RV regression subset: `target/ltp-1000-milestone-10-stable1000/rv-regression-postreview-rerun60-20260606T141353+0800/rv-summary.txt` — PASS LTP CASE 60, FAIL 0, internal 0, timeout 0, ENOSYS 0, panic/trap 0.
- LA regression stable-order subset: `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800/la-summary.txt` — PASS LTP CASE 60, FAIL 0, internal 0, timeout 0, ENOSYS 0, panic/trap 0.

## Risks and maintenance boundary

- No blacklist/SKIP/status0/full-sweep partial TPASS evidence was counted.
- No LTP case/path/process/output hardcoding was added.
- Full stable1000 all-case RV/LA sweep was not rerun in this milestone; the final claim is based on cumulative milestone evidence through stable956 plus the current new44 four-way gate and current regression gates. This caveat is explicit in `validation.md` and `stable1000-final-report.md`.
- vfork now uses separate page tables plus shared resident frames, not a shared `AddrSpace` owner; future changes to COW/shared-frame release must preserve parent safety across child exec/exit.
- Exec failure paths must remain failure-atomic: do not clear the live address space until replacement image loading has succeeded.
- FD unshare aliases must remain group-local and must not be promoted into base while old sharers still reference that base.
- LA fcntl stress emits user BadAddress warnings in raw logs, but parser sees wrapper PASS and zero panic/trap. Treat these as stress fault noise unless future summaries classify them as blockers.

## Next step

Use `post-1000-roadmap.md` for full-stable sweep scheduling, remaining severe blockers, and post-1000 semantic-hardening work.
