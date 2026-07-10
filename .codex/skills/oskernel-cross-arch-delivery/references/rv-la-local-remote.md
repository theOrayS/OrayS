<a id="rv-la-local-remote"></a>
# RV/LA local and remote

<a id="rv-la-local-remote-inspect-live-facts-first"></a>
## Inspect live facts first

Read `Makefile`, `run-eval.sh`, the selected platform config, and Git state. Do not rely on remembered branch names, addresses, or artifact freshness.

<a id="rv-la-local-remote-evidence-matrix"></a>
## Evidence matrix

| Mode | Typical command | What it proves |
| --- | --- | --- |
| RV local evaluator | `./run-eval.sh rv` | Current local RV QEMU/evaluator path |
| LA local evaluator | `./run-eval.sh la` | Current local LA QEMU/evaluator path |
| RV submission build | `make kernel-rv` | Root `kernel-rv` submission artifact can be built |
| LA submission build | `make kernel-la` | Root `kernel-la` using the remote submission platform config can be built |
| Both submissions | `make all` | Both required root ELF artifacts can be produced |

<a id="remote-la-config-and-address-map"></a>
## Remote LA config and address map

The retained baseline and current inspected tree distinguish these LoongArch inputs:

- local QEMU package config `configs/platforms/axplat-loongarch64-qemu-virt.toml` uses
  `0xffff_0000_8000_0000`;
- remote submission config `configs/remote-eval/axplat-loongarch64-qemu-virt.toml` uses
  `0xffff_8000_8000_0000`.

Re-read both configs, `Makefile`, and `KERNEL_BASE_VADDR` before each diagnosis or delivery because mappings and
wiring can drift. Do not substitute the remote config into local `run-la` unless the task explicitly tests the remote
submission build. Derive boot page-table indices from the selected live address; never hardcode high-half slot `0`.
A local LA silence, trap, or zero result proves only the local path, while official remote execution is authoritative
for the remote configuration.

<a id="rv-la-local-remote-reporting"></a>
## Reporting

Report separately whether local RV, local LA, `kernel-rv`, `kernel-la`, and `make all` were run and passed. State whether local/remote address or platform rules changed. Do not describe an unrun remote evaluator as passing merely because submission artifacts were built.
