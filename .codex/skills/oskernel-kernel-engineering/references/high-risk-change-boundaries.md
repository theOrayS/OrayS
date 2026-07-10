<a id="high-risk-change-boundaries"></a>
# High-Risk Change Boundaries

<a id="high-risk-change-boundaries-general-patch-discipline"></a>
## General Patch Discipline

- Make the smallest local and reversible patch; reuse existing utilities and conventions first.
- Preserve platform and feature structure. Do not merge architecture branches merely to shorten control flow.
- Avoid repo-wide replacement, mechanical renaming, import normalization, or broad formatting.
- Avoid unsupported `unwrap()`/`expect()` in runtime, syscall, filesystem, networking, and user-input paths.
- Use the existing logging facility in `kernel/`, `api/`, and `ulib/`; visible shell/evaluator output is an interface.

<a id="high-risk-change-boundaries-rust-and-unsafe"></a>
## Rust and Unsafe

Existing low-level `unsafe` is not a license for a wider trust boundary. Keep each new block narrow, validate external
preconditions before entry, describe non-obvious invariants with `// SAFETY:`, and preserve ownership/lifetime rules
across interrupts, scheduling, and concurrent access.

<a id="high-risk-change-boundaries-posix-and-user-memory"></a>
## POSIX and User Memory

Raw pointers, counts, offsets, iovecs, strings, and ABI structs are untrusted. Check nullability, range overflow,
alignment where required, access direction, address-space validity, and partial-copy behavior before constructing Rust
references or slices. Never retain a borrowed userspace reference across a scheduling or mutation boundary.

<a id="high-risk-change-boundaries-path-specific-gates"></a>
## Path-Specific Gates

<a id="high-risk-change-boundaries-api-arceos-posix-api"></a>
### `api/arceos_posix_api/`

Audit syscall dispatch, return/errno mapping, flags, ABI layout, copy-in/copy-out, and FD/process/signal/futex/mmap/ELF
interactions. `src/uspace.rs` is a broad integration file: prefer a small helper or early return to a sweeping rewrite.

<a id="high-risk-change-boundaries-examples-shell"></a>
### `examples/shell/`

Preserve wrapper marker syntax and expose inner LTP results, `TCONF`, timeout, `ENOSYS`, panic, and trap honestly.
Case selection, timeout, and cleanup changes need general operational reasons and must not recognize case identities.

<a id="high-risk-change-boundaries-kernel-arch-axhal-kernel-runtime-axruntime-kernel-task-axtask"></a>
### `kernel/arch/axhal/`, `kernel/runtime/axruntime/`, `kernel/task/axtask/`

Preserve cross-architecture cfg and state RV/LA impact. These paths affect boot, traps, scheduling, and userspace return;
build before QEMU/evaluator validation, and never extrapolate local LA address mapping to remote LA configuration.

<a id="high-risk-change-boundaries-vendor-cargo-home-tools-bin"></a>
### `vendor/`, `cargo-home/`, `tools/bin/`

Do not modify these paths without an explicit dependency, offline-build, or helper-behavior objective. Such a change
requires online and offline-style closure evidence; if it cannot be run, report the gap rather than assuming parity.

<a id="high-risk-change-boundaries-required-handoffs"></a>
## Required Handoffs

Use `$oskernel-validation` for the validation matrix and evidence/claim boundary,
`$oskernel-cross-arch-delivery` for platform and remote/offline procedures, and
`$oskernel-repo-hygiene` before long or disk-intensive execution. Do not copy their runbooks here.
