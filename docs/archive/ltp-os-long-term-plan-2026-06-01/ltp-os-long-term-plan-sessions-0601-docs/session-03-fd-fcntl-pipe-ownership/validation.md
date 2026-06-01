# Session 3 validation

## Live stable count

After promotion edit:

```text
466 total / 466 unique / 0 duplicate
```

## Build / formatting

- `rustfmt examples/shell/src/uspace/fd_table.rs examples/shell/src/uspace/process_lifecycle.rs`
- `rustfmt examples/shell/src/cmd.rs`
- `df -h / /root && make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session3/session3-build-final-record-locks.log 2>&1 && df -h / /root`
  - Result: build passed; `/` and `/root` remained about `23G used / 34G available / 41%`.

## Parser-backed LTP gates

All summaries below are from `python3 -B scripts/ltp_summary.py <log>` or `--json`; raw logs stay under `target/` and are not committed.

### Final combined gate: RV FD/fcntl/pipe

- Log: `target/ltp-long-term-session3/session3-rv-fd-final-combined.log`
- Text summary: `target/ltp-long-term-session3/session3-rv-fd-final-combined-summary.txt`
- JSON summary: `target/ltp-long-term-session3/session3-rv-fd-final-combined-summary.json`
- Parser result: `PASS LTP CASE 26`, `FAIL 0`, internal `{}`, timeout `0`, ENOSYS `0`, panic/trap `0`.
- Promotion cases covered: `fcntl11`, `fcntl14`, `fcntl19`, `fcntl22` × musl/glibc.
- Adjacent stable regression subset covered: `fcntl07`, `fcntl12`, `fcntl13`, `fcntl18`, `fcntl29`, `pipe02`, `pipe08`, `pipe2_02`, `dup05` × musl/glibc.

### Final combined gate: LA FD/fcntl/pipe

- Log: `target/ltp-long-term-session3/session3-la-fd-final-combined.log`
- Text summary: `target/ltp-long-term-session3/session3-la-fd-final-combined-summary.txt`
- JSON summary: `target/ltp-long-term-session3/session3-la-fd-final-combined-summary.json`
- Parser result: `PASS LTP CASE 26`, `FAIL 0`, internal `{}`, timeout `0`, ENOSYS `0`, panic/trap `0`.
- Promotion cases covered: `fcntl11`, `fcntl14`, `fcntl19`, `fcntl22` × musl/glibc.
- Adjacent stable regression subset covered: `fcntl07`, `fcntl12`, `fcntl13`, `fcntl18`, `fcntl29`, `pipe02`, `pipe08`, `pipe2_02`, `dup05` × musl/glibc.

## Scout failures intentionally not promoted

Initial RV scout log: `target/ltp-long-term-session3/session3-rv-fd-scout.log`.

- `fcntl11`/`fcntl14`/`fcntl19`/`fcntl22` initially failed because `F_GETLK` always returned `F_UNLCK` and `F_SETLK` ignored conflicts. Fixed and promoted only after four-way clean gates.
- `fcntl30`: blocked by missing `/proc/sys/fs/pipe-max-size`.
- `pipe07`: blocked by missing `/proc/self/fd`.
- `pipe15`: blocked by missing `/proc/sys/fs/pipe-user-pages-soft`.
- `writev03`: TCONF/unsupported device-style condition; not a semantic PASS.
- `pwritev03`: TBROK/`ENOSPC` creating `test_dev.img`; not a semantic PASS.

## Checksums

Selected final evidence checksums:

```text
b414e73d8fc8cf014b60d11504bdfbb194dd3f17fb53a1323e75ac7bd71d455b  target/ltp-long-term-session3/session3-rv-fd-final-combined.log
e4a0c76678080a8402b4f28bcafe3e46724b5dd9a4827c16e0015e406757e6d6  target/ltp-long-term-session3/session3-la-fd-final-combined.log
5fe8c389a70454a35f0c27d03a15bde5df6c97509e4187a0d9f48b493bcabe75  target/ltp-long-term-session3/session3-rv-fd-final-combined-summary.txt
4101516061b5a90b701e21374e930879320a3dba99a9e83dd77b9db55a08497a  target/ltp-long-term-session3/session3-la-fd-final-combined-summary.txt
d0d674541a3566faacec6db4ba9dce27be2542177a60646f90078e91e6013f7f  target/ltp-long-term-session3/session3-rv-fd-final-combined-summary.json
a02327108979f91a5d84e8e23725bcd37e7c8e1bb9e471e8c6393f2aa73f162c  target/ltp-long-term-session3/session3-la-fd-final-combined-summary.json
```
