# Session 7 blacklist change report

## Active blacklist edits

Edited file: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt`.

Removed LA-only entries:

```text
creat07
tcp4-uni-basic01
```

Counts:

| File | Before | After | Delta |
| --- | ---: | ---: | ---: |
| `blacklist-la.txt` | 376 | 374 | -2 |
| `blacklist-rv.txt` | 1 | 1 | 0 |
| `blacklist-common.txt` | 5 | 5 | 0 |

## Removal rationale

- `creat07` original reason: LA-only sweep hang/incomplete after TBROK checkpoint timeout. Session 7 targeted LA LTP-only run now exits normally with wrapper FAIL/TBROK in both musl and glibc, no timeout/panic/trap/resource marker. It remains a real failing LTP case, but no longer satisfies severe-blocker criteria.
- `tcp4-uni-basic01` original reason: inherited LA network stress resource-pollution family blacklist. Session 7 targeted LA LTP-only run exits normally with wrapper FAIL/TCONF in both musl and glibc, no timeout/panic/trap/resource marker. Only this single case is removed; remaining network family entries retain original severe blocker status until individually or shard-proven.

## Non-removal / boundaries

- No stable promotion. These cases are ordinary FAIL/TBROK/TCONF, not PASS.
- No change to common or RV blacklist.
- No removal of allocator panic/hang entries such as `fsync02`, `lftest`, `mmstress`, `write01`, `futex_wait01`, `futex_wait05`, `nice05`.
- No claim that the entire network family is safe; only `tcp4-uni-basic01` was removed.
