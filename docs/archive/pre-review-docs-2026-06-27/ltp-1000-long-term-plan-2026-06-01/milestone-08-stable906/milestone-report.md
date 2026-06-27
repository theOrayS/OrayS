# Milestone 08: stable906

## Goal

Advance `LTP_STABLE_CASES` from stable856 to stable906 with 50 trustworthy unique cases, without counting blacklist/SKIP/status0/full-sweep partial evidence.

## Result

- Stable baseline: 906 total / 906 unique / 0 duplicate.
- New promoted cases: 50, listed in `targeted-cases.txt`.
- Fresh final gate: RV + LA x musl + glibc, 50 cases per libc/arch, parser-clean.
- Adjacent SysV shm stable regression subset: RV + LA x musl + glibc, parser-clean.

## Code and stable-list changes

- Added `examples/shell/src/uspace/sysv_msg.rs` with bounded in-memory SysV message queue support.
- Added syscall dispatch for `msgget`, `msgsnd`, `msgrcv`, `msgctl` in `examples/shell/src/uspace/syscall_dispatch.rs`.
- Registered the module in `examples/shell/src/uspace/mod.rs`.
- Added 50 cases to `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.

## Evidence

Primary evidence is in `validation.md`:

- New50 RV final: `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.summary.txt` => PASS 100 / FAIL 0 / parser-clean.
- New50 LA final: `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.summary.txt` => PASS 100 / FAIL 0 / parser-clean.
- Combined new50: `target/ltp-1000-milestone-08-stable906/stable906-new50-rvla-final-gate-20260605T115135+0800.txt` => 50 candidates, 4 combos each, 0 blocked.
- SysV msg targeted RV/LA: `target/ltp-1000-milestone-08-stable906/rv-sysv-msg-fix-20260605T113226+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/la-sysv-msg-fix-20260605T113700+0800.summary.txt` => 18+18 PASS, parser-clean.
- Adjacent SysV shm RV/LA: `target/ltp-1000-milestone-08-stable906/rv-sysv-ipc-adjacent-stable-regression-20260605T120125+0800.summary.txt`, `target/ltp-1000-milestone-08-stable906/la-sysv-ipc-adjacent-stable-regression-20260605T120551+0800.summary.txt` => 22+22 PASS, parser-clean.

## Conclusion

stable906 promotion is accepted for this milestone. The new SysV msg semantics are real syscall behavior, not wrapper output or case-name hardcoding, and the final new50 gate is clean across both architectures and libc variants.

## Risks and next steps

- SysV msg support is intentionally minimal: no blocking wait queues, no namespace model, and bounded queue/message sizes. Future broader SysV IPC tests may require blocking/EIDRM/wakeup behavior.
- Full stable906 sweep was not run in this milestone gate; next milestone should preserve targeted gates and schedule a stable-regression shard before stable956.
- Continue toward stable956 by prioritizing low-risk real semantic lanes with four-way parser-clean evidence.
