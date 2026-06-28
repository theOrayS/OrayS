# G010 full-stable blocker repair report (2026-06-02)

## Scope

This is a blocker-repair report for ultragoal `G010-la-severe-blocker-full-sweep-shard-s` after the stable706 milestone.  It does **not** promote new stable cases and does **not** change `LTP_STABLE_CASES`.

The reproduced post-stable706 blocker subset was:

- `mmap10`: `/dev/zero` `MAP_PRIVATE` mappings raised unexpected `SIGBUS` on RV musl/glibc.
- `mmap-corruption01`: large file-backed `MAP_SHARED` stress exited as signal/code 135 on RV musl/glibc.
- `test_ioctl`: wrapper timed out on RV musl/glibc while expanding `/dev/tty*`.

Initial reproduction evidence:

- Raw: `target/ltp-1000-g010-fullstable-blockers/rv-stable706-fullblocker-targeted-20260602T224249+0800.log`
- Summary: `target/ltp-1000-g010-fullstable-blockers/rv-stable706-fullblocker-targeted-20260602T224249+0800.log.summary.txt`
- Parser result: PASS 14, FAIL 6, internal `TBROK: 4`, timeout 2, ENOSYS 0, panic/trap 0.

## Source basis

Official LTP source was checked in `/tmp/ltp-src` (git HEAD `fb34519`) for the affected tests:

- `testcases/kernel/syscalls/mmap/mmap10.c`: opens `/dev/zero`, maps it `MAP_PRIVATE`, and expects read/write/munmap plus child checks to succeed; `/dev/zero` mappings should behave like zero-filled anonymous memory, not as EOF-backed file pages.
- `testcases/kernel/mem/mmapstress/mmap-corruption01.c`: creates `test.mmap-corruption`, `ftruncate(fd, 128 << 20)`, maps it `MAP_SHARED`, writes/scans the whole range for 5 seconds, and expects a clean exit.
- `testcases/kernel/syscalls/ioctl/test_ioctl`: loops over `/dev/tty*`; if there are no tty devices, shell glob/listing must reach EOF quickly rather than spinning forever.

## Code changes

Files changed:

- `examples/shell/src/uspace/memory_map.rs`
- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/metadata.rs`

Implemented semantics:

1. `/dev/zero` mmap is treated as anonymous zero-fill for population/SIGBUS purposes.  Ordinary file-backed mappings still copy file bytes and keep EOF-to-`SIGBUS` behavior.
2. Large `truncate`/`ftruncate` extension now records sparse length without physically materializing zero-filled content past `MAX_IN_MEMORY_FILE_SIZE`; shrink and small-extension paths still call the underlying file truncate.  This keeps large sparse-file users like `mmap-corruption01` from exhausting memory while preserving visible file size.
3. Synthetic path symlinks/block-device `getdents64` entries are emitted once after the backing directory reaches EOF and then marked emitted for that directory handle.  This lets `/dev` enumeration terminate without replaying synthetic entries; `test_ioctl` now observes `ls: /dev/tty*: No such file or directory` and exits 0 instead of timing out.

## ABI/POSIX-visible impact

- `/dev/zero` file mappings now match Linux/POSIX expectations for zero-fill anonymous-like memory.  This intentionally changes prior incorrect `SIGBUS` behavior for `/dev/zero` only.
- Regular file mappings retain EOF/SIGBUS semantics; `mmap13` was included in adjacent regression to guard this boundary.
- Large sparse `truncate`/`ftruncate` no longer eagerly allocates all zero pages in the in-memory file implementation.  Visible size is still updated through sparse-file metadata, and adjacent `ftruncate01`, `ftruncate01_64`, `ftruncate03`, `ftruncate03_64`, `mmap09`, `fsync01`, and related mmap tests were re-run.
- `getdents64` now reaches EOF for synthetic `/dev` entries instead of replaying them forever.  Adjacent `readdir01` and `statfs01_64` were re-run.  No stable list or blacklist behavior was changed.

## Validation evidence

All promotion-quality evidence below was parsed with `scripts/ltp_summary.py`; raw logs remain in `target/` and are not committed.

| Gate | Log | Parser summary | Result |
| --- | --- | --- | --- |
| RV fixed blockers | `target/ltp-1000-g010-fullstable-blockers/rv-fixed-blockers-after-g010-fixes-20260602T231435+0800.log` | `.summary.txt` | 6 PASS, 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap |
| LA fixed blockers | `target/ltp-1000-g010-fullstable-blockers/la-fixed-blockers-after-g010-fixes-20260602T231735+0800.log` | `.summary.txt` | 6 PASS, 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap |
| RV adjacent stable regression | `target/ltp-1000-g010-fullstable-blockers/rv-adjacent-stable-regression-after-g010-fixes-20260602T231943+0800.log` | `.summary.txt` | 48 PASS, 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap |
| LA adjacent stable regression | `target/ltp-1000-g010-fullstable-blockers/la-adjacent-stable-regression-after-g010-fixes-20260602T232258+0800.log` | `.summary.txt` | 48 PASS, 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap |
| RV final getdents state check | `target/ltp-1000-g010-fullstable-blockers/rv-getdents-final-after-synthetic-state-20260602T233055+0800.log` | `.summary.txt` | 6 PASS, 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap |
| LA final getdents state check | `target/ltp-1000-g010-fullstable-blockers/la-getdents-final-after-synthetic-state-20260602T233227+0800.log` | `.summary.txt` | 6 PASS, 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap |

Adjacent stable regression case set (24 cases per libc/arch):

```text
mmap01,mmap02,mmap03,mmap04,mmap09,mmap10,mmap11,mmap12,mmap13,mmap19,mmap20,munmap01,munmap02,mincore01,mprotect05,ftruncate01,ftruncate01_64,ftruncate03,ftruncate03_64,readdir01,statfs01_64,fsync01,test_ioctl,mmap-corruption01
```

Additional diagnostic run:

- `target/ltp-1000-g010-fullstable-blockers/rv-fullblocker-subset-after-g010-fixes-20260602T230959+0800.log.summary.txt`
- Result: 19 PASS, 1 FAIL.  The failure was `clock_gettime04` musl with two `TFAIL` rows caused by a tight timing/jitter threshold; the fixed blockers passed in both libcs in that same run.  This mixed run is recorded for context but is **not** used as a clean G010 gate.

Static/build checks:

- `cargo fmt --check` passed.
- `make kernel-la -j$(nproc)` passed; log `/tmp/g010-kernel-la-build.log`.
- `run-eval.sh rv` and `run-eval.sh la` rebuilt the corresponding kernels before the parser-backed LTP runs above.

## Checksums

```text
19f833b20edcdd0575eed4c4af324b312c4896bd3cffc2e45679b7ce8d7a5b69  target/ltp-1000-g010-fullstable-blockers/rv-fixed-blockers-after-g010-fixes-20260602T231435+0800.log
7bb5c63ada69edb74f4208cdd3a76c93694f6051367e124d5eabc59cd91505aa  target/ltp-1000-g010-fullstable-blockers/rv-fixed-blockers-after-g010-fixes-20260602T231435+0800.log.summary.txt
8dbe23193bf356b85364c0529a063d4eaac8eace0a8101f48696c40a031609d0  target/ltp-1000-g010-fullstable-blockers/la-fixed-blockers-after-g010-fixes-20260602T231735+0800.log
15f3467331a06674ed8e9d57f5fea093827c503fe9198aae31a252ea848f909d  target/ltp-1000-g010-fullstable-blockers/la-fixed-blockers-after-g010-fixes-20260602T231735+0800.log.summary.txt
f784c79c921d884dbaf969632eeeae8bbdfab34f07d31496fedbcfffd012e43a  target/ltp-1000-g010-fullstable-blockers/rv-adjacent-stable-regression-after-g010-fixes-20260602T231943+0800.log
c7a3d8f9fd36e262f72a4c0de3ee04fe96f65e8e099554435aee010068c20b6a  target/ltp-1000-g010-fullstable-blockers/rv-adjacent-stable-regression-after-g010-fixes-20260602T231943+0800.log.summary.txt
95f6bab6792be6804622933edf2a17888cbdb90535dc71f9e0adde08c273ac15  target/ltp-1000-g010-fullstable-blockers/la-adjacent-stable-regression-after-g010-fixes-20260602T232258+0800.log
093ce43dc325581fca0047aa166dee83fc6faf5da9e352745f1c08838899c480  target/ltp-1000-g010-fullstable-blockers/la-adjacent-stable-regression-after-g010-fixes-20260602T232258+0800.log.summary.txt
ae6e5866360ca546517f75874f2ec1ed901b3e66bec6d2818977c6954c88f47c  target/ltp-1000-g010-fullstable-blockers/rv-fullblocker-subset-after-g010-fixes-20260602T230959+0800.log.summary.txt
84d2e9d32993882e110caf06e7eca241c1f97a97d5b252033e3d29cf081b53ef  target/ltp-1000-g010-fullstable-blockers/rv-stable706-fullblocker-targeted-20260602T224249+0800.log.summary.txt
61fb54255b98c5f912d6452dec46f8ac1d0577967627106b6becce5886633feb  target/ltp-1000-g010-fullstable-blockers/rv-getdents-final-after-synthetic-state-20260602T233055+0800.log
c59299aca7421238af1e863503587855d33bc328e13a8a321410460559ef1986  target/ltp-1000-g010-fullstable-blockers/rv-getdents-final-after-synthetic-state-20260602T233055+0800.log.summary.txt
8533c097f1c42c04e6253e589809d66af1ddcc00e1d2b64e603be8c13c4a861f  target/ltp-1000-g010-fullstable-blockers/la-getdents-final-after-synthetic-state-20260602T233227+0800.log
2a35c2508ccd158da613254614ff546e90500c9f540d9c804ab90db9bc0dc55f  target/ltp-1000-g010-fullstable-blockers/la-getdents-final-after-synthetic-state-20260602T233227+0800.log.summary.txt
```

## Caveats and next step

- No new stable cases were promoted in this repair; current stable count remains 706 total / 706 unique / 0 duplicate.
- A full `LTP_CASES=stable` RV/LA four-way final gate was not rerun in this G010 repair slice.  The focused blocker and adjacent stable regressions are clean, and the remaining post-706 expansion work should resume from a stable-first/full-sweep scouting lane.
- The diagnostic `clock_gettime04` musl jitter should be tracked separately in the time lane if it recurs under full stable runs; it is not caused by the `/dev/zero`, sparse truncate, or synthetic getdents fixes.
