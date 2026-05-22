# stable101 -> stable109 promotion gate report

Date: 2026-05-22 phase-b

## Promoted cases

Added to `LTP_STABLE_CASES` in `examples/shell/src/cmd.rs`:

```text
dup202
mkdirat01
openat01
pipe04
pipe05
pread01
pwrite01
sysinfo01
```

Stable list count after edit: 109 unique cases.

## Why these cases are eligible

Evidence source: Wave A2 targeted RV and LA candidate rerun.

- RV Wave A2 raw log: `wave-a2-fd-fs-proc-rv.log`
- RV parser summary: `wave-a2-fd-fs-proc-rv-summary.txt`
- LA targeted raw log: `wave-a2-new-candidates-la.log`
- LA parser summary: `wave-a2-new-candidates-la-summary.txt`
- Matrix gate: `wave-a2-promotion-candidates.md`

`scripts/ltp_summary.py --promotion-candidates` reports exactly 8 promotion candidates across required `rv,la` x `musl,glibc` combos:

```text
dup202
mkdirat01
openat01
pipe04
pipe05
pread01
pwrite01
sysinfo01
```

For those 8 cases: wrapper status is PASS/code 0 in all four combos, internal `TFAIL=0`, `TBROK=0`, `TCONF=0`, timeout=0, ENOSYS=0, panic/trap=0.

## Explicitly not promoted from Wave A2

- `faccessat01`: RV clean, LA musl clean, but LA glibc has `TFAIL=3`, ENOSYS=3.
- `unlinkat01`: RV clean, LA musl clean, but LA glibc has `TBROK=1` / wrapper fail.
- Any RV-only clean cases already in stable or lacking LA proof were not newly promoted.

## Next required gate

Run stable targeted LA and RV after the stable list edit. Promotion must be reverted or fixed if either stable targeted run shows timeout, TFAIL/TBROK (beyond known transparent `read02` TCONF), ENOSYS, panic/trap, or wrapper failures.
