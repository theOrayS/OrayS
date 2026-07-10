---
name: oskernel-cross-arch-delivery
description: Diagnose and deliver OSKernel across RISC-V and LoongArch local QEMU, remote evaluator platform configurations, and offline submission builds. Use for RV/LA parity, address-map/config differences, kernel-rv/kernel-la, make all, vendor/tool closure, and submission packaging; do not use for generic validation planning, LTP scoring semantics, kernel feature design, or repository cleanup.
---

# OSKernel Cross-Architecture Delivery

Separate local runtime evidence from remote-submission evidence before changing code or configuration.

## Workflow

1. Identify architecture, execution mode, expected artifact, current Git/config facts, and whether network access is permitted.
2. Consult [RV/LA local and remote](references/rv-la-local-remote.md) for platform/address-map and evidence boundaries.
3. Consult [offline submission](references/offline-submission.md) when `make all`, tool/vendor closure, or evaluator packaging is involved.
4. Run the smallest architecture-specific prerequisite, then the matching runtime or submission check.
5. Report RV local, LA local, remote-submission build, and configuration-map status separately.

## Boundaries and handoffs

- Hand generic test-layer choice and regression evidence to `$oskernel-validation`.
- Hand syscall/ABI/boot implementation design to `$oskernel-kernel-engineering`.
- Hand LTP case selection and scorer interpretation to `$oskernel-compatibility-evaluation` unless the failure is specifically platform delivery.
- Hand disk/image/cache preflight and cleanup to `$oskernel-repo-hygiene`.
- Never infer remote LA parity from local QEMU alone or substitute a local config for the remote submission config without an explicit task reason.

## Stop condition

Stop when each relevant architecture/mode has matching evidence or a named prerequisite gap, and the produced submission artifacts and address/config rules are unambiguous.
