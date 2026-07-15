## Repository

OrayS is a Rust no_std operating-system project derived from ArceOS.

The primary competition architectures are:

- riscv64
- loongarch64

The Linux/POSIX compatibility implementation is currently concentrated in:

- user/shell/src/uspace
- api/arceos_posix_api

Before editing, read:

- README.md
- Cargo.toml
- user/shell/Cargo.toml
- user/shell/src/uspace/mod.rs
- user/shell/src/uspace/linux_abi.rs
- user/shell/src/uspace/user_memory.rs
- user/shell/src/uspace/syscall_dispatch.rs
- .github/workflows/build.yml
- .github/workflows/test.yml

## Required workflow

- Inspect the current implementation before proposing changes.
- State the planned files and invariants before editing.
- Make one reviewable milestone at a time.
- Do not commit or push unless explicitly instructed.
- Do not modify unrelated files.
- Do not perform repository-wide formatting.
- Do not run cargo update.
- Do not upgrade dependency versions.
- Do not change rust-toolchain.toml.
- Do not add a crates.io dependency without approval.
- Do not access the network unless explicitly approved.
- Treat existing build or test failures as baseline failures.
- Do not fix unrelated baseline failures in this PR.

## PR1 objective

Establish architectural boundaries for the Linux compatibility layer
without changing externally visible behavior.

The intended dependency direction is:

orays-linux-abi
    ↓
orays-linux
    ↓
arceos-shell

The target responsibilities are:

### orays-linux-abi

- no_std
- Linux ABI constants
- syscall numbers
- repr(C) ABI data structures
- architecture-specific ABI definitions
- no dependency on OrayS kernel implementation crates
- no dependency on arceos-shell

### orays-linux

- no_std, with alloc only where required
- typed user addresses, pointers, slices, and access markers
- syscall metadata types
- implementation-independent interfaces
- no dependency on arceos-shell
- no ownership of UserProcess during the first milestone

### arceos-shell

- implements backend adapters
- continues to own UserProcess and existing syscall handlers during PR1
- uses compatibility re-exports while migration is in progress

## PR1 non-goals

- Do not change syscall return values or errno behavior.
- Do not change blocking, signal, process, FD, VM, or scheduling semantics.
- Do not move the complete uspace directory in one change.
- Do not move UserProcess in the first milestone.
- Do not redesign FdTable, process lifecycle, signals, or mmap.
- Do not generate the syscall dispatcher from metadata yet.
- Do not remove legacy module paths until all callers are migrated.
- Do not introduce broad allow attributes to silence warnings.
- Do not introduce new unsafe code outside an explicitly designated
  low-level user-copy module.

## Compatibility requirements

Preserve:

- all syscall numbers
- repr(C) layouts
- size and alignment of ABI structures
- cfg(target_arch) behavior
- Cargo feature behavior
- LinuxError and errno mapping
- existing public and crate-visible paths through temporary re-exports
- current RISC-V64 and LoongArch64 build behavior

When moving declarations, prefer compatibility re-exports over changing
all call sites at once.

## Validation

After every implementation milestone, run:

```bash
cargo fmt --all -- --check
make clippy ARCH=riscv64
make clippy ARCH=loongarch64
make ARCH=riscv64 A=user/shell
make ARCH=loongarch64 A=user/shell
make unittest_no_fail_fast
git diff --check
````

Also report:

* every command executed
* its exit status
* changed files
* known failures
* unsafe blocks added, removed, or moved
* Cargo.lock changes and why they occurred

## Stop conditions

Stop and report before continuing when:

* a dependency cycle is discovered
* an ABI value or layout would need to change
* existing syscall behavior appears inconsistent
* passing tests would require an unrelated fix
* moving a symbol requires moving UserProcess
* a new external dependency appears necessary
* more than one major subsystem must be modified
