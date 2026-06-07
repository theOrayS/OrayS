# G008 musl runtime patch manifest

Date: 2026-06-07
Scope: `G008-g008-phase-7-8-musl-patch-stable`
Status: source-level manifest and promotion quarantine gate; runtime ELF offsets/hashes still require evaluator/runtime image evidence before promotion.

## Policy

Runtime byte patching is a temporary compatibility bridge, not a stable-promotion foundation. A case that depends on any patch below may be released from quarantine only when all of these are true:

1. the patch appears in the source manifest in `examples/shell/src/uspace/program_loader.rs`;
2. the actual runtime ELF observation records target libc, symbol, file offset, original-prefix hash, patched-prefix hash, and runtime image path/hash;
3. the underlying raw syscall behavior is validated directly;
4. the same behavior is validated through musl and glibc wrappers;
5. the RV + LA x musl + glibc evidence is parser-clean through `scripts/ltp_summary.py --promotion-candidates`;
6. no TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap/prior failure event is hidden.

If runtime ELF files are unavailable, the offset/hash cells below remain `runtime-required`; that is a blocker for promotion, not a reason to infer success.

## Source manifest

The loader now carries an explicit manifest and validates it before applying musl patches:

- `RISCV_MUSL_PATCH_MANIFEST`
- `LOONGARCH_MUSL_PATCH_MANIFEST`
- `MUSL_PATCH_RETIREMENT_DIRECTIVE`

Any new musl patch symbol must be added to the manifest first, or the patch entry path should fail during loading with a manifest error.

## RISC-V patch entries

| Arch | Target | Symbol | Raw syscall/API proof required | Wrapper proof required | Offset/hash status | Retirement condition |
| --- | --- | --- | --- | --- | --- | --- |
| RV64 | main executable | `brk` | raw `brk` grows/queries program break with Linux-compatible errno | musl `brk`/allocation and glibc `brk`/allocation pass the same smoke/LTP behavior | runtime-required | rebuild musl or remove ENOSYS stub dependency |
| RV64 | musl interpreter | `brk` | raw `brk` | musl + glibc brk/allocation comparison | runtime-required | rebuild musl or remove loader patch |
| RV64 | musl interpreter | `sbrk` | raw `brk` query/grow/fail cases | musl `sbrk` and glibc `sbrk` compare old break and errno | runtime-required | libc wrapper fixed upstream/local runtime rebuilt |
| RV64 | musl interpreter | `nice` | raw `getpriority` + `setpriority` including permission errno | musl `nice` and glibc `nice` errno/value comparison | runtime-required | musl wrapper no longer needs runtime rewrite |
| RV64 | musl interpreter | `gethostname` | raw `uname`/hostname source and ENAMETOOLONG boundary | musl `gethostname` and glibc `gethostname` match value/errno | runtime-required | runtime wrapper rebuilt or kernel exposes compatible route without patch |

## LoongArch patch entries

| Arch | Target | Symbol | Raw syscall/API proof required | Wrapper proof required | Offset/hash status | Retirement condition |
| --- | --- | --- | --- | --- | --- | --- |
| LA64 | main executable | `brk` | raw `brk` grows/queries program break with Linux-compatible errno | musl/glibc brk/allocation comparison | runtime-required | rebuild musl or remove ENOSYS stub dependency |
| LA64 | musl interpreter | `sched_setparam` | raw `sched_setparam` supports/denies with real errno | musl/glibc wrapper errno comparison | runtime-required | musl wrapper rebuilt |
| LA64 | musl interpreter | `sched_getparam` | raw `sched_getparam` returns observable scheduler parameters or honest errno | musl/glibc wrapper comparison | runtime-required | musl wrapper rebuilt |
| LA64 | musl interpreter | `sched_setscheduler` | raw `sched_setscheduler` policy/permission behavior | musl/glibc wrapper comparison | runtime-required | musl wrapper rebuilt |
| LA64 | musl interpreter | `sched_getscheduler` | raw `sched_getscheduler` policy readback or honest errno | musl/glibc wrapper comparison | runtime-required | musl wrapper rebuilt |
| LA64 | musl interpreter | `brk` | raw `brk` | musl/glibc brk/allocation comparison | runtime-required | rebuild musl or remove loader patch |
| LA64 | musl interpreter | `sbrk` | raw `brk` query/grow/fail cases | musl/glibc `sbrk` comparison | runtime-required | libc wrapper fixed upstream/local runtime rebuilt |
| LA64 | musl interpreter | `gethostname` | raw `uname`/hostname source and ENAMETOOLONG boundary | musl/glibc `gethostname` comparison | runtime-required | runtime wrapper rebuilt or kernel exposes compatible route without patch |
| LA64 | musl interpreter | `readlink` | raw `readlinkat(AT_FDCWD, ...)` including ENOENT/EINVAL/size behavior | musl/glibc `readlink` comparison | runtime-required | musl wrapper rebuilt |
| LA64 | musl interpreter | `readlinkat` | raw `readlinkat` including dirfd and size behavior | musl/glibc `readlinkat` comparison | runtime-required | musl wrapper rebuilt |

## Current G008 evidence boundary

This checkout does not expose host `/musl` or `/glibc` runtime ELF files, and this story did not run QEMU/LTP runtime. Therefore G008 does not promote or restore any stable case from musl patch evidence. It only makes the patch surface explicit and blocks future promotion unless runtime offsets/hashes and cross-libc evidence are supplied.

## Required runtime observation schema

Future runtime/evaluator evidence for each patch must include a JSON or Markdown table with at least:

```json
{
  "arch": "rv|la",
  "libc": "musl",
  "runtime_image": "<path>",
  "runtime_image_sha256": "<sha256>",
  "target": "main-executable|interpreter",
  "symbol": "<symbol>",
  "file_offset": "0x...",
  "original_prefix_sha256": "<sha256>",
  "patched_prefix_sha256": "<sha256>",
  "raw_syscall_evidence": "<log/summary path>",
  "musl_evidence": "<log/summary path>",
  "glibc_evidence": "<log/summary path>",
  "promotion_summary": "scripts/ltp_summary.py --promotion-candidates output"
}
```

Missing fields keep the affected cases quarantined.
