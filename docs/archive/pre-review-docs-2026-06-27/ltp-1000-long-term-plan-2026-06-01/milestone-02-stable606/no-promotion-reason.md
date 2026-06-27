# milestone-02-stable606 no active no-promotion reason

Stable606 is now promoted. This artifact remains to record why earlier preflight "no promotion" decisions were superseded rather than hidden.

## Final state

- Current stable list: 606 total / 606 unique / 0 duplicate.
- Final promoted delta from the live 556 baseline: +50 unique cases.
- Final RV gate: `rv-stable606-final-gate-20260601T200557Z.log`, wrapper 606/606 on musl + glibc, parser 1212 PASS / 0 FAIL, no timeout/ENOSYS/panic/trap.
- Final LA gate: `la-stable606-final-gate-retry-20260601T211001Z.log`, wrapper 606/606 on musl + glibc, parser 1212 PASS / 0 FAIL, no timeout/ENOSYS/panic/trap.
- Known caveat: `read02` emits inherited `TCONF=2` per arch/libc combo, matching the earlier stable506 caveat boundary. It is disclosed and not counted as a new parser-clean row.

## Superseded blockers

Earlier blockers were not erased:

1. The candidate bank was short of +50 until the final `fcntl30`, `mknod01`, and `pipe15` evidence closed the gap.
2. The first LA full gate exited 143 with `rename14`, `kill02`, and later `times03` trouble; it is kept as non-promotion evidence.
3. Targeted LA recovery and local-order shard runs were parser-clean, then a fresh full LA retry passed the stable606 gate.
4. `statx01` and failed scout rows remain non-countable and were not promoted.

## Final +50 promoted cases

- `modify_ldt01`
- `modify_ldt02`
- `modify_ldt03`
- `print_caps`
- `test_ioctl`
- `tst_kvcmp`
- `tst_ncpus`
- `tst_ncpus_conf`
- `tst_ncpus_max`
- `tst_supported_fs`
- `fanotify_child`
- `genload`
- `gensin`
- `gensinh`
- `gensqrt`
- `gentan`
- `gentanh`
- `geny0`
- `geny1`
- `tst_exit`
- `tst_hexdump`
- `socket01`
- `nanosleep01`
- `mmap04`
- `vma01`
- `times03`
- `mmap14`
- `mmap12`
- `open10`
- `creat08`
- `chmod07`
- `fchmod02`
- `access04`
- `chmod06`
- `chown04`
- `fchmod06`
- `fchown04`
- `pipe07`
- `mknod03`
- `mknod04`
- `mknod09`
- `fchownat02`
- `setrlimit04`
- `clock_gettime04`
- `locktests`
- `ltpServer`
- `stress`
- `fcntl30`
- `mknod01`
- `pipe15`
