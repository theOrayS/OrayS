# stable423 stretch report

Date: 2026-05-26
Status: deferred, not delivered.

## Decision

The phase delivered the main target stable413 with 30 new four-way clean cases. Stretch stable423 was not promoted because no additional 10-case subset had completed the full RV+LA x musl+glibc promotion path by the final gate point.

## Rationale

The user requested easy/low-risk clean cases first and explicitly rejected quantity-driven promotion. After stable413, remaining low-risk pools still contained candidates with incomplete LA confirmation, known negative evidence, or likely narrow fixes required before honest aggregate promotion. Therefore the correct stop state is stable413, not a speculative stable423.

## Suggested next stretch lanes

Continue from the worker reports and candidate matrix:

- Lightweight syscall/process: `poll02`, `times03`, `gethostname02`, `getpgid01`, `getcpu01`, and selected `fork`/`clone` cases after fresh RV scout.
- Metadata/statfs/getdents: revisit only cases with both libc binaries present and no ABI/copy-out blocker.
- VFS small create/remove: scout one small batch at a time and stop at the first setup/tool/mount-style blocker.

Each future stretch candidate still needs RV+LA aggregate promotion evidence and must keep `read02` TCONF transparent.
