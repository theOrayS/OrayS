# G008 stable re-proof gate

Date: 2026-06-07
Scope: final Phase 8 gate after fake-success cleanup.

## Live stable baseline

`user/shell/src/cmd.rs::LTP_STABLE_CASES` was re-read during G008:

| Metric | Value |
| --- | ---: |
| Total entries | 1000 |
| Unique cases | 1000 |
| Duplicate extra entries | 0 |
| Duplicate case names | 0 |

This is a baseline only. It is not a renewed promotion claim after the fake-success cleanup.

## Required matrix

A case can be considered trusted stable only after all four combinations are present and clean:

| Architecture | libc |
| --- | --- |
| RV64 | musl |
| RV64 | glibc |
| LA64 | musl |
| LA64 | glibc |

The parser command for promotion review is:

```bash
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc <rv-log> <la-log>
```

The default parser settings already require `rv,la` and `musl,glibc`; the explicit flags are shown to make reports auditable.

## Hard blockers

A result is not promotion evidence if any of these appear in the parser row, group, or report:

- missing RV/LA or musl/glibc combination;
- non-stable selection mode such as blacklist, score-blacklist, all-minus-blacklist, sweep:blacklist, stable-plus-blacklist, or stable-plus-all-minus-blacklist;
- TCONF, TBROK, TFAIL, ENOSYS/not implemented, timeout, panic, trap, or prior fail event;
- status0-only result without parser-clean case evidence;
- synthetic `/proc`, `/dev`, `/etc`, config, metadata, or userdb probe-only success;
- named LTP case/path/process special handling;
- runtime musl byte patch evidence without the manifest and runtime offset/hash/cross-check requirements in `musl-runtime-patch-manifest.md`.

## Re-proof groups after G001-G007

The existing 1000-case list should be revalidated in groups before any stable restoration/extension:

1. FS/stat/metadata: cases that rely on stat/lstat/readlink/symlink fields.
2. FD/fcntl/rlimit/sysconf: cases that rely on set/get/readback or fd limits.
3. runner/parser: cases previously run through stable-plus or blacklist-adjacent modes.
4. synthetic proc/dev/config: cases that probe kernel features or block devices.
5. socket/time/mempolicy: cases that need observable socket options, timer delivery, or NUMA policy behavior.
6. runtime libc wrappers: cases that depend on musl loader patches listed in `musl-runtime-patch-manifest.md`.

## G008 result boundary

G008 does not add or remove entries from `LTP_STABLE_CASES`. The correct final state for this story is:

- source-level musl patch manifest exists and is guarded;
- promotion gate requires RV + LA x musl + glibc parser-clean evidence;
- current missing runtime/QEMU evidence is documented as `Not-tested`, not inferred;
- future stable changes must have their own milestone docs and commits.
