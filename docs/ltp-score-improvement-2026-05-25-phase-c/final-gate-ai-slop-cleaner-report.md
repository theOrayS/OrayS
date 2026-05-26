# Final gate AI slop cleaner report

Date: 2026-05-26
Scope: `examples/shell/src/uspace/fd_socket.rs`, `examples/shell/src/cmd.rs`, phase-c reports.

## Behavior lock

- Pre-cleaner targeted evidence: `raw/target-stable400-chmod-fchmod-rv-001-summary.txt` and `raw/target-stable400-chmod-fchmod-la-001-summary.txt` were clean for `chmod05,fchmod05` on RV+LA x musl+glibc.
- Aggregate evidence: `raw/stable381-rv-gate-001-summary.txt` and `raw/stable381-la-gate-001-summary.txt` were clean for stable381.
- Post-review targeted evidence after the errno/path-resolution cleanup: `raw/target-stable400-chmod-fchmod-rv-002-summary.txt` and `raw/target-stable400-chmod-fchmod-la-002-summary.txt` are clean.

## Cleanup plan and pass

1. Keep the promotion list edit minimal: add only `chmod05` and `fchmod05`.
2. Remove the masking fallback behavior where local AF_UNIX socket `connect()` fell through to the network socket-only bridge and returned blanket `ENOTSOCK`.
3. Keep the AF_UNIX change explicitly partial: no fake listener registry, no broad bind/listen/accept implementation.
4. Resolve the reviewer finding by using process-visible path resolution for pathname connects and returning `ECONNREFUSED` for existing pathname targets until a real AF_UNIX listener registry exists.

## Fallback findings

- Finding: local AF_UNIX `connect()` previously used a network-socket-only fallback path and returned `ENOTSOCK` for valid local socket fds.
  - Classification: masking fallback slop.
  - Resolution: replaced with a bounded local-socket connect errno shim that preserves errors for invalid user pointers/families and reports missing pathname as `ENOENT`.
- Finding: no real AF_UNIX pathname socket namespace exists.
  - Classification: grounded partial compatibility boundary.
  - Resolution: documented in code and reports; no broad fake support was added.

## Quality gates

- `cargo fmt --all -- --check`: PASS.
- `make A=examples/shell ARCH=riscv64`: PASS; this repo target also rebuilt remote `kernel-rv`/`kernel-la` outputs.
- `git diff --check`: PASS.
- RV targeted post-review: `target-stable400-chmod-fchmod-rv-002-summary.txt` PASS 4 / FAIL 0, internal 0.
- LA targeted post-review: `target-stable400-chmod-fchmod-la-002-summary.txt` PASS 4 / FAIL 0, internal 0.

## Remaining risks

- This is still partial AF_UNIX pathname connect support, not a complete AF_UNIX namespace. Future broader AF_UNIX tests must implement real bind/listen/accept/connect semantics instead of relying on this errno shim.
