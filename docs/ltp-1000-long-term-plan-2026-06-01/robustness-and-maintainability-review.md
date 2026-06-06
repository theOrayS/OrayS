# stable1000 robustness and maintainability review

## Strengthened areas

- User-entry register restore is now structurally simpler and less sensitive to compiler prologue/codegen details.
- vfork behavior separates child teardown safety from parent-visible pre-exec writes, reducing the parent address-space corruption risk that blocked glibc `system()`-based tests.
- exec replacement is failure-atomic: the live image is not cleared until scratch loading succeeds, and the task page-table root is synchronized after swap.
- FD-table unshare/clone-files semantics use explicit alias ownership instead of mutating the base shared table observed by older processes.
- Busybox applet alias resolution is path-generic and rooted in real runtime files, avoiding temporary wrapper execution-mode artifacts.
- Final validation includes both new-candidate gates and regression subsets for the lanes most affected by the repairs.

## Maintenance boundaries

- Preserve generic Linux/POSIX semantics. Do not add case-name heuristics around `clone05`, `creat07`, `pipeio`, close_range, execve, or fcntl tests.
- Re-run the vfork/exec/pipe/close_range regression subset before changing shared-frame retention, address-space clear ownership, executable tracking, FD table aliasing, task page-table roots, or applet alias logic.
- Keep exec failure atomicity as a hard invariant: load into a scratch address space first, then commit/swap only after success.
- Treat LA BadAddress stress log noise as a cleanup target, not a reason to weaken parser gates or hide faults.
- Keep milestone promotion commits separate; future stable expansions beyond 1000 should continue to use parser-backed RV/LA x musl/glibc gates.

## Remaining risks

- Full stable1000 all-case sweep cost remains high and was not closed here.
- Shared-frame/COW interactions need continued attention for nested fork/vfork combinations.
- FD-table alias lifetime needs regression coverage if the table model is refactored.
- Broader full-sweep blockers outside the stable list remain future roadmap work rather than stable1000 evidence.
