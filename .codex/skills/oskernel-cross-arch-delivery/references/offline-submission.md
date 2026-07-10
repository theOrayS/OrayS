<a id="offline-submission"></a>
# Offline submission

Remote evaluation may be offline or network-unreliable. Submission builds must not silently download crates or install tools.

<a id="offline-submission-dependency-closure"></a>
## Dependency closure

When a task intentionally changes dependencies, helpers, toolchains, or platform data, inspect and keep the applicable closure synchronized:

- `tools/bin/`
- `configs/platforms/` and remote evaluator configs
- `cargo-home/`
- `vendor/cargo-vendor.tar.gz` and restored `vendor/cargo/`

Such changes require an explicit task reason and matching validation; do not refresh vendor/tool content during unrelated delivery work.

<a id="offline-submission-checks"></a>
## Checks

After the repository-hygiene disk preflight, run the smallest applicable sequence:

```bash
make all
CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all
file kernel-rv kernel-la
```

Use `scripts/ensure-cargo-vendor.sh` only when the current build contract requires restoring the vendored directory. Record any missing cross tool, archive, QEMU, image, or time budget as a delivery gap.

Never edit or commit `kernel-rv`, `kernel-la`, images, build trees, or logs unless the task explicitly owns those generated artifacts.
