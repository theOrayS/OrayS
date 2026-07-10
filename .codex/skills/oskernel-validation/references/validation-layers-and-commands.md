<a id="validation-layers-and-commands"></a>
# Validation layers and commands

<a id="validation-layers-and-commands-selection-order"></a>
## Selection order

Choose the lowest layer that proves the claim and add higher layers when the touched boundary or failure mode demands them.

1. **Text and schema:** validate Markdown/YAML/JSON structure, links, parsers, and `git diff --check`.
2. **Format and static analysis:** use touched-file formatting first; use `make fmt`, `make fmt_c`, `cargo fmt --all -- --check`, or `make clippy` only when their scope matches the change.
3. **Unit and module:** run the narrow test or package check, then `make unittest_no_fail_fast` when shared library behavior changed.
4. **Build:** build the touched example or boundary. POSIX/userspace work requires at least `make A=examples/shell ARCH=riscv64`.
5. **Runtime:** use `make run-rv ARCH=riscv64`, `make run-la ARCH=loongarch64`, or the relevant QEMU-backed example when boot/runtime behavior is part of the claim.
6. **Evaluator and submission:** use `./run-eval.sh rv`, `./run-eval.sh la`, `make kernel-rv`, `make kernel-la`, or `make all` only when evaluator/submission behavior is in scope.

Run long builds, QEMU, vendoring, Docker, and full evaluators only after the repository-hygiene disk gate. Preserve raw failures and summarize LTP output with `python3 scripts/ltp_summary.py <log>` rather than visually inferring success.

<a id="validation-layers-and-commands-toolchain-facts"></a>
## Toolchain facts

- Read `rust-toolchain.toml` live; pinned-toolchain failure is a regression.
- Read the `Makefile` and current configs before selecting `ARCH`, `A`/`APP`, features, platform config, or runtime flags.
- `make testsuite-sdcard` requires the testsuite checkout or an explicit `TESTSUITE_DIR`.
- C work uses the repository `.clang-format`; prefer targeted checks in dirty/team worktrees.

<a id="validation-layers-and-commands-high-risk-staged-validation"></a>
## High-risk staged validation

For boot, trap, scheduler, task, user-pointer, POSIX, or HAL flows:

1. Prove the smallest build/static prerequisite.
2. Run targeted behavior tests.
3. Run adjacent subsystem regression.
4. Add QEMU/evaluator evidence only when required by the claim.

Record missing QEMU, image, cross-toolchain, testsuite, credential, network, or time-window prerequisites as gaps, not passes.
