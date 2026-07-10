<a id="architecture-and-semantics"></a>
# Architecture and Semantics

<a id="architecture-and-semantics-canonical-identity"></a>
## Canonical Identity

This repository is an ArceOS-based modular operating-system kernel for Linux/POSIX userspace compatibility. Preserve
modularity, configurability, and multi-architecture support; do not claim complete Linux coverage or an unbounded
general-purpose kernel.

<a id="architecture-and-semantics-quality-order"></a>
## Quality Order

1. Correctness and honest semantics.
2. Robustness plus security and resource boundaries.
3. Architectural integrity and maintainability.
4. Performance and resource efficiency without weakening real semantics.
5. Isolated, measurable, reversible experimental innovation.
6. Official scores and existing cases as regression floors, not the default roadmap.

<a id="architecture-and-semantics-layer-map"></a>
## Layer Map

- `kernel/`: runtime, architecture, tasks, drivers, and low-level subsystems.
- `api/arceos_posix_api/`: Linux/POSIX system-call and userspace boundary.
- `ulib/`: userspace library and ABI-facing wrappers.
- `examples/shell/`: shell plus evaluator integration surface, not merely a demo.
- `configs/`: platform and evaluator configurations.
- `scripts/` and `tools/`: build and verification helpers.

Keep a change in its natural owner. A cross-layer patch requires a concrete interface reason, an explicit impact map,
and validation at every changed boundary; convenience alone is not sufficient.

<a id="architecture-and-semantics-semantic-change-checklist"></a>
## Semantic Change Checklist

For any Linux/POSIX-visible change, record:

- syscall number and dispatch path;
- success return, failure sentinel, and precise errno behavior;
- accepted/rejected flags and unknown-flag handling;
- ABI struct size, alignment, field layout, signedness, and copy direction;
- FD lifetime, sharing, offsets, close/dup behavior, and resource limits;
- signal, futex, task/process, mmap, network, and ELF-loading interactions;
- raw-pointer validation, copy-in/copy-out partial-failure behavior, and overflow checks;
- musl/glibc and RV/LA exposure;
- observable logging/output effects.

If an item is unaffected, say so rather than omitting it. Do not turn an unsupported operation into `Ok(0)`, a null
success, an empty state transition, or a wrapper PASS. Choose the native failure convention for the API layer.

<a id="architecture-and-semantics-architecture-change-gate"></a>
## Architecture Change Gate

Before changing a subsystem boundary, document the current owner, proposed owner, invariant preserved, rejected
smaller alternative, dependency direction, feature/cfg impact, rollback surface, and validation owner. Cross-cutting
abstractions require evidence of repeated need; speculative generalization is technical debt, not architecture.
