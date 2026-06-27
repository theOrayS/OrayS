# ABI and behavior impact for stable1000

## Stable-list visible impact

`LTP_STABLE_CASES` increases from 956 to 1000 total / 1000 unique / 0 duplicate. This changes evaluator-visible stable selection only after RV+LA x musl+glibc parser-clean evidence closed for the 44 new cases.

## Syscall / ABI changes

- RISC-V user return: the user-mode entry path now uses a naked trampoline to restore the saved trap frame directly. This is an ABI-visible correctness fix for clone/exec children because saved callee registers are no longer overwritten by Rust function prologue/temporaries.
- vfork/clone address-space semantics: vfork-style `CLONE_VM|CLONE_VFORK` no longer shares the same `AddrSpace` owner for child exec/exit. The child receives a separate page table but shares resident writable frames when needed, preserving Linux-visible child writes before exec/exit while avoiding parent address-space teardown corruption.
- exec failure atomicity: `execve()` replacement images are loaded into a scratch address space and swapped into the live process only after success. Failed `execve()` returns errno without destroying or partially rebuilding the caller's existing image.
- exec page-table lifetime: after a successful exec swap, the current hardware page table and saved task context page-table root are updated together, preventing later context switches from restoring the stale root.
- vfork parent wakeup: successful child exec marks `vfork_exec_done` and wakes the blocked parent; child exit still wakes through the existing teardown path.
- FD sharing semantics: `CLOSE_RANGE_UNSHARE` private tables can be shared with a later `clone(CLONE_FILES)` child through explicit alias ownership, without replacing the base table seen by older sharers.
- exec path/app-alias behavior: rooted `/musl/<applet>` and `/glibc/<applet>` busybox aliases resolve through real busybox binaries when the target applet path is missing. This is generic applet resolution, not LTP-case-specific behavior.
- executable lifetime/open-for-write behavior: running executable tracking is used by `creat07`-class ETXTBUSY behavior and process teardown untracking.
- LTP harness PATH behavior: stable runs prefer real `/musl`/`/glibc` applet aliases and avoid non-executable `/tmp` wrapper scripts shadowing them. This affects evaluator integration only; it does not forge PASS output.

## Resource/lifetime risk

- Shared vfork frames must be retained and released correctly through `Backend::new_shared`; future COW/shared-frame changes should re-run `clone05`, `creat07`, `execve01..06`, and close_range/pipe regressions.
- Separate page tables for vfork reduce parent-teardown risk, but exec/exit paths must continue to clear only owned address spaces.
- Exec scratch loading must remain the only path that clears a replacement address space before swap; future loader changes must not reintroduce live-aspace mutation before all fallible steps finish.
- FD alias owners must be removed or transferred correctly during close/exit; future fd-table refactors must cover `close_range01/02` and `clone(CLONE_FILES)` sharing.
- Executable tracking must remain path-generic and balanced on fork/exec/teardown to avoid stale ETXTBUSY state.
- LA fcntl stress warnings are currently parser-clean; future work should reduce noisy BadAddress logs without weakening fault semantics.

## Non-impact boundaries

- No testsuite/evaluator script was modified to force PASS.
- No LTP case/path/process/output names were hardcoded in kernel behavior.
- No blacklist/SKIP/status0/full-sweep local TPASS was counted.
- No new external dependency was introduced.
