# runtime-binary-patch-prohibition musl runtime patch retirement note

Date: 2026-06-18
Scope: `fix/self-check-compliance-20260618`
Status: retired; runtime byte patching is prohibited.

## Policy

The loader must not patch ELF bytes, symbol bodies, instruction prefixes, or executable LOAD segments at runtime.  If a musl wrapper or shipped runtime contains an incompatible stub, the compliant options are to rebuild or replace the runtime, implement the underlying Linux syscall semantics in the kernel, or report the raw failure honestly.  Runtime byte patching is prohibited because it depends on binary features, fixed symbol names, and runtime layout rather than a general kernel contract.

Any future repair must be validated at the syscall/API boundary instead of by binary rewriting:

1. raw syscall behavior is checked directly;
2. musl and glibc wrapper behavior is compared against that raw syscall behavior;
3. `TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap` output remains visible;
4. promotion still requires the RV64/LA64 x musl/glibc matrix in `stable-reproof-gate.md`.

## Retired source surface

`user/shell/src/uspace/program_loader.rs` no longer carries musl patch manifests, symbol-offset lookup helpers, executable-segment patch-area reservations, or architecture-specific `patch_*_musl_*` functions.  Reintroducing any of those helpers should fail `test/checks/check_runtime_binary_patch_prohibition.py` until a non-binary-rewrite design is reviewed.

## Replacement rule

do not patch ELF bytes in the loader.  For missing musl compatibility, rebuild or replace the runtime or implement the real kernel semantics.  If neither is available, return/report the real unsupported behavior instead of hiding it with loader-side machine-code modification.
