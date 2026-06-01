# milestone-02-stable606 no-promotion reason

Stable606 is not promoted in this preflight because the evidence does not yet satisfy the milestone gate:

1. Only 38 plausible candidates are currently banked (21 deferred from milestone-01 plus `socket01`, tentative `nanosleep01`, `mmap04`, `vma01`, `times03`, `mmap14`, `mmap12`, `open10`, `creat08`, `chmod07`, `fchmod02`, `access04`, `chmod06`, `chown04`, `fchmod06`, `fchown04`, and `pipe07`), short of the required +50 unique cases.
2. The 80-case RV scout produced many real failures/caveats: TFAIL/TBROK/TCONF, timeout, and ENOSYS rows remain visible and cannot be counted.
3. `nanosleep01` has one earlier grouped RV timing failure and needs later grouped gate confirmation.
4. No final stable606 RV + LA x musl + glibc gate was run.
5. `examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains at 556/556/0.
