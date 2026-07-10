<a id="layout-and-generated-artifacts"></a>
# Layout and generated artifacts

<a id="layout-and-generated-artifacts-repository-map"></a>
## Repository map

- `kernel/`: runtime, architecture, tasks, drivers, and other kernel subsystems.
- `api/arceos_posix_api/`: Linux/POSIX and userspace boundary.
- `ulib/`: userspace libraries.
- `examples/shell/`: shell plus evaluator/LTP integration.
- `configs/`: platform and remote evaluator configuration.
- `scripts/`, `tools/`: build, evaluation, and support tools.
- `vendor/`, `cargo-home/`: offline dependency/tool closure.
- `.codex/`, `.omx/`: project guidance and runtime/evidence state.

Read the live tree before acting; names and ownership can change.

<a id="layout-and-generated-artifacts-protected-generated-and-local-evidence"></a>
## Protected generated and local evidence

Unless the task explicitly owns them, do not edit, delete, stage, or commit:

```text
kernel-rv
kernel-la
sdcard-*.img
disk*.img
output*.md
*.log
.axconfig.toml
build/
target/
```

Local symlinks, evaluator output, raw logs, images, and ignored evidence may be required for reproduction even when Git does not track them. Inspect provenance and current users before cleanup.

<a id="layout-and-generated-artifacts-dirty-baseline"></a>
## Dirty baseline

Assume every worktree may contain another owner's changes. Capture `git status --porcelain=v1 -uall`, identify the task allowlist, and preserve unrelated tracked, staged, and untracked content byte-for-byte. Do not revert, overwrite, format, stage, or commit outside the owned scope.
