# final gate ai-slop-cleaner report

Status: **anti-slop audit completed for stable270 partial delivery**.

## Cleanup / slop audit

- No new dependencies were added.
- No vendor changes were made.
- No generated kernel, sdcard/disk image, or large raw log is required for commit.
- The stable list change is small and evidence-backed: +20 cases, total 270 unique.
- The implementation reuses existing user-memory, fd-table, process, signal, and syscall dispatch structures instead of adding a parallel LTP harness.
- Failed candidates remain failed in reports; none were converted to PASS/TCONF/SKIP.

## Behavior lock evidence

- `cargo fmt --all -- --check`: PASS.
- `make A=examples/shell ARCH=riscv64`: PASS earlier in the run through evaluator build path.
- RV stable270 aggregate: PASS 540 / FAIL 0 with only known `read02` TCONF.
- LA stable270 aggregate: PASS 540 / FAIL 0 with only known `read02` TCONF.
- Marker prefix check: 1118 marker lines checked, 0 bad marker lines.

## Explicitly rejected shortcuts

- Rejected promoting `waitpid01`: musl still has internal TFAIL.
- Rejected promoting `pipe13`: timeout is not PASS.
- Rejected promoting openat2/close_range/timer-create/statx/mmap failures as TCONF/SKIP: they remain blockers.
- Rejected promoting four post270 clean candidates without the stable285 aggregate gate; this would not satisfy the requested phase milestone.
