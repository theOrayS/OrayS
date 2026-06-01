# Milestone 03 stable656 ABI and behavior impact

This checkpoint changes documentation only. It does not modify kernel, POSIX API, userspace ABI, evaluator scripts, or `LTP_STABLE_CASES`.

## Code/ABI impact

- Syscalls: no implementation changes.
- Errno values: no implementation changes.
- Flags and struct layouts: no implementation changes.
- File descriptors / signal / futex / mmap / user pointer semantics: no implementation changes.
- Stable LTP list: unchanged at `606 total / 606 unique / 0 duplicate`.

## Behavior gaps exposed by the scout

The RV scout points to real semantics work before promotion can continue in this lane:

1. `mmap05` / `munmap01`: likely recoverable user page-fault signal delivery gaps. Tests that should be able to handle a user `SIGSEGV` instead terminate or break.
2. `mmap13`: file-backed mapping beyond EOF does not deliver the expected `SIGBUS` behavior.
3. `futex_wait03`: futex wait timeout path does not complete within the case timeout and is killed.
4. `mmap10_1`: testcase is absent from both guest LTP trees, so it cannot become stable evidence in the current inventory.
5. `vma02`: libnuma requirement produces `TCONF`, so it is environment/configuration-blocked rather than a kernel PASS.

## Maintenance boundary

Future fixes must be generic Linux/POSIX-visible behavior fixes. Do not hardcode these LTP case names, paths, processes, or outputs. Any signal/futex/mmap changes must be covered by adjacent regression sets before promotion.
